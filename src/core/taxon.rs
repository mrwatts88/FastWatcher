use crate::models::Taxon;
use anyhow::{Context, Result};
use rusqlite::{Connection, params};

/// Create a new taxon
pub fn create_taxon(
    conn: &Connection,
    rank: &str,
    kingdom: &str,
    phylum: Option<&str>,
    class: Option<&str>,
    order: Option<&str>,
    family: Option<&str>,
    genus: Option<&str>,
    species_epithet: Option<&str>,
    common_name: &str,
) -> Result<i64> {
    let sql = r#"
        INSERT INTO taxa (rank, kingdom, phylum, class, "order", family, genus, species_epithet, common_name)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
    "#;

    conn.execute(
        sql,
        params![rank, kingdom, phylum, class, order, family, genus, species_epithet, common_name],
    )
    .context("Failed to insert taxon")?;

    let id = conn.last_insert_rowid();
    Ok(id)
}

/// Get a taxon by ID
pub fn get_taxon_by_id(conn: &Connection, id: i64) -> Result<Taxon> {
    let sql = r#"
        SELECT id, rank, kingdom, phylum, class, "order", family, genus, species_epithet, common_name
        FROM taxa
        WHERE id = ?1
    "#;

    let taxon = conn.query_row(sql, params![id], |row| {
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
    }).context("Failed to fetch taxon")?;

    Ok(taxon)
}

/// Delete a taxon by ID
pub fn delete_taxon(conn: &Connection, id: i64) -> Result<usize> {
    let sql = "DELETE FROM taxa WHERE id = ?1";
    let rows_affected = conn.execute(sql, params![id])
        .context("Failed to delete taxon")?;
    Ok(rows_affected)
}
