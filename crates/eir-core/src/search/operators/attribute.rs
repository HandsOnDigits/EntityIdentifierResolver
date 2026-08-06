use crate::{
    query::Filter,
    search::{context::SearchContext, signal::Signal},
};

pub fn execute(ctx: &mut SearchContext) {
    for filter in &ctx.query.filters {
        let Filter::Attribute { key, value } = filter else {
            continue;
        };

        for entity in ctx.resolver.attribute_lookup(key, value) {
            ctx.candidates.add_signal(entity, Signal::Property);
        }
    }
}
