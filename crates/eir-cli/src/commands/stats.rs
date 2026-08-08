use rkyv::{access, rancor::Error};
use std::fs;

pub fn execute(path: std::path::PathBuf) -> anyhow::Result<()> {
    let bytes = fs::read(path)?;

    let database = access::<eir_core::engine::database::ArchivedDatabaseRecord, Error>(&bytes)?;

    println!("Database Statistics");
    println!("===================");
    println!();
    println!("Entities:           {}", database.entities.len());
    println!("Tags:               {}", database.indexes.tags.index.len());
    println!(
        "Sources:            {}",
        database.indexes.sources.index.len()
    );
    println!(
        "attributes:         {}",
        database.indexes.attribute_keys.index.len()
    );
    println!(
        "Relationship Types: {}",
        database.relationship_types.values.len()
    );

    println!();
    println!("Indexes");
    println!("-------");
    println!("Aliases:     {}", database.indexes.alias.entries.len());
    println!("Trie:        {}", database.indexes.trie.entries.len());
    println!("BK-Tree:     {}", database.indexes.bk_tree.entries.len());
    println!("Tokens:      {}", database.indexes.inverted.index.len());
    println!("Tags:        {}", database.indexes.tags.index.len());
    println!("Sources:     {}", database.indexes.sources.index.len());
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
