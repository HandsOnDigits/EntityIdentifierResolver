use std::path::{Path, PathBuf};

use rkyv::{Archive, Deserialize, Serialize};

use crate::error::{Error, Result};

use super::{posting_list::PostingListRecord, segment::Segment};

use crate::entity::prelude::{
    EntityDocument,
    types::{SourceID, TagID},
};

#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct StoreRecord {
    pub entities: Vec<EntityDocument>,
    pub tags: PostingListRecord<TagID>,
    pub sources: PostingListRecord<SourceID>,
}

#[derive(Debug)]
pub struct Store {
    path: PathBuf,
    pub entities: Vec<EntityDocument>,
    pub tags: PostingListRecord<TagID>,
    pub sources: PostingListRecord<SourceID>,
}

impl Store {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            entities: Vec::new(),
            tags: PostingListRecord::default(),
            sources: PostingListRecord::default(),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn to_record(&self) -> StoreRecord {
        StoreRecord {
            entities: self.entities.clone(),
            tags: self.tags.clone(),
            sources: self.sources.clone(),
        }
    }

    fn from_record(path: PathBuf, record: StoreRecord) -> Self {
        Self {
            path,
            entities: record.entities,
            tags: record.tags,
            sources: record.sources,
        }
    }

    pub fn save(&self) -> Result<()> {
        let record = self.to_record();

        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&record)
            .map_err(|error| Error::Serialization(error.to_string()))?;

        let segment = Segment::create(&self.path)?;

        segment.write(&bytes)?;

        Ok(())
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        let segment = Segment::open(&path)?;
        let bytes = segment.read()?;

        let record = rkyv::from_bytes::<StoreRecord, rkyv::rancor::Error>(&bytes)
            .map_err(|error| Error::Serialization(error.to_string()))?;

        Ok(Self::from_record(path, record))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::entity::prelude::types::EntityID;

    #[test]
    fn store_roundtrip() -> Result<()> {
        let path = std::env::temp_dir().join(format!("eir-store-{}.eir", std::process::id()));

        let mut store = Store::new(&path);

        store.entities.push(EntityDocument {
            id: EntityID::new(100),
            aliases: vec!["Test Berry".into()],
            tags: vec![],
            attributes: vec![],
            relationships: vec![],
            sources: vec![],
        });

        store.save()?;

        let loaded = Store::load(&path)?;

        assert_eq!(loaded.entities.len(), 1);
        assert_eq!(loaded.entities[0].id, EntityID::new(100));
        assert_eq!(loaded.entities[0].aliases, vec!["Test Berry".into()]);

        std::fs::remove_file(path).ok();

        Ok(())
    }

    #[test]
    fn store_load_missing_file_returns_error() {
        let path =
            std::env::temp_dir().join(format!("eir-store-missing-{}.eir", std::process::id()));

        let result = Store::load(&path);

        assert!(matches!(result, Err(Error::Io(_))));
    }
}
