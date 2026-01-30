---
description: Create stories for all detected coverage gaps in an epic
argument-hint: "EPIC-NNN [--sprint=NAME] [--priority=LEVEL]"
model: opus
allowed-tools: Read, Glob, Grep, Bash, AskUserQuestion, Skill, TodoWrite
---

# /create-missing-stories - Batch Story Creation from Coverage Gaps

Create stories for all detected coverage gaps in an epic using batch mode.

---

## Quick Reference

```bash
# Create stories for all gaps in EPIC-015
/create-missing-stories EPIC-015

# Display help
/create-missing-stories --help
/create-missing-stories help
```

---

## Command Workflow

### Phase 0: Argument Validation

**Parse command arguments:**
```
ARG=$1

IF ARG == "--help" OR ARG == "help":
    GOTO display_help

IF ARG is empty:
    Display: "❌ Epic ID required"
    Display: ""
    Display: "Usage: /create-missing-stories EPIC-NNN"
    Display: "Example: /create-missing-stories EPIC-015"
    HALT

IF ARG does NOT match "^EPIC-[0-9]{3}$" (case-insensitive):
    Display: "❌ Invalid epic ID format: ${ARG}"
    Display: ""
    Display: "Expected format: EPIC-NNN (e.g., EPIC-015)"
    Display: "The ID is case-insensitive (epic-015 works)."
    HALT

EPIC_ID = normalize(ARG)  # Uppercase, e.g., EPIC-015
```

**Verify epic exists:**
```
Glob(pattern="devforgeai/specs/Epics/${EPIC_ID}*.epic.md")

IF file not found:
    Display: "❌ Epic not found: ${EPIC_ID}"
    Display: ""
    Display: "Valid epics:"
    FOR epic in Glob("devforgeai/specs/Epics/*.epic.md"):
        Extract epic_id from filename
        Display: "  • ${epic_id}"
    Display: ""
    Display: "💡 Run /create-epic to create a new epic"
    HALT
```

**Display header:**
```
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  Batch Story Creation: ${EPIC_ID}"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""
```

---

### Phase 1: Gap Detection

**Execute gap detector:**
```
Bash(command="devforgeai/traceability/gap-detector.sh ${EPIC_ID}")

# Parse JSON output for gaps
gaps = parse_json_output(gap_detector_output)
```

**Handle edge cases:**

**No features defined (empty epic):**
```
IF gaps.total_features == 0:
    Display: "ℹ️ No features defined in ${EPIC_ID}"
    Display: ""
    Display: "To define features, edit the epic file:"
    Display: "  devforgeai/specs/Epics/${EPIC_ID}*.epic.md"
    Display: ""
    Display: "Or run /ideate to generate features from a business idea."
    HALT
```

**100% coverage (no gaps):**
```
IF gaps.missing_features.length == 0:
    Display: "✅ ${EPIC_ID} has 100% coverage!"
    Display: ""
    Display: "All ${gaps.total_features} features have stories."
    Display: "No stories to create."
    HALT
```

**Display gap summary:**
```
Display: "📊 Coverage Analysis: ${EPIC_ID}"
Display: ""
Display: "  Total features: ${gaps.total_features}"
Display: "  Covered: ${gaps.covered_features}"
Display: "  Gaps: ${gaps.missing_features.length}"
Display: ""

FOR gap in gaps.missing_features:
    Display: "  ❌ Feature ${gap.feature_number}: ${gap.feature_title}"
```

---

### Phase 2: Shared Metadata Collection

**Prompt for batch settings:**
```
Display: ""
Display: "📋 Configure Batch Settings"
Display: ""

# Sprint selection
AskUserQuestion:
    Question: "Which sprint for these ${gaps.missing_features.length} stories?"
    Options:
        - "Backlog (default)"
        - "Current Sprint"
        - "Next Sprint"
    # User can select "Other" to type custom sprint name

SPRINT = user_selection OR "Backlog"

# Priority selection
AskUserQuestion:
    Question: "Default priority for these stories?"
    Options:
        - "Medium (default)"
        - "High"
        - "Low"
        - "Set individually per story"

PRIORITY = user_selection OR "Medium"
INDIVIDUAL_PRIORITY = (user_selection == "Set individually per story")

# Points selection
AskUserQuestion:
    Question: "Default story points?"
    Options:
        - "5 points (default for features)"
        - "3 points (small features)"
        - "8 points (large features)"
        - "Set individually per story"

POINTS = user_selection OR "5"
INDIVIDUAL_POINTS = (user_selection == "Set individually per story")
```

---

### Phase 3: Batch Story Creation

**Initialize progress tracking:**
```
results = {"success": [], "failed": []}
total = gaps.missing_features.length

Display: ""
Display: "🚀 Creating ${total} stories..."
Display: ""
```

**Create stories in batch mode:**
```
FOR i, gap IN enumerate(gaps.missing_features):
    # Generate story ID
    next_story_id = get_next_story_id()

    # Individual prompts if requested
    IF INDIVIDUAL_PRIORITY:
        AskUserQuestion: "Priority for '${gap.feature_title}'?"
        gap_priority = user_selection
    ELSE:
        gap_priority = PRIORITY

    IF INDIVIDUAL_POINTS:
        AskUserQuestion: "Points for '${gap.feature_title}'?"
        gap_points = user_selection
    ELSE:
        gap_points = POINTS

    # Display progress
    Display: "[${i+1}/${total}] Creating: ${gap.feature_title}"

    # Generate batch context markers
    Set context markers:
        **Story ID:** ${next_story_id}
        **Epic ID:** ${EPIC_ID}
        **Feature Number:** ${gap.feature_number}
        **Feature Name:** ${gap.feature_title}
        **Feature Description:** ${gap.feature_title} - ${gap.feature_description}. Implements ${EPIC_ID} Feature ${gap.feature_number}.
        **Priority:** ${gap_priority}
        **Points:** ${gap_points}
        **Sprint:** ${SPRINT}
        **Batch Mode:** true
        **Batch Index:** ${i}
        **Batch Total:** ${total}
        **Created From:** /create-missing-stories

    # Invoke story creation skill
    TRY:
        Skill(command="devforgeai-story-creation")

        # Verify story was created
        IF story file exists:
            results.success.append({
                "story_id": next_story_id,
                "feature": gap.feature_title
            })
            Display: "  ✅ Created ${next_story_id}"
        ELSE:
            RAISE "Story file not created"

    CATCH Exception as e:
        # BR-004: Continue to next story on failure
        results.failed.append({
            "feature": gap.feature_title,
            "error": str(e)
        })
        Display: "  ❌ Failed: ${e}"
        Display: "     Continuing to next story..."
```

---

### Phase 4: Summary Report

**Display completion summary:**
```
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  Batch Creation Complete"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""
Display: "✅ Successfully created: ${results.success.length} stories"

IF results.failed.length > 0:
    Display: "⚠️ Failed to create: ${results.failed.length} stories"
    Display: ""
    Display: "Failed stories:"
    FOR failure IN results.failed:
        Display: "  • ${failure.feature}: ${failure.error}"
    Display: ""
    Display: "💡 Run /create-story manually for failed features"

Display: ""
Display: "Created stories:"
FOR story IN results.success:
    Display: "  • ${story.story_id}: ${story.feature}"

Display: ""
Display: "Next steps:"
Display: "  • Review created stories in devforgeai/specs/Stories/"
Display: "  • Run /validate-epic-coverage ${EPIC_ID} to verify coverage"
Display: "  • Run /dev STORY-XXX to start implementation"
```

---

## Help Text

When `--help` or `help` is provided:

```
/create-missing-stories - Create stories for coverage gaps in an epic

USAGE:
    /create-missing-stories EPIC-NNN

ARGUMENTS:
    EPIC-NNN    Required. Epic ID to fill gaps for (e.g., EPIC-015)
                Case-insensitive (epic-015 works).

DESCRIPTION:
    This command detects coverage gaps in an epic and creates stories
    for all missing features in batch mode.

    Process:
    1. Detects gaps using the same algorithm as /validate-epic-coverage
    2. Prompts for shared metadata (sprint, priority, points)
    3. Creates stories in batch mode with progress display
    4. Reports success/failure for each story

EXAMPLES:
    # Create stories for all gaps in EPIC-015
    /create-missing-stories EPIC-015

    # Check gaps before creating (preview mode)
    /validate-epic-coverage EPIC-015

OUTPUT:
    - Gap count summary before creation
    - Progress display during creation
    - Success/failure report with story IDs
    - Next steps guidance

ERROR HANDLING:
    - Invalid epic ID: Shows format help
    - Epic not found: Lists valid epics
    - No features: Suggests /ideate
    - 100% coverage: Skips with success message
    - Creation failure: Continues to next story (no rollback)

RELATED COMMANDS:
    /validate-epic-coverage    Check coverage before batch creation
    /create-story              Create individual story
    /create-epic               Create new epic with features
    /dev                       Start story implementation

EXIT CODES:
    0    Success (all stories created or no gaps found)
    1    Partial success (some stories failed)
    2    Error (invalid arguments or epic not found)
```

---

## Error Handling

### Invalid Epic ID Format
```
❌ Invalid epic ID format: ${PROVIDED_ID}

Expected format: EPIC-NNN (e.g., EPIC-015)
The ID is case-insensitive (epic-015 works).
```

### Epic Not Found
```
❌ Epic not found: ${EPIC_ID}

Valid epics:
  • EPIC-001
  • EPIC-002
  • ...

💡 Run /create-epic to create a new epic
```

### Epic Parse Failure
```
❌ Failed to parse epic file: ${EPIC_ID}

Error at line ${LINE_NUMBER}: ${ERROR_MESSAGE}

Please check the epic file format and try again.
```

### Partial Batch Failure
```
⚠️ Batch creation completed with errors

Successfully created: 3 stories
Failed to create: 2 stories

Failed stories:
  • Feature 1.4: File permission error
  • Feature 1.5: Story ID conflict

💡 Run /create-story manually for failed features
```

---

## Business Rules

1. **Epic ID Format (BR-001):** Accepts case-insensitive input. `epic-015` normalized to `EPIC-015`.

2. **Batch Failure Handling (BR-004):** Creation failure for story N does not affect story N+1. Batch continues to completion with partial success report.

3. **Default Values:** Sprint defaults to "Backlog", priority to "Medium", points to "5" if user skips selection.

4. **Story ID Uniqueness:** Each story gets unique ID. Conflicts resolved by incrementing ID.

---

## Performance Targets

- Gap detection: <2 seconds
- Story creation: ~3 seconds per story
- Batch of 10 stories: <30 seconds total

---

## Implementation Notes

**Architecture (STORY-088):**

This command follows **lean orchestration** pattern:
- **Argument validation:** Parse epic ID, verify existence
- **Gap detection:** Delegate to gap-detector.sh
- **Metadata collection:** AskUserQuestion for shared settings
- **Batch creation:** Invoke devforgeai-story-creation skill in batch mode
- **Result display:** Progress and summary report

**Key dependencies:**
- `devforgeai/traceability/gap-detector.sh` - Gap detection
- `devforgeai-story-creation` skill - Story file generation
- `references/gap-to-story-conversion.md` - Template generation
- `references/batch-mode-configuration.md` - Batch mode markers

---

## Story Quality Gates (RCA-020 Fix)

This section documents evidence verification requirements added after RCA-020 identified false claims in generated stories.

**Required Story Elements:**

All stories generated by `/create-missing-stories` must include:

1. **verified_violations section** - When technical specifications claim violations exist, the story MUST include a verified_violations section proving those violations were found.

2. **Specific file paths and line numbers** - Verified violations must include actual line numbers like `lines: [469, 598, 599]`, not generic descriptions.

3. **Target file validation** - All files mentioned in verified_violations are checked to exist and contain the claimed violations.

4. **No placeholders** - Values like "TBD", "TODO", or "N/A" are not permitted in verified_violations sections.

**Story Creation Failure Reasons:**

| Failure Reason | Description | Fix |
|----------------|-------------|-----|
| Target file not found | File path in verified_violations does not exist | Verify file path with Glob/Read before citing |
| Claim not verified | Claimed violation not found at specified lines | Run Grep to confirm violation exists |
| Generic description | Line numbers missing or described generically | Use actual line numbers from Read output |
| Placeholder values | TBD, TODO, N/A in verified_violations | Replace with verified evidence or remove claim |

**Example error message:**
```
CRITICAL: Story creation blocked - Evidence verification failed

Claim: "No specific test output validation patterns"
File: src/claude/skills/devforgeai-story-creation/SKILL.md
Issue: Claimed violation not found at lines 598-599

Action: Verify claim with Read() before including in story.
```

**Why Evidence-Based Verification?**

DevForgeAI enforces evidence-based story creation to ensure:

1. **No false claims** - Every technical specification is backed by verified evidence
2. **Implementable stories** - Developers can trust that claimed violations actually exist
3. **No post-hoc verification** - Stories are ready to implement without re-checking claims
4. **Framework integrity** - Follows Read-Quote-Cite-Verify protocol from `.claude/rules/core/citation-requirements.md`

**References:**
- Grounding Protocol: `.claude/rules/core/citation-requirements.md` (Grounding Protocol section)
- Evidence Gate Implementation: STORY-211
- Citation Validation: STORY-212

---

**Created:** 2025-12-13 (STORY-088)
**Pattern:** Lean orchestration (orchestrate, don't implement)
