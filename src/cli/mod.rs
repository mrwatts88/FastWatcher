use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "fast_watcher", version, about = "Offline watching CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search for text in the archive
    Search { query: String },

    /// Reindex files into the SQLite cache
    Index,
}
