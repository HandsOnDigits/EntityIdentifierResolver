use crate::search::{context::SearchContext, signal::Signal};

pub fn execute(ctx: &mut SearchContext) {
    for token in &ctx.query.tokens {
        for entity_id in ctx.resolver.lookup(token) {
            ctx.candidates.add_signal(entity_id, Signal::Token);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        index::Resolver,
        query::Query,
        search::{CandidateSet, context::SearchContext, test_utils::test_entity},
    };

    #[test]
    fn token_adds_candidate() {
        let mut resolver = Resolver::default();

        resolver.add(test_entity(1, "FizzBerry Spark"));

        let query = Query::parse("spark");

        let mut ctx = SearchContext {
            database: None,
            resolver,
            query: &query,
            candidates: CandidateSet::default(),
        };

        execute(&mut ctx);

        assert!(
            ctx.candidates
                .get(1)
                .unwrap()
                .signals
                .contains(&Signal::Token)
        );
    }
}
