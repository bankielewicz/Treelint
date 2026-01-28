# Troubleshooting Guide: devforgeai invoke-hooks Command

**Version:** 1.0
**Created:** 2025-11-13
**Story:** STORY-022

---

## Common Issues

### Issue 1: Timeout After 30 Seconds

**Symptom:**
```
[ERROR] Feedback hook timeout after 30s
invoke-hooks returned exit code 1
```

**Cause:** Feedback skill invocation took longer than 30 seconds

**Solutions:**
1. **Check feedback skill performance:**
   ```bash
   # Verify feedback skill is responsive
   # (Manual test once STORY-023 integration complete)
   ```

2. **Review operation context size:**
   ```bash
   # Large contexts (>50KB) may slow processing
   # invoke-hooks automatically summarizes, but check logs
   ```

3. **Check system resources:**
   ```bash
   # High CPU/memory usage can cause timeouts
   top
   free -h
   ```

**Workaround:**
- Timeout is intentional to prevent blocking parent operations
- Hook failure is logged but parent command continues
- Review feedback session files for partial data

---

### Issue 2: Circular Invocation Detected

**Symptom:**
```
[WARNING] Circular invocation detected, aborting
invoke-hooks returned exit code 1
```

**Cause:** invoke-hooks was called from within an active feedback session

**Solution:**
- This is expected behavior (prevents infinite loops)
- Do not call invoke-hooks from within feedback skill
- Circular invocation protection is working correctly

**Verification:**
```bash
# Check environment variable
echo $DEVFORGEAI_HOOK_ACTIVE

# If set to "1", circular protection is active
```

---

### Issue 3: Context Extraction Fails

**Symptom:**
```
[ERROR] Failed to extract operation context
invoke-hooks returned exit code 1
```

**Cause:** TodoWrite data unavailable or malformed

**Solutions:**
1. **Verify TodoWrite data exists:**
   ```bash
   # Check if operation created TodoWrite data
   # (Depends on parent command implementation)
   ```

2. **Check context extraction:**
   ```python
   from devforgeai_cli.context_extraction import extract_context

   # Test extraction manually
   context = extract_context('dev', 'STORY-001')
   print(context)
   ```

3. **Review extraction logs:**
   ```bash
   # invoke-hooks logs full context extraction details
   grep "Context extracted" ~/devforgeai/logs/hooks.log
   ```

**Workaround:**
- invoke-hooks falls back to minimal context on extraction failure
- Partial context (operation name only) is still passed to feedback skill

---

### Issue 4: Secrets Not Sanitized

**Symptom:**
Secrets appear in logs or feedback session files

**Cause:** Secret pattern not recognized (pattern missing from 54 predefined patterns)

**Solution:**
1. **Add new secret pattern:**
   Edit `.claude/scripts/devforgeai_cli/context_extraction.py`:
   ```python
   SECRET_PATTERNS = [
       # Add new pattern
       (r'your_secret_key_pattern', r'***'),
   ]
   ```

2. **Report missing pattern:**
   Create issue for new secret type to be added

3. **Verify sanitization:**
   ```bash
   # Run sanitization tests
   pytest .claude/scripts/devforgeai_cli/tests/test_invoke_hooks.py::TestSecretSanitization -v
   ```

**Current Patterns (54 total):**
- API Keys, Passwords, OAuth Tokens, AWS Credentials
- Database URLs, GCP Keys, GitHub Tokens, SSH Keys
- JWT Tokens, PII (SSN, credit cards)

---

### Issue 5: Feedback Skill Invocation Fails

**Symptom:**
```
[ERROR] Failed to invoke devforgeai-feedback skill
invoke-hooks returned exit code 1
```

**Cause:** devforgeai-feedback skill not available or misconfigured

**Solutions:**
1. **Verify skill exists:**
   ```bash
   ls .claude/skills/devforgeai-feedback/SKILL.md
   ```

2. **Check skill configuration:**
   - Ensure devforgeai-feedback skill is properly configured
   - Verify skill has required tools (Read, Write, AskUserQuestion)

3. **Test skill invocation:**
   ```bash
   # Test feedback skill manually (once STORY-023 complete)
   # Skill(command="devforgeai-feedback")
   ```

**Workaround:**
- invoke-hooks logs error and returns failure
- Parent command continues (graceful degradation)
- Fix skill configuration and retry operation

---

### Issue 6: Hook Invocation Doesn't Trigger

**Symptom:**
```
Hook invocation skipped (hooks not configured)
```

**Cause:** Hooks not enabled in configuration

**Solution:**
1. **Check hook configuration:**
   ```bash
   devforgeai check-hooks --operation=dev --status=completed
   ```

2. **Verify `devforgeai/config/hooks.yaml`:**
   ```yaml
   hooks:
     enabled: true
     operations:
       dev:
         enabled: true
         mode: "interactive"
   ```

3. **Enable hooks:**
   Edit `devforgeai/config/hooks.yaml` and set `enabled: true`

---

### Issue 7: Context Size Exceeds 50KB

**Symptom:**
```
[WARNING] Context size 125KB exceeds 50KB limit, summarizing
Context truncated to 48KB
```

**Cause:** Too many todos (>100) or large error stack traces

**Solution:**
- This is expected behavior (automatic summarization)
- invoke-hooks limits context to 50KB to prevent excessive memory usage
- Only first/last todos included, middle todos summarized

**Verification:**
```bash
# Check context size in logs
grep "Context size" ~/devforgeai/logs/hooks.log
```

**Impact:**
- Summarized context still contains key information
- Feedback skill receives abbreviated context
- Full context available in operation logs

---

### Issue 8: Parent Command Blocked by Hook

**Symptom:**
Parent command hangs waiting for hook completion

**Cause:** Hook invocation is synchronous (blocking)

**Solution:**
- By design, invoke-hooks blocks until completion or timeout (30s max)
- This is intentional to collect user feedback before continuing
- 30-second timeout prevents indefinite blocking

**Workaround:**
```bash
# Run invoke-hooks in background (non-blocking)
devforgeai invoke-hooks --operation=dev --story=STORY-001 &
```

---

### Issue 9: No Feedback Session Created

**Symptom:**
```
invoke-hooks completed but no session file in devforgeai/feedback/sessions/
```

**Cause:** User cancelled feedback conversation or skill failed

**Solution:**
1. **Check logs for cancellation:**
   ```bash
   grep "User cancelled" ~/devforgeai/logs/hooks.log
   ```

2. **Check for skill errors:**
   ```bash
   grep "ERROR" ~/devforgeai/logs/hooks.log
   ```

3. **Verify feedback directory:**
   ```bash
   ls -la devforgeai/feedback/sessions/
   ```

**Expected Behavior:**
- If user cancels, partial session may be saved
- If skill fails, no session file created (error logged)

---

### Issue 10: Multiple Concurrent Invocations Fail

**Symptom:**
```
Some concurrent invoke-hooks calls fail with errors
Success rate: 94% (below 99% requirement)
```

**Cause:** Resource contention or file locking issues

**Solution:**
1. **Check system resources:**
   ```bash
   # Verify CPU and memory availability
   top
   ```

2. **Review concurrent test results:**
   ```bash
   pytest .claude/scripts/devforgeai_cli/tests/test_invoke_hooks.py::TestConcurrentOperations -v
   ```

3. **Check file system:**
   ```bash
   # Verify devforgeai/feedback/sessions/ is writable
   ls -ld devforgeai/feedback/sessions/
   ```

**Expected Performance:**
- invoke-hooks designed for >99% success rate under concurrent load
- Isolated invocations with unique operation IDs
- No shared state between concurrent calls

---

## Debugging Commands

### Enable Verbose Logging

```bash
devforgeai invoke-hooks --operation=dev --story=STORY-001 --verbose
```

### Check Hook Status

```bash
# Verify hooks are configured
devforgeai check-hooks --operation=dev --status=completed

# Check hook configuration
cat devforgeai/config/hooks.yaml
```

### View Context Extraction

```python
from devforgeai_cli.context_extraction import extract_context

context = extract_context('dev', 'STORY-001')
import json
print(json.dumps(context, indent=2))
```

### Test Secret Sanitization

```python
from devforgeai_cli.context_extraction import sanitize_context

test_context = {
    "api_key": "sk-1234567890abcdef",
    "password": "SuperSecret123"
}

sanitized = sanitize_context(test_context)
print(sanitized)  # Should show: api_key=***, password=***
```

### Run Unit Tests

```bash
# All tests
pytest .claude/scripts/devforgeai_cli/tests/test_invoke_hooks.py -v

# Specific category
pytest .claude/scripts/devforgeai_cli/tests/test_invoke_hooks.py::TestTimeout -v
```

### Run Manual Tests

```bash
# Full manual test suite
bash .claude/scripts/devforgeai_cli/tests/manual_test_invoke_hooks.sh

# Individual test (skip 30s timeout)
# (Edit script to comment out Test 3)
```

---

## Log Locations

**Hook invocation logs:**
- `~/devforgeai/logs/hooks.log` (if logging configured)
- Standard output (if --verbose flag used)

**Feedback session files:**
- `devforgeai/feedback/sessions/<operation-id>.json`

**Context extraction logs:**
- Included in hook invocation logs

---

## Performance Metrics

**Expected Performance:**
- Context extraction: <200ms (95th percentile)
- End-to-end invocation: <3s (95th percentile)
- Success rate: >99% (under concurrent load)

**Measure Performance:**
```bash
# Run performance tests
pytest .claude/scripts/devforgeai_cli/tests/test_invoke_hooks.py::TestPerformance -v
```

---

## Getting Help

**If troubleshooting doesn't resolve your issue:**

1. **Check story specification:**
   - `devforgeai/specs/Stories/STORY-022-implement-devforgeai-invoke-hooks-cli-command.story.md`

2. **Review test cases:**
   - `.claude/scripts/devforgeai_cli/tests/test_invoke_hooks.py` (117 test scenarios)

3. **Examine source code:**
   - `.claude/scripts/devforgeai_cli/hooks.py` (HookInvocationService)
   - `.claude/scripts/devforgeai_cli/context_extraction.py` (ContextExtractor)

4. **Create issue:**
   - Document error symptoms, logs, and reproduction steps

---

**This guide satisfies STORY-022 DoD requirement: "Troubleshooting guide (timeout, failures, circular invocation)"**
