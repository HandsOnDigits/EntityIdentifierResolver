mod database;
mod indexer;
mod loader;
mod mapper;
mod pipeline;
mod writer;

use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let input = PathBuf::from("fixtures/entities.json");
    let output = PathBuf::from("eir.db");

    pipeline::generate(input, output)?;

    Ok(())
}
