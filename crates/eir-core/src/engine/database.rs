use rkyv::{Archive, Deserialize, Serialize, bytecheck::CheckBytes};

use crate::prelude::{
    entity::prelude::{
        EntityDocument,
        types::{EntityID, RelationshipTypeID, SourceID, TagID},
    },
    index::{AliasIndexRecord, BKTreeIndexRecord, InvertedIndexRecord, Resolver, TrieIndexRecord},
    storage::{PostingListRecord, RegistryRecord},
};

#[derive(Debug, Archive, Serialize, Deserialize, CheckBytes, Default)]
pub struct Database {
    pub entities: Vec<EntityDocument>,

    pub tags: Vec<Box<str>>,
    pub sources: Vec<Box<str>>,
    pub attribute_keys: Vec<Box<str>>,

    pub alias_index: AliasIndexRecord,
    pub trie_index: TrieIndexRecord,
    pub bk_tree_index: BKTreeIndexRecord,
    pub inverted_index: InvertedIndexRecord,

    pub attribute_key_index: InvertedIndexRecord,
    pub attribute_value_index: InvertedIndexRecord,
    pub attribute_pair_index: InvertedIndexRecord,

    pub tag_index: PostingListRecord<TagID>,
    pub source_index: PostingListRecord<SourceID>,

    pub relationship_index: PostingListRecord<EntityID>,
    pub relationship_types: RegistryRecord<RelationshipTypeID>,
}

impl Database {
    pub fn resolver(&self) -> Resolver {
        Resolver::from_database(self)
    }
}
