use anyhow::Result;
use eir_core::entity::Entity;
use rkyv::rancor::Error;
use rkyv::{Archived, access};
use std::collections::HashMap;
use std::fs;

use crate::database::DATABASE_PATH;

fn relationship_type_name(
    kind: &eir_core::entity::types::ArchivedRelationshipType,
) -> &'static str {
    match kind {
        eir_core::entity::types::ArchivedRelationshipType::IsA => "IsA",

        eir_core::entity::types::ArchivedRelationshipType::InstanceOf => "InstanceOf",

        eir_core::entity::types::ArchivedRelationshipType::PartOf => "PartOf",

        eir_core::entity::types::ArchivedRelationshipType::MadeBy => "MadeBy",

        eir_core::entity::types::ArchivedRelationshipType::OwnedBy => "OwnedBy",

        eir_core::entity::types::ArchivedRelationshipType::LocatedIn => "LocatedIn",

        eir_core::entity::types::ArchivedRelationshipType::SimilarTo => "SimilarTo",

        eir_core::entity::types::ArchivedRelationshipType::ReplacedBy => "ReplacedBy",
    }
}

pub fn read_database() -> Result<()> {
    let entity_bytes = fs::read(format!("{}/entities.bin", DATABASE_PATH))?;

    let tag_bytes = fs::read(format!("{}/tags.bin", DATABASE_PATH))?;

    let entities = access::<Archived<Vec<Entity>>, Error>(&entity_bytes)?;

    let tags = access::<Archived<Vec<(u32, String)>>, Error>(&tag_bytes)?;

    let tag_map: HashMap<u32, &str> = tags
        .iter()
        .map(|entry| (entry.0.to_native(), entry.1.as_str()))
        .collect();

    println!("Entities: {}", entities.len());

    for entity in entities.iter() {
        println!("\nID: {}", entity.id.0);

        for name in entity.names.iter() {
            println!("Name: {}", name);
        }

        println!("Tags:");

        for tag in entity.tags.iter() {
            if let Some(name) = tag_map.get(&tag.to_native()) {
                println!("  - {}", name);
            }
        }

        println!("Relationships:");

        for relation in entity.relationships.iter() {
            println!(
                "  {} -> {}",
                relationship_type_name(&relation.kind),
                relation.target.0
            );
        }
    }

    Ok(())
}
