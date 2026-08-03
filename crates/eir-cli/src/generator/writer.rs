use eir_core::prelude::Database;
use rkyv::{rancor, to_bytes};

pub fn write_database(database: Database) -> anyhow::Result<Vec<u8>> {
    let bytes = to_bytes::<rancor::Error>(&database)?;

    Ok(bytes.to_vec())
}
