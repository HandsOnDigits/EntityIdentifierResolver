use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, Write},
    path::{Path, PathBuf},
};

use crate::{
    entity::prelude::{input::EntityInput, types::EntityID},
    error::{Error, Result},
};

#[derive(Debug, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum WalOperation {
    Insert(EntityInput),
    Remove(EntityID),
}

pub struct Wal {
    path: PathBuf,
    file: File,
}

impl Wal {
    pub fn create(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = OpenOptions::new()
            .create_new(true)
            .read(true)
            .write(true)
            .open(&path)?;

        Ok(Self { path, file })
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        if !path.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "WAL does not exist",
            )));
        }

        let file = OpenOptions::new().read(true).write(true).open(&path)?;

        Ok(Self { path, file })
    }

    pub fn append(&mut self, operation: &WalOperation) -> Result<()> {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(operation)
            .map_err(|error| Error::Serialization(error.to_string()))?;

        let length = u32::try_from(bytes.len())
            .map_err(|_| Error::Serialization("WAL record is too large".into()))?;

        self.file.seek(std::io::SeekFrom::End(0))?;
        self.file.write_all(&length.to_le_bytes())?;
        self.file.write_all(&bytes)?;
        self.file.flush()?;
        self.file.sync_data()?;

        Ok(())
    }

    pub fn replay(&self) -> Result<Vec<WalOperation>> {
        let mut file = File::open(&self.path)?;
        let mut operations = Vec::new();

        loop {
            let mut length = [0u8; 4];

            match file.read_exact(&mut length) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => {
                    break;
                }
                Err(error) => return Err(error.into()),
            }

            let length = u32::from_le_bytes(length) as usize;
            let mut bytes = vec![0u8; length];

            match file.read_exact(&mut bytes) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => {
                    // Ignore an incomplete final record caused by a crash.
                    break;
                }
                Err(error) => return Err(error.into()),
            }

            let operation = rkyv::from_bytes::<WalOperation, rkyv::rancor::Error>(&bytes)
                .map_err(|error| Error::Serialization(error.to_string()))?;

            operations.push(operation);
        }

        Ok(operations)
    }

    pub fn truncate(&mut self) -> Result<()> {
        self.file.set_len(0)?;
        self.file.seek(std::io::SeekFrom::Start(0))?;
        self.file.sync_all()?;

        Ok(())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wal_roundtrip() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("wal.eir");

        let mut wal = Wal::create(&path)?;

        wal.append(&WalOperation::Remove(EntityID::new(42)))?;

        let operations = wal.replay()?;

        assert_eq!(operations.len(), 1);

        match &operations[0] {
            WalOperation::Remove(id) => {
                assert_eq!(*id, EntityID::new(42));
            }
            _ => panic!("expected remove operation"),
        }

        Ok(())
    }

    #[test]
    fn wal_truncate_removes_operations() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("wal.eir");

        let mut wal = Wal::create(&path)?;

        wal.append(&WalOperation::Remove(EntityID::new(42)))?;

        assert_eq!(wal.replay()?.len(), 1);

        wal.truncate()?;

        assert!(wal.replay()?.is_empty());

        Ok(())
    }

    #[test]
    fn wal_replays_multiple_operations() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("wal.eir");

        let mut wal = Wal::create(&path)?;

        wal.append(&WalOperation::Remove(EntityID::new(1)))?;
        wal.append(&WalOperation::Remove(EntityID::new(2)))?;
        wal.append(&WalOperation::Remove(EntityID::new(3)))?;

        let operations = wal.replay()?;

        assert_eq!(operations.len(), 3);

        assert!(matches!(
            operations[0],
            WalOperation::Remove(id) if id == EntityID::new(1)
        ));

        assert!(matches!(
            operations[1],
            WalOperation::Remove(id) if id == EntityID::new(2)
        ));

        assert!(matches!(
            operations[2],
            WalOperation::Remove(id) if id == EntityID::new(3)
        ));

        Ok(())
    }

    #[test]
    fn wal_ignores_truncated_length_header() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("wal.eir");

        let mut wal = Wal::create(&path)?;

        wal.append(&WalOperation::Remove(EntityID::new(42)))?;

        // Simulate a crash while writing the next record's length.
        wal.file.write_all(&[0x01, 0x02])?;
        wal.file.flush()?;

        let operations = wal.replay()?;

        assert_eq!(operations.len(), 1);

        Ok(())
    }

    #[test]
    fn wal_ignores_truncated_final_payload() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("wal.eir");

        let mut wal = Wal::create(&path)?;

        wal.append(&WalOperation::Remove(EntityID::new(42)))?;

        // Simulate a crash after writing a length but only part
        // of the payload.
        wal.file.write_all(&10u32.to_le_bytes())?;
        wal.file.write_all(&[1, 2, 3])?;
        wal.file.flush()?;

        let operations = wal.replay()?;

        assert_eq!(operations.len(), 1);

        match &operations[0] {
            WalOperation::Remove(id) => {
                assert_eq!(*id, EntityID::new(42));
            }
            _ => panic!("expected remove operation"),
        }

        Ok(())
    }

    #[test]
    fn wal_reports_corrupt_record() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("wal.eir");

        let mut wal = Wal::create(&path)?;

        wal.append(&WalOperation::Remove(EntityID::new(42)))?;

        let corrupt = [0xffu8; 8];

        let length = u32::try_from(corrupt.len()).unwrap();

        wal.file.write_all(&length.to_le_bytes())?;
        wal.file.write_all(&corrupt)?;
        wal.file.flush()?;

        let result = wal.replay();

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn wal_survives_reopen() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("wal.eir");

        {
            let mut wal = Wal::create(&path)?;
            wal.append(&WalOperation::Remove(EntityID::new(42)))?;
            wal.file.sync_all()?;
        }

        {
            let wal = Wal::open(&path)?;
            let operations = wal.replay()?;

            assert_eq!(operations.len(), 1);

            assert!(matches!(
                operations[0],
                WalOperation::Remove(id) if id == EntityID::new(42)
            ));
        }

        Ok(())
    }
}
