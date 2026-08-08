use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use eir_core::{engine::load_database_owned, entity::prelude::types::EntityID};

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Database file
    pub database: PathBuf,

    /// Entity IDs to remove
    #[arg(short, long)]
    pub entity: Vec<u64>,
}

pub fn execute(args: RemoveArgs) -> Result<()> {
    let mut database = load_database_owned(&args.database)?;

    for id in args.entity {
        database.remove(EntityID(id))?;
    }

    database.save(&args.database)?;

    Ok(())
}
