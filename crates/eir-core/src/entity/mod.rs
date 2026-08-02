pub mod types;

use types::*;

use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EntitySource {
    pub id: SourceID,

    pub provider: String,

    pub verified: bool,

    pub created: Date,

    pub updated: Date,
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Entity {
    pub id: EntityID,

    pub entity_type: EntityType,

    pub names: Vec<EntityName>,

    pub tags: Vec<Tag>,

    pub properties: Vec<Property>,

    pub relationships: Vec<Relationship>,

    pub sources: Vec<EntitySource>,
}

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EntityDocument {
    pub id: EntityID,

    pub entity_type: EntityType,

    pub sources: Vec<SourceID>,

    pub properties: Vec<PropertyID>,
}

pub struct EntityInput {
    pub document: EntityDocument,

    pub aliases: Vec<EntityName>,

    pub tags: Vec<TagID>,
}
