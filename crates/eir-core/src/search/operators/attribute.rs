use crate::{
    index::Resolver, query::Filter, search::candidate::CandidateSet, search::signal::Signal,
};

pub fn execute(resolver: &Resolver, candidates: &mut CandidateSet, filter: &Filter) {
    let Filter::Attribute { key, op, value } = filter else {
        return;
    };

    for entity in resolver.attribute_compare(key.as_ref(), *op, value) {
        candidates.add_signal(entity, Signal::Property);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        entity::prelude::types::{AttributeKeyID, EntityID, Value},
        index::Resolver,
        query::{Filter, FilterExpr, Query},
        search::CandidateSet,
        test_utils::test_entity_with_attribute,
    };

    fn resolver() -> Resolver {
        let mut resolver = Resolver::default();

        let brand = AttributeKeyID::new(1);
        let price = AttributeKeyID::new(2);

        resolver.register_attribute(brand, "brand".into());
        resolver.register_attribute(price, "price".into());

        resolver.add(test_entity_with_attribute(
            EntityID::new(1),
            "Acme Product",
            brand,
            Value::String("Acme".into()),
        ));

        resolver.add(test_entity_with_attribute(
            EntityID::new(2),
            "Other Product",
            brand,
            Value::String("Other".into()),
        ));

        resolver.add(test_entity_with_attribute(
            EntityID::new(3),
            "Cheap Product",
            price,
            Value::Integer(5),
        ));

        resolver.add(test_entity_with_attribute(
            EntityID::new(4),
            "Expensive Product",
            price,
            Value::Integer(20),
        ));

        resolver
    }

    fn filter(query: &str) -> Filter {
        let query = Query::parse(query);

        match query.filter.as_ref().unwrap() {
            FilterExpr::Filter(filter) => filter.clone(),
            _ => panic!("expected a single filter"),
        }
    }

    #[test]
    fn attribute_equals_adds_matching_candidate() {
        let resolver = resolver();
        let filter = filter("brand=Acme");

        let mut candidates = CandidateSet::default();

        execute(&resolver, &mut candidates, &filter);

        assert!(candidates.get(EntityID::new(1)).is_some());
        assert!(candidates.get(EntityID::new(2)).is_none());
        assert!(candidates.get(EntityID::new(3)).is_none());
        assert!(candidates.get(EntityID::new(4)).is_none());

        assert!(
            candidates
                .get(EntityID::new(1))
                .unwrap()
                .signals
                .contains(&Signal::Property)
        );
    }

    #[test]
    fn attribute_equals_does_not_match_different_value() {
        let resolver = resolver();
        let filter = filter("brand=Missing");

        let mut candidates = CandidateSet::default();

        execute(&resolver, &mut candidates, &filter);

        assert!(candidates.get(EntityID::new(1)).is_none());
        assert!(candidates.get(EntityID::new(2)).is_none());
    }

    #[test]
    fn attribute_not_equals_excludes_matching_value() {
        let resolver = resolver();
        let filter = filter("brand!=Acme");

        let mut candidates = CandidateSet::default();

        execute(&resolver, &mut candidates, &filter);

        assert!(candidates.get(EntityID::new(1)).is_none());
        assert!(candidates.get(EntityID::new(2)).is_some());
    }

    #[test]
    fn attribute_greater_than_matches_numeric_values() {
        let resolver = resolver();
        let filter = filter("price>10");

        let mut candidates = CandidateSet::default();

        execute(&resolver, &mut candidates, &filter);

        assert!(candidates.get(EntityID::new(3)).is_none());
        assert!(candidates.get(EntityID::new(4)).is_some());
    }

    #[test]
    fn attribute_less_than_or_equal_matches_numeric_values() {
        let resolver = resolver();
        let filter = filter("price<=5");

        let mut candidates = CandidateSet::default();

        execute(&resolver, &mut candidates, &filter);

        assert!(candidates.get(EntityID::new(3)).is_some());
        assert!(candidates.get(EntityID::new(4)).is_none());
    }

    #[test]
    fn attribute_filter_does_not_match_entity_without_attribute() {
        let resolver = resolver();
        let filter = filter("brand=Acme");

        let mut candidates = CandidateSet::default();

        execute(&resolver, &mut candidates, &filter);

        // Entity 3 has only price.
        assert!(candidates.get(EntityID::new(3)).is_none());
    }

    #[test]
    fn non_attribute_filter_is_ignored() {
        let resolver = resolver();

        let filter = Filter::Tag { tag: "food".into() };

        let mut candidates = CandidateSet::default();

        execute(&resolver, &mut candidates, &filter);

        assert!(candidates.get(EntityID::new(1)).is_none());
        assert!(candidates.get(EntityID::new(2)).is_none());
        assert!(candidates.get(EntityID::new(3)).is_none());
        assert!(candidates.get(EntityID::new(4)).is_none());
    }

    #[test]
    fn attribute_less_than_matches_numeric_values() {
        let resolver = resolver();
        let filter = filter("price<10");

        let mut candidates = CandidateSet::default();

        execute(&resolver, &mut candidates, &filter);

        assert!(candidates.get(EntityID::new(3)).is_some());
        assert!(candidates.get(EntityID::new(4)).is_none());
    }

    #[test]
    fn attribute_greater_than_or_equal_matches_numeric_values() {
        let resolver = resolver();
        let filter = filter("price>=20");

        let mut candidates = CandidateSet::default();

        execute(&resolver, &mut candidates, &filter);

        assert!(candidates.get(EntityID::new(3)).is_none());
        assert!(candidates.get(EntityID::new(4)).is_some());
    }

    #[test]
    fn attribute_comparison_does_not_match_incompatible_value_types() {
        let resolver = resolver();
        let filter = filter("price>abc");

        let mut candidates = CandidateSet::default();

        execute(&resolver, &mut candidates, &filter);

        assert!(candidates.get(EntityID::new(3)).is_none());
        assert!(candidates.get(EntityID::new(4)).is_none());
    }
}
