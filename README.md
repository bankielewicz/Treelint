# Treelint

**AST-aware code search CLI that reduces AI coding assistant token consumption by 40-80%**

Treelint uses tree-sitter to provide semantic code navigation, returning functions and classes instead of raw line matches. Purpose-built for AI assistants like Claude Code.

## Why Treelint?

Text-based search tools (grep, ripgrep) return false positives from comments, strings, and variable names. This wastes 40-83% of an AI's context window on irrelevant results.

| Tool | Context Utilization | Approach |
|------|---------------------|----------|
| ripgrep (current) | 20-30% | Text search |
| Treelint | 5-10% | AST-aware search |
| **Savings** | **40-80%** | Semantic filtering |

## Current Status

**v0.4.0** - Search Command (Released 2026-01-27)

| Feature | Status |
|---------|--------|
| CLI argument parsing | ✅ Complete |
| JSON/text output | ✅ Complete |
| Symbol type filtering | ✅ Complete |
| AST parsing | ✅ Complete |
| SQLite index storage | ✅ Complete |
| Incremental re-indexing | ✅ Complete |
| **Search command** | ✅ Complete |
| **Auto-indexing** | ✅ Complete |
| **Regex search** | ✅ Complete |
| **Case-insensitive search** | ✅ Complete |
| Background indexing daemon | Coming soon |
| Repository mapping | Coming soon |

**Build Stats:**
- Tests: 262 passing (50 for search module)
- Binary size: 7.5 MB
- Query latency: <50ms (p95)

## Features

### Symbol-Based Search

- **Function search** - Find functions by name across all supported languages
- **Class search** - Locate class definitions
- **Method search** - Find methods within classes
- **JSON output** - Structured data optimized for AI consumption
- **Configurable context** - Return N lines around symbols

### Parser Module

The parser module provides deep AST analysis using tree-sitter with embedded grammars:

**Language Detection:**
- Automatic detection from file extensions
- Supported: `.py`, `.ts`, `.tsx`, `.js`, `.jsx`, `.rs`, `.md`, `.markdown`

**Symbol Extraction:**
| Symbol Type | Python | TypeScript | Rust | Markdown |
|-------------|--------|------------|------|----------|
| Functions | Yes | Yes | Yes | - |
| Classes | Yes | Yes | Yes | - |
| Methods | Yes | Yes | Yes | - |
| Variables | Yes | Yes | Yes | - |
| Constants | Yes | Yes | Yes | - |
| Imports | Yes | Yes | Yes | - |
| Exports | - | Yes | Yes | - |
| Headings | - | - | - | Yes |

**Error Handling:**
- Graceful degradation for malformed files
- Partial results returned when possible
- Clear error messages for unsupported file types

**Performance:**
- Grammar initialization: < 10ms
- Parsing 10K lines: < 50ms
- Memory-safe Rust implementation

### Index Storage Module

The index module provides persistent symbol storage using SQLite with WAL mode:

**Storage Features:**
- **SQLite with WAL mode** - Concurrent reads, crash recovery
- **Bulk operations** - Insert 1000 symbols in <500ms
- **Upsert support** - Re-index files without duplicates
- **File tracking** - SHA-256 hash-based change detection

**Query Operations:**
| Query Type | Description |
|------------|-------------|
| `query_by_name()` | Exact symbol name match |
| `query_by_name_case_insensitive()` | Case-insensitive search |
| `query_by_type()` | Filter by symbol type |
| `query_by_file()` | All symbols in a file |
| `query()` | Combined filters |

**Error Handling:**
- `StorageError::DatabaseCorrupted` - Integrity check failed
- `StorageError::PermissionDenied` - File access denied
- `StorageError::ConnectionFailed` - Database connection error
- `StorageError::QueryFailed` - SQL execution error

**Performance:**
- Query latency: <50ms (p95) with 100K symbols
- Bulk insert: <500ms for 1000 symbols
- Database location: `.treelint/index.db`

### Coming Soon

- **Background indexing daemon** - Auto-index on file changes
- **Repository map** - Generate symbol summaries (Aider-style)
- **Dependency graph** - Track function call relationships

## Installation

```bash
# From crates.io (coming soon)
cargo install treelint

# From source
git clone https://github.com/yourusername/treelint
cd treelint
cargo build --release
```

## Usage

### Search Command

```bash
# Basic symbol search
treelint search validateUser

# Search with type filter (function, class, method)
treelint search validateUser --type function

# Case-insensitive search
treelint search validateuser -i

# Regex pattern search
treelint search 'validate.*' -r

# Combined filters (AND logic)
treelint search 'validate.*' -r -i --type function

# JSON output (default)
treelint search validateUser --format json

# Text output
treelint search validateUser --format text
```

### Auto-Indexing

Treelint automatically builds an index on first search:

```bash
# First search creates .treelint/index.db automatically
treelint search main

# Progress shown on stderr during indexing
# Subsequent searches use existing index (instant)
```

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success with results |
| 1 | Error (invalid regex, I/O error) |
| 2 | Success but no matching results |

### Legacy Syntax (Coming Soon)

```bash
# Alternative search syntax
treelint --function validateUser
treelint --class AuthService

# Generate repository map
treelint map --output repo-map.json
```

## Output Format

```json
{
  "query": {
    "symbol": "validateUser",
    "type": "function",
    "case_insensitive": true,
    "regex": false
  },
  "results": [
    {
      "type": "function",
      "name": "validateUser",
      "file": "src/auth/validator.py",
      "lines": [10, 45],
      "signature": "def validateUser(email: str, password: str) -> bool",
      "body": "...",
      "language": "python"
    }
  ],
  "stats": {
    "files_searched": 150,
    "elapsed_ms": 36
  }
}
```

## Supported Languages

| Language | Status | File Extensions |
|----------|--------|-----------------|
| Python | Complete | `.py` |
| TypeScript | Complete | `.ts`, `.tsx` |
| JavaScript | Complete | `.js`, `.jsx` |
| Rust | Complete | `.rs` |
| Markdown | Complete | `.md`, `.markdown` |

## Architecture

```
+----------------------------------------------------------+
|                      Treelint CLI                         |
+----------------------------------------------------------+
|  +-------------+  +-------------+  +-----------------+   |
|  |   Parser    |  |   Index     |  |     Output      |   |
|  | (tree-sitter)|  |  (daemon)   |  |   (JSON/Text)   |   |
|  +------+------+  +------+------+  +--------+--------+   |
|         |                |                   |            |
|  +------v----------------v-------------------v--------+   |
|  |                  Symbol Store                      |   |
|  |  (functions, classes, methods, variables)          |   |
|  +----------------------------------------------------+   |
+----------------------------------------------------------+
|  Embedded Grammars: Python, TypeScript, Rust, Markdown   |
+----------------------------------------------------------+
```

**Core Components:**

- **Rust** - Single binary, no runtime dependencies
- **tree-sitter 0.22** - Embedded grammars for all supported languages (no external grammar files)
- **SQLite (rusqlite)** - Bundled database with WAL mode for concurrent access
- **Parser Module** - Language detection, symbol extraction, error handling
- **Index Module** - Symbol storage, queries, incremental re-indexing
- **Cross-platform** - Windows, macOS, Linux

## Development

```bash
# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run -- --function main

# Build release
cargo build --release

# Run clippy
cargo clippy
```

## Research

This project is informed by research into AI coding assistant token efficiency:
- [RESEARCH-005: Claude Code Search Token Efficiency](devforgeai/specs/research/RESEARCH-001-claude-code-search-token-efficiency.research.md)

## Documentation

- [Getting Started Guide](docs/guides/getting-started.md)
- [CLI Reference](docs/api/cli-reference.md)
- [Library API Reference](docs/api/library-reference.md)
- [Contributing Guidelines](CONTRIBUTING.md)
- [Changelog](CHANGELOG.md)

## License

MIT

## Contributing

Contributions welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) first.
