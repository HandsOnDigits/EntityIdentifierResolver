use crate::search::{context::SearchContext, signal::Signal};

pub fn execute(ctx: &mut SearchContext) {
    for token in &ctx.query.tokens {
        for &target in ctx.resolver.resolve(token) {
            for (entity, _) in ctx.resolver.entities_related_to(target) {
                ctx.candidates.add_signal(entity, Signal::Relationship);
            }
        }
    }
}
