use anyhow::Result;
use std::fs;

use rkyv::{access, deserialize, rancor::Error}; // bring the trait into scope

use eir_core::storage::Store;

pub fn read_database() -> Result<()> {
    let bytes = fs::read("eir.db")?;
    let archived = access::<rkyv::Archived<Store>, Error>(&bytes)?;

    let database: Store = deserialize::<Store, Error>(archived)?;
    println!("{:#?}", database);

    Ok(())
}
