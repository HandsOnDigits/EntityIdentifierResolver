use crate::{entity::prelude::types::Value, query::Query};

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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum FilterExpr {
    Filter(Filter),
    And(Box<FilterExpr>, Box<FilterExpr>),
    Or(Box<FilterExpr>, Box<FilterExpr>),
    Not(Box<FilterExpr>),
}

fn parse_single_filter(part: &str) -> Option<Filter> {
    if let Some(parts) = part.strip_prefix("relation:") {
        if let Some((kind, target)) = parts.split_once(':') {
            return Some(Filter::Relationship {
                kind: kind.into(),
                target: target.into(),
            });
        }

        return None;
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
                return Some(Filter::Attribute {
                    key: key.into(),
                    op,
                    value: Query::parse_value(value),
                });
            }

            return None;
        }
    }

    None
}

pub fn parse_filters(query: &str) -> Vec<Filter> {
    query
        .split_whitespace()
        .filter_map(parse_single_filter)
        .collect()
}

pub fn parse_filter_expr(query: &str) -> Option<FilterExpr> {
    let mut parser = FilterParser::new(query);

    let expr = parser.parse_or()?;

    parser.skip_whitespace();

    if parser.is_at_end() { Some(expr) } else { None }
}

struct FilterParser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> FilterParser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    fn parse_or(&mut self) -> Option<FilterExpr> {
        let mut expr = self.parse_and()?;

        loop {
            self.skip_whitespace();

            if !self.consume('|') {
                break;
            }

            let rhs = self.parse_and()?;

            expr = FilterExpr::Or(Box::new(expr), Box::new(rhs));
        }

        Some(expr)
    }

    fn parse_and(&mut self) -> Option<FilterExpr> {
        let mut expr = self.parse_unary()?;

        loop {
            self.skip_whitespace();

            if !self.consume('&') {
                break;
            }

            let rhs = self.parse_unary()?;

            expr = FilterExpr::And(Box::new(expr), Box::new(rhs));
        }

        Some(expr)
    }

    fn parse_unary(&mut self) -> Option<FilterExpr> {
        self.skip_whitespace();

        if self.consume('!') {
            let expr = self.parse_unary()?;
            return Some(FilterExpr::Not(Box::new(expr)));
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Option<FilterExpr> {
        self.skip_whitespace();

        if self.consume('(') {
            let expr = self.parse_or()?;

            self.skip_whitespace();

            if !self.consume(')') {
                return None;
            }

            return Some(expr);
        }

        self.parse_filter().map(FilterExpr::Filter)
    }

    fn parse_filter(&mut self) -> Option<Filter> {
        self.skip_whitespace();

        let start = self.position;

        while let Some(ch) = self.peek() {
            if ch.is_whitespace() || matches!(ch, '&' | '|' | '(' | ')') {
                break;
            }

            self.position += ch.len_utf8();
        }

        if self.position == start {
            return None;
        }

        let token = &self.input[start..self.position];

        parse_single_filter(token)
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.position += self.peek().unwrap().len_utf8();
        }
    }

    fn consume(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.position += expected.len_utf8();
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
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

    #[test]
    fn comparison_operators_match_values() {
        let ten = Value::Integer(10);
        let twenty = Value::Integer(20);

        assert!(ComparisonOp::Eq.matches(&ten, &ten));
        assert!(!ComparisonOp::Eq.matches(&ten, &twenty));

        assert!(!ComparisonOp::NotEq.matches(&ten, &ten));
        assert!(ComparisonOp::NotEq.matches(&ten, &twenty));

        assert!(ComparisonOp::Lt.matches(&ten, &twenty));
        assert!(!ComparisonOp::Lt.matches(&twenty, &ten));

        assert!(ComparisonOp::LtEq.matches(&ten, &ten));
        assert!(ComparisonOp::LtEq.matches(&ten, &twenty));

        assert!(ComparisonOp::Gt.matches(&twenty, &ten));
        assert!(!ComparisonOp::Gt.matches(&ten, &twenty));

        assert!(ComparisonOp::GtEq.matches(&twenty, &ten));
        assert!(ComparisonOp::GtEq.matches(&ten, &ten));
    }

    #[test]
    fn comparison_operators_do_not_match_incompatible_values() {
        let string = Value::String("10".into());
        let integer = Value::Integer(10);

        assert!(!ComparisonOp::Lt.matches(&string, &integer));
        assert!(!ComparisonOp::Gt.matches(&string, &integer));
        assert!(!ComparisonOp::LtEq.matches(&string, &integer));
        assert!(!ComparisonOp::GtEq.matches(&string, &integer));
    }
}
