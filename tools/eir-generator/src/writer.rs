use std::path::Path;

use crate::indexer::Indexes;
use eir_core::entity::EntityInput;
use eir_utils::registry::Registry;

pub fn write(
    path: impl AsRef<Path>,
    inputs: &[EntityInput],
    registry: &Registry,
    indexes: &Indexes,
) -> anyhow::Result<()> {
    let _ = (path.as_ref(), inputs, registry, indexes);
    todo!("serialize database")
}
