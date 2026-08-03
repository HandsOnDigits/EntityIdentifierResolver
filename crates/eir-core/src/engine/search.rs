use crate::{entity::EntityDocument, error::Result};

pub struct SearchResult {
    pub entity: EntityDocument,
    pub score: f32,
}

pub trait SearchEngine {
    fn search(&self, query: &str) -> Result<Vec<SearchResult>>;
}
