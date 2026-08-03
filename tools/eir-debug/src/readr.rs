use anyhow::Result;
use std::fs;

use rkyv::{access, deserialize, rancor::Error}; // bring the trait into scope

use eir_core::storage::database::Database;

pub fn read_database() -> Result<()> {
    let bytes = fs::read("eir.db")?;
    let archived = access::<rkyv::Archived<Database>, Error>(&bytes)?;

    let database: Database = deserialize::<Database, Error>(archived)?;
    println!("{:#?}", database);

    Ok(())
}
