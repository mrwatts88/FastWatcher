use anyhow::Result;
use rusqlite::Connection;
use std::fs;

/// Connects (or creates) the database file and ensures all tables exist.
/// It loads and executes the bundled init.sql schema.
pub fn connect() -> Result<Connection> {
    let conn = Connection::open("fastwatcher.db")?;

    // Enable write-ahead logging and foreign keys for performance and integrity
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;

    Ok(conn)
}

pub fn execute_sql_file(conn: &Connection, path: &str) -> Result<()> {
    let sql = fs::read_to_string(path)?;
    conn.execute_batch(&sql)?;
    Ok(())
}
