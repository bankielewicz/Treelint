# Timeout Handling for Parallel Orchestration

**Story:** STORY-110
**Version:** 1.0
**Epic:** EPIC-017 (Parallel Task Orchestration)

---

## Quick Reference

| Pattern | Purpose | Key Tool |
|---------|---------|----------|
| Timeout Monitoring | Track task duration | `TaskOutput(block=false)` |
| Task Termination | Kill hung tasks | `KillShell(shell_id)` |
| Timeout Logging | Structured timeout logs | Correlation ID + duration |

**NFR-002:** Timeout detection latency < 1 second

---

## Overview

Parallel tasks may hang or exceed their allocated time. This document defines patterns for:

1. **Monitoring task duration** against configured `timeout_ms`
2. **Terminating timed-out tasks** via `KillShell`
3. **Logging timeout events** with full context

**Configuration Reference:** See `parallel-config.md` and `devforgeai/config/parallel-orchestration.yaml`

---

## Timeout Monitoring Pattern

### Purpose

Track elapsed time for each parallel task and detect when `timeout_ms` is exceeded.

### Configuration

```yaml
# From parallel-orchestration.yaml
profiles:
  pro:
    timeout_ms: 120000    # 2 minutes
  max:
    timeout_ms: 180000    # 3 minutes
  api:
    timeout_ms: 300000    # 5 minutes

timeout_settings:
  check_interval_ms: 1000     # Poll every 1 second
  grace_period_ms: 5000       # Extra 5s before force kill
  detection_latency_ms: 1000  # NFR-002 requirement
```

### Monitoring Algorithm

```
FUNCTION monitor_task_timeout(task_id: String, timeout_ms: Int):
    start_time = current_time_ms()

    WHILE task.is_running:
        elapsed = current_time_ms() - start_time

        IF elapsed > timeout_ms:
            # Timeout detected
            log_timeout(task_id, elapsed, timeout_ms)
            terminate_task(task_id)
            RETURN TimeoutResult(
                task_id=task_id,
                elapsed_ms=elapsed,
                timeout_ms=timeout_ms,
                terminated=true
            )

        # Check task status (non-blocking)
        status = TaskOutput(task_id=task_id, block=false)
        IF status.completed:
            RETURN status

        # Wait before next check (NFR-002: < 1 second latency)
        sleep(check_interval_ms)  # 1000ms default

    RETURN task.result
```

### Parallel Timeout Monitoring

When monitoring multiple parallel tasks:

```
FUNCTION monitor_parallel_tasks(tasks: List[Task], timeout_ms: Int):
    start_time = current_time_ms()
    pending = set(tasks)
    completed = []
    timed_out = []

    WHILE pending AND (current_time_ms() - start_time) < timeout_ms:
        FOR task IN pending.copy():
            status = TaskOutput(task_id=task.id, block=false, timeout=100)

            IF status.completed:
                completed.append(task)
                pending.remove(task)

        # Check overall elapsed time
        elapsed = current_time_ms() - start_time
        IF elapsed > timeout_ms:
            # Terminate all remaining tasks
            FOR task IN pending:
                timed_out.append(task)
                terminate_task(task.id)
            BREAK

        sleep(check_interval_ms)

    RETURN (completed, timed_out)
```

---

## KillShell Integration

### Purpose

Terminate background tasks that exceed `timeout_ms` using the `KillShell` tool.

### KillShell Tool Usage

```
# KillShell terminates a running background shell by ID
KillShell(shell_id="task-abc123")

# Returns:
# - Success: "Shell task-abc123 terminated"
# - Failure: "Shell task-abc123 not found" or "Already terminated"
```

### Termination Workflow

```
FUNCTION terminate_task(task_id: String):
    # Step 1: Attempt graceful termination
    result = KillShell(shell_id=task_id)

    IF result.success:
        log_termination(task_id, "graceful")
        RETURN TerminationResult(success=true, method="graceful")

    # Step 2: If still running after grace period, force kill
    sleep(grace_period_ms)  # 5000ms default

    IF task_still_running(task_id):
        result = KillShell(shell_id=task_id)  # Retry
        log_termination(task_id, "forced")
        RETURN TerminationResult(success=result.success, method="forced")

    RETURN TerminationResult(success=true, method="already_terminated")
```

### Integration with Task Tool

When launching background tasks that need timeout protection:

```python
# Launch task in background
task = Task(
    description="Analyze codebase",
    prompt="Search for security vulnerabilities...",
    subagent_type="Explore",
    run_in_background=true
)

# Monitor with timeout
start_time = now()
timeout_ms = config.profiles[current_profile].timeout_ms

WHILE true:
    elapsed = now() - start_time

    # Check for timeout
    IF elapsed > timeout_ms:
        Display: f"Task {task.id} exceeded timeout ({timeout_ms}ms)"
        KillShell(shell_id=task.id)
        BREAK

    # Check for completion
    result = TaskOutput(task_id=task.id, block=false)
    IF result.completed:
        RETURN result

    sleep(1000)  # 1 second poll interval
```

---

## Timeout Logging Pattern

### Purpose

Provide structured logs for timeout events with full context for debugging.

### Log Format

```
[TIMEOUT] {correlation_id}
  task_id: {task_id}
  duration_ms: {actual_elapsed_ms}
  timeout_ms: {configured_timeout_ms}
  exceeded_by_ms: {duration_ms - timeout_ms}
  termination_method: {graceful|forced|failed}
  timestamp: {ISO-8601}
  profile: {pro|max|api|custom}
```

### Example Timeout Log

```
[TIMEOUT] corr-abc123
  task_id: explore-agent-3
  duration_ms: 125000
  timeout_ms: 120000
  exceeded_by_ms: 5000
  termination_method: graceful
  timestamp: 2025-12-18T14:35:42Z
  profile: pro
```

### Log Aggregation

```
# Find all timeouts for a correlation ID
grep "[TIMEOUT] corr-abc123" logs/parallel.log

# Find all timeouts exceeding threshold by >10 seconds
grep "exceeded_by_ms: [0-9]\{5,\}" logs/parallel.log

# Count timeouts by profile
grep -o "profile: [a-z]*" logs/parallel.log | sort | uniq -c
```

---

## Examples

### Example 1: Single Task Timeout

```python
# Task exceeds 2-minute timeout
task = Task(
    description="Deep analysis",
    prompt="Analyze entire codebase for patterns...",
    run_in_background=true
)

# After 125 seconds...
# [TIMEOUT] task-xyz789
#   duration_ms: 125000
#   timeout_ms: 120000
#   exceeded_by_ms: 5000

KillShell(shell_id="task-xyz789")
# Output: Shell task-xyz789 terminated

# Create TaskFailure for aggregation
failure = TaskFailure(
    task_id="task-xyz789",
    error_type="Timeout",
    error_message="Task exceeded timeout_ms (120000ms)",
    retry_count=0,
    is_retryable=true
)
```

### Example 2: Parallel Batch with Mixed Timeouts

```python
# Launch 5 parallel tasks with 3-minute timeout
tasks = [
    Task("Analyze security"),      # Completes in 45s
    Task("Analyze performance"),   # Completes in 60s
    Task("Analyze complexity"),    # Completes in 90s
    Task("Analyze dependencies"),  # TIMEOUT at 180s
    Task("Analyze documentation")  # TIMEOUT at 180s
]

# Result after monitoring:
completed = [task_1, task_2, task_3]  # 3 successful
timed_out = [task_4, task_5]          # 2 terminated

# Timeout logs:
# [TIMEOUT] corr-batch001 task_id: task-4 duration_ms: 180500
# [TIMEOUT] corr-batch001 task_id: task-5 duration_ms: 180800
```

---

## Configuration Reference

### Profile Timeouts

| Profile | timeout_ms | Use Case |
|---------|------------|----------|
| pro | 120000 (2 min) | Standard development |
| max | 180000 (3 min) | Complex analysis |
| api | 300000 (5 min) | Large codebase operations |
| custom | User-defined | Special requirements |

### Timeout Tuning

```yaml
# Increase timeout for specific operations
timeout_overrides:
  full_codebase_analysis: 600000  # 10 minutes
  quick_file_check: 30000         # 30 seconds
  integration_test: 900000        # 15 minutes
```

---

## Troubleshooting

### Issue: Tasks Timing Out Frequently

**Symptoms:** Multiple timeout logs, low success rate

**Diagnosis:**
1. Check if timeout_ms is appropriate for task complexity
2. Review task prompts for scope creep
3. Check system resource availability

**Resolution:**
- Increase `timeout_ms` in profile configuration
- Split large tasks into smaller parallel chunks
- Switch to higher tier profile (e.g., pro → max)

### Issue: KillShell Fails

**Symptoms:** Task continues running after KillShell

**Diagnosis:**
1. Verify shell_id matches task ID
2. Check if task already completed
3. Review grace_period_ms setting

**Resolution:**
- Retry KillShell after grace period
- Check TaskOutput for actual status
- File bug if persistent

### Issue: Timeout Detection Latency > 1 Second

**Symptoms:** NFR-002 violation, tasks run significantly past timeout

**Diagnosis:**
1. Check check_interval_ms setting
2. Review system load during parallel execution
3. Verify polling loop efficiency

**Resolution:**
- Reduce check_interval_ms (minimum 500ms)
- Optimize polling loop
- Review parallel task count vs. system capacity

---

## Related Documentation

- `parallel-config.md` - Configuration reference
- `error-handling-patterns.md` - Partial failure recovery
- `retry-patterns.md` - Retry logic for timed-out tasks
- `sequential-fallback.md` - Fallback when parallel fails
- `devforgeai/config/parallel-orchestration.yaml` - Configuration file

---

**Last Updated:** 2025-12-18
**Story:** STORY-110
