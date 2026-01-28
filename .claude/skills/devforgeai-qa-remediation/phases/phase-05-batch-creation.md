# Phase 05: Batch Story Creation

**Purpose:** Convert selected gaps to user stories via batch mode.

---

## Auto Mode (--create-stories Flag)

When `$CREATE_STORIES=true` (set by --create-stories flag):
- **Skip confirmation prompt** - ALL selected gaps automatically become stories
- **Batch mode invocation** without individual prompts
- **Story title pattern**: "Remediate {gap_type}: {description}"
- **Points**: Default 2 (standard remediation effort)
- **Priority**: Inherited from gap severity mapping
- **Epic**: Inherited from --epic flag if provided, otherwise null

```
IF $CREATE_STORIES == true:
    batch_mode = true
    skip_confirmation = true

    # Process ALL selected gaps without prompting
    for each gap in $SELECTED_GAPS:
        # Generate story with remediation title pattern
        story_title = f"Remediate {gap.gap_type}: {gap.description}"
        story_priority = map_severity_to_priority(gap.severity)
        story_points = 2  # Default for remediation
        story_epic = $EPIC_ID or null

        # Invoke story creation in batch mode
        Skill(command="devforgeai-story-creation", args="--batch")

        # Track created story ID
        $CREATED_STORIES.append({
            story_id: new_story_id,
            gap_id: gap.id,
            gap_type: gap.gap_type
        })

    Display: "Created {len($CREATED_STORIES)} remediation stories from QA gaps (--create-stories mode): {story_ids}"
```

### Story Title Pattern

For auto mode, story titles follow the pattern:
```
"Remediate {gap_type}: {description}"
```

Examples:
- "Remediate coverage_gap: Improve test coverage for business layer in indexer.py"
- "Remediate anti_pattern: Fix God Object violation in service.py"
- "Remediate code_quality: Reduce cyclomatic complexity in processor.py"

### Severity to Priority Mapping

| Gap Severity | Story Priority |
|--------------|----------------|
| CRITICAL | High |
| HIGH | High |
| MEDIUM | Medium |
| LOW | Low |

---

## Standard Batch Creation Steps

When not in auto mode, follow standard Phase 05 from SKILL.md:

1. Initialize Tracking (Step 5.1)
2. Get Next Story ID (Step 5.2)
3. For Each Selected Gap (Step 5.3)
   - Generate Story Context Markers
   - Invoke Story Creation
4. Batch Completion Summary (Step 5.4)

---

## Combined Flag Operation

When BOTH `$CREATE_STORIES` AND `$ADD_TO_DEBT` are true:

1. **Phase 05 executes FIRST** - Stories are created
2. **Phase 07 executes SECOND** - Gaps are added to debt register
3. **Story IDs are passed to Phase 07** for Follow-up field pre-population

```
IF $CREATE_STORIES AND $ADD_TO_DEBT:
    # Phase 05 completes first
    execute_batch_story_creation()

    # Pass story IDs to Phase 07 context
    $STORY_ID_MAP = map gap_id -> story_id from $CREATED_STORIES

    # Phase 07 will use this map for Follow-up field
```

---

## Phase 05 Output

| Variable | Value |
|----------|-------|
| `$CREATED_STORIES` | Array of {story_id, gap_id, gap_type} |
| `$FAILED_STORIES` | Array of {gap_id, error} |
| `$STORIES_CREATED_COUNT` | Count of successful creations |
| `$STORY_ID_MAP` | Map of gap_id to story_id (for combined flag mode) |
