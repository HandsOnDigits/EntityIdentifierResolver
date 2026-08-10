use std::path::PathBuf;

use clap::Args;

use eir_core::{engine::Engine, entity::loader::load_entities};

#[derive(Args, Debug)]
pub struct InsertArgs {
    /// Database file
    pub database: PathBuf,

    /// Entity dataset
    pub input: PathBuf,
}

pub fn execute(args: InsertArgs) -> anyhow::Result<()> {
    let mut engine = Engine::open(&args.database)?;

    let entities = load_entities(&args.input)?;

    for entity in entities {
        println!("Inserting: {} {:?}", entity.id, entity.aliases);
        engine.insert(entity)?;
    }

    println!(
        "Search before flush: {} result(s)",
        engine.search("Test Berry").len()
    );

    engine.flush()?;

    Ok(())
}
