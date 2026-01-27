---
last_updated: 2026-01-27
status: LOCKED
requires_approval: Tech Lead
project: Treelint
---

# Source Tree Structure

**CRITICAL:** This document defines the ONLY approved directory structure for this project.

**AI agents MUST NOT create files or directories outside this structure without explicit approval.**

---

## Project Root

```
treelint/
├── .github/                    # GitHub Actions workflows
│   └── workflows/
│       ├── ci.yml              # Continuous integration
│       ├── release.yml         # Release automation
│       └── cross-platform.yml  # Cross-platform builds
│
├── .treelint/                  # Runtime data directory (gitignored)
│   └── index.db                # SQLite index (created at runtime)
│
├── devforgeai/                 # DevForgeAI framework artifacts
│   ├── specs/
│   │   ├── context/            # This directory - immutable constraints
│   │   ├── Epics/              # Epic documents
│   │   ├── Stories/            # User story documents
│   │   ├── adrs/               # Architecture Decision Records
│   │   ├── brainstorms/        # Brainstorm sessions
│   │   ├── research/           # Research documents
│   │   └── requirements/       # Requirements specifications
│   ├── feedback/               # Workflow feedback
│   └── RCA/                    # Root Cause Analysis documents
│
├── src/                        # Rust source code
│   ├── main.rs                 # CLI entry point
│   ├── lib.rs                  # Library root (optional)
│   ├── cli/                    # CLI argument parsing
│   │   ├── mod.rs
│   │   ├── args.rs             # Argument definitions (clap)
│   │   └── commands/           # Command implementations
│   │       ├── mod.rs
│   │       ├── search.rs       # treelint search
│   │       ├── index.rs        # treelint index
│   │       ├── daemon.rs       # treelint daemon
│   │       ├── map.rs          # treelint map
│   │       └── deps.rs         # treelint deps
│   │
│   ├── parser/                 # tree-sitter parsing logic
│   │   ├── mod.rs
│   │   ├── languages.rs        # Language detection & grammar loading
│   │   ├── symbols.rs          # Symbol extraction
│   │   ├── queries/            # tree-sitter query definitions
│   │   │   ├── mod.rs
│   │   │   ├── python.rs
│   │   │   ├── typescript.rs
│   │   │   ├── rust.rs
│   │   │   └── markdown.rs
│   │   └── context.rs          # Context extraction (lines, full, signatures)
│   │
│   ├── index/                  # SQLite indexing service
│   │   ├── mod.rs
│   │   ├── schema.rs           # Database schema
│   │   ├── storage.rs          # SQLite operations
│   │   ├── search.rs           # Query execution
│   │   └── relevance.rs        # PageRank-style scoring
│   │
│   ├── daemon/                 # Background daemon service
│   │   ├── mod.rs
│   │   ├── server.rs           # IPC server
│   │   ├── watcher.rs          # File watcher (notify)
│   │   └── protocol.rs         # IPC message protocol
│   │
│   ├── output/                 # Output formatting
│   │   ├── mod.rs
│   │   ├── json.rs             # JSON formatter
│   │   ├── text.rs             # Human-readable formatter
│   │   └── graph.rs            # Mermaid/graph output
│   │
│   ├── graph/                  # Dependency graph analysis
│   │   ├── mod.rs
│   │   ├── calls.rs            # Call graph extraction
│   │   └── imports.rs          # Import graph extraction
│   │
│   └── error.rs                # Error types (thiserror)
│
├── tests/                      # Integration tests
│   ├── common/                 # Test utilities
│   │   └── mod.rs
│   ├── fixtures/               # Test fixtures (sample codebases)
│   │   ├── python/
│   │   ├── typescript/
│   │   ├── rust/
│   │   └── markdown/
│   ├── search_tests.rs         # Search command tests
│   ├── index_tests.rs          # Index command tests
│   ├── output_tests.rs         # Output format tests
│   └── daemon_tests.rs         # Daemon tests
│
├── benches/                    # Performance benchmarks
│   ├── search_bench.rs
│   └── index_bench.rs
│
├── docs/                       # Documentation
│   ├── architecture/           # Architecture docs
│   │   └── decisions/          # ADR exports (optional)
│   ├── guides/                 # User guides
│   └── api/                    # API documentation
│
├── Cargo.toml                  # Rust dependencies
├── Cargo.lock                  # Locked dependencies
├── build.rs                    # Build script (grammar compilation)
├── CLAUDE.md                   # AI agent instructions
├── README.md                   # Project readme
├── LICENSE                     # License file
└── .gitignore                  # Git ignore rules
```

---

## Directory Rules

### Source Code (`src/`)

| Directory | Purpose | Rules |
|-----------|---------|-------|
| `src/cli/` | CLI argument parsing and command routing | clap derive macros, thin layer |
| `src/cli/commands/` | Command implementations | One file per command |
| `src/parser/` | tree-sitter parsing logic | Language-agnostic interface |
| `src/parser/queries/` | tree-sitter query definitions | One file per language |
| `src/index/` | SQLite storage and search | rusqlite operations |
| `src/daemon/` | Background service | IPC, file watching |
| `src/output/` | Output formatting | JSON, text, graph |
| `src/graph/` | Dependency analysis | Call/import graphs |

### Tests (`tests/`)

| Directory | Purpose | Rules |
|-----------|---------|-------|
| `tests/common/` | Shared test utilities | Helpers, setup/teardown |
| `tests/fixtures/` | Sample codebases for testing | Realistic test data |
| `tests/*.rs` | Integration test files | One file per feature area |

### DevForgeAI (`devforgeai/`)

| Directory | Purpose | Rules |
|-----------|---------|-------|
| `devforgeai/specs/context/` | Immutable constraint files | LOCKED, requires ADR to change |
| `devforgeai/specs/Epics/` | Epic documents | EPIC-NNN-name.epic.md |
| `devforgeai/specs/Stories/` | User stories | STORY-NNN-name.story.md |
| `devforgeai/specs/adrs/` | Architecture decisions | ADR-NNN-title.md |

---

## File Naming Conventions

### Rust Files

```
✅ CORRECT:
  src/parser/symbols.rs          # Lowercase, underscores
  src/cli/commands/search.rs     # Descriptive name
  tests/search_tests.rs          # _tests suffix

❌ FORBIDDEN:
  src/Parser/Symbols.rs          # No PascalCase directories
  src/cli/commands/Search.rs     # No PascalCase files
  src/symbolExtractor.rs         # No camelCase
```

### Test Fixtures

```
✅ CORRECT:
  tests/fixtures/python/simple_module.py
  tests/fixtures/typescript/component.tsx
  tests/fixtures/rust/lib_example.rs

❌ FORBIDDEN:
  tests/fixtures/Python/              # No PascalCase
  tests/testdata/                     # Use fixtures/
```

### DevForgeAI Files

```
✅ CORRECT:
  devforgeai/specs/Epics/EPIC-001-core-cli.epic.md
  devforgeai/specs/Stories/STORY-001-symbol-search.story.md
  devforgeai/specs/adrs/ADR-001-database-selection.md

❌ FORBIDDEN:
  devforgeai/specs/epics/epic-001.md    # Wrong case, missing suffix
  devforgeai/specs/stories/story1.md    # Wrong naming
```

---

## Module Organization Rules

### Rule 1: One Responsibility Per Module

```
✅ CORRECT:
  src/parser/symbols.rs      # Only symbol extraction
  src/parser/languages.rs    # Only language detection
  src/index/search.rs        # Only search queries

❌ FORBIDDEN:
  src/parser/everything.rs   # God module
  src/utils.rs               # Generic "utils" (be specific)
```

### Rule 2: Public API in mod.rs

```
✅ CORRECT:
  // src/parser/mod.rs
  pub mod symbols;
  pub mod languages;
  pub use symbols::Symbol;
  pub use languages::Language;

❌ FORBIDDEN:
  // Exposing internals directly
  pub mod internal_helpers;
```

### Rule 3: Tests Next to Code OR in tests/

```
✅ CORRECT:
  // Unit tests at bottom of src/parser/symbols.rs
  #[cfg(test)]
  mod tests { ... }

  // Integration tests in tests/
  tests/search_tests.rs

❌ FORBIDDEN:
  src/parser/symbols_test.rs   # Separate unit test file
  test/parser_tests.rs         # Wrong directory name
```

---

## Prohibited Structures

### ❌ Do NOT Create

```
src/helpers/              # Be specific about what helpers
src/utils/                # Be specific about utilities
src/common/               # Only in tests/
src/shared/               # Use proper modules
src/misc/                 # Never
lib/                      # Rust uses src/
bin/                      # Use src/main.rs
app/                      # Not Rust convention
```

### ❌ Do NOT Place

```
*.rs in project root      # Rust code goes in src/
*.sql in src/             # Use embedded strings or migrations/
*.json in src/            # Config files in project root
```

---

## Creation Rules

### Before Creating Any File

1. **Check this document** for approved location
2. **If location not listed** → Use AskUserQuestion
3. **Never create directories** not in this document without approval

### AskUserQuestion Template

```
Question: "I need to create [file/directory]. Where should it go?"
Options:
  - "[Suggested location based on similar files]"
  - "[Alternative location]"
  - "Create new directory (requires source-tree.md update)"
```

---

## Related Documents

- [tech-stack.md](./tech-stack.md) - Approved technologies
- [coding-standards.md](./coding-standards.md) - Code style rules
- [architecture-constraints.md](./architecture-constraints.md) - Layer boundaries

---

## Change Log

| Date | Change | Approver | Reason | ADR |
|------|--------|----------|--------|-----|
| 2026-01-27 | Initial version | Bryan | Project setup | - |
