mod cli;
mod core;
mod models;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use core::db::{connect, execute_sql_file};
use core::search::run_search;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search { query } => {
            let conn = connect()?;
            let results = run_search(&conn, &query)?;

            if results.is_empty() {
                println!("No matches found.");
            } else {
                for sighting in results {
                    println!(
                        "[{}] {} {} ({}) – trip {}",
                        sighting.id,
                        sighting.genus,
                        sighting.species_epithet,
                        sighting.common_name,
                        sighting.trip_id
                    );
                }
            }
        }

        Commands::InitDb => {
            let conn: rusqlite::Connection = connect()?;
            execute_sql_file(&conn, "init.sql")?;
            execute_sql_file(&conn, "seed_taxa.sql")?;
            execute_sql_file(&conn, "seed_trips.sql")?;
            execute_sql_file(&conn, "seed_sightings.sql")?;
            println!("Database initialized and seeded");
        }
    }

    Ok(())
}
