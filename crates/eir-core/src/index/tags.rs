use std::collections::HashMap;

use roaring::RoaringTreemap;

use crate::entity::types::{EntityID, TagID};

#[derive(Default)]
pub struct TagIndex {
    index: HashMap<TagID, RoaringTreemap>,
}

impl TagIndex {
    pub fn insert(&mut self, tag: TagID, entity: EntityID) {
        self.index.entry(tag).or_default().insert(entity);
    }

    pub fn lookup(&self, tag: TagID) -> Vec<EntityID> {
        self.index
            .get(&tag)
            .map(|x| x.iter().collect())
            .unwrap_or_default()
    }
}
