use std::{collections::HashMap, hash::Hash};

use roaring::RoaringTreemap;

use crate::entity::types::EntityID;

#[derive(Default)]
pub struct PostingList<K> {
    index: HashMap<K, RoaringTreemap>,
}

impl<K> PostingList<K>
where
    K: Copy + Eq + Hash,
{
    pub fn insert(&mut self, key: K, entity: EntityID) {
        self.index.entry(key).or_default().insert(entity);
    }

    pub fn lookup(&self, key: K) -> Vec<EntityID> {
        self.index
            .get(&key)
            .map(|entities| entities.iter().collect())
            .unwrap_or_default()
    }
}
