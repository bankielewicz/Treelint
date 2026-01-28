---
last_updated: 2026-01-27
status: LOCKED
requires_approval: Tech Lead
project: Treelint
complexity_tier: 2
architecture_pattern: Modular Monolith
---

# Architecture Constraints

**CRITICAL:** This document defines architectural boundaries AI agents MUST NOT violate.

**Violations result in technical debt and will be rejected during review.**

---

## Architecture Overview

**Pattern:** Modular Monolith (Tier 2)
**Structure:** Single binary with clear module boundaries
**Layers:** CLI → Application → Domain → Infrastructure

```
┌─────────────────────────────────────────────────────────┐
│                      CLI Layer                          │
│    (clap, argument parsing, command routing)            │
├─────────────────────────────────────────────────────────┤
│                  Application Layer                      │
│    (commands/, orchestration, workflow coordination)    │
├─────────────────────────────────────────────────────────┤
│                    Domain Layer                         │
│    (parser/, index/, graph/, output/)                   │
├─────────────────────────────────────────────────────────┤
│                 Infrastructure Layer                    │
│    (SQLite, file system, IPC, tree-sitter bindings)     │
└─────────────────────────────────────────────────────────┘
```

---

## Layer Rules

### CLI Layer (`src/cli/`, `src/main.rs`)

**Purpose:** User interface, argument parsing, output formatting

**Allowed:**
- ✅ clap argument parsing
- ✅ Calling application layer
- ✅ Formatting output for display
- ✅ TTY detection and color handling

**Forbidden:**
- ❌ Direct database access
- ❌ Direct tree-sitter calls
- ❌ Business logic
- ❌ File parsing

```rust
✅ CORRECT:
// src/cli/commands/search.rs
pub fn execute(args: SearchArgs) -> Result<()> {
    let searcher = Searcher::new()?;          // Application layer
    let results = searcher.search(&args)?;    // Application layer
    output::display(results, args.format)?;   // CLI layer formatting
    Ok(())
}

❌ FORBIDDEN:
// src/cli/commands/search.rs
pub fn execute(args: SearchArgs) -> Result<()> {
    let conn = Connection::open(".treelint/index.db")?;  // WRONG: Direct DB
    let mut parser = tree_sitter::Parser::new();         // WRONG: Direct parser
    // ...
}
```

### Application Layer (`src/cli/commands/`)

**Purpose:** Orchestration, coordination, workflow management

**Allowed:**
- ✅ Coordinating domain services
- ✅ Transaction boundaries
- ✅ Input validation (business rules)
- ✅ Error aggregation

**Forbidden:**
- ❌ Direct infrastructure calls
- ❌ tree-sitter API directly
- ❌ SQLite API directly
- ❌ UI/display logic

```rust
✅ CORRECT:
// src/cli/commands/search.rs
use crate::index::Searcher;
use crate::parser::SymbolExtractor;

pub struct SearchCommand {
    searcher: Searcher,
}

impl SearchCommand {
    pub fn execute(&self, query: &str) -> Result<Vec<Symbol>> {
        self.searcher.find_symbols(query)  // Domain layer call
    }
}

❌ FORBIDDEN:
// Direct rusqlite in application layer
use rusqlite::Connection;

pub fn search(query: &str) -> Result<Vec<Symbol>> {
    let conn = Connection::open("...")?;  // WRONG: Use Searcher service
    conn.prepare("SELECT * FROM symbols WHERE name = ?")?;
}
```

### Domain Layer (`src/parser/`, `src/index/`, `src/graph/`, `src/output/`)

**Purpose:** Core business logic, algorithms, domain models

**Allowed:**
- ✅ Symbol extraction logic
- ✅ Search algorithms
- ✅ Graph analysis
- ✅ Domain types (Symbol, File, Query)
- ✅ Calling infrastructure layer

**Forbidden:**
- ❌ CLI concerns (colors, progress bars)
- ❌ Direct file I/O (use infrastructure)
- ❌ Configuration reading

```rust
✅ CORRECT:
// src/parser/symbols.rs
pub struct SymbolExtractor {
    parser: Box<dyn Parser>,  // Abstraction over tree-sitter
}

impl SymbolExtractor {
    pub fn extract(&self, source: &str, language: Language) -> Vec<Symbol> {
        // Business logic for symbol extraction
    }
}

❌ FORBIDDEN:
// src/parser/symbols.rs
use indicatif::ProgressBar;  // WRONG: UI concern in domain

pub fn extract(path: &Path) -> Vec<Symbol> {
    let pb = ProgressBar::new(100);  // WRONG: UI in domain
    let content = std::fs::read_to_string(path)?;  // WRONG: Direct I/O
}
```

### Infrastructure Layer (`src/index/storage.rs`, `src/daemon/`)

**Purpose:** External system integration, persistence, I/O

**Allowed:**
- ✅ SQLite operations
- ✅ File system operations
- ✅ tree-sitter bindings
- ✅ IPC (daemon communication)
- ✅ File watching (notify)

**Forbidden:**
- ❌ Business logic
- ❌ Domain calculations
- ❌ CLI/display concerns

```rust
✅ CORRECT:
// src/index/storage.rs
pub struct SqliteStorage {
    conn: Connection,
}

impl Storage for SqliteStorage {
    fn save_symbol(&self, symbol: &Symbol) -> Result<()> {
        self.conn.execute(
            "INSERT INTO symbols (name, type, file) VALUES (?1, ?2, ?3)",
            params![&symbol.name, &symbol.symbol_type, &symbol.file_path],
        )?;
        Ok(())
    }
}
```

---

## Module Boundaries

### Parser Module (`src/parser/`)

**Responsibility:** AST parsing and symbol extraction

**Public API:**
- `Symbol` - Domain type
- `Language` - Enum of supported languages
- `SymbolExtractor` - Main service
- `extract_symbols(source, language)` - Core function

**Dependencies (inward only):**
- ← CLI commands call parser
- → Parser calls nothing in application/CLI

### Index Module (`src/index/`)

**Responsibility:** Storage and retrieval of symbols

**Public API:**
- `Searcher` - Search service
- `IndexBuilder` - Index construction
- `Storage` - Trait for persistence

**Dependencies:**
- ← CLI commands call index
- → Index may call parser (for indexing)
- → Index calls infrastructure (SQLite)

### Graph Module (`src/graph/`)

**Responsibility:** Dependency analysis

**Public API:**
- `CallGraph` - Function call relationships
- `ImportGraph` - Module import relationships

**Dependencies:**
- ← CLI commands call graph
- → Graph calls parser (for analysis)
- → Graph calls index (for symbol lookup)

### Output Module (`src/output/`)

**Responsibility:** Formatting results

**Public API:**
- `JsonFormatter`
- `TextFormatter`
- `MermaidFormatter`

**Dependencies:**
- ← CLI calls output
- → Output reads domain types

---

## Dependency Rules

### Allowed Dependencies

```
CLI → Application → Domain → Infrastructure
 ↓         ↓          ↓            ↓
 │         │          │            │
 └─────────┴──────────┴────────────┘
           (downward only)
```

### Forbidden Dependencies

```
❌ Infrastructure → Domain
❌ Infrastructure → Application
❌ Infrastructure → CLI
❌ Domain → CLI
❌ Application → CLI (except return values)
```

### Circular Dependencies

**STRICTLY FORBIDDEN:**

```rust
❌ FORBIDDEN:
// src/parser/mod.rs
use crate::index::Searcher;  // Parser depends on Index

// src/index/mod.rs
use crate::parser::SymbolExtractor;  // Index depends on Parser

// This creates: parser ↔ index circular dependency
```

**Resolution:**
1. Extract shared types to common module
2. Use dependency injection
3. Use traits for abstraction

---

## Abstraction Rules

### Rule 1: Depend on Abstractions

```rust
✅ CORRECT:
// Define trait in domain
pub trait Storage {
    fn save_symbol(&self, symbol: &Symbol) -> Result<()>;
    fn find_symbols(&self, query: &str) -> Result<Vec<Symbol>>;
}

// Implement in infrastructure
pub struct SqliteStorage { ... }
impl Storage for SqliteStorage { ... }

// Use trait in application
pub struct SearchCommand<S: Storage> {
    storage: S,
}

❌ FORBIDDEN:
// Direct concrete dependency
pub struct SearchCommand {
    storage: SqliteStorage,  // Concrete type
}
```

### Rule 2: No Leaky Abstractions

```rust
✅ CORRECT:
// Domain types don't expose infrastructure
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub file_path: String,
    pub lines: (usize, usize),
}

❌ FORBIDDEN:
// Exposing database IDs
pub struct Symbol {
    pub id: i64,  // SQLite implementation detail
    pub row_id: i64,  // Leaky abstraction
}
```

---

## Error Handling Architecture

### Error Boundaries

Each layer has its own error types:

```rust
// Domain errors
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),
}

// Infrastructure errors
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
}

// Application errors (aggregate)
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
}
```

### Error Translation

Infrastructure errors must be translated at boundaries:

```rust
✅ CORRECT:
impl Storage for SqliteStorage {
    fn find_symbols(&self, query: &str) -> Result<Vec<Symbol>, StorageError> {
        self.conn.query_row(...)
            .map_err(StorageError::Database)?;  // Translate at boundary
    }
}

❌ FORBIDDEN:
// Leaking rusqlite::Error to domain
fn find_symbols(&self, query: &str) -> Result<Vec<Symbol>, rusqlite::Error> {
    self.conn.query_row(...)?;  // Exposes infrastructure error
}
```

---

## Performance Constraints

### Query Latency

- **Target:** < 50ms for indexed queries
- **Maximum:** 100ms (triggers optimization review)

### Memory Usage

- **Indexing:** Chunked processing, bounded memory
- **Search Results:** Paginated, lazy loading for large result sets

### Binary Size

- **Target:** < 50MB
- **Action if exceeded:** Review grammar embedding, consider lazy loading

---

## Concurrency Rules

### SQLite Access

```rust
✅ CORRECT:
// WAL mode for concurrent reads
conn.pragma_update(None, "journal_mode", "WAL")?;

// Single writer, multiple readers
// Daemon writes, CLI reads

❌ FORBIDDEN:
// Multiple writers without coordination
thread::spawn(|| { conn.execute(...) });
thread::spawn(|| { conn.execute(...) });
```

### Daemon Communication

- IPC via Unix socket (or named pipe on Windows)
- JSON message protocol
- Request-response pattern

---

## Related Documents

- [tech-stack.md](./tech-stack.md) - Approved technologies
- [source-tree.md](./source-tree.md) - File structure
- [coding-standards.md](./coding-standards.md) - Code style
- [anti-patterns.md](./anti-patterns.md) - Forbidden patterns

---

## Change Log

| Date | Change | Approver | Reason | ADR |
|------|--------|----------|--------|-----|
| 2026-01-27 | Initial version | Bryan | Project setup | ADR-001 |
