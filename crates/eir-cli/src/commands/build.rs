use std::path::PathBuf;

use crate::builder;

pub fn execute(input: PathBuf, database: PathBuf) -> anyhow::Result<()> {
    builder::pipeline::build(input, database)
}
