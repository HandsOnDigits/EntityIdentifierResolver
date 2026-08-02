use eir_core::entity::types::Tag;
use std::collections::HashMap;

pub struct Registry {
    next_tag: u32,
    tags: HashMap<String, Tag>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            next_tag: 1,
            tags: HashMap::new(),
        }
    }

    pub fn tag(&mut self, name: &str) -> Tag {
        if let Some(tag) = self.tags.get(name) {
            return *tag;
        }

        let id = self.next_tag;
        self.next_tag += 1;

        self.tags.insert(name.to_owned(), id);

        id
    }
}
