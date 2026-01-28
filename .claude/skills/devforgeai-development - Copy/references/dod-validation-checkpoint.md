# Definition of Done Validation Checkpoint

**Skill Reference:** devforgeai-development
**Phase:** Phase 08, Step 1b (Git Workflow - Layer 2 Validation)
**Loaded:** On-demand via progressive disclosure (only when DoD items incomplete)
**Token Cost:** ~5,000-10,000 tokens (separate from main skill context)

**Purpose:** Mandatory user interaction gate for all incomplete Definition of Done items. Prevents autonomous deferrals and ensures user approval for all deferred work.

**Integration Pattern:** The devforgeai-development skill loads this reference via:
```
Read(file_path=".claude/skills/devforgeai-development/references/dod-validation-checkpoint.md")
```

**Progressive Disclosure:** This reference is NOT loaded unless the story has incomplete DoD items, optimizing token usage. Only loaded after TDD cycle completes (Phases 02-05) and Layer 1 validation (Python format check) passes.

**Enforcement:** Git commit is BLOCKED until this checkpoint passes.

---

## Skill Context Available

When this reference is loaded by the skill, the following context is available:

- **STORY_ID:** Extracted from conversation (e.g., STORY-006)
- **STORY_FILE:** Path to story file (devforgeai/specs/Stories/STORY-006.story.md)
- **Story Content:** Already loaded in conversation via @ reference from /dev command
- **Completion Status:** TDD cycle complete, Layer 1 validation passed
- **Variables from Phase 01:**
  - `$WORKFLOW_MODE` (git_based or file_based)
  - `$GIT_AVAILABLE` (true or false)
  - `$TEST_COMMAND` (detected test framework command)

**The skill has already completed:**
- ✅ Phase 01: Pre-Flight Validation (Git, context files, tech stack)
- ✅ Phase 02-05: TDD Cycle (Red → Green → Refactor → Integration)
- ✅ Phase 08 Step 1a: Layer 1 validation (Python format check)
- ⏳ Phase 08 Step 1b: NOW - This checkpoint (Layer 2)

---

## Input Requirements

From skill context (available in conversation when this reference loads):
- **STORY_ID:** Story identifier (e.g., STORY-006) - extracted by skill from conversation
- **STORY_FILE:** Full path to story file (e.g., devforgeai/specs/Stories/STORY-006-feature.story.md)
- **Story content:** Already loaded in conversation via @ reference from /dev command

**Note:** The skill has already extracted story ID and validated story file exists before loading this reference.

---

## Execution Protocol

### Step 1: Extract Incomplete DoD Items

<extraction_phase>
  <method>Use Grep tool to find all incomplete DoD items</method>
  <pattern>Lines starting with "- [ ]" in Definition of Done section</pattern>
  <efficiency>Native Grep tool (60% token savings vs Bash grep)</efficiency>
</extraction_phase>

**Use Grep to find incomplete items:**

```
Grep(
  pattern="^- \[ \]",
  path="${STORY_FILE}",
  output_mode="content",
  -A=2
)
```

**Parse results:**
```
incomplete_items = []

FOR each match from Grep:
  Extract item text (line starting with "- [ ]")
  Extract context (next 2 lines via -A=2 parameter)

  Check if item has deferral justification:
    - Look for "Deferred to STORY-XXX:" in context lines
    - Look for "Blocked by:" in context lines
    - Look for "Out of scope: ADR-XXX" in context lines

  IF no justification found:
    incomplete_items.append({
      text: item_text,
      has_justification: false
    })
  ELSE:
    # Item already has justification (from previous iteration)
    # Skip - user already approved this deferral
    continue
```

**Result:**
- `incomplete_items` = List of DoD items WITHOUT justifications
- These require MANDATORY user interaction

---

### Step 2: Enforce User Interaction for Each Item

<enforcement_checkpoint>
  <policy>ZERO autonomous deferrals</policy>
  <mechanism>AskUserQuestion REQUIRED for every unjustified incomplete item</mechanism>
  <consequence>Git commit BLOCKED until all items have user approval</consequence>
</enforcement_checkpoint>

**IF incomplete_items is empty:**
```
Display: "✓ All DoD items either complete or have user-approved justifications"
Proceed to Step 3 (exit checkpoint)
```

**IF incomplete_items contains items:**
```
Display:
"🛑 MANDATORY CHECKPOINT: {len(incomplete_items)} DoD items require user approval

The following items are incomplete and lack justification:
{list each item}

You MUST make a decision for EACH item.
Autonomous deferral is FORBIDDEN."

FOR EACH item in incomplete_items:

  <halt_condition>
    <trigger>Incomplete DoD item without user approval</trigger>
    <item>'{item.text}'</item>
    <action>MANDATORY AskUserQuestion - CANNOT SKIP</action>
  </halt_condition>

  AskUserQuestion:
    Question: "DoD item incomplete: '{item.text}'. How should we proceed?"
    Header: "DoD Item"
    Options:
      - "Complete it now (I'll wait while you implement)"
      - "Defer to story (I'll provide story ID)"
      - "Scope change (I'll document in ADR)"
      - "External blocker (I'll describe dependency)"
    multiSelect: false

  user_selection = response

  BASED ON user_selection:

  ## Option 1: "Complete it now"
  ```
  Display: "Returning to implementation for: {item.text}"

  HALT this checkpoint
  Return control to devforgeai-development skill

  Instructions for /dev command:
  "Re-invoke Skill(command='devforgeai-development') with focus on completing: {item.text}

   After implementation, the skill will return to Phase 08 Step 1b to revalidate DoD items.
   This checkpoint will be invoked again to check remaining items."
  ```

  ## Option 2: "Defer to story"
  ```
  AskUserQuestion:
    Question: "What story should '{item.text}' be deferred to?"
    Header: "Deferral Target"
    Options:
      - "Create new story now (I'll provide details)"
      - "Existing story (I'll provide STORY-ID)"
      - "Technical debt backlog (I'll provide reason)"
    multiSelect: false

  IF "Create new story now":
    AskUserQuestion:
      Question: "Provide brief description for new story to track '{item.text}'"
      Header: "Story Description"
      # User provides free-form text

    story_description = user_input

    Task(
      subagent_type="requirements-analyst",
      description="Create follow-up story",
      prompt="Create follow-up story for deferred work:

              Parent Story: ${STORY_ID}
              Deferred DoD Item: '{item.text}'
              Description: '{story_description}'

              Requirements:
              - Extract acceptance criteria from deferred DoD item
              - Set prerequisite_stories: [${STORY_ID}]
              - Set epic: {current story's epic}
              - Set status: Backlog
              - Set priority: Medium (deferred work)

              Return new story ID and file path."
    )

    new_story_id = extract from subagent response
    new_story_file = extract from subagent response

    Verify story created:
    Read(file_path=new_story_file)

    justification = "Deferred to ${new_story_id}: ${story_description}"

  ELSE IF "Existing story":
    AskUserQuestion:
      Question: "What is the target story ID for '{item.text}'?"
      Header: "Story ID"
      # User provides STORY-XXX

    target_story_id = user_input

    Validate story exists:
    Glob(pattern="devforgeai/specs/Stories/${target_story_id}*.story.md")

    IF not found:
      Display: "⚠️ WARNING: Story ${target_story_id} doesn't exist yet. You must create it."

    AskUserQuestion:
      Question: "Provide brief reason for deferring '{item.text}' to ${target_story_id}"
      Header: "Justification"
      # User provides free-form text

    reason = user_input
    justification = "Deferred to ${target_story_id}: ${reason}"

  ELSE IF "Technical debt backlog":
    AskUserQuestion:
      Question: "Provide reason for adding '{item.text}' to technical debt backlog"
      Header: "Debt Reason"
      # User provides free-form text

    reason = user_input
    justification = "Technical debt: ${reason}. Tracked in devforgeai/technical-debt-register.md"

    # Update technical debt register
    Check if devforgeai/technical-debt-register.md exists
    IF not exists:
      Read template: .claude/skills/devforgeai-orchestration/assets/templates/technical-debt-register-template.md
      Write(devforgeai/technical-debt-register.md, template)

    Read(devforgeai/technical-debt-register.md)
    Append to "Open Debt Items":
      "- {item.text} (from ${STORY_ID}): ${reason} | Date: {current_date} | Status: Open"
    Edit to save changes

  # Update story file with justification
  Read(file_path="${STORY_FILE}")

  Edit(
    file_path="${STORY_FILE}",
    old_string="- [ ] {item.text}",
    new_string="- [ ] {item.text}
    {justification}"
  )

  Display: "✓ Deferral documented for: {item.text}"
  ```

  ## Option 3: "Scope change"
  ```
  AskUserQuestion:
    Question: "Create ADR documenting scope change for '{item.text}' now or later?"
    Header: "ADR Creation"
    Options:
      - "Create now (I'll provide justification)"
      - "I'll create manually later (provide ADR number)"
    multiSelect: false

  IF "Create now":
    AskUserQuestion:
      Question: "Why was '{item.text}' removed from scope?"
      Header: "Scope Change Reason"
      # User provides free-form justification

    scope_reason = user_input

    Task(
      subagent_type="architect-reviewer",
      description="Create scope change ADR",
      prompt="Create Architecture Decision Record for scope change:

              Story: ${STORY_ID}
              Descoped Item: '{item.text}'
              Reason: '{scope_reason}'

              Document:
              - Context: Why requirement was initially included
              - Decision: Remove from scope
              - Rationale: {scope_reason}
              - Consequences: Impact on system functionality
              - Alternatives: What alternatives were considered

              Return ADR number and file path."
    )

    adr_number = extract from subagent response
    adr_file = extract from subagent response

    Verify ADR created:
    Read(file_path=adr_file)

    justification = "Out of scope: ADR-${adr_number} documents scope change rationale"

  ELSE:
    AskUserQuestion:
      Question: "What ADR number documents the scope change for '{item.text}'?"
      Header: "ADR Number"
      # User provides ADR-XXX

    adr_number = user_input

    Validate ADR exists:
    Glob(pattern="devforgeai/specs/adrs/ADR-${adr_number}*.md")

    IF not found:
      Display: "⚠️ WARNING: ADR-${adr_number} doesn't exist. You must create it to document scope change."

    justification = "Out of scope: ADR-${adr_number}"

  # Update story file
  Read(file_path="${STORY_FILE}")

  Edit(
    file_path="${STORY_FILE}",
    old_string="- [ ] {item.text}",
    new_string="- [ ] {item.text}
    {justification}"
  )

  Display: "✓ Scope change documented for: {item.text}"
  ```

  ## Option 4: "External blocker"
  ```
  AskUserQuestion:
    Question: "Describe the external blocker preventing '{item.text}'"
    Header: "Blocker Details"
    # User provides free-form text
    # Example: "Payment API v2 not available until 2025-12-01"

  blocker_description = user_input

  # Validate truly external (sanity check)
  IF blocker_description contains ["our code", "our API", "our module", "our team"]:
    AskUserQuestion:
      Question: "This seems like an internal blocker. Is it truly external (outside your control)?"
      Header: "Blocker Type"
      Options:
        - "Yes - external dependency (we cannot control timeline)"
        - "No - internal (I can resolve it now)"
      multiSelect: false

    IF "No - internal":
      Display: "Treating as internal work. Returning to 'Complete it now' option."
      # Jump back to Option 1 logic
      GOTO Option 1 processing

  justification = "Blocked by: ${blocker_description}"

  # Update story file
  Read(file_path="${STORY_FILE}")

  Edit(
    file_path="${STORY_FILE}",
    old_string="- [ ] {item.text}",
    new_string="- [ ] {item.text}
    {justification}"
  )

  # Track in technical debt register
  Check if devforgeai/technical-debt-register.md exists
  IF not exists:
    Read template: .claude/skills/devforgeai-orchestration/assets/templates/technical-debt-register-template.md
    Write(devforgeai/technical-debt-register.md, template)

  Read(devforgeai/technical-debt-register.md)
  Append to "Open Debt Items":
    "- {item.text} (from ${STORY_ID}): Blocked by ${blocker_description} | Date: {current_date} | Status: Open"
  Edit to save changes

  Display: "✓ External blocker documented for: {item.text}"
  ```

# End of FOR EACH loop
```

---

### Step 3: Final Validation and Summary

<validation_summary>
  <requirement>All incomplete DoD items now have user-approved justifications</requirement>
  <check>Verify each item processed through AskUserQuestion</check>
  <output>Summary of deferrals with justifications</output>
</validation_summary>

**Display final summary:**

```
Display:
"✅ DoD Validation Checkpoint PASSED

Definition of Done Status:
- Total Items: {total_count}
- Complete: {complete_count}
- Deferred: {deferred_count}
  - Story splits: {story_split_count} (follow-up stories created/referenced)
  - Scope changes: {scope_change_count} (ADRs created/referenced)
  - External blockers: {blocker_count} (tracked in tech debt register)

All deferrals have user approval and proper justification ✓

Story file updated with justifications.
Proceeding to git commit..."
```

**Exit checkpoint - return control to /dev command**

---

## Success Criteria

This checkpoint succeeds when:
- [ ] All incomplete DoD items identified via Grep
- [ ] User interaction completed for each unjustified item
- [ ] Story file updated with justifications
- [ ] Follow-up stories created (if user selected "Create now")
- [ ] ADRs created (if user selected "Create now")
- [ ] Technical debt register updated (if external blockers)
- [ ] All deferrals have proper justification format

**On success:** /dev command proceeds to git commit

**On failure:** /dev command HALTS, user must resolve issues

---

## Integration Notes

**Invoked by:** `/dev` command (after devforgeai-development skill completes)

**References:**
- `.claude/skills/devforgeai-development/SKILL.md` (Phase 08 Step 1b has similar logic)
- `.claude/skills/devforgeai-orchestration/assets/templates/technical-debt-register-template.md`
- RCA-006 Recommendation 1 (mandatory DoD checkpoint)

**Token Efficiency:**
- Uses native Grep tool (60% savings vs Bash grep)
- Uses Edit tool for updates (75% savings vs sed)
- Loads story file once (already in context)
- Estimated: 5,000-10,000 tokens per execution

**Character Count:** ~4,800 characters (well under 15K budget per task file)

---

## Error Handling

### Story File Not Loaded
```
ERROR: Story file not available in conversation context

Expected:
- Story file loaded via @ reference in /dev command
- OR story file path provided via conversation

Actions:
1. Verify /dev command includes: @devforgeai/specs/Stories/${STORY_ID}.story.md
2. Check story file exists: Glob(pattern="devforgeai/specs/Stories/${STORY_ID}*.story.md")
3. Re-run /dev command with proper story reference
```

### Grep Returns No Results
```
No incomplete DoD items found - all items are [x] complete.

This checkpoint is not needed.
Proceeding to git commit...
```

### Story File Update Fails
```
ERROR: Cannot update story file with justifications

Possible causes:
- File not readable (check permissions)
- old_string not unique (Edit tool requires unique match)
- File modified during checkpoint execution

Actions:
1. Read story file again: Read(file_path="${STORY_FILE}")
2. Verify item text matches exactly
3. Use Edit with unique context if needed
4. Re-run checkpoint
```

---

## Protocol Compliance

**This checkpoint enforces:**
- DevForgeAI zero technical debt policy
- RCA-006 findings (prevent autonomous deferrals)
- "Ask, Don't Assume" framework principle

**User approval required for:**
- Every deferred DoD item
- Every scope change
- Every external blocker
- Every technical debt entry

**No exceptions. No autonomous decisions.**
