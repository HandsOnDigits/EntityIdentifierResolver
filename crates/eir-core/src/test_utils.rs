use crate::{
    Database,
    entity::EntityDocument,
    entity::prelude::types::{Attribute, AttributeKeyID, EntityID, SourceID, TagID, Value},
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

pub fn test_entity_with_attribute(
    id: EntityID,
    name: &str,
    key: AttributeKeyID,
    value: Value,
) -> EntityDocument {
    EntityDocument {
        id,
        aliases: vec![name.into()],
        tags: vec![],
        attributes: vec![Attribute { key, value }],
        relationships: vec![],
        sources: vec![],
    }
}

pub fn fixture_database() -> Database {
    let mut database = Database::default();

    database.entities.push(EntityDocument {
        id: EntityID::new(1),
        aliases: vec!["Nestle".into()],
        tags: vec![TagID::new(1)],
        attributes: vec![],
        relationships: vec![],
        sources: vec![SourceID::new(1)],
    });

    database
}

pub fn test_entity_with_attributes(
    id: EntityID,
    alias: &str,
    attributes: Vec<(AttributeKeyID, Value)>,
) -> EntityDocument {
    EntityDocument {
        id,
        aliases: vec![alias.into()],
        tags: vec![],
        attributes: attributes
            .into_iter()
            .map(|(key, value)| Attribute { key, value })
            .collect(),
        relationships: vec![],
        sources: vec![],
    }
}
