mod builder;
pub mod database;
mod search;

use std::path::Path;

pub use builder::EngineBuilder;
pub use database::{Database, DatabaseRecord, DatabaseStats};

use crate::{
    config::{Config, DatabasePaths, StorageConfig},
    entity::prelude::{EntityDocument, input::EntityInput, types::EntityID},
    error::{Error, Result},
    index::Resolver,
    search::result::SearchResult,
    storage::{Backend, wal::WalOperation},
};

pub use search::SearchEngine;

pub struct Engine {
    backend: Backend,
    database: Database,
    resolver: Resolver,
}

impl Engine {
    /// Create a new logical `.eir` database.
    ///
    /// The `.eir` file is the public database identity.
    /// Physical storage is created alongside it.
    pub fn create(parent: impl AsRef<Path>, name: &str) -> Result<Self> {
        let paths = DatabasePaths::new(parent, name);

        std::fs::create_dir_all(&paths.root)?;

        let config = StorageConfig {
            root: paths.root.clone(),
            name: name.to_owned(),
            ..StorageConfig::default()
        };

        let backend = Backend::create(config)?;

        // The .eir file is the logical database identity.
        std::fs::File::create(&paths.database)?;

        let database = Database::default();
        let resolver = database.resolver();

        Ok(Self {
            backend,
            database,
            resolver,
        })
    }

    /// Open an existing logical `.eir` database.
    ///
    /// Physical storage is resolved from the logical database path.
    /// The snapshot is loaded first, followed by any pending WAL
    /// operations.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let paths = DatabasePaths::from_database(path);

        if !paths.database.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "database file does not exist",
            )));
        }

        let config = if paths.config.exists() {
            Config::load(&paths.config)?
        } else {
            Config {
                storage: StorageConfig {
                    name: paths
                        .database
                        .file_stem()
                        .and_then(|x| x.to_str())
                        .unwrap_or("database")
                        .to_string(),
                    root: paths.root.clone(),
                    ..StorageConfig::default()
                },
            }
        };

        let backend = Backend::open(config.storage)?;

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

    pub fn stats(&self) -> DatabaseStats {
        self.database.stats()
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
        let id = EntityID::new(input.id);

        if self.database.entity(id).is_some() {
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
    fn create_and_open_use_the_same_database_layout() -> Result<()> {
        let temp = tempfile::tempdir()?;

        let mut engine = Engine::create(temp.path(), "nutrition")?;

        engine.insert(EntityInput {
            id: 1000,
            aliases: vec!["Test Food".into()],
            tags: vec![],
            properties: vec![],
            relationships: vec![],
            sources: vec![],
        })?;

        engine.flush()?;
        drop(engine);

        let database_path = temp.path().join("nutrition").join("nutrition.eir");

        assert!(database_path.exists());

        let engine = Engine::open(&database_path)?;

        assert!(engine.entity(EntityID::new(1000)).is_some());

        Ok(())
    }

    #[test]
    fn engine_roundtrip_persists_database() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("test").join("test.eir");

        let mut engine = Engine::create(temp.path(), "test")?;

        engine.insert(EntityInput {
            id: 9200,
            aliases: vec!["Roundtrip Berry".into()],
            tags: vec![],
            properties: vec![],
            relationships: vec![],
            sources: vec![],
        })?;

        engine.flush()?;
        drop(engine);

        let engine = Engine::open(&path)?;

        let entity = engine.entity(EntityID::new(9200));

        assert!(entity.is_some());
        assert_eq!(entity.unwrap().aliases, vec!["Roundtrip Berry".into()]);

        let results = engine.search("Roundtrip Berry");

        assert!(!results.is_empty());
        assert_eq!(results[0].entity.id, EntityID::new(9200));

        Ok(())
    }

    #[test]
    fn engine_recovers_unflushed_insert_from_wal() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("test").join("test.eir");

        {
            let mut engine = Engine::create(temp.path(), "test")?;

            engine.insert(EntityInput {
                id: 9300,
                aliases: vec!["WAL Berry".into()],
                tags: vec![],
                properties: vec![],
                relationships: vec![],
                sources: vec![],
            })?;
        }

        let engine = Engine::open(&path)?;

        assert!(engine.entity(EntityID::new(9300)).is_some());
        assert!(!engine.search("WAL Berry").is_empty());

        Ok(())
    }

    #[test]
    fn engine_recovers_unflushed_remove_from_wal() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join("test").join("test.eir");

        {
            let mut engine = Engine::create(temp.path(), "test")?;

            engine.insert(EntityInput {
                id: 9400,
                aliases: vec!["Removed Berry".into()],
                tags: vec![],
                properties: vec![],
                relationships: vec![],
                sources: vec![],
            })?;

            engine.flush()?;
        }

        {
            let mut engine = Engine::open(&path)?;
            engine.remove(EntityID::new(9400))?;
        }

        let engine = Engine::open(&path)?;

        assert!(engine.entity(EntityID::new(9400)).is_none());
        assert!(engine.search("Removed Berry").is_empty());

        Ok(())
    }

    #[test]
    fn engine_flush_truncates_wal() -> Result<()> {
        let root = std::env::temp_dir();
        let name = format!("eir-engine-wal-flush-{}", std::process::id());

        let path = root.join(&name).join(format!("{name}.eir"));

        let mut engine = Engine::create(&root, &name)?;

        engine.insert(EntityInput {
            id: 9300,
            aliases: vec!["WAL Flush Berry".into()],
            tags: vec![],
            properties: vec![],
            relationships: vec![],
            sources: vec![],
        })?;

        assert!(!engine.backend().wal().replay()?.is_empty());

        engine.flush()?;

        assert!(engine.backend().wal().replay()?.is_empty());

        drop(engine);

        let engine = Engine::open(&path)?;

        assert!(engine.entity(EntityID::new(9300)).is_some());

        std::fs::remove_dir_all(root.join(&name)).ok();

        Ok(())
    }

    #[test]
    fn engine_does_not_wal_invalid_insert() -> Result<()> {
        let temp = tempfile::tempdir()?;
        let mut engine = Engine::create(temp.path(), "test")?;

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
