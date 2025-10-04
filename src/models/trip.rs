#[derive(Debug)]
pub struct Trip {
    pub id: i64,
    pub name: String,
    pub date: Option<String>, // TEXT in DB → Option<String>
    pub location: Option<String>,
    pub notes: Option<String>,
}
