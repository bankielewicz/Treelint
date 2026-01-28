# Orchestration Troubleshooting Guide

Common issues and solutions for the DevForgeAI orchestration skill.

## Purpose

**Quick reference for resolving orchestration problems:**
- Mode detection failures
- Checkpoint issues
- Quality gate blocks
- Skill invocation failures
- Status transition errors

**When to consult:** Orchestration HALTS, unexpected behavior, or error messages.

---

## Issue 1: Mode Detection Fails

### Symptom

Skill defaults to Story Management mode when Epic/Sprint mode expected, OR skill HALTS with "Cannot determine orchestration mode" error.

### Cause

Missing or malformed context markers in conversation.

### Diagnosis

```
Check conversation for markers:

Expected for Epic mode:
  **Command:** create-epic
  **Epic name:** {name}

Expected for Sprint mode:
  **Command:** create-sprint
  **Sprint Name:** {name}
  **Selected Stories:** {story-ids}

Expected for Story mode:
  **Story ID:** STORY-NNN
```

### Solution

**If markers missing:**
1. Add explicit context markers to conversation
2. Use exact format: `**Marker Name:** value` (bold markdown, colon, space)
3. Verify no typos in marker names (case-sensitive)

**If markers present but ignored:**
1. Check marker priority (Epic > Sprint > Story)
2. Remove conflicting markers (only include markers for intended mode)
3. Restart skill invocation with correct markers

**Reference:** See `mode-detection.md` for complete marker list

---

## Issue 2: Checkpoint Not Detected

### Symptom

Workflow restarts from beginning instead of resuming from checkpoint.

### Cause

Checkpoint markers not in story workflow history OR malformed checkpoint entry.

### Diagnosis

```
Read story file:
  Read(file_path="devforgeai/specs/Stories/{STORY_ID}.story.md")

Search workflow history section:
  Grep(pattern="Checkpoint: (DEV_COMPLETE|QA_APPROVED|STAGING_COMPLETE)")

Expected format:
  Checkpoint: DEV_COMPLETE
  Timestamp: 2025-01-06T14:23:45Z

IF not found:
  Checkpoint missing - workflow will start from beginning
```

### Solution

**If checkpoint missing:**
1. Verify previous phase completed successfully
2. Check if devforgeai-development/qa/release skills created checkpoint
3. Manually add checkpoint if workflow was completed outside orchestration

**If checkpoint malformed:**
```
Correct format:
Checkpoint: DEV_COMPLETE
Timestamp: 2025-01-06T14:23:45Z

Incorrect formats (will not detect):
- "Checkpoint - DEV_COMPLETE" ❌ (wrong separator)
- "checkpoint: dev_complete" ❌ (case sensitive)
- "DEV_COMPLETE checkpoint" ❌ (wrong order)
```

**Manual fix:**
```
Edit(
  file_path="devforgeai/specs/Stories/{STORY_ID}.story.md",
  old_string="[malformed checkpoint]",
  new_string="Checkpoint: DEV_COMPLETE\nTimestamp: 2025-01-06T14:23:45Z"
)
```

**Reference:** See `checkpoint-detection.md` for checkpoint types and formats

---

## Issue 3: Quality Gate Blocks Progression

### Symptom

Story cannot advance to next status, orchestration HALTS with quality gate error.

### Cause

Quality gate requirements not met for attempted transition.

### Diagnosis

**Check which gate is blocking:**

```
Error message will indicate:
"Quality Gate {number} blocks progression"

Gate 1: Context Validation (Architecture → Ready for Dev)
Gate 2: Test Passing (Dev Complete → QA In Progress)
Gate 3: QA Approval (QA Approved → Releasing)
Gate 4: Release Readiness (Releasing → Released)
```

### Solution

**Gate 1 (Context Validation):**
```
Requirements:
- All 6 context files exist
- No placeholder content (TODO, TBD)

Fix:
1. Run: /create-context {project-name}
2. Verify all 6 files created:
   Glob(pattern="devforgeai/specs/context/*.md")
3. Check for placeholders:
   Grep(pattern="TODO|TBD", path="devforgeai/specs/context/")
4. Replace placeholders with actual content
```

**Gate 2 (Test Passing):**
```
Requirements:
- Build succeeds
- All tests pass (100% pass rate)
- Light QA validation passed

Fix:
1. Run build: {build_command}
2. Fix compilation errors
3. Run tests: {test_command}
4. Fix failing tests
5. Run /dev {STORY_ID} to complete TDD workflow
```

**Gate 3 (QA Approval):**
```
Requirements:
- Deep QA validation PASSED
- Coverage thresholds met (95%/85%/80%)
- Zero CRITICAL/HIGH violations

Fix:
1. Review QA report: devforgeai/qa/reports/{STORY_ID}-qa-report.md
2. Fix violations (coverage, anti-patterns, spec compliance)
3. Run /dev {STORY_ID} to apply fixes
4. Run /orchestrate {STORY_ID} to retry QA
```

**Gate 4 (Release Readiness):**
```
Requirements:
- QA approved
- Staging deployment complete
- Smoke tests passed

Fix:
1. Ensure QA_APPROVED checkpoint exists
2. Verify staging deployment succeeded
3. Check smoke test results
4. Resolve deployment issues if needed
```

**Reference:** See `quality-gates.md` for complete gate requirements

---

## Issue 4: QA Retry Loop Exceeds Max Attempts

### Symptom

QA failed 3 times, workflow halted with "Maximum Retry Attempts Exceeded" error.

### Cause

Persistent issues not resolved between attempts OR story scope too large.

### Diagnosis

```
Read story workflow history:
  Grep(pattern="QA Attempt [0-9]+")
  Count: 3 attempts

Review all 3 QA reports:
  Read(devforgeai/qa/reports/{STORY_ID}-qa-report.md)

Analyze pattern:
  - Same issues recurring? → Systemic problem
  - Different issues each time? → Story too complex
  - Coverage gaps moving around? → Story too large
```

### Solution

**Option 1: Split Story (Recommended)**
```
Story: STORY-042 (8 points)
Split into:
  - STORY-046: Core functionality (3 points)
  - STORY-047: Secondary feature (3 points)
  - STORY-048: Edge cases (2 points)

Rationale:
- Smaller stories easier to test comprehensively
- Each story focused on single responsibility
- Reduces complexity per story
```

**Option 2: Address Root Cause**
```
Review all 3 attempt reports:
1. Identify root cause (not symptoms)
2. Fix underlying issue (not band-aid)
3. Run /dev {STORY_ID} with comprehensive fix
4. Start fresh orchestration (resets attempt count)
```

**Option 3: Escalate**
```
If issues are:
- External blockers (APIs not ready)
- Infrastructure problems (environments unstable)
- Team capacity issues (lack of expertise)

Action:
1. Document blockers
2. Escalate to tech lead or product owner
3. De-prioritize story until blockers resolved
```

**Reference:** See `qa-retry-workflow.md` for retry logic and max attempts

---

## Issue 5: Sprint Planning Capacity Exceeded

### Symptom

Sprint planning warns about over-capacity (selected stories >40 points).

### Cause

Too many stories selected for sprint, exceeds recommended 20-40 point range.

### Diagnosis

```
Calculate total points:
  selected_stories = [STORY-001 (5pts), STORY-002 (8pts), STORY-003 (12pts), ...]
  total_points = sum(story.points for story in selected_stories)

Recommended range: 20-40 points
Actual: {total_points} points
```

### Solution

**Remove lower-priority stories:**
```
Sort stories by priority:
  High > Medium > Low

Remove low-priority stories until total ≤40 points

Defer removed stories to next sprint:
  Move to backlog, add note: "Deferred from Sprint-3 due to capacity"
```

**OR adjust story estimates:**
```
If estimates seem high:
1. Re-estimate stories (may be over-estimated)
2. Break large stories into smaller ones
3. Move complex stories to future sprint
```

**Reference:** See `sprint-planning-guide.md` for capacity guidelines

---

## Issue 6: Skill Invocation Fails

### Symptom

Orchestration invokes skill (devforgeai-development, qa, release) but skill returns error.

### Cause

Skill execution error (not validation failure, but execution problem).

### Diagnosis

```
Read skill error message from orchestration output

Common skill errors:
- "Story file not found" → Story ID wrong or file missing
- "Context files missing" → Architecture not run first
- "Prerequisites not met" → Quality gate blocking
- "Execution error: {details}" → Skill internal error
```

### Solution

**Story file not found:**
```
Fix:
1. Verify story ID spelling: STORY-042 (not STORY-42 or STORY-0042)
2. Check file exists: Glob(pattern="devforgeai/specs/Stories/{STORY_ID}*.md")
3. Create story if missing: /create-story {description}
```

**Context files missing:**
```
Fix:
1. Run: /create-context {project-name}
2. Verify 6 files created:
   Glob(pattern="devforgeai/specs/context/*.md")
   Expected: tech-stack, source-tree, dependencies, coding-standards, architecture-constraints, anti-patterns
```

**Prerequisites not met:**
```
Fix:
1. Check which quality gate blocking
2. Review gate requirements: quality-gates.md
3. Fix requirements, retry transition
```

**Skill internal error:**
```
Action:
1. Read skill error details
2. Check skill-specific logs/reports
3. Verify skill has required tools (allowed-tools)
4. Check for skill bugs (report if framework issue)
```

---

## Issue 7: Invalid Status Transition

### Symptom

Skill attempts to update story status but transition is invalid.

### Cause

Attempted transition violates state transition rules (skipping states, invalid progression).

### Diagnosis

```
Error message shows:
"Invalid transition: {current_status} → {attempted_status}"

Check valid transitions:
  Read(.claude/skills/devforgeai-orchestration/references/state-transitions.md)

Example invalid transitions:
- Backlog → Dev Complete ❌ (skips Architecture, Ready for Dev, In Development)
- Released → In Development ❌ (backward without reason)
- QA Approved → Backlog ❌ (backward without reason)
```

### Solution

**If skipping states:**
```
Fix:
1. Progress through states sequentially
2. Don't skip: Use each status in order
3. Example correct flow:
   Backlog → Architecture → Ready for Dev → In Development → Dev Complete → QA In Progress → QA Approved → Releasing → Released
```

**If backward transition:**
```
Valid backward transitions:
- QA Failed → In Development (to fix issues)
- Dev Complete → In Development (to rework)

Invalid backward transitions:
- Released → Anything (story complete, create new story instead)
- QA Approved → Anything except Releasing (already validated)

Fix:
1. Create new story for rework (don't revert Released story)
2. OR use valid backward transition (QA Failed → Dev)
```

**Reference:** See `state-transitions.md` for all valid transitions

---

## Issue 8: Story File Malformed

### Symptom

Orchestration loads story but fails validation with "Invalid story format" error.

### Cause

Story file missing required sections or YAML frontmatter malformed.

### Diagnosis

```
Read story file:
  Read(file_path="devforgeai/specs/Stories/{STORY_ID}.story.md")

Check for:
- YAML frontmatter (lines 1-X, starts with ---, ends with ---)
- Required fields: id, title, status, points, priority
- Required sections: User Story, Acceptance Criteria, Technical Specification, Definition of Done, Workflow Status
```

### Solution

**Missing YAML frontmatter:**
```
Story must begin with:
---
id: STORY-042
title: {title}
status: Backlog
points: 5
priority: High
---

Fix:
1. Edit story file
2. Add YAML frontmatter at top
3. Ensure proper format (---, fields, ---)
```

**Missing required sections:**
```
Required sections:
## User Story
## Acceptance Criteria
## Technical Specification
## Non-Functional Requirements
## Definition of Done
## Workflow Status

Fix:
1. Use /create-story to regenerate with proper template
2. OR manually add missing sections using story-template.md
```

**Reference:** See `story-validation.md` for complete validation rules

---

## Issue 9: Deployment Fails (Staging or Production)

### Symptom

Release phase fails with deployment error.

### Cause

Infrastructure issues, configuration problems, or smoke test failures.

### Diagnosis

```
Read deployment logs:
  Check release skill output for error details

Common issues:
- "Connection refused" → Infrastructure not accessible
- "Smoke tests failed" → Deployed code has issues
- "Health checks failing" → Application not starting correctly
```

### Solution

**Infrastructure issues:**
```
Fix:
1. Verify environments accessible (staging/production URLs)
2. Check infrastructure status (cloud console, kubectl)
3. Ensure deployment credentials configured
4. Verify network connectivity
```

**Smoke test failures:**
```
Fix:
1. Review smoke test output (what failed?)
2. Fix application issues causing test failures
3. Test locally before re-deploying
4. Run /dev {STORY_ID} to fix code issues
5. Retry: /orchestrate {STORY_ID} (will resume from checkpoint)
```

**Health check failures:**
```
Fix:
1. Check application logs for startup errors
2. Verify database connections
3. Check environment variable configuration
4. Ensure all dependencies available
```

**Reference:** See devforgeai-release skill documentation

---

## Issue 10: Epic Validation Fails

### Symptom

Epic creation fails validation with "Epic validation failed" error.

### Cause

Missing required epic metadata, malformed epic file, or validation check failures.

### Diagnosis

```
Check validation error details:

Common failures:
- "Missing success criteria" → <3 success criteria defined
- "Missing stakeholders" → <3 stakeholders identified
- "Circular dependencies" → Features depend on each other in circle
- "Over-scoped" → >8 features in epic
- "Under-scoped" → <3 features in epic
```

### Solution

**Missing success criteria:**
```
Fix:
1. Define 3+ SMART success criteria
2. Ensure criteria are measurable (not vague like "improve UX")
3. Example: "Reduce login time from 3s to <1s"
```

**Circular dependencies:**
```
Fix:
1. Review feature dependency graph
2. Break circular chain (redesign features)
3. Option A: Merge dependent features into one
4. Option B: Introduce intermediary feature
5. Option C: Remove dependencies (make features independent)
```

**Scoping issues:**
```
Over-scoped (>8 features):
  Split into multiple epics (3-8 features each)

Under-scoped (<3 features):
  Expand scope OR create stories directly (not epic)
```

**Reference:** See `epic-validation-checklist.md` for all validation rules

---

## Issue 11: Context Markers Extracted Incorrectly

### Symptom

Skill extracts wrong values from context markers (story ID, mode, etc.).

### Cause

Marker format doesn't match expected pattern OR multiple markers conflict.

### Diagnosis

```
Skill searches for:
  Pattern: \*\*Story ID:\*\*\s+STORY-[0-9]+
  Pattern: \*\*Command:\*\*\s+(create-epic|create-sprint)

Check conversation for exact format:
  **Story ID:** STORY-042 ✅
  ** Story ID:** STORY-042 ❌ (extra space before marker)
  **StoryID:** STORY-042 ❌ (no space in marker name)
  Story ID: STORY-042 ❌ (not bold markdown)
```

### Solution

**Use exact format:**
```
Correct:
  **Story ID:** STORY-042
  **Command:** create-epic
  **Epic name:** User Authentication

Incorrect:
  Story ID: STORY-042 (not bold)
  **Story-ID:** STORY-042 (hyphen instead of space)
  **story id:** STORY-042 (lowercase)
```

**Check for conflicts:**
```
Don't mix markers for different modes:
  **Command:** create-epic
  **Story ID:** STORY-042

↑ Conflicting (Epic mode vs Story mode)

Remove one set of markers
```

---

## Issue 12: Workflow History Malformed

### Symptom

Cannot parse workflow history, checkpoint detection fails, or status updates fail.

### Cause

Workflow history section missing, malformed, or has invalid format.

### Diagnosis

```
Read story file:
  Read(file_path="devforgeai/specs/Stories/{STORY_ID}.story.md")

Expected section:
  ## Workflow Status

  ### 2025-01-06T14:23:45Z - Dev Complete
  **Previous Status:** In Development
  ...

IF "## Workflow Status" section missing:
  Story missing workflow history → Validation will fail
```

### Solution

**Add workflow history section:**
```
Edit story file, add:

## Workflow Status

### {timestamp} - {current_status}
**Previous Status:** Backlog
**Action Taken:** Story created
**Result:** Story in backlog, ready for sprint planning
**Next Steps:** Assign to sprint, then begin development

(Place before any existing history entries)
```

**Fix malformed entries:**
```
Correct format:
### {ISO 8601 timestamp} - {Status}
**Previous Status:** {old_status}
**Action Taken:** {description}
**Result:** {outcome}
**Next Steps:** {recommendations}

Checkpoint: {CHECKPOINT_NAME} (if applicable)
Timestamp: {ISO 8601}
```

**Reference:** See `story-status-update.md` for workflow history format

---

## Issue 13: Subagent Invocation Fails

### Symptom

Orchestration invokes subagent (requirements-analyst, sprint-planner, etc.) but subagent fails.

### Cause

Subagent missing required context, tools not accessible, or framework constraints violated.

### Diagnosis

```
Check subagent error message:

Common errors:
- "Cannot read context file" → File missing or permission issue
- "Framework constraint violation" → Violates tech-stack.md or anti-patterns.md
- "Invalid input format" → Prompt malformed or missing required parameters
```

### Solution

**Context file missing:**
```
Fix:
1. Ensure all 6 context files exist: /create-context
2. Verify subagent has Read tool access
3. Check file permissions (should be readable)
```

**Framework violation:**
```
Fix:
1. Review violated constraint (tech-stack.md, anti-patterns.md, etc.)
2. Adjust proposal to comply with constraint
3. OR use AskUserQuestion to request exception (requires ADR)
```

**Invalid input:**
```
Fix:
1. Check subagent prompt includes all required information
2. Verify context markers present in conversation
3. Add missing context before invoking subagent
```

---

## Debugging Tips

### 1. Check Context Markers

**Use Grep to search conversation for markers:**
```
Grep(pattern="\\*\\*Story ID:\\*\\*")
Grep(pattern="\\*\\*Command:\\*\\*")
Grep(pattern="\\*\\*Epic name:\\*\\*")
```

**Verify exact format** (bold markdown, colon, space, value)

---

### 2. Verify File Structure

**Ensure story file has all required sections:**
```
Glob(pattern="devforgeai/specs/Stories/{STORY_ID}*.md")
Read(file_path="{story_file}")

Check for:
- YAML frontmatter (---)
- ## User Story
- ## Acceptance Criteria
- ## Definition of Done
- ## Workflow Status
```

---

### 3. Review Workflow History

**Check story Status History for clues:**
```
Read(file_path="devforgeai/specs/Stories/{STORY_ID}.story.md")

Look for:
- Checkpoints (DEV_COMPLETE, QA_APPROVED, etc.)
- Previous errors or failures
- QA attempt history
- Status transition log
```

---

### 4. Read Reference Files

**Most issues documented in phase-specific references:**
- Mode detection issues → `mode-detection.md`
- Checkpoint issues → `checkpoint-detection.md`
- Quality gate blocks → `quality-gates.md`
- QA retry issues → `qa-retry-workflow.md`
- Status transition errors → `state-transitions.md`

---

### 5. Test Mode Detection

**Try explicit markers if auto-detection fails:**
```
Instead of relying on inference:
  Epic mentioned in conversation

Use explicit markers:
  **Command:** create-epic
  **Epic name:** User Authentication System
```

**Force specific mode** to bypass auto-detection issues.

---

## Getting Help

**If issue persists after troubleshooting:**

1. **Document current state:**
   - Story ID and status
   - Checkpoints in workflow history
   - Error messages (exact text)
   - What you tried so far

2. **Review relevant reference file:**
   - Find reference for failing phase
   - Check for known issues or solutions
   - Verify your approach matches documentation

3. **Check quality gates:**
   - If blocked at gate, review `quality-gates.md`
   - Verify all gate requirements met
   - Fix requirements before retrying

4. **Use AskUserQuestion:**
   - Request user guidance if truly stuck
   - Present options based on situation
   - Let user make final decision

5. **Check framework memory files:**
   - `.claude/memory/skills-reference.md` - Skill usage
   - `.claude/memory/subagents-reference.md` - Subagent reference
   - `.claude/memory/commands-reference.md` - Command syntax
   - `CLAUDE.md` - Framework overview

---

## Common Error Messages

### "Cannot determine orchestration mode"
**Fix:** Add explicit context markers (see Issue 1)

### "Quality Gate X blocks progression"
**Fix:** Review gate requirements, fix issues (see Issue 3)

### "QA Max Retries Exceeded"
**Fix:** Split story or address root cause (see Issue 4)

### "Story file not found"
**Fix:** Verify story ID, create story if missing (see Issue 6)

### "Checkpoint not detected"
**Fix:** Verify checkpoint format in workflow history (see Issue 2)

### "Sprint capacity exceeded"
**Fix:** Remove stories or adjust estimates (see Issue 5)

### "Invalid story format"
**Fix:** Add missing YAML or sections (see Issue 8)

### "Skill invocation failed"
**Fix:** Check skill error, verify prerequisites (see Issue 6)

---

## Quick Diagnostic Checklist

**When orchestration fails, check these in order:**

1. [ ] Context markers present and formatted correctly?
2. [ ] Story file exists and has all required sections?
3. [ ] YAML frontmatter valid with required fields?
4. [ ] Quality gate requirements met?
5. [ ] Checkpoints formatted correctly (if resuming)?
6. [ ] Context files exist (all 6)?
7. [ ] Story status is valid workflow state?
8. [ ] No circular deferrals detected?
9. [ ] Sprint capacity within 20-40 points?
10. [ ] Subagents have required tools and context?

**If all checked and still failing:** Review error message, consult phase-specific reference file.

---

## Related Files

- **mode-detection.md** - Issue 1 (mode detection)
- **checkpoint-detection.md** - Issue 2 (checkpoint problems)
- **quality-gates.md** - Issue 3 (gate blocks)
- **qa-retry-workflow.md** - Issue 4 (max retries)
- **sprint-planning-guide.md** - Issue 5 (capacity)
- **story-validation.md** - Issue 8 (malformed stories)
- **state-transitions.md** - Issue 7 (invalid transitions)
- **skill-invocation.md** - Issue 6 (skill failures)
