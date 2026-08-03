use std::path::PathBuf;

use super::{context::GeneratorContext, loader, mapper};

use eir_core::storage::indexes::IndexBuilder;

pub fn generate(input: PathBuf, output: PathBuf) -> anyhow::Result<()> {
    println!("Loading entities...");

    let fixtures = loader::load(input)?;

    println!("Building registry...");

    let mut ctx = GeneratorContext::new();

    println!("Mapping entities...");

    let entities = fixtures
        .into_iter()
        .map(|entity| mapper::map(entity, &mut ctx))
        .collect::<Vec<_>>();

    println!("Building indexes...");

    let indexes = IndexBuilder::build(&entities);

    println!("Writing database...");

    Ok(())
}
