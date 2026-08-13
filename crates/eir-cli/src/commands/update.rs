use anyhow::{Result, bail};
use clap::Args;
use std::path::PathBuf;

use eir_core::{
    engine::Engine,
    entity::{loader::load_entities, prelude::types::EntityID},
};

#[derive(Args, Debug)]
pub struct UpdateArgs {
    /// Database to update
    pub database: PathBuf,

    /// Entity ID to update
    pub entity: EntityID,

    /// JSON file containing the replacement entity
    #[arg(short, long)]
    pub input: PathBuf,
}

pub fn execute(args: UpdateArgs) -> Result<()> {
    let entities = load_entities(&args.input)?;

    if entities.len() != 1 {
        bail!(
            "update requires exactly one entity in the input file, found {}",
            entities.len()
        );
    }

    let entity = entities.into_iter().next().unwrap();

    if entity.id != args.entity {
        bail!(
            "entity ID mismatch: command specifies {}, input contains {}",
            args.entity,
            entity.id
        );
    }

    let mut engine = Engine::open(&args.database)?;

    engine.update(entity)?;
    engine.flush()?;

    println!("Updated entity {}", args.entity);

    Ok(())
}
