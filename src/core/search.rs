use crate::models::Sighting;
use anyhow::{Result, bail};
use rusqlite::{Connection, params}; // assuming you have a shared model

/// Performs a basic search over the sightings table.
pub fn run_search(conn: &Connection, query: &str) -> Result<Vec<Sighting>> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        bail!("empty query not allowed");
    }

    let sql = r#"
        SELECT id, trip_id, taxon_id, genus, species_epithet, common_name,
               notes, media_path, date, location
        FROM sightings
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
    let mut stmt = conn.prepare(sql)?;
    let rows = stmt.query_map(params![pattern], |row| {
        Ok(Sighting {
            id: row.get(0)?,
            trip_id: row.get(1)?,
            taxon_id: row.get(2)?,
            genus: row.get(3)?,
            species_epithet: row.get(4)?,
            common_name: row.get(5)?,
            notes: row.get(6)?,
            media_path: row.get(7)?,
            date: row.get(8)?,
            location: row.get(9)?,
        })
    })?;

    let results = rows.filter_map(Result::ok).collect();
    Ok(results)
}
