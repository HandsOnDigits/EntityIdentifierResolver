use crate::{
    query::Filter,
    search::{context::SearchContext, signal::Signal},
};

pub fn execute(ctx: &mut SearchContext) {
    for filter in &ctx.query.filters {
        let Filter::Attribute { key, op, value } = filter else {
            continue;
        };

        for entity in ctx.resolver.attribute_compare(key, *op, value) {
            ctx.candidates.add_signal(entity, Signal::Property);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        entity::prelude::types::{AttributeKeyID, EntityID, Value},
        index::Resolver,
        query::Query,
        search::{CandidateSet, context::SearchContext},
        test_utils::{test_entity, test_entity_with_attribute},
    };

    #[test]
    fn attribute_equals_adds_candidate() {
        let mut resolver = Resolver::default();

        let attribute_key = AttributeKeyID::new(1);
        let entity_id = EntityID::new(1);

        resolver.register_attribute(attribute_key, "brand".into());

        resolver.add(test_entity_with_attribute(
            entity_id,
            "FizzBerry Spark",
            attribute_key,
            Value::String("Acme".into()),
        ));

        let query = Query::parse("brand=Acme");

        let mut ctx = SearchContext {
            database: None,
            resolver,
            query: &query,
            candidates: CandidateSet::default(),
        };

        execute(&mut ctx);

        assert!(
            ctx.candidates
                .get(entity_id)
                .unwrap()
                .signals
                .contains(&Signal::Property)
        );
    }

    #[test]
    fn attribute_equals_does_not_match_different_value() {
        let mut resolver = Resolver::default();

        let attribute_key = AttributeKeyID::new(1);
        let entity_id = EntityID::new(1);

        resolver.register_attribute(attribute_key, "brand".into());

        resolver.add(test_entity_with_attribute(
            entity_id,
            "FizzBerry Spark",
            attribute_key,
            Value::String("Acme".into()),
        ));

        let query = Query::parse("brand=Other");

        let mut ctx = SearchContext {
            database: None,
            resolver,
            query: &query,
            candidates: CandidateSet::default(),
        };

        execute(&mut ctx);

        assert!(ctx.candidates.get(entity_id).is_none());
    }

    #[test]
    fn attribute_not_equals_matches_different_value() {
        let mut resolver = Resolver::default();

        let attribute_key = AttributeKeyID::new(1);

        resolver.register_attribute(attribute_key, "brand".into());

        resolver.add(test_entity_with_attribute(
            EntityID::new(1),
            "Acme Product",
            attribute_key,
            Value::String("Acme".into()),
        ));

        resolver.add(test_entity_with_attribute(
            EntityID::new(2),
            "Other Product",
            attribute_key,
            Value::String("Other".into()),
        ));

        resolver.add(test_entity(EntityID::new(3), "No Brand Product"));

        let query = Query::parse("brand!=Acme");

        let mut ctx = SearchContext {
            database: None,
            resolver,
            query: &query,
            candidates: CandidateSet::default(),
        };

        execute(&mut ctx);

        // brand = Acme → excluded
        assert!(ctx.candidates.get(EntityID::new(1)).is_none());

        // brand = Other → included
        let candidate = ctx.candidates.get(EntityID::new(2)).unwrap();

        assert!(candidate.signals.contains(&Signal::Property));

        // No brand → excluded
        assert!(ctx.candidates.get(EntityID::new(3)).is_none());
    }
}
