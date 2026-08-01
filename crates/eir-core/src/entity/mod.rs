pub mod traits;
pub mod types;

use types::*;

use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityID,

    pub names: Vec<EntityName>,
    pub aliases: Vec<Alias>,

    pub tags: Vec<Tag>,
    pub properties: Vec<Property>,

    pub relationships: Vec<Relationship>,
}
