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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "foreign_keys", "ON").unwrap();

        let schema = std::fs::read_to_string("init.sql").unwrap();
        conn.execute_batch(&schema).unwrap();

        conn
    }

    #[test]
    fn test_create_trip_with_all_fields() {
        let conn = setup_test_db();

        let id = create_trip(
            &conn,
            "Morning Birding",
            Some("2025-01-15"),
            Some("Central Park"),
            Some("Cold morning, lots of activity"),
        ).unwrap();

        assert!(id > 0);

        let trip = get_trip_by_id(&conn, id).unwrap();
        assert_eq!(trip.name, "Morning Birding");
        assert_eq!(trip.date, Some("2025-01-15".to_string()));
        assert_eq!(trip.location, Some("Central Park".to_string()));
        assert_eq!(trip.notes, Some("Cold morning, lots of activity".to_string()));
    }

    #[test]
    fn test_create_trip_minimal_fields() {
        let conn = setup_test_db();

        let id = create_trip(&conn, "Quick Walk", None, None, None).unwrap();

        let trip = get_trip_by_id(&conn, id).unwrap();
        assert_eq!(trip.name, "Quick Walk");
        assert_eq!(trip.date, None);
        assert_eq!(trip.location, None);
        assert_eq!(trip.notes, None);
    }

    #[test]
    fn test_delete_trip() {
        let conn = setup_test_db();

        let id = create_trip(&conn, "Test Trip", None, None, None).unwrap();
        let rows = delete_trip(&conn, id).unwrap();
        assert_eq!(rows, 1);

        let result = get_trip_by_id(&conn, id);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_nonexistent_trip() {
        let conn = setup_test_db();
        let result = get_trip_by_id(&conn, 99999);
        assert!(result.is_err());
    }
}
