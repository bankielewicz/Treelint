---
name: remediation-story-creator
description: Creates follow-up stories from technical debt entries
---

# Remediation Story Creator

**Skill Reference:** devforgeai-development
**Phase:** Phase 06.5 (After Deferral Challenge, Before DoD Update)
**Loaded:** After debt items added to technical-debt-register.md
**Token Cost:** ~6,000-8,000 tokens

**Purpose:** Offer to create follow-up remediation stories from newly added debt items with automatic pre-fill from debt data.

**Trigger:** IMMEDIATELY after debt confirmation display in:
- /dev Phase 06 (STORY-286 integration)
- /qa hook completion (STORY-287 integration)

---

## Workflow Overview

```
Debt Item Added → Confirmation Displayed → Remediation Prompt (this workflow) → User Decision → Story Created OR Continue
```

---

## Step 1: Display Remediation Prompt [MANDATORY]

**IMMEDIATELY after debt confirmation, display AskUserQuestion:**

```
AskUserQuestion(questions=[{
  question: "Create a follow-up story for DEBT-NNN?",
  header: "Remediation Story",
  options: [
    {label: "Yes, create remediation story now", description: "Create story with pre-filled data from debt entry"},
    {label: "No, I'll create it later", description: "Skip for now - you can create via /create-story later"}
  ],
  multiSelect: false
}])
```

**Batch Mode (Multiple Debt Items):**

When N > 1 debt items added (common in /qa hook scenario):

```
AskUserQuestion(questions=[{
  question: "Create remediation stories for these N debt items?",
  header: "Remediation Story",
  options: [
    {label: "Yes, create N stories now", description: "Create stories with pre-filled data from each debt entry"},
    {label: "No, I'll create them later", description: "Skip for now - you can create via /create-story later"}
  ],
  multiSelect: false
}])
```

---

## Step 2: Handle User Decision

### If User Selects "Yes, create remediation story now":

Continue to Step 3 (Data Extraction).

### If User Selects "No, I'll create it later":

```
Display: "Story creation skipped - you can create a remediation story later via /create-story or manually update the Follow-up field in the register"

# NO side effects:
- Do NOT invoke devforgeai-story-creation skill
- Do NOT modify the technical debt register Follow-up field
- Continue to parent workflow:
  - If from /dev: Continue to Phase 07 (DoD Update)
  - If from /qa: Continue to Phase 3 completion
```

---

## Step 3: Extract Data from Debt Entry [MANDATORY]

**Read debt entry from register:**

```
Grep(pattern="^\\| DEBT-NNN \\|", path="devforgeai/technical-debt-register.md")
```

**Extract 8 fields from DEBT-NNN entry:**

Extract field ID (DEBT-NNN identifier for back-linking).
Extract field Date (entry creation date for context).
Extract field Source (dev_phase_06 or qa_discovery for classification).
Extract field Type (Story Split, Scope Change, External Blocker for title pattern).
Extract field Priority (Critical/High/Medium/Low for story priority).
Extract field Status (not used in story creation).
Extract field Effort (points estimate for story sizing).
Extract field Description (from deferred item text).

| Field | Source Column | Example | Maps To |
|-------|---------------|---------|---------|
| ID | Column 1 | DEBT-001 | Story back-link reference |
| Date | Column 2 | 2026-01-20 | Story context |
| Source | Column 3 | dev_phase_06 or qa_discovery | Story type classification |
| Type | Column 4 | Story Split, Scope Change, External Blocker | Story title pattern |
| Priority | Column 5 | Critical, High, Medium, Low | Story priority |
| Status | Column 6 | Pending, In Progress, Resolved | (not used) |
| Effort | Column 7 | 1 point, 2-3 points, 5+ points, TBD | Story points |
| Follow-up | Column 8 | STORY-XXX, ADR-XXX, or empty | Back-link target |

**Parse description from deferred item context:**

```
# Description comes from the original DoD item text that was deferred
# Available in deferral context when this workflow is triggered
DESCRIPTION = deferred_item_text
```

---

## Step 4: Build Story Context [MANDATORY]

### 4.1 Construct Feature Description

```
FEATURE_DESCRIPTION = "{TYPE}: {DESCRIPTION}"

# Examples:
# "Story Split: Complete integration testing for authentication module"
# "Scope Change: Add pagination support requested during review"
# "External Blocker: Resolve API dependency on external service"
```

### 4.2 Map Debt Type to Story Title Pattern

| Debt Type | Story Title Pattern |
|-----------|---------------------|
| Story Split | "Complete deferred work from STORY-XXX: {description}" |
| Scope Change | "Address scope change for STORY-XXX: {description}" |
| External Blocker | "Resolve external dependency for STORY-XXX: {description}" |
| (default) | "Remediate DEBT-NNN: {description}" |

### 4.3 Map Effort to Story Points

| Effort Field | Story Points |
|--------------|--------------|
| "1 point" | 1 |
| "TBD" | 2 (default) |
| "2-3 points" | 3 |
| "5+ points" | 5 |
| "Needs decomposition" | 8 (with warning) |

**If effort is "Needs decomposition":**
```
Display: "Warning: Consider splitting this story - 8 points may be too large"
```

### 4.4 Inherit Epic from Related Story

```
# Get Related-Story from debt entry (e.g., STORY-100)
RELATED_STORY = debt_entry.related_story

IF RELATED_STORY exists:
    # Read related story to get its epic
    Read(file_path="devforgeai/specs/Stories/${RELATED_STORY}*.story.md")
    EPIC_ID = related_story.frontmatter.epic

    IF EPIC_ID is null or empty:
        Display: "Remediation story created as standalone (no epic linkage available)"
        EPIC_ID = null
ELSE:
    EPIC_ID = null
    Display: "Remediation story created as standalone (no related story)"
```

### 4.5 Set Batch Mode Markers

```
BATCH_MODE = true  # Skip redundant interactive questions in story creation
```

---

## Step 5: Invoke Story Creation Skill [MANDATORY]

**Pre-filled context for devforgeai-story-creation:**

```
Skill(command="devforgeai-story-creation")

**Pre-filled Context:**
- Feature Description: {FEATURE_DESCRIPTION}
- Epic: {EPIC_ID or "standalone"}
- Sprint: Backlog
- Priority: {PRIORITY from debt entry}
- Points: {POINTS from effort mapping}
- Type: feature
- Batch Mode: true

**Skip Interactive Questions:**
- Feature description (provided from debt)
- Epic selection (inherited from related story)
- Priority (from debt entry)
- Points estimate (from effort mapping)
```

**Capture returned story ID:**

```
CREATED_STORY_ID = skill_result.story_id  # e.g., STORY-XXX
```

### Error Handling: Skill Failure

```
IF skill invocation fails:
    Display: "Story creation failed: {error message}"

    # Retry mechanism (max 2 attempts)
    retry_count = 0
    max_retries = 2

    WHILE retry_count < max_retries:
        AskUserQuestion(questions=[{
          question: "Retry story creation?",
          header: "Retry",
          options: [
            {label: "Yes, retry", description: "Attempt story creation again"},
            {label: "No, skip", description: "Skip story creation, continue workflow"}
          ],
          multiSelect: false
        }])

        IF user selects "Yes, retry":
            retry_count += 1
            # Retry skill invocation
            Skill(command="devforgeai-story-creation")

            IF success:
                BREAK  # Exit retry loop

        ELSE:
            # User chose to skip
            Display: "Story creation skipped after failure"
            # Do NOT update register Follow-up field
            Continue to parent workflow
            RETURN

    IF retry_count >= max_retries AND still failing:
        Display: "Story creation failed after {max_retries} attempts"
        # Do NOT update register Follow-up field
        Continue to parent workflow
        RETURN
```

---

## Step 6: Update Debt Register Back-Link [MANDATORY]

**After successful story creation:**

### 6.1 Update Follow-up Field

```
# Read current debt entry
Grep(pattern="^\\| DEBT-NNN \\|", path="devforgeai/technical-debt-register.md")

# Get current Follow-up value
current_followup = debt_entry.followup  # May be empty, or "ADR-001", etc.

IF current_followup is empty:
    new_followup = CREATED_STORY_ID
ELSE:
    # Append with comma separator
    new_followup = "{current_followup}, {CREATED_STORY_ID}"
    # Example: "ADR-001" becomes "ADR-001, STORY-150"

# Update the entry
Edit(
    file_path="devforgeai/technical-debt-register.md",
    old_string="| DEBT-NNN | ... | {current_followup} |",
    new_string="| DEBT-NNN | ... | {new_followup} |"
)
```

### 6.2 Update YAML Frontmatter last_updated

```
# Get current date in ISO format
current_date = "{ISO_8601_DATE}"  # Current date in YYYY-MM-DD format

Edit(
    file_path="devforgeai/technical-debt-register.md",
    old_string="last_updated: {old_date}",
    new_string="last_updated: {current_date}"
)
```

### 6.3 Display Confirmation

```
Display: "Created STORY-XXX for DEBT-NNN - Follow-up link updated in register"
```

---

## Step 7: Continue Parent Workflow

**After remediation story creation (or decline), continue:**

```
IF triggered from /dev Phase 06:
    Continue to Phase 07 (DoD Update)

IF triggered from /qa hook:
    Continue to Phase 3 completion (QA reporting)
```

---

## Batch Story Creation

**For multiple debt items (N > 1):**

```
# After user confirms "Yes, create N stories now"

FOR each debt_item in debt_items:
    # Extract data (Step 3)
    data = extract_debt_data(debt_item)

    # Build context (Step 4)
    context = build_story_context(data)

    # Invoke skill (Step 5)
    story_id = invoke_story_creation(context)

    # Update register (Step 6)
    update_backlink(debt_item, story_id)

    # Progress indicator
    Display: "Creating story {current}/{total}..."

# Summary after batch completes
Display: "Created {N} stories: STORY-XXX, STORY-YYY, STORY-ZZZ"
```

---

## Edge Cases

### Edge Case 1: Related Story Has No Epic

**Scenario:** Debt entry references STORY-100, but STORY-100 has `epic: null`

**Handling:**
```
Display: "Remediation story created as standalone (no epic linkage available)"
EPIC_ID = null  # Story created without epic association
```

### Edge Case 2: Follow-up Field Already Populated

**Scenario:** DEBT-001 Follow-up field already contains "STORY-050"

**Handling:**
```
Display: "DEBT-001 already has follow-up: STORY-050. Create another remediation story?"

AskUserQuestion(questions=[{
  question: "DEBT-001 already has follow-up: STORY-050. Create another remediation story?",
  header: "Existing Follow-up",
  options: [
    {label: "Yes, create additional story", description: "Append new story to existing follow-up"},
    {label: "No, skip", description: "Keep existing follow-up only"}
  ],
  multiSelect: false
}])

IF user confirms:
    # Append with comma: "STORY-050, STORY-151"
    new_followup = "{current_followup}, {CREATED_STORY_ID}"
```

### Edge Case 3: User Timeout/Cancel

**Scenario:** User presses Ctrl+C during AskUserQuestion prompt

**Handling:**
```
# Treat as decline (same as "No, I'll create it later")
Display: "Story creation skipped"
# Do NOT invoke skill
# Do NOT modify register
Continue to parent workflow
```

---

## Validation Checklist

Before completing this workflow, verify:

- [ ] AskUserQuestion displayed with header "Remediation Story"
- [ ] User decision captured (Yes or No)
- [ ] If Yes: All 8 debt fields extracted
- [ ] If Yes: Story context built with correct mapping
- [ ] If Yes: devforgeai-story-creation skill invoked
- [ ] If Yes: Follow-up field updated in register
- [ ] If Yes: last_updated field updated in register
- [ ] If Yes: Confirmation message displayed
- [ ] If No: Skip message displayed
- [ ] If No: No skill invocation
- [ ] If No: No register modification
- [ ] Parent workflow continued

---

## References

- **STORY-285:** Register Format Standardization (v2.0 YAML format)
- **STORY-286:** /dev Phase 06 Automation (debt confirmation trigger)
- **STORY-287:** QA Hook Integration (hook completion trigger)
- **devforgeai-story-creation skill:** Story creation interface
- **technical-debt-register.md:** Debt tracking register
