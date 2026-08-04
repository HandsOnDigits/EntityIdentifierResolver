use std::path::Path;

use rkyv::{api::high::access, rancor::Error};

use crate::engine::database::ArchivedDatabase;

pub fn load_database(path: impl AsRef<Path>) -> anyhow::Result<&'static ArchivedDatabase> {
    let bytes = std::fs::read(path)?;

    let boxed = bytes.into_boxed_slice();
    let leaked: &'static [u8] = Box::leak(boxed);

    let database = access::<ArchivedDatabase, Error>(leaked)?;

    Ok(database)
}
