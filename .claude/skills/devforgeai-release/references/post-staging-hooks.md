# Phase 2.5: Post-Staging Hooks

**Purpose:** Trigger retrospective feedback collection after staging deployment completes (success or failure).

**Part of:** STORY-025 - Wire hooks into /release command

**Execution:** After Phase 2 (Staging Deployment) completes, before proceeding to Phase 3 (Production Deployment)

---

## Overview

This phase integrates the DevForgeAI hook system with the release workflow to automatically prompt for deployment feedback when staging deployments complete. This enables continuous improvement through contextual retrospective conversations without manual intervention.

**Key Principles:**
- **Non-Blocking:** Hook failures never affect deployment status
- **Graceful Degradation:** Missing hook CLI, config errors, or hook crashes are logged and workflow continues
- **Contextual:** Operation context includes environment (staging), deployment status (SUCCESS/FAILURE), rollback status, deployed services
- **Configurable:** Hooks can be enabled/disabled, trigger conditions customizable per environment

---

## Workflow

### Step 1: Determine Deployment Status

**Extract deployment outcome from Phase 2:**
```
IF Phase 2 staging deployment succeeded (smoke tests passed):
    DEPLOYMENT_STATUS = "SUCCESS"
ELSE:
    DEPLOYMENT_STATUS = "FAILURE"
```

**Capture deployment metadata:**
```
ENVIRONMENT = "staging"
STORY_ID = {extracted from conversation context}
ROLLBACK_TRIGGERED = {true if rollback executed, false otherwise}
DEPLOYED_SERVICES = {list of services deployed successfully}
FAILED_SERVICES = {list of services that failed deployment}
DEPLOYMENT_DURATION_SECONDS = {total time from deployment start to completion}
```

---

### Step 2: Check Hook Eligibility

**Invoke check-hooks CLI to determine if feedback should be collected:**

```bash
# Check if staging hooks are enabled and eligible
devforgeai-validate check-hooks --operation=release-staging --status=$DEPLOYMENT_STATUS

# Capture exit code
HOOK_ELIGIBLE=$?
```

**Exit codes:**
- `0` = Eligible (hooks enabled, trigger matches configuration)
- `1` = Not eligible (hooks disabled OR trigger doesn't match)
- `2+` = Error (CLI not found, config missing, etc.)

**Performance requirement:** check-hooks must complete in <100ms

---

### Step 3: Invoke Hooks (Conditional)

**Only execute if check-hooks returned 0 (eligible):**

```bash
IF HOOK_ELIGIBLE == 0:
    # Build operation context JSON
    OPERATION_CONTEXT='{
        "operation": "release-staging",
        "story_id": "'$STORY_ID'",
        "environment": "staging",
        "deployment_status": "'$DEPLOYMENT_STATUS'",
        "rollback_triggered": '$ROLLBACK_TRIGGERED',
        "deployed_services": '$DEPLOYED_SERVICES',
        "failed_services": '$FAILED_SERVICES',
        "deployment_duration_seconds": '$DEPLOYMENT_DURATION_SECONDS',
        "timestamp": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'"
    }'

    # Invoke hooks (timeout: 30 seconds)
    timeout 30 devforgeai-validate invoke-hooks \
        --operation=release-staging \
        --story=$STORY_ID \
        --context="$OPERATION_CONTEXT" \
        2>&1 | tee devforgeai/logs/release-hooks-$STORY_ID.log

    # Capture exit code
    HOOK_INVOCATION_EXIT=$?

    # Graceful degradation
    IF HOOK_INVOCATION_EXIT != 0:
        # Log warning, continue deployment
        Display: "⚠️  Note: Post-deployment feedback unavailable (hook error)"
        Log to devforgeai/logs/release-hooks-$STORY_ID.log:
            "Hook invocation failed with exit code $HOOK_INVOCATION_EXIT"
            "Deployment continues normally (hook failures are non-blocking)"

ELIF HOOK_ELIGIBLE == 1:
    # Not eligible (trigger doesn't match or hooks disabled)
    Display: "ℹ️  Staging feedback skipped (hooks not configured for this scenario)"
    # No logging needed (normal behavior)

ELSE:
    # Error checking eligibility
    Display: "⚠️  Note: Unable to check hook eligibility (check-hooks error)"
    Log to devforgeai/logs/release-hooks-$STORY_ID.log:
        "check-hooks failed with exit code $HOOK_ELIGIBLE"
        "Deployment continues normally"
```

**Performance requirement:** invoke-hooks must complete in <3 seconds (excluding user interaction time)

---

### Step 4: Proceed to Next Phase

**Regardless of hook outcome, always proceed to Phase 3 (Production Deployment):**

```
Display: ""
Display: "✓ Phase 2.5 complete - Proceeding to production deployment..."
Display: ""

# Continue to Phase 3 (Production Deployment)
```

**Critical:** Hook failures, timeouts, or errors NEVER block deployment progression.

---

## Operation Context Schema

**Passed to invoke-hooks CLI via --context parameter:**

```json
{
  "operation": "release-staging",
  "story_id": "STORY-025",
  "environment": "staging",
  "deployment_status": "SUCCESS"|"FAILURE",
  "rollback_triggered": true|false,
  "deployed_services": ["service1", "service2"],
  "failed_services": [],
  "deployment_duration_seconds": 45,
  "timestamp": "2025-11-14T15:30:00Z"
}
```

**This context enables:**
- Deployment-specific questions (staging vs production)
- Failure context awareness (rollback questions, service-specific probing)
- Performance tracking (deployment duration trends)
- Audit trail (timestamp, services deployed)

---

## Hook Configuration Example

**In `devforgeai/config/hooks.yaml`:**

```yaml
operations:
  release-staging:
    enabled: true
    trigger_on: "all"  # Trigger on both success and failure
    questions:
      - "How did the staging deployment go?"
      - "Any issues or surprises during deployment?"
      - "Performance observations? (deployment speed, service startup time)"
      - "Would you deploy this to production right now?"
    metadata:
      environment: "staging"
      severity: "medium"
```

**Default behavior:**
- Staging: Trigger on both success and failure (all deployments)
- Production: Trigger on failures only (see post-production-hooks.md)

---

## Error Scenarios & Graceful Degradation

### Scenario 1: devforgeai CLI Not Installed

**Detection:** `command not found: devforgeai`

**Handling:**
```bash
if ! command -v devforgeai &> /dev/null; then
    echo "⚠️  Note: Post-deployment feedback unavailable (devforgeai CLI not installed)"
    echo "Install: pip install --break-system-packages -e .claude/scripts/"
    # Continue deployment (no hooks, no failure)
fi
```

### Scenario 2: hooks.yaml Missing or Invalid

**Detection:** check-hooks exits with code 2

**Handling:**
```bash
# check-hooks returns 2 on config errors
# Log warning, continue deployment
echo "⚠️  Hook configuration error - feedback skipped"
# Deployment proceeds
```

### Scenario 3: Hook Script Timeout

**Detection:** invoke-hooks exceeds 30 seconds

**Handling:**
```bash
timeout 30 devforgeai-validate invoke-hooks ... || {
    echo "⚠️  Hook invocation timeout - feedback skipped"
    echo "Deployment continues normally"
}
```

### Scenario 4: User Aborts Feedback (Ctrl+C)

**Detection:** invoke-hooks receives SIGINT

**Handling:**
- Hook process terminates gracefully
- Partial feedback saved (questions answered before abort)
- Deployment continues (user returned to deployment workflow)

---

## Logging Requirements

**All hook invocations logged to:**
`devforgeai/logs/release-hooks-{STORY-ID}.log`

**Log format:**
```
[2025-11-14T15:30:00Z] Staging deployment SUCCESS
[2025-11-14T15:30:00Z] check-hooks returned: 0 (eligible)
[2025-11-14T15:30:01Z] invoke-hooks started (PID: 12345)
[2025-11-14T15:30:25Z] invoke-hooks completed (exit code: 0)
[2025-11-14T15:30:25Z] Feedback saved: devforgeai/feedback/releases/STORY-025-staging-2025-11-14T15:30:25Z.json
```

**On errors:**
```
[2025-11-14T15:30:00Z] check-hooks failed (exit code: 2)
[2025-11-14T15:30:00Z] Error: hooks.yaml not found
[2025-11-14T15:30:00Z] Deployment continues without feedback
```

---

## Performance Monitoring

**Track hook integration overhead:**
- check-hooks: Target <100ms (p95)
- invoke-hooks: Target <3s (p95, excluding user interaction)
- Total overhead: Target <3.5s (p95)

**Measure:**
```bash
START=$(date +%s%N)
# ... hook operations ...
END=$(date +%s%N)
DURATION_MS=$(( (END - START) / 1000000 ))

IF DURATION_MS > 3500:
    Log warning: "Hook integration overhead: ${DURATION_MS}ms (target: <3500ms)"
```

---

## Integration Points

**From Phase 2 (Staging Deployment):**
- Deployment status (SUCCESS/FAILURE)
- Rollback flag (if rollback executed)
- Deployed services list
- Deployment duration

**To Phase 3 (Production Deployment):**
- Always proceeds (hooks never block)
- Feedback collection happens asynchronously (non-blocking)

**Feedback Persistence:**
- Saved to `devforgeai/feedback/releases/{STORY-ID}-staging-{timestamp}.json`
- Indexed in `devforgeai/feedback/index.json`
- Searchable via `devforgeai feedback-search --operation=release-staging`

---

## Testing Checklist

**Before deploying hook integration:**
- [ ] check-hooks CLI available and executable
- [ ] invoke-hooks CLI available and executable
- [ ] hooks.yaml exists with release-staging configuration
- [ ] Feedback directory `devforgeai/feedback/releases/` exists
- [ ] Log directory `devforgeai/logs/` exists
- [ ] Hook integration adds <3.5s overhead
- [ ] Hook failures don't break deployment
- [ ] Deployment status accurate regardless of hook status

---

## Rollback Plan

**If Phase 2.5 causes issues:**

1. **Comment out hook invocations:**
   ```bash
   # devforgeai-validate check-hooks --operation=release-staging --status=$DEPLOYMENT_STATUS
   # IF eligible: devforgeai-validate invoke-hooks ...
   ```

2. **Skip Phase 2.5 entirely:**
   - Modify skill to skip from Phase 2 → Phase 3 directly
   - No functional impact (hooks are optional enhancement)

3. **Disable hooks in config:**
   ```yaml
   operations:
     release-staging:
       enabled: false  # Disable without code changes
   ```

**Rollback time:** <5 minutes (comment out code OR config change)

---

## Success Criteria

Phase 2.5 succeeds when:
- [ ] check-hooks invoked with correct operation and status
- [ ] invoke-hooks invoked only when check-hooks returns 0
- [ ] Operation context includes all required metadata
- [ ] Hook failures logged but don't affect deployment
- [ ] Deployment proceeds to Phase 3 regardless of hook outcome
- [ ] Feedback saved (if hooks succeeded and user completed)
- [ ] Performance <3.5s overhead (check + invoke time)

---

**This phase is OPTIONAL and NON-BLOCKING. The deployment workflow is the primary concern; feedback is a value-add enhancement.**
