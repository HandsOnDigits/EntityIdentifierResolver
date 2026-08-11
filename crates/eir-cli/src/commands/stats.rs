use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use eir_core::{
    engine::Engine,
    utils::{directory_size, format_bytes},
};

#[derive(Args, Debug)]
pub struct StatsArgs {
    /// Logical `.eir` database
    pub input: PathBuf,
}

pub fn execute(args: StatsArgs) -> Result<()> {
    let engine = Engine::open(&args.input)?;

    let stats = engine.stats();

    let storage_path = args
        .input
        .parent()
        .ok_or_else(|| anyhow::anyhow!("database path has no parent directory"))?;

    let size = directory_size(storage_path)?;

    println!("Database Statistics");
    println!("===================");
    println!();

    println!("Entities:           {}", stats.entities);
    println!("Tags:               {}", stats.tags);
    println!("Sources:            {}", stats.sources);
    println!("Attributes:         {}", stats.attributes);
    println!("Relationship Types: {}", stats.relationship_types);

    println!();
    println!("Indexes");
    println!("-------");
    println!("Aliases:       {}", stats.aliases);
    println!("Trie:          {}", stats.trie);
    println!("Fuzzy Aliases: {}", stats.fuzzy_aliases);
    println!("Tokens:        {}", stats.tokens);
    println!("Tags:          {}", stats.tags);
    println!("Sources:       {}", stats.sources);
    println!("Relationships: {}", stats.relationships);
    println!("Size:     {}", format_bytes(size));

    Ok(())
}
