use std::collections::HashMap;

use crate::{entity::types::EntityID, index::utils::normalize};

#[derive(Default, Debug)]
pub struct AliasIndex {
    aliases: HashMap<String, Vec<EntityID>>,
}

impl AliasIndex {
    pub fn insert(&mut self, alias: impl Into<String>, entity_id: EntityID) {
        let key = normalize(&alias.into());

        let ids = self.aliases.entry(key).or_default();

        if !ids.contains(&entity_id) {
            ids.push(entity_id);
        }
    }

    pub fn resolve(&self, alias: &str) -> Option<&[EntityID]> {
        self.aliases
            .get(&normalize(alias))
            .map(|ids| ids.as_slice())
    }
}
