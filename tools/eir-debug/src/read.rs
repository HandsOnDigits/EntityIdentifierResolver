use anyhow::Result;
use eir_core::entity::Entity;
use rkyv::rancor::Error;
use rkyv::{Archived, access};
use std::fs;

pub fn read_entities(path: &str) -> Result<()> {
    let bytes = fs::read(path)?;

    let archived = access::<Archived<Vec<Entity>>, Error>(&bytes)?;

    println!("Entities: {}", archived.len());

    for entity in archived.iter() {
        println!("ID: {}", entity.id.0);

        for name in entity.names.iter() {
            println!("Name: {}", name);
        }

        println!("Tags: {:?}", entity.tags);

        println!("---");
    }

    Ok(())
}
