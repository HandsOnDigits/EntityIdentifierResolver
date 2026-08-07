use eir_core::entity::prelude::{
    EntityDocument, EntitySource,
    types::{Attribute, Value},
};

use super::BuilderContext;
use super::fixture::{FixtureEntity, FixtureSource};

fn map_source(source: FixtureSource, ctx: &mut BuilderContext) -> EntitySource {
    let id = ctx.sources.intern(&source.provider);

    EntitySource {
        id,
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

        attributes: entity
            .attributes
            .iter()
            .map(|attribute| Attribute {
                key: ctx.attribute_keys.intern(&attribute.key),
                value: Value::String(attribute.value.clone().into_boxed_str()),
            })
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
