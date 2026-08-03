use rkyv::{Archive, Deserialize, Serialize};

use crate::entity::{
    EntityDocument, EntitySource,
    types::{EntityID, EntityName, Property, TagID},
};

use std::collections::HashMap;

#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct Database {
    pub entities: Vec<EntityDocument>,

    // entity_id -> aliases
    pub aliases: HashMap<EntityID, Vec<EntityName>>,

    // tag_id -> entities
    pub tags: HashMap<TagID, Vec<EntityID>>,

    pub sources: Vec<EntitySource>,

    pub properties: Vec<Property>,
}
