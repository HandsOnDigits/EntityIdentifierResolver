use bytecheck::CheckBytes;

use rkyv::{Archive, Deserialize, Serialize};

use std::collections::HashMap;

use crate::{
    entity::types::{Alias, EntityID},
    utils::normalize,
};

#[derive(Default, Debug, Clone)]
pub struct AliasIndex {
    index: HashMap<Box<str>, Vec<EntityID>>,
}

impl AliasIndex {
    pub fn insert(&mut self, alias: impl Into<Box<str>>, entity_id: EntityID) {
        let key = normalize(&alias.into());

        let ids = self.index.entry(key).or_default();

        if !ids.contains(&entity_id) {
            ids.push(entity_id);
        }
    }

    pub fn resolve(&self, alias: &str) -> Option<&[EntityID]> {
        self.index.get(&normalize(alias)).map(|ids| ids.as_slice())
    }
}

#[derive(Archive, Serialize, Deserialize, CheckBytes, Debug)]
pub struct AliasIndexRecord {
    pub entries: HashMap<Alias, Vec<EntityID>>,
}

impl AliasIndex {
    pub fn to_record(&self) -> AliasIndexRecord {
        AliasIndexRecord {
            entries: self.index.clone(),
        }
    }
}
