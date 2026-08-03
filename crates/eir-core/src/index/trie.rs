use std::collections::HashMap;

use crate::entity::types::EntityID;

#[derive(Default, Debug)]
pub struct TrieIndex {
    entries: HashMap<String, Vec<EntityID>>,
}

impl TrieIndex {
    pub fn insert(&mut self, text: &str, entity_id: EntityID) {
        self.entries
            .entry(text.to_lowercase())
            .or_default()
            .push(entity_id);
    }

    pub fn prefix(&self, prefix: &str) -> Vec<EntityID> {
        self.entries
            .iter()
            .filter(|(key, _)| key.starts_with(prefix))
            .flat_map(|(_, ids)| ids.clone())
            .collect()
    }
}
