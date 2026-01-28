# Hook Integration Pattern for DevForgeAI Commands (Design Specification)

**Status:** DESIGN SPECIFICATION - Not yet implemented in live commands
**Story:** STORY-023 (Design validated via tests, implementation pending)

---

## Overview

This document specifies the standard pattern for integrating feedback hooks into DevForgeAI slash commands.

**Design status:**
- ✅ Pattern designed and documented
- ✅ Tests validate pattern will work (23 tests passing)
- ❌ NOT yet implemented in any live command (including /dev)

**When implemented, applies to:** All 11 DevForgeAI commands

---

## Commands Planned for Integration

**NOTE:** None of these commands currently have Phase 6 implemented. This is the PLANNED rollout.

| Command | Hook Operation | Status Values | Priority | Planned Story |
|---------|----------------|---------------|----------|---------------|
| `/dev` | dev | completed, failed | **PILOT** | **STORY-023 (This story - design only)** |
| `/qa` | qa | completed, failed | High | Planned: STORY-024 |
| `/release` | release | completed, failed | High | Planned: STORY-025 |
| `/orchestrate` | orchestrate | completed, failed | High | Planned: STORY-026 |
| `/create-story` | create-story | completed, failed | Medium | Planned: STORY-027 |
| `/create-epic` | create-epic | completed, failed | Medium | Planned: STORY-028 |
| `/create-sprint` | create-sprint | completed, failed | Medium | Planned: STORY-029 |
| `/ideate` | ideate | completed, failed | Low | Planned: STORY-030 |
| `/create-context` | create-context | completed, failed | Low | Planned: STORY-031 |
| `/create-ui` | create-ui | completed, failed | Low | Planned: STORY-032 |
| `/audit-deferrals` | audit-deferrals | completed, failed | Low | Planned: STORY-033 |

---

## Standard Integration Code Template

**Add this as the final phase in each command:**

```bash
### Phase N: Invoke Feedback Hook

# Determine status based on command outcome
if [ "$COMMAND_SUCCEEDED" = "true" ]; then
  STATUS="completed"
else
  STATUS="failed"
fi

# Check if hooks should trigger (respects configuration)
devforgeai check-hooks --operation=OPERATION_NAME --status=$STATUS
if [ $? -eq 0 ]; then
  # Invoke feedback hook (errors logged, not thrown)
  devforgeai invoke-hooks --operation=OPERATION_NAME --story=$STORY_ID || {
    echo "⚠️ Feedback hook failed, continuing..."
  }
fi

# Command completes successfully regardless of hook outcome
```

**Variables to replace:**
- `OPERATION_NAME` → command name (e.g., "qa", "release", "orchestrate")
- `$COMMAND_SUCCEEDED` → command-specific success variable
- `$STORY_ID` → story identifier from command context
- `N` → actual phase number (e.g., Phase 5, Phase 6, Phase 7)

---

## Command-Specific Adaptations

### /qa Command

```bash
### Phase 6: Invoke Feedback Hook

# Determine status from QA result
if [ "$QA_PASSED" = "true" ]; then
  STATUS="completed"
else
  STATUS="failed"
fi

devforgeai check-hooks --operation=qa --status=$STATUS
if [ $? -eq 0 ]; then
  devforgeai invoke-hooks --operation=qa --story=$STORY_ID || {
    echo "⚠️ Feedback hook failed, continuing..."
  }
fi
```

### /release Command

```bash
### Phase 7: Invoke Feedback Hook

# Determine status from deployment result
if [ "$DEPLOYMENT_SUCCESSFUL" = "true" ]; then
  STATUS="completed"
else
  STATUS="failed"
fi

devforgeai check-hooks --operation=release --status=$STATUS
if [ $? -eq 0 ]; then
  devforgeai invoke-hooks --operation=release --story=$STORY_ID || {
    echo "⚠️ Feedback hook failed, continuing..."
  }
fi
```

### /orchestrate Command

```bash
### Phase 8: Invoke Feedback Hook

# Determine status from full workflow
if [ "$ALL_PHASES_PASSED" = "true" ]; then
  STATUS="completed"
else
  STATUS="failed"
fi

devforgeai check-hooks --operation=orchestrate --status=$STATUS
if [ $? -eq 0 ]; then
  devforgeai invoke-hooks --operation=orchestrate --story=$STORY_ID || {
    echo "⚠️ Feedback hook failed, continuing..."
  }
fi
```

---

## Integration Checklist

**For each command integration:**

### Pre-Integration
- [ ] Review command structure (identify final phase)
- [ ] Identify command success variable
- [ ] Determine appropriate phase number (N)
- [ ] Check if $STORY_ID available in command context

### Implementation
- [ ] Add Phase N after current final phase
- [ ] Implement STATUS determination logic
- [ ] Add `devforgeai check-hooks` call with correct operation name
- [ ] Add conditional `invoke-hooks` call (if exit code 0)
- [ ] Add error handling wrapper (|| { echo "warning" })
- [ ] Update command documentation

### Testing
- [ ] Create 18+ integration tests (follow STORY-023 pattern)
- [ ] Test enabled configuration (feedback triggers)
- [ ] Test disabled configuration (hooks skip)
- [ ] Test failures-only mode
- [ ] Test hook failure handling (non-blocking)
- [ ] Test skip tracking (3 skips → disable prompt)
- [ ] Verify performance <5s overhead
- [ ] Edge cases: timeout, circular invocation, missing CLI

### Configuration
- [ ] Add operation to `devforgeai/config/hooks.yaml`
- [ ] Set appropriate trigger conditions (on_success, on_failure)
- [ ] Configure skip tracking (threshold: 3)
- [ ] Document configuration in user guide

### QA & Rollout
- [ ] Code review (target ⭐⭐⭐⭐⭐)
- [ ] All tests passing (100% pass rate)
- [ ] No regression in command functionality
- [ ] Performance baseline established
- [ ] Pilot testing with real usage (if high-priority command)

---

## Configuration Templates

### High-Value Commands (qa, release, orchestrate)

```yaml
hooks:
  operations:
    OPERATION_NAME:
      enabled: true
      on_success: true     # Trigger on both success and failure
      on_failure: true
      skip_tracking:
        enabled: true
        threshold: 3
```

### Creation Commands (create-story, create-epic, create-sprint)

```yaml
hooks:
  operations:
    OPERATION_NAME:
      enabled: true
      on_success: true     # Trigger only on success
      on_failure: false
      skip_tracking:
        enabled: true
        threshold: 3
```

### Utility Commands (ideate, create-context, create-ui, audit-deferrals)

```yaml
hooks:
  operations:
    OPERATION_NAME:
      enabled: false       # Disabled by default, user can enable
      on_success: false
      on_failure: true     # Only trigger if issues found
      skip_tracking:
        enabled: true
        threshold: 3
```

---

## Rollout Strategy

### Phase 1: High-Priority Commands (Week 1-2)

**Commands:** /qa, /release, /orchestrate

**Rationale:**
- Most valuable feedback opportunities
- Critical workflow points
- High user engagement

**Success Criteria:**
- Hook integration complete
- 18+ tests passing per command
- Performance <5s verified
- No command breakage reported

---

### Phase 2: Creation Commands (Week 3-4)

**Commands:** /create-story, /create-epic, /create-sprint

**Rationale:**
- Moderate feedback value
- Planning workflow insights
- User decision validation

**Success Criteria:**
- All 3 commands integrated
- Configuration tested
- User feedback collected

---

### Phase 3: Remaining Commands (Week 5-6)

**Commands:** /ideate, /create-context, /create-ui, /audit-deferrals

**Rationale:**
- Lower usage frequency
- Specialized workflows
- Optional feedback

**Success Criteria:**
- 100% command coverage (11/11 commands)
- Framework-wide hook system operational
- Comprehensive documentation complete

---

## Testing Pattern

**Follow STORY-023 test suite structure:**

1. **Test Classes** (8 per command)
   - TestPhase{N}Addition (Phase N added correctly)
   - TestFeedbackTriggersOnSuccess
   - TestFeedbackSkipsWhenDisabled
   - TestFeedbackFailuresOnly
   - TestHookFailureHandling
   - TestSkipTracking
   - TestPerformanceImpact
   - TestEdgeCases

2. **Test Fixtures** (7 reusable)
   - test_project_dir - Temporary project structure
   - hooks_config_enabled - Enabled configuration
   - hooks_config_disabled - Disabled configuration
   - hooks_config_failures_only - Failures-only mode
   - mock_command - Command mocking
   - mock_check_hooks - check-hooks CLI mock
   - mock_invoke_hooks - invoke-hooks CLI mock

3. **Test Count** (18+ minimum)
   - 2-3 tests per AC
   - Edge cases (timeout, circular, missing CLI)
   - Configuration modes (3-4 variations)

---

## Performance Targets

**Per-command overhead budget:**
- check-hooks execution: <100ms
- Context extraction: <200ms
- Total Phase N overhead: <5 seconds

**Measurement:**
```bash
# Measure Phase N execution
time {
  devforgeai check-hooks --operation=OPERATION --status=completed
  if [ $? -eq 0 ]; then
    devforgeai invoke-hooks --operation=OPERATION --story=STORY-ID
  fi
}

# Target: <5.0 seconds total
```

---

## Support Resources

**Documentation:**
- `devforgeai/docs/hooks/user-guide.md` - Enable/disable configuration
- `devforgeai/docs/hooks/troubleshooting.md` - This file
- `devforgeai/docs/hooks/integration-pattern.md` - Integration checklist

**Reference Implementation:**
- STORY-023: `/dev` command pilot integration
- `.claude/commands/dev.md` - Phase 6 implementation
- `tests/integration/test_phase6_hooks_integration.py` - Test suite template

**CLI Tools:**
- STORY-021: `devforgeai check-hooks` implementation
- STORY-022: `devforgeai invoke-hooks` implementation

---

**Created:** 2025-11-13 (STORY-023)
**Last Updated:** 2025-11-13
**Status:** Production Ready (pilot validated)
