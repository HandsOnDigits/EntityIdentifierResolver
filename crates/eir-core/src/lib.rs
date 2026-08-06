pub mod config;
pub mod engine;
pub mod entity;
pub mod error;
pub mod index;
pub mod query;
pub mod search;
pub mod storage;
pub mod utils;

pub use engine::Database;
pub use storage::{IndexBuilder, Indexes};

pub mod prelude {
    pub use crate::{
        engine::{Database, Engine, EngineBuilder},
        entity::types::*,
        entity::*,
        storage::{IndexBuilder, Indexes, PostingList, PostingListRecord, Registry, Store},
    };
}
