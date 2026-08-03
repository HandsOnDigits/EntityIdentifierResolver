use std::path::PathBuf;

use crate::generator;

pub fn generate(input: PathBuf, output: PathBuf) -> anyhow::Result<()> {
    generator::pipeline::generate(input, output)
}
