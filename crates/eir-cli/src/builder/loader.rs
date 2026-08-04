use std::fs::File;
use std::path::Path;

use super::fixture::FixtureEntity;

pub fn load_entities(path: impl AsRef<Path>) -> anyhow::Result<Vec<FixtureEntity>> {
    let file = File::open(path)?;
    let entities: Vec<FixtureEntity> = serde_json::from_reader(file)?;

    Ok(entities)
}
