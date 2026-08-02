use eir_core::entity::{EntityDocument, EntityInput, types::*};
use eir_core::index::Resolver;

fn main() {
    let mut resolver = Resolver::new();

    let drink = resolver.tags.intern("drink");
    let soft_drink = resolver.tags.intern("soft drink");
    let brand = resolver.properties.intern("brand");

    resolver.add(EntityInput {
        document: EntityDocument {
            id: EntityID(1),
            entity_type: EntityType::Product,
            sources: vec![],
            properties: vec![brand],
        },
        aliases: vec!["Coca Cola".into(), "Coke".into()],
        tags: vec![drink, soft_drink],
    });

    resolver.add(EntityInput {
        document: EntityDocument {
            id: EntityID(2),
            entity_type: EntityType::Product,
            sources: vec![],
            properties: vec![],
        },
        aliases: vec!["Pepsi".into()],
        tags: vec![drink],
    });

    resolver.add(EntityInput {
        document: EntityDocument {
            id: EntityID(3),
            entity_type: EntityType::Product,
            sources: vec![],
            properties: vec![],
        },
        aliases: vec!["Coconut Water".into()],
        tags: vec![drink],
    });

    println!("{:#?}", resolver.search("coca"));
    println!("{:#?}", resolver.search("coke"));
    println!("{:#?}", resolver.search("coc"));
    println!("{:#?}", resolver.search("pepsi"));
}
