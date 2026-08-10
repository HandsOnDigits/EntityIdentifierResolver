use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use eir_core::engine::Engine;

#[derive(Args, Debug)]
pub struct SearchArgs {
    /// Database directory
    pub database: PathBuf,

    /// Search query
    pub query: String,

    /// Maximum results
    #[arg(short, long, default_value_t = 10)]
    pub limit: usize,
}

pub fn execute(args: SearchArgs) -> Result<()> {
    let engine = Engine::open(&args.database)?;

    let results = engine.search(&args.query);

    println!("Search: {}", args.query);
    println!();

    for result in results.into_iter().take(args.limit) {
        let name = result
            .entity
            .aliases
            .first()
            .map(|s| s.as_ref())
            .unwrap_or("Unknown");

        println!("{} score={:.2}", name, result.score);

        println!("  Signals:");
        for source in &result.sources {
            println!("    {:?}", source);
        }

        println!("  Why:");
        for explanation in &result.explanations {
            println!("    {:?}", explanation);
        }

        println!();
    }

    Ok(())
}
