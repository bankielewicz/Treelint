# Release Hooks Troubleshooting Guide

**Part of:** STORY-025 - Wire hooks into /release command

**Purpose:** Diagnose and resolve issues with hook integration in the /release command

**Created:** 2025-11-14

---

## Quick Diagnosis

### Hooks Not Triggering

**Symptom:** `/release` completes but no feedback prompt appears

**Check:**
1. Is hooks.yaml configured?
   ```bash
   cat devforgeai/config/hooks.yaml | grep -A 5 "release-staging\|release-production"
   ```

2. Are hooks enabled?
   ```bash
   devforgeai check-hooks --operation=release-staging --status=SUCCESS
   echo "Exit code: $?"  # Should be 0 if eligible
   ```

3. Does trigger mode match deployment status?
   - Staging SUCCESS: Should trigger (default: `trigger_on: all`)
   - Production SUCCESS: Should NOT trigger (default: `trigger_on: failures-only`)
   - Production FAILURE: Should trigger (failures always eligible)

4. Is devforgeai CLI installed?
   ```bash
   which devforgeai
   # Should return: /c/Users/bryan/bin/devforgeai or similar
   ```

5. Check logs:
   ```bash
   cat devforgeai/logs/release-hooks-{STORY-ID}.log
   ```

**Solution Paths:**

| Problem | Solution |
|---------|----------|
| hooks.yaml missing | Copy `devforgeai/config/hooks.yaml.example-release` to `hooks.yaml` |
| enabled: false | Set `enabled: true` in hooks.yaml |
| Wrong trigger mode | Change `trigger_on` or `on_success`/`on_failure` flags |
| CLI not installed | Run: `pip install --break-system-packages -e .claude/scripts/` |
| Logs show errors | See "Error Scenarios" section below |

---

## Error Scenarios

### Error 1: "command not found: devforgeai"

**Full Error:**
```
/release: line XXX: devforgeai: command not found
⚠️  Note: Post-deployment feedback unavailable (hook CLI error)
```

**Cause:** devforgeai CLI not installed or not in PATH

**Impact:** Hooks skipped, deployment continues successfully

**Solution:**
```bash
# Install devforgeai CLI
pip install --break-system-packages -e .claude/scripts/

# Verify installation
devforgeai --version
which devforgeai
```

**Prevention:** Add CLI installation check to pre-release validation

---

### Error 2: "hooks.yaml not found"

**Full Error:**
```
check-hooks failed (exit code: 2)
Error: Configuration file not found: devforgeai/config/hooks.yaml
Deployment continues without feedback
```

**Cause:** hooks.yaml not configured

**Impact:** Hooks skipped, deployment continues successfully

**Solution:**
```bash
# Copy example configuration
cp devforgeai/config/hooks.yaml.example-release devforgeai/config/hooks.yaml

# Customize for your needs
nano devforgeai/config/hooks.yaml
```

**Prevention:** Check hooks.yaml exists during pre-release validation

---

### Error 3: "Hook invocation timeout"

**Full Error:**
```
⚠️  Hook invocation timeout - feedback skipped
Deployment continues normally
```

**Cause:** invoke-hooks exceeded 30-second timeout (user didn't respond)

**Impact:** No feedback collected, deployment continues successfully

**Solution:**
```bash
# Option 1: Increase timeout in hooks.yaml
operations:
  release-staging:
    timeout_seconds: 300  # 5 minutes instead of 30 seconds

# Option 2: User responds faster (or skips questions)
# Option 3: Reduce question count (fewer questions = faster completion)
```

**Prevention:** Use reasonable question counts (3-5 for staging, 5-7 for production)

---

### Error 4: "Production deployment FAILED + Hook error"

**Full Error:**
```
❌ Production deployment FAILED
⚠️  Unable to collect failure feedback (hook CLI error)
⚠️  Manual incident response required
```

**Cause:** Production deployment failed AND hook CLI failed

**Impact:** Production failure is accurate, feedback collection unsuccessful

**Critical:** Production failure is the PRIMARY issue, not hook failure.

**Solution:**
1. **Address production failure FIRST:**
   ```bash
   # Check deployment logs
   cat devforgeai/deployment/logs/{STORY-ID}-production.log

   # Execute rollback if needed
   /rollback STORY-001

   # Fix production issue
   ```

2. **Then debug hook issue:**
   ```bash
   # Check hook logs
   cat devforgeai/logs/release-hooks-{STORY-ID}.log

   # Test hook CLI manually
   devforgeai check-hooks --operation=release-production --status=FAILURE
   ```

**Prevention:** Monitor hook system health separately from deployment health

---

### Error 5: "Hook skipped: trigger does not match (failures-only mode)"

**Message:**
```
ℹ️  Production feedback skipped (failures-only mode, deployment succeeded)
```

**Cause:** Production deployment succeeded, hooks configured for failures-only

**Impact:** None (this is EXPECTED behavior for production success)

**Is This an Error?** NO - This is normal and desired behavior.

**Explanation:**
- Production hooks default to `failures-only` mode
- Production SUCCESS: Hooks skip (less intrusive for successful deployments)
- Production FAILURE: Hooks trigger (collect critical incident feedback)

**Override (if you want feedback on production success):**
```yaml
operations:
  release-production:
    trigger_on: "all"      # Change from failures-only
    on_success: true       # Explicitly trigger on success
    on_failure: true
```

---

## Deployment Status vs Hook Status

### Critical Principle

**Deployment status is INDEPENDENT of hook status.**

| Deployment Outcome | Hook Outcome | Final Deployment Status | Correct? |
|--------------------|--------------|------------------------|----------|
| SUCCESS | Success (feedback collected) | SUCCESS | ✅ Yes |
| SUCCESS | Failed (hook CLI error) | SUCCESS | ✅ Yes |
| SUCCESS | Skipped (not eligible) | SUCCESS | ✅ Yes |
| FAILURE | Success (feedback collected) | FAILURE | ✅ Yes |
| FAILURE | Failed (hook CLI error) | FAILURE | ✅ Yes |
| FAILURE | Skipped (hooks disabled) | FAILURE | ✅ Yes |

**If deployment status changes based on hook status:** ❌ BUG - This violates hook integration pattern

---

## Performance Issues

### Issue: check-hooks Taking >100ms

**Measurement:**
```bash
time devforgeai check-hooks --operation=release-staging --status=SUCCESS
# Target: <100ms (p95)
```

**If exceeds target:**
1. Profile hook CLI (identify bottleneck)
2. Check hooks.yaml size (large configs slow parsing)
3. Check file system performance (slow disk?)
4. Consider caching hook config in memory

**Impact:** Minor delay in deployment workflow

### Issue: invoke-hooks Taking >3s

**Measurement:**
```bash
time devforgeai invoke-hooks --operation=release-staging --story=STORY-025
# Target: <3s (p95, excluding user interaction)
```

**If exceeds target:**
1. Check question count (too many questions?)
2. Check file I/O performance (slow disk writes?)
3. Profile feedback persistence layer
4. Consider async feedback writing

**Impact:** Noticeable delay in deployment workflow

### Issue: Total Hook Overhead >3.5s

**Measurement:**
```bash
# Measure total time from check-hooks to feedback saved
START=$(date +%s%N)
devforgeai check-hooks ... && devforgeai invoke-hooks ...
END=$(date +%s%N)
echo "Duration: $(( (END - START) / 1000000 ))ms"
# Target: <3500ms (p95)
```

**If exceeds target:**
1. Optimize check-hooks (reduce to <50ms)
2. Optimize invoke-hooks (reduce to <2s)
3. Reduce question count
4. Consider async hook invocation (non-blocking)

**Impact:** Cumulative delay across staging + production

---

## Configuration Issues

### Issue: Hooks Triggering When They Shouldn't

**Symptom:** Production SUCCESS triggers feedback (but you want failures-only)

**Diagnosis:**
```bash
cat devforgeai/config/hooks.yaml | grep -A 10 "release-production"
```

**Check:**
- Is `trigger_on: "all"` when it should be `"failures-only"`?
- Is `on_success: true` when it should be `false`?

**Solution:**
```yaml
operations:
  release-production:
    trigger_on: "failures-only"   # Change from "all"
    on_success: false              # Explicit
    on_failure: true
```

### Issue: Hooks NOT Triggering When They Should

**Symptom:** Production FAILURE doesn't trigger feedback

**Diagnosis:**
```bash
# Test hook eligibility manually
devforgeai check-hooks --operation=release-production --status=FAILURE
echo "Exit code: $?"  # Should be 0 (eligible)
```

**Check:**
- Is `enabled: false` when it should be `true`?
- Is `on_failure: false` when it should be `true`?
- Is operation name correct? (`release-production` not `production` or `release`)

**Solution:**
```yaml
operations:
  release-production:
    enabled: true                  # Must be true
    trigger_on: "failures-only"
    on_failure: true               # Must be true for failures to trigger
```

---

## Logging Issues

### Issue: Log File Not Created

**Symptom:** `devforgeai/logs/release-hooks-{STORY-ID}.log` doesn't exist after deployment

**Cause:** Hook check returned 1 (not eligible) - no logging for skipped hooks

**Is This an Error?** NO - This is normal behavior when hooks are skipped.

**Logs only created when:**
- check-hooks returns 0 (eligible) → invoke-hooks executed
- check-hooks returns 2+ (error) → error logged

**Logs NOT created when:**
- check-hooks returns 1 (not eligible) → hooks skipped normally

### Issue: Log File Permissions Error

**Symptom:**
```
Error: Permission denied: devforgeai/logs/release-hooks-STORY-025.log
```

**Solution:**
```bash
# Create logs directory with correct permissions
mkdir -p devforgeai/logs
chmod 755 devforgeai/logs

# Ensure user can write to logs
touch devforgeai/logs/release-hooks-test.log
rm devforgeai/logs/release-hooks-test.log
```

---

## Feedback Persistence Issues

### Issue: Feedback File Not Created

**Symptom:** Hook invoked, user answered questions, but no `devforgeai/feedback/releases/*.json` file

**Diagnosis:**
```bash
# Check feedback directory exists
ls -la devforgeai/feedback/releases/

# Check invoke-hooks exit code
echo $?  # Should be 0 if successful
```

**Possible causes:**
1. Directory doesn't exist → Create: `mkdir -p devforgeai/feedback/releases`
2. Permission denied → Check: `ls -ld devforgeai/feedback/`
3. invoke-hooks failed → Check logs: `devforgeai/logs/release-hooks-{STORY-ID}.log`
4. User aborted early (Ctrl+C) → Partial feedback should still save

**Solution:**
```bash
# Create feedback directory
mkdir -p devforgeai/feedback/releases
chmod 755 devforgeai/feedback/releases

# Test feedback writing
devforgeai invoke-hooks --operation=release-staging --story=TEST-001
# Answer questions, verify file created
```

### Issue: Feedback File Wrong Location

**Expected:** `devforgeai/feedback/releases/STORY-025-staging-{timestamp}.json`

**Actual:** `devforgeai/feedback/dev/STORY-025-{timestamp}.json`

**Cause:** Operation name incorrect in invoke-hooks call

**Fix in skill code:**
```bash
# WRONG:
devforgeai invoke-hooks --operation=dev --story=$STORY_ID

# CORRECT:
devforgeai invoke-hooks --operation=release-staging --story=$STORY_ID
```

---

## Integration Issues

### Issue: Hook Phase Not Executing

**Symptom:** Deployment completes but Phase 2.5 or 3.5 never executes

**Diagnosis:**
```bash
# Check if skill includes hook phases
grep "Phase 2.5\|Phase 3.5" .claude/skills/devforgeai-release/SKILL.md

# Check reference files exist
ls .claude/skills/devforgeai-release/references/post-*-hooks.md
```

**Possible causes:**
1. Skill not updated → Update: devforgeai-release SKILL.md
2. Reference files missing → Create: post-staging-hooks.md, post-production-hooks.md
3. Skill phase logic skips hook phases → Review skill execution flow

**Solution:** Verify skill modifications from STORY-025 are present

### Issue: Hook Executed Twice (Staging + Production)

**Symptom:** User gets two feedback prompts during single `/release STORY-001 production`

**Diagnosis:** This is EXPECTED behavior if deploying to production (staging → production sequence)

**Deployment flow:**
```
/release STORY-001 production
  ↓
Phase 2: Staging Deployment (staging always happens first)
  ↓
Phase 2.5: Post-Staging Hooks (feedback #1)
  ↓
Phase 3: Production Deployment
  ↓
Phase 3.5: Post-Production Hooks (feedback #2, if production fails or configured)
```

**Is This an Error?** NO - Staging is always deployed before production (safe practice)

**To skip staging feedback:**
```yaml
operations:
  release-staging:
    enabled: false  # Disable staging hooks
```

---

## Testing Issues

### Issue: Tests Passing But Real Deployment Fails

**Symptom:** `pytest tests/integration/test_release_hooks_integration.py` passes, but actual `/release` hook integration fails

**Cause:** Tests use mocking (unittest.mock) - they don't test real CLI integration

**Diagnosis:**
```bash
# Run manual test with real deployment
/release STORY-025 staging

# Check if hooks actually invoked
cat devforgeai/logs/release-hooks-STORY-025.log
```

**Solution:**
1. Run manual tests (see "Manual Testing Scenarios" below)
2. Add end-to-end tests (no mocking)
3. Test with real devforgeai CLI

### Issue: Test Suite Timeout

**Symptom:** `pytest` hangs or exceeds timeout

**Cause:** pytest.ini has `timeout = 30` but pytest-timeout plugin not installed

**Solution:**
```bash
# Option 1: Install plugin
pip install pytest-timeout

# Option 2: Remove timeout from pytest.ini (already done in STORY-025)
# timeout = 30  # Commented out
```

---

## Manual Testing Scenarios

### Scenario 1: Staging Deployment Success with Hooks Enabled

**Setup:**
```bash
# Ensure hooks enabled
cat > devforgeai/config/hooks.yaml << 'EOF'
enabled: true
operations:
  release-staging:
    enabled: true
    trigger_on: "all"
EOF
```

**Execute:**
```bash
/release STORY-025 staging
```

**Expected:**
1. Staging deployment completes successfully
2. Feedback prompt appears: "How did the staging deployment go?"
3. User answers questions
4. Feedback saved to `devforgeai/feedback/releases/STORY-025-staging-{timestamp}.json`
5. Deployment proceeds to completion

**Verify:**
```bash
# Check feedback file exists
ls -la devforgeai/feedback/releases/STORY-025-staging-*.json

# Check log
cat devforgeai/logs/release-hooks-STORY-025.log
```

---

### Scenario 2: Staging Deployment Failure with Hooks

**Setup:** Same as Scenario 1

**Simulate Failure:**
- Intentionally fail smoke tests OR
- Break deployment configuration

**Expected:**
1. Staging deployment FAILS
2. Feedback prompt appears with failure-specific questions
3. User provides failure context
4. Feedback saved with `"deployment_status": "FAILURE"`
5. Deployment workflow stops (deployment failed, not hook issue)

**Verify:**
```bash
# Check feedback includes failure status
cat devforgeai/feedback/releases/STORY-025-staging-*.json | jq '.operation_context.deployment_status'
# Should return: "FAILURE"
```

---

### Scenario 3: Production Success (Failures-Only Mode)

**Setup:**
```bash
cat > devforgeai/config/hooks.yaml << 'EOF'
enabled: true
operations:
  release-production:
    enabled: true
    trigger_on: "failures-only"
    on_success: false
    on_failure: true
EOF
```

**Execute:**
```bash
/release STORY-025 production
```

**Expected:**
1. Staging deployment completes (may trigger feedback)
2. Production deployment completes successfully
3. **NO production feedback prompt** (failures-only mode, deployment succeeded)
4. Log shows: "Production feedback skipped (failures-only mode, deployment succeeded)"

**Verify:**
```bash
# Should NOT have production feedback file (or only staging file)
ls devforgeai/feedback/releases/STORY-025-production-*.json
# Expected: No such file (correct behavior)

# Check log
grep "failures-only mode" devforgeai/logs/release-hooks-STORY-025.log
```

---

### Scenario 4: Production Failure (Triggers Feedback)

**Setup:** Same as Scenario 3 (failures-only mode)

**Simulate Failure:**
- Fail production smoke tests OR
- Break production deployment

**Expected:**
1. Production deployment FAILS
2. **Feedback prompt appears** (failures always trigger, even in failures-only mode)
3. Critical incident questions presented
4. User provides failure/rollback observations
5. Feedback saved with severity="critical"

**Verify:**
```bash
# Production failure feedback exists
cat devforgeai/feedback/releases/STORY-025-production-*.json | jq '.metadata.severity'
# Should return: "critical"
```

---

### Scenario 5: Hook CLI Not Installed

**Setup:**
```bash
# Temporarily remove devforgeai from PATH
mv /c/Users/bryan/bin/devforgeai /c/Users/bryan/bin/devforgeai.backup
```

**Execute:**
```bash
/release STORY-025 staging
```

**Expected:**
1. Deployment proceeds normally (no hooks)
2. No feedback prompt
3. No errors displayed
4. Deployment completes successfully

**Verify:**
```bash
# No logs created (hooks never attempted)
ls devforgeai/logs/release-hooks-STORY-025.log
# Expected: No such file (correct behavior)

# Restore CLI
mv /c/Users/bryan/bin/devforgeai.backup /c/Users/bryan/bin/devforgeai
```

---

### Scenario 6: User Aborts Feedback (Ctrl+C)

**Setup:** Hooks enabled

**Execute:**
```bash
/release STORY-025 staging
# When feedback prompt appears, press Ctrl+C
```

**Expected:**
1. Hook process terminates
2. Partial feedback saved (questions answered before abort)
3. Deployment continues immediately
4. No deployment status change (still SUCCESS/FAILURE based on deployment)

**Verify:**
```bash
# Partial feedback file exists
cat devforgeai/feedback/releases/STORY-025-staging-*.json | jq '.questions[] | select(.skipped == true)'
# Shows questions user didn't answer (skipped due to abort)
```

---

### Scenario 7: Multiple Deployment Retries

**Execute:**
```bash
# First attempt (simulate failure)
/release STORY-025 staging  # Fail smoke tests

# Second attempt (fix and retry)
/release STORY-025 staging  # Success
```

**Expected:**
1. First attempt: Feedback collected, saved to `STORY-025-staging-{timestamp1}.json`
2. Second attempt: Feedback collected, saved to `STORY-025-staging-{timestamp2}.json`
3. Two separate feedback files (timestamp differentiation)
4. No file overwrites

**Verify:**
```bash
# Two separate files exist
ls -la devforgeai/feedback/releases/STORY-025-staging-*.json
# Should show 2 files with different timestamps
```

---

### Scenario 8: Config Changed Mid-Deployment

**Execute:**
```bash
# Start deployment
/release STORY-025 staging &
DEPLOY_PID=$!

# While deployment running, disable hooks
echo "enabled: false" > devforgeai/config/hooks.yaml

# Wait for deployment to complete
wait $DEPLOY_PID
```

**Expected:**
1. Hook eligibility checked at deployment COMPLETION time (not start time)
2. If hooks disabled by completion, check-hooks returns 1 (not eligible)
3. No feedback prompt
4. Deployment completes normally

**Verify:**
```bash
# No feedback file created
ls devforgeai/feedback/releases/STORY-025-staging-*.json
# Expected: No such file (hooks disabled at completion time)
```

---

## Common Mistakes

### Mistake 1: Checking Hook Status Before Deployment Status

**WRONG:**
```bash
# Check hooks FIRST
devforgeai check-hooks --operation=release-staging --status=???
# What status? Deployment hasn't completed yet!
```

**CORRECT:**
```bash
# Determine deployment status FIRST
if deployment_succeeded; then
    STATUS="SUCCESS"
else
    STATUS="FAILURE"
fi

# THEN check hooks
devforgeai check-hooks --operation=release-staging --status=$STATUS
```

### Mistake 2: Throwing Errors on Hook Failures

**WRONG:**
```bash
devforgeai check-hooks ... || exit 1  # ❌ Breaks deployment
devforgeai invoke-hooks ... || exit 1  # ❌ Breaks deployment
```

**CORRECT:**
```bash
devforgeai check-hooks ... || {
    echo "⚠️  Hook check failed - feedback skipped"
    # Deployment continues
}

devforgeai invoke-hooks ... || {
    echo "⚠️  Hook invocation failed - feedback skipped"
    # Deployment continues
}
```

### Mistake 3: Using Wrong Operation Name

**WRONG:**
```bash
devforgeai check-hooks --operation=staging --status=SUCCESS
# Operation name is "release-staging", not "staging"
```

**CORRECT:**
```bash
devforgeai check-hooks --operation=release-staging --status=SUCCESS
devforgeai check-hooks --operation=release-production --status=FAILURE
```

**Operation names:**
- ✅ `release-staging`
- ✅ `release-production`
- ❌ `staging`
- ❌ `production`
- ❌ `release`

---

## Emergency Procedures

### Emergency: Hooks Breaking Production Deployment

**If production deployment fails DUE TO hooks (this should NEVER happen):**

1. **Immediate: Comment out hook integration in skill:**
   ```bash
   # Edit: .claude/skills/devforgeai-release/SKILL.md
   # Comment out Phase 2.5 and 3.5 hook invocations
   ```

2. **Fast: Disable hooks via config (no code change):**
   ```bash
   cat > devforgeai/config/hooks.yaml << 'EOF'
   enabled: false
   EOF
   ```

3. **Verify deployment works without hooks:**
   ```bash
   /release {STORY-ID} production
   # Should succeed without feedback prompts
   ```

4. **Root cause analysis:**
   - Review logs: `devforgeai/logs/release-hooks-{STORY-ID}.log`
   - Identify why hooks affected deployment status
   - File incident report: `devforgeai/incidents/hook-deployment-interference.md`

5. **Fix hook integration bug:**
   - Ensure hook failures are truly non-blocking
   - Add error handling: `|| true` or `|| { ... }`
   - Test again with hooks enabled

**Rollback time:** <2 minutes (config change) OR <5 minutes (code change + terminal restart)

---

## Health Check

**Run this checklist periodically to ensure hook system healthy:**

```bash
# 1. CLI available
devforgeai --version

# 2. Config valid
devforgeai check-hooks --operation=release-staging --status=SUCCESS

# 3. Feedback directory writable
touch devforgeai/feedback/releases/test.json && rm devforgeai/feedback/releases/test.json

# 4. Logs directory writable
touch devforgeai/logs/test.log && rm devforgeai/logs/test.log

# 5. Performance acceptable
time devforgeai check-hooks --operation=release-staging --status=SUCCESS
# Should be <100ms

# 6. Recent feedback files exist (if hooks enabled)
ls -lt devforgeai/feedback/releases/ | head -5
```

**All checks pass:** ✅ Hook system healthy

---

## Support

**For additional help:**
- Main troubleshooting guide: `devforgeai/docs/INVOKE-HOOKS-TROUBLESHOOTING.md`
- Hook integration pattern: `devforgeai/docs/hook-integration-pattern.md`
- Configuration reference: `devforgeai/config/hooks.yaml.example-release`
- Test suite: `tests/integration/test_release_hooks_integration.py`

**Issue tracking:**
- Create issue in: `devforgeai/issues/`
- Include: Logs, config, error messages, reproduction steps

---

**Remember: Hooks are OPTIONAL. If hooks cause issues, disable them and deployment continues normally. Deployment correctness is the priority.**
