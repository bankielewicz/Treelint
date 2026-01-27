# Treelint - Requirements Specification

**Version:** 1.0
**Date:** 2026-01-27
**Status:** Draft
**Author:** DevForgeAI Ideation
**Complexity Score:** 23/60 (Tier 2: Moderate Application)
**Source:** BRAINSTORM-001, RESEARCH-001

---

## 1. Project Overview

### 1.1 Project Context
| Attribute | Value |
|-----------|-------|
| **Type** | Greenfield |
| **Domain** | Developer Tools / AI Infrastructure |
| **Timeline** | < 2 weeks (MVP) |
| **Team** | 1 developer (solo) |
| **Language** | Rust |
| **Platform** | Windows, macOS, Linux |

### 1.2 Problem Statement

> AI coding assistants experience **excessive token consumption** when searching code because text-based tools (ripgrep/grep) return **false positives from comments and strings**, resulting in **40-83% wasted context window** and degraded response quality.

**Root Cause Analysis:**
1. Text-based search returns excessive false positives
2. Comments and strings contain code-like text that matches keywords
3. All false positives waste tokens + confuse reasoning
4. No purpose-built CLI optimized for AI consumption exists
5. CLI works with any AI tool via Bash/subprocess, no vendor lock-in

### 1.3 Solution Overview

**Treelint** is a Rust-based CLI that uses tree-sitter AST parsing to return semantic code units (functions, classes) instead of raw text lines. JSON output enables AI assistants to intelligently select what context to include.

**Key Differentiators:**
- AST-aware search (not text-based)
- Semantic unit extraction (function boundaries, not arbitrary lines)
- Structured JSON output (AI-consumable)
- Single binary distribution (no dependencies)
- Embedded grammars (no external files)

### 1.4 Success Criteria

| Metric | Target | Measurement |
|--------|--------|-------------|
| Token Reduction | ≥ 40% | Compare grep vs Treelint output size |
| Query Latency | < 50ms | Benchmark on 10K file codebase |
| Accuracy (Precision) | ≥ 95% | Manual review of search results |
| Binary Size | < 50MB | Build artifact measurement |
| Platform Coverage | 3/3 | CI matrix (Win/Mac/Linux) |

---

## 2. User Roles & Personas

### 2.1 Primary Users

| User Type | Description | Primary Goals |
|-----------|-------------|---------------|
| AI Coding Assistants | Claude, GPT, Copilot via Bash tool | Token efficiency, accurate code context |
| Individual Developers | Human users exploring codebases | Fast symbol search, readable output |

### 2.2 User Personas

**Persona 1: Claude Code (AI Assistant)**
- **Role:** AI coding assistant executing via Bash subprocess
- **Goals:** Minimize token consumption, get precise code context, understand codebase structure
- **Needs:** JSON output, semantic search, repository overview
- **Pain Points:** False positives from text search, truncated functions, context window limits

**Persona 2: Alex (Individual Developer)**
- **Role:** Solo developer exploring unfamiliar codebase
- **Goals:** Quickly find function definitions, understand dependencies
- **Needs:** Human-readable output, fast queries, simple CLI
- **Pain Points:** grep returns too many irrelevant matches

---

## 3. Functional Requirements

### 3.1 User Stories

#### Symbol Search (F1)
1. **As an** AI assistant, **I want** to search for function definitions by name, **so that** I get the exact function I need without false positives
2. **As a** developer, **I want** to find all classes matching a pattern, **so that** I can explore related code structures
3. **As an** AI assistant, **I want** regex search on symbol names, **so that** I can find related symbols with patterns
4. **As a** developer, **I want** case-insensitive search, **so that** I find symbols regardless of naming convention

#### Output Formatting (F2)
5. **As an** AI assistant, **I want** JSON output with metadata, **so that** I can programmatically process results
6. **As a** developer, **I want** human-readable terminal output, **so that** I can scan results quickly
7. **As an** AI assistant, **I want** auto-detected output format, **so that** I get JSON when piped automatically
8. **As a** developer, **I want** signature-only output, **so that** I can get a quick overview

#### Context Modes (F3)
9. **As an** AI assistant, **I want** to control context lines around matches, **so that** I balance token usage vs completeness
10. **As an** AI assistant, **I want** full semantic unit context, **so that** I get complete functions without cutoff
11. **As a** developer, **I want** configurable context, **so that** I can see surrounding code when needed

#### Background Indexing (F4)
12. **As a** developer, **I want** a background daemon that indexes my project, **so that** searches are instant
13. **As a** developer, **I want** file watching for automatic re-indexing, **so that** my index stays fresh
14. **As an** AI assistant, **I want** on-demand indexing fallback, **so that** search works even without daemon

#### Repository Map (F5)
15. **As an** AI assistant, **I want** a repository map of all symbols, **so that** I can understand codebase structure
16. **As an** AI assistant, **I want** relevance-ranked symbols, **so that** I prioritize important code
17. **As a** developer, **I want** symbol hierarchy by file, **so that** I can navigate the codebase

#### Dependency Graph (F6)
18. **As an** AI assistant, **I want** to see which functions call which, **so that** I understand code flow
19. **As a** developer, **I want** import/export graph, **so that** I understand module dependencies
20. **As an** AI assistant, **I want** graph output in JSON and Mermaid, **so that** I can visualize relationships

### 3.2 Features Breakdown

| Feature | Capabilities | Priority |
|---------|-------------|----------|
| **F1: Symbol Search** | tree-sitter parsing, SQLite index, search command, modifiers | P0 |
| **F2: JSON/Text Output** | JSON formatter, text formatter, TTY detection, format flags | P0 |
| **F3: Context Modes** | Line context, full unit, signatures-only | P0 |
| **F4: Background Indexing** | Daemon, file watcher, incremental updates, manual index | P1 |
| **F5: Repository Map** | Full listing, hierarchy, PageRank scoring | P1 |
| **F6: Dependency Graph** | Call graph, import graph, JSON/Mermaid output | P2 |

---

## 4. Data Requirements

### 4.1 Data Model

#### Entity: Symbol
| Field | Type | Description |
|-------|------|-------------|
| id | INTEGER | Primary key |
| name | TEXT | Symbol name (function, class, etc.) |
| type | TEXT | function, class, method, variable, constant, import, export |
| visibility | TEXT | public, private, protected |
| file_path | TEXT | Relative path to source file |
| line_start | INTEGER | Starting line number |
| line_end | INTEGER | Ending line number |
| signature | TEXT | Full signature (e.g., `def foo(x: int) -> bool`) |
| body | TEXT | Full body content (nullable, populated on demand) |
| relevance_score | REAL | PageRank-style score (default 0) |
| updated_at | INTEGER | Unix timestamp of last update |

#### Entity: File
| Field | Type | Description |
|-------|------|-------------|
| path | TEXT | Primary key, relative path |
| language | TEXT | Detected language (python, typescript, rust, markdown) |
| hash | TEXT | Content hash for change detection |
| indexed_at | INTEGER | Unix timestamp of last index |

#### Entity: Metadata
| Field | Type | Description |
|-------|------|-------------|
| key | TEXT | Primary key |
| value | TEXT | Configuration value |

### 4.2 Relationships
- Symbol → File: Many-to-one (file contains multiple symbols)
- Symbol → Symbol: Many-to-many (call graph, import graph)

### 4.3 Data Constraints
- Symbol name: Required, non-empty
- Symbol type: Enum (function, class, method, variable, constant, import, export)
- File path: Unique, valid filesystem path
- Hash: SHA-256 of file content

---

## 5. Integration Requirements

### 5.1 External Services
**None.** Treelint is a standalone offline tool with no external dependencies.

### 5.2 Consumption Interfaces

#### CLI Interface
```bash
# Symbol search
treelint search <symbol> [options]
  --type <function|class|method|...>  Filter by type
  -i, --ignore-case                   Case-insensitive
  -r, --regex                         Regex pattern
  --format <json|text>                Output format
  --context <N|full>                  Context lines or full unit
  --signatures                        Signature-only output

# Index management
treelint index [options]
  --force                             Full re-index

# Daemon management
treelint daemon <start|stop|status>

# Repository map
treelint map [options]
  --ranked                            Include relevance scores
  --format <json|text>

# Dependency graph
treelint deps [options]
  --calls                             Function call graph
  --imports                           Import/export graph
  --format <json|mermaid>
```

#### JSON Output Schema
```json
{
  "query": {
    "symbol": "validateUser",
    "type": "function",
    "context_lines": 5
  },
  "results": [{
    "type": "function",
    "name": "validateUser",
    "file": "src/auth/validator.py",
    "lines": [10, 45],
    "signature": "def validateUser(email: str, password: str) -> bool",
    "body": "...",
    "relevance_score": 0.85
  }],
  "metadata": {
    "files_searched": 150,
    "files_skipped": 12,
    "skipped_by_type": {".json": 5, ".yaml": 3},
    "languages_searched": ["python", "typescript", "rust"],
    "elapsed_ms": 47
  }
}
```

---

## 6. Non-Functional Requirements

### 6.1 Performance

| Metric | Target | Rationale |
|--------|--------|-----------|
| Query latency (indexed) | < 50ms p95 | Aggressive target for competitive advantage |
| Index build (100K files) | < 5 minutes | Acceptable first-time overhead |
| File re-index (single file) | < 1 second | Incremental updates must be fast |
| Memory (indexing) | Chunked, bounded | Avoid OOM on large repos |

### 6.2 Security
- No network access (offline-only)
- Read-only file system access (no writes except index)
- No credential storage
- Index stored in `.treelint/index.db` (user-writable)
- Socket protected by filesystem permissions (daemon)

### 6.3 Scalability
| Factor | Target |
|--------|--------|
| Repository size | 100K+ files |
| Symbol count | 1M+ symbols |
| Languages | 4 (Python, TypeScript, Rust, Markdown) |

### 6.4 Availability
- N/A (local CLI tool, no uptime SLA)
- Daemon should auto-restart on crash (optional, v2)

### 6.5 Compatibility
| Platform | Support Level |
|----------|---------------|
| Windows | Full (x86_64) |
| macOS | Full (x86_64, ARM64) |
| Linux | Full (x86_64, ARM64) |

---

## 7. Complexity Assessment

**Total Score:** 23/60
**Architecture Tier:** Tier 2 (Moderate Application)

### Score Breakdown

| Dimension | Score | Notes |
|-----------|-------|-------|
| Functional | 16/20 | 2 user types, 3 entities, 0 integrations, linear workflows |
| Technical | 10/20 | Local data, daemon concurrency, file watching |
| Team/Org | 5/10 | Solo developer |
| Non-Functional | 6/10 | Aggressive <50ms latency, no compliance |

### Architecture Recommendation
- **Pattern:** Modular Monolith (single binary with clear module boundaries)
- **Layers:** CLI → Application → Domain → Infrastructure
- **Database:** SQLite (single file, WAL mode for concurrency)
- **Deployment:** Single binary distribution

---

## 8. Feasibility Analysis

### 8.1 Technical Feasibility: ✅ FEASIBLE

| Hypothesis | Validation | Status |
|------------|------------|--------|
| tree-sitter grammars embed in Rust | tree-sitter-languages crate proven | ✅ Validated |
| JSON reduces tokens 40%+ | RESEARCH-001 data confirms | ✅ Validated |
| Claude Code parses JSON output | Standard subprocess, trivial | ✅ Validated |
| 4 languages in 2 weeks | Pre-built grammars available | ✅ Feasible |

### 8.2 Business Feasibility: ✅ FEASIBLE

| Factor | Assessment |
|--------|------------|
| Budget | $0 (open-source, no cloud) |
| Timeline | 2 weeks (achievable) |
| Market | Validated demand (RESEARCH-001) |

### 8.3 Resource Feasibility: ✅ FEASIBLE

| Factor | Assessment |
|--------|------------|
| Team | 1 FTE (sufficient for scope) |
| Skills | Rust proficiency (assumed) |
| Dependencies | All crates available |

### 8.4 Risk Register

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Binary size > 50MB | Medium | Medium | Embed only 4 grammars, use LTO |
| Timeline slip | Medium | Medium | Defer F6 (dep graph) if needed |
| Query latency > 50ms | Low | Medium | SQLite indexes, benchmark early |
| Daemon IPC complexity | Medium | Low | Simple Unix socket pattern |

### 8.5 Overall Feasibility: ✅ FEASIBLE

**Recommendation:** PROCEED

---

## 9. Constraints & Assumptions

### 9.1 Technical Constraints
| Constraint | Rationale |
|------------|-----------|
| Single binary | Easy distribution, no dependencies |
| Embedded grammars | No external files to manage |
| < 50MB binary | Reasonable download size |
| Offline-only | No network latency, privacy |

### 9.2 Business Constraints
| Constraint | Rationale |
|------------|-----------|
| < 2 weeks | Business need for DevForgeAI integration |
| Solo developer | Resource constraint |
| Open-source | Community adoption strategy |

### 9.3 Assumptions (Require Validation)
| Assumption | Validation Method |
|------------|-------------------|
| Rust proficiency available | Pre-requisite |
| tree-sitter grammars stable | Test early in Sprint 1 |
| 40% token reduction achievable | A/B test after MVP |

---

## 10. Epic Breakdown

### Epic Roadmap

```
Week 1: EPIC-001 (Core CLI Foundation)
├── F1: Symbol Search (13 points)
├── F2: JSON/Text Output (5 points)
└── F3: Context Modes (5 points)
    → v0.1.0 Release

Week 2: EPIC-002 (Advanced Features)
├── F4: Background Indexing (13 points)
├── F5: Repository Map (8 points)
└── F6: Dependency Graph (8 points)
    → v1.0.0 Release
```

### Epic Summaries

| Epic | Business Goal | Features | Points |
|------|---------------|----------|--------|
| EPIC-001 | Core CLI with AST search | F1, F2, F3 | 23 |
| EPIC-002 | Production-scale features | F4, F5, F6 | 29 |
| **Total** | | **6 features** | **52** |

### Dependency Graph

```
EPIC-001 (Core CLI) ────► EPIC-002 (Advanced Features)
```

---

## 11. Next Steps

1. **Architecture Phase:** Run `/create-context treelint`
   - Generate tech-stack.md, source-tree.md, dependencies.md
   - Create architecture-constraints.md, anti-patterns.md, coding-standards.md

2. **Sprint Planning:** Run `/create-sprint 1`
   - Create detailed stories from EPIC-001 features
   - Assign story points, define acceptance criteria

3. **Development:** Run `/dev STORY-XXX`
   - TDD implementation of each story
   - Continuous validation against context files

---

## Appendices

### A. Glossary

| Term | Definition |
|------|------------|
| AST | Abstract Syntax Tree - structured representation of code parsed by tree-sitter |
| tree-sitter | Incremental parsing system that generates ASTs for 100+ languages |
| Token | Unit of text consumed by AI models; search results consume input tokens |
| Context window | Maximum tokens an AI can process in one request (~128K-200K) |
| Symbol | Named code construct (function, class, method, variable, etc.) |
| Repository map | Summary of all symbols in a codebase (Aider's approach) |
| WAL | Write-Ahead Logging - SQLite mode for concurrent reads/writes |
| PageRank | Algorithm for ranking items by reference count (used for relevance scoring) |

### B. References

| Source | Description |
|--------|-------------|
| BRAINSTORM-001 | `devforgeai/specs/brainstorms/BRAINSTORM-001-treelint-ast-code-search.brainstorm.md` |
| RESEARCH-001 | `devforgeai/specs/research/RESEARCH-001-claude-code-search-token-efficiency.research.md` |
| Aider Repo Map | https://aider.chat/docs/repomap.html |
| tree-sitter | https://tree-sitter.github.io/tree-sitter/ |
| rusqlite | https://docs.rs/rusqlite |

### C. Open Questions

| Question | Status |
|----------|--------|
| Exact tree-sitter-languages crate version? | Resolve in Sprint 1 |
| Windows Unix socket alternative? | Named pipes via interprocess crate |
| Binary size with 4 grammars? | Measure in Sprint 1 |

---

**Document Version:** 1.0
**Last Updated:** 2026-01-27
**Generated By:** DevForgeAI Ideation Skill
