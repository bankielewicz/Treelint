# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Present multiple interpretations — Don't pick silently when ambiguity exists.  Use AskUserQuestion tool to ask me questions.

## Build & Development Commands

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Release build (LTO enabled)

# Test
cargo test                     # Run all tests
cargo test --test story_001    # Run tests for a specific story
cargo test test_search_exact   # Run tests matching name pattern
cargo test -- --nocapture      # Show println! output
cargo test -- --test-threads=1 # Single-threaded (for debugging)

# Lint & Format
cargo clippy -- -D warnings    # Lint (fail on warnings)
cargo fmt --check              # Check formatting
cargo fmt                      # Apply formatting

# Pre-commit check (run all quality gates)
cargo test && cargo clippy -- -D warnings && cargo fmt --check
```

## Architecture

Treelint is an AST-aware code search CLI that uses tree-sitter to return semantic code units (functions, classes) instead of raw text matches. It reduces AI coding assistant token consumption by 40-80%.

### Module Structure

```
src/
├── main.rs          # Entry point, delegates to cli module
├── lib.rs           # Public API exports
├── error.rs         # TreelintError enum (thiserror)
├── cli/
│   ├── args.rs      # Clap argument definitions (SearchArgs, SymbolType, OutputFormat)
│   └── commands/
│       └── search.rs # Search command implementation
├── parser/
│   ├── languages.rs # Language enum, file extension detection
│   ├── symbols.rs   # Symbol struct, SymbolExtractor trait
│   └── queries/     # Tree-sitter query definitions per language
├── index/
│   ├── schema.rs    # SQLite schema version management
│   ├── storage.rs   # IndexStorage CRUD operations
│   └── search.rs    # Query filters and execution
└── output/
    └── json.rs      # JSON serialization
```

### Data Flow

1. CLI parses args → `SearchArgs`
2. Check/build index → `IndexStorage` (SQLite in `.treelint/index.db`)
3. Parse source files → `SymbolExtractor` extracts `Symbol` structs
4. Query index → Filter by name/type/regex
5. Format output → JSON or text to stdout

### Key Types

- `Symbol` - Extracted code unit (name, type, file, lines, signature, body)
- `SymbolType` - Function, Class, Method, Variable, Constant, Import, Export
- `Language` - Python, TypeScript, Rust, Markdown
- `IndexStorage` - SQLite wrapper for symbol persistence
- `TreelintError` - Error enum covering parse, storage, and CLI errors

## Test Organization

Tests are organized by story and acceptance criteria:

```
tests/
├── story_001.rs              # Entry point for STORY-001 tests
├── STORY-001/
│   ├── test_ac1_cargo_build.rs
│   ├── test_ac2_argument_parsing.rs
│   └── ...
├── story_002_tests/          # Alternative organization
│   └── test_ac1_grammar_loading.rs
└── fixtures/                 # Test data files
    ├── rust/
    ├── python/
    └── typescript/
```

Test patterns use `assert_cmd` for CLI testing and `tempfile` for isolated test directories.

## DevForgeAI Framework

This project uses DevForgeAI for spec-driven development with TDD.

**Workflow:** `/brainstorm` → `/ideate` → `/create-context` → `/create-epic` → `/dev` → `/qa` → `/release`

**Key Commands:**
- `/dev STORY-XXX` - TDD implementation (Red → Green → Refactor)
- `/qa STORY-XXX` - Quality validation
- `/create-story` - Create user stories with acceptance criteria

**Context Files (immutable constraints):**

| File | Purpose |
|------|---------|
| `devforgeai/specs/context/tech-stack.md` | Allowed technologies |
| `devforgeai/specs/context/source-tree.md` | File/folder structure |
| `devforgeai/specs/context/dependencies.md` | Cargo dependencies |
| `devforgeai/specs/context/architecture-constraints.md` | Layer boundaries |
| `devforgeai/specs/context/anti-patterns.md` | Forbidden patterns |
| `devforgeai/specs/context/coding-standards.md` | Code style |

**Rules:**
- Check context files before making changes
- No `.unwrap()` or `.expect()` in production code
- Tests before implementation (TDD)
- Binary size must stay < 50MB
- Grammars must be embedded (no external files)
