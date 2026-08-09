use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use eir_core::engine::Engine;
use eir_core::entity::prelude::types::EntityID;

#[derive(Args, Debug)]
pub struct InspectArgs {
    /// Database file
    pub input: PathBuf,

    /// Entity IDs to inspect
    #[arg(short, long, required = true)]
    pub entity: Vec<EntityID>,

    /// Show internal IDs
    #[arg(short, long)]
    pub verbose: bool,
}

pub fn execute(args: InspectArgs) -> Result<()> {
    let engine = Engine::open(&args.input)?;
    let database = engine.database();

    for id in args.entity {
        let Some(entity) = database.entity(id) else {
            println!("Entity {} not found", id);
            println!();
            continue;
        };

        println!("Entity: {}", entity.id);
        println!();

        println!("Names:");
        for name in &entity.aliases {
            println!("  {}", name);
        }

        println!();

        println!("Tags:");
        for &tag in &entity.tags {
            let name = database.tags.get(tag).unwrap_or("Unknown");

            if args.verbose {
                println!("  {} ({})", name, tag);
            } else {
                println!("  {}", name);
            }
        }

        println!();

        println!("Properties:");
        for attribute in &entity.attributes {
            if args.verbose {
                println!("  {:?}: {}", attribute.key, attribute.value);
            } else {
                println!("  {}", attribute.value);
            }
        }

        println!();

        println!("Relationships:");
        for relationship in &entity.relationships {
            let target = database.entity(relationship.target);

            match target {
                Some(target) => {
                    let name = target
                        .aliases
                        .first()
                        .map(|x| x.as_ref())
                        .unwrap_or("Unknown");

                    if args.verbose {
                        println!(
                            "  {} -> {} ({})",
                            relationship.kind, name, relationship.target
                        );
                    } else {
                        println!("  {} -> {}", relationship.kind, name);
                    }
                }

                None => {
                    println!(
                        "  {} -> Unknown ({})",
                        relationship.kind, relationship.target
                    );
                }
            }
        }

        println!();

        println!("Sources:");
        for &source in &entity.sources {
            let name = database.sources.get(source).unwrap_or("Unknown");

            if args.verbose {
                println!("  {} ({})", name, source);
            } else {
                println!("  {}", name);
            }
        }

        println!();
    }

    Ok(())
}
