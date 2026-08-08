use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

use std::collections::HashMap;

use crate::{storage::ids::RegistryID, utils::normalize};

#[derive(Debug, Clone)]
pub struct Registry<ID: RegistryID> {
    values: Vec<Option<Box<str>>>,
    lookup: HashMap<Box<str>, ID>,
    free_list: Vec<ID>,
}

impl<ID: RegistryID> Registry<ID> {
    pub fn intern(&mut self, value: &str) -> ID {
        let value = normalize(value);

        if let Some(&id) = self.lookup.get(value.as_ref()) {
            return id;
        }

        let id = if let Some(id) = self.free_list.pop() {
            self.values[id.index()] = Some(value.clone());
            id
        } else {
            let id = ID::new(self.values.len());

            self.values.push(Some(value.clone()));

            id
        };

        self.lookup.insert(value, id);

        id
    }

    pub fn remove(&mut self, id: ID) -> Option<Box<str>> {
        let value = self.values.get_mut(id.index())?.take()?;

        self.lookup.remove(value.as_ref());
        self.free_list.push(id);

        Some(value)
    }

    /// Returns an iterator over all active (non-deleted) string values in the registry.
    pub fn values(&self) -> impl Iterator<Item = &Box<str>> {
        self.values.iter().filter_map(|v| v.as_ref())
    }

    pub fn get(&self, id: ID) -> Option<&str> {
        self.values.get(id.index()).and_then(Option::as_deref)
    }

    pub fn id(&self, value: &str) -> Option<ID> {
        self.lookup.get(value).copied()
    }

    pub fn contains(&self, value: &str) -> bool {
        self.lookup.contains_key(value)
    }

    pub fn iter(&self) -> impl Iterator<Item = (ID, &str)> {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(index, value)| value.as_deref().map(|value| (ID::new(index), value)))
    }

    pub fn len(&self) -> usize {
        self.values.iter().filter(|value| value.is_some()).count()
    }

    pub fn slot_count(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn to_record(&self) -> RegistryRecord {
        RegistryRecord {
            values: self.values.clone(),
        }
    }

    pub fn from_record(record: RegistryRecord) -> Self {
        let mut lookup = HashMap::new();
        let mut free_list = Vec::new();

        for (index, value) in record.values.iter().enumerate() {
            let id = ID::new(index);

            match value {
                Some(value) => {
                    lookup.insert(value.clone(), id);
                }
                None => {
                    free_list.push(id);
                }
            }
        }

        Self {
            values: record.values,
            lookup,
            free_list,
        }
    }
}

impl<ID: RegistryID> Default for Registry<ID> {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            lookup: HashMap::new(),
            free_list: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Archive, Serialize, Deserialize, CheckBytes)]
pub struct RegistryRecord {
    pub values: Vec<Option<Box<str>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::storage::ids::TagID;

    #[test]
    fn registry_preserves_ids_after_record_roundtrip() {
        let mut registry = Registry::<TagID>::default();

        let apple = registry.intern("Apple");
        let banana = registry.intern("Banana");

        let record = registry.to_record();
        let restored = Registry::<TagID>::from_record(record);

        assert_eq!(restored.id("apple"), Some(apple));
        assert_eq!(restored.id("banana"), Some(banana));
    }

    #[test]
    fn registry_reuses_freed_id_after_roundtrip() {
        let mut registry = Registry::<TagID>::default();

        let apple = registry.intern("Apple");
        let banana = registry.intern("Banana");

        registry.remove(apple);

        let record = registry.to_record();
        let mut restored = Registry::<TagID>::from_record(record);

        let cherry = restored.intern("Cherry");

        assert_eq!(cherry, apple);
        assert_eq!(restored.id("banana"), Some(banana));
    }
}
