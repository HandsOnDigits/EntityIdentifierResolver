use rkyv::{Archive, Deserialize, Serialize};

use super::posting_list::PostingList;

use crate::entity::{
    EntityDocument,
    types::{EntityID, SourceID, TagID},
};

use std::hash::Hash;

#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct Database {
    pub entities: Vec<EntityDocument>,

    pub tags: ArchivedPostingList<TagID>,
    pub sources: ArchivedPostingList<SourceID>,
}

#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct ArchivedPostingList<K> {
    pub keys: Vec<K>,
    pub values: Vec<Vec<EntityID>>,
}

impl<K> PostingList<K>
where
    K: Copy + Eq + Hash,
{
    pub fn to_archived(&self) -> ArchivedPostingList<K> {
        let mut keys = Vec::new();
        let mut values = Vec::new();

        for (key, entities) in self.iter() {
            keys.push(*key);
            values.push(entities.iter().collect());
        }

        ArchivedPostingList { keys, values }
    }
}

impl<K> PostingList<K>
where
    K: Copy + Eq + Hash,
{
    pub fn archive(&self) -> ArchivedPostingList<K> {
        let mut keys = Vec::new();
        let mut values = Vec::new();

        for (key, entities) in self.iter() {
            keys.push(*key);
            values.push(entities.iter().collect());
        }

        ArchivedPostingList { keys, values }
    }
}
