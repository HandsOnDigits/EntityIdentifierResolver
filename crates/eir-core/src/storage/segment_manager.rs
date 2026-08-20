use std::path::Path;

use rand::distr::{Alphanumeric, SampleString};

use crate::{
    DatabaseRecord,
    config::StorageConfig,
    error::{Error, Result},
};

use super::{
    deir::{DeirFile, DeirKind},
    segment::Segment,
};

const SEGMENT_EXTENSION: &str = "deir";
const RECORD_HEADER_SIZE: usize = 8;
const SEGMENT_NAME_LENGTH: usize = 16;

fn random_segment_name() -> String {
    Alphanumeric.sample_string(&mut rand::rng(), SEGMENT_NAME_LENGTH)
}

pub struct SegmentManager {
    config: StorageConfig,
    active: Segment,
}

impl SegmentManager {
    pub fn create(config: StorageConfig) -> Result<Self> {
        let directory = config.segment_path();

        std::fs::create_dir_all(&directory)?;

        let active = Self::create_segment(&directory)?;

        Ok(Self { config, active })
    }

    fn create_segment(directory: &Path) -> Result<Segment> {
        loop {
            let name = random_segment_name();
            let path = directory.join(format!("{name}.{SEGMENT_EXTENSION}"));

            match Segment::create(&path) {
                Ok(segment) => return Ok(segment),
                Err(Error::Io(error)) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    continue;
                }
                Err(error) => return Err(error),
            }
        }
    }

    pub fn open(config: StorageConfig) -> Result<Self> {
        let directory = config.segment_path();

        if !directory.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "segment directory does not exist",
            )));
        }

        let mut segments = Vec::new();

        for entry in std::fs::read_dir(&directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|x| x.to_str()) != Some(SEGMENT_EXTENSION) {
                continue;
            }

            segments.push(path);
        }

        let active_path = segments.into_iter().next().ok_or_else(|| {
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "no segments found",
            ))
        })?;

        let active = Segment::open(active_path)?;

        Ok(Self { config, active })
    }

    #[cfg(test)]
    pub fn path(&self) -> &Path {
        self.active.path()
    }

    pub fn should_rotate(&self, payload_len: u64) -> Result<bool> {
        Ok(self.active.size()? + payload_len > self.config.max_segment_size)
    }

    fn segment_count(&self) -> Result<usize> {
        let directory = self.config.segment_path();

        Ok(std::fs::read_dir(directory)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().extension().and_then(|x| x.to_str()) == Some(SEGMENT_EXTENSION)
            })
            .count())
    }

    pub fn rotate(&mut self) -> Result<()> {
        if self.segment_count()? >= self.config.max_segments {
            return Err(Error::StorageLimit {
                max_segments: self.config.max_segments,
            });
        }

        self.active = Self::create_segment(&self.config.segment_path())?;

        Ok(())
    }

    pub fn write_record(&mut self, record: &DatabaseRecord) -> Result<()> {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(record)
            .map_err(|error| Error::Serialization(error.to_string()))?;

        let framed = Self::encode_record(&bytes)?;

        let framed_len = framed.len() as u64;

        if self.should_rotate(framed_len)? {
            self.rotate()?;
        }

        self.active.append(&framed)?;

        Ok(())
    }

    pub fn read_record(&self) -> Result<DatabaseRecord> {
        let bytes = self.active.read()?;

        let record = Self::decode_last_record(&bytes)?;

        rkyv::from_bytes::<DatabaseRecord, rkyv::rancor::Error>(record)
            .map_err(|error| Error::Serialization(error.to_string()))
    }

    pub fn rewrite(&mut self, record: &DatabaseRecord) -> Result<()> {
        let directory = self.config.segment_path();

        let name = random_segment_name();

        let temp_path = directory.join(format!(".rewrite.{name}.tmp"));
        let new_path = directory.join(format!("{name}.{SEGMENT_EXTENSION}"));

        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(record)
            .map_err(|error| Error::Serialization(error.to_string()))?;

        {
            let temporary = DeirFile::create(&temp_path, DeirKind::Segment)?;

            let framed = Self::encode_record(&bytes)?;
            temporary.write(&framed)?;
        }

        std::fs::rename(&temp_path, &new_path)?;

        for entry in std::fs::read_dir(&directory)? {
            let entry = entry?;
            let path = entry.path();

            if path == new_path {
                continue;
            }

            if path.extension().and_then(|x| x.to_str()) == Some(SEGMENT_EXTENSION) {
                std::fs::remove_file(path)?;
            }
        }

        self.active = Segment::open(&new_path)?;

        Ok(())
    }

    fn encode_record(payload: &[u8]) -> Result<Vec<u8>> {
        let len = u64::try_from(payload.len())
            .map_err(|_| Error::InvalidFormat("snapshot is too large".into()))?;

        let mut framed = Vec::with_capacity(RECORD_HEADER_SIZE + payload.len());

        framed.extend_from_slice(&len.to_le_bytes());
        framed.extend_from_slice(payload);

        Ok(framed)
    }

    fn decode_last_record(bytes: &[u8]) -> Result<&[u8]> {
        if bytes.is_empty() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "database snapshot does not exist",
            )));
        }

        let mut offset = 0;

        while offset < bytes.len() {
            if bytes.len() - offset < RECORD_HEADER_SIZE {
                return Err(Error::InvalidFormat(
                    "truncated segment record header".into(),
                ));
            }

            let len = u64::from_le_bytes(
                bytes[offset..offset + RECORD_HEADER_SIZE]
                    .try_into()
                    .unwrap(),
            );

            let len = usize::try_from(len)
                .map_err(|_| Error::InvalidFormat("snapshot is too large".into()))?;

            let payload_start = offset + RECORD_HEADER_SIZE;
            let payload_end = payload_start
                .checked_add(len)
                .ok_or_else(|| Error::InvalidFormat("snapshot length overflow".into()))?;

            if payload_end > bytes.len() {
                return Err(Error::InvalidFormat(
                    "truncated segment record payload".into(),
                ));
            }

            offset = payload_end;

            if offset == bytes.len() {
                return Ok(&bytes[payload_start..payload_end]);
            }
        }

        Err(Error::InvalidFormat(
            "segment contains no complete record".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Database;

    fn test_config(root: &Path, max_segment_size: u64, max_segments: usize) -> StorageConfig {
        StorageConfig {
            name: "test".into(),
            root: root.to_path_buf(),
            max_segment_size,
            max_segments,
        }
    }

    #[test]
    fn manager_creates_first_segment() -> Result<()> {
        let temp = tempfile::tempdir()?;

        let config = test_config(temp.path(), 1024, 4);
        let manager = SegmentManager::create(config)?;

        let path = manager.path();

        assert!(path.exists());
        assert_eq!(path.parent(), Some(temp.path().join("segments").as_path()));
        assert_eq!(
            path.extension().and_then(|x| x.to_str()),
            Some(SEGMENT_EXTENSION)
        );

        Ok(())
    }

    #[test]
    fn manager_rotates_segment() -> Result<()> {
        let temp = tempfile::tempdir()?;

        let config = test_config(temp.path(), 1024, 4);
        let mut manager = SegmentManager::create(config)?;

        let first = manager.path().to_owned();

        manager.rotate()?;

        let second = manager.path().to_owned();

        assert_ne!(first, second);
        assert!(first.exists());
        assert!(second.exists());

        manager.rotate()?;

        let third = manager.path().to_owned();

        assert_ne!(second, third);
        assert_ne!(first, third);
        assert!(third.exists());

        Ok(())
    }

    #[test]
    fn segment_reports_size() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("segment.deir");

        let segment = Segment::create(&path)?;
        segment.write(b"hello")?;

        assert!(segment.size()? > 0);

        Ok(())
    }

    #[test]
    fn manager_rejects_rotation_at_segment_limit() -> Result<()> {
        let temp = tempfile::tempdir()?;

        let config = test_config(temp.path(), 1024, 2);
        let mut manager = SegmentManager::create(config)?;

        manager.rotate()?;

        let active = manager.path().to_owned();

        let result = manager.rotate();

        assert!(matches!(
            result,
            Err(Error::StorageLimit { max_segments: 2 })
        ));

        assert_eq!(manager.path(), active);

        Ok(())
    }

    #[test]
    fn manager_read_empty_segment_returns_not_found() -> Result<()> {
        let temp = tempfile::tempdir()?;

        let config = test_config(temp.path(), 64 * 1024 * 1024, 16);

        let manager = SegmentManager::create(config)?;

        let result = manager.read_record();

        assert!(matches!(
            result,
            Err(Error::Io(error))
                if error.kind() == std::io::ErrorKind::NotFound
        ));

        Ok(())
    }

    #[test]
    fn manager_can_rotate_after_rewrite() -> Result<()> {
        let temp = tempfile::tempdir()?;

        let config = test_config(temp.path(), 1024 * 1024, 2);
        let mut manager = SegmentManager::create(config)?;

        manager.rotate()?;

        let before_rewrite = manager.path().to_owned();

        let database = Database::default();
        manager.rewrite(&database.to_record())?;

        let after_rewrite = manager.path().to_owned();

        // Rewrite leaves only one physical segment.
        assert_ne!(before_rewrite, after_rewrite);

        // We should be able to create another segment because only one
        // physical segment remains.
        manager.rotate()?;

        let after_rotate = manager.path().to_owned();

        assert_ne!(after_rewrite, after_rotate);

        Ok(())
    }

    #[test]
    fn manager_appends_snapshots_and_reads_latest() -> Result<()> {
        let temp = tempfile::tempdir()?;

        let config = test_config(temp.path(), 1024 * 1024, 4);
        let mut manager = SegmentManager::create(config)?;

        let first = Database::default().to_record();

        manager.write_record(&first)?;

        let first_size = manager.path().metadata()?.len();

        let second = Database::default().to_record();

        manager.write_record(&second)?;

        let second_size = manager.path().metadata()?.len();

        assert!(second_size > first_size);

        let loaded = manager.read_record()?;

        assert_eq!(loaded.entities.len(), second.entities.len());

        Ok(())
    }

    #[test]
    fn manager_rewrite_reduces_append_only_storage() -> Result<()> {
        let temp = tempfile::tempdir()?;

        let config = test_config(temp.path(), 1024 * 1024, 4);
        let mut manager = SegmentManager::create(config)?;

        let database = Database::default();
        let record = database.to_record();

        manager.write_record(&record)?;
        manager.write_record(&record)?;
        manager.write_record(&record)?;

        let before = manager.path().metadata()?.len();

        manager.rewrite(&record)?;

        let after = manager.path().metadata()?.len();

        assert!(after < before);

        Ok(())
    }

    #[test]
    fn manager_generates_random_segment_names() -> Result<()> {
        let temp = tempfile::tempdir()?;

        let config = test_config(temp.path(), 1024, 4);
        let mut manager = SegmentManager::create(config)?;

        let first = manager.path().to_owned();

        manager.rotate()?;

        let second = manager.path().to_owned();

        assert_ne!(first, second);

        assert_eq!(
            first.extension().and_then(|x| x.to_str()),
            Some(SEGMENT_EXTENSION)
        );

        assert_eq!(
            second.extension().and_then(|x| x.to_str()),
            Some(SEGMENT_EXTENSION)
        );

        Ok(())
    }
}
