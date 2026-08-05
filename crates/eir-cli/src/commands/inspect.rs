use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use eir_core::engine::load_database;

#[derive(Args, Debug)]
pub struct InspectArgs {
    pub input: PathBuf,

    #[arg(short, long)]
    pub entity: u64,

    /// Show internal IDs
    #[arg(short, long)]
    pub verbose: bool,
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
        if args.verbose {
            println!("  {} ({})", database.tags[tag.to_native() as usize], tag);
        } else {
            println!("  {}", database.tags[tag.to_native() as usize]);
        }
    }

    println!();

    println!("Properties:");
    for attribute in entity.attributes.iter() {
        if args.verbose {
            println!(
                "  {:?}: {:?}",
                attribute.value.display_value(),
                attribute.key
            );
        } else {
            println!("{}", attribute.value.display_value());
        }
    }

    println!();

    println!("Relationships:");
    for relationship in entity.relationships.iter() {
        let target = database
            .entities
            .iter()
            .find(|e| e.id == relationship.target);

        match target {
            Some(target) => {
                let name = target.aliases.first().map(|x| &**x).unwrap_or("Unknown");

                if args.verbose {
                    println!(
                        "  {} -> {} ({})",
                        relationship.kind.as_str(),
                        name,
                        relationship.target
                    );
                } else {
                    println!("  {} -> {}", relationship.kind.as_str(), name);
                }
            }
            None => {
                println!(
                    "  {} -> Unknown ({})",
                    relationship.kind.as_str(),
                    relationship.target
                );
            }
        }
    }

    println!();

    println!("Sources:");
    for source in entity.sources.iter() {
        let name = &database.sources[source.to_native() as usize];

        if args.verbose {
            println!("  {} ({})", name, source.to_native());
        } else {
            println!("  {}", name);
        }
    }

    Ok(())
}
