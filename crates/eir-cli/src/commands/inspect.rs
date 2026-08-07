use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use eir_core::entity::archived_id_index;

use eir_core::{engine::load_database, entity::prelude::types::EntityID};

#[derive(Args, Debug)]
pub struct InspectArgs {
    pub input: PathBuf,

    #[arg(short, long)]
    pub entity: EntityID,

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

    println!("Entity: {}", archived_id_index!(entity.id));
    println!();

    println!("Names:");
    for name in entity.aliases.iter() {
        println!("  {}", name);
    }

    println!();

    println!("Tags:");
    for tag in entity.tags.iter() {
        let index = archived_id_index!(tag);

        if args.verbose {
            println!("  {} ({})", database.tags[index], index);
        } else {
            println!("  {}", database.tags[index]);
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
                        archived_id_index!(relationship.kind),
                        name,
                        archived_id_index!(relationship.target)
                    );
                } else {
                    println!("  {} -> {}", archived_id_index!(relationship.kind), name);
                }
            }
            None => {
                println!(
                    "  {} -> Unknown ({})",
                    archived_id_index!(relationship.kind),
                    archived_id_index!(relationship.target)
                );
            }
        }
    }

    println!();

    println!("Sources:");
    for source in entity.sources.iter() {
        let index = archived_id_index!(source);

        let name = &database.sources[index];

        if args.verbose {
            println!("  {} ({})", name, index);
        } else {
            println!("  {}", name);
        }
    }

    Ok(())
}
