#[cfg(test)]
mod tests {
    use crate::{
        entity::prelude::EntityDocument,
        entity::prelude::types::{AttributeKeyID, EntityID, Value},
        index::Resolver,
        query::Query,
        search::{CandidateSet, SearchExecutor, SearchPlan, context::SearchContext},
        test_utils::{test_entity, test_entity_with_attributes},
    };

    fn search_fixture() -> Resolver {
        let mut resolver = Resolver::default();

        let brand = AttributeKeyID::new(1);
        let price = AttributeKeyID::new(2);

        resolver.register_attribute(brand, "brand".into());
        resolver.register_attribute(price, "price".into());

        // 1: Acme, cheap ($5)
        resolver.add(test_entity_with_attributes(
            EntityID::new(1),
            "Acme Cheap Product",
            vec![
                (brand, Value::String("acme".into())),
                (price, Value::Integer(5)),
            ],
        ));

        // 2: Other, expensive ($20)
        resolver.add(test_entity_with_attributes(
            EntityID::new(2),
            "Other Expensive Product",
            vec![
                (brand, Value::String("other".into())),
                (price, Value::Integer(20)),
            ],
        ));

        // 3: Other, cheap ($5)
        resolver.add(test_entity_with_attributes(
            EntityID::new(3),
            "Other Cheap Product",
            vec![
                (brand, Value::String("other".into())),
                (price, Value::Integer(5)),
            ],
        ));

        // 4: Acme, expensive ($20)
        resolver.add(test_entity_with_attributes(
            EntityID::new(4),
            "Acme Expensive Product",
            vec![
                (brand, Value::String("acme".into())),
                (price, Value::Integer(20)),
            ],
        ));

        // 5: no attributes
        resolver.add(test_entity(EntityID::new(5), "No Attributes Product"));

        resolver
    }

    #[test]
    fn search_attribute_equality_end_to_end() {
        let resolver = search_fixture();
        let query = Query::parse("brand=Acme");

        let mut ctx = SearchContext {
            database: None,
            resolver,
            query: &query,
            candidates: CandidateSet::default(),
        };

        let plan = SearchPlan::from_query(&query);
        SearchExecutor::execute(&plan, &mut ctx);

        assert!(ctx.candidates.get(EntityID::new(1)).is_some());
        assert!(ctx.candidates.get(EntityID::new(4)).is_some());

        assert!(ctx.candidates.get(EntityID::new(2)).is_none());
        assert!(ctx.candidates.get(EntityID::new(3)).is_none());
        assert!(ctx.candidates.get(EntityID::new(5)).is_none());
    }

    #[test]
    fn search_numeric_attribute_comparison_end_to_end() {
        let resolver = search_fixture();
        let query = Query::parse("price>=10");

        let mut ctx = SearchContext {
            database: None,
            resolver,
            query: &query,
            candidates: CandidateSet::default(),
        };

        let plan = SearchPlan::from_query(&query);
        SearchExecutor::execute(&plan, &mut ctx);

        assert!(ctx.candidates.get(EntityID::new(2)).is_some());
        assert!(ctx.candidates.get(EntityID::new(4)).is_some());

        assert!(ctx.candidates.get(EntityID::new(1)).is_none());
        assert!(ctx.candidates.get(EntityID::new(3)).is_none());
        assert!(ctx.candidates.get(EntityID::new(5)).is_none());
    }

    #[test]
    fn search_and_requires_both_filters() {
        let resolver = search_fixture();

        let query = Query::parse("brand=Acme & price>=10");

        let mut ctx = SearchContext {
            database: None,
            resolver,
            query: &query,
            candidates: CandidateSet::default(),
        };

        let plan = SearchPlan::from_query(&query);
        SearchExecutor::execute(&plan, &mut ctx);

        // Acme + price >= 10
        assert!(ctx.candidates.get(EntityID::new(4)).is_some());

        // Acme, but too cheap.
        assert!(ctx.candidates.get(EntityID::new(1)).is_none());

        // Expensive, but not Acme.
        assert!(ctx.candidates.get(EntityID::new(2)).is_none());

        // Neither.
        assert!(ctx.candidates.get(EntityID::new(3)).is_none());

        // Missing attributes.
        assert!(ctx.candidates.get(EntityID::new(5)).is_none());
    }

    #[test]
    fn search_or_matches_either_filter() {
        let resolver = search_fixture();

        let query = Query::parse("brand=Acme | brand=Other");

        let mut ctx = SearchContext {
            database: None,
            resolver,
            query: &query,
            candidates: CandidateSet::default(),
        };

        let plan = SearchPlan::from_query(&query);
        SearchExecutor::execute(&plan, &mut ctx);

        assert!(ctx.candidates.get(EntityID::new(1)).is_some());
        assert!(ctx.candidates.get(EntityID::new(2)).is_some());
        assert!(ctx.candidates.get(EntityID::new(3)).is_some());
        assert!(ctx.candidates.get(EntityID::new(4)).is_some());

        assert!(ctx.candidates.get(EntityID::new(5)).is_none());
    }

    #[test]
    fn search_multiple_and_filters_require_all() {
        let resolver = search_fixture();

        let query = Query::parse("brand=Acme & price>=10 & price<=30");

        let mut ctx = SearchContext {
            database: None,
            resolver,
            query: &query,
            candidates: CandidateSet::default(),
        };

        let plan = SearchPlan::from_query(&query);
        SearchExecutor::execute(&plan, &mut ctx);

        // Acme and 10 <= price <= 30.
        assert!(ctx.candidates.get(EntityID::new(4)).is_some());

        assert!(ctx.candidates.get(EntityID::new(1)).is_none());
        assert!(ctx.candidates.get(EntityID::new(2)).is_none());
        assert!(ctx.candidates.get(EntityID::new(3)).is_none());
    }

    #[test]
    fn search_mixed_and_or_preserves_boolean_semantics() {
        let resolver = search_fixture();

        let query = Query::parse("brand=Acme & price>=10 | brand=Other");

        let mut ctx = SearchContext {
            database: None,
            resolver,
            query: &query,
            candidates: CandidateSet::default(),
        };

        let plan = SearchPlan::from_query(&query);
        SearchExecutor::execute(&plan, &mut ctx);

        // Left side:
        //
        // brand=Acme & price>=10
        //
        // matches entity 4.

        // Right side:
        //
        // brand=Other
        //
        // matches entities 2 and 3.

        assert!(ctx.candidates.get(EntityID::new(4)).is_some());
        assert!(ctx.candidates.get(EntityID::new(2)).is_some());
        assert!(ctx.candidates.get(EntityID::new(3)).is_some());

        // Entity 1 is Acme but price is only 5.
        assert!(ctx.candidates.get(EntityID::new(1)).is_none());

        assert!(ctx.candidates.get(EntityID::new(5)).is_none());
    }

    #[test]
    fn search_parentheses_change_boolean_grouping() {
        let resolver = search_fixture();

        let query = Query::parse("brand=Acme & (price>=10 | price<=5)");

        let mut ctx = SearchContext {
            database: None,
            resolver,
            query: &query,
            candidates: CandidateSet::default(),
        };

        let plan = SearchPlan::from_query(&query);
        SearchExecutor::execute(&plan, &mut ctx);

        // Both Acme products satisfy the price expression:
        //
        // Acme AND (price >= 10 OR price <= 5)

        assert!(ctx.candidates.get(EntityID::new(1)).is_some());
        assert!(ctx.candidates.get(EntityID::new(4)).is_some());

        assert!(ctx.candidates.get(EntityID::new(2)).is_none());
        assert!(ctx.candidates.get(EntityID::new(3)).is_none());
    }

    #[test]
    fn search_numeric_attribute_less_than_or_equal() {
        let resolver = search_fixture();

        let query = Query::parse("price<=5");

        let mut ctx = SearchContext {
            database: None,
            resolver,
            query: &query,
            candidates: CandidateSet::default(),
        };

        let plan = SearchPlan::from_query(&query);
        SearchExecutor::execute(&plan, &mut ctx);

        assert!(ctx.candidates.get(EntityID::new(1)).is_some());
        assert!(ctx.candidates.get(EntityID::new(3)).is_some());

        assert!(ctx.candidates.get(EntityID::new(2)).is_none());
        assert!(ctx.candidates.get(EntityID::new(4)).is_none());
    }

    #[test]
    fn search_numeric_attribute_greater_than() {
        let resolver = search_fixture();

        let query = Query::parse("price>5");

        let mut ctx = SearchContext {
            database: None,
            resolver,
            query: &query,
            candidates: CandidateSet::default(),
        };

        let plan = SearchPlan::from_query(&query);
        SearchExecutor::execute(&plan, &mut ctx);

        assert!(ctx.candidates.get(EntityID::new(2)).is_some());
        assert!(ctx.candidates.get(EntityID::new(4)).is_some());

        assert!(ctx.candidates.get(EntityID::new(1)).is_none());
        assert!(ctx.candidates.get(EntityID::new(3)).is_none());
    }
}
