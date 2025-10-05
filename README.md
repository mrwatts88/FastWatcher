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
├── build.rs             # Slint build script
├── init.sql             # Database schema
├── seed_*.sql           # Seed data files
├── src/
│   ├── lib.rs           # Library interface for tests
│   ├── main.rs          # Entry point (GUI/CLI mode switcher)
│   ├── cli/             # Argument parsing & command routing
│   │   └── mod.rs
│   ├── models/          # Data models
│   │   ├── mod.rs
│   │   ├── sighting.rs
│   │   ├── taxon.rs
│   │   └── trip.rs
│   ├── core/            # Core logic (with unit tests)
│   │   ├── mod.rs
│   │   ├── db.rs        # Database connection & utilities
│   │   ├── search.rs    # Search functions
│   │   ├── sighting.rs  # Sighting CRUD operations
│   │   ├── taxon.rs     # Taxon CRUD operations
│   │   └── trip.rs      # Trip CRUD operations
│   └── ui/              # Slint GUI
│       ├── mod.rs       # UI bridge (Rust ↔ Slint)
│       └── app.slint    # UI markup & styling
└── tests/
    └── integration_test.rs  # Full workflow integration tests
```

### 🮀 Tech Stack

- **Language:** Rust (edition 2021)
- **CLI Framework:** [clap](https://docs.rs/clap/latest/clap/) (`derive` API)
- **Error Handling:** [anyhow](https://docs.rs/anyhow) with `.context()` for detailed error messages
- **Database:** [rusqlite](https://docs.rs/rusqlite) (SQLite with WAL mode + foreign keys)
- **Search:** LIKE queries (FTS5 planned for future)
- **UI:** [Slint](https://slint.dev/) for native desktop interface

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
- [x] Comprehensive test suite (42 tests: 32 unit + 10 integration)
- [x] `cargo install --path .` for system-wide binary

### ✅ Phase 2 — Slint UI (Completed)

Built a native desktop UI with [Slint](https://slint.dev/) featuring instant search and hierarchical navigation.

#### Features Implemented

**Search Interface:**
- Real-time search (no debounce) with 3-character minimum
- Sectioned results display (Sightings, Taxa, Trips)
- Click any result to navigate to detail page
- Custom color scheme (#e0e1dd background, #1d1a05 text, #778da9/#17255a accents)

**Hierarchical Taxon Queries:**
- [x] `get_sightings_by_taxon(&Taxon)` - Matches by taxonomic rank (e.g., family "Corvidae" shows all Blue Jay sightings)
- [x] `get_sightings_by_trip_id(trip_id)` - All sightings from a trip
- [x] `get_trips_by_taxon(&Taxon)` - All trips where taxon (or descendants) were seen

**Detail Pages:**

1. **Sighting Detail**
   - Entity type label, common name, full taxonomy
   - All metadata: date, location, notes, media path
   - Related taxon link (always present)
   - Related trip link (if sighting has trip)

2. **Taxon Detail**
   - Entity type label, common name, rank badge
   - Complete taxonomy breakdown (kingdom → species)
   - Related sightings list (includes all descendant taxa)
   - Related trips list (all trips where this taxon was seen)

3. **Trip Detail**
   - Entity type label, trip name
   - Trip metadata: date, location, notes
   - Related taxa list (distinct taxa from sightings)
   - Related sightings list

**Navigation:**
- Back button on all detail pages
- Click related entities to navigate between pages
- Hierarchical queries ensure family-level taxa show species-level sightings

**UI Polish:**
- Proper vertical alignment (no spacing issues with optional content)
- Hover states on all clickable cards
- Responsive flickable layouts for long content

### 🚧 Phase 3 — Full-Text Search

**Blazing Fast Note Search:**
- [ ] SQLite FTS5 (Full-Text Search) for instant note/description searching
- [ ] Index all text fields: sighting notes, trip notes, taxon common names
- [ ] Support advanced FTS5 queries:
  - Phrase search: `"red tailed hawk"`
  - Boolean operators: `hawk AND (red OR tail)`
  - Prefix matching: `cor*` matches "corvid", "corvidae", "corn"
  - Near operator: `NEAR(blue jay, 5)` - words within 5 tokens
- [ ] Integrate FTS results into existing search UI
- [ ] Highlight matching text snippets in results
- [ ] Performance target: <10ms for any note search on 10,000+ sightings

### 🚧 Phase 4 — Enhanced UI & Navigation

**Navigation Stack:**
- [ ] Implement proper navigation history stack
  - Back button navigates to previous page (not just search)
  - Example flow: Search → Taxon → Sighting → Trip → (back) → Sighting → (back) → Taxon
- [ ] "Home" button always visible to return to search from any page
- [ ] Breadcrumb trail showing current navigation path

**Taxonomy Breadcrumb Navigation:**
- [ ] Click any rank in taxonomy string to view that taxon's detail page
  - Example: "Animalia / Chordata / Aves / **Passeriformes** / Corvidae / Cyanocitta / cristata"
  - Click "Passeriformes" → view Order detail page with all related sightings/trips
- [ ] Auto-create taxon entries for parent ranks if they don't exist

**Paginated Lists:**
- [ ] Limit related entity lists on detail pages (show first 5-10 items)
- [ ] "See All" button to navigate to dedicated list page
  - Example: Corvidae taxon page shows 6 sightings → "See All 24 Sightings"
  - Dedicated page: "Corvidae - All Sightings" with full list
- [ ] Apply to all detail/list combinations:
  - Taxon → Sightings, Taxon → Trips
  - Trip → Sightings, Trip → Taxa
  - (Sighting pages already show single related entities)

**Advanced Search Syntax:**
- [ ] Type-specific search: `type:query`
  - `sighting:hawk` - Search only sightings
  - `taxon:corvidae` - Search only taxa
  - `trip:ozark` - Search only trips
- [ ] Date range filters: `date:2025-01-01..2025-12-31`
- [ ] Location filters: `location:park`
- [ ] Combined filters: `sighting:hawk date:2025 location:park`

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

### Run GUI (default)

```bash
# Launch GUI with no arguments
cargo run

# Or after install:
fast-watcher
```

### Run CLI

```bash
# CLI requires subcommand arguments
cargo run -- search-sightings "hawk"
cargo run -- add-trip "Morning hike" -l "Yosemite"

# Or after install:
fast-watcher search-sightings "hawk"
fast-watcher add-trip "Morning hike" -l "Yosemite"
```

### Build and install system-wide

```bash
cargo install --path .
fast-watcher              # Launch GUI
fast-watcher init-db      # Use CLI
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

- **32 unit tests** in `src/core/` modules (taxon, trip, sighting, search)
- **10 integration tests** in `tests/integration_test.rs`
- All tests use in-memory SQLite databases (won't affect `fast_watcher.db`)
- **42 total tests** - all passing ✅

### Test organization

- **Unit tests** (`#[cfg(test)]` modules): Test individual CRUD and search functions
- **Integration tests** (`tests/` directory): Test complete workflows and cross-entity operations

---

## 🟞️ Roadmap

- **Phase 1** ✅ → Core engine + CLI + SQLite + comprehensive tests
- **Phase 2** ✅ → Slint UI + hierarchical navigation + instant search
- **Phase 3** 🚧 → FTS5 full-text search (blazing fast note search)
- **Phase 4** → Enhanced UI (navigation stack, breadcrumb nav, pagination, advanced search)
- **Phase 5** → Portable export/import (Markdown + media archives)
