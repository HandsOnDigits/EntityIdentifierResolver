mod database;
mod generate_db;
mod loader;
mod mapper;
mod registry;
mod writer;

use anyhow::Result;
use database::DatabaseMetadata;

fn main() -> Result<()> {
    std::fs::create_dir_all("output")?;

    let db = generate_db::generate_database()?;

    writer::write_binary("output/entities.bin", &db.entities)?;

    writer::write_binary("output/tags.bin", &db.tags)?;

    writer::write_binary("output/relationships.bin", &db.relationships)?;

    let metadata = DatabaseMetadata {
        version: 1,

        created: eir_core::entity::types::Date::now(),

        entity_count: db.entities.len() as u64,

        tag_count: db.tags.len() as u32,

        property_count: db.properties.len() as u32,

        relationship_count: db.relationships.len() as u64,
    };

    writer::write_binary("output/metadata.bin", &metadata)?;

    println!(
        "Generated database: {} entities, {} tags",
        db.entities.len(),
        db.tags.len()
    );

    Ok(())
}
