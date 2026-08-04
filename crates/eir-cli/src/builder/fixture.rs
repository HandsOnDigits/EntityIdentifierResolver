use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use eir_core::prelude::*;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct FixtureProperty {
    pub key: String,
    pub value: String,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct FixtureEntity {
    pub id: EntityID,

    pub names: Vec<EntityName>,

    pub tags: Vec<Box<str>>,

    pub properties: Vec<FixtureProperty>,

    pub relationships: Vec<Relationship>,

    pub sources: Vec<FixtureSource>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct FixtureSource {
    pub provider: Box<str>,

    pub verified: bool,

    pub created: Option<Date>,

    pub updated: Option<Date>,
}
