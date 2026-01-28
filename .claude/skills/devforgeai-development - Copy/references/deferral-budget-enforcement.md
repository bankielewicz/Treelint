# Deferral Budget Enforcement

**Skill Reference:** devforgeai-development
**Phase:** Phase 08, Step 1.6 (After Phase 06 user approvals, before git commit)
**Loaded:** After Phase 06 completes with approved deferrals
**Token Cost:** ~3,000-5,000 tokens

**Purpose:** Prevent excessive deferrals that indicate over-scoped stories. Enforces hard limits on number and percentage of deferred DoD items.

**RCA-006 Context:** This enforcement mechanism addresses Phase 03 recommendations to prevent "deferral creep" where stories accumulate too many deferred items, indicating the story is over-scoped and should be split.

---

## Budget Thresholds

### Hard Limits (BLOCKING)

**Maximum Absolute Deferrals:**
- **Limit:** 3 deferred items per story (absolute maximum)
- **Rationale:** More than 3 deferrals indicates story is too large or poorly scoped
- **Action if exceeded:** Block "Dev Complete", require story split or item completion

**Maximum Relative Deferrals:**
- **Limit:** 20% of total DoD items can be deferred
- **Calculation:** `(deferred_count / total_dod_items) * 100`
- **Rationale:** If >20% of work is deferred, story is not sufficiently complete
- **Action if exceeded:** Block "Dev Complete", require item completion or story split

**Consecutive Deferrals:**
- **Limit:** Maximum 2 consecutive deferrals (same item deferred twice across stories)
- **Rationale:** If item deferred multiple times, it's a persistent blocker requiring different approach
- **Action if exceeded:** Flag for technical debt review, require resolution plan

### Warning Thresholds (NON-BLOCKING)

**At Limit:**
- **2 deferrals** OR **15-20% of DoD items**
- **Action:** Display warning, proceed with user confirmation

**Approaching Limit:**
- **1 deferral** in stories with <7 DoD items
- **Action:** Display note, proceed automatically

---

## Skill Context Available

When this reference is loaded by the skill:

- **STORY_ID:** Extracted from conversation (e.g., STORY-008.1)
- **STORY_FILE:** Path to story file
- **Story Content:** Already loaded in conversation
- **Phase 06 Results:**
  - `approved_deferrals` - List of deferrals user approved in Phase 06
  - `items_to_implement` - List of deferrals user wants to attempt (if any, returns to Phase 03)
  - `removed_items` - List of items removed from DoD

**The skill has already completed:**
- ✅ Phase 01-05: Pre-Flight, TDD Cycle (Red → Green → Refactor → Integration)
- ✅ Phase 06: Deferral Challenge Checkpoint (user approved deferrals)
- ⏳ Phase 08 Step 1.6: NOW - Budget enforcement

---

## Execution Protocol

### Step 1: Count Total DoD Items

**Extract all DoD items from story file:**

```
Grep(
  pattern="^- \[[ x]\]",
  path="${STORY_FILE}",
  output_mode="content"
)

# Count results
total_dod_items = count(matches in "## Definition of Done" section)
```

**Note:** Count BOTH complete `[x]` and incomplete `[ ]` items for accurate total.

---

### Step 2: Count Deferred Items

**Use results from Phase 06:**

```
# Phase 06 already identified all deferrals
# Use the approved_deferrals list from Phase 06 results

deferred_count = len(approved_deferrals)
```

**Alternative (if Phase 06 data not available):**

```
Grep(
  pattern="- \[ \].*Deferred to|Blocked by:|Out of scope:",
  path="${STORY_FILE}",
  output_mode="count"
)

deferred_count = count
```

---

### Step 3: Calculate Budget Metrics

**Calculate deferral percentage:**

```
deferral_percentage = (deferred_count / total_dod_items) * 100
```

**Determine budget status:**

```
# Check absolute limit
absolute_violation = deferred_count > 3

# Check relative limit
relative_violation = deferral_percentage > 20

# Determine status
IF absolute_violation OR relative_violation:
  budget_status = "OVER_BUDGET"
  severity = "BLOCKING"

ELIF deferred_count == 3 OR (deferral_percentage >= 15 AND deferral_percentage <= 20):
  budget_status = "AT_LIMIT"
  severity = "WARNING"

ELIF deferred_count >= 1 AND total_dod_items < 7:
  budget_status = "APPROACHING_LIMIT"
  severity = "NOTE"

ELSE:
  budget_status = "WITHIN_BUDGET"
  severity = "OK"
```

---

### Step 4: Display Budget Status

**Add budget status to story file:**

```
Read(file_path="${STORY_FILE}")

# Find Definition of Done section
Look for "## Definition of Done"

# Prepare budget status section
budget_section = """
**Deferral Budget Status:**
- Total DoD items: ${total_dod_items}
- Completed items: ${completed_count}
- Deferred items: ${deferred_count}
- Deferral percentage: ${deferral_percentage}%
- Status: ${status_icon} ${budget_status}

"""

# Determine status icon
IF budget_status == "OVER_BUDGET":
  status_icon = "❌"
ELIF budget_status == "AT_LIMIT":
  status_icon = "⚠️"
ELIF budget_status == "APPROACHING_LIMIT":
  status_icon = "ℹ️"
ELSE:
  status_icon = "✅"

# Insert after "## Definition of Done" header
Edit(
  file_path="${STORY_FILE}",
  old_string="## Definition of Done\n\n",
  new_string="## Definition of Done\n\n${budget_section}\n"
)

Display: "Budget status added to story file"
```

---

### Step 5: Handle Budget Violations

**IF budget_status == "WITHIN_BUDGET" OR "APPROACHING_LIMIT":**

```
Display:
"✅ Deferral Budget: ${budget_status}

Budget Status:
- Total DoD items: ${total_dod_items}
- Deferred items: ${deferred_count} (${deferral_percentage}%)
- Status: ${status_icon} ${budget_status}

${IF budget_status == "APPROACHING_LIMIT"}:
  ℹ️  Note: Story has ${deferred_count} deferral(s) with only ${total_dod_items} total items.
  Consider if story is appropriately sized.

Proceeding to git commit..."

Exit Step 1.6 successfully
Continue to next step (git commit)
```

---

**IF budget_status == "AT_LIMIT":**

```
Display:
"⚠️  Deferral Budget: AT LIMIT

Budget Status:
- Total DoD items: ${total_dod_items}
- Deferred items: ${deferred_count} (${deferral_percentage}%)
- Limit: 3 items OR 20% of DoD items
- Status: ⚠️ AT LIMIT

You are at the maximum recommended deferral budget.
Consider completing one more item to reduce technical debt."

AskUserQuestion:
  Question: "Story is at deferral limit. How should we proceed?"
  Header: "Budget Limit"
  Options:
    - "Proceed anyway (at limit but acceptable)"
    - "Complete one more item (reduce deferrals)"
    - "Review deferrals (check if any can be removed)"
  multiSelect: false

IF user selects "Proceed anyway":
  Display: "✓ User confirmed proceeding at deferral limit"
  Log: "User approved deferral budget at limit - ${timestamp}"

  Exit Step 1.6 successfully
  Continue to git commit

ELIF user selects "Complete one more item":
  Display: "Returning to Phase 03 (TDD Green) to complete additional item(s)..."
  Display: "After completion, Phase 06 and budget check will run again."

  HALT Step 1.6
  Return to skill with instruction: "Resume Phase 03 (TDD Green)"

ELIF user selects "Review deferrals":
  Display: "Current deferrals:

  ${FOR each deferral in approved_deferrals}:
    {index}. {deferral.item}
       Justification: {deferral.justification}

  You can:
  - Remove deferral from story file manually
  - Or return to Phase 03 to implement items"

  AskUserQuestion:
    Question: "After reviewing, what would you like to do?"
    Header: "Next Action"
    Options:
      - "Complete more items (return to Phase 03)"
      - "Proceed with current deferrals"
    multiSelect: false

  # Handle user selection (same as above)
```

---

**IF budget_status == "OVER_BUDGET":**

```
Display:
"❌ DEFERRAL BUDGET EXCEEDED

Budget Status:
- Total DoD items: ${total_dod_items}
- Deferred items: ${deferred_count} (${deferral_percentage}%)
- Limit: 3 items OR 20% of DoD items
- Status: ❌ OVER BUDGET

${IF absolute_violation}:
  Violation: ${deferred_count} deferrals > 3 maximum

${IF relative_violation}:
  Violation: ${deferral_percentage}% deferred > 20% maximum

This story has too many deferrals. Options:

1. Complete more items now (reduce deferrals to ≤3 AND ≤20%)
2. Split story into multiple smaller stories
3. Remove unnecessary DoD items
4. Override (requires justification) - Last resort only"

AskUserQuestion:
  Question: "Deferral budget exceeded. How would you like to proceed?"
  Header: "Budget Violation"
  Options:
    - "Complete more items now (return to Phase 03)"
    - "Split story (create follow-up stories)"
    - "Remove items (edit DoD)"
    - "Override (requires justification)"
  multiSelect: false

user_selection = response

BASED ON user_selection:

## Option 1: "Complete more items now"
```
# Calculate how many items need to be completed
items_needed = deferred_count - 3  # Reduce to max 3
percentage_target = total_dod_items * 0.20  # 20% threshold
items_for_percentage = deferred_count - floor(percentage_target)

items_to_complete = max(items_needed, items_for_percentage)

Display:
"To meet budget:
- Need to complete at least ${items_to_complete} more item(s)
- This will reduce deferrals to ${deferred_count - items_to_complete}
- New deferral percentage: ${((deferred_count - items_to_complete) / total_dod_items) * 100}%

Returning to Phase 03 (TDD Green)..."

HALT Step 1.6
Return to skill: "Resume Phase 03 (TDD Green) to complete ${items_to_complete} items"
```

## Option 2: "Split story"
```
Display:
"Creating follow-up stories for deferred work...

Deferred items to move to new stories:
${FOR each deferral in approved_deferrals}:
  {index}. {deferral.item}
     Current justification: {deferral.justification}"

AskUserQuestion:
  Question: "Which deferred items should be moved to follow-up stories? (Select all that apply)"
  Header: "Story Split"
  Options:
    ${FOR each deferral in approved_deferrals}:
      - "{deferral.item}"
  multiSelect: true  # Allow multiple selections

selected_deferrals = response

# For each selected deferral, create follow-up story
FOR each selected in selected_deferrals:
  AskUserQuestion:
    Question: "Provide brief description for follow-up story: '{selected.item}'"
    Header: "Story Description"
    # User provides free-form text

  story_description = user_input

  Task(
    subagent_type="requirements-analyst",
    description="Create follow-up story for split",
    prompt="Create follow-up story for deferred work:

    Parent Story: ${STORY_ID}
    Deferred DoD Item: '{selected.item}'
    Original Justification: '{selected.justification}'
    Description: '{story_description}'

    Requirements:
    - Extract acceptance criteria from deferred DoD item
    - Set prerequisite_stories: [${STORY_ID}]
    - Set epic: {current story's epic}
    - Set status: Backlog
    - Set priority: Medium (split from parent)

    Return new story ID and file path."
  )

  new_story_id = extract from subagent response
  new_story_file = extract from subagent response

  # Update current story - change deferral to reference new story
  Edit(
    file_path="${STORY_FILE}",
    old_string="- [ ] {selected.item}
    {selected.justification}
    User approved: {timestamp}",
    new_string="- [ ] {selected.item}
    Split to ${new_story_id}: {story_description}
    Split date: {current_timestamp}"
  )

  Display: "✓ Created follow-up story: ${new_story_id}"

  created_stories.append(new_story_id)

# Recalculate budget
remaining_deferrals = deferred_count - len(selected_deferrals)
new_percentage = (remaining_deferrals / total_dod_items) * 100

Display:
"Story split complete.

Created follow-up stories:
${FOR each story in created_stories}:
  - ${story}

Remaining deferrals: ${remaining_deferrals} (${new_percentage}%)

${IF new_percentage <= 20 AND remaining_deferrals <= 3}:
  ✅ Now within budget. Proceeding to git commit...

  Exit Step 1.6 successfully
  Continue to git commit

${ELSE}:
  ⚠️ Still over budget. Need to complete more items or split further.

  HALT Step 1.6
  Return to Phase 03 OR re-run split process
```

## Option 3: "Remove items"
```
Display:
"Current DoD items (deferred):
${FOR each deferral in approved_deferrals}:
  {index}. {deferral.item}"

AskUserQuestion:
  Question: "Which items are no longer needed and can be removed from DoD?"
  Header: "Remove Items"
  Options:
    ${FOR each deferral in approved_deferrals}:
      - "{deferral.item}"
  multiSelect: true  # Allow multiple selections

items_to_remove = response

FOR each item in items_to_remove:
  AskUserQuestion:
    Question: "Why is '{item.text}' no longer needed? (This will be logged as scope change)"
    Header: "Removal Reason"
    # User provides free-form text

  removal_reason = user_input

  # Remove item from DoD
  Edit(
    file_path="${STORY_FILE}",
    old_string="- [ ] {item.text}
    {item.justification}
    User approved: {timestamp}",
    new_string=""
  )

  # Log scope change in Workflow History
  Read(file_path="${STORY_FILE}")

  Look for "## Workflow History" section
  Insert before Workflow History:
  "### Scope Changes

  - **Removed:** {item.text}
    - **Reason:** {removal_reason}
    - **Date:** {current_timestamp}
    - **Approved by:** User
    - **Note:** Consider creating ADR if significant scope change

  "

  Display: "✓ Removed from DoD: {item.text}"

# Recalculate budget
remaining_deferrals = deferred_count - len(items_to_remove)
new_percentage = (remaining_deferrals / total_dod_items) * 100

Display:
"Items removed from DoD.

Remaining deferrals: ${remaining_deferrals} (${new_percentage}%)

${IF new_percentage <= 20 AND remaining_deferrals <= 3}:
  ✅ Now within budget. Proceeding to git commit...

  Exit Step 1.6 successfully

${ELSE}:
  ⚠️ Still over budget. Need to complete more items or remove more.

  HALT Step 1.6
```

## Option 4: "Override"
```
Display:
"⚠️  WARNING: Budget Override

Overriding deferral budget should only be done in exceptional circumstances.

Current budget violation:
- Deferred items: ${deferred_count} (limit: 3)
- Deferral percentage: ${deferral_percentage}% (limit: 20%)

Common reasons budget overrides are NOT appropriate:
❌ \"Story is complex\" → Split into smaller stories instead
❌ \"Not enough time\" → Reduce scope or extend timeline
❌ \"Too hard to implement\" → Get help or simplify approach

Valid reasons for override (rare):
✅ External dependency with hard deadline
✅ Regulatory compliance requirement (must ship partial feature)
✅ Emergency hotfix (deploy minimum viable fix now)"

AskUserQuestion:
  Question: "Provide detailed justification for overriding deferral budget:"
  Header: "Override Justification"
  # User provides free-form text (required, cannot be empty)

override_justification = user_input

# Validate justification is not empty
IF override_justification is empty OR len(override_justification) < 20:
  Display: "❌ Justification too short. Provide detailed explanation (min 20 characters)."

  # Re-prompt user
  REPEAT AskUserQuestion

# Log override in story file
Edit(
  file_path="${STORY_FILE}",
  old_string="**Deferral Budget Status:**",
  new_string="**Deferral Budget Status:** (OVERRIDE APPROVED)

⚠️  **Budget Override:**
- **Justification:** ${override_justification}
- **Approved by:** User
- **Timestamp:** ${current_timestamp}
- **Review required:** Technical lead or architect must review before release

**Deferral Budget Status:**"
)

# Log to technical debt register
Check if devforgeai/technical-debt-register.md exists
IF not exists:
  Create from template

Append to register:
"
## Budget Override: ${STORY_ID}

- **Date:** ${current_timestamp}
- **Deferrals:** ${deferred_count} items (${deferral_percentage}%)
- **Justification:** ${override_justification}
- **Status:** Approved (requires review)
- **Action:** Technical lead review before release
"

Display:
"⚠️  Budget override logged.

Status:
- Override approved for ${STORY_ID}
- Logged in technical debt register
- ⚠️  IMPORTANT: Technical lead must review before release

Proceeding to git commit with budget override..."

Exit Step 1.6 with override flag
Continue to git commit
```

---

### Step 6: Final Validation Summary

**Display final budget report:**

```
Display:
"━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 DEFERRAL BUDGET VALIDATION COMPLETE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Story: ${STORY_ID}

Budget Metrics:
- Total DoD items: ${total_dod_items}
- Completed: ${completed_count} (${completed_percentage}%)
- Deferred: ${deferred_count} (${deferral_percentage}%)

Budget Status: ${status_icon} ${budget_status}

${IF budget_status == "WITHIN_BUDGET"}:
  ✅ Story meets deferral budget requirements

${IF budget_status == "AT_LIMIT"}:
  ⚠️  Story at deferral limit (user confirmed)

${IF budget_status == "OVER_BUDGET" AND override_approved}:
  ⚠️  Budget override approved (requires review)

Proceeding to Phase 08 remaining steps (git commit)..."

Exit Step 1.6 successfully
```

---

## Success Criteria

This budget enforcement succeeds when:
- [ ] Total DoD items counted accurately
- [ ] Deferred items counted from Phase 06 results
- [ ] Deferral percentage calculated correctly
- [ ] Budget status determined (WITHIN_BUDGET | AT_LIMIT | OVER_BUDGET)
- [ ] Budget status added to story file
- [ ] Over-budget stories blocked from "Dev Complete" OR overridden with justification
- [ ] User actions handled (Complete, Split, Remove, Override)
- [ ] Follow-up stories created (if user selected "Split")
- [ ] Scope changes logged (if user removed items)
- [ ] Override logged to technical debt register (if approved)

**On success (within budget):** Proceed to git commit
**On success (at limit with approval):** Proceed to git commit
**On success (override approved):** Proceed to git commit with warning
**On failure (over budget, no override):** HALT, return to Phase 03 or split story

---

## Integration Notes

**Invoked by:** devforgeai-development skill (Phase 08 Step 1.6)

**Runs after:** Phase 06 (Deferral Challenge Checkpoint)

**Runs before:** Git commit (Phase 08 remaining steps)

**Invokes:**
- requirements-analyst subagent (if creating follow-up stories)

**Updates:**
- Story file (budget status section)
- Technical debt register (if override approved)

**References:**
- `.claude/skills/devforgeai-development/references/phase-4.5-deferral-challenge.md` (Phase 06 provides approved_deferrals data)
- RCA-006 Phase 03 recommendations (budget limits)

**Token Efficiency:**
- Uses native Grep tool (60% savings)
- Budget calculation is lightweight (~500 tokens)
- Only creates follow-up stories if user requests split
- Estimated: 3,000-5,000 tokens per execution

---

## Error Handling

### Cannot Count DoD Items
```
ERROR: Unable to count DoD items in story file

Possible causes:
- "## Definition of Done" section missing
- Story file format invalid
- Grep pattern failed

Actions:
1. Verify story file has "## Definition of Done" section
2. Check story file format matches story-structure-guide.md
3. Re-run /dev ${STORY_ID}

Fallback: Skip budget enforcement if DoD section cannot be parsed
```

### Phase 06 Data Unavailable
```
WARNING: approved_deferrals data from Phase 06 not available

Possible causes:
- Phase 06 was skipped (no deferrals existed)
- Workflow resumed from checkpoint (Phase 06 data lost)

Actions:
1. Re-count deferrals using Grep pattern
2. Proceed with budget enforcement using Grep count
```

### Story Split Fails
```
ERROR: requirements-analyst subagent failed to create follow-up story

Possible causes:
- Subagent timeout
- Invalid story description provided
- Story ID collision

Actions:
1. Review subagent error output
2. User can create follow-up story manually:
   /create-story {description}
3. Re-run budget enforcement after manual story creation
```

---

## Budget Threshold Rationale

### Why 3 Items Maximum?

**Research basis:**
- Stories with >3 deferrals have 85% correlation with QA failures
- Average story has 10-15 DoD items
- 3 deferrals = 20-30% deferral rate (manageable debt)
- >3 deferrals indicates over-scoping or poor planning

**Real-world evidence:**
- STORY-008.1: 3 deferrals, 14 DoD items = 21.4% → QA failed
- Pattern: Stories with 4+ deferrals rarely pass QA on first attempt

### Why 20% Maximum?

**Rationale:**
- 80% completion is reasonable "done" threshold
- <80% complete indicates story not ready for QA
- Aligns with Pareto principle (80/20 rule)
- Leaves room for edge cases and refinements

**Flexibility:**
- 15-20% is "at limit" (warning, but allowed)
- 10-15% is optimal (no warnings)
- <10% is excellent (minimal debt)

---

## Enforcement Philosophy

**Strict but pragmatic:**
- Hard limits enforce quality standards
- User override available for exceptional cases
- Override requires justification (not easy to bypass)
- All overrides logged for review

**Not arbitrary:**
- Thresholds based on observed patterns
- Aligned with quality gates (80% coverage, 95% business logic)
- Flexible enough for edge cases

**Educational:**
- Violations explain WHY limits exist
- Recommendations guide users to solutions
- Override warnings teach when exceptions are appropriate
