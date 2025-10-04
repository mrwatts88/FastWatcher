#[derive(Debug)]
pub struct Taxon {
    pub id: i64,
    pub kingdom: Option<String>,
    pub phylum: Option<String>,
    pub class: Option<String>,
    pub order: Option<String>,
    pub family: Option<String>,
    pub genus: Option<String>,
    pub species_epithet: Option<String>,
    pub common_name: Option<String>,
}
