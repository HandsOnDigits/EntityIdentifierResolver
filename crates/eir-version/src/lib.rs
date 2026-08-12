mod api;
mod error;
mod merge;
mod validate;

pub use api::{MergeReport, merge};
pub use error::{Error, Result};
