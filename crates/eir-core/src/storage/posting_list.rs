use rkyv::{Archive, Deserialize, Serialize, bytecheck::CheckBytes};

use std::{collections::HashMap, hash::Hash};

use roaring::RoaringTreemap;

use crate::entity::prelude::types::EntityID;

#[derive(Debug, Clone)]
pub struct PostingList<K> {
    pub index: HashMap<K, RoaringTreemap>,
}

impl<K> Default for PostingList<K> {
    fn default() -> Self {
        Self {
            index: HashMap::new(),
        }
    }
}

impl<K> PostingList<K>
where
    K: Copy + Eq + Hash + Clone,
{
    pub fn from_record(archived: PostingListRecord<K>) -> Self {
        let index = archived
            .index
            .into_iter()
            .map(|(key, entities)| {
                let mut bitmap = RoaringTreemap::new();

                for entity in entities {
                    bitmap.insert(entity.into());
                }

                (key, bitmap)
            })
            .collect();

        Self { index }
    }

    pub fn to_record(&self) -> PostingListRecord<K> {
        let index = self
            .index
            .iter()
            .map(|(key, entities)| (*key, entities.iter().map(EntityID::from).collect()))
            .collect();

        PostingListRecord { index }
    }

    pub fn insert(&mut self, key: K, entity: EntityID) {
        self.index.entry(key).or_default().insert(entity.into());
    }

    pub fn lookup(&self, key: K) -> Vec<EntityID> {
        self.index
            .get(&key)
            .map(|entities| entities.iter().map(EntityID::from).collect())
            .unwrap_or_default()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &RoaringTreemap)> {
        self.index.iter()
    }
}

#[derive(Archive, Serialize, Deserialize, CheckBytes, Debug, Clone)]
pub struct PostingListRecord<K> {
    pub index: Vec<(K, Vec<EntityID>)>,
}

impl<K> Default for PostingListRecord<K> {
    fn default() -> Self {
        Self { index: Vec::new() }
    }
}
