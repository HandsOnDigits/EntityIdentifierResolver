use eir_core::prelude::*;

pub struct BuilderContext {
    pub tags: Registry<TagID>,
    pub sources: Registry<SourceID>,
    pub properties: Registry<PropertyID>,
}

impl BuilderContext {
    pub fn new() -> Self {
        Self {
            tags: Registry::default(),
            sources: Registry::default(),
            properties: Registry::default(),
        }
    }
}
