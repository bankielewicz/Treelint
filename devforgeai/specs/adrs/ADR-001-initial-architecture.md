# ADR-001: Initial Architecture - Rust CLI with tree-sitter and SQLite

**Status:** Accepted
**Date:** 2026-01-27
**Decision Makers:** Bryan
**Context:** Treelint MVP architecture

---

## Context

We are building Treelint, an AST-aware code search CLI for AI coding assistants. The tool must:

1. Parse code semantically (not text-based search)
2. Extract symbols (functions, classes, methods)
3. Return structured JSON output
4. Achieve < 50ms query latency
5. Ship as a single binary (no dependencies)
6. Support 4 languages: Python, TypeScript, Rust, Markdown
7. Be cross-platform (Windows, macOS, Linux)
8. Complete MVP in < 2 weeks

Market research (RESEARCH-001) shows competing tools using AST achieve 40-83% token reduction vs text search.

---

## Decision

### Technology Stack

| Layer | Technology | Rationale |
|-------|------------|-----------|
| **Language** | Rust | Single binary, cross-platform, performance |
| **Parser** | tree-sitter | Incremental parsing, 100+ languages, proven |
| **Database** | SQLite | Portable, zero-config, WAL for concurrency |
| **CLI** | clap | Industry standard, derive macros |

### Architecture Pattern

**Modular Monolith (Tier 2)** with clear layer boundaries:

```
CLI → Application → Domain → Infrastructure
```

### Key Design Decisions

1. **Embedded Grammars:** tree-sitter grammars compiled into binary (no external files)
2. **SQLite bundled:** Using `rusqlite` with `bundled` feature for zero dependencies
3. **Hybrid Indexing:** Daemon mode with file watcher + on-demand fallback
4. **Auto-detect Output:** JSON when piped, text when terminal

---

## Alternatives Considered

### Alternative 1: Python with External Grammar Files

**Pros:**
- Faster development
- More familiar ecosystem

**Cons:**
- Requires runtime dependency
- Slower execution
- Distribution complexity

**Decision:** Rejected - doesn't meet single-binary requirement

### Alternative 2: Go with Custom Parser

**Pros:**
- Simple cross-compilation
- Good performance

**Cons:**
- No tree-sitter bindings as mature as Rust
- Would need custom parsing for each language

**Decision:** Rejected - tree-sitter ecosystem is more mature

### Alternative 3: Node.js MCP Server

**Pros:**
- Native tree-sitter bindings
- Easy integration with existing MCP ecosystem

**Cons:**
- Requires Node.js runtime
- Not a standalone CLI
- Larger distribution size

**Decision:** Rejected - CLI approach more flexible for AI tools

---

## Consequences

### Positive

1. **Single Binary:** Easy distribution via `cargo install` or direct download
2. **Performance:** Rust + SQLite achieves < 50ms query target
3. **Cross-Platform:** Cargo handles cross-compilation
4. **Future Languages:** tree-sitter supports 100+ languages for expansion

### Negative

1. **Build Complexity:** Grammar embedding requires build.rs script
2. **Binary Size:** Embedded grammars increase binary (~40-50MB)
3. **Learning Curve:** Rust may slow initial development if unfamiliar

### Risks

| Risk | Probability | Mitigation |
|------|-------------|------------|
| Binary too large | Medium | Limit to 4 grammars, use LTO |
| tree-sitter issues | Low | Well-tested library |
| Timeline slip | Medium | Defer F6 (dep graph) if needed |

---

## Validation

This decision will be validated by:

1. **Sprint 1:** Proof-of-concept with Python parsing
2. **Benchmark:** < 50ms query on 10K file codebase
3. **Binary Size:** < 50MB with all 4 grammars

---

## Related Documents

- [BRAINSTORM-001](../brainstorms/BRAINSTORM-001-treelint-ast-code-search.brainstorm.md)
- [RESEARCH-001](../research/RESEARCH-001-claude-code-search-token-efficiency.research.md)
- [tech-stack.md](../context/tech-stack.md)
- [architecture-constraints.md](../context/architecture-constraints.md)

---

## Changelog

| Date | Change | Author |
|------|--------|--------|
| 2026-01-27 | Initial decision | Bryan |
