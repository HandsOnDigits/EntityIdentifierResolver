use eir_core::{
    engine::{Database, Engine},
    entity::prelude::{
        EntityDocument,
        input::{AttributesInput, EntityInput, RelationshipInput, SourceInput},
        types::RelationshipType,
    },
};

use crate::{MergeReport, Result};

fn entity_to_input(database: &Database, entity: &EntityDocument) -> EntityInput {
    EntityInput {
        id: entity.id,

        aliases: entity.aliases.clone(),

        tags: entity
            .tags
            .iter()
            .filter_map(|id| database.tags.get(*id))
            .map(Into::into)
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
                        value: attribute.value.to_string().into_boxed_str(),
                    })
            })
            .collect(),

        relationships: entity
            .relationships
            .iter()
            .filter_map(|relationship| {
                let kind = match relationship.kind {
                    RelationshipType::Custom(id) => database.relationship_types.get(id)?,
                    _ => return None,
                };

                Some(RelationshipInput {
                    target: relationship.target,
                    kind: kind.into(),
                })
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

    // Insert left database first. Its registry values establish
    // the initial IDs in the output registries.
    for entity in &left.database().entities {
        if output.entity(entity.id).is_some() {
            report.entities_skipped += 1;
            continue;
        }

        output.insert(entity_to_input(left.database(), entity))?;
        report.entities_added += 1;
    }

    // Insert right database through EntityInput as well.
    //
    // IMPORTANT: do not copy registry IDs from the right database.
    // Database::insert() interns the logical values ("country",
    // "origin", etc.) into the output registries and therefore
    // automatically remaps their IDs.
    for entity in &right.database().entities {
        if output.entity(entity.id).is_some() {
            report.entities_skipped += 1;
            continue;
        }

        output.insert(entity_to_input(right.database(), entity))?;
        report.entities_added += 1;
    }

    output.flush()?;

    Ok(report)
}
