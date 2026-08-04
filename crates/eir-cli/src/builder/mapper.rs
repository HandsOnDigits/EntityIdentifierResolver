use eir_core::prelude::*;

use super::BuilderContext;
use super::fixture::{FixtureEntity, FixtureSource};

fn map_source(source: FixtureSource, ctx: &mut BuilderContext) -> EntitySource {
    EntitySource {
        id: ctx.sources.intern(&source.provider),

        provider: source.provider,

        verified: source.verified,

        created: source.created,
        updated: source.updated,
    }
}

pub fn map(entity: FixtureEntity, ctx: &mut BuilderContext) -> EntityDocument {
    EntityDocument {
        id: entity.id,

        aliases: entity.names,

        tags: entity
            .tags
            .into_iter()
            .map(|tag| ctx.tags.intern(&tag))
            .collect(),

        properties: entity
            .properties
            .into_iter()
            .map(|property| ctx.properties.intern(&property.key))
            .collect(),

        relationships: entity.relationships,

        sources: entity
            .sources
            .into_iter()
            .map(|source| {
                let source = map_source(source, ctx);
                source.id
            })
            .collect(),
    }
}
