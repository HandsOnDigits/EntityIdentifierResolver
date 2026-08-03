use eir_core::prelude::*;

use super::GeneratorContext;
use super::fixture::{FixtureEntity, FixtureSource};

fn map_source(source: FixtureSource, ctx: &mut GeneratorContext) -> EntitySource {
    EntitySource {
        id: ctx.sources.intern(&source.provider),

        provider: source.provider,

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
                .map(|source| {
                    let source = map_source(source, ctx);
                    source.id
                })
                .collect(),

            properties: entity
                .properties
                .into_iter()
                .map(|property| ctx.properties.intern(&property))
                .collect(),
        },

        aliases: entity.names,

        tags: entity
            .tags
            .into_iter()
            .map(|tag| ctx.tags.intern(&tag))
            .collect(),
    }
}
