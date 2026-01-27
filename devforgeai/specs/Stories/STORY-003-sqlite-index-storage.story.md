---
id: STORY-003
title: SQLite Index Storage
type: feature
epic: EPIC-001
sprint: SPRINT-001
status: Ready for Dev
points: 3
depends_on: ["STORY-002"]
priority: Critical
assigned_to: Unassigned
created: 2026-01-27
format_version: "2.7"
---

# Story: SQLite Index Storage

## Description

**As a** code search index manager,
**I want** to persist extracted symbols in a SQLite database with WAL mode for concurrent access,
**so that** subsequent searches can query the index instantly without re-parsing files, reducing query latency from seconds to under 50ms.

## Provenance

```xml
<provenance>
  <origin document="BRAINSTORM-001" section="indexing-architecture">
    <quote>"Background indexing daemon with incremental re-index modified files only"</quote>
    <line_reference>lines 296-354</line_reference>
    <quantified_impact>Query latency < 100ms for typical searches</quantified_impact>
  </origin>

  <decision rationale="sqlite-over-alternatives">
    <selected>SQLite with rusqlite bundled - portable single-file database, zero config, WAL mode</selected>
    <rejected alternative="PostgreSQL">
      Requires server installation, overkill for local tool
    </rejected>
    <rejected alternative="RocksDB">
      Too complex for our needs, less mature Rust bindings
    </rejected>
    <trade_off>Single-file simplicity vs distributed scalability</trade_off>
  </decision>

  <stakeholder role="AI Coding Assistant" goal="instant-queries">
    <quote>"Token efficiency, accuracy, speed"</quote>
    <source>BRAINSTORM-001, Stakeholder Analysis</source>
  </stakeholder>

  <hypothesis id="H2" validation="a-b-comparison" success_criteria="Measured in Claude Code sessions">
    JSON output reduces tokens by 40%+ vs grep
  </hypothesis>
</provenance>
```

---

## Acceptance Criteria

### AC#1: Database Initialization and Schema Creation

```xml
<acceptance_criteria id="AC1" implements="CFG-001,CFG-002,CFG-003,SVC-001,SVC-002">
  <given>The Treelint CLI starts and no .treelint/index.db exists</given>
  <when>The storage layer initializes</when>
  <then>The database is created at .treelint/index.db with: symbols table (id, name, type, visibility, file_path, line_start, line_end, signature, body, language, updated_at), files table (path, language, hash, indexed_at), metadata table (key, value), indexes (idx_symbols_name, idx_symbols_type, idx_symbols_file), PRAGMA settings (journal_mode=WAL, synchronous=NORMAL, foreign_keys=ON)</then>
  <verification>
    <source_files>
      <file hint="Schema DDL statements">src/index/schema.rs</file>
      <file hint="Database initialization">src/index/storage.rs</file>
    </source_files>
    <test_file>tests/STORY-003/test_ac1_schema_creation.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#2: Symbol CRUD Operations - Insert and Bulk Insert

```xml
<acceptance_criteria id="AC2" implements="SVC-003,SVC-004,SVC-005">
  <given>A list of Symbol structs extracted from parsed files (from STORY-002)</given>
  <when>The storage layer saves the symbols</when>
  <then>Single symbols insertable via insert_symbol(), bulk inserts via insert_symbols() use transaction for atomicity, existing symbols for same file_path are replaced (upsert), updated_at set to Unix timestamp, insert of 1000 symbols completes in under 500ms</then>
  <verification>
    <source_files>
      <file hint="Symbol CRUD operations">src/index/storage.rs</file>
    </source_files>
    <test_file>tests/STORY-003/test_ac2_symbol_crud.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#3: File Tracking for Change Detection

```xml
<acceptance_criteria id="AC3" implements="SVC-006,SVC-007,SVC-008">
  <given>A file has been indexed</given>
  <when>The storage layer records the file metadata</when>
  <then>File path, language, SHA-256 hash, and indexed_at stored in files table; get_file_hash() returns stored hash; needs_reindex() returns true if hash differs or file not in database; re-indexed files have old symbols deleted first</then>
  <verification>
    <source_files>
      <file hint="File tracking operations">src/index/storage.rs</file>
    </source_files>
    <test_file>tests/STORY-003/test_ac3_file_tracking.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#4: Symbol Query Operations

```xml
<acceptance_criteria id="AC4" implements="SEARCH-001,SEARCH-002,SEARCH-003,SEARCH-004,SEARCH-005">
  <given>The database contains indexed symbols</given>
  <when>Search queries are executed</when>
  <then>query_by_name() returns exact matches, query_by_name_case_insensitive() ignores case, query_by_type() filters by SymbolType, query_by_file() returns symbols in file, combined filters work, all queries return Vec of Symbol with fully populated fields</then>
  <verification>
    <source_files>
      <file hint="Search query functions">src/index/search.rs</file>
    </source_files>
    <test_file>tests/STORY-003/test_ac4_query_operations.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#5: Error Handling and Recovery

```xml
<acceptance_criteria id="AC5" implements="DM-001,DM-002">
  <given>The database may encounter errors (corruption, permissions, disk full)</given>
  <when>Storage operations fail</when>
  <then>All errors wrapped in StorageError enum with variants (DatabaseCorrupted, PermissionDenied, DiskFull, ConnectionFailed, QueryFailed), no panics on any error, corrupted database detected via PRAGMA integrity_check, permission errors return helpful message</then>
  <verification>
    <source_files>
      <file hint="Error type definitions">src/index/storage.rs</file>
    </source_files>
    <test_file>tests/STORY-003/test_ac5_error_handling.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

## Technical Specification

```yaml
technical_specification:
  format_version: "2.0"
  story_id: "STORY-003"

  components:
    - type: "Configuration"
      name: "Schema Definition"
      file_path: "src/index/schema.rs"
      requirements:
        - id: "CFG-001"
          description: "Define SCHEMA_VERSION constant for future migrations"
          testable: true
          test_requirement: "Test: SCHEMA_VERSION is accessible and returns version string"
          priority: "Medium"
          implements_ac: ["AC#1"]
        - id: "CFG-002"
          description: "Define create_tables() function with SQL DDL for symbols, files, metadata"
          testable: true
          test_requirement: "Test: create_tables() creates expected schema on fresh connection"
          priority: "Critical"
          implements_ac: ["AC#1"]
        - id: "CFG-003"
          description: "Define PRAGMA settings and apply_pragmas() for WAL, synchronous, foreign_keys"
          testable: true
          test_requirement: "Test: apply_pragmas() sets journal_mode=WAL"
          priority: "Critical"
          implements_ac: ["AC#1"]

    - type: "DataModel"
      name: "StorageError"
      file_path: "src/index/storage.rs"
      requirements:
        - id: "DM-001"
          description: "Define StorageError enum with DatabaseCorrupted, PermissionDenied, DiskFull, ConnectionFailed, QueryFailed"
          testable: true
          test_requirement: "Test: Each variant has descriptive Display message"
          priority: "Critical"
          implements_ac: ["AC#5"]
        - id: "DM-002"
          description: "Implement From<rusqlite::Error> for automatic conversion"
          testable: true
          test_requirement: "Test: rusqlite::Error maps to StorageError variant"
          priority: "High"
          implements_ac: ["AC#5"]

    - type: "Service"
      name: "IndexStorage"
      file_path: "src/index/storage.rs"
      requirements:
        - id: "SVC-001"
          description: "Implement new(path: &Path) that creates/opens database and directory"
          testable: true
          test_requirement: "Test: new() creates .treelint/index.db if not exists"
          priority: "Critical"
          implements_ac: ["AC#1"]
        - id: "SVC-002"
          description: "Implement initialize_schema() that creates tables and indexes"
          testable: true
          test_requirement: "Test: Fresh database has all required tables and indexes"
          priority: "Critical"
          implements_ac: ["AC#1"]
        - id: "SVC-003"
          description: "Implement insert_symbol(symbol: &Symbol)"
          testable: true
          test_requirement: "Test: Inserted symbol retrievable with all fields"
          priority: "Critical"
          implements_ac: ["AC#2"]
        - id: "SVC-004"
          description: "Implement insert_symbols(symbols: &[Symbol]) with transaction"
          testable: true
          test_requirement: "Test: 1000 symbols insert atomically in under 500ms"
          priority: "Critical"
          implements_ac: ["AC#2"]
        - id: "SVC-005"
          description: "Implement delete_symbols_for_file(file_path: &str)"
          testable: true
          test_requirement: "Test: All symbols for file deleted, returns count"
          priority: "High"
          implements_ac: ["AC#2"]
        - id: "SVC-006"
          description: "Implement record_file(path, language, hash)"
          testable: true
          test_requirement: "Test: File record stored and retrievable"
          priority: "High"
          implements_ac: ["AC#3"]
        - id: "SVC-007"
          description: "Implement get_file_hash(path: &str)"
          testable: true
          test_requirement: "Test: Returns Some(hash) for indexed, None for unknown"
          priority: "High"
          implements_ac: ["AC#3"]
        - id: "SVC-008"
          description: "Implement needs_reindex(path, current_hash)"
          testable: true
          test_requirement: "Test: Returns true if hash differs or file unknown"
          priority: "High"
          implements_ac: ["AC#3"]

    - type: "Service"
      name: "IndexSearch"
      file_path: "src/index/search.rs"
      requirements:
        - id: "SEARCH-001"
          description: "Implement query_by_name(name: &str)"
          testable: true
          test_requirement: "Test: Exact name match returns matching symbols"
          priority: "Critical"
          implements_ac: ["AC#4"]
        - id: "SEARCH-002"
          description: "Implement query_by_name_case_insensitive(name: &str)"
          testable: true
          test_requirement: "Test: Case-insensitive match works"
          priority: "High"
          implements_ac: ["AC#4"]
        - id: "SEARCH-003"
          description: "Implement query_by_type(symbol_type: SymbolType)"
          testable: true
          test_requirement: "Test: Returns all symbols of type"
          priority: "High"
          implements_ac: ["AC#4"]
        - id: "SEARCH-004"
          description: "Implement query_by_file(file_path: &str)"
          testable: true
          test_requirement: "Test: Returns all symbols in file"
          priority: "High"
          implements_ac: ["AC#4"]
        - id: "SEARCH-005"
          description: "Implement query(filters: QueryFilters) for combined filters"
          testable: true
          test_requirement: "Test: Combined filters (name+type, type+file) work"
          priority: "Critical"
          implements_ac: ["AC#4"]

  business_rules:
    - id: "BR-001"
      rule: "All queries must use parameterized statements (no SQL injection)"
      test_requirement: "Test: No SQL concatenation in any query function"
    - id: "BR-002"
      rule: "Re-indexing a file must delete old symbols before inserting new"
      test_requirement: "Test: Re-index produces clean state, no duplicate symbols"
    - id: "BR-003"
      rule: "Database must use WAL mode for concurrent read access"
      test_requirement: "Test: 10 concurrent readers return correct results"

  non_functional_requirements:
    - id: "NFR-001"
      category: "Performance"
      requirement: "Query latency under 50ms"
      metric: "< 50ms for single symbol lookup (p95) with 100K symbols"
      test_requirement: "Test: Benchmark query_by_name on 100K symbol database < 50ms"
    - id: "NFR-002"
      category: "Performance"
      requirement: "Bulk insert under 500ms"
      metric: "< 500ms for 1000 symbols with transaction"
      test_requirement: "Test: Benchmark insert_symbols(1000) < 500ms"
    - id: "NFR-003"
      category: "Reliability"
      requirement: "No panics on errors"
      metric: "All errors return Result, no unwrap in production"
      test_requirement: "Test: Corrupted database returns StorageError, not panic"
    - id: "NFR-004"
      category: "Security"
      requirement: "No SQL injection"
      metric: "All queries use parameterized statements"
      test_requirement: "Test: Malicious input in name parameter does not execute"
```

---

## Technical Limitations

```yaml
technical_limitations: []
# rusqlite with bundled feature provides all needed functionality
```

---

## Non-Functional Requirements (NFRs)

### Performance

**Query Latency:**
- Single symbol lookup: < 50ms (p95) with 100K symbols
- Query by type: < 100ms (p95) with 100K symbols

**Write Performance:**
- Single insert: < 10ms
- Bulk insert (1000 symbols): < 500ms with transaction

**Connection:**
- Open and verify: < 50ms
- Memory: < 10MB per connection

---

### Security

**SQL Injection Prevention:** All queries use parameterized statements
**No Credentials:** Database contains only code metadata
**Path Validation:** Reject paths with `..` or absolute paths outside project

---

### Reliability

**WAL Mode:** Crash recovery, concurrent reads
**Transactions:** All multi-statement operations atomic
**Integrity Checks:** Optional verify_integrity() method

---

## Dependencies

### Prerequisite Stories

- [x] **STORY-002:** tree-sitter Parser Integration
  - **Why:** Provides Symbol struct to store
  - **Status:** Backlog

### Technology Dependencies

- [ ] **rusqlite:** 0.31 with bundled feature
  - **Purpose:** SQLite bindings with embedded SQLite
  - **Approved:** Yes (dependencies.md)

---

## Test Strategy

### Unit Tests

**Coverage Target:** 95%+ for index module

**Test Scenarios:**
1. **Happy Path:** Create database, insert symbols, query back
2. **Edge Cases:**
   - Empty database queries
   - Unicode symbol names
   - Very long signatures
   - Concurrent reads
3. **Error Cases:**
   - Permission denied
   - Corrupted database
   - Disk full simulation

---

## Acceptance Criteria Verification Checklist

### AC#1: Database Initialization and Schema Creation

- [ ] Schema DDL defined - **Phase:** 2 - **Evidence:** src/index/schema.rs
- [ ] symbols table created correctly - **Phase:** 3 - **Evidence:** test_ac1
- [ ] files table created correctly - **Phase:** 3 - **Evidence:** test_ac1
- [ ] metadata table created correctly - **Phase:** 3 - **Evidence:** test_ac1
- [ ] Indexes created - **Phase:** 3 - **Evidence:** test_ac1
- [ ] WAL mode enabled - **Phase:** 3 - **Evidence:** test_ac1

### AC#2: Symbol CRUD Operations

- [ ] insert_symbol() works - **Phase:** 3 - **Evidence:** test_ac2
- [ ] insert_symbols() with transaction - **Phase:** 3 - **Evidence:** test_ac2
- [ ] Upsert behavior (replace existing) - **Phase:** 3 - **Evidence:** test_ac2
- [ ] 1000 symbols < 500ms - **Phase:** 3 - **Evidence:** benchmark
- [ ] delete_symbols_for_file() works - **Phase:** 3 - **Evidence:** test_ac2

### AC#3: File Tracking

- [ ] record_file() stores metadata - **Phase:** 3 - **Evidence:** test_ac3
- [ ] get_file_hash() retrieves hash - **Phase:** 3 - **Evidence:** test_ac3
- [ ] needs_reindex() detects changes - **Phase:** 3 - **Evidence:** test_ac3

### AC#4: Query Operations

- [ ] query_by_name() exact match - **Phase:** 3 - **Evidence:** test_ac4
- [ ] query_by_name_case_insensitive() - **Phase:** 3 - **Evidence:** test_ac4
- [ ] query_by_type() - **Phase:** 3 - **Evidence:** test_ac4
- [ ] query_by_file() - **Phase:** 3 - **Evidence:** test_ac4
- [ ] Combined filters - **Phase:** 3 - **Evidence:** test_ac4

### AC#5: Error Handling

- [ ] StorageError enum defined - **Phase:** 2 - **Evidence:** src/index/storage.rs
- [ ] No panics on errors - **Phase:** 3 - **Evidence:** test_ac5
- [ ] Corrupted database detected - **Phase:** 3 - **Evidence:** test_ac5

---

**Checklist Progress:** 0/22 items complete (0%)

---

## Definition of Done

### Implementation
- [ ] src/index/schema.rs with DDL and PRAGMA settings
- [ ] src/index/storage.rs with IndexStorage struct and CRUD operations
- [ ] src/index/search.rs with query functions
- [ ] src/index/mod.rs with module exports
- [ ] StorageError enum with all variants
- [ ] Directory creation (.treelint/) if not exists

### Quality
- [ ] All 5 acceptance criteria have passing tests
- [ ] Edge cases covered (unicode, large data, concurrent access)
- [ ] No SQL injection vulnerabilities
- [ ] Code coverage > 95% for src/index/
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo fmt --check` passes

### Testing
- [ ] Unit tests for schema creation
- [ ] Unit tests for symbol CRUD
- [ ] Unit tests for file tracking
- [ ] Unit tests for query operations
- [ ] Unit tests for error handling
- [ ] Benchmark: query < 50ms on 100K symbols
- [ ] Benchmark: bulk insert < 500ms for 1000 symbols

### Documentation
- [ ] All public items have `///` doc comments
- [ ] Module-level `//!` comments for index module
- [ ] SQL schema documented inline

---

## Change Log

**Current Status:** Ready for Dev

| Date | Author | Phase/Action | Change | Files Affected |
|------|--------|--------------|--------|----------------|
| 2026-01-27 15:30 | claude/story-requirements-analyst | Created | Story created from EPIC-001 F1.3 | STORY-003-sqlite-index-storage.story.md |
| 2026-01-27 | claude/sprint-planner | Sprint Planning | Status: Backlog → Ready for Dev, Added to SPRINT-001 | STORY-003-sqlite-index-storage.story.md |

## Notes

**Design Decisions:**
- Using rusqlite bundled feature for zero-dependency SQLite
- WAL mode enables concurrent reads (important for daemon in EPIC-002)
- Upsert behavior simplifies re-indexing logic

**SQL Schema Reference:**
- From EPIC-001, lines 175-212

**References:**
- EPIC-001: Core CLI Foundation
- BRAINSTORM-001: Indexing Architecture section
- tech-stack.md: rusqlite 0.31 bundled approved
- dependencies.md: rusqlite approved

---

Story Template Version: 2.7
Last Updated: 2026-01-27
