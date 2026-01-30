# Phase 3.5: Post-Production Hooks

**Purpose:** Trigger retrospective feedback collection after production deployment completes (primarily on failures, optionally on success).

**Part of:** STORY-025 - Wire hooks into /release command

**Execution:** After Phase 3 (Production Deployment) completes, before proceeding to Phase 4 (Post-Deployment Validation)

---

## Overview

This phase integrates the DevForgeAI hook system with production deployments to automatically prompt for critical deployment feedback. Unlike staging (which triggers on all deployments), production hooks default to **failures-only mode** to minimize disruption during successful deployments.

**Key Differences from Staging:**
- **Default Trigger:** Failures only (on_success: false, on_failure: true)
- **Severity:** Higher (production failures are critical incidents)
- **Questions:** Production-specific (impact assessment, rollback observations, customer impact)
- **Urgency:** Higher priority feedback (production issues require immediate attention)

**Shared Principles:**
- **Non-Blocking:** Hook failures never affect deployment status
- **Graceful Degradation:** Missing hook CLI, config errors, or hook crashes are logged and workflow continues
- **Contextual:** Operation context includes environment (production), deployment status, rollback status, deployed services

---

## Workflow

### Step 1: Determine Production Deployment Status

**Extract deployment outcome from Phase 3:**
```
IF Phase 3 production deployment succeeded (all validation passed):
    DEPLOYMENT_STATUS = "SUCCESS"
ELSE:
    DEPLOYMENT_STATUS = "FAILURE"
```

**Capture production deployment metadata:**
```
ENVIRONMENT = "production"
STORY_ID = {extracted from conversation context}
ROLLBACK_TRIGGERED = {true if rollback executed, false otherwise}
DEPLOYED_SERVICES = {list of services deployed successfully}
FAILED_SERVICES = {list of services that failed deployment}
DEPLOYMENT_DURATION_SECONDS = {total time from deployment start to completion}
DEPLOYMENT_STRATEGY = {blue-green | canary | rolling | recreate}
TRAFFIC_PERCENTAGE = {if canary: percentage of traffic routed to new version}
HEALTH_CHECK_STATUS = {PASS | FAIL | DEGRADED}
ERROR_RATE = {errors per minute during/after deployment}
```

---

### Step 2: Check Hook Eligibility

**Invoke check-hooks CLI to determine if feedback should be collected:**

```bash
# Check if production hooks are enabled and eligible
devforgeai-validate check-hooks --operation=release-production --status=$DEPLOYMENT_STATUS

# Capture exit code
HOOK_ELIGIBLE=$?
```

**Exit codes:**
- `0` = Eligible (hooks enabled, trigger matches configuration)
- `1` = Not eligible (hooks disabled OR trigger doesn't match)
- `2+` = Error (CLI not found, config missing, etc.)

**Default behavior:**
- **Production SUCCESS:** check-hooks returns 1 (not eligible) - failures-only mode
- **Production FAILURE:** check-hooks returns 0 (eligible) - always trigger on failures
- **User can override:** Set `on_success: true` in hooks.yaml to trigger on production success

**Performance requirement:** check-hooks must complete in <100ms

---

### Step 3: Invoke Hooks (Conditional)

**Only execute if check-hooks returned 0 (eligible):**

```bash
IF HOOK_ELIGIBLE == 0:
    # Build production-specific operation context JSON
    OPERATION_CONTEXT='{
        "operation": "release-production",
        "story_id": "'$STORY_ID'",
        "environment": "production",
        "deployment_status": "'$DEPLOYMENT_STATUS'",
        "rollback_triggered": '$ROLLBACK_TRIGGERED',
        "deployed_services": '$DEPLOYED_SERVICES',
        "failed_services": '$FAILED_SERVICES',
        "deployment_duration_seconds": '$DEPLOYMENT_DURATION_SECONDS',
        "deployment_strategy": "'$DEPLOYMENT_STRATEGY'",
        "traffic_percentage": '$TRAFFIC_PERCENTAGE',
        "health_check_status": "'$HEALTH_CHECK_STATUS'",
        "error_rate": '$ERROR_RATE',
        "timestamp": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'"
    }'

    # Invoke hooks (timeout: 30 seconds)
    timeout 30 devforgeai-validate invoke-hooks \
        --operation=release-production \
        --story=$STORY_ID \
        --context="$OPERATION_CONTEXT" \
        2>&1 | tee -a devforgeai/logs/release-hooks-$STORY_ID.log

    # Capture exit code
    HOOK_INVOCATION_EXIT=$?

    # Graceful degradation
    IF HOOK_INVOCATION_EXIT != 0:
        # Log warning, continue deployment
        Display: "⚠️  Note: Post-deployment feedback unavailable (hook error)"
        Log to devforgeai/logs/release-hooks-$STORY_ID.log:
            "Production hook invocation failed with exit code $HOOK_INVOCATION_EXIT"
            "Deployment continues normally (hook failures are non-blocking)"

ELIF HOOK_ELIGIBLE == 1:
    # Not eligible (trigger doesn't match - likely production SUCCESS with failures-only mode)
    Display: "ℹ️  Production feedback skipped (failures-only mode, deployment succeeded)"
    # No logging needed (normal behavior for successful production deployments)

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

**Regardless of hook outcome, always proceed to Phase 4 (Post-Deployment Validation):**

```
Display: ""
Display: "✓ Phase 3.5 complete - Proceeding to post-deployment validation..."
Display: ""

# Continue to Phase 4 (Post-Deployment Validation)
```

**Critical:** Hook failures, timeouts, or errors NEVER block deployment progression.

---

## Operation Context Schema

**Passed to invoke-hooks CLI via --context parameter:**

```json
{
  "operation": "release-production",
  "story_id": "STORY-025",
  "environment": "production",
  "deployment_status": "SUCCESS"|"FAILURE",
  "rollback_triggered": true|false,
  "deployed_services": ["service1", "service2"],
  "failed_services": [],
  "deployment_duration_seconds": 120,
  "deployment_strategy": "blue-green",
  "traffic_percentage": 100,
  "health_check_status": "PASS"|"FAIL"|"DEGRADED",
  "error_rate": 0.5,
  "timestamp": "2025-11-14T15:35:00Z"
}
```

**Production-specific fields:**
- `deployment_strategy`: Informs questions about specific strategy used
- `traffic_percentage`: For canary deployments, track traffic routing
- `health_check_status`: Post-deployment health status
- `error_rate`: Errors per minute (detect degradation even on "success")

---

## Hook Configuration Example

**In `devforgeai/config/hooks.yaml`:**

```yaml
operations:
  release-production:
    enabled: true
    trigger_on: "failures-only"  # Default: Skip success, trigger failures
    on_success: false             # Explicit: Don't trigger on production success
    on_failure: true              # Explicit: Always trigger on production failure
    questions:
      - "What caused the production deployment failure?"
      - "Was rollback smooth? Any issues during rollback?"
      - "Customer impact? (user reports, error logs, metrics)"
      - "What would prevent this failure in the future?"
      - "Incident severity? (SEV1: critical, SEV2: high, SEV3: medium)"
    metadata:
      environment: "production"
      severity: "critical"
```

**Alternative: Trigger on Success** (opt-in)

```yaml
operations:
  release-production:
    enabled: true
    trigger_on: "all"            # Override: Trigger on both success and failure
    on_success: true             # Explicit: Trigger on production success too
    on_failure: true
    questions:
      - "Production deployment complete. Any observations?"
      - "Performance after deployment? (latency, error rate)"
      - "Monitoring alerts? (any unexpected warnings/errors)"
```

---

## Production-Specific Questions

**Failure scenarios:**
- "What triggered the rollback?" (if rollback_triggered: true)
- "Which service failed deployment?" (if failed_services non-empty)
- "Customer-facing impact?" (user-visible errors, downtime duration)
- "Root cause observations?" (what went wrong, why)
- "Prevention measures?" (what changes would prevent recurrence)

**Partial success scenarios:**
- "Which services deployed successfully?" (deployed_services)
- "Which services failed?" (failed_services)
- "Can the system operate with partial deployment?" (degraded mode viability)

**Rollback scenarios:**
- "Was rollback smooth?" (rollback process observations)
- "Any data loss during rollback?" (data consistency check)
- "Current production state?" (fully rolled back, partially rolled back, degraded)

---

## Error Scenarios & Graceful Degradation

### Scenario 1: Production Failure + Hook CLI Error

**Critical:** Production failure is primary concern, not hook failure.

**Handling:**
```bash
# Production deployment FAILED
DEPLOYMENT_STATUS="FAILURE"

# Try to invoke hooks for failure feedback
devforgeai-validate check-hooks --operation=release-production --status=FAILURE || {
    echo "⚠️  Production deployment FAILED"
    echo "⚠️  Unable to collect failure feedback (hook CLI error)"
    echo "⚠️  Manual incident response required"
    # Deployment marked as FAILED (accurate)
    # Hook error is secondary issue
}
```

**Result:** Deployment status = FAILURE (accurate), hook error logged but doesn't hide production failure

### Scenario 2: Hook Timeout During Production Failure

**Handling:**
```bash
timeout 30 devforgeai-validate invoke-hooks ... || {
    echo "⚠️  Production deployment FAILED"
    echo "⚠️  Feedback collection timeout"
    echo "⚠️  Deployment status: FAILED (accurate)"
    # Mark deployment FAILED, skip feedback
}
```

**Result:** Production failure is the critical issue, hook timeout is logged but not blocking

### Scenario 3: User Aborts Feedback During Critical Failure

**Scenario:** Production deployment failed, user starts feedback, then aborts (Ctrl+C)

**Handling:**
- Partial feedback saved (questions answered before abort)
- Deployment status remains FAILURE (accurate)
- User returned to deployment workflow
- Incident response can proceed immediately

---

## Logging Requirements

**All production hook invocations logged to:**
`devforgeai/logs/release-hooks-{STORY-ID}.log`

**Log format (success):**
```
[2025-11-14T15:35:00Z] Production deployment SUCCESS
[2025-11-14T15:35:00Z] check-hooks returned: 1 (not eligible - failures-only mode)
[2025-11-14T15:35:00Z] Feedback skipped (normal for production success)
```

**Log format (failure):**
```
[2025-11-14T15:35:00Z] Production deployment FAILURE
[2025-11-14T15:35:00Z] Rollback triggered: true
[2025-11-14T15:35:00Z] check-hooks returned: 0 (eligible)
[2025-11-14T15:35:01Z] invoke-hooks started (PID: 12346)
[2025-11-14T15:35:45Z] invoke-hooks completed (exit code: 0)
[2025-11-14T15:35:45Z] Feedback saved: devforgeai/feedback/releases/STORY-025-production-2025-11-14T15:35:45Z.json
[2025-11-14T15:35:45Z] Incident feedback captured for post-mortem
```

---

## Performance Monitoring

**Track production hook integration overhead:**
- check-hooks: Target <100ms (p95)
- invoke-hooks: Target <3s (p95, excluding user interaction)
- Total overhead: Target <3.5s (p95)

**Production failures:**
- User interaction time NOT counted in overhead (incident response priority)
- Feedback collection time NOT counted against deployment time
- Production failure is the critical metric, not hook performance

---

## Integration Points

**From Phase 3 (Production Deployment):**
- Deployment status (SUCCESS/FAILURE)
- Rollback flag (if rollback executed)
- Deployed services list
- Deployment strategy used
- Health check results
- Error rate metrics

**To Phase 4 (Post-Deployment Validation):**
- Always proceeds (hooks never block)
- Feedback collection happens asynchronously (non-blocking)
- Production status accurate regardless of hook outcome

**Feedback Persistence:**
- Saved to `devforgeai/feedback/releases/{STORY-ID}-production-{timestamp}.json`
- Indexed in `devforgeai/feedback/index.json`
- Searchable via `devforgeai feedback-search --operation=release-production --severity=critical`
- Used for post-mortem analysis (production failures)

---

## Testing Checklist

**Before deploying production hook integration:**
- [ ] check-hooks CLI available and executable
- [ ] invoke-hooks CLI available and executable
- [ ] hooks.yaml exists with release-production configuration
- [ ] Failures-only mode configured (`on_success: false`)
- [ ] Production SUCCESS skips feedback (expected behavior)
- [ ] Production FAILURE triggers feedback (critical path)
- [ ] Feedback directory `devforgeai/feedback/releases/` exists
- [ ] Log directory `devforgeai/logs/` exists
- [ ] Hook integration adds <3.5s overhead
- [ ] Hook failures don't break deployment
- [ ] Deployment status accurate regardless of hook status
- [ ] Production failure status never masked by hook errors

---

## Rollback Plan

**If Phase 3.5 causes issues in production:**

1. **Immediate: Comment out hook invocations in skill:**
   ```bash
   # devforgeai-validate check-hooks --operation=release-production --status=$DEPLOYMENT_STATUS
   # IF eligible: devforgeai-validate invoke-hooks ...
   ```

2. **Fast: Disable hooks in config (no code deployment):**
   ```yaml
   operations:
     release-production:
       enabled: false  # Disable without code changes
   ```

3. **Safest: Skip Phase 3.5 entirely:**
   - Modify skill to skip from Phase 3 → Phase 4 directly
   - No functional impact (hooks are optional enhancement)

**Rollback time:** <2 minutes (config change) OR <5 minutes (code change)

**Critical:** Production deployment functionality is NEVER affected by hook integration. Hooks are value-add, not critical path.

---

## Success Criteria

Phase 3.5 succeeds when:
- [ ] check-hooks invoked with correct operation and status
- [ ] Production SUCCESS: check-hooks returns 1 (not eligible) - failures-only mode working
- [ ] Production FAILURE: check-hooks returns 0 (eligible) - failures always trigger
- [ ] invoke-hooks invoked only when check-hooks returns 0
- [ ] Operation context includes production-specific metadata (strategy, traffic%, health, error rate)
- [ ] Hook failures logged but don't affect deployment
- [ ] Deployment proceeds to Phase 4 regardless of hook outcome
- [ ] Feedback saved (if hooks succeeded and user completed)
- [ ] Performance <3.5s overhead (check + invoke time, excluding user interaction)
- [ ] Production deployment status ALWAYS accurate (hooks never mask failures)

---

**Production deployments are critical operations. This phase is OPTIONAL and NON-BLOCKING. The deployment workflow and accurate status reporting are the primary concerns; feedback is a value-add enhancement for continuous improvement.**
