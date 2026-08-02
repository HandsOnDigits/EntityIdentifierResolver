use std::collections::HashMap;

use crate::entity::types::{EntityID, PropertyID};

#[derive(Default)]
pub struct PropertyIndex {
    index: HashMap<PropertyID, Vec<EntityID>>,
}

impl PropertyIndex {
    pub fn insert(&mut self, property: PropertyID, entity: EntityID) {
        self.index.entry(property).or_default().push(entity);
    }

    pub fn lookup(&self, property: PropertyID) -> Vec<EntityID> {
        self.index.get(&property).cloned().unwrap_or_default()
    }
}
