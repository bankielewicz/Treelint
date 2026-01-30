# Treelint Developer Guide

Complete guide for developers contributing to or extending Treelint.

---

## Overview

Treelint is an AST-aware code search CLI written in Rust. This guide covers:
- Development environment setup
- Architecture and code organization
- Coding standards and conventions
- Testing practices
- Contribution workflow

---

## Development Environment

### Prerequisites

| Requirement | Version | Purpose |
|-------------|---------|---------|
| Rust | 1.70.0+ | Primary language |
| Cargo | (bundled) | Build system |
| Git | 2.x+ | Version control |

### Setup

```bash
# Clone the repository
git clone https://github.com/treelint/treelint
cd treelint

# Verify Rust version
rustc --version
# rustc 1.70.0 or higher

# Build debug version
cargo build

# Run tests to verify setup
cargo test
```

### IDE Configuration

**VS Code (Recommended):**
- Install `rust-analyzer` extension
- Enable format on save with rustfmt

**IntelliJ IDEA:**
- Install `IntelliJ Rust` plugin

---

## Build Commands

```bash
# Debug build (fast compilation, slower runtime)
cargo build

# Release build (slow compilation, optimized runtime)
cargo build --release

# Check without building (fastest feedback)
cargo check

# Build documentation
cargo doc --open
```

### Build Artifacts

| Artifact | Location | Size |
|----------|----------|------|
| Debug binary | `target/debug/treelint` | ~50 MB |
| Release binary | `target/release/treelint` | ~7.6 MB |
| Index database | `.treelint/index.db` | varies |

---

## Project Structure

```
treelint/
├── src/                      # Source code
│   ├── main.rs               # Entry point
│   ├── lib.rs                # Library exports
│   ├── error.rs              # Error types
│   ├── cli/                  # CLI layer
│   │   ├── args.rs           # Argument definitions
│   │   └── commands/         # Command implementations
│   ├── parser/               # AST parsing
│   │   ├── languages.rs      # Language detection
│   │   ├── symbols.rs        # Symbol extraction
│   │   ├── context.rs        # Context modes
│   │   └── queries/          # tree-sitter queries
│   ├── index/                # Symbol storage
│   │   ├── schema.rs         # DB schema
│   │   ├── storage.rs        # SQLite operations
│   │   └── search.rs         # Query execution
│   ├── daemon/               # Background service
│   │   ├── server.rs         # IPC server
│   │   ├── protocol.rs       # NDJSON protocol
│   │   └── watcher.rs        # File watcher
│   └── output/               # Output formatting
│       ├── json.rs           # JSON formatter
│       └── text.rs           # Text formatter
├── tests/                    # Integration tests
│   ├── story_*.rs            # Story-based test files
│   ├── STORY-*/              # Per-story test modules
│   ├── fixtures/             # Test data
│   └── common/               # Test utilities
├── docs/                     # Documentation
│   ├── api/                  # API reference
│   ├── architecture/         # Architecture docs
│   └── guides/               # User guides
├── devforgeai/               # DevForgeAI artifacts
│   └── specs/
│       ├── context/          # Constraint files
│       ├── Stories/          # User stories
│       └── adrs/             # Architecture decisions
├── Cargo.toml                # Dependencies
├── Cargo.lock                # Locked dependencies
├── CLAUDE.md                 # AI agent instructions
├── README.md                 # Project readme
└── CHANGELOG.md              # Version history
```

---

## Architecture

### Layer Architecture

Treelint follows a strict 4-layer architecture:

```
┌─────────────────────────────────────────────┐
│                CLI Layer                     │
│  src/cli/, src/main.rs                      │
│  Argument parsing, command routing          │
├─────────────────────────────────────────────┤
│            Application Layer                 │
│  src/cli/commands/                          │
│  Command implementations, orchestration     │
├─────────────────────────────────────────────┤
│              Domain Layer                    │
│  src/parser/, src/index/, src/output/       │
│  Business logic, algorithms                 │
├─────────────────────────────────────────────┤
│           Infrastructure Layer               │
│  src/daemon/, SQLite, tree-sitter           │
│  External integrations, I/O                 │
└─────────────────────────────────────────────┘
```

### Dependency Rules

| Layer | Can Import From | Cannot Import From |
|-------|-----------------|-------------------|
| CLI | Application, Domain, Infra | - |
| Application | Domain, Infrastructure | CLI |
| Domain | Infrastructure | CLI, Application |
| Infrastructure | - | All upper layers |

### Key Modules

| Module | Purpose | Key Types |
|--------|---------|-----------|
| `parser` | AST extraction | `SymbolExtractor`, `Language`, `Symbol` |
| `index` | Symbol storage | `IndexStorage`, `QueryFilters` |
| `daemon` | Background service | `DaemonServer`, `FileWatcher` |
| `output` | Result formatting | `JsonFormatter`, `TextFormatter` |

---

## Coding Standards

### Formatting

```bash
# Check formatting
cargo fmt --check

# Apply formatting
cargo fmt
```

**Rules:**
- Use `rustfmt` defaults
- Maximum line length: 100 characters
- Run `cargo fmt` before every commit

### Linting

```bash
# Run clippy (fail on warnings)
cargo clippy -- -D warnings
```

**Rules:**
- All clippy warnings must be resolved
- Use `#[allow(...)]` with documented justification only

### Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Files | snake_case | `symbol_extractor.rs` |
| Types | PascalCase | `SymbolType` |
| Functions | snake_case | `extract_symbols()` |
| Constants | SCREAMING_SNAKE | `MAX_FILE_SIZE` |
| Variables | snake_case | `file_path` |

### Error Handling

```rust
// ✅ CORRECT: Use Result with custom error
pub fn parse_file(path: &Path) -> Result<Vec<Symbol>, TreelintError> {
    let content = std::fs::read_to_string(path)?;
    // ...
}

// ❌ FORBIDDEN: No .unwrap() in production code
pub fn parse_file(path: &Path) -> Vec<Symbol> {
    let content = std::fs::read_to_string(path).unwrap(); // WRONG
}
```

### Documentation

```rust
/// Extracts symbols from a source file.
///
/// # Arguments
///
/// * `path` - Path to the source file
///
/// # Returns
///
/// Vector of extracted symbols, or error if parsing fails.
///
/// # Example
///
/// ```
/// let symbols = extractor.extract_from_file(Path::new("main.py"))?;
/// ```
pub fn extract_from_file(&self, path: &Path) -> Result<Vec<Symbol>> {
    // ...
}
```

---

## Testing

### Test Organization

```
tests/
├── story_001.rs          # Entry point for STORY-001
├── STORY-001/
│   ├── mod.rs            # Module exports
│   ├── test_ac1_*.rs     # AC#1 tests
│   └── test_ac2_*.rs     # AC#2 tests
├── fixtures/
│   ├── python/           # Python test files
│   ├── typescript/       # TypeScript test files
│   └── rust/             # Rust test files
└── common/
    └── mod.rs            # Shared test utilities
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for specific story
cargo test --test story_008

# Run tests matching pattern
cargo test test_watcher

# Run with output
cargo test -- --nocapture

# Run single-threaded (for debugging)
cargo test -- --test-threads=1
```

### Test Naming

```rust
#[test]
fn test_extract_function_returns_name_and_lines() {
    // Test name: test_<function>_<scenario>_<expected>
}
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_language_detection_for_python() {
        let lang = Language::from_extension(".py");
        assert_eq!(lang, Some(Language::Python));
    }

    #[test]
    fn test_symbol_extraction_handles_empty_file() {
        let extractor = SymbolExtractor::new();
        let symbols = extractor.extract_from_content("", Language::Python).unwrap();
        assert!(symbols.is_empty());
    }
}
```

### Coverage Targets

| Layer | Target | Enforcement |
|-------|--------|-------------|
| Business Logic | 95% | QA blocking |
| Application | 85% | QA blocking |
| Infrastructure | 80% | QA blocking |

---

## Development Workflow

### TDD Cycle

Treelint follows Test-Driven Development:

1. **Red**: Write failing test
2. **Green**: Write minimal code to pass
3. **Refactor**: Improve without changing behavior

```bash
# 1. Write test (it should fail)
cargo test test_new_feature
# FAILED

# 2. Implement feature
# ... write code ...

# 3. Verify test passes
cargo test test_new_feature
# PASSED

# 4. Refactor if needed
cargo clippy
cargo fmt
```

### Pre-Commit Checklist

```bash
# Run all quality gates
cargo test && cargo clippy -- -D warnings && cargo fmt --check
```

### Story-Based Development

Each feature is developed against a user story:

1. Read story in `devforgeai/specs/Stories/`
2. Understand acceptance criteria
3. Write tests for each AC
4. Implement to pass tests
5. Run QA validation

---

## Adding New Features

### Adding a New Language

1. **Add grammar dependency** in `Cargo.toml`:
   ```toml
   tree-sitter-go = "0.21"
   ```

2. **Update Language enum** in `src/parser/languages.rs`:
   ```rust
   pub enum Language {
       // ...
       Go,
   }
   ```

3. **Add extension mapping**:
   ```rust
   fn from_extension(ext: &str) -> Option<Language> {
       match ext {
           ".go" => Some(Language::Go),
           // ...
       }
   }
   ```

4. **Create query file** in `src/parser/queries/go.rs`

5. **Add tests** in `tests/fixtures/go/`

### Adding a New Command

1. **Create command file** in `src/cli/commands/`:
   ```rust
   // src/cli/commands/map.rs
   pub fn execute(args: MapArgs) -> Result<()> {
       // ...
   }
   ```

2. **Register in mod.rs**:
   ```rust
   pub mod map;
   ```

3. **Add to args.rs**:
   ```rust
   #[derive(Subcommand)]
   pub enum Commands {
       Map(MapArgs),
   }
   ```

4. **Add tests** in `tests/`

---

## Debugging

### Debug Logging

```bash
# Enable debug output
RUST_LOG=debug cargo run -- search main

# Enable trace output (very verbose)
RUST_LOG=trace cargo run -- search main
```

### Common Issues

| Issue | Solution |
|-------|----------|
| "Index not found" | Run search first to create index |
| Build failures | Check `rustc --version` ≥ 1.70.0 |
| Test failures | Run `cargo test -- --test-threads=1` |
| Clippy warnings | Fix all warnings before commit |

### Profiling

```bash
# Build with profiling
cargo build --release

# Use perf (Linux)
perf record ./target/release/treelint search main
perf report

# Use Instruments (macOS)
instruments -t "Time Profiler" ./target/release/treelint search main
```

---

## Architecture Decision Records

Major decisions are documented in ADRs:

| ADR | Decision |
|-----|----------|
| [ADR-001](../../devforgeai/specs/adrs/ADR-001-initial-architecture.md) | Rust + tree-sitter + SQLite |
| [ADR-002](../../devforgeai/specs/adrs/ADR-002-sha2-crate-for-file-hashing.md) | SHA-256 for file change detection |

When making architectural changes, create a new ADR.

---

## Context Files (Constraints)

These files define immutable project constraints:

| File | Purpose |
|------|---------|
| `tech-stack.md` | Approved technologies only |
| `source-tree.md` | File/folder structure |
| `dependencies.md` | Approved Cargo dependencies |
| `coding-standards.md` | Code style rules |
| `architecture-constraints.md` | Layer boundaries |
| `anti-patterns.md` | Forbidden patterns |

**Changes require an ADR and Tech Lead approval.**

---

## Release Process

1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG.md** with new version
3. **Run full test suite**: `cargo test`
4. **Build release**: `cargo build --release`
5. **Tag release**: `git tag v0.x.0`
6. **Push**: `git push && git push --tags`

---

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [tree-sitter Documentation](https://tree-sitter.github.io/tree-sitter/)
- [SQLite Documentation](https://sqlite.org/docs.html)
- [Keep a Changelog](https://keepachangelog.com/)

---

## Getting Help

- **Issues**: Open a GitHub issue
- **Architecture Questions**: Check ADRs first
- **Code Questions**: Check coding-standards.md

---

**Version:** 0.8.0
**Generated:** 2026-01-30
**Source:** CLAUDE.md, coding-standards.md, architecture-constraints.md
