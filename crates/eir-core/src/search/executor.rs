use super::{
    candidate::CandidateSet,
    context::SearchContext,
    operators,
    planner::{SearchPlan, SearchStage},
};

use crate::{
    index::Resolver,
    query::{Filter, FilterExpr},
};

pub struct SearchExecutor;

impl SearchExecutor {
    pub fn execute(plan: &SearchPlan, ctx: &mut SearchContext) {
        for stage in &plan.stages {
            match stage {
                SearchStage::ExactAlias => {
                    operators::exact_alias::execute(ctx);
                }

                SearchStage::PrefixAlias => {
                    operators::prefix_alias::execute(ctx);
                }

                SearchStage::FuzzyAlias { distance } => {
                    operators::fuzzy_alias::execute(ctx, *distance);
                }

                SearchStage::Token => {
                    operators::token::execute(ctx);
                }

                SearchStage::Tag => {
                    operators::tag::execute(ctx);
                }

                SearchStage::Attribute | SearchStage::Relationship => {
                    if let Some(expr) = ctx.query.filter.as_ref() {
                        let candidates = Self::evaluate_filter_expr(&ctx.resolver, expr);

                        ctx.candidates = candidates;
                    }
                }
            }
        }
    }

    fn evaluate_filter_expr(resolver: &Resolver, expr: &FilterExpr) -> CandidateSet {
        match expr {
            FilterExpr::Filter(filter) => {
                let mut candidates = CandidateSet::default();

                match filter {
                    Filter::Attribute { .. } => {
                        operators::attribute::execute(resolver, &mut candidates, filter);
                    }

                    Filter::Relationship { .. } => {
                        operators::relationship::execute(resolver, &mut candidates, filter);
                    }

                    _ => {}
                }

                candidates
            }

            FilterExpr::And(left, right) => {
                let mut left = Self::evaluate_filter_expr(resolver, left);
                let right = Self::evaluate_filter_expr(resolver, right);

                left.intersect_with(&right);
                left
            }

            FilterExpr::Or(left, right) => {
                let mut left = Self::evaluate_filter_expr(resolver, left);
                let right = Self::evaluate_filter_expr(resolver, right);

                left.union_with(right);
                left
            }

            FilterExpr::Not(inner) => {
                let inner = Self::evaluate_filter_expr(resolver, inner);

                let result = CandidateSet::default();

                // NOT needs the resolver's complete entity universe.
                // Leave this unsupported until Resolver exposes that.
                let _ = inner;

                result
            }
        }
    }
}
