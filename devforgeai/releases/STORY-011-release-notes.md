# Release Notes: STORY-011

**Release Date:** 2026-01-30
**Version:** 0.1.0 (includes STORY-011)
**Environment:** Test

---

## Feature: Dependency Graph with Call and Import Extraction

### Summary

Added the `treelint deps` command for visualizing function call relationships and import dependencies as graphs.

### New Features

- **Call Graph Analysis** (`--calls`): Detect and visualize function call relationships
  - Supports Python, TypeScript, and Rust
  - Tracks call count per edge
  - Excludes external library calls (unresolvable)

- **Import Graph Analysis** (`--imports`): Detect and visualize import relationships
  - Supports Python (import/from), TypeScript (import/export), Rust (use/mod)
  - Resolves module paths to actual files

- **Output Formats**:
  - JSON (default): Machine-readable graph with nodes and edges
  - Mermaid (`--format mermaid`): Visual diagrams for GitHub/markdown

- **Symbol Filtering** (`--symbol <name>`): Focus on specific function's callers and callees

- **Incremental Updates**: Graph data cached in SQLite, only re-analyze changed files

### Technical Details

- 72 tests covering all 8 acceptance criteria
- 100% test pass rate
- Binary size: 7.8 MB (within 50MB limit)

### Files Changed

**Created:**
- src/cli/commands/deps.rs
- src/graph/mod.rs, calls.rs, imports.rs
- src/output/graph.rs
- tests/STORY-011/*.rs (8 test files)

**Modified:**
- src/cli/args.rs (DepsArgs)
- src/index/storage.rs (call_edges, import_edges tables)

### Usage Examples

```bash
# Show call graph in JSON
treelint deps --calls

# Show import graph as Mermaid diagram
treelint deps --imports --format mermaid

# Focus on specific function
treelint deps --calls --symbol validateUser
```

---

**QA Status:** Approved (2026-01-30)
**Story:** STORY-011-dependency-graph.story.md
