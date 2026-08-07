use std::fs;

use eir_core::engine::database::ArchivedDatabase;
use rkyv::{access, rancor::Error};

pub fn execute(path: std::path::PathBuf) -> anyhow::Result<()> {
    let bytes = fs::read(path)?;

    let database: &ArchivedDatabase = access::<ArchivedDatabase, Error>(&bytes)?;

    println!("Database Statistics");
    println!("===================");
    println!();
    println!("Entities:           {}", database.entities.len());
    println!("Tags:               {}", database.tags.len());
    println!("Sources:            {}", database.sources.len());
    println!("attributes:         {}", database.attribute_keys.len());
    println!(
        "Relationship Types: {}",
        database.relationship_types.values.len()
    );

    println!();
    println!("Indexes");
    println!("-------");
    println!("Aliases:     {}", database.alias_index.entries.len());
    println!("Trie:        {}", database.trie_index.entries.len());
    println!("BK-Tree:     {}", database.bk_tree_index.entries.len());
    println!("Tokens:      {}", database.inverted_index.entries.len());
    println!("Tags:        {}", database.tag_index.index.len());
    println!("Sources:     {}", database.source_index.index.len());
    println!(
        "Relationships: {}",
        database
            .entities
            .iter()
            .map(|e| e.relationships.len())
            .sum::<usize>()
    );

    Ok(())
}
