---
id: STORY-009
title: Daemon CLI Commands and Auto-Detection
type: feature
epic: EPIC-002
sprint: Backlog
status: Released
points: 3
depends_on: ["STORY-007", "STORY-001"]
priority: High
assigned_to: Unassigned
created: 2026-01-27
format_version: "2.7"
---

# Story: Daemon CLI Commands and Auto-Detection

## Description

**As a** developer using treelint,
**I want** CLI commands to manage the daemon (start/stop/status) and manual indexing,
**so that** I have full control over the background service and can trigger re-indexing when needed.

**Business Value:** The CLI commands are the user interface to the daemon. Without them, users cannot start, stop, or check the daemon status. The auto-detection feature makes the tool "just work" - queries go through the daemon if running, or fall back gracefully.

## Provenance

```xml
<provenance>
  <origin document="EPIC-002" section="Feature F4: Background Indexing">
    <quote>"`treelint daemon start/stop/status` commands, `treelint index` manual command with progress bar, Auto-detection: query daemon if running, else on-demand index"</quote>
    <line_reference>lines 62-65</line_reference>
    <quantified_impact>User control over daemon lifecycle and seamless query routing</quantified_impact>
  </origin>

  <decision rationale="subcommand-structure">
    <selected>`treelint daemon start|stop|status` subcommand structure</selected>
    <rejected alternative="treelint-daemon-binary">
      Separate binary would complicate installation and PATH management
    </rejected>
    <trade_off>Slightly longer command in exchange for single-binary simplicity</trade_off>
  </decision>

  <stakeholder role="Developer" goal="daemon-lifecycle-control">
    <quote>"Developer wants control over when daemon runs"</quote>
    <source>EPIC-002, User Stories</source>
  </stakeholder>
</provenance>
```

---

## Acceptance Criteria

### AC#1: Daemon Start Command

```xml
<acceptance_criteria id="AC1" implements="CLI-001,CLI-002">
  <given>No daemon is currently running</given>
  <when>User runs `treelint daemon start`</when>
  <then>
    - Daemon process spawns in background (detached from terminal)
    - Command returns immediately with success message
    - Message includes PID and socket path
    - Exit code is 0
  </then>
  <verification>
    <source_files>
      <file hint="Daemon command implementation">src/cli/commands/daemon.rs</file>
    </source_files>
    <test_file>tests/daemon_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#2: Daemon Start When Already Running

```xml
<acceptance_criteria id="AC2" implements="CLI-003">
  <given>Daemon is already running</given>
  <when>User runs `treelint daemon start`</when>
  <then>
    - Command detects existing daemon
    - Prints message: "Daemon already running (PID: XXXX)"
    - Does not start second daemon
    - Exit code is 0 (not an error)
  </then>
  <verification>
    <source_files>
      <file hint="Already running check">src/cli/commands/daemon.rs</file>
    </source_files>
    <test_file>tests/daemon_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#3: Daemon Stop Command

```xml
<acceptance_criteria id="AC3" implements="CLI-004,CLI-005">
  <given>Daemon is running</given>
  <when>User runs `treelint daemon stop`</when>
  <then>
    - Command sends shutdown signal to daemon
    - Waits for daemon to stop (up to 5 seconds)
    - Prints success message
    - Exit code is 0
  </then>
  <verification>
    <source_files>
      <file hint="Stop command">src/cli/commands/daemon.rs</file>
    </source_files>
    <test_file>tests/daemon_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#4: Daemon Stop When Not Running

```xml
<acceptance_criteria id="AC4" implements="CLI-006">
  <given>No daemon is running</given>
  <when>User runs `treelint daemon stop`</when>
  <then>
    - Command detects no daemon
    - Prints message: "No daemon running"
    - Exit code is 0 (not an error)
  </then>
  <verification>
    <source_files>
      <file hint="Not running check">src/cli/commands/daemon.rs</file>
    </source_files>
    <test_file>tests/daemon_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#5: Daemon Status Command

```xml
<acceptance_criteria id="AC5" implements="CLI-007,CLI-008">
  <given>User wants to check daemon state</given>
  <when>User runs `treelint daemon status`</when>
  <then>
    If daemon running:
    - Shows: status (ready/indexing), PID, uptime, indexed files/symbols, socket path
    - Exit code: 0

    If daemon not running:
    - Shows: "Daemon not running"
    - Exit code: 1
  </then>
  <verification>
    <source_files>
      <file hint="Status command">src/cli/commands/daemon.rs</file>
    </source_files>
    <test_file>tests/daemon_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#6: Manual Index Command

```xml
<acceptance_criteria id="AC6" implements="CLI-009,CLI-010">
  <given>User wants to manually build/rebuild the index</given>
  <when>User runs `treelint index`</when>
  <then>
    - Full index build starts
    - Progress bar shown if > 1000 files and stdout is TTY
    - Progress shows: percentage, files processed, current file, speed, ETA
    - Completion message shows: files indexed, symbols found, index size, duration
    - Exit code: 0 on success
  </then>
  <verification>
    <source_files>
      <file hint="Index command">src/cli/commands/index.rs</file>
    </source_files>
    <test_file>tests/index_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#7: Force Re-index Flag

```xml
<acceptance_criteria id="AC7" implements="CLI-011">
  <given>Index already exists with cached file hashes</given>
  <when>User runs `treelint index --force`</when>
  <then>
    - All files are re-parsed regardless of hash
    - Existing index is rebuilt from scratch
    - Progress bar shows full file count
  </then>
  <verification>
    <source_files>
      <file hint="Force flag handling">src/cli/commands/index.rs</file>
    </source_files>
    <test_file>tests/index_tests.rs</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#8: Auto-Detection for Search Command

```xml
<acceptance_criteria id="AC8" implements="CLI-012,CLI-013,CLI-014">
  <given>User runs `treelint search foo`</given>
  <when>Search command processes the request</when>
  <then>
    Auto-detection logic:
    1. Check if daemon socket/pipe exists and responds → Query daemon (~5ms)
    2. Else check if index.db exists and is fresh → Query index directly (~20ms)
    3. Else build index on-demand, then query (~5-60s first time)

    Behavior is transparent to user (same output format)
  </then>
  <verification>
    <source_files>
      <file hint="Auto-detection logic">src/cli/commands/search.rs</file>
    </source_files>
    <test_file>tests/search_tests.rs</test_file>
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
    # Daemon Command Handler
    - type: "Service"
      name: "DaemonCommand"
      file_path: "src/cli/commands/daemon.rs"
      interface: "pub fn execute(action: DaemonAction) -> Result<()>"
      lifecycle: "CLI invocation"
      dependencies:
        - "DaemonServer"
        - "clap"
      requirements:
        - id: "CLI-001"
          description: "Spawn daemon process in background"
          testable: true
          test_requirement: "Test: Daemon process detaches from terminal"
          priority: "Critical"
        - id: "CLI-002"
          description: "Report daemon PID and socket path on start"
          testable: true
          test_requirement: "Test: Start output includes PID number and path"
          priority: "High"
        - id: "CLI-003"
          description: "Detect already-running daemon before start"
          testable: true
          test_requirement: "Test: Second start shows 'already running' message"
          priority: "High"
        - id: "CLI-004"
          description: "Send shutdown signal to running daemon"
          testable: true
          test_requirement: "Test: Stop command terminates daemon process"
          priority: "Critical"
        - id: "CLI-005"
          description: "Wait for daemon shutdown with timeout"
          testable: true
          test_requirement: "Test: Stop waits up to 5 seconds for clean exit"
          priority: "High"
        - id: "CLI-006"
          description: "Handle stop when no daemon running"
          testable: true
          test_requirement: "Test: Stop with no daemon shows message, exit 0"
          priority: "Medium"
        - id: "CLI-007"
          description: "Query daemon status via IPC"
          testable: true
          test_requirement: "Test: Status shows all daemon metrics"
          priority: "High"
        - id: "CLI-008"
          description: "Different exit codes for running vs not running"
          testable: true
          test_requirement: "Test: Status exit code 0 if running, 1 if not"
          priority: "Medium"

    # Index Command Handler
    - type: "Service"
      name: "IndexCommand"
      file_path: "src/cli/commands/index.rs"
      interface: "pub fn execute(args: IndexArgs) -> Result<()>"
      lifecycle: "CLI invocation"
      dependencies:
        - "IndexBuilder"
        - "indicatif"
        - "clap"
      requirements:
        - id: "CLI-009"
          description: "Build full index with progress reporting"
          testable: true
          test_requirement: "Test: Index command processes all supported files"
          priority: "Critical"
        - id: "CLI-010"
          description: "Show progress bar for large repositories"
          testable: true
          test_requirement: "Test: Progress bar visible for > 1000 files"
          priority: "High"
        - id: "CLI-011"
          description: "Force flag bypasses hash-based caching"
          testable: true
          test_requirement: "Test: --force re-parses all files regardless of hash"
          priority: "High"

    # Auto-Detection Logic
    - type: "Service"
      name: "QueryRouter"
      file_path: "src/cli/commands/search.rs"
      interface: "fn route_query(query: &Query) -> QueryResult"
      lifecycle: "Per-search"
      dependencies:
        - "DaemonClient"
        - "Searcher"
        - "IndexBuilder"
      requirements:
        - id: "CLI-012"
          description: "Detect daemon availability"
          testable: true
          test_requirement: "Test: Query routed to daemon when running"
          priority: "Critical"
        - id: "CLI-013"
          description: "Fall back to direct index query"
          testable: true
          test_requirement: "Test: Query uses index.db when daemon not running"
          priority: "Critical"
        - id: "CLI-014"
          description: "Build index on-demand when no index exists"
          testable: true
          test_requirement: "Test: First search triggers index build"
          priority: "Critical"

    # CLI Arguments
    - type: "Configuration"
      name: "DaemonArgs"
      file_path: "src/cli/args.rs"
      required_keys:
        - key: "daemon_action"
          type: "DaemonAction enum"
          example: "start | stop | status"
          required: true
          default: "N/A"
          validation: "One of: start, stop, status"
          test_requirement: "Test: Subcommand parsing for daemon"
        - key: "index_force"
          type: "bool"
          example: "--force"
          required: false
          default: "false"
          validation: "Boolean flag"
          test_requirement: "Test: --force flag parsed correctly"

    # Progress Bar
    - type: "Service"
      name: "IndexProgressBar"
      file_path: "src/cli/commands/index.rs"
      interface: "fn create_progress_bar(total: u64) -> ProgressBar"
      lifecycle: "Per-index-operation"
      dependencies:
        - "indicatif"
        - "atty"
      requirements:
        - id: "CLI-015"
          description: "Display progress bar in terminal"
          testable: true
          test_requirement: "Test: Progress bar shows percentage, files, ETA"
          priority: "High"
        - id: "CLI-016"
          description: "Suppress progress bar when piped"
          testable: true
          test_requirement: "Test: No progress output when stdout is not TTY"
          priority: "Medium"

  business_rules:
    - id: "BR-001"
      rule: "Daemon start is idempotent (no error if already running)"
      trigger: "`treelint daemon start` when daemon running"
      validation: "Check socket before spawning"
      error_handling: "Return success with 'already running' message"
      test_requirement: "Test: Multiple starts don't create multiple daemons"
      priority: "High"

    - id: "BR-002"
      rule: "Daemon stop is idempotent (no error if not running)"
      trigger: "`treelint daemon stop` when daemon not running"
      validation: "Check socket before sending signal"
      error_handling: "Return success with 'not running' message"
      test_requirement: "Test: Stop when no daemon returns exit 0"
      priority: "High"

    - id: "BR-003"
      rule: "Progress bar only shown for large repos in TTY"
      trigger: "Index command execution"
      validation: "Check file count > 1000 AND stdout is TTY"
      error_handling: "N/A (conditional display)"
      test_requirement: "Test: Small repo has no progress bar"
      priority: "Medium"

  non_functional_requirements:
    - id: "NFR-001"
      category: "Performance"
      requirement: "Auto-detection must not add significant latency"
      metric: "< 10ms overhead for daemon detection"
      test_requirement: "Test: Measure search latency with/without daemon"
      priority: "High"

    - id: "NFR-002"
      category: "Usability"
      requirement: "Progress bar must show meaningful information"
      metric: "Update at least 10 times per second during indexing"
      test_requirement: "Test: Progress bar updates smoothly during index"
      priority: "Medium"
```

---

## Technical Limitations

```yaml
technical_limitations: []
# CLI framework (clap) handles all command parsing robustly
```

---

## Non-Functional Requirements (NFRs)

### Performance

**Latency:**
- Auto-detection overhead: < 10ms
- Daemon start command: < 500ms to return

**Progress Bar:**
- Updates: ≥ 10 per second
- Responsive to Ctrl+C cancellation

### Usability

**Command Output:**
- Clear success/failure messages
- Consistent exit codes (0 success, 1 failure)
- Helpful error messages

---

## Dependencies

### Prerequisite Stories

- [x] **STORY-007:** Daemon Core Architecture
  - **Why:** Commands interact with daemon via IPC
  - **Status:** Backlog

- [x] **STORY-001:** Project Setup + CLI Skeleton
  - **Why:** Commands extend existing CLI structure
  - **Status:** Backlog

### Technology Dependencies

All dependencies already approved in dependencies.md:
- clap (4.5) - CLI argument parsing
- indicatif (0.17) - Progress bars
- atty (0.2) - TTY detection

---

## Test Strategy

### Unit Tests

**Coverage Target:** 95% for src/cli/commands/daemon.rs, index.rs

**Test Scenarios:**
1. **Happy Path:**
   - Start daemon successfully
   - Stop daemon successfully
   - Status shows correct info
   - Index completes with progress
2. **Edge Cases:**
   - Start when already running
   - Stop when not running
   - Index empty directory
   - Index with no supported files
3. **Error Cases:**
   - Daemon socket permission error
   - Index write permission error

### Integration Tests

**Coverage Target:** 85%

**Test Scenarios:**
1. **Full Lifecycle:** Start → Status → Search → Stop
2. **Auto-Detection:** Search with daemon running vs not
3. **Progress Bar:** Visual verification on large repo

---

## Acceptance Criteria Verification Checklist

### AC#1: Daemon Start Command

- [ ] Daemon spawns in background - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs
- [ ] Returns immediately - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs
- [ ] Shows PID and socket path - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs

### AC#2: Daemon Start When Already Running

- [ ] Detects existing daemon - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs
- [ ] Shows "already running" - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs
- [ ] Exit code 0 - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs

### AC#3: Daemon Stop Command

- [ ] Sends shutdown signal - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs
- [ ] Waits for shutdown - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs

### AC#4: Daemon Stop When Not Running

- [ ] Detects no daemon - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs
- [ ] Exit code 0 - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs

### AC#5: Daemon Status Command

- [ ] Shows all metrics when running - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs
- [ ] Shows "not running" when stopped - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs
- [ ] Correct exit codes (0/1) - **Phase:** 3 - **Evidence:** tests/daemon_tests.rs

### AC#6: Manual Index Command

- [ ] Full index build - **Phase:** 3 - **Evidence:** tests/index_tests.rs
- [ ] Progress bar for large repos - **Phase:** 3 - **Evidence:** tests/index_tests.rs
- [ ] Completion summary - **Phase:** 3 - **Evidence:** tests/index_tests.rs

### AC#7: Force Re-index Flag

- [ ] --force bypasses hash cache - **Phase:** 3 - **Evidence:** tests/index_tests.rs

### AC#8: Auto-Detection

- [ ] Daemon query when available - **Phase:** 3 - **Evidence:** tests/search_tests.rs
- [ ] Direct index fallback - **Phase:** 3 - **Evidence:** tests/search_tests.rs
- [ ] On-demand index build - **Phase:** 3 - **Evidence:** tests/search_tests.rs

---

**Checklist Progress:** 0/18 items complete (0%)

---

## Definition of Done

### Implementation
- [x] `treelint daemon start` command
- [x] `treelint daemon stop` command
- [x] `treelint daemon status` command
- [x] `treelint index` command with progress bar
- [x] `treelint index --force` flag
- [x] Auto-detection logic in search command
- [x] Progress bar with indicatif
- [x] TTY detection for progress display

### Quality
- [x] All 8 acceptance criteria have passing tests
- [x] Edge cases covered (idempotent start/stop)
- [x] Exit codes correct
- [x] NFRs met (< 10ms auto-detection overhead)
- [x] Code coverage > 95% for daemon.rs, index.rs

### Testing
- [x] Unit tests for daemon commands
- [x] Unit tests for index command
- [x] Integration tests for auto-detection
- [x] Integration tests for daemon lifecycle

### Documentation
- [x] CLI --help updated with daemon subcommand
- [x] CLI --help updated with index command
- [x] Progress bar format documented

---

## Implementation Notes

- [x] `treelint daemon start` command - Completed: src/cli/commands/daemon.rs
- [x] `treelint daemon stop` command - Completed: src/cli/commands/daemon.rs
- [x] `treelint daemon status` command - Completed: src/cli/commands/daemon.rs
- [x] `treelint index` command with progress bar - Completed: src/cli/commands/index.rs
- [x] `treelint index --force` flag - Completed: src/cli/commands/index.rs
- [x] Auto-detection logic in search command - Completed: src/cli/commands/search.rs
- [x] Progress bar with indicatif - Completed: indicatif crate integration
- [x] TTY detection for progress display - Completed: atty crate integration
- [x] All 8 acceptance criteria have passing tests - Completed: 61 tests in tests/STORY-009/
- [x] Edge cases covered (idempotent start/stop) - Completed: test_ac2, test_ac4
- [x] Exit codes correct - Completed: 0 for success, 1 for daemon not running
- [x] NFRs met (< 10ms auto-detection overhead) - Completed: test_auto_detection_overhead_minimal
- [x] Code coverage > 95% for daemon.rs, index.rs - Completed: full coverage
- [x] Unit tests for daemon commands - Completed: tests/STORY-009/test_ac1-5
- [x] Unit tests for index command - Completed: tests/STORY-009/test_ac6-7
- [x] Integration tests for auto-detection - Completed: tests/STORY-009/test_ac8
- [x] Integration tests for daemon lifecycle - Completed: full lifecycle tests
- [x] CLI --help updated with daemon subcommand - Completed: clap derive
- [x] CLI --help updated with index command - Completed: clap derive
- [x] Progress bar format documented - Completed: inline comments

**Developer:** DevForgeAI AI Agent
**Implemented:** 2026-01-30

## Change Log

**Current Status:** Released

| Date | Author | Phase/Action | Change | Files Affected |
|------|--------|--------------|--------|----------------|
| 2026-01-27 13:30 | claude/story-creation | Created | Story created from EPIC-002 F4 (split 3/3) | STORY-009-daemon-cli-commands.story.md |
| 2026-01-30 | claude/qa-result-interpreter | QA Deep | PASSED: 61 tests pass, 0 violations, 2/2 validators | - |
| 2026-01-30 | claude/deployment-engineer | Released | Deployed v0.1.0 to test environment | STORY-009-release-notes.md |

## Notes

**Design Decisions:**
- Daemon start/stop are idempotent (exit 0 even if already running/stopped)
- Progress bar only for large repos to avoid clutter
- Auto-detection is transparent to user (same output regardless of path)

**Related ADRs:**
- Reference: EPIC-002 Auto-Detection Logic (lines 153-163)

---

Story Template Version: 2.7
Last Updated: 2026-01-27
