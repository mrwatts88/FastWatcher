use std::fmt;

#[derive(Debug)]
pub struct Taxon {
    pub id: i64,
    pub rank: String,
    pub kingdom: String,
    pub phylum: Option<String>,
    pub class: Option<String>,
    pub order: Option<String>,
    pub family: Option<String>,
    pub subfamily: Option<String>,
    pub genus: Option<String>,
    pub species_epithet: Option<String>,
    pub common_name: String,
}

impl fmt::Display for Taxon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = vec![self.kingdom.clone()];

        if let Some(ref p) = self.phylum { parts.push(p.clone()); }
        if let Some(ref c) = self.class { parts.push(c.clone()); }
        if let Some(ref o) = self.order { parts.push(o.clone()); }
        if let Some(ref fam) = self.family { parts.push(fam.clone()); }
        if let Some(ref sf) = self.subfamily { parts.push(sf.clone()); }
        if let Some(ref g) = self.genus { parts.push(g.clone()); }
        if let Some(ref s) = self.species_epithet { parts.push(s.clone()); }

        write!(
            f,
            "{} [{}]: {} ({})",
            self.id,
            self.rank,
            parts.join("/"),
            self.common_name
        )
    }
}
