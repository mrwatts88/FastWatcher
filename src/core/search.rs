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
        SELECT id, trip_id, taxon_id, kingdom, phylum, class, "order", family, genus, species_epithet, common_name, notes, media_path, date, location
        FROM sightings
        WHERE kingdom LIKE ?1
            OR phylum LIKE ?1
            OR class LIKE ?1
            OR "order" LIKE ?1
            OR family LIKE ?1
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
            genus: row.get(8)?,
            species_epithet: row.get(9)?,
            common_name: row.get(10)?,
            notes: row.get(11)?,
            media_path: row.get(12)?,
            date: row.get(13)?,
            location: row.get(14)?,
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
        SELECT id, rank, kingdom, phylum, class, "order", family, genus, species_epithet, common_name
        FROM taxa
        WHERE kingdom LIKE ?1
           OR phylum LIKE ?1
           OR class LIKE ?1
           OR "order" LIKE ?1
           OR family LIKE ?1
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
            genus: row.get(7)?,
            species_epithet: row.get(8)?,
            common_name: row.get(9)?,
        })
    }).context("Failed to execute taxa search")?;

    let results: Vec<Taxon> = rows.collect::<Result<Vec<_>, _>>()
        .context("Failed to parse taxon rows")?;
    Ok(results)
}
