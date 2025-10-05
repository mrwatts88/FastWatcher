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
    subfamily: Option<&str>,
    genus: Option<&str>,
    species_epithet: Option<&str>,
    common_name: &str,
) -> Result<i64> {
    let sql = r#"
        INSERT INTO taxa (rank, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
    "#;

    conn.execute(
        sql,
        params![rank, kingdom, phylum, class, order, family, subfamily, genus, species_epithet, common_name],
    )
    .context("Failed to insert taxon")?;

    let id = conn.last_insert_rowid();
    Ok(id)
}

/// Get a taxon by ID
pub fn get_taxon_by_id(conn: &Connection, id: i64) -> Result<Taxon> {
    let sql = r#"
        SELECT id, rank, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name
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
            subfamily: row.get(7)?,
            genus: row.get(8)?,
            species_epithet: row.get(9)?,
            common_name: row.get(10)?,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "foreign_keys", "ON").unwrap();

        // Create schema
        let schema = std::fs::read_to_string("init.sql").unwrap();
        conn.execute_batch(&schema).unwrap();

        conn
    }

    #[test]
    fn test_create_species_level_taxon() {
        let conn = setup_test_db();

        let id = create_taxon(
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

        assert!(id > 0);

        let taxon = get_taxon_by_id(&conn, id).unwrap();
        assert_eq!(taxon.rank, "species");
        assert_eq!(taxon.kingdom, "Animalia");
        assert_eq!(taxon.phylum, Some("Chordata".to_string()));
        assert_eq!(taxon.species_epithet, Some("migratorius".to_string()));
        assert_eq!(taxon.common_name, "American Robin");
    }

    #[test]
    fn test_create_family_level_taxon() {
        let conn = setup_test_db();

        let id = create_taxon(
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
            "Crow Family",
        ).unwrap();

        let taxon = get_taxon_by_id(&conn, id).unwrap();
        assert_eq!(taxon.rank, "family");
        assert_eq!(taxon.family, Some("Corvidae".to_string()));
        assert_eq!(taxon.genus, None);
        assert_eq!(taxon.species_epithet, None);
    }

    #[test]
    fn test_create_genus_level_taxon() {
        let conn = setup_test_db();

        let id = create_taxon(
            &conn,
            "genus",
            "Animalia",
            Some("Chordata"),
            Some("Aves"),
            Some("Accipitriformes"),
            Some("Accipitridae"),
            None,
            Some("Buteo"),
            None,
            "Buteo Hawks",
        ).unwrap();

        let taxon = get_taxon_by_id(&conn, id).unwrap();
        assert_eq!(taxon.rank, "genus");
        assert_eq!(taxon.genus, Some("Buteo".to_string()));
        assert_eq!(taxon.species_epithet, None);
    }

    #[test]
    fn test_delete_taxon() {
        let conn = setup_test_db();

        let id = create_taxon(
            &conn,
            "species",
            "Animalia",
            Some("Chordata"),
            Some("Aves"),
            Some("Passeriformes"),
            Some("Testidae"),
            None,
            Some("Test"),
            Some("temp"),
            "Temp Bird",
        ).unwrap();

        let rows = delete_taxon(&conn, id).unwrap();
        assert_eq!(rows, 1);

        // Verify it's gone
        let result = get_taxon_by_id(&conn, id);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_rank() {
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
            None,
            "Test",
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_get_nonexistent_taxon() {
        let conn = setup_test_db();
        let result = get_taxon_by_id(&conn, 99999);
        assert!(result.is_err());
    }
}
