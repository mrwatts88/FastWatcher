use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "fast_watcher", version, about = "Offline watching CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search for sightings
    SearchSightings { query: String },

    /// Search for trips
    SearchTrips { query: String },

    /// Search for taxa
    SearchTaxa { query: String },

    /// Initialize the database and seed initial data
    InitDb,

    /// Drop all tables in the database (use with caution!)
    DropDb,

    // Trip commands
    /// Add a new trip
    AddTrip {
        name: String,
        #[arg(short, long)]
        date: Option<String>,
        #[arg(short, long)]
        location: Option<String>,
        #[arg(short, long)]
        notes: Option<String>,
    },

    /// Show trip details by ID
    ShowTrip { id: i64 },

    /// Delete a trip by ID
    DeleteTrip { id: i64 },

    // Taxon commands
    /// Add a new taxon
    AddTaxon {
        /// Taxonomic rank (kingdom, phylum, class, order, family, subfamily, genus, species)
        rank: String,
        /// Kingdom (required)
        kingdom: String,
        /// Common name
        common_name: String,
        #[arg(long)]
        phylum: Option<String>,
        #[arg(long)]
        class: Option<String>,
        #[arg(long)]
        order: Option<String>,
        #[arg(long)]
        family: Option<String>,
        #[arg(long)]
        subfamily: Option<String>,
        #[arg(long)]
        genus: Option<String>,
        #[arg(long)]
        species_epithet: Option<String>,
    },

    /// Show taxon details by ID
    ShowTaxon { id: i64 },

    /// Delete a taxon by ID
    DeleteTaxon { id: i64 },

    // Sighting commands
    /// Add a new sighting
    AddSighting {
        #[arg(short, long)]
        trip_id: Option<i64>,
        taxon_id: i64,
        #[arg(short, long)]
        notes: Option<String>,
        #[arg(short, long)]
        media_path: Option<String>,
        #[arg(short, long)]
        date: Option<String>,
        #[arg(short, long)]
        location: Option<String>,
    },

    /// Show sighting details by ID
    ShowSighting { id: i64 },

    /// Delete a sighting by ID
    DeleteSighting { id: i64 },
}
