use crate::models::{Sighting, Taxon, Trip};
use anyhow::{Context, Result, bail};
use rusqlite::{Connection, params};

/// Performs a basic search over the sightings table.
pub fn run_search_sightings(conn: &Connection, query: &str) -> Result<Vec<Sighting>> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        bail!("empty query not allowed");
    }

    let sql = r#"
        SELECT id, trip_id, taxon_id, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name, notes, media_path, date, location
        FROM sightings
        WHERE kingdom LIKE ?1
            OR phylum LIKE ?1
            OR class LIKE ?1
            OR "order" LIKE ?1
            OR family LIKE ?1
            OR subfamily LIKE ?1
            OR genus LIKE ?1
            OR species_epithet LIKE ?1
            OR common_name LIKE ?1
            OR date LIKE ?1
            OR location LIKE ?1
        LIMIT 100
    "#;

    let pattern = format!("%{}%", trimmed);
    let mut stmt = conn.prepare(sql).context("Failed to prepare sightings search query")?;
    let rows = stmt.query_map(params![pattern], |row| {
        Ok(Sighting {
            id: row.get(0)?,
            trip_id: row.get(1)?,
            taxon_id: row.get(2)?,
            kingdom: row.get(3)?,
            phylum: row.get(4)?,
            class: row.get(5)?,
            order: row.get(6)?,
            family: row.get(7)?,
            subfamily: row.get(8)?,
            genus: row.get(9)?,
            species_epithet: row.get(10)?,
            common_name: row.get(11)?,
            notes: row.get(12)?,
            media_path: row.get(13)?,
            date: row.get(14)?,
            location: row.get(15)?,
        })
    }).context("Failed to execute sightings search")?;

    let results: Vec<Sighting> = rows.collect::<Result<Vec<_>, _>>()
        .context("Failed to parse sighting rows")?;
    Ok(results)
}

pub fn run_search_trips(conn: &Connection, query: &str) -> Result<Vec<Trip>> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        bail!("empty query not allowed");
    }

    let sql = r#"
        SELECT DISTINCT trips.id, trips.name, trips.date, trips.location, trips.notes
            FROM trips
            LEFT JOIN sightings ON sightings.trip_id = trips.id
            WHERE
            (
                trips.name LIKE ?1 OR
                trips.location LIKE ?1 OR
                sightings.kingdom LIKE ?1 OR
                sightings.phylum LIKE ?1 OR
                sightings.class LIKE ?1 OR
                sightings."order" LIKE ?1 OR
                sightings.family LIKE ?1 OR
                sightings.subfamily LIKE ?1 OR
                sightings.genus LIKE ?1 OR
                sightings.species_epithet LIKE ?1 OR
                sightings.common_name LIKE ?1
                OR trips.date LIKE ?1
            )
        LIMIT 100
"#;

    let pattern = format!("%{}%", trimmed);
    let mut stmt = conn.prepare(sql).context("Failed to prepare trips search query")?;
    let rows = stmt.query_map(params![pattern], |row| {
        Ok(Trip {
            id: row.get(0)?,
            name: row.get(1)?,
            date: row.get(2)?,
            location: row.get(3)?,
            notes: row.get(4)?,
        })
    }).context("Failed to execute trips search")?;

    let results: Vec<Trip> = rows.collect::<Result<Vec<_>, _>>()
        .context("Failed to parse trip rows")?;
    Ok(results)
}

pub fn run_search_taxa(conn: &Connection, query: &str) -> Result<Vec<Taxon>> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        bail!("empty query not allowed");
    }

    let sql = r#"
        SELECT id, rank, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name
        FROM taxa
        WHERE kingdom LIKE ?1
           OR phylum LIKE ?1
           OR class LIKE ?1
           OR "order" LIKE ?1
           OR family LIKE ?1
           OR subfamily LIKE ?1
           OR genus LIKE ?1
           OR species_epithet LIKE ?1
           OR common_name LIKE ?1
        LIMIT 100
    "#;

    let pattern = format!("%{}%", trimmed);
    let mut stmt = conn.prepare(sql).context("Failed to prepare taxa search query")?;
    let rows = stmt.query_map(params![pattern], |row| {
        Ok(Taxon {
            id: row.get(0)?,
            rank: row.get(1)?,
            kingdom: row.get(2)?,
            phylum: row.get(3)?,
            class: row.get(4)?,
            order: row.get(5)?,
            family: row.get(6)?,
            subfamily: row.get(7)?,
            genus: row.get(8)?,
            species_epithet: row.get(9)?,
            common_name: row.get(10)?,
        })
    }).context("Failed to execute taxa search")?;

    let results: Vec<Taxon> = rows.collect::<Result<Vec<_>, _>>()
        .context("Failed to parse taxon rows")?;
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::taxon::create_taxon;
    use crate::core::trip::create_trip;
    use crate::core::sighting::create_sighting;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "foreign_keys", "ON").unwrap();

        let schema = std::fs::read_to_string("init.sql").unwrap();
        conn.execute_batch(&schema).unwrap();

        conn
    }

    #[test]
    fn test_search_taxa_by_common_name() {
        let conn = setup_test_db();

        create_taxon(&conn, "species", "Animalia", Some("Chordata"), Some("Aves"), Some("Passeriformes"), Some("Turdidae"), None, Some("Turdus"), Some("migratorius"), "American Robin").unwrap();

        let results = run_search_taxa(&conn, "Robin").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].common_name, "American Robin");
    }

    #[test]
    fn test_search_taxa_by_family() {
        let conn = setup_test_db();

        create_taxon(&conn, "family", "Animalia", Some("Chordata"), Some("Aves"), Some("Passeriformes"), Some("Corvidae"), None, None, None, "Crow Family").unwrap();

        let results = run_search_taxa(&conn, "Corvidae").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].family, Some("Corvidae".to_string()));
        assert_eq!(results[0].rank, "family");
    }

    #[test]
    fn test_search_sightings_by_species() {
        let conn = setup_test_db();

        let taxon_id = create_taxon(&conn, "species", "Animalia", Some("Chordata"), Some("Aves"), Some("Passeriformes"), Some("Turdidae"), None, Some("Turdus"), Some("migratorius"), "American Robin").unwrap();
        create_sighting(&conn, None, taxon_id, Some("Test note"), None, None, None).unwrap();

        let results = run_search_sightings(&conn, "Robin").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].common_name, "American Robin");
    }

    #[test]
    fn test_search_sightings_by_family() {
        let conn = setup_test_db();

        let taxon_id = create_taxon(&conn, "family", "Animalia", Some("Chordata"), Some("Aves"), Some("Passeriformes"), Some("Corvidae"), None, None, None, "Crow Family").unwrap();
        create_sighting(&conn, None, taxon_id, None, None, None, None).unwrap();

        let results = run_search_sightings(&conn, "Corvidae").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].family, Some("Corvidae".to_string()));
    }

    #[test]
    fn test_search_sightings_by_location() {
        let conn = setup_test_db();

        let taxon_id = create_taxon(&conn, "species", "Animalia", Some("Chordata"), Some("Aves"), Some("Passeriformes"), Some("Turdidae"), None, Some("Turdus"), Some("migratorius"), "American Robin").unwrap();
        create_sighting(&conn, None, taxon_id, None, None, None, Some("Near the pond")).unwrap();

        let results = run_search_sightings(&conn, "pond").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].location, Some("Near the pond".to_string()));
    }

    #[test]
    fn test_search_trips_by_name() {
        let conn = setup_test_db();

        create_trip(&conn, "Morning Birding", Some("2025-01-15"), Some("Central Park"), None).unwrap();

        let results = run_search_trips(&conn, "Birding").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Morning Birding");
    }

    #[test]
    fn test_search_trips_by_location() {
        let conn = setup_test_db();

        create_trip(&conn, "Morning Walk", None, Some("Central Park"), None).unwrap();

        let results = run_search_trips(&conn, "Central Park").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].location, Some("Central Park".to_string()));
    }

    #[test]
    fn test_search_trips_by_sighting_taxonomy() {
        let conn = setup_test_db();

        let trip_id = create_trip(&conn, "Birdwatching", None, None, None).unwrap();
        let taxon_id = create_taxon(&conn, "family", "Animalia", Some("Chordata"), Some("Aves"), Some("Passeriformes"), Some("Corvidae"), None, None, None, "Crow Family").unwrap();
        create_sighting(&conn, Some(trip_id), taxon_id, None, None, None, None).unwrap();

        let results = run_search_trips(&conn, "Corvidae").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Birdwatching");
    }

    #[test]
    fn test_empty_query() {
        let conn = setup_test_db();

        let result = run_search_taxa(&conn, "");
        assert!(result.is_err());

        let result = run_search_sightings(&conn, "   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_search_taxa_by_subfamily() {
        let conn = setup_test_db();

        // Create subfamily-level taxon
        create_taxon(
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
            "Corvinae Subfamily",
        ).unwrap();

        let results = run_search_taxa(&conn, "Corvinae").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].rank, "subfamily");
        assert_eq!(results[0].subfamily, Some("Corvinae".to_string()));
    }

    #[test]
    fn test_search_sightings_by_subfamily() {
        let conn = setup_test_db();

        // Create subfamily-level taxon
        let taxon_id = create_taxon(
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
            "Corvinae Subfamily",
        ).unwrap();

        create_sighting(&conn, None, taxon_id, None, None, None, None).unwrap();

        let results = run_search_sightings(&conn, "Corvinae").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].subfamily, Some("Corvinae".to_string()));
    }

    #[test]
    fn test_search_trips_by_subfamily() {
        let conn = setup_test_db();

        let trip_id = create_trip(&conn, "Corvid Watch", None, None, None).unwrap();

        let taxon_id = create_taxon(
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

        create_sighting(&conn, Some(trip_id), taxon_id, None, None, None, None).unwrap();

        let results = run_search_trips(&conn, "Corvinae").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Corvid Watch");
    }
}
