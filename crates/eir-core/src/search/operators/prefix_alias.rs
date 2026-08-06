use crate::search::{context::SearchContext, signal::Signal};

pub fn execute(ctx: &mut SearchContext) {
    for token in &ctx.query.tokens {
        for entity_id in ctx.resolver.prefix(token) {
            ctx.candidates.add_signal(entity_id, Signal::PrefixAlias);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        index::Resolver,
        query::Query,
        search::{CandidateSet, context::SearchContext},
    };

    use crate::search::test_utils::test_entity;

    #[test]
    fn prefix_alias_adds_candidate() {
        let mut resolver = Resolver::default();

        resolver.add(test_entity(1, "FizzBerry Spark"));

        let query = Query::parse("Fizz");

        let mut ctx = SearchContext {
            database: None,
            resolver,
            query: &query,
            candidates: CandidateSet::default(),
        };

        execute(&mut ctx);

        let candidate = ctx.candidates.get(1).unwrap();

        assert!(candidate.signals.contains(&Signal::PrefixAlias));
    }
}
