use eir_core::entity::types::{PropertyID, Tag};
use std::collections::HashMap;

pub struct Registry {
    next_tag: Tag,
    tags: HashMap<String, Tag>,

    next_property: PropertyID,
    properties: HashMap<String, PropertyID>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            next_tag: 1,
            tags: HashMap::new(),

            next_property: 1,
            properties: HashMap::new(),
        }
    }

    pub fn tag(&mut self, value: &str) -> Tag {
        if let Some(id) = self.tags.get(value) {
            return *id;
        }

        let id = self.next_tag;

        self.next_tag += 1;

        self.tags.insert(value.to_string(), id);

        id
    }

    pub fn property(&mut self, value: &str) -> PropertyID {
        if let Some(id) = self.properties.get(value) {
            return *id;
        }

        let id = self.next_property;

        self.next_property += 1;

        self.properties.insert(value.to_string(), id);

        id
    }

    pub fn export_tags(&self) -> Vec<(Tag, String)> {
        let mut tags = self
            .tags
            .iter()
            .map(|(name, id)| (*id, name.clone()))
            .collect::<Vec<_>>();

        tags.sort_by_key(|(id, _)| *id);

        tags
    }

    pub fn export_properties(&self) -> Vec<(PropertyID, String)> {
        let mut properties = self
            .properties
            .iter()
            .map(|(name, id)| (*id, name.clone()))
            .collect::<Vec<_>>();

        properties.sort_by_key(|(id, _)| *id);

        properties
    }
}
