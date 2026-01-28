# Sequential Fallback for Parallel Orchestration

**Story:** STORY-110
**Version:** 1.0
**Epic:** EPIC-017 (Parallel Task Orchestration)

---

## Quick Reference

| Pattern | Purpose | Trigger |
|---------|---------|---------|
| Complete Failure Detection | Identify 0% success | `success_rate = 0` |
| Sequential Re-execution | Fallback mode | All parallel tasks failed |
| Fallback Logging | Detailed diagnostics | Mode transition + per-task logs |

**Business Rule BR-004:** Fallback to sequential only when ALL parallel tasks fail (success_rate = 0)

---

## Overview

When all parallel tasks fail, the system can fall back to sequential execution as a last resort. This document defines patterns for:

1. **Detecting complete failure** - All parallel tasks failed
2. **Re-executing sequentially** - Run tasks one-by-one
3. **Logging fallback operations** - Detailed diagnostics

**Configuration Reference:** See `parallel-config.md` and `devforgeai/config/parallel-orchestration.yaml`

---

## Complete Failure Detection

### Purpose

Identify when all parallel tasks have failed and fallback should be triggered.

### Business Rule BR-004

> Fallback to sequential only when ALL parallel tasks fail (success_rate = 0)

### Configuration

```yaml
# From parallel-orchestration.yaml
fallback:
  fallback_to_sequential: true   # Enable sequential fallback
  min_parallel_attempts: 1       # Minimum parallel attempts before fallback
  sequential_timeout_ms: 300000  # 5 minutes per task in sequential mode
```

### Detection Algorithm

```
FUNCTION detect_complete_failure(partial_result: PartialResult) -> Boolean:
    # BR-004: Fallback only when ALL tasks fail
    IF partial_result.success_rate == 0:
        RETURN true

    # Any success means we don't fallback
    IF len(partial_result.successes) > 0:
        RETURN false

    # Double-check: all tasks in failures list
    IF len(partial_result.failures) == partial_result.total_tasks:
        RETURN true

    RETURN false
```

### Fallback Decision Flow

```
Parallel Execution Complete
            │
            ▼
    ┌───────────────┐
    │ Count Results │
    └───────┬───────┘
            │
            ▼
    ┌───────────────┐
    │success_rate=0?│
    └───────┬───────┘
            │
       ┌────┴────┐
       │         │
       ▼         ▼
      Yes        No
       │         │
       ▼         ▼
  ┌─────────┐  ┌─────────────┐
  │Check    │  │Continue with│
  │fallback │  │partial      │
  │enabled  │  │results      │
  └────┬────┘  └─────────────┘
       │
   ┌───┴───┐
   │       │
   ▼       ▼
Enabled  Disabled
   │       │
   ▼       ▼
┌───────┐ ┌──────────┐
│Fallback│ │HALT with │
│to seq  │ │error     │
└───────┘ └──────────┘
```

### Example: 0/5 Success Triggers Fallback

```python
# All 5 parallel tasks failed
partial_result = PartialResult(
    successes=[],
    failures=[task_1, task_2, task_3, task_4, task_5],
    total_tasks=5,
    success_rate=0.0  # 0%
)

# Detection
if detect_complete_failure(partial_result):
    # success_rate = 0 → Trigger fallback
    if config.fallback.fallback_to_sequential:
        log_fallback_triggered(partial_result)
        execute_sequential_fallback(original_tasks)
    else:
        HALT("All parallel tasks failed, fallback disabled")
```

---

## Sequential Re-execution Pattern

### Purpose

Re-execute failed tasks one at a time when parallel execution completely fails.

### Algorithm

```
FUNCTION execute_sequential_fallback(tasks: List[Task]) -> PartialResult:
    log_fallback_start(len(tasks))

    successes = []
    failures = []

    FOR i, task IN enumerate(tasks):
        log_sequential_task_start(task.id, i + 1, len(tasks))

        TRY:
            # Execute single task (blocking)
            result = execute_task_with_timeout(
                task,
                timeout_ms=config.fallback.sequential_timeout_ms
            )

            successes.append(result)
            log_sequential_task_success(task.id, i + 1)

        CATCH error:
            failure = TaskFailure(
                task_id=task.id,
                error_type=classify_error(error),
                error_message=error.message,
                retry_count=0,
                is_retryable=is_retryable(error)
            )
            failures.append(failure)
            log_sequential_task_failure(task.id, i + 1, error)

    log_fallback_complete(len(successes), len(failures))

    RETURN PartialResult(
        successes=successes,
        failures=failures,
        total_tasks=len(tasks),
        success_rate=len(successes) / len(tasks)
    )
```

### Sequential Execution Characteristics

| Aspect | Parallel | Sequential Fallback |
|--------|----------|---------------------|
| Execution | Concurrent | One-at-a-time |
| Timeout | Per profile | sequential_timeout_ms |
| Resource usage | High | Low |
| Error isolation | Independent | Can diagnose step-by-step |
| Speed | Fast (when working) | Slower but more reliable |

### Fallback Benefits

1. **Reduced Resource Contention** - Only one task runs at a time
2. **Better Error Isolation** - Each failure is independent
3. **Incremental Progress** - Some tasks may succeed
4. **Debugging Visibility** - Clear per-task logging

---

## Fallback Logging Pattern

### Purpose

Provide detailed logs during sequential fallback for diagnostics and debugging.

### Log Format: Fallback Start

```
[FALLBACK_TRIGGERED] {correlation_id}
  reason: complete_parallel_failure
  parallel_tasks: {count}
  parallel_failures: {failure_summary}
  fallback_mode: sequential
  timestamp: {ISO-8601}
```

### Log Format: Sequential Task Execution

```
[SEQUENTIAL_TASK] {correlation_id}
  task_id: {task_id}
  position: {n}/{total}
  status: {starting|completed|failed}
  duration_ms: {elapsed}
  error: {error_message if failed}
  timestamp: {ISO-8601}
```

### Log Format: Fallback Complete

```
[FALLBACK_COMPLETE] {correlation_id}
  mode: sequential
  total_tasks: {count}
  successes: {success_count}
  failures: {failure_count}
  success_rate: {percentage}
  total_duration_ms: {elapsed}
  outcome: {success|partial_success|complete_failure}
  timestamp: {ISO-8601}
```

### Example Fallback Log Sequence

```
[FALLBACK_TRIGGERED] corr-xyz789
  reason: complete_parallel_failure
  parallel_tasks: 5
  parallel_failures: [timeout, timeout, rate_limit, network, timeout]
  fallback_mode: sequential
  timestamp: 2025-12-18T15:00:00Z

[SEQUENTIAL_TASK] corr-xyz789
  task_id: task-1
  position: 1/5
  status: starting
  timestamp: 2025-12-18T15:00:01Z

[SEQUENTIAL_TASK] corr-xyz789
  task_id: task-1
  position: 1/5
  status: completed
  duration_ms: 45000
  timestamp: 2025-12-18T15:00:46Z

[SEQUENTIAL_TASK] corr-xyz789
  task_id: task-2
  position: 2/5
  status: starting
  timestamp: 2025-12-18T15:00:47Z

[SEQUENTIAL_TASK] corr-xyz789
  task_id: task-2
  position: 2/5
  status: completed
  duration_ms: 38000
  timestamp: 2025-12-18T15:01:25Z

... (tasks 3-4 similar)

[SEQUENTIAL_TASK] corr-xyz789
  task_id: task-5
  position: 5/5
  status: failed
  duration_ms: 120000
  error: "Resource unavailable after timeout"
  timestamp: 2025-12-18T15:05:45Z

[FALLBACK_COMPLETE] corr-xyz789
  mode: sequential
  total_tasks: 5
  successes: 4
  failures: 1
  success_rate: 80%
  total_duration_ms: 284000
  outcome: partial_success
  timestamp: 2025-12-18T15:05:46Z
```

---

## Examples

### Example 1: Successful Sequential Recovery

```python
# Parallel execution: 0/3 success
parallel_result = PartialResult(
    successes=[],
    failures=[task_1, task_2, task_3],
    total_tasks=3,
    success_rate=0.0
)

# Trigger fallback
if detect_complete_failure(parallel_result):
    # [FALLBACK_TRIGGERED] reason: complete_parallel_failure

    sequential_result = execute_sequential_fallback([task_1, task_2, task_3])
    # Task 1: success (resource now available)
    # Task 2: success
    # Task 3: success

    # [FALLBACK_COMPLETE] successes: 3, failures: 0, outcome: success
```

### Example 2: Partial Recovery in Sequential Mode

```python
# Parallel: 0/5 success (all timeouts)
# Sequential fallback:
#   Task 1: success
#   Task 2: success
#   Task 3: failure (permanent error)
#   Task 4: success
#   Task 5: success

sequential_result = PartialResult(
    successes=[task_1, task_2, task_4, task_5],
    failures=[task_3],
    total_tasks=5,
    success_rate=0.8  # 80% - much better than 0%
)

# [FALLBACK_COMPLETE] outcome: partial_success
```

### Example 3: Complete Failure Even in Sequential

```python
# Parallel: 0/3 success
# Sequential: 0/3 success (fundamental issue)

sequential_result = PartialResult(
    successes=[],
    failures=[task_1, task_2, task_3],
    total_tasks=3,
    success_rate=0.0
)

# [FALLBACK_COMPLETE] outcome: complete_failure
Display: "ERROR: All tasks failed in both parallel and sequential modes"
Display: "Review task configuration and external dependencies"
HALT
```

---

## Configuration Reference

### Fallback Settings

```yaml
# parallel-orchestration.yaml
fallback:
  fallback_to_sequential: true   # Enable/disable fallback
  min_parallel_attempts: 1       # Attempts before fallback
  sequential_timeout_ms: 300000  # 5 min per task
  max_sequential_tasks: 10       # Safety limit
```

### Disabling Fallback

```yaml
fallback:
  fallback_to_sequential: false  # HALT on complete failure
```

When disabled, complete parallel failure results in immediate HALT with error message.

---

## Performance Considerations

### Parallel vs. Sequential Time

| Tasks | Parallel (success) | Sequential |
|-------|-------------------|------------|
| 3 | ~120s (max of 3) | ~360s (sum of 3) |
| 5 | ~180s (max of 5) | ~600s (sum of 5) |
| 10 | ~300s (max of 10) | ~1200s (sum of 10) |

**Note:** Sequential is 3-5x slower but more likely to succeed when resources are constrained.

### When Fallback Helps

1. **Resource Contention** - Too many parallel tasks overwhelmed a service
2. **Rate Limiting** - Parallel requests triggered throttling
3. **Memory Pressure** - Sequential reduces memory footprint
4. **Network Saturation** - One-at-a-time uses less bandwidth

### When Fallback Doesn't Help

1. **Fundamental Errors** - Missing files, invalid config
2. **Authentication Issues** - Credentials are wrong
3. **External Service Down** - No amount of retrying helps

---

## Troubleshooting

### Issue: Fallback Not Triggering

**Symptoms:** Complete parallel failure but no fallback

**Diagnosis:**
1. Check `fallback_to_sequential` is true
2. Verify success_rate is exactly 0
3. Check if any task partially succeeded

**Resolution:**
- Enable fallback in configuration
- Review partial success detection logic

### Issue: Fallback Too Slow

**Symptoms:** Sequential execution takes too long

**Diagnosis:**
1. Check `sequential_timeout_ms` setting
2. Count total tasks being retried
3. Review individual task durations

**Resolution:**
- Reduce `sequential_timeout_ms`
- Limit `max_sequential_tasks`
- Consider aborting after N sequential failures

### Issue: Sequential Also Fails

**Symptoms:** Fallback complete but still 0% success

**Diagnosis:**
1. Review individual task errors in logs
2. Check for common root cause
3. Verify external dependencies

**Resolution:**
- Fix underlying issue (config, auth, resources)
- Check external service availability
- Review task requirements

---

## Related Documentation

- `parallel-config.md` - Configuration reference
- `error-handling-patterns.md` - Partial failure recovery
- `timeout-handling.md` - Timeout monitoring
- `retry-patterns.md` - Retry logic for transient failures
- `devforgeai/config/parallel-orchestration.yaml` - Configuration file

---

**Last Updated:** 2025-12-18
**Story:** STORY-110
