use std::collections::HashMap;

use crate::entity::types::EntityID;

#[derive(Default)]
pub struct InvertedIndex {
    terms: HashMap<String, Vec<EntityID>>,
}

impl InvertedIndex {
    pub fn insert(&mut self, term: &str, entity_id: EntityID) {
        self.terms
            .entry(term.to_lowercase())
            .or_default()
            .push(entity_id);
    }

    pub fn lookup(&self, term: &str) -> Vec<EntityID> {
        self.terms
            .get(&term.to_lowercase())
            .cloned()
            .unwrap_or_default()
    }
}
