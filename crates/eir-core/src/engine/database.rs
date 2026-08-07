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

    pub fn entity(&self, id: EntityID) -> Option<&EntityDocument> {
        self.entities.iter().find(|entity| entity.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{entity::prelude::types::EntityID, test::fixture_database};

    #[test]
    fn database_creates_resolver() {
        let database = Database::default();

        let resolver = database.resolver();

        let result = resolver.resolve("unknown");

        assert!(result.is_empty());
    }

    #[test]
    fn database_stores_entity() {
        let mut database = Database::default();

        database.entities.push(EntityDocument {
            id: EntityID(1),
            aliases: vec!["Chocolate".into()],
            tags: vec![],
            attributes: vec![],
            relationships: vec![],
            sources: vec![],
        });

        assert_eq!(database.entities.len(), 1);
        assert_eq!(database.entities[0].id, EntityID(1));
    }

    #[test]
    fn database_finds_entity_by_id() {
        let database = fixture_database();

        let entity = database.entity(EntityID(1));

        assert!(entity.is_some());
        assert_eq!(entity.unwrap().id, EntityID(1));
    }

    #[test]
    fn database_serializes() {
        let database = Database::default();

        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&database).expect("serialize failed");

        let archived = rkyv::access::<rkyv::Archived<Database>, rkyv::rancor::Error>(&bytes)
            .expect("archive failed");

        assert!(archived.entities.is_empty());
    }
}
