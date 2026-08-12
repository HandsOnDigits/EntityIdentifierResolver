use eir_core::{
    engine::{Database, Engine},
    entity::prelude::input::{AttributesInput, EntityInput, RelationshipInput, SourceInput},
};

use crate::{MergeReport, Result};

fn entity_to_input(
    database: &Database,
    entity: &eir_core::entity::prelude::EntityDocument,
) -> EntityInput {
    EntityInput {
        id: entity.id,

        aliases: entity.aliases.clone(),

        tags: entity
            .tags
            .iter()
            .filter_map(|id| {
                database
                    .tags
                    .get(*id)
                    .map(|tag| tag.to_owned().into_boxed_str())
            })
            .collect(),

        sources: entity
            .sources
            .iter()
            .filter_map(|id| {
                database.sources.get(*id).map(|provider| SourceInput {
                    provider: provider.into(),
                    verified: false,
                    created: None,
                    updated: None,
                })
            })
            .collect(),

        attributes: entity
            .attributes
            .iter()
            .filter_map(|attribute| {
                database
                    .attribute_keys
                    .get(attribute.key)
                    .map(|key| AttributesInput {
                        key: key.into(),
                        value: attribute.value.normalized(),
                    })
            })
            .collect(),

        relationships: entity
            .relationships
            .iter()
            .map(|relationship| RelationshipInput {
                target: relationship.target,
                kind: relationship.kind.to_string().into(),
            })
            .collect(),
    }
}

pub(crate) fn merge_databases(
    left: &Engine,
    right: &Engine,
    output: &mut Engine,
) -> Result<MergeReport> {
    let mut report = MergeReport::default();

    for entity in &left.database().entities {
        let input = entity_to_input(left.database(), entity);

        output.insert(input)?;
        report.entities_added += 1;
    }

    for entity in &right.database().entities {
        if output.entity(entity.id).is_some() {
            report.entities_skipped += 1;
            continue;
        }

        let input = entity_to_input(right.database(), entity);

        output.insert(input)?;
        report.entities_added += 1;
    }

    output.flush()?;

    Ok(report)
}
