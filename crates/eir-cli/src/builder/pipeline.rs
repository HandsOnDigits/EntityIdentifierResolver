use eir_core::{Database, engine::indexes::Indexes, storage::IndexBuilder};

use super::{context::BuilderContext, loader::load_entities, mapper, writer::write_database};

pub fn build(
    input: impl AsRef<std::path::Path>,
    output: impl AsRef<std::path::Path>,
) -> anyhow::Result<()> {
    let mut ctx = BuilderContext::new();

    let fixtures = load_entities(input)?;

    let entities = fixtures
        .into_iter()
        .map(|entity| mapper::map(entity, &mut ctx))
        .collect::<Vec<_>>();

    let built = IndexBuilder::build(&entities, ctx.attribute_keys.values());

    let indexes = Indexes::from_builder(built);

    let database = Database {
        entities,
        tags: ctx.tags,
        sources: ctx.sources,
        attribute_keys: ctx.attribute_keys,
        relationship_types: ctx.relationship_types,
        indexes,
    };

    write_database(database, output)?;

    Ok(())
}
