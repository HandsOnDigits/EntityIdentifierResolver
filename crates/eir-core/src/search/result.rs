use crate::entity::prelude::{
    EntityDocument,
    types::{Alias, EntityID, RelationshipTypeID, SourceID, TagID},
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

    AttributeKey,
    AttributeValue,
    AttributeKeyValue,

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
        kind: RelationshipTypeID,
        target: EntityID,
    },

    Tag {
        tag: TagID,
    },

    AttributeKey {
        term: Box<str>,
    },

    AttributeValue {
        term: Box<str>,
    },

    AttributeKeyValue {
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
