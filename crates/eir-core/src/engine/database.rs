use rkyv::{Archive, Deserialize, Serialize, bytecheck::CheckBytes};

use crate::{
    entity::{
        EntityDocument,
        types::{EntityID, SourceID, TagID},
    },
    storage::PostingListRecord,
};

use std::collections::HashMap;

#[derive(Debug, Archive, Serialize, Deserialize, CheckBytes)]
pub struct Database {
    pub entities: Vec<EntityDocument>,

    // entity_id -> aliases
    pub aliases: HashMap<EntityID, Vec<Box<str>>>,

    pub tags: Vec<Box<str>>,
    pub sources: Vec<Box<str>>,
    pub properties: Vec<Box<str>>,

    pub tag_index: PostingListRecord<TagID>,
    pub source_index: PostingListRecord<SourceID>,
}
