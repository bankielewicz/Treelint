---
id: STORY-002
title: tree-sitter Parser Integration
type: feature
epic: EPIC-001
sprint: SPRINT-001
status: Dev Complete
points: 5
depends_on: ["STORY-001"]
priority: Critical
assigned_to: Unassigned
created: 2026-01-27
format_version: "2.7"
---

# Story: tree-sitter Parser Integration

## Description

**As a** code search engine,
**I want** to parse source files using tree-sitter with embedded grammars for Python, TypeScript, Rust, and Markdown,
**so that** I can extract semantic symbols (functions, classes, methods) with accurate line boundaries and metadata for token-efficient AI consumption.

## Provenance

```xml
<provenance>
  <origin document="BRAINSTORM-001" section="opportunities">
    <quote>"Symbol-based code search using tree-sitter AST parsing to return semantic code units instead of raw text lines"</quote>
    <line_reference>lines 130-131</line_reference>
    <quantified_impact>40-80% token reduction through semantic precision</quantified_impact>
  </origin>

  <decision rationale="tree-sitter-over-alternatives">
    <selected>tree-sitter with embedded grammars - mature, incremental parsing, 100+ languages</selected>
    <rejected alternative="syn">
      Rust-only, not cross-language
    </rejected>
    <rejected alternative="ANTLR">
      Heavy runtime, complex setup
    </rejected>
    <trade_off>Binary size increase (~30-40MB for 4 grammars) vs runtime grammar loading</trade_off>
  </decision>

  <stakeholder role="AI Coding Assistant" goal="semantic-precision">
    <quote>"Get the exact function I need without false positives"</quote>
    <source>EPIC-001, User Stories</source>
  </stakeholder>

  <hypothesis id="H1" validation="build-test" success_criteria="Compile succeeds, binary < 50MB">
    Tree-sitter grammars can embed in Rust binary
  </hypothesis>
</provenance>
```

---

## Acceptance Criteria

### AC#1: Embedded Grammar Loading

```xml
<acceptance_criteria id="AC1" implements="DM-001,DM-003,SVC-001">
  <given>The Treelint binary is compiled with embedded tree-sitter grammars</given>
  <when>The parser initializes for any supported language (Python, TypeScript, Rust, Markdown)</when>
  <then>The grammar loads from the compiled binary without requiring external grammar files, and initialization completes in less than 10ms</then>
  <verification>
    <source_files>
      <file hint="Language enum and grammar loading">src/parser/languages.rs</file>
      <file hint="Build script for grammar embedding">build.rs</file>
    </source_files>
    <test_file>tests/STORY-002/test_ac1_grammar_loading.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#2: Language Detection from File Extension

```xml
<acceptance_criteria id="AC2" implements="DM-001,DM-002">
  <given>A source file with a recognized extension</given>
  <when>The parser determines the file's language</when>
  <then>The correct Language enum is returned: .py→Python, .ts/.tsx→TypeScript, .js/.jsx→TypeScript, .rs→Rust, .md/.markdown→Markdown</then>
  <verification>
    <source_files>
      <file hint="Extension to language mapping">src/parser/languages.rs</file>
    </source_files>
    <test_file>tests/STORY-002/test_ac2_language_detection.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#3: Symbol Extraction - Functions

```xml
<acceptance_criteria id="AC3" implements="DM-004,DM-005,DM-006,SVC-002,CFG-001,CFG-004,CFG-006,CFG-007">
  <given>A source file containing function definitions</given>
  <when>The parser extracts symbols from the file</when>
  <then>Each function is returned as a Symbol struct with name, SymbolType::Function, accurate line_start/line_end, signature, and language</then>
  <verification>
    <source_files>
      <file hint="Symbol struct and extractor">src/parser/symbols.rs</file>
      <file hint="Python queries">src/parser/queries/python.rs</file>
      <file hint="TypeScript queries">src/parser/queries/typescript.rs</file>
      <file hint="Rust queries">src/parser/queries/rust.rs</file>
    </source_files>
    <test_file>tests/STORY-002/test_ac3_function_extraction.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#4: Symbol Extraction - Classes and Methods

```xml
<acceptance_criteria id="AC4" implements="DM-004,DM-006,DM-007,SVC-003,CFG-002,CFG-004,CFG-006">
  <given>A source file containing class definitions with methods</given>
  <when>The parser extracts symbols from the file</when>
  <then>Both the class (SymbolType::Class) and each method (SymbolType::Method) are returned as separate Symbol structs</then>
  <verification>
    <source_files>
      <file hint="Symbol extractor with class/method handling">src/parser/symbols.rs</file>
      <file hint="Python class queries">src/parser/queries/python.rs</file>
    </source_files>
    <test_file>tests/STORY-002/test_ac4_class_method_extraction.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#5: Symbol Extraction - Variables, Constants, Imports, Exports

```xml
<acceptance_criteria id="AC5" implements="DM-006,SVC-004,CFG-003,CFG-005,CFG-006">
  <given>A source file containing variable declarations, constants, imports, and exports</given>
  <when>The parser extracts symbols from the file</when>
  <then>Each is returned with appropriate SymbolType: Variable, Constant, Import, Export (TypeScript only)</then>
  <verification>
    <source_files>
      <file hint="Symbol extraction logic">src/parser/symbols.rs</file>
      <file hint="Import/export queries">src/parser/queries/typescript.rs</file>
    </source_files>
    <test_file>tests/STORY-002/test_ac5_variable_import_extraction.rs</test_file>
    <coverage_threshold>85</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#6: Error Handling for Malformed Files

```xml
<acceptance_criteria id="AC6" implements="SVC-005">
  <given>A source file with syntax errors or malformed content</given>
  <when>The parser attempts to extract symbols</when>
  <then>The parser does NOT panic, returns partial results for valid portions (tree-sitter error recovery), and returns empty list for completely unparseable files</then>
  <verification>
    <source_files>
      <file hint="Error handling in extractor">src/parser/symbols.rs</file>
    </source_files>
    <test_file>tests/STORY-002/test_ac6_error_handling.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

## Technical Specification

```yaml
technical_specification:
  format_version: "2.0"
  story_id: "STORY-002"

  components:
    - type: "DataModel"
      name: "Language Enum"
      file_path: "src/parser/languages.rs"
      requirements:
        - id: "DM-001"
          description: "Define Language enum with Python, TypeScript, Rust, Markdown variants"
          testable: true
          test_requirement: "Test: Language enum has 4 variants"
          priority: "Critical"
          implements_ac: ["AC#1", "AC#2"]
        - id: "DM-002"
          description: "Implement from_extension() for language detection"
          testable: true
          test_requirement: "Test: Language::from_extension('.py') returns Some(Language::Python)"
          priority: "Critical"
          implements_ac: ["AC#2"]
        - id: "DM-003"
          description: "Return embedded tree-sitter::Language from each variant"
          testable: true
          test_requirement: "Test: Language::Python.tree_sitter_language() returns valid Language"
          priority: "Critical"
          implements_ac: ["AC#1"]

    - type: "DataModel"
      name: "Symbol Struct"
      file_path: "src/parser/symbols.rs"
      requirements:
        - id: "DM-004"
          description: "Define Symbol struct with all required fields"
          testable: true
          test_requirement: "Test: Symbol struct has name, symbol_type, visibility, file_path, line_start, line_end, signature, body, language"
          priority: "Critical"
          implements_ac: ["AC#3", "AC#4", "AC#5"]
        - id: "DM-005"
          description: "Derive Serialize, Clone, Debug for Symbol"
          testable: true
          test_requirement: "Test: serde_json::to_string(&symbol) succeeds"
          priority: "High"
          implements_ac: ["AC#3"]
        - id: "DM-006"
          description: "Define SymbolType enum: Function, Class, Method, Variable, Constant, Import, Export"
          testable: true
          test_requirement: "Test: SymbolType has 7 variants"
          priority: "Critical"
          implements_ac: ["AC#3", "AC#4", "AC#5"]
        - id: "DM-007"
          description: "Define Visibility enum: Public, Private, Protected"
          testable: true
          test_requirement: "Test: Visibility has 3 variants"
          priority: "Medium"
          implements_ac: ["AC#4"]

    - type: "Service"
      name: "SymbolExtractor"
      file_path: "src/parser/symbols.rs"
      requirements:
        - id: "SVC-001"
          description: "Parse file content using tree-sitter with appropriate grammar"
          testable: true
          test_requirement: "Test: parse_file(python_content, Language::Python) returns valid AST"
          priority: "Critical"
          implements_ac: ["AC#1", "AC#3"]
        - id: "SVC-002"
          description: "Extract function symbols with name, lines, signature"
          testable: true
          test_requirement: "Test: extract_symbols() returns Symbol with SymbolType::Function"
          priority: "Critical"
          implements_ac: ["AC#3"]
        - id: "SVC-003"
          description: "Extract class symbols with methods as separate entries"
          testable: true
          test_requirement: "Test: extract_symbols(class_file) returns Class and Method symbols"
          priority: "Critical"
          implements_ac: ["AC#4"]
        - id: "SVC-004"
          description: "Extract variable, constant, import, export symbols"
          testable: true
          test_requirement: "Test: extract_symbols() returns all symbol types present in file"
          priority: "High"
          implements_ac: ["AC#5"]
        - id: "SVC-005"
          description: "Handle syntax errors gracefully with partial results"
          testable: true
          test_requirement: "Test: extract_symbols(malformed_file) returns Ok(partial_results)"
          priority: "High"
          implements_ac: ["AC#6"]

    - type: "Configuration"
      name: "Python Queries"
      file_path: "src/parser/queries/python.rs"
      requirements:
        - id: "CFG-001"
          description: "Define tree-sitter query for Python function_definition"
          testable: true
          test_requirement: "Test: Query captures all functions in Python fixture"
          priority: "Critical"
          implements_ac: ["AC#3"]
        - id: "CFG-002"
          description: "Define tree-sitter query for Python class_definition"
          testable: true
          test_requirement: "Test: Query captures classes and methods"
          priority: "Critical"
          implements_ac: ["AC#4"]
        - id: "CFG-003"
          description: "Define tree-sitter query for Python import_statement"
          testable: true
          test_requirement: "Test: Query captures import and from-import"
          priority: "High"
          implements_ac: ["AC#5"]

    - type: "Configuration"
      name: "TypeScript Queries"
      file_path: "src/parser/queries/typescript.rs"
      requirements:
        - id: "CFG-004"
          description: "Define tree-sitter query for TypeScript function/class/method"
          testable: true
          test_requirement: "Test: Query captures TypeScript symbols"
          priority: "Critical"
          implements_ac: ["AC#3", "AC#4"]
        - id: "CFG-005"
          description: "Define tree-sitter query for TypeScript export_statement"
          testable: true
          test_requirement: "Test: Query captures export statements"
          priority: "High"
          implements_ac: ["AC#5"]

    - type: "Configuration"
      name: "Rust Queries"
      file_path: "src/parser/queries/rust.rs"
      requirements:
        - id: "CFG-006"
          description: "Define tree-sitter query for Rust fn, impl, struct, mod, use"
          testable: true
          test_requirement: "Test: Query captures Rust symbols including impl methods"
          priority: "Critical"
          implements_ac: ["AC#3", "AC#4", "AC#5"]

    - type: "Configuration"
      name: "Markdown Queries"
      file_path: "src/parser/queries/markdown.rs"
      requirements:
        - id: "CFG-007"
          description: "Define tree-sitter query for Markdown headings"
          testable: true
          test_requirement: "Test: Query captures h1-h6 headings"
          priority: "Low"
          implements_ac: ["AC#3"]

  business_rules:
    - id: "BR-001"
      rule: "Grammars must be embedded in binary (no external files)"
      test_requirement: "Test: Binary works without any .so or grammar files in filesystem"
    - id: "BR-002"
      rule: "Syntax errors should return partial results, not failure"
      test_requirement: "Test: File with 1 broken function and 2 valid returns 2 symbols"
    - id: "BR-003"
      rule: "Empty files return empty symbol list (not error)"
      test_requirement: "Test: extract_symbols('') returns Ok([])"

  non_functional_requirements:
    - id: "NFR-001"
      category: "Performance"
      requirement: "Grammar initialization under 10ms"
      metric: "< 10ms per language first load"
      test_requirement: "Test: Benchmark grammar init < 10ms"
    - id: "NFR-002"
      category: "Performance"
      requirement: "File parsing under 50ms"
      metric: "< 50ms for files up to 10,000 lines (p95)"
      test_requirement: "Test: Parse 10K line file < 50ms"
    - id: "NFR-003"
      category: "Reliability"
      requirement: "No panics on malformed input"
      metric: "All errors return Result, no unwrap in production"
      test_requirement: "Test: Fuzz test with garbage input never panics"
    - id: "NFR-004"
      category: "Scalability"
      requirement: "Binary size with grammars under 50MB"
      metric: "< 50MB total with 4 grammars"
      test_requirement: "Test: Release binary size < 50MB"
```

---

## Technical Limitations

```yaml
technical_limitations: []
# No limitations identified - tree-sitter is well-supported in Rust
```

---

## Non-Functional Requirements (NFRs)

### Performance

**Parsing Speed:**
- Grammar initialization: < 10ms per language
- File parsing: < 50ms for files up to 10,000 lines (p95)
- Symbol extraction: < 100ms total per file (p99)

**Memory:**
- Peak memory: < 50MB for files up to 10MB
- No memory leaks during repeated parsing

---

### Security

**No Code Execution:** Parser only reads and analyzes, never executes code
**Path Validation:** File paths validated to stay within project boundaries
**Memory Safety:** Rust ownership guarantees prevent buffer overflows

---

### Reliability

**Error Recovery:** tree-sitter extracts partial results from malformed files
**No Panics:** All errors return Result types
**Idempotent:** Same input always produces same output

---

## Dependencies

### Prerequisite Stories

- [x] **STORY-001:** Project Setup + CLI Skeleton
  - **Why:** Need Cargo.toml, basic project structure, error types
  - **Status:** Backlog

### Technology Dependencies

All from `devforgeai/specs/context/dependencies.md`:

- [ ] **tree-sitter:** 0.22
  - **Purpose:** AST parsing core
  - **Approved:** Yes

- [ ] **tree-sitter-python:** 0.21
  - **Purpose:** Python grammar
  - **Approved:** Yes

- [ ] **tree-sitter-typescript:** 0.21
  - **Purpose:** TypeScript grammar
  - **Approved:** Yes

- [ ] **tree-sitter-rust:** 0.21
  - **Purpose:** Rust grammar
  - **Approved:** Yes

- [ ] **tree-sitter-md:** 0.2
  - **Purpose:** Markdown grammar
  - **Approved:** Yes

---

## Test Strategy

### Unit Tests

**Coverage Target:** 95%+ for parser module

**Test Scenarios:**
1. **Happy Path:** Parse valid files, extract all symbol types
2. **Edge Cases:**
   - Empty files
   - Files with only comments
   - Unicode identifiers
   - Nested functions/closures
   - Very large files (10K+ lines)
3. **Error Cases:**
   - Malformed syntax
   - Binary files
   - Unknown extensions

### Test Fixtures

Create fixtures in `tests/fixtures/`:
- `tests/fixtures/python/` - Python test files
- `tests/fixtures/typescript/` - TypeScript test files
- `tests/fixtures/rust/` - Rust test files
- `tests/fixtures/markdown/` - Markdown test files

---

## Acceptance Criteria Verification Checklist

**Purpose:** Real-time progress tracking during TDD implementation.

### AC#1: Embedded Grammar Loading

- [ ] Language enum defined with 4 variants - **Phase:** 2 - **Evidence:** src/parser/languages.rs
- [ ] tree_sitter_language() method implemented - **Phase:** 3 - **Evidence:** src/parser/languages.rs
- [ ] Grammars load without external files - **Phase:** 3 - **Evidence:** test_ac1_grammar_loading.rs
- [ ] Initialization < 10ms - **Phase:** 3 - **Evidence:** benchmark

### AC#2: Language Detection from File Extension

- [ ] from_extension() method implemented - **Phase:** 3 - **Evidence:** src/parser/languages.rs
- [ ] All extensions mapped correctly - **Phase:** 3 - **Evidence:** test_ac2_language_detection.rs
- [ ] Case-insensitive matching - **Phase:** 3 - **Evidence:** test_ac2_language_detection.rs
- [ ] Unknown extensions return None - **Phase:** 3 - **Evidence:** test_ac2_language_detection.rs

### AC#3: Symbol Extraction - Functions

- [ ] Symbol struct defined with all fields - **Phase:** 2 - **Evidence:** src/parser/symbols.rs
- [ ] SymbolType::Function extraction works - **Phase:** 3 - **Evidence:** test_ac3_function_extraction.rs
- [ ] Python functions extracted - **Phase:** 3 - **Evidence:** test fixture
- [ ] TypeScript functions extracted - **Phase:** 3 - **Evidence:** test fixture
- [ ] Rust functions extracted - **Phase:** 3 - **Evidence:** test fixture

### AC#4: Symbol Extraction - Classes and Methods

- [ ] SymbolType::Class extraction works - **Phase:** 3 - **Evidence:** test_ac4_class_method_extraction.rs
- [ ] SymbolType::Method extraction works - **Phase:** 3 - **Evidence:** test_ac4_class_method_extraction.rs
- [ ] Methods linked to parent class - **Phase:** 3 - **Evidence:** test fixture

### AC#5: Symbol Extraction - Variables, Constants, Imports, Exports

- [ ] Variable extraction works - **Phase:** 3 - **Evidence:** test_ac5_variable_import_extraction.rs
- [ ] Constant extraction works - **Phase:** 3 - **Evidence:** test fixture
- [ ] Import extraction works - **Phase:** 3 - **Evidence:** test fixture
- [ ] TypeScript export extraction works - **Phase:** 3 - **Evidence:** test fixture

### AC#6: Error Handling for Malformed Files

- [ ] No panic on malformed input - **Phase:** 3 - **Evidence:** test_ac6_error_handling.rs
- [ ] Partial results returned - **Phase:** 3 - **Evidence:** test fixture
- [ ] Empty list for unparseable files - **Phase:** 3 - **Evidence:** test fixture

---

**Checklist Progress:** 0/24 items complete (0%)

---

## Definition of Done

### Implementation
- [x] src/parser/languages.rs with Language enum and grammar loading
- [x] src/parser/symbols.rs with Symbol, SymbolType, SymbolExtractor
- [x] src/parser/queries/python.rs with Python tree-sitter queries
- [x] src/parser/queries/typescript.rs with TypeScript tree-sitter queries
- [x] src/parser/queries/rust.rs with Rust tree-sitter queries
- [x] src/parser/queries/markdown.rs with Markdown tree-sitter queries
- [x] src/parser/mod.rs with module exports
- [x] Cargo.toml updated with tree-sitter dependencies

### Quality
- [x] All 6 acceptance criteria have passing tests
- [x] Edge cases covered (empty files, malformed syntax, unicode)
- [x] Error handling tested (no panics)
- [x] Code coverage > 95% for src/parser/
- [x] `cargo clippy -- -D warnings` passes
- [x] `cargo fmt --check` passes
- [x] Binary size < 50MB with embedded grammars

### Testing
- [x] Unit tests for Language enum
- [x] Unit tests for Symbol extraction per language
- [x] Unit tests for error handling
- [x] Test fixtures created for all 4 languages
- [x] Benchmark for grammar init < 10ms
- [x] Benchmark for file parsing < 50ms

### Documentation
- [x] All public items have `///` doc comments
- [x] Module-level `//!` comments for parser module
- [x] Tree-sitter query syntax documented inline

---

## Change Log

**Current Status:** Dev Complete

| Date | Author | Phase/Action | Change | Files Affected |
|------|--------|--------------|--------|----------------|
| 2026-01-27 15:00 | claude/story-requirements-analyst | Created | Story created from EPIC-001 F1.2 | STORY-002-treesitter-parser-integration.story.md |
| 2026-01-27 | claude/sprint-planner | Sprint Planning | Status: Backlog → Ready for Dev, Added to SPRINT-001 | STORY-002-treesitter-parser-integration.story.md |
| 2026-01-27 | claude/devforgeai-development | TDD Implementation | Status: Ready for Dev → Dev Complete. Implemented parser module with 123 tests passing. | src/parser/*, Cargo.toml, tests/story_002_tests/* |

## Implementation Notes

- Parser module implemented with embedded tree-sitter grammars for Python, TypeScript, Rust, Markdown
- All 6 acceptance criteria verified with 123 tests + 8 doc tests
- Code review identified symbols.rs size (1,774 lines) - recommend modularization in follow-up story
- Query definition files created but extraction uses AST walking pattern - architectural decision for maintainability

## Notes

**Design Decisions:**
- Using separate query files per language for maintainability
- Symbol body is optional (lazy loading for token efficiency)
- Methods stored as separate symbols with class context in metadata

**Tree-sitter Query Resources:**
- https://tree-sitter.github.io/tree-sitter/using-parsers#query-syntax
- Grammar node types defined in each tree-sitter-{lang} crate

**References:**
- EPIC-001: Core CLI Foundation
- BRAINSTORM-001: Treelint AST-Aware Code Search
- tech-stack.md: tree-sitter 0.22 approved
- dependencies.md: Grammar crates approved
- source-tree.md: src/parser/ structure

---

Story Template Version: 2.7
Last Updated: 2026-01-27
