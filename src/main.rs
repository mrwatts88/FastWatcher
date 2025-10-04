mod cli;
mod core;

use anyhow::{Ok, Result};
use clap::Parser;
use cli::{Cli, Commands};
use core::files::reindex_all;
use core::search::run_search;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search { query } => {
            let results = run_search(query)?;
            println!("{results}");
        }
        Commands::Index => reindex_all()?,
    }

    Ok(())
}
