# Phase 4.5-R: Resumption Decision Point

**Purpose:** Enforce "no deferrals = work until 100%" policy by looping back to appropriate TDD phase when user rejects deferrals and DoD is incomplete

**Execution:** After Phase 4.5-5 Bridge (DoD Update), before Phase 5 (Git Workflow)

**Token Cost:** ~500 tokens (lightweight decision logic)

**Created:** 2025-11-22 (RCA-013 Recommendation 1)

---

## When to Execute

**Trigger Conditions (ALL must be true):**
1. Phase 4.5 (Deferral Challenge) complete
2. Phase 4.5-5 Bridge (DoD Update) complete
3. User rejected deferrals (chose "Continue to 100%" or equivalent)
4. DoD completion <100%

**Skip Conditions (ANY triggers skip):**
1. DoD completion ==100% (all items checked)
2. User approved deferrals (documented in Implementation Notes)
3. Iteration count >=5 (limit reached, requires user approval for continuation)

---

## Phase Progress Indicator

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 4.5-R: Resumption Decision Point
  TDD Iteration: {iteration_count}/5
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Display this indicator at the start of Phase 4.5-R execution.**

---

## Step 1: Calculate DoD Completion

**Count DoD items:**

```
Read story file section: ## Definition of Done

Extract all checkbox lines:
  Grep(pattern="^- \[(x| )\]", path=story_file, output_mode="content")

Parse results:
  total_dod_items = count(all checkbox lines)
  checked_items = count(lines with "[x]")
  unchecked_items = count(lines with "[ ]")

Calculate completion:
  completion_pct = (checked_items / total_dod_items) × 100

Display:
"DoD Status: {checked_items}/{total_dod_items} complete ({completion_pct}%)"
```

---

## Step 2: Check User Decision from Phase 4.5

**Search conversation for deferral decision:**

```
Search recent conversation (Phase 4.5 section) for:
  - User selected "Continue to 100%" OR
  - User selected "Continue development to 100%" OR
  - User selected option containing "Continue" AND "100%"

IF found:
  user_decision = "continue_to_100"
  user_rejected_deferrals = true

ELSE search for:
  - User selected "Document deferrals" OR
  - User selected "Approve deferrals" OR
  - Conversation contains "Approved Deferrals" section added

IF found:
  user_decision = "approved_deferrals"
  user_rejected_deferrals = false

ELSE:
  # Ambiguous - need to confirm
  Display: "⚠️ Cannot determine deferral decision from Phase 4.5"

  AskUserQuestion:
    Question: "DoD is {completion_pct}% complete. How should we proceed?"
    Header: "Next Action"
    Options:
      - "Continue working to 100%"
      - "Defer remaining work with approval"
      - "Commit current progress and stop"
    multiSelect: false

  Extract user_decision from response
```

---

## Step 3: Resumption Decision Logic

**Apply decision tree:**

```
IF completion_pct == 100:
  Display: ""
  Display: "✅ DoD 100% Complete - All work finished!"
  Display: "   Proceeding to Phase 5 (Git Workflow)..."
  Display: ""

  GOTO Phase 5

ELSE IF user_decision == "approved_deferrals":
  Display: ""
  Display: "✅ Deferrals Documented - User approved incomplete items"
  Display: "   DoD: {checked_items}/{total_dod_items} ({completion_pct}%)"
  Display: "   Proceeding to Phase 5 (Git Workflow)..."
  Display: ""

  GOTO Phase 5

ELSE IF user_decision == "continue_to_100" AND iteration_count < 5:
  Display: ""
  Display: "════════════════════════════════════════════════════════════"
  Display: "⚠️  RESUMPTION TRIGGERED"
  Display: "════════════════════════════════════════════════════════════"
  Display: ""
  Display: "DoD Status: {completion_pct}% complete ({unchecked_items} items remaining)"
  Display: "User Decision: Continue to 100% (deferrals rejected)"
  Display: "Iteration: {iteration_count}/5"
  Display: ""
  Display: "Calculating resumption point..."
  Display: ""

  GOTO Step 4 (Determine Resumption Phase)

ELSE IF user_decision == "continue_to_100" AND iteration_count >= 5:
  Display: ""
  Display: "════════════════════════════════════════════════════════════"
  Display: "⚠️  ITERATION LIMIT REACHED"
  Display: "════════════════════════════════════════════════════════════"
  Display: ""
  Display: "Story has required 5 TDD iterations without reaching 100%."
  Display: "Current completion: {completion_pct}%"
  Display: "Remaining items: {unchecked_items}"
  Display: ""
  Display: "This may indicate:"
  Display: "  • Story scope too large for single session"
  Display: "  • Unexpected blockers encountered"
  Display: "  • Complex implementation requiring decomposition"
  Display: ""

  GOTO Step 6 (Iteration Limit Handler)

ELSE IF user_decision == "commit_current_progress":
  Display: ""
  Display: "✅ Committing Progress - User chose to stop at {completion_pct}%"
  Display: "   Note: Story will remain 'In Development' status"
  Display: "   Remaining work: {unchecked_items} DoD items"
  Display: ""

  GOTO Phase 5 (commit current progress)
```

---

## Step 4: Determine Resumption Phase

**Analyze unchecked DoD items by category:**

```
Read DoD section, categorize unchecked items:

implementation_unchecked = count(DoD.Implementation where status == "[ ]")
quality_unchecked = count(DoD.Quality where status == "[ ]")
testing_unchecked = count(DoD.Testing where status == "[ ]")
documentation_unchecked = count(DoD.Documentation where status == "[ ]")

Display breakdown:
"Remaining Work by Category:"
"  • Implementation: {implementation_unchecked} items"
"  • Quality: {quality_unchecked} items"
"  • Testing: {testing_unchecked} items"
"  • Documentation: {documentation_unchecked} items"
""

IF implementation_unchecked > 0:
  resume_phase_num = 2
  resume_phase_name = "Phase 2: Implementation (Green Phase)"
  resume_action = "Complete remaining implementation items"
  resume_subagent = "backend-architect or frontend-developer"

ELSE IF quality_unchecked > 0:
  resume_phase_num = 3
  resume_phase_name = "Phase 3: Refactoring & Quality"
  resume_action = "Complete remaining quality validations"
  resume_subagent = "refactoring-specialist and code-reviewer"

ELSE IF testing_unchecked > 0:
  resume_phase_num = 4
  resume_phase_name = "Phase 4: Integration Testing"
  resume_action = "Add missing integration tests"
  resume_subagent = "integration-tester"

ELSE IF documentation_unchecked > 0:
  resume_phase_num = 3
  resume_phase_name = "Phase 3: Documentation & Review"
  resume_action = "Complete documentation items"
  resume_subagent = "code-reviewer for documentation"

ELSE:
  # Edge case: All categorized but items exist?
  resume_phase_num = 2
  resume_phase_name = "Phase 2: Implementation"
  resume_action = "Review and complete remaining items"

Display:
"Resumption Point: {resume_phase_name}"
"Action: {resume_action}"
"Subagent: {resume_subagent}"
""
```

---

## Step 5: Update TodoWrite for Loop-Back

**Reset todo statuses to enable resumption:**

```
Display: "Updating workflow tracker for resumption..."
""

# Mark resumption phase and all subsequent phases as "pending"
FOR phase_num = resume_phase_num TO 7:
  IF phase_num == resume_phase_num:
    # Mark as pending, will become in_progress when phase starts
    todos[phase_num].status = "pending"
  ELSE IF phase_num > resume_phase_num AND phase_num <= 7:
    # Reset all later phases
    todos[phase_num].status = "pending"

# Keep phases 0 through resume_phase_num-1 as "completed"
# This preserves audit trail

TodoWrite(todos=updated_todos)

Example result for resume_phase_num=2, iteration_count=2:
[1. [completed] Phase 0: Pre-Flight]
[2. [completed] Phase 1: Test-First Design]
[3. [pending] Phase 2: Implementation]     ← RESUMED (was completed, now pending again)
[4. [pending] Phase 3: Refactoring]        ← RESET
[5. [pending] Phase 4: Integration]        ← RESET
[6. [pending] Phase 4.5: Deferral Challenge] ← RESET
[7. [pending] Phase 4.5-5 Bridge: DoD Update] ← RESET
[8. [pending] Phase 4.5-R: Resumption]     ← RESET
[9. [pending] Phase 5: Git Workflow]
[10. [pending] Phase 6: Feedback Hook]
[11. [pending] Phase 7: Result Interpretation]

Display: "✓ Workflow tracker updated - phases {resume_phase_num}-7 marked pending"
""
```

---

## Step 6: Increment Iteration Counter

**Track iterations for monitoring and limit enforcement:**

```
iteration_count++

Display:
"════════════════════════════════════════════════════════════"
"RESUMING TDD WORKFLOW - Iteration {iteration_count}"
"════════════════════════════════════════════════════════════"
""
"Story: {STORY_ID}"
"Current Completion: {completion_pct}%"
"Remaining Work: {unchecked_items} DoD items"
"Resumption Point: {resume_phase_name}"
""
"Continuing TDD cycle..."
""
```

**Store iteration count for next pass:**
```
# iteration_count will be checked in next Phase 4.5-R execution
# If reaches 5, triggers iteration limit handler (Step 6 in decision tree)
```

---

## Step 7: Jump to Resumption Phase

**Execute phase jump:**

```
Display: "Jumping to {resume_phase_name}..."
Display: ""

GOTO Phase {resume_phase_num}

# Workflow will execute:
# - Phase resume_phase_num (Implementation/Quality/Integration)
# - Phase 3 (if resumed from 2)
# - Phase 4 (if resumed from 2 or 3)
# - Phase 4.5 (Deferral Challenge again)
# - Phase 4.5-5 Bridge (DoD Update again)
# - Phase 4.5-R (THIS PHASE - will check completion again)
#
# If DoD still <100%, loop continues
# If DoD reaches 100%, proceeds to Phase 5
```

**Implementation note:**
```
# In practice, this means:
# 1. Claude sees "GOTO Phase 2" instruction
# 2. Claude scrolls to Phase 2 section in SKILL.md
# 3. Claude executes Phase 2 instructions
# 4. Continues through phases sequentially
# 5. Returns to Phase 4.5-R
# 6. Checks completion again
```

---

## Step 6 (Alternative Path): Iteration Limit Handler

**Execute if iteration_count >= 5:**

```
Display:
"════════════════════════════════════════════════════════════"
"⚠️  ITERATION LIMIT REACHED (5 iterations)"
"════════════════════════════════════════════════════════════"
""
"Story: {STORY_ID}"
"Current Completion: {completion_pct}%"
"Remaining Work: {unchecked_items} DoD items"
"Iterations Completed: 5"
""
"The story has required 5 TDD iterations without reaching 100%."
"This suggests one of the following:"
""
"  1. Story scope is too large for single implementation"
"     → Consider: Break into smaller stories"
""
"  2. Unexpected blockers are preventing completion"
"     → Consider: Document blockers, defer with approval"
""
"  3. Implementation approach needs revision"
"     → Consider: Review approach with user, adjust strategy"
""
"  4. Story is nearly complete, one more iteration would finish"
"     → Consider: Extend limit, continue to completion"
""

AskUserQuestion:
  Question: "Story has required 5 iterations ({completion_pct}% complete). How should we proceed?"
  Header: "Iteration Limit"
  Options:
    - "Show me the {unchecked_items} remaining DoD items"
    - "Continue (one more iteration)"
    - "Document progress and defer remaining work"
    - "Stop and explain what's blocking completion"
  multiSelect: false

Handle response:

  CASE "Show me the remaining DoD items":
    Display unchecked DoD items with categories
    Re-ask question (recurse to this AskUserQuestion)

  CASE "Continue (one more iteration)":
    iteration_limit = 6  # Extend by 1
    Display: "✓ Extending iteration limit to 6"
    Display: "  Resuming development..."
    ""
    GOTO Step 4 (Determine Resumption Phase)
    # Will resume one more time

  CASE "Document progress and defer remaining work":
    Display: ""
    Display: "Documenting deferrals with user approval..."
    Display: "Timestamp: {current_datetime}"
    Display: ""

    Add to story Implementation Notes:
    """
    **Approved Deferrals (Iteration Limit):**
    - Approved: {current_datetime}
    - Reason: Story required 5+ iterations, remaining work deferred per user approval
    - Completion at deferral: {completion_pct}%
    - Remaining items: {unchecked_items}
    - Follow-up: Create continuation story for remaining work

    Deferred Items:
    {List all unchecked DoD items}
    """

    Display: "✓ Deferrals documented with user approval"
    GOTO Phase 5 (commit with deferrals)

  CASE "Stop and explain what's blocking completion":
    Display: ""
    Display: "════════════════════════════════════════════════════════════"
    Display: "ANALYSIS: What's Preventing Completion"
    Display: "════════════════════════════════════════════════════════════"
    Display: ""

    Analyze unchecked items:
      Group by: Category, Type, Complexity

    FOR each unchecked_item:
      Display: "• {category}/{item_text}"
      Analyze: Why incomplete? (tests failing, blocker, unclear requirement)
      Display: "  Status: {analysis}"

    Display: ""
    Display: "Recommendations:"
    FOR each blocker identified:
      Display: "  • {recommendation to unblock}"

    Display: ""
    Display: "Next Steps:"
    Display: "  1. Address blockers above"
    Display: "  2. Re-run: /dev {STORY_ID} (will resume from Phase 2)"
    Display: "  3. Or use: /resume-dev {STORY_ID} 2 (manual resumption)"
    Display: ""

    HALT workflow (user to address blockers, then re-run)
```

---

## Resumption Phase Selection Logic

**Detailed decision tree:**

```
┌─ Unchecked Items Analysis ─────────────────────────────────┐
│                                                              │
│  IF implementation_unchecked > 0:                            │
│    Resume: Phase 2 (Green - Implementation)                 │
│    Reason: Core implementation incomplete                    │
│    Subagent: backend-architect OR frontend-developer         │
│                                                              │
│  ELSE IF quality_unchecked > 0:                              │
│    Resume: Phase 3 (Refactor - Quality)                     │
│    Reason: Code quality issues unresolved                    │
│    Subagent: refactoring-specialist, code-reviewer          │
│                                                              │
│  ELSE IF testing_unchecked > 0:                              │
│    Resume: Phase 4 (Integration Testing)                    │
│    Reason: Test coverage gaps                                │
│    Subagent: integration-tester                             │
│    Note: May also need Phase 1 (test-automator) if new      │
│          tests required (not just integration)               │
│                                                              │
│  ELSE IF documentation_unchecked > 0:                        │
│    Resume: Phase 3 (Documentation finalization)             │
│    Reason: Documentation incomplete                          │
│    Subagent: code-reviewer (documentation review)           │
│                                                              │
│  ELSE (edge case):                                           │
│    Resume: Phase 2 (default)                                 │
│    Reason: Unclear categorization, start from              │
│            implementation phase                              │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

---

## Iteration Counter Behavior

**Purpose:** Prevent infinite loops when story cannot be completed

**Default Limit:** 5 iterations
**Configurable:** Can be extended by user when limit reached

**Counter Semantics:**
- `iteration_count = 1`: First pass through TDD cycle
- `iteration_count = 2`: First resumption (looped back once)
- `iteration_count = 3`: Second resumption
- `iteration_count = 5`: Limit reached, requires user decision

**Why 5?**
- Most stories complete in 1-2 iterations
- 3-4 iterations indicates large story or complexity
- 5 iterations is reasonable upper bound before user intervention

**User can:**
- Extend limit (continue iteration 6)
- Defer remaining work (document deferrals)
- Stop and review blockers (analyze what's preventing completion)

---

## Integration with Phase 4.5 (Deferral Challenge)

**Relationship:**

Phase 4.5 validates deferrals and obtains user approval
  ↓
Phase 4.5-5 Bridge updates DoD format
  ↓
**Phase 4.5-R checks if user rejected deferrals** ← THIS PHASE
  ↓
  IF rejected AND incomplete: Loop back to Phase 2/3/4
  IF approved OR complete: Proceed to Phase 5

**Key Distinction:**
- Phase 4.5: Validates EXISTING deferrals (pre-existing + new)
- Phase 4.5-R: Enforces user's decision about CONTINUING WORK vs DEFERRING

**User workflow:**
1. Phase 4.5 asks: "Should we defer these {N} items?"
2. User chooses: "Continue to 100%" (rejects deferrals)
3. Phase 4.5-R detects: User rejected deferrals AND DoD <100%
4. Phase 4.5-R action: Resume TDD cycle to complete work

---

## Success Criteria

Phase 4.5-R succeeds when:
- [ ] DoD completion calculated correctly
- [ ] User decision detected (continue vs defer)
- [ ] Resumption phase determined (2, 3, or 4)
- [ ] TodoWrite updated (phases marked pending)
- [ ] Iteration counter incremented
- [ ] Resumption message displayed
- [ ] Workflow jumps to correct phase
- [ ] OR: DoD 100%/deferrals approved → Phase 5
- [ ] OR: Iteration limit → User approval for continuation

---

## Error Handling

**Error: Cannot determine user decision**
```
Recovery: AskUserQuestion with explicit options
Continue: After user clarifies
```

**Error: DoD section malformed (cannot parse)**
```
Recovery:
  Display: "⚠️ DoD section format error - cannot calculate completion"
  Display: "Assuming incomplete, proceeding to Phase 2"
  resume_phase_num = 2
Continue: Resume from Phase 2 (safe default)
```

**Error: Iteration counter lost (undefined)**
```
Recovery:
  iteration_count = 1  # Reset to default
  Display: "⚠️ Iteration counter reset to 1"
Continue: Proceed with counter reset
```

**Error: TodoWrite update fails**
```
Recovery:
  Display warning
  Proceed without TodoWrite update (manual tracking)
  Log: "TodoWrite update failed during resumption"
Continue: Resumption still works (jump to phase)
```

---

## Edge Cases

### Edge Case 1: DoD reaches 100% during iteration

**Scenario:** Story at 87% in iteration 1, reaches 100% in iteration 2

**Behavior:**
- Iteration 2 Phase 4.5-R: Calculate completion = 100%
- Display: "✅ DoD 100% Complete - All work finished!"
- Proceed to Phase 5 (no more iterations needed)

**Validation:** Iteration counter stops when no longer needed

---

### Edge Case 2: User changes mind about deferrals

**Scenario:** Iteration 1 user says "continue", iteration 2 (after 3 more hours) user says "defer remaining work"

**Behavior:**
- Iteration 2 Phase 4.5: User asked again about deferrals
- User chooses "defer" this time
- Phase 4.5-R: Detects user_decision = "approved_deferrals"
- Proceeds to Phase 5 (no more resumption)

**Validation:** User can change decision between iterations

---

### Edge Case 3: Resumption phase already complete

**Scenario:** All implementation done, only documentation incomplete, resume_phase_num = 3

**Behavior:**
- Jump to Phase 3
- refactoring-specialist finds no refactoring needed (code already clean)
- code-reviewer validates documentation
- Light QA passes
- Phase 4: integration-tester finds all tests passing
- Phase 4.5-R: DoD now 100%
- Proceeds to Phase 5

**Validation:** Resumption works even if resumed phase has little work left

---

### Edge Case 4: Multiple resumptions needed

**Scenario:** Story needs 3 iterations (87% → 93% → 98% → 100%)

**Behavior:**
- Iteration 1: 87% complete, resume from Phase 2
- Iteration 2 Phase 4.5-R: 93% complete (gained 6%), resume from Phase 3
- Iteration 3 Phase 4.5-R: 98% complete (gained 5%), resume from Phase 2
- Iteration 4 Phase 4.5-R: 100% complete, proceed to Phase 5

**Validation:** Multiple resumptions work correctly, each targeting appropriate phase

---

### Edge Case 5: Iteration limit with high completion

**Scenario:** After 5 iterations, story at 98% complete (only 2 DoD items remaining)

**Behavior:**
- Phase 4.5-R: iteration_count = 5, completion_pct = 98
- Display: Iteration limit reached
- Show: Only 2 items remaining
- AskUserQuestion with 4 options
- User likely chooses: "Continue (one more iteration)" since nearly done
- Extend limit to 6, resume one more time
- Iteration 6 finishes last 2 items, reaches 100%

**Validation:** Iteration limit is guidance, not absolute block (user can extend)

---

## Testing Procedures

### Test 1: Basic Resumption (Single Iteration)

**Setup:**
1. Create test story with 20 DoD items
2. Implement 70% (14 items) in Phase 2
3. Run `/dev TEST-STORY`

**Execute:**
4. Phase 4.5: User chooses "Continue to 100%"
5. Phase 4.5-R: Detects 30% incomplete, resumes from Phase 2

**Verify:**
- [ ] DoD completion calculated as 70%
- [ ] Resume phase determined as Phase 2
- [ ] TodoWrite updated (Phase 2-7 marked pending)
- [ ] Iteration counter = 2
- [ ] Workflow jumps to Phase 2
- [ ] Phase 2 continues implementation
- [ ] After Phase 2-4.5 complete, Phase 4.5-R checks again
- [ ] If now 100%, proceeds to Phase 5

**Success:** Story completes in 2 iterations

---

### Test 2: Multiple Resumptions

**Setup:**
1. Create test story with 30 DoD items (complex story)
2. Implement 60% (18 items) in first pass

**Execute:**
3. Iteration 1 Phase 4.5-R: 60% complete, resume from Phase 2
4. Iteration 2 adds 20% (now 80%), resume from Phase 3
5. Iteration 3 adds 15% (now 95%), resume from Phase 2
6. Iteration 4 adds final 5% (now 100%), proceed to Phase 5

**Verify:**
- [ ] Each iteration correctly calculates progress
- [ ] Resumption phase varies based on unchecked category
- [ ] Iteration counter tracks 1→2→3→4
- [ ] Final iteration reaches 100%
- [ ] Story commits successfully

**Success:** Story completes in 4 iterations

---

### Test 3: Iteration Limit Reached

**Setup:**
1. Create very large test story (60 DoD items)
2. Each iteration completes ~15% (slow progress)

**Execute:**
3. Iterations 1-5 reach 15%, 30%, 45%, 60%, 75%
4. Iteration 5 Phase 4.5-R: Limit reached at 75%

**Verify:**
- [ ] Warning displayed about iteration limit
- [ ] User shown 4 options (show items, continue, defer, explain blockers)
- [ ] User chooses "Continue"
- [ ] Limit extended to 6
- [ ] Iteration 6 executes
- [ ] OR: User chooses "Defer", deferrals documented with approval

**Success:** Limit is guidance (user can override), not absolute block

---

### Test 4: DoD Reaches 100% Mid-Iteration

**Setup:**
1. Story at 95% after iteration 1
2. Iteration 2 completes final 5% during Phase 2

**Execute:**
3. Iteration 2 Phase 2: Implements last items, marks DoD checkboxes
4. Phase 3: Light QA passes
5. Phase 4: Integration tests pass
6. Phase 4.5-R: Calculates completion = 100%

**Verify:**
- [ ] Phase 4.5-R detects 100% completion
- [ ] Displays "✅ DoD 100% Complete"
- [ ] Proceeds to Phase 5 (no more resumption)
- [ ] Story commits successfully

**Success:** Resumption stops when no longer needed

---

### Test 5: User Approves Deferrals After Multiple Iterations

**Setup:**
1. Story requires 3 iterations to reach 90%
2. Remaining 10% has blockers (external dependency)

**Execute:**
3. Iterations 1-3 reach 70%, 82%, 90%
4. Iteration 3 Phase 4.5: User chooses "Defer remaining work"
5. Phase 4.5-R: Detects user_decision = "approved_deferrals"

**Verify:**
- [ ] Phase 4.5-R skips resumption logic
- [ ] Displays "✅ Deferrals Documented"
- [ ] Proceeds to Phase 5
- [ ] Story commits with deferral documentation

**Success:** User can approve deferrals at any iteration

---

## Backward Compatibility

**This phase is NEW (RCA-013).**

**Existing stories:**
- No impact (Phase 4.5-R only triggers if user rejects deferrals AND DoD <100%)
- Most stories complete in 1 iteration (Phase 4.5-R sees 100%, skips to Phase 5)
- Stories with approved deferrals proceed normally (Phase 4.5-R skips resumption)

**New behavior:**
- ONLY affects stories where user explicitly wants to continue to 100%
- Previously: Workflow would stop, display "incomplete", user re-runs `/dev`
- Now: Workflow automatically resumes, completes to 100%, user happy

**Breaking changes:** None (additive enhancement)

---

## Reference Updates Required

**File:** `.claude/skills/devforgeai-development/references/phase-resumption-workflow.md` (THIS FILE)
- **Status:** NEW
- **Lines:** ~400
- **Created:** 2025-11-22

**Also update:**
1. `tdd-patterns.md` - Add section on iteration best practices
2. `dod-validation-checkpoint.md` - Reference Phase 4.5-R for resumption cases
3. Add to reference files list in SKILL.md line 724-735

---

## Success Indicators

**When Phase 4.5-R works correctly:**

1. ✓ User rejects deferrals → Work continues automatically (no re-run needed)
2. ✓ Multi-iteration stories complete to 100% without user intervention
3. ✓ Iteration counter provides transparency (user sees "Iteration 2/5")
4. ✓ Iteration limit prevents infinite loops
5. ✓ User can extend limit when nearly complete
6. ✓ Clear resumption messages show which phase restarting and why
7. ✓ TodoWrite tracker updates correctly for each iteration

**User experience improvement:**
- **Before RCA-013:** User runs `/dev STORY-057` → 87% → re-run → 87% again → frustrated
- **After RCA-013:** User runs `/dev STORY-057` → Phase 4.5-R resumes → 100% → committed → happy

---

**End of Phase 4.5-R Reference**

**Version:** 1.0
**RCA:** RCA-013 (Development Workflow Stops Before Completion)
**Implementation Date:** 2025-11-22
