use std::collections::HashMap;

use crate::{
    engine::Database,
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
    Source,
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

    tag_lookup: HashMap<Box<str>, TagID>,
    property_lookup: HashMap<Box<str>, PropertyID>,
    source_lookup: HashMap<Box<str>, SourceID>,

    relationship_targets: PostingList<EntityID>,
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

        for relationship in &input.relationships {
            self.relationship_targets.insert(relationship.target, id);
        }

        self.documents.insert(id, input);
    }

    pub fn register_tag(&mut self, id: TagID, name: Box<str>) {
        self.tag_lookup.insert(name, id);
    }

    pub fn register_property(&mut self, id: PropertyID, name: Box<str>) {
        self.property_lookup.insert(name, id);
    }

    pub fn register_source(&mut self, id: SourceID, name: Box<str>) {
        self.source_lookup.insert(name, id);
    }

    pub fn get(&self, id: EntityID) -> Option<&EntityDocument> {
        self.documents.get(&id)
    }

    /// Exact alias lookup.
    pub fn resolve(&self, alias: &str) -> &[EntityID] {
        self.alias.resolve(&normalize(alias)).unwrap_or(&[])
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

    pub fn related_by_target(&self, entity: EntityID) -> Vec<EntityID> {
        self.relationship_targets.lookup(entity)
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

            for related in self.related_by_target(*id) {
                add_hit(related, 0.7, SearchSource::Relationship);
            }
        }

        for id in self.prefix(query) {
            add_hit(id, 0.8, SearchSource::PrefixAlias);

            for related in self.related_by_target(id) {
                add_hit(related, 0.6, SearchSource::Relationship);
            }
        }

        for id in self.fuzzy(query, 1) {
            add_hit(id, 0.6, SearchSource::FuzzyAlias);
        }

        for id in self.lookup(query) {
            add_hit(id, 0.5, SearchSource::Token);
        }

        for id in self.tag_search(query) {
            add_hit(id, 0.4, SearchSource::Tag);
        }

        for id in self.property_search(query) {
            add_hit(id, 0.3, SearchSource::Property);
        }

        for id in self.source_search(query) {
            add_hit(id, 0.2, SearchSource::Source);
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

    pub fn tag_search(&self, tag: &str) -> Vec<EntityID> {
        let key = normalize(tag);

        match self.tag_lookup.get(key.as_ref()) {
            Some(id) => self.tags.lookup(*id),
            None => Vec::new(),
        }
    }

    pub fn property_search(&self, property: &str) -> Vec<EntityID> {
        let key = normalize(property);

        match self.property_lookup.get(key.as_ref()) {
            Some(id) => self.properties.lookup(*id),
            None => Vec::new(),
        }
    }

    pub fn source_search(&self, source: &str) -> Vec<EntityID> {
        let key = normalize(source);

        match self.source_lookup.get(key.as_ref()) {
            Some(id) => self.sources.lookup(*id),
            None => Vec::new(),
        }
    }

    pub fn from_database(database: &Database) -> Self {
        let mut resolver = Self::new();

        for (id, tag) in database.tags.iter().enumerate() {
            resolver.register_tag(id as TagID, tag.clone());
        }

        for (id, property) in database.properties.iter().enumerate() {
            resolver.register_property(id as PropertyID, property.clone());
        }

        for (id, source) in database.sources.iter().enumerate() {
            resolver.register_source(id as SourceID, source.clone());
        }

        for entity in &database.entities {
            resolver.add(entity.clone());
        }

        resolver
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

            tag_lookup: HashMap::new(),
            property_lookup: HashMap::new(),
            source_lookup: HashMap::new(),

            relationship_targets: PostingList::<EntityID>::default(),
        }
    }
}
