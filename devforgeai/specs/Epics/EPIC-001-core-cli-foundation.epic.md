---
id: EPIC-001
title: Core CLI Foundation - AST-Aware Symbol Search
status: Planning
start_date: 2026-01-27
target_date: 2026-02-03
total_points: 23
completed_points: 0
created: 2026-01-27
owner: Bryan
tech_lead: Bryan
team: Treelint
complexity_score: 23
complexity_tier: 2
brainstorm_ref: BRAINSTORM-001
research_ref: RESEARCH-001
requirements_ref: devforgeai/specs/requirements/treelint-requirements.md
context_files:
  - devforgeai/specs/context/tech-stack.md
  - devforgeai/specs/context/source-tree.md
  - devforgeai/specs/context/dependencies.md
  - devforgeai/specs/context/coding-standards.md
  - devforgeai/specs/context/architecture-constraints.md
  - devforgeai/specs/context/anti-patterns.md
---

# Epic: Core CLI Foundation - AST-Aware Symbol Search

## Business Goal

Deliver a working Rust CLI that uses tree-sitter AST parsing to search codebases for semantic symbols (functions, classes, methods), reducing AI coding assistant token consumption by 40-80% compared to text-based search (ripgrep/grep).

**Problem Solved:** AI coding assistants waste 40-83% of context window on false positives from text-based code search. Comments, strings, and variable names match keywords indiscriminately, consuming tokens without adding value.

**Value Proposition:**
- 40-80% token reduction for AI coding assistants
- Semantic precision (only actual code constructs, not text matches)
- Structured JSON output optimized for AI consumption
- Cross-platform single binary distribution

## Success Metrics

- **Token Reduction:** ≥ 40% reduction compared to ripgrep output size
- **Query Latency:** < 50ms for indexed repositories
- **Accuracy (Precision):** ≥ 95% - search results are actual symbols, not false positives
- **Binary Size:** < 50MB with embedded grammars
- **Platform Coverage:** 3/3 (Windows, macOS, Linux)

**Measurement Plan:**
- Token reduction: A/B comparison of Treelint vs ripgrep output for same queries
- Query latency: Benchmark on 10K file codebase
- Accuracy: Manual review of 100 random search results
- Binary size: Build artifact measurement
- Platform: CI matrix validation

## Scope

### In Scope

1. **Feature F1: Symbol Search**
   - tree-sitter parsing with embedded Python, TypeScript, Rust, Markdown grammars
   - Symbol extraction: functions, classes, methods, variables, constants, imports, exports
   - SQLite index storage with WAL mode
   - Search command with exact/case-insensitive/regex modifiers
   - File discovery with language detection

2. **Feature F2: JSON/Text Output**
   - JSON output with query metadata and results
   - Human-readable text output for terminal
   - TTY auto-detection (text for terminal, JSON when piped)
   - `--format` flag override
   - Signature-only output mode

3. **Feature F3: Context Modes**
   - `--context N` for N lines before/after
   - `--context full` for complete semantic unit (tree-sitter node boundaries)
   - `--signatures` for minimal output

### Out of Scope

- ❌ Background daemon (EPIC-002)
- ❌ File watching (EPIC-002)
- ❌ Repository map generation (EPIC-002)
- ❌ Dependency graph (EPIC-002)
- ❌ IDE/Editor integration
- ❌ GUI interface
- ❌ Languages beyond Python, TypeScript, Rust, Markdown

---

## Technical Specification

> **CRITICAL FOR STORY CREATION:** This section contains exact specifications required for acceptance criteria. Stories MUST reference these specs.

### CLI Interface Specification

```bash
# Primary command: symbol search
treelint search <SYMBOL> [OPTIONS]

Options:
  --type <TYPE>          Filter by symbol type (function|class|method|variable|constant|import|export)
  -i, --ignore-case      Case-insensitive matching
  -r, --regex            Treat SYMBOL as regex pattern
  --format <FORMAT>      Output format: json|text (default: auto-detect TTY)
  --context <CONTEXT>    Context mode: N (lines) | full (semantic unit)
  --signatures           Output signatures only (minimal tokens)

# Examples:
treelint search validateUser                      # Exact match, auto-format
treelint search validateUser --type function      # Filter to functions only
treelint search 'validate.*' -r                   # Regex pattern
treelint search validateuser -i                   # Case-insensitive
treelint search validateUser --context 5          # 5 lines before/after
treelint search validateUser --context full       # Complete function body
treelint search validateUser --signatures         # Signature only
treelint search validateUser --format json        # Force JSON output
treelint search validateUser -i -r --context full # Combined flags
```

### JSON Output Schema

```json
{
  "query": {
    "symbol": "validateUser",
    "type": "function",           // null if not filtered
    "case_insensitive": false,
    "regex": false,
    "context_mode": "full"        // "lines:5" | "full" | "signatures"
  },
  "results": [
    {
      "type": "function",
      "name": "validateUser",
      "file": "src/auth/validator.py",
      "lines": [10, 45],
      "signature": "def validateUser(email: str, password: str) -> bool",
      "body": "def validateUser(email: str, password: str) -> bool:\n    ...",
      "language": "python"
    }
  ],
  "metadata": {
    "files_searched": 150,
    "files_skipped": 12,
    "skipped_by_type": {
      ".json": 5,
      ".yaml": 3,
      ".toml": 2,
      ".env": 2
    },
    "languages_searched": ["python", "typescript", "rust", "markdown"],
    "elapsed_ms": 47
  }
}
```

### Text Output Format

```
validateUser (function) in src/auth/validator.py:10-45
├── Signature: def validateUser(email: str, password: str) -> bool
└── Body:
    def validateUser(email: str, password: str) -> bool:
        """Validate user credentials."""
        if not email:
            raise ValueError("Email required")
        ...

Found 1 result in 47ms (150 files searched, 12 skipped)
```

### SQLite Schema

```sql
-- Location: .treelint/index.db

CREATE TABLE symbols (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    type TEXT NOT NULL,          -- function|class|method|variable|constant|import|export
    visibility TEXT,             -- public|private|protected (nullable)
    file_path TEXT NOT NULL,
    line_start INTEGER NOT NULL,
    line_end INTEGER NOT NULL,
    signature TEXT NOT NULL,
    body TEXT,                   -- nullable, populated on demand
    language TEXT NOT NULL,
    updated_at INTEGER NOT NULL  -- Unix timestamp
);

CREATE INDEX idx_symbols_name ON symbols(name);
CREATE INDEX idx_symbols_type ON symbols(type);
CREATE INDEX idx_symbols_file ON symbols(file_path);

CREATE TABLE files (
    path TEXT PRIMARY KEY,
    language TEXT NOT NULL,
    hash TEXT NOT NULL,          -- SHA-256 for change detection
    indexed_at INTEGER NOT NULL  -- Unix timestamp
);

CREATE TABLE metadata (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- Required connection settings (WAL mode for concurrent access)
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA foreign_keys = ON;
```

### Symbol Types

| Type | Description | Languages |
|------|-------------|-----------|
| `function` | Standalone function definitions | Python, TypeScript, Rust |
| `class` | Class definitions | Python, TypeScript |
| `method` | Methods inside classes | Python, TypeScript, Rust |
| `variable` | Variable/constant declarations | Python, TypeScript, Rust |
| `constant` | Constant declarations (const, UPPER_CASE) | Python, TypeScript, Rust |
| `import` | Import statements | Python, TypeScript, Rust |
| `export` | Export statements | TypeScript |

### File Path Mappings (per source-tree.md)

| Component | File Path | Responsibility |
|-----------|-----------|----------------|
| CLI entry point | `src/main.rs` | Binary entry, clap setup |
| CLI arguments | `src/cli/args.rs` | Argument definitions |
| Search command | `src/cli/commands/search.rs` | Search orchestration |
| Language detection | `src/parser/languages.rs` | File → Language mapping |
| Symbol extraction | `src/parser/symbols.rs` | tree-sitter queries |
| Python queries | `src/parser/queries/python.rs` | Python-specific queries |
| TypeScript queries | `src/parser/queries/typescript.rs` | TypeScript-specific queries |
| Rust queries | `src/parser/queries/rust.rs` | Rust-specific queries |
| Markdown queries | `src/parser/queries/markdown.rs` | Markdown-specific queries |
| Context extraction | `src/parser/context.rs` | Lines/full/signatures |
| SQLite schema | `src/index/schema.rs` | Table definitions |
| SQLite storage | `src/index/storage.rs` | CRUD operations |
| Search queries | `src/index/search.rs` | Symbol search logic |
| JSON output | `src/output/json.rs` | JSON formatter |
| Text output | `src/output/text.rs` | Human-readable formatter |
| Error types | `src/error.rs` | thiserror definitions |

### Behavior Specifications

**Unsupported File Handling:**
- Skip silently (no stderr warnings by default)
- Include skip summary in JSON `metadata.files_skipped` and `metadata.skipped_by_type`
- `--verbose` flag adds per-file skip warnings to stderr

**TTY Auto-Detection:**
- If stdout is TTY (terminal): Output text format
- If stdout is pipe/redirect: Output JSON format
- `--format` flag overrides auto-detection

**Error Handling:**
- Return structured JSON errors: `{"error": {"code": "E001", "message": "..."}}`
- Exit codes: 0 (success), 1 (error), 2 (no results)
- Errors go to stderr, results go to stdout

**Index Behavior (EPIC-001 scope):**
- On-demand indexing only (no daemon)
- Build index on first search if not exists
- Store in `.treelint/index.db` in working directory

### Crate Dependencies (per dependencies.md)

| Feature | Crate | Version |
|---------|-------|---------|
| CLI | clap | 4.5 |
| Python parsing | tree-sitter-python | 0.21 |
| TypeScript parsing | tree-sitter-typescript | 0.21 |
| Rust parsing | tree-sitter-rust | 0.21 |
| Markdown parsing | tree-sitter-md | 0.2 |
| Database | rusqlite (bundled) | 0.31 |
| JSON | serde_json | 1.0 |
| Regex | regex | 1.10 |
| TTY detection | atty | 0.2 |
| Errors | thiserror, anyhow | 1.0 |

---

## Target Sprints

### Sprint 1 (SPRINT-001): Core CLI Foundation
**Goal:** Deliver working symbol search with JSON/text output
**Estimated Points:** 23
**Duration:** Week 1 (5 days)

**Features:**
- F1: Symbol Search (13 points)
  - tree-sitter integration with embedded grammars
  - SQLite schema and index storage
  - Symbol extraction for all types
  - Search command with modifiers
  - File discovery and language detection
- F2: JSON/Text Output (5 points)
  - JSON formatter with metadata
  - Text formatter for humans
  - TTY auto-detection
- F3: Context Modes (5 points)
  - Line-based context
  - Full semantic unit extraction
  - Signatures-only mode

**Key Deliverables:**
- `treelint search <symbol>` command works
- JSON output consumable by AI assistants
- < 50ms query latency on indexed repos
- Cross-platform binary

## User Stories

### Created Stories

| Story ID | Title | Points | Status | Feature |
|----------|-------|--------|--------|---------|
| [STORY-001](../Stories/STORY-001-project-setup-cli-skeleton.story.md) | Project Setup + CLI Skeleton | 3 | Backlog | F1.1 |
| [STORY-002](../Stories/STORY-002-treesitter-parser-integration.story.md) | tree-sitter Parser Integration | 5 | Backlog | F1.2 |
| [STORY-003](../Stories/STORY-003-sqlite-index-storage.story.md) | SQLite Index Storage | 3 | Backlog | F1.3 |
| [STORY-004](../Stories/STORY-004-search-command-logic.story.md) | Search Command Logic | 2 | Backlog | F1.4 |
| [STORY-005](../Stories/STORY-005-json-text-output.story.md) | JSON/Text Output | 5 | Backlog | F2 |
| [STORY-006](../Stories/STORY-006-context-modes.story.md) | Context Modes for Symbol Search Output Control | 5 | Backlog | F3 |

### Planned Stories (Not Yet Created)

*All features have stories created.*

### High-Level User Stories

1. **As an** AI coding assistant, **I want** to search for function definitions by name, **so that** I get precise code context without false positives
2. **As an** AI coding assistant, **I want** JSON output with symbol metadata, **so that** I can programmatically select relevant context
3. **As a** developer, **I want** human-readable terminal output, **so that** I can use Treelint for manual code exploration
4. **As an** AI coding assistant, **I want** to control context size, **so that** I can balance token usage vs code completeness

## Technical Considerations

### Architecture Impact
- **New Components:**
  - CLI parser (clap crate)
  - tree-sitter parser wrapper
  - SQLite index manager
  - Output formatters (JSON, Text)
- **Pattern:** Modular monolith with clear module boundaries

### Technology Decisions
- **Language:** Rust (single binary, cross-platform, high performance)
- **Parser:** tree-sitter with embedded grammars (tree-sitter-languages crate)
- **Database:** SQLite (rusqlite with bundled feature)
- **CLI:** clap crate
- **Regex:** regex crate
- **TTY Detection:** atty crate

### Security & Compliance
- No network access (offline-only tool)
- No credential storage
- Read-only file access
- No compliance requirements

### Performance Requirements
- Query latency: < 50ms (p95)
- Index build: < 5 minutes for 100K files
- Binary size: < 50MB
- Memory: Chunked processing to limit memory usage

## Dependencies

### Internal Dependencies
- None (greenfield project, first epic)

### External Dependencies
- [ ] **tree-sitter grammars:** Python, TypeScript, Rust, Markdown
  - **Owner:** tree-sitter community
  - **Status:** Available (published crates)
- [ ] **Rust toolchain:** 1.70+ stable
  - **Owner:** Rust project
  - **Status:** Available

## Risks & Mitigation

### Risk 1: Tree-sitter Embedding Fails
- **Probability:** Low
- **Impact:** High
- **Mitigation:** Use tree-sitter-languages crate (proven approach); test early
- **Contingency:** Fall back to external grammar files

### Risk 2: Binary Size > 50MB
- **Probability:** Medium
- **Impact:** Medium
- **Mitigation:** Embed only 4 grammars; use LTO and build optimization
- **Contingency:** Accept larger binary or lazy-load grammars

### Risk 3: Query Latency > 50ms
- **Probability:** Low
- **Impact:** Medium
- **Mitigation:** SQLite indexes; benchmark early; optimize hot paths
- **Contingency:** Accept slightly higher latency if needed

## Stakeholders

### Primary Stakeholders
- **Product Owner:** Bryan - Requirements, prioritization
- **Tech Lead:** Bryan - Architecture, implementation
- **User Representative:** AI coding assistants (Claude, GPT)

### Additional Stakeholders
- **Open Source Community:** Future contributors
- **Tree-sitter Community:** Grammar maintainers

## Communication Plan

### Status Updates
- **Frequency:** Daily (during 2-week sprint)
- **Format:** Commit messages, story updates
- **Audience:** Self (solo project)

### Milestones
- Day 2: tree-sitter integration working
- Day 3: SQLite index functional
- Day 4: Search command complete
- Day 5: Output formatting and context modes

## Timeline

```
Epic Timeline:
════════════════════════════════════════════════════
Day 1-2:  F1 - tree-sitter + SQLite foundation
Day 3:    F2 - JSON/Text output formatting
Day 4:    F3 - Context modes
Day 5:    Integration testing, CLI polish
════════════════════════════════════════════════════
Total Duration: 5 days (1 week)
Target Release: v0.1.0
```

### Key Milestones
- [ ] **Day 2:** tree-sitter parsing Python/TypeScript/Rust/Markdown
- [ ] **Day 3:** SQLite index storage working
- [ ] **Day 4:** Search command with all modifiers
- [ ] **Day 5:** v0.1.0 release (Core CLI)

## Progress Tracking

### Sprint Summary

| Sprint | Status | Points | Stories | Completed | In Progress | Blocked |
|--------|--------|--------|---------|-----------|-------------|---------|
| SPRINT-001 | Not Started | 23 | 5 | 0 | 0 | 0 |
| **Total** | **0%** | **23** | **5** | **0** | **0** | **0** |

### Burndown
- **Total Points:** 23
- **Completed:** 0
- **Remaining:** 23
- **Velocity:** TBD after first sprint

## Retrospective (Post-Epic)

*To be completed after epic completes*

---

**Epic Template Version:** 1.0
**Last Updated:** 2026-01-27
