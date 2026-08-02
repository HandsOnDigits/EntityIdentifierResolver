use std::collections::HashMap;

use crate::entity::{EntityDocument, types::EntityID};

use super::{
    alias::AliasIndex, bk_tree::BKTreeIndex, inverted::InvertedIndex, trie::TrieIndex,
    utils::normalize,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchSource {
    Alias,
    Prefix,
    Fuzzy,
    Token,
}

#[derive(Debug, Clone)]
pub struct SearchResult<'a> {
    pub entity: &'a EntityDocument,
    pub score: f32,
    pub source: SearchSource,
}

#[derive(Default)]
pub struct Resolver {
    documents: HashMap<EntityID, EntityDocument>,
    alias: AliasIndex,
    trie: TrieIndex,
    fuzzy: BKTreeIndex,
    inverted: InvertedIndex,
}

impl Resolver {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers an entity document.
    pub fn add(&mut self, document: EntityDocument) {
        let id = document.id;

        for alias in &document.aliases {
            let normalized = normalize(alias);

            self.alias.insert(normalized.clone(), id);
            self.trie.insert(&normalized, id);
            self.fuzzy.insert(&normalized, id);

            for token in alias.split_whitespace() {
                self.inverted.insert(&normalize(token), id);
            }
        }

        for tag in &document.tags {
            self.inverted.insert(&normalize(tag), id);
        }

        for property in &document.properties {
            self.inverted.insert(&normalize(property), id);
        }

        self.documents.insert(id, document);
    }

    pub fn get(&self, id: EntityID) -> Option<&EntityDocument> {
        self.documents.get(&id)
    }

    /// Exact alias lookup.
    pub fn resolve(&self, alias: &str) -> Option<EntityID> {
        self.alias.resolve(alias)
    }

    /// Prefix search.
    pub fn prefix(&self, prefix: &str) -> Vec<EntityID> {
        self.trie.prefix(prefix)
    }

    /// Fuzzy search.
    pub fn fuzzy(&self, query: &str, distance: usize) -> Vec<EntityID> {
        self.fuzzy.search(query, distance)
    }

    /// Token lookup.
    pub fn lookup(&self, term: &str) -> Vec<EntityID> {
        self.inverted.lookup(term)
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

        if let Some(id) = self.resolve(query) {
            add_hit(id, 1.0, SearchSource::Alias);
        }

        for id in self.prefix(query) {
            add_hit(id, 0.8, SearchSource::Prefix);
        }

        for id in self.fuzzy(query, 1) {
            add_hit(id, 0.6, SearchSource::Fuzzy);
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
