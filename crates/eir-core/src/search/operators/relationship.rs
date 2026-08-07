use crate::{
    query::Filter,
    search::{context::SearchContext, signal::Signal},
};

pub fn execute(ctx: &mut SearchContext) {
    for filter in &ctx.query.filters {
        let Filter::Relationship { kind, target } = filter else {
            continue;
        };

        let Some(kind_id) = ctx.resolver.relationship_type_id(kind) else {
            continue;
        };

        let Some(target_id) = ctx.resolver.resolve(target).first() else {
            continue;
        };

        for entity_id in ctx.resolver.relationship_lookup(kind_id, *target_id) {
            ctx.candidates.add_signal(entity_id, Signal::Relationship);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        entity::prelude::{
            EntityDocument,
            types::{EntityID, Relationship},
        },
        index::Resolver,
        query::{Query, QueryIntent},
        search::{SearchPlan, SearchStage, candidate::CandidateSet},
        test::test_entity,
    };

    #[test]
    fn relationship_adds_candidate() {
        let mut resolver = Resolver::default();

        let entity_id_1 = EntityID(1);
        let entity_id_2 = EntityID(10);

        let manufacturer_type = resolver.register_relationship_type("manufacturer");

        let manufacturer = test_entity(entity_id_2, "Nestle");

        let product = EntityDocument {
            id: entity_id_1,
            aliases: vec!["Chocolate Bar".into()],
            relationships: vec![Relationship {
                kind: manufacturer_type,
                target: entity_id_2,
            }],
            attributes: Vec::new(),
            sources: Vec::new(),
            tags: Vec::new(),
        };

        resolver.add(manufacturer);
        resolver.add(product);

        let query = Query::parse("relation:manufacturer:nestle");

        let mut ctx = SearchContext {
            database: None,
            resolver,
            query: &query,
            candidates: CandidateSet::default(),
        };

        execute(&mut ctx);

        let candidate = ctx.candidates.get(entity_id_1).unwrap();
        assert!(candidate.signals.contains(&Signal::Relationship));
    }

    #[test]
    fn relationship_lookup_finds_target() {
        let mut resolver = Resolver::default();

        let entity_id_1 = EntityID(1);
        let entity_id_2 = EntityID(10);

        let manufacturer_type = resolver.register_relationship_type("manufacturer");

        let nestle = test_entity(entity_id_2, "Nestle");

        let chocolate = EntityDocument {
            id: entity_id_1,
            aliases: vec!["Chocolate Bar".into()],
            relationships: vec![Relationship {
                kind: manufacturer_type,
                target: entity_id_2,
            }],
            attributes: Vec::new(),
            sources: Vec::new(),
            tags: Vec::new(),
        };

        resolver.add(nestle);
        resolver.add(chocolate);

        let results = resolver.relationship_lookup(manufacturer_type, entity_id_2);

        assert!(results.contains(&entity_id_1));
    }

    #[test]
    fn relationship_creates_relationship_plan() {
        let query = Query {
            original: "relation:manufacturer:nestle".into(),
            normalized: "relation:manufacturer:nestle".into(),
            tokens: vec!["nestle".into()],
            intent: QueryIntent::Relationship,
            filters: Vec::new(),
        };

        let plan = SearchPlan::from_query(&query);

        assert!(plan.stages.contains(&SearchStage::Relationship));
    }
}
