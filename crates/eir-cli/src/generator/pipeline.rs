use std::path::PathBuf;

use crate::{indexer::IndexBuilder, loader, mapper, writer};

use eir_utils::registry::Registry;

pub fn generate(input: PathBuf, output: PathBuf) -> anyhow::Result<()> {
    println!("Loading entities...");

    let fixtures = loader::load(input)?;

    println!("Building registry...");

    let mut registry = Registry::default();

    println!("Mapping entities...");

    let entities = fixtures
        .into_iter()
        .map(|entity| mapper::map(entity, &mut registry))
        .collect::<Vec<_>>();

    println!("Building indexes...");

    let indexes = IndexBuilder::build(&entities);

    println!("Writing database...");

    writer::write(output, &entities, &registry, &indexes)?;

    Ok(())
}
