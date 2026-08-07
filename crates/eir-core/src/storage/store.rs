use rkyv::{Archive, Deserialize, Serialize};

use super::posting_list::PostingListRecord;

use crate::entity::prelude::{
    EntityDocument,
    types::{SourceID, TagID},
};

use crate::error::Result;

#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct Store {
    pub entities: Vec<EntityDocument>,

    pub tags: PostingListRecord<TagID>,

    pub sources: PostingListRecord<SourceID>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            tags: PostingListRecord::default(),
            sources: PostingListRecord::default(),
        }
    }

    pub fn load(_path: impl AsRef<std::path::Path>) -> Result<Self> {
        // read serialized data
        todo!()
    }

    pub fn save(&self) -> Result<()> {
        Ok(())
    }
}
