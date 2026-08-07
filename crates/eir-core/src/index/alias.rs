use bytecheck::CheckBytes;

use rkyv::{Archive, Deserialize, Serialize};

use std::collections::HashMap;

use crate::{
    entity::prelude::types::{Alias, EntityID},
    utils::normalize,
};

#[derive(Default, Debug, Clone)]
pub struct AliasIndex {
    entries: HashMap<Box<str>, Vec<EntityID>>,
}

impl AliasIndex {
    pub fn insert(&mut self, alias: impl Into<Box<str>>, entity_id: EntityID) {
        let key = normalize(&alias.into());

        let ids = self.entries.entry(key).or_default();

        if !ids.contains(&entity_id) {
            ids.push(entity_id);
        }
    }

    pub fn resolve(&self, alias: &str) -> Option<&[EntityID]> {
        self.entries
            .get(&normalize(alias))
            .map(|ids| ids.as_slice())
    }
}

#[derive(Archive, Serialize, Deserialize, CheckBytes, Debug, Clone)]
pub struct AliasIndexRecord {
    pub entries: HashMap<Alias, Vec<EntityID>>,
}

impl AliasIndex {
    pub fn to_record(&self) -> AliasIndexRecord {
        AliasIndexRecord {
            entries: self.entries.clone(),
        }
    }

    pub fn from_record(record: AliasIndexRecord) -> Self {
        Self {
            entries: record.entries,
        }
    }
}
