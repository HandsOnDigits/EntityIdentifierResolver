use crate::utils::normalize;

use super::{Query, QueryIntent, detect_intent};

impl Query {
    pub fn parse(input: &str) -> Self {
        let normalized = normalize(input);

        let tokens = normalized
            .split_whitespace()
            .map(Box::<str>::from)
            .collect();

        let mut query = Self {
            original: input.into(),
            normalized,
            tokens,
            intent: QueryIntent::Unknown,
            filters: Vec::new(),
        };

        query.intent = detect_intent(&query);

        query
    }
}
