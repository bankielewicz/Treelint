# Phase 6: Epic/Sprint Linking

Update epic and sprint documents to include references to newly created story.

## Overview

This phase establishes bidirectional links between the story and its parent epic/sprint documents for traceability.

---

## Step 6.1: Update Epic File (If Associated)

**Objective:** Add story reference to epic's Stories section

**If epic_id is not null:**

```
# Read epic file
epic_file_path = f"devforgeai/specs/Epics/{epic_id}.epic.md"
epic_content = Read(file_path=epic_file_path)

# Find "Stories" section or create it
if "## Stories" not in epic_content:
    # Add Stories section before "## Workflow History" or at end
    Edit(
      file_path=epic_file_path,
      old_string="## Workflow History",
      new_string="""## Stories

- [{story_id}] {story_title} - {status} ({points} points, Priority: {priority})

## Workflow History"""
    )
else:
    # Append to existing Stories section
    Edit(
      file_path=epic_file_path,
      old_string="## Stories\n",
      new_string=f"""## Stories

- [{story_id}] {story_title} - {status} ({points} points, Priority: {priority})
"""
    )
```

**Update epic statistics:**
```
# Count total story points in epic
# Update epic frontmatter or summary
```

---

## Step 6.2: Update Sprint File (If Associated)

**Objective:** Add story reference to sprint's Sprint Backlog section

**If sprint_id is not "Backlog":**

```
# Read sprint file
sprint_file_path = f"devforgeai/specs/Sprints/{sprint_id}.md"
sprint_content = Read(file_path=sprint_file_path)

# Find "Sprint Backlog" section
Edit(
  file_path=sprint_file_path,
  old_string="## Sprint Backlog\n",
  new_string=f"""## Sprint Backlog

- [{story_id}] {story_title} - {status} ({points} points, Priority: {priority})
"""
)

# Update sprint capacity
# Total points = sum of all story points in sprint
```

---

## Step 6.3: Verify Linking

**Objective:** Validate updates succeeded

**Validate updates:**
```
# Re-read epic file
if epic_id:
    epic_content = Read(file_path=epic_file_path)
    if story_id not in epic_content:
        ERROR: Epic linking failed
        # Retry or report to user

# Re-read sprint file
if sprint_id != "Backlog":
    sprint_content = Read(file_path=sprint_file_path)
    if story_id not in sprint_content:
        ERROR: Sprint linking failed
        # Retry or report to user
```

---

## Output

**Phase 6 produces:**
- ✅ Epic file updated with story reference (if applicable)
- ✅ Sprint file updated with story reference (if applicable)
- ✅ Story counts and points updated
- ✅ Links verified functional

---

## Error Handling

**Error 1: Epic file not found**
- **Detection:** epic_id specified but file doesn't exist at devforgeai/specs/Epics/{epic_id}.epic.md
- **Recovery:** Ask user if epic ID correct, or create story without epic association

**Error 2: Sprint file not found**
- **Detection:** sprint_id specified but file doesn't exist at devforgeai/specs/Sprints/{sprint_id}.md
- **Recovery:** Ask user if sprint ID correct, or set sprint to "Backlog"

**Error 3: Edit failed (section not found)**
- **Detection:** "## Stories" or "## Sprint Backlog" section missing, Edit tool fails
- **Recovery:** Create section in correct location, retry Edit

**Error 4: Verification failed (story ID not in file after update)**
- **Detection:** Re-read shows story_id missing from updated file
- **Recovery:** Retry Edit with different old_string anchor, or manually insert

See `error-handling.md` for comprehensive error recovery procedures.

---

## Next Phase

**After Phase 6 completes →** Phase 7: Self-Validation

Load `story-validation-workflow.md` for Phase 7 workflow.
