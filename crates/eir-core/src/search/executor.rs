use crate::entity::{EntityDocument, types::EntityID};

use super::{
    context::SearchContext,
    planner::{SearchPlan, SearchStage},
    signal::Signal,
};

pub struct SearchExecutor;

impl SearchExecutor {
    pub fn execute(plan: &SearchPlan, ctx: &mut SearchContext) {
        for stage in &plan.stages {
            match stage {
                SearchStage::ExactAlias => {
                    Self::execute_exact_alias(ctx);
                }

                SearchStage::PrefixAlias => {
                    Self::execute_prefix_alias(ctx);
                }

                SearchStage::FuzzyAlias { distance } => {
                    Self::execute_fuzzy_alias(ctx, *distance);
                }

                SearchStage::Token => {
                    Self::execute_token(ctx);
                }

                SearchStage::Tag => {
                    Self::execute_tag(ctx);
                }

                SearchStage::Property => {
                    Self::execute_property(ctx);
                }

                SearchStage::Relationship => {
                    Self::execute_relationship(ctx);
                }
            }
        }
    }

    fn execute_exact_alias(ctx: &mut SearchContext) {
        for token in &ctx.query.tokens {
            let entities = ctx.resolver.resolve(token);

            for entity_id in entities {
                ctx.candidates.add_signal(*entity_id, Signal::ExactAlias);
            }
        }
    }

    fn execute_prefix_alias(ctx: &mut SearchContext) {
        for token in &ctx.query.tokens {
            let entities = ctx.resolver.prefix(token);

            for entity_id in entities {
                ctx.candidates.add_signal(entity_id, Signal::PrefixAlias);
            }
        }
    }

    fn execute_fuzzy_alias(ctx: &mut SearchContext, distance: usize) {
        for token in &ctx.query.tokens {
            let entities = ctx.resolver.fuzzy(token, distance);

            for entity_id in entities {
                ctx.candidates.add_signal(entity_id, Signal::FuzzyAlias);
            }
        }
    }

    fn execute_token(ctx: &mut SearchContext) {
        for token in &ctx.query.tokens {
            let entities = ctx.resolver.lookup(token);

            for entity_id in entities {
                ctx.candidates.add_signal(entity_id, Signal::Token);
            }
        }
    }

    fn execute_tag(_ctx: &mut SearchContext) {}

    fn execute_property(_ctx: &mut SearchContext) {}

    fn execute_relationship(_ctx: &mut SearchContext) {}
}

