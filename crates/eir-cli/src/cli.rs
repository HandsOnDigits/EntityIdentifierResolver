use clap::{Parser, Subcommand, ValueEnum};

use crate::commands::{
    insert::InsertArgs, inspect::InspectArgs, remove::RemoveArgs, search::SearchArgs,
};

use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "eir", version, about = "Entity Identifier Resolver")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Build database from dataset
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

    /// Insert entities into database
    Insert(InsertArgs),

    /// Remove entities from database
    Remove(RemoveArgs),

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
