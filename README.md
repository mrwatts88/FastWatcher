# **Fast Watcher**

### 🦜️ Personal Field Archive for Trips, Sightings, and Species

**Fast Watcher** is an **offline-first, local archive app** for nature enthusiasts, field biologists, and hobbyist observers — think _bird-watching_, but for **any** organism.

Data is stored in a **blazing-fast SQLite database** with denormalized fields optimized for instant search. Taxonomic data is duplicated into sightings to eliminate expensive JOINs during search operations.

The philosophy: **your data lives locally, searches are instant.**

---

## 🧯 Project Overview

| Entity       | Description                                                                  |
| ------------ | ---------------------------------------------------------------------------- |
| **Trip**     | A single outing (date, location, notes). Optional for sightings.             |
| **Sighting** | A single observation of a taxon, optionally linked to a trip (notes, media). |
| **Taxon**    | Canonical taxonomy record at any rank (kingdom → species) with common name.  |

Relationship:

```
Trip (0..1) ───< Sighting >─── (1) Taxon
```

**Note:** Sightings denormalize taxonomic fields from Taxa for blazing-fast search without JOINs.

**Partial Taxonomy Support:** Taxa can be identified at any rank (e.g., family-level for "Corvidae" when species is unknown). All taxonomic fields except kingdom are optional.

---

## 🧹 Current Architecture

```
fast_watcher/
├── Cargo.toml
├── init.sql             # Database schema
├── seed_*.sql           # Seed data files
├── src/
│   ├── lib.rs           # Library interface for tests
│   ├── main.rs          # CLI entrypoint
│   ├── cli/             # Argument parsing & command routing
│   │   └── mod.rs
│   ├── models/          # Data models
│   │   ├── mod.rs
│   │   ├── sighting.rs
│   │   ├── taxon.rs
│   │   └── trip.rs
│   └── core/            # Core logic (with unit tests)
│       ├── mod.rs
│       ├── db.rs        # Database connection & utilities
│       ├── search.rs    # Search functions
│       ├── sighting.rs  # Sighting CRUD operations
│       ├── taxon.rs     # Taxon CRUD operations
│       └── trip.rs      # Trip CRUD operations
└── tests/
    └── integration_test.rs  # Full workflow integration tests
```

### 🮀 Tech Stack

- **Language:** Rust (edition 2021)
- **CLI Framework:** [clap](https://docs.rs/clap/latest/clap/) (`derive` API)
- **Error Handling:** [anyhow](https://docs.rs/anyhow) with `.context()` for detailed error messages
- **Database:** [rusqlite](https://docs.rs/rusqlite) (SQLite with WAL mode + foreign keys)
- **Search:** LIKE queries (FTS5 planned for future)
- **UI (future):** [Slint](https://slint.dev/) for a native desktop layer

---

## ⚙️ CLI Commands

### Database Management

```bash
fast-watcher init-db              # Initialize database and seed with sample data
fast-watcher drop-db              # Drop all tables (use with caution!)
```

### Search Commands

```bash
fast-watcher search-sightings <query>   # Search for sightings
fast-watcher search-trips <query>       # Search for trips
fast-watcher search-taxa <query>        # Search for taxa
```

### Trip Commands

```bash
fast-watcher add-trip <name> [OPTIONS]
  -d, --date <DATE>           Optional date
  -l, --location <LOCATION>   Optional location
  -n, --notes <NOTES>         Optional notes

fast-watcher show-trip <id>    # Show trip details
fast-watcher delete-trip <id>  # Delete a trip
```

### Taxon Commands

```bash
fast-watcher add-taxon <rank> <kingdom> <common_name> [OPTIONS]
  --phylum <PHYLUM>                   Optional phylum
  --class <CLASS>                     Optional class
  --order <ORDER>                     Optional order
  --family <FAMILY>                   Optional family
  --genus <GENUS>                     Optional genus
  --species-epithet <SPECIES_EPITHET> Optional species epithet

fast-watcher show-taxon <id>    # Show taxon details
fast-watcher delete-taxon <id>  # Delete a taxon
```

### Sighting Commands

```bash
fast-watcher add-sighting <taxon_id> [OPTIONS]
  -t, --trip-id <TRIP_ID>      Optional trip ID
  -n, --notes <NOTES>          Optional notes
  -m, --media-path <PATH>      Optional media path
  -d, --date <DATE>            Optional date
  -l, --location <LOCATION>    Optional location

fast-watcher show-sighting <id>    # Show sighting details
fast-watcher delete-sighting <id>  # Delete a sighting
```

### Examples

```bash
# Initialize database
$ fast-watcher init-db

# Search for blue jays
$ fast-watcher search-sightings "blue jay"
1: Animalia/Chordata/Aves/Passeriformes/Corvidae/Cyanocitta/cristata/Blue Jay

# Add a new trip
$ fast-watcher add-trip "Morning walk" -d "2025-01-15" -l "Central Park"
Trip created with ID: 4

# Add a species-level taxon (full taxonomy)
$ fast-watcher add-taxon species Animalia "Blue Jay" \
  --phylum Chordata \
  --class Aves \
  --order Passeriformes \
  --family Corvidae \
  --genus Cyanocitta \
  --species-epithet cristata
Taxon created with ID: 10

# Add a family-level taxon (partial taxonomy - when species is unknown)
$ fast-watcher add-taxon family Animalia "Crow Family" \
  --phylum Chordata \
  --class Aves \
  --order Passeriformes \
  --family Corvidae
Taxon created with ID: 11

# Add a sighting linked to a trip
$ fast-watcher add-sighting 10 --trip-id 4 --notes "Spotted near the pond"
Sighting created with ID: 15

# Add a sighting without a trip
$ fast-watcher add-sighting 10 --notes "Backyard sighting"
Sighting created with ID: 16

# View sighting details
$ fast-watcher show-sighting 15
```

---

## 🧱 Development Status

### ✅ Phase 1 — CLI Engine (Current)

- [x] Modular project structure (`core`, `cli`, `models`, `main`)
- [x] `clap`-based argument parsing with subcommands
- [x] `anyhow` error handling with `bail!`, `?`, and `.context()`
- [x] SQLite database with WAL mode and foreign keys
- [x] CRUD operations for Trips, Taxa, and Sightings
- [x] Search functionality across all entities (LIKE queries)
- [x] Denormalized taxonomic data in sightings for fast search
- [x] Partial taxonomic identification (family/genus/species ranks)
- [x] Database initialization and seeding
- [x] Comprehensive test suite (58 tests: unit + integration)
- [x] `cargo install --path .` for system-wide binary

### 🚧 Phase 2 — Slint UI

Build a native desktop UI with [Slint](https://slint.dev/) while keeping all core logic in Rust.

#### Core Functions to Add

**Search:**

- UI calls existing search functions separately for sectioned display
- `run_search_sightings(query)` → Sightings section
- `run_search_taxa(query)` → Taxa section
- `run_search_trips(query)` → Trips section

**Related Entity Queries (3 functions, 6 relationships):**

- [x] `get_sightings_by_taxon_id(taxon_id)` → `Vec<Sighting>` - For taxon detail page → sightings list
- [x] `get_sightings_by_trip_id(trip_id)` → `Vec<Sighting>` - For trip detail page → sightings list
- [x] `get_trips_by_taxon_id(taxon_id)` → `Vec<Trip>` - For taxon detail page → trips list

**Covered by existing functions + sighting denormalization:**

- Sighting → Taxon: Use `get_taxon_by_id(sighting.taxon_id)` - For sighting detail page → taxon link
- Sighting → Trip: Use `get_trip_by_id(sighting.trip_id)` if Some - For sighting detail page → trip link
- Trip → Taxa: Use `get_sightings_by_trip_id()` (sightings contain denormalized taxonomy) - For trip detail page → taxa list

#### UI Architecture

**Search View:**

- Single search box with unified results
- Results list with type indicators (🐦 Sighting / 📋 Taxon / 🎒 Trip)
- Click → route to appropriate detail view

**Three Detail Views:**

1. **Sighting Detail** (Primary)

   - Full observation record (date, location, notes, media)
   - Embedded taxonomy display (denormalized data)
   - Optional links: "Part of: [Trip]" and "See all: [Taxon]"

2. **Taxon Detail**

   - Canonical taxonomy at specified rank
   - List of all sightings of this taxon
   - List of all trips where this taxon was seen
   - Stats: total sightings, first/last seen, trip count

3. **Trip Detail**
   - Trip metadata (name, date, location, notes)
   - List of all sightings from this trip
   - Summary stats

**Implementation Flow:**

```
User types "blue" → Unified search → Results:
  🐦 Sighting #15 - Blue Jay (2025-01-15)
  📋 Taxon [species] - Blue Jay
  🎒 Trip - Bluebird Trail Hike

Click Sighting → Sighting Detail → Links to taxon/trip
Click Taxon → Taxon Detail → Lists all sightings
Click Trip → Trip Detail → Lists all sightings
```

### Phase 3 - Content Search

- [ ] SQLite FTS5 full-text search (future optimization)

---

## 🧠 Rust Concepts We Covered

| Concept                     | Summary                                                                               | Example                                                      |
| --------------------------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| **Modules (`mod` / `use`)** | `mod` declares a module; `use` brings items into scope.                               | `mod core; use core::search::run_search_sightings;`          |
| **Results & `?` operator**  | `Result<T, E>` encodes success/failure; `?` bubbles up errors.                        | `let contents = fs::read_to_string(p)?;`                     |
| **`anyhow`**                | Universal error wrapper; `bail!()` for quick returns; `.context()` adds cause chains. | `bail!("empty query");` / `.context("Failed to insert")?`    |
| **Traits / Impl blocks**    | Behavior defined separately from data.                                                | `impl Dog { fn bark(&self){} }` / `impl Speak for Dog { … }` |
| **`into()` / `from()`**     | Generic, type-inferred conversions.                                                   | `"hawk".into()` → `String` or `Vec<u8>`                      |
| **Implicit returns**        | Last line without `;` is returned automatically.                                      | `Ok(format!("Results for {query}"))`                         |
| **CLI via Clap**            | Derive `Parser` + `Subcommand` → instant help, validation, routing.                   | `#[derive(Parser)] struct Cli { ... }`                       |
| **`cargo install`**         | Builds & installs binary to `~/.cargo/bin`.                                           | `cargo install --path .`                                     |

---

## 🚀 Quick Start

### Initialize database

```bash
cargo run -- init-db
```

### Run locally

```bash
cargo run -- search-sightings "hawk"
cargo run -- add-trip "Morning hike" -l "Yosemite"
```

### Build and install system-wide

```bash
cargo install --path .
fast-watcher search-sightings "hawk"
```

### Uninstall

```bash
cargo uninstall fast-watcher
```

---

## 🧪 Testing

### Run all tests

```bash
cargo test
```

**Test coverage:**

- **24 unit tests** in `src/core/` modules (taxon, trip, sighting, search)
- **10 integration tests** in `tests/integration_test.rs`
- All tests use in-memory SQLite databases (won't affect `fast_watcher.db`)

### Test organization

- **Unit tests** (`#[cfg(test)]` modules): Test individual CRUD and search functions
- **Integration tests** (`tests/` directory): Test complete workflows and cross-entity operations

---

## 🟞️ Roadmap

- **Phase 1** → core engine + SQLite search
- **Phase 2** → rich indexing
- **Phase 3** → native UI with Slint
- **Phase 4** → portable export/import (Markdown + media)
