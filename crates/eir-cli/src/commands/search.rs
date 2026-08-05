use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use eir_core::engine::load_database_owned;

#[derive(Args, Debug)]
pub struct SearchArgs {
    pub input: PathBuf,
    pub query: String,

    #[arg(short, long, default_value_t = 10)]
    pub limit: usize,
}

pub fn execute(args: SearchArgs) -> Result<()> {
    let database = load_database_owned(&args.input)?;
    let resolver = database.resolver();
    let results = resolver.search(&args.query);

    println!("Search: {}", args.query);
    println!();

    for result in results.into_iter().take(args.limit) {
        let name = result
            .entity
            .aliases
            .first()
            .map(|s| s.as_ref())
            .unwrap_or("Unknown");

        println!(
            "{}  score={:.2}  via={:?}",
            name, result.score, result.source
        );
    }

    Ok(())
}
