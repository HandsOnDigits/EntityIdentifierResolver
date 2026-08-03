use eir_core::{
    entity::types::{EntityID, PropertyID, SourceID, TagID},
    storage::{Registry, Store, indexes::Indexes},
};

use super::fixture::FixtureSource;

pub struct GeneratorContext {
    pub store: Store,

    pub indexes: Indexes,

    pub entities: Registry<EntityID>,

    pub tags: Registry<TagID>,
    pub sources: Registry<SourceID>,

    pub properties: Registry<PropertyID>,
}

impl GeneratorContext {
    pub fn new() -> Self {
        Self {
            store: Store::new(),

            indexes: Indexes::default(),

            entities: Registry::default(),

            tags: Registry::default(),
            sources: Registry::default(),
            properties: Registry::default(),
        }
    }

    pub fn intern_source(&mut self, source: FixtureSource) -> SourceID {
        self.sources.intern(&source.provider)
    }
}
