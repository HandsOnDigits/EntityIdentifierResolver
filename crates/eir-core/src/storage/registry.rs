use std::collections::HashMap;

pub trait RegistryID: Copy + Eq + std::hash::Hash {
    fn from_u32(value: u32) -> Self;
    fn as_u32(self) -> u32;
}

#[derive(Default)]
pub struct Registry<ID: RegistryID> {
    ids: HashMap<String, ID>,
    values: Vec<String>,
}

impl<ID: RegistryID> Registry<ID> {
    pub fn intern(&mut self, value: &str) -> ID {
        let value = value.to_lowercase();

        if let Some(id) = self.ids.get(&value) {
            return *id;
        }

        let id = ID::from_u32(self.values.len() as u32);

        self.ids.insert(value.clone(), id);
        self.values.push(value);

        id
    }

    pub fn get(&self, id: ID) -> Option<&str> {
        self.values.get(id.as_u32() as usize).map(|x| x.as_str())
    }
}
