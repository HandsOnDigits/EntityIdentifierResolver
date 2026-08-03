use eir_core::entity::{Entity, types::*};
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct DatabaseMetadata {
    pub version: u32,

    pub created: Date,

    pub entity_count: u64,

    pub tag_count: u32,

    pub property_count: u32,

    pub relationship_count: u64,
}

pub struct GeneratedDatabase {
    pub entities: Vec<Entity>,

    pub tags: Vec<(Tag, String)>,

    pub properties: Vec<(PropertyID, String)>,

    pub relationships: Vec<Relationship>,
}
