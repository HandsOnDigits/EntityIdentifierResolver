pub mod types;

use types::*;

use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct EntitySource {
    pub external_id: String,

    pub provider: EntityName,

    pub verified: bool,

    pub created: Date,
    pub updated: Date,
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Entity {
    pub id: EntityID,

    pub entity_type: EntityType,

    pub names: Vec<EntityName>,
    pub aliases: Vec<Alias>,

    pub tags: Vec<Tag>,
    pub properties: Vec<Property>,

    pub relationships: Vec<Relationship>,
    pub sources: Vec<EntitySource>,
}

#[derive(Debug, Clone)]
pub struct EntityDocument {
    pub id: EntityID,

    pub aliases: Vec<String>,
    pub tags: Vec<String>,
    pub properties: Vec<String>,

    pub sources: Vec<EntitySource>,
    pub relationships: Vec<Relationship>,
}
