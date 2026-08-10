use std::path::Path;

use crate::{
    entity::prelude::{input::EntityInput, types::EntityID},
    error::{Error, Result},
    storage::deir::{DeirFile, DeirKind},
};

#[derive(Debug, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum WalOperation {
    Insert(EntityInput),
    Remove(EntityID),
}

pub struct Wal {
    file: DeirFile,
}

impl Wal {
    pub fn create(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            file: DeirFile::create(path, DeirKind::Wal)?,
        })
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            file: DeirFile::open(path, DeirKind::Wal)?,
        })
    }

    pub fn append(&mut self, operation: &WalOperation) -> Result<()> {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(operation)
            .map_err(|error| Error::Serialization(error.to_string()))?;

        let length = u32::try_from(bytes.len())
            .map_err(|_| Error::Serialization("WAL record is too large".into()))?;

        let mut record = Vec::with_capacity(4 + bytes.len());

        record.extend_from_slice(&length.to_le_bytes());
        record.extend_from_slice(&bytes);

        self.file.append(&record)?;

        Ok(())
    }

    pub fn replay(&self) -> Result<Vec<WalOperation>> {
        let bytes = self.file.read()?;
        let mut cursor = 0;
        let mut operations = Vec::new();

        while cursor < bytes.len() {
            let remaining = bytes.len() - cursor;

            if remaining < 4 {
                // Incomplete final length header.
                break;
            }

            let length = u32::from_le_bytes(
                bytes[cursor..cursor + 4]
                    .try_into()
                    .expect("checked length"),
            ) as usize;

            cursor += 4;

            if bytes.len() - cursor < length {
                // Incomplete final payload.
                break;
            }

            let payload = &bytes[cursor..cursor + length];
            cursor += length;

            let mut aligned: rkyv::util::AlignedVec = rkyv::util::AlignedVec::with_capacity(length);
            aligned.extend_from_slice(payload);

            let operation = rkyv::from_bytes::<WalOperation, rkyv::rancor::Error>(&aligned)
                .map_err(|error| Error::Serialization(error.to_string()))?;

            operations.push(operation);
        }

        Ok(operations)
    }

    pub fn truncate(&mut self) -> Result<()> {
        self.file.truncate()
    }

    pub fn sync(&self) -> Result<()> {
        self.file.sync()
    }

    pub fn path(&self) -> &Path {
        self.file.path()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::prelude::types::EntityID;

    #[test]
    fn wal_append_multiple_roundtrip() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("wal.deir");

        let mut wal = Wal::create(&path)?;

        wal.append(&WalOperation::Remove(EntityID::new(1)))?;
        wal.append(&WalOperation::Remove(EntityID::new(2)))?;

        let operations = wal.replay()?;

        assert_eq!(operations.len(), 2);

        assert!(matches!(
            operations[0],
            WalOperation::Remove(id) if id == EntityID::new(1)
        ));

        assert!(matches!(
            operations[1],
            WalOperation::Remove(id) if id == EntityID::new(2)
        ));

        Ok(())
    }

    #[test]
    fn wal_insert_roundtrip() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("wal.deir");

        let mut wal = Wal::create(&path)?;

        wal.append(&WalOperation::Insert(EntityInput {
            id: 9300,
            aliases: vec!["WAL Berry".into()],
            tags: vec![],
            properties: vec![],
            relationships: vec![],
            sources: vec![],
        }))?;

        let operations = wal.replay()?;

        assert_eq!(operations.len(), 1);

        match &operations[0] {
            WalOperation::Insert(input) => {
                assert_eq!(input.id, 9300);
                assert_eq!(input.aliases, vec!["WAL Berry".into()]);
            }
            _ => panic!("expected insert operation"),
        }

        Ok(())
    }

    #[test]
    fn wal_insert_survives_reopen() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("wal.deir");

        {
            let mut wal = Wal::create(&path)?;

            wal.append(&WalOperation::Insert(EntityInput {
                id: 9300,
                aliases: vec!["WAL Berry".into()],
                tags: vec![],
                properties: vec![],
                relationships: vec![],
                sources: vec![],
            }))?;

            wal.sync()?;
        }

        {
            let wal = Wal::open(&path)?;
            let operations = wal.replay()?;

            assert_eq!(operations.len(), 1);

            match &operations[0] {
                WalOperation::Insert(input) => {
                    assert_eq!(input.id, 9300);
                    assert_eq!(input.aliases, vec!["WAL Berry".into()]);
                }
                _ => panic!("expected insert operation"),
            }
        }

        Ok(())
    }
}
