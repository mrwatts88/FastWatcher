use std::fmt;

#[derive(Debug)]
pub struct Trip {
    pub id: i64,
    pub name: String,
    pub date: Option<String>,
    pub location: Option<String>,
    pub notes: Option<String>,
}

impl fmt::Display for Trip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.name)
    }
}
