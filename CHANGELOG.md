# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.0] - 2026-01-29

### Added

- Background daemon with IPC communication ([STORY-007])
  - Unix domain socket (`.treelint/daemon.sock`) on Unix/macOS
  - Named pipe (`\\.\pipe\treelint-daemon`) on Windows
  - NDJSON protocol for request/response communication
  - `search` method handler (same JSON format as CLI)
  - `status` method handler (7 fields: status, indexed_files, indexed_symbols, last_index_time, uptime_seconds, pid, socket_path)
  - Graceful shutdown with 5-second timeout for active connections
  - Error codes: E001 (index not ready), E002 (invalid method), E003 (invalid params)

### Technical

- 79 new tests for daemon module (393 + 79 = 472 total)
- DaemonServer with start/stop/status lifecycle
- ProtocolHandler trait for NDJSON parsing
- Cross-platform IPC via `interprocess` crate
- Socket permissions: 0600 (user-only) on Unix
- Binary size: 7.9 MB

## [0.6.0] - 2026-01-29

### Added

- Context modes for symbol search output control ([STORY-006])
  - `--context N` for N lines before/after symbols (e.g., `--context 5`)
  - `--context full` for complete semantic unit (default behavior)
  - `--signatures` for minimal output (signature only, no body)
  - Mutual exclusivity: cannot use --context and --signatures together
  - `context_mode` field in JSON: "lines:N", "full", or "signatures"
  - Edge case handling for file boundaries

### Technical

- 76 new tests for context modes (393 total)
- ContextMode enum: Lines(usize), Full, Signatures
- extract_lines_context() with boundary clamping
- Binary size: 7.6 MB

## [0.5.0] - 2026-01-29

### Added

- JSON/Text output with TTY auto-detection ([STORY-005])
  - Auto-selects JSON when piped, text when in terminal
  - `--format json|text` flag to override auto-detection
  - `--signatures` mode omits body for 60-80% smaller output
  - `context_mode` field in JSON schema ("full" or "signatures")
  - Colored text output (cyan names, green paths, yellow line numbers)

### Technical

- 55 new tests for output module (317 total)
- Uses `atty` crate for TTY detection
- Uses `colored` crate for terminal colors

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

[unreleased]: https://github.com/treelint/treelint/compare/v0.7.0...HEAD
[0.7.0]: https://github.com/treelint/treelint/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/treelint/treelint/compare/v0.5.0...v0.6.0
[STORY-007]: devforgeai/specs/Stories/archive/STORY-007-daemon-core-ipc.story.md
[0.5.0]: https://github.com/treelint/treelint/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/treelint/treelint/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/treelint/treelint/compare/v0.1.0...v0.3.0
[0.1.0]: https://github.com/treelint/treelint/releases/tag/v0.1.0
[STORY-001]: devforgeai/specs/Stories/STORY-001-project-setup-cli-skeleton.story.md
[STORY-002]: devforgeai/specs/Stories/archive/STORY-002-treesitter-parser-integration.story.md
[STORY-003]: devforgeai/specs/Stories/archive/STORY-003-sqlite-index-storage.story.md
[STORY-004]: devforgeai/specs/Stories/archive/STORY-004-search-command-logic.story.md
[STORY-005]: devforgeai/specs/Stories/archive/STORY-005-json-text-output.story.md
[STORY-006]: devforgeai/specs/Stories/archive/STORY-006-context-modes.story.md
