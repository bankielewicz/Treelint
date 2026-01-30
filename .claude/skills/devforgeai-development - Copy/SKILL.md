---
name: devforgeai-development
description: Implement features using Test-Driven Development (TDD) while enforcing architectural constraints from context files. Use when implementing user stories, building features, or writing code that must comply with tech-stack.md, source-tree.md, and dependencies.md. Automatically invokes devforgeai-architecture skill if context files are missing.
allowed-tools:
  - Read
  - Write
  - Edit
  - Glob
  - Grep
  - AskUserQuestion
  - Task
  - Bash(git:*)
  - Bash(npm:*)
  - Bash(pytest:*)
  - Bash(dotnet:*)
  - Bash(cargo:*)
  - Bash(mvn:*)
  - Bash(gradle:*)
  - Bash(python:*)
  - Skill
model: claude-opus-4-5-20251101
---

# DevForgeAI Development Skill

Implement user stories using Test-Driven Development while enforcing architectural constraints to prevent technical debt.

Do not skip any phases in the devforgeai-development skill.

---

## ⚠️ EXECUTION MODEL: This Skill Expands Inline

**After invocation, YOU (Claude) execute these instructions phase by phase.**

**When you invoke this skill:**
1. This SKILL.md content is now in your conversation
2. You execute each phase sequentially
3. You display results as you work through phases
4. You complete with success/failure report

**Do NOT:**
- ❌ Wait passively for skill to "return results"
- ❌ Assume skill is executing elsewhere
- ❌ Stop workflow after invocation
- ❌ Stop to ask about token budget (CLAUDE.md: "context window is plenty big")
- ❌ Stop to ask about time constraints (CLAUDE.md: "no time constraints")
- ❌ Stop to ask about scope/approach (execute as documented)
- ❌ Offer execution options (just execute the workflow)

**CRITICAL:** Skill invocation means "execute now" - not "ask if you should execute"

**Proceed to "Parameter Extraction" section below and begin execution.**

---

## ⚠️ IMMEDIATE EXECUTION CHECKPOINT

**YOU HAVE JUST INVOKED THIS SKILL. EXECUTE PHASE 0 NOW.**

**BEFORE PROCEEDING, VERIFY YOU ARE NOT:**

```
Self-Check (Check boxes if TRUE - any checked = VIOLATION):

- [ ] Stopping to ask about token budget (CLAUDE.md says "context window is plenty big")
- [ ] Stopping to ask about time constraints (CLAUDE.md says "no time constraints")
- [ ] Stopping to ask about approach/scope (story already defines approach)
- [ ] Stopping to offer execution options (execute the workflow as documented)
- [ ] Waiting passively for results (skills expand inline, YOU execute)
- [ ] Asking "should I execute this?" (YES - that's why skill was invoked)
```

**Validation Logic:**

```
IF you checked ANY box above:
  You are VIOLATING the skill execution model.

  Display:
  "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  "❌ EXECUTION MODEL VIOLATION DETECTED"
  ""
  "You stopped after skill invocation instead of executing Phase 01."
  ""
  "CLAUDE.md states:"
  "  • 'There are no time constraints'"
  "  • 'Your context window is plenty big'"
  "  • 'Focus on quality'"
  ""
  "You must execute the workflow immediately, not ask permission."
  "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  SELF-CORRECT: Go directly to Parameter Extraction and Phase 01 now.
                Do not ask questions.
                Execute the workflow.

IF all boxes UNCHECKED:
  ✓ You are executing correctly
  ✓ Proceed to Parameter Extraction immediately
```

**Evidence from test session (tmp/output.md):**
- Line 27-28: Skill invoked ✓
- Line 30: "I need to pause here" ✗ VIOLATION
- Lines 43-73: Offered Options A/B/C instead of executing ✗ VIOLATION

**This checkpoint prevents that behavior.**

---

## Parameter Extraction

This skill extracts the story ID from conversation context (loaded story file YAML frontmatter, context markers, or natural language).

**See `references/parameter-extraction.md` for complete extraction algorithm.**

---

## Workflow Execution Checklist

**After parameter extraction, BEFORE Phase 01, create execution tracker:**

**Initialize iteration counter:**
```
iteration_count = 1  # Track TDD cycle iterations (for Phase 06 resumption - RCA-014 fix)
```

TodoWrite(
  todos=[
    {content: "Execute Phase 01: Pre-Flight Validation (10 steps)", status: "pending", activeForm: "Executing Phase 01 Pre-Flight Validation"},
    {content: "Execute Phase 02: Test-First Design (4 steps + Tech Spec Coverage)", status: "pending", activeForm: "Executing Phase 02 Test-First Design"},
    {content: "Execute Phase 03: Implementation (backend-architect + context-validator)", status: "pending", activeForm: "Executing Phase 03 Implementation"},
    {content: "Execute Phase 04: Refactoring (refactoring-specialist + code-reviewer + Light QA)", status: "pending", activeForm: "Executing Phase 04 Refactoring"},
    {content: "Execute Phase 05: Integration Testing (integration-tester)", status: "pending", activeForm: "Executing Phase 05 Integration Testing"},
    {content: "Execute Phase 06: Deferral Challenge (validate incomplete items, immediate resumption if needed)", status: "pending", activeForm: "Executing Phase 06 Deferral Challenge"},
    {content: "Execute Phase 07: Update DoD Checkboxes (mark completed items [x])", status: "pending", activeForm: "Executing Phase 07 DoD Update"},
    {content: "Execute Phase 08: Git Workflow (validate DoD format + commit)", status: "pending", activeForm: "Executing Phase 08 Git Workflow"},
    {content: "Execute Phase 09: Feedback Hook (check-hooks + invoke-hooks)", status: "pending", activeForm: "Executing Phase 09 Feedback Hook"},
    {content: "Execute Phase 10: Result Interpretation (dev-result-interpreter)", status: "pending", activeForm: "Executing Phase 10 Result Interpretation"}
  ]
)

**Usage During Workflow:**
- Mark phase "in_progress" when starting each phase
- Mark phase "completed" when checkpoint validation passes
- Update user on progress as phases complete
- User can see visual progress through TDD cycle
- Self-monitoring: If Phase 04 todo still "pending" when trying Phase 08, something is wrong

**Benefits:**
- Visual progress tracking for user
- Forces Claude to consciously mark phases complete
- Self-monitoring mechanism (detects skipped phases)
- Audit trail of workflow execution

**TodoWrite purpose:** User-facing progress visualization (advisory)
**Enforcement:** Validation checkpoints at phase transitions (mandatory)
**Note:** TodoWrite does not provide read API - checkpoints verify actual execution

---

## Purpose

Implement features following strict TDD workflow (Red → Green → Refactor) while enforcing all 6 context file constraints.

**Context files are THE LAW:** tech-stack.md, source-tree.md, dependencies.md, coding-standards.md, architecture-constraints.md, anti-patterns.md

**If ambiguous or conflicts detected → HALT and use AskUserQuestion**

**See `references/ambiguity-protocol.md` for resolution procedures.**

---

## When to Use This Skill

**Prerequisites:** Git repo (recommended), context files (6), story file

**Git modes:** Full workflow (with Git) OR file-based tracking (without Git) - auto-detects

**Invoked by:** `/dev [STORY-ID]` command, devforgeai-orchestration skill, manual skill call

---

## Pre-Flight Validation (Phase 01)

**⚠️ EXECUTION STARTS HERE - You are now executing the skill's workflow.**

**This is Phase 01. Execute these steps now:**

11-step validation before TDD begins:

1. Validate Git status (git-validator subagent)
1.5. **User consent for git operations (if uncommitted changes >10)** (RCA-008)
1.6. **Stash warning and confirmation (if user chooses to stash)** (RCA-008)
1.7. **Check for existing plan file** (STORY-127) ← NEW
   - Glob(".claude/plans/*.md") to list all plan files
   - Grep for story ID pattern (e.g., "STORY-127") with word boundaries
   - If match found, offer to resume via AskUserQuestion
   - Prioritize files with story ID in filename over random-named files
   - If resumed, HALT (don't proceed with new plan creation)
2. **Git Worktree Auto-Management** (git-worktree-manager subagent) ← NEW (STORY-091)
3. Adapt workflow (Git vs file-based)
4. File-based tracking setup (if no Git)
5. Validate 6 context files exist
6. Load story specification
7. Validate spec vs context conflicts
8. Detect tech stack (tech-stack-detector subagent)
9. Detect QA failures (recovery mode)
9.5. Load structured gap data (if gaps.json exists)

**See `references/preflight-validation.md` for complete workflow.**

---

### Phase 01 Validation Checkpoint (HALT IF FAILED)

**Before proceeding to Phase 02 or Remediation Mode, verify Phase 01 completed:**

CHECK CONVERSATION HISTORY FOR EVIDENCE:

- [ ] Step a.: git-validator subagent invoked?
      Search for: Task(subagent_type="git-validator")
      Found? YES → Check box | NO → Leave unchecked

- [ ] Step d.: Context files validated (6 files)?
      Search for: Read(file_path="devforgeai/specs/context/tech-stack.md")
      Found? YES → Check box | NO → Leave unchecked

- [ ] Step e.: Story specification loaded?
      Search for: Read with story file path
      Found? YES → Check box | NO → Leave unchecked

- [ ] Step g.: tech-stack-detector subagent invoked?
      Search for: Task(subagent_type="tech-stack-detector")
      Found? YES → Check box | NO → Leave unchecked

IF any checkbox UNCHECKED:
  Display:
  "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  "❌ PHASE 0 INCOMPLETE - Pre-flight validation not executed"
  ""
  FOR each unchecked item:
    Display: "  ✗ {item description}"
  ""
  Display: "Missing validation prevents safe development. HALT."
  Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  HALT workflow (do not proceed to Phase 02 or Remediation Mode)

IF all checkboxes CHECKED:
  Display:
  "✓ Phase 01 Validation Passed - All pre-flight checks completed"
  "  ✓ Git repository validated"
  "  ✓ Context files loaded"
  "  ✓ Story specification loaded"
  "  ✓ Tech stack detected"
  ""
  Display: "Proceeding to Phase 02 (or Remediation Mode if QA gaps detected)..."

  Proceed to next phase

**Purpose:** Prevents skipping git validation, context file loading, and tech stack detection (RCA: STORY-080 skipped Phase 01 completely)

---

## Remediation Mode Decision Point (After Phase 01)

**CRITICAL:** After Phase 01 completes, check `$REMEDIATION_MODE` flag set by Step h.1..

```
IF $REMEDIATION_MODE == true:
    # gaps.json exists from previous QA failure
    # Execute targeted remediation workflow instead of full TDD

    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: "  🔧 REMEDIATION MODE ACTIVE"
    Display: "  Targeted workflow to fix QA gaps"
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    # Load and execute remediation workflow
    Read(file_path=".claude/skills/devforgeai-development/references/qa-remediation-workflow.md")

    # Execute phases 1R, 2R, 3R, 4R, 4.5R, 5R from remediation workflow
    # These replace normal Phases 1-5 with targeted versions

    SKIP: Normal TDD Phases 1-5 below
    GOTO: Phase 09 (Feedback Hook) after remediation complete

ELSE:
    # Normal TDD workflow
    Proceed with Phase 02 below
```

**What Remediation Mode Does:**
- Phase 02R: Generate tests for `$QA_COVERAGE_GAPS` files ONLY (not full story)
- Phase 03R: Implement code for gap files ONLY
- Phase 04R: Fix `$QA_ANTIPATTERN_GAPS` violations ONLY
- Phase 05R: Verify coverage gaps are closed
- Phase 06R: Resolve `$QA_DEFERRAL_GAPS` issues
- Phase 08R: Commit remediation

**Reference:** `qa-remediation-workflow.md`

---

## TDD Workflow (6 Phases)

### Phase 02: Test-First Design
Write failing tests from AC → test-automator subagent → Tests RED → **Update AC Checklist (test items) ✓ MANDATORY**
**Reference:** `tdd-red-phase.md`
**AC Updates:** Test count, test coverage, test file creation items

---

### Phase 02 Validation Checkpoint (HALT IF FAILED)

**Before proceeding to Phase 03, verify Phase 02 completed:**

CHECK CONVERSATION HISTORY FOR EVIDENCE:

- [ ] Steps 1-3: test-automator subagent invoked?
      Search for: Task(subagent_type="test-automator")
      Found? YES → Check box | NO → Leave unchecked

- [ ] Step 4: Tech Spec Coverage Validation completed?
      Search for: coverage validation OR tech spec verification
      Found? YES → Check box | NO → Leave unchecked

- [ ] AC Checklist (test items) updated? ✓ MANDATORY
      Search for: Edit with AC Checklist test items marked [x]
      Found? YES → Check box | NO → Leave unchecked

IF any checkbox UNCHECKED:
  Display:
  "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  "❌ PHASE 1 INCOMPLETE - Test generation not verified"
  ""
  FOR each unchecked item:
    Display: "  ✗ {item description}"
  ""
  Display: "HALT - Cannot proceed to Phase 03 until Phase 02 complete"
  Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  HALT workflow

IF all checkboxes CHECKED:
  Display:
  "✓ Phase 02 Validation Passed - Tests RED and documented"
  "  ✓ test-automator invoked"
  "  ✓ Tech spec coverage validated"
  "  ✓ AC Checklist updated"
  ""
  Display: "Proceeding to Phase 03..."

  Proceed to Phase 03

**Purpose:** Ensures tests are generated before implementation begins (TDD Red phase complete)

---

### Phase 03: Implementation
Minimal code to pass tests → backend-architect/frontend-developer → Tests GREEN → **Update AC Checklist (implementation items) ✓ MANDATORY**
**Reference:** `tdd-green-phase.md`
**AC Updates:** Code implementation, business logic location, size metrics items

### Phase 04: Refactoring
Improve quality, keep tests green → refactoring-specialist, code-reviewer, Light QA → Code improved → **Update AC Checklist (quality items) ✓ MANDATORY**
**Reference:** `tdd-refactor-phase.md`
**Steps:** 1-4 Refactoring + code review, 5 Light QA validation [MANDATORY], 6 AC Checklist update
**AC Updates:** Code quality, pattern compliance, review findings items

### Phase 05: Integration & Validation
Cross-component testing, coverage validation → integration-tester → Thresholds met → **Update AC Checklist (integration items) ✓ MANDATORY**
**Reference:** `integration-testing.md`
**AC Updates:** Integration tests, performance, coverage threshold items

---

### Phase 05 Validation Checkpoint (HALT IF FAILED)

**Before proceeding to Phase 06, verify Phase 05 completed:**

CHECK CONVERSATION HISTORY FOR EVIDENCE:

- [ ] Phase 05.0: Anti-Gaming Validation PASSED? [NEW - BLOCKING - RUN FIRST]
      Search for: integration-tester response with "✓ Anti-gaming validation passed"
      OR: gaming_scan.status == "PASS"
      Found? YES → Check box | NO → Leave unchecked
      IF FAIL: HALT - Test gaming detected, coverage scores INVALID

- [ ] Step 1: integration-tester subagent invoked?
      Search for: Task(subagent_type="integration-tester")
      Found? YES → Check box | NO → Leave unchecked

- [ ] Coverage thresholds validated (95%/85%/80%)?
      Search for: coverage results OR threshold validation
      Found? YES → Check box | NO → Leave unchecked

- [ ] AC Checklist (integration items) updated? ✓ MANDATORY
      Search for: Edit with AC Checklist integration items marked [x]
      Found? YES → Check box | NO → Leave unchecked

IF any checkbox UNCHECKED:
  Display:
  "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  "❌ PHASE 4 INCOMPLETE - Integration testing not verified"
  ""
  FOR each unchecked item:
    Display: "  ✗ {item description}"
  ""
  IF anti-gaming validation FAILED:
    Display: "  🚨 TEST GAMING DETECTED - Cannot calculate authentic coverage"
    Display: "  Fix: Remove skip decorators, add assertions, reduce mocking, remove TODO placeholders"
  ""
  Display: "HALT - Cannot proceed to Phase 06 until Phase 05 complete"
  Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  HALT workflow

IF all checkboxes CHECKED:
  Display:
  "✓ Phase 05 Validation Passed - Integration testing complete"
  "  ✓ Anti-gaming validation PASSED"
  "  ✓ integration-tester invoked"
  "  ✓ Coverage thresholds met"
  "  ✓ AC Checklist updated"
  ""
  Display: "Proceeding to Phase 06..."

  Proceed to Phase 06

**Purpose:** Ensures anti-gaming validation and integration testing complete before deferral checkpoint

---

### Phase 06: Deferral Challenge Checkpoint (NEW - RCA-006)
Challenge ALL deferrals (pre-existing + new) → deferral-validator → User approval required → **Update AC Checklist (deferral items) ✓ MANDATORY**
**Reference:** `phase-06-deferral-challenge.md`
**Purpose:** Prevent autonomous deferrals, enforce "Attempt First, Defer Only If Blocked" pattern
**AC Updates:** Deferral validation, follow-up story creation items

---

### Phase 06 Validation Checkpoint (HALT IF FAILED)

**Before proceeding to Phase 07, verify Phase 06 completed:**

CHECK CONVERSATION HISTORY FOR EVIDENCE:

- [ ] DoD reviewed for incomplete items?
      Search for: DoD review OR deferral detection
      Found? YES → Check box | NO → Leave unchecked

- [ ] IF deferrals exist: deferral-validator subagent invoked?
      Search for: Task(subagent_type="deferral-validator") OR "No deferrals"
      Found OR no deferrals? YES → Check box | NO → Leave unchecked

- [ ] Step 6: AskUserQuestion invoked for EVERY deferral? [ENFORCED]
      Search for: AskUserQuestion with deferral decision options
      Options MUST include "HALT and implement NOW" as FIRST option
      Found for EACH deferral OR no deferrals? YES → Check box | NO → Leave unchecked
      IF SKIPPED: HALT - Autonomous deferral approval is FORBIDDEN

- [ ] Step c.1.: User approval timestamp recorded? [NEW - MANDATORY]
      Search for: "User approved: YYYY-MM-DD" timestamp in story file
      FOR EACH kept deferral, timestamp MUST exist
      Found for ALL OR no deferrals kept? YES → Check box | NO → Leave unchecked
      IF MISSING: HALT - Deferrals without explicit user approval are INVALID

- [ ] AC Checklist (deferral items) updated? ✓ MANDATORY
      Search for: Edit with AC Checklist deferral items marked [x]
      Found? YES → Check box | NO → Leave unchecked

IF any checkbox UNCHECKED:
  Display:
  "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  "❌ PHASE 4.5 INCOMPLETE - Deferral validation not verified"
  ""
  FOR each unchecked item:
    Display: "  ✗ {item description}"
  ""
  IF Step 6 (AskUserQuestion) SKIPPED:
    Display: "  🚨 AUTONOMOUS DEFERRAL DETECTED"
    Display: "  Claude MUST use AskUserQuestion for EVERY deferral"
    Display: "  First option MUST be 'HALT and implement NOW'"
  ""
  IF Step c.1. (timestamp) MISSING:
    Display: "  🚨 DEFERRAL WITHOUT USER APPROVAL"
    Display: "  Every kept deferral MUST have 'User approved: timestamp'"
    Display: "  Deferrals without timestamps are INVALID"
  ""
  Display: "HALT - Cannot proceed to Bridge until Phase 06 complete"
  Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  HALT workflow

IF all checkboxes CHECKED:
  Display:
  "✓ Phase 06 Validation Passed - Deferrals validated with user approval"
  "  ✓ DoD reviewed"
  "  ✓ Deferrals validated (or none exist)"
  "  ✓ AskUserQuestion invoked for every deferral"
  "  ✓ User approval timestamps recorded"
  "  ✓ AC Checklist updated"
  ""
  Display: "Proceeding to Phase 07..."

  Proceed to Bridge

**Purpose:** Ensures deferrals have EXPLICIT user approval (not autonomous) before DoD update. Step c.1. is defense-in-depth against auto-approval.

---

### Phase 07: DoD Update Workflow ✓ MANDATORY (NEW - RCA-009, Enforced - RCA-010)
Update DoD format for git commit → Validate format → Prepare for Phase 08
**Reference:** `dod-update-workflow.md`
**Purpose:** Ensure DoD items formatted correctly (flat list in Implementation Notes, no ### subsections)
**CRITICAL:** Execute AFTER Phase 06, BEFORE Phase 08 - git commit will FAIL if skipped
**Note (RCA-014):** Phase 06-R removed - resumption now happens immediately in Phase 06 Step 7

**MANDATORY: Load and Execute Reference Document**

```
Read(file_path=".claude/skills/devforgeai-development/references/dod-update-workflow.md")
```

**After reading, execute ALL steps in dod-update-workflow.md:**
1. Step 1: Mark completed items [x] in Definition of Done section
2. Step 2: Add DoD items to Implementation Notes (FLAT LIST - no ### subsections)
3. Step 3: Validate format: `devforgeai-validate validate-dod ${STORY_FILE}`
4. Step 4: Update Workflow Status section
5. Step 5: Final validation (exit code 0 required)

**DO NOT PROCEED TO PHASE 08 UNTIL dod-update-workflow.md IS FULLY EXECUTED**

**Pre-Check: Implementation Notes Section [MANDATORY]**

Before executing DoD update workflow, verify story has Implementation Notes section:

```
Grep(pattern="^## Implementation Notes", path="${STORY_FILE}")

IF NOT found:
    Display: "❌ Story file missing ## Implementation Notes section"
    Display: ""
    Display: "Required section structure:"
    Display: "  ## Implementation Notes"
    Display: "  **Developer:** [name]"
    Display: "  **Implemented:** [date]"
    Display: "  - [x] DoD items... (flat list, no ### subsections)"
    Display: "  ### TDD Workflow Summary (optional)"
    Display: "  ### Files Created/Modified (optional)"
    Display: "  ### Test Results (optional)"
    Display: ""
    Display: "Adding Implementation Notes section..."

    # Auto-create section before Workflow Status
    Edit(
        file_path="${STORY_FILE}",
        old_string="## Workflow Status",
        new_string="## Implementation Notes\n\n**Developer:** DevForgeAI AI Agent\n**Implemented:** {current_date}\n\n## Workflow Status"
    )

    Display: "✓ Implementation Notes section created"
```

---

### Bridge Validation Checkpoint (HALT IF FAILED)

**Before proceeding to Phase 08 (Git Commit), verify Bridge completed:**

CHECK CONVERSATION HISTORY FOR EVIDENCE:

- [ ] DoD items marked [x] in story file?
      Search for: Edit with DoD items marked [x]
      Found? YES → Check box | NO → Leave unchecked

- [ ] Implementation Notes flat list added?
      Search for: Edit with "Definition of Done - Completed Items"
      Found? YES → Check box | NO → Leave unchecked

- [ ] DoD format validated?
      Search for: devforgeai-validate validate-dod OR Bash with validate-dod
      Found? YES → Check box | NO → Leave unchecked

IF any checkbox UNCHECKED:
  Display:
  "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  "❌ BRIDGE INCOMPLETE - DoD not updated before commit"
  ""
  FOR each unchecked item:
    Display: "  ✗ {item description}"
  ""
  Display: "Git commit will FAIL without DoD validation. HALT."
  Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  HALT workflow (do not execute git commit)

IF all checkboxes CHECKED:
  Display:
  "✓ Bridge Validation Passed - DoD validated"
  "  ✓ DoD items marked complete"
  "  ✓ Implementation Notes added"
  "  ✓ Format validation passed"
  ""
  Display: "Proceeding to Phase 08 (Git Commit)..."

  Proceed to Phase 08

**Purpose:** Prevents git commit without proper DoD documentation (RCA: STORY-080 skipped Bridge)

---

### Phase 08: Git Workflow & DoD Validation
Budget enforcement → Handle incomplete items → **Lock acquisition** → Git commit → **Lock release** → Story complete → **Update AC Checklist (deployment items) ✓ MANDATORY**
**References:** `dod-update-workflow.md` (pre-requisite), `deferral-budget-enforcement.md`, `git-workflow-conventions.md`, **`lock-file-coordination.md` (NEW - STORY-096)**, `dod-validation-checkpoint.md`, `ac-checklist-update-workflow.md`
**Steps:** Pre-req: DoD format validated, 1.6 Budget enforcement, 1.7 Handle new incomplete items, **1.8 Lock acquisition (STORY-096)**, 2.0+ Git commit, **2.1 Lock release (STORY-096)**, 2.2+ AC Checklist final update
**AC Updates:** Git commit, status update, backward compatibility items

**See `references/tdd-patterns.md` for comprehensive TDD guidance across all phases.**

---

### Phase 08 Validation Checkpoint (HALT IF FAILED)

**Before proceeding to Phase 09, verify Phase 08 completed:**

CHECK CONVERSATION HISTORY FOR EVIDENCE:

- [ ] Git commit succeeded?
      Search for: git commit output showing success OR commit hash
      Found? YES → Check box | NO → Leave unchecked

- [ ] Story file included in commit?
      Search for: git add with story file path
      Found? YES → Check box | NO → Leave unchecked

- [ ] AC Checklist (deployment items) updated? ✓ MANDATORY
      Search for: Edit with AC Checklist deployment items marked [x]
      Found? YES → Check box | NO → Leave unchecked
      **See `references/dod-update-workflow.md` for comprehensive knowledge of the DOD workflow.**

IF any checkbox UNCHECKED:
  Display:
  "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  "❌ PHASE 5 INCOMPLETE - Git commit not verified"
  ""
  FOR each unchecked item:
    Display: "  ✗ {item description}"
  ""
  Display: "HALT - Cannot proceed to Phase 09 until Phase 08 complete"
  Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  HALT workflow

IF all checkboxes CHECKED:
  Display:
  "✓ Phase 08 Validation Passed - Code committed"
  "  ✓ Git commit successful"
  "  ✓ Story file included"
  "  ✓ AC Checklist updated"
  ""
  Display: "Proceeding to Phase 09..."

  Proceed to Phase 09

**Purpose:** Ensures git commit includes story file and documentation

---

## Complete Workflow Execution Map

**Visual guide showing all mandatory steps and common skip points:**

```
START
  ↓
Phase 01: Pre-Flight (preflight-validation.md)
  ├─ Step a.: git-validator ✓ MANDATORY
  ├─ Step a.1.: User consent (RCA-008) ✓ MANDATORY IF uncommitted > 10
  ├─ Step d.: Validate 6 context files ✓ MANDATORY
  ├─ Step g.: tech-stack-detector ✓ MANDATORY
  ├─ Step h.: Detect QA failures ✓ MANDATORY
  └─ Step h.1.: Load gaps.json ✓ MANDATORY IF QA failed
  ↓
┌─── DECISION: Check $REMEDIATION_MODE ───┐
│                                          │
│  IF true:                               │
│    ↓                                    │
│  REMEDIATION WORKFLOW (qa-remediation-workflow.md)
│    ├─ Phase 02R: Targeted test gen       │
│    ├─ Phase 03R: Targeted implementation │
│    ├─ Phase 04R: Anti-pattern fixes      │
│    ├─ Phase 05R: Coverage verification   │
│    ├─ Phase 06R: Deferral resolution   │
│    └─ Phase 08R: Commit remediation      │
│    ↓                                    │
│    GOTO Phase 09 (Feedback Hook)         │
│                                          │
│  ELSE:                                   │
│    ↓                                    │
│  NORMAL TDD WORKFLOW (below)            │
└──────────────────────────────────────────┘
  ↓
Phase 02: Red (tdd-red-phase.md)
  ├─ Step 1-3: Generate failing tests ✓ MANDATORY
  └─ Step 4: Tech Spec Coverage Validation ✓ MANDATORY ← OFTEN MISSED
  ↓
Phase 03: Green (tdd-green-phase.md)
  ├─ Step 1-2: backend-architect OR frontend-developer ✓ MANDATORY
  └─ Step 3: context-validator ✓ MANDATORY ← OFTEN MISSED
  ↓
Phase 04: Refactor (tdd-refactor-phase.md + refactoring-patterns.md)
  ├─ Step 1-2: refactoring-specialist ✓ MANDATORY
  ├─ Step 3: code-reviewer ✓ MANDATORY
  ├─ **Step 4: Anti-Gaming Validation ✓ MANDATORY [NEW]** ← BLOCKS IF GAMING DETECTED
  │   └─ HALT if: skip decorators, empty tests, excessive mocking (>2x) detected
  └─ Step 5: Light QA (devforgeai-qa --mode=light) ✓ MANDATORY ← OFTEN MISSED
  ↓
Phase 05: Integration (integration-testing.md)
  ├─ **Phase 05.0: Anti-Gaming Validation ✓ MANDATORY [NEW]** ← RUN FIRST, BLOCKS COVERAGE
  │   └─ HALT if: gaming patterns detected BEFORE coverage calculation
  └─ Step 1: integration-tester ✓ MANDATORY
  ↓
Phase 06: Deferral Challenge (phase-06-deferral-challenge.md)
  ├─ Detect deferrals ✓ MANDATORY
  ├─ deferral-validator ✓ MANDATORY IF deferrals exist
  ├─ **Step 6: AskUserQuestion for EVERY deferral ✓ MANDATORY [ENFORCED]**
  ├─ **Step c.1.: Mandatory HALT Verification ✓ MANDATORY [NEW]** ← BLOCKS IF AUTO-APPROVED
  │   └─ HALT if: ANY deferral lacks explicit user approval timestamp
  └─ User approval timestamp recorded ✓ MANDATORY IF deferrals kept
  ↓
Phase 07: DoD Update (dod-update-workflow.md ← NEW)
  ├─ Mark DoD items [x] ✓ MANDATORY
  ├─ Add items to Implementation Notes (FLAT LIST) ✓ MANDATORY
  ├─ Validate format: devforgeai-validate validate-dod ✓ MANDATORY
  └─ Update Workflow Status ✓ MANDATORY
  ↓
Phase 08: Git Workflow (git-workflow-conventions.md)
  ├─ Budget enforcement ✓ MANDATORY
  ├─ Handle new incomplete items ✓ MANDATORY
  └─ Git commit (validator passes) ✓ MANDATORY
  ↓
Phase 09: Feedback Hook
  ├─ check-hooks ✓ MANDATORY
  └─ invoke-hooks ✓ MANDATORY IF enabled
  ↓
END (Story Status = "Dev Complete")
```

**Legend:**
- ✓ MANDATORY = Must execute, no exceptions
- ✓ MANDATORY IF = Conditional execution based on state
- ← OFTEN MISSED = Common skip points (extra attention needed)

**Purpose:** Visual representation of complete TDD workflow helps prevent phase skipping and highlights critical validation steps.

---

### Phase 09: Feedback Hook Integration

**Purpose:** Invoke feedback hooks if configured for retrospective insights
**Execution:** After Phase 08 (Git commit) completes
**Reference:** See STORY-023 implementation notes

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 09/9: Feedback Hook (89% → 100% complete)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Display this indicator at the start of Phase 09 execution.**

**Steps:**
1. Check hooks configuration: `devforgeai-validate check-hooks --operation=dev --status=success`
2. Invoke hooks if enabled: `devforgeai-validate invoke-hooks --operation=dev --story=$STORY_ID`
3. Non-blocking: Hook failures don't prevent workflow completion

---

### Phase 10: Result Interpretation (NEW - STORY-051)

**Purpose:** Generate user-facing display template and structured result summary
**Execution:** After Phase 09 (Feedback Hook) completes, before returning to /dev command
**Reference:** dev-result-interpreter uses `references/dev-result-formatting-guide.md`

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 10/9: Result Interpretation (95% → 100% complete)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Display this indicator at the start of Phase 10 execution.**

**Workflow:**

**Step a.: Invoke dev-result-interpreter Subagent**

```
Task(
  subagent_type="dev-result-interpreter",
  description="Interpret dev workflow results for {STORY_ID}",
  prompt="""
Interpret development workflow results for story {STORY_ID}.

Story file: {STORY_FILE_PATH}

Task:
1. Read story file and extract:
   - Current status (from YAML frontmatter)
   - TDD phases completed (from Status History or Implementation Notes)
   - Test results (passing count, coverage %)
   - DoD completion status (from Implementation Notes)
   - Deferred items (from Implementation Notes)

2. Determine overall result:
   - SUCCESS: status="Dev Complete", all tests passing
   - INCOMPLETE: status="In Development", some work remaining
   - FAILURE: workflow error or blocking issue

3. Generate display template appropriate for result type

4. Provide next step recommendations based on story state

Return structured JSON with:
- status: "success|incomplete|failure"
- display.template: "..." (formatted display text)
- display.next_steps: [...] (actionable recommendations)
- story_status: "..." (current story status)
- tdd_phases_completed: [...] (phases finished)
- workflow_summary: "..." (brief summary)
"""
)
```

**Step b.: Receive Structured Result**

```
# Subagent returns JSON:
result = {
  "status": "success",
  "display": {
    "template": "╔═══════════...║  DEVELOPMENT COMPLETE ✅  ║...",
    "next_steps": [
      "Run QA validation: /qa {STORY_ID}",
      "Or run full orchestration: /orchestrate {STORY_ID}",
      "Review implementation: Read story file Implementation Notes"
    ]
  },
  "story_status": "Dev Complete",
  "tdd_phases_completed": ["Phase 01", "Phase 02", "Phase 03", "Phase 04", "Phase 05", "Phase 08", "Phase 09"],
  "workflow_summary": "TDD cycle complete, 165/168 tests passing (98.2%)"
}
```

**Step c.: Return Result to Command**

```
# Skill returns the result object to /dev command
# Command will display result.display.template
# No further processing needed in skill

Display: ""
Display: "Phase 10 complete - Result interpretation ready"
Display: ""

RETURN result to command
```

---

### Phase 10 Validation Checkpoint (HALT IF FAILED)

**Before returning result to /dev command, verify Phase 10 completed:**

```
CHECK CONVERSATION HISTORY FOR EVIDENCE:

- [ ] Step a.: dev-result-interpreter invoked?
      Search for: Task(subagent_type="dev-result-interpreter")
      Found? YES → Check box | NO → Leave unchecked

- [ ] Step c.: Structured result returned to command?
      Search for: RETURN result to command statement
      Found? YES → Check box | NO → Leave unchecked
```

**Validation Logic:**

```
IF any checkbox UNCHECKED:
  Display:
  "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  "❌ PHASE 7 INCOMPLETE - Missing mandatory steps:"
  ""
  FOR each unchecked item:
    Display: "  ✗ {item description}"
  ""
  Display: "HALT - Cannot complete workflow without result interpretation"
  Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  HALT workflow (do not return to /dev command)

IF all checkboxes CHECKED:
  Display:
  "✓ Phase 10 Validation Passed - Result interpretation complete"
  "  ✓ dev-result-interpreter invoked"
  "  ✓ Structured result prepared"
  ""
  Display: "Returning result to /dev command..."

  RETURN result to command
```

**Purpose:** This checkpoint ensures dev-result-interpreter subagent is always invoked to generate proper result display template (STORY-051 requirement).

---

**Integration:**
- dev-result-interpreter operates in isolated context (8K tokens max)
- Reference guide (dev-result-formatting-guide.md) provides framework constraints
- Subagent generates templates matching original /dev output format (backward compatibility)
- No business logic in command (follows lean orchestration pattern)

**Benefits:**
- 184 lines extracted from /dev command (Phase 04 + Phase 05 verification/reporting)
- Token efficiency: Display generation in isolated subagent context
- Maintainability: Single source of truth for display templates in subagent
- Reusability: Pattern applicable to other development commands

---

## QA Deferral Recovery

Triggered when QA fails due to deferrals. Phase 01 Step h. detects, then 3-step resolution workflow executes.

**See `references/qa-deferral-recovery.md` for complete procedure.**

---

## Integration Points

**From:** devforgeai-story-creation (story+AC), devforgeai-architecture (context files)
**To:** devforgeai-qa (validation), devforgeai-release (deployment)
**Auto-invokes:** devforgeai-architecture (if missing), devforgeai-qa (light mode), devforgeai-story-creation (deferrals)

---

## Subagent Coordination

**Subagent Invocation Sequences (Execute in Order):**

### Phase 01: Pre-Flight Validation

1. **git-validator** (Git availability check) [MANDATORY]
   - Purpose: Detect Git status, provide workflow strategy (Git vs file-based)
   - Token cost: ~5K (isolated)
   - Returns: Git status, recommended workflow
   - Success: Git available OR file-based strategy confirmed

2. **git-worktree-manager** (Worktree auto-management) [CONDITIONAL - IF GIT AVAILABLE]
   - Purpose: Create/manage Git worktrees for parallel story development (STORY-091)
   - Token cost: ~2K (isolated)
   - Returns: Worktree status, actions needed, idle detection results
   - Success: Worktree created/resumed OR user chose to skip

3. **tech-stack-detector** (Technology detection) [MANDATORY AFTER CONTEXT FILES VALIDATED]
   - Purpose: Auto-detect project technologies, validate against tech-stack.md
   - Token cost: ~10K (isolated)
   - Returns: Detected tech stack, validation results
   - HALT if tech-stack.md conflicts detected

**Sequence:** git-validator → git-worktree-manager → tech-stack-detector (sequential)

---

### Phase 02: Test-First Design

1. **test-automator** (Test generation) [MANDATORY]
   - Purpose: Generate failing tests from acceptance criteria
   - Token cost: ~50K (isolated)
   - Returns: Test files, test command, coverage analysis
   - Success: All tests RED (failing as expected)

**Note:** Phase 02 also includes Step 4 (Tech Spec Coverage Validation) - see `tdd-red-phase.md` for user approval workflow

---

### Phase 03: Implementation

1. **backend-architect OR frontend-developer** (Implementation) [MANDATORY - CHOOSE ONE]
   - Purpose: Write minimal code to pass tests
   - Token cost: ~50K (isolated)
   - Returns: Implementation code, test results
   - Success: All tests GREEN

2. **context-validator** (Fast constraint validation) [MANDATORY AFTER STEP 1]
   - Purpose: Validate against 6 context files (tech-stack, source-tree, dependencies, coding-standards, architecture-constraints, anti-patterns)
   - Token cost: ~5K (isolated)
   - Returns: Validation report
   - HALT if violations detected

**Sequence:** (backend-architect OR frontend-developer) → context-validator (sequential)

---

### Phase 03 Validation Checkpoint (HALT IF FAILED)

**Before proceeding to Phase 04, verify ALL Phase 03 mandatory steps completed:**

```
CHECK CONVERSATION HISTORY FOR EVIDENCE:

- [ ] Step 1-2: backend-architect OR frontend-developer invoked?
      Search for: Task(subagent_type="backend-architect") OR Task(subagent_type="frontend-developer")
      Found? YES → Check box | NO → Leave unchecked

- [ ] Step 3: context-validator invoked?
      Search for: Task(subagent_type="context-validator")
      Found? YES → Check box | NO → Leave unchecked
```

**Validation Logic:**

```
IF any checkbox UNCHECKED:
  Display:
  "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  "❌ PHASE 2 INCOMPLETE - Missing mandatory steps:"
  ""
  FOR each unchecked item:
    Display: "  ✗ {item description}"
  ""
  Display: "HALT - Cannot proceed to Phase 04 until Phase 03 complete"
  Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  HALT workflow (do not execute Phase 04)

IF all checkboxes CHECKED:
  Display:
  "✓ Phase 03 Validation Passed - All mandatory steps completed"
  "  ✓ backend-architect OR frontend-developer invoked"
  "  ✓ context-validator invoked"
  ""
  Display: "Proceeding to Phase 04..."

  Proceed to Phase 04
```

**Purpose:** This checkpoint prevents Claude from skipping mandatory subagent invocations by requiring explicit verification before phase progression.

---

### Phase 04: Refactoring

1. **refactoring-specialist** (Code improvement) [MANDATORY]
   - Purpose: Apply refactoring patterns, remove code smells
   - Token cost: ~40K (isolated)
   - Returns: Refactored code
   - Success: Tests remain GREEN, quality improved

2. **code-reviewer** (Code review) [MANDATORY AFTER STEP 1]
   - Purpose: Review for quality, security, maintainability, standards compliance
   - Token cost: ~30K (isolated)
   - Returns: Review feedback
   - Success: No critical issues

3. **devforgeai-qa (light mode)** (Intermediate quality gate) [MANDATORY AFTER STEP 2]
   - Purpose: Build validation, test execution, quick anti-pattern scan
   - Token cost: ~10K (isolated)
   - Returns: Light QA report
   - HALT if validation fails

**Sequence:** refactoring-specialist → code-reviewer → devforgeai-qa (light) (sequential)

---

### Phase 04 Validation Checkpoint (HALT IF FAILED)

**Before proceeding to Phase 05, verify ALL Phase 04 mandatory steps completed:**

```
CHECK CONVERSATION HISTORY FOR EVIDENCE:

- [ ] Step 1-2: refactoring-specialist invoked?
      Search for: Task(subagent_type="refactoring-specialist")
      Found? YES → Check box | NO → Leave unchecked

- [ ] Step 3: code-reviewer invoked?
      Search for: Task(subagent_type="code-reviewer")
      Found? YES → Check box | NO → Leave unchecked

- [ ] Step 4: Anti-Gaming Validation PASSED? [NEW - BLOCKING]
      Search for: code-reviewer response with gaming_validation.status == "PASS"
      OR: "✓ Anti-gaming validation passed"
      Found? YES → Check box | NO → Leave unchecked
      IF FAIL: HALT - Test gaming detected, fix violations before proceeding

- [ ] Step 5: Light QA (devforgeai-qa --mode=light) executed?
      Search for: Skill(skill="devforgeai-qa") with **Validation mode:** light
      Found? YES → Check box | NO → Leave unchecked
```

**Validation Logic:**

```
IF any checkbox UNCHECKED:
  Display:
  "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  "❌ PHASE 3 INCOMPLETE - Missing mandatory steps:"
  ""
  FOR each unchecked item:
    Display: "  ✗ {item description}"
  ""
  IF anti-gaming validation FAILED:
    Display: "  🚨 TEST GAMING DETECTED - Coverage scores are invalid"
    Display: "  Fix: Remove skip decorators, add assertions to empty tests, reduce mocking"
  ""
  Display: "HALT - Cannot proceed to Phase 05 until Phase 04 complete"
  Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  HALT workflow (do not execute Phase 05)

IF all checkboxes CHECKED:
  Display:
  "✓ Phase 04 Validation Passed - All mandatory steps completed"
  "  ✓ refactoring-specialist invoked"
  "  ✓ code-reviewer invoked"
  "  ✓ Anti-gaming validation PASSED"
  "  ✓ Light QA executed"
  ""
  Display: "Proceeding to Phase 05..."

  Proceed to Phase 05
```

**Purpose:** This checkpoint prevents Claude from skipping refactoring-specialist, anti-gaming validation, and Light QA (marked "← OFTEN MISSED") by requiring explicit verification before phase progression.

---

### Phase 05: Integration & Validation

1. **integration-tester** (Cross-component testing) [MANDATORY]
   - Purpose: Validate cross-component interactions, API contracts, integration scenarios
   - Token cost: ~40K (isolated)
   - Returns: Integration test results, coverage report
   - Success: Integration tests pass, coverage thresholds met

---

### Phase 06: Deferral Challenge Checkpoint

1. **deferral-validator** (Blocker validation) [MANDATORY IF DEFERRALS EXIST]
   - Purpose: Validate deferral justifications, detect circular deferrals, check story references
   - Token cost: ~5K (isolated)
   - Returns: Deferral validation report, resolvable vs valid categories
   - Success: All deferrals have user approval OR no deferrals

**Note:** User approval required for EVERY deferred item (zero autonomous deferrals)

---

### Phase 07: DoD Update Workflow

**No subagents** - Direct file operations to update DoD format

**CRITICAL:** Execute dod-update-workflow.md AFTER Phase 06, BEFORE Phase 08

---

### Phase 08: Git Workflow & DoD Validation

**No subagents** - Direct git operations and validation

**Prerequisites:**
- Phase 06 complete (deferrals validated)
- DoD format validated (devforgeai-validate validate-dod passes)
- New incomplete items handled (AskUserQuestion if needed)

---

**See phase-specific reference files for detailed coordination procedures.**

---

## Reference Files

Load these on-demand during workflow execution:

### Core Workflow
- **parameter-extraction.md** (92 lines) - Story ID extraction from conversation
- **preflight-validation.md** (982 lines) - Phase 01: 10-step validation (git, user consent, stash warning, context, tech stack)
- **tdd-red-phase.md** (125 lines) - Phase 02: Test-first design
- **tdd-green-phase.md** (167 lines) - Phase 03: Minimal implementation
- **tdd-refactor-phase.md** (202 lines) - Phase 04: Code improvement
- **integration-testing.md** (189 lines) - Phase 05: Cross-component tests

### Phase 06 & 5 (Deferrals & Git)
- **phase-06-deferral-challenge.md** (~900 lines) - Phase 06: Challenge ALL deferrals + immediate resumption (RCA-006, RCA-014)
- **dod-update-workflow.md** (~400 lines) - Phase 07: DoD format update and validation (RCA-009 Rec 4)
- **deferral-budget-enforcement.md** (290 lines) - Phase 08 Step b.: Budget limits (RCA-006 Phase 03)
- **git-workflow-conventions.md** (~1,300 lines) - Git operations, stash safety protocol (RCA-008), DoD prerequisites, pre-Phase-5 validation (RCA-014)
- **dod-validation-checkpoint.md** (519 lines) - Phase 08 Step c.: Handle new incomplete items
- **~~phase-resumption-workflow.md~~** (~400 lines) - REMOVED (RCA-014 REC-2): Resumption now in Phase 06 Step 7

### Background Execution (STORY-112)
- **background-executor.md** (~200 lines) - Background test execution patterns, thresholds, timeout handling
- **parallel-context-loader.md** (~100 lines) - Parallel Read patterns for 6 context files (83% time savings)
- **task-result-aggregation.md** (~150 lines) - TaskOutput integration and result retrieval

### Supporting Files
- **tdd-patterns.md** (1,013 lines) - Comprehensive TDD guidance (all phases)
- **refactoring-patterns.md** (797 lines) - Code smell detection and fixes
- **story-documentation-pattern.md** (532 lines) - Story update procedures
- **qa-deferral-recovery.md** (218 lines) - QA failure resolution
- **ambiguity-protocol.md** (234 lines) - When to ask user questions

**Total reference content:** ~6,800 lines (loaded progressively as needed - includes RCA-008, RCA-014, STORY-112 safeguards)

---

## Success Criteria

This skill succeeds when:

- [ ] All tests pass (100% pass rate)
- [ ] Coverage meets thresholds (95%/85%/80%)
- [ ] Light QA validation passed
- [ ] No context file violations
- [ ] All AC implemented
- [ ] Code follows coding-standards.md
- [ ] No anti-patterns from anti-patterns.md
- [ ] DoD validation passed (3 layers)
- [ ] Changes committed (or file-tracked)
- [ ] Story status = "Dev Complete"
- [ ] Token usage <85K (isolated context)

**The goal: Zero technical debt from wrong assumptions, fully tested features that comply with architectural decisions.**
