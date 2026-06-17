//! HexaKit CLI — fleet repo bootstrap.

mod boundary;
mod init;
mod registry;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hexakit", version, about = "HexaKit — Phenotype fleet scaffolding")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Bootstrap a new fleet repository (boundary, hooks, CI docs).
    Init(init::InitArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init(args) => init::run(args),
    }
}
