use anyhow::Result;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct EntityFixture {
    pub id: u64,

    pub entity_type: String,

    pub name: String,

    #[serde(default)]
    pub aliases: Vec<String>,

    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(default)]
    pub properties: Vec<PropertyFixture>,

    #[serde(default)]
    pub relationships: Vec<RelationshipFixture>,
}

#[derive(Debug, Deserialize)]
pub struct RelationshipFixture {
    pub target: u64,
    pub kind: String,
}

#[derive(Debug, Deserialize)]
pub struct PropertyFixture {
    pub key: String,
    pub value: String,
}

pub fn load_entities(path: &str) -> Result<Vec<EntityFixture>> {
    let data = fs::read_to_string(path)?;

    Ok(serde_json::from_str(&data)?)
}
