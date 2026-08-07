use crate::{
    Database,
    entity::EntityDocument,
    entity::prelude::types::{EntityID, SourceID, TagID},
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

pub fn fixture_database() -> Database {
    let mut database = Database::default();

    database.entities.push(EntityDocument {
        id: EntityID(1),
        aliases: vec!["Nestle".into()],
        tags: vec![TagID(1)],
        attributes: vec![],
        relationships: vec![],
        sources: vec![SourceID(1)],
    });

    database
}
