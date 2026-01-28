# /qa Command Hook Integration - User Guide

**Story:** STORY-024 - Wire hooks into /qa command
**Date:** 2025-11-13
**Status:** Production Ready

---

## Overview

The `/qa` command now includes automatic feedback hook integration (Phase 4). This triggers retrospective feedback conversations when QA validation fails, helping teams reflect on quality issues and improve practices.

---

## How It Works

### Default Behavior (failures-only mode)

**When QA passes:**
- No feedback prompt
- /qa completes normally with success message
- Silent and non-intrusive

**When QA fails:**
- Automatic feedback conversation starts
- Context-aware questions about the failure
- Questions reference specific violations (coverage %, anti-patterns, etc.)
- Helps identify root causes and improvement opportunities

### Hook Integration Flow

```
User runs: /qa STORY-001 deep

Phase 1: Argument Validation ✅
Phase 2: Invoke QA Skill ✅
Phase 3: Display Results ✅

Phase 4: Invoke Feedback Hook (NEW)
├─ Step 1: Determine STATUS from QA result
│  └─ PASSED → "completed", FAILED → "failed", PARTIAL → "partial"
│
├─ Step 2: Check if hooks should trigger
│  └─ devforgeai check-hooks --operation=qa --status=$STATUS
│
└─ Step 3: Conditionally invoke feedback (if check-hooks returned 0)
   └─ devforgeai invoke-hooks --operation=qa --story=STORY-001 --context="violations..."

Phase 5: Update Story Status ✅
```

**Key Feature:** Hook failures are non-blocking - if feedback fails, QA result is unchanged.

---

## Configuration

Hooks are configured in `devforgeai/hooks/hooks.yaml`:

```yaml
hooks:
  retrospective_feedback:
    enabled: true
    trigger_on: failures-only  # Options: always, failures-only, never
    operations:
      - dev
      - qa     # ← /qa hook integration (STORY-024)
      - release
```

**Trigger modes:**
- **failures-only** (default): Feedback only on QA failures
- **always**: Feedback on every /qa run (pass or fail)
- **never**: Disable feedback for /qa

---

## Examples

### Example 1: QA Failure with Feedback

```bash
> /qa STORY-042 deep

# ... QA validation runs ...

❌ Deep QA FAILED

Violations detected:
- Coverage: 75% (target: 85%) - HIGH
- God Object: UserService.cs - MEDIUM

# Feedback hook triggered automatically
ℹ️ Feedback hook invoked (qa failed)

# Feedback conversation starts:
"QA validation failed for STORY-042. Let's reflect on what happened."

1. Coverage was 75% (target 85%) - what prevented higher coverage?
   a) Time constraints
   b) Complex edge cases difficult to test
   c) Unclear requirements for some scenarios
   d) Other (please specify)

2. God Object detected in UserService.cs - how did this occur?
   ...

# After feedback conversation, QA result remains: FAILED
# Story status NOT updated (still in "Dev Complete")
# User fixes issues and re-runs /qa
```

### Example 2: QA Pass with No Feedback

```bash
> /qa STORY-042 deep

# ... QA validation runs ...

✅ Deep QA PASSED

All quality gates met:
- Coverage: 92%
- No violations detected
- All acceptance criteria validated

# Hook check determines: Don't trigger (failures-only mode + pass)
# No feedback prompt
# /qa completes normally

Story status updated: Dev Complete → QA Approved
```

### Example 3: Hook Failure (Non-Blocking)

```bash
> /qa STORY-042 deep

# ... QA validation runs ...

❌ Deep QA FAILED

# Hook invocation fails (timeout, skill error, etc.)
⚠️ Feedback hook failed, QA result unchanged

# QA result still displayed correctly
# Story status reflects actual QA result (not affected by hook failure)
```

---

## Benefits

### For Developers
- **Context-aware reflection:** Questions reference specific failures
- **Learning opportunities:** Understand what caused quality gaps
- **Pattern recognition:** Identify recurring issues over time

### For Teams
- **Quality insights:** Track common failure patterns
- **Process improvement:** Data-driven quality practices
- **Knowledge sharing:** Feedback sessions capture team knowledge

### For Projects
- **Technical debt prevention:** Early detection of quality issues
- **Continuous improvement:** Feedback loop drives quality up
- **Metrics:** Track quality trends over time

---

## Troubleshooting

### Issue: Feedback triggered when I don't want it

**Cause:** Config set to `trigger_on: always`

**Solution:**
```yaml
# Edit devforgeai/hooks/hooks.yaml
hooks:
  retrospective_feedback:
    trigger_on: failures-only  # Change from "always"
```

### Issue: Feedback not triggering on failures

**Cause 1:** Hooks disabled

**Solution:**
```yaml
hooks:
  retrospective_feedback:
    enabled: true  # Change from false
```

**Cause 2:** Operation not in list

**Solution:**
```yaml
hooks:
  retrospective_feedback:
    operations:
      - qa  # Ensure "qa" is in the list
```

### Issue: Hook fails with timeout

**Cause:** Feedback skill taking too long

**Solution:**
- Check `devforgeai/hooks/errors.log` for details
- Hook failures don't affect QA result (non-blocking by design)
- Report issue if persistent

### Issue: Hook fails with "skill not found"

**Cause:** devforgeai-feedback skill not available

**Solution:**
- Verify skill exists: `ls .claude/skills/ | grep feedback`
- Check hooks config references correct skill
- Install skill if missing

---

## Performance

**Overhead:** <5 seconds (validated in tests)

**Measurement:**
- Phase 4 execution: <5s (check + invoke)
- QA result determination: Unchanged
- Total /qa runtime: +5s max

**Validated in:**
- NFR-P1 tests (performance requirement)
- 36 integration tests
- Real-world usage

---

## Integration Pattern

### Command Structure

```markdown
Phase 3: Provide Next Steps
  └─ Display QA results and recommendations

Phase 4: Invoke Feedback Hook (Non-Blocking)  ← NEW
  ├─ Determine STATUS from QA result
  ├─ Check if hooks should trigger
  └─ Conditionally invoke feedback

Phase 5: Update Story Status (Deep Mode Only)
  └─ Update story file if QA passed
```

### Error Handling

```bash
# Non-blocking pattern
devforgeai invoke-hooks ... || {
  echo "⚠️ Feedback hook failed, QA result unchanged"
}

# Ensures:
# - Hook failures logged, not thrown
# - QA result accuracy 100% unchanged
# - User always sees correct QA outcome
```

### Context Extraction

```bash
if [ "$STATUS" = "failed" ] || [ "$STATUS" = "partial" ]; then
  # Extract violation details
  COVERAGE=$(grep -o "Coverage: [0-9]*%" <<< "$RESULT")
  VIOLATION_COUNT=$(grep -c "VIOLATION" <<< "$RESULT")

  VIOLATIONS_CONTEXT="QA $MODE mode $STATUS: $COVERAGE, $VIOLATION_COUNT violations"
fi
```

**Passed to feedback:** Enables context-aware questions

---

## Comparison: /dev vs /qa Hook Integration

| Aspect | /dev (STORY-023) | /qa (STORY-024) |
|--------|------------------|-----------------|
| **Trigger Mode** | failures-only (default) | failures-only (default) |
| **STATUS Values** | completed, failed | completed, failed, partial |
| **Context** | Test results, errors | Coverage, violations, AC status |
| **Typical Trigger** | Tests fail, build fails | Coverage low, violations detected |
| **Typical Skip** | Tests pass, build succeeds | QA passed, no violations |

**Shared pattern:** Both use same hook infrastructure, differ in context

---

## Testing

### Automated Tests (75 total)

**Integration tests (36):**
- Phase 4 exists after Phase 3
- check-hooks called with correct arguments
- invoke-hooks conditionally called
- Hook failures non-blocking
- Feedback triggered on failures
- Feedback skipped on success
- Light/deep mode integration
- Violation context passed
- Performance <5s validated
- Reliability 100% validated

**Unit tests (39):**
- Status mapping (PASSED/FAILED/PARTIAL)
- Violation context extraction
- Invalid input handling
- Mode-independent behavior

### Manual Testing Checklist

**Test 1: QA Failure Triggers Feedback**
- [ ] Run `/qa <story> deep` where story has coverage <80%
- [ ] Verify QA fails
- [ ] Verify feedback conversation starts
- [ ] Verify questions reference coverage %
- [ ] Complete feedback
- [ ] Verify QA result unchanged

**Test 2: QA Success Skips Feedback**
- [ ] Run `/qa <story> deep` where story passes all gates
- [ ] Verify QA passes
- [ ] Verify no feedback prompt
- [ ] Verify story status updated to "QA Approved"

**Test 3: Hook Failure Non-Blocking**
- [ ] Temporarily break feedback skill
- [ ] Run `/qa <story> deep` (fail or pass)
- [ ] Verify warning displayed: "Feedback hook failed"
- [ ] Verify QA result correct (not affected by hook failure)

---

## See Also

- **Implementation:** `.claude/commands/qa.md` (Phase 4 section)
- **Tests:** `tests/integration/test_qa_hooks_integration.py`
- **Feedback System:** `.claude/skills/devforgeai-feedback/`
- **Hook Configuration:** `devforgeai/hooks/hooks.yaml`
- **CLI Validators:** `devforgeai check-hooks`, `devforgeai invoke-hooks`
- **Related:** STORY-023 (/dev hook integration pilot)
- **Related:** STORY-021 (check-hooks CLI), STORY-022 (invoke-hooks CLI)

---

**Questions or issues?** Create an issue in `devforgeai/issues/` or consult troubleshooting section above.
