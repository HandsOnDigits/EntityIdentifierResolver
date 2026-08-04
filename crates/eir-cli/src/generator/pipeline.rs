use eir_core::{
    Database,
    entity::types::{EntityID, TagID},
};

use super::{context::GeneratorContext, loader::load_entities, mapper, writer::write_database};

use std::collections::HashMap;
use std::path::Path;

pub fn generate(input: impl AsRef<Path>, output: impl AsRef<Path>) -> anyhow::Result<()> {
    let mut ctx = GeneratorContext::new();

    let fixtures = load_entities(input)?;

    let inputs = fixtures
        .into_iter()
        .map(|entity| mapper::map(entity, &mut ctx))
        .collect::<Vec<_>>();

    let mut entities = Vec::new();
    let mut tags: HashMap<TagID, Vec<EntityID>> = HashMap::new();

    for input in inputs {
        entities.push(input.clone());

        for tag in &input.tags {
            tags.entry(*tag).or_default().push(input.id);
        }
    }

    let database = Database {
        entities,
        tags: ctx.tags.into_inner(),
        sources: ctx.sources.into_inner(),
        properties: ctx.properties.into_inner(),
    };

    write_database(database)?;

    Ok(())
}
