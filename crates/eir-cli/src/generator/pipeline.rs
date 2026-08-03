use eir_core::Database;

use super::{context::GeneratorContext, loader::load_entities, mapper, writer::write_database};

pub fn generate() -> anyhow::Result<()> {
    let mut ctx = GeneratorContext::new();

    let fixtures = load_entities()?;

    let inputs = fixtures
        .into_iter()
        .map(|entity| mapper::map(entity, &mut ctx))
        .collect::<Vec<_>>();

    let entities = inputs.iter().map(|input| input.document.clone()).collect();

    let aliases = inputs.iter().map(|input| input.aliases.clone()).collect();

    let database = Database {
        entities,
        aliases,
        sources: ctx.sources.into_inner(),
        properties: ctx.properties.into_inner(),
        tags: ctx.tags.into_inner(),
    };

    write_database(database)?;

    Ok(())
}
