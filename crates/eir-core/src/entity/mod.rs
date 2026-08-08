pub mod input;
pub mod loader;
mod macros;
mod types;

use rkyv::{Archive, Deserialize, Serialize, bytecheck::CheckBytes};

use types::*;

pub use macros::archived_id_index;

pub mod prelude {
    pub use crate::entity::input;

    pub mod types {
        pub use crate::entity::types::{
            Alias, Attribute, AttributeKeyID, Date, EntityID, EntityName, Relationship,
            RelationshipTypeID, SourceID, Tag, TagID, Value,
        };
    }

    pub use super::{Entity, EntityDocument, EntitySource};
}

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EntitySource {
    pub id: SourceID,

    pub provider: Box<str>,

    pub verified: bool,

    pub created: Option<Date>,
    pub updated: Option<Date>,
}

#[derive(Archive, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Entity {
    pub id: EntityID,

    pub names: Vec<EntityName>,

    pub tags: Vec<Tag>,

    pub attributes: Vec<Attribute>,

    pub relationships: Vec<Relationship>,

    pub sources: Vec<EntitySource>,
}

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq, Clone, CheckBytes)]
pub struct EntityDocument {
    pub id: EntityID,

    pub aliases: Vec<Alias>,

    pub tags: Vec<TagID>,

    pub attributes: Vec<Attribute>,

    pub relationships: Vec<Relationship>,

    pub sources: Vec<SourceID>,
}
