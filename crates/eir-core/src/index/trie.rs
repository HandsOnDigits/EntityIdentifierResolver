use bytecheck::CheckBytes;

use rkyv::{Archive, Deserialize, Serialize};

use std::collections::HashMap;

use crate::{entity::prelude::types::EntityID, utils::normalize};

#[derive(Default, Debug)]
pub struct TrieIndex {
    root: HashMap<Box<str>, Vec<EntityID>>,
}

impl TrieIndex {
    pub fn insert(&mut self, text: &str, entity_id: EntityID) {
        self.root
            .entry(normalize(text))
            .or_default()
            .push(entity_id);
    }

    pub fn prefix(&self, prefix: &str) -> Vec<EntityID> {
        self.root
            .iter()
            .filter(|(key, _)| key.starts_with(prefix))
            .flat_map(|(_, ids)| ids.clone())
            .collect()
    }

    pub fn export(&self) -> Vec<(Box<str>, Vec<EntityID>)> {
        self.root
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect()
    }
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone, CheckBytes, Default)]
pub struct TrieIndexRecord {
    pub entries: Vec<(Box<str>, Vec<EntityID>)>,
}

impl TrieIndex {
    pub fn to_record(&self) -> TrieIndexRecord {
        TrieIndexRecord {
            entries: self.export(),
        }
    }

    pub fn from_record(record: TrieIndexRecord) -> Self {
        let mut trie = Self::default();

        for (word, ids) in record.entries {
            for id in ids {
                trie.insert(&word, id);
            }
        }

        trie
    }
}
