use anyhow::Result;

use crate::{loader::*, registry::*};

use eir_core::entity::types::*;

use eir_core::entity::*;

pub fn generate_entities() -> Result<Vec<Entity>> {
    let products = load_products(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures/products.json"
    ))?;

    let mut registry = Registry::new();

    let entities = products
        .into_iter()
        .map(|p| {
            let tags = p.tags.iter().map(|t| registry.tag(t)).collect();

            Entity {
                id: EntityID(p.id),

                names: vec![p.name.into_boxed_str()],

                aliases: p.aliases.into_iter().map(String::into_boxed_str).collect(),

                tags,

                properties: vec![],

                relationships: vec![
                    Relationship {
                        target: EntityID(p.company),
                        kind: RelationshipType::MadeBy,
                    },
                    Relationship {
                        target: EntityID(p.category),
                        kind: RelationshipType::IsA,
                    },
                    Relationship {
                        target: EntityID(p.country),
                        kind: RelationshipType::LocatedIn,
                    },
                ],

                sources: vec![EntitySource {
                    external_id: format!("product:{}", p.id),
                    provider: "fixture".into(),
                    verified: true,
                    created: Date::now(),
                    updated: Date::now(),
                }],
            }
        })
        .collect();

    Ok(entities)
}
