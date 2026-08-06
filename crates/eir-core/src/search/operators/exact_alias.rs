use crate::search::{context::SearchContext, signal::Signal};

pub fn execute(ctx: &mut SearchContext) {
    let entities = ctx.resolver.resolve(&ctx.query.normalized);

    for entity_id in entities {
        ctx.candidates.add_signal(*entity_id, Signal::ExactAlias);
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
    fn exact_alias_adds_candidate() {
        let mut resolver = Resolver::default();

        resolver.add(test_entity(1, "FizzBerry Spark"));

        let query = Query::parse("fizzberry spark");

        let mut ctx = SearchContext {
            database: None,
            resolver,
            query: &query,
            candidates: CandidateSet::default(),
        };

        execute(&mut ctx);

        let candidate = ctx.candidates.get(1).unwrap();

        assert!(candidate.signals.contains(&Signal::ExactAlias));
    }
}
