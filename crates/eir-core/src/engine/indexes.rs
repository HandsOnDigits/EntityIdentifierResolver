use rkyv::{Archive, Deserialize, Serialize};

use crate::{
    entity::prelude::types::{EntityID, SourceID, TagID},
    index::Resolver,
    index::*,
    storage::*,
};

use super::Database;

#[derive(Debug, Default)]
pub struct Indexes {
    pub alias: AliasIndex,
    pub trie: TrieIndex,
    pub bk_tree: BKTreeIndex,
    pub inverted: InvertedIndex,

    pub tag: PostingList<TagID>,
    pub source: PostingList<SourceID>,
    pub relationship: PostingList<EntityID>,

    pub attribute_key: InvertedIndex,
    pub attribute_value: InvertedIndex,
    pub attribute_pair: InvertedIndex,
}
impl Indexes {
    pub fn build(database: &Database) -> Self {
        let resolver = Resolver::from_database(database);

        Self {
            alias: resolver.alias().clone(),
            trie: resolver.trie().clone(),
            bk_tree: resolver.fuzzy_index().clone(),
            inverted: resolver.tokens().clone(),

            tag: resolver.tags().clone(),
            source: resolver.sources().clone(),
            relationship: resolver.relationship_targets().clone(),

            attribute_key: resolver.attribute_keys().clone(),
            attribute_value: resolver.attribute_values().clone(),
            attribute_pair: resolver.attribute_pairs().clone(),
        }
    }

    pub fn to_record(&self) -> IndexRecord {
        IndexRecord {
            alias: self.alias.to_record(),
            trie: self.trie.to_record(),
            bk_tree: self.bk_tree.to_record(),
            inverted: self.inverted.to_record(),

            tag: self.tag.to_record(),
            source: self.source.to_record(),
            relationship: self.relationship.to_record(),

            attribute_key: self.attribute_key.to_record(),
            attribute_value: self.attribute_value.to_record(),
            attribute_pair: self.attribute_pair.to_record(),
        }
    }

    pub fn from_record(record: IndexRecord) -> Self {
        Self {
            alias: AliasIndex::from_record(record.alias),
            trie: TrieIndex::from_record(record.trie),
            bk_tree: BKTreeIndex::from_record(record.bk_tree),
            inverted: InvertedIndex::from_record(record.inverted),

            tag: PostingList::from_record(record.tag),
            source: PostingList::from_record(record.source),
            relationship: PostingList::from_record(record.relationship),

            attribute_key: InvertedIndex::from_record(record.attribute_key),
            attribute_value: InvertedIndex::from_record(record.attribute_value),
            attribute_pair: InvertedIndex::from_record(record.attribute_pair),
        }
    }

    pub fn from_builder(indexes: crate::storage::Indexes) -> Self {
        Self {
            alias: indexes.alias,
            trie: indexes.trie,
            bk_tree: indexes.bk_tree,
            inverted: indexes.inverted,

            attribute_key: indexes.attribute_keys,
            attribute_value: indexes.attribute_values,
            attribute_pair: indexes.attribute_pairs,

            tag: indexes.tags,
            source: indexes.sources,
            relationship: indexes.relationships,
        }
    }
}

#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct IndexRecord {
    pub alias: AliasIndexRecord,
    pub trie: TrieIndexRecord,
    pub bk_tree: BKTreeIndexRecord,
    pub inverted: InvertedIndexRecord,

    pub tag: PostingListRecord<TagID>,
    pub source: PostingListRecord<SourceID>,
    pub relationship: PostingListRecord<EntityID>,

    pub attribute_key: InvertedIndexRecord,
    pub attribute_value: InvertedIndexRecord,
    pub attribute_pair: InvertedIndexRecord,
}
