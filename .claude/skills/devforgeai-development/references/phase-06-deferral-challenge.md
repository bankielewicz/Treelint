# Phase 06: Deferral Challenge Checkpoint

**Skill Reference:** devforgeai-development
**Phase:** Phase 06 (After Integration Testing, Before DoD Update Bridge)
**Loaded:** After Phase 05 completes, if story has any deferred DoD items
**Token Cost:** ~8,000-12,000 tokens (separate from main skill context)

**Purpose:** Challenge ALL deferred Definition of Done items (pre-existing + new) to prevent autonomous deferrals and ensure user approval. This is the PRIMARY enforcement mechanism for RCA-006.

**Trigger:** After Phase 05 (Integration Testing) completes successfully, before Phase 07

**Integration Pattern:** The devforgeai-development skill loads this reference via:
```
Read(file_path=".claude/skills/devforgeai-development/references/phase-4.5-deferral-challenge.md")
```

---

## Phase Progress Indicator

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 06/9: Deferral Challenge (56% → 67% complete)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Display this indicator at the start of Phase 06 execution.**

**Progressive Disclosure:** This reference is loaded ONLY if story has deferred DoD items (pre-existing or new), optimizing token usage.

**RCA-006 Context:** This checkpoint addresses the root cause identified in RCA-006 where pre-justified deferrals in story templates bypassed validation, allowing autonomous deferrals without user approval.

---

## Skill Context Available

When this reference is loaded by the skill, the following context is available:

- **STORY_ID:** Extracted from conversation (e.g., STORY-008.1)
- **STORY_FILE:** Path to story file (devforgeai/specs/Stories/STORY-008.1-feature.story.md)
- **Story Content:** Already loaded in conversation via @ reference from /dev command
- **Completion Status:** Phases 01-05 complete (Pre-Flight, Red, Green, Refactor, Integration)
- **Variables from Phase 01:**
  - `$WORKFLOW_MODE` (git_based or file_based)
  - `$GIT_AVAILABLE` (true or false)
  - `$TEST_COMMAND` (detected test framework command)

**The skill has already completed:**
- ✅ Phase 01: Pre-Flight Validation (Git, context files, tech stack)
- ✅ Phase 02-05: TDD Cycle (Red → Green → Refactor → Integration)
- ⏳ Phase 06: NOW - This checkpoint (Deferral Challenge)

---

## Understanding DoD Validation (Two Validators, Different Purposes)

**This phase uses TWO validators with different purposes. Understanding the distinction prevents confusion.**

### Validator Comparison

| Aspect | deferral-validator (AI) | devforgeai-validate validate-dod (CLI) |
|--------|------------------------|--------------------------------|
| **Type** | AI subagent (Claude) | Python script (deterministic) |
| **Runs** | Phase 06 Step 3 | Pre-commit hook (Phase 08) |
| **Checks** | Semantic justification validity | Format compliance (DoD ↔ Impl Notes) |
| **Validates** | • Circular deferrals<br>• Blocker accuracy<br>• Story references exist | • DoD [x] items in Impl Notes<br>• Text match exact<br>• Format (flat list, no subsections) |
| **Output** | Recommendations, resolvable vs. valid | PASS/FAIL, fix instructions |
| **Speed** | ~5K tokens, ~30 seconds | <100ms, deterministic |
| **Can HALT** | No (advisory) | Yes (blocks git commit) |

### Workflow Handoff

The two validators work together in sequence:

1. **Phase 06 (This checkpoint):** deferral-validator validates semantic correctness
   - Are blockers real? (dependency stories exist, toolchains missing, etc.)
   - Are deferrals circular? (STORY-A defers to STORY-B which defers to STORY-A)
   - Do story/ADR references exist?
   - Returns: Resolvable vs. valid categories

2. **Phase 07 (DoD Update):** Update DoD items in correct format
   - Mark completed items [x] in Definition of Done section
   - Add items to Implementation Notes (FLAT LIST - no ### subsections)
   - See: `dod-update-workflow.md` for detailed formatting requirements

3. **Phase 08 (Git Commit):** devforgeai-validate validate-dod validates format before commit
   - Runs as pre-commit hook (automatic)
   - Checks DoD [x] items appear in Implementation Notes
   - Checks text matches exactly
   - Checks format (no subsection headers)
   - BLOCKS commit if validation fails (exit code 1)

**Both validators must pass for successful Phase 08 completion.**

---

## Checkpoint Workflow

### Step 1: Detect All Incomplete DoD Items [MANDATORY]

**CRITICAL CHANGE (RCA-014):** Detect ANY unchecked DoD item, not just explicitly deferred items.

<detection_phase>
  <method>Grep for ALL unchecked DoD items (explicit and implicit deferrals)</method>
  <pattern>Lines with "- [ ]" in Definition of Done section</pattern>
  <scope>ALL incomplete work (explicit deferrals + implicit deferrals without justification)</scope>
  <rationale>CLAUDE.md mandates "Deferrals are not acceptable!" - leaving work unchecked IS a deferral (implicit)</rationale>
</detection_phase>

**Purpose:** Enforce CLAUDE.md directive by treating ALL incomplete DoD work as requiring user approval.

**Rationale (RCA-014):** The original detection only caught EXPLICIT deferrals (with "Deferred to..." text), creating a loophole for IMPLICIT deferrals (plain unchecked boxes). This fix closes that loophole per user guidance: "Deferrals are not acceptable! HALT! on deferrals of implementation."

**Use Grep to find ALL unchecked DoD items:**

```
Grep(
  pattern="^- \[ \]",
  path="${STORY_FILE}",
  output_mode="content",
  -B=1,
  -A=3
)
```

**Parse results to identify incomplete items:**

```
incomplete_items = []
in_dod_section = false

FOR each match from Grep:
  item_text = line starting with "- [ ]"
  preceding_line = previous line (via -B=1)
  context_lines = next 3 lines (via -A=3)

  # Section filtering: Only process items in Definition of Done section
  # Skip AC Checklist, Implementation Notes, other sections

  IF preceding_line contains "## Definition of Done":
    in_dod_section = true

  IF preceding_line contains "## Acceptance Criteria":
    in_dod_section = false

  IF preceding_line contains "## Workflow Status":
    in_dod_section = false

  IF preceding_line contains "## Implementation Notes":
    in_dod_section = false

  IF preceding_line contains "### AC#":
    in_dod_section = false  # AC Checklist section

  IF preceding_line contains "Checklist":
    in_dod_section = false

  # Only process if in DoD section
  IF NOT in_dod_section:
    CONTINUE  # Skip this item

  # Classify incomplete item
  classification = classify_incomplete_item(context_lines)
  justification = extract_justification(context_lines)

  incomplete_items.append({
    text: item_text,
    classification: classification,  # "explicit_deferral" | "implicit_deferral"
    justification: justification or "NONE",
    has_approval: check_for_approval(context_lines)
  })

FUNCTION classify_incomplete_item(context_lines):
  """
  Classify whether unchecked item has explicit deferral justification.

  Returns:
    - "explicit_deferral" if justification text found
    - "implicit_deferral" if NO justification (RCA-014 fix - treats as deferral)
  """
  # Check for explicit deferral justification
  FOR each line in context_lines:
    IF line contains "Deferred to STORY-":
      RETURN "explicit_deferral"
    IF line contains "Blocked by:":
      RETURN "explicit_deferral"
    IF line contains "Out of scope: ADR-":
      RETURN "explicit_deferral"
    IF line contains "Approved by user on":
      RETURN "explicit_deferral"  # User-approved deferral

  # No justification found = implicit deferral (BUG FIX!)
  RETURN "implicit_deferral"

FUNCTION extract_justification(context_lines):
  """Extract deferral justification text if present."""
  FOR each line in context_lines:
    IF line contains "Deferred to":
      RETURN line.strip()
    IF line contains "Blocked by:":
      RETURN line.strip()
    IF line contains "Out of scope:":
      RETURN line.strip()
    IF line contains "Approved by user on":
      RETURN line.strip()
  RETURN null

FUNCTION check_for_approval(context_lines):
  """Check if item has user approval timestamp."""
  FOR each line in context_lines:
    IF line contains "Approved by user on" AND line contains timestamp_pattern:
      RETURN true
  RETURN false
```

**Result:**
- `incomplete_items` = List of ALL incomplete DoD items (explicit + implicit deferrals)
- Includes BOTH pre-existing deferrals AND new incomplete work from TDD cycle
- Classification enables different handling: explicit deferrals validated by deferral-validator, implicit deferrals require immediate user approval

---

### Step 2: Skip Checkpoint if No Incomplete Items (RCA-014 Updated)

**IF incomplete_items is empty:**

```
Display: "✓ All DoD items complete (100%) - no deferrals detected"
Display: "Skipping Phase 06 (Deferral Challenge Checkpoint)"
Display: "Proceeding to Phase 07 (DoD Update)..."

Exit this checkpoint
Return control to skill for Phase 07
```

**OTHERWISE:** Continue to Step 3

**Note:** With RCA-014 fix, "no incomplete items" means DoD is 100% complete (all boxes checked). Any unchecked box triggers this checkpoint, whether it has explicit justification ("Deferred to...") or not (implicit deferral).

---

### Step 3: Invoke deferral-validator Subagent [MANDATORY IF deferrals exist]

<deferral_validation>
  <purpose>Validate that blockers are still accurate and detect resolvable deferrals</purpose>
  <subagent>deferral-validator</subagent>
  <input>All deferred items with justifications</input>
  <output>Categorization: can_resolve_now vs must_stay_deferred</output>
</deferral_validation>

**Display checkpoint notice:**

```
Display:
"━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Phase 06: DEFERRAL CHALLENGE CHECKPOINT
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Found {len(deferred_items)} deferred Definition of Done items:

{FOR each item in deferred_items}:
  {index}. {item.text}
     Justification: {item.justification}

Validating blockers with deferral-validator subagent..."
```

**Invoke deferral-validator subagent:**

```
Task(
  subagent_type="deferral-validator",
  description="Validate deferred items for ${STORY_ID}",
  prompt="""
Validate the following deferred items:

Story: ${STORY_ID}
Deferred Items:
{FOR each item in deferred_items}:
  - {item.text}
    {item.justification}

For each deferral:
1. If deferred to story:
   - Check: git log --grep="{target_story}" --oneline
   - Check: devforgeai/specs/Stories/{target_story}*.story.md status
   - Determine if dependency story is complete

2. If blocked by toolchain:
   - Check: rustup toolchain list (Rust)
   - Check: npm --version (Node.js)
   - Check: dotnet --list-sdks (.NET)
   - Determine if toolchain is available

3. If blocked by artifacts:
   - Check: ls -la {expected_artifact_paths}
   - Determine if artifacts exist

4. If out of scope (ADR):
   - Check: devforgeai/specs/adrs/{adr_reference}.md exists
   - Determine if ADR is documented

Return JSON:
{
  "can_resolve_now": [
    {
      "item": "...",
      "reason": "Blocker resolved: ...",
      "command": "... (command to attempt resolution)"
    }
  ],
  "must_stay_deferred": [
    {
      "item": "...",
      "reason": "Blocker still valid: ...",
      "blocker": "..."
    }
  ],
  "violations": [
    {
      "item": "...",
      "severity": "CRITICAL | HIGH | MEDIUM",
      "issue": "Circular deferral | Missing target | Invalid justification"
    }
  ]
}
"""
)
```

**Wait for subagent result...**

---

### Step 4: Handle Validation Results [MANDATORY IF deferrals exist]

**Extract subagent result:**

```
validation_result = subagent response (parsed JSON)

can_resolve_count = len(validation_result.can_resolve_now)
must_defer_count = len(validation_result.must_stay_deferred)
violation_count = len(validation_result.violations)
```

**Display validation summary:**

```
Display:
"
DEFERRAL VALIDATION RESULTS:

✅ Resolvable Deferrals: {can_resolve_count}
{FOR each item in can_resolve_now}:
   - {item.item}
     Reason: {item.reason}
     Action: {item.command}

⏸️  Valid Deferrals: {must_defer_count}
{FOR each item in must_stay_deferred}:
   - {item.item}
     Blocker: {item.blocker}
     Reason: {item.reason}

{IF violation_count > 0}:
❌ VIOLATIONS DETECTED: {violation_count}
{FOR each violation in violations}:
   - {violation.item}
     Severity: {violation.severity}
     Issue: {violation.issue}
"
```

---

### Step 5: Handle Violations [MANDATORY IF violations detected]

**IF violations with severity CRITICAL:**

```
Display:
"❌ CRITICAL DEFERRAL VIOLATIONS DETECTED

The following violations MUST be resolved before proceeding:

{FOR each violation in violations WHERE severity == "CRITICAL"}:
  - {violation.item}
    Issue: {violation.issue}

Common critical violations:
  • Circular deferral chains (STORY-A → STORY-B → STORY-A)
  • Missing target stories/ADRs that don't exist
  • Invalid justification format

Actions Required:
1. Fix circular deferral chains (redesign dependencies)
2. Create missing target stories/ADRs
3. Update deferral justifications to valid format

WORKFLOW HALTED - Cannot proceed to git commit until resolved.

Fix issues and re-run: /dev ${STORY_ID}"

HALT execution
Exit Phase 06
Return error to skill
```

**IF violations with severity HIGH (non-critical):**

```
AskUserQuestion:
  Question: "HIGH-severity deferral violations detected. How should we proceed?"
  Header: "Deferral Violations"
  Options:
    - "Fix violations now (I'll update deferrals)"
    - "Override and proceed (I'll provide justification)"
  multiSelect: false

IF user selects "Fix violations now":
  Display: "Please fix violations manually:

  {FOR each violation in violations WHERE severity == "HIGH"}:
    - {violation.item}: {violation.issue}

  After fixing, re-run: /dev ${STORY_ID}"

  HALT execution
  Exit Phase 06

ELSE IF user selects "Override and proceed":
  AskUserQuestion:
    Question: "Provide justification for overriding HIGH-severity violations:"
    Header: "Override Justification"
    # User provides free-form text

  override_justification = user_input

  Log to story file:
  "⚠️ HIGH-severity deferral violations overridden by user
  Justification: {override_justification}
  Timestamp: {current_timestamp}"

  Proceed to Step 6
```

---

### Step 6: Challenge ALL Incomplete Items with User Approval [MANDATORY IF incomplete items exist]

**UPDATED (RCA-014):** Challenge both explicit AND implicit deferrals.

<user_approval_gate>
  <policy>ZERO autonomous deferrals (explicit OR implicit)</policy>
  <mechanism>AskUserQuestion for EVERY incomplete DoD item</mechanism>
  <scope>ALL incomplete work (resolvable + valid + implicit deferrals)</scope>
  <enforcement>Git commit BLOCKED until all approved OR implemented</enforcement>
  <rca_context>RCA-014 discovered implicit deferrals (unchecked without justification) bypassed Phase 06</rca_context>
</user_approval_gate>

**FOR EACH incomplete item (explicit deferrals + implicit deferrals):**

**Note:** Explicit deferrals have justifications ("Deferred to...", "Blocked by..."). Implicit deferrals are plain unchecked boxes (no justification). Both require user approval per CLAUDE.md: "Deferrals are not acceptable!"

```
Display:
"━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 DEFERRAL #{index}/{total_deferrals}
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Item: {item.text}
Current Justification: {item.justification}

{IF item in can_resolve_now}:
⚠️  BLOCKER RESOLVED: {item.reason}
Recommended Action: {item.command}

{ELSE IF item in must_stay_deferred}:
✓ BLOCKER VALID: {item.reason}
Blocker: {item.blocker}
"

AskUserQuestion:
  Question: "How should we handle: '{item.text}'?"
  Header: "Deferral Decision"
  Options:
    - "HALT and implement NOW"           ← FIRST (strongest challenge) [NEW ORDER]
      Description: "Stop workflow and implement this item immediately"
    - "Keep deferred (blocker is valid)"
      Description: "I confirm this cannot be done now. Add my approval timestamp."
    - "Update justification (blocker changed)"
      Description: "Blocker changed, update the reason"
    - "Remove from DoD (not needed)"
      Description: "This item is no longer needed"
  multiSelect: false

user_decision = response

BASED ON user_decision:

## Option 1: "Attempt now"
```
Display: "Returning to Phase 03 (TDD Green) to implement: {item.text}"

# Remove deferral justification from story file
Edit(
  file_path="${STORY_FILE}",
  old_string="- [ ] {item.text}
    {item.justification}",
  new_string="- [ ] {item.text}"
)

# Mark item for implementation
items_to_implement.append(item.text)

Display: "Deferral removed. Item will be implemented."

# After ALL items processed, return to Phase 03
IF index == total_deferrals:
  IF items_to_implement is not empty:
    Display:
    "Returning to Phase 03 (TDD Green) to implement {len(items_to_implement)} items:
    {list items_to_implement}

    After implementation, Phase 06 will run again to validate remaining deferrals."

    HALT Phase 06
    Return to skill with instruction: "Resume Phase 03 for items: {items_to_implement}"
```

## Option 2: "Keep deferred"
```
# Add user approval timestamp to justification
current_timestamp = $(date -u +"%Y-%m-%d %H:%M:%S UTC")

Edit(
  file_path="${STORY_FILE}",
  old_string="{item.justification}",
  new_string="{item.justification}
    User approved: {current_timestamp}"
)

Display: "✓ Deferral approved and timestamped: {item.text}"

# Log approval
approved_deferrals.append({
  item: item.text,
  justification: item.justification,
  timestamp: current_timestamp
})
```

## Option 3: "Update justification"
```
AskUserQuestion:
  Question: "Provide updated justification for deferring '{item.text}'"
  Header: "New Justification"
  # User provides free-form text

new_justification = user_input

# Determine deferral type from new justification
IF "Deferred to STORY-" in new_justification:
  target_story = extract STORY-ID from new_justification

  # Validate target story exists
  Glob(pattern="devforgeai/specs/Stories/{target_story}*.story.md")

  IF not found:
    Display: "⚠️ WARNING: Story {target_story} doesn't exist yet. You must create it."

ELSE IF "Blocked by:" in new_justification:
  blocker = extract blocker description

  # Log to technical debt register
  Check if devforgeai/technical-debt-register.md exists
  IF not exists:
    Create from template

  Append to register:
  "- {item.text} (from ${STORY_ID}): {blocker} | Date: {current_date} | Status: Open"

ELSE IF "Out of scope: ADR-" in new_justification:
  adr_reference = extract ADR-XXX

  # Validate ADR exists
  Glob(pattern="devforgeai/specs/adrs/{adr_reference}*.md")

  IF not found:
    Display: "⚠️ WARNING: ADR {adr_reference} doesn't exist. You must create it to document scope change."

# Update story file with new justification
current_timestamp = $(date -u +"%Y-%m-%d %H:%M:%S UTC")

Edit(
  file_path="${STORY_FILE}",
  old_string="- [ ] {item.text}
    {item.justification}",
  new_string="- [ ] {item.text}
    {new_justification}
    User approved: {current_timestamp}"
)

Display: "✓ Justification updated and approved: {item.text}"
```

## Option 4: "Remove from DoD"
```
AskUserQuestion:
  Question: "Why is '{item.text}' no longer needed? (This will be logged as scope change)"
  Header: "Removal Reason"
  # User provides free-form text

removal_reason = user_input

# Remove item from DoD entirely
Edit(
  file_path="${STORY_FILE}",
  old_string="- [ ] {item.text}
    {item.justification}",
  new_string=""
)

# Log scope change
Read(file_path="${STORY_FILE}")

Look for "## Workflow History" section
Append before Workflow History:
"### Scope Changes

- **Removed:** {item.text}
  - **Reason:** {removal_reason}
  - **Date:** {current_timestamp}
  - **Approved by:** User

"

Display: "✓ Item removed from DoD: {item.text}"
Display: "ℹ️  Consider creating ADR to document scope change if significant."

removed_items.append({
  item: item.text,
  reason: removal_reason
})
```

# END FOR EACH loop
```

---

### Step 6.5: Mandatory HALT Verification [NEW - CANNOT BE SKIPPED]

**Purpose:** Ensure Step 6 (User Approval) was NOT bypassed autonomously.

**CRITICAL:** This step exists because Claude has been observed autonomously approving deferrals without user consent (RCA-006). This is a defense-in-depth checkpoint against autonomous deferrals.

**Why This Step Is Necessary:**

User reported: *"deferral-validator does not halt and automatically approves deferrals"*

This checkpoint verifies that EVERY deferral went through the Step 6 AskUserQuestion workflow and received explicit user approval.

#### 6.5.1 Verify User Approval Occurred for ALL Deferrals

```
unapproved_deferrals = []

FOR each deferral in all_deferrals:
    IF deferral.status == "kept" OR deferral.status == "deferred":
        # Check if user approval timestamp exists
        IF deferral.user_approval_timestamp IS EMPTY:
            unapproved_deferrals.append(deferral)

IF unapproved_deferrals is NOT empty:

    HALT IMMEDIATELY with message:
    ```
    ════════════════════════════════════════════════════════════════
    🚨 AUTONOMOUS DEFERRAL DETECTED - WORKFLOW HALTED
    ════════════════════════════════════════════════════════════════

    {len(unapproved_deferrals)} deferral(s) were approved WITHOUT user consent.

    This is a CRITICAL violation of DevForgeAI protocol (RCA-006).

    The following deferral(s) lack user approval timestamp:
    ```

    FOR each deferral in unapproved_deferrals:
        Display: "  - {deferral.text}"
        Display: "    Justification: {deferral.justification}"
        Display: "    Missing: User approved: YYYY-MM-DD HH:MM:SS UTC"
        Display: ""

    Display: "ACTION REQUIRED:"
    Display: "User MUST explicitly approve or reject each deferral."
    Display: "═══════════════════════════════════════════════════════════════"
    Display: ""

    # Force user decision NOW for each unapproved deferral
    FOR each deferral in unapproved_deferrals:
        Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        Display: "MANDATORY APPROVAL REQUIRED"
        Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        Display: "Item: {deferral.text}"
        Display: "Justification: {deferral.justification}"
        Display: ""

        AskUserQuestion:
            Question: "Deferral '{deferral.text}' was auto-approved. What should happen?"
            Header: "MANDATORY Approval"
            Options:
                - "HALT and implement NOW (reject deferral)"  ← FIRST OPTION
                  Description: "Stop workflow, implement this item immediately"
                - "Approve deferral (I explicitly consent)"
                  Description: "I confirm blocker is valid. Add my approval timestamp."
                - "Remove from DoD (not needed)"
                  Description: "This item should not have been in DoD"
            multiSelect: false

        user_choice = response

        # Record approval with timestamp
        IF user_choice == "Approve deferral (I explicitly consent)":
            timestamp = $(date -u +"%Y-%m-%d %H:%M:%S UTC")

            Edit(
              file_path="${STORY_FILE}",
              old_string="{deferral.justification}",
              new_string="{deferral.justification}
    User approved: {timestamp}"
            )

            Display: "✓ Deferral approved by user at {timestamp}"

        ELIF user_choice == "HALT and implement NOW (reject deferral)":
            Display: "User chose to implement immediately."
            Display: "Returning to Phase 03 to implement: {deferral.text}"

            # Remove deferral, add to implementation queue
            items_to_implement.append(deferral.text)

            HALT: "User rejected deferral. Return to Phase 03 for implementation."

        ELIF user_choice == "Remove from DoD (not needed)":
            Edit(
              file_path="${STORY_FILE}",
              old_string="- [ ] {deferral.text}
    {deferral.justification}",
              new_string=""
            )

            Display: "✓ Item removed from DoD: {deferral.text}"
```

#### 6.5.2 Audit Trail Requirement

Every kept deferral MUST have this format:

```markdown
- [ ] {deferral_text}
  Blocker: {blocker_type}
  Justification: {detailed_reason}
  User approved: {YYYY-MM-DD HH:MM:SS UTC}  ← MANDATORY
```

**Deferrals WITHOUT "User approved:" timestamp are INVALID and will fail Phase 06 checkpoint.**

#### 6.5.3 Final Checkpoint Verification

```
unapproved_count = count_deferrals_without_timestamp()

IF unapproved_count > 0:
    HALT: "Cannot proceed to Bridge. {unapproved_count} deferral(s) lack user approval timestamp."
    Display: "Re-run Step 6.5 to force user approval for all deferrals."

ELSE:
    Display: ""
    Display: "✓ Step 6.5 Complete: All deferrals have user approval timestamps"
    Display: "  - {len(approved_deferrals)} deferrals approved"
    Display: "  - {len(items_to_implement)} deferrals rejected (will implement)"
    Display: "  - {len(removed_items)} items removed from DoD"
    Display: ""

    # Determine next action
    IF items_to_implement is NOT empty:
        Display: "Returning to Phase 03 to implement rejected deferrals..."
        HALT Phase 06
        GOTO Phase 03 (Step 7 handles this)
    ELSE:
        Display: "No implementation needed. Proceeding to Step 7..."
        PROCEED to Step 7
```

---

### Step 6.6: Technical Debt Register Update [MANDATORY - UNCONDITIONAL]

For full details, see: [technical-debt-register-workflow.md](technical-debt-register-workflow.md) (Step 6.6: Technical Debt Register Update)

**Summary:** When user approves a deferral in Step 6, the technical debt register is automatically updated with:
- Sequential DEBT-NNN ID generation
- Entry with all 8 required fields
- Analytics counter updates
- Confirmation display

**Load this reference during Step 6.6 execution:**
```
Read(file_path=".claude/skills/devforgeai-development/references/technical-debt-register-workflow.md")
```

---

### Step 7: Immediate Resumption Decision [MANDATORY - RCA-014 Fix]

**CRITICAL CHANGE (RCA-014/REC-2):** Resumption decision happens IMMEDIATELY in Phase 06 (not in separate Phase 06-R).

**Purpose:** If user chose "Attempt now" for ANY items, determine resumption phase and loop back IMMEDIATELY (before DoD update, before Phase 08).

**Rationale:** RCA-014 identified that Phase 06-R had circular dependency (expected DoD already updated). Moving resumption logic here eliminates dependency and enables immediate loop back.

<resumption_logic>
  <trigger>User selected "Attempt now" for ≥1 items</trigger>
  <decision>Determine which TDD phase to resume from (2, 3, or 4)</decision>
  <action>GOTO resumption phase (immediate loop back)</action>
  <prevents>Committing incomplete work when user said "Continue to 100%"</prevents>
</resumption_logic>

```
# Count items by decision type
attempt_now_count = count(items where user selected "Attempt now")
approved_count = count(items where user selected "Keep deferred")
removed_count = count(items where user selected "Remove from DoD")

# Determine next action based on user decisions
IF attempt_now_count > 0:
  Display: ""
  Display: "════════════════════════════════════════════════════════════"
  Display: "⚠️  RESUMPTION TRIGGERED (User Chose 'Attempt Now')"
  Display: "════════════════════════════════════════════════════════════"
  Display: ""
  Display: "User Decision: Implement {attempt_now_count} deferred items now"
  Display: "Items to implement:"
  FOR each item in items_to_implement:
    Display: "  • {item}"
  Display: ""
  Display: "Determining which TDD phase to resume from..."
  Display: ""

  # Analyze incomplete items to determine resumption phase
  needs_implementation = false
  needs_refactoring = false
  needs_integration = false

  FOR each item in items_to_implement:
    item_lower = item.text.lowercase()

    # Check if item requires new code implementation
    IF item_lower contains "feature" OR "implemented" OR "code written" OR "functionality":
      needs_implementation = true

    # Check if item requires quality improvements
    IF item_lower contains "refactor" OR "code quality" OR "complexity" OR "review":
      needs_refactoring = true

    # Check if item requires integration/end-to-end testing
    IF item_lower contains "integration test" OR "end-to-end" OR "e2e" OR "cross-component":
      needs_integration = true

  # Determine earliest phase needed (implementation before refactoring before integration)
  IF needs_implementation:
    resumption_phase = 2
    resumption_name = "Phase 03 (Implementation - Green Phase)"
    Display: "Resuming at: {resumption_name}"
    Display: "Reason: {attempt_now_count} items require code implementation"
    Display: ""

    # Increment iteration counter
    iteration_count = iteration_count + 1

    Display: "TDD Iteration: {iteration_count}/5"
    Display: ""

    # Check iteration limit
    IF iteration_count >= 5:
      Display: "⚠️  WARNING: Iteration limit reached (5 TDD cycles)"
      Display: ""

      AskUserQuestion:
        Question: "Story has required 5 TDD iterations. How should we proceed?"
        Header: "Iteration Limit"
        Options:
          - "Continue anyway (allow iteration 6+)"
          - "Commit current progress with documented deferrals"
          - "Review what's blocking completion (investigate)"
        multiSelect: false

      IF user selects "Continue anyway":
        Display: "✓ User approved continuation beyond iteration 5"
        Display: "  Proceeding to {resumption_name}..."
        # Continue to GOTO Phase 03 below

      ELSE IF user selects "Commit current progress":
        Display: "✓ User approved committing with deferrals"
        Display: "  Proceeding to Phase 07 (DoD Update)..."
        GOTO Phase 07  # Will add "Approved Deferrals" section

      ELSE IF user selects "Review what's blocking":
        Display: "Investigation recommended. Run: /rca \"Story requiring >5 TDD iterations\" HIGH"
        HALT workflow
        EXIT

    GOTO Phase 03

  ELSE IF needs_refactoring:
    resumption_phase = 3
    resumption_name = "Phase 04 (Refactoring)"
    Display: "Resuming at: {resumption_name}"
    Display: "Reason: {attempt_now_count} items require code quality improvements"
    Display: ""

    iteration_count = iteration_count + 1
    Display: "TDD Iteration: {iteration_count}/5"
    Display: ""

    GOTO Phase 04

  ELSE IF needs_integration:
    resumption_phase = 4
    resumption_name = "Phase 05 (Integration Testing)"
    Display: "Resuming at: {resumption_name}"
    Display: "Reason: {attempt_now_count} items require integration tests"
    Display: ""

    iteration_count = iteration_count + 1
    Display: "TDD Iteration: {iteration_count}/5"
    Display: ""

    GOTO Phase 05

  ELSE:
    # Unclear which phase - ask user
    Display: "Unable to determine resumption phase from item descriptions."
    Display: ""

    AskUserQuestion:
      Question: "Which TDD phase should we resume from to implement these {attempt_now_count} items?"
      Header: "Resumption Phase"
      Options:
        - "Phase 03 (Implementation - write new code)"
        - "Phase 04 (Refactoring - improve existing code)"
        - "Phase 05 (Integration - add cross-component tests)"
      multiSelect: false

    user_phase_choice = response

    IF user selects "Phase 03":
      GOTO Phase 03
    ELSE IF user selects "Phase 04":
      GOTO Phase 04
    ELSE IF user selects "Phase 05":
      GOTO Phase 05

ELSE IF approved_count > 0:
  # All incomplete items were approved for deferral
  Display: ""
  Display: "✓ All {approved_count} incomplete items approved for deferral"
  Display: "  Proceeding to Phase 07 (DoD Update)..."
  Display: ""

  GOTO Phase 07  # Will mark items [x] and add "Approved Deferrals" section

ELSE:
  # All items were removed from DoD
  Display: ""
  Display: "✓ All {removed_count} items removed from DoD (scope changes documented)"
  Display: "  Proceeding to Phase 07 (DoD Update)..."
  Display: ""

  GOTO Phase 07
```

**Note:** This replaces Phase 06-R entirely. Resumption decision happens IMMEDIATELY after user approval, not in separate phase. This eliminates the circular dependency (Phase 06-R expected DoD updated, but ran before DoD update).

---

### Step 8: Final Summary Display [MANDATORY]

**Display comprehensive summary of Phase 06 results:**

```
Display:
"━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 PHASE 4.5 COMPLETE: Deferral Challenge Checkpoint
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Incomplete Items Challenged: {total_incomplete_items}
- Explicit deferrals: {count with classification='explicit_deferral'}
- Implicit deferrals: {count with classification='implicit_deferral'} (RCA-014 fix)

User Decisions:
- Attempt now (implementing): {attempt_now_count}
- Approved (staying deferred): {approved_count}
- Removed from DoD: {removed_count}

{IF attempt_now_count > 0}:
⟳ RESUMING AT: {resumption_name}
  Items to implement: {list items_to_implement}
  After completion, workflow will re-run Phase 06 for final validation.

{IF approved_count > 0}:
✓ Approved Deferrals (with timestamps):
{FOR each deferral in approved_deferrals}:
  - {deferral.item}
    Justification: {deferral.justification}
    Approved: {deferral.timestamp}

{IF removed_count > 0}:
🗑️  Removed from DoD:
{FOR each item in removed_items}:
  - {item.item}
    Reason: {item.reason}
    (Logged in Workflow History)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
```

**This summary is displayed regardless of next action (resumption, approval, or removal).**

---

### Step 9: Update AC Verification Checklist (Phase 06 Items) [NEW - RCA-011]

**Purpose:** Check off AC items related to deferral validation (real-time progress tracking)

**Execution:** After Step 8 (Final Summary Display), before Phase 06 checkpoint

**Load AC Checklist Update Workflow:**
```
Read(file_path=".claude/skills/devforgeai-development/references/ac-checklist-update-workflow.md")
```

**Identify Phase 06 AC Items:**
```
Grep(pattern="Phase.*: 4.5", path="${STORY_FILE}", output_mode="content", -B=1)
```

**Common Phase 06 items:**
- [ ] All deferrals user-approved
- [ ] No circular deferral chains
- [ ] Follow-up stories created (if applicable)
- [ ] Deferral timestamps recorded
- [ ] Technical debt register updated

**Update Procedure:** Batch-update all Phase 06 items that are complete

**Display:** "Phase 06 AC Checklist: ✓ {count} items checked | AC Progress: {X}/{Y}"

**Graceful Skip:**
```
IF AC Verification Checklist section not found in story:
  Display: "ℹ️ Story uses DoD-only tracking (AC Checklist not present)"
  Skip AC checklist updates
  Continue to Phase 06 Checkpoint
```

**Performance:** ~15-30 seconds for 2-4 items

---

## Success Criteria (Updated RCA-014)

This checkpoint succeeds when:
- [ ] All incomplete DoD items detected via Grep (explicit + implicit deferrals)
- [ ] Section filtering applied (only DoD, not AC Checklist)
- [ ] deferral-validator subagent invoked for explicit deferrals (if any)
- [ ] CRITICAL violations halted workflow (if any)
- [ ] HIGH violations handled (fixed or overridden)
- [ ] User interaction completed for EVERY incomplete item (via AskUserQuestion)
- [ ] Story file updated with user approvals (timestamps) for approved deferrals
- [ ] Items to implement flagged and resumption phase determined
- [ ] Removed items logged in Workflow History
- [ ] Immediate resumption decision made (no delay until separate phase)

**On success (all approved for deferral):** Proceed to Phase 07 (DoD Update)

**On success (user chose "Attempt now"):** IMMEDIATE loop back to Phase 03/3/4 (resumption)

**On success (items removed):** Proceed to Phase 07 (DoD Update)

**On failure (CRITICAL violations):** HALT workflow, user must fix and re-run /dev

**Key Change (RCA-014):** Resumption happens in Phase 06 Step 7 (not separate Phase 06-R), eliminating circular dependency

---

## Integration Notes

**Invoked by:** devforgeai-development skill (after Phase 05, before Phase 08)

**Invokes:**
- deferral-validator subagent (blocker validation)
- requirements-analyst subagent (if creating follow-up stories)
- architect-reviewer subagent (if creating ADRs for scope changes)

**Updates:**
- Story file (deferral approvals, justifications, scope changes)
- Technical debt register (external blockers)

**References:**
- `.claude/agents/deferral-validator.md` (subagent for blocker validation)
- `.claude/skills/devforgeai-development/references/dod-validation-checkpoint.md` (Phase 08 checkpoint, different purpose)
- RCA-006 findings and recommendations

**Token Efficiency:**
- Uses native Grep tool (60% savings vs Bash grep)
- Uses Edit tool for updates (75% savings vs sed)
- Story file already loaded in context
- Estimated: 8,000-12,000 tokens per execution

**Character Count:** ~11,500 characters (well under 15K budget per task file)

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

### deferral-validator Subagent Fails
```
ERROR: deferral-validator subagent failed to complete

Possible causes:
- Git unavailable (cannot check dependency stories)
- Toolchain detection commands failed
- Artifact paths inaccessible

Actions:
1. Review subagent error output above
2. Fix environment issues (install git, toolchains)
3. Re-run /dev ${STORY_ID}

Fallback: If subagent fails repeatedly, proceed with manual validation:
- User reviews each deferral manually
- User confirms blockers without automated checks
```

### Story File Update Fails
```
ERROR: Cannot update story file with approval timestamps

Possible causes:
- File not writable (check permissions)
- old_string not unique (Edit tool requires unique match)
- File modified during checkpoint execution

Actions:
1. Read story file again: Read(file_path="${STORY_FILE}")
2. Verify item text matches exactly
3. Use Edit with unique context if needed
4. Re-run checkpoint
```

### User Cancels Approval Process
```
WARNING: User cancelled deferral approval process

This checkpoint requires user approval for ALL deferred items.
Cannot proceed to git commit without approvals.

Actions:
1. Re-run /dev ${STORY_ID} when ready to approve deferrals
2. Or complete deferred items first, then re-run
```

---

## Protocol Compliance

**This checkpoint enforces:**
- DevForgeAI zero technical debt policy
- RCA-006 findings (prevent autonomous deferrals)
- "Ask, Don't Assume" framework principle
- "Attempt First, Defer Only If Blocked" pattern

**User approval required for:**
- Every pre-existing deferral (from story template)
- Every new deferral (created during TDD cycle)
- Every blocker justification update
- Every scope change (DoD item removal)

**No exceptions. No autonomous decisions. Zero tolerance for unapproved deferrals.**

---

## Difference from Phase 08 DoD Validation

**Phase 06 (This Checkpoint):**
- Purpose: Challenge ALL deferrals (pre-existing + new)
- Scope: Only deferred items
- Timing: After TDD cycle, before git commit
- Validation: deferral-validator checks blocker validity
- Action: User approves/updates/removes each deferral

**Phase 08 DoD Validation (Existing):**
- Purpose: Handle incomplete items WITHOUT justifications
- Scope: Items developer couldn't complete during TDD
- Timing: During git commit preparation
- Validation: User provides justifications for new incomplete items
- Action: User defers/completes/scopes out new incomplete items

**Both checkpoints work together:**
1. Phase 06 challenges existing deferrals
2. Phase 08 handles new incomplete items (if any remain after Phase 06)

**No duplication:** Phase 06 only processes items WITH justifications. Phase 08 only processes items WITHOUT justifications.

---

## ✅ PHASE 4.5 COMPLETION CHECKPOINT

**Before proceeding to Phase 07 (DoD Update Workflow), verify ALL steps executed:**

### Mandatory Steps Executed

- [ ] **Step 1:** Detect All Deferred Items [MANDATORY]
  - Verification: Grep scan complete for deferred DoD items
  - Output: All deferred items identified and listed

- [ ] **Step 2:** Skip If No Deferrals OR Continue
  - Verification: IF no deferrals → Phase 06 skipped (proceed to bridge)
  - Verification: IF deferrals exist → Steps 3-7 executed

**If deferrals exist, also verify:**

- [ ] **Step 3:** Invoke deferral-validator Subagent [MANDATORY]
  - Verification: deferral-validator invoked for blocker validation
  - Output: Validation report with resolvable vs. valid categories

- [ ] **Step 4:** Parse Validation Results [MANDATORY]
  - Verification: Report parsed, deferrals categorized
  - Output: Resolvable and valid deferrals identified

- [ ] **Step 5:** Present Deferrals to User [MANDATORY]
  - Verification: All deferrals displayed with validation results
  - Output: User presented with approval decision for EACH deferral

- [ ] **Step 6:** Capture User Decisions [MANDATORY]
  - Verification: User decisions captured for ALL deferrals
  - Output: Approved deferrals, items to implement, items to remove
  - HALT if user cancels (cannot proceed without approvals)

- [ ] **Step 7:** Update Story File with Approvals [MANDATORY]
  - Verification: Approval markers added to story file
  - Output: Story file updated with timestamps and justifications

### Success Criteria

- [ ] All deferrals have user approval OR no deferrals exist
- [ ] Zero autonomous deferrals (every deferral user-approved)
- [ ] Approval timestamps recorded in story file
- [ ] deferral-validator validation passed
- [ ] Items to implement identified (if any)

**IF ANY ITEM UNCHECKED:**
```
❌ PHASE 4.5 INCOMPLETE - Review missing steps above
⚠️  DO NOT PROCEED TO PHASE 4.5-5 BRIDGE until all checkpoints pass

Most commonly missed:
  - Step 6 (User approval for EVERY deferral) ← Required, zero exceptions
  - Approval timestamps ← Must update story file with timestamps
  - Items to implement ← May need to return to Phase 03

Proceeding without complete Phase 06 results in:
  - Unapproved deferrals (violates RCA-006 zero autonomous policy)
  - Missing approval audit trail (cannot verify user consent)
  - Git commit may fail (DoD validation requires approved deferrals)
```

- [ ] **Step 9:** AC Verification Checklist updated (Phase 06 items) [NEW - RCA-011]
  - Verification: All Phase 06 AC items checked off (deferral validation items)
  - Output: "AC Progress: X/Y items complete" displayed
  - Graceful: Skipped if story doesn't have AC Checklist section

**IF ALL ITEMS CHECKED:**
```
✅ PHASE 4.5 COMPLETE - Deferral Challenge Checkpoint Done

Deferrals processed: {deferral_count}
User approvals: {approved_count}
Items to implement: {implement_count}
Items removed: {removed_count}

All deferrals have user approval. Zero autonomous deferrals.
AC Checklist updated with deferral validation results.
Ready to update DoD format for git commit.

**Update Progress Tracker:**
Mark "Execute Phase 06" todo as "completed"

**See Also (CRITICAL - Phase 06 → 5 Handoff):**
- `dod-update-workflow.md` - DoD formatting requirements (MUST execute before Phase 08)
- `git-workflow-conventions.md` - Git commit workflow and conventions
- `dod-validation-checkpoint.md` - Handle new incomplete items (Phase 08 Step 1.7)
- `deferral-budget-enforcement.md` - Deferral budget limits (Phase 08 Step 1.6)
- `deferral-validator` subagent - Blocker validation specialist
```

**Next: Load dod-update-workflow.md and execute Phase 07**

---

## Phase 06 Complete: Handoff to DoD Update Bridge

**After all deferral validation completes, proceed to DoD format update workflow.**

### Next Phase: DoD Update Workflow (Phase 07)

**Purpose:** Update Definition of Done items in correct format for git commit validation

**CRITICAL:** Phase 08 git commit will FAIL if DoD items not formatted correctly. This bridge ensures format compliance.

**Load and execute:**

```
Read(file_path=".claude/skills/devforgeai-development/references/dod-update-workflow.md")
```

**What the bridge does:**
1. Marks completed DoD items [x] in Definition of Done section
2. Adds items to Implementation Notes (FLAT LIST - no ### subsections)
3. Validates format: `devforgeai-validate validate-dod ${STORY_FILE}` (must pass)
4. Updates Workflow Status checkboxes

**Pre-Phase-5 Checklist:**
- [ ] All deferred items have user approval (Phase 06 validation)
- [ ] DoD items marked [x] in Definition of Done section
- [ ] DoD items added to Implementation Notes (flat list, no ### subsection)
- [ ] Workflow Status updated (Development [x], QA/Release [ ])
- [ ] devforgeai-validate validate-dod passes (exit code 0)
- [ ] Ready for git commit (no format blockers)

**If ANY checkbox unchecked:** DO NOT proceed to Phase 08 - complete bridge workflow first

**See:** `dod-update-workflow.md` for detailed DoD formatting requirements and common errors
