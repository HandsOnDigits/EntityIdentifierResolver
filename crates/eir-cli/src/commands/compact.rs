use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use eir_core::{
    engine::Engine,
    utils::{directory_size, format_bytes},
};

#[derive(Args, Debug)]
pub struct CompactArgs {
    /// Logical `.eir` database
    pub input: PathBuf,
}

pub fn execute(args: CompactArgs) -> Result<()> {
    let mut engine = Engine::open(&args.input)?;

    let storage_path = args
        .input
        .parent()
        .ok_or_else(|| anyhow::anyhow!("database path has no parent directory"))?;

    let before = directory_size(storage_path)?;

    engine.compact()?;

    let after = directory_size(storage_path)?;

    let reclaimed = before.saturating_sub(after);

    println!("Database Compacted");
    println!("==================");
    println!();
    println!("Before:    {}", format_bytes(before));
    println!("After:     {}", format_bytes(after));
    println!("Reclaimed: {}", format_bytes(reclaimed));

    if before > 0 {
        let percentage = (reclaimed as f64 / before as f64) * 100.0;
        println!("Savings:   {:.1}%", percentage);
    }

    Ok(())
}
