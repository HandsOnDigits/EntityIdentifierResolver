mod generate_db;
mod loader;
mod writer;

use anyhow::Result;

fn main() -> Result<()> {
    let entities = generate_db::generate_entities()?;

    writer::write_binary("output/entities.bin", &entities)?;

    println!("Generated {} entities", entities.len());

    Ok(())
}
