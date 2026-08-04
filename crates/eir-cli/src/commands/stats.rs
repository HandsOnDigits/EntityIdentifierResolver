use std::fs;

use eir_core::engine::database::ArchivedDatabase;
use rkyv::{access, rancor::Error};

pub fn execute(path: std::path::PathBuf) -> anyhow::Result<()> {
    let bytes = fs::read(path)?;

    let database: &ArchivedDatabase = access::<ArchivedDatabase, Error>(&bytes)?;

    println!("Database Statistics");
    println!();

    println!("Entities:    {}", database.entities.len());
    println!("Tags:        {}", database.tags.len());
    println!("Sources:     {}", database.sources.len());
    println!("Properties:  {}", database.properties.len());

    println!();

    println!("Indexes:");
    println!("  Tags:      {}", database.tag_index.index.len());
    println!("  Sources:   {}", database.source_index.index.len());

    Ok(())
}
