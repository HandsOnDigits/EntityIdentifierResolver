use crate::error::{Error, Result};
use bytecheck::CheckBytes;
use rkyv::{Archive, Deserialize, Serialize};

use crate::prelude::{
    entity::prelude::{EntityDocument, input::EntityInput, types::*},
    index::Resolver,
    storage::{IndexBuilder, IndexRecord, Indexes, Registry, RegistryRecord},
};

#[derive(Debug, Default, Clone)]
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

    pub fn merge(&mut self, other: &Database) -> Result<()> {
        // Validate entity IDs before modifying anything.
        for entity in &other.entities {
            if self
                .entities
                .iter()
                .any(|existing| existing.id == entity.id)
            {
                return Err(Error::EntityAlreadyExists(entity.id.index().to_string()));
            }
        }

        // Work on a clone so a failed merge cannot partially modify `self`.
        let mut merged = self.clone();

        // Build registry ID mappings from `other` -> `merged`.
        let mut tag_ids = std::collections::HashMap::new();
        for (id, value) in other.tags.iter() {
            tag_ids.insert(id, merged.tags.intern(value));
        }

        let mut source_ids = std::collections::HashMap::new();
        for (id, value) in other.sources.iter() {
            source_ids.insert(id, merged.sources.intern(value));
        }

        let mut attribute_key_ids = std::collections::HashMap::new();
        for (id, value) in other.attribute_keys.iter() {
            attribute_key_ids.insert(id, merged.attribute_keys.intern(value));
        }

        let mut relationship_type_ids = std::collections::HashMap::new();
        for (id, value) in other.relationship_types.iter() {
            relationship_type_ids.insert(id, merged.relationship_types.intern(value));
        }

        for entity in &other.entities {
            let entity = EntityDocument {
                id: entity.id,

                aliases: entity.aliases.clone(),

                tags: entity
                    .tags
                    .iter()
                    .filter_map(|id| tag_ids.get(id).copied())
                    .collect(),

                sources: entity
                    .sources
                    .iter()
                    .filter_map(|id| source_ids.get(id).copied())
                    .collect(),

                attributes: entity
                    .attributes
                    .iter()
                    .map(|attribute| Attribute {
                        key: attribute_key_ids
                            .get(&attribute.key)
                            .copied()
                            .expect("attribute key mapping must exist"),
                        value: attribute.value.clone(),
                    })
                    .collect(),

                relationships: entity
                    .relationships
                    .iter()
                    .map(|relationship| Relationship {
                        target: relationship.target,
                        kind: match relationship.kind {
                            RelationshipType::Custom(id) => RelationshipType::Custom(
                                relationship_type_ids
                                    .get(&id)
                                    .copied()
                                    .expect("relationship type mapping must exist"),
                            ),

                            other => other,
                        },
                    })
                    .collect(),
            };

            merged.entities.push(entity);
        }

        // All entity-local references have now been remapped.
        merged.rebuild_indexes();

        // Commit only after the complete merge succeeded.
        *self = merged;

        Ok(())
    }
}

pub struct DatabaseStats {
    pub entities: usize,
    pub tags: usize,
    pub sources: usize,
    pub attributes: usize,
    pub relationship_types: usize,

    pub aliases: usize,
    pub trie: usize,
    pub fuzzy_aliases: usize,
    pub tokens: usize,
    pub tag_index: usize,
    pub source_index: usize,
    pub relationships: usize,
}

impl Database {
    pub fn stats(&self) -> DatabaseStats {
        DatabaseStats {
            entities: self.entities.len(),
            tags: self.indexes.tags.index.len(),
            sources: self.indexes.sources.index.len(),

            attributes: self.indexes.attribute_keys.tokens.len(),

            relationship_types: self.relationship_types.values().count(),

            aliases: self.indexes.alias.len(),
            trie: self.indexes.trie.len(),
            fuzzy_aliases: self.indexes.bk_tree.len(),
            tokens: self.indexes.inverted.tokens.len(),

            tag_index: self.indexes.tags.index.len(),
            source_index: self.indexes.sources.index.len(),

            relationships: self
                .entities
                .iter()
                .map(|entity| entity.relationships.len())
                .sum(),
        }
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

    use crate::entity::input::SourceInput;

    #[test]
    fn database_merge_combines_entities() -> Result<()> {
        let mut left = Database::default();

        left.insert(EntityInput {
            id: 1000,
            aliases: vec!["Apple".into()],
            tags: vec![],
            properties: vec![],
            relationships: vec![],
            sources: vec![],
        })?;

        let mut right = Database::default();

        right.insert(EntityInput {
            id: 2000,
            aliases: vec!["Banana".into()],
            tags: vec![],
            properties: vec![],
            relationships: vec![],
            sources: vec![],
        })?;

        left.merge(&right)?;

        assert!(left.entity(EntityID::new(1000)).is_some());
        assert!(left.entity(EntityID::new(2000)).is_some());

        assert!(!left.resolver().search("Apple").is_empty());
        assert!(!left.resolver().search("Banana").is_empty());

        Ok(())
    }

    #[test]
    fn database_merge_remaps_registries() -> Result<()> {
        let mut left = Database::default();

        left.insert(EntityInput {
            id: 1000,
            aliases: vec!["Apple".into()],
            tags: vec!["fruit".into()],
            properties: vec![],
            relationships: vec![],
            sources: vec![SourceInput {
                provider: "Source A".into(),
                verified: false,
            }],
        })?;

        let mut right = Database::default();

        right.insert(EntityInput {
            id: 2000,
            aliases: vec!["Berry".into()],
            tags: vec!["fruit".into(), "berry".into()],
            properties: vec![],
            relationships: vec![],
            sources: vec![SourceInput {
                provider: "Source B".into(),
                verified: false,
            }],
        })?;

        left.merge(&right)?;

        assert_eq!(left.tags.id("fruit"), Some(TagID::new(0)));
        assert_eq!(left.tags.id("berry"), Some(TagID::new(1)));

        assert!(left.sources.id("source a").is_some());
        assert!(left.sources.id("source b").is_some());

        Ok(())
    }

    #[test]
    fn database_merge_duplicate_entity_is_atomic() -> Result<()> {
        let mut left = Database::default();

        left.insert(EntityInput {
            id: 1000,
            aliases: vec!["Apple".into()],
            tags: vec!["fruit".into()],
            properties: vec![],
            relationships: vec![],
            sources: vec![],
        })?;

        let before = left.to_record();

        let mut right = Database::default();

        right.insert(EntityInput {
            id: 1000,
            aliases: vec!["Different Apple".into()],
            tags: vec!["different".into()],
            properties: vec![],
            relationships: vec![],
            sources: vec![],
        })?;

        let result = left.merge(&right);

        assert!(matches!(result, Err(Error::EntityAlreadyExists(_))));

        assert_eq!(left.entities.len(), 1);
        assert_eq!(left.to_record().entities, before.entities);

        assert!(left.entity(EntityID::new(1000)).is_some());
        assert!(left.entity(EntityID::new(2000)).is_none());

        Ok(())
    }
}
