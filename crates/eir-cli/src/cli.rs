use clap::{Parser, Subcommand, ValueEnum};

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

    Stats {
        input: PathBuf,
    },

    /// Build or manage indexes
    Index {
        #[command(subcommand)]
        command: IndexCommands,
    },

    /// Search entities
    Search {
        query: String,
    },

    /// Generate shell completions
    Completions {
        shell: Shell,
    },
}

#[derive(Subcommand, Debug)]
pub enum IndexCommands {
    Build {
        #[arg(short, long)]
        input: String,

        #[arg(short, long)]
        output: String,
    },

    Stats {
        path: String,
    },
}

#[derive(Clone, ValueEnum, Debug)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}
