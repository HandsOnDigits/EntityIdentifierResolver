mod database;
mod generate_db;
mod loader;
mod mapper;
mod registry;
mod writer;

use anyhow::Result;
use database::DatabaseMetadata;
use eir_core::entity::types::EntityID;

use std::collections::HashSet;

fn main() -> Result<()> {
    std::fs::create_dir_all("output")?;

    let db = generate_db::generate_database()?;

    let entity_ids: HashSet<EntityID> = db.entities.iter().map(|e| e.id).collect();

    for entity in &db.entities {
        for relation in &entity.relationships {
            if !entity_ids.contains(&relation.target) {
                anyhow::bail!(
                    "Entity {} references missing entity {}",
                    entity.id.0,
                    relation.target.0
                );
            }
        }
    }

    writer::write_binary("output/entities.bin", &db.entities)?;

    writer::write_binary("output/tags.bin", &db.tags)?;

    writer::write_binary("output/relationships.bin", &db.relationships)?;

    writer::write_binary("output/properties.bin", &db.properties)?;

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
        "Generated database: {} entities, {} tags, {} relationships",
        db.entities.len(),
        db.tags.len(),
        db.relationships.len()
    );

    Ok(())
}
