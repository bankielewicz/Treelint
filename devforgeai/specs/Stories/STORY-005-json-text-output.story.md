---
id: STORY-005
title: JSON/Text Output
type: feature
epic: EPIC-001
sprint: SPRINT-001
status: Ready for Dev
points: 5
depends_on: ["STORY-004"]
priority: High
assigned_to: Unassigned
created: 2026-01-27
format_version: "2.7"
---

# Story: JSON/Text Output

## Description

**As a** developer integrating Treelint with AI coding assistants,
**I want** search results formatted as structured JSON for programmatic consumption and human-readable text for terminal use with automatic TTY detection,
**so that** AI assistants receive optimized token-efficient data while human users see easily scannable output without manual format switching.

## Provenance

```xml
<provenance>
  <origin document="BRAINSTORM-001" section="output-format-specification">
    <quote>"JSON Schema (Recommended) with query metadata, results array, and stats"</quote>
    <line_reference>lines 244-273</line_reference>
    <quantified_impact>40-80% token reduction vs raw grep output</quantified_impact>
  </origin>

  <decision rationale="json-for-ai-text-for-humans">
    <selected>Dual format with TTY auto-detection - best of both worlds</selected>
    <rejected alternative="JSON only">
      Poor human UX for terminal exploration
    </rejected>
    <rejected alternative="Text only">
      Requires AI to parse unstructured text, wastes tokens
    </rejected>
    <trade_off>Code complexity vs user experience</trade_off>
  </decision>

  <stakeholder role="AI Coding Assistant" goal="token-efficiency">
    <quote>"JSON output with symbol metadata so that I can programmatically select relevant context"</quote>
    <source>EPIC-001, User Stories</source>
  </stakeholder>

  <stakeholder role="Developer" goal="terminal-usability">
    <quote>"Human-readable terminal output so that I can use Treelint for manual code exploration"</quote>
    <source>EPIC-001, User Stories</source>
  </stakeholder>
</provenance>
```

---

## Acceptance Criteria

### AC#1: JSON Output Format Matches Schema

```xml
<acceptance_criteria id="AC1" implements="SVC-001,SVC-002,SVC-003">
  <given>A search returns one or more symbol results</given>
  <when>treelint search validateUser --format json is executed</when>
  <then>Output is valid JSON with: query object (symbol, type, case_insensitive, regex, context_mode), results array (type, name, file, lines, signature, body, language per item), metadata object (files_searched, files_skipped, skipped_by_type, languages_searched, elapsed_ms)</then>
  <verification>
    <source_files>
      <file hint="JSON serialization">src/output/json.rs</file>
    </source_files>
    <test_file>tests/STORY-005/test_ac1_json_format.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#2: Text Output Format for Human Readability

```xml
<acceptance_criteria id="AC2" implements="SVC-004,SVC-005,SVC-006,SVC-007">
  <given>A search returns one or more symbol results</given>
  <when>treelint search validateUser --format text is executed</when>
  <then>Output displays: header line with name/type/file/lines, tree structure with signature, body indented 4 spaces, summary line with count/elapsed_ms/files stats, colors when TTY (cyan function names, green paths, yellow line numbers)</then>
  <verification>
    <source_files>
      <file hint="Text formatting">src/output/text.rs</file>
    </source_files>
    <test_file>tests/STORY-005/test_ac2_text_format.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#3: TTY Auto-Detection Selects Appropriate Format

```xml
<acceptance_criteria id="AC3" implements="SVC-008,SVC-009">
  <given>No --format flag is provided</given>
  <when>treelint search validateUser is executed</when>
  <then>If stdout is TTY: text format with colors; if stdout is piped/redirected: JSON format without colors; detection uses atty::is(atty::Stream::Stdout)</then>
  <verification>
    <source_files>
      <file hint="TTY detection and routing">src/output/mod.rs</file>
    </source_files>
    <test_file>tests/STORY-005/test_ac3_tty_detection.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#4: Format Flag Overrides Auto-Detection

```xml
<acceptance_criteria id="AC4" implements="SVC-009">
  <given>The --format flag is specified</given>
  <when>treelint search validateUser --format json is executed in a TTY terminal</when>
  <then>JSON output is produced regardless of TTY status; --format text | cat produces text without colors</then>
  <verification>
    <source_files>
      <file hint="Format flag handling">src/output/mod.rs</file>
    </source_files>
    <test_file>tests/STORY-005/test_ac4_format_override.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#5: Signatures-Only Mode Minimizes Token Output

```xml
<acceptance_criteria id="AC5" implements="SVC-010">
  <given>The --signatures flag is specified</given>
  <when>treelint search validateUser --signatures is executed</when>
  <then>JSON results have body as null/omitted, text shows only header and signature (no body), output size reduced 60-80%, context_mode shows "signatures"</then>
  <verification>
    <source_files>
      <file hint="Signatures mode">src/output/json.rs</file>
      <file hint="Signatures mode">src/output/text.rs</file>
    </source_files>
    <test_file>tests/STORY-005/test_ac5_signatures_mode.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

## Technical Specification

```yaml
technical_specification:
  format_version: "2.0"
  story_id: "STORY-005"

  components:
    - type: "Service"
      name: "JsonFormatter"
      file_path: "src/output/json.rs"
      requirements:
        - id: "SVC-001"
          description: "Serialize SearchResult to JSON matching EPIC-001 schema"
          testable: true
          test_requirement: "Test: JSON output validates against schema"
          priority: "Critical"
          implements_ac: ["AC#1"]
        - id: "SVC-002"
          description: "Handle all field types (Option becomes null, Vec becomes array)"
          testable: true
          test_requirement: "Test: Null body serializes as JSON null"
          priority: "High"
          implements_ac: ["AC#1"]
        - id: "SVC-003"
          description: "Escape special characters per RFC 8259"
          testable: true
          test_requirement: "Test: Backslash in path serializes as \\\\"
          priority: "High"
          implements_ac: ["AC#1"]

    - type: "Service"
      name: "TextFormatter"
      file_path: "src/output/text.rs"
      requirements:
        - id: "SVC-004"
          description: "Format results with tree-style layout"
          testable: true
          test_requirement: "Test: Output matches EPIC-001 text format spec"
          priority: "Critical"
          implements_ac: ["AC#2"]
        - id: "SVC-005"
          description: "Apply colors only when stdout is TTY"
          testable: true
          test_requirement: "Test: Piped output has no ANSI escape codes"
          priority: "High"
          implements_ac: ["AC#2"]
        - id: "SVC-006"
          description: "Indent body content with 4 spaces per level"
          testable: true
          test_requirement: "Test: Multi-line body has 4-space indentation"
          priority: "Medium"
          implements_ac: ["AC#2"]
        - id: "SVC-007"
          description: "Generate summary line with accurate statistics"
          testable: true
          test_requirement: "Test: Summary shows correct counts and elapsed_ms"
          priority: "High"
          implements_ac: ["AC#2"]

    - type: "Service"
      name: "OutputRouter"
      file_path: "src/output/mod.rs"
      requirements:
        - id: "SVC-008"
          description: "Detect TTY status using atty::is(atty::Stream::Stdout)"
          testable: true
          test_requirement: "Test: TTY detection works for terminal vs pipe"
          priority: "Critical"
          implements_ac: ["AC#3"]
        - id: "SVC-009"
          description: "Select formatter based on --format flag or TTY"
          testable: true
          test_requirement: "Test: No flag+TTY=text, no flag+pipe=JSON, flag overrides"
          priority: "Critical"
          implements_ac: ["AC#3", "AC#4"]
        - id: "SVC-010"
          description: "Handle --signatures flag to reduce output"
          testable: true
          test_requirement: "Test: --signatures omits body in both formats"
          priority: "High"
          implements_ac: ["AC#5"]

    - type: "DataModel"
      name: "SearchOutput"
      file_path: "src/output/mod.rs"
      requirements:
        - id: "DM-001"
          description: "Define SearchOutput struct with query, results, metadata"
          testable: true
          test_requirement: "Test: SearchOutput can be constructed from search results"
          priority: "Critical"
          implements_ac: ["AC#1"]
        - id: "DM-002"
          description: "Derive Serialize for JSON output"
          testable: true
          test_requirement: "Test: serde_json::to_string(&output) succeeds"
          priority: "Critical"
          implements_ac: ["AC#1"]

  business_rules:
    - id: "BR-001"
      rule: "JSON output must be valid per RFC 8259"
      test_requirement: "Test: All JSON output parses without error"
    - id: "BR-002"
      rule: "Colors only when stdout is TTY"
      test_requirement: "Test: Piped output has zero ANSI codes"
    - id: "BR-003"
      rule: "Errors to stderr, results to stdout"
      test_requirement: "Test: Error messages on stderr only"

  non_functional_requirements:
    - id: "NFR-001"
      category: "Performance"
      requirement: "JSON serialization under 5ms"
      metric: "< 5ms for 100 results (p95)"
      test_requirement: "Test: Benchmark JSON serialization < 5ms"
    - id: "NFR-002"
      category: "Performance"
      requirement: "Text formatting under 10ms"
      metric: "< 10ms for 100 results (p95)"
      test_requirement: "Test: Benchmark text formatting < 10ms"
    - id: "NFR-003"
      category: "Reliability"
      requirement: "Graceful color fallback"
      metric: "No panic if terminal detection fails"
      test_requirement: "Test: Plain text produced on detection error"
    - id: "NFR-004"
      category: "Scalability"
      requirement: "Handle large result sets"
      metric: "10,000+ results without OOM"
      test_requirement: "Test: Format 10K results < 50MB memory"
```

---

## Technical Limitations

```yaml
technical_limitations: []
# serde_json and colored crates handle all requirements
```

---

## Non-Functional Requirements (NFRs)

### Performance

**Serialization:**
- JSON: < 5ms for 100 results (p95)
- Text: < 10ms for 100 results (p95)

**Memory:**
- < 50MB for formatting 10,000 results

---

### Reliability

**Graceful Degradation:**
- Color detection failure → plain text (no panic)
- Atomic JSON output (no partial on interrupt)

**Error Separation:**
- Errors → stderr
- Results → stdout

---

## Dependencies

### Prerequisite Stories

- [x] **STORY-004:** Search Command Logic (provides search results)

### Technology Dependencies

- [ ] **serde_json:** 1.0 - JSON serialization
- [ ] **colored:** 2.1 - Terminal colors
- [ ] **atty:** 0.2 - TTY detection

All approved in dependencies.md.

---

## Test Strategy

### Unit Tests

**Coverage Target:** 95%+ for output module

**Test Scenarios:**
1. **Happy Path:** JSON format, text format, TTY detection
2. **Edge Cases:**
   - Empty results
   - Unicode in names/paths
   - Very large bodies
   - Special characters
3. **Error Cases:**
   - Invalid format flag
   - Color fallback

---

## Acceptance Criteria Verification Checklist

### AC#1: JSON Output Format

- [ ] SearchOutput struct defined - **Phase:** 2 - **Evidence:** src/output/mod.rs
- [ ] JSON serialization works - **Phase:** 3 - **Evidence:** test_ac1
- [ ] Schema matches EPIC-001 spec - **Phase:** 3 - **Evidence:** test_ac1
- [ ] Special chars escaped - **Phase:** 3 - **Evidence:** test_ac1

### AC#2: Text Output Format

- [ ] Header line format correct - **Phase:** 3 - **Evidence:** test_ac2
- [ ] Tree structure with signature - **Phase:** 3 - **Evidence:** test_ac2
- [ ] Body indentation (4 spaces) - **Phase:** 3 - **Evidence:** test_ac2
- [ ] Summary line accurate - **Phase:** 3 - **Evidence:** test_ac2
- [ ] Colors when TTY - **Phase:** 3 - **Evidence:** test_ac2

### AC#3: TTY Auto-Detection

- [ ] atty detection implemented - **Phase:** 3 - **Evidence:** src/output/mod.rs
- [ ] TTY → text with colors - **Phase:** 3 - **Evidence:** test_ac3
- [ ] Pipe → JSON no colors - **Phase:** 3 - **Evidence:** test_ac3

### AC#4: Format Flag Override

- [ ] --format json overrides TTY - **Phase:** 3 - **Evidence:** test_ac4
- [ ] --format text works when piped - **Phase:** 3 - **Evidence:** test_ac4

### AC#5: Signatures-Only Mode

- [ ] JSON body null with --signatures - **Phase:** 3 - **Evidence:** test_ac5
- [ ] Text omits body section - **Phase:** 3 - **Evidence:** test_ac5
- [ ] context_mode shows "signatures" - **Phase:** 3 - **Evidence:** test_ac5

---

**Checklist Progress:** 0/18 items complete (0%)

---

## Definition of Done

### Implementation
- [ ] src/output/mod.rs with OutputRouter, SearchOutput, format selection
- [ ] src/output/json.rs with JsonFormatter
- [ ] src/output/text.rs with TextFormatter
- [ ] TTY detection using atty
- [ ] Color support using colored (TTY only)
- [ ] --signatures flag handling

### Quality
- [ ] All 5 acceptance criteria have passing tests
- [ ] Edge cases covered (empty, unicode, large bodies)
- [ ] JSON validates against EPIC-001 schema
- [ ] Code coverage > 95% for src/output/
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo fmt --check` passes

### Testing
- [ ] Unit tests for JSON format
- [ ] Unit tests for text format
- [ ] Unit tests for TTY detection
- [ ] Unit tests for format override
- [ ] Unit tests for signatures mode
- [ ] Benchmark: JSON < 5ms, text < 10ms

### Documentation
- [ ] All public items have `///` doc comments
- [ ] Module-level `//!` comments for output module
- [ ] JSON schema documented inline

---

## Change Log

**Current Status:** Ready for Dev

| Date | Author | Phase/Action | Change | Files Affected |
|------|--------|--------------|--------|----------------|
| 2026-01-27 16:30 | claude/story-requirements-analyst | Created | Story created from EPIC-001 F2 | STORY-005-json-text-output.story.md |
| 2026-01-27 | claude/sprint-planner | Sprint Planning | Status: Backlog → Ready for Dev, Added to SPRINT-001 | STORY-005-json-text-output.story.md |

## Notes

**Design Decisions:**
- TTY auto-detection provides best UX for both AI and human users
- JSON when piped ensures AI tools always get structured data
- Colors disabled when not TTY to avoid ANSI codes in files/pipes

**JSON Schema Reference:**
- EPIC-001, lines 121-156

**Text Format Reference:**
- EPIC-001, lines 158-171

**References:**
- EPIC-001: Core CLI Foundation
- BRAINSTORM-001: Output Format Specification
- tech-stack.md: serde_json, colored, atty approved
- dependencies.md: All crates approved

---

Story Template Version: 2.7
Last Updated: 2026-01-27
