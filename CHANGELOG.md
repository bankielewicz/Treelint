# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2026-01-27

### Added

- Search command with full query capabilities ([STORY-004])
  - Exact match, type filtering, case-insensitive, regex patterns
  - Auto-indexing on first search when index missing
  - Query latency < 50ms, binary size 7.5 MB

### Technical

- 50 new tests for search command (262 total)
- Binary size: 7.5 MB with all embedded grammars

## [0.3.0] - 2026-01-27

### Added

- SQLite index storage with WAL mode for concurrent access ([STORY-003])
- Symbol CRUD operations with bulk insert and upsert support
- File tracking for incremental re-indexing (hash-based change detection)
- Query operations: by name, type, file, with combined filters
- StorageError enum with comprehensive error handling
- tree-sitter parser integration with embedded grammars for Python, TypeScript, Rust, Markdown ([STORY-002])
- Language detection from file extensions (.py, .ts, .tsx, .js, .jsx, .rs, .md, .markdown)
- Symbol extraction for functions, classes, methods, variables, constants, imports, exports
- Graceful error handling for malformed source files (partial results, no panics)

### Documentation

- Library API Reference (docs/api/library-reference.md) - Complete Rust API docs
- Updated CLI Reference for v0.2.0 with supported languages section

### Technical

- tree-sitter 0.22 with embedded grammars (no external grammar files)
- 131 tests (123 unit + 8 doc-tests) covering all 6 acceptance criteria
- Binary size: 723KB (well under 50MB target)

## [0.1.0] - 2026-01-27

### Added

- CLI skeleton with `search` command ([STORY-001])
- Argument parsing with clap (symbol, --type, -i, -r, --format, --context, --signatures)
- JSON and text output formats
- Help text with usage examples
- Error types with thiserror (Io, Parse, Cli variants)
- Placeholder search results (actual AST parsing in future stories)

### Technical

- Rust 2021 edition with MSRV 1.70.0
- Dependencies: clap 4.5, serde 1.0, serde_json 1.0, thiserror 1.0, anyhow 1.0
- Release build with LTO optimization (740KB binary)
- 81 integration tests covering all acceptance criteria

[unreleased]: https://github.com/treelint/treelint/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/treelint/treelint/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/treelint/treelint/compare/v0.1.0...v0.3.0
[0.1.0]: https://github.com/treelint/treelint/releases/tag/v0.1.0
[STORY-001]: devforgeai/specs/Stories/STORY-001-project-setup-cli-skeleton.story.md
[STORY-002]: devforgeai/specs/Stories/archive/STORY-002-treesitter-parser-integration.story.md
[STORY-003]: devforgeai/specs/Stories/archive/STORY-003-sqlite-index-storage.story.md
[STORY-004]: devforgeai/specs/Stories/archive/STORY-004-search-command-logic.story.md
