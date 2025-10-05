use crate::models::Trip;
use anyhow::{Context, Result};
use rusqlite::{Connection, params};

/// Create a new trip
pub fn create_trip(
    conn: &Connection,
    name: &str,
    date: Option<&str>,
    location: Option<&str>,
    notes: Option<&str>,
) -> Result<i64> {
    let sql = r#"
        INSERT INTO trips (name, date, location, notes)
        VALUES (?1, ?2, ?3, ?4)
    "#;

    conn.execute(sql, params![name, date, location, notes])
        .context("Failed to insert trip")?;

    let id = conn.last_insert_rowid();
    Ok(id)
}

/// Get a trip by ID
pub fn get_trip_by_id(conn: &Connection, id: i64) -> Result<Trip> {
    let sql = r#"
        SELECT id, name, date, location, notes
        FROM trips
        WHERE id = ?1
    "#;

    let trip = conn.query_row(sql, params![id], |row| {
        Ok(Trip {
            id: row.get(0)?,
            name: row.get(1)?,
            date: row.get(2)?,
            location: row.get(3)?,
            notes: row.get(4)?,
        })
    }).context("Failed to fetch trip")?;

    Ok(trip)
}

/// Delete a trip by ID
pub fn delete_trip(conn: &Connection, id: i64) -> Result<usize> {
    let sql = "DELETE FROM trips WHERE id = ?1";
    let rows_affected = conn.execute(sql, params![id])
        .context("Failed to delete trip")?;
    Ok(rows_affected)
}
