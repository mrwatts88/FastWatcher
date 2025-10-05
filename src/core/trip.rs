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

/// Get all trips where a specific taxon was sighted
pub fn get_trips_by_taxon_id(conn: &Connection, taxon_id: i64) -> Result<Vec<Trip>> {
    let sql = r#"
        SELECT DISTINCT trips.id, trips.name, trips.date, trips.location, trips.notes
        FROM trips
        INNER JOIN sightings ON sightings.trip_id = trips.id
        WHERE sightings.taxon_id = ?1
        ORDER BY trips.date DESC, trips.id DESC
    "#;

    let mut stmt = conn.prepare(sql)
        .context("Failed to prepare get trips by taxon query")?;

    let rows = stmt.query_map(params![taxon_id], |row| {
        Ok(Trip {
            id: row.get(0)?,
            name: row.get(1)?,
            date: row.get(2)?,
            location: row.get(3)?,
            notes: row.get(4)?,
        })
    }).context("Failed to execute get trips by taxon query")?;

    let results: Vec<Trip> = rows.collect::<Result<Vec<_>, _>>()
        .context("Failed to parse trip rows")?;
    Ok(results)
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

    #[test]
    fn test_get_trips_by_taxon_id() {
        let conn = setup_test_db();

        use crate::core::taxon::create_taxon;
        use crate::core::sighting::create_sighting;

        // Create a taxon
        let taxon_id = create_taxon(
            &conn,
            "species",
            "Animalia",
            Some("Chordata"),
            Some("Aves"),
            Some("Passeriformes"),
            Some("Corvidae"),
            Some("Cyanocitta"),
            Some("cristata"),
            "Blue Jay",
        ).unwrap();

        // Create 3 trips
        let trip1 = create_trip(&conn, "Morning Walk", Some("2025-01-20"), Some("Park"), None).unwrap();
        let trip2 = create_trip(&conn, "Afternoon Hike", Some("2025-01-15"), Some("Trail"), None).unwrap();
        let trip3 = create_trip(&conn, "Evening Stroll", Some("2025-01-25"), Some("Beach"), None).unwrap();

        // Create sightings of the taxon on trip1 and trip2 (not trip3)
        create_sighting(&conn, Some(trip1), taxon_id, None, None, None, None).unwrap();
        create_sighting(&conn, Some(trip2), taxon_id, None, None, None, None).unwrap();

        let results = get_trips_by_taxon_id(&conn, taxon_id).unwrap();
        assert_eq!(results.len(), 2);

        // Should be ordered by date DESC (trip1: 2025-01-20, trip2: 2025-01-15)
        assert_eq!(results[0].name, "Morning Walk");
        assert_eq!(results[1].name, "Afternoon Hike");

        // Verify trip3 is not in results (no sightings)
        assert!(!results.iter().any(|t| t.id == trip3));
    }

    #[test]
    fn test_get_trips_by_taxon_id_no_trips() {
        let conn = setup_test_db();

        use crate::core::taxon::create_taxon;

        // Create a taxon with no sightings
        let taxon_id = create_taxon(
            &conn,
            "species",
            "Animalia",
            Some("Chordata"),
            Some("Aves"),
            Some("Passeriformes"),
            Some("Turdidae"),
            Some("Turdus"),
            Some("migratorius"),
            "American Robin",
        ).unwrap();

        let results = get_trips_by_taxon_id(&conn, taxon_id).unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_get_trips_by_taxon_id_with_sightings_without_trip() {
        let conn = setup_test_db();

        use crate::core::taxon::create_taxon;
        use crate::core::sighting::create_sighting;

        let taxon_id = create_taxon(
            &conn,
            "species",
            "Animalia",
            Some("Chordata"),
            Some("Aves"),
            Some("Passeriformes"),
            Some("Turdidae"),
            Some("Turdus"),
            Some("migratorius"),
            "American Robin",
        ).unwrap();

        // Create sighting without trip
        create_sighting(&conn, None, taxon_id, None, None, None, None).unwrap();

        let results = get_trips_by_taxon_id(&conn, taxon_id).unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_get_trips_by_taxon_id_distinct() {
        let conn = setup_test_db();

        use crate::core::taxon::create_taxon;
        use crate::core::sighting::create_sighting;

        let taxon_id = create_taxon(
            &conn,
            "species",
            "Animalia",
            Some("Chordata"),
            Some("Aves"),
            Some("Passeriformes"),
            Some("Corvidae"),
            Some("Cyanocitta"),
            Some("cristata"),
            "Blue Jay",
        ).unwrap();

        let trip_id = create_trip(&conn, "Morning Walk", Some("2025-01-15"), Some("Park"), None).unwrap();

        // Create multiple sightings of same taxon on same trip
        create_sighting(&conn, Some(trip_id), taxon_id, None, None, None, None).unwrap();
        create_sighting(&conn, Some(trip_id), taxon_id, None, None, None, None).unwrap();
        create_sighting(&conn, Some(trip_id), taxon_id, None, None, None, None).unwrap();

        // Should return trip only once (DISTINCT)
        let results = get_trips_by_taxon_id(&conn, taxon_id).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Morning Walk");
    }
}
