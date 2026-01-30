# Feedback Hooks Workflow - Phase 6

**Purpose:** Invoke retrospective feedback hooks after QA validation completes, enabling context-aware reflection on QA results.

**From:** Phase 5 (QA Report Generation) → Phase 6 (Feedback Hooks) → Phase 7 (Story Updates)

**Non-Blocking:** Hook failures do NOT affect QA result (result determined in Phase 5).

---

## When Phase 6 Executes

**Always executes** after Phase 5 completes (both light and deep modes).

**Inputs:**
- QA result from Phase 5 (PASSED, FAILED, PARTIAL)
- Story ID from conversation context
- Mode from conversation context (light, deep)

**Outputs:**
- Hook invocation status (triggered, skipped, failed)
- Status included in final QA result returned to command

---

## Phase 6 Workflow (5 Steps)

### Step 6.1: Determine QA Status for Hooks

Map QA result to hook status parameter:

```bash
# Extract QA result from Phase 5
QA_RESULT="[PASSED|FAILED|PARTIAL]"

# Map to hook status
if [ "$QA_RESULT" = "PASSED" ]; then
  STATUS="completed"
elif [ "$QA_RESULT" = "FAILED" ]; then
  STATUS="failed"
elif [ "$QA_RESULT" = "PARTIAL" ]; then
  STATUS="partial"
else
  STATUS="unknown"
fi
```

**Status Mappings:**
- PASSED → `completed`
- FAILED → `failed`
- PARTIAL → `partial`

**Rationale:** Hook configuration uses "completed/failed/partial" terminology (not PASSED/FAILED).

---

### Step 6.2: Check if Hooks Should Trigger

Call DevForgeAI CLI to check hook configuration:

```bash
devforgeai-validate check-hooks --operation=qa --status=$STATUS
EXIT_CODE=$?
```

**Exit codes:**
- `0`: Hooks should trigger (proceed to Step 6.3)
- `1`: Hooks should skip (proceed to Step 6.4 - skip path)
- Other: Error (treat as skip)

**What check-hooks does:**
1. Reads `devforgeai/feedback/hooks.yaml` configuration
2. Checks `enabled` flag (true/false)
3. Checks `trigger_on` mode (all, failures-only, successes-only)
4. Returns 0 if hooks should run, 1 if should skip

**Examples:**
```yaml
# failures-only config
enabled: true
trigger_on: failures-only

# Result:
# check-hooks --status=failed → exit 0 (trigger)
# check-hooks --status=completed → exit 1 (skip)
```

---

### Step 6.3: Invoke Feedback Hooks (If Exit Code 0)

Conditionally invoke hooks based on check-hooks result:

```bash
if [ $EXIT_CODE -eq 0 ]; then
  # Hooks should trigger
  devforgeai-validate invoke-hooks --operation=qa --story=$STORY_ID || {
    echo "⚠️ Feedback hook failed, QA result unchanged"
  }
fi
```

**Non-blocking error handling:**
- `||` pattern ensures hook failure doesn't break QA
- QA result (from Phase 5) is immutable by hook outcome
- Warning message displayed if hook fails
- Skill continues to Phase 7 regardless

**What invoke-hooks does:**
1. Reads story file for context (coverage, violations, etc.)
2. Generates context-aware feedback questions
3. Prompts user for retrospective input
4. Saves responses to `devforgeai/feedback/sessions/`
5. Returns 0 on success, non-zero on failure (timeout, error)

**Timeout handling:**
- Default timeout: 300 seconds (5 minutes)
- If user doesn't respond, hook times out
- Timeout is NOT an error (feedback is optional)

---

### Step 6.4: Record Hook Status

Track hook invocation outcome:

```bash
if [ $EXIT_CODE -eq 0 ]; then
  if [ $? -eq 0 ]; then
    HOOK_STATUS="triggered"
  else
    HOOK_STATUS="failed"
  fi
else
  HOOK_STATUS="skipped"
fi
```

**Hook status values:**
- `triggered`: Hooks invoked successfully
- `skipped`: check-hooks returned 1 (configuration skipped)
- `failed`: invoke-hooks returned non-zero (error, timeout)

---

### Step 6.5: Return Status to Skill Result

Include hook status in final result returned to command:

```
result = {
  "qa_result": "[PASSED|FAILED|PARTIAL]",
  "display": "[template from qa-result-interpreter]",
  "hook_status": "[triggered|skipped|failed]",
  "story_update": "[pending|completed|skipped]",  # Phase 7 result
  "next_steps": "[...]"
}
```

**Command receives this complete result and displays it.**

---

## DevForgeAI CLI Commands

### devforgeai-validate check-hooks

**Purpose:** Check if hooks should trigger based on configuration.

**Usage:**
```bash
devforgeai-validate check-hooks --operation=qa --status=[completed|failed|partial]
```

**Arguments:**
- `--operation`: Operation type (qa, dev, release)
- `--status`: Operation status (completed, failed, partial)

**Exit Codes:**
- `0`: Hooks should trigger (configuration allows)
- `1`: Hooks should skip (configuration blocks)
- `2`: Error reading configuration

**Implementation:** `.claude/scripts/devforgeai_cli/cli.py` (check_hooks command)

---

### devforgeai-validate invoke-hooks

**Purpose:** Invoke feedback hooks with context.

**Usage:**
```bash
devforgeai-validate invoke-hooks --operation=qa --story=[STORY-ID]
```

**Arguments:**
- `--operation`: Operation type (qa, dev, release)
- `--story`: Story ID for context

**Exit Codes:**
- `0`: Hook invoked successfully
- `1`: Hook failed (timeout, error, user cancelled)

**Implementation:** `.claude/scripts/devforgeai_cli/cli.py` (invoke_hooks command)

---

## Integration with Framework

### Relationship to /qa Command Phases (OLD)

**Before STORY-034 refactoring:**
- Phase 4 in `/qa` command invoked feedback hooks
- Business logic in command (violates lean orchestration)

**After STORY-034 refactoring:**
- Phase 6 in `devforgeai-qa` skill invokes feedback hooks
- Command only orchestrates (delegates to skill)
- Pattern compliance: 100%

### Relationship to STORY-024

**STORY-024:** Wire hooks into /qa command (added Phase 4)
**STORY-034:** Move Phase 4 to skill as Phase 6 (lean orchestration compliance)

**No functional change:** Hooks still work identically, just invoked from correct architectural layer.

---

## Edge Cases

### Hook Configuration Missing

**Scenario:** `devforgeai/feedback/hooks.yaml` doesn't exist

**Behavior:**
- `check-hooks` returns exit code 1 (skip)
- Phase 6 Step 6.4 records `HOOK_STATUS="skipped"`
- QA continues normally
- No error displayed

### Hook Times Out

**Scenario:** User doesn't respond to feedback questions within timeout

**Behavior:**
- `invoke-hooks` returns exit code 1 (timeout)
- Non-blocking pattern catches error: `|| { echo "Warning..." }`
- Phase 6 Step 6.4 records `HOOK_STATUS="failed"`
- QA result unchanged
- Warning displayed: "⚠️ Feedback hook failed, QA result unchanged"

### Hook Invocation Crashes

**Scenario:** `invoke-hooks` crashes (Python exception, missing dependency)

**Behavior:**
- Exit code non-zero
- Non-blocking pattern catches error
- QA result unchanged
- Warning displayed
- Skill continues to Phase 7

### Multiple QA Runs

**Scenario:** User runs `/qa STORY-001 deep` twice

**Behavior:**
- Each run independently triggers Phase 6
- Hook check happens for each run
- If both fail, both invoke hooks (2 feedback sessions created)
- Sessions timestamped uniquely
- No duplicate suppression (each run is independent)

---

## Performance Characteristics

**Overhead:**
- `check-hooks`: <10ms (reads YAML, returns exit code)
- `invoke-hooks` (if triggered): 1-300 seconds (depends on user response time)
- **Total Phase 6 overhead:** <100ms if skipped, 1-300s if triggered

**NFR-P1 Compliance:**
- Requirement: <5 second overhead for Phase 4 (now Phase 6)
- Measured: <100ms if hooks skipped (most common)
- Measured: 1-5 seconds if hooks triggered and user responds quickly
- **Status:** COMPLIANT

---

## Testing Checklist

**Unit Tests:**
- [ ] Status determination (PASSED→completed, FAILED→failed, PARTIAL→partial)
- [ ] check-hooks called with correct arguments
- [ ] invoke-hooks conditionally called based on exit code 0
- [ ] Non-blocking error handling (|| pattern)

**Integration Tests:**
- [ ] Full QA flow with hooks enabled (failures-only)
- [ ] QA result unchanged by hook failure
- [ ] Hook invocation logged in feedback sessions
- [ ] Performance <100ms overhead when skipped

**Edge Case Tests:**
- [ ] hooks.yaml missing → hooks skipped
- [ ] Hook timeout → QA continues, warning displayed
- [ ] Hook crash → QA continues, warning displayed

---

## Success Criteria

Phase 6 succeeds when:
- [ ] Status correctly determined from QA result
- [ ] check-hooks called with --operation=qa --status=$STATUS
- [ ] invoke-hooks conditionally called (only if exit code 0)
- [ ] Hook failures handled without breaking QA
- [ ] Hook status included in result returned to command
- [ ] QA result immutable by hook outcome
- [ ] Performance <100ms overhead when skipped

---

**This workflow ensures feedback hooks integrate seamlessly with QA validation while maintaining non-blocking behavior and lean orchestration pattern compliance.**
