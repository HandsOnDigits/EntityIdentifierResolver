use crate::query::{Filter, FilterExpr, Query, QueryIntent};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SearchStage {
    ExactAlias,
    PrefixAlias,
    FuzzyAlias { distance: usize },
    Token,
    Tag,
    Attribute,
    Relationship,
}

#[derive(Debug)]
pub struct SearchPlan {
    pub stages: Vec<SearchStage>,
}

impl SearchPlan {
    pub fn from_query(query: &Query) -> Self {
        let stages = match query.intent {
            QueryIntent::Lookup => vec![
                SearchStage::ExactAlias,
                SearchStage::PrefixAlias,
                SearchStage::FuzzyAlias { distance: 1 },
                SearchStage::Token,
            ],

            QueryIntent::Browse => vec![SearchStage::Tag, SearchStage::Token],

            QueryIntent::Filter => {
                let mut stages = Vec::new();

                if let Some(filter) = &query.filter {
                    Self::collect_stages(filter, &mut stages);
                }

                stages
            }

            QueryIntent::Relationship => {
                vec![SearchStage::Relationship, SearchStage::Token]
            }

            QueryIntent::Similar => {
                vec![SearchStage::Tag, SearchStage::Attribute]
            }

            QueryIntent::Tag => vec![SearchStage::Tag],

            QueryIntent::Unknown => vec![SearchStage::Token],
        };

        Self { stages }
    }

    fn collect_stages(expr: &FilterExpr, stages: &mut Vec<SearchStage>) {
        match expr {
            FilterExpr::Filter(filter) => match filter {
                Filter::Attribute { .. } => {
                    if !stages.contains(&SearchStage::Attribute) {
                        stages.push(SearchStage::Attribute);
                    }
                }

                Filter::Tag { .. } => {
                    if !stages.contains(&SearchStage::Tag) {
                        stages.push(SearchStage::Tag);
                    }
                }

                Filter::Relationship { .. } => {
                    if !stages.contains(&SearchStage::Relationship) {
                        stages.push(SearchStage::Relationship);
                    }
                }

                Filter::Source { .. } => {
                    // Add SearchStage::Source once you introduce it.
                }

                Filter::EntityType { .. } => {
                    // Add an entity-type stage once supported.
                }
            },

            FilterExpr::And(left, right) | FilterExpr::Or(left, right) => {
                Self::collect_stages(left, stages);
                Self::collect_stages(right, stages);
            }

            FilterExpr::Not(expr) => {
                Self::collect_stages(expr, stages);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::{Query, QueryIntent};

    #[test]
    fn lookup_creates_lookup_plan() {
        let query = Query {
            original: "FizzBerry".into(),
            normalized: "fizzberry".into(),
            tokens: vec!["fizzberry".into()],
            intent: QueryIntent::Lookup,
            filter: None,
        };

        let plan = SearchPlan::from_query(&query);

        assert!(plan.stages.contains(&SearchStage::Token));
        assert!(plan.stages.contains(&SearchStage::ExactAlias));
    }

    #[test]
    fn attribute_filter_creates_property_plan() {
        let query = Query::parse("price>=10");

        let plan = SearchPlan::from_query(&query);

        assert!(plan.stages.contains(&SearchStage::Attribute));
    }

    #[test]
    fn lookup_does_not_create_property_plan() {
        let query = Query::parse("FizzBerry");

        let plan = SearchPlan::from_query(&query);

        assert!(!plan.stages.contains(&SearchStage::Attribute));
    }
}
