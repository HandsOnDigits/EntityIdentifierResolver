use crate::utils::normalize;

use super::{Filter, Query, QueryIntent};

impl Query {
    pub fn parse(input: &str) -> Self {
        let normalized = normalize(input);

        let mut tokens = Vec::new();
        let mut filters = Vec::new();

        for part in normalized.split_whitespace() {
            if let Some((key, value)) = part.split_once(':') {
                filters.push(Filter::Attribute {
                    key: key.into(),
                    value: value.into(),
                });
            } else {
                tokens.push(part.into());
            }
        }

        Self {
            original: input.into(),
            normalized,
            tokens,
            intent: QueryIntent::Unknown,
            filters,
        }
    }
}
