use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

use std::collections::HashMap;

use crate::{
    entity::prelude::types::{Alias, EntityID},
    utils::normalize,
};

#[derive(Default, Debug, Clone)]
pub struct InvertedIndex {
    pub tokens: HashMap<Alias, Vec<EntityID>>,
}

impl InvertedIndex {
    pub fn insert(&mut self, term: &str, entity_id: EntityID) {
        let normalized = normalize(term);

        self.tokens.entry(normalized).or_default().push(entity_id);
    }

    pub fn lookup(&self, term: &str) -> Vec<EntityID> {
        self.tokens
            .get(&normalize(term))
            .cloned()
            .unwrap_or_default()
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone, CheckBytes, Default)]
pub struct InvertedIndexRecord {
    pub index: HashMap<Box<str>, Vec<EntityID>>,
}

impl InvertedIndex {
    pub fn to_record(&self) -> InvertedIndexRecord {
        InvertedIndexRecord {
            index: self.tokens.clone(),
        }
    }

    pub fn from_record(record: InvertedIndexRecord) -> Self {
        Self {
            tokens: record.index,
        }
    }
}
