use std::collections::HashMap;

use eir_core::{Database, storage::IndexBuilder};

use super::{context::BuilderContext, loader::load_entities, mapper, writer::write_database};

pub fn build(
    input: impl AsRef<std::path::Path>,
    output: impl AsRef<std::path::Path>,
) -> anyhow::Result<()> {
    let mut ctx = BuilderContext::new();

    let fixtures = load_entities(input)?;

    let inputs = fixtures
        .into_iter()
        .map(|entity| mapper::map(entity, &mut ctx))
        .collect::<Vec<_>>();

    let mut entities = Vec::with_capacity(inputs.len());
    let mut aliases = HashMap::new();

    for entity in &inputs {
        entities.push(entity.clone());

        aliases.insert(entity.id, entity.clone());
    }

    let indexes = IndexBuilder::build(&inputs);

    let database = Database {
        entities,
        tags: ctx.tags.into_inner(),
        sources: ctx.sources.into_inner(),
        properties: ctx.properties.into_inner(),

        tag_index: indexes.tags.to_archive(),
        source_index: indexes.sources.to_archive(),
    };

    write_database(database, output)?;

    Ok(())
}
