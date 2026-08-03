use eir_core::entity::{
    EntityDocument, EntityInput, EntitySource,
    types::{PropertyID, TagID},
};

use super::GeneratorContext;

use super::fixture::{FixtureEntity, FixtureSource};

fn map_source(source: FixtureSource, ctx: &mut GeneratorContext) -> EntitySource {
    EntitySource {
        id: ctx.sources.intern(&source.provider),

        provider: source.provider.to_string(),

        verified: source.verified,

        created: source.created,
        updated: source.updated,
    }
}

pub fn map(entity: FixtureEntity, ctx: &mut GeneratorContext) -> EntityInput {
    EntityInput {
        document: EntityDocument {
            id: entity.id,
            entity_type: entity.entity_type,

            sources: entity
                .sources
                .into_iter()
                .map(|source| ctx.intern_source(source))
                .collect(),

            properties: entity
                .properties
                .into_iter()
                .map(|property| ctx.properties.intern(&property))
                .collect::<Vec<PropertyID>>(),
        },

        aliases: entity.names,

        tags: entity
            .tags
            .into_iter()
            .map(|tag| ctx.tags.intern(&tag))
            .collect::<Vec<TagID>>(),
    }
}
