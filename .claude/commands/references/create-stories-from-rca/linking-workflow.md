# Phase 11: RCA-Story Linking Workflow (STORY-158)

**Purpose:** Update RCA document with story references for traceability

**Entry Gate:**
```bash
devforgeai-validate phase-check ${RCA_ID} --from=10 --to=11
# Exit code 0: Transition allowed
# Exit code 1: Phase 10 not complete - HALT
# Exit code 2: No created stories from Phase 10 - HALT (no linking needed)
```

---

## Input

`created_stories` array from Phase 10 with mapping of REC-ID → STORY-ID

---

## Step 1: Update Implementation Checklist (AC#1)

```
FOR story in created_stories:
    rec_id = story.source_recommendation  # e.g., "REC-1"
    story_id = story.story_id              # e.g., "STORY-155"

    # Check idempotency - skip if already linked (BR-002)
    IF RCA_FILE contains "- [ ] ${rec_id}: See STORY-":
        Display: "  ⚠ ${rec_id} already linked, skipping"
        CONTINUE

    # Update checklist line
    Edit(
        file_path="${RCA_FILE}",
        old_string="- [ ] ${rec_id}",
        new_string="- [ ] ${rec_id}: See ${story_id}"
    )
    Display: "  ✓ Updated checklist: ${rec_id} → ${story_id}"
```

---

## Step 2: Add Inline Story References (AC#2)

```
FOR story in created_stories:
    rec_id = story.source_recommendation
    story_id = story.story_id

    # Find recommendation section header (Pattern: "### REC-N: Title")
    header_pattern = "### ${rec_id}:"

    # Check idempotency - skip if already has inline reference
    IF RCA_FILE contains "**Implemented in:** ${story_id}" after header_pattern:
        Display: "  ⚠ Inline reference exists for ${rec_id}, skipping"
        CONTINUE

    # Get the full header line (includes title)
    header_line = Grep(pattern="^### ${rec_id}:.*$", path="${RCA_FILE}")

    # Add inline reference after header
    Edit(
        file_path="${RCA_FILE}",
        old_string="${header_line}",
        new_string="${header_line}\n**Implemented in:** ${story_id}"
    )
    Display: "  ✓ Added inline reference: ${rec_id} → ${story_id}"
```

---

## Step 3: Preserve Original Content (AC#3)

- Edit tool performs atomic string replacement
- Only target strings are modified
- All other content (Five Whys, Evidence, descriptions) preserved unchanged
- No full file rewrites

---

## Step 4: Handle Partial Story Creation (AC#4)

```
# Only process created_stories array
# failed_stories array is NOT processed
# Recommendations without stories remain unmarked

linked_count = len(created_stories)
unlinked_count = len(failed_stories)

Display: "Linking Summary:"
Display: "  ✓ Linked: ${linked_count} recommendations"
Display: "  ○ Unlinked: ${unlinked_count} recommendations (story creation failed)"
```

---

## Step 5: Update RCA Status Field (AC#5)

```
total_recommendations = len(created_stories) + len(failed_stories)

IF len(created_stories) == total_recommendations:
    # All recommendations have stories - update status
    Edit(
        file_path="${RCA_FILE}",
        old_string="status: OPEN",
        new_string="status: IN_PROGRESS"
    )
    Display: "  ✓ RCA status updated: OPEN → IN_PROGRESS"
ELSE:
    # Partial completion - keep status as OPEN
    Display: "  ○ RCA status unchanged (partial completion: ${len(created_stories)}/${total_recommendations})"
```

---

## Step 6: Display Summary

```
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  RCA-Story Linking Complete"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  RCA Document: ${RCA_ID}"
Display: "  Linked: ${linked_count} recommendations"
Display: "  Status: ${new_status}"
Display: ""
Display: "  Traceability established:"
FOR story in created_stories:
    Display: "    ${story.source_recommendation} → ${story.story_id}"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
```

---

## Business Rules (STORY-158 AC#6)

| Rule | Implementation |
|------|----------------|
| BR-001: Traceability | Each story linked to source RCA via `source_rca` and `source_recommendation` fields |
| BR-002: Idempotency | Check for existing `: See STORY-` before adding link (no duplicates) |
| BR-003: Partial Linking | Only link recommendations in `created_stories` array; `failed_stories` remain unmarked |
| BR-004: Status Transition | RCA status → IN_PROGRESS only when ALL recommendations have stories |

---

## Validation Checkpoint

Before workflow completion, verify:

- [ ] Checklist items updated for all created stories (AC#1)
- [ ] Inline references added for all created stories (AC#2)
- [ ] Original RCA content preserved (AC#3)
- [ ] Partial linking handled correctly (AC#4)
- [ ] RCA status field updated appropriately (AC#5)
- [ ] Summary display generated

**IF any checkbox UNCHECKED:** HALT workflow

---

## Exit Gate

```bash
devforgeai-validate phase-complete ${RCA_ID} --phase=11 --checkpoint-passed
# Exit code 0: Phase complete, RCA document updated with story links
# Exit code 1: Cannot complete - edit operations failed
```
