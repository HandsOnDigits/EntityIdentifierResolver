use clap::{Parser, Subcommand, ValueEnum};

use crate::commands::{
    compact::CompactArgs, insert::InsertArgs, inspect::InspectArgs, merge::MergeArgs,
    remove::RemoveArgs, search::SearchArgs, server::ServerArgs, stats::StatsArgs,
    update::UpdateArgs,
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
    /// Initialize an empty EIR database
    Init {
        /// Parent directory for the database
        parent: PathBuf,

        /// Database name
        name: String,
    },

    /// Build a database from an entity dataset
    Build {
        /// Input dataset
        #[arg(short, long)]
        input: PathBuf,

        /// Database directory
        #[arg(short, long)]
        database: PathBuf,
    },

    /// Show database statistics
    Stats(StatsArgs),

    /// Inspect one or more entities from database
    Inspect(InspectArgs),

    /// Search in database
    Search(SearchArgs),

    /// Insert entities into database
    Insert(InsertArgs),

    /// Remove entities from database
    Remove(RemoveArgs),

    /// Compact database storage
    Compact(CompactArgs),

    /// Update existing entity
    Update(UpdateArgs),

    Merge(MergeArgs),

    /// Manage the EIR server
    Server(ServerArgs),

    /// Generate shell completions
    Completions {
        shell: Shell,
    },
}

#[derive(Clone, ValueEnum, Debug)]
pub enum Shell {
    Bash,

    Zsh,

    Fish,

    #[value(name = "powershell")]
    Power,

    Elvish,
}
