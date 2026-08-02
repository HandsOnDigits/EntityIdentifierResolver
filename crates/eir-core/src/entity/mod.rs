pub mod traits;
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

    pub names: Vec<EntityName>,
    pub aliases: Vec<Alias>,

    pub tags: Vec<Tag>,
    pub properties: Vec<Property>,

    pub relationships: Vec<Relationship>,
    pub sources: Vec<EntitySource>,
}
