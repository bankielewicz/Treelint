# Parallel Smoke Tests for Release Skill

**Story:** STORY-113
**Version:** 1.0
**Epic:** EPIC-017 (Parallel Task Orchestration)

---

## Overview

This reference defines the parallel smoke test pattern for the Release skill, enabling concurrent execution of health checks and smoke tests during post-deployment validation.

**Performance Impact:** 3-5x faster deployment validation (50s sequential → 10-17s parallel)

**Key Principle:** Health checks and smoke tests run concurrently in the same batch.

---

## Parallel Configuration Loading

### Load Parallel Orchestration Config

Before executing parallel tests, load the configuration:

```pseudocode
Read(file_path="devforgeai/config/parallel-orchestration.yaml")

# Extract settings from active profile
max_concurrent_tasks = config.profiles[active_profile].max_concurrent_tasks  # Default: 4
timeout_ms = config.profiles[active_profile].timeout_ms  # Default: 120000
min_success_rate = config.error_handling.min_success_rate  # Default: 0.5
```

### Profile Selection

| Profile | max_concurrent_tasks | timeout_ms | Use Case |
|---------|---------------------|-----------|----------|
| Pro | 4 | 120000 | Standard release |
| Max | 6 | 180000 | Large deployments |
| API | 8 | 300000 | Enterprise/CI |

---

## Parallel Invocation Pattern

### Single Message with Multiple Bash Calls

Execute health checks AND smoke tests in ONE message (parallel execution):

```pseudocode
# All Bash calls in a SINGLE message - they execute in parallel

# Health check endpoints (2-3 concurrent)
Bash(command="curl -s -o /dev/null -w '%{http_code}' https://api.example.com/health")
Bash(command="curl -s -o /dev/null -w '%{http_code}' https://api.example.com/ready")

# Smoke tests (3-5 concurrent)
Bash(command="npm test -- --testNamePattern='smoke' --json --outputFile=smoke-results.json")
Bash(command="pytest tests/smoke/ -v --tb=short -q")
Bash(command="curl -s -X POST https://api.example.com/test-endpoint -d '{}'")
```

**CRITICAL:** All Bash calls MUST be in the same message for parallel execution.

### Batching Strategy

If smoke tests exceed `max_concurrent_tasks`, batch them:

```pseudocode
total_tests = len(health_checks) + len(smoke_tests)

IF total_tests > max_concurrent_tasks:
    # Batch 1: Health checks + first N smoke tests
    batch_1 = health_checks + smoke_tests[:max_concurrent_tasks - len(health_checks)]

    # Batch 2: Remaining smoke tests
    batch_2 = smoke_tests[max_concurrent_tasks - len(health_checks):]

    Execute batch_1 (parallel)
    Wait for results
    Execute batch_2 (parallel)
ELSE:
    Execute all in single batch (parallel)
```

---

## Result Aggregation

### Using PartialResult Model

Uses the `PartialResult` data model from error-handling-patterns.md (STORY-110):

```pseudocode
# Collect results from all parallel Bash commands
health_results = [result for result in bash_outputs if "health" in result.command]
smoke_results = [result for result in bash_outputs if "smoke" in result.command or "test" in result.command]

# Aggregate into PartialResult
partial_result = aggregate_parallel_results(bash_outputs)

# Separate analysis
health_success_rate = count_successes(health_results) / len(health_results)
smoke_success_rate = count_successes(smoke_results) / len(smoke_results)
```

### Success Determination

```pseudocode
# Health check success: HTTP 200 response
health_passed = result.output == "200"

# Smoke test success: Exit code 0
smoke_passed = result.exit_code == 0
```

---

## Success Threshold

### Release Requires 50% Success Rate

```pseudocode
min_success_rate = 0.5  # 50% threshold (more lenient than QA)

IF partial_result.success_rate < min_success_rate:
    Display: "❌ Post-deployment validation failed"
    Display: f"  Success rate: {partial_result.success_rate * 100}%"
    Display: f"  Required: 50%"
    Trigger rollback procedure
    HALT workflow

ELSE:
    Display: "✓ Post-deployment validation passed"
    Continue to release documentation
```

### Threshold Rationale

| Scenario | Tests | Success Rate | Decision |
|----------|-------|--------------|----------|
| All pass | 5/5 | 100% | Continue (ideal) |
| 4 pass, 1 fail | 4/5 | 80% | Continue |
| 3 pass, 2 fail | 3/5 | 60% | Continue |
| 2 pass, 3 fail | 2/5 | 40% | **HALT + Rollback** |
| All fail | 0/5 | 0% | **HALT + Rollback** |

---

## Display Format

### Parallel Post-Deployment Display

```
✓ Phase 4 Complete: Parallel Post-Deployment Validation
  Health checks: [X] of [Y] passed
    - /health: [200 ✓ / 5XX ✗]
    - /ready: [200 ✓ / 5XX ✗]
  Smoke tests: [X] of [Y] passed
    - npm smoke: [PASS ✓ / FAIL ✗]
    - pytest smoke: [PASS ✓ / FAIL ✗]
    - API test: [PASS ✓ / FAIL ✗]
  Overall success rate: [X]% (threshold: 50%)
  Duration: [X]s (vs ~[5X]s sequential)
```

### Example Output (4 of 5 Pass)

```
✓ Phase 4 Complete: Parallel Post-Deployment Validation
  Health checks: 2 of 2 passed
    - /health: 200 ✓
    - /ready: 200 ✓
  Smoke tests: 2 of 3 passed
    - npm smoke: PASS ✓
    - pytest smoke: FAIL ✗ (timeout after 30s)
    - API test: PASS ✓
  Overall success rate: 80% (threshold: 50%)
  Duration: 12s (vs ~55s sequential)
```

---

## Error Handling

### Integration with error-handling-patterns.md (STORY-110)

This reference uses error handling patterns defined in:
`.claude/skills/devforgeai-orchestration/references/error-handling-patterns.md`

**Key concepts applied:**
1. **Partial Failure Recovery** - Continue if success_rate >= 0.5
2. **Result Aggregation** - Use PartialResult model for mixed results
3. **Rollback Trigger** - Automatic rollback if threshold not met

### Failure Classification

| Error Type | Retryable | Action |
|------------|-----------|--------|
| Timeout | Yes | Log, include in failure count |
| HTTP 5xx | Maybe | Log, retry once, then count as failure |
| HTTP 4xx | No | Log, count as failure |
| Exit code != 0 | Depends | Log, count as failure |

### Rollback Integration

When validation fails below threshold:

```pseudocode
IF partial_result.success_rate < min_success_rate:
    # Trigger rollback
    Read(file_path=".claude/skills/devforgeai-release/references/rollback-procedures.md")
    Execute rollback workflow
    Update story status to "Rollback"
    HALT
```

---

## Concurrent Health + Smoke Pattern

### Why Concurrent?

Health checks and smoke tests are **independent**:
- Health checks verify basic service availability
- Smoke tests verify critical paths work

Running them together maximizes parallelism without risk.

### Same Batch Execution

```pseudocode
# All in ONE message for maximum parallelism
parallel_batch = [
    # Health checks
    Bash(command="curl -s $HEALTH_ENDPOINT_1 -o /dev/null -w '%{http_code}'"),
    Bash(command="curl -s $HEALTH_ENDPOINT_2 -o /dev/null -w '%{http_code}'"),

    # Smoke tests
    Bash(command="npm test -- --testNamePattern='smoke'"),
    Bash(command="pytest tests/smoke/ -v"),
    Bash(command="curl -s -X GET $API_TEST_ENDPOINT")
]

# Execute all in parallel (single message)
results = execute_parallel(parallel_batch)

# Aggregate both types together
combined_result = aggregate_parallel_results(results)
```

---

## Phase Integration

### Where This Runs in Release Workflow

This parallel validation updates Phase 4 (Post-Deployment Validation):

```
Phase 1-3: Pre-release and deployment (unchanged)
    ↓
Phase 4: Parallel Post-Deployment Validation ← THIS REFERENCE
    - Health checks (parallel)
    - Smoke tests (parallel)
    - Concurrent execution in same batch
    ↓
Phase 5-6: Documentation and closure (unchanged)
```

---

## Related Documentation

- `error-handling-patterns.md` (STORY-110) - PartialResult model, failure handling
- `parallel-config.md` (STORY-108) - Configuration schema
- `rollback-procedures.md` - Rollback workflow when validation fails
- `smoke-testing-guide.md` - Detailed smoke test definitions

---

**Last Updated:** 2025-12-19
**Story:** STORY-113
