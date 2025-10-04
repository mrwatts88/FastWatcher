#[derive(Debug)]
pub struct Sighting {
    pub id: i64,
    pub trip_id: i64,
    pub taxon_id: Option<i64>,
    pub genus: String,
    pub species_epithet: String,
    pub common_name: String,
    pub notes: Option<String>,
    pub media_path: Option<String>,
    pub date: Option<String>,
    pub location: Option<String>,
}
