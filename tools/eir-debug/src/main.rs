mod database;
mod read;

use anyhow::Result;

fn main() -> Result<()> {
    read::read_database()?;

    Ok(())
}
