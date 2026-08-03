use rkyv::{Archive, Deserialize, Serialize};

use std::{collections::HashMap, hash::Hash};

use roaring::RoaringTreemap;

use crate::entity::types::EntityID;

#[derive(Debug, Default, Clone)]
pub struct PostingList<K> {
    pub index: HashMap<K, RoaringTreemap>,
}

impl<K> PostingList<K>
where
    K: Copy + Eq + Hash,
{
    pub fn from_archive(archived: PostingListRecord<K>) -> Self {
        let index = archived
            .index
            .into_iter()
            .map(|(key, entities)| {
                let mut bitmap = RoaringTreemap::new();

                for entity in entities {
                    bitmap.insert(entity);
                }

                (key, bitmap)
            })
            .collect();

        Self { index }
    }

    pub fn to_archive(&self) -> PostingListRecord<K> {
        let index = self
            .index
            .iter()
            .map(|(key, entities)| (*key, entities.iter().collect()))
            .collect();

        PostingListRecord { index }
    }

    pub fn insert(&mut self, key: K, entity: EntityID) {
        self.index.entry(key).or_default().insert(entity);
    }

    pub fn lookup(&self, key: K) -> Vec<EntityID> {
        self.index
            .get(&key)
            .map(|entities| entities.iter().collect())
            .unwrap_or_default()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &RoaringTreemap)> {
        self.index.iter()
    }
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone, Default)]
pub struct PostingListRecord<K> {
    pub index: HashMap<K, Vec<EntityID>>,
}
