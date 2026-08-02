use anyhow::Result;

use crate::{database::GeneratedDatabase, loader::*, mapper::*, registry::*};

use eir_core::entity::types::*;
use eir_core::entity::*;

pub fn generate_database() -> Result<GeneratedDatabase> {
    let fixtures = load_entities(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures/entities.json"
    ))?;

    let mut registry = Registry::new();

    let mut relationships = Vec::new();

    let entities: Vec<Entity> = fixtures
        .into_iter()
        .map(|fixture| {
            let tags: Vec<Tag> = fixture.tags.iter().map(|t| registry.tag(t)).collect();

            let entity_relationships = fixture
                .relationships
                .into_iter()
                .map(|r| Relationship {
                    target: EntityID(r.target),
                    kind: map_relationship(&r.kind),
                })
                .collect::<Vec<_>>();

            relationships.extend(entity_relationships.clone());

            Entity {
                id: EntityID(fixture.id),

                entity_type: map_entity_type(&fixture.entity_type),

                names: vec![fixture.name.into_boxed_str()],

                aliases: fixture
                    .aliases
                    .into_iter()
                    .map(String::into_boxed_str)
                    .collect(),

                tags,

                properties: fixture
                    .properties
                    .into_iter()
                    .map(|p| Property {
                        key: registry.property(&p.key),
                        value: Value::String(p.value),
                    })
                    .collect(),

                relationships: entity_relationships,

                sources: vec![],
            }
        })
        .collect();

    let tags = registry.export_tags();
    let properties = registry.export_properties();

    Ok(GeneratedDatabase {
        entities,
        tags,
        properties,
        relationships,
    })
}
