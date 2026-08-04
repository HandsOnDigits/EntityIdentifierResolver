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

    let indexes = IndexBuilder::build(&inputs);

    let mut aliases = HashMap::new();
    let mut entities = Vec::new();

    for input in inputs {
        aliases.insert(input.id, input.aliases.clone());

        entities.push(input);
    }

    let database = Database {
        entities,
        aliases,

        tags: ctx.tags.into_inner(),
        sources: ctx.sources.into_inner(),
        properties: ctx.properties.into_inner(),

        tag_index: indexes.tags.to_record(),
        source_index: indexes.sources.to_record(),
    };

    write_database(database, output)?;

    Ok(())
}
