use crate::query::{Query, QueryIntent};

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

            QueryIntent::Filter => vec![SearchStage::Attribute, SearchStage::Tag],

            QueryIntent::Relationship => vec![SearchStage::Relationship, SearchStage::Token],

            QueryIntent::Similar => vec![SearchStage::Tag, SearchStage::Attribute],

            QueryIntent::Tag => vec![SearchStage::Tag],

            QueryIntent::Unknown => vec![SearchStage::Token],
        };

        Self { stages }
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
            filters: Vec::new(),
        };

        let plan = SearchPlan::from_query(&query);

        assert!(plan.stages.contains(&SearchStage::Token));
        assert!(plan.stages.contains(&SearchStage::ExactAlias));
    }

    #[test]
    fn tag_creates_tag_plan() {
        let query = Query {
            original: "tag:drink".into(),
            normalized: "tag:drink".into(),
            tokens: vec!["drink".into()],
            intent: QueryIntent::Tag,
            filters: Vec::new(),
        };

        let plan = SearchPlan::from_query(&query);

        assert_eq!(plan.stages, vec![SearchStage::Tag]);
    }
}
