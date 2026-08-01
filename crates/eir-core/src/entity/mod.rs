pub mod types;

use types::*;

pub struct Entity {
    pub id: EntityID,
    pub entity_type: EntityType,

    pub aliases: Vec<Alias>,
    pub tags: Vec<Tag>,
    pub properties: Vec<Property>,
    pub relationships: Vec<Relationship>,
}
