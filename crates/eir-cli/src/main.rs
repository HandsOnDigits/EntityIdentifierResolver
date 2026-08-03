mod cli;
mod commands;
mod debug;
mod generator;

use clap::CommandFactory;

use clap::Parser;
use clap_complete::{
    generate,
    shells::{Bash, Elvish, Fish, PowerShell, Zsh},
};

use cli::{Cli, Commands, Shell};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();

            match shell {
                Shell::Bash => {
                    generate(Bash, &mut cmd, "eir", &mut std::io::stdout());
                }

                Shell::Zsh => {
                    generate(Zsh, &mut cmd, "eir", &mut std::io::stdout());
                }

                Shell::Fish => {
                    generate(Fish, &mut cmd, "eir", &mut std::io::stdout());
                }

                Shell::PowerShell => {
                    generate(PowerShell, &mut cmd, "eir", &mut std::io::stdout());
                }

                Shell::Elvish => {
                    generate(Elvish, &mut cmd, "eir", &mut std::io::stdout());
                }
            }
        }

        Commands::Search { query } => {
            println!("Search: {}", query);
        }

        Commands::Generate { amount } => {
            println!("Generate {} entities", amount);
        }

        Commands::Index { command } => {
            println!("Index command: {:?}", command);
        }
    }

    Ok(())
}
