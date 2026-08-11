use eir_core::{
    entity::{EntityDocument, EntityInput},
    index::Resolver,
};

use eir_utils::registry::Registry;

fn main() {
    let mut registry = Registry::default();
    let mut resolver = Resolver::new();

    let drink = registry.tags.intern("drink");
    let soft_drink = registry.tags.intern("soft drink");
    let brand = registry.attributes.intern("brand");

    resolver.add(EntityInput {
        document: EntityDocument {
            id: 1,
            entity_type: *b"PROD",
            sources: vec![],
            attributes: vec![brand],
        },
        aliases: vec!["Coca Cola".into(), "Coke".into()],
        tags: vec![drink, soft_drink],
    });
}
