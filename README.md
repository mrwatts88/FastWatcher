# **Fast Watcher**

### 🦜️ Personal Field Archive for Trips, Sightings, and Species

**Fast Watcher** is an **offline-first, local archive app** for nature enthusiasts, field biologists, and hobbyist observers — think _bird-watching_, but for **any** organism.

Data is stored in a **blazing-fast SQLite database** with denormalized fields optimized for instant search. Taxonomic data is duplicated into sightings to eliminate expensive JOINs during search operations.

The philosophy: **your data lives locally, searches are instant.**

---

## 🧯 Project Overview

| Entity       | Description                                                                     |
| ------------ | ------------------------------------------------------------------------------- |
| **Trip**     | A single outing (date, location, notes). Optional for sightings.                |
| **Sighting** | A single observation of a taxon, optionally linked to a trip (notes, media).    |
| **Taxon**    | Canonical taxonomy record (kingdom → species_epithet) with common name.         |

Relationship:

```
Trip (0..1) ───< Sighting >─── (1) Taxon
```

**Note:** Sightings denormalize taxonomic fields from Taxa for blazing-fast search without JOINs.

---

## 🧹 Current Architecture

```
fast_watcher/
├── Cargo.toml
├── init.sql             # Database schema
├── seed_*.sql           # Seed data files
└── src/
    ├── main.rs          # CLI entrypoint
    ├── cli/             # Argument parsing & command routing
    │   └── mod.rs
    ├── models/          # Data models
    │   ├── mod.rs
    │   ├── sighting.rs
    │   ├── taxon.rs
    │   └── trip.rs
    └── core/            # Core logic
        ├── mod.rs
        ├── db.rs        # Database connection & utilities
        ├── search.rs    # Search functions
        ├── sighting.rs  # Sighting CRUD operations
        ├── taxon.rs     # Taxon CRUD operations
        └── trip.rs      # Trip CRUD operations
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
fast-watcher add-taxon <kingdom> <phylum> <class> <order> <family> <genus> <species_epithet> <common_name>
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

# Add a taxon
$ fast-watcher add-taxon Animalia Chordata Aves Passeriformes Corvidae Cyanocitta cristata "Blue Jay"
Taxon created with ID: 10

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
- [x] Database initialization and seeding
- [x] `cargo install --path .` for system-wide binary
- [ ] SQLite FTS5 full-text search (future optimization)
- [ ] File system indexing (watcher)

### 🚧 Phase 2 — Slint UI

- Bind Rust core logic to a native desktop app
- Keep file-based storage philosophy intact
- Add offline-search and browsing experience

---

## 🧠 Rust Concepts We Covered

| Concept                     | Summary                                                                               | Example                                                      |
| --------------------------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| **Modules (`mod` / `use`)** | `mod` declares a module; `use` brings items into scope.                               | `mod core; use core::search::run_search_sightings;`          |
| **Results & `?` operator**  | `Result<T, E>` encodes success/failure; `?` bubbles up errors.                        | `let contents = fs::read_to_string(p)?;`                     |
| **`anyhow`**                | Universal error wrapper; `bail!()` for quick returns; `.context()` adds cause chains. | `bail!("empty query");` / `.context("Failed to insert")?`   |
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

## 🟞️ Roadmap

- **Phase 1** → core engine + SQLite search
- **Phase 2** → file watcher + rich indexing
- **Phase 3** → native UI with Slint
- **Phase 4** → portable export/import (Markdown + media)
