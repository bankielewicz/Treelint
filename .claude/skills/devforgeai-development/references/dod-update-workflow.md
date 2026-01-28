# DoD Update Workflow (Phase 07)

**Purpose:** Update Definition of Done items after validation and prepare for git commit with correct formatting

**Execution:** After Phase 06 (Deferral Challenge) completes, BEFORE Phase 08 (Git Commit)

**Why This Bridge Exists:** Phase 06 validates deferral semantics (via deferral-validator AI subagent), but Phase 08 git commit requires DoD format compliance (via devforgeai-validate validate-dod CLI validator). This bridge ensures both validators' requirements are met.

**Token Cost:** ~1,500 tokens (loaded on-demand after Phase 06)

**Template File:** See `.claude/skills/devforgeai-development/assets/templates/implementation-notes-template.md` for the minimal Implementation Notes template format (≤25 lines).

---

## Phase Progress Indicator

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 07/9: Update DoD Checkboxes (67% → 78% complete)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Display this indicator at the start of Phase 07 execution.**

---

## Overview: Two Validators, Different Requirements

### Validator 1: deferral-validator (AI Subagent)
- **Runs:** Phase 06 Step 3
- **Purpose:** Semantic validation (are deferrals justified? circular chains?)
- **Checks:** Blocker validity, story references exist, ADR references exist
- **Output:** Advisory (resolvable vs. valid deferrals)
- **Cannot HALT:** Provides recommendations only

### Validator 2: devforgeai-validate validate-dod (CLI Pre-Commit Hook)
- **Runs:** Phase 08 during git commit
- **Purpose:** Format validation (DoD items in Implementation Notes?)
- **Checks:** Text match between DoD section and Implementation Notes section
- **Output:** PASS/FAIL (blocks commit if FAIL)
- **Can HALT:** Prevents git commit with exit code 1

### Why Both Are Needed

**deferral-validator:** Understands semantics ("Is this blocker still valid?")
**devforgeai-validate validate-dod:** Enforces format ("Are completed items documented in Implementation Notes?")

**This bridge workflow ensures BOTH validators pass.**

---

## Step 1: Mark Completed Items in Definition of Done Section

**For each DoD item that was IMPLEMENTED during TDD workflow:**

### 1.1: Read Current DoD Section

```
Read(file_path="${STORY_FILE}")

# Extract Definition of Done section
# Look for lines starting with "- [ ]" (unchecked items)
```

### 1.2: Identify Completed Items

**Completed items are those where:**
- Implementation code written (Phase 03)
- Tests passing (Phase 05)
- Not deferred to another story
- Actually implemented in this story's scope

**Example:**
```
DoD Section (BEFORE):
- [ ] Hook integration phase added to /create-story command
- [ ] Hook check executes in <100ms
- [ ] All 6 acceptance criteria have passing tests

Implementation Reality (AFTER TDD):
- ✅ Hook integration implemented (Phase 03)
- ✅ Performance validated (Phase 05 - actual: 50ms p95)
- ✅ All 6 ACs tested (Phase 02 - 62 tests generated)
```

### 1.3: Mark Items Complete with Edit Tool

**For each completed item, use Edit to mark [x] and add completion note:**

```
Edit(
  file_path="${STORY_FILE}",
  old_string="- [ ] Hook integration phase added to /create-story command (Phase N after story file creation)",
  new_string="- [x] Hook integration phase added to /create-story command (Phase N after story file creation) - Completed: Phase 08 added to .claude/commands/create-story.md with full hook integration workflow"
)
```

**Completion Note Format:**
```
- [x] {item_text} - Completed: {what_was_done}
```

**Examples:**
```
- [x] Hook check executes in <100ms - Completed: Hook check executes in ~50ms p95 (50% better than target), verified by 5 performance tests

- [x] All 6 acceptance criteria have passing tests - Completed: 62 tests covering all 6 ACs (100% coverage)

- [x] Code coverage >95% - Completed: 69 comprehensive tests covering all integration logic paths
```

---

## Step 2: Add DoD Status to Implementation Notes

**CRITICAL FORMAT REQUIREMENT:** Items must be DIRECTLY under `## Implementation Notes`, NOT under a `### Definition of Done Status` subsection.

**⚠️ KEY LESSON LEARNED (STORY-041, 2025-11-18):**
The DoD validator expects Implementation Notes to be a **flat checklist** directly under `## Implementation Notes`, not nested under subsections like `### Completed DoD Items` or `### Definition of Done Status`. The validator's `extract_section()` function stops at the FIRST `###` header, so any items under subsections are NOT FOUND and will cause commit failures.

### 2.1: Why Format Matters

**The pre-commit validator uses `extract_section()` function:**

```python
# extract_section(content, "Implementation Notes")
# Returns: Content from "## Implementation Notes" up to NEXT "##" or "###" header

# This function STOPS at the FIRST ### subsection header!
```

**What this means:**
```markdown
## Implementation Notes

**Developer:** DevForgeAI
**Implemented:** 2025-11-14

### Definition of Done Status  ← VALIDATOR STOPS HERE!
- [x] Item 1 - Completed: ...   ← NOT EXTRACTED (under ### header)
- [x] Item 2 - Completed: ...   ← NOT EXTRACTED

Result: extract_section() returns only "**Developer:**..." (155 chars)
Validator sees: 0 DoD items in Implementation Notes
Git commit: ❌ BLOCKED
```

**Correct format:**
```markdown
## Implementation Notes

**Developer:** DevForgeAI
**Implemented:** 2025-11-14

- [x] Item 1 - Completed: ...   ← EXTRACTED (directly under ##)
- [x] Item 2 - Completed: ...   ← EXTRACTED
- [x] Item 3 - Completed: ...   ← EXTRACTED

### TDD Workflow Summary  ← Subsections AFTER DoD items OK
...

Result: extract_section() returns all items
Validator sees: All DoD items present
Git commit: ✅ PASSES
```

### 2.2: Implementation Steps

**Read story file to find Implementation Notes section:**

```
Read(file_path="${STORY_FILE}")

# Find ## Implementation Notes section
# Identify where to insert DoD items (after developer/commit info, before first ### subsection)
```

**Locate insertion point:**

```
## Implementation Notes

**Developer:** DevForgeAI AI Agent
**Implemented:** 2025-11-14
**Commit:** {SHA}
**Branch:** {branch}

← INSERT DoD ITEMS HERE (before any ### subsections)

### TDD Workflow Summary
...
```

**Insert all completed DoD items:**

```
# Build DoD items list from Definition of Done section
dod_items_list = """
- [x] Hook integration phase added to /create-story command (Phase N after story file creation) - Completed: Phase 08 added to .claude/commands/create-story.md with full hook integration workflow
- [x] Hook check executes in <100ms - Completed: Hook check executes in ~50ms p95 (50% better than target), verified by 5 performance tests
... [all completed items]
"""

# Insert after developer info, before first ### subsection
Edit(
  file_path="${STORY_FILE}",
  old_string="**Branch:** {branch}\n\n### TDD Workflow Summary",
  new_string="**Branch:** {branch}\n\n${dod_items_list}\n\n### TDD Workflow Summary"
)
```

**If no ### subsections exist yet:**

```
Edit(
  file_path="${STORY_FILE}",
  old_string="**Branch:** {branch}\n\n## Change Log",
  new_string="**Branch:** {branch}\n\n${dod_items_list}\n\n## Change Log"
)
```

---

## Step 3: Validate Format Before Git Commit

**MANDATORY:** Run devforgeai-validate validate-dod BEFORE attempting git commit

### 3.1: Run Validator

```
Bash(command="devforgeai-validate validate-dod ${STORY_FILE}")
```

**Expected Output:**
```
✅ {STORY_FILE}: All DoD items validated
```

### 3.2: Handle Validation Failures

**If validator returns exit code 1:**

```
Display: "❌ DoD format validation FAILED"
Display: ""
Display: "The pre-commit hook will block git commit until format is correct."
Display: ""
Display: "Review validator output above for specific errors."
Display: ""
Display: "Common issues:"
Display: "  1. DoD items under ### subsection (move to flat list under ##)"
Display: "  2. Text mismatch between DoD and Implementation Notes"
Display: "  3. Missing Implementation Notes section"
Display: ""
Display: "Fix issues and re-run: devforgeai-validate validate-dod ${STORY_FILE}"
Display: ""

HALT workflow
Do NOT proceed to git commit
```

### 3.3: Validation Success

**If validator returns exit code 0:**

```
Display: "✅ DoD format validation PASSED"
Display: "Ready for Phase 08 git commit"
```

**Proceed to Step 4**

---

## Step 4: Update Change Log Section

**Append Development phase complete entry to ## Change Log:**

```
Read(file_path="${STORY_FILE}")

# Find ## Change Log section
# Locate last table entry

# Append changelog entry for DoD Update phase
Edit(
  file_path="${STORY_FILE}",
  old_string="| {last_date} | {last_author} | {last_action} | {last_change} | {last_files} |",
  new_string="| {last_date} | {last_author} | {last_action} | {last_change} | {last_files} |\n| {current_date} | claude/opus | DoD Update (Phase 07) | Development complete, DoD validated | ${STORY_FILE} |"
)

# Update **Current Status:** field
Edit(
  file_path="${STORY_FILE}",
  old_string="**Current Status:** In Development",
  new_string="**Current Status:** Dev Complete"
)
```

**Note:** The unified `## Change Log` section replaces the deprecated Workflow Status section as of STORY-152 (template v2.5).

---

## Step 5: Add TDD Workflow Summary (Optional but Recommended)

**If Implementation Notes doesn't have TDD workflow summary yet:**

```
# After DoD items, before Change Log section, add:

### TDD Workflow Summary

**Phase 02 (Red): Test-First Design**
- Generated {test_count} comprehensive tests covering all {ac_count} acceptance criteria
- Tests placed in tests/unit/, tests/integration/, tests/e2e/
- All tests follow AAA pattern (Arrange/Act/Assert)
- Test frameworks: {TEST_FRAMEWORK}

**Phase 03 (Green): Implementation**
- Implemented minimal code to pass tests via {backend-architect/frontend-developer} subagent
- {implementation_summary}
- All {test_count} tests passing (100% pass rate)

**Phase 04 (Refactor): Code Quality**
- Code quality improved (complexity reduction, naming clarity, etc.)
- {refactoring_summary}
- All tests remain green after refactoring

**Phase 05 (Integration): Full Validation**
- Full test suite executed
- Performance verified: {metrics}
- Reliability verified
- No regressions introduced

**Phase 06 (Deferral Challenge): DoD Validation**
- All Definition of Done items validated
- {deferred_count} deferrals (if any) with user approval
- No blockers detected

**Phase 08 (Git Workflow): Version Control**
- Changes committed ({files_count} files, {insertions} insertions)
- Story status updated to "Dev Complete"
- Pre-commit validation passed

### Files Created/Modified

**Modified:**
- {list of modified files}

**Created:**
- {list of created files}

### Test Results

- **Total tests:** {count}
- **Pass rate:** {percentage}%
- **Coverage:** {percentage}% for {layer}
- **Execution time:** {seconds} seconds
```

---

## Success Criteria

**This bridge workflow succeeds when:**

- [ ] All completed DoD items marked [x] in Definition of Done section
- [ ] All completed DoD items added to Implementation Notes (flat list, directly under ##, no ### subsection)
- [ ] devforgeai-validate validate-dod passes (exit code 0)
- [ ] Change Log section updated (Current Status, new entry appended)
- [ ] Implementation Notes contains developer info (name, date, commit, branch)
- [ ] Optional: TDD Workflow Summary added for traceability
- [ ] Ready for Phase 08 git commit (no format blockers)

---

## Common Errors and Fixes

### Error 1: "DoD item marked [x] but missing from Implementation Notes"

**Cause:** Items not added to Implementation Notes, or under ### subsection

**Fix:**
1. Ensure items are DIRECTLY under `## Implementation Notes`
2. NOT under `### Definition of Done Status` subsection
3. Items should appear AFTER developer info, BEFORE any ### subsections

**Example fix:**
```markdown
## Implementation Notes

**Developer:** ...
**Commit:** ...

- [x] Item 1 - Completed: ...  ← Correct (directly under ##)
- [x] Item 2 - Completed: ...

### TDD Workflow Summary  ← Subsections AFTER items
```

---

### Error 2: "Text mismatch between DoD and Implementation Notes"

**Cause:** Item text doesn't match exactly (typo, different wording)

**Fix:**
1. Copy EXACT text from Definition of Done section
2. Paste into Implementation Notes
3. Add " - Completed: ..." suffix
4. Ensure backticks, quotes, special chars match exactly

**Example:**
```
DoD: - [x] `devforgeai-validate check-hooks` command functional
Impl: - [x] `devforgeai-validate check-hooks` command functional - Completed: ...
       ↑ Backticks must match exactly
```

---

### Error 3: "Implementation Notes section missing"

**Cause:** Story file doesn't have `## Implementation Notes` section

**Fix:**
```
# Add section after Definition of Done, before Change Log

Edit(
  file_path="${STORY_FILE}",
  old_string="## Change Log",
  new_string="## Implementation Notes\n\n**Developer:** DevForgeAI AI Agent\n**Implemented:** {date}\n**Commit:** {SHA}\n**Branch:** {branch}\n\n{dod_items_list}\n\n## Change Log"
)
```

---

### Error 4: Validator passes locally but fails in pre-commit hook

**Cause:** Story file changes not saved, or reading cached version

**Fix:**
1. Ensure Edit operations completed successfully
2. Re-read story file to verify changes applied
3. Run validator again: `devforgeai-validate validate-dod ${STORY_FILE}`
4. Only proceed to git commit if exit code 0

---

## Workflow Diagram

```
Phase 06: Deferral Challenge
       ↓
   Validation passed?
       ↓
┌──────────────────────────────────────────┐
│  Phase 07: DoD Update          │
│                                          │
│  Step 1: Mark DoD items [x]              │
│          ↓                                │
│  Step 2: Add items to Implementation     │
│          Notes (FLAT LIST)               │
│          ↓                                │
│  Step 3: Validate format                 │
│          (devforgeai-validate validate-dod)       │
│          ↓                                │
│  Step 4: Update Change Log               │
│          ↓                                │
│  Step 5: Optional TDD Summary            │
└──────────────────────────────────────────┘
       ↓
   Format validated?
       ↓ YES
Phase 08: Git Workflow
   (git commit will pass)
       ↓
   Story Status = "Dev Complete"
```

---

## Integration with Other Workflows

### Phase 06 Handoff

**At end of phase-4.5-deferral-challenge.md:**

```markdown
## Phase 06 Complete

All deferrals validated. Now update DoD format for git commit.

**Next:** Load and execute dod-update-workflow.md (Phase 07)

Read(file_path=".claude/skills/devforgeai-development/references/dod-update-workflow.md")
```

### Phase 08 Pre-Requisites

**At beginning of git-workflow-conventions.md (Phase 08):**

```markdown
## Pre-Requisites for Phase 08

Before executing git commit workflow, ensure:

- [ ] Phase 06 (Deferral Challenge) complete
- [ ] DoD Update Workflow (dod-update-workflow.md) executed
- [ ] devforgeai-validate validate-dod passes (exit code 0)

If ANY prerequisite fails, DO NOT proceed to git commit.

**See:** dod-update-workflow.md for DoD formatting requirements
```

---

## Example: Complete DoD Update Flow

### Scenario: STORY-027 with 22 DoD Items (All Complete)

**Input State (After Phase 06):**
```yaml
Definition of Done:
  - [ ] Hook integration phase added...
  - [ ] Hook check executes in <100ms...
  - [ ] All 6 acceptance criteria have passing tests...
  [19 more items, all unchecked]

Implementation Notes:
  ## Implementation Notes

  **This story wires hook integration...**
```

**Step 1: Mark completed items [x] in DoD section**

```
Edit 22 times (one per item):
- [ ] Item → - [x] Item - Completed: {description}
```

**Step 2: Add items to Implementation Notes (flat list)**

```
Edit(
  old_string="**This story wires hook integration...",
  new_string="**Developer:** DevForgeAI AI Agent
**Implemented:** 2025-11-14
**Commit:** 064e0f2
**Branch:** phase2-week3-ai-integration

- [x] Hook integration phase added... - Completed: ...
- [x] Hook check executes... - Completed: ...
... [all 22 items]

**This story wires hook integration..."
)
```

**Step 3: Validate format**

```
Bash(command="devforgeai-validate validate-dod devforgeai/specs/Stories/STORY-027...")

Output: ✅ All DoD items validated
Exit code: 0
```

**Step 4: Update Change Log**

```
Edit:
- [ ] Development phase complete
  → [x] Development phase complete - Completed: 2025-11-14, commit 064e0f2
```

**Output State:**
```
✅ devforgeai-validate validate-dod: PASS
✅ Ready for git commit (will succeed)
```

---

## Detailed Format Specification

### Correct Format (Validator PASSES)

```markdown
## Implementation Notes

**Developer:** DevForgeAI AI Agent
**Implemented:** 2025-11-14
**Commit:** abc123def456
**Branch:** feature-branch

- [x] First DoD item text here - Completed: Description of what was done
- [x] Second DoD item text here - Completed: Another description
- [x] Third DoD item with `code` or **bold** - Completed: Preserves formatting

### TDD Workflow Summary

**Phase 02 (Red):**
- Generated tests
...

### Files Created/Modified

**Modified:**
- file1.md
...

## Change Log

**Current Status:** Dev Complete

| Date | Author | Phase/Action | Change | Files Affected |
|------|--------|--------------|--------|----------------|
| 2025-11-14 14:30 | claude/opus | DoD Update (Phase 07) | Development complete | story.md |
```

**Why this works:**
- DoD items are DIRECT children of `## Implementation Notes`
- No intervening `###` headers before DoD items
- `extract_section("Implementation Notes")` captures from `##` to next `###`
- All DoD items included in extraction
- Validator finds all items, passes validation

---

### Incorrect Format (Validator FAILS)

**Anti-Pattern 1: Subsection Header**

```markdown
## Implementation Notes

**Developer:** ...

### Definition of Done Status  ← WRONG: Subsection header
- [x] Item 1 - Completed: ...
```

**Why fails:** `extract_section()` stops at `###`, returns only "**Developer:**...", doesn't include items

---

**Anti-Pattern 2: Items After Subsection**

```markdown
## Implementation Notes

**Developer:** ...

### TDD Workflow Summary
... workflow content ...

- [x] Item 1 - Completed: ...  ← WRONG: After subsection
```

**Why fails:** Items are AFTER `###` header, treated as part of "TDD Workflow Summary" section, not "Implementation Notes"

---

**Anti-Pattern 3: Missing Developer Info**

```markdown
## Implementation Notes

- [x] Item 1 - Completed: ...  ← Missing developer, date, commit
```

**Why fails:** Doesn't fail validator, but loses traceability (who implemented? when? which commit?)

---

## Step-by-Step Checklist

**Execute these steps in order:**

- [ ] **Step 1:** Mark all completed DoD items [x] in Definition of Done section
  - [ ] Read story file to identify completed items
  - [ ] For each completed item: Edit to change `[ ]` → `[x]` and add " - Completed: ..." note
  - [ ] Verify all items marked (count should match actual implementation)

- [ ] **Step 2:** Add DoD items to Implementation Notes
  - [ ] Read story file to find Implementation Notes section
  - [ ] Locate insertion point (after developer info, BEFORE first ### subsection)
  - [ ] Build flat list of all [x] items from DoD section
  - [ ] Insert list using Edit tool (preserving exact text from DoD)
  - [ ] Verify format: Items directly under `## Implementation Notes`, no ### before items

- [ ] **Step 3:** Validate format with CLI validator
  - [ ] Run: `devforgeai-validate validate-dod ${STORY_FILE}`
  - [ ] Check exit code (must be 0)
  - [ ] If fails: Review errors, fix format, re-validate
  - [ ] If passes: Proceed to Step 4

- [ ] **Step 4:** Update Change Log
  - [ ] Append DoD Update entry with author `claude/opus`
  - [ ] Update **Current Status:** to "Dev Complete"
  - [ ] Verify changelog table format is valid

- [ ] **Step 5:** Final validation
  - [ ] Re-run: `devforgeai-validate validate-dod ${STORY_FILE}`
  - [ ] Confirm: Exit code 0
  - [ ] Display: "✅ DoD Update Workflow Complete - Ready for Phase 08"

**If ALL checkboxes checked:** ✅ Bridge workflow complete, proceed to Phase 08
**If ANY checkbox unchecked:** ❌ Bridge incomplete, do NOT proceed to git commit

---

## Token Budget

**Estimated token usage for this bridge workflow:**
- Reading story file: ~500 tokens
- Marking DoD items [x]: ~200 tokens (22 Edit operations)
- Adding items to Implementation Notes: ~300 tokens (1 large Edit)
- Validation: ~100 tokens (Bash command)
- Change Log updates: ~200 tokens (3 Edit operations)
- **Total:** ~1,300 tokens

**Benefit:** Prevents 3 failed commit attempts (~1,500 tokens each = 4,500 tokens wasted)
**Net Savings:** ~3,200 tokens per story

---

## Next Steps After Bridge Complete

**After this bridge workflow completes:**

1. **Proceed to Phase 08:** Load git-workflow-conventions.md
2. **Execute git commit:** Validator will pass (DoD format correct)
3. **Update story status:** Change to "Dev Complete"
4. **Proceed to Phase 09:** Feedback hook integration

**The bridge ensures seamless handoff from Phase 06 (semantic validation) to Phase 08 (format validation and git commit).**

---

## Reference

**Loaded by:** devforgeai-development skill after Phase 06 completes
**Cross-referenced by:**
- phase-4.5-deferral-challenge.md (Phase 06 → Bridge handoff)
- git-workflow-conventions.md (Bridge → Phase 08 prerequisites)
- SKILL.md (Phase 07 documentation)

**Related RCA:** RCA-009 (Skill Execution Incomplete Workflow)
**Implements:** Recommendation 4 (DoD Update Workflow Bridge)

---

**File Version:** 1.0
**Created:** 2025-11-14
**Purpose:** Prevent DoD format errors and git commit failures
