use crate::entity::{EntityDocument, types::EntityID};

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SearchSource {
    ExactAlias,
    PrefixAlias,
    FuzzyAlias,
    Token,
    Tag,
    Property,
    Relationship,
    Source,
}

#[derive(Debug, Clone)]
pub struct SearchResult<'a> {
    pub entity: &'a EntityDocument,
    pub score: f32,
    pub sources: HashSet<SearchSource>,
}

#[derive(Debug, Clone)]
pub struct SearchHit {
    pub entity_id: EntityID,
    pub score: f32,
    pub sources: HashSet<SearchSource>,
}
