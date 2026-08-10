#[cfg(test)]
use std::path::Path;

use crate::{DatabaseRecord, config::StorageConfig, error::Error, error::Result};

use super::segment::Segment;

pub struct SegmentManager {
    config: StorageConfig,
    active: Segment,
    active_id: u64,
}

impl SegmentManager {
    pub fn create(config: StorageConfig) -> Result<Self> {
        let directory = config.segment_path();

        std::fs::create_dir_all(&directory)?;

        let path = directory.join("000001.eir");

        if path.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "storage already contains segments",
            )));
        }

        let active = Segment::create(&path);

        Ok(Self {
            config,
            active,
            active_id: 1,
        })
    }

    pub fn open(config: StorageConfig) -> Result<Self> {
        let directory = config.segment_path();

        if !directory.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "segment directory does not exist",
            )));
        }

        let mut ids = Vec::new();

        for entry in std::fs::read_dir(&directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|x| x.to_str()) != Some("eir") {
                continue;
            }

            let Some(stem) = path.file_stem().and_then(|x| x.to_str()) else {
                continue;
            };

            if let Ok(id) = stem.parse::<u64>() {
                ids.push((id, path));
            }
        }

        let (active_id, active_path) =
            ids.into_iter().max_by_key(|(id, _)| *id).ok_or_else(|| {
                Error::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "no segments found",
                ))
            })?;

        let active = Segment::open(active_path)?;

        Ok(Self {
            config,
            active,
            active_id,
        })
    }

    #[cfg(test)]
    pub fn path(&self) -> &Path {
        self.active.path()
    }

    #[cfg(test)]
    pub fn active_id(&self) -> u64 {
        self.active_id
    }

    pub fn should_rotate(&self, payload_len: u64) -> Result<bool> {
        Ok(self.active.size()? + payload_len > self.config.max_segment_size)
    }

    pub fn rotate(&mut self) -> Result<()> {
        if self.active_id as usize >= self.config.max_segments {
            return Err(Error::StorageLimit {
                max_segments: self.config.max_segments,
            });
        }

        let next_id = self.active_id + 1;
        let directory = self.config.segment_path();
        let path = directory.join(format!("{:06}.eir", next_id));

        if path.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "next segment already exists",
            )));
        }

        self.active = Segment::create(path);
        self.active_id = next_id;

        Ok(())
    }

    pub fn write(&mut self, payload: &[u8]) -> Result<()> {
        let payload_len = payload.len() as u64;

        if self.should_rotate(payload_len)? {
            self.rotate()?;
        }

        self.active.write(payload)
    }

    pub fn write_record(&mut self, record: &DatabaseRecord) -> Result<()> {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(record)
            .map_err(|error| Error::Serialization(error.to_string()))?;

        self.write(&bytes)
    }

    pub fn read_record(&self) -> Result<DatabaseRecord> {
        let bytes = self.active.read()?;

        rkyv::from_bytes::<DatabaseRecord, rkyv::rancor::Error>(&bytes)
            .map_err(|error| Error::Serialization(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manager_creates_first_segment() -> Result<()> {
        let temp = tempfile::tempdir()?;

        let config = StorageConfig {
            root: temp.path().to_path_buf(),
            max_segment_size: 1024,
            max_segments: 4,
        };

        let manager = SegmentManager::create(config)?;

        assert_eq!(
            manager.path(),
            temp.path().join("segments").join("000001.eir")
        );

        Ok(())
    }

    #[test]
    fn manager_rotates_segment() -> Result<()> {
        let temp = tempfile::tempdir()?;

        let config = StorageConfig {
            root: temp.path().to_path_buf(),
            max_segment_size: 1024,
            max_segments: 4,
        };

        let mut manager = SegmentManager::create(config)?;

        assert_eq!(manager.active_id(), 1);
        assert!(manager.path().ends_with("000001.eir"));

        manager.rotate()?;

        assert_eq!(manager.active_id(), 2);
        assert!(manager.path().ends_with("000002.eir"));

        manager.rotate()?;

        assert_eq!(manager.active_id(), 3);
        assert!(manager.path().ends_with("000003.eir"));

        Ok(())
    }

    #[test]
    fn segment_reports_size() -> Result<()> {
        let path = std::env::temp_dir().join(format!("eir-size-{}.eir", std::process::id()));

        let segment = Segment::create(&path);
        segment.write(b"hello")?;

        assert!(segment.size()? > 0);

        std::fs::remove_file(path).ok();
        Ok(())
    }

    #[test]
    fn manager_opens_existing_segments() -> Result<()> {
        let temp = tempfile::tempdir()?;

        let config = StorageConfig {
            root: temp.path().to_path_buf(),
            max_segment_size: 1024,
            max_segments: 4,
        };

        let mut manager = SegmentManager::create(config.clone())?;

        println!("root: {:?}", config.root);
        println!("segment dir: {:?}", config.segment_path());
        println!("active path: {:?}", manager.path());

        assert!(config.segment_path().exists());

        manager.write(b"hello")?;

        println!("active exists: {:?}", manager.path().exists());

        for entry in std::fs::read_dir(config.segment_path())? {
            println!("entry: {:?}", entry?.path());
        }

        let reopened = SegmentManager::open(config)?;

        assert_eq!(reopened.active_id(), manager.active_id());

        Ok(())
    }

    #[test]
    fn manager_rejects_rotation_at_segment_limit() -> Result<()> {
        let temp = tempfile::tempdir()?;

        let config = StorageConfig {
            root: temp.path().to_path_buf(),
            max_segment_size: 1024,
            max_segments: 2,
        };

        let mut manager = SegmentManager::create(config)?;

        manager.rotate()?;

        assert_eq!(manager.active_id(), 2);

        let result = manager.rotate();

        assert!(matches!(
            result,
            Err(Error::StorageLimit { max_segments: 2 })
        ));

        assert_eq!(manager.active_id(), 2);

        Ok(())
    }
}
