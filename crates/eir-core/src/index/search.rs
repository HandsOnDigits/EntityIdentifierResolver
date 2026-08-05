use crate::entity::{
    EntityDocument,
    types::{EntityID, EntityName, PropertyID, RelationshipType, SourceID, TagID},
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
    Property,
    Relationship,
    Source,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchExplanation {
    ExactAlias {
        alias: EntityName,
    },

    PrefixAlias {
        alias: EntityName,
    },

    FuzzyAlias {
        alias: EntityName,
    },

    Token {
        token: EntityName,
    },

    Relationship {
        kind: RelationshipType,
        target: EntityID,
    },

    Tag {
        tag: TagID,
    },

    Property {
        property: PropertyID,
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
