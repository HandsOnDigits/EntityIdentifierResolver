use rkyv::{Archive, Deserialize, Serialize, bytecheck::CheckBytes};

use crate::{
    entity::{
        EntityDocument,
        types::{SourceID, TagID},
    },
    storage::PostingListRecord,
};

#[derive(Debug, Archive, Serialize, Deserialize, CheckBytes)]
pub struct Database {
    pub entities: Vec<EntityDocument>,

    pub tags: Vec<Box<str>>,
    pub sources: Vec<Box<str>>,
    pub properties: Vec<Box<str>>,

    pub tag_index: PostingListRecord<TagID>,
    pub source_index: PostingListRecord<SourceID>,
}
