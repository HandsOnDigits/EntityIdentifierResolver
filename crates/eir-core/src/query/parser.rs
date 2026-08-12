use crate::{
    entity::prelude::types::Value,
    query::{Query, QueryIntent, filters::parse_filter_expr},
    utils::normalize,
};

impl Query {
    /// Parses a raw query string into a structured `Query` instance.
    pub fn parse(input: &str) -> Self {
        let original = input.into();
        let normalized_str = normalize(input);
        let normalized = normalized_str.clone().into();
        let filter = parse_filter_expr(input);

        let tokens = normalized_str
            .split_whitespace()
            .map(|s| s.into())
            .collect();

        let intent = if filter.is_some() {
            QueryIntent::Filter
        } else if normalized_str.contains("similar")
            || normalized_str.contains("like")
            || normalized_str.contains("alternative")
        {
            QueryIntent::Similar
        } else if normalized_str.contains("by")
            || normalized_str.contains("from")
            || normalized_str.contains("made")
        {
            QueryIntent::Relationship
        } else {
            QueryIntent::Lookup
        };

        Self {
            original,
            normalized,
            tokens,
            intent,
            filter,
        }
    }

    pub(crate) fn parse_value(value: &str) -> Value {
        let normalized = normalize(value);

        if let Ok(val) = normalized.parse::<i64>() {
            return Value::Integer(val);
        }

        match normalized.as_ref() {
            "true" => Value::Boolean(true),
            "false" => Value::Boolean(false),
            _ => Value::String(normalized),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        entity::prelude::types::Value,
        query::{
            FilterExpr, QueryIntent,
            filters::{ComparisonOp, Filter},
        },
    };

    #[test]
    fn normal_query_is_lookup() {
        let query = Query::parse("FizzBerry Spark");

        assert_eq!(query.intent, QueryIntent::Lookup);
        assert!(query.filter.is_none());
    }

    #[test]
    fn attribute_query_creates_filter() {
        let query = Query::parse("brand=Acme");

        assert_eq!(query.intent, QueryIntent::Filter);

        match query.filter.as_ref() {
            Some(FilterExpr::Filter(Filter::Attribute { key, op, value })) => {
                assert_eq!(key.as_ref(), "brand");
                assert_eq!(*op, ComparisonOp::Eq);
                assert_eq!(value, &Value::String("acme".into()));
            }
            _ => panic!("expected attribute filter"),
        }
    }

    #[test]
    fn attribute_filter_creates_filter_expr() {
        let query = Query::parse("brand=Acme");

        assert_eq!(query.intent, QueryIntent::Filter);

        match query.filter.as_ref().unwrap() {
            FilterExpr::Filter(Filter::Attribute { key, op, value }) => {
                assert_eq!(key.as_ref(), "brand");
                assert_eq!(*op, ComparisonOp::Eq);
                assert_eq!(value, &Value::String("acme".into()));
            }
            _ => panic!("expected attribute filter"),
        }
    }

    #[test]
    fn and_creates_and_expression() {
        let expr = parse_filter_expr("brand=Acme & price>=10").unwrap();
        assert!(matches!(expr, FilterExpr::And(_, _)));
    }

    #[test]
    fn or_creates_or_expression() {
        let expr = parse_filter_expr("brand=Acme | brand=Other").unwrap();
        assert!(matches!(expr, FilterExpr::Or(_, _)));
    }

    #[test]
    fn mixed_and_or_creates_expected_structure() {
        let expr = parse_filter_expr("brand=Acme & price>=10 | brand=Other").unwrap();

        match expr {
            FilterExpr::Or(left, right) => {
                assert!(matches!(*right, FilterExpr::Filter(_)));
                match *left {
                    FilterExpr::And(_, _) => {}
                    _ => panic!("expected AND expression inside OR"),
                }
            }
            _ => panic!("expected OR expression containing AND"),
        }
    }

    #[test]
    fn parentheses_change_boolean_grouping() {
        let expr = parse_filter_expr("brand=Acme & (price>=10 | price<=5)").unwrap();

        match expr {
            FilterExpr::And(left, right) => {
                assert!(matches!(*left, FilterExpr::Filter(_)));
                match *right {
                    FilterExpr::Or(_, _) => {}
                    _ => panic!("expected parenthesized OR expression"),
                }
            }
            _ => panic!("expected AND expression"),
        }
    }
}
