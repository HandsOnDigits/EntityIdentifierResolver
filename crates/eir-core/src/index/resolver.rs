use std::collections::HashMap;

use crate::{
    entity::EntityDocument,
    entity::types::{EntityID, PropertyID, SourceID, TagID},
    storage::PostingList,
};

use super::{
    alias::AliasIndex, bk_tree::BKTreeIndex, inverted::InvertedIndex, trie::TrieIndex,
    utils::normalize,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchSource {
    ExactAlias,
    PrefixAlias,
    FuzzyAlias,
    Token,
    Tag,
    Property,
    Relationship,
}

#[derive(Debug, Clone)]
pub struct SearchResult<'a> {
    pub entity: &'a EntityDocument,
    pub score: f32,
    pub source: SearchSource,
}

#[derive(Debug)]
pub struct Resolver {
    documents: HashMap<EntityID, EntityDocument>,

    alias: AliasIndex,
    trie: TrieIndex,
    fuzzy: BKTreeIndex,
    tokens: InvertedIndex,

    tags: PostingList<TagID>,
    properties: PostingList<PropertyID>,
    sources: PostingList<SourceID>,
}

impl Resolver {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers an entity document.
    pub fn add(&mut self, input: EntityDocument) {
        let id = input.id;

        for alias in &input.aliases {
            let normalized = normalize(alias);

            self.alias.insert(normalized.clone(), id);
            self.trie.insert(&normalized, id);
            self.fuzzy.insert(&normalized, id);

            for token in alias.split_whitespace() {
                self.tokens.insert(&normalize(token), id);
            }
        }

        for tag in &input.tags {
            self.tags.insert(*tag, id);
        }

        for property in &input.properties {
            self.properties.insert(*property, id);
        }

        for source in &input.sources {
            self.sources.insert(*source, id);
        }

        self.documents.insert(id, input);
    }

    pub fn get(&self, id: EntityID) -> Option<&EntityDocument> {
        self.documents.get(&id)
    }

    /// Exact alias lookup.
    pub fn resolve(&self, alias: &str) -> &[EntityID] {
        self.alias.resolve(alias).unwrap_or(&[])
    }

    /// Prefix search.
    pub fn prefix(&self, prefix: &str) -> Vec<EntityID> {
        self.trie.prefix(&normalize(prefix))
    }

    /// Fuzzy search.
    pub fn fuzzy(&self, query: &str, distance: usize) -> Vec<EntityID> {
        self.fuzzy.search(&normalize(query), distance)
    }

    /// Token lookup.
    pub fn lookup(&self, term: &str) -> Vec<EntityID> {
        self.tokens.lookup(&normalize(term))
    }

    /// High-level search.
    pub fn search<'a>(&'a self, query: &str) -> Vec<SearchResult<'a>> {
        #[derive(Debug, Clone, Copy)]
        struct Hit {
            score: f32,
            source: SearchSource,
        }

        let mut merged: HashMap<EntityID, Hit> = HashMap::new();

        let mut add_hit = |id: EntityID, score: f32, source: SearchSource| {
            merged
                .entry(id)
                .and_modify(|existing| {
                    if score > existing.score {
                        *existing = Hit { score, source };
                    }
                })
                .or_insert(Hit { score, source });
        };

        for id in self.resolve(query) {
            add_hit(*id, 1.0, SearchSource::ExactAlias);
        }

        for id in self.prefix(query) {
            add_hit(id, 0.8, SearchSource::PrefixAlias);
        }

        for id in self.fuzzy(query, 1) {
            add_hit(id, 0.6, SearchSource::FuzzyAlias);
        }

        for id in self.lookup(query) {
            add_hit(id, 0.5, SearchSource::Token);
        }

        let mut results: Vec<SearchResult<'a>> = merged
            .into_iter()
            .filter_map(|(id, hit)| {
                self.documents.get(&id).map(|entity| SearchResult {
                    entity,
                    score: hit.score,
                    source: hit.source,
                })
            })
            .collect();

        results.sort_by(|a, b| b.score.total_cmp(&a.score));
        results
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self {
            documents: HashMap::new(),

            alias: AliasIndex::default(),
            trie: TrieIndex::default(),
            fuzzy: BKTreeIndex::default(),
            tokens: InvertedIndex::default(),

            tags: PostingList::<TagID>::default(),
            properties: PostingList::<PropertyID>::default(),
            sources: PostingList::<SourceID>::default(),
        }
    }
}
