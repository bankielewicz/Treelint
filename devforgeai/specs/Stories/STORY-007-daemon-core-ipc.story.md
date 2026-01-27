---
id: STORY-007
title: Daemon Core Architecture with IPC Communication
type: feature
epic: EPIC-002
sprint: Backlog
status: Backlog
points: 5
depends_on: ["STORY-001", "STORY-003"]
priority: High
assigned_to: Unassigned
created: 2026-01-27
format_version: "2.7"
---

# Story: Daemon Core Architecture with IPC Communication

## Description

**As a** developer using treelint,
**I want** a background daemon process that maintains the symbol index,
**so that** my search queries return instantly without waiting for on-demand indexing.

**Business Value:** The daemon is the foundation for instant query performance. Without it, every search must either wait for indexing or use a potentially stale index. The daemon enables sub-5ms query latency by keeping the index always fresh in memory.

## Provenance

```xml
<provenance>
  <origin document="EPIC-002" section="Feature F4: Background Indexing">
    <quote>"Daemon architecture with IPC (Unix socket), File watcher integration, Incremental index updates"</quote>
    <line_reference>lines 58-65</line_reference>
    <quantified_impact>Instant queries (~5ms vs ~20-60s for on-demand indexing)</quantified_impact>
  </origin>

  <decision rationale="unix-socket-ipc-for-simplicity">
    <selected>Unix domain socket for IPC (Named pipe on Windows)</selected>
    <rejected alternative="HTTP-server">
      HTTP adds unnecessary overhead and complexity for local IPC
    </rejected>
    <rejected alternative="gRPC">
      gRPC is overkill for simple local communication (per tech-stack.md)
    </rejected>
    <trade_off>Platform-specific code paths in exchange for minimal latency</trade_off>
  </decision>

  <stakeholder role="Developer" goal="instant-query-performance">
    <quote>"Instant queries via always-fresh daemon index"</quote>
    <source>EPIC-002, Value Proposition</source>
  </stakeholder>
</provenance>
```

---

## Acceptance Criteria

### AC#1: Daemon Process Lifecycle

```xml
<acceptance_criteria id="AC1" implements="DAEMON-001,DAEMON-002">
  <given>No daemon process is running</given>
  <when>The daemon is started programmatically or via internal API</when>
  <then>A background daemon process spawns, creates the IPC socket/pipe, and enters ready state within 2 seconds</then>
  <verification>
    <source_files>
      <file hint="Daemon server implementation">src/daemon/server.rs</file>
      <file hint="Daemon module">src/daemon/mod.rs</file>
    </source_files>
    <test_file>tests/daemon_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#2: IPC Socket/Pipe Creation

```xml
<acceptance_criteria id="AC2" implements="DAEMON-003,DAEMON-004">
  <given>Daemon process is starting</given>
  <when>Daemon initializes IPC transport</when>
  <then>
    - On Unix/macOS: Creates socket at `.treelint/daemon.sock`
    - On Windows: Creates named pipe at `\\.\pipe\treelint-daemon`
    - Socket/pipe has appropriate permissions (user-only access)
  </then>
  <verification>
    <source_files>
      <file hint="IPC transport abstraction">src/daemon/server.rs</file>
    </source_files>
    <test_file>tests/daemon_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#3: NDJSON Message Protocol

```xml
<acceptance_criteria id="AC3" implements="DAEMON-005,DAEMON-006">
  <given>Daemon is running and a client connects</given>
  <when>Client sends a newline-delimited JSON request</when>
  <then>
    Daemon parses the request, processes it, and returns a newline-delimited JSON response with:
    - `id` field matching request ID
    - `result` field with response data (or null on error)
    - `error` field with error details (or null on success)
  </then>
  <verification>
    <source_files>
      <file hint="Protocol parsing">src/daemon/protocol.rs</file>
      <file hint="Request handling">src/daemon/server.rs</file>
    </source_files>
    <test_file>tests/daemon_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#4: Search Method Handler

```xml
<acceptance_criteria id="AC4" implements="DAEMON-007">
  <given>Daemon is running with a loaded index</given>
  <when>Client sends `{"id": "req-001", "method": "search", "params": {"symbol": "foo", "type": "function"}}`</when>
  <then>Daemon returns search results in the standard JSON output format (same schema as CLI search)</then>
  <verification>
    <source_files>
      <file hint="Search handler">src/daemon/server.rs</file>
      <file hint="Index search">src/index/search.rs</file>
    </source_files>
    <test_file>tests/daemon_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#5: Status Method Handler

```xml
<acceptance_criteria id="AC5" implements="DAEMON-008">
  <given>Daemon is running</given>
  <when>Client sends `{"id": "req-002", "method": "status", "params": {}}`</when>
  <then>
    Daemon returns status response with:
    - `status`: "starting" | "ready" | "indexing" | "stopping"
    - `indexed_files`: count of indexed files
    - `indexed_symbols`: count of indexed symbols
    - `last_index_time`: ISO 8601 timestamp
    - `uptime_seconds`: integer
    - `pid`: process ID
    - `socket_path`: path to IPC socket/pipe
  </then>
  <verification>
    <source_files>
      <file hint="Status handler">src/daemon/server.rs</file>
    </source_files>
    <test_file>tests/daemon_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#6: Graceful Shutdown

```xml
<acceptance_criteria id="AC6" implements="DAEMON-009,DAEMON-010">
  <given>Daemon is running with active connections</given>
  <when>Daemon receives shutdown signal (SIGTERM/SIGINT or explicit stop)</when>
  <then>
    - Daemon stops accepting new connections
    - Existing requests complete (up to 5 second timeout)
    - IPC socket/pipe is removed
    - Process exits with code 0
  </then>
  <verification>
    <source_files>
      <file hint="Shutdown handling">src/daemon/server.rs</file>
    </source_files>
    <test_file>tests/daemon_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#7: Error Response Format

```xml
<acceptance_criteria id="AC7" implements="DAEMON-011">
  <given>Client sends an invalid or failing request</given>
  <when>Daemon processes the request</when>
  <then>
    Daemon returns error response with:
    - `id` matching request ID
    - `result`: null
    - `error`: {"code": "EXXX", "message": "Human-readable error"}
    Standard error codes: E001 (index not ready), E002 (invalid method), E003 (invalid params)
  </then>
  <verification>
    <source_files>
      <file hint="Error handling">src/daemon/protocol.rs</file>
    </source_files>
    <test_file>tests/daemon_tests.rs</test_file>
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
    # Daemon Server
    - type: "Service"
      name: "DaemonServer"
      file_path: "src/daemon/server.rs"
      interface: "pub struct DaemonServer"
      lifecycle: "Long-running process"
      dependencies:
        - "interprocess::local_socket"
        - "SqliteStorage"
        - "SymbolExtractor"
      requirements:
        - id: "DAEMON-001"
          description: "Start daemon process and enter event loop"
          testable: true
          test_requirement: "Test: Daemon starts and accepts connections within 2 seconds"
          priority: "Critical"
        - id: "DAEMON-002"
          description: "Track daemon state (starting, ready, indexing, stopping)"
          testable: true
          test_requirement: "Test: State transitions correctly during lifecycle"
          priority: "High"
        - id: "DAEMON-003"
          description: "Create Unix socket at .treelint/daemon.sock"
          testable: true
          test_requirement: "Test: Socket file created with correct permissions (0600)"
          priority: "Critical"
        - id: "DAEMON-004"
          description: "Create named pipe on Windows at \\.\pipe\treelint-daemon"
          testable: true
          test_requirement: "Test: Named pipe accessible on Windows"
          priority: "Critical"
        - id: "DAEMON-009"
          description: "Handle graceful shutdown on SIGTERM/SIGINT"
          testable: true
          test_requirement: "Test: Daemon exits cleanly within 5 seconds of signal"
          priority: "High"
        - id: "DAEMON-010"
          description: "Clean up socket/pipe file on shutdown"
          testable: true
          test_requirement: "Test: Socket file removed after daemon stops"
          priority: "High"

    # IPC Protocol Handler
    - type: "Service"
      name: "ProtocolHandler"
      file_path: "src/daemon/protocol.rs"
      interface: "pub trait ProtocolHandler"
      lifecycle: "Per-connection"
      dependencies:
        - "serde_json"
      requirements:
        - id: "DAEMON-005"
          description: "Parse NDJSON request messages"
          testable: true
          test_requirement: "Test: Valid JSON parsed, invalid JSON returns error"
          priority: "Critical"
        - id: "DAEMON-006"
          description: "Serialize NDJSON response messages"
          testable: true
          test_requirement: "Test: Response includes id, result, error fields"
          priority: "Critical"
        - id: "DAEMON-011"
          description: "Return structured error responses with codes"
          testable: true
          test_requirement: "Test: E001, E002, E003 error codes returned appropriately"
          priority: "High"

    # Request Types
    - type: "DataModel"
      name: "DaemonRequest"
      table: "N/A (in-memory)"
      purpose: "IPC request message structure"
      fields:
        - name: "id"
          type: "String"
          constraints: "Required"
          description: "Unique request identifier for correlation"
          test_requirement: "Test: Response id matches request id"
        - name: "method"
          type: "String"
          constraints: "Required, enum: search|status|index"
          description: "Method to invoke"
          test_requirement: "Test: Unknown method returns E002"
        - name: "params"
          type: "Value (JSON)"
          constraints: "Required (can be empty object)"
          description: "Method-specific parameters"
          test_requirement: "Test: Missing params returns E003"

    # Response Types
    - type: "DataModel"
      name: "DaemonResponse"
      table: "N/A (in-memory)"
      purpose: "IPC response message structure"
      fields:
        - name: "id"
          type: "String"
          constraints: "Required, matches request"
          description: "Request identifier for correlation"
          test_requirement: "Test: Response id always matches request id"
        - name: "result"
          type: "Value (JSON) | null"
          constraints: "null when error"
          description: "Success response data"
          test_requirement: "Test: result is null when error is non-null"
        - name: "error"
          type: "DaemonError | null"
          constraints: "null on success"
          description: "Error details"
          test_requirement: "Test: error is null when result is non-null"

    # Search Handler
    - type: "Service"
      name: "SearchHandler"
      file_path: "src/daemon/server.rs"
      interface: "fn handle_search(params: SearchParams) -> SearchResult"
      lifecycle: "Per-request"
      dependencies:
        - "Searcher"
      requirements:
        - id: "DAEMON-007"
          description: "Execute search query using loaded index"
          testable: true
          test_requirement: "Test: Search returns same results as CLI search command"
          priority: "Critical"

    # Status Handler
    - type: "Service"
      name: "StatusHandler"
      file_path: "src/daemon/server.rs"
      interface: "fn handle_status() -> StatusResult"
      lifecycle: "Per-request"
      dependencies: []
      requirements:
        - id: "DAEMON-008"
          description: "Return comprehensive daemon status"
          testable: true
          test_requirement: "Test: Status includes all 7 required fields"
          priority: "High"

  business_rules:
    - id: "BR-001"
      rule: "Only one daemon instance can run per working directory"
      trigger: "Daemon startup"
      validation: "Check for existing socket/pipe before binding"
      error_handling: "Return error if socket already exists and daemon is responsive"
      test_requirement: "Test: Second daemon start fails with clear error message"
      priority: "High"

    - id: "BR-002"
      rule: "Daemon must respond to requests within 100ms (excluding index build)"
      trigger: "Request processing"
      validation: "Timeout warning in logs if exceeded"
      error_handling: "Log warning, continue processing"
      test_requirement: "Test: Search request completes within 100ms on indexed data"
      priority: "Medium"

    - id: "BR-003"
      rule: "Stale socket files must be cleaned up on startup"
      trigger: "Daemon startup"
      validation: "Attempt to connect to existing socket; if fails, remove and recreate"
      error_handling: "Remove stale socket, proceed with startup"
      test_requirement: "Test: Daemon starts successfully after unclean shutdown"
      priority: "High"

  non_functional_requirements:
    - id: "NFR-001"
      category: "Performance"
      requirement: "Daemon idle CPU usage must be minimal"
      metric: "< 0.1% CPU when idle (no active requests)"
      test_requirement: "Test: Monitor CPU for 60 seconds with no requests"
      priority: "High"

    - id: "NFR-002"
      category: "Performance"
      requirement: "Search query latency via daemon must be fast"
      metric: "< 5ms for search query (p95) on indexed data"
      test_requirement: "Test: Benchmark 1000 search queries, verify p95 < 5ms"
      priority: "Critical"

    - id: "NFR-003"
      category: "Reliability"
      requirement: "Daemon must handle concurrent connections"
      metric: "Support 10+ simultaneous client connections"
      test_requirement: "Test: 10 clients send requests concurrently, all receive responses"
      priority: "High"
```

---

## Technical Limitations

```yaml
technical_limitations: []
# interprocess crate handles cross-platform IPC abstraction
```

---

## Non-Functional Requirements (NFRs)

### Performance

**Response Time:**
- **Search via daemon:** < 5ms (p95)
- **Status query:** < 1ms (p95)

**Resource Usage:**
- Idle CPU: < 0.1%
- Memory: < 50MB base + index size

### Security

**IPC Security:**
- Unix socket permissions: 0600 (user-only)
- Windows named pipe: DACL restricts to current user
- No network exposure

### Reliability

**Daemon Stability:**
- Target: 8+ hours continuous operation without issues
- Graceful handling of malformed requests (no crash)
- Automatic recovery from transient errors

---

## Dependencies

### Prerequisite Stories

- [x] **STORY-001:** Project Setup + CLI Skeleton
  - **Why:** Daemon needs CLI infrastructure and project structure
  - **Status:** Backlog

- [x] **STORY-003:** SQLite Index Storage
  - **Why:** Daemon serves queries from SQLite index
  - **Status:** Backlog

### Technology Dependencies

All dependencies already approved in dependencies.md:
- interprocess (2.0) - Cross-platform IPC
- serde_json (1.0) - JSON parsing
- log (0.4) - Logging

---

## Test Strategy

### Unit Tests

**Coverage Target:** 95% for src/daemon/

**Test Scenarios:**
1. **Happy Path:**
   - Daemon starts and accepts connection
   - Search request returns results
   - Status request returns all fields
2. **Edge Cases:**
   - Request with missing id field
   - Request with unknown method
   - Request with invalid params
   - Empty request body
3. **Error Cases:**
   - Daemon already running (socket exists)
   - Invalid JSON in request
   - Index not ready during search

### Integration Tests

**Coverage Target:** 85%

**Test Scenarios:**
1. **End-to-End:** Client connects, sends search, receives results
2. **Shutdown:** Daemon stops cleanly on signal
3. **Concurrent:** Multiple clients query simultaneously

---

## Acceptance Criteria Verification Checklist

### AC#1: Daemon Process Lifecycle

- [ ] DaemonServer struct implemented - **Phase:** 3 - **Evidence:** src/daemon/server.rs
- [ ] Daemon enters ready state within 2 seconds - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs
- [ ] State tracking (starting/ready/indexing/stopping) - **Phase:** 3 - **Evidence:** src/daemon/server.rs

### AC#2: IPC Socket/Pipe Creation

- [ ] Unix socket created at .treelint/daemon.sock - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs
- [ ] Windows named pipe created - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs
- [ ] Permissions set correctly (user-only) - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs

### AC#3: NDJSON Message Protocol

- [ ] Request parsing implemented - **Phase:** 3 - **Evidence:** src/daemon/protocol.rs
- [ ] Response serialization implemented - **Phase:** 3 - **Evidence:** src/daemon/protocol.rs
- [ ] id/result/error fields in response - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs

### AC#4: Search Method Handler

- [ ] Search handler implemented - **Phase:** 3 - **Evidence:** src/daemon/server.rs
- [ ] Returns standard JSON output format - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs

### AC#5: Status Method Handler

- [ ] Status handler implemented - **Phase:** 3 - **Evidence:** src/daemon/server.rs
- [ ] All 7 status fields returned - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs

### AC#6: Graceful Shutdown

- [ ] Signal handlers installed - **Phase:** 3 - **Evidence:** src/daemon/server.rs
- [ ] Existing requests complete - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs
- [ ] Socket/pipe cleaned up - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs

### AC#7: Error Response Format

- [ ] Error codes defined (E001, E002, E003) - **Phase:** 3 - **Evidence:** src/daemon/protocol.rs
- [ ] Error responses formatted correctly - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs

---

**Checklist Progress:** 0/20 items complete (0%)

---

## Definition of Done

### Implementation
- [ ] DaemonServer struct with start/stop/status methods
- [ ] ProtocolHandler for NDJSON parsing
- [ ] Unix socket creation and binding
- [ ] Windows named pipe creation (conditional compilation)
- [ ] Search method handler
- [ ] Status method handler
- [ ] Signal handling for graceful shutdown
- [ ] Stale socket cleanup on startup

### Quality
- [ ] All 7 acceptance criteria have passing tests
- [ ] Edge cases covered (malformed requests, concurrent connections)
- [ ] Error responses use standard codes
- [ ] NFRs met (< 5ms search latency, < 0.1% idle CPU)
- [ ] Code coverage > 95% for src/daemon/

### Testing
- [ ] Unit tests for ProtocolHandler
- [ ] Unit tests for request/response parsing
- [ ] Integration tests for daemon lifecycle
- [ ] Integration tests for concurrent clients
- [ ] Platform-specific tests (Unix socket, Windows pipe)

### Documentation
- [ ] Daemon protocol documented in code comments
- [ ] Error codes documented
- [ ] Architecture documented in technical spec

---

## Change Log

**Current Status:** Backlog

| Date | Author | Phase/Action | Change | Files Affected |
|------|--------|--------------|--------|----------------|
| 2026-01-27 13:00 | claude/story-creation | Created | Story created from EPIC-002 F4 (split 1/3) | STORY-007-daemon-core-ipc.story.md |

## Notes

**Design Decisions:**
- Use `interprocess` crate for cross-platform IPC abstraction
- NDJSON (newline-delimited JSON) for simple, streaming protocol
- Single-threaded event loop with non-blocking I/O (sufficient for local IPC)

**Open Questions:**
- [ ] Should daemon support multiple simultaneous index operations? - **Owner:** Tech Lead - **Due:** Implementation

**Related ADRs:**
- Reference: tech-stack.md IPC section (lines 193-200)

---

Story Template Version: 2.7
Last Updated: 2026-01-27
