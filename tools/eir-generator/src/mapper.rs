use eir_core::entity::{
    EntityDocument, EntityInput, EntitySource,
    types::{PropertyID, TagID},
};

use eir_utils::registry::Registry;

use eir_utils::fixture::{FixtureEntity, FixtureSource};

fn map_source(source: FixtureSource, registry: &mut Registry) -> EntitySource {
    EntitySource {
        id: registry.sources.intern(&source.provider),

        provider: source.provider.to_string(),

        verified: source.verified,

        created: source.created,
        updated: source.updated,
    }
}

pub fn map(entity: FixtureEntity, registry: &mut Registry) -> EntityInput {
    EntityInput {
        document: EntityDocument {
            id: entity.id,
            entity_type: entity.entity_type,

            sources: entity
                .sources
                .into_iter()
                .map(|source| registry.intern_source(source))
                .collect(),

            properties: entity
                .properties
                .into_iter()
                .map(|property| registry.properties.intern(&property))
                .collect::<Vec<PropertyID>>(),
        },

        aliases: entity.names,

        tags: entity
            .tags
            .into_iter()
            .map(|tag| registry.tags.intern(&tag))
            .collect::<Vec<TagID>>(),
    }
}
