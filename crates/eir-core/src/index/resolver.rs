use super::search::{SearchExplanation, SearchResult, SearchSource};

use std::collections::HashMap;

use crate::{
    engine::Database,
    entity::EntityDocument,
    entity::types::{EntityID, PropertyID, Relationship, SourceID, TagID},
    storage::PostingList,
};

use super::{
    alias::AliasIndex, bk_tree::BKTreeIndex, inverted::InvertedIndex, ranker::Ranker,
    trie::TrieIndex, utils::normalize,
};

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
        self.tag_lookup.insert(normalize(&name), id);
    }

    pub fn register_property(&mut self, id: PropertyID, name: Box<str>) {
        self.property_lookup.insert(normalize(&name), id);
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

        for (entity_id, doc) in &self.documents {
            for relationship in &doc.relationships {
                if relationship.target == target {
                    out.push((*entity_id, relationship));
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

        if let Some(property_id) = self.property_search(&query) {
            for id in self.properties.lookup(property_id) {
                candidates.push((
                    id,
                    SearchSource::Property,
                    SearchExplanation::Property {
                        property: property_id,
                    },
                ));
            }
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

    pub fn property_search(&self, property: &str) -> Option<PropertyID> {
        let key = normalize(property);

        self.property_lookup.get(key.as_ref()).copied()
    }

    pub fn source_search(&self, source: &str) -> Option<SourceID> {
        let key = normalize(source);

        self.source_lookup.get(key.as_ref()).copied()
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

            ranker: Ranker::default(),
        }
    }
}
