pub mod config;
pub mod engine;
pub mod entity;
pub mod error;
mod index;
pub mod query;
pub mod search;
pub mod storage;
pub mod utils;

#[cfg(test)]
pub mod test_utils;

pub use engine::{Database, DatabaseRecord};
pub use storage::{IndexBuilder, Indexes};

pub mod prelude {
    pub use super::entity;

    pub mod engine {
        pub use crate::engine::{Database, DatabaseRecord, Engine, EngineBuilder};
    }

    pub mod index {
        pub use crate::index::*;
    }

    pub mod storage {
        pub use crate::storage::{
            IndexBuilder, IndexRecord, Indexes, PostingList, PostingListRecord, Registry,
            RegistryRecord, Store,
        };
    }
}
