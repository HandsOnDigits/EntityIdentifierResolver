use crate::search::{context::SearchContext, signal::Signal};

pub fn execute(ctx: &mut SearchContext) {
    for token in &ctx.query.tokens {
        let Some(tag_id) = ctx.resolver.tag_search(token) else {
            continue;
        };

        for entity in ctx.resolver.entities_with_tag(tag_id) {
            ctx.candidates.add_signal(entity, Signal::Tag);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        entity::prelude::types::{EntityID, TagID},
        index::Resolver,
        query::Query,
        search::{CandidateSet, context::SearchContext},
        test_utils::test_entity_with_tag,
    };

    #[test]
    fn tag_adds_candidate() {
        let mut resolver = Resolver::default();

        let tag_id = TagID::new(1);
        let entity_id = EntityID::new(1);

        resolver.register_tag(tag_id, "drink".into());

        resolver.add(test_entity_with_tag(entity_id, "FizzBerry Spark", tag_id));

        let query = Query::parse("drink");

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
                .contains(&Signal::Tag)
        );
    }
}
