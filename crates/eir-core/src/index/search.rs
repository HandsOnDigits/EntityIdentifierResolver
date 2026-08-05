use crate::entity::{
    EntityDocument,
    types::{Alias, EntityID, RelationshipType, SourceID, TagID},
};

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct SearchHit {
    pub entity_id: EntityID,
    pub score: f32,
    pub sources: HashSet<SearchSource>,
    pub explanations: HashSet<SearchExplanation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SearchSource {
    ExactAlias,
    PrefixAlias,
    FuzzyAlias,
    Token,
    Tag,
    Attribute,
    AttributeValue,
    Relationship,
    Source,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchExplanation {
    ExactAlias {
        alias: Alias,
    },

    PrefixAlias {
        alias: Alias,
    },

    FuzzyAlias {
        alias: Alias,
    },

    Token {
        token: Alias,
    },

    Relationship {
        kind: RelationshipType,
        target: EntityID,
    },

    Tag {
        tag: TagID,
    },

    Attribute {
        term: Box<str>,
    },

    AttributeValue {
        key: Box<str>,
        value: Box<str>,
    },

    Source {
        source: SourceID,
    },
}

#[derive(Debug, Clone)]
pub struct SearchResult<'a> {
    pub entity: &'a EntityDocument,
    pub score: f32,
    pub sources: HashSet<SearchSource>,
    pub explanations: HashSet<SearchExplanation>,
}
