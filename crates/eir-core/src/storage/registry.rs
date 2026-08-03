use std::collections::HashMap;

pub trait RegistryID: Copy {
    fn from_index(index: usize) -> Self;
    fn index(self) -> usize;
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
    values: Vec<String>,
    lookup: HashMap<String, ID>,
}

impl<ID: RegistryID> Registry<ID> {
    pub fn intern(&mut self, value: &str) -> ID {
        if let Some(id) = self.lookup.get(value) {
            return *id;
        }

        let id = ID::from_index(self.values.len());

        self.values.push(value.to_string());
        self.lookup.insert(value.to_string(), id);

        id
    }

    pub fn get(&self, id: ID) -> Option<&str> {
        self.values.get(id.index()).map(String::as_str)
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}
