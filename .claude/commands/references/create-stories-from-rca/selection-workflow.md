# Interactive Selection Workflow (Phases 6-9)

Detailed pseudocode for interactive recommendation selection.

---

## Phase 6: Display Recommendation Summary Table (AC#1)

```
# function display_recommendation_table($1 = rca_document.recommendations)
# Display formatted table (readable in 80-char terminal)
# Uses printf-style padding for align column width formatting
IF rca_document.recommendations.length > 0:
    Display: ""
    Display: "┌─────────┬──────────┬────────────────────────────────────┬────────┐"
    Display: "│ REC ID  │ Priority │ Title                              │ Effort │"
    Display: "├─────────┼──────────┼────────────────────────────────────┼────────┤"

    FOR rec in rca_document.recommendations:
        # Truncate title to fit 34 chars
        display_title = rec.title[:34] IF len(rec.title) > 34 ELSE rec.title.ljust(34)
        effort_str = format_effort_estimate(rec.effort_hours)
        Display: "│ ${rec.id.ljust(7)} │ ${rec.priority.ljust(8)} │ ${display_title} │ ${effort_str.rjust(6)} │"

    Display: "└─────────┴──────────┴────────────────────────────────────┴────────┘"
    Display: ""
```

---

## Phase 7: Interactive Selection (AC#2, AC#3, AC#4)

```
# function prompt_user_for_selection($1 = recommendations_array)
# Integrates with STORY-155 RCA parser output format
# Returns selected_options after user selection

# Edge case: No recommendations after filtering
IF rca_document.recommendations.length == 0:
    Display: "No recommendations meet effort threshold. Exiting."
    HALT

# Build options array for AskUserQuestion
options = []

# Add "All recommendations" option first (recommended)
options.append({
    label: "All recommendations (Recommended)",
    description: "Create stories for all ${rca_document.recommendations.length} eligible recommendations"
})

# Add individual recommendation options
FOR rec in rca_document.recommendations:
    effort_str = format_effort_estimate(rec.effort_hours)
    options.append({
        label: "${rec.id}: ${rec.title[:30]}",
        description: "Priority: ${rec.priority}, Effort: ${effort_str}"
    })

# Add "None - cancel" option last
options.append({
    label: "None - cancel",
    description: "Exit without creating stories"
})

# Prompt user with multiSelect: true
AskUserQuestion(
    questions=[{
        question: "Which recommendations should be converted to stories?",
        header: "Select",
        multiSelect: true,
        options: options
    }]
)

# Capture user selection
# echo selected_recs, return selection for downstream
user_selection = captured from AskUserQuestion response
```

---

## Phase 8: Handle Selection

```
selected_recommendations = []

# Handle "None - cancel" option - exit gracefully without creating stories
# if "None" or if cancel detected: return early, skip creation, prevents downstream
# echo "No recommendations" message printed before exit (no cleanup required)
IF user_selection contains "None - cancel":
    Display: "No recommendations selected. Exiting."
    return 0  # exit 0 gracefully
    HALT

# Handle "All recommendations" option - Handles "All" selection
# Excludes ineligible recommendations (already filtered in Phase 4)
# if All = true: selected_recommendations = all eligible
IF user_selection contains "All recommendations":
    selected_recommendations = rca_document.recommendations  # All = selected
    Display: "Selected all ${selected_recommendations.length} recommendations"

# Handle individual selections
ELSE:
    FOR selection in user_selection:
        # Extract REC ID from selection label
        IF selection matches "REC-[0-9]+":
            rec_id = extract REC ID
            rec = find_recommendation_by_id(rec_id)
            IF rec:
                selected_recommendations.append(rec)
            ELSE:
                Display: "Warning: Invalid REC ID '${rec_id}', ignoring"

    # Handle "Other" (custom comma-separated input)
    IF user_selection contains custom text:
        custom_ids = parse comma-separated REC IDs
        FOR rec_id in custom_ids:
            rec = find_recommendation_by_id(rec_id)
            IF rec:
                selected_recommendations.append(rec)
            ELSE:
                Display: "Warning: Invalid REC ID '${rec_id}', ignoring"

# Validate minimum selection (BR-001)
IF selected_recommendations.length == 0:
    Display: "No valid recommendations selected. Please try again."
    GOTO Phase 7  # Re-prompt

Display: "Selected ${selected_recommendations.length} recommendation(s) for story creation"
```

---

## Phase 9: Pass to Batch Story Creation (AC#5)

```
# Selected recommendations passed to next phase - pass selection forward
# No data loss in transformation - all fields complete and intact
# Output format compatible with batch creation - expected format for batch
# Preserve all metadata for batch creation (BR-002)
batch_input = {
    rca_document: {
        id: rca_document.id,
        title: rca_document.title,
        severity: rca_document.severity
    },
    selected_recommendations: selected_recommendations,
    selection_count: selected_recommendations.length
}

# Each recommendation preserves full metadata:
# - id (REC-N)
# - priority (CRITICAL|HIGH|MEDIUM|LOW)
# - title (string)
# - description (string)
# - effort_hours (integer|null)
# - effort_points (integer|null)
# - success_criteria (array)

Display: ""
Display: "Proceeding to batch story creation with ${selection_count} recommendation(s)..."
Display: ""

# Return batch_input for next phase (batch story creation)
```

---

## Edge Cases Handled

| Edge Case | Behavior |
|-----------|----------|
| Single recommendation | Still display selection prompt (allow cancel) |
| No recommendations after filter | Display "No recommendations meet effort threshold" and exit |
| User selects "Other" | Parse comma-separated REC IDs from custom input |
| Invalid REC ID in selection | Log warning and ignore invalid entries |
| Empty selection | Re-prompt user for valid selection |
