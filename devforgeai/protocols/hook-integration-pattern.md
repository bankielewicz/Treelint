# Hook Integration Pattern for DevForgeAI Commands

**Version:** 1.1
**Status:** Production Ready
**Reference Stories:** STORY-023 (/dev pilot), STORY-030 (/create-context), STORY-031 (/ideate), STORY-032 (/create-ui), STORY-033 (/audit-deferrals - executable bash)
**Date:** 2025-11-17 (Updated with STORY-033 performance optimizations)

---

## Purpose

This document defines the standard pattern for integrating feedback hooks into DevForgeAI slash commands. Hooks enable users to provide retrospective feedback about their experience using the framework, contributing to continuous improvement.

**Pattern Origin:** Validated through STORY-023 (/dev command pilot)

---

## Pattern Overview

Each command that modifies framework state should trigger feedback hooks **after the primary operation completes successfully**. This enables users to reflect on their experience while it's fresh.

### When to Implement Hook Integration

Commands that should integrate hooks:
- ✅ `/dev` - Implement user story (STORY-023 - COMPLETE)
- ✅ `/create-context` - Generate context files (STORY-030 - COMPLETE)
- ✅ `/ideate` - Transform requirements (STORY-031 - PENDING)
- ✅ `/create-ui` - Generate UI specs (STORY-032 - PENDING)
- ✅ `/audit-deferrals` - Audit technical debt (STORY-033 - PENDING)

Commands that should NOT integrate hooks:
- ❌ `/qa` - Validation only (passive, doesn't modify state)
- ❌ `/release` - High-stakes deployment (separate feedback pattern)
- ❌ `/audit-budget` - Read-only analysis (no state change)
- ❌ `/orchestrate` - Orchestrates other commands (hooks via sub-commands)

---

## Implementation Pattern (4 Steps)

### Step 1: Determine Operation Status

Verify that the primary operation completed successfully and determine status:

**For `/dev` command:**
```bash
if [ "$TESTS_PASSED" = "true" ]; then
  STATUS="completed"
else
  STATUS="failed"
fi
```

**For `/create-context` command:**
```bash
if [ -f "devforgeai/context/tech-stack.md" ] && \
   [ -f "devforgeai/context/dependencies.md" ] && \
   [ -f "devforgeai/context/coding-standards.md" ] && \
   [ -f "devforgeai/context/architecture-constraints.md" ] && \
   [ -f "devforgeai/context/anti-patterns.md" ] && \
   [ -f "devforgeai/context/source-tree.md" ]; then
  OPERATION_STATUS="completed"
else
  OPERATION_STATUS="failed"
fi
```

**Pattern:** Verify primary operation success before checking hooks.

---

### Step 2: Check Hook Eligibility

Check if the user is eligible for feedback hooks based on their configuration:

```bash
# Query hook configuration (respects hooks.yaml rules)
devforgeai check-hooks --operation=[OPERATION] --status=[STATUS]
HOOK_CHECK_EXIT=$?

# Exit code 0 = eligible for feedback
# Exit code 1 = not eligible (skip patterns, rate limits, configuration)
```

**Operation Values:**
- `dev` (for /dev command)
- `create-context` (for /create-context command)
- `ideate` (for /ideate command)
- `create-ui` (for /create-ui command)
- `audit-deferrals` (for /audit-deferrals command)

**Status Values:**
- `completed` (primary operation succeeded)
- `failed` (primary operation failed/incomplete)

**Pattern:** Always check eligibility before invoking hooks (respects user configuration).

---

### Step 3: Invoke Hooks if Eligible

If eligible (exit code 0), trigger the feedback conversation:

```bash
if [ $HOOK_CHECK_EXIT -eq 0 ]; then
  # Invoke feedback hook (errors are non-blocking)
  devforgeai invoke-hooks --operation=[OPERATION] [--story=$STORY_ID] || {
    # Hook invocation failed, but continue anyway
    # Primary operation already completed (success criteria met)
    # Failure is logged to devforgeai/feedback/sessions/ (if applicable)
  }
fi

# Step 3 complete - user may have provided optional feedback
```

**Operation-Specific Notes:**
- `/dev`: Include `--story=$STORY_ID` argument
- `/create-context`: No story ID (creates context, not a story)
- `/ideate`: No story ID (creates requirements, not a story)
- `/create-ui`: Optional `--story=$STORY_ID` if story context exists
- `/audit-deferrals`: No special arguments needed

**Pattern:** Non-blocking error handling (failures don't prevent command completion).

---

### Step 4: Phase Complete

Regardless of hook outcome, the command proceeds to its final completion phase:

```bash
# All primary operations remain completed (✅ success criteria met)
# Hook failures don't prevent command completion (✅ non-blocking)
# Command continues to final success report/summary
```

**Pattern:** Hooks are always secondary; primary operation success is what matters.

---

## Key Characteristics

Every hook integration must exhibit these 5 characteristics:

### 1. Non-Blocking ✅

Hook failures don't interrupt the primary command:
- Exit code from hook is captured but never propagates
- `||` operator ensures command continues on hook error
- Primary operation remains completed regardless

### 2. Configuration-Aware ✅

Hooks respect user configuration in `hooks.yaml`:
- `enabled`: true/false (global override)
- `trigger_on`: all | success-only | failures-only | never
- `skip_patterns`: List of operations to skip
- `rate_limit`: Max feedback conversations per period

**Command must never bypass user configuration.**

### 3. Optional Feedback ✅

Users can opt out or skip feedback:
- Users may see feedback survey if configured and eligible
- Skipping the survey is always allowed (no judgment)
- Partial responses are saved (user can exit mid-conversation)

### 4. Persistent ✅

User responses are saved for later analysis:
- Feedback saved to `devforgeai/feedback/sessions/` directory
- Responses remain available even if user closes terminal mid-survey
- Metadata captured: timestamp, operation, story_id, status

### 5. Lightweight ✅

Hook integration adds minimal overhead:
- `check-hooks` call: 5-20ms (usually <10ms)
- `invoke-hooks` call: 30-90 seconds (only if user engages with feedback)
- When skipped (exit code 1): <5ms overhead

**No impact on command performance when hooks disabled.**

---

## Position in Command Workflow

Hook integration always follows this sequence:

```
...
Phase N-1: Primary Operation Complete (all work done)
Phase N: Feedback Hook Integration ← You are here
  └─ Determine status
  └─ Check eligibility
  └─ Invoke hooks (if eligible)
  └─ Continue regardless of outcome
Phase N+1: Success Report / Final Summary
...
```

**Critical:** Hooks execute AFTER primary operation completes, BEFORE final success message displays.

---

## Error Handling Strategy

### Happy Path (Normal Case)

```
Primary Operation Succeeds
  ↓
User eligible (exit code 0)
  ↓
User provides feedback
  ↓
Command displays success report
```

### Hook Skipped (User Not Eligible)

```
Primary Operation Succeeds
  ↓
User not eligible (exit code 1)
  ↓
No hooks invoked
  ↓
Command displays success report (without mention of feedback)
```

### Hook Check Fails (CLI Missing/Config Invalid)

```
Primary Operation Succeeds
  ↓
check-hooks command fails/returns error
  ↓
invoke-hooks is skipped (never called)
  ↓
Command displays success report (continues normally)
```

### Hook Invocation Fails (Feedback System Error)

```
Primary Operation Succeeds
  ↓
User eligible (exit code 0)
  ↓
invoke-hooks command fails
  ↓
Failure caught by || operator (non-blocking)
  ↓
Command displays success report (continues despite hook error)
```

**All paths lead to success report.** Primary operation success is what matters.

---

## Testing Requirements

Every hook integration must pass these test categories:

### 1. Happy Path Tests
- Primary operation succeeds → hooks eligible → feedback collected → success
- Verify all 3 steps execute correctly
- Verify feedback is saved to session directory

### 2. Eligibility Tests
- User eligible (exit code 0) → hooks invoked
- User not eligible (exit code 1) → hooks skipped
- No error message displayed when skipped

### 3. Error Handling Tests
- Hook check fails → invoke-hooks skipped → command succeeds
- Hook invocation fails → non-blocking → command succeeds
- All error scenarios maintain primary operation success

### 4. Configuration Tests
- `enabled: false` → hooks skipped → success
- `trigger_on: failures-only` + success status → hooks skipped
- `skip_patterns: [operation]` → hooks skipped

### 5. Performance Tests
- Hook check <100ms (target) when skipped
- Actual overhead 5-20ms (well under target)
- No command delay when hooks disabled

### 6. Backward Compatibility Tests
- Existing command usage unchanged when hooks disabled
- No breaking changes to command interface
- Old projects work without `hooks.yaml`

---

## Code Quality Standards

Hook integration code must follow these standards:

### Bash Best Practices
```bash
# ✅ CORRECT
devforgeai check-hooks --operation=create-context --status=$OPERATION_STATUS
HOOK_CHECK_EXIT=$?

# ❌ WRONG
HOOK_CHECK_EXIT=$(devforgeai check-hooks --operation=create-context --status=$OPERATION_STATUS)
# (uses command substitution, harder to debug)
```

### Error Handling
```bash
# ✅ CORRECT
devforgeai invoke-hooks --operation=create-context || {
  # Hook failed, but continue
}

# ❌ WRONG
devforgeai invoke-hooks --operation=create-context || echo "Warning: hooks failed"
# (error messages confuse users about non-blocking behavior)
```

### Comments
```bash
# ✅ CORRECT
# Query hook configuration (respects hooks.yaml rules)
devforgeai check-hooks --operation=create-context --status=$OPERATION_STATUS
HOOK_CHECK_EXIT=$?

# Exit code 0 = eligible for feedback
# Exit code 1 = not eligible (skip patterns, rate limits, configuration)

# ❌ WRONG
devforgeai check-hooks # check for eligibility
# (comment doesn't explain status parameter or exit codes)
```

---

## Integration Checklist

Before marking hook integration complete:

- [ ] Step 1: Operation status determined correctly
- [ ] Step 2: Hook eligibility check implemented
- [ ] Step 3: Conditional hook invocation implemented
- [ ] Step 4: Non-blocking error handling with `||` operator
- [ ] Comments explain each step and exit codes
- [ ] Pattern matches /dev pilot exactly (or justified adaptation)
- [ ] All 6 test categories pass (happy path, eligibility, errors, config, performance, compat)
- [ ] Command still works with hooks disabled (backward compat)
- [ ] Performance overhead <100ms when hooks skipped
- [ ] Code review passed (pattern consistency verified)

---

## Related Documentation

**Implementation Examples:**
- `.claude/commands/dev.md` - Phase 6 implementation (pilot)
- `.claude/commands/create-context.md` - Phase N implementation (this pattern)

**Supporting Documentation:**
- `devforgeai/hooks/hook-integration-guide.md` - Detailed CLI guide
- `STORY-023-wire-hooks-into-dev-command-pilot.story.md` - Pilot story
- `STORY-030-wire-hooks-into-create-context-command.story.md` - This story

**Related Commands:**
- `/dev [STORY-ID]` - Development workflow with feedback hooks
- `/create-context [project-name]` - Context file creation with feedback hooks
- `/ideate [description]` - Requirements discovery with feedback hooks (planned)

---

## Maintenance

This pattern is validated through production use in `/dev` and `/create-context` commands. Updates to this pattern should:

1. Be documented via ADR (if significant change)
2. Be validated through new pilot story (before applying to all commands)
3. Be backward compatible with existing implementations
4. Include rationale for change

**Last Updated:** 2025-11-17
**Validated By:** code-reviewer (95.7/100 quality score)
**Status:** Production Ready

