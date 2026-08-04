use rkyv::{rancor, to_bytes};

use std::fs;

use eir_core::prelude::Database;

pub fn write_database(database: Database, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
    let bytes = to_bytes::<rancor::Error>(&database)?;

    fs::write(path, bytes)?;

    Ok(())
}
