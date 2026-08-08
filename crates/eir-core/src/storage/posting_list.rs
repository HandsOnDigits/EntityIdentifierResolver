use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

use std::{collections::HashMap, hash::Hash};

use roaring::RoaringBitmap;

use crate::entity::prelude::types::EntityID;

#[derive(Debug, Clone)]
pub struct PostingList<K> {
    pub index: HashMap<K, RoaringBitmap>,
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
    K: Copy + Eq + Hash,
{
    pub fn insert(&mut self, key: K, entity: EntityID) {
        let entity =
            u32::try_from(entity.index()).expect("EntityID exceeds RoaringBitmap's u32 capacity");

        self.index.entry(key).or_default().insert(entity);
    }

    pub fn lookup(&self, key: K) -> Vec<EntityID> {
        self.index
            .get(&key)
            .map(|entities| {
                entities
                    .iter()
                    .map(|id| EntityID::new(id as usize))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &RoaringBitmap)> {
        self.index.iter()
    }

    pub fn to_record(&self) -> PostingListRecord<K> {
        let index = self
            .index
            .iter()
            .map(|(key, entities)| (*key, entities.iter().collect()))
            .collect();

        PostingListRecord { index }
    }

    pub fn from_record(record: PostingListRecord<K>) -> Self {
        let index = record
            .index
            .into_iter()
            .map(|(key, entities)| {
                let bitmap = entities.into_iter().collect();
                (key, bitmap)
            })
            .collect();

        Self { index }
    }
}

#[derive(Archive, Serialize, Deserialize, CheckBytes, Debug, Clone)]
pub struct PostingListRecord<K> {
    pub index: Vec<(K, Vec<u32>)>,
}

impl<K> Default for PostingListRecord<K> {
    fn default() -> Self {
        Self { index: Vec::new() }
    }
}
