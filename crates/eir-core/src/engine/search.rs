use crate::index::SearchResult;

use super::Engine;

#[allow(dead_code)]
pub trait SearchEngine {
    fn search<'a>(&'a self, query: &str) -> Vec<SearchResult<'a>>;
}

impl SearchEngine for Engine {
    fn search<'a>(&'a self, query: &str) -> Vec<SearchResult<'a>> {
        self.resolver.search(query)
    }
}
