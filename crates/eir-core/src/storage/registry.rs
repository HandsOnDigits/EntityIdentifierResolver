use crate::utils::normalize;

use std::{collections::HashMap, ops::Index};

pub trait RegistryID: Copy {
    fn from_index(index: usize) -> Self;
    fn index(self) -> usize;
}

impl RegistryID for u16 {
    fn from_index(index: usize) -> Self {
        index as u16
    }

    fn index(self) -> usize {
        self as usize
    }
}

impl RegistryID for u32 {
    fn from_index(index: usize) -> Self {
        index as u32
    }

    fn index(self) -> usize {
        self as usize
    }
}

impl RegistryID for u64 {
    fn from_index(index: usize) -> Self {
        index as u64
    }

    fn index(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Default)]
pub struct Registry<ID: RegistryID> {
    values: Vec<Box<str>>,
    lookup: HashMap<Box<str>, ID>,
}

impl<ID: RegistryID> Registry<ID> {
    pub fn intern(&mut self, value: &str) -> ID {
        let value = normalize(value);

        if let Some(id) = self.lookup.get(value.as_ref()) {
            return *id;
        }

        let id = ID::from_index(self.values.len());

        self.values.push(value.clone());
        self.lookup.insert(value, id);

        id
    }

    pub fn get(&self, id: ID) -> Option<&str> {
        self.values.get(id.index()).map(Box::as_ref)
    }

    pub fn contains(&self, value: &str) -> bool {
        self.lookup.contains_key(value)
    }

    pub fn id(&self, value: &str) -> Option<ID> {
        self.lookup.get(value).copied()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.values.iter().map(Box::as_ref)
    }

    pub fn into_inner(self) -> Vec<Box<str>> {
        self.values
    }
}

impl<ID: RegistryID> Index<ID> for Registry<ID> {
    type Output = str;

    fn index(&self, id: ID) -> &Self::Output {
        &self.values[id.index()]
    }
}
