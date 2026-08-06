use super::{Filter, QueryIntent};

pub type Token = Box<str>;
pub type Message = Box<str>;

pub struct Query {
    pub original: Message,
    pub normalized: Message,
    pub tokens: Vec<Token>,
    pub intent: QueryIntent,
    pub filters: Vec<Filter>,
}
