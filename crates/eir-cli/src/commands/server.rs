use std::path::PathBuf;

use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct ServerArgs {
    #[command(subcommand)]
    pub command: ServerCommand,
}

#[derive(Subcommand, Debug)]
pub enum ServerCommand {
    /// Start the EIR server
    Start {
        /// Database file
        database: PathBuf,

        //// Address to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        /// Port to listen on
        #[arg(short, long, default_value_t = 8765)]
        port: u16,
    },

    /// Stop the running EIR server
    Close,
}
