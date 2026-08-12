use crate::{entity::prelude::types::Value, utils::normalize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOp {
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Contains,
    StartsWith,
    EndsWith,
    In,
}

impl ComparisonOp {
    pub fn matches(self, actual: &Value, expected: &Value) -> bool {
        match self {
            Self::Eq => actual.matches(expected),

            Self::NotEq => !actual.matches(expected),

            Self::Lt => actual
                .compare(expected)
                .is_some_and(|ordering| ordering == std::cmp::Ordering::Less),

            Self::LtEq => actual
                .compare(expected)
                .is_some_and(|ordering| ordering != std::cmp::Ordering::Greater),

            Self::Gt => actual
                .compare(expected)
                .is_some_and(|ordering| ordering == std::cmp::Ordering::Greater),

            Self::GtEq => actual
                .compare(expected)
                .is_some_and(|ordering| ordering != std::cmp::Ordering::Less),

            Self::Contains | Self::StartsWith | Self::EndsWith | Self::In => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Filter {
    Attribute {
        key: Box<str>,
        op: ComparisonOp,
        value: Value,
    },

    Tag {
        tag: Box<str>,
    },

    Source {
        source: Box<str>,
    },

    Relationship {
        kind: Box<str>,
        target: Box<str>,
    },

    EntityType {
        kind: Box<str>,
    },
}

pub fn parse_filters(query: &str) -> Vec<Filter> {
    let mut filters = Vec::new();

    for part in query.split_whitespace() {
        if let Some(parts) = part.strip_prefix("relation:") {
            if let Some((kind, target)) = parts.split_once(':') {
                filters.push(Filter::Relationship {
                    kind: kind.into(),
                    target: target.into(),
                });
            }

            continue;
        }

        for (operator, op) in [
            (">=", ComparisonOp::GtEq),
            ("<=", ComparisonOp::LtEq),
            ("!=", ComparisonOp::NotEq),
            ("=", ComparisonOp::Eq),
            (">", ComparisonOp::Gt),
            ("<", ComparisonOp::Lt),
        ] {
            if let Some((key, value)) = part.split_once(operator) {
                if !key.is_empty() && !value.is_empty() {
                    filters.push(Filter::Attribute {
                        key: key.into(),
                        op,
                        value: Value::String(normalize(value)),
                    });
                }

                break;
            }
        }
    }

    filters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_attribute_equality() {
        let filters = parse_filters("brand=Acme");

        assert_eq!(filters.len(), 1);

        match &filters[0] {
            Filter::Attribute { key, op, value } => {
                assert_eq!(key.as_ref(), "brand");
                assert_eq!(*op, ComparisonOp::Eq);
                assert_eq!(value, &Value::String("acme".into()));
            }
            _ => panic!("expected attribute filter"),
        }
    }

    #[test]
    fn parses_attribute_comparison_operators() {
        let filters = parse_filters("price=10 price!=20 price<30 price<=40 price>50 price>=60");

        assert_eq!(filters.len(), 6);

        let operators = filters
            .iter()
            .map(|filter| match filter {
                Filter::Attribute { op, .. } => *op,
                _ => panic!("expected attribute filter"),
            })
            .collect::<Vec<_>>();

        assert_eq!(
            operators,
            vec![
                ComparisonOp::Eq,
                ComparisonOp::NotEq,
                ComparisonOp::Lt,
                ComparisonOp::LtEq,
                ComparisonOp::Gt,
                ComparisonOp::GtEq,
            ]
        );
    }

    #[test]
    fn parses_relationship_separately() {
        let filters = parse_filters("relation:made_by:Acme");

        assert_eq!(filters.len(), 1);

        assert!(matches!(
            &filters[0],
            Filter::Relationship { kind, target }
                if kind.as_ref() == "made_by"
                    && target.as_ref() == "Acme"
        ));
    }

    #[test]
    fn unsupported_attribute_operator_does_not_use_exact_lookup() {
        let filters = parse_filters("brand!=Acme");

        let Filter::Attribute { op, .. } = &filters[0] else {
            panic!("expected attribute filter");
        };

        assert_eq!(*op, ComparisonOp::NotEq);

        // Once the operator executor is wired into a SearchContext:
        // assert!(results.is_empty());
    }
}
