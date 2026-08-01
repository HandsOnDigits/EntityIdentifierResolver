use super::EntityID;

pub struct SearchResult<T> {
    pub source: String,
    pub id: String,
    pub score: f32,
    pub data: T,
}

pub struct ResolvedResult<T> {
    pub result: SearchResult<T>,
    pub entity: Option<EntityID>,
    pub confidence: f32,
}

pub trait EntityProvider {
    type Data;

    fn search(&self, query: &str) -> Vec<SearchResult<Self::Data>>;
}

pub trait EntityResolver {
    type Data;

    fn resolve(&self, results: Vec<SearchResult<Self::Data>>) -> Vec<ResolvedResult<Self::Data>>;
}
