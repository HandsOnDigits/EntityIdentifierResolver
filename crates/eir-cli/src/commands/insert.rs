use std::path::PathBuf;

use clap::Args;

use eir_core::{engine::load_database_owned, entity::loader::load_entities};

#[derive(Args, Debug)]
pub struct InsertArgs {
    /// Database file
    pub database: PathBuf,

    /// Entity dataset
    pub input: PathBuf,
}

pub fn execute(args: InsertArgs) -> anyhow::Result<()> {
    let mut database = load_database_owned(&args.database)?;

    let entities = load_entities(&args.input)?;

    for entity in entities {
        database.insert(entity)?;
    }

    database.save(&args.database)?;

    Ok(())
}
