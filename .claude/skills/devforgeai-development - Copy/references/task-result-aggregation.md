# Task Result Aggregation for Background Execution

**Story:** STORY-112
**Version:** 1.0
**Purpose:** Document TaskOutput patterns for retrieving and aggregating background task results

---

## Quick Reference

| Operation | Pattern | Use Case |
|-----------|---------|----------|
| Blocking retrieval | `TaskOutput(task_id, block=true)` | Before phase transitions |
| Non-blocking check | `TaskOutput(task_id, block=false, timeout=100)` | During implementation |
| Result aggregation | `aggregate_parallel_results([...])` | Multiple background tasks |

---

## TaskOutput Patterns

### Blocking Retrieval (Phase Transition)

Used before phase checkpoints to ensure background tasks complete:

```pseudocode
# MANDATORY: Call before any phase transition
result = TaskOutput(
    task_id=background_task.id,
    block=true,
    timeout=timeout_ms
)

IF result.status == "completed":
    process_test_results(result.output)
ELIF result.status == "timeout":
    handle_timeout(result)
ELIF result.status == "error":
    handle_error(result)
```

**Key Points:**
- `block=true` waits until task completes or timeout
- Must be called BEFORE proceeding to next phase
- Prevents race conditions in phase transitions

### Non-Blocking Status Check (Optional)

Used during work to check progress without waiting:

```pseudocode
# Optional: Check progress without blocking
status = TaskOutput(
    task_id=background_task.id,
    block=false,
    timeout=100
)

IF status.completed:
    Display: "Tests finished early!"
ELSE:
    Display: "Tests still running..."
```

---

## Result Aggregation

### For Multiple Background Tasks

When multiple background tasks run concurrently:

```pseudocode
FUNCTION aggregate_parallel_results(task_outputs: List[TaskOutput]) -> PartialResult:
    successes = []
    failures = []

    FOR output IN task_outputs:
        IF output.status == "completed" AND output.error IS NULL:
            successes.append(output)
        ELSE:
            failures.append(output)

    total = len(task_outputs)
    rate = len(successes) / total IF total > 0 ELSE 0.0

    RETURN PartialResult(
        successes=successes,
        failures=failures,
        total_tasks=total,
        success_rate=rate
    )
```

### PartialResult Data Model

```yaml
PartialResult:
  successes: List[TaskOutput]   # Successfully completed tasks
  failures: List[TaskOutput]    # Failed or timed out tasks
  total_tasks: Integer          # Total tasks in batch
  success_rate: Float           # 0.0 to 1.0
```

---

## Phase Transition Checkpoints

### Checkpoint Pattern

**CRITICAL:** All phase transitions MUST wait for background tasks:

```pseudocode
# BEFORE moving from Phase 03 (Green) to Phase 04 (Refactor)
IF background_task_running:
    Display: "Waiting for background tests to complete..."

    result = TaskOutput(
        task_id=background_task.id,
        block=true,
        timeout=timeout_ms
    )

    # Validate result before proceeding
    IF result.exit_code != 0:
        HALT: "Tests failed - cannot proceed to Phase 04"

    Display: "Background tests complete - proceeding to Phase 04"
```

### Why This Matters

- Prevents advancing with failing tests
- Ensures test results are available for decision making
- Maintains TDD workflow integrity

---

## Error Status Handling

### Timeout Handling

```pseudocode
IF result.status == "timeout":
    Log: "[BACKGROUND_TIMEOUT] Task {task_id} exceeded {timeout_ms}ms"

    # Terminate hung task
    KillShell(shell_id=task_id)

    # Create failure record
    failure = TaskFailure(
        task_id=task_id,
        error_type="Timeout",
        error_message="Exceeded timeout ({timeout_ms}ms)",
        is_retryable=true
    )

    # Consider sequential fallback
    IF should_retry:
        RETURN execute_foreground(command)
```

### Failure Handling

```pseudocode
IF result.status == "error" OR result.exit_code != 0:
    Log: "[BACKGROUND_FAILURE] Task {task_id} failed: {result.error}"

    failure = TaskFailure(
        task_id=task_id,
        error_type=classify_error(result.error),
        error_message=result.error,
        is_retryable=is_transient_error(result.error)
    )

    # Sequential fallback for transient errors
    IF failure.is_retryable:
        Display: "Retrying in foreground..."
        RETURN execute_foreground(command)
```

### TaskFailure Data Model

```yaml
TaskFailure:
  task_id: String
  error_type: Enum[Timeout, TransientError, PermanentError, Unknown]
  error_message: String
  retry_count: Integer
  is_retryable: Boolean
```

---

## Token Overhead Analysis

### Zero Additional Tokens

Background execution with TaskOutput:
- Same commands executed
- Same output captured
- Same result parsing
- No duplicate API calls

**Comparison:**
| Approach | API Calls | Token Usage |
|----------|-----------|-------------|
| Foreground (blocking) | 1 | X tokens |
| Background + TaskOutput | 1 + 1 | X tokens (same content) |

The second TaskOutput call retrieves cached results, not re-executing.

---

## Integration with TDD Phases

### Phase 03 (Green) Integration

```markdown
### Step 4a: Background Test Execution [CONDITIONAL]

IF estimated_duration > BACKGROUND_THRESHOLD:
    # Launch in background
    task = Bash(command=TEST_CMD, run_in_background=true)

    # Wait efficiently (simple approach)
    Display: "Tests running in background..."

    # WAIT BEFORE CHECKPOINT
    result = TaskOutput(task_id=task.id, block=true)

    # Validate tests passed
    IF result.exit_code != 0:
        HALT: "Tests failed"
```

---

## Related Documentation

- `background-executor.md` - Background execution patterns and thresholds
- `parallel-context-loader.md` - Parallel Read patterns
- `tdd-green-phase.md` - Phase 03 workflow
- `error-handling-patterns.md` (orchestration) - Error classification
- `timeout-handling.md` (orchestration) - Timeout management
