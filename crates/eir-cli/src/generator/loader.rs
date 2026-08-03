use rkyv::{access, deserialize, rancor::Error};

use eir_utils::fixture::FixtureEntity;

pub fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Vec<FixtureEntity>> {
    let bytes = std::fs::read(path)?;

    let archived = access::<rkyv::Archived<Vec<FixtureEntity>>, Error>(&bytes)?;

    let entities: Vec<FixtureEntity> = deserialize::<Vec<FixtureEntity>, Error>(archived)?;

    Ok(entities)
}
