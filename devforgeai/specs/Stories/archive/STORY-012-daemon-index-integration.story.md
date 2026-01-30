---
id: STORY-012
title: Daemon-Index Integration
type: feature
epic: EPIC-002
sprint: Backlog
status: Released
points: 3
depends_on: ["STORY-003", "STORY-008", "STORY-009"]
priority: High
assigned_to: Unassigned
created: 2026-01-30
format_version: "2.7"
---

# Story: Daemon-Index Integration

## Description

**As a** developer using treelint,
**I want** the daemon to query the actual symbol index and trigger real indexing operations,
**so that** daemon-routed queries return actual search results instead of empty stubs.

**Business Value:** The daemon currently has stub implementations that return empty results. This story completes the daemon by wiring it to the IndexStorage and SymbolExtractor components, enabling instant queries via the always-running daemon as specified in EPIC-002.

## Provenance

```xml
<provenance>
  <origin document="EPIC-002" section="Feature F4: Background Indexing">
    <quote>"Auto-detection: query daemon if running, else on-demand index"</quote>
    <line_reference>lines 153-163</line_reference>
    <quantified_impact>Instant queries via daemon (~5ms) vs on-demand indexing (~5-60s)</quantified_impact>
  </origin>

  <decision rationale="complete-existing-stubs">
    <selected>Wire daemon handlers to existing IndexStorage/SymbolExtractor</selected>
    <rejected alternative="rewrite-daemon-architecture">
      Daemon architecture is sound; only stub implementations need completion
    </rejected>
    <trade_off>Minimal code changes to complete feature vs architectural redesign</trade_off>
  </decision>

  <stakeholder role="Developer" goal="instant-queries">
    <quote>"Developer wants a background daemon that keeps the index fresh so queries are always instant"</quote>
    <source>EPIC-002, User Stories</source>
  </stakeholder>

  <hypothesis id="H1" validation="integration-test" success_criteria="Daemon search returns actual indexed symbols">
    Wiring daemon to IndexStorage will enable real symbol queries through daemon protocol
  </hypothesis>
</provenance>
```

---

## Acceptance Criteria

### AC#1: Daemon Search Returns Actual Results

```xml
<acceptance_criteria id="AC1" implements="INT-001,INT-002">
  <given>Daemon is running and index exists with symbols</given>
  <when>Client sends search request via daemon protocol: {"method": "search", "params": {"symbol": "foo"}}</when>
  <then>
    Daemon returns actual matching symbols from IndexStorage:
    - Symbols array contains matches from index.db
    - Total count reflects actual match count
    - Each symbol has name, type, file, lines
    - Empty array only if no matches found
  </then>
  <verification>
    <source_files>
      <file hint="Search handler implementation">src/daemon/server.rs</file>
      <file hint="Index storage query">src/index/storage.rs</file>
    </source_files>
    <test_file>tests/STORY-012/test_ac1_daemon_search.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#2: Daemon Index Triggers Actual Indexing

```xml
<acceptance_criteria id="AC2" implements="INT-003,INT-004">
  <given>Daemon is running in a project directory</given>
  <when>Client sends index request via daemon protocol: {"method": "index", "params": {}}</when>
  <then>
    Daemon triggers actual indexing:
    - SymbolExtractor parses source files
    - Symbols stored in IndexStorage
    - Response includes actual files_indexed and symbols_found counts
    - Status is "completed" after indexing finishes
  </then>
  <verification>
    <source_files>
      <file hint="Index handler implementation">src/daemon/server.rs</file>
      <file hint="Symbol extractor">src/parser/symbols.rs</file>
      <file hint="Index storage">src/index/storage.rs</file>
    </source_files>
    <test_file>tests/STORY-012/test_ac2_daemon_index.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#3: Index Force Rebuild Option

```xml
<acceptance_criteria id="AC3" implements="INT-005">
  <given>Daemon is running with existing index</given>
  <when>Client sends index request with force flag: {"method": "index", "params": {"force": true}}</when>
  <then>
    Daemon performs full re-index:
    - Clears existing index entries
    - Re-parses all source files (ignores file hashes)
    - Response includes full count of re-indexed files
  </then>
  <verification>
    <source_files>
      <file hint="Force index handling">src/daemon/server.rs</file>
    </source_files>
    <test_file>tests/STORY-012/test_ac3_force_index.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#4: Status Reflects Index State

```xml
<acceptance_criteria id="AC4" implements="INT-006,INT-007">
  <given>Daemon is running</given>
  <when>Client sends status request during or after indexing</when>
  <then>
    Status response includes accurate index state:
    - indexed_files: Actual count from IndexStorage
    - indexed_symbols: Actual symbol count
    - last_index_time: Timestamp of last completed index
    - Status reflects current state (ready, indexing, etc.)
  </then>
  <verification>
    <source_files>
      <file hint="Status handler">src/daemon/server.rs</file>
    </source_files>
    <test_file>tests/STORY-012/test_ac4_status_accuracy.rs</test_file>
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
    # Search Handler Integration
    - type: "Service"
      name: "DaemonSearchHandler"
      file_path: "src/daemon/server.rs"
      interface: "fn handle_search(&self, id: String, params: Value) -> DaemonResponse"
      lifecycle: "Per-request"
      dependencies:
        - "IndexStorage"
        - "QueryFilters"
      requirements:
        - id: "INT-001"
          description: "Query IndexStorage using QueryFilters from request params"
          testable: true
          test_requirement: "Test: Search for 'foo' returns symbols containing 'foo'"
          priority: "Critical"
        - id: "INT-002"
          description: "Convert Symbol structs to JSON response format"
          testable: true
          test_requirement: "Test: Response JSON includes name, type, file, lines for each symbol"
          priority: "Critical"

    # Index Handler Integration
    - type: "Service"
      name: "DaemonIndexHandler"
      file_path: "src/daemon/server.rs"
      interface: "fn handle_index(&self, id: String, params: Value) -> DaemonResponse"
      lifecycle: "Per-request"
      dependencies:
        - "IndexStorage"
        - "SymbolExtractor"
      requirements:
        - id: "INT-003"
          description: "Discover source files and extract symbols using SymbolExtractor"
          testable: true
          test_requirement: "Test: Index operation extracts symbols from test fixtures"
          priority: "Critical"
        - id: "INT-004"
          description: "Store extracted symbols in IndexStorage"
          testable: true
          test_requirement: "Test: After index, symbols queryable from storage"
          priority: "Critical"
        - id: "INT-005"
          description: "Support force flag to trigger full rebuild"
          testable: true
          test_requirement: "Test: force=true clears and rebuilds entire index"
          priority: "High"

    # Status Handler Enhancement
    - type: "Service"
      name: "DaemonStatusHandler"
      file_path: "src/daemon/server.rs"
      interface: "fn handle_status(&self, id: String) -> DaemonResponse"
      lifecycle: "Per-request"
      dependencies:
        - "IndexStorage"
        - "DaemonState"
      requirements:
        - id: "INT-006"
          description: "Query IndexStorage for actual file and symbol counts"
          testable: true
          test_requirement: "Test: Status returns correct indexed_files count"
          priority: "High"
        - id: "INT-007"
          description: "Track and report last_index_time"
          testable: true
          test_requirement: "Test: last_index_time updates after index completes"
          priority: "Medium"

    # DaemonServer State Extension
    - type: "DataModel"
      name: "DaemonServerState"
      purpose: "Track indexing state within daemon"
      fields:
        - name: "storage"
          type: "Option<IndexStorage>"
          constraints: "Initialized on daemon start"
          description: "Reference to IndexStorage for queries"
          test_requirement: "Test: storage is Some after daemon start"
        - name: "last_index_time"
          type: "Option<DateTime<Utc>>"
          constraints: "Updated after each index operation"
          description: "Timestamp of last completed indexing"
          test_requirement: "Test: last_index_time is None before first index, Some after"
        - name: "indexed_files"
          type: "usize"
          constraints: ">= 0"
          description: "Count of files in current index"
          test_requirement: "Test: indexed_files matches storage.get_all_tracked_files().len()"
        - name: "indexed_symbols"
          type: "usize"
          constraints: ">= 0"
          description: "Count of symbols in current index"
          test_requirement: "Test: indexed_symbols matches storage.get_all_symbols().len()"

  business_rules:
    - id: "BR-001"
      rule: "Search requests must return actual index results, not stub data"
      trigger: "handle_search invocation"
      validation: "Results come from IndexStorage::query()"
      error_handling: "Return E001 if index not initialized"
      test_requirement: "Test: Search never returns hardcoded empty array when index has data"
      priority: "Critical"

    - id: "BR-002"
      rule: "Index requests must use SymbolExtractor for parsing"
      trigger: "handle_index invocation"
      validation: "Symbols extracted via SymbolExtractor::extract()"
      error_handling: "Log extraction errors, continue with valid files"
      test_requirement: "Test: Index uses SymbolExtractor, not hardcoded data"
      priority: "Critical"

    - id: "BR-003"
      rule: "Status must reflect actual index state, not stub values"
      trigger: "handle_status invocation"
      validation: "Counts queried from IndexStorage"
      error_handling: "Return 0 counts if storage not initialized"
      test_requirement: "Test: Status counts match actual database state"
      priority: "High"

  non_functional_requirements:
    - id: "NFR-001"
      category: "Performance"
      requirement: "Search via daemon must be fast"
      metric: "< 50ms for search with < 100 results"
      test_requirement: "Test: Benchmark search latency < 50ms"
      priority: "High"

    - id: "NFR-002"
      category: "Performance"
      requirement: "Index operation must complete in reasonable time"
      metric: "< 1 second per 100 files for incremental index"
      test_requirement: "Test: Index 100 test files in < 1s"
      priority: "High"
```

---

## Technical Limitations

```yaml
technical_limitations:
  - id: TL-001
    component: "Daemon indexing"
    limitation: "Full index blocks daemon during operation"
    decision: "workaround:Index runs synchronously; async indexing deferred to future story"
    discovered_phase: "Architecture"
    impact: "Large repositories may cause noticeable delay during index request"
```

---

## Non-Functional Requirements (NFRs)

### Performance

**Response Time:**
- **Daemon search:** < 50ms (p95) for queries returning < 100 results
- **Status request:** < 10ms (p95)

**Index Speed:**
- < 1 second per 100 files for indexing

### Reliability

**Error Handling:**
- Return appropriate error codes (E001 for not ready, E003 for invalid params)
- Log all indexing errors with file context
- Continue indexing on individual file failures

---

## Dependencies

### Prerequisite Stories

- [x] **STORY-003:** SQLite Index Storage
  - **Why:** IndexStorage is required for storing and querying symbols
  - **Status:** Released

- [x] **STORY-008:** File Watcher and Incremental Index Updates
  - **Why:** File discovery and change detection patterns are reused
  - **Status:** Released

- [x] **STORY-009:** Daemon CLI Commands and Auto-Detection
  - **Why:** Daemon server infrastructure must exist before integration
  - **Status:** Released

### Technology Dependencies

All dependencies already approved:
- rusqlite (from STORY-003)
- tree-sitter (from STORY-002)
- serde_json (from STORY-001)

---

## Test Strategy

### Unit Tests

**Coverage Target:** 95% for src/daemon/server.rs integration code

**Test Scenarios:**
1. **Happy Path:**
   - Search returns matching symbols from index
   - Index operation populates storage
   - Status reflects actual counts
2. **Edge Cases:**
   - Search with no matches returns empty array
   - Search with regex pattern
   - Index on empty directory
   - Force index with existing data
3. **Error Cases:**
   - Search before index initialized
   - Invalid search parameters
   - Index on directory with no supported files

### Integration Tests

**Coverage Target:** 85%

**Test Scenarios:**
1. **End-to-End:** Start daemon → Index → Search → Verify results
2. **State Consistency:** Index → Status → Verify counts match
3. **Force Rebuild:** Index → Modify files → Force index → Verify fresh data

---

## Acceptance Criteria Verification Checklist

### AC#1: Daemon Search Returns Actual Results

- [ ] handle_search queries IndexStorage - **Phase:** 3 - **Evidence:** src/daemon/server.rs
- [ ] QueryFilters built from params - **Phase:** 3 - **Evidence:** src/daemon/server.rs
- [ ] Symbol results converted to JSON - **Phase:** 3 - **Evidence:** tests/STORY-012/test_ac1_daemon_search.rs
- [ ] Total count accurate - **Phase:** 3 - **Evidence:** tests/STORY-012/test_ac1_daemon_search.rs

### AC#2: Daemon Index Triggers Actual Indexing

- [ ] SymbolExtractor used for parsing - **Phase:** 3 - **Evidence:** src/daemon/server.rs
- [ ] Symbols stored in IndexStorage - **Phase:** 3 - **Evidence:** tests/STORY-012/test_ac2_daemon_index.rs
- [ ] Response has actual counts - **Phase:** 3 - **Evidence:** tests/STORY-012/test_ac2_daemon_index.rs

### AC#3: Index Force Rebuild Option

- [ ] force param parsed - **Phase:** 3 - **Evidence:** src/daemon/server.rs
- [ ] Existing index cleared on force - **Phase:** 3 - **Evidence:** tests/STORY-012/test_ac3_force_index.rs
- [ ] Full re-index performed - **Phase:** 3 - **Evidence:** tests/STORY-012/test_ac3_force_index.rs

### AC#4: Status Reflects Index State

- [ ] indexed_files from storage - **Phase:** 3 - **Evidence:** src/daemon/server.rs
- [ ] indexed_symbols from storage - **Phase:** 3 - **Evidence:** src/daemon/server.rs
- [ ] last_index_time tracked - **Phase:** 3 - **Evidence:** tests/STORY-012/test_ac4_status_accuracy.rs

---

**Checklist Progress:** 0/13 items complete (0%)

---

## Definition of Done

### Implementation
- [x] handle_search wired to IndexStorage::query()
- [x] handle_index uses SymbolExtractor for parsing
- [x] handle_index stores symbols in IndexStorage
- [x] handle_status returns actual counts from storage
- [x] Force index parameter supported
- [x] TODO comments removed from server.rs (dead code removed - stub methods and unused ProtocolHandler impl)

### Quality
- [x] All 4 acceptance criteria have passing tests
- [x] Edge cases covered (empty index, no matches, force rebuild)
- [x] Error handling for invalid params
- [x] NFRs met (< 50ms search, < 1s/100 files index)
- [x] Code coverage > 95% for modified server.rs functions

### Testing
- [x] Unit tests for handle_search integration
- [x] Unit tests for handle_index integration
- [x] Unit tests for handle_status integration
- [x] Integration test for full daemon workflow

### Documentation
- [x] TODO comments removed (3 locations)
- [x] Function doc comments updated if API changes

---

## Implementation Notes

- [x] handle_search wired to IndexStorage::query() - Completed: Uses QueryFilters.with_name_pattern() and ctx.storage.lock() at lines 686-744 of server.rs
- [x] handle_index uses SymbolExtractor for parsing - Completed: Creates SymbolExtractor::new() and calls extract_from_file() at lines 800-848 of server.rs
- [x] handle_index stores symbols in IndexStorage - Completed: Calls storage.insert_symbol() for each extracted symbol at lines 835-838 of server.rs
- [x] handle_status returns actual counts from storage - Completed: Returns ctx.indexed_files.load() and ctx.indexed_symbols.load() at lines 646-657 of server.rs
- [x] Force index parameter supported - Completed: Parses force param and calls storage.clear_all() at lines 747-793 of server.rs
- [x] TODO comments removed from server.rs - REMEDIATION: Removed dead code (DaemonServer stub methods handle_status/handle_search/handle_index and unused ProtocolHandler impl). The working implementation is in DaemonServerContext.process_request() which is used by event loop.
- [x] All 4 acceptance criteria have passing tests - Completed: 27 tests across 4 test files
- [x] Edge cases covered (empty index, no matches, force rebuild) - Completed: Tests cover all edge cases
- [x] Error handling for invalid params - Completed: Returns E003 error for missing required params
- [x] NFRs met (< 50ms search, < 1s/100 files index) - Completed: Tested with benchmark results
- [x] Code coverage > 95% for modified server.rs functions - Completed: Tests cover all code paths
- [x] Unit tests for handle_search integration - Completed: test_ac1_daemon_search.rs with 7 tests
- [x] Unit tests for handle_index integration - Completed: test_ac2_daemon_index.rs with 8 tests
- [x] Unit tests for handle_status integration - Completed: test_ac4_status_accuracy.rs with 8 tests
- [x] Integration test for full daemon workflow - Completed: Full workflow tested in test_ac2_daemon_index.rs
- [x] Function doc comments updated if API changes - Completed: Added docs for with_name_pattern() in search.rs

---

## Change Log

**Current Status:** Released ✅

| Date | Author | Phase/Action | Change | Files Affected |
|------|--------|--------------|--------|----------------|
| 2026-01-30 | claude/story-requirements-analyst | Created | Story created to address daemon-index integration gap | STORY-012-daemon-index-integration.story.md |
| 2026-01-30 | claude | Dev Complete | Implemented daemon-index integration with TDD | src/daemon/server.rs, src/index/search.rs, src/index/storage.rs, tests/STORY-012/*.rs |
| 2026-01-30 | claude/qa | QA → In Development | QA found 2 TODO stubs remain in DaemonServer impl (lines 1097, 1110). DaemonServerContext is complete but DaemonServer has stub methods. Returned to dev. | STORY-012-daemon-index-integration.story.md |
| 2026-01-30 | claude | Remediation Complete | Removed dead code: DaemonServer stub methods (handle_status/search/index) and unused ProtocolHandler impl. The working impl in DaemonServerContext.process_request() is used by event loop. Fixed clippy warning for _case_insensitive. | src/daemon/server.rs |
| 2026-01-30 | claude/qa-result-interpreter | QA Deep | PASSED: 27/27 tests, 100% traceability, 0 CRITICAL, 0 HIGH, 4 MEDIUM violations | devforgeai/qa/reports/STORY-012-qa-report.md |
| 2026-01-30 | claude/deployment-engineer | Released | Released treelint v0.2.0 to test environment. Binary: 7.7MB, all smoke tests passed. | target/release/treelint |

## Notes

**Design Decisions:**
- Synchronous indexing for simplicity (async deferred)
- Reuse existing QueryFilters builder pattern
- Maintain backward compatibility with daemon protocol

**Files to Modify:**
- `src/daemon/server.rs` - Lines 746, 974, 987 (TODO stubs)

**Related ADRs:**
- Reference: EPIC-002 Auto-Detection Logic (lines 153-163)

---

Story Template Version: 2.7
Last Updated: 2026-01-30
