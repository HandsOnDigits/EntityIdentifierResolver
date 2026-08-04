use std::path::PathBuf;

use crate::builder;

pub fn execute(input: PathBuf, output: PathBuf) -> anyhow::Result<()> {
    builder::pipeline::build(input, output)
}
