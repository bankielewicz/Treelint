---
id: STORY-011
title: Dependency Graph with Call and Import Extraction
type: feature
epic: EPIC-002
sprint: Backlog
status: Backlog
points: 8
depends_on: ["STORY-002", "STORY-003", "STORY-010"]
priority: High
assigned_to: Unassigned
created: 2026-01-27
format_version: "2.7"
---

# Story: Dependency Graph with Call and Import Extraction

## Description

**As a** developer or AI coding assistant,
**I want** to visualize function call relationships and import dependencies as graphs,
**so that** I can understand code flow and trace module dependencies across the codebase.

**Business Value:** Dependency graphs reveal the hidden structure of a codebase. For AI assistants, understanding which functions call which enables smarter context selection - if you need to understand function A, you probably also need the functions it calls. For developers, visual call graphs help with refactoring, debugging, and onboarding.

## Provenance

```xml
<provenance>
  <origin document="EPIC-002" section="Feature F6: Dependency Graph">
    <quote>"Import/export relationship extraction, Function call detection (tree-sitter queries), Graph output formats (JSON, Mermaid), `treelint deps --calls` for call graph, `treelint deps --imports` for import graph"</quote>
    <line_reference>lines 73-78</line_reference>
    <quantified_impact>≥ 90% call/import relationships detected</quantified_impact>
  </origin>

  <decision rationale="mermaid-for-visualization">
    <selected>Mermaid diagram output for easy visualization</selected>
    <rejected alternative="graphviz-dot">
      Mermaid is more accessible (renders in GitHub, many markdown viewers)
    </rejected>
    <rejected alternative="svg-output">
      Requires complex rendering logic; Mermaid lets tools handle rendering
    </rejected>
    <trade_off>Less control over layout in exchange for broad compatibility</trade_off>
  </decision>

  <stakeholder role="Developer" goal="understand-code-flow">
    <quote>"Developer wants to see which functions call which"</quote>
    <source>EPIC-002, User Stories</source>
  </stakeholder>

  <hypothesis id="H1" validation="accuracy-measurement" success_criteria="≥90% relationships detected">
    Tree-sitter queries can accurately extract call and import relationships
  </hypothesis>
</provenance>
```

---

## Acceptance Criteria

### AC#1: Call Graph Command

```xml
<acceptance_criteria id="AC1" implements="DEPS-001,DEPS-002">
  <given>A repository with indexed symbols containing function calls</given>
  <when>User runs `treelint deps --calls`</when>
  <then>
    Output shows function call relationships:
    - Nodes: all functions/methods with their file paths
    - Edges: caller → callee relationships with call count
    - Default format is JSON
  </then>
  <verification>
    <source_files>
      <file hint="Deps command">src/cli/commands/deps.rs</file>
      <file hint="Call graph extraction">src/graph/calls.rs</file>
    </source_files>
    <test_file>tests/deps_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#2: Import Graph Command

```xml
<acceptance_criteria id="AC2" implements="DEPS-003,DEPS-004">
  <given>A repository with files that import/export modules</given>
  <when>User runs `treelint deps --imports`</when>
  <then>
    Output shows import relationships:
    - Nodes: all files/modules
    - Edges: importer → imported module relationships
    - Default format is JSON
  </then>
  <verification>
    <source_files>
      <file hint="Deps command">src/cli/commands/deps.rs</file>
      <file hint="Import graph extraction">src/graph/imports.rs</file>
    </source_files>
    <test_file>tests/deps_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#3: JSON Graph Output

```xml
<acceptance_criteria id="AC3" implements="DEPS-005">
  <given>User wants machine-readable graph output</given>
  <when>User runs `treelint deps --calls --format json`</when>
  <then>
    JSON output matches schema:
    - `graph_type`: "calls" or "imports"
    - `nodes`: array of {id, file, type}
    - `edges`: array of {from, to, count (for calls)}
  </then>
  <verification>
    <source_files>
      <file hint="JSON graph formatter">src/output/json.rs</file>
    </source_files>
    <test_file>tests/deps_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#4: Mermaid Diagram Output

```xml
<acceptance_criteria id="AC4" implements="DEPS-006,DEPS-007">
  <given>User wants visual graph output</given>
  <when>User runs `treelint deps --calls --format mermaid`</when>
  <then>
    Mermaid output is valid diagram:
    - Starts with `graph TD` (for calls) or `graph LR` (for imports)
    - Nodes include function/file name and path
    - Edges show relationship with optional count label
    - Output can be pasted into Mermaid-compatible renderer
  </then>
  <verification>
    <source_files>
      <file hint="Mermaid formatter">src/output/graph.rs</file>
    </source_files>
    <test_file>tests/deps_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#5: Symbol-Specific Graph

```xml
<acceptance_criteria id="AC5" implements="DEPS-008">
  <given>User wants graph for a specific symbol</given>
  <when>User runs `treelint deps --calls --symbol validateUser`</when>
  <then>
    Output shows graph centered on specified symbol:
    - Includes direct callers (functions that call validateUser)
    - Includes direct callees (functions validateUser calls)
    - Optionally includes N levels of depth (default 1)
  </then>
  <verification>
    <source_files>
      <file hint="Symbol filtering">src/cli/commands/deps.rs</file>
      <file hint="Graph traversal">src/graph/calls.rs</file>
    </source_files>
    <test_file>tests/deps_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#6: Function Call Detection

```xml
<acceptance_criteria id="AC6" implements="DEPS-009,DEPS-010">
  <given>Source files with function calls</given>
  <when>Call graph extraction runs</when>
  <then>
    Detects function calls via tree-sitter:
    - Python: call_expression nodes
    - TypeScript: call_expression nodes
    - Rust: call_expression, method_call_expression nodes
    - Resolves callee to indexed symbol when possible
    - Records call count (how many times A calls B)
  </then>
  <verification>
    <source_files>
      <file hint="Call detection">src/graph/calls.rs</file>
      <file hint="Python queries">src/parser/queries/python.rs</file>
      <file hint="TypeScript queries">src/parser/queries/typescript.rs</file>
      <file hint="Rust queries">src/parser/queries/rust.rs</file>
    </source_files>
    <test_file>tests/graph_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#7: Import Relationship Detection

```xml
<acceptance_criteria id="AC7" implements="DEPS-011,DEPS-012">
  <given>Source files with import statements</given>
  <when>Import graph extraction runs</when>
  <then>
    Detects imports via tree-sitter:
    - Python: import_statement, import_from_statement
    - TypeScript: import_statement, export_statement
    - Rust: use_declaration, mod_item
    - Resolves module path to actual file when possible
    - Handles relative and absolute imports
  </then>
  <verification>
    <source_files>
      <file hint="Import detection">src/graph/imports.rs</file>
    </source_files>
    <test_file>tests/graph_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#8: Graph Storage and Caching

```xml
<acceptance_criteria id="AC8" implements="DEPS-013,DEPS-014">
  <given>Repository with call/import relationships</given>
  <when>Graph data is extracted</when>
  <then>
    - Call edges stored in SQLite (caller_id, callee_id, count)
    - Import edges stored in SQLite (importer_path, imported_path)
    - Incremental update: only re-analyze changed files
    - Full rebuild with `--force` flag
  </then>
  <verification>
    <source_files>
      <file hint="Graph storage">src/graph/calls.rs</file>
      <file hint="Graph storage">src/graph/imports.rs</file>
      <file hint="Schema">src/index/schema.rs</file>
    </source_files>
    <test_file>tests/graph_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

## Technical Specification

```yaml
technical_specification:
  format_version: "2.0"

  components:
    # Deps Command Handler
    - type: "Service"
      name: "DepsCommand"
      file_path: "src/cli/commands/deps.rs"
      interface: "pub fn execute(args: DepsArgs) -> Result<()>"
      lifecycle: "CLI invocation"
      dependencies:
        - "CallGraphExtractor"
        - "ImportGraphExtractor"
        - "OutputFormatter"
      requirements:
        - id: "DEPS-001"
          description: "Execute call graph extraction with --calls flag"
          testable: true
          test_requirement: "Test: --calls produces call graph output"
          priority: "Critical"
        - id: "DEPS-002"
          description: "Output call graph with nodes and edges"
          testable: true
          test_requirement: "Test: Output includes all call relationships"
          priority: "Critical"
        - id: "DEPS-003"
          description: "Execute import graph extraction with --imports flag"
          testable: true
          test_requirement: "Test: --imports produces import graph output"
          priority: "Critical"
        - id: "DEPS-004"
          description: "Output import graph with file nodes and edges"
          testable: true
          test_requirement: "Test: Output includes all import relationships"
          priority: "Critical"
        - id: "DEPS-008"
          description: "Filter graph to specific symbol with --symbol flag"
          testable: true
          test_requirement: "Test: --symbol foo shows only foo's relationships"
          priority: "High"

    # Call Graph Extractor
    - type: "Service"
      name: "CallGraphExtractor"
      file_path: "src/graph/calls.rs"
      interface: "pub fn extract_calls(storage: &SqliteStorage) -> CallGraph"
      lifecycle: "Post-index"
      dependencies:
        - "tree_sitter::Parser"
        - "SqliteStorage"
      requirements:
        - id: "DEPS-009"
          description: "Detect function calls via tree-sitter queries"
          testable: true
          test_requirement: "Test: foo() calling bar() creates edge foo→bar"
          priority: "Critical"
        - id: "DEPS-010"
          description: "Track call count per edge"
          testable: true
          test_requirement: "Test: 3 calls from A to B shows count=3"
          priority: "High"
        - id: "DEPS-013"
          description: "Store call edges in SQLite"
          testable: true
          test_requirement: "Test: call_edges table populated after extraction"
          priority: "High"

    # Import Graph Extractor
    - type: "Service"
      name: "ImportGraphExtractor"
      file_path: "src/graph/imports.rs"
      interface: "pub fn extract_imports(storage: &SqliteStorage) -> ImportGraph"
      lifecycle: "Post-index"
      dependencies:
        - "tree_sitter::Parser"
        - "SqliteStorage"
      requirements:
        - id: "DEPS-011"
          description: "Detect import statements via tree-sitter"
          testable: true
          test_requirement: "Test: 'import foo' creates edge file→foo"
          priority: "Critical"
        - id: "DEPS-012"
          description: "Resolve module paths to file paths"
          testable: true
          test_requirement: "Test: 'from .utils import bar' resolves to utils.py"
          priority: "High"
        - id: "DEPS-014"
          description: "Store import edges in SQLite"
          testable: true
          test_requirement: "Test: import_edges table populated after extraction"
          priority: "High"

    # Mermaid Formatter
    - type: "Service"
      name: "MermaidFormatter"
      file_path: "src/output/graph.rs"
      interface: "fn format_mermaid(graph: &Graph) -> String"
      lifecycle: "Per-output"
      dependencies: []
      requirements:
        - id: "DEPS-006"
          description: "Generate valid Mermaid diagram syntax"
          testable: true
          test_requirement: "Test: Output starts with 'graph TD' or 'graph LR'"
          priority: "Critical"
        - id: "DEPS-007"
          description: "Include node labels with file paths"
          testable: true
          test_requirement: "Test: Node labels include function name and file"
          priority: "High"

    # JSON Graph Formatter
    - type: "Service"
      name: "GraphJsonFormatter"
      file_path: "src/output/json.rs"
      interface: "fn format_graph_json(graph: &Graph) -> String"
      lifecycle: "Per-output"
      dependencies:
        - "serde_json"
      requirements:
        - id: "DEPS-005"
          description: "Generate JSON matching specified schema"
          testable: true
          test_requirement: "Test: JSON has graph_type, nodes, edges arrays"
          priority: "Critical"

    # Database Schema Extension
    - type: "DataModel"
      name: "CallEdge"
      table: "call_edges"
      purpose: "Store function call relationships"
      fields:
        - name: "caller_id"
          type: "INTEGER"
          constraints: "FOREIGN KEY symbols(id)"
          description: "ID of calling function"
          test_requirement: "Test: caller_id references valid symbol"
        - name: "callee_id"
          type: "INTEGER"
          constraints: "FOREIGN KEY symbols(id)"
          description: "ID of called function"
          test_requirement: "Test: callee_id references valid symbol"
        - name: "call_count"
          type: "INTEGER"
          constraints: "DEFAULT 1, >= 1"
          description: "Number of times caller calls callee"
          test_requirement: "Test: call_count increments on duplicate edges"
      indexes:
        - name: "idx_call_edges_caller"
          fields: ["caller_id"]
          unique: false
          purpose: "Fast lookup of what a function calls"
        - name: "idx_call_edges_callee"
          fields: ["callee_id"]
          unique: false
          purpose: "Fast lookup of what calls a function"

    - type: "DataModel"
      name: "ImportEdge"
      table: "import_edges"
      purpose: "Store import/export relationships"
      fields:
        - name: "importer_path"
          type: "TEXT"
          constraints: "NOT NULL"
          description: "Path of file that imports"
          test_requirement: "Test: importer_path is valid file path"
        - name: "imported_path"
          type: "TEXT"
          constraints: "NOT NULL"
          description: "Path of imported module/file"
          test_requirement: "Test: imported_path resolved to actual file"
        - name: "import_type"
          type: "TEXT"
          constraints: "CHECK(import_type IN ('direct', 'from', 'star'))"
          description: "Type of import (direct, from, star)"
          test_requirement: "Test: import_type captured correctly"
      indexes:
        - name: "idx_import_edges_importer"
          fields: ["importer_path"]
          unique: false
          purpose: "Fast lookup of what a file imports"

    # CLI Arguments
    - type: "Configuration"
      name: "DepsArgs"
      file_path: "src/cli/args.rs"
      required_keys:
        - key: "calls"
          type: "bool"
          example: "--calls"
          required: false
          default: "false"
          validation: "Boolean flag"
          test_requirement: "Test: --calls flag parsed"
        - key: "imports"
          type: "bool"
          example: "--imports"
          required: false
          default: "false"
          validation: "Boolean flag"
          test_requirement: "Test: --imports flag parsed"
        - key: "format"
          type: "Option<GraphFormat>"
          example: "--format mermaid"
          required: false
          default: "json"
          validation: "json or mermaid"
          test_requirement: "Test: --format flag parsed"
        - key: "symbol"
          type: "Option<String>"
          example: "--symbol validateUser"
          required: false
          default: "None (full graph)"
          validation: "Valid symbol name"
          test_requirement: "Test: --symbol filters graph"

  business_rules:
    - id: "BR-001"
      rule: "At least one of --calls or --imports must be specified"
      trigger: "deps command execution"
      validation: "Check at least one flag is true"
      error_handling: "Return error: 'Specify --calls or --imports'"
      test_requirement: "Test: deps without flags shows error"
      priority: "High"

    - id: "BR-002"
      rule: "Unresolved calls (external libraries) are excluded from graph"
      trigger: "Call graph extraction"
      validation: "Only include edges where both endpoints are indexed symbols"
      error_handling: "Skip silently"
      test_requirement: "Test: Call to os.path.join not in graph"
      priority: "Medium"

    - id: "BR-003"
      rule: "Graph extraction runs incrementally after file changes"
      trigger: "File watcher update"
      validation: "Only re-analyze changed files"
      error_handling: "N/A"
      test_requirement: "Test: Single file change doesn't rebuild entire graph"
      priority: "High"

  non_functional_requirements:
    - id: "NFR-001"
      category: "Performance"
      requirement: "Graph extraction must be fast for large repos"
      metric: "< 30 seconds for 100K file repository"
      test_requirement: "Test: Benchmark deps on 100K file repo"
      priority: "High"

    - id: "NFR-002"
      category: "Accuracy"
      requirement: "Call detection must be accurate"
      metric: ">= 90% of actual function calls detected"
      test_requirement: "Test: Manual verification of 50 sample calls"
      priority: "High"

    - id: "NFR-003"
      category: "Accuracy"
      requirement: "Import detection must be accurate"
      metric: ">= 90% of imports detected and resolved"
      test_requirement: "Test: Manual verification of 50 sample imports"
      priority: "High"
```

---

## Technical Limitations

```yaml
technical_limitations:
  - id: TL-001
    component: "Call detection"
    limitation: "Cannot detect calls through dynamic dispatch or reflection"
    decision: "descope:N/A"
    discovered_phase: "Architecture"
    impact: "Some call relationships may be missed (estimated < 10%)"

  - id: TL-002
    component: "Import resolution"
    limitation: "Cannot resolve imports that depend on runtime configuration (sys.path)"
    decision: "workaround:use heuristics (check common locations)"
    discovered_phase: "Architecture"
    impact: "Some imports may not resolve to actual files"
```

---

## Non-Functional Requirements (NFRs)

### Performance

**Latency:**
- **Graph extraction:** < 30 seconds for 100K files
- **Symbol-specific graph:** < 1 second

### Accuracy

**Detection Rates:**
- Function calls: ≥ 90%
- Import relationships: ≥ 90%

---

## Dependencies

### Prerequisite Stories

- [x] **STORY-002:** tree-sitter Parser Integration
  - **Why:** Call/import detection uses tree-sitter queries
  - **Status:** Backlog

- [x] **STORY-003:** SQLite Index Storage
  - **Why:** Graph data stored in SQLite
  - **Status:** Backlog

- [x] **STORY-010:** Repository Map with Relevance Scoring
  - **Why:** Shares reference extraction logic
  - **Status:** Backlog

### Technology Dependencies

All dependencies already approved in dependencies.md:
- serde_json (1.0) - JSON output

---

## Test Strategy

### Unit Tests

**Coverage Target:** 95% for src/graph/calls.rs, imports.rs, src/cli/commands/deps.rs

**Test Scenarios:**
1. **Happy Path:**
   - Call graph for simple project
   - Import graph for simple project
   - Mermaid output generation
   - Symbol-specific filtering
2. **Edge Cases:**
   - Self-calling function (recursion)
   - Circular imports
   - Unresolved imports (external library)
   - No calls/imports in project
3. **Error Cases:**
   - Neither --calls nor --imports specified
   - Invalid symbol name for --symbol

### Integration Tests

**Coverage Target:** 85%

**Test Scenarios:**
1. **End-to-End:** Index → Deps --calls → Verify relationships
2. **Multi-Language:** Call detection across Python, TypeScript, Rust
3. **Large Repo:** Performance benchmark on 100K files

---

## Acceptance Criteria Verification Checklist

### AC#1: Call Graph Command

- [ ] --calls flag works - **Phase:** 3 - **Evidence:** tests/deps_tests.rs
- [ ] Nodes include functions - **Phase:** 3 - **Evidence:** tests/deps_tests.rs
- [ ] Edges show caller→callee - **Phase:** 3 - **Evidence:** tests/deps_tests.rs

### AC#2: Import Graph Command

- [ ] --imports flag works - **Phase:** 3 - **Evidence:** tests/deps_tests.rs
- [ ] Nodes include files - **Phase:** 3 - **Evidence:** tests/deps_tests.rs
- [ ] Edges show importer→imported - **Phase:** 3 - **Evidence:** tests/deps_tests.rs

### AC#3: JSON Graph Output

- [ ] Schema matches spec - **Phase:** 3 - **Evidence:** tests/deps_tests.rs
- [ ] Includes graph_type - **Phase:** 3 - **Evidence:** tests/deps_tests.rs

### AC#4: Mermaid Diagram Output

- [ ] Valid Mermaid syntax - **Phase:** 3 - **Evidence:** tests/deps_tests.rs
- [ ] Node labels with paths - **Phase:** 3 - **Evidence:** tests/deps_tests.rs
- [ ] Edge labels with counts - **Phase:** 3 - **Evidence:** tests/deps_tests.rs

### AC#5: Symbol-Specific Graph

- [ ] --symbol flag works - **Phase:** 3 - **Evidence:** tests/deps_tests.rs
- [ ] Shows callers and callees - **Phase:** 3 - **Evidence:** tests/deps_tests.rs

### AC#6: Function Call Detection

- [ ] Python calls detected - **Phase:** 3 - **Evidence:** tests/graph_tests.rs
- [ ] TypeScript calls detected - **Phase:** 3 - **Evidence:** tests/graph_tests.rs
- [ ] Rust calls detected - **Phase:** 3 - **Evidence:** tests/graph_tests.rs
- [ ] Call count tracked - **Phase:** 3 - **Evidence:** tests/graph_tests.rs

### AC#7: Import Relationship Detection

- [ ] Python imports detected - **Phase:** 3 - **Evidence:** tests/graph_tests.rs
- [ ] TypeScript imports detected - **Phase:** 3 - **Evidence:** tests/graph_tests.rs
- [ ] Rust use detected - **Phase:** 3 - **Evidence:** tests/graph_tests.rs
- [ ] Paths resolved - **Phase:** 3 - **Evidence:** tests/graph_tests.rs

### AC#8: Graph Storage

- [ ] call_edges table - **Phase:** 3 - **Evidence:** src/index/schema.rs
- [ ] import_edges table - **Phase:** 3 - **Evidence:** src/index/schema.rs
- [ ] Incremental update works - **Phase:** 3 - **Evidence:** tests/graph_tests.rs

---

**Checklist Progress:** 0/24 items complete (0%)

---

## Definition of Done

### Implementation
- [ ] `treelint deps` command
- [ ] --calls flag for call graph
- [ ] --imports flag for import graph
- [ ] --format flag (json/mermaid)
- [ ] --symbol flag for filtering
- [ ] CallGraphExtractor service
- [ ] ImportGraphExtractor service
- [ ] MermaidFormatter
- [ ] call_edges table in SQLite
- [ ] import_edges table in SQLite

### Quality
- [ ] All 8 acceptance criteria have passing tests
- [ ] Edge cases covered (recursion, circular imports)
- [ ] Detection accuracy ≥ 90%
- [ ] NFRs met (< 30s for 100K files)
- [ ] Code coverage > 95% for graph modules

### Testing
- [ ] Unit tests for CallGraphExtractor
- [ ] Unit tests for ImportGraphExtractor
- [ ] Unit tests for MermaidFormatter
- [ ] Integration tests for deps command
- [ ] Multi-language tests

### Documentation
- [ ] CLI --help updated with deps subcommand
- [ ] Mermaid output format documented
- [ ] Graph storage schema documented

---

## Change Log

**Current Status:** Backlog

| Date | Author | Phase/Action | Change | Files Affected |
|------|--------|--------------|--------|----------------|
| 2026-01-27 14:30 | claude/story-creation | Created | Story created from EPIC-002 F6 | STORY-011-dependency-graph.story.md |

## Notes

**Design Decisions:**
- Mermaid over GraphViz for broad compatibility
- Store edges in SQLite for incremental updates
- Exclude external library calls from graph (can't resolve)

**Open Questions:**
- [ ] Should depth parameter be added for --symbol? (show N levels) - **Owner:** Tech Lead - **Due:** v1.1

**Related ADRs:**
- Reference: EPIC-002 Dependency Graph Output (lines 229-266)

---

Story Template Version: 2.7
Last Updated: 2026-01-27
