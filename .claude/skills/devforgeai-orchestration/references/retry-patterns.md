# Retry Patterns for Parallel Orchestration

**Story:** STORY-110
**Version:** 1.0
**Epic:** EPIC-017 (Parallel Task Orchestration)

---

## Quick Reference

| Pattern | Purpose | Formula |
|---------|---------|---------|
| Exponential Backoff | Prevent thundering herd | `delay = base_delay_ms * (2 ^ attempt)` |
| Error Classification | Determine retryability | Transient vs Permanent |
| Max Attempts | Limit retry count | Stop after N attempts |

**Business Rules:**
- BR-002: Transient errors are retried; permanent errors are not
- BR-003: Exponential backoff doubles delay, capped at `max_backoff_ms`

---

## Overview

When parallel tasks fail, some failures are transient (network timeout, rate limit) and can be retried, while others are permanent (validation error, missing resource) and should not be retried.

This document defines patterns for:

1. **Classifying errors** as transient or permanent
2. **Implementing exponential backoff** to prevent overload
3. **Respecting max attempts** to avoid infinite loops

**Configuration Reference:** See `parallel-config.md` and `devforgeai/config/parallel-orchestration.yaml`

---

## Exponential Backoff Pattern

### Purpose

Increase delay between retry attempts to reduce load on failing resources and prevent thundering herd problems.

### Formula

```
delay_ms = base_delay_ms * (2 ^ attempt_number)
```

Where:
- `base_delay_ms` = Starting delay (default: 1000ms)
- `attempt_number` = Current retry attempt (0, 1, 2, ...)
- Cap at `max_backoff_ms` (default: 10000ms)

### Business Rule BR-003

> Exponential backoff doubles delay on each retry, capped at `max_backoff_ms` (10 seconds default)

### Configuration

```yaml
# From parallel-orchestration.yaml
retry:
  enabled: true
  max_attempts: 3
  base_delay_ms: 1000     # 1 second
  max_backoff_ms: 10000   # 10 seconds (cap)
  jitter: true            # Add randomness to prevent sync
```

### Delay Calculation Examples

| Attempt | Formula | Delay (ms) | Capped |
|---------|---------|------------|--------|
| 0 | 1000 * 2^0 | 1000 | No |
| 1 | 1000 * 2^1 | 2000 | No |
| 2 | 1000 * 2^2 | 4000 | No |
| 3 | 1000 * 2^3 | 8000 | No |
| 4 | 1000 * 2^4 | 16000 → 10000 | Yes |
| 5 | 1000 * 2^5 | 32000 → 10000 | Yes |

### Algorithm with Jitter

```
FUNCTION calculate_backoff(attempt: Int, config: RetryConfig) -> Int:
    # Calculate base exponential delay
    delay = config.base_delay_ms * (2 ^ attempt)

    # Apply cap
    IF delay > config.max_backoff_ms:
        delay = config.max_backoff_ms

    # Add jitter (±25%) to prevent synchronized retries
    IF config.jitter:
        jitter_range = delay * 0.25
        delay = delay + random(-jitter_range, +jitter_range)

    RETURN delay
```

---

## Error Classification

### Purpose

Determine whether an error is retryable (transient) or should fail immediately (permanent).

### Business Rule BR-002

> Transient errors are retried; permanent errors are not

### Error Type Enum

```yaml
ErrorType:
  - Timeout         # Retryable - task exceeded time limit
  - TransientError  # Retryable - temporary failure
  - PermanentError  # Not retryable - fundamental issue
  - Unknown         # Default to not retryable (safe)
```

### Classification Table

| Error | Type | Retryable | Reason |
|-------|------|-----------|--------|
| Network timeout | Transient | Yes | Network may recover |
| Rate limit (429) | Transient | Yes | Wait and retry |
| Service unavailable (503) | Transient | Yes | Service may recover |
| Task timeout | Timeout | Yes | May succeed with more time |
| Connection reset | Transient | Yes | Network glitch |
| Validation error | Permanent | No | Input is invalid |
| File not found | Permanent | No | Resource doesn't exist |
| Permission denied | Permanent | No | Auth issue won't auto-fix |
| Syntax error | Permanent | No | Code is broken |
| Authentication failed | Permanent | No | Credentials are wrong |

### Classification Algorithm

```
FUNCTION classify_error(error: Error) -> ErrorType:
    message = error.message.lower()

    # Timeout patterns
    IF "timeout" IN message OR "exceeded" IN message:
        RETURN ErrorType.Timeout

    # Transient patterns (network, rate limits)
    transient_patterns = [
        "rate limit", "429", "503", "502",
        "network", "connection", "temporary",
        "unavailable", "retry", "overload"
    ]
    FOR pattern IN transient_patterns:
        IF pattern IN message:
            RETURN ErrorType.TransientError

    # Permanent patterns
    permanent_patterns = [
        "not found", "404", "invalid", "validation",
        "permission", "denied", "401", "403",
        "syntax", "parse error", "authentication"
    ]
    FOR pattern IN permanent_patterns:
        IF pattern IN message:
            RETURN ErrorType.PermanentError

    # Default to unknown (not retryable for safety)
    RETURN ErrorType.Unknown


FUNCTION is_retryable(error_type: ErrorType) -> Boolean:
    RETURN error_type IN [ErrorType.Timeout, ErrorType.TransientError]
```

---

## Max Attempts Pattern

### Purpose

Limit the number of retry attempts to prevent infinite loops and resource exhaustion.

### Configuration

```yaml
# From parallel-orchestration.yaml
retry:
  max_attempts: 3  # Total attempts including initial
```

### Algorithm

```
FUNCTION retry_with_backoff(task: Task, config: RetryConfig) -> TaskResult:
    attempt = 0

    WHILE attempt < config.max_attempts:
        TRY:
            result = execute_task(task)
            RETURN result  # Success

        CATCH error:
            error_type = classify_error(error)

            # Check if retryable
            IF NOT is_retryable(error_type):
                log_permanent_failure(task.id, error, attempt)
                RETURN TaskFailure(
                    task_id=task.id,
                    error_type=error_type,
                    error_message=error.message,
                    retry_count=attempt,
                    is_retryable=false
                )

            # Increment attempt counter
            attempt += 1

            # Check if max attempts reached
            IF attempt >= config.max_attempts:
                log_max_attempts_reached(task.id, error, attempt)
                RETURN TaskFailure(
                    task_id=task.id,
                    error_type=error_type,
                    error_message=error.message,
                    retry_count=attempt,
                    is_retryable=true  # Could retry with higher limit
                )

            # Calculate backoff delay
            delay = calculate_backoff(attempt, config)
            log_retry_attempt(task.id, attempt, delay)

            # Wait before retrying
            sleep(delay)

    # Should not reach here, but handle gracefully
    RETURN TaskFailure(task_id=task.id, error_type=ErrorType.Unknown, ...)
```

### Retry State Tracking

```yaml
RetryState:
  task_id: String
  original_error: Error
  attempt_count: Int
  delays: List[Int]      # Actual delays used
  timestamps: List[ISO8601]
  final_status: "success" | "permanent_failure" | "max_attempts_reached"
```

---

## Examples

### Example 1: Successful Retry After Rate Limit

```python
# Initial attempt fails with rate limit
attempt_0 = execute_task(task)
# Error: "Rate limit exceeded (429)"
# Classification: TransientError → Retryable

# Wait 1000ms (base delay)
sleep(1000)

# Retry attempt 1 succeeds
attempt_1 = execute_task(task)
# Success!

# Log:
# [RETRY] task-abc123 attempt=1 delay_ms=1000 status=success
```

### Example 2: Permanent Failure (No Retry)

```python
# Task fails with validation error
result = execute_task(task)
# Error: "Invalid configuration: missing required field 'name'"
# Classification: PermanentError → Not Retryable

# No retry - immediate failure
failure = TaskFailure(
    task_id=task.id,
    error_type=ErrorType.PermanentError,
    error_message="Invalid configuration: missing required field 'name'",
    retry_count=0,
    is_retryable=false
)

# Log:
# [PERMANENT_FAILURE] task-abc123 error="validation error" retry_count=0
```

### Example 3: Max Attempts Exhausted

```python
# Attempt 0: Network timeout
# Attempt 1: Network timeout (after 1000ms wait)
# Attempt 2: Network timeout (after 2000ms wait)
# Attempt 3: max_attempts reached

failure = TaskFailure(
    task_id=task.id,
    error_type=ErrorType.Timeout,
    error_message="Task exceeded timeout_ms after 3 attempts",
    retry_count=3,
    is_retryable=true  # Could work with more attempts
)

# Log:
# [MAX_ATTEMPTS] task-abc123 attempts=3 total_wait_ms=3000
```

### Example 4: Exponential Backoff in Action

```
Task execution timeline:

T+0s:      Attempt 0 - FAIL (rate limit)
T+1s:      Wait 1000ms
T+1s:      Attempt 1 - FAIL (rate limit)
T+3s:      Wait 2000ms
T+3s:      Attempt 2 - FAIL (rate limit)
T+7s:      Wait 4000ms
T+7s:      Attempt 3 - SUCCESS

Total time: 7 seconds
Total wait: 7000ms (1000 + 2000 + 4000)
```

---

## Retry Decision Flow

```
┌─────────────────┐
│   Task Fails    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Classify Error  │
└────────┬────────┘
         │
    ┌────┴────┐
    │         │
    ▼         ▼
Transient   Permanent
    │         │
    ▼         ▼
┌─────────┐  ┌────────────┐
│Retryable│  │Not Retryable│
└────┬────┘  └─────┬──────┘
     │             │
     ▼             ▼
┌──────────┐  ┌─────────────┐
│Check Max │  │Return Failure│
│Attempts  │  └─────────────┘
└────┬─────┘
     │
  ┌──┴──┐
  │     │
  ▼     ▼
Under   At Max
Limit   Limit
  │       │
  ▼       ▼
┌──────┐ ┌─────────────┐
│Retry │ │Return Failure│
└──────┘ │(max attempts)│
         └─────────────┘
```

---

## Configuration Reference

### Full Retry Configuration

```yaml
# parallel-orchestration.yaml
retry:
  enabled: true
  max_attempts: 3
  base_delay_ms: 1000
  max_backoff_ms: 10000
  jitter: true
  jitter_factor: 0.25

  # Per-error-type overrides
  overrides:
    rate_limit:
      max_attempts: 5
      base_delay_ms: 2000
    timeout:
      max_attempts: 2
      base_delay_ms: 500
```

### Disabling Retry

```yaml
retry:
  enabled: false  # All errors fail immediately
```

---

## Troubleshooting

### Issue: Retries Not Happening

**Symptoms:** Transient errors cause immediate failure

**Diagnosis:**
1. Check `retry.enabled` is true
2. Verify error classification is correct
3. Check `max_attempts` is > 1

**Resolution:**
- Enable retry in configuration
- Review error classification logic
- Increase max_attempts if needed

### Issue: Too Many Retries

**Symptoms:** Tasks retry indefinitely, consuming resources

**Diagnosis:**
1. Check `max_attempts` setting
2. Verify errors are correctly classified as permanent

**Resolution:**
- Reduce `max_attempts`
- Fix error classification for permanent errors

### Issue: Backoff Too Aggressive

**Symptoms:** Long delays between retries, slow recovery

**Diagnosis:**
1. Check `base_delay_ms` and `max_backoff_ms`
2. Review if jitter is causing high values

**Resolution:**
- Reduce `base_delay_ms`
- Lower `max_backoff_ms` cap
- Adjust jitter_factor

---

## Related Documentation

- `parallel-config.md` - Configuration reference
- `error-handling-patterns.md` - Partial failure recovery
- `timeout-handling.md` - Timeout monitoring and termination
- `sequential-fallback.md` - Fallback when all retries fail
- `devforgeai/config/parallel-orchestration.yaml` - Configuration file

---

**Last Updated:** 2025-12-18
**Story:** STORY-110
