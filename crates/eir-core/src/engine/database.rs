use crate::error::{Error, Result};
use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

use crate::prelude::{
    entity::prelude::{EntityDocument, input::EntityInput, types::*},
    index::Resolver,
    storage::{IndexBuilder, IndexRecord, Indexes, Registry, RegistryRecord},
};

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
            return Err(Error::EntityAlreadyExists(input.id.to_string()));
        }

        let entity = EntityDocument {
            id: EntityID::new(input.id),

            aliases: input.aliases.into_iter().collect(),

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
            return Err(Error::EntityAlreadyExists(entity.id.index().to_string()));
        }

        self.entities.push(entity);

        self.rebuild_indexes();

        Ok(())
    }

    pub fn remove(&mut self, id: EntityID) -> Result<()> {
        let before = self.entities.len();

        self.entities.retain(|entity| entity.id != id);

        if self.entities.len() == before {
            return Err(Error::EntityNotFound(id.index()));
        }

        self.rebuild_indexes();

        Ok(())
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

    use crate::{engine::Engine, entity::prelude::types::EntityID};

    #[test]
    fn engine_roundtrip_persists_database() -> Result<()> {
        let path =
            std::env::temp_dir().join(format!("eir-engine-roundtrip-{}.eir", std::process::id()));

        let mut engine = Engine::create(&path)?;

        engine.insert(EntityInput {
            id: 9200,
            aliases: vec!["Roundtrip Berry".into()],
            tags: vec![],
            properties: vec![],
            relationships: vec![],
            sources: vec![],
        })?;

        assert!(!engine.search("Roundtrip Berry").is_empty());

        engine.flush()?;
        drop(engine);

        let engine = Engine::open(&path)?;

        let entity = engine.entity(EntityID::new(9200));

        assert!(entity.is_some());
        assert_eq!(entity.unwrap().aliases, vec!["Roundtrip Berry".into()]);

        let results = engine.search("Roundtrip Berry");

        assert!(!results.is_empty());
        assert_eq!(results[0].entity.id, EntityID::new(9200));

        std::fs::remove_file(path).ok();

        Ok(())
    }
}
