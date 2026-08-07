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
        Shell::PowerShell => generate(PowerShell, &mut cmd, "eir", &mut stdout),
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

        Commands::Search(args) => commands::search::execute(args)?,

        Commands::Build { input, output } => {
            commands::build::execute(input, output)?;
        }

        Commands::Stats { input } => {
            commands::stats::execute(input)?;
        }

        Commands::Inspect(args) => {
            commands::inspect::execute(args)?;
        }
    }

    Ok(())
}
