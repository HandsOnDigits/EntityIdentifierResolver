use crate::search::{context::SearchContext, signal::Signal};

pub fn execute(ctx: &mut SearchContext, distance: usize) {
    for token in &ctx.query.tokens {
        for entity_id in ctx.resolver.fuzzy(token, distance) {
            ctx.candidates.add_signal(entity_id, Signal::FuzzyAlias);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        entity::prelude::types::EntityID,
        index::Resolver,
        query::Query,
        search::{CandidateSet, context::SearchContext},
        test::test_entity,
    };

    #[test]
    fn fuzzy_alias_adds_candidate() {
        let entity_id = EntityID(1);

        let mut resolver = Resolver::default();

        resolver.add(test_entity(entity_id, "FizzBerry"));

        let query = Query::parse("FizBerry");

        let mut ctx = SearchContext {
            database: None,
            resolver,
            query: &query,
            candidates: CandidateSet::default(),
        };

        execute(&mut ctx, 1);

        let candidate = ctx.candidates.get(entity_id).unwrap();

        assert!(candidate.signals.contains(&Signal::FuzzyAlias));
    }
}
