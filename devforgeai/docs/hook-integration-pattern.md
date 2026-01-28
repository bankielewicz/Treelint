# Hook Integration Pattern

**Purpose:** Standardized pattern for integrating DevForgeAI hook system into slash commands and skills

**Created:** 2025-11-14 (STORY-025: Wire hooks into /release command)

**Version:** 1.0

---

## Overview

The hook integration pattern provides a consistent, non-blocking way to collect retrospective feedback after operations complete. This document defines the standard pattern used across all DevForgeAI commands (/dev, /qa, /release, etc.).

**Core Principles:**
1. **Non-Blocking:** Hook failures NEVER affect operation outcome
2. **Graceful Degradation:** Missing CLI, config errors, crashes are logged and skipped
3. **Contextual:** Operation-specific metadata enables targeted questions
4. **Configurable:** Per-operation enable/disable, trigger modes (all, failures-only)
5. **Consistent UX:** Same interaction patterns across all commands

---

## Architecture

```
Operation completes
  ↓
Determine STATUS (SUCCESS | FAILURE)
  ↓
Build operation CONTEXT (metadata)
  ↓
devforgeai check-hooks --operation={op} --status={STATUS}
  ↓
Exit code 0? (eligible) ────→ NO (exit 1) ──→ Skip, continue
  ↓ YES                                           ↓
devforgeai invoke-hooks --story={ID}              ↓
  --operation={op} --context={JSON}              ↓
  ↓                                               ↓
Feedback collected ────────────────────────────→ ↓
  ↓                                               ↓
Operation proceeds (ALWAYS)  ←──────────────────┘
```

**Key:** Operation outcome (success/failure) is determined BEFORE hooks. Hooks collect feedback AFTER the fact. Hook status never affects operation status.

---

## Integration Steps

### Step 1: Determine Operation Status

**After operation completes, capture outcome:**

```bash
# Example: After /dev TDD cycle
IF all_tests_passed:
    STATUS="SUCCESS"
ELSE:
    STATUS="FAILURE"

# Example: After /qa validation
IF qa_validation_passed:
    STATUS="SUCCESS"
ELSE:
    STATUS="FAILURE"

# Example: After /release staging deployment
IF staging_deployment_succeeded AND smoke_tests_passed:
    STATUS="SUCCESS"
ELSE:
    STATUS="FAILURE"
```

**Critical:** Operation status determined by operation logic, NOT hooks.

---

### Step 2: Build Operation Context

**Create JSON context with operation-specific metadata:**

```bash
# Minimal context (all operations)
OPERATION_CONTEXT='{
    "operation": "'$OPERATION_NAME'",
    "story_id": "'$STORY_ID'",
    "status": "'$STATUS'",
    "timestamp": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'"
}'

# Extended context (operation-specific)
# For /dev:
OPERATION_CONTEXT='{
    "operation": "dev",
    "story_id": "'$STORY_ID'",
    "status": "'$STATUS'",
    "tests_passed": '$TESTS_PASSED',
    "coverage_percentage": '$COVERAGE',
    "timestamp": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'"
}'

# For /qa:
OPERATION_CONTEXT='{
    "operation": "qa",
    "story_id": "'$STORY_ID'",
    "status": "'$STATUS'",
    "mode": "'$VALIDATION_MODE'",
    "violations_critical": '$CRITICAL_COUNT',
    "violations_high": '$HIGH_COUNT',
    "coverage_percentage": '$COVERAGE',
    "timestamp": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'"
}'

# For /release staging:
OPERATION_CONTEXT='{
    "operation": "release-staging",
    "story_id": "'$STORY_ID'",
    "status": "'$STATUS'",
    "environment": "staging",
    "rollback_triggered": '$ROLLBACK',
    "deployed_services": '$DEPLOYED',
    "failed_services": '$FAILED',
    "deployment_duration_seconds": '$DURATION',
    "timestamp": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'"
}'
```

**Principle:** Include metadata that enables context-aware questions.

---

### Step 3: Check Hook Eligibility

**Invoke check-hooks CLI to determine if hooks should run:**

```bash
devforgeai check-hooks --operation=$OPERATION_NAME --status=$STATUS
HOOK_ELIGIBLE=$?
```

**Exit codes:**
- `0` = Eligible (hooks enabled AND trigger condition matches)
- `1` = Not eligible (hooks disabled OR trigger doesn't match config)
- `2+` = Error (CLI not found, config missing, CLI crash)

**Example eligibility logic:**

| Operation | Status | Config Trigger | check-hooks Exit Code |
|-----------|--------|----------------|----------------------|
| dev | SUCCESS | all | 0 (eligible) |
| dev | FAILURE | all | 0 (eligible) |
| qa | SUCCESS | failures-only | 1 (not eligible) |
| qa | FAILURE | failures-only | 0 (eligible) |
| release-staging | SUCCESS | all | 0 (eligible) |
| release-staging | FAILURE | all | 0 (eligible) |
| release-production | SUCCESS | failures-only | 1 (not eligible) |
| release-production | FAILURE | failures-only | 0 (eligible) |

**Performance:** check-hooks must complete in <100ms (p95)

---

### Step 4: Invoke Hooks (Conditional)

**Only if check-hooks returned 0 (eligible):**

```bash
IF HOOK_ELIGIBLE == 0:
    # Invoke hooks with timeout
    timeout 30 devforgeai invoke-hooks \
        --operation=$OPERATION_NAME \
        --story=$STORY_ID \
        --context="$OPERATION_CONTEXT" \
        2>&1 | tee -a devforgeai/logs/${OPERATION_NAME}-hooks-$STORY_ID.log

    # Capture exit code
    HOOK_EXIT=$?

    # Graceful degradation
    IF HOOK_EXIT != 0:
        echo "⚠️  Note: Feedback collection failed (hook error)"
        # Operation continues normally
    ELSE:
        echo "✓ Feedback collected"

ELIF HOOK_ELIGIBLE == 1:
    # Not eligible (normal - trigger doesn't match or hooks disabled)
    # No logging needed, continue operation

ELSE:
    # Error (check-hooks failed)
    echo "⚠️  Note: Unable to check hook eligibility (check-hooks error)"
    # Operation continues normally
```

**Performance:** invoke-hooks must complete in <3s (p95), excluding user interaction time

**Timeout:** 30 seconds default (configurable in hooks.yaml)

---

### Step 5: Continue Operation

**ALWAYS proceed, regardless of hook outcome:**

```bash
# Hook integration complete (success, failure, or skipped)
# Operation continues to next phase

echo ""
echo "✓ Operation complete"
echo ""

# Proceed to next command/phase
```

**Critical:** Hooks are OPTIONAL. Operation flow never depends on hook success.

---

## Error Handling

### Scenario 1: devforgeai CLI Not Installed

**Detection:** `command not found: devforgeai`

**Response:**
```bash
if ! command -v devforgeai &> /dev/null; then
    # Hook CLI not available - skip hooks
    # No error, no logging, no user notification needed
    # Operation proceeds normally
    :  # No-op in bash
fi
```

**User Impact:** None (hooks are optional enhancement)

### Scenario 2: hooks.yaml Missing or Invalid

**Detection:** check-hooks exits with code 2

**Response:**
```bash
devforgeai check-hooks --operation=$OP --status=$STATUS
if [ $? -ge 2 ]; then
    # Config error - skip hooks
    echo "⚠️  Hook configuration error - feedback skipped"
    # Operation continues
fi
```

**User Impact:** Warning displayed, operation continues

### Scenario 3: Hook Invocation Timeout

**Detection:** invoke-hooks exceeds timeout (30s default)

**Response:**
```bash
timeout 30 devforgeai invoke-hooks ... || {
    echo "⚠️  Hook invocation timeout - feedback skipped"
    # Operation continues
}
```

**User Impact:** Warning displayed, operation continues

### Scenario 4: User Aborts Feedback (Ctrl+C)

**Detection:** invoke-hooks receives SIGINT

**Response:**
- Hook process terminates
- Partial feedback saved (questions answered before abort)
- Operation continues (user returned to workflow)

**User Impact:** Partial feedback saved, operation continues immediately

### Scenario 5: Hook CLI Crashes

**Detection:** invoke-hooks exits with non-zero code (not timeout)

**Response:**
```bash
devforgeai invoke-hooks ... || {
    echo "⚠️  Hook invocation failed (exit code $?) - feedback skipped"
    # Log error details to devforgeai/logs/
    # Operation continues
}
```

**User Impact:** Error logged, operation continues

---

## Logging Standards

### Log Location

**Per-operation log files:**
- `/dev`: `devforgeai/logs/dev-hooks-{STORY-ID}.log`
- `/qa`: `devforgeai/logs/qa-hooks-{STORY-ID}.log`
- `/release`: `devforgeai/logs/release-hooks-{STORY-ID}.log`

### Log Format

**Successful hook execution:**
```
[2025-11-14T15:30:00Z] Operation: dev, Status: SUCCESS
[2025-11-14T15:30:00Z] check-hooks returned: 0 (eligible)
[2025-11-14T15:30:01Z] invoke-hooks started (PID: 12345)
[2025-11-14T15:30:15Z] invoke-hooks completed (exit code: 0)
[2025-11-14T15:30:15Z] Feedback saved: devforgeai/feedback/dev/STORY-025-2025-11-14T15:30:15Z.json
```

**Hook skipped (not eligible):**
```
[2025-11-14T15:30:00Z] Operation: release-production, Status: SUCCESS
[2025-11-14T15:30:00Z] check-hooks returned: 1 (not eligible - failures-only mode)
[2025-11-14T15:30:00Z] Feedback skipped (normal for production success)
```

**Hook error:**
```
[2025-11-14T15:30:00Z] Operation: qa, Status: FAILURE
[2025-11-14T15:30:00Z] check-hooks failed (exit code: 2)
[2025-11-14T15:30:00Z] Error: hooks.yaml not found
[2025-11-14T15:30:00Z] Operation continues without feedback
```

### Log Rotation

**Automatic rotation at 10MB:**
```
release-hooks-STORY-025.log       (current)
release-hooks-STORY-025.log.1     (previous)
release-hooks-STORY-025.log.2     (older)
```

---

## Feedback Persistence

### File Structure

**Feedback saved to:**
`devforgeai/feedback/{operation}/{STORY-ID}-{timestamp}.json`

**Examples:**
- `devforgeai/feedback/dev/STORY-025-2025-11-14T15:30:15Z.json`
- `devforgeai/feedback/qa/STORY-025-2025-11-14T15:35:20Z.json`
- `devforgeai/feedback/releases/STORY-025-staging-2025-11-14T15:40:30Z.json`
- `devforgeai/feedback/releases/STORY-025-production-2025-11-14T15:45:40Z.json`

### JSON Schema

```json
{
  "version": "2.0",
  "story_id": "STORY-025",
  "operation": "release-staging",
  "timestamp": "2025-11-14T15:40:30Z",
  "user": "developer@example.com",
  "operation_context": {
    "operation": "release-staging",
    "story_id": "STORY-025",
    "status": "SUCCESS",
    "environment": "staging",
    "rollback_triggered": false,
    "deployed_services": ["api", "web"],
    "failed_services": [],
    "deployment_duration_seconds": 45
  },
  "questions": [
    {
      "question": "How did the staging deployment go?",
      "answer": "Smooth, no issues",
      "skipped": false
    },
    {
      "question": "Any issues or surprises during deployment?",
      "answer": "",
      "skipped": true
    }
  ],
  "metadata": {
    "environment": "staging",
    "severity": "medium",
    "category": "deployment"
  },
  "feedback_duration_seconds": 45
}
```

---

## Configuration Examples

### Example 1: /dev Command Hooks

```yaml
operations:
  dev:
    enabled: true
    trigger_on: "all"
    questions:
      - "How did the TDD cycle go?"
      - "Any test failures or coverage gaps?"
      - "Code quality observations?"
    metadata:
      severity: "medium"
```

### Example 2: /qa Command Hooks (Failures-Only)

```yaml
operations:
  qa:
    enabled: true
    trigger_on: "failures-only"
    on_success: false
    on_failure: true
    questions:
      - "What caused the QA failure?"
      - "Which acceptance criteria failed?"
      - "Test coverage gaps identified?"
    metadata:
      severity: "high"
```

### Example 3: /release Staging Hooks (All Deployments)

```yaml
operations:
  release-staging:
    enabled: true
    trigger_on: "all"
    questions:
      - "How did the staging deployment go?"
      - "Any deployment issues?"
      - "Ready for production?"
    metadata:
      environment: "staging"
      severity: "medium"
```

### Example 4: /release Production Hooks (Failures-Only)

```yaml
operations:
  release-production:
    enabled: true
    trigger_on: "failures-only"
    questions:
      - "What caused the production deployment failure?"
      - "Customer impact?"
      - "Rollback observations?"
      - "Prevention measures?"
    metadata:
      environment: "production"
      severity: "critical"
```

---

## Testing Checklist

**Before integrating hooks into a new command:**

- [ ] devforgeai CLI installed and accessible
- [ ] check-hooks command available
- [ ] invoke-hooks command available
- [ ] hooks.yaml exists with operation configuration
- [ ] Feedback directory exists (`devforgeai/feedback/{operation}/`)
- [ ] Log directory exists (`devforgeai/logs/`)
- [ ] Hook integration adds <3.5s overhead (check + invoke time)
- [ ] Hook failures don't affect operation status
- [ ] Operation status accurate regardless of hook outcome
- [ ] Timeout behavior tested (30s limit)
- [ ] Ctrl+C abort tested (partial feedback saved)
- [ ] CLI not installed scenario tested (graceful skip)
- [ ] Config missing scenario tested (graceful skip)
- [ ] Logging verified (success, skip, error scenarios)

---

## Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| check-hooks latency | <100ms | p95 |
| invoke-hooks latency | <3s | p95 (excluding user interaction) |
| Total overhead | <3.5s | p95 |
| Timeout limit | 30s | Configurable in hooks.yaml |
| Log file size | <10MB | Auto-rotation |

---

## Commands Using This Pattern

| Command | Status | Phases with Hooks | Notes |
|---------|--------|-------------------|-------|
| `/dev` | ✅ Implemented | Phase 6 | After TDD cycle completes |
| `/qa` | ✅ Implemented | Phase 5 | After validation completes |
| `/release` | ✅ Implemented | Phase 2.5, 3.5 | After staging and production deployments |
| `/orchestrate` | ⏳ Planned | End of workflow | After full lifecycle completes |

---

## Best Practices

1. **Placement:** Add hook integration AFTER operation completes, BEFORE next phase
2. **Status Determination:** Always determine operation status BEFORE hook check
3. **Graceful Degradation:** NEVER throw errors on hook failures
4. **Logging:** Log all hook attempts (success, skip, error) for debugging
5. **Timeouts:** Use reasonable timeouts (30s default, 10min for production incidents)
6. **Context Richness:** Include operation-specific metadata in context JSON
7. **Consistent UX:** Same question style, skip behavior, feedback flow across all commands
8. **Performance:** Monitor hook overhead, ensure <3.5s target met
9. **Testing:** Comprehensive testing of error scenarios (CLI missing, config invalid, timeout)
10. **Documentation:** Document hook integration in command/skill reference files

---

## Troubleshooting Guide

### Issue: Hooks Not Triggering

**Checklist:**
1. Is `hooks.yaml` in `devforgeai/config/`?
2. Is `enabled: true` (global and operation-specific)?
3. Does `trigger_on` match operation status?
4. Is `devforgeai` CLI installed?
5. Check logs: `devforgeai/logs/{operation}-hooks-{STORY-ID}.log`

### Issue: Hook Errors Breaking Operation

**This should NEVER happen.**

**If operation fails:**
1. Check operation logs (NOT hook logs)
2. Verify operation status determination logic
3. Ensure hook integration is in `|| true` block (non-blocking)
4. Review hook integration code for missing error handling

**Operation status must be accurate regardless of hook status.**

### Issue: Slow Hook Performance

**Measurements:**
```bash
time devforgeai check-hooks --operation=dev --status=SUCCESS
# Target: <100ms

time devforgeai invoke-hooks --operation=dev --story=STORY-025
# Target: <3s (excluding user interaction)
```

**Optimization:**
1. Check hook system performance (CLI overhead)
2. Review hooks.yaml question count (keep <10)
3. Verify timeout settings (30s default, adjust if needed)
4. Profile hook CLI (identify bottlenecks)

---

## Version History

**v1.0 (2025-11-14):**
- Initial hook integration pattern documented
- Based on /dev (STORY-023), /qa (STORY-024), /release (STORY-025) implementations
- Standardizes check-hooks → invoke-hooks workflow
- Defines graceful degradation, logging, performance standards

---

**This pattern ensures consistent, reliable, non-blocking feedback collection across all DevForgeAI commands. Follow this pattern when adding hooks to new commands.**
