# /resume-dev - Resume Development from Specific Phase

Resume TDD workflow from specified phase when previous `/dev` execution was incomplete. Implements the "rewind" capability requested in RCA-013.
argument-hint: [STORY-ID]
model: opus
allowed-tools: Read, Skill, Bash(git:*)
---

## Quick Reference

```bash
# Resume from specific phase
/resume-dev STORY-057 2    # Resume from Phase 2 (Implementation)
/resume-dev STORY-057 3    # Resume from Phase 3 (Refactoring)

# Auto-detect resumption point
/resume-dev STORY-057      # Analyzes DoD, determines appropriate phase
```

---

## Command Workflow

### Phase 0: Argument Validation

**Validate story ID (required):**
```
STORY_ID=$1

IF $1 empty OR NOT match "STORY-[0-9]+":
    Display: "Usage: /resume-dev STORY-NNN [phase-number]"
    Display: ""
    Display: "Examples:"
    Display: "  /resume-dev STORY-057 2     # Resume from Phase 2 (Implementation)"
    Display: "  /resume-dev STORY-057       # Auto-detect resumption point"
    Display: ""
    Display: "Valid phase numbers:"
    Display: "  0 = Pre-Flight Validation"
    Display: "  1 = Red Phase (Test Generation)"
    Display: "  2 = Green Phase (Implementation)"
    Display: "  3 = Refactor Phase (Quality)"
    Display: "  4 = Integration Testing"
    Display: ""
    HALT
```

**Load story file:**
```
@devforgeai/specs/Stories/$1*.story.md

IF file not found:
    Display: "Story not found: $1"
    Display: ""
    Display: "List stories: Glob(pattern='devforgeai/specs/Stories/*.story.md')"
    HALT

Display: "✓ Story loaded: $1"
```

**Parse phase number (optional):**
```
PHASE_NUM=$2

IF $2 provided:
  # Manual mode - user specified phase
  IF $2 NOT match "[0-7]":
    Display: "Invalid phase number: $2"
    Display: ""
    Display: "Valid phases: 0-7"
    Display: "  0 = Pre-Flight"
    Display: "  1 = Red (Test Generation)"
    Display: "  2 = Green (Implementation)"
    Display: "  3 = Refactor (Quality)"
    Display: "  4 = Integration"
    Display: "  5 = Git Workflow (rarely needed)"
    Display: "  6 = Feedback Hook (rarely needed)"
    Display: "  7 = Result Interpretation (rarely needed)"
    Display: ""
    HALT

  PHASE_NUM=$2
  RESUME_MODE="manual"
  Display: "✓ Manual resumption: Phase $2"

ELSE:
  # Auto mode - detect from DoD completion
  RESUME_MODE="auto"
  Display: "Auto-detecting resumption point from DoD status..."
```

---

### Phase 1: Essential Pre-Flight Checks

**Execute critical Phase 0 validations (skip git-related steps):**

**Step 1.1: Validate Context Files Exist**
```
Bash(command="devforgeai-validate validate-context", description="Validate 6 context files exist")

IF validation fails:
  Display: "❌ Context files missing or invalid"
  Display: "   Run: /create-context to generate context files"
  HALT

Display: "✓ Context files validated (6/6 present)"
```

**Step 1.2: Validate Technology Stack**
```
Task(
  subagent_type="tech-stack-detector",
  description="Validate technology stack for resumed story",
  prompt="Detect and validate project technology stack against tech-stack.md.

  This is a RESUME operation - story was previously started, now resuming.

  Validate:
  1. Project technologies match tech-stack.md (no drift)
  2. No unapproved dependencies added since last run
  3. No library substitutions

  Return validation result (PASS/FAIL) and any conflicts.

  If FAIL: List conflicts that must be resolved before resuming development."
)

IF validation fails:
  Display: "❌ Tech stack conflicts detected"
  Display: "   Conflicts: {conflicts from subagent}"
  Display: "   Resolve conflicts before resuming development"
  HALT

Display: "✓ Technology stack validated"
```

**Step 1.3: Validate Spec vs Context Files**
```
Read story Technical Specification section

IF technical_spec contains technology NOT in tech-stack.md:
  Display: "❌ Spec-Context conflict detected"
  Display: "   Spec requires: {technology}"
  Display: "   tech-stack.md locked: {locked_technologies}"
  Display: ""

  AskUserQuestion:
    Question: "Spec requires technology not in tech-stack.md. How to proceed?"
    Header: "Conflict"
    Options:
      - "Use spec requirement (update tech-stack.md + create ADR)"
      - "Use tech-stack.md standard (spec is incorrect)"
      - "Stop and review conflict with user"
    multiSelect: false

  Handle response accordingly

Display: "✓ Spec validated against context files"
```

**Why these validations are essential:**
1. **Context files:** Story implementation depends on architectural constraints (tech-stack.md, source-tree.md, etc.)
2. **Tech stack:** Ensures no technology drift since story started (prevents incompatible code)
3. **Spec vs Context:** Prevents implementing features that violate locked architectural decisions
4. **Git validation skipped:** Not needed for resumption (story already initialized, git status irrelevant for continuing work)

**Time saved vs full Phase 0:** ~3 minutes (skip git-validator, git consent, stash warning)

**Token saved vs full Phase 0:** ~3K tokens (git-related steps eliminated)

**Critical validations preserved:** Context files (Step 0.4), Tech stack (Step 0.7), Spec conflicts (Step 0.6)

---

### Step 1.0: Session Checkpoint Detection (AC#3 - STORY-120)

**Execute FIRST if RESUME_MODE == "auto" (before DoD analysis):**

```
IF RESUME_MODE == "auto":
  # Check for session checkpoint file
  checkpoint_path = "devforgeai/sessions/$STORY_ID/checkpoint.json"

  Bash(command="python3 -c '
from devforgeai_cli.session.checkpoint import read_checkpoint
import json
cp = read_checkpoint(\"$STORY_ID\")
if cp:
    print(f\"CHECKPOINT_FOUND|{cp[\"phase\"]}|{cp[\"phase_name\"]}|{cp[\"progress_percentage\"]}\")
else:
    print(\"NO_CHECKPOINT\")
'")

  IF output starts with "CHECKPOINT_FOUND":
    # Parse checkpoint data
    CHECKPOINT_PHASE = parsed[1]  # Last completed phase
    PHASE_NAME = parsed[2]
    PROGRESS = parsed[3]
    PHASE_NUM = CHECKPOINT_PHASE + 1  # Resume from NEXT phase

    Display: ""
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: "  ✓ SESSION CHECKPOINT DETECTED"
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: ""
    Display: "  Last checkpoint: Phase {CHECKPOINT_PHASE} ({PHASE_NAME})"
    Display: "  Progress: {PROGRESS}%"
    Display: "  Resuming from: Phase {PHASE_NUM}"
    Display: ""

    # Skip DoD analysis - checkpoint provides exact resumption point
    SKIP: "Step 1.1: Read DoD Section"
    GOTO: "Phase 2: Set Resume Context Markers"

  ELSE IF output contains "error" or "corrupt":
    Display: "⚠ Checkpoint corrupted - falling back to DoD analysis"
    Display: ""
    Continue to Step 1.1 (DoD analysis)

  ELSE:
    # No checkpoint file - AC#5 graceful fallback
    Display: "ℹ No session checkpoint found - analyzing DoD for resumption point"
    Display: ""
    Continue to Step 1.1 (DoD analysis)
```

**Why checkpoint-first:**
- Checkpoints are precise (exact phase number from /dev execution)
- DoD analysis is approximate (infers phase from incomplete items)
- Checkpoint provides faster resumption (no DoD parsing needed)

---

### Phase 2: Auto-Detect Resumption Point (If Applicable)

**Execute if RESUME_MODE == "auto" AND no checkpoint detected:**

**Step 1.1: Read DoD Section**
```
Grep(pattern="^## Definition of Done", path=story_file, output_mode="content", -A=100)

Extract DoD items by category:
  implementation_unchecked = count(Implementation section "[ ]" items)
  quality_unchecked = count(Quality section "[ ]" items)
  testing_unchecked = count(Testing section "[ ]" items)
  documentation_unchecked = count(Documentation section "[ ]" items)

  total_unchecked = sum(all categories)
  total_items = count(all DoD checkbox items)
  completion_pct = ((total_items - total_unchecked) / total_items) * 100
```

**Step 1.2: Determine Resumption Phase**
```
Display: ""
Display: "DoD Analysis:"
Display: "  Completion: {completion_pct}% ({total_unchecked} items remaining)"
Display: "  Implementation: {implementation_unchecked} incomplete"
Display: "  Quality: {quality_unchecked} incomplete"
Display: "  Testing: {testing_unchecked} incomplete"
Display: "  Documentation: {documentation_unchecked} incomplete"
Display: ""

IF total_unchecked == 0:
  Display: "✓ DoD 100% complete - no resumption needed"
  Display: "  Story appears finished. Run /qa instead?"
  HALT

IF implementation_unchecked > 0:
  PHASE_NUM=2
  PHASE_NAME="Phase 2: Implementation (Green Phase)"
  REASON="Implementation items incomplete"

ELSE IF quality_unchecked > 0:
  PHASE_NUM=3
  PHASE_NAME="Phase 3: Refactoring & Quality"
  REASON="Quality validations incomplete"

ELSE IF testing_unchecked > 0:
  PHASE_NUM=4
  PHASE_NAME="Phase 4: Integration Testing"
  REASON="Test coverage gaps"

ELSE IF documentation_unchecked > 0:
  PHASE_NUM=3
  PHASE_NAME="Phase 3: Documentation"
  REASON="Documentation incomplete"

Display: "Auto-detected resumption point: {PHASE_NAME}"
Display: "Reason: {REASON}"
Display: ""
```

---

### Phase 2: Set Resume Context Markers

**Set explicit context for skill to detect:**

```
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  DevForgeAI Development Workflow (RESUME MODE)"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""
Display: "**Story ID:** $STORY_ID"
Display: "**Resume from Phase:** $PHASE_NUM"
Display: "**Resume Mode:** $RESUME_MODE"
Display: ""

IF RESUME_MODE == "auto":
  Display: "**Auto-Detection Results:**"
  Display: "  DoD Completion: {completion_pct}%"
  Display: "  Remaining Work: {total_unchecked} items"
  Display: "  Resumption Point: {PHASE_NAME}"
  Display: ""

Display: "Resuming TDD workflow from Phase $PHASE_NUM..."
Display: ""
```

**These context markers allow devforgeai-development skill to:**
1. Extract "Resume from Phase: {N}" marker
2. Skip phases 0 through N-1
3. Start execution at Phase N
4. Continue normal workflow from there

---

### Phase 3: Invoke Skill

**Invoke devforgeai-development skill:**

```
Skill(command="devforgeai-development")
```

**Skill will:**
1. Execute Parameter Extraction (sees resume markers)
2. Set resume_mode = true, resume_from_phase = N
3. Skip phases 0 through N-1 (mark as "skipped" in TodoWrite)
4. Start at Phase N
5. Execute N → 4.5 → 4.5-5 Bridge → 4.5-R → check completion
6. If DoD <100%, Phase 4.5-R may resume again (iteration)
7. If DoD ==100%, proceed to Phase 5 (commit)

---

### Phase 4: Display Results

**Skill returns result to command.**

Command displays skill's result (no processing in command, lean orchestration).

---

## Use Cases

### Use Case 1: Fix Test Failures

**Scenario:** `/dev STORY-057` completed but 8 tests failing, user wants to fix and re-run testing phases

**Command:**
```bash
/resume-dev STORY-057 4
```

**Behavior:**
- Skips Phase 0-3 (pre-flight, tests generated, implementation done, refactored)
- Starts at Phase 4 (Integration Testing)
- Runs integration-tester with fixed code
- Continues to Phase 5 (commit if tests pass)

---

### Use Case 2: Complete Documentation

**Scenario:** Implementation and tests done, only documentation items remain

**Command:**
```bash
/resume-dev STORY-057 3
```

**Behavior:**
- Skips Phase 0-2 (pre-flight, tests, implementation)
- Starts at Phase 3 (Refactoring & Quality)
- code-reviewer validates documentation
- Continues to Phase 4-5 (integration, commit)

---

### Use Case 3: Auto-Detect Resumption

**Scenario:** User doesn't know which phase to resume, wants automatic detection

**Command:**
```bash
/resume-dev STORY-057
```

**Behavior:**
- Analyzes DoD: 18/30 items complete (60%)
- Finds: 5 implementation items unchecked
- Auto-detects: Resume from Phase 2 (Implementation)
- Displays: "Auto-detected: Phase 2 (Implementation incomplete)"
- Resumes from Phase 2

---

### Use Case 4: Second Run After Incomplete

**Scenario:** User ran `/dev STORY-057`, stopped at 87%, wants explicit resumption control

**Command:**
```bash
/resume-dev STORY-057 2
```

**Behavior:**
- More explicit than re-running `/dev STORY-057` (which starts from Phase 0)
- Skips pre-flight and test generation (already done)
- Starts fresh at Phase 2 with implementation work
- Faster (saves ~5 minutes by skipping Phase 0-1)

---

## Integration with devforgeai-development Skill

**Skill changes required** (already implemented in REC-1):

1. **Parameter Extraction** enhanced to detect resume markers
2. **Phase skip logic** implemented (mark phases 0 through N-1 as "skipped")
3. **GOTO Phase N** capability (start execution at specified phase)

**No changes needed in:**
- Individual phase implementations (Phases 0-7 unchanged)
- Subagent invocations (work same in resume mode)
- Quality gates (still enforced)

---

## Error Handling

**Error: Story already complete (DoD 100%)**
```
Display: "Story {STORY_ID} appears complete (DoD 100%)"
Display: "Status: {story_status}"
Display: ""
Display: "If you need to:"
Display: "  • Re-run tests: pytest tests/..."
Display: "  • Re-run QA: /qa {STORY_ID}"
Display: "  • Make changes: Edit files manually, then /dev {STORY_ID}"
Display: ""
HALT
```

**Error: Story not started (status: Backlog)**
```
Display: "Story {STORY_ID} not started (status: Backlog)"
Display: ""
Display: "Use: /dev {STORY_ID} to start development"
Display: "(Cannot resume what hasn't started)"
HALT
```

**Error: Invalid phase number**
```
Display: "Invalid phase: {PHASE_NUM}"
Display: "Valid: 0-7"
Display: "Use: /resume-dev {STORY_ID} [0-7]"
HALT
```

---

## Success Criteria

/resume-dev command successful when:
- [ ] Story ID validated
- [ ] Resume phase determined (manual or auto)
- [ ] Context markers set correctly
- [ ] devforgeai-development skill invoked
- [ ] Skill detects resume mode
- [ ] Skill skips phases 0 through N-1
- [ ] Skill starts at Phase N
- [ ] Workflow proceeds normally from there
- [ ] Character budget <5K (lean orchestration)

---

## Performance

**Token Budget:**
- Command overhead: ~2K tokens (parameter validation, context markers)
- Skill execution: Same as normal /dev (depends on phases executed)
- **Savings from skipping phases:** ~5-15K tokens (phases 0-1 can be skipped)

**Execution Time:**
- Manual mode: Immediate (no auto-detection)
- Auto mode: +30 seconds (DoD analysis)
- **Time saved:** Skipping Phase 0-1 saves ~3-5 minutes

**Character Budget:**
- Current: ~4,800 characters
- Target: <6K
- Hard limit: <15K
- Status: Well within budget ✓

---

## Examples

### Example 1: Resume from Implementation

```bash
$ /resume-dev STORY-057 2

✓ Story loaded: STORY-057
✓ Manual resumption: Phase 2

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  DevForgeAI Development Workflow (RESUME MODE)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**Story ID:** STORY-057
**Resume from Phase:** 2
**Resume Mode:** manual

Resuming TDD workflow from Phase 2...

[Skill executes Phase 2 → 3 → 4 → 4.5 → 5 → 6 → 7]
[Story reaches 100%, commits]
```

### Example 2: Auto-Detect Resumption

```bash
$ /resume-dev STORY-057

✓ Story loaded: STORY-057
Auto-detecting resumption point from DoD status...

DoD Analysis:
  Completion: 87% (7/30 complete, 23 remaining)
  Implementation: 6 incomplete
  Quality: 8 incomplete
  Testing: 7 incomplete
  Documentation: 2 incomplete

Auto-detected resumption point: Phase 2: Implementation (Green Phase)
Reason: Implementation items incomplete

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  DevForgeAI Development Workflow (RESUME MODE)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**Story ID:** STORY-057
**Resume from Phase:** 2
**Resume Mode:** auto

**Auto-Detection Results:**
  DoD Completion: 87%
  Remaining Work: 23 items
  Resumption Point: Phase 2: Implementation (Green Phase)

Resuming TDD workflow from Phase 2...

[Skill executes, completes remaining work]
```

---

## Comparison with /dev Command

| Aspect | /dev STORY-ID | /resume-dev STORY-ID 2 |
|--------|---------------|------------------------|
| **Phases executed** | 0-7 (all phases) | 2-7 (skips 0-1) |
| **Time** | ~20-30 minutes | ~15-20 minutes |
| **Use when** | Starting new story | Continuing incomplete story |
| **Skips** | None | Phases 0 through N-1 |
| **Token cost** | ~80-120K | ~60-90K (saves 20-30K) |

**When to use /resume-dev:**
- Previous `/dev` run was incomplete
- You know which phase needs to resume
- Want to skip pre-flight and test generation (already done)
- Want faster execution (skip completed phases)

**When to use /dev:**
- Starting new story
- Want full validation (including pre-flight)
- Not sure if previous work is still valid
- Prefer comprehensive workflow

---

## Related Commands

**Development workflow:**
- `/dev STORY-ID` - Full TDD workflow (Phase 0-7)
- `/resume-dev STORY-ID [phase]` - Resume from specific phase ← THIS COMMAND
- `/qa STORY-ID [mode]` - Quality validation only
- `/orchestrate STORY-ID` - Full lifecycle (dev + qa + release)

**Framework analysis:**
- `/rca [issue]` - Root cause analysis
- `/audit-deferrals` - Audit deferred work

---

## Integration Pattern

**Typical workflow:**

```
1. User starts story
   $ /dev STORY-057
   ↓
2. Workflow reaches 87%, user rejects deferrals
   ↓
3a. AUTOMATIC (REC-1): Phase 4.5-R resumes automatically
    [Loop back to Phase 2, continue to 100%]

3b. MANUAL (REC-2): User uses rewind command
   $ /resume-dev STORY-057 2
   [Skip Phase 0-1, resume from Phase 2]
```

**Both paths work:**
- REC-1 (automatic): No user action needed, workflow self-corrects
- REC-2 (manual): User control, faster (skips phases), explicit

---

## Character Budget Analysis

**Current implementation:**
- Lines: ~280
- Characters: ~10,500
- Budget: <15K (hard limit)
- Status: 70% of budget ✓

**Optimizations applied:**
- Lean orchestration (no business logic, delegates to skill)
- Minimal argument parsing (story ID + optional phase number)
- Context markers only (skill does heavy lifting)
- No result parsing (skill returns formatted display)

---

## Success Indicators

**When /resume-dev works correctly:**

1. ✓ User can manually resume from any phase (0-7)
2. ✓ Auto-detect correctly identifies resumption point
3. ✓ Phases 0 through N-1 skipped (marked "skipped" in TodoWrite)
4. ✓ Phase N starts fresh with prior implementation state loaded
5. ✓ Workflow proceeds normally after resumption
6. ✓ Iteration counter tracks multiple resumptions
7. ✓ Story completes to 100% after resumption

**User experience:**
- **Before:** Run `/dev STORY-057` → 87% → frustrated → run again → 87% again → very frustrated
- **After:** Run `/dev STORY-057` → 87% → run `/resume-dev STORY-057` → 100% → satisfied

---

**End of /resume-dev Command**

**Total:** ~280 lines
**Character count:** ~10,500 (70% of 15K budget)
**Created:** 2025-11-22 (RCA-013 Recommendation 2)
**Implements:** User-requested "rewind" capability
