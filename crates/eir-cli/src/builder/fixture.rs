use rkyv::{Archive, Deserialize, Serialize};

use eir_core::prelude::*;

#[derive(Archive, Serialize, Deserialize, Debug, Clone)]
pub struct FixtureEntity {
    pub id: EntityID,

    pub entity_type: [u8; 4],

    pub names: Vec<Box<str>>,

    pub tags: Vec<Box<str>>,

    pub properties: Vec<Box<str>>,

    pub sources: Vec<FixtureSource>,
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone)]
pub struct FixtureSource {
    pub provider: Box<str>,

    pub verified: bool,

    pub created: Date,

    pub updated: Date,
}
