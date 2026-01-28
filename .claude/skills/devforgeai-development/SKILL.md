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

<!-- ═══════════════════════════════════════════════════════════════════════════ -->
<!-- TOP 40-60%: METHODOLOGY & CORE PRINCIPLES                                   -->
<!-- Place methodology first so Claude processes constraints before execution    -->
<!-- ═══════════════════════════════════════════════════════════════════════════ -->

# DevForgeAI Development Skill

Implement user stories using Test-Driven Development while enforcing architectural constraints to prevent technical debt.

**Phase Enforcement Active:** This 10-phase workflow uses progressive phase loading with CLI validation gates.

---

## Purpose

Implement features following strict TDD workflow (Red → Green → Refactor) while enforcing all 6 context file constraints.

**Context files are THE LAW:** tech-stack.md, source-tree.md, dependencies.md, coding-standards.md, architecture-constraints.md, anti-patterns.md

**If ambiguous or conflicts detected → HALT and use AskUserQuestion**

---

## Execution Model: This Skill Expands Inline

**After invocation, YOU (Claude) execute these instructions phase by phase.**

**When you invoke this skill:**
1. This SKILL.md content is now in your conversation
2. You execute each phase sequentially via phase files
3. You display results as you work through phases
4. CLI gates block progression if phases incomplete

**Do NOT:**
- Wait passively for skill to "return results"
- Assume skill is executing elsewhere
- Stop workflow after invocation
- Stop to ask about token budget (CLAUDE.md: "context window is plenty big")
- Stop to ask about time constraints (CLAUDE.md: "no time constraints")
- Stop to ask about scope/approach (execute as documented)
- Offer execution options (just execute the workflow)

**CRITICAL:** Skill invocation means "execute now" - not "ask if you should execute"

**Proceed to Phase State Initialization section below and begin execution.**

---

## Immediate Execution Checkpoint

**YOU HAVE JUST INVOKED THIS SKILL. EXECUTE PHASE STATE INITIALIZATION NOW.**

**BEFORE PROCEEDING, VERIFY YOU ARE NOT:**

```
Self-Check (Check boxes if TRUE - any checked = VIOLATION):

- [ ] Stopping to ask about token budget
- [ ] Stopping to ask about time constraints
- [ ] Stopping to ask about approach/scope
- [ ] Stopping to offer execution options
- [ ] Waiting passively for results
- [ ] Asking "should I execute this?"
```

**IF any box checked:**

```
EXECUTION MODEL VIOLATION DETECTED

You are stopping to ask for permission instead of executing.

Per CLAUDE.md:
- "There are no time constraints"
- "Your context window is plenty big"
- "Focus on quality"

RECOVERY: Go directly to Phase 0 now. Do not ask questions.
```

**Reference:** See CLAUDE.md for complete execution model guidance.

---

## Parameter Extraction

This skill extracts the story ID from conversation context (loaded story file YAML frontmatter, context markers, or natural language).

**See `references/parameter-extraction.md` for complete extraction algorithm.**

---

## Workflow Deviation Protocol

**Purpose:** Require explicit user consent before phase skipping, subagent omission, or out-of-sequence execution. **(Source: RCA-019 REC-4)**

### Deviation Types

| Type | Description | Trigger |
|------|-------------|---------|
| **Phase Skipping** | Skip Phase N to N+2 | Claude considers phase unnecessary |
| **Subagent Omission** | Skip required subagent | Claude believes subagent adds no value |
| **Out-of-Sequence** | Execute N+1 before N | Claude wants to parallelize/reorder |

**MANDATORY vs OPTIONAL:** BLOCKING subagents = MANDATORY (cannot omit). Conditional subagents = OPTIONAL (may skip with reason).

### Consent Protocol (MANDATORY)

**IF considering ANY workflow deviation, you MUST invoke AskUserQuestion:**
```
HALT - Before proceeding, invoke:
AskUserQuestion(questions=[{
  question: "I am considering: {deviation}. This deviates from TDD workflow. How should I proceed?",
  header: "Workflow Deviation Request",
  options: [
    {label: "Follow workflow", description: "Execute required {phase/subagent} as documented"},
    {label: "Skip with documentation", description: "Skip and document in story Implementation Notes"},
    {label: "User override", description: "I authorize this specific deviation"}
  ]
}])
```
**CRITICAL:** Do NOT proceed until user responds.

### Response Processing

| Option | Action | Story File Update |
|--------|--------|-------------------|
| **Follow workflow** | Execute required phase/subagent | None |
| **Skip with documentation** | Update Implementation Notes with: Deviation, Reason, Authorization timestamp, Impact | `### Authorized Deviations` table |
| **User override** | Record override with timestamp | `### User Overrides` list |

**RCA Recommendation (Optional):** After "Skip with documentation", display: *"Consider running '/rca {reason}' to analyze deviation pattern. (Optional - not blocking)"*

### Enforcement Notes

Per architecture-constraints.md HALT pattern (lines 116-132): Claude MUST NOT rationalize deviations without explicit user consent via AskUserQuestion.

**Reference:** RCA-019, architecture-constraints.md

---

## Required Subagents Per Phase

| Phase | Required Subagents | Enforcement |
|-------|-------------------|-------------|
| 01 | git-validator, tech-stack-detector | BLOCKING |
| 02 | test-automator | BLOCKING |
| 03 | backend-architect OR frontend-developer, context-validator | BLOCKING |
| 04 | refactoring-specialist, code-reviewer | BLOCKING |
| 4.5 | ac-compliance-verifier | BLOCKING |
| 05 | integration-tester | BLOCKING |
| 5.5 | ac-compliance-verifier | BLOCKING |
| 06 | deferral-validator (if deferrals) | CONDITIONAL |
| 07 | (none - file operations) | N/A |
| 08 | (none - git operations) | N/A |
| 09 | framework-analyst | BLOCKING |
| 10 | dev-result-interpreter | BLOCKING |

**Note:** Phase files contain subagent invocation templates.

---

## Phase Brief Summary

Quick reference for each of the 10 phases. Detailed execution guides in `phases/` directory (e.g., `phases/phase-01-preflight.md`):

| Phase | Name | Primary Focus |
|-------|------|---------------|
| 01 | Pre-Flight | Context validation, Git status, story loading |
| 02 | Test-First (Red) | Write failing tests from acceptance criteria |
| 03 | Implementation (Green) | Minimal code to pass tests |
| 04 | Refactoring | Improve code quality, maintain tests |
| 04.5 | AC Verification | Fresh-context acceptance criteria check |
| 05 | Integration | Cross-component interaction testing |
| 05.5 | AC Verification | Post-integration acceptance criteria check |
| 06 | Deferral Challenge | Validate deferred items have justification |
| 07 | DoD Update | Update Definition of Done checkboxes |
| 08 | Git Workflow | Commit changes with conventional message |
| 09 | Feedback Hook | Trigger post-dev feedback collection |
| 10 | Result | Generate workflow result summary |

---

## Phase Completion Self-Check Displays (RCA-011)

**Purpose:** Visual confirmation of mandatory step completion with line number references for audit trail.

**Line Number Reference Format:** `(lines XXX-YYY)` where XXX-YYY are the conversation lines where the Task/Skill was invoked.

### Display Template Pattern

All phase completion displays follow this structure:
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase NN/10: [Phase Name] - Mandatory Steps Completed
  TDD Iteration: 1/5
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[checkmarks for completed steps with line references]

All Phase NN mandatory steps completed. Proceeding to Phase NN+1...
```

**Iteration Counter Display Rules:**
- Display "TDD Iteration: X/5" in every phase header
- IF iteration_count >= 4: Display "TDD Iteration: X/5 ⚠️ Approaching limit"
- IF iteration_count >= max_iterations (5): HALT with "Maximum iterations reached"

### Phase 03 Completion Display (TDD Green)

**Before marking Phase 03 (Implementation) complete, display:**

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 03/10: Implementation - Mandatory Steps Completed
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  backend-architect invoked (lines XXX-YYY)
  context-validator invoked (lines XXX-YYY)
  AC Checklist items updated (implementation items)

All Phase 03 mandatory steps completed. Proceeding to Phase 04...
```

**Required checks:**
- backend-architect OR frontend-developer subagent was invoked
- context-validator subagent was invoked
- All tests GREEN (passing)

### Phase 04 Completion Display (Refactoring)

**Before marking Phase 04 (Refactoring) complete, display:**

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 04/10: Refactoring - Mandatory Steps Completed
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  refactoring-specialist invoked (lines XXX-YYY)
  code-reviewer invoked (lines XXX-YYY)
  Light QA executed (lines XXX-YYY)
  AC Checklist items updated (quality items)

All Phase 04 mandatory steps completed. Proceeding to Phase 05...
```

**Required checks:**
- refactoring-specialist subagent was invoked
- code-reviewer subagent was invoked
- Light QA validation executed
- All tests still passing

### Phase 10 Completion Display (Result Interpretation)

**Before returning results (Phase 10), display:**

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 10/10: Result Interpretation - Mandatory Steps Completed
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  dev-result-interpreter invoked (lines XXX-YYY)

All mandatory steps completed. Returning final results...
```

### Phase Validation Checkpoint Template (STORY-169 / RCA-013)

**Purpose:** Verify all mandatory subagents were invoked before proceeding to next phase.

**Verification Method:** For each required subagent, check for Task() call in conversation history.

**Subagent Verification Logic:**

```
FOR required_subagent in phase_required_subagents:
  # Check for Task() call in conversation for this subagent
  IF conversation contains Task(subagent_type="{required_subagent}"):
    mark_verified(required_subagent)
  ELSE:
    add_to_missing(required_subagent)
```

**IF any check fails:**
```
Display: "Phase X incomplete: {missing items}"
HALT (do not proceed to Phase X+1)
Prompt: "Complete missing items before proceeding"
```

**IF all checks pass:**
```
Display: "Phase X validation passed - all mandatory steps completed"
Proceed to Phase X+1
```

---

## Technical Debt Override Banner Display (STORY-289)

**Purpose:** Display persistent warning banner when user accepts debt override.

**Trigger:** When `$DEBT_OVERRIDE_BANNER = true` (set in Phase 01 Step 10 during debt threshold evaluation)

**Banner Display Pattern:**
```
IF $DEBT_OVERRIDE_BANNER == true:
    # Display debt banner at start of each phase
    Display:
    "┌──────────────────────────────────────────────────────────────┐"
    "│ ⚠️  DEBT OVERRIDE ACTIVE - Proceeding with elevated debt     │"
    "│    Override logged in phase-state.json for audit trail      │"
    "└──────────────────────────────────────────────────────────────┘"
```

**Persistence Rules:**
- Banner displays at start of EVERY phase (02-10) when override is active
- Banner persists throughout entire workflow execution
- Override information logged to phase-state.json for compliance review

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
- [ ] DoD validation passed
- [ ] Changes committed (or file-tracked)
- [ ] Story status = "Dev Complete"
- [ ] All 10 phases completed in state file

**The goal: Zero technical debt from wrong assumptions, fully tested features that comply with architectural decisions.**

<!-- ═══════════════════════════════════════════════════════════════════════════ -->
<!-- MIDDLE 30%: PHASE INSTRUCTIONS                                              -->
<!-- Execution instructions and phase orchestration                              -->
<!-- ═══════════════════════════════════════════════════════════════════════════ -->

---

## Workflow Execution Checklist

**After parameter extraction, BEFORE Phase 01, create execution tracker:**

**Initialize iteration counter:**
```
iteration_count = 1  # Track TDD cycle iterations (for Phase 06 resumption)
max_iterations = 5   # Maximum allowed iterations before blocking
last_iteration_date = null

# Resume logic: Read iteration_count from phase-state.json if resuming
IF resuming from existing workflow:
    iteration_count = phase_state.iteration_count OR 1
    last_iteration_date = phase_state.last_iteration_date OR null
```

TodoWrite(
  todos=[
    {content: "Execute Phase 01: Pre-Flight Validation", status: "pending", activeForm: "Executing Phase 01 Pre-Flight Validation"},
    {content: "Execute Phase 02: Test-First Design (test-automator)", status: "pending", activeForm: "Executing Phase 02 test-automator"},
    {content: "Execute Phase 02 Step 4: Tech Spec Coverage Validation", status: "pending", activeForm: "Validating Tech Spec Coverage"},
    {content: "Execute Phase 03 Step 1-2: backend-architect OR frontend-developer", status: "pending", activeForm: "Executing backend/frontend architect"},
    {content: "Execute Phase 03 Step 3: context-validator", status: "pending", activeForm: "Validating context constraints"},
    {content: "Execute Phase 04 Step 1-2: refactoring-specialist", status: "pending", activeForm: "Executing refactoring specialist"},
    {content: "Execute Phase 04 Step 3: code-reviewer", status: "pending", activeForm: "Executing code reviewer"},
    {content: "Execute Phase 04 Step 5: Light QA", status: "pending", activeForm: "Executing Light QA validation"},
    {content: "Execute Phase 05: Integration Testing (integration-tester)", status: "pending", activeForm: "Executing integration testing"},
    {content: "Execute Phase 06: Deferral Challenge", status: "pending", activeForm: "Executing deferral challenge"},
    {content: "Execute Phase 07: DoD Update (Bridge)", status: "pending", activeForm: "Updating DoD checkboxes"},
    {content: "Execute Phase 08: Git Workflow", status: "pending", activeForm: "Executing git workflow"},
    {content: "Execute Phase 09: Feedback Hooks", status: "pending", activeForm: "Executing feedback hooks"},
    {content: "Execute Phase 10 Step 10.1: dev-result-interpreter", status: "pending", activeForm: "Interpreting dev results"}
  ]
)

**Usage:** Mark phase "in_progress" when starting, "completed" when checkpoint passes.

---

### TodoWrite-Gate Integration Pattern (MANDATORY)

**ENFORCEMENT:** Phase completion status in TodoWrite is GATED by CLI validation.

**5-Step TodoWrite Enforcement Pattern:**

```
FOR each phase N in [01..10]:

  1. Mark phase "in_progress" at phase start
     TodoWrite(mark phase N "in_progress")
     Display: "Phase {N}/{10}: {phase_name}"

  2. Execute all phase steps
     Read(file_path=".claude/skills/devforgeai-development/references/{phase-reference}.md")
     [Execute all steps from reference file]

  3. Call CLI gate: devforgeai-validate phase-complete
     Bash(command="devforgeai-validate phase-complete ${STORY_ID} --phase={N} --checkpoint-passed")

  4. IF gate exit code 0: Mark phase "completed"
     TodoWrite(mark phase N "completed")
     Display: "✓ Phase {N} complete"
     Proceed to Phase N+1

  5. IF gate exit code != 0: Keep "in_progress", HALT
     Display: "❌ Phase {N} gate failed - see error message"
     HALT (keep phase N as "in_progress")
```

**CRITICAL RULES for TodoWrite Completion:**

- ❌ **CANNOT** mark phase "completed" without gate exit code 0
- ❌ **CANNOT** start Phase X+1 while Phase X shows "in_progress" or "pending"
- ✅ Phase **MUST** remain "in_progress" until CLI gate passes
- ✅ CLI gate **MUST** be called before TodoWrite status change
- ✅ **Never** mark completed without gate call - gate failure = HALT
- ✅ Gate failure = HALT (address issues before retry)

---

## Phase State Initialization [MANDATORY FIRST]

**Initialize phase tracking before any TDD work:**

```bash
# Initialize state file for this story
devforgeai-validate phase-init ${STORY_ID} --project-root=.
```

**Handle return codes:**
```
IF exit_code == 0:
    # New workflow - state file created
    Display: "✓ Phase state initialized for ${STORY_ID}"
    CURRENT_PHASE = "01"
    GOTO Phase Orchestration Loop

IF exit_code == 1:
    # Existing workflow - resume from current phase
    Display: "📋 Resuming existing workflow for ${STORY_ID}"
    devforgeai-validate phase-status ${STORY_ID}
    CURRENT_PHASE = parse from output
    Display: "  Current phase: ${CURRENT_PHASE}"
    GOTO Phase Orchestration Loop (starting at CURRENT_PHASE)

IF exit_code == 2:
    # Invalid story ID
    Display: "❌ Invalid story ID: ${STORY_ID}"
    Display: "  Story ID must match pattern STORY-XXX (e.g., STORY-001)"
    HALT workflow
```

**Backward Compatibility Warning:**
```
IF devforgeai-validate command not found (exit code 127):
    Display: "⚠️  Warning: Phase enforcement CLI not installed"
    Display: "    To enable enforcement, run: pip install devforgeai-validate"
    Display: "    Continuing workflow without enforcement (backward compatibility mode)"
```

---

## Phase Orchestration Loop

**Execute phases sequentially, loading each phase file on demand:**

```
FOR phase_num in range(CURRENT_PHASE, 11):
    phase_id = f"{phase_num:02d}"

    # 1. Determine phase file
    phase_files = {
        "01": "phase-01-preflight.md",
        "02": "phase-02-test-first.md",
        "03": "phase-03-implementation.md",
        "04": "phase-04-refactoring.md",
        "4.5": "phase-04.5-ac-verification.md",
        "05": "phase-05-integration.md",
        "5.5": "phase-05.5-ac-verification.md",
        "06": "phase-06-deferral.md",
        "07": "phase-07-dod-update.md",
        "08": "phase-08-git-workflow.md",
        "09": "phase-09-feedback.md",
        "10": "phase-10-result.md"
    }

    # 2. Load phase file (progressive loading)
    Read(file_path=f".claude/skills/devforgeai-development/phases/{phase_files[phase_id]}")

    # 3-7. Execute phase content, gates, and record subagents
    # See phase files for details
    Read(file_path=".claude/skills/devforgeai-development/references/dod-update-workflow.md")
```

---

## Phase File Structure

Each phase file in `phases/` directory contains:

```markdown
# Phase NN: [Phase Name]

**Entry Gate:**
devforgeai-validate phase-check ${STORY_ID} --from={N-1} --to={N}
# Exit code 0: Proceed
# Exit code 1: Previous phase incomplete - HALT
# Exit code 2: Missing subagents - HALT

---

## Phase Workflow
[Specific phase steps, subagent invocations, validations]

---

## Validation Checkpoint
[Checklist of what must be verified]

---

**Exit Gate:**
devforgeai-validate phase-complete ${STORY_ID} --phase={N} --checkpoint-passed
```

---

## Remediation Mode Decision Point

**After Phase 01 completes, check `$REMEDIATION_MODE` flag:**

```
IF $REMEDIATION_MODE == true:
    Display: "🔧 REMEDIATION MODE ACTIVE"
    Read(file_path=".claude/skills/devforgeai-development/references/qa-remediation-workflow.md")
    SKIP: Normal TDD Phases 02-08
    GOTO: Phase 09 (Feedback Hook) after remediation
ELSE:
    Continue with Phase 02
```

---

## Phase Brief Descriptions

**Phase 00:** Initialization - Phase state file creation via `devforgeai-validate phase-init`

**Phase 01:** Pre-Flight - Context validation, Git status, story loading. Subagents: git-validator, tech-stack-detector

**Phase 02:** Test-First (Red) - Write failing tests from acceptance criteria. Subagents: test-automator

**Phase 03:** Implementation (Green) - Implement minimal code to pass tests. Subagents: backend-architect OR frontend-developer, context-validator

**Phase 04:** Refactoring - Improve code quality without changing behavior. Subagents: refactoring-specialist, code-reviewer

**Phase 04.5:** AC Compliance Verification (Post-Refactor) - Verify acceptance criteria fulfillment using fresh-context technique. Subagents: ac-compliance-verifier

**Phase 05:** Integration Testing - Verify cross-component interactions. Subagents: integration-tester

**Phase 05.5:** AC Compliance Verification (Post-Integration) - Verify acceptance criteria fulfillment after integration. Subagents: ac-compliance-verifier

**Phase 06:** Deferral Challenge - Validate deferred items have proper justification. Subagents: deferral-validator (conditional)

**Phase 07:** DoD Update - Update Definition of Done checkboxes in story file

**Phase 08:** Git Workflow - Commit changes with conventional commit message

**Phase 09:** Feedback Hook - Trigger post-dev feedback collection if hooks enabled

**Phase 10:** Result Interpretation - Generate workflow result summary. Subagents: dev-result-interpreter

---

## Phase Transition Validation Calls (STORY-153)

**Validation calls are required at every phase transition.** See `devforgeai/config/validation-call-locations.yaml` for complete mapping.

**Phase-check calls at each transition (11 total):**

| From | To | Command |
|------|-----|---------|
| 01 | 02 | `devforgeai-validate phase-check STORY-XXX --from=01 --to=02` |
| 02 | 03 | `devforgeai-validate phase-check STORY-XXX --from=02 --to=03` |
| 03 | 04 | `devforgeai-validate phase-check STORY-XXX --from=03 --to=04` |
| 04 | 4.5 | `devforgeai-validate phase-check STORY-XXX --from=04 --to=4.5` |
| 4.5 | 05 | `devforgeai-validate phase-check STORY-XXX --from=4.5 --to=05` |
| 05 | 5.5 | `devforgeai-validate phase-check STORY-XXX --from=05 --to=5.5` |
| 5.5 | 06 | `devforgeai-validate phase-check STORY-XXX --from=5.5 --to=06` |
| 06 | 07 | `devforgeai-validate phase-check STORY-XXX --from=06 --to=07` |
| 07 | 08 | `devforgeai-validate phase-check STORY-XXX --from=07 --to=08` |
| 08 | 09 | `devforgeai-validate phase-check STORY-XXX --from=08 --to=09` |
| 09 | 10 | `devforgeai-validate phase-check STORY-XXX --from=09 --to=10` |

**Complete-phase calls at phase end (12 total):**

| Phase | Command |
|-------|---------|
| 01-10 | `devforgeai-validate complete-phase STORY-XXX --phase=NN --checkpoint-passed` |
| 4.5, 5.5 | `devforgeai-validate complete-phase STORY-XXX --phase=4.5 --checkpoint-passed` |

**Record-subagent calls after Task invocations:**
```bash
devforgeai-validate record-subagent STORY-XXX --phase=NN --subagent=SUBAGENT_NAME
```

---

## Complete Workflow Execution Map

```
START
  ↓
Phase State Initialization (devforgeai-validate phase-init)
  ↓
┌─── RESUME CHECK ───┐
│ IF state exists:   │
│   Resume from      │
│   current_phase    │
└────────────────────┘
  ↓
FOR each phase 01-10:
  ├─ Load phase file (Read)
  ├─ Execute Entry Gate (CLI phase-check)
  │   └─ IF blocked: HALT
  ├─ Execute Phase Workflow
  │   ├─ Invoke required subagents
  │   ├─ Record subagent invocations
  │   └─ Validate checkpoint
  └─ Execute Exit Gate (CLI phase-complete)
      └─ IF fails: HALT
  ↓
Phase 10 Complete → Story Status = "Dev Complete"
  ↓
END
```

---

## Workflow Completion Self-Check (MANDATORY BEFORE RESULT DISPLAY)

**Before displaying final result or returning to user, verify ALL phases completed:**

```
FINAL VALIDATION:

completed_count = 0
missing_phases = []

FOR each phase in [01, 02, 03, 04, 05, 06, 07, 08, 09, 10]:
  IF phase.status == "completed":
    completed_count += 1
  ELSE:
    missing_phases.append(phase)

IF completed_count < 10:
  Display: "❌ WORKFLOW INCOMPLETE - Cannot declare completion"
  Display: "Phases completed: {completed_count}/10"
  HALT (do not display "Workflow Complete" banner)

IF completed_count == 10:
  Display: "✓ All 10 phases completed - Workflow validation passed"
  Proceed to display final result
```

---

## Phase Resumption Protocol

**When workflow stops incomplete:**

### User Recovery Command
```
Continue /dev workflow for STORY-XXX from Phase Y.
Resume execution now.
```

### Claude Resumption Steps

1. **Check TodoWrite State** - Find first incomplete phase
2. **Verify Previous Phases** - Check evidence (state markers, artifacts)
3. **Load Phase Reference** - Read appropriate phase file
4. **Execute Remaining Phases** - Complete all phases from current to 10
5. **Final Validation** - Execute Workflow Completion Self-Check

### Resumption vs Fresh Start Decision

| Scenario | Recommendation |
|----------|----------------|
| Phase state markers exist | Resume (state is reliable) |
| No evidence of prior execution | Fresh start |
| Git conflicts detected | Fresh start after resolving |

<!-- ═══════════════════════════════════════════════════════════════════════════ -->
<!-- BOTTOM 10%: INVOCATION PATTERN                                              -->
<!-- Trigger conditions, integration points, and reference documentation         -->
<!-- ═══════════════════════════════════════════════════════════════════════════ -->

---

## When to Use This Skill

**Prerequisites:** Git repo (recommended), context files (6), story file

**Git modes:** Full workflow (with Git) OR file-based tracking (without Git) - auto-detects

**Invoked by:** `/dev [STORY-ID]` command, devforgeai-orchestration skill, manual skill call

---

## Integration Points

**From:** devforgeai-story-creation (story+AC), devforgeai-architecture (context files)
**To:** devforgeai-qa (validation), devforgeai-release (deployment)
**Auto-invokes:** devforgeai-architecture (if missing), devforgeai-qa (light mode), devforgeai-story-creation (deferrals)

---

## Reference Files

Load on-demand during workflow execution:

### Core Workflow (in phases/ directory)
- **phase-01-preflight.md** - Pre-Flight Validation
- **phase-02-test-first.md** - Test-First Design (TDD Red)
- **phase-03-implementation.md** - Implementation (TDD Green)
- **phase-04-refactoring.md** - Refactoring + Light QA
- **phase-05-integration.md** - Integration Testing
- **phase-06-deferral.md** - Deferral Challenge
- **phase-07-dod-update.md** - DoD Update (Bridge)
- **phase-08-git-workflow.md** - Git Workflow
- **phase-09-feedback.md** - Feedback Hook
- **phase-10-result.md** - Result Interpretation

### Supporting References (in references/ directory)
- **parameter-extraction.md** - Story ID extraction
- **preflight-validation.md** - Phase 01 detailed workflow
- **tdd-red-phase.md** - Phase 02 detailed workflow
- **tdd-green-phase.md** - Phase 03 detailed workflow
- **tdd-refactor-phase.md** - Phase 04 detailed workflow
- **ac-verification-workflow.md** - AC Verification for Phase 4.5/5.5
- **integration-testing.md** - Phase 05 detailed workflow
- **phase-06-deferral-challenge.md** - Phase 06 detailed workflow
- **dod-update-workflow.md** - Phase 07 detailed workflow
- **git-workflow-conventions.md** - Phase 08 detailed workflow
- **tdd-patterns.md** - Comprehensive TDD guidance
- **ambiguity-protocol.md** - When to ask user questions
- **memory-file-schema.md** - Memory file YAML schema (STORY-303)
- **memory-file-operations.md** - Memory file read/write operations (STORY-303)

---

## Memory File State Persistence (STORY-303)

Session state is persisted to memory files for cross-session recovery.

**Location:** `.claude/memory/sessions/{STORY_ID}-{workflow}-session.md`

**Capabilities:**
- Resume from last known phase on session restart
- Track key decisions with rationale
- Record blockers for audit trail
- 39% performance improvement in context recovery

**Backward Compatibility:**
- Legacy workflows without memory files start normally from Phase 01
- No memory file required for initial run
- Memory file created automatically on first phase completion

**Reference:**
- `references/memory-file-schema.md` (field definitions)
- `references/memory-file-operations.md` (read/write operations)

---

## CLI Commands Reference

**Phase State Management:**
```bash
devforgeai-validate phase-init STORY-XXX --project-root=.
devforgeai-validate phase-check STORY-XXX --from=01 --to=02
devforgeai-validate phase-complete STORY-XXX --phase=02 --checkpoint-passed
devforgeai-validate phase-status STORY-XXX
devforgeai-validate phase-record STORY-XXX --phase=02 --subagent=test-automator
```

**Exit Codes:**
| Code | Meaning |
|------|---------|
| 0 | Success / Allowed |
| 1 | State exists (resume) / Previous phase incomplete |
| 2 | Invalid story ID / Missing required subagents |
| 3 | Cannot skip phases |

---

## Rollback Recovery

If workflow fails mid-execution:

1. Check current phase: `devforgeai-validate phase-status STORY-XXX`
2. Review phase file for checkpoint requirements
3. Fix blocking issue
4. Re-run `/dev STORY-XXX` - will resume from current phase

**State file location:** `devforgeai/workflows/STORY-XXX-phase-state.json`

---

## Change Log

| Date | Change | Reference |
|------|--------|-----------|
| 2025-11-13 | Added RCA-008 git safety enhancements | RCA-008 |
| 2026-01-02 | Added Phase Completion Self-Check Displays | RCA-011, STORY-164 |
| 2026-01-04 | Added Visual Iteration Counter | RCA-013, STORY-170 |
| 2026-01-22 | Restructured with Inverted Pyramid pattern | STORY-293 |
| 2026-01-23 | Added 10-phase summary, Phase Brief Summary table, baseline metrics | STORY-298 |
