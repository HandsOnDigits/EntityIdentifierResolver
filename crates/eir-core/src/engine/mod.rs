mod builder;
pub mod database;
mod loader;
mod search;

pub use builder::EngineBuilder;
pub use database::Database;
pub use loader::{load_database, load_database_owned};

use std::path::Path;

use crate::{error::Result, index::Resolver, storage::Backend};

pub struct Engine {
    backend: Backend,
    resolver: Resolver,
}

impl Engine {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            backend: Backend::open(path)?,
            resolver: Resolver::default(),
        })
    }

    pub fn flush(&self) -> Result<()> {
        self.backend.flush()
    }
}
