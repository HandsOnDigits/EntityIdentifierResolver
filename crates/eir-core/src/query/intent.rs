use super::Query;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryIntent {
    Lookup,
    Browse,
    Filter,
    Relationship,
    Similar,
    Unknown,
}

pub fn detect_intent(query: &Query) -> QueryIntent {
    let text = query.normalized.as_ref();

    if text.contains("similar") || text.contains("like") || text.contains("alternative") {
        return QueryIntent::Similar;
    }

    if text.contains("by") || text.contains("from") || text.contains("made") {
        return QueryIntent::Relationship;
    }

    if query.tokens.len() == 1 {
        return QueryIntent::Lookup;
    }

    QueryIntent::Browse
}
