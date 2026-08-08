use std::path::Path;

use super::input::EntityInput;

pub fn load_entities(path: impl AsRef<Path>) -> anyhow::Result<Vec<EntityInput>> {
    let json = std::fs::read_to_string(path)?;
    let entities = serde_json::from_str(&json)?;
    Ok(entities)
}
