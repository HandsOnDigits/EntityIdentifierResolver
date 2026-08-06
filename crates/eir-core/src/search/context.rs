use crate::{engine::Database, index::Resolver, query::Query};

use super::candidate::CandidateSet;

pub struct SearchContext<'a> {
    pub database: Option<&'a Database>,
    pub resolver: Resolver,
    pub query: &'a Query,
    pub candidates: CandidateSet,
}

impl<'a> SearchContext<'a> {
    pub fn new(database: &'a Database, query: &'a Query) -> Self {
        Self {
            database: Some(database),
            resolver: database.resolver(),
            query,
            candidates: CandidateSet::default(),
        }
    }

    pub fn with_resolver(database: &'a Database, query: &'a Query, resolver: Resolver) -> Self {
        Self {
            database: Some(database),
            resolver,
            query,
            candidates: CandidateSet::default(),
        }
    }
}
