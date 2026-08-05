use std::{fs, path::Path};

use rkyv::{api::high::access, from_bytes, rancor::Error};

use crate::engine::database::{ArchivedDatabase, Database};

pub fn load_database(path: impl AsRef<Path>) -> anyhow::Result<&'static ArchivedDatabase> {
    let bytes = std::fs::read(path)?;

    let boxed = bytes.into_boxed_slice();
    let leaked: &'static [u8] = Box::leak(boxed);

    let database = access::<ArchivedDatabase, Error>(leaked)?;

    Ok(database)
}

pub fn load_database_owned(path: impl AsRef<Path>) -> anyhow::Result<Database> {
    let bytes = fs::read(path)?;
    let database = from_bytes::<Database, Error>(&bytes)?;
    Ok(database)
}
