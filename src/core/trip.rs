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

/// Get all trips where a specific taxon was sighted (matches based on taxonomic hierarchy)
pub fn get_trips_by_taxon(conn: &Connection, taxon: &crate::models::Taxon) -> Result<Vec<Trip>> {
    // Build WHERE clause based on taxon rank
    let mut conditions = vec!["sightings.kingdom = ?1".to_string()];
    let mut params: Vec<&str> = vec![&taxon.kingdom];

    if taxon.rank != "kingdom" {
        if let Some(ref p) = taxon.phylum {
            conditions.push("sightings.phylum = ?".to_string());
            params.push(p);
        }
    }

    if taxon.rank != "kingdom" && taxon.rank != "phylum" {
        if let Some(ref c) = taxon.class {
            conditions.push("sightings.class = ?".to_string());
            params.push(c);
        }
    }

    if taxon.rank == "order" || taxon.rank == "family" || taxon.rank == "subfamily" || taxon.rank == "genus" || taxon.rank == "species" {
        if let Some(ref o) = taxon.order {
            conditions.push("sightings.\"order\" = ?".to_string());
            params.push(o);
        }
    }

    if taxon.rank == "family" || taxon.rank == "subfamily" || taxon.rank == "genus" || taxon.rank == "species" {
        if let Some(ref f) = taxon.family {
            conditions.push("sightings.family = ?".to_string());
            params.push(f);
        }
    }

    if taxon.rank == "subfamily" || taxon.rank == "genus" || taxon.rank == "species" {
        if let Some(ref sf) = taxon.subfamily {
            conditions.push("sightings.subfamily = ?".to_string());
            params.push(sf);
        }
    }

    if taxon.rank == "genus" || taxon.rank == "species" {
        if let Some(ref g) = taxon.genus {
            conditions.push("sightings.genus = ?".to_string());
            params.push(g);
        }
    }

    if taxon.rank == "species" {
        if let Some(ref s) = taxon.species_epithet {
            conditions.push("sightings.species_epithet = ?".to_string());
            params.push(s);
        }
    }

    let where_clause = conditions.join(" AND ");
    let sql = format!(
        r#"
        SELECT DISTINCT trips.id, trips.name, trips.date, trips.location, trips.notes
        FROM trips
        INNER JOIN sightings ON sightings.trip_id = trips.id
        WHERE {}
        ORDER BY trips.date DESC, trips.id DESC
        "#,
        where_clause
    );

    let mut stmt = conn.prepare(&sql)
        .context("Failed to prepare get trips by taxon query")?;

    let rows = stmt.query_map(rusqlite::params_from_iter(params.iter()), |row| {
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
            None,
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

        let taxon = crate::core::taxon::get_taxon_by_id(&conn, taxon_id).unwrap();
        let results = get_trips_by_taxon(&conn, &taxon).unwrap();
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
            None,
            Some("Turdus"),
            Some("migratorius"),
            "American Robin",
        ).unwrap();

        let taxon = crate::core::taxon::get_taxon_by_id(&conn, taxon_id).unwrap();
        let results = get_trips_by_taxon(&conn, &taxon).unwrap();
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
            None,
            Some("Turdus"),
            Some("migratorius"),
            "American Robin",
        ).unwrap();

        // Create sighting without trip
        create_sighting(&conn, None, taxon_id, None, None, None, None).unwrap();

        let taxon = crate::core::taxon::get_taxon_by_id(&conn, taxon_id).unwrap();
        let results = get_trips_by_taxon(&conn, &taxon).unwrap();
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
            None,
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
        let taxon = crate::core::taxon::get_taxon_by_id(&conn, taxon_id).unwrap();
        let results = get_trips_by_taxon(&conn, &taxon).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Morning Walk");
    }

    #[test]
    fn test_get_trips_by_subfamily_taxon() {
        let conn = setup_test_db();

        use crate::core::taxon::create_taxon;
        use crate::core::sighting::create_sighting;

        // Create subfamily taxon
        let subfamily_id = create_taxon(
            &conn,
            "subfamily",
            "Animalia",
            Some("Chordata"),
            Some("Aves"),
            Some("Passeriformes"),
            Some("Corvidae"),
            Some("Corvinae"),
            None,
            None,
            "Corvinae",
        ).unwrap();

        // Create species within that subfamily
        let species_id = create_taxon(
            &conn,
            "species",
            "Animalia",
            Some("Chordata"),
            Some("Aves"),
            Some("Passeriformes"),
            Some("Corvidae"),
            Some("Corvinae"),
            Some("Corvus"),
            Some("corax"),
            "Common Raven",
        ).unwrap();

        // Create trips
        let trip1 = create_trip(&conn, "Trip 1", Some("2025-01-15"), None, None).unwrap();
        let trip2 = create_trip(&conn, "Trip 2", Some("2025-01-20"), None, None).unwrap();
        let trip3 = create_trip(&conn, "Trip 3", Some("2025-01-25"), None, None).unwrap();

        // Create sightings: trip1 has subfamily sighting, trip2 has species sighting, trip3 has neither
        create_sighting(&conn, Some(trip1), subfamily_id, None, None, None, None).unwrap();
        create_sighting(&conn, Some(trip2), species_id, None, None, None, None).unwrap();

        // Query by subfamily should return both trip1 and trip2
        let subfamily_taxon = crate::core::taxon::get_taxon_by_id(&conn, subfamily_id).unwrap();
        let results = get_trips_by_taxon(&conn, &subfamily_taxon).unwrap();
        assert_eq!(results.len(), 2);

        // Verify correct trips returned
        let trip_names: Vec<String> = results.iter().map(|t| t.name.clone()).collect();
        assert!(trip_names.contains(&"Trip 1".to_string()));
        assert!(trip_names.contains(&"Trip 2".to_string()));
    }

    #[test]
    fn test_get_trips_by_family_includes_subfamily_sightings() {
        let conn = setup_test_db();

        use crate::core::taxon::create_taxon;
        use crate::core::sighting::create_sighting;

        // Create family taxon
        let family_id = create_taxon(
            &conn,
            "family",
            "Animalia",
            Some("Chordata"),
            Some("Aves"),
            Some("Passeriformes"),
            Some("Corvidae"),
            None,
            None,
            None,
            "Corvidae",
        ).unwrap();

        // Create subfamily within that family
        let subfamily_id = create_taxon(
            &conn,
            "subfamily",
            "Animalia",
            Some("Chordata"),
            Some("Aves"),
            Some("Passeriformes"),
            Some("Corvidae"),
            Some("Corvinae"),
            None,
            None,
            "Corvinae",
        ).unwrap();

        // Create trip with subfamily sighting
        let trip_id = create_trip(&conn, "Corvid Trip", Some("2025-01-15"), None, None).unwrap();
        create_sighting(&conn, Some(trip_id), subfamily_id, None, None, None, None).unwrap();

        // Query by family should return trip (family includes its subfamilies)
        let family_taxon = crate::core::taxon::get_taxon_by_id(&conn, family_id).unwrap();
        let results = get_trips_by_taxon(&conn, &family_taxon).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Corvid Trip");
    }
}
