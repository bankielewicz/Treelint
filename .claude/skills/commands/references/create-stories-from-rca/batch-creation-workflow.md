# Batch Story Creation Workflow (Phase 10)

Detailed pseudocode for batch story creation from STORY-157.

---

## AC#1: Map Recommendation Fields to Story Batch Markers

```
# function map_recommendation_to_batch_markers($1 = recommendation)
# Maps RCA recommendation fields to devforgeai-story-creation batch context markers
# Returns batch_context object ready for skill invocation

batch_context = {
    # Story ID: Next available STORY-NNN (generated sequentially)
    story_id: get_next_story_id(),

    # Epic ID: From RCA source or standalone
    epic_id: rca_document.epic_id OR null,

    # Feature Name: From recommendation title
    feature_name: recommendation.title,

    # Feature Description: From recommendation description
    feature_description: recommendation.description,

    # Priority: Map RCA priority to story priority
    # BR-001: CRITICAL/HIGH -> High, MEDIUM -> Medium, LOW -> Low
    priority: map_priority(recommendation.priority),

    # Points: Use recommendation effort_points or default to 5
    # BR-002: Points Calculation
    points: recommendation.effort_points OR 5,

    # Type: Always "feature" for RCA recommendations
    type: "feature",

    # Sprint: User selection or "Backlog"
    sprint: selected_sprint OR "Backlog",

    # Batch Mode: Always true for batch creation
    batch_mode: true,

    # Source RCA: RCA document ID (e.g., RCA-022)
    source_rca: rca_document.id,

    # Source Recommendation: Recommendation ID (e.g., REC-1)
    source_recommendation: recommendation.id
}

RETURN batch_context
```

**Priority Mapping Function:**
```
# function map_priority($1 = rca_priority)
# BR-001: Priority Mapping - Maps RCA priority to story priority

SWITCH rca_priority:
    CASE "CRITICAL": RETURN "High"
    CASE "HIGH":     RETURN "High"
    CASE "MEDIUM":   RETURN "Medium"
    CASE "LOW":      RETURN "Low"
    DEFAULT:         RETURN "Medium"
```

**Story ID Generation Function:**
```
# function get_next_story_id()
# BR-003: Story ID Generation - Returns next sequential STORY-NNN

# Find highest existing story ID
Glob(pattern="devforgeai/specs/Stories/STORY-*.story.md")
existing_ids = extract_story_numbers(glob_results)
max_id = max(existing_ids) OR 0

# Return next sequential ID (no gaps)
next_id = max_id + 1
RETURN "STORY-${next_id}"
```

---

## AC#2: Invoke Story Creation Skill in Batch Mode

```
# function invoke_story_creation_batch($1 = batch_context)
# Invokes devforgeai-story-creation skill in batch mode
# Skips Phase 1 questions, executes Phases 2-7

# Set context markers for skill
$STORY_ID = batch_context.story_id
$EPIC_ID = batch_context.epic_id
$FEATURE_NAME = batch_context.feature_name
$FEATURE_DESCRIPTION = batch_context.feature_description
$PRIORITY = batch_context.priority
$POINTS = batch_context.points
$TYPE = batch_context.type
$SPRINT = batch_context.sprint
$BATCH_MODE = true  # Enables batch mode: skips Phase 1 questions
$SOURCE_RCA = batch_context.source_rca
$SOURCE_RECOMMENDATION = batch_context.source_recommendation

# Invoke skill with batch mode flag
# Phase 1 (interactive questions) is SKIPPED in batch mode
# Phases 2-7 execute normally:
#   Phase 2: Requirements Analysis
#   Phase 3: Acceptance Criteria Creation
#   Phase 4: Technical Specification
#   Phase 5: UI Specification (if needed)
#   Phase 6: Story File Creation
#   Phase 7: Validation

Skill(command="devforgeai-story-creation", args="--batch")

# Skill returns story creation result
RETURN skill_result
```

---

## AC#3: Create Stories Sequentially with Progress Display

```
# Phase 10.3: Sequential Story Creation
# Creates stories one at a time with progress display

created_stories = []   # Array of successfully created story IDs
failed_stories = []    # Array of {title, error_message}
total_count = selected_recommendations.length
current_index = 0

FOR recommendation in selected_recommendations:
    current_index = current_index + 1

    # Display progress: "[N/Total] Creating: {title}"
    Display: "[${current_index}/${total_count}] Creating: ${recommendation.title}"

    # Map recommendation to batch context markers (AC#1)
    batch_context = map_recommendation_to_batch_markers(recommendation)

    # Invoke skill in batch mode (AC#2)
    TRY:
        result = invoke_story_creation_batch(batch_context)

        IF result.success:
            created_stories.append({
                story_id: batch_context.story_id,
                feature_title: recommendation.title
            })
            Display: "  ✓ Created: ${batch_context.story_id}"
        ELSE:
            # Handle skill failure (AC#4)
            failed_stories.append({
                feature_title: recommendation.title,
                error_message: result.error_message
            })
            Display: "  ✗ Failed: ${result.error_message}"

    CATCH error:
        # Handle exception (AC#4)
        failed_stories.append({
            feature_title: recommendation.title,
            error_message: error.message
        })
        Display: "  ✗ Error: ${error.message}"

        # BR-004: Failure Isolation - Continue to next recommendation
        CONTINUE  # Do not stop, proceed to next

Display: ""
Display: "Batch creation complete."
```

---

## AC#4: Handle Story Creation Failure

```
# Error Handling for Batch Story Creation
# Implements BR-004: Failure Isolation

# Error types and handling:
# 1. Validation Error: Story spec fails validation
#    - Log: "Validation failed: {reason}"
#    - Action: Add to failed_stories, continue to next
#
# 2. Skill Invocation Error: devforgeai-story-creation fails
#    - Log: "Skill error: {message}"
#    - Action: Add to failed_stories, continue to next
#
# 3. Story ID Conflict: ID already exists (edge case)
#    - Log: "ID conflict: ${story_id}, retrying with next ID"
#    - Action: Increment ID and retry once, then fail if still conflicts
#
# 4. Context Window Limit: Too many recommendations
#    - Log: "Processing in batches of 5 due to context limit"
#    - Action: Process recommendations in groups of 5

# Failure tracking structure
failed_story = {
    feature_title: "string",     # Original recommendation title
    error_message: "string",     # Detailed error message
    recommendation_id: "REC-N",  # Source recommendation ID
    attempted_story_id: "STORY-NNN"  # Attempted story ID
}

# BR-004: Failure Isolation Implementation
# - Failure in story N does NOT affect story N+1
# - Each story creation is independent
# - Errors are logged and tracked, not propagated
# - Processing continues until all recommendations processed
```

---

## AC#5: Report Success and Failure Summary

```
# Phase 10.5: Summary Report
# Display final results after batch creation completes

Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  Batch Story Creation Summary"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""

# Success count and list
success_count = created_stories.length
IF success_count > 0:
    Display: "✅ Created: ${success_count} stories"
    FOR story in created_stories:
        Display: "   • ${story.story_id}: ${story.feature_title}"
ELSE:
    Display: "✅ Created: 0 stories"

Display: ""

# Failure count and list
failure_count = failed_stories.length
IF failure_count > 0:
    Display: "❌ Failed: ${failure_count} stories"
    FOR failure in failed_stories:
        Display: "   • ${failure.feature_title}"
        Display: "     Reason: ${failure.error_message}"
ELSE:
    Display: "❌ Failed: 0 stories"

Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Return summary
batch_result = {
    success: created_stories,
    failed: failed_stories,
    success_count: success_count,
    failure_count: failure_count,
    total_processed: total_count
}

RETURN batch_result
```

---

## Return Value Structure

```json
{
  "rca_document": {
    "id": "RCA-NNN",
    "title": "string",
    "date": "YYYY-MM-DD",
    "severity": "CRITICAL|HIGH|MEDIUM|LOW",
    "status": "OPEN|IN_PROGRESS|RESOLVED",
    "reporter": "string",
    "recommendations": [
      {
        "id": "REC-N",
        "priority": "CRITICAL|HIGH|MEDIUM|LOW",
        "title": "string",
        "description": "string",
        "effort_hours": "integer|null",
        "effort_points": "integer|null",
        "success_criteria": ["string"]
      }
    ]
  },
  "filter_applied": "boolean",
  "threshold_hours": "integer",
  "recommendations_count": "integer",
  "selected_recommendations": [...],
  "selection_count": "integer",
  "selection_mode": "all|individual|cancel"
}
```

---

## Business Rules (STORY-157)

| Rule | Implementation |
|------|----------------|
| BR-001: Priority Mapping | CRITICAL/HIGH -> High, MEDIUM -> Medium, LOW -> Low |
| BR-002: Points Calculation | Use recommendation.effort_points OR default to 5 |
| BR-003: Story ID Generation | Sequential STORY-NNN (no gaps in numbering) |
| BR-004: Failure Isolation | Each story creation is independent, failures logged and skipped |

---

## Error Handling

| Error Type | Handling | Recovery |
|------------|----------|----------|
| Validation Error | Log error, add to failed_stories | Continue to next recommendation |
| Skill Invocation Error | Log error, add to failed_stories | Continue to next recommendation |
| Story ID Conflict | Increment ID, retry once | Fail if still conflicts |
| Context Window Limit | Process in batches of 5 | Automatic batching |

**Key Principle:** BR-004 ensures that a failure creating story N does not prevent or affect the creation of story N+1.
