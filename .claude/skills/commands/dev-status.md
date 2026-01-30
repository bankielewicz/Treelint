---
name: dev-status
description: Display development progress for a story without invoking full workflow
argument-hint: [STORY-ID]
model: opus
allowed-tools: Read, Glob, Grep
---

# /dev-status - Show Development Progress

Display current development progress for a story without invoking full workflow.

**Read-only command - no side effects, no skill invocation**

---

## Quick Reference

```bash
# Show status for a story
/dev-status STORY-057

# Example output
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Development Status: STORY-057
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**Story:** Additional Skill Integrations
**Status:** In Development

**Progress:**
- Current Phase: 06 (Deferral Challenge)
- DoD Completion: 26/30 (87%)
- TDD Iteration: 2

**Remaining DoD Items:**
- Implementation: 2 items
- Quality: 1 item
- Testing: 1 item

**Suggested Next Action:**
Run `/dev STORY-057` to continue development
OR
Run `/qa STORY-057` when development complete
```

---

## Command Workflow

### Phase 0: Argument Validation

**Parse arguments:**
```
STORY_ID = null

# Parse arguments
FOR arg in arguments:
    IF arg matches "STORY-[0-9]+":
        STORY_ID = arg

IF STORY_ID empty:
    Display: "Usage: /dev-status STORY-NNN"
    Display: "Example: /dev-status STORY-057"
    HALT
```

**Locate story file:**
```
Glob(pattern="devforgeai/specs/Stories/${STORY_ID}*.story.md")

IF no file found:
    Display: "❌ Story not found: ${STORY_ID}"
    Display: "Run: Glob(pattern='devforgeai/specs/Stories/*.story.md') to list stories"
    HALT

STORY_FILE = matched file path
```

---

### Phase 1: Extract Current State

**1.1 Load story file and extract metadata:**
```
Read(file_path="${STORY_FILE}")

# Extract from YAML frontmatter:
STORY_TITLE = value of "title:" field
STORY_STATUS = value of "status:" field
```

**1.2 Load phase state (if exists):**
```
Glob(pattern="devforgeai/workflows/${STORY_ID}-phase-state.json")

IF file exists:
    Read(file_path="devforgeai/workflows/${STORY_ID}-phase-state.json")

    CURRENT_PHASE = value of "current_phase" field
    ITERATION_COUNT = value of "iteration_count" field (default: 1)
ELSE:
    CURRENT_PHASE = "Not Started"
    ITERATION_COUNT = 1
```

**1.3 Count DoD items:**
```
# Parse Definition of Done section from story file
# Count checkboxes: - [ ] (incomplete) and - [x] (complete)

TOTAL_DOD = count of all "- [ ]" and "- [x]" in DoD section
COMPLETE_DOD = count of "- [x]" in DoD section
DOD_PERCENTAGE = (COMPLETE_DOD / TOTAL_DOD) * 100

# Categorize remaining items by section header
REMAINING_IMPLEMENTATION = incomplete items under "### Implementation"
REMAINING_TESTING = incomplete items under "### Testing"
REMAINING_DOCUMENTATION = incomplete items under "### Documentation"
```

**Phase Name Mapping:**
```
PHASE_NAMES = {
    "01": "Pre-Flight Validation",
    "02": "Red Phase (Test Generation)",
    "03": "Green Phase (Implementation)",
    "04": "Refactor Phase",
    "05": "Integration Testing",
    "06": "Deferral Challenge",
    "07": "DoD Update",
    "08": "Git Workflow",
    "09": "Feedback Hook",
    "10": "Result Interpretation"
}

PHASE_DISPLAY = "${CURRENT_PHASE} (${PHASE_NAMES[CURRENT_PHASE]})"
```

---

### Phase 2: Display Status

**Format and display output:**
```
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  Development Status: ${STORY_ID}"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""
Display: "**Story:** ${STORY_TITLE}"
Display: "**Status:** ${STORY_STATUS}"
Display: ""
Display: "**Progress:**"
Display: "- Current Phase: ${PHASE_DISPLAY}"
Display: "- DoD Completion: ${COMPLETE_DOD}/${TOTAL_DOD} (${DOD_PERCENTAGE}%)"
Display: "- TDD Iteration: ${ITERATION_COUNT}"
Display: ""
```

**Display remaining items (if any):**
```
IF TOTAL_DOD > COMPLETE_DOD:
    Display: "**Remaining DoD Items:**"

    IF REMAINING_IMPLEMENTATION > 0:
        Display: "- Implementation: ${REMAINING_IMPLEMENTATION} item(s)"
        FOR item in incomplete_implementation_items:
            Display: "  ${item}"

    IF REMAINING_TESTING > 0:
        Display: "- Testing: ${REMAINING_TESTING} item(s)"
        FOR item in incomplete_testing_items:
            Display: "  ${item}"

    IF REMAINING_DOCUMENTATION > 0:
        Display: "- Documentation: ${REMAINING_DOCUMENTATION} item(s)"
        FOR item in incomplete_documentation_items:
            Display: "  ${item}"

    Display: ""
```

---

### Phase 3: Suggest Next Action

**Determine appropriate next action:**
```
Display: "**Suggested Next Action:**"

IF STORY_STATUS == "Dev Complete" OR DOD_PERCENTAGE == 100:
    Display: "Run \`/qa ${STORY_ID}\` - development complete"

ELIF STORY_STATUS == "QA Approved":
    Display: "Run \`/release ${STORY_ID}\` - ready for release"

ELIF CURRENT_PHASE != "Not Started" AND CURRENT_PHASE != "10":
    Display: "Run \`/dev ${STORY_ID}\` to continue development"
    Display: "OR"
    Display: "Run \`/resume-dev ${STORY_ID} ${CURRENT_PHASE}\` to resume from Phase ${CURRENT_PHASE}"

ELSE:
    Display: "Run \`/dev ${STORY_ID}\` to start development"

Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
```

---

## Error Handling

**Invalid Story ID:** Usage: /dev-status STORY-NNN (e.g., /dev-status STORY-057)

**Story Not Found:** Run `Glob(pattern="devforgeai/specs/Stories/*.story.md")` to list available stories

**No Phase State:** Shows "Not Started" for current phase (story hasn't been developed yet)

---

## Non-Functional Requirements

**Performance:** Command executes in <2 seconds (read-only operations only)

**Read-Only:** Command does NOT:
- Modify story file
- Modify phase-state.json
- Invoke any skills
- Make any Git operations

---

**Created 2026-01-04 (STORY-171):** RCA-013 REC-5 implementation
**Pattern:** Lightweight status display - no skill invocation, read-only
