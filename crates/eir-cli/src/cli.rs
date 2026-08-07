use clap::{Parser, Subcommand, ValueEnum};

use crate::commands::{inspect::InspectArgs, search::SearchArgs};

use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "eir", version, about = "Entity Identifier Resolver")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Build datasets
    Build {
        /// Input dataset
        #[arg(short, long)]
        input: PathBuf,

        /// Output database
        #[arg(short, long)]
        output: PathBuf,
    },

    /// Show database statistics
    Stats { input: PathBuf },

    /// Inspect an entity in the database
    Inspect(InspectArgs),

    /// Search entities
    Search(SearchArgs),

    /// Generate shell completions
    Completions { shell: Shell },
}

#[derive(Clone, ValueEnum, Debug)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}
