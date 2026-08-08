use anyhow::Result;
use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

use crate::prelude::{
    entity::prelude::{EntityDocument, input::EntityInput, types::*},
    index::Resolver,
    storage::{IndexBuilder, IndexRecord, Indexes, Registry, RegistryRecord},
};

use std::path::Path;

#[derive(Debug, Default)]
pub struct Database {
    pub entities: Vec<EntityDocument>,

    pub tags: Registry<TagID>,
    pub sources: Registry<SourceID>,
    pub attribute_keys: Registry<AttributeKeyID>,
    pub relationship_types: Registry<RelationshipTypeID>,

    pub indexes: Indexes,
}

impl Database {
    pub fn rebuild_indexes(&mut self) {
        self.indexes = IndexBuilder::build(
            &self.entities,
            self.attribute_keys.iter().map(|(_, value)| value),
        );
    }

    pub fn resolver(&self) -> Resolver {
        Resolver::from_database(self)
    }

    pub fn to_record(&self) -> DatabaseRecord {
        DatabaseRecord {
            entities: self.entities.clone(),

            tags: self.tags.to_record(),
            sources: self.sources.to_record(),
            attribute_keys: self.attribute_keys.to_record(),
            relationship_types: self.relationship_types.to_record(),

            indexes: self.indexes.to_record(),
        }
    }

    pub fn from_record(record: DatabaseRecord) -> Self {
        Self {
            entities: record.entities,

            tags: Registry::from_record(record.tags),
            sources: Registry::from_record(record.sources),
            attribute_keys: Registry::from_record(record.attribute_keys),
            relationship_types: Registry::from_record(record.relationship_types),

            indexes: Indexes::from_record(record.indexes),
        }
    }

    pub fn insert(&mut self, input: EntityInput) -> Result<()> {
        if self
            .entities
            .iter()
            .any(|e| e.id == EntityID::new(input.id))
        {
            anyhow::bail!("Entity already exists: {}", input.id);
        }

        let entity = EntityDocument {
            id: EntityID::new(input.id),

            aliases: input.aliases.into_iter().map(Into::into).collect(),

            tags: input.tags.iter().map(|tag| self.tags.intern(tag)).collect(),

            sources: input
                .sources
                .iter()
                .map(|source| self.sources.intern(&source.provider))
                .collect(),

            attributes: input
                .properties
                .into_iter()
                .map(|property| Attribute {
                    key: self.attribute_keys.intern(&property.key),
                    value: Value::String(property.value),
                })
                .collect(),

            relationships: input
                .relationships
                .into_iter()
                .map(|relationship| Relationship {
                    target: EntityID::new(relationship.target),
                    kind: RelationshipType::Custom(
                        self.relationship_types.intern(&relationship.kind),
                    ),
                })
                .collect(),
        };

        if self.entities.iter().any(|e| e.id == entity.id) {
            anyhow::bail!("Entity already exists");
        }

        self.entities.push(entity);

        self.rebuild_indexes();

        Ok(())
    }

    pub fn remove(&mut self, id: EntityID) -> anyhow::Result<()> {
        let before = self.entities.len();

        self.entities.retain(|entity| entity.id != id);

        if self.entities.len() == before {
            anyhow::bail!("Entity not found: {}", id.index());
        }

        self.rebuild_indexes();

        Ok(())
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let record = self.to_record();

        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&record)?;

        std::fs::write(path, bytes)?;

        Ok(())
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Database> {
        let bytes = std::fs::read(path)?;

        let record = rkyv::from_bytes::<DatabaseRecord, rkyv::rancor::Error>(&bytes)?;

        Ok(Database::from_record(record))
    }

    pub fn entity(&self, id: EntityID) -> Option<&EntityDocument> {
        self.entities.iter().find(|entity| entity.id == id)
    }
}

#[derive(Debug, Archive, Serialize, Deserialize, CheckBytes)]
pub struct DatabaseRecord {
    pub entities: Vec<EntityDocument>,

    pub tags: RegistryRecord,
    pub sources: RegistryRecord,
    pub attribute_keys: RegistryRecord,
    pub relationship_types: RegistryRecord,

    pub indexes: IndexRecord,
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

        let entity_id = EntityID::new(1);

        database.entities.push(EntityDocument {
            id: entity_id,
            aliases: vec!["Chocolate".into()],
            tags: vec![],
            attributes: vec![],
            relationships: vec![],
            sources: vec![],
        });

        assert_eq!(database.entities.len(), 1);
        assert_eq!(database.entities[0].id, entity_id);
    }

    #[test]
    fn database_finds_entity_by_id() {
        let database = fixture_database();

        let entity_id = EntityID::new(1);

        let entity = database.entity(entity_id);

        assert!(entity.is_some());
        assert_eq!(entity.unwrap().id, entity_id);
    }

    #[test]
    fn empty_database_resolves_safely() {
        let database = Database::default();

        let resolver = database.resolver();

        assert!(resolver.resolve("anything").is_empty());
    }

    #[test]
    fn database_missing_entity_returns_none() {
        let database = fixture_database();

        assert!(database.entity(EntityID::new(999)).is_none());
    }

    #[test]
    fn database_serializes() {
        let database = Database::default();

        let record = database.to_record();

        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&record).expect("serialize failed");

        let archived = rkyv::access::<rkyv::Archived<DatabaseRecord>, rkyv::rancor::Error>(&bytes)
            .expect("archive failed");

        assert!(archived.entities.is_empty());
    }

    #[test]
    fn database_insert_is_searchable() {
        let mut database = Database::default();

        database
            .insert(EntityInput {
                id: 9100,
                aliases: vec!["Test Berry".into()],
                tags: vec![],
                properties: vec![],
                relationships: vec![],
                sources: vec![],
            })
            .unwrap();

        assert_eq!(database.entities.len(), 1);
        assert_eq!(database.entities[0].aliases, vec!["Test Berry".into()]);

        let resolver = database.resolver();

        dbg!(&database.indexes);
        dbg!(resolver.resolve("Test Berry"));

        assert!(!resolver.resolve("Test Berry").is_empty());
    }

    #[test]
    fn database_remove_removes_entity_from_search_index() {
        let mut database = Database::default();

        database
            .insert(EntityInput {
                id: 9100,
                aliases: vec!["Test Berry".into()],
                tags: vec![],
                properties: vec![],
                relationships: vec![],
                sources: vec![],
            })
            .unwrap();

        assert!(!database.resolver().resolve("Test Berry").is_empty());

        database.remove(EntityID::new(9100)).unwrap();

        assert!(database.entity(EntityID::new(9100)).is_none());
        assert!(database.resolver().resolve("Test Berry").is_empty());
    }

    #[test]
    fn database_remove_persists_across_reload() {
        let mut database = Database::default();

        database
            .insert(EntityInput {
                id: 9100,
                aliases: vec!["Test Berry".into()],
                tags: vec![],
                properties: vec![],
                relationships: vec![],
                sources: vec![],
            })
            .unwrap();

        database.remove(EntityID::new(9100)).unwrap();

        let path = std::env::temp_dir().join("eir-test-remove.eir");
        database.save(&path).unwrap();

        let loaded = Database::load(&path).unwrap();

        assert!(loaded.entity(EntityID::new(9100)).is_none());
        assert!(loaded.resolver().resolve("Test Berry").is_empty());

        std::fs::remove_file(path).ok();
    }

    #[test]
    fn database_remove_preserves_other_entities_in_search_index() {
        let mut database = Database::default();

        database
            .insert(EntityInput {
                id: 9100,
                aliases: vec!["Test Berry".into()],
                tags: vec![],
                properties: vec![],
                relationships: vec![],
                sources: vec![],
            })
            .unwrap();

        database
            .insert(EntityInput {
                id: 9101,
                aliases: vec!["Test Berry Plus".into()],
                tags: vec![],
                properties: vec![],
                relationships: vec![],
                sources: vec![],
            })
            .unwrap();

        database.remove(EntityID::new(9100)).unwrap();

        let resolver = database.resolver();

        let results = resolver.search("Test Berry Plus");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].entity.id, EntityID::new(9101));
    }
}
