mod filters;
mod intent;
pub mod parser;
pub mod types;

pub use filters::Filter;
pub use intent::QueryIntent;

use types::*;

pub struct Query {
    pub original: Message,
    pub normalized: Message,
    pub tokens: Vec<Token>,
    pub intent: QueryIntent,
    pub filters: Vec<Filter>,
}
