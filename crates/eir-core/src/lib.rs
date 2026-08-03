pub mod config;
pub mod engine;
pub mod entity;
pub mod error;
pub mod index;
pub mod storage;

pub use engine::Database;

pub mod prelude {
    pub use crate::{
        engine::{Database, Engine, EngineBuilder},
        entity::types::*,
        entity::*,
        storage::{Registry, Store, indexes::Indexes, posting_list::PostingList},
    };
}
