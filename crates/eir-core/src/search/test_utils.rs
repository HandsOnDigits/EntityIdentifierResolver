use crate::{
    entity::EntityDocument,
    entity::prelude::types::{EntityID, TagID},
};

pub fn test_entity(id: EntityID, alias: &str) -> EntityDocument {
    EntityDocument {
        id,
        aliases: vec![alias.into()],
        attributes: Vec::new(),
        relationships: Vec::new(),
        sources: Vec::new(),
        tags: Vec::new(),
    }
}

pub fn test_entity_with_tag(id: EntityID, alias: &str, tag: TagID) -> EntityDocument {
    EntityDocument {
        id,
        aliases: vec![alias.into()],
        attributes: Vec::new(),
        relationships: Vec::new(),
        sources: Vec::new(),
        tags: vec![tag],
    }
}
