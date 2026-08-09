mod builder;
pub mod database;
mod loader;
mod search;

pub use builder::EngineBuilder;
pub use database::{Database, DatabaseRecord};
pub use loader::{load_database, load_database_owned};

use std::path::Path;

pub use search::SearchEngine;

use crate::{error::Result, index::Resolver};

pub struct Engine {
    database: Database,
    resolver: Resolver,
}

impl Engine {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let database = Database::load(path)?;
        let resolver = database.resolver();

        Ok(Self { database, resolver })
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
    fn engine_searches_loaded_database() {
        let engine = Engine::open("...").unwrap();

        let results = engine.search("Test Berry");

        assert!(!results.is_empty());
    }
}
