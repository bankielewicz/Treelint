//! Treelint CLI Entry Point
//!
//! This is the main entry point for the Treelint CLI application.
//! It parses command-line arguments and routes to the appropriate command.

use clap::Parser;
use treelint::cli::commands;
use treelint::cli::{Args, Commands};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Search(search_args) => {
            commands::execute_search(search_args)?;
        }
    }

    Ok(())
}
