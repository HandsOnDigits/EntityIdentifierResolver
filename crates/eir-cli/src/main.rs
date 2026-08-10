mod builder;
mod cli;
mod commands;

use clap::CommandFactory;

use clap::Parser;
use clap_complete::{
    generate,
    shells::{Bash, Elvish, Fish, PowerShell, Zsh},
};

use cli::{Cli, Commands, Shell};

fn generate_completions(shell: Shell) -> std::io::Result<()> {
    let mut cmd = Cli::command();
    let mut stdout = std::io::stdout();

    match shell {
        Shell::Bash => generate(Bash, &mut cmd, "eir", &mut stdout),
        Shell::Zsh => generate(Zsh, &mut cmd, "eir", &mut stdout),
        Shell::Fish => generate(Fish, &mut cmd, "eir", &mut stdout),
        Shell::Power => generate(PowerShell, &mut cmd, "eir", &mut stdout),
        Shell::Elvish => generate(Elvish, &mut cmd, "eir", &mut stdout),
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Completions { shell } => {
            generate_completions(shell)?;
        }

        Commands::Init { parent, name } => {
            commands::init::execute(commands::init::InitArgs { parent, name })?;
        }

        Commands::Search(args) => {
            commands::search::execute(args)?;
        }

        Commands::Build { input, database } => {
            commands::build::execute(input, database)?;
        }

        Commands::Stats(args) => {
            commands::stats::execute(args)?;
        }

        Commands::Inspect(args) => {
            commands::inspect::execute(args)?;
        }

        Commands::Insert(args) => {
            commands::insert::execute(args)?;
        }

        Commands::Remove(args) => {
            commands::remove::execute(args)?;
        }

        Commands::Server(_args) => {
            todo!("Server is not yet setup");
        }
    }

    Ok(())
}
