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
    let mut engine = if args.database.exists() {
        Engine::open(&args.database)?
    } else {
        Engine::create(&args.database)?
    };

    let entities = load_entities(&args.input)?;

    for entity in entities {
        engine.insert(entity)?;
    }

    engine.flush()?;

    Ok(())
}
