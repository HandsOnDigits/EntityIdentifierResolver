use eir_core::{
    entity::prelude::{
        EntityDocument, EntitySource,
        types::{Attribute, Relationship, Value},
    },
    utils::normalize,
};

use super::{
    BuilderContext,
    fixture::{FixtureEntity, FixtureSource},
};

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

        tags: entity.tags.iter().map(|tag| ctx.tags.intern(tag)).collect(),

        attributes: entity
            .attributes
            .iter()
            .map(|attribute| Attribute {
                key: ctx.attribute_keys.intern(&attribute.key),
                value: Value::String(attribute.value.clone().into_boxed_str()),
            })
            .collect(),

        relationships: entity
            .relationships
            .into_iter()
            .map(|relationship| Relationship {
                target: relationship.target,
                kind: eir_core::entity::prelude::types::RelationshipType::Custom(
                    ctx.relationship_types
                        .intern(&normalize(&relationship.kind)),
                ),
            })
            .collect(),

        sources: entity
            .sources
            .into_iter()
            .map(|source| map_source(source, ctx).id)
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Result;

    use eir_core::entity::prelude::types::EntityID;

    use crate::builder::fixture::FixtureEntity;

    #[test]
    fn mapper_interns_tags_and_sources() -> Result<()> {
        let mut ctx = BuilderContext::new();

        let entity = FixtureEntity {
            id: EntityID::new(9100),
            names: vec!["Test Berry".into()],
            tags: vec!["fruit".into(), "berry".into()],
            sources: vec![FixtureSource {
                provider: "Test Source".into(),
                verified: false,
                created: None,
                updated: None,
            }],
            attributes: vec![],
            relationships: vec![],
        };

        let mapped = map(entity, &mut ctx);

        assert_eq!(mapped.id, EntityID::new(9100));
        assert_eq!(mapped.aliases, ["Test Berry".into()]);

        assert_eq!(mapped.tags.len(), 2);
        assert_eq!(ctx.tags.get(mapped.tags[0]), Some("fruit"));
        assert_eq!(ctx.tags.get(mapped.tags[1]), Some("berry"));

        assert_eq!(mapped.sources.len(), 1);
        assert_eq!(ctx.sources.get(mapped.sources[0]), Some("test source"));

        Ok(())
    }
}
