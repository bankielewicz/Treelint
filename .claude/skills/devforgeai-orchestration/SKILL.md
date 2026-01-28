---
name: devforgeai-orchestration
description: Coordinates spec-driven development workflow from Epic → Sprint → Story → Architecture → Development → QA → Release. Manages story lifecycle, enforces quality gates, and orchestrates skill invocation. Use when starting epics/sprints, creating stories, managing workflow progression, or enforcing quality checkpoints.
model: claude-model: opus-4-5-20251001
allowed-tools:
  - Read
  - Write
  - Edit
  - Glob
  - Grep
  - AskUserQuestion
  - Skill
  - Task
---

# DevForgeAI Orchestration Skill

Coordinate the complete spec-driven development lifecycle with automated skill orchestration and quality gate enforcement.

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

**Proceed to "Purpose" section below and begin execution.**

---

## Purpose

This skill is the **workflow coordinator** for the entire spec-driven development framework. It manages the progression of work from high-level business initiatives (Epics) through implementation (Stories) to production release.

### Core Responsibilities

1. **Project Management Integration** - Support Epic → Sprint → Story hierarchy with stories as atomic work units
2. **Skill Coordination** - Auto-invoke architecture, development, QA, and release skills at appropriate workflow stages
3. **State Management** - Track and validate story status across 11 workflow states with sequential progression
4. **Quality Gate Enforcement** - Block transitions when quality standards not met (context validation, test passing, QA approval, release readiness)

### Philosophy

- **Epic → Sprint → Story decomposition** - Break large initiatives into manageable stories
- **Story is atomic unit** - Each story is independently deliverable
- **Quality over speed** - Never skip quality gates
- **No workflow shortcuts** - Every stage must complete successfully
- **Automated orchestration** - Skills invoke each other automatically
- **Transparency** - Complete workflow history in story documents

---

## When to Use This Skill

**Use this skill when:**
- Starting a new epic or sprint
- Creating stories from requirements
- Managing story workflow progression
- Checking story status
- Enforcing quality gates
- Coordinating multi-story releases
- Tracking deferred work (RCA-006 enhanced)
- Analyzing technical debt

**Entry point:**
```
Skill(command="devforgeai-orchestration")
```

**Context markers determine mode:**
- `**Command:** create-epic` + `**Epic name:** {name}` → Epic Creation Mode
- `**Command:** create-sprint` + `**Sprint Name:** {name}` → Sprint Planning Mode
- `**Command:** audit-deferrals` → Audit Deferrals Mode (STORY-050)
- `**Story ID:** STORY-NNN` → Story Management Mode (default)

---

## Mode Detection

This skill operates in **5 modes** based on conversation context markers.

**See `references/mode-detection.md` for complete detection logic.**

### Supported Modes

1. **Epic Creation Mode** - Phase 4A workflow (8-phase comprehensive process)
2. **Sprint Planning Mode** - Phase 3 workflow (capacity validation, story selection)
3. **Audit Deferrals Mode** - Phase 7 workflow (deferral audit + hook integration) - NEW (STORY-050)
4. **Story Management Mode** - Complete story lifecycle orchestration (default)
5. **Default Mode** - Analyze context to infer operation OR HALT if ambiguous

**Mode priority:** Epic > Sprint > Audit > Story > Default

---

## Workflow States

**⚠️ EXECUTION STARTS HERE - You are now executing the skill's workflow.**

Stories progress through **11 sequential states:**

```
Backlog → Architecture → Ready for Dev → In Development → Dev Complete →
QA In Progress → [QA Approved | QA Failed] → Releasing → Released
```

**See `references/workflow-states.md` for complete state definitions.**
**See `references/state-transitions.md` for valid transitions and prerequisites.**

---

## Orchestration Phases

This skill executes workflows in **distinct phases**. Each phase loads its reference file on-demand for detailed implementation logic.

### Phase 0: Checkpoint Detection with Parallel Context Loading
**Purpose:** Resume interrupted workflows and load context files in parallel
**References:** `checkpoint-detection.md`, `context-loader.md`
**Checkpoints:** DEV_COMPLETE, QA_APPROVED, STAGING_COMPLETE, PRODUCTION_COMPLETE
**Parallel Pattern:** 6 Read calls in single message for implicit parallel execution (STORY-111)

### Phase 1: Story Validation
**Purpose:** Load and validate story file before execution
**Reference:** `story-validation.md`
**Validates:** File exists, status valid, prerequisites met, quality gates allow progression

### Phase 2: Skill Invocation
**Purpose:** Coordinate automatic invocation of other skills
**Reference:** `skill-invocation.md`
**Skills invoked:** devforgeai-architecture, development, qa, release (based on status)

### Phase 3: Sprint Planning
**Purpose:** Create sprints with capacity validation
**Reference:** `sprint-planning-guide.md`
**Subagent:** sprint-planner (document generation, story updates)

### Phase 3A: Story Status Update
**Purpose:** Update story status and workflow history
**Reference:** `story-status-update.md`
**Updates:** Status, timestamps, checkboxes, history entries

### Phase 3.5: QA Retry Loop
**Purpose:** Manage QA failure recovery (max 3 attempts)
**Reference:** `qa-retry-workflow.md`
**Logic:** Attempt tracking, deferral handling, loop prevention, automatic retry

### Phase 4A: Epic Creation
**Purpose:** Generate epics from requirements (9-phase workflow with post-epic feedback hook)
**References:** `epic-management.md`, `feature-decomposition-patterns.md`, `technical-assessment-guide.md`, `epic-validation-checklist.md`
**Subagents:** requirements-analyst (feature decomposition), architect-reviewer (technical assessment)

#### Phase 4A.9: Post-Epic Feedback Hook (NEW - STORY-028)
**Purpose:** Trigger retrospective feedback conversation after successful epic creation
**Prerequisites:** Epic file created (Phase 4A.5), validation passed (Phase 4A.7)
**Behavior:** Non-blocking hook invocation with graceful error handling

### Phase 4.5: Deferred Work Tracking
**Purpose:** Track and validate deferred DoD items
**Reference:** `deferred-tracking.md`
**Subagent:** technical-debt-analyzer (debt trend analysis, circular detection)

### Phase 5: Next Action Determination
**Purpose:** Recommend next steps based on current state
**Reference:** `next-action-determination.md`
**Output:** Recommended actions, manual commands, workflow guidance

### Phase 6: Orchestration Finalization
**Purpose:** Generate completion summary and finalize workflow
**Reference:** `orchestration-finalization.md`
**Output:** Timeline, phases executed, quality gates passed, metrics summary

### Phase 7: Hook Integration for Audit Deferrals (STORY-050 - NEW)
**Purpose:** Execute complete deferral audit workflow with feedback hook integration (7 substeps)
**Reference:** `audit-deferrals-workflow.md`
**Trigger:** `/audit-deferrals` command invocation
**Output:** Deferral audit report, resolvable/valid/invalid categorization, hook invocation (if eligible)

#### Step 7.1: Check Eligibility
Validate hooks.yaml config and operation enabled status.

#### Step 7.2: Prepare Context
Extract audit metadata from generated report (resolvable/valid/invalid counts, oldest age, circular chains).

#### Step 7.3: Sanitization
Remove sensitive information (credentials, API keys) before passing to hooks. Sanitize audit metadata.

#### Step 7.4: Invocation
Call `devforgeai-validate invoke-hooks` with audit context and metadata. Invoke feedback hooks.

#### Step 7.5: Logging
Append hook invocation record to `devforgeai/feedback/logs/hook-invocations.log`. Log all hook operations.

#### Step 7.6: Handle Errors Gracefully
Non-blocking error handling - log failures, audit continues successfully.

#### Step 7.7: Prevent Circular Invocations
Depth tracking to prevent command → hook → command → hook loops.

### Phase 7A: Sprint Retrospective (RCA-006 Phase 2)
**Purpose:** Auto-audit technical debt at sprint completion, create debt reduction sprints
**Reference:** `sprint-retrospective.md`
**Trigger:** Last story in sprint reaches "Released" status
**Output:** Sprint metrics, resolvable deferrals, debt reduction recommendations

---

## Quality Gate Enforcement

**Four gates block workflow progression** when requirements not met:

1. **Gate 1: Context Validation** (Architecture → Ready for Dev)
2. **Gate 2: Test Passing** (Dev Complete → QA In Progress)
3. **Gate 3: QA Approval** (QA Approved → Releasing)
4. **Gate 4: Release Readiness** (Releasing → Released)

**See `references/quality-gates.md` for complete gate requirements and enforcement logic.**

---

## Subagent Coordination

This skill delegates specialized tasks to **4 subagents:**

- **requirements-analyst** - Epic feature decomposition, requirements spec generation
- **architect-reviewer** - Epic technical assessment, complexity scoring (0-10)
- **sprint-planner** - Sprint creation, capacity validation, story status updates
- **technical-debt-analyzer** - Deferred work analysis, debt trend reporting, circular deferral detection

**See `references/skill-invocation.md` for subagent coordination patterns.**

---

## Reference Files

**Load these on-demand during workflow execution:**

### Core Workflow (10 files)
- `mode-detection.md` - Mode detection logic and context markers (329 lines)
- `checkpoint-detection.md` - Checkpoint recovery workflow (474 lines)
- `context-loader.md` - Parallel context file loading pattern (150 lines) - NEW (STORY-111)
- `story-validation.md` - Story file validation procedures (345 lines)
- `skill-invocation.md` - Skill coordination patterns (509 lines)
- `story-status-update.md` - Status update and history procedures (278 lines)
- `qa-retry-workflow.md` - QA failure recovery with max 3 attempts (919 lines)
- `deferred-tracking.md` - Technical debt tracking and analysis (714 lines)
- `next-action-determination.md` - Next step recommendations (287 lines)
- `orchestration-finalization.md` - Completion summary generation (513 lines)

### Epic Management (6 files)
- `epic-management.md` - Epic creation phases 1-2 (496 lines)
- `feature-decomposition-patterns.md` - Phase 3 patterns by domain (903 lines)
- `feature-analyzer.md` - Parallel feature analysis with batching (180 lines) - NEW (STORY-111)
- `dependency-graph.md` - Dependency detection for safe parallelization (120 lines) - NEW (STORY-111)
- `technical-assessment-guide.md` - Phase 4 complexity scoring (914 lines)
- `epic-validation-checklist.md` - Phase 7 validation and self-healing (760 lines)

### Sprint Management (2 files)
- `sprint-planning-guide.md` - Sprint creation and capacity (631 lines)
- `sprint-retrospective.md` - Sprint retrospective and debt audit (390 lines - RCA-006 Phase 2)

### Audit Management (1 file) - NEW (STORY-050)
- `audit-deferrals-workflow.md` - Complete deferral audit + hook integration (10.2K chars - Phase 8)

### State Management (2 files)
- `workflow-states.md` - 11 state definitions (585 lines)
- `state-transitions.md` - Valid transitions and rules (1,105 lines)

### Supporting Files (4 files)
- `quality-gates.md` - 4 gate requirements and enforcement (1,017 lines)
- `story-management.md` - Story lifecycle procedures (633 lines)
- `user-interaction-patterns.md` - AskUserQuestion templates (513 lines)
- `troubleshooting.md` - Common issues and solutions (935 lines)

**Total reference content:** 24 files, ~12,450 lines (loaded progressively as needed)

---

## Phase 4A.9: Post-Epic Feedback Hook Implementation

### Overview

**Phase 4A.9** extends the epic creation workflow with an optional, non-blocking feedback hook that triggers after successful epic creation. This phase collects retrospective feedback about epic quality, feature decomposition effectiveness, and technical complexity assessment while details are fresh.

**Key Characteristics:**
- **Non-blocking:** Hook failures don't break epic creation (exit code 0 always)
- **Optional:** Respects hook configuration (can be disabled)
- **Contextual:** Passes complete epic metadata to feedback system
- **Fast:** Hook check <100ms, total overhead <3 seconds (p95)
- **Safe:** Epic ID validated before CLI invocation (no command injection)

### Execution Flow

**When to invoke Phase 4A.9:**
- After Phase 4A.7 (Validation) completes successfully
- After Phase 4A.5 (Epic File Creation) confirms file exists
- Before Phase 4A.8 (Completion Summary) displays results

**Step 4A.9.1: Check Hook Configuration State**

```bash
# Purpose: Determine if hooks are enabled for epic-create operation
# Overhead: <100ms (fast configuration check)
# Exit handling: Graceful - if check fails, assume hooks disabled

# Command: Query hook system for epic-create operation status
devforgeai-validate check-hooks --operation=epic-create

# Expected output (JSON):
# {
#   "operation": "epic-create",
#   "enabled": true|false,
#   "available": true|false,
#   "timeout": 30000
# }

# Exit codes:
# 0 = Valid response (enabled/available fields present)
# 1 = Operation not registered in hooks.yaml
# 2 = Configuration invalid/malformed
```

**Error Handling for Step 4A.9.1:**
```bash
# If check-hooks command fails (not found, timeout, crash)
if [ $? -ne 0 ]; then
  # Log warning to devforgeai/feedback/logs/hooks.log
  echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] WARNING: check-hooks failed" >> devforgeai/feedback/logs/hooks.log

  # Assume hooks disabled (safe fallback)
  HOOKS_ENABLED=false

  # Continue (don't fail epic creation)
fi
```

**Step 4A.9.2: Parse Hook Configuration**

```bash
# If check-hooks succeeded, parse JSON response
if [ "$HOOKS_ENABLED" != "false" ]; then
  # Extract enabled boolean from JSON
  HOOKS_ENABLED=$(devforgeai-validate check-hooks --operation=epic-create --output=json | jq -r '.enabled')
  HOOK_TIMEOUT=$(devforgeai-validate check-hooks --operation=epic-create --output=json | jq -r '.timeout // 30000')
fi
```

**Error Handling for Step 4A.9.2:**
```bash
# If JSON parsing fails (malformed JSON, missing fields)
if [ -z "$HOOKS_ENABLED" ] || [ "$HOOKS_ENABLED" = "null" ]; then
  # Default to disabled (safe fallback)
  HOOKS_ENABLED=false
  HOOK_TIMEOUT=30000
fi
```

**Step 4A.9.3: Validate Epic Context**

```bash
# Purpose: Ensure epic file exists and epic ID is valid before invocation
# Prerequisite: EPIC_ID from Phase 4A.1 (Discovery), EPIC_FILE from Phase 4A.5 (Creation)

# Validation 1: Epic file exists
if [ ! -f "devforgeai/specs/Epics/$EPIC_ID-*.epic.md" ]; then
  # Epic creation incomplete - skip hook invocation
  echo "⚠️ Epic file not found (epic creation may be incomplete)" >> devforgeai/feedback/logs/hooks.log
  HOOKS_ENABLED=false
  return 0  # Non-blocking - continue
fi

# Validation 2: Epic ID format (EPIC-NNN pattern)
if ! [[ "$EPIC_ID" =~ ^EPIC-[0-9]{3}$ ]]; then
  # Invalid format - skip hook invocation, log error
  echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] ERROR: Invalid epic ID format: $EPIC_ID" >> devforgeai/feedback/logs/hook-errors.log
  return 0  # Non-blocking - continue
fi
```

**Step 4A.9.4: Invoke Feedback Hook (Conditional)**

```bash
# Only invoke if hooks are enabled AND configuration state is valid
if [ "$HOOKS_ENABLED" = "true" ]; then
  # Step 4A.9.4a: Set up timeout handling
  # Hook invocation must complete or timeout within $HOOK_TIMEOUT milliseconds
  HOOK_TIMEOUT_SECONDS=$((HOOK_TIMEOUT / 1000))

  # Step 4A.9.4b: Invoke hook with epic context (non-blocking subprocess)
  timeout $HOOK_TIMEOUT_SECONDS devforgeai-validate invoke-hooks \
    --operation=epic-create \
    --epic-id="$EPIC_ID" \
    --timeout=$HOOK_TIMEOUT &

  # Capture subprocess PID for potential timeout handling
  HOOK_PID=$!

  # Step 4A.9.4c: Log hook invocation start
  echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] INFO: Hook invoked - operation=epic-create epic-id=$EPIC_ID pid=$HOOK_PID" >> devforgeai/feedback/logs/hooks.log
fi
```

**Step 4A.9.5: Wait for Hook Completion (with Timeout)**

```bash
# Purpose: Allow hook to run, but don't block epic creation if timeout occurs

# Graceful waiting (max $HOOK_TIMEOUT milliseconds)
if [ ! -z "$HOOK_PID" ]; then
  # Wait for subprocess with timeout
  if wait $HOOK_PID 2>/dev/null; then
    # Hook completed successfully
    HOOK_EXIT_CODE=$?
    echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] INFO: Hook completed - exit_code=$HOOK_EXIT_CODE" >> devforgeai/feedback/logs/hooks.log
  else
    # Hook timeout or other failure
    HOOK_EXIT_CODE=$?
    if [ $HOOK_EXIT_CODE -eq 124 ]; then
      # Timeout occurred (exit code 124 from timeout command)
      echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] WARNING: Hook timed out after ${HOOK_TIMEOUT_SECONDS}s - epic-id=$EPIC_ID" >> devforgeai/feedback/logs/hook-errors.log
      echo "⚠️ Feedback session timed out (you can run 'devforgeai-validate invoke-hooks --operation=epic-create --epic-id=$EPIC_ID' manually later)"

      # Kill hook process if still running
      kill -9 $HOOK_PID 2>/dev/null || true
    else
      # Hook process error
      echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] ERROR: Hook failed with exit code $HOOK_EXIT_CODE - epic-id=$EPIC_ID" >> devforgeai/feedback/logs/hook-errors.log
      echo "⚠️ Feedback hook failed (continuing with epic creation)"
    fi
  fi
fi
```

**Step 4A.9.6: Handle Hook Errors Gracefully**

```bash
# All hook errors are non-blocking for epic creation

# If any error occurred:
# 1. Error is logged to devforgeai/feedback/logs/hook-errors.log
# 2. Warning message displayed to user ("Feedback hook unavailable (continuing)")
# 3. Epic creation exits with status 0 (success)
# 4. No user interaction required

# Example error scenarios and handling:
case $HOOK_EXIT_CODE in
  0)
    # Success - hook executed, feedback received
    ;;
  1)
    # Hook CLI not found or command error
    logger_error "Hook CLI not found - ensure devforgeai CLI installed"
    ;;
  2)
    # Invalid epic ID format
    logger_error "Hook validation failed - invalid epic context"
    ;;
  3)
    # Epic file not found
    logger_error "Hook validation failed - epic file missing"
    ;;
  124)
    # Timeout (already handled above)
    ;;
  *)
    # Unknown error
    logger_error "Hook failed with unknown error - exit code $HOOK_EXIT_CODE"
    ;;
esac

# Continue regardless of error - epic creation always succeeds
return 0
```

### Logging Requirements

**Success Logging** (to `devforgeai/feedback/logs/hooks.log`):
```
[2025-11-16T14:32:45Z] INFO: Hook invoked - operation=epic-create epic-id=EPIC-001 pid=12345
[2025-11-16T14:32:46Z] INFO: Hook completed - exit_code=0 duration=1200ms
```

**Error Logging** (to `devforgeai/feedback/logs/hook-errors.log`):
```
[2025-11-16T14:32:47Z] ERROR: Hook failed with exit code 1 - epic-id=EPIC-001 error="CLI not found"
[2025-11-16T14:32:48Z] WARNING: Hook timed out after 30s - epic-id=EPIC-002
[2025-11-16T14:32:49Z] ERROR: Invalid epic ID format: EPIC-99999
```

### Data Flow

```
Phase 4A.5 (Epic File Creation)
    ↓ (epic file written to disk)
Phase 4A.7 (Validation)
    ↓ (validation passed)
Phase 4A.9 (Post-Epic Feedback Hook) ← YOU ARE HERE
    ├─ Step 4A.9.1: Check hook configuration
    │   └─ If disabled → skip to Phase 4A.8
    │   └─ If enabled → continue to Step 2
    ├─ Step 4A.9.2: Parse hook config (timeout, enabled flag)
    ├─ Step 4A.9.3: Validate epic context (file exists, ID format)
    ├─ Step 4A.9.4: Invoke hook with epic-id
    │   └─ Non-blocking subprocess (timeout $HOOK_TIMEOUT)
    ├─ Step 4A.9.5: Wait for completion (with timeout)
    │   └─ If timeout → log warning, kill process, continue
    │   └─ If error → log error, continue
    ├─ Step 4A.9.6: Handle errors gracefully
    │   └─ All errors are non-blocking
    │   └─ Exit code 0 always (epic creation succeeds)
    ↓
Phase 4A.8 (Completion Summary)
    ↓
Return to command for display
```

### Integration with Lean Orchestration Pattern

**Skill responsibility:** Phase 4A.9 logic resides in orchestration skill (SKILL.md)
**Command responsibility:** Command displays hook result (if applicable) in Phase 4 (<20 lines)
**Budget impact:**
- Skill: +~80 lines for Phase 4A.9 implementation
- Command: +~10 lines for result display
- Total: Well within character budgets (skill isolated, command <15K)

### Success Criteria

Phase 4A.9 is complete when:
- [x] Hook configuration checked before invocation (<100ms)
- [x] Hook disabled in config → skipped entirely (zero overhead)
- [x] Hook invocation passes complete epic context (--epic-id)
- [x] Hook timeout enforced (30 seconds default, configurable)
- [x] Hook failures logged without breaking epic creation
- [x] Epic creation always exits with code 0 (non-blocking)
- [x] Logging structured (timestamp, operation, epic-id, status)
- [x] Epic ID validated before shell invocation (no command injection)

---

## Common Issues

**Top 5 issues and quick solutions:**

1. **Mode detection fails** → Check context markers (see `mode-detection.md`)
2. **Checkpoint not detected** → Verify Status History format (see `checkpoint-detection.md`)
3. **Quality gate blocks** → Review gate requirements (see `quality-gates.md`)
4. **QA retry exceeds 3** → Address root cause or split story (see `qa-retry-workflow.md`)
5. **Sprint capacity exceeded** → Remove lower-priority stories (see `sprint-planning-guide.md`)

**See `references/troubleshooting.md` for complete troubleshooting guide with 13 common issues.**

---

**The orchestration skill ensures every story follows the same high-quality workflow: Architecture → Development → QA → Release, with no shortcuts and complete transparency.**
