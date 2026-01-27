---
id: STORY-010
title: Repository Map with Symbol Hierarchy and Relevance Scoring
type: feature
epic: EPIC-002
sprint: Backlog
status: Backlog
points: 8
depends_on: ["STORY-003", "STORY-002"]
priority: High
assigned_to: Unassigned
created: 2026-01-27
format_version: "2.7"
---

# Story: Repository Map with Symbol Hierarchy and Relevance Scoring

## Description

**As an** AI coding assistant or developer,
**I want** to generate a comprehensive map of all symbols in the repository with relevance scores,
**so that** I can understand the codebase structure and prioritize the most important code units.

**Business Value:** The repository map is essential for AI assistants to understand codebase structure without reading every file. The PageRank-style relevance scoring helps identify the most important symbols (those referenced most frequently), enabling smarter context selection and reduced token waste.

## Provenance

```xml
<provenance>
  <origin document="EPIC-002" section="Feature F5: Repository Map">
    <quote>"`treelint map` command for full symbol listing, Symbol hierarchy output (by file, by type), PageRank-style reference counting, Relevance score integration (`--ranked` flag)"</quote>
    <line_reference>lines 66-71</line_reference>
    <quantified_impact>Full feature parity with Aider's repo map capabilities</quantified_impact>
  </origin>

  <decision rationale="reference-counting-over-full-pagerank">
    <selected>Simple reference counting (incoming_references + 1) / total_symbols</selected>
    <rejected alternative="full-pagerank-algorithm">
      Full PageRank is computationally expensive and overkill for local symbol importance
    </rejected>
    <trade_off>Less sophisticated ranking in exchange for fast computation</trade_off>
  </decision>

  <stakeholder role="AI Coding Assistant" goal="understand-codebase-structure">
    <quote>"AI coding assistant wants a repository map showing all symbols"</quote>
    <source>EPIC-002, User Stories</source>
  </stakeholder>

  <hypothesis id="H1" validation="user-feedback" success_criteria="100% symbol coverage">
    Repository map with relevance scores will help AI assistants prioritize important code
  </hypothesis>
</provenance>
```

---

## Acceptance Criteria

### AC#1: Map Command Basic Output

```xml
<acceptance_criteria id="AC1" implements="MAP-001,MAP-002">
  <given>A repository with indexed symbols</given>
  <when>User runs `treelint map`</when>
  <then>
    Output includes:
    - Total symbol count and file count
    - All symbols organized by file
    - Each symbol shows: name, type, line range
    - Default format is JSON (if piped) or text (if TTY)
  </then>
  <verification>
    <source_files>
      <file hint="Map command implementation">src/cli/commands/map.rs</file>
    </source_files>
    <test_file>tests/map_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#2: JSON Output Format

```xml
<acceptance_criteria id="AC2" implements="MAP-003">
  <given>User wants machine-readable output</given>
  <when>User runs `treelint map --format json`</when>
  <then>
    JSON output matches schema:
    - `total_symbols`: integer
    - `total_files`: integer
    - `by_file`: object with file paths as keys, each containing `language` and `symbols` array
    - `by_type`: object with symbol type counts
    Each symbol has: name, type, lines (array), relevance (if --ranked)
  </then>
  <verification>
    <source_files>
      <file hint="JSON formatter">src/output/json.rs</file>
      <file hint="Map command">src/cli/commands/map.rs</file>
    </source_files>
    <test_file>tests/map_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#3: Text Output Format

```xml
<acceptance_criteria id="AC3" implements="MAP-004">
  <given>User wants human-readable output</given>
  <when>User runs `treelint map --format text`</when>
  <then>
    Text output shows:
    - Header with total counts
    - Directory tree structure
    - Files grouped by directory
    - Symbols indented under files with type and line range
    - Relevance score shown with ★ if --ranked
  </then>
  <verification>
    <source_files>
      <file hint="Text formatter">src/output/text.rs</file>
      <file hint="Map command">src/cli/commands/map.rs</file>
    </source_files>
    <test_file>tests/map_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#4: Symbol Type Filtering

```xml
<acceptance_criteria id="AC4" implements="MAP-005">
  <given>User wants to see only specific symbol types</given>
  <when>User runs `treelint map --type function`</when>
  <then>
    Output includes only symbols of the specified type (function, class, method, variable, constant, import, export)
    Total counts reflect filtered results
  </then>
  <verification>
    <source_files>
      <file hint="Type filtering">src/cli/commands/map.rs</file>
    </source_files>
    <test_file>tests/map_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#5: Relevance Score Calculation

```xml
<acceptance_criteria id="AC5" implements="MAP-006,MAP-007">
  <given>Repository has symbols that reference each other</given>
  <when>Relevance scoring is calculated</when>
  <then>
    - Each symbol gets relevance score: (incoming_references + 1) / total_symbols
    - Score normalized to 0.0 - 1.0 range
    - Symbols referenced more frequently have higher scores
    - Score stored in symbols.relevance_score column
  </then>
  <verification>
    <source_files>
      <file hint="Relevance calculation">src/index/relevance.rs</file>
    </source_files>
    <test_file>tests/relevance_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#6: Ranked Output Flag

```xml
<acceptance_criteria id="AC6" implements="MAP-008">
  <given>User wants to see relevance scores</given>
  <when>User runs `treelint map --ranked`</when>
  <then>
    - Output includes relevance score for each symbol
    - Symbols sorted by relevance (highest first) within each file
    - JSON includes `relevance` field
    - Text shows ★ with score (e.g., ★ 0.85)
  </then>
  <verification>
    <source_files>
      <file hint="Ranked output">src/cli/commands/map.rs</file>
    </source_files>
    <test_file>tests/map_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#7: Reference Extraction for Scoring

```xml
<acceptance_criteria id="AC7" implements="MAP-009,MAP-010">
  <given>Repository has files with function calls and imports</given>
  <when>Reference counting runs during index build</when>
  <then>
    - Function calls detected via tree-sitter (call_expression nodes)
    - Import references detected via tree-sitter (import_statement nodes)
    - Reference counts stored for relevance calculation
    - Works for Python, TypeScript, Rust, Markdown
  </then>
  <verification>
    <source_files>
      <file hint="Reference extraction">src/index/relevance.rs</file>
      <file hint="Parser queries">src/parser/symbols.rs</file>
    </source_files>
    <test_file>tests/relevance_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#8: Large Repository Performance

```xml
<acceptance_criteria id="AC8" implements="MAP-011">
  <given>Repository has 100,000+ files</given>
  <when>User runs `treelint map`</when>
  <then>
    - Map generation completes within 10 seconds
    - Memory usage bounded (streaming output if needed)
    - Progress indicator shown if generation takes > 2 seconds
  </then>
  <verification>
    <source_files>
      <file hint="Performance optimization">src/cli/commands/map.rs</file>
    </source_files>
    <test_file>tests/map_tests.rs</test_file>
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
    # Map Command Handler
    - type: "Service"
      name: "MapCommand"
      file_path: "src/cli/commands/map.rs"
      interface: "pub fn execute(args: MapArgs) -> Result<()>"
      lifecycle: "CLI invocation"
      dependencies:
        - "SqliteStorage"
        - "RelevanceScorer"
        - "OutputFormatter"
      requirements:
        - id: "MAP-001"
          description: "Generate comprehensive symbol listing from index"
          testable: true
          test_requirement: "Test: Map output includes all indexed symbols"
          priority: "Critical"
        - id: "MAP-002"
          description: "Auto-detect output format based on TTY"
          testable: true
          test_requirement: "Test: TTY gets text, pipe gets JSON"
          priority: "High"
        - id: "MAP-005"
          description: "Filter symbols by type"
          testable: true
          test_requirement: "Test: --type function shows only functions"
          priority: "High"
        - id: "MAP-008"
          description: "Include relevance scores with --ranked flag"
          testable: true
          test_requirement: "Test: --ranked adds relevance to output"
          priority: "High"
        - id: "MAP-011"
          description: "Complete within 10 seconds for large repos"
          testable: true
          test_requirement: "Test: 100K file repo map completes in < 10s"
          priority: "High"

    # JSON Map Formatter
    - type: "Service"
      name: "MapJsonFormatter"
      file_path: "src/output/json.rs"
      interface: "fn format_map(map: &RepoMap) -> String"
      lifecycle: "Per-output"
      dependencies:
        - "serde_json"
      requirements:
        - id: "MAP-003"
          description: "Generate JSON matching specified schema"
          testable: true
          test_requirement: "Test: JSON has total_symbols, total_files, by_file, by_type"
          priority: "Critical"

    # Text Map Formatter
    - type: "Service"
      name: "MapTextFormatter"
      file_path: "src/output/text.rs"
      interface: "fn format_map_text(map: &RepoMap) -> String"
      lifecycle: "Per-output"
      dependencies:
        - "colored"
      requirements:
        - id: "MAP-004"
          description: "Generate tree-style text output"
          testable: true
          test_requirement: "Test: Output shows directory hierarchy with tree characters"
          priority: "High"

    # Relevance Scorer
    - type: "Service"
      name: "RelevanceScorer"
      file_path: "src/index/relevance.rs"
      interface: "pub fn calculate_relevance(storage: &SqliteStorage) -> Result<()>"
      lifecycle: "Post-index"
      dependencies:
        - "SqliteStorage"
      requirements:
        - id: "MAP-006"
          description: "Calculate relevance score for each symbol"
          testable: true
          test_requirement: "Test: Score = (incoming_refs + 1) / total_symbols"
          priority: "Critical"
        - id: "MAP-007"
          description: "Normalize scores to 0.0-1.0 range"
          testable: true
          test_requirement: "Test: Max score <= 1.0, min score >= 0.0"
          priority: "High"
        - id: "MAP-009"
          description: "Extract function call references"
          testable: true
          test_requirement: "Test: foo() calling bar() increases bar's count"
          priority: "Critical"
        - id: "MAP-010"
          description: "Extract import references"
          testable: true
          test_requirement: "Test: 'import foo' increases foo module's count"
          priority: "High"

    # Schema Extension
    - type: "DataModel"
      name: "SymbolWithRelevance"
      table: "symbols (extended)"
      purpose: "Add relevance score to symbol record"
      fields:
        - name: "relevance_score"
          type: "REAL"
          constraints: "DEFAULT 0.0, >= 0.0, <= 1.0"
          description: "PageRank-style relevance score"
          test_requirement: "Test: relevance_score column exists in symbols table"
        - name: "incoming_references"
          type: "INTEGER"
          constraints: "DEFAULT 0, >= 0"
          description: "Count of references to this symbol"
          test_requirement: "Test: incoming_references updated after analysis"
      indexes:
        - name: "idx_symbols_relevance"
          fields: ["relevance_score"]
          unique: false
          purpose: "Fast sorting by relevance"

    # CLI Arguments
    - type: "Configuration"
      name: "MapArgs"
      file_path: "src/cli/args.rs"
      required_keys:
        - key: "format"
          type: "Option<OutputFormat>"
          example: "--format json | --format text"
          required: false
          default: "Auto-detect (TTY=text, pipe=JSON)"
          validation: "json or text"
          test_requirement: "Test: --format flag parsed correctly"
        - key: "type"
          type: "Option<SymbolType>"
          example: "--type function"
          required: false
          default: "None (all types)"
          validation: "Valid symbol type enum"
          test_requirement: "Test: --type filters output"
        - key: "ranked"
          type: "bool"
          example: "--ranked"
          required: false
          default: "false"
          validation: "Boolean flag"
          test_requirement: "Test: --ranked includes relevance scores"

  business_rules:
    - id: "BR-001"
      rule: "Relevance scores must be recalculated after index updates"
      trigger: "Index build or incremental update"
      validation: "Relevance calculation runs after symbol extraction"
      error_handling: "Log warning if calculation fails, return scores as 0.0"
      test_requirement: "Test: After file change, relevance scores are updated"
      priority: "High"

    - id: "BR-002"
      rule: "Symbols with no references have minimum score (1/total)"
      trigger: "Relevance calculation"
      validation: "Formula includes +1 to avoid zero scores"
      error_handling: "N/A"
      test_requirement: "Test: Unreferenced symbol has non-zero score"
      priority: "Medium"

    - id: "BR-003"
      rule: "Map output is deterministic (same input = same output)"
      trigger: "Map generation"
      validation: "Symbols sorted consistently (by file path, then line number)"
      error_handling: "N/A"
      test_requirement: "Test: Two map calls produce identical output"
      priority: "Medium"

  non_functional_requirements:
    - id: "NFR-001"
      category: "Performance"
      requirement: "Map generation must be fast for large repos"
      metric: "< 10 seconds for 100K file repository"
      test_requirement: "Test: Benchmark map on 100K file repo"
      priority: "High"

    - id: "NFR-002"
      category: "Performance"
      requirement: "Relevance calculation must not slow indexing significantly"
      metric: "< 5% additional indexing time for relevance"
      test_requirement: "Test: Compare index time with/without relevance"
      priority: "Medium"

    - id: "NFR-003"
      category: "Accuracy"
      requirement: "Reference detection must be accurate"
      metric: ">= 90% accuracy for function calls and imports"
      test_requirement: "Test: Manual verification of 50 sample references"
      priority: "High"
```

---

## Technical Limitations

```yaml
technical_limitations:
  - id: TL-001
    component: "Reference extraction"
    limitation: "Dynamic references (eval, reflection) cannot be detected"
    decision: "descope:N/A"
    discovered_phase: "Architecture"
    impact: "Some references may be missed, affecting relevance accuracy slightly"
```

---

## Non-Functional Requirements (NFRs)

### Performance

**Latency:**
- **Map generation:** < 10 seconds for 100K files
- **Relevance calculation:** < 5% overhead during indexing

**Resource Usage:**
- Memory bounded (streaming output for very large repos)

### Accuracy

**Reference Detection:**
- ≥ 90% accuracy for function calls
- ≥ 90% accuracy for imports

---

## Dependencies

### Prerequisite Stories

- [x] **STORY-003:** SQLite Index Storage
  - **Why:** Map reads from SQLite index
  - **Status:** Backlog

- [x] **STORY-002:** tree-sitter Parser Integration
  - **Why:** Reference extraction uses tree-sitter queries
  - **Status:** Backlog

### Technology Dependencies

All dependencies already approved in dependencies.md:
- serde_json (1.0) - JSON output
- colored (2.1) - Text output formatting

---

## Test Strategy

### Unit Tests

**Coverage Target:** 95% for src/cli/commands/map.rs, src/index/relevance.rs

**Test Scenarios:**
1. **Happy Path:**
   - Map with all symbols
   - Map with type filter
   - Map with ranked output
   - JSON and text formats
2. **Edge Cases:**
   - Empty repository
   - Single file repository
   - No references (all scores equal)
   - Self-referential symbol
3. **Error Cases:**
   - Index not found
   - Corrupted index

### Integration Tests

**Coverage Target:** 85%

**Test Scenarios:**
1. **End-to-End:** Index → Map → Verify all symbols present
2. **Relevance:** Create references → Verify scores reflect relationships
3. **Large Repo:** Performance benchmark on 100K files

---

## Acceptance Criteria Verification Checklist

### AC#1: Map Command Basic Output

- [ ] Map command implemented - **Phase:** 3 - **Evidence:** src/cli/commands/map.rs
- [ ] Shows total counts - **Phase:** 3 - **Evidence:** tests/map_tests.rs
- [ ] Auto-detects format - **Phase:** 3 - **Evidence:** tests/map_tests.rs

### AC#2: JSON Output Format

- [ ] Schema matches specification - **Phase:** 3 - **Evidence:** tests/map_tests.rs
- [ ] by_file structure correct - **Phase:** 3 - **Evidence:** tests/map_tests.rs
- [ ] by_type counts correct - **Phase:** 3 - **Evidence:** tests/map_tests.rs

### AC#3: Text Output Format

- [ ] Tree structure output - **Phase:** 3 - **Evidence:** tests/map_tests.rs
- [ ] Directories grouped - **Phase:** 3 - **Evidence:** tests/map_tests.rs
- [ ] Relevance stars shown - **Phase:** 3 - **Evidence:** tests/map_tests.rs

### AC#4: Symbol Type Filtering

- [ ] --type flag works - **Phase:** 3 - **Evidence:** tests/map_tests.rs
- [ ] Counts updated for filter - **Phase:** 3 - **Evidence:** tests/map_tests.rs

### AC#5: Relevance Score Calculation

- [ ] Formula implemented - **Phase:** 3 - **Evidence:** src/index/relevance.rs
- [ ] Scores normalized 0-1 - **Phase:** 3 - **Evidence:** tests/relevance_tests.rs
- [ ] Stored in database - **Phase:** 3 - **Evidence:** tests/relevance_tests.rs

### AC#6: Ranked Output Flag

- [ ] --ranked flag works - **Phase:** 3 - **Evidence:** tests/map_tests.rs
- [ ] Sorted by relevance - **Phase:** 3 - **Evidence:** tests/map_tests.rs

### AC#7: Reference Extraction

- [ ] Function calls detected - **Phase:** 3 - **Evidence:** tests/relevance_tests.rs
- [ ] Import references detected - **Phase:** 3 - **Evidence:** tests/relevance_tests.rs
- [ ] Works for all languages - **Phase:** 5 - **Evidence:** tests/relevance_tests.rs

### AC#8: Large Repository Performance

- [ ] < 10 seconds for 100K files - **Phase:** 5 - **Evidence:** benches/map_bench.rs

---

**Checklist Progress:** 0/19 items complete (0%)

---

## Definition of Done

### Implementation
- [ ] `treelint map` command
- [ ] JSON output formatter for map
- [ ] Text output formatter for map
- [ ] --format flag (json/text)
- [ ] --type flag for filtering
- [ ] --ranked flag for relevance scores
- [ ] RelevanceScorer service
- [ ] Reference extraction (calls, imports)
- [ ] Database schema extension (relevance_score, incoming_references)

### Quality
- [ ] All 8 acceptance criteria have passing tests
- [ ] Edge cases covered (empty repo, no references)
- [ ] Reference detection ≥ 90% accuracy
- [ ] NFRs met (< 10s for 100K files)
- [ ] Code coverage > 95% for map.rs, relevance.rs

### Testing
- [ ] Unit tests for MapCommand
- [ ] Unit tests for RelevanceScorer
- [ ] Unit tests for JSON/text formatters
- [ ] Integration tests for full workflow
- [ ] Performance benchmarks

### Documentation
- [ ] CLI --help updated with map subcommand
- [ ] Relevance algorithm documented
- [ ] Output schemas documented

---

## Change Log

**Current Status:** Backlog

| Date | Author | Phase/Action | Change | Files Affected |
|------|--------|--------------|--------|----------------|
| 2026-01-27 14:00 | claude/story-creation | Created | Story created from EPIC-002 F5 | STORY-010-repository-map.story.md |

## Notes

**Design Decisions:**
- Simple reference counting over full PageRank for performance
- Relevance calculated post-index (not during parsing)
- +1 in formula ensures non-zero scores for unreferenced symbols

**Open Questions:**
- [ ] Should relevance include depth of call chain (transitive references)? - **Owner:** Tech Lead - **Due:** v1.1

**Related ADRs:**
- Reference: EPIC-002 PageRank-Style Relevance Scoring (lines 268-279)

---

Story Template Version: 2.7
Last Updated: 2026-01-27
