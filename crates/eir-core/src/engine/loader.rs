use crate::error::{Error, Result};
use std::{fs, path::Path};

use crate::engine::database::{Database, DatabaseRecord};

pub fn load_database(path: impl AsRef<Path>) -> Result<Database> {
    let bytes = std::fs::read(path)?;

    let record = rkyv::from_bytes::<DatabaseRecord, rkyv::rancor::Error>(&bytes)
        .map_err(|error| Error::Serialization(error.to_string()))?;

    Ok(Database::from_record(record))
}

pub fn load_database_owned(path: impl AsRef<Path>) -> Result<Database> {
    let bytes = fs::read(path)?;

    let record = rkyv::from_bytes::<DatabaseRecord, rkyv::rancor::Error>(&bytes)
        .map_err(|error| Error::Serialization(error.to_string()))?;

    let database = Database::from_record(record);

    Ok(database)
}
