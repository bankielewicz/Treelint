# AC Verification Checklist Update Workflow

**Purpose:** Real-time progress tracking by updating Acceptance Criteria Verification Checklist during TDD phases

**Execution:** After each TDD phase (1-5) completes successfully

**Token Cost:** ~500 tokens per phase update (loaded on-demand at phase boundaries)

**Related:** RCA-011 (AC Checklist described as real-time tracker but not updated)

---

## Overview

The Acceptance Criteria Verification Checklist is a granular progress tracker that breaks down high-level Acceptance Criteria into checkable sub-items mapped to specific TDD phases.

### Three Tracking Mechanisms

| Mechanism | Granularity | Updated When | Purpose |
|-----------|-------------|--------------|---------|
| **TodoWrite** | Phase-level | Real-time during phases | AI self-monitoring, visual progress |
| **AC Checklist** | AC sub-item level | End of each phase | User visibility into AC completion |
| **Definition of Done** | DoD item level | Phase 07 | Official completion record, git validation |

**Why all three:**
- TodoWrite: AI tracks where it is in workflow (prevents skipped phases)
- AC Checklist: User sees granular progress on ACs (transparency)
- DoD: Framework validates completion (quality gate)

---

## When to Update AC Checklist

**Update Trigger:** At the END of each TDD phase, batch-update all items mapped to that phase

### Phase 02 (Red - Test Generation) → Update Test-Related AC Items
**After test-automator subagent completes:**
- [ ] Test count items (e.g., "Unit tests ≥15", "Integration tests ≥12")
- [ ] Test coverage items (e.g., "All ACs have tests", "Edge cases covered")
- [ ] Test file creation items (e.g., "Test files in correct location")

### Phase 03 (Green - Implementation) → Update Implementation AC Items
**After backend-architect/frontend-developer completes:**
- [ ] Code implementation items (e.g., "Endpoint created", "Logic implemented")
- [ ] Business logic items (e.g., "No business logic in command", "All logic in skill")
- [ ] File modification items (e.g., "Character count ≤15K", "Line count ≤350")

### Phase 04 (Refactor - Code Quality) → Update Quality AC Items
**After refactoring-specialist and code-reviewer complete:**
- [ ] Code quality items (e.g., "Complexity <10", "No duplication")
- [ ] Pattern compliance items (e.g., "5-responsibility checklist", "Anti-patterns avoided")
- [ ] Refactoring completion items (e.g., "Code review passed")

### Phase 05 (Integration - Cross-Component) → Update Integration AC Items
**After integration-tester completes:**
- [ ] Integration test items (e.g., "All scenarios pass", "Cross-component tests pass")
- [ ] Performance items (e.g., "Response time <200ms", "Token efficiency ≥75%")
- [ ] Coverage items (e.g., "Coverage ≥95%", "All paths tested")

### Phase 06 (Deferral Challenge) → Update Deferral AC Items
**After deferral-validator completes:**
- [ ] Deferral validation items (e.g., "All deferrals approved", "No circular chains")
- [ ] Follow-up story items (e.g., "Follow-up stories created")

### Phase 08 (Git Workflow) → Update Deployment Readiness AC Items
**After git commit completes:**
- [ ] Commit items (e.g., "Git commit created", "Semantic message")
- [ ] Status items (e.g., "Story status updated", "Backward compatible")
- [ ] Documentation items (e.g., "Integration notes added")

---

## Update Procedure (End-of-Phase Batch)

### Step 1: Identify AC Items for Current Phase

**At phase completion, determine which AC items map to this phase:**

```
current_phase = {1-5}

Read(file_path="${STORY_FILE}")

# Extract AC Verification Checklist section
# Look for: **Phase:** {current_phase} markers

phase_items = [
  items where "**Phase:** {current_phase}" appears
]
```

**Example for Phase 03:**
```
Phase 03 completes
  ↓
Search checklist for "**Phase:** 2"
  ↓
Find items:
  - [ ] Character count ≤15,000 - **Phase:** 2 - **Evidence:** wc -c
  - [ ] Line count ≤350 - **Phase:** 2 - **Evidence:** wc -l
  - [ ] No business logic - **Phase:** 2 - **Evidence:** grep check
  ↓
Update all 3 items (batch Edit)
```

---

### Step 2: Validate Item Completion

**For each item in phase_items:**

```
Verify item is actually complete:
  - Check evidence location (does file/test exist?)
  - Verify assertion (does metric meet target?)
  - Confirm implementation (was code actually written?)

IF item NOT complete:
  Skip update (leave [ ] unchecked)
  Note: Item will be deferred or completed in later phase

IF item IS complete:
  Proceed to Step 3 (update checklist)
```

**Example validation:**
```
Item: "Character count ≤15,000 - **Phase:** 2 - **Evidence:** wc -c"

Validation:
  Bash(command="wc -c < .claude/commands/release.md")
  Result: 7,416 chars
  Check: 7,416 ≤ 15,000? YES ✓

  Item IS complete → proceed to update
```

---

### Step 3: Batch Update Checklist Items

**Use Edit tool to update all items for this phase:**

```
FOR each completed_item in phase_items:
  old_text = "- [ ] {item_text} - **Phase:** {N} - **Evidence:** {evidence}"
  new_text = "- [x] {item_text} - **Phase:** {N} - **Evidence:** {evidence}"

  Edit(
    file_path="${STORY_FILE}",
    old_string=old_text,
    new_string=new_text
  )

  Display: "  ✓ {item_text}"
```

**Batch optimization (if multiple items):**
```
# Can combine multiple replacements into single Edit if items are contiguous
Edit(
  file_path="${STORY_FILE}",
  old_string="""- [ ] Item 1
- [ ] Item 2
- [ ] Item 3""",
  new_string="""- [x] Item 1
- [x] Item 2
- [x] Item 3"""
)
```

---

### Step 4: Display Phase Completion Summary

**After all items updated:**

```
Display:
"
Phase {N} Checklist Update:
  ✓ {count} AC items checked
  - {item 1 brief}
  - {item 2 brief}
  ...

AC Progress: {checked}/{total} items complete ({percentage}%)
"
```

**Example:**
```
Phase 03 Checklist Update:
  ✓ 3 AC items checked
  - Character count ≤15,000
  - Line count ≤350
  - No business logic in command

AC Progress: 15/42 items complete (36%)
```

---

## Integration into TDD Workflow

### Modified Phase Flow

**Current flow:**
```
Phase {N} Execute
  ↓
Phase {N} Validation
  ↓
Mark phase "completed" in TodoWrite
  ↓
Proceed to Phase {N+1}
```

**Enhanced flow with AC Checklist:**
```
Phase {N} Execute
  ↓
Phase {N} Validation
  ↓
Mark phase "completed" in TodoWrite
  ↓
**Update AC Checklist items for Phase {N}**  ← NEW STEP
  ↓
Display AC progress summary
  ↓
Proceed to Phase {N+1}
```

---

## Reference File Integration Points

### tdd-red-phase.md (Phase 02)
**Add at end of phase (after Step 4: Tech Spec Coverage Validation):**

```markdown
### Step 5: Update AC Checklist (Phase 02 Items)

**Load AC checklist update workflow:**
```
Read(file_path=".claude/skills/devforgeai-development/references/ac-checklist-update-workflow.md")
```

**Execute:** "Update Procedure" for current_phase = 1

**AC items for Phase 02:**
- Test count items (unit, integration, regression)
- Test coverage items (all ACs tested)
- Test file creation items (correct location, naming)

**Display:** Phase 02 AC progress summary
```

### tdd-green-phase.md (Phase 03)
**Add at end of phase (after Step 3: context-validator):**

```markdown
### Step 4: Update AC Checklist (Phase 03 Items)

[Same pattern as Phase 02]

**AC items for Phase 03:**
- Implementation items (code written, endpoints created)
- Business logic items (logic location correct)
- Size/metric items (character count, line count)
```

### tdd-refactor-phase.md (Phase 04)
**Add after Step 5 (Light QA):**

```markdown
### Step 6: Update AC Checklist (Phase 04 Items)

[Same pattern]

**AC items for Phase 04:**
- Quality items (complexity, duplication, maintainability)
- Pattern compliance items (5-responsibility, anti-patterns)
- Code review items (review passed, issues resolved)
```

### integration-testing.md (Phase 05)
**Add at end:**

```markdown
### Step 4: Update AC Checklist (Phase 05 Items)

[Same pattern]

**AC items for Phase 05:**
- Integration test items (scenarios pass, cross-component works)
- Performance items (latency, throughput, efficiency)
- Coverage items (thresholds met, all paths covered)
```

### phase-06-deferral-challenge.md (Phase 06)
**Add at end:**

```markdown
### Step 7: Update AC Checklist (Phase 06 Items)

[Same pattern]

**AC items for Phase 06:**
- Deferral validation items (all validated, user approved)
- Follow-up items (stories created if needed)
```

### git-workflow-conventions.md (Phase 08)
**Add after commit succeeds:**

```markdown
### Step 8: Update AC Checklist (Phase 08 Items)

[Same pattern]

**AC items for Phase 08:**
- Deployment readiness items (commit created, status updated)
- Backward compatibility items (verified, tested)
- Documentation items (notes added, examples included)
```

---

## Error Handling

### AC Checklist Section Not Found

**Error:** Story file doesn't have AC Verification Checklist section

**Recovery:**
```
Display: "⚠️ AC Verification Checklist not found in story (may be older format)"
Display: "Skipping AC checklist updates (DoD tracking still active)"

Continue workflow normally
```

**Rationale:** Backward compatible with stories created before checklist feature

### AC Item Text Doesn't Match

**Error:** Edit operation fails because item text changed

**Recovery:**
```
Display: "⚠️ AC item text mismatch (may have been manually edited)"
Display: "Skipping item: {item_text}"

Continue with remaining items
```

### No AC Items for Phase

**Error:** Current phase has no mapped AC items

**Recovery:**
```
Display: "ℹ️ No AC items mapped to Phase {N}"

Continue to next phase
```

**Example:** Phase 06 may have no items for simple stories without deferrals

---

## Performance Considerations

### Estimated Overhead

**Per Story:**
- Phase 02: ~2-4 AC items → ~30 seconds
- Phase 03: ~4-8 AC items → ~1 minute
- Phase 04: ~2-4 AC items → ~30 seconds
- Phase 05: ~3-6 AC items → ~45 seconds
- Phase 06: ~1-2 AC items → ~15 seconds
- Phase 08: ~2-4 AC items → ~30 seconds

**Total: ~3-4 minutes per story**

### Optimization Strategies

**1. Batch Edits:**
```
# Instead of 5 separate Edits
Edit(...item 1...)
Edit(...item 2...)
Edit(...item 3...)
Edit(...item 4...)
Edit(...item 5...)

# Do single Edit with multi-line replacement
Edit(
  old_string="""- [ ] Item 1
- [ ] Item 2
- [ ] Item 3
- [ ] Item 4
- [ ] Item 5""",
  new_string="""- [x] Item 1
- [x] Item 2
- [x] Item 3
- [x] Item 4
- [x] Item 5"""
)
```

**Savings:** 5 Edit operations → 1 Edit operation (~80% time reduction)

**2. Skip if Disabled:**
```
# Configuration option (future enhancement)
IF config.ac_checklist_updates_enabled == false:
  Skip AC checklist updates entirely
  Continue with DoD-only workflow
```

---

## Success Criteria

AC Checklist update workflow succeeds when:
- [ ] Checklist exists in story file (or gracefully skips if not)
- [ ] Items mapped to current phase identified
- [ ] Each item validated for completion
- [ ] Completed items checked off ([ ] → [x])
- [ ] Progress summary displayed to user
- [ ] Performance impact <5 minutes per story
- [ ] Backward compatible (works with/without checklist)

---

## Backward Compatibility

**Graceful Handling of Stories Without Checklist:**

```
IF checklist section not found:
  Display: "ℹ️ Story uses DoD-only tracking (AC Checklist not present)"
  Skip all AC checklist update steps
  Continue with DoD workflow (unchanged)

Result: Older stories work identically, newer stories get enhanced tracking
```

---

**Status:** Reference file complete, ready for workflow integration
**Next:** Integrate into phase-specific reference files (tdd-red-phase.md, tdd-green-phase.md, etc.)
