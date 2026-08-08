use crate::{
    entity::prelude::{
        EntityDocument,
        types::{EntityID, RelationshipTypeID, SourceID, TagID},
    },
    index::prelude::*,
    storage::{PostingList, PostingListRecord, Registry, RegistryRecord},
    utils::normalize,
};

#[derive(Debug, Default, Clone)]
pub struct Indexes {
    pub alias: AliasIndex,
    pub trie: TrieIndex,
    pub bk_tree: BKTreeIndex,
    pub inverted: InvertedIndex,

    pub tags: PostingList<TagID>,
    pub sources: PostingList<SourceID>,

    pub attribute_keys: InvertedIndex,
    pub attribute_values: InvertedIndex,
    pub attribute_pairs: InvertedIndex,

    pub relationships: PostingList<EntityID>,
    pub relationship_types: Registry<RelationshipTypeID>,
}

impl Indexes {
    pub fn to_record(&self) -> IndexRecord {
        IndexRecord {
            alias: self.alias.to_record(),
            trie: self.trie.to_record(),
            bk_tree: self.bk_tree.to_record(),
            inverted: self.inverted.to_record(),

            tags: self.tags.to_record(),
            sources: self.sources.to_record(),

            attribute_keys: self.attribute_keys.to_record(),
            attribute_values: self.attribute_values.to_record(),
            attribute_pairs: self.attribute_pairs.to_record(),

            relationships: self.relationships.to_record(),
            relationship_types: self.relationship_types.to_record(),
        }
    }

    pub fn from_record(record: IndexRecord) -> Self {
        Self {
            alias: AliasIndex::from_record(record.alias),
            trie: TrieIndex::from_record(record.trie),
            bk_tree: BKTreeIndex::from_record(record.bk_tree),
            inverted: InvertedIndex::from_record(record.inverted),

            tags: PostingList::from_record(record.tags),
            sources: PostingList::from_record(record.sources),

            attribute_keys: InvertedIndex::from_record(record.attribute_keys),
            attribute_values: InvertedIndex::from_record(record.attribute_values),
            attribute_pairs: InvertedIndex::from_record(record.attribute_pairs),

            relationships: PostingList::from_record(record.relationships),
            relationship_types: Registry::from_record(record.relationship_types),
        }
    }
}

pub struct IndexBuilder;

impl IndexBuilder {
    pub fn build<'a>(
        inputs: &[EntityDocument],
        attribute_keys: impl IntoIterator<Item = &'a str>,
    ) -> Indexes {
        let mut indexes = Indexes::default();

        let keys: Vec<&str> = attribute_keys.into_iter().collect();

        for entity in inputs {
            let id = entity.id;

            for alias in &entity.aliases {
                let normalized = normalize(alias);

                indexes.alias.insert(normalized.clone(), id);
                indexes.trie.insert(&normalized, id);
                indexes.bk_tree.insert(&normalized, id);

                for token in normalized.split_whitespace() {
                    indexes.inverted.insert(token, id);
                }
            }

            for tag in &entity.tags {
                indexes.tags.insert(*tag, id);
            }

            for source in &entity.sources {
                indexes.sources.insert(*source, id);
            }

            for attribute in &entity.attributes {
                let Some(&key_str) = keys.get(attribute.key.index()) else {
                    continue;
                };

                let key = normalize(key_str);
                let value = attribute.value.normalized();

                indexes.attribute_keys.insert(&key, id);
                indexes.attribute_values.insert(&value, id);

                indexes
                    .attribute_pairs
                    .insert(&format!("{key}:{value}"), id);

                for token in value.split_whitespace() {
                    indexes.attribute_values.insert(token, id);
                }
            }

            for relationship in &entity.relationships {
                indexes.relationships.insert(relationship.target, id);
            }
        }

        indexes
    }
}

use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct IndexRecord {
    pub alias: AliasIndexRecord,
    pub trie: TrieIndexRecord,
    pub bk_tree: BKTreeIndexRecord,
    pub inverted: InvertedIndexRecord,

    pub tags: PostingListRecord<TagID>,
    pub sources: PostingListRecord<SourceID>,

    pub attribute_keys: InvertedIndexRecord,
    pub attribute_values: InvertedIndexRecord,
    pub attribute_pairs: InvertedIndexRecord,

    pub relationships: PostingListRecord<EntityID>,
    pub relationship_types: RegistryRecord,
}
