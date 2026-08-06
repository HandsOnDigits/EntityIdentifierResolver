use super::{
    context::SearchContext,
    operators,
    planner::{SearchPlan, SearchStage},
};

pub struct SearchExecutor;

impl SearchExecutor {
    pub fn execute(plan: &SearchPlan, ctx: &mut SearchContext) {
        for stage in &plan.stages {
            match stage {
                SearchStage::ExactAlias => operators::exact_alias::execute(ctx),

                SearchStage::PrefixAlias => operators::prefix_alias::execute(ctx),

                SearchStage::FuzzyAlias { distance } => {
                    operators::fuzzy_alias::execute(ctx, *distance)
                }

                SearchStage::Token => operators::token::execute(ctx),

                SearchStage::Tag => operators::tag::execute(ctx),

                SearchStage::Attribute => operators::attribute::execute(ctx),

                SearchStage::Relationship => operators::relationship::execute(ctx),
            }
        }
    }
}
