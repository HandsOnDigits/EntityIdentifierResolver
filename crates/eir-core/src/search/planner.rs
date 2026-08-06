use crate::query::{Query, QueryIntent};

#[derive(Debug, Clone, Copy)]
pub enum SearchStage {
    ExactAlias,
    PrefixAlias,
    FuzzyAlias { distance: usize },
    Token,
    Tag,
    Property,
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
            ],

            QueryIntent::Browse => vec![SearchStage::Tag, SearchStage::Token],

            QueryIntent::Filter => vec![SearchStage::Property, SearchStage::Tag],

            QueryIntent::Relationship => vec![SearchStage::Relationship, SearchStage::Token],

            QueryIntent::Similar => vec![SearchStage::Tag, SearchStage::Property],

            QueryIntent::Unknown => vec![SearchStage::Token],
        };

        Self { stages }
    }
}
