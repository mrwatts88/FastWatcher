# **Fast Watcher**

### 🦜️ Personal Field Archive for Trips, Sightings, and Species

**Fast Watcher** is an **offline-first, local archive app** for nature enthusiasts, field biologists, and hobbyist observers — think _bird-watching_, but for **any** organism.

Each record you create lives as plain Markdown + media on your disk while being indexed in a blazing-fast SQLite full-text database for instant search.

The philosophy: **your data lives in files you own.**
The database is just an index; the app is replaceable.

---

## 🧯 Project Overview

| Entity       | Description                                                                     |
| ------------ | ------------------------------------------------------------------------------- |
| **Trip**     | A single outing (date, location, notes, folder).                                |
| **Sighting** | A single observation of one species on a trip (notes, media).                   |
| **Species**  | Canonical taxonomy record (species, genus, family) with parent/child hierarchy. |

Relationship:

```
Trip (1) ───< Sighting >─── (1) Species
```

---

## 🧹 Current Architecture

```
fast_watcher/
├── Cargo.toml
└── src/
    ├── main.rs          # CLI entrypoint
    ├── cli/             # Argument parsing & command routing
    │   └── mod.rs
    └── core/            # Core logic
        ├── mod.rs
        ├── search.rs    # Search functions
        ├── files.rs     # Indexing / filesystem
        └── taxonomy.rs  # (future) species tree logic
```

### 🮀 Tech Stack

- **Language:** Rust
- **CLI Framework:** [clap](https://docs.rs/clap/latest/clap/) (`derive` API)
- **Error Handling:** [anyhow](https://docs.rs/anyhow)
- **Database:** SQLite + FTS5 (planned)
- **UI (future):** [Slint](https://slint.dev/) for a native desktop layer

---

## ⚙️ Current CLI Commands

```
fast-watcher search <query>
fast-watcher index
```

### Example

```
$ fast-watcher search bluejay
Results for bluejay
```

---

## 🧱 Development Status

### ✅ Phase 1 — CLI Engine (Current)

- [x] Modular project structure (`core`, `cli`, `main`)
- [x] `clap`-based argument parsing
- [x] `anyhow` error handling with `bail!` and `?`
- [x] `cargo install --path .` for system-wide binary
- [x] Debugging via VS Code + CodeLLDB
- [ ] File system indexing (watcher)
- [ ] SQLite + FTS integration

### 🚧 Phase 2 — Slint UI

- Bind Rust core logic to a native desktop app
- Keep file-based storage philosophy intact
- Add offline-search and browsing experience

---

## 🧠 Rust Concepts We Covered

| Concept                     | Summary                                                                               | Example                                                      |
| --------------------------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| **Modules (`mod` / `use`)** | `mod` declares a module; `use` brings items into scope.                               | `mod core; use core::search::run_search;`                    |
| **Results & `?` operator**  | `Result<T, E>` encodes success/failure; `?` bubbles up errors.                        | `let contents = fs::read_to_string(p)?;`                     |
| **`anyhow`**                | Universal error wrapper; `bail!()` for quick returns; `.context()` adds cause chains. | `bail!("empty query");`                                      |
| **Traits / Impl blocks**    | Behavior defined separately from data.                                                | `impl Dog { fn bark(&self){} }` / `impl Speak for Dog { … }` |
| **`into()` / `from()`**     | Generic, type-inferred conversions.                                                   | `"hawk".into()` → `String` or `Vec<u8>`                      |
| **Implicit returns**        | Last line without `;` is returned automatically.                                      | `Ok(format!("Results for {query}"))`                         |
| **CLI via Clap**            | Derive `Parser` + `Subcommand` → instant help, validation, routing.                   | `#[derive(Parser)] struct Cli { ... }`                       |
| **`cargo install`**         | Builds & installs binary to `~/.cargo/bin`.                                           | `cargo install --path .`                                     |

---

## 🚀 Quick Start

### Run locally

```
cargo run -- search "hawk"
```

### Build and install system-wide

```
cargo install --path .
fast-watcher search "hawk"
```

### Uninstall

```
cargo uninstall fast-watcher
```

---

## 🟞️ Roadmap

- **Phase 1** → core engine + SQLite search
- **Phase 2** → file watcher + rich indexing
- **Phase 3** → native UI with Slint
- **Phase 4** → portable export/import (Markdown + media)
