use crate::{
    query::filters::ComparisonOp,
    search::{
        Ranker,
        result::{SearchExplanation, SearchResult, SearchSource},
    },
};

use std::collections::HashMap;

use crate::{
    engine::Database,
    entity::EntityDocument,
    entity::prelude::types::{
        AttributeKeyID, EntityID, Relationship, RelationshipTypeID, SourceID, TagID, Value,
    },
    storage::{PostingList, Registry},
    utils::normalize,
};

use super::{alias::AliasIndex, bk_tree::BKTreeIndex, inverted::InvertedIndex, trie::TrieIndex};

pub struct AttributeQuery<'a> {
    pub key: &'a str,
    pub op: ComparisonOp,
    pub value: &'a str,
}

#[derive(Clone)]
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

    attribute_keys: InvertedIndex,
    attribute_values: InvertedIndex,
    attribute_pairs: InvertedIndex,

    tag_lookup: HashMap<Box<str>, TagID>,
    source_lookup: HashMap<Box<str>, SourceID>,

    relationship_targets: PostingList<EntityID>,
    relationship_types: Registry<RelationshipTypeID>,

    ranker: Ranker,
}

// Getters
impl Resolver {
    pub fn alias(&self) -> &AliasIndex {
        &self.alias
    }

    pub fn trie(&self) -> &TrieIndex {
        &self.trie
    }

    pub fn fuzzy_index(&self) -> &BKTreeIndex {
        &self.fuzzy
    }

    pub fn tokens(&self) -> &InvertedIndex {
        &self.tokens
    }

    pub fn tags(&self) -> &PostingList<TagID> {
        &self.tags
    }

    pub fn sources(&self) -> &PostingList<SourceID> {
        &self.sources
    }

    pub fn relationship_targets(&self) -> &PostingList<EntityID> {
        &self.relationship_targets
    }

    pub fn attribute_keys(&self) -> &InvertedIndex {
        &self.attribute_keys
    }

    pub fn attribute_values(&self) -> &InvertedIndex {
        &self.attribute_values
    }

    pub fn attribute_pairs(&self) -> &InvertedIndex {
        &self.attribute_pairs
    }
}

// Logic
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
                .get(attribute.key.index())
                .map(|x| x.to_string())
                .expect("invalid attribute key");

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
        let name = normalize(&name);
        let index = id.index();

        if self.attribute_names.len() <= index {
            self.attribute_names.resize(index + 1, "".into());
        }

        self.attribute_names[index] = name.clone();
        self.attribute_lookup.insert(name, id);
    }

    fn index_attribute(&mut self, key: &str, value: &str, id: EntityID) {
        let key = normalize(key);
        let value = normalize(value);

        self.attribute_keys.insert(&key, id);

        self.attribute_values.insert(&value, id);

        self.attribute_pairs.insert(&format!("{key}:{value}"), id);

        for token in value.split_whitespace() {
            self.attribute_values.insert(token, id);
        }
    }

    pub fn parse_attribute_query(query: &str) -> Option<AttributeQuery<'_>> {
        for (operator, op) in [
            (">=", ComparisonOp::GtEq),
            ("<=", ComparisonOp::LtEq),
            ("!=", ComparisonOp::NotEq),
            ("=", ComparisonOp::Eq),
            (">", ComparisonOp::Gt),
            ("<", ComparisonOp::Lt),
        ] {
            if let Some((key, value)) = query.split_once(operator) {
                if !key.is_empty() && !value.is_empty() {
                    return Some(AttributeQuery { key, op, value });
                }
            }
        }

        None
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

    pub fn relationships_by_kind(
        &self,
        entity: EntityID,
        kind: RelationshipTypeID,
    ) -> Vec<(EntityID, &Relationship)> {
        let mut result = Vec::new();

        if let Some(doc) = self.documents.get(&entity) {
            for relationship in &doc.relationships {
                if relationship.kind.to_id() == kind {
                    result.push((relationship.target, relationship));
                }
            }
        }

        result
    }

    pub fn relationships_for(&self, entity: EntityID) -> Vec<&Relationship> {
        self.documents
            .get(&entity)
            .map(|doc| doc.relationships.iter().collect())
            .unwrap_or_default()
    }

    pub fn register_relationship_type(&mut self, name: &str) -> RelationshipTypeID {
        self.relationship_types.intern(name)
    }

    pub fn relationship_type_id(&self, name: &str) -> Option<RelationshipTypeID> {
        self.relationship_types.id(name)
    }

    pub fn entities_related_to(&self, target: EntityID) -> Vec<(EntityID, &Relationship)> {
        self.related_to_target(target)
    }

    pub fn entities_with_tag(&self, tag: TagID) -> Vec<EntityID> {
        self.tags.lookup(tag)
    }

    pub fn attribute_lookup(&self, key: &str, value: &Value) -> Vec<EntityID> {
        let key = normalize(key);
        let value = value.normalized();

        self.attribute_pairs.lookup(&format!("{key}:{value}"))
    }

    pub fn attribute_compare(&self, key: &str, op: ComparisonOp, value: &Value) -> Vec<EntityID> {
        let key = normalize(key);

        let Some(key_id) = self.attribute_lookup.get(key.as_ref()).copied() else {
            return Vec::new();
        };

        self.documents
            .values()
            .filter_map(|document| {
                let matches = document
                    .attributes
                    .iter()
                    .filter(|attribute| attribute.key == key_id)
                    .any(|attribute| op.matches(&attribute.value, value));

                matches.then_some(document.id)
            })
            .collect()
    }

    pub fn attribute_not_equal(&self, key: &str, value: &Value) -> Vec<EntityID> {
        let key = normalize(key);

        let Some(key_id) = self.attribute_lookup.get(key.as_ref()).copied() else {
            return Vec::new();
        };

        self.documents
            .values()
            .filter_map(|document| {
                let attributes = document
                    .attributes
                    .iter()
                    .filter(|attribute| attribute.key == key_id);

                let mut found = false;

                for attribute in attributes {
                    found = true;

                    if attribute.value == *value {
                        return None;
                    }
                }

                found.then_some(document.id)
            })
            .collect()
    }

    pub fn relationship_lookup(&self, kind: RelationshipTypeID, target: EntityID) -> Vec<EntityID> {
        let mut result = Vec::new();

        for (id, document) in &self.documents {
            for relationship in &document.relationships {
                if relationship.kind.to_id() == kind && relationship.target == target {
                    result.push(*id);
                }
            }
        }

        result
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
    pub fn search<'a>(&'a self, raw_query: &str) -> Vec<SearchResult<'a>> {
        let attribute = Self::parse_attribute_query(raw_query);

        let query = normalize(raw_query);

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

        // Relationship matches
        for &target in self.resolve(&query) {
            for (entity_id, relationship) in self.related_to_target(target) {
                candidates.push((
                    entity_id,
                    SearchSource::Relationship,
                    SearchExplanation::Relationship {
                        kind: relationship.kind.to_id(),
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

        // Attribute key:value search
        if let Some(attribute) = attribute {
            let key = normalize(attribute.key);
            let value = normalize(attribute.value);

            for id in self.attribute_pairs.lookup(&format!("{key}:{value}")) {
                candidates.push((
                    id,
                    SearchSource::AttributeKeyValue,
                    SearchExplanation::AttributeKeyValue {
                        key: key.clone(),
                        value: value.clone(),
                    },
                ));
            }
        } else {
            // attribute key
            for id in self.attribute_keys.lookup(&query) {
                candidates.push((
                    id,
                    SearchSource::AttributeKey,
                    SearchExplanation::AttributeKey {
                        term: query.clone(),
                    },
                ));
            }

            // attribute value
            for id in self.attribute_values.lookup(&query) {
                candidates.push((
                    id,
                    SearchSource::AttributeValue,
                    SearchExplanation::AttributeValue {
                        term: query.clone(),
                    },
                ));
            }
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
        Self {
            documents: database
                .entities
                .iter()
                .map(|entity| (entity.id, entity.clone()))
                .collect(),

            alias: database.indexes.alias.clone(),
            trie: database.indexes.trie.clone(),
            fuzzy: database.indexes.bk_tree.clone(),
            tokens: database.indexes.inverted.clone(),

            tags: database.indexes.tags.clone(),
            sources: database.indexes.sources.clone(),

            attribute_lookup: database
                .attribute_keys
                .iter()
                .map(|(id, name)| (normalize(name), id))
                .collect(),

            attribute_names: database
                .attribute_keys
                .iter()
                .map(|(_, name)| normalize(name))
                .collect(),

            attribute_keys: database.indexes.attribute_keys.clone(),
            attribute_values: database.indexes.attribute_values.clone(),
            attribute_pairs: database.indexes.attribute_pairs.clone(),

            tag_lookup: database
                .tags
                .iter()
                .map(|(id, name)| (normalize(name), id))
                .collect(),

            source_lookup: database
                .sources
                .iter()
                .map(|(id, name)| (normalize(name), id))
                .collect(),

            relationship_targets: database.indexes.relationships.clone(),

            relationship_types: database.relationship_types.clone(),

            ranker: Ranker,
        }
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

            attribute_keys: InvertedIndex::default(),
            attribute_values: InvertedIndex::default(),
            attribute_pairs: InvertedIndex::default(),

            tag_lookup: HashMap::new(),
            source_lookup: HashMap::new(),

            relationship_targets: PostingList::<EntityID>::default(),
            relationship_types: Registry::<RelationshipTypeID>::default(),

            ranker: Ranker,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_utils::test_entity_with_attribute;

    #[test]
    fn attribute_less_than_matches() {
        let mut resolver = Resolver::default();

        let key = AttributeKeyID::new(1);
        resolver.register_attribute(key, "price".into());

        resolver.add(test_entity_with_attribute(
            EntityID::new(1),
            "Cheap",
            key,
            Value::Integer(5),
        ));

        resolver.add(test_entity_with_attribute(
            EntityID::new(2),
            "Exact",
            key,
            Value::Integer(10),
        ));

        resolver.add(test_entity_with_attribute(
            EntityID::new(3),
            "Expensive",
            key,
            Value::Integer(20),
        ));

        let result = resolver.attribute_compare("price", ComparisonOp::Lt, &Value::Integer(10));

        assert_eq!(result, vec![EntityID::new(1)]);
    }
}
