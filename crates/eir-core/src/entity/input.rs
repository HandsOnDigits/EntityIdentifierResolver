use serde::Deserialize;

use super::types::Alias;

#[derive(Debug, Clone, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct EntityInput {
    pub id: usize,

    pub aliases: Vec<Alias>,

    pub tags: Vec<Alias>,

    pub properties: Vec<PropertyInput>,

    pub relationships: Vec<RelationshipInput>,

    pub sources: Vec<SourceInput>,
}

#[derive(Debug, Clone, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct PropertyInput {
    pub key: Alias,
    pub value: Alias,
}

#[derive(Debug, Clone, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct RelationshipInput {
    pub target: usize,
    pub kind: Alias,
}

#[derive(Debug, Clone, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct SourceInput {
    pub provider: Alias,
    pub verified: bool,
}
