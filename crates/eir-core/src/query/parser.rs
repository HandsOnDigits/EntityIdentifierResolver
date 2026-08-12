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
    use crate::{
        entity::prelude::types::Value,
        query::{
            QueryIntent,
            filters::{ComparisonOp, Filter},
        },
    };

    #[test]
    fn normal_query_is_lookup() {
        let query = Query::parse("FizzBerry Spark");

        assert_eq!(query.intent, QueryIntent::Lookup);
        assert!(query.filters.is_empty());
    }

    #[test]
    fn attribute_query_creates_filter() {
        let query = Query::parse("brand=Acme");

        assert_eq!(query.intent, QueryIntent::Filter);
        assert_eq!(query.filters.len(), 1);

        match &query.filters[0] {
            Filter::Attribute { key, op, value } => {
                assert_eq!(key.as_ref(), "brand");
                assert_eq!(*op, ComparisonOp::Eq);
                assert_eq!(value, &Value::String("acme".into()));
            }
            _ => panic!("expected attribute filter"),
        }
    }
}
