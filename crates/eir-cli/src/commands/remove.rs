use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use eir_core::{engine::Engine, entity::prelude::types::EntityID};

#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Database file
    pub database: PathBuf,

    /// Entity IDs to remove
    pub entity: Vec<usize>,
}

pub fn execute(args: RemoveArgs) -> Result<()> {
    let mut engine = Engine::open(&args.database)?;

    for id in args.entity {
        engine.remove(EntityID::new(id))?;
    }

    engine.flush()?;

    Ok(())
}
