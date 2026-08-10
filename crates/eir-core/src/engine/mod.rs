mod builder;
pub mod database;
mod search;

use std::path::Path;

pub use builder::EngineBuilder;
pub use database::{Database, DatabaseRecord};

use crate::storage::wal::WalOperation;

use crate::{
    config::StorageConfig,
    entity::prelude::{EntityDocument, input::EntityInput, types::EntityID},
    error::{Error, Result},
    index::Resolver,
    search::result::SearchResult,
    storage::Backend,
};

pub use search::SearchEngine;

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
            ..Default::default()
        };

        let backend = Backend::open(config)?;

        let mut database = match backend.read() {
            Ok(record) => Database::from_record(record),
            Err(Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
                Database::default()
            }
            Err(error) => return Err(error),
        };

        for operation in backend.replay()? {
            match operation {
                WalOperation::Insert(input) => {
                    database.insert(input)?;
                }
                WalOperation::Remove(id) => {
                    database.remove(id)?;
                }
            }
        }

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
        if self.database.entity(EntityID::new(input.id)).is_some() {
            return Err(Error::EntityAlreadyExists(input.id.to_string()));
        }

        self.backend.append(&WalOperation::Insert(input.clone()))?;

        self.database.insert(input)?;
        self.resolver = self.database.resolver();

        Ok(())
    }

    pub fn remove(&mut self, id: EntityID) -> Result<()> {
        if self.database.entity(id).is_none() {
            return Err(Error::EntityNotFound(id.index()));
        }

        self.backend.append(&WalOperation::Remove(id))?;

        self.database.remove(id)?;
        self.resolver = self.database.resolver();

        Ok(())
    }

    pub fn database(&self) -> &Database {
        &self.database
    }

    pub fn backend(&self) -> &Backend {
        &self.backend
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

        Ok(())
    }

    #[test]
    fn engine_flushes_database() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path();

        let mut engine = Engine::create(path)?;

        engine.flush()?;

        assert!(path.exists());

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

        Ok(())
    }

    #[test]
    fn engine_recovers_unflushed_insert_from_wal() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("database.eir");

        {
            let mut engine = Engine::create(&path)?;

            engine.insert(EntityInput {
                id: 9300,
                aliases: vec!["WAL Berry".into()],
                tags: vec![],
                properties: vec![],
                relationships: vec![],
                sources: vec![],
            })?;

            // Deliberately do NOT flush.
            //
            // The entity should only exist in the in-memory database
            // and WAL at this point.
        }

        // Reopen as if the process had restarted.
        let engine = Engine::open(&path)?;

        let entity = engine.entity(EntityID::new(9300));

        assert!(entity.is_some());
        assert_eq!(entity.unwrap().aliases, vec!["WAL Berry".into()]);

        let results = engine.search("WAL Berry");

        assert!(!results.is_empty());
        assert_eq!(results[0].entity.id, EntityID::new(9300));

        Ok(())
    }

    #[test]
    fn engine_recovers_unflushed_remove_from_wal() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("database.eir");

        {
            let mut engine = Engine::create(&path)?;

            engine.insert(EntityInput {
                id: 9400,
                aliases: vec!["Removed Berry".into()],
                tags: vec![],
                properties: vec![],
                relationships: vec![],
                sources: vec![],
            })?;

            // Persist the initial state.
            engine.flush()?;
        }

        {
            let mut engine = Engine::open(&path)?;

            assert!(engine.entity(EntityID::new(9400)).is_some());

            engine.remove(EntityID::new(9400))?;

            // Deliberately do NOT flush.
            //
            // The remove should now exist only in the WAL.
        }

        // Reopen as if the process had crashed.
        let engine = Engine::open(&path)?;

        assert!(engine.entity(EntityID::new(9400)).is_none());

        let results = engine.search("Removed Berry");

        assert!(results.is_empty());

        Ok(())
    }

    #[test]
    fn engine_flush_truncates_wal() -> Result<()> {
        let path =
            std::env::temp_dir().join(format!("eir-engine-wal-flush-{}.eir", std::process::id()));

        let mut engine = Engine::create(&path)?;

        engine.insert(EntityInput {
            id: 9300,
            aliases: vec!["WAL Flush Berry".into()],
            tags: vec![],
            properties: vec![],
            relationships: vec![],
            sources: vec![],
        })?;

        // The mutation must be present in the WAL before flush.
        assert!(!engine.backend().wal().replay()?.is_empty());

        engine.flush()?;

        // After the snapshot is durable, the WAL should be empty.
        assert!(engine.backend().wal().replay()?.is_empty());

        drop(engine);

        let engine = Engine::open(&path)?;

        assert!(engine.entity(EntityID::new(9300)).is_some());

        std::fs::remove_dir_all(path).ok();

        Ok(())
    }

    #[test]
    fn engine_recovers_snapshot_plus_wal() -> Result<()> {
        let path = std::env::temp_dir().join(format!(
            "eir-engine-snapshot-wal-{}.eir",
            std::process::id()
        ));

        let mut engine = Engine::create(&path)?;

        engine.insert(EntityInput {
            id: 9400,
            aliases: vec!["Snapshot Berry".into()],
            tags: vec![],
            properties: vec![],
            relationships: vec![],
            sources: vec![],
        })?;

        engine.flush()?;

        // This entity exists only in the WAL now.
        engine.insert(EntityInput {
            id: 9401,
            aliases: vec!["WAL Berry".into()],
            tags: vec![],
            properties: vec![],
            relationships: vec![],
            sources: vec![],
        })?;

        drop(engine);

        let engine = Engine::open(&path)?;

        assert!(engine.entity(EntityID::new(9400)).is_some());
        assert!(engine.entity(EntityID::new(9401)).is_some());

        assert!(!engine.search("Snapshot Berry").is_empty());
        assert!(!engine.search("WAL Berry").is_empty());

        std::fs::remove_dir_all(path).ok();

        Ok(())
    }

    #[test]
    fn engine_does_not_wal_invalid_insert() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path();

        let mut engine = Engine::create(path)?;

        engine.insert(EntityInput {
            id: 9500,
            aliases: vec!["Existing Berry".into()],
            tags: vec![],
            properties: vec![],
            relationships: vec![],
            sources: vec![],
        })?;

        assert!(
            engine
                .insert(EntityInput {
                    id: 9500,
                    aliases: vec!["Duplicate Berry".into()],
                    tags: vec![],
                    properties: vec![],
                    relationships: vec![],
                    sources: vec![],
                })
                .is_err()
        );

        let operations = engine.backend().wal().replay()?;

        assert_eq!(operations.len(), 1);

        Ok(())
    }
}
