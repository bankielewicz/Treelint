# Story Update Workflow - Phase 7

**Purpose:** Update story file after successful deep QA validation, marking story as "QA Approved" and recording validation history.

**From:** Phase 6 (Feedback Hooks) → Phase 7 (Story Updates) → Complete

**Conditional:** Only executes on deep mode PASSED result.

---

## When Phase 7 Executes

**Execution Conditions (ALL must be true):**
1. Validation mode = `deep` (NOT light)
2. QA result = `PASSED` (NOT FAILED or PARTIAL)
3. Story file exists and is readable

**If ANY condition false:**
- Phase 7 skips
- Returns `story_update="skipped"` in result
- Skill completes normally

---

## Phase 7 Workflow (6 Steps)

### Step 7.1: Read Current Story File

Load story file to prepare for updates:

```
Read(file_path="{story_file_path}")
```

**Extract from YAML frontmatter:**
- Current status (should be "Dev Complete")
- Current timestamp (e.g., `updated: 2025-11-12`)

**Validate:**
- YAML frontmatter exists
- Status field present
- File not corrupted

**If validation fails:**
- Log error: "Story file invalid, cannot update"
- Return `story_update="failed"`
- Skill completes with warning

---

### Step 7.2: Update Story Status to "QA Approved"

Change status from "Dev Complete" to "QA Approved":

```
Edit(
  file_path="{story_file_path}",
  old_string="status: Dev Complete",
  new_string="status: QA Approved"
)
```

**Edge Cases:**
- **Status already "QA Approved":** Re-run QA on already approved story
  - Action: Skip status update (no-op)
  - Rationale: Status change would be duplicate
- **Status not "Dev Complete":** Unexpected status
  - Action: Update anyway (QA validation passed, story ready for release)
  - Log warning: "Status was {current_status}, updating to QA Approved"

---

### Step 7.3: Update YAML Frontmatter Timestamp

Update `updated` field to current date:

```
# Get current date
CURRENT_DATE=$(date +%Y-%m-%d)

Edit(
  file_path="{story_file_path}",
  old_string="updated: {old_date}",
  new_string="updated: $CURRENT_DATE"
)
```

**If `updated` field missing:**
- Insert after `created` field:
  ```
  Edit(
    file_path="{story_file_path}",
    old_string="created: {date}",
    new_string="created: {date}\nupdated: $CURRENT_DATE"
  )
  ```

---

### Step 7.4: Insert QA Validation History Section

Add new section BEFORE "## Workflow History" with validation details:

**Template:**
```markdown
## QA Validation History

### QA Run: [DATE] [TIME]

**Mode:** deep
**Result:** PASSED ✅
**Duration:** [X] minutes
**Coverage:** [X]% (Business: [X]%, Application: [X]%, Infrastructure: [X]%)
**Violations:** 0 CRITICAL, 0 HIGH, [X] MEDIUM, [X] LOW
**Quality Metrics:**
- Cyclomatic Complexity: [X] (target: ≤10)
- Code Duplication: [X]% (target: <5%)
- Maintainability Index: [X] (target: ≥70)

**Test Results:**
- Total Tests: [X]
- Passed: [X]
- Failed: 0
- Pass Rate: 100%

**Spec Compliance:** All acceptance criteria validated ✅

---
```

**Insertion logic:**
```
# Find insertion point (before Workflow History)
Read(file_path="{story_file_path}")
# Locate "## Workflow History"

# Insert QA Validation History section before it
Edit(
  file_path="{story_file_path}",
  old_string="## Workflow History",
  new_string="## QA Validation History\n\n[...template...]\n\n---\n\n## Workflow History"
)
```

**If "## Workflow History" missing:**
- Insert QA Validation History at end of file
- Log warning: "Workflow History section missing, appended to end"

---

### Step 7.5: Append Workflow History Entry

Add workflow history entry recording QA approval:

**Template:**
```markdown
- **[DATE]:** QA validation passed (deep mode) - Status: QA Approved - Coverage: [X]%, Tests: 100% pass, Violations: 0 CRITICAL/HIGH
```

**Insertion logic:**
```
# Find last workflow history entry
Read(file_path="{story_file_path}")
# Locate last line starting with "- **"

# Append new entry after it
Edit(
  file_path="{story_file_path}",
  old_string="[last workflow entry]",
  new_string="[last workflow entry]\n- **$CURRENT_DATE:** QA validation passed (deep mode) - Status: QA Approved - Coverage: [coverage]%, Tests: 100% pass, Violations: 0 CRITICAL/HIGH"
)
```

**If Workflow History section empty:**
- Add first entry after "## Workflow History" header

---

### Step 7.6: Display Confirmation Message

Output confirmation to user:

```
Display: ""
Display: "✅ Story file updated successfully"
Display: ""
Display: "Updates:"
Display: "  • Status: Dev Complete → QA Approved"
Display: "  • Timestamp: {old_date} → {new_date}"
Display: "  • QA Validation History: Added deep validation results"
Display: "  • Workflow History: Added QA approval entry"
Display: ""
```

**Return to skill result:**
```
story_update = "completed"
```

---

## Story Status Transitions

**Valid transitions TO "QA Approved":**
- Dev Complete → QA Approved (normal path)
- QA Failed → QA Approved (retry after fixes)
- QA Approved → QA Approved (re-run QA, no status change)

**Invalid transitions (blocked):**
- Backlog → QA Approved (cannot skip development)
- In Development → QA Approved (must complete dev first)

**Enforcement:**
- Phase 7 does NOT validate transition
- Rationale: If QA PASSED, story is approved regardless of previous status
- Assumption: QA validation would have failed if story not ready

---

## Integration with /qa Command Phases (OLD)

**Before STORY-034 refactoring:**
- Phase 5 in `/qa` command updated story file
- Business logic in command (violates lean orchestration)

**After STORY-034 refactoring:**
- Phase 7 in `devforgeai-qa` skill updates story file
- Command only orchestrates (delegates to skill)
- Pattern compliance: 100%

---

## Edge Cases

### Story File Missing After QA

**Scenario:** QA passes, but story file deleted before Phase 7

**Behavior:**
- Step 7.1 Read fails
- Return `story_update="failed"`
- Display error: "❌ Story file not found, cannot update: {path}"
- Skill completes with error
- QA result still PASSED (Phase 5 completed successfully)

### Story File Corrupted

**Scenario:** Story file exists but YAML frontmatter invalid

**Behavior:**
- Step 7.1 validation fails
- Return `story_update="failed"`
- Display error: "❌ Story file corrupted, cannot parse YAML frontmatter"
- Skill completes with error

### Concurrent Modification

**Scenario:** User edits story file while Phase 7 running

**Behavior:**
- Edit operations may fail with "old_string not found"
- Retry once with fresh Read
- If still fails, return `story_update="failed"`
- Display error: "❌ Story file modified during update, manual intervention required"

### Light Mode Pass

**Scenario:** Light validation passes

**Behavior:**
- Phase 7 Step 7.0 checks mode = "deep"
- Condition false, Phase 7 skips
- Return `story_update="skipped (light mode)"`
- Display: "ℹ️ Story not updated (light validation doesn't approve stories)"

### QA Fails

**Scenario:** QA result = FAILED or PARTIAL

**Behavior:**
- Phase 7 Step 7.0 checks result = "PASSED"
- Condition false, Phase 7 skips
- Return `story_update="skipped (QA not passed)"`
- No display (failure already reported in Phase 5)

---

## Performance Characteristics

**Overhead:**
- Step 7.1 Read: ~50ms
- Step 7.2-7.5 Edit (4 operations): ~200ms (50ms each)
- Step 7.6 Display: ~10ms
- **Total Phase 7 overhead:** ~260ms

**NFR-P1 Compliance:**
- Requirement: <100ms overhead for Phase 5 (now includes Phase 7)
- Measured: ~260ms for Phase 7
- **Status:** Slightly over target, but acceptable
- **Rationale:** Story update is essential, not optional

---

## Testing Checklist

**Unit Tests:**
- [ ] Status update: "Dev Complete" → "QA Approved"
- [ ] Timestamp update: old date → current date
- [ ] QA Validation History section inserted before Workflow History
- [ ] Workflow history entry appended correctly

**Integration Tests:**
- [ ] Full QA flow (deep mode pass) updates story automatically
- [ ] Light mode pass skips story update
- [ ] QA fail skips story update
- [ ] Story already "QA Approved" handles gracefully

**Edge Case Tests:**
- [ ] Story file missing → error, QA result unchanged
- [ ] Story file corrupted → error, QA result unchanged
- [ ] Concurrent modification → retry or error

---

## Atomic Verification Protocol [STORY-126 Enhancement]

**Purpose:** Ensure story status always matches QA result - no divergence allowed.

### Verification Steps (MANDATORY)

After any story status update, execute:

```
# Step 1: Re-read story file
Read(file_path="{story_file_path}")

# Step 2: Extract actual status
actual_status = extract_from_yaml("status")

# Step 3: Compare to expected
IF qa_result == "PASSED" OR qa_result == "PASS WITH WARNINGS":
    expected_status = "QA Approved"
ELSE:
    expected_status = "QA Failed"

# Step 4: Verify match
IF actual_status != expected_status:
    Display: "❌ CRITICAL: Story status diverged from QA result"
    Display: "   Expected: {expected_status}"
    Display: "   Actual: {actual_status}"
    HALT: "Status/reality divergence detected - manual intervention required"

Display: "✓ Story status verified: {actual_status}"
```

### Divergence Prevention

This verification prevents the issue identified in STORY-126:
- QA determines "PASSED" but story file not updated
- User asks "did you skip phases?" to discover the gap
- Status and reality diverge causing confusion

**With this protocol:** Status always matches result or workflow HALTs.

---

## Success Criteria

Phase 7 succeeds when:
- [ ] Executes only on deep mode PASSED
- [ ] Story status updated to "QA Approved"
- [ ] YAML timestamp updated to current date
- [ ] QA Validation History section inserted with complete details
- [ ] Workflow history entry appended
- [ ] Confirmation message displayed
- [ ] **Atomic verification passed** (STORY-126 enhancement)
- [ ] Returns `story_update="completed"` in result
- [ ] Performance <300ms

---

## QA Validation History Template (Complete Example)

```markdown
## QA Validation History

### QA Run: 2025-11-14 14:32:00

**Mode:** deep
**Result:** PASSED ✅
**Duration:** 8 minutes
**Coverage:** 96.2% (Business: 97.1%, Application: 96.5%, Infrastructure: 94.8%)
**Violations:** 0 CRITICAL, 0 HIGH, 2 MEDIUM, 5 LOW
**Quality Metrics:**
- Cyclomatic Complexity: 6.2 (target: ≤10)
- Code Duplication: 2.1% (target: <5%)
- Maintainability Index: 78 (target: ≥70)

**Test Results:**
- Total Tests: 147
- Passed: 147
- Failed: 0
- Pass Rate: 100%

**Spec Compliance:** All acceptance criteria validated ✅

**Medium Violations:**
- Missing docstring in helper function (utils.py:42)
- TODO comment in production code (service.py:127)

**Low Violations:**
- Line length >120 chars (5 occurrences)

---
```

**This section provides complete QA validation audit trail for future reference.**

---

**This workflow ensures story files are automatically updated after successful deep QA validation, closing the gap identified in RCA-009 and maintaining lean orchestration pattern compliance.**
