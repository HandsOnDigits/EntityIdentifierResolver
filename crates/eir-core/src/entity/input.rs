use serde::Deserialize;

use super::types::Alias;

#[derive(Debug, Deserialize)]
pub struct EntityInput {
    pub id: u64,

    #[serde(rename = "names")]
    pub aliases: Vec<Alias>,

    pub tags: Vec<Alias>,

    pub properties: Vec<PropertyInput>,

    pub relationships: Vec<RelationshipInput>,

    pub sources: Vec<SourceInput>,
}

#[derive(Debug, Deserialize)]
pub struct PropertyInput {
    pub key: Alias,
    pub value: Alias,
}

#[derive(Debug, Deserialize)]
pub struct RelationshipInput {
    pub target: u64,
    pub kind: Alias,
}

#[derive(Debug, Deserialize)]
pub struct SourceInput {
    pub provider: Alias,
    pub verified: bool,
}
