use crate::models::Sighting;
use anyhow::{Context, Result};
use rusqlite::{Connection, params};

/// Create a new sighting (looks up taxon data automatically)
pub fn create_sighting(
    conn: &Connection,
    trip_id: Option<i64>,
    taxon_id: i64,
    notes: Option<&str>,
    media_path: Option<&str>,
    date: Option<&str>,
    location: Option<&str>,
) -> Result<i64> {
    // Look up the taxon to get taxonomic fields
    let taxon_sql = r#"
        SELECT kingdom, phylum, class, "order", family, genus, species_epithet, common_name
        FROM taxa
        WHERE id = ?1
    "#;

    let (kingdom, phylum, class, order, family, genus, species_epithet, common_name): (
        String,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        String,
    ) = conn
        .query_row(taxon_sql, params![taxon_id], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
            ))
        })
        .context("Failed to fetch taxon for sighting")?;

    // Insert sighting with duplicated taxonomic fields
    let sql = r#"
        INSERT INTO sightings (
            trip_id, taxon_id, kingdom, phylum, class, "order", family,
            genus, species_epithet, common_name, notes, media_path, date, location
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
    "#;

    conn.execute(
        sql,
        params![
            trip_id,
            taxon_id,
            kingdom,
            phylum,
            class,
            order,
            family,
            genus,
            species_epithet,
            common_name,
            notes,
            media_path,
            date,
            location
        ],
    )
    .context("Failed to insert sighting")?;

    let id = conn.last_insert_rowid();
    Ok(id)
}

/// Get a sighting by ID
pub fn get_sighting_by_id(conn: &Connection, id: i64) -> Result<Sighting> {
    let sql = r#"
        SELECT id, trip_id, taxon_id, kingdom, phylum, class, "order", family,
               genus, species_epithet, common_name, notes, media_path, date, location
        FROM sightings
        WHERE id = ?1
    "#;

    let sighting = conn.query_row(sql, params![id], |row| {
        Ok(Sighting {
            id: row.get(0)?,
            trip_id: row.get(1)?,
            taxon_id: row.get(2)?,
            kingdom: row.get(3)?,
            phylum: row.get(4)?,
            class: row.get(5)?,
            order: row.get(6)?,
            family: row.get(7)?,
            genus: row.get(8)?,
            species_epithet: row.get(9)?,
            common_name: row.get(10)?,
            notes: row.get(11)?,
            media_path: row.get(12)?,
            date: row.get(13)?,
            location: row.get(14)?,
        })
    }).context("Failed to fetch sighting")?;

    Ok(sighting)
}

/// Delete a sighting by ID
pub fn delete_sighting(conn: &Connection, id: i64) -> Result<usize> {
    let sql = "DELETE FROM sightings WHERE id = ?1";
    let rows_affected = conn.execute(sql, params![id])
        .context("Failed to delete sighting")?;
    Ok(rows_affected)
}

/// Get all sightings of a specific taxon
pub fn get_sightings_by_taxon_id(conn: &Connection, taxon_id: i64) -> Result<Vec<Sighting>> {
    let sql = r#"
        SELECT id, trip_id, taxon_id, kingdom, phylum, class, "order", family,
               genus, species_epithet, common_name, notes, media_path, date, location
        FROM sightings
        WHERE taxon_id = ?1
        ORDER BY date DESC, id DESC
    "#;

    let mut stmt = conn.prepare(sql)
        .context("Failed to prepare get sightings by taxon query")?;

    let rows = stmt.query_map(params![taxon_id], |row| {
        Ok(Sighting {
            id: row.get(0)?,
            trip_id: row.get(1)?,
            taxon_id: row.get(2)?,
            kingdom: row.get(3)?,
            phylum: row.get(4)?,
            class: row.get(5)?,
            order: row.get(6)?,
            family: row.get(7)?,
            genus: row.get(8)?,
            species_epithet: row.get(9)?,
            common_name: row.get(10)?,
            notes: row.get(11)?,
            media_path: row.get(12)?,
            date: row.get(13)?,
            location: row.get(14)?,
        })
    }).context("Failed to execute get sightings by taxon query")?;

    let results: Vec<Sighting> = rows.collect::<Result<Vec<_>, _>>()
        .context("Failed to parse sighting rows")?;
    Ok(results)
}

/// Get all sightings from a specific trip
pub fn get_sightings_by_trip_id(conn: &Connection, trip_id: i64) -> Result<Vec<Sighting>> {
    let sql = r#"
        SELECT id, trip_id, taxon_id, kingdom, phylum, class, "order", family,
               genus, species_epithet, common_name, notes, media_path, date, location
        FROM sightings
        WHERE trip_id = ?1
        ORDER BY id ASC
    "#;

    let mut stmt = conn.prepare(sql)
        .context("Failed to prepare get sightings by trip query")?;

    let rows = stmt.query_map(params![trip_id], |row| {
        Ok(Sighting {
            id: row.get(0)?,
            trip_id: row.get(1)?,
            taxon_id: row.get(2)?,
            kingdom: row.get(3)?,
            phylum: row.get(4)?,
            class: row.get(5)?,
            order: row.get(6)?,
            family: row.get(7)?,
            genus: row.get(8)?,
            species_epithet: row.get(9)?,
            common_name: row.get(10)?,
            notes: row.get(11)?,
            media_path: row.get(12)?,
            date: row.get(13)?,
            location: row.get(14)?,
        })
    }).context("Failed to execute get sightings by trip query")?;

    let results: Vec<Sighting> = rows.collect::<Result<Vec<_>, _>>()
        .context("Failed to parse sighting rows")?;
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::taxon::create_taxon;
    use crate::core::trip::create_trip;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "foreign_keys", "ON").unwrap();

        let schema = std::fs::read_to_string("init.sql").unwrap();
        conn.execute_batch(&schema).unwrap();

        conn
    }

    #[test]
    fn test_create_sighting_with_species_and_trip() {
        let conn = setup_test_db();

        // Create taxon and trip first
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

        let trip_id = create_trip(&conn, "Morning Walk", Some("2025-01-15"), Some("Park"), None).unwrap();

        // Create sighting
        let sighting_id = create_sighting(
            &conn,
            Some(trip_id),
            taxon_id,
            Some("Foraging on ground"),
            None,
            Some("2025-01-15"),
            Some("Near pond"),
        ).unwrap();

        assert!(sighting_id > 0);

        let sighting = get_sighting_by_id(&conn, sighting_id).unwrap();
        assert_eq!(sighting.trip_id, Some(trip_id));
        assert_eq!(sighting.taxon_id, taxon_id);
        assert_eq!(sighting.kingdom, "Animalia");
        assert_eq!(sighting.species_epithet, Some("migratorius".to_string()));
        assert_eq!(sighting.common_name, "American Robin");
    }

    #[test]
    fn test_create_sighting_with_family_level_taxon() {
        let conn = setup_test_db();

        let taxon_id = create_taxon(
            &conn,
            "family",
            "Animalia",
            Some("Chordata"),
            Some("Aves"),
            Some("Passeriformes"),
            Some("Corvidae"),
            None,
            None,
            "Crow Family",
        ).unwrap();

        let sighting_id = create_sighting(
            &conn,
            None,
            taxon_id,
            Some("Black bird, couldn't ID to species"),
            None,
            None,
            None,
        ).unwrap();

        let sighting = get_sighting_by_id(&conn, sighting_id).unwrap();
        assert_eq!(sighting.trip_id, None);
        assert_eq!(sighting.family, Some("Corvidae".to_string()));
        assert_eq!(sighting.genus, None);
        assert_eq!(sighting.species_epithet, None);
        assert_eq!(sighting.common_name, "Crow Family");
    }

    #[test]
    fn test_create_sighting_with_genus_level_taxon() {
        let conn = setup_test_db();

        let taxon_id = create_taxon(
            &conn,
            "genus",
            "Animalia",
            Some("Chordata"),
            Some("Aves"),
            Some("Accipitriformes"),
            Some("Accipitridae"),
            Some("Buteo"),
            None,
            "Buteo Hawks",
        ).unwrap();

        let sighting_id = create_sighting(
            &conn,
            None,
            taxon_id,
            Some("Large hawk overhead"),
            None,
            None,
            None,
        ).unwrap();

        let sighting = get_sighting_by_id(&conn, sighting_id).unwrap();
        assert_eq!(sighting.genus, Some("Buteo".to_string()));
        assert_eq!(sighting.species_epithet, None);
    }

    #[test]
    fn test_create_sighting_with_invalid_taxon() {
        let conn = setup_test_db();

        let result = create_sighting(
            &conn,
            None,
            99999,
            None,
            None,
            None,
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_delete_sighting() {
        let conn = setup_test_db();

        let taxon_id = create_taxon(
            &conn,
            "species",
            "Animalia",
            Some("Chordata"),
            Some("Aves"),
            Some("Passeriformes"),
            Some("Testidae"),
            Some("Test"),
            Some("temp"),
            "Test Bird",
        ).unwrap();

        let sighting_id = create_sighting(&conn, None, taxon_id, None, None, None, None).unwrap();
        let rows = delete_sighting(&conn, sighting_id).unwrap();
        assert_eq!(rows, 1);

        let result = get_sighting_by_id(&conn, sighting_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_sightings_by_taxon_id() {
        let conn = setup_test_db();

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

        // Create 3 sightings of the same taxon
        create_sighting(&conn, None, taxon_id, None, None, Some("2025-01-15"), None).unwrap();
        create_sighting(&conn, None, taxon_id, None, None, Some("2025-01-20"), None).unwrap();
        create_sighting(&conn, None, taxon_id, None, None, Some("2025-01-10"), None).unwrap();

        let results = get_sightings_by_taxon_id(&conn, taxon_id).unwrap();
        assert_eq!(results.len(), 3);

        // Should be ordered by date DESC
        assert_eq!(results[0].date, Some("2025-01-20".to_string()));
        assert_eq!(results[1].date, Some("2025-01-15".to_string()));
        assert_eq!(results[2].date, Some("2025-01-10".to_string()));

        // All should have same taxon
        for sighting in &results {
            assert_eq!(sighting.taxon_id, taxon_id);
            assert_eq!(sighting.common_name, "American Robin");
        }
    }

    #[test]
    fn test_get_sightings_by_taxon_id_empty() {
        let conn = setup_test_db();

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

        // No sightings created
        let results = get_sightings_by_taxon_id(&conn, taxon_id).unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_get_sightings_by_trip_id() {
        let conn = setup_test_db();

        let trip_id = create_trip(&conn, "Morning Walk", Some("2025-01-15"), Some("Park"), None).unwrap();

        let taxon1 = create_taxon(&conn, "species", "Animalia", Some("Chordata"), Some("Aves"), Some("Passeriformes"), Some("Turdidae"), Some("Turdus"), Some("migratorius"), "American Robin").unwrap();
        let taxon2 = create_taxon(&conn, "species", "Animalia", Some("Chordata"), Some("Aves"), Some("Accipitriformes"), Some("Accipitridae"), Some("Buteo"), Some("jamaicensis"), "Red-tailed Hawk").unwrap();

        // Create 2 sightings for the trip
        create_sighting(&conn, Some(trip_id), taxon1, None, None, None, None).unwrap();
        create_sighting(&conn, Some(trip_id), taxon2, None, None, None, None).unwrap();

        let results = get_sightings_by_trip_id(&conn, trip_id).unwrap();
        assert_eq!(results.len(), 2);

        // All should belong to same trip
        for sighting in &results {
            assert_eq!(sighting.trip_id, Some(trip_id));
        }
    }

    #[test]
    fn test_get_sightings_by_trip_id_empty() {
        let conn = setup_test_db();

        let trip_id = create_trip(&conn, "Morning Walk", None, None, None).unwrap();

        // No sightings for this trip
        let results = get_sightings_by_trip_id(&conn, trip_id).unwrap();
        assert_eq!(results.len(), 0);
    }
}
