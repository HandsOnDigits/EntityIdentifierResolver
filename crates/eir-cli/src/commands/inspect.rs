use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use eir_core::engine::load_database;

#[derive(Args, Debug)]
pub struct InspectArgs {
    pub input: PathBuf,

    #[arg(short, long)]
    pub entity: u64,
}

pub fn execute(args: InspectArgs) -> Result<()> {
    let database = load_database(&args.input)?;

    let entity = database
        .entities
        .iter()
        .find(|entity| entity.id == args.entity)
        .ok_or_else(|| anyhow::anyhow!("Entity not found"))?;

    println!("Entity: {}", entity.id);
    println!();

    println!("Names:");
    for name in entity.aliases.iter() {
        println!("  {}", name);
    }

    println!();

    println!("Tags:");
    for tag in entity.tags.iter() {
        println!("  {}", database.tags[tag.to_native() as usize]);
    }

    println!();

    println!("Properties:");
    for property in entity.properties.iter() {
        println!("  {}", database.properties[property.to_native() as usize]);
    }

    println!();

    println!("Relationships:");
    for relationship in entity.relationships.iter() {
        let target = database
            .entities
            .iter()
            .find(|e| e.id == relationship.target)
            .and_then(|e| e.aliases.first())
            .map(|name| &**name)
            .unwrap_or("Unknown");

        println!("  {} -> {}", relationship.kind.as_str(), target);
    }

    println!();

    println!("Sources:");
    for source in entity.sources.iter() {
        println!("  {}", database.sources[source.to_native() as usize]);
    }

    Ok(())
}
