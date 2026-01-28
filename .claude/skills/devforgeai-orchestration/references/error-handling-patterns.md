# Error Handling Patterns for Parallel Orchestration

**Story:** STORY-110
**Version:** 1.0
**Epic:** EPIC-017 (Parallel Task Orchestration)

---

## Quick Reference

| Pattern | Purpose | Key Concept |
|---------|---------|-------------|
| Partial Failure Recovery | Continue with successful tasks | `success_rate >= min_success_rate` |
| Result Aggregation | Merge successes, track failures | `PartialResult` data model |
| Failure Logging | Structured error reporting | `TaskFailure` with correlation ID |

---

## Overview

When executing parallel tasks with the `Task` tool, some tasks may fail while others succeed. This document defines patterns for:

1. **Recovering from partial failures** - Continue workflow with successful results
2. **Aggregating mixed results** - Separate successes from failures
3. **Logging failures** - Structured format for diagnostics

**Key Principle:** One failed subagent should not block five successful ones.

**Configuration Reference:** See `parallel-config.md` and `devforgeai/config/parallel-orchestration.yaml`

---

## Partial Failure Recovery Pattern

### Purpose

Enable workflows to continue when some parallel tasks succeed even if others fail.

### Business Rule BR-001

> Partial success continues workflow if `success_rate >= min_success_rate`

**Threshold Configuration:**
```yaml
# From parallel-orchestration.yaml
error_handling:
  min_success_rate: 0.5  # 50% minimum success to continue
  fail_fast: false       # Don't abort on first failure
```

### Algorithm

```
FUNCTION handle_parallel_results(task_results: List[TaskOutput]):
    successes = []
    failures = []

    FOR result IN task_results:
        IF result.status == "completed":
            successes.append(result)
        ELSE:
            failures.append(create_task_failure(result))

    total_tasks = len(task_results)
    success_rate = len(successes) / total_tasks

    IF success_rate >= config.min_success_rate:
        # Continue workflow with partial results
        log_failures(failures)
        RETURN PartialResult(
            successes=successes,
            failures=failures,
            total_tasks=total_tasks,
            success_rate=success_rate
        )
    ELSE:
        # Threshold not met - HALT workflow
        HALT with error: "Success rate {success_rate} below threshold {min_success_rate}"
```

### Example: 3 Success + 2 Failure Scenario

**Scenario:** 5 parallel Explore agents launched, 3 complete successfully, 2 fail.

```
Parallel Task Execution:
├── Agent 1: ✓ Success (file analysis complete)
├── Agent 2: ✓ Success (code search complete)
├── Agent 3: ✗ Failure (timeout after 120s)
├── Agent 4: ✓ Success (pattern matching complete)
└── Agent 5: ✗ Failure (resource not found)

Result Calculation:
- total_tasks: 5
- successes: 3
- failures: 2
- success_rate: 3/5 = 0.60 (60%)

Decision (min_success_rate = 0.50):
- 0.60 >= 0.50 → Continue workflow
- Log failures for agent 3 and 5
- Process results from agents 1, 2, and 4
```

### When to Continue vs. HALT

| Scenario | Success Rate | min_success_rate | Action |
|----------|--------------|------------------|--------|
| 5/5 pass | 100% | 50% | Continue (all results) |
| 3/5 pass | 60% | 50% | Continue (partial) |
| 2/5 pass | 40% | 50% | **HALT** (below threshold) |
| 0/5 pass | 0% | 50% | **HALT** + fallback |

---

## Result Aggregation Pattern

### Purpose

Merge successful results while tracking failures for diagnostics and potential retry.

### PartialResult Data Model

```yaml
PartialResult:
  description: "Represents outcome of parallel execution with mixed success/failure"
  fields:
    - name: successes
      type: List[TaskResult]
      constraints: Required
      description: "Successfully completed task results"

    - name: failures
      type: List[TaskFailure]
      constraints: Required
      description: "Failed task details with error info"

    - name: total_tasks
      type: Int
      constraints: Required, >= 0
      description: "Total number of tasks attempted"
      validation: "total_tasks = len(successes) + len(failures)"

    - name: success_rate
      type: Float
      constraints: Required, 0.0-1.0
      description: "Percentage of tasks that succeeded"
      calculation: "len(successes) / total_tasks"
```

### Aggregation Algorithm

```
FUNCTION aggregate_parallel_results(task_outputs: List[TaskOutput]) -> PartialResult:
    successes = []
    failures = []

    FOR output IN task_outputs:
        IF output.status == "completed" AND output.error IS NULL:
            successes.append(TaskResult(
                task_id=output.task_id,
                result=output.result,
                duration_ms=output.duration_ms
            ))
        ELSE:
            failures.append(TaskFailure(
                task_id=output.task_id,
                error_type=classify_error(output.error),
                error_message=output.error_message,
                retry_count=output.retry_count OR 0,
                is_retryable=is_transient_error(output.error)
            ))

    total = len(task_outputs)
    rate = len(successes) / total IF total > 0 ELSE 0.0

    RETURN PartialResult(
        successes=successes,
        failures=failures,
        total_tasks=total,
        success_rate=rate
    )
```

### Result Merge Strategies

**Strategy 1: Concatenate Results**
```
merged_content = ""
FOR success IN partial_result.successes:
    merged_content += success.result + "\n---\n"
```

**Strategy 2: Structured Merge**
```
merged = {
    "sources": [s.task_id for s in successes],
    "content": [s.result for s in successes],
    "missing": [f.task_id for f in failures]
}
```

---

## Failure Logging Pattern

### Purpose

Provide structured, searchable logs for parallel task failures with correlation IDs.

### TaskFailure Data Model

```yaml
TaskFailure:
  description: "Represents a single task failure with diagnostic info"
  fields:
    - name: task_id
      type: String
      constraints: Required
      description: "Unique identifier for the failed task"
      example: "task-abc123"

    - name: error_type
      type: Enum(Timeout, TransientError, PermanentError, Unknown)
      constraints: Required
      description: "Classification of failure type"

    - name: error_message
      type: String
      constraints: Required
      description: "Human-readable error description"

    - name: retry_count
      type: Int
      constraints: Required, >= 0
      description: "Number of retry attempts made"

    - name: is_retryable
      type: Boolean
      constraints: Required
      description: "Whether error type allows retry"
      examples:
        - "Timeout: true"
        - "RateLimitError: true"
        - "ValidationError: false"
        - "AuthenticationError: false"
```

### Log Format

```
[PARALLEL_FAILURE] {correlation_id}
  task_id: {task_id}
  error_type: {Timeout|TransientError|PermanentError|Unknown}
  error_message: {message}
  retry_count: {count}
  is_retryable: {true|false}
  timestamp: {ISO-8601}
  duration_ms: {elapsed}
```

### Example Log Output

```
[PARALLEL_FAILURE] corr-xyz789
  task_id: explore-agent-3
  error_type: Timeout
  error_message: Task exceeded timeout_ms (120000ms)
  retry_count: 0
  is_retryable: true
  timestamp: 2025-12-18T14:32:15Z
  duration_ms: 120500

[PARALLEL_FAILURE] corr-xyz789
  task_id: explore-agent-5
  error_type: PermanentError
  error_message: Resource not found: /path/to/missing/file
  retry_count: 0
  is_retryable: false
  timestamp: 2025-12-18T14:32:18Z
  duration_ms: 3200
```

### Correlation ID Usage

All tasks in a parallel batch share a correlation ID for tracing:

```
correlation_id = generate_uuid()

parallel_batch:
  - task_1 (corr: abc123)
  - task_2 (corr: abc123)
  - task_3 (corr: abc123)

# Query all failures for this batch:
grep "[PARALLEL_FAILURE] abc123" logs/
```

---

## Examples

### Example 1: Successful Partial Recovery

```python
# 6 parallel code analysis tasks
results = await parallel_execute([
    Task("analyze security"),
    Task("analyze performance"),
    Task("analyze complexity"),
    Task("analyze coverage"),
    Task("analyze dependencies"),
    Task("analyze documentation")
])

# 4 succeed, 2 fail (timeout)
partial = aggregate_parallel_results(results)
# partial.success_rate = 0.67 (67%)
# partial.successes = [security, performance, complexity, coverage]
# partial.failures = [dependencies, documentation]

# 67% >= 50% threshold → Continue
if partial.success_rate >= config.min_success_rate:
    process_successful_analyses(partial.successes)
    log_for_retry(partial.failures)
```

### Example 2: Threshold Not Met

```python
# 5 parallel tasks, only 1 succeeds
partial = PartialResult(
    successes=[task_3],
    failures=[task_1, task_2, task_4, task_5],
    total_tasks=5,
    success_rate=0.20  # 20%
)

# 20% < 50% threshold → HALT
Display: "ERROR: Parallel execution below threshold"
Display: f"Success rate: {partial.success_rate * 100}%"
Display: f"Required: {config.min_success_rate * 100}%"
Display: "Failed tasks:"
for failure in partial.failures:
    Display: f"  - {failure.task_id}: {failure.error_message}"
HALT
```

---

## Troubleshooting

### Issue: All Tasks Failing

**Symptoms:** success_rate = 0, all tasks in failures list

**Diagnosis:**
1. Check if common dependency is unavailable
2. Verify task configuration is valid
3. Check for resource exhaustion

**Resolution:** See `sequential-fallback.md` for fallback pattern

### Issue: Intermittent Failures

**Symptoms:** Same task fails sometimes, succeeds other times

**Diagnosis:**
1. Check error_type - likely TransientError
2. Review retry_count - is retry enabled?

**Resolution:** See `retry-patterns.md` for retry configuration

### Issue: Timeout Failures

**Symptoms:** error_type = Timeout across multiple tasks

**Diagnosis:**
1. Check configured timeout_ms in parallel-orchestration.yaml
2. Compare actual task duration to limit

**Resolution:** See `timeout-handling.md` for timeout configuration

---

## Related Documentation

- `parallel-config.md` - Configuration reference for parallel orchestration
- `timeout-handling.md` - Timeout monitoring and KillShell integration
- `retry-patterns.md` - Retry logic for transient failures
- `sequential-fallback.md` - Fallback when all parallel tasks fail
- `devforgeai/config/parallel-orchestration.yaml` - Configuration file

---

**Last Updated:** 2025-12-18
**Story:** STORY-110
