use super::search::{SearchExplanation, SearchResult, SearchSource};

use std::collections::HashMap;

use crate::{
    engine::Database,
    entity::EntityDocument,
    entity::types::{AttributeKeyID, EntityID, Relationship, SourceID, TagID},
    storage::PostingList,
    utils::normalize,
};

use super::{
    alias::AliasIndex, bk_tree::BKTreeIndex, inverted::InvertedIndex, ranker::Ranker,
    trie::TrieIndex,
};

pub struct Resolver {
    documents: HashMap<EntityID, EntityDocument>,

    alias: AliasIndex,
    trie: TrieIndex,
    fuzzy: BKTreeIndex,
    tokens: InvertedIndex,

    tags: PostingList<TagID>,
    sources: PostingList<SourceID>,

    attribute_lookup: HashMap<Box<str>, AttributeKeyID>,
    attribute_names: Vec<Box<str>>,

    tag_lookup: HashMap<Box<str>, TagID>,
    attribute_index: InvertedIndex,
    source_lookup: HashMap<Box<str>, SourceID>,

    relationship_targets: PostingList<EntityID>,

    ranker: Ranker,
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

        for attribute in &input.attributes {
            let key = self
                .attribute_names
                .get(attribute.key as usize)
                .map(|x| x.to_string())
                .unwrap_or_default();

            let value = attribute.value.normalized();

            self.index_attribute(&key, &value, id);
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
        self.tag_lookup.insert(normalize(&name), id);
    }

    pub fn register_attribute(&mut self, id: AttributeKeyID, name: Box<str>) {
        self.attribute_lookup.insert(normalize(&name), id);
    }

    fn index_attribute(&mut self, key: &str, value: &str, id: EntityID) {
        let key = normalize(key);
        let value = normalize(value);

        self.attribute_index.insert(&key, id);
        self.attribute_index.insert(&value, id);
        self.attribute_index.insert(&format!("{key}:{value}"), id);

        for token in value.split_whitespace() {
            self.attribute_index.insert(token, id);
        }
    }

    pub fn attribute_search(&self, query: &str) -> Vec<EntityID> {
        self.attribute_index.lookup(&normalize(query))
    }

    pub fn register_source(&mut self, id: SourceID, name: Box<str>) {
        self.source_lookup.insert(normalize(&name), id);
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

    pub fn relationships_for(&self, entity: EntityID) -> Vec<&Relationship> {
        self.documents
            .get(&entity)
            .map(|doc| doc.relationships.iter().collect())
            .unwrap_or_default()
    }

    pub fn related_to_target(&self, target: EntityID) -> Vec<(EntityID, &Relationship)> {
        let mut out = Vec::new();

        for source_id in self.relationship_targets.lookup(target) {
            let doc = &self.documents[&source_id];

            for relationship in &doc.relationships {
                if relationship.target == target {
                    out.push((source_id, relationship));
                }
            }
        }

        out
    }

    /// High-level search.
    pub fn search<'a>(&'a self, query: &str) -> Vec<SearchResult<'a>> {
        let query = normalize(query);

        let mut candidates: Vec<(EntityID, SearchSource, SearchExplanation)> = Vec::new();

        if let Some(tag_id) = self.tag_search(&query) {
            for id in self.tags.lookup(tag_id) {
                candidates.push((
                    id,
                    SearchSource::Tag,
                    SearchExplanation::Tag { tag: tag_id },
                ));
            }
        }

        for id in self.attribute_search(&query) {
            candidates.push((
                id,
                SearchSource::Attribute,
                SearchExplanation::Attribute {
                    term: query.clone(),
                },
            ));
        }

        // Relationship matches
        for &target in self.resolve(&query) {
            for (entity_id, relationship) in self.related_to_target(target) {
                candidates.push((
                    entity_id,
                    SearchSource::Relationship,
                    SearchExplanation::Relationship {
                        kind: relationship.kind.clone(),
                        target,
                    },
                ));
            }
        }

        for &id in self.resolve(&query) {
            candidates.push((
                id,
                SearchSource::ExactAlias,
                SearchExplanation::ExactAlias {
                    alias: query.clone(),
                },
            ));
        }

        for id in self.prefix(&query) {
            candidates.push((
                id,
                SearchSource::PrefixAlias,
                SearchExplanation::PrefixAlias {
                    alias: query.clone(),
                },
            ));
        }

        for id in self.fuzzy(&query, 1) {
            candidates.push((
                id,
                SearchSource::FuzzyAlias,
                SearchExplanation::FuzzyAlias {
                    alias: query.clone(),
                },
            ));
        }

        for id in self.lookup(&query) {
            candidates.push((
                id,
                SearchSource::Token,
                SearchExplanation::Token {
                    token: query.clone(),
                },
            ));
        }

        if let Some(source_id) = self.source_search(&query) {
            for id in self.sources.lookup(source_id) {
                candidates.push((
                    id,
                    SearchSource::Source,
                    SearchExplanation::Source { source: source_id },
                ));
            }
        }

        self.ranker
            .rank(candidates)
            .into_iter()
            .filter_map(|hit| {
                self.documents
                    .get(&hit.entity_id)
                    .map(|entity| SearchResult {
                        entity,
                        score: hit.score,
                        sources: hit.sources,
                        explanations: hit.explanations,
                    })
            })
            .collect()
    }

    pub fn tag_search(&self, tag: &str) -> Option<TagID> {
        let key = normalize(tag);

        self.tag_lookup.get(key.as_ref()).copied()
    }

    pub fn source_search(&self, source: &str) -> Option<SourceID> {
        let key = normalize(source);

        self.source_lookup.get(key.as_ref()).copied()
    }

    pub fn from_database(database: &Database) -> Self {
        let mut resolver = Self::new();

        resolver.attribute_names = database.attribute_keys.clone();

        for (id, key) in database.attribute_keys.iter().enumerate() {
            resolver.register_attribute(id as AttributeKeyID, key.clone());
        }

        for (id, tag) in database.tags.iter().enumerate() {
            resolver.register_tag(id as TagID, tag.clone());
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
            sources: PostingList::<SourceID>::default(),

            attribute_lookup: HashMap::new(),
            attribute_names: Vec::new(),

            tag_lookup: HashMap::new(),
            attribute_index: InvertedIndex::default(),
            source_lookup: HashMap::new(),

            relationship_targets: PostingList::<EntityID>::default(),

            ranker: Ranker::default(),
        }
    }
}
