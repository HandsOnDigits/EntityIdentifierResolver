use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use eir_version::api::merge;

#[derive(Args, Debug)]
pub struct MergeArgs {
    pub left: PathBuf,
    pub right: PathBuf,
    pub output: PathBuf,
}

pub fn execute(args: MergeArgs) -> Result<()> {
    let report = merge(&args.left, &args.right, &args.output)?;

    println!("Merge complete.");
    println!("Entities added: {}", report.entities_added);
    println!("Entities skipped: {}", report.entities_skipped);

    Ok(())
}
