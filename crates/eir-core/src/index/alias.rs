use std::collections::HashMap;

use crate::entity::types::EntityID;

#[derive(Default)]
pub struct AliasIndex {
    aliases: HashMap<String, EntityID>,
}

impl AliasIndex {
    pub fn insert(&mut self, alias: impl Into<String>, entity_id: EntityID) {
        self.aliases.insert(alias.into().to_lowercase(), entity_id);
    }

    pub fn resolve(&self, alias: &str) -> Option<EntityID> {
        self.aliases.get(&alias.to_lowercase()).copied()
    }
}
