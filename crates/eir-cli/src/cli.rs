use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "eir", version, about = "Entity Identifier Resolver")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Build or manage indexes
    Index {
        #[command(subcommand)]
        command: IndexCommands,
    },

    /// Search entities
    Search { query: String },

    /// Generate datasets
    Generate { amount: usize },

    /// Generate shell completions
    Completions { shell: Shell },
}

#[derive(Subcommand, Debug)]
pub enum IndexCommands {
    Build { path: String },

    Stats,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}
