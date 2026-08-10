mod builder;
pub mod database;
mod search;

pub use builder::EngineBuilder;
pub use database::{Database, DatabaseRecord};

use std::path::Path;

use crate::{
    config::StorageConfig,
    entity::prelude::{EntityDocument, input::EntityInput, types::EntityID},
    search::result::SearchResult,
    storage::Backend,
};

pub use search::SearchEngine;

use crate::{error::Result, index::Resolver};

pub struct Engine {
    backend: Backend,
    database: Database,
    resolver: Resolver,
}

impl Engine {
    pub fn create(path: impl AsRef<Path>) -> Result<Self> {
        let config = StorageConfig {
            root: path.as_ref().to_path_buf(),
            ..StorageConfig::default()
        };

        let backend = Backend::create(config)?;
        let database = Database::default();
        let resolver = database.resolver();

        Ok(Self {
            backend,
            database,
            resolver,
        })
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let config = StorageConfig {
            root: path.as_ref().to_path_buf(),
            ..StorageConfig::default()
        };

        let backend = Backend::open(config)?;
        let record = backend.read()?;

        let database = Database::from_record(record);
        let resolver = database.resolver();

        Ok(Self {
            backend,
            database,
            resolver,
        })
    }

    pub fn search(&self, query: &str) -> Vec<SearchResult<'_>> {
        self.resolver.search(query)
    }

    pub fn entity(&self, id: EntityID) -> Option<&EntityDocument> {
        self.database.entity(id)
    }

    pub fn flush(&mut self) -> Result<()> {
        self.backend.write(&self.database.to_record())
    }

    pub fn insert(&mut self, input: EntityInput) -> Result<()> {
        self.database.insert(input)?;
        self.resolver = self.database.resolver();

        Ok(())
    }

    pub fn remove(&mut self, id: EntityID) -> Result<()> {
        self.database.remove(id)?;
        self.resolver = self.database.resolver();

        Ok(())
    }

    pub fn database(&self) -> &Database {
        &self.database
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<Engine>();
    }

    #[test]
    fn engine_searches_loaded_database() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path();

        {
            let mut engine = Engine::create(path)?;

            engine.insert(EntityInput {
                id: 9100,
                aliases: vec!["Test Berry".into()],
                tags: vec![],
                properties: vec![],
                relationships: vec![],
                sources: vec![],
            })?;

            engine.flush()?;
        }

        let engine = Engine::open(path)?;

        let results = engine.search("Test Berry");

        assert!(!results.is_empty());
        assert_eq!(results[0].entity.id, EntityID::new(9100));

        std::fs::remove_file(path).ok();

        Ok(())
    }

    #[test]
    fn engine_flushes_database() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path();

        let mut engine = Engine::create(path)?;

        engine.flush()?;

        assert!(path.exists());

        std::fs::remove_file(path).ok();

        Ok(())
    }

    #[test]
    fn engine_roundtrip_persists_database() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path();

        // Create a new engine/database.
        let mut engine = Engine::create(path)?;

        // Insert data into the database.
        engine.insert(EntityInput {
            id: 9200,
            aliases: vec!["Roundtrip Berry".into()],
            tags: vec![],
            properties: vec![],
            relationships: vec![],
            sources: vec![],
        })?;

        // Persist it.
        engine.flush()?;

        // Drop the original engine before reopening the file.
        drop(engine);

        // Reopen from disk.
        let engine = Engine::open(path)?;

        // Verify the entity survived persistence.
        let entity = engine.entity(EntityID::new(9200));

        assert!(entity.is_some());
        assert_eq!(entity.unwrap().aliases, vec!["Roundtrip Berry".into()]);

        // Verify the search index was restored too.
        let results = engine.search("Roundtrip Berry");

        assert!(!results.is_empty());
        assert_eq!(results[0].entity.id, EntityID::new(9200));

        std::fs::remove_file(path).ok();

        Ok(())
    }
}
