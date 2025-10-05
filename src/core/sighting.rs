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
