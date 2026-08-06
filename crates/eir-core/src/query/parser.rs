use crate::utils::normalize;

use super::{Query, QueryIntent};

impl Query {
    pub fn parse(input: &str) -> Self {
        let normalized = normalize(input);

        let tokens = normalized.split_whitespace().map(|x| x.into()).collect();

        let filters = super::filters::parse_filters(&normalized);

        let intent = if !filters.is_empty() {
            QueryIntent::Filter
        } else {
            QueryIntent::Lookup
        };

        Self {
            original: input.into(),
            normalized,
            tokens,
            intent,
            filters,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::QueryIntent;

    #[test]
    fn normal_query_is_lookup() {
        let query = Query::parse("FizzBerry Spark");

        assert_eq!(query.intent, QueryIntent::Lookup);

        assert!(query.filters.is_empty());
    }

    #[test]
    fn attribute_query_creates_filter() {
        let query = Query::parse("brand:coca");

        assert_eq!(query.intent, QueryIntent::Filter);

        assert_eq!(query.filters.len(), 1);
    }
}
