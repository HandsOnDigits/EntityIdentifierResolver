use eir_core::prelude::*;

pub struct BuilderContext {
    pub tags: Registry<TagID>,

    pub sources: Registry<SourceID>,

    pub attribute_keys: Registry<AttributeKeyID>,
}

impl BuilderContext {
    pub fn new() -> Self {
        Self {
            tags: Registry::default(),

            sources: Registry::default(),

            attribute_keys: Registry::default(),
        }
    }
}
