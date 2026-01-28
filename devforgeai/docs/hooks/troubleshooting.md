# Troubleshooting: Hook Integration Issues (Design Reference)

**Status:** DESIGN DOCUMENTATION - Based on test scenarios, not production issues
**Story:** STORY-023 (Tests validate these scenarios, implementation pending)

---

## Important Note

This troubleshooting guide is based on **integration test scenarios** (23 tests in test_phase6_hooks_integration.py), NOT actual production issues, since Phase 6 is not yet implemented in live commands.

**What this covers:**
- Issues that WOULD occur if/when Phase 6 is implemented
- Based on test cases (all 23 tests currently passing)
- Validated resolution approaches

**What this does NOT cover:**
- Actual production issues (Phase 6 not live yet)
- Real user reports (pilot phase hasn't started)

---

## Tested Scenarios and Resolutions

### Issue 1: Hooks Not Triggering (check-hooks returns 1)

**Symptom:** `/dev` completes but no feedback conversation appears

**Diagnosis:**
```bash
# Check hook configuration
cat devforgeai/config/hooks.yaml

# Manually test check-hooks
devforgeai check-hooks --operation=dev --status=completed
echo $?  # Should return 0 if hooks should trigger
```

**Common Causes:**
1. **Hooks globally disabled**
   - Fix: Set `hooks.enabled: true` in config

2. **Operation disabled**
   - Fix: Set `hooks.operations.dev.enabled: true`

3. **Wrong trigger mode**
   - Status=completed but `on_success: false`
   - Fix: Set `on_success: true` for success triggers

4. **Failures-only mode active**
   - Mode="failures_only" skips success status
   - Fix: Change `mode: "all"` or set status to "failed"

**Resolution:**
```yaml
# Correct configuration for success triggers
hooks:
  enabled: true
  mode: "all"
  operations:
    dev:
      enabled: true
      on_success: true    # ← Must be true for success triggers
      on_failure: false
```

---

### Issue 2: Hook Failures Breaking /dev Command

**Symptom:** `/dev` exits with error when hooks fail

**Diagnosis:**
```bash
# Check Phase 6 implementation in command
grep -A 5 "invoke-hooks" .claude/commands/dev.md

# Should see: || { echo "warning..." }
```

**Cause:** Missing error handling wrapper

**Resolution:**
```bash
# WRONG (hook error breaks command):
devforgeai invoke-hooks --operation=dev --story=$STORY_ID

# CORRECT (error caught, command continues):
devforgeai invoke-hooks --operation=dev --story=$STORY_ID || {
  echo "⚠️ Feedback hook failed, continuing..."
}
```

**Verification:**
```bash
# Simulate hook failure
devforgeai invoke-hooks --operation=dev --story=NONEXISTENT 2>/dev/null || echo "Error caught"

# Command should still exit 0
echo $?  # Should be 0 (success)
```

---

### Issue 3: Hook Timeout (>5 seconds)

**Symptom:** Hook invocation hangs or takes >5 seconds

**Diagnosis:**
```bash
# Measure hook execution time
time devforgeai invoke-hooks --operation=dev --story=STORY-023
```

**Common Causes:**
1. **Skill execution slow**
   - devforgeai-feedback skill taking too long
   - Check skill token usage (should be <50K tokens)

2. **Network latency**
   - If using external APIs
   - Check API response times

3. **File I/O bottleneck**
   - Too many feedback session files
   - Check `devforgeai/feedback/sessions/` size

**Resolution:**
```yaml
# Add timeout to check-hooks
hooks:
  timeout: 5    # Kill after 5 seconds

# In command Phase 6:
timeout 5 devforgeai check-hooks --operation=dev --status=$STATUS || {
  echo "⚠️ Hook timeout, skipping feedback"
  exit 0
}
```

**Performance Optimization:**
```bash
# Clean old feedback sessions (>30 days)
find devforgeai/feedback/sessions/ -mtime +30 -delete

# Reduce skill token usage
# (Review devforgeai-feedback skill for optimization)
```

---

### Issue 4: Circular Invocation (Hook triggers /dev which triggers hook)

**Symptom:** Infinite loop, /dev keeps re-triggering itself

**Diagnosis:**
```bash
# Check for DEVFORGEAI_HOOK_ACTIVE guard
env | grep DEVFORGEAI_HOOK_ACTIVE

# Check invocation depth
ps aux | grep "devforgeai invoke-hooks" | wc -l
# Should be 0 or 1, NOT >1
```

**Cause:** Missing circular invocation guard

**Resolution:**
```bash
# Add guard in invoke-hooks implementation
if [ "$DEVFORGEAI_HOOK_ACTIVE" = "1" ]; then
  echo "⚠️ Circular invocation detected, skipping hook"
  exit 1
fi

export DEVFORGEAI_HOOK_ACTIVE=1

# ... invoke hook logic ...

unset DEVFORGEAI_HOOK_ACTIVE
```

**Prevention:**
```yaml
# In hooks.yaml, ensure circular detection enabled
hooks:
  circular_detection: true    # Default: true
  max_depth: 1                # Maximum hook invocation depth
```

---

### Issue 5: Missing CLI Tools (check-hooks or invoke-hooks not found)

**Symptom:** `command not found: devforgeai`

**Diagnosis:**
```bash
# Check if CLI installed
which devforgeai

# Check version
devforgeai --version
```

**Resolution:**
```bash
# Install DevForgeAI CLI
pip install --break-system-packages -e .claude/scripts/

# Verify installation
devforgeai --version

# Should output: devforgeai-cli version X.X.X
```

**Fallback Behavior:**
```bash
# Add CLI check in Phase 6
if ! command -v devforgeai &> /dev/null; then
  echo "⚠️ devforgeai CLI not found, skipping hooks"
  exit 0  # Continue without hooks
fi

# Then proceed with check-hooks
devforgeai check-hooks --operation=dev --status=$STATUS
```

---

### Issue 6: Configuration File Missing or Invalid

**Symptom:** `FileNotFoundError: hooks.yaml not found`

**Diagnosis:**
```bash
# Check if config exists
ls -la devforgeai/config/hooks.yaml

# Validate YAML syntax
python3 -c "import yaml; yaml.safe_load(open('devforgeai/config/hooks.yaml'))"
```

**Resolution:**
```bash
# Create default config if missing
mkdir -p devforgeai/config
cat > devforgeai/config/hooks.yaml << 'EOF'
hooks:
  enabled: true
  mode: "all"
  operations:
    dev:
      enabled: true
      on_success: true
      on_failure: false
EOF

# Fix YAML syntax errors
# (Use yamllint or Python to identify issues)
```

---

## Quick Diagnostics Checklist

**When hooks aren't working:**
1. [ ] Check `hooks.enabled: true` in config
2. [ ] Check `operations.dev.enabled: true`
3. [ ] Verify `on_success` or `on_failure` matches status
4. [ ] Run `devforgeai check-hooks` manually (exit code 0?)
5. [ ] Check devforgeai CLI is installed (`which devforgeai`)
6. [ ] Verify `devforgeai/config/hooks.yaml` exists and valid
7. [ ] Check for error messages in command output
8. [ ] Test with `mode: "all"` to eliminate mode issues

**When hooks break commands:**
1. [ ] Verify `|| { echo "warning" }` wrapper present
2. [ ] Check command exit code (should be 0 despite hook failure)
3. [ ] Review Phase 6 implementation for missing error handling
4. [ ] Add timeout wrapper if hooks hanging

**Performance Issues:**
1. [ ] Measure hook execution time (<5s target)
2. [ ] Check skill token usage (<50K target)
3. [ ] Clean old feedback sessions
4. [ ] Review devforgeai-feedback skill for optimization

---

## Support Resources

**Documentation:**
- STORY-021: devforgeai check-hooks CLI implementation
- STORY-022: devforgeai invoke-hooks CLI implementation
- STORY-023: Phase 6 hook integration (pilot)

**Testing:**
- `tests/integration/test_phase6_hooks_integration.py` - 23 test cases
- All edge cases covered (timeout, circular, failures)

**Configuration Examples:**
- `devforgeai/config/hooks.yaml` - Default configuration
- See `devforgeai/docs/hooks/user-guide.md` for all configuration modes

---

**Created:** 2025-11-13 (STORY-023)
**Updated:** 2025-11-13
