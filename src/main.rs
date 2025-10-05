mod cli;
mod core;
mod models;
mod ui;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use core::db::{connect, drop_all_tables, execute_sql_file};
use core::search::{run_search_sightings, run_search_taxa, run_search_trips};
use core::sighting::{create_sighting, delete_sighting, get_sighting_by_id};
use core::taxon::{create_taxon, delete_taxon, get_taxon_by_id};
use core::trip::{create_trip, delete_trip, get_trip_by_id};

fn main() -> Result<()> {
    // Check if any CLI arguments were provided
    let args: Vec<String> = std::env::args().collect();

    // If no arguments (just the binary name), launch GUI
    if args.len() == 1 {
        ui::run_ui()?;
        return Ok(());
    }

    // Otherwise, run CLI
    let cli = Cli::parse();

    match cli.command {
        Commands::SearchSightings { query } => {
            let conn = connect()?;
            let results = run_search_sightings(&conn, &query)?;

            if results.is_empty() {
                println!("No matches found.");
            } else {
                for sighting in results {
                    println!("{}", sighting);
                }
            }
        }

        Commands::SearchTrips { query } => {
            let conn = connect()?;
            let results = run_search_trips(&conn, &query)?;

            if results.is_empty() {
                println!("No matches found.");
            } else {
                for trip in results {
                    println!("{}", trip);
                }
            }
        }

        Commands::SearchTaxa { query } => {
            let conn = connect()?;
            let results = run_search_taxa(&conn, &query)?;

            if results.is_empty() {
                println!("No matches found.");
            } else {
                for taxon in results {
                    println!("{}", taxon);
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

        Commands::DropDb => {
            let conn = connect()?;
            drop_all_tables(&conn)?;
            println!("All tables dropped. Use with caution!");
        }

        // Trip commands
        Commands::AddTrip {
            name,
            date,
            location,
            notes,
        } => {
            let conn = connect()?;
            let id = create_trip(
                &conn,
                &name,
                date.as_deref(),
                location.as_deref(),
                notes.as_deref(),
            )?;
            println!("Trip created with ID: {}", id);
        }

        Commands::ShowTrip { id } => {
            let conn = connect()?;
            let trip = get_trip_by_id(&conn, id)?;
            println!("{}", trip);
        }

        Commands::DeleteTrip { id } => {
            let conn = connect()?;
            let rows = delete_trip(&conn, id)?;
            if rows > 0 {
                println!("Trip {} deleted", id);
            } else {
                println!("Trip {} not found", id);
            }
        }

        // Taxon commands
        Commands::AddTaxon {
            rank,
            kingdom,
            common_name,
            phylum,
            class,
            order,
            family,
            subfamily,
            genus,
            species_epithet,
        } => {
            let conn = connect()?;
            let id = create_taxon(
                &conn,
                &rank,
                &kingdom,
                phylum.as_deref(),
                class.as_deref(),
                order.as_deref(),
                family.as_deref(),
                subfamily.as_deref(),
                genus.as_deref(),
                species_epithet.as_deref(),
                &common_name,
            )?;
            println!("Taxon created with ID: {}", id);
        }

        Commands::ShowTaxon { id } => {
            let conn = connect()?;
            let taxon = get_taxon_by_id(&conn, id)?;
            println!("{}", taxon);
        }

        Commands::DeleteTaxon { id } => {
            let conn = connect()?;
            let rows = delete_taxon(&conn, id)?;
            if rows > 0 {
                println!("Taxon {} deleted", id);
            } else {
                println!("Taxon {} not found", id);
            }
        }

        // Sighting commands
        Commands::AddSighting {
            trip_id,
            taxon_id,
            notes,
            media_path,
            date,
            location,
        } => {
            let conn = connect()?;
            let id = create_sighting(
                &conn,
                trip_id,
                taxon_id,
                notes.as_deref(),
                media_path.as_deref(),
                date.as_deref(),
                location.as_deref(),
            )?;
            println!("Sighting created with ID: {}", id);
        }

        Commands::ShowSighting { id } => {
            let conn = connect()?;
            let sighting = get_sighting_by_id(&conn, id)?;
            println!("{}", sighting);
        }

        Commands::DeleteSighting { id } => {
            let conn = connect()?;
            let rows = delete_sighting(&conn, id)?;
            if rows > 0 {
                println!("Sighting {} deleted", id);
            } else {
                println!("Sighting {} not found", id);
            }
        }
    }

    Ok(())
}
