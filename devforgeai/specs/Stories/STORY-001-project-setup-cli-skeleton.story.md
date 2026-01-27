---
id: STORY-001
title: Project Setup + CLI Skeleton
type: feature
epic: EPIC-001
sprint: SPRINT-001
status: Ready for Dev
points: 3
depends_on: []
priority: Critical
assigned_to: Unassigned
created: 2026-01-27
format_version: "2.7"
---

# Story: Project Setup + CLI Skeleton

## Description

**As a** Rust developer building AI-assisted coding tools,
**I want** a working Treelint CLI skeleton with the search command structure,
**so that** I have a foundation to incrementally add AST-aware search functionality without architectural rework.

## Provenance

```xml
<provenance>
  <origin document="BRAINSTORM-001" section="problem-statement">
    <quote>"AI coding assistants experience excessive token consumption when searching code because text-based tools (ripgrep/grep) return false positives from comments and strings, resulting in 40-83% wasted context window"</quote>
    <line_reference>lines 76-78</line_reference>
    <quantified_impact>40-83% token reduction opportunity</quantified_impact>
  </origin>

  <decision rationale="cli-first-approach">
    <selected>Rust CLI with clap for argument parsing - single binary, cross-platform</selected>
    <rejected alternative="MCP server">
      MCP server requires specific client support; CLI works with any AI tool via Bash
    </rejected>
    <trade_off>CLI invocation overhead vs universal compatibility</trade_off>
  </decision>

  <stakeholder role="AI Coding Assistant" goal="token-efficiency">
    <quote>"Token efficiency, accuracy, speed"</quote>
    <source>BRAINSTORM-001, Stakeholder Analysis</source>
  </stakeholder>

  <hypothesis id="H3" validation="integration-test" success_criteria="Works via Bash tool">
    Claude Code can parse and use JSON output via Bash subprocess
  </hypothesis>
</provenance>
```

---

## Acceptance Criteria

### AC#1: Cargo Project Compiles Successfully

```xml
<acceptance_criteria id="AC1" implements="CFG-001,CFG-002,CFG-003">
  <given>A fresh clone of the repository</given>
  <when>cargo build is executed</when>
  <then>The project compiles with zero errors and produces a treelint binary in target/debug/</then>
  <verification>
    <source_files>
      <file hint="Package configuration">Cargo.toml</file>
      <file hint="CLI entry point">src/main.rs</file>
    </source_files>
    <test_file>tests/STORY-001/test_ac1_cargo_build.rs</test_file>
    <coverage_threshold>80</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#2: Search Command Parses All Required Arguments

```xml
<acceptance_criteria id="AC2" implements="DM-001,DM-002,DM-003,DM-004,SVC-002">
  <given>The treelint binary is compiled</given>
  <when>treelint search validateUser --type function -i -r --format json --context 10 --signatures is executed</when>
  <then>All arguments are parsed correctly (symbol: "validateUser", type: "function", ignore_case: true, regex: true, format: json, context: 10, signatures: true) and no parsing errors occur</then>
  <verification>
    <source_files>
      <file hint="CLI arguments definition">src/cli/args.rs</file>
      <file hint="Command routing">src/cli/mod.rs</file>
    </source_files>
    <test_file>tests/STORY-001/test_ac2_argument_parsing.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#3: Help Text Displays Complete CLI Documentation

```xml
<acceptance_criteria id="AC3" implements="SVC-001,DM-001">
  <given>The treelint binary is compiled</given>
  <when>treelint --help or treelint search --help is executed</when>
  <then>Help text displays: program name and version (from Cargo.toml), description of the search command, all options with descriptions (--type, -i, -r, --format, --context, --signatures), and usage examples</then>
  <verification>
    <source_files>
      <file hint="CLI arguments with doc comments">src/cli/args.rs</file>
      <file hint="Entry point">src/main.rs</file>
    </source_files>
    <test_file>tests/STORY-001/test_ac3_help_text.rs</test_file>
    <coverage_threshold>80</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#4: Search Command Returns Placeholder JSON Output

```xml
<acceptance_criteria id="AC4" implements="SVC-004,SVC-005,SVC-006">
  <given>The treelint binary is compiled</given>
  <when>treelint search testSymbol --format json is executed</when>
  <then>The command returns valid JSON with structure: {"query": {"symbol": "testSymbol", "type": null}, "results": [], "stats": {"files_searched": 0, "elapsed_ms": 0}} and exits with code 0</then>
  <verification>
    <source_files>
      <file hint="Search command handler">src/cli/commands/search.rs</file>
      <file hint="Output formatting">src/output/json.rs</file>
    </source_files>
    <test_file>tests/STORY-001/test_ac4_placeholder_output.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#5: Error Types Defined Using thiserror

```xml
<acceptance_criteria id="AC5" implements="DM-005,DM-006,DM-007,SVC-003">
  <given>The source file src/error.rs exists</given>
  <when>The error module is inspected</when>
  <then>It contains a TreelintError enum using #[derive(thiserror::Error)] with variants for: Io (wrapping std::io::Error), Parse (for future parsing errors), Cli (for argument validation errors)</then>
  <verification>
    <source_files>
      <file hint="Error type definitions">src/error.rs</file>
    </source_files>
    <test_file>tests/STORY-001/test_ac5_error_types.rs</test_file>
    <coverage_threshold>80</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

## Technical Specification

```yaml
technical_specification:
  format_version: "2.0"
  story_id: "STORY-001"

  components:
    - type: "Configuration"
      name: "Cargo.toml"
      file_path: "Cargo.toml"
      requirements:
        - id: "CFG-001"
          description: "Define package metadata: name='treelint', version='0.1.0', edition='2021'"
          testable: true
          test_requirement: "Test: cargo metadata returns correct name/version"
          priority: "Critical"
          implements_ac: ["AC#1"]
        - id: "CFG-002"
          description: "Include only approved dependencies from dependencies.md"
          testable: true
          test_requirement: "Test: All dependencies exist in devforgeai/specs/context/dependencies.md"
          priority: "Critical"
          implements_ac: ["AC#1"]
        - id: "CFG-003"
          description: "Set rust-version = '1.70.0' (MSRV)"
          testable: true
          test_requirement: "Test: rust-version field present in [package] section"
          priority: "High"
          implements_ac: ["AC#1"]
        - id: "CFG-004"
          description: "Configure release profile with LTO optimization"
          testable: true
          test_requirement: "Test: [profile.release] contains lto=true"
          priority: "Medium"
          implements_ac: []

    - type: "Service"
      name: "CLI Entry Point"
      file_path: "src/main.rs"
      requirements:
        - id: "SVC-001"
          description: "Parse CLI arguments using clap derive macros"
          testable: true
          test_requirement: "Test: treelint --version outputs version from Cargo.toml"
          priority: "Critical"
          implements_ac: ["AC#3"]
        - id: "SVC-002"
          description: "Route to command handlers based on subcommand"
          testable: true
          test_requirement: "Test: treelint search foo invokes search command handler"
          priority: "Critical"
          implements_ac: ["AC#2"]
        - id: "SVC-003"
          description: "Use anyhow for top-level error handling with exit codes"
          testable: true
          test_requirement: "Test: Invalid arguments produce exit code 1"
          priority: "High"
          implements_ac: ["AC#5"]

    - type: "DataModel"
      name: "CLI Arguments"
      file_path: "src/cli/args.rs"
      requirements:
        - id: "DM-001"
          description: "Define Args struct with clap derive for root command"
          testable: true
          test_requirement: "Test: Args struct has #[derive(Parser)]"
          priority: "Critical"
          implements_ac: ["AC#2", "AC#3"]
        - id: "DM-002"
          description: "Define SearchArgs struct with all specified options"
          testable: true
          test_requirement: "Test: SearchArgs parses symbol, type, ignore_case, regex, format, context, signatures"
          priority: "Critical"
          implements_ac: ["AC#2"]
        - id: "DM-003"
          description: "Define SymbolType enum for --type flag"
          testable: true
          test_requirement: "Test: SymbolType has function, class, method, variable, constant, import, export variants"
          priority: "High"
          implements_ac: ["AC#2"]
        - id: "DM-004"
          description: "Define OutputFormat enum for --format flag"
          testable: true
          test_requirement: "Test: OutputFormat has json and text variants"
          priority: "High"
          implements_ac: ["AC#2"]

    - type: "Service"
      name: "Search Command Handler"
      file_path: "src/cli/commands/search.rs"
      requirements:
        - id: "SVC-004"
          description: "Accept SearchArgs and return Result with anyhow"
          testable: true
          test_requirement: "Test: Function signature is pub fn execute(args: SearchArgs) -> anyhow::Result<()>"
          priority: "Critical"
          implements_ac: ["AC#4"]
        - id: "SVC-005"
          description: "Output placeholder JSON when format is json"
          testable: true
          test_requirement: "Test: JSON output validates against expected schema with empty results"
          priority: "High"
          implements_ac: ["AC#4"]
        - id: "SVC-006"
          description: "Output placeholder text when format is text"
          testable: true
          test_requirement: "Test: Text output shows 'No results found for: {symbol}'"
          priority: "Medium"
          implements_ac: ["AC#4"]

    - type: "DataModel"
      name: "Error Types"
      file_path: "src/error.rs"
      requirements:
        - id: "DM-005"
          description: "Define TreelintError enum with thiserror derive"
          testable: true
          test_requirement: "Test: #[derive(thiserror::Error, Debug)] attribute present"
          priority: "Critical"
          implements_ac: ["AC#5"]
        - id: "DM-006"
          description: "Include Io variant wrapping std::io::Error"
          testable: true
          test_requirement: "Test: TreelintError::Io compiles and displays underlying error"
          priority: "High"
          implements_ac: ["AC#5"]
        - id: "DM-007"
          description: "Include Parse and Cli variants for future use"
          testable: true
          test_requirement: "Test: Enum has at least 3 variants: Io, Parse, Cli"
          priority: "Medium"
          implements_ac: ["AC#5"]

  business_rules:
    - id: "BR-001"
      rule: "Symbol argument must be non-empty UTF-8 string"
      test_requirement: "Test: treelint search '' returns error with 'Symbol cannot be empty'"
    - id: "BR-002"
      rule: "Invalid --type values must be rejected with enumerated valid values"
      test_requirement: "Test: treelint search foo --type invalid returns clap error listing valid types"
    - id: "BR-003"
      rule: "Exit code 0 for success, 1 for user errors, 2 for no results"
      test_requirement: "Test: Verify exit codes for success, bad args, and empty results scenarios"

  non_functional_requirements:
    - id: "NFR-001"
      category: "Performance"
      requirement: "Clean debug build completes within reasonable time"
      metric: "< 60 seconds on modern hardware (i7/M1 equivalent)"
      test_requirement: "Test: time cargo build --release < 60s"
    - id: "NFR-002"
      category: "Performance"
      requirement: "Binary startup is fast"
      metric: "< 50ms to parse arguments (no heavy initialization)"
      test_requirement: "Test: time treelint --help < 50ms"
    - id: "NFR-003"
      category: "Reliability"
      requirement: "Error messages are human-readable"
      metric: "All errors include actionable guidance"
      test_requirement: "Test: Error messages contain context via anyhow"
    - id: "NFR-004"
      category: "Maintainability"
      requirement: "Code passes linting without warnings"
      metric: "cargo clippy produces 0 warnings"
      test_requirement: "Test: cargo clippy -- -D warnings exits with code 0"
    - id: "NFR-005"
      category: "Maintainability"
      requirement: "Code is properly formatted"
      metric: "cargo fmt produces no changes"
      test_requirement: "Test: cargo fmt --check exits with code 0"
```

---

## Technical Limitations

```yaml
technical_limitations: []
# No limitations identified for CLI skeleton
```

---

## Non-Functional Requirements (NFRs)

### Performance

**Response Time:**
- **CLI startup:** < 50ms (p95)
- **Argument parsing:** < 10ms (p95)

**Build Time:**
- Clean debug build: < 60 seconds
- Incremental build: < 10 seconds

---

### Security

**Authentication:** Not applicable (local CLI tool)

**Data Protection:**
- No network access
- No credential storage
- Read-only file system access

---

### Maintainability

**Code Quality:**
- `cargo fmt --check` produces no changes
- `cargo clippy -- -D warnings` produces no warnings
- All public items have doc comments

**Module Structure:**
- Clear separation: cli/, error
- One responsibility per module

---

## Dependencies

### Prerequisite Stories

None - this is the foundation story.

### External Dependencies

- [ ] **Rust Toolchain:** 1.70.0+
  - **Owner:** Rust project
  - **Status:** Available
  - **Impact if unavailable:** Cannot compile

### Technology Dependencies

All dependencies from `devforgeai/specs/context/dependencies.md`:

- [ ] **clap:** 4.5 with derive, cargo features
  - **Purpose:** CLI argument parsing
  - **Approved:** Yes (dependencies.md)

- [ ] **serde:** 1.0 with derive feature
  - **Purpose:** Serialization framework
  - **Approved:** Yes (dependencies.md)

- [ ] **serde_json:** 1.0
  - **Purpose:** JSON output
  - **Approved:** Yes (dependencies.md)

- [ ] **thiserror:** 1.0
  - **Purpose:** Error type derive macros
  - **Approved:** Yes (dependencies.md)

- [ ] **anyhow:** 1.0
  - **Purpose:** Application error handling
  - **Approved:** Yes (dependencies.md)

---

## Test Strategy

### Unit Tests

**Coverage Target:** 80%+ for CLI skeleton

**Test Scenarios:**
1. **Happy Path:** Valid arguments parsed correctly
2. **Edge Cases:**
   - Empty symbol argument
   - All flags combined
   - Minimum arguments (just symbol)
3. **Error Cases:**
   - Invalid --type value
   - Invalid --format value
   - Missing required symbol

---

### Integration Tests

**Coverage Target:** 80%+ for CLI binary

**Test Scenarios:**
1. **Binary Execution:** `treelint search foo` runs and exits
2. **Help Text:** `treelint --help` displays usage
3. **JSON Output:** `treelint search foo --format json` returns valid JSON

---

## Acceptance Criteria Verification Checklist

**Purpose:** Real-time progress tracking during TDD implementation.

### AC#1: Cargo Project Compiles Successfully

- [x] Cargo.toml created with correct metadata - **Phase:** 2 - **Evidence:** Cargo.toml
- [x] All dependencies from dependencies.md included - **Phase:** 2 - **Evidence:** Cargo.toml
- [x] MSRV set to 1.70.0 - **Phase:** 2 - **Evidence:** Cargo.toml
- [x] `cargo build` succeeds - **Phase:** 3 - **Evidence:** CI output

### AC#2: Search Command Parses All Required Arguments

- [x] Args struct with Parser derive - **Phase:** 2 - **Evidence:** src/cli/args.rs
- [x] SearchArgs struct with all options - **Phase:** 2 - **Evidence:** src/cli/args.rs
- [x] SymbolType enum defined - **Phase:** 2 - **Evidence:** src/cli/args.rs
- [x] OutputFormat enum defined - **Phase:** 2 - **Evidence:** src/cli/args.rs
- [x] All flags parsed correctly - **Phase:** 3 - **Evidence:** test_ac2_argument_parsing.rs

### AC#3: Help Text Displays Complete CLI Documentation

- [x] Version from Cargo.toml displayed - **Phase:** 3 - **Evidence:** test_ac3_help_text.rs
- [x] All options documented - **Phase:** 3 - **Evidence:** test_ac3_help_text.rs
- [x] Usage examples present - **Phase:** 3 - **Evidence:** test_ac3_help_text.rs

### AC#4: Search Command Returns Placeholder JSON Output

- [x] execute() function implemented - **Phase:** 3 - **Evidence:** src/cli/commands/search.rs
- [x] JSON output matches schema - **Phase:** 3 - **Evidence:** test_ac4_placeholder_output.rs
- [x] Text output for terminal - **Phase:** 3 - **Evidence:** test_ac4_placeholder_output.rs
- [x] Exit code 0 on success - **Phase:** 3 - **Evidence:** test_ac4_placeholder_output.rs

### AC#5: Error Types Defined Using thiserror

- [x] TreelintError enum with thiserror - **Phase:** 2 - **Evidence:** src/error.rs
- [x] Io variant wrapping std::io::Error - **Phase:** 2 - **Evidence:** src/error.rs
- [x] Parse and Cli variants - **Phase:** 2 - **Evidence:** src/error.rs
- [x] Error display working - **Phase:** 3 - **Evidence:** test_ac5_error_types.rs

---

**Checklist Progress:** 20/20 items complete (100%)

---

## Definition of Done

### Implementation
- [x] Cargo.toml with package metadata and dependencies
- [x] src/main.rs with clap argument parsing
- [x] src/cli/args.rs with Args, SearchArgs, SymbolType, OutputFormat
- [x] src/cli/mod.rs with module exports
- [x] src/cli/commands/mod.rs with command routing
- [x] src/cli/commands/search.rs with placeholder execute()
- [x] src/error.rs with TreelintError enum
- [x] src/output/mod.rs with output module stub
- [x] src/output/json.rs with JSON placeholder output

### Quality
- [x] All 5 acceptance criteria have passing tests
- [x] Edge cases covered (empty symbol, invalid type, combined flags)
- [x] Exit codes documented (0, 1, 2)
- [x] Code coverage > 80% for src/cli/
- [x] `cargo clippy -- -D warnings` passes
- [x] `cargo fmt --check` passes

### Testing
- [x] Unit tests for argument parsing
- [x] Unit tests for error types
- [x] Integration tests for binary execution
- [x] Integration tests for help text
- [x] Integration tests for JSON output

### Documentation
- [x] All public items have `///` doc comments
- [x] Module-level `//!` comments present
- [x] README.md updated with basic usage

---

## Change Log

**Current Status:** Dev Complete

| Date | Author | Phase/Action | Change | Files Affected |
|------|--------|--------------|--------|----------------|
| 2026-01-27 14:30 | claude/story-requirements-analyst | Created | Story created from EPIC-001 F1.1 | STORY-001-project-setup-cli-skeleton.story.md |
| 2026-01-27 | claude/sprint-planner | Sprint Planning | Status: Backlog → Ready for Dev, Added to SPRINT-001 | STORY-001-project-setup-cli-skeleton.story.md |
| 2026-01-27 | claude/dev | Implementation | TDD workflow complete, 81 tests passing | Cargo.toml, src/*, tests/* |

## Implementation Notes

- Created Cargo.toml with all dependencies from dependencies.md
- Implemented CLI entry point with clap Parser derive
- Created SearchArgs struct with all required options (symbol, type, ignore-case, regex, format, context, signatures)
- Created SymbolType enum with 7 variants (function, class, method, variable, constant, import, export)
- Created OutputFormat enum (json, text)
- Implemented placeholder execute() returning empty results JSON
- Implemented TreelintError enum with thiserror (Io, Parse, Cli variants)
- 81 integration tests covering all 5 acceptance criteria
- clippy and fmt checks passing

## Notes

**Design Decisions:**
- Using clap derive macros for declarative CLI definition (per tech-stack.md)
- Placeholder output returns empty results (actual search logic in STORY-002+)
- Error types defined early for consistent error handling throughout project

**File Structure (per source-tree.md):**
```
src/
├── main.rs
├── cli/
│   ├── mod.rs
│   ├── args.rs
│   └── commands/
│       ├── mod.rs
│       └── search.rs
├── output/
│   ├── mod.rs
│   └── json.rs
└── error.rs
```

**References:**
- EPIC-001: Core CLI Foundation
- BRAINSTORM-001: Treelint AST-Aware Code Search
- tech-stack.md: Approved technologies
- dependencies.md: Cargo dependencies
- source-tree.md: File structure

---

Story Template Version: 2.7
Last Updated: 2026-01-27
