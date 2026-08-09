use crate::error::{Error, Result};
use std::path::Path;

use super::input::EntityInput;

pub fn load_entities(path: impl AsRef<Path>) -> Result<Vec<EntityInput>> {
    let json = std::fs::read_to_string(path)?;
    let entities =
        serde_json::from_str(&json).map_err(|error| Error::Serialization(error.to_string()))?;
    Ok(entities)
}
