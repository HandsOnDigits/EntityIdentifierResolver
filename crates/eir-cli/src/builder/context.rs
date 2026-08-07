use eir_core::{entity::prelude::types::*, storage::Registry};

pub struct BuilderContext {
    pub tags: Registry<TagID>,

    pub sources: Registry<SourceID>,

    pub attribute_keys: Registry<AttributeKeyID>,

    pub relationship_types: Registry<RelationshipTypeID>,
}

impl BuilderContext {
    pub fn new() -> Self {
        Self {
            tags: Registry::default(),

            sources: Registry::default(),

            attribute_keys: Registry::default(),

            relationship_types: Registry::default(),
        }
    }
}
