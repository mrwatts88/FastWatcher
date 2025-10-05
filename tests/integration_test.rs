use rusqlite::Connection;
use fast_watcher::core::taxon::{create_taxon, get_taxon_by_id, delete_taxon};
use fast_watcher::core::trip::{create_trip, get_trip_by_id, delete_trip};
use fast_watcher::core::sighting::{create_sighting, get_sighting_by_id, delete_sighting};
use fast_watcher::core::search::{run_search_taxa, run_search_sightings, run_search_trips};

/// Helper function to set up a test database with schema
fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.pragma_update(None, "foreign_keys", "ON").unwrap();

    let schema = std::fs::read_to_string("init.sql").unwrap();
    conn.execute_batch(&schema).unwrap();

    conn
}

// ==========================================
// TAXON INTEGRATION TESTS
// ==========================================

#[test]
fn test_taxon_workflow() {
    let conn = setup_test_db();

    // Create species-level taxon
    let robin_id = create_taxon(
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
    assert!(robin_id > 0);

    // Create family-level taxon
    let warbler_fam_id = create_taxon(
        &conn,
        "family",
        "Animalia",
        Some("Chordata"),
        Some("Aves"),
        Some("Passeriformes"),
        Some("Parulidae"),
        None,
        None,
        "Warbler Family",
    ).unwrap();
    assert!(warbler_fam_id > 0);

    // Create genus-level taxon
    let buteo_id = create_taxon(
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
    assert!(buteo_id > 0);

    // Retrieve and verify species-level taxon
    let robin = get_taxon_by_id(&conn, robin_id).unwrap();
    assert_eq!(robin.rank, "species");
    assert_eq!(robin.kingdom, "Animalia");
    assert_eq!(robin.species_epithet, Some("migratorius".to_string()));
    assert_eq!(robin.common_name, "American Robin");

    // Retrieve and verify family-level taxon
    let warbler_fam = get_taxon_by_id(&conn, warbler_fam_id).unwrap();
    assert_eq!(warbler_fam.rank, "family");
    assert_eq!(warbler_fam.family, Some("Parulidae".to_string()));
    assert_eq!(warbler_fam.genus, None);
    assert_eq!(warbler_fam.species_epithet, None);

    // Search taxa by common name
    let results = run_search_taxa(&conn, "Robin").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].common_name, "American Robin");

    // Search taxa by family
    let results = run_search_taxa(&conn, "Parulidae").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].family, Some("Parulidae".to_string()));

    // Delete taxon and verify
    let temp_id = create_taxon(
        &conn,
        "species",
        "Animalia",
        Some("Chordata"),
        Some("Aves"),
        Some("Passeriformes"),
        Some("Testidae"),
        Some("Test"),
        Some("temp"),
        "Temp Bird",
    ).unwrap();
    let rows = delete_taxon(&conn, temp_id).unwrap();
    assert_eq!(rows, 1);
    assert!(get_taxon_by_id(&conn, temp_id).is_err());
}

#[test]
fn test_invalid_taxon_rank() {
    let conn = setup_test_db();

    let result = create_taxon(
        &conn,
        "invalid_rank",
        "Animalia",
        None,
        None,
        None,
        None,
        None,
        None,
        "Test",
    );

    assert!(result.is_err());
}

// ==========================================
// TRIP INTEGRATION TESTS
// ==========================================

#[test]
fn test_trip_workflow() {
    let conn = setup_test_db();

    // Create trip with all fields
    let trip1_id = create_trip(
        &conn,
        "Morning Birding",
        Some("2025-01-15"),
        Some("Central Park"),
        Some("Cold morning, lots of activity"),
    ).unwrap();
    assert!(trip1_id > 0);

    // Create trip with minimal fields
    let trip2_id = create_trip(&conn, "Quick Walk", None, None, None).unwrap();
    assert!(trip2_id > 0);

    // Retrieve and verify trip with all fields
    let trip1 = get_trip_by_id(&conn, trip1_id).unwrap();
    assert_eq!(trip1.name, "Morning Birding");
    assert_eq!(trip1.date, Some("2025-01-15".to_string()));
    assert_eq!(trip1.location, Some("Central Park".to_string()));
    assert_eq!(trip1.notes, Some("Cold morning, lots of activity".to_string()));

    // Retrieve and verify trip with minimal fields
    let trip2 = get_trip_by_id(&conn, trip2_id).unwrap();
    assert_eq!(trip2.name, "Quick Walk");
    assert_eq!(trip2.date, None);

    // Search trips by location
    let results = run_search_trips(&conn, "Central Park").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Morning Birding");

    // Delete trip and verify
    let rows = delete_trip(&conn, trip2_id).unwrap();
    assert_eq!(rows, 1);
    assert!(get_trip_by_id(&conn, trip2_id).is_err());
}

// ==========================================
// SIGHTING INTEGRATION TESTS
// ==========================================

#[test]
fn test_sighting_workflow() {
    let conn = setup_test_db();

    // Set up prerequisites
    let robin_id = create_taxon(
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

    let warbler_fam_id = create_taxon(
        &conn,
        "family",
        "Animalia",
        Some("Chordata"),
        Some("Aves"),
        Some("Passeriformes"),
        Some("Parulidae"),
        None,
        None,
        "Warbler Family",
    ).unwrap();

    let buteo_id = create_taxon(
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

    let trip_id = create_trip(
        &conn,
        "Morning Birding",
        Some("2025-01-15"),
        Some("Central Park"),
        None,
    ).unwrap();

    // Create species-level sighting with trip
    let sighting1_id = create_sighting(
        &conn,
        Some(trip_id),
        robin_id,
        Some("Foraging on the ground"),
        None,
        Some("2025-01-15"),
        Some("Near the pond"),
    ).unwrap();
    assert!(sighting1_id > 0);

    // Create family-level sighting without trip
    let sighting2_id = create_sighting(
        &conn,
        None,
        warbler_fam_id,
        Some("Small yellow bird, couldn't ID to species"),
        None,
        Some("2025-01-15"),
        None,
    ).unwrap();
    assert!(sighting2_id > 0);

    // Create genus-level sighting with trip
    let sighting3_id = create_sighting(
        &conn,
        Some(trip_id),
        buteo_id,
        Some("Large hawk circling overhead, Buteo sp."),
        None,
        None,
        None,
    ).unwrap();
    assert!(sighting3_id > 0);

    // Retrieve and verify species-level sighting
    let sighting1 = get_sighting_by_id(&conn, sighting1_id).unwrap();
    assert_eq!(sighting1.trip_id, Some(trip_id));
    assert_eq!(sighting1.taxon_id, robin_id);
    assert_eq!(sighting1.kingdom, "Animalia");
    assert_eq!(sighting1.species_epithet, Some("migratorius".to_string()));
    assert_eq!(sighting1.common_name, "American Robin");
    assert_eq!(sighting1.location, Some("Near the pond".to_string()));

    // Retrieve and verify family-level sighting
    let sighting2 = get_sighting_by_id(&conn, sighting2_id).unwrap();
    assert_eq!(sighting2.trip_id, None);
    assert_eq!(sighting2.family, Some("Parulidae".to_string()));
    assert_eq!(sighting2.genus, None);
    assert_eq!(sighting2.species_epithet, None);
    assert_eq!(sighting2.common_name, "Warbler Family");

    // Retrieve and verify genus-level sighting
    let sighting3 = get_sighting_by_id(&conn, sighting3_id).unwrap();
    assert_eq!(sighting3.genus, Some("Buteo".to_string()));
    assert_eq!(sighting3.species_epithet, None);

    // Search sightings by common name
    let results = run_search_sightings(&conn, "Robin").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].common_name, "American Robin");

    // Search sightings by family
    let results = run_search_sightings(&conn, "Parulidae").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].common_name, "Warbler Family");

    // Search sightings by genus
    let results = run_search_sightings(&conn, "Buteo").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].common_name, "Buteo Hawks");

    // Search sightings by location
    let results = run_search_sightings(&conn, "pond").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].common_name, "American Robin");

    // Delete sighting and verify
    let rows = delete_sighting(&conn, sighting3_id).unwrap();
    assert_eq!(rows, 1);
    assert!(get_sighting_by_id(&conn, sighting3_id).is_err());
}

#[test]
fn test_sighting_with_invalid_taxon() {
    let conn = setup_test_db();

    let result = create_sighting(
        &conn,
        None,
        99999,
        Some("Test"),
        None,
        None,
        None,
    );

    assert!(result.is_err());
}

// ==========================================
// CROSS-ENTITY INTEGRATION TESTS
// ==========================================

#[test]
fn test_search_trips_by_sighting_taxonomy() {
    let conn = setup_test_db();

    // Create a trip
    let trip_id = create_trip(&conn, "Birdwatching", None, None, None).unwrap();

    // Create a taxon
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

    // Create a sighting linking them
    create_sighting(&conn, Some(trip_id), taxon_id, None, None, None, None).unwrap();

    // Search trips by sighting's taxonomic field
    let results = run_search_trips(&conn, "Corvidae").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Birdwatching");
}

#[test]
fn test_search_across_multiple_ranks() {
    let conn = setup_test_db();

    // Create taxa at different ranks
    let robin_id = create_taxon(
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

    let warbler_fam_id = create_taxon(
        &conn,
        "family",
        "Animalia",
        Some("Chordata"),
        Some("Aves"),
        Some("Passeriformes"),
        Some("Parulidae"),
        None,
        None,
        "Warbler Family",
    ).unwrap();

    // Create sightings for both
    create_sighting(&conn, None, robin_id, None, None, None, None).unwrap();
    create_sighting(&conn, None, warbler_fam_id, None, None, None, None).unwrap();

    // Search by class - should find both
    let results = run_search_sightings(&conn, "Aves").unwrap();
    assert!(results.len() >= 2);

    let common_names: Vec<String> = results.iter()
        .map(|s| s.common_name.clone())
        .collect();
    assert!(common_names.contains(&"American Robin".to_string()));
    assert!(common_names.contains(&"Warbler Family".to_string()));
}

#[test]
fn test_empty_search_queries() {
    let conn = setup_test_db();

    // Empty query should fail for all search types
    assert!(run_search_taxa(&conn, "").is_err());
    assert!(run_search_sightings(&conn, "   ").is_err());
    assert!(run_search_trips(&conn, "").is_err());
}

#[test]
fn test_nonexistent_entity_retrieval() {
    let conn = setup_test_db();

    // All get_by_id functions should fail for non-existent IDs
    assert!(get_taxon_by_id(&conn, 99999).is_err());
    assert!(get_trip_by_id(&conn, 99999).is_err());
    assert!(get_sighting_by_id(&conn, 99999).is_err());
}

// ==========================================
// SEEDED DATA INTEGRATION TEST
// ==========================================

#[test]
fn test_seeded_data() {
    let conn = setup_test_db();

    // Load seed data
    let seed_sql = std::fs::read_to_string("seed_taxa.sql").unwrap();
    conn.execute_batch(&seed_sql).unwrap();

    // Search for seeded Blue Jay taxon
    let results = run_search_taxa(&conn, "Blue Jay").unwrap();
    assert!(results.len() > 0);
    assert!(results.iter().any(|t| t.common_name == "Blue Jay"));

    // Search for seeded Crow Family
    let results = run_search_taxa(&conn, "Corvidae").unwrap();
    assert!(results.len() > 0);
    assert!(results.iter().any(|t| t.family == Some("Corvidae".to_string())));

    // Verify both species and family level taxa are seeded
    let blue_jay = results.iter().find(|t| t.common_name == "Blue Jay");
    assert!(blue_jay.is_some());
    assert_eq!(blue_jay.unwrap().rank, "species");

    let crow_family = results.iter().find(|t| t.common_name == "Crow Family");
    assert!(crow_family.is_some());
    assert_eq!(crow_family.unwrap().rank, "family");
}
