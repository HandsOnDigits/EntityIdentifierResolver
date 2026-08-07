use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

use eir_core::entity::prelude::types::{Date, EntityID, EntityName, Relationship};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct FixtureAttribute {
    pub key: String,
    pub value: String,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct FixtureEntity {
    pub id: EntityID,

    pub names: Vec<EntityName>,

    #[serde(default)]
    pub tags: Vec<Box<str>>,

    #[serde(default)]
    pub attributes: Vec<FixtureAttribute>,

    #[serde(default)]
    pub relationships: Vec<Relationship>,

    #[serde(default)]
    pub sources: Vec<FixtureSource>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Serialize, Deserialize, Debug, Clone)]
pub struct FixtureSource {
    pub provider: Box<str>,

    pub verified: bool,

    pub created: Option<Date>,

    pub updated: Option<Date>,
}
