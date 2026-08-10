use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use eir_core::engine::Engine;

#[derive(Args, Debug)]
pub struct InitArgs {
    /// Parent directory for the database
    pub parent: PathBuf,

    /// Database name
    pub name: String,
}

pub fn execute(args: InitArgs) -> Result<()> {
    Engine::create(args.parent, &args.name)?;
    Ok(())
}
