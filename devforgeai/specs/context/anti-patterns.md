---
last_updated: 2026-01-27
status: LOCKED
requires_approval: Tech Lead
project: Treelint
---

# Anti-Patterns

**CRITICAL:** This document defines patterns that are EXPLICITLY FORBIDDEN in this project.

**AI agents MUST NOT implement these patterns. Detection results in immediate code rejection.**

---

## Category 1: Dependency Violations

### Anti-Pattern 1.1: Library Substitution

**Severity:** CRITICAL - Blocks merge

**Description:** Replacing an approved library with an alternative without ADR approval.

```rust
❌ FORBIDDEN:
// Substituting JSON library
use json::JsonValue;  // WRONG: Not approved, serde_json is locked

// Substituting database
use sled::Db;  // WRONG: SQLite is the approved database

// Substituting CLI parser
use argh::FromArgs;  // WRONG: clap is locked
```

```rust
✅ CORRECT:
// Use only approved libraries
use serde_json::Value;
use rusqlite::Connection;
use clap::Parser;
```

**Detection:** Grep for `use <unapproved_crate>`

**Resolution:** Remove unapproved crate, use approved alternative, or create ADR for exception.

---

### Anti-Pattern 1.2: Unapproved Feature Flags

**Severity:** HIGH

**Description:** Enabling Cargo features not listed in dependencies.md.

```toml
❌ FORBIDDEN:
[dependencies]
rusqlite = { version = "0.31", features = ["bundled", "backup", "blob", "hooks"] }
# Only "bundled" is approved

clap = { version = "4.5", features = ["derive", "cargo", "unicode", "wrap_help"] }
# Only "derive" and "cargo" are approved
```

```toml
✅ CORRECT:
[dependencies]
rusqlite = { version = "0.31", features = ["bundled"] }
clap = { version = "4.5", features = ["derive", "cargo"] }
```

---

## Category 2: Architecture Violations

### Anti-Pattern 2.1: Layer Bypass

**Severity:** CRITICAL - Blocks merge

**Description:** Calling infrastructure directly from CLI or calling CLI from domain.

```rust
❌ FORBIDDEN:
// CLI directly accessing database
// src/cli/commands/search.rs
use rusqlite::Connection;

pub fn execute(args: SearchArgs) -> Result<()> {
    let conn = Connection::open(".treelint/index.db")?;  // WRONG
    conn.execute("SELECT * FROM symbols", [])?;
}

// Domain importing CLI
// src/parser/symbols.rs
use crate::cli::output::format_result;  // WRONG
```

```rust
✅ CORRECT:
// CLI uses application layer
// src/cli/commands/search.rs
use crate::index::Searcher;

pub fn execute(args: SearchArgs) -> Result<()> {
    let searcher = Searcher::new()?;
    let results = searcher.search(&args.query)?;
    Ok(results)
}
```

---

### Anti-Pattern 2.2: Circular Dependencies

**Severity:** CRITICAL - Blocks merge

**Description:** Module A depends on Module B, which depends on Module A.

```rust
❌ FORBIDDEN:
// src/parser/mod.rs
use crate::index::Storage;  // Parser depends on Index

// src/index/mod.rs
use crate::parser::SymbolExtractor;  // Index depends on Parser

// Creates: parser ↔ index circular dependency
```

```rust
✅ CORRECT:
// Extract shared types to common module
// src/types/mod.rs
pub struct Symbol { ... }

// src/parser/mod.rs
use crate::types::Symbol;

// src/index/mod.rs
use crate::types::Symbol;
// Index can call Parser (one-way dependency)
use crate::parser::SymbolExtractor;
```

---

### Anti-Pattern 2.3: God Module

**Severity:** HIGH

**Description:** A single module with > 500 lines or multiple unrelated responsibilities.

```rust
❌ FORBIDDEN:
// src/everything.rs (800 lines)
// Contains: parsing, storage, output, CLI logic
pub fn parse_file() { ... }
pub fn save_to_db() { ... }
pub fn format_json() { ... }
pub fn handle_args() { ... }
```

```rust
✅ CORRECT:
// Split into focused modules
// src/parser/symbols.rs - only parsing
// src/index/storage.rs - only storage
// src/output/json.rs - only JSON formatting
// src/cli/args.rs - only argument handling
```

---

## Category 3: Error Handling Anti-Patterns

### Anti-Pattern 3.1: Unwrap in Production Code

**Severity:** CRITICAL - Blocks merge

**Description:** Using `.unwrap()` or `.expect()` outside of tests.

```rust
❌ FORBIDDEN:
// Production code
pub fn parse_file(path: &Path) -> Vec<Symbol> {
    let content = std::fs::read_to_string(path).unwrap();  // CRASH risk
    let ast = parser.parse(&content, None).unwrap();  // CRASH risk
    // ...
}
```

```rust
✅ CORRECT:
pub fn parse_file(path: &Path) -> Result<Vec<Symbol>, ParseError> {
    let content = std::fs::read_to_string(path)?;
    let ast = parser.parse(&content, None)
        .ok_or(ParseError::ParseFailed)?;
    // ...
}
```

**Exception:** Test code may use `unwrap()` with justification.

---

### Anti-Pattern 3.2: Swallowing Errors

**Severity:** HIGH

**Description:** Catching errors and ignoring them silently.

```rust
❌ FORBIDDEN:
pub fn index_file(path: &Path) {
    if let Ok(symbols) = parse_file(path) {
        save_symbols(symbols);
    }
    // Error silently ignored!
}

pub fn search(query: &str) -> Vec<Symbol> {
    storage.find(query).unwrap_or_default()  // Error hidden
}
```

```rust
✅ CORRECT:
pub fn index_file(path: &Path) -> Result<(), IndexError> {
    let symbols = parse_file(path)?;
    save_symbols(symbols)?;
    Ok(())
}

pub fn search(query: &str) -> Result<Vec<Symbol>, SearchError> {
    storage.find(query)
}
```

---

### Anti-Pattern 3.3: String Errors

**Severity:** MEDIUM

**Description:** Using `String` or `&str` as error types.

```rust
❌ FORBIDDEN:
pub fn parse_file(path: &Path) -> Result<Vec<Symbol>, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("IO error: {}", e))?;  // Loses error info
}
```

```rust
✅ CORRECT:
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Failed to read file: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn parse_file(path: &Path) -> Result<Vec<Symbol>, ParseError> {
    let content = std::fs::read_to_string(path)?;
}
```

---

## Category 4: Security Anti-Patterns

### Anti-Pattern 4.1: SQL Injection

**Severity:** CRITICAL - Blocks merge

**Description:** Constructing SQL queries with string concatenation.

```rust
❌ FORBIDDEN:
pub fn search(name: &str) -> Result<Vec<Symbol>> {
    let sql = format!("SELECT * FROM symbols WHERE name = '{}'", name);
    conn.execute(&sql, [])?;  // SQL INJECTION RISK
}
```

```rust
✅ CORRECT:
pub fn search(name: &str) -> Result<Vec<Symbol>> {
    conn.prepare("SELECT * FROM symbols WHERE name = ?1")?
        .query_map(params![name], |row| { ... })?;
}
```

---

### Anti-Pattern 4.2: Hardcoded Paths

**Severity:** HIGH

**Description:** Hardcoding absolute paths instead of using proper path resolution.

```rust
❌ FORBIDDEN:
const DB_PATH: &str = "/home/user/.treelint/index.db";
const CONFIG_PATH: &str = "C:\\Users\\user\\.treelint\\config.toml";
```

```rust
✅ CORRECT:
fn get_db_path() -> PathBuf {
    let base = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    base.join(".treelint").join("index.db")
}
```

---

### Anti-Pattern 4.3: Unsafe Without Justification

**Severity:** CRITICAL - Requires ADR

**Description:** Using `unsafe` blocks without documented safety guarantees.

```rust
❌ FORBIDDEN:
unsafe {
    ptr::read(data.as_ptr())  // No safety comment
}
```

```rust
✅ CORRECT (requires ADR):
// SAFETY: `data` is guaranteed to be non-empty and properly aligned
// because we checked `!data.is_empty()` above and Vec<T> guarantees
// alignment for type T.
unsafe {
    ptr::read(data.as_ptr())
}
```

---

## Category 5: Performance Anti-Patterns

### Anti-Pattern 5.1: Blocking in Async (N/A - sync codebase)

Not applicable - Treelint uses synchronous Rust.

---

### Anti-Pattern 5.2: Unnecessary Cloning

**Severity:** MEDIUM

**Description:** Cloning data when borrowing would suffice.

```rust
❌ FORBIDDEN:
pub fn process_symbols(symbols: Vec<Symbol>) {  // Takes ownership
    for symbol in symbols.clone() {  // Unnecessary clone
        println!("{}", symbol.name.clone());  // Unnecessary clone
    }
}
```

```rust
✅ CORRECT:
pub fn process_symbols(symbols: &[Symbol]) {  // Borrows
    for symbol in symbols {
        println!("{}", &symbol.name);  // Reference
    }
}
```

---

### Anti-Pattern 5.3: Loading Entire File for Small Reads

**Severity:** MEDIUM

**Description:** Reading entire large files when only part is needed.

```rust
❌ FORBIDDEN:
pub fn get_first_line(path: &Path) -> String {
    let content = std::fs::read_to_string(path).unwrap();  // Reads ALL
    content.lines().next().unwrap().to_string()
}
```

```rust
✅ CORRECT:
pub fn get_first_line(path: &Path) -> io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    Ok(line)
}
```

---

## Category 6: Code Quality Anti-Patterns

### Anti-Pattern 6.1: Magic Numbers

**Severity:** LOW

**Description:** Using unexplained numeric literals.

```rust
❌ FORBIDDEN:
if file_size > 10485760 {  // What is this number?
    return Err(Error::FileTooLarge);
}

if symbols.len() > 1000 {  // Why 1000?
    paginate();
}
```

```rust
✅ CORRECT:
const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;  // 10 MB
const MAX_RESULTS_PER_PAGE: usize = 1000;

if file_size > MAX_FILE_SIZE {
    return Err(Error::FileTooLarge);
}
```

---

### Anti-Pattern 6.2: Deep Nesting

**Severity:** MEDIUM

**Description:** More than 3 levels of nesting.

```rust
❌ FORBIDDEN:
pub fn process(data: Option<Data>) {
    if let Some(d) = data {
        if d.is_valid() {
            for item in d.items {
                if item.enabled {
                    if let Some(value) = item.value {
                        // 5 levels deep!
                    }
                }
            }
        }
    }
}
```

```rust
✅ CORRECT:
pub fn process(data: Option<Data>) {
    let d = match data {
        Some(d) if d.is_valid() => d,
        _ => return,
    };

    for item in d.items.iter().filter(|i| i.enabled) {
        if let Some(value) = &item.value {
            process_value(value);
        }
    }
}
```

---

### Anti-Pattern 6.3: TODO/FIXME in Production

**Severity:** LOW (but tracked)

**Description:** Leaving TODO/FIXME comments in released code.

```rust
❌ FORBIDDEN (in released code):
pub fn search(query: &str) -> Vec<Symbol> {
    // TODO: implement regex support
    // FIXME: this is slow
    // HACK: temporary workaround
}
```

```rust
✅ CORRECT:
// Create story for future work
// Remove TODO and implement or document limitation

/// Note: Regex support planned for v1.1 (see EPIC-002)
pub fn search(query: &str) -> Vec<Symbol> {
    // Clean implementation
}
```

---

## Detection and Prevention

### Automated Checks

```bash
# Run before every commit
cargo fmt --check
cargo clippy -- -D warnings

# Check for anti-patterns
grep -r "\.unwrap()" src/ --include="*.rs" | grep -v "test"
grep -r "\.expect(" src/ --include="*.rs" | grep -v "test"
```

### CI/CD Enforcement

All anti-pattern checks run in CI. PRs with violations are blocked.

---

## Related Documents

- [tech-stack.md](./tech-stack.md) - Approved technologies
- [architecture-constraints.md](./architecture-constraints.md) - Layer rules
- [coding-standards.md](./coding-standards.md) - Style guide

---

## Change Log

| Date | Change | Approver | Reason |
|------|--------|----------|--------|
| 2026-01-27 | Initial version | Bryan | Project setup |
