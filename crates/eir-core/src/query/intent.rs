use super::Query;

#[derive(Debug, Clone, PartialEq)]
pub enum QueryIntent {
    Unknown,

    /// Normal entity lookup
    Lookup,

    /// Browse categories/tags
    Browse,

    /// Find similar entities
    Similar,

    /// key:value filtering
    Filter,

    /// Relationship queries
    Relationship,

    /// Explicit tag queries
    Tag,
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
