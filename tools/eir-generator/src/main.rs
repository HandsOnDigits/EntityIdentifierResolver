mod generate_db;
mod loader;
mod registry;
mod writer;

use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let entities = generate_db::generate_entities()?;

    fs::create_dir_all("output")?;

    writer::write_binary("output/entities.bin", &entities)?;

    println!("Generated {} entities", entities.len());

    Ok(())
}
