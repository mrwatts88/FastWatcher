-- ========================
-- FastWatcher Database Init
-- ========================
PRAGMA foreign_keys = ON;

-- ---------- taxa ----------
CREATE TABLE IF NOT EXISTS taxa (
    id INTEGER PRIMARY KEY,
    rank TEXT NOT NULL CHECK(rank IN ('kingdom', 'phylum', 'class', 'order', 'family', 'genus', 'species')),
    kingdom TEXT NOT NULL,
    phylum TEXT,
    class TEXT,
    "order" TEXT,
    family TEXT,
    genus TEXT,
    species_epithet TEXT,
    common_name TEXT,
    UNIQUE(
        kingdom,
        phylum,
        class,
        "order",
        family,
        genus,
        species_epithet
    )
);

-- ---------- trips ----------
CREATE TABLE IF NOT EXISTS trips (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    date TEXT,
    location TEXT,
    notes TEXT
);

-- ---------- sightings ----------
CREATE TABLE IF NOT EXISTS sightings (
    id INTEGER PRIMARY KEY,
    trip_id INTEGER REFERENCES trips(id) ON DELETE CASCADE,
    taxon_id INTEGER NOT NULL REFERENCES taxa(id),
    kingdom TEXT,
    phylum TEXT,
    class TEXT,
    "order" TEXT,
    family TEXT,
    genus TEXT,
    species_epithet TEXT,
    common_name TEXT,
    notes TEXT,
    media_path TEXT,
    date TEXT,
    location TEXT
);

-- ---------- indexes ----------
-- taxa hierarchy
CREATE INDEX IF NOT EXISTS idx_taxa_rank ON taxa(rank);

CREATE INDEX IF NOT EXISTS idx_taxa_kingdom ON taxa(kingdom);

CREATE INDEX IF NOT EXISTS idx_taxa_phylum ON taxa(phylum);

CREATE INDEX IF NOT EXISTS idx_taxa_class ON taxa(class);

CREATE INDEX IF NOT EXISTS idx_taxa_order ON taxa("order");

CREATE INDEX IF NOT EXISTS idx_taxa_family ON taxa(family);

CREATE INDEX IF NOT EXISTS idx_taxa_genus ON taxa(genus);

CREATE INDEX IF NOT EXISTS idx_taxa_species_epithet ON taxa(species_epithet);

-- trips
CREATE INDEX IF NOT EXISTS idx_trips_date ON trips(date);

-- sightings hierarchy + relations
CREATE INDEX IF NOT EXISTS idx_sightings_trip_id ON sightings(trip_id);

CREATE INDEX IF NOT EXISTS idx_sightings_taxon_id ON sightings(taxon_id);

CREATE INDEX IF NOT EXISTS idx_sightings_kingdom ON sightings(kingdom);

CREATE INDEX IF NOT EXISTS idx_sightings_phylum ON sightings(phylum);

CREATE INDEX IF NOT EXISTS idx_sightings_class ON sightings(class);

CREATE INDEX IF NOT EXISTS idx_sightings_order ON sightings("order");

CREATE INDEX IF NOT EXISTS idx_sightings_family ON sightings(family);

CREATE INDEX IF NOT EXISTS idx_sightings_genus ON sightings(genus);

CREATE INDEX IF NOT EXISTS idx_sightings_species_epithet ON sightings(species_epithet);

CREATE INDEX IF NOT EXISTS idx_sightings_date ON sightings(date);

CREATE INDEX IF NOT EXISTS idx_sightings_location ON sightings(location);