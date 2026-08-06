use crate::{Database, query::Query};

use super::candidate::CandidateSet;

pub struct SearchContext<'a> {
    pub database: &'a Database,
    pub query: &'a Query,
    pub candidates: CandidateSet,
}
