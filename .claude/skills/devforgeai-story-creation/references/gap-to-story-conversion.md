# Gap-to-Story Conversion Reference

Convert coverage gaps detected by `/validate-epic-coverage` into properly-structured story creation requests.

---

## Purpose

This reference defines the algorithm for extracting feature metadata from epic files and generating story context markers for batch mode story creation. It enables seamless conversion of detected coverage gaps into implementation-ready user stories.

---

## Gap Data Structure (Input)

**Source:** Output from `gap-detector.sh` or `generate-report.sh`

```yaml
gap:
  epic_id: "EPIC-015"              # Required - Source epic ID
  feature_number: "1.3"            # Required - Feature section number (X.Y format)
  feature_title: "Story Creation Integration"  # Required - Feature name
  feature_description: "Integrate gap detection with story creation workflow"  # Optional
  feature_sub_bullets:             # Optional - Sub-items from epic
    - "Interactive prompts for gap resolution"
    - "Batch creation mode"
  section_reference: "### Feature 1.3: Story Creation Integration"  # Full header text
```

---

## Story Context Markers (Output)

**Target:** Context markers for devforgeai-story-creation skill batch mode

```
**Story ID:** STORY-{next_id}
**Epic ID:** {gap.epic_id}
**Feature Number:** {gap.feature_number}
**Feature Name:** {gap.feature_title}
**Feature Description:** {generated_description}
**Priority:** {user_selected or "Medium"}
**Points:** {user_selected or "5"}
**Sprint:** {user_selected or "Backlog"}
**Batch Mode:** true
**Batch Index:** {current_index}
```

---

## Feature Extraction Algorithm

### Step 1: Parse Epic File for Features

**Regex pattern for feature sections:**
```
^###\s+Feature\s+(\d+\.\d+):\s*(.+)$
```

**Extraction:**
```python
for line in epic_content:
    match = regex.match(feature_pattern, line)
    if match:
        feature_number = match.group(1)  # e.g., "1.3"
        feature_title = match.group(2).strip()  # e.g., "Story Creation Integration"

        # Extract description (lines until next ### or ##)
        description = extract_until_next_header(epic_content, current_line)

        # Extract sub-bullets (lines starting with "- ")
        sub_bullets = extract_bullets(description)
```

### Step 2: Generate Story Description Template

**Template format (mandatory):**
```
{Feature Title} - {Feature Description}. Implements {EPIC-ID} Feature {X.Y}.
```

**Example:**
```
Story Creation Integration - Integrate gap detection with story creation workflow. Implements EPIC-015 Feature 1.3.
```

**Extended template (when sub-bullets exist):**
```
{Feature Title} - {Feature Description}. Implements {EPIC-ID} Feature {X.Y}.

Key capabilities:
- {sub_bullet_1}
- {sub_bullet_2}
```

### Step 3: Populate Frontmatter Fields

**YAML frontmatter structure:**
```yaml
---
id: STORY-{next_id}
title: {feature_title}
epic: {epic_id}
sprint: {sprint}
status: Backlog
points: {points}
priority: {priority}
assigned_to: Unassigned
created: {today}
format_version: "2.1"
---
```

**Title derivation rules:**
1. Use feature title directly if ≤50 characters
2. If >50 characters, truncate at word boundary + "..."
3. Remove special characters except hyphen and space

### Step 4: Add Traceability Section

**Traceability section (appended to story):**
```markdown
## Traceability

**Parent Epic:** {epic_id}
**Feature Reference:** Feature {feature_number}: {feature_title}
**Gap Detected:** {detection_timestamp}
**Created By:** /create-missing-stories or /validate-epic-coverage
```

---

## Validation Rules

### Required Fields (BR-001)

| Field | Required | Default | Validation |
|-------|----------|---------|------------|
| epic_id | Yes | - | Must match `^EPIC-\d{3}$` |
| feature_number | Yes | - | Must match `^\d+\.\d+$` |
| feature_title | Yes | - | Non-empty string, ≤100 chars |
| feature_description | No | "(No description)" | String, ≤500 words |

### Fallback Behavior

**Missing feature description:**
```
Display: "⚠️ Feature has no description"
Prompt: AskUserQuestion("Provide description or use placeholder?")
Options:
  - "Use placeholder: (Implementation details to be defined)"
  - "Enter description manually"
```

**Feature title too vague (<10 characters):**
```
Display: "⚠️ Feature title may be too vague: '{title}'"
Prompt: AskUserQuestion("Enhance title or proceed?")
Options:
  - "Proceed with current title"
  - "Enter enhanced title manually"
```

---

## Batch Mode Context Detection

**Detection logic:**
```python
def is_batch_mode(context):
    return "**Batch Mode:** true" in context

def extract_batch_markers(context):
    markers = {}
    required = ["Story ID", "Epic ID", "Feature Description", "Priority", "Points", "Sprint"]

    for marker in required:
        pattern = f"\\*\\*{marker}:\\*\\*\\s*(.+)"
        match = regex.search(pattern, context)
        if match:
            markers[marker] = match.group(1).strip()

    return markers
```

**Validation:**
- All required markers must be present for batch mode
- Missing markers trigger fallback to interactive mode (BR-002)

---

## Integration Points

### With /validate-epic-coverage

1. Gap detected → Extract feature metadata
2. User selects "Yes" → Generate context markers
3. Invoke devforgeai-story-creation skill with markers

### With /create-missing-stories

1. Parse epic → Detect all gaps
2. Prompt for shared metadata (sprint, priority, points)
3. For each gap:
   - Generate context markers
   - Set batch mode = true
   - Set batch index
4. Invoke skill in batch mode

---

## Examples

### Example 1: Simple Feature Conversion

**Input (from epic):**
```markdown
### Feature 1.3: Story Creation Integration
Integrate gap detection with story creation workflow.
```

**Output (context markers):**
```
**Story ID:** STORY-089
**Epic ID:** EPIC-015
**Feature Number:** 1.3
**Feature Name:** Story Creation Integration
**Feature Description:** Story Creation Integration - Integrate gap detection with story creation workflow. Implements EPIC-015 Feature 1.3.
**Priority:** Medium
**Points:** 5
**Sprint:** Backlog
**Batch Mode:** true
**Batch Index:** 0
```

### Example 2: Feature with Sub-bullets

**Input (from epic):**
```markdown
### Feature 1.3: Story Creation Integration
Integrate gap detection with story creation workflow.
- Interactive prompts for gap resolution
- Batch creation mode
```

**Output (story description):**
```
Story Creation Integration - Integrate gap detection with story creation workflow. Implements EPIC-015 Feature 1.3.

Key capabilities:
- Interactive prompts for gap resolution
- Batch creation mode
```

---

## Performance Targets

- Feature extraction: <100ms per epic
- Template generation: <50ms per story
- Total conversion (10 gaps): <500ms

---

**Created:** 2025-12-13 (STORY-088)
**Related:** AC#4 (Template Generation), AC#7 (Story Template Population)
