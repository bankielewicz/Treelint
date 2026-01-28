---
last_updated: 2026-01-27
status: LOCKED
requires_approval: Tech Lead
project: Treelint
complexity_tier: 2
---

# Technology Stack

**CRITICAL:** This document defines the ONLY approved technologies for this project.

**AI agents MUST NOT introduce alternatives without explicit user approval via AskUserQuestion.**

---

## Language & Runtime

**Primary Language:** Rust
- **Edition:** 2021
- **MSRV:** 1.70.0 (Minimum Supported Rust Version)
- **Rationale:** Single binary distribution, cross-platform, high performance, memory safety
- **Prohibited:** C/C++, Go, Python (unless for tooling scripts)

---

## Parser

**AST Parser:** tree-sitter
- **Crate:** tree-sitter (core) + tree-sitter-{language} per grammar
- **Grammars:** Embedded via build.rs compilation
- **Rationale:** Incremental parsing, 100+ language support, mature Rust bindings
- **Prohibited:**
  - ❌ syn (Rust-only, not cross-language)
  - ❌ ANTLR (heavy runtime, complex setup)
  - ❌ pest (less mature, smaller ecosystem)

**Supported Languages (v1):**
- Python (tree-sitter-python)
- TypeScript (tree-sitter-typescript)
- Rust (tree-sitter-rust)
- Markdown (tree-sitter-markdown)

**⚠️ CRITICAL RULE:** Grammar embedding is mandatory. DO NOT use external grammar files.

---

## Database

**Storage:** SQLite
- **Crate:** rusqlite 0.31.x with `bundled` feature
- **Why:** Portable single-file database, zero configuration, WAL mode for concurrency
- **Location:** `.treelint/index.db` in project root
- **Prohibited:**
  - ❌ PostgreSQL (requires server, overkill for local tool)
  - ❌ MySQL (requires server)
  - ❌ RocksDB (too complex for our needs)
  - ❌ sled (less mature than SQLite)

**⚠️ CRITICAL RULE:** Use `bundled` feature to embed SQLite. No external SQLite dependency.

### Connection Configuration

```rust
// Required connection settings
conn.pragma_update(None, "journal_mode", "WAL")?;
conn.pragma_update(None, "synchronous", "NORMAL")?;
conn.pragma_update(None, "foreign_keys", "ON")?;
```

---

## CLI Framework

**CLI Parser:** clap 4.x
- **Features:** derive, cargo
- **Why:** Industry standard, derive macros, automatic help generation
- **Prohibited:**
  - ❌ structopt (deprecated, merged into clap)
  - ❌ argh (less feature-rich)
  - ❌ pico-args (too minimal)

---

## File System

**File Watching:** notify 6.x
- **Why:** Cross-platform (inotify, FSEvents, ReadDirectoryChangesW), mature
- **Prohibited:**
  - ❌ hotwatch (wrapper around notify, unnecessary)
  - ❌ Custom polling (inefficient)

**Path Handling:** std::path (built-in)
- **Additional:** walkdir 2.x for directory traversal
- **Prohibited:**
  - ❌ glob crate for file discovery (use walkdir)

---

## User Interface

**Progress Bars:** indicatif 0.17.x
- **Why:** Beautiful terminal progress, spinners, multi-progress support
- **Prohibited:**
  - ❌ pbr (less feature-rich)
  - ❌ progressbar (unmaintained)

**TTY Detection:** atty 0.2.x OR is-terminal 0.4.x
- **Why:** Auto-detect terminal vs pipe for output formatting

**Colored Output:** colored 2.x OR termcolor 1.x
- **Why:** Cross-platform ANSI color support

---

## Regex & Pattern Matching

**Regex Engine:** regex 1.x
- **Why:** Fast, Unicode-aware, well-documented
- **Prohibited:**
  - ❌ fancy-regex (unless lookahead/lookbehind needed, requires ADR)
  - ❌ pcre2 (external C dependency)

---

## Serialization

**JSON:** serde + serde_json 1.x
- **Features:** derive
- **Why:** De-facto standard, derive macros, excellent performance
- **Prohibited:**
  - ❌ json (less feature-rich)
  - ❌ simd-json (premature optimization)

**TOML:** toml 0.8.x (for config files)

---

## Error Handling

**Error Types:** thiserror 1.x
- **Why:** Derive macros for custom error types
- **Application Errors:** anyhow 1.x
- **Why:** Ergonomic error handling for applications

---

## Testing

**Unit/Integration Tests:** Built-in Rust testing
- **Framework:** `#[test]`, `#[cfg(test)]`
- **Assertion:** assert!, assert_eq!, assert_ne!
- **Additional:** pretty_assertions 1.x (for diff output)
- **Mocking:** mockall 0.12.x (if needed)

**Coverage Target:** 80%
- **Tool:** cargo-tarpaulin or cargo-llvm-cov

**Prohibited:**
  - ❌ External test frameworks (built-in is sufficient)

---

## Build & Distribution

**Build System:** Cargo (built-in)
- **Profile Optimization:** release profile with LTO
- **Target Platforms:**
  - x86_64-unknown-linux-gnu
  - x86_64-apple-darwin
  - aarch64-apple-darwin
  - x86_64-pc-windows-msvc

**Binary Size Optimization:**
```toml
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

---

## Logging

**Logging Facade:** log 0.4.x
- **Implementation:** env_logger 0.11.x OR tracing 0.1.x
- **Why:** Standard logging trait, configurable via RUST_LOG

---

## IPC (Daemon Mode)

**Unix Socket / Named Pipe:** interprocess 1.x OR std::os::unix
- **Why:** Cross-platform IPC for daemon communication
- **Protocol:** Simple line-based JSON messages
- **Prohibited:**
  - ❌ gRPC (overkill for local IPC)
  - ❌ HTTP (unnecessary overhead)

---

## Ambiguity Resolution Protocol

**CRITICAL:** When encountering ANY ambiguity, AI agents MUST follow this protocol:

### Step 1: Check This File
Before adding any crate or making any technology decision:
1. Search this file for relevant category
2. If documented → Use ONLY the specified crate
3. If not documented → Proceed to Step 2

### Step 2: Use AskUserQuestion
**Never assume.** Always ask:
```
Question: "This feature requires [functionality]. tech-stack.md doesn't specify. Which crate should I use?"
Options: [2-4 relevant crate options with rationale]
```

### Step 3: Update This File
After user confirms, update this file with the new crate and rationale.

---

## Common AI Mistakes to Prevent

### ❌ Mistake 1: Adding External Dependencies for Embedded Features

**WRONG:**
```rust
// Adding external SQLite
[dependencies]
libsqlite3-sys = "0.28"  // WRONG - requires system SQLite
```

**CORRECT:**
```rust
// Use bundled feature
[dependencies]
rusqlite = { version = "0.31", features = ["bundled"] }
```

### ❌ Mistake 2: Using External Grammar Files

**WRONG:**
```rust
let language = tree_sitter::Language::new(
    tree_sitter_python::language(),
    include_bytes!("../grammars/python.so")  // WRONG - external file
);
```

**CORRECT:**
```rust
let language = tree_sitter_python::language();  // Compiled in
```

### ❌ Mistake 3: Substituting Crates Due to "Issues"

**WRONG:**
```
AI: "rusqlite is having issues. Let me use sqlite instead."
```

**CORRECT:**
```
AI: "rusqlite query failed. Let me use AskUserQuestion to debug."
```

---

## Related Documents

- [dependencies.md](./dependencies.md) - Cargo.toml approved dependencies
- [architecture-constraints.md](./architecture-constraints.md) - Module boundaries
- [anti-patterns.md](./anti-patterns.md) - Forbidden patterns

---

## Change Log

| Date | Change | Approver | Reason | ADR |
|------|--------|----------|--------|-----|
| 2026-01-27 | Initial version | Bryan | Project setup | ADR-001 |

---

## Notes

> **Binary Size Target:** < 50MB with all 4 embedded grammars. If exceeded, evaluate grammar lazy-loading.

> **Performance Target:** < 50ms query latency on indexed repositories.
