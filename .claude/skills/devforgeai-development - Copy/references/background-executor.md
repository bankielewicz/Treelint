# Background Execution Patterns for TDD Workflow

**Story:** STORY-112
**Version:** 1.0
**Purpose:** Run tests in background while waiting efficiently for results

---

## Quick Reference

| Attribute | Value |
|-----------|-------|
| **Trigger** | Test execution in Phase 03 (Green) |
| **Background Threshold** | > 2 minutes (120000ms) estimated duration |
| **Foreground Threshold** | < 30 seconds (30000ms) estimated duration |
| **Fallback** | Sequential foreground if background fails |
| **Config** | devforgeai/config/parallel-orchestration.yaml |
| **Performance Target** | 50-80% reduction in perceived wait time |

---

## When to Use Background Execution

### Threshold Decision Logic

| Estimated Duration | Execution Mode | Rationale |
|-------------------|----------------|-----------|
| < 30 seconds | Foreground | Overhead not worth parallelization |
| 30s - 2 minutes | Foreground (default) | Wait time acceptable |
| > 2 minutes | **Background** | Terminal not blocked, efficient wait |

### Duration Estimation Heuristics

Estimate test duration based on:
1. **Historical data:** Previous test runs for this story/project
2. **Test count:** ~100ms per unit test (rough heuristic)
3. **Framework overhead:** pytest ~2s, jest ~1s, dotnet ~3s
4. **Integration tests:** ~500ms-2s per test

---

## Background Execution Algorithm

### Core Pattern (Simple - Zero Technical Debt)

**Token Overhead:** Zero additional tokens - same commands, same results, different timing.

```pseudocode
FUNCTION execute_tests_with_threshold(test_command: String):
    # Load timeout from parallel config
    config = Read(file_path="devforgeai/config/parallel-orchestration.yaml")
    timeout_ms = config.profiles[active_profile].timeout_ms  # Default: 120000

    # Check duration threshold
    estimated_duration = estimate_test_duration(test_command)

    IF estimated_duration > 120000:  # 2 minutes (BACKGROUND_THRESHOLD_MS)
        # Launch tests in background using Bash with run_in_background parameter
        task = Bash(command=test_command, run_in_background=true, timeout=timeout_ms)

        Display: "Tests running in background (task: {task.id})..."
        Display: "   Waiting efficiently for results..."

        # Wait for completion (simple approach - no parallel work)
        result = TaskOutput(
            task_id=task.id,
            block=true,
            timeout=timeout_ms
        )

        # Time savings: Terminal not blocked, results ready immediately
        RETURN result

    ELSE:
        # Foreground execution (standard)
        result = Bash(command=test_command)
        RETURN result
```

### Design Decision

**Simple wait approach chosen over parallel work:**
- Zero technical debt risk
- No coordination complexity
- Time savings from efficient terminal handling
- Results retrieved immediately when available

---

## TaskOutput Integration

### Blocking Retrieval Pattern

```pseudocode
# MANDATORY before phase transitions
result = TaskOutput(
    task_id=background_task.id,
    block=true,
    timeout=timeout_ms
)

IF result.status == "completed":
    # Process test results
    parse_test_output(result.output)

ELIF result.status == "timeout":
    # Handle timeout per STORY-110 patterns
    handle_timeout(result)

ELIF result.status == "error":
    # Handle error - fall back to foreground
    handle_error(result)
```

---

## Error Handling Integration

### Timeout Handling (from STORY-110 timeout-handling.md)

```pseudocode
IF elapsed > timeout_ms:
    # Task exceeded timeout
    log_timeout(task_id, elapsed, timeout_ms)

    # Graceful termination
    KillShell(shell_id=task_id)

    # Return timeout failure
    RETURN TaskFailure(
        task_id=task_id,
        error_type="Timeout",
        error_message="Test execution exceeded timeout ({timeout_ms}ms)",
        is_retryable=true
    )
```

### Sequential Fallback Pattern

```pseudocode
# If background test fails, fall back to foreground
IF background_result.status == "failed" OR background_result.status == "timeout":
    Display: "Background tests failed, retrying in foreground..."

    # Sequential fallback (from STORY-110 sequential-fallback.md)
    foreground_result = Bash(
        command=test_command,
        run_in_background=false,
        timeout=timeout_ms
    )

    RETURN foreground_result
```

---

## Configuration Integration

### Loading Parallel Config

```pseudocode
# Load from STORY-108 parallel configuration
config = Read(file_path="devforgeai/config/parallel-orchestration.yaml")

# Extract timeout settings
timeout_ms = config.profiles[config.default_profile].timeout_ms

# Profile-specific timeouts:
# - pro:  120000ms (2 minutes)
# - max:  180000ms (3 minutes)
# - api:  300000ms (5 minutes)
```

---

## Performance Analysis

### Time Savings Model

**Without Background (Blocked Terminal):**
- Test runs: 180 seconds
- Terminal blocked: 180 seconds
- Perceived wait: 180 seconds

**With Background (Efficient Wait):**
- Test runs: 180 seconds
- Terminal responsive during run
- Results ready immediately when complete
- Perceived improvement: Terminal not frozen

### Token Overhead

**Zero additional token consumption:**
- Same commands executed
- Same results returned
- Only difference is execution timing

---

## Related Documentation

- `parallel-context-loader.md` - Parallel Read patterns for context files
- `task-result-aggregation.md` - TaskOutput integration patterns
- `error-handling-patterns.md` (orchestration) - Partial failure recovery
- `timeout-handling.md` (orchestration) - Timeout management
- `sequential-fallback.md` (orchestration) - Fallback when parallel fails
