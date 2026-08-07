use bytecheck::CheckBytes;

use rkyv::{Archive, Deserialize, Serialize};

use std::collections::HashMap;

use crate::{
    entity::prelude::types::{Alias, EntityID},
    utils::normalize,
};

#[derive(Default, Debug, Clone)]
pub struct InvertedIndex {
    pub index: HashMap<Alias, Vec<EntityID>>,
}

impl InvertedIndex {
    pub fn insert(&mut self, term: &str, entity_id: EntityID) {
        self.index
            .entry(normalize(term))
            .or_default()
            .push(entity_id);
    }

    pub fn lookup(&self, term: &str) -> Vec<EntityID> {
        self.index
            .get(&normalize(term))
            .cloned()
            .unwrap_or_default()
    }
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone, CheckBytes, Default)]
pub struct InvertedIndexRecord {
    pub entries: HashMap<Box<str>, Vec<EntityID>>,
}

impl InvertedIndex {
    pub fn to_record(&self) -> InvertedIndexRecord {
        InvertedIndexRecord {
            entries: self.index.clone(),
        }
    }

    pub fn from_record(record: InvertedIndexRecord) -> Self {
        Self {
            index: record.entries,
        }
    }
}
