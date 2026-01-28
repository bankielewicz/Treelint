# Phase 03: Implementation (Green Phase)

**Purpose:** Write minimal code to make failing tests pass.

**Execution Order:** After Phase 02 (Red phase) - tests are failing

**Expected Outcome:** All tests GREEN (passing), ready for refactoring

**Token Cost:** ~900 tokens in skill context (~50,000 in isolated subagent context)

---

## Phase Progress Indicator

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 03/10: Implementation - Green Phase (20% → 30% complete)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Display this indicator at the start of Phase 03 execution.**

---

## Overview

The Green phase focuses on making tests pass with minimal, clean code. No over-engineering allowed.

**Core Principle:** Simplest code that works.

---

## Remediation Mode Check [MANDATORY FIRST]

**CRITICAL:** Before executing normal Phase 03, check if remediation mode is active.

```
# Check remediation mode flag from Phase 01 Step h.1.
IF $REMEDIATION_MODE == true:

    Display: ""
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: "  🔧 REMEDIATION MODE - Phase 03R (Targeted Implementation)"
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: ""

    # Load remediation workflow instead of normal Phase 03
    Read(file_path=".claude/skills/devforgeai-development/references/qa-remediation-workflow.md")

    # Execute Phase 03R: Targeted Implementation from qa-remediation-workflow.md
    # This implements ONLY code for $QA_COVERAGE_GAPS files, not full story

    RETURN after Phase 03R completes
    # Do NOT execute normal Phase 03 below

ELSE:
    # Normal Phase 03 - proceed with full story implementation
    Display: "Proceeding with normal Phase 03 (full story implementation)"
```

**Why this matters:**
- In remediation mode, backend-architect receives specific gap files to fix
- In normal mode, backend-architect receives full story specification
- Wrong mode = implementing full story when only gaps need fixing

---

## Phase 03: Implementation (Green Phase)

**Delegate implementation to backend-architect or frontend-developer subagent.**

### Step 1: Determine Implementation Subagent [MANDATORY]

```
# Check story technical specification or framework type
IF story involves UI/frontend work OR FRAMEWORK in ["React", "Vue", "Angular", "Blazor", "WPF"]:
    IMPLEMENTATION_AGENT = "frontend-developer"

ELIF story involves API/backend work OR FRAMEWORK in ["FastAPI", "Express", "ASP.NET Core", "Spring Boot"]:
    IMPLEMENTATION_AGENT = "backend-architect"

ELSE:
    # Default to backend-architect for business logic
    IMPLEMENTATION_AGENT = "backend-architect"
```

### Step 2: Invoke Implementation Subagent [MANDATORY]

```
Task(
  subagent_type=IMPLEMENTATION_AGENT,
  description="Implement minimal code to pass tests",
  prompt="Implement code to make the failing tests pass (Green phase of TDD).

  Story and tests are already available in conversation.

  Context files to enforce:
  - devforgeai/specs/context/tech-stack.md (locked technologies: {LANGUAGE}, {FRAMEWORK})
  - devforgeai/specs/context/source-tree.md (file placement rules)
  - devforgeai/specs/context/dependencies.md (approved packages only)
  - devforgeai/specs/context/coding-standards.md (code patterns, naming conventions)
  - devforgeai/specs/context/architecture-constraints.md (layer boundaries, DI patterns)
  - devforgeai/specs/context/anti-patterns.md (forbidden patterns to avoid)

  Implementation requirements:
  1. Write MINIMAL code to pass tests (no over-engineering)
  2. Follow coding-standards.md patterns
  3. Respect architecture-constraints.md layer boundaries
  4. Use dependencies.md packages ONLY (ask before adding new ones)
  5. Avoid anti-patterns.md forbidden patterns
  6. Use native tools (Read/Edit/Write, not Bash for files)

  Test command: {TEST_COMMAND}

  If new dependency needed:
  - HALT and use AskUserQuestion to get approval
  - Update dependencies.md after approval
  - Create ADR if significant decision

  Return:
  - Implementation files created/modified
  - Approach taken
  - Any assumptions made
  - Test run status (should be GREEN)"
)
```

### Step 3: Parse Subagent Response [MANDATORY]

```javascript
result = extract_from_subagent_output(response)

files_modified = result["files_modified"]
approach = result["approach"]

Display: "✓ Phase 03 (Green): Implementation by {IMPLEMENTATION_AGENT}"
Display: "  - Files modified: {len(files_modified)}"

FOR file in files_modified:
    Display: "    • {file['path']}: {file['purpose']}"

Display: "  - Approach: {approach}"
```

### Step 4: Verify Tests Pass (Green Phase) [MANDATORY]

```
Bash(command=TEST_COMMAND)

IF all tests pass:
    Display: "✓ GREEN phase confirmed - all tests passing"
    Display: "  Ready for Phase 04 (Refactor)"

ELIF some tests still failing:
    Display: "⚠️ Warning: {failed_count} tests still failing"
    Display: "  Re-invoking {IMPLEMENTATION_AGENT} to fix..."

    # Re-invoke subagent with additional context about failures
    # [Same Task call with failure details added to prompt]

ELSE (tests error):
    Display: "❌ ERROR: Test execution failed"
    HALT development
```

---

### Step 4a: Background Test Execution [CONDITIONAL] (STORY-112)

**Purpose:** Run long-running tests in background while waiting efficiently

**Trigger:** When estimated test duration > 2 minutes (120000ms)

**Reference:** `background-executor.md`, `task-result-aggregation.md`

```
# Load timeout from parallel config
config = Read(file_path="devforgeai/config/parallel-orchestration.yaml")
timeout_ms = config.profiles[config.default_profile].timeout_ms

# Estimate test duration (heuristic)
estimated_duration = estimate_test_duration(TEST_COMMAND, TEST_COUNT)

IF estimated_duration > 120000:  # BACKGROUND_THRESHOLD_MS (2 minutes)

    # Launch tests in background
    background_task = Bash(
        command=TEST_COMMAND,
        run_in_background=true,
        timeout=timeout_ms,
        description="Run test suite in background"
    )

    Display: "Tests running in background (task: {background_task.id})..."
    Display: "   Waiting efficiently for results..."

    # WAIT BEFORE PHASE CHECKPOINT (MANDATORY)
    # Simple approach: wait efficiently without parallel work (zero debt)
    test_result = TaskOutput(
        task_id=background_task.id,
        block=true,
        timeout=timeout_ms
    )

    Display: "Background tests complete"

ELSE:
    # Foreground execution (standard) - tests < 2 minutes
    test_result = Bash(command=TEST_COMMAND)

# Verify results (same for both paths)
IF test_result.exit_code == 0:
    Display: "✓ GREEN phase confirmed - all tests passing"
ELSE:
    Display: "Tests failed - see error output"
    # Handle failure per existing logic
```

**Error Handling:**

```
IF background execution fails OR times out:
    # Fall back to foreground (sequential)
    Display: "Background execution failed, retrying in foreground..."

    foreground_result = Bash(
        command=TEST_COMMAND,
        run_in_background=false,
        timeout=timeout_ms
    )

    test_result = foreground_result
```

**Performance Target:** 50-80% reduction in perceived wait time for long test runs.

---

## Subagents Invoked

**backend-architect OR frontend-developer:**
- Implements minimal code to pass tests
- Enforces all 6 context file constraints
- Uses native tools (Edit/Write for file operations)
- Avoids anti-patterns from anti-patterns.md
- Follows coding-standards.md patterns

---

## Success Criteria

Phase 03 succeeds when:
- [ ] All tests pass (100% pass rate)
- [ ] Minimal code written (no over-engineering)
- [ ] All context file constraints enforced
- [ ] No anti-patterns introduced
- [ ] Test command executes successfully

---

## Common Issues

**Issue 1: New dependency needed**
- HALT development
- Use AskUserQuestion to get user approval
- Update dependencies.md after approval
- Create ADR if significant technology decision

**Issue 2: Tests still failing after implementation**
- Re-invoke implementation subagent with failure details
- May need to refine acceptance criteria
- Check for missing test setup/teardown

**Issue 3: Anti-pattern violation detected**
- Refactor immediately (don't wait for Phase 04)
- Follow anti-patterns.md guidance
- Re-run tests to ensure still passing

---

### Step 4: Update AC Verification Checklist (Phase 03 Items) [NEW - RCA-011]

**Purpose:** Check off AC items related to implementation (real-time progress tracking)

**Execution:** After tests verified GREEN, before Phase 03 checkpoint

**Load AC Checklist Update Workflow:**
```
Read(file_path=".claude/skills/devforgeai-development/references/ac-checklist-update-workflow.md")
```

**Identify Phase 03 AC Items:**
```
Grep(pattern="Phase.*: 2", path="${STORY_FILE}", output_mode="content", -B=1)
```

**Common Phase 03 items:**
- [ ] Implementation code written
- [ ] Business logic in correct location (skill, not command)
- [ ] Character count ≤{target} (for refactoring stories)
- [ ] Line count ≤{target} (for refactoring stories)
- [ ] API endpoints created (for API stories)
- [ ] Database models created (for CRUD stories)

**Update Procedure:** Batch-update all Phase 03 items that are complete

**Display:** "Phase 03 AC Checklist: ✓ {count} items checked | AC Progress: {X}/{Y}"

**Graceful Skip:**
```
IF AC Verification Checklist section not found in story:
  Display: "ℹ️ Story uses DoD-only tracking (AC Checklist not present)"
  Skip AC checklist updates
  Continue to Phase 03 Checkpoint
```

**Performance:** ~30-60 seconds for 4-8 items

---

## ✅ PHASE 03 COMPLETION CHECKPOINT

**Before proceeding to Phase 04 (Refactor), verify ALL steps executed:**

### Mandatory Steps Executed

- [ ] **Step 1:** Implementation subagent determined (backend-architect OR frontend-developer)
  - Verification: Correct agent selected based on story type (UI → frontend, API → backend)

- [ ] **Step 2:** Implementation subagent invoked
  - Verification: Minimal code written to pass tests
  - Output: Files modified list displayed

- [ ] **Step 3:** Implementation response parsed and tests verified GREEN
  - Verification: Executed {TEST_COMMAND}, all tests pass
  - Output: Green phase confirmed message displayed

- [ ] **Step 4:** AC Verification Checklist updated (Phase 03 items)
  - Verification: All Phase 03 AC items checked off (implementation items)
  - Output: "AC Progress: X/Y items complete" displayed
  - Graceful: Skipped if story doesn't have AC Checklist section

### Success Criteria

- [ ] All tests pass (100% pass rate)
- [ ] Minimal code written (no over-engineering)
- [ ] All context file constraints enforced
- [ ] No anti-patterns introduced
- [ ] Test command executes successfully
- [ ] Ready for refactoring
- [ ] AC Checklist updated (Phase 03 items checked)

### Checkpoint Validation

**IF ANY ITEM UNCHECKED:**
```
❌ PHASE 2 INCOMPLETE - Review missing steps above
⚠️  DO NOT PROCEED TO PHASE 3 until all checkpoints pass

Most commonly missed:
  - Step 4 (Verify tests pass) ← Must confirm GREEN before refactoring
  - Context file enforcement ← backend-architect should enforce, verify

If tests still failing:
  - Re-invoke implementation subagent with failure details
  - Check acceptance criteria (may need clarification)
  - Verify test setup/teardown correct
```

**IF ALL ITEMS CHECKED:**
```
✅ PHASE 2 COMPLETE - Implementation (Green Phase) Done

Tests: All GREEN (passing)
Code: Minimal implementation complete
Context: All constraints enforced
Anti-patterns: None introduced

Ready to refactor and improve code quality.

**Update Progress Tracker:**
Mark "Execute Phase 03" todo as "completed"

**See Also:**
- `tdd-refactor-phase.md` - Phase 04 workflow (code improvement)
- `context-validator` subagent - Fast constraint enforcement
- `backend-architect` / `frontend-developer` subagents - Implementation specialists

Next: Load tdd-refactor-phase.md and execute Phase 04 (Refactor Phase)
```
