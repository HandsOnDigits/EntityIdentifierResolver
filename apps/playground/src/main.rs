use eir_core::entity::{EntityDocument, types::EntityID};
use eir_core::index::Resolver;

fn main() {
    let mut resolver = Resolver::new();

    resolver.add(EntityDocument {
        id: EntityID(1),

        aliases: vec!["Coca Cola".into(), "Coke".into()],

        tags: vec!["drink".into(), "soft drink".into()],

        properties: vec!["brand".into()],

        sources: vec![],

        relationships: vec![],
    });

    resolver.add(EntityDocument {
        id: EntityID(2),

        aliases: vec!["Pepsi".into()],

        tags: vec!["drink".into()],

        properties: vec![],

        sources: vec![],

        relationships: vec![],
    });

    resolver.add(EntityDocument {
        id: EntityID(3),

        aliases: vec!["Coconut Water".into()],

        tags: vec!["drink".into()],

        properties: vec![],

        sources: vec![],

        relationships: vec![],
    });

    println!("{:#?}", resolver.search("coca"));
    println!("{:#?}", resolver.search("coke"));
    println!("{:#?}", resolver.search("coc"));
    println!("{:#?}", resolver.search("pepsi"));
}
