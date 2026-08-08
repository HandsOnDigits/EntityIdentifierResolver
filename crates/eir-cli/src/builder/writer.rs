use std::{fs, path::Path};

use rkyv::{rancor::Error, to_bytes};

use eir_core::engine::Database;

pub fn write_database(database: Database, path: impl AsRef<Path>) -> anyhow::Result<()> {
    let record = database.to_record();

    let bytes = to_bytes::<Error>(&record)?;

    fs::write(path, bytes)?;

    Ok(())
}
