# Batch Mode Configuration Reference

Configuration and validation rules for batch story creation from coverage gaps.

---

## Purpose

Define the context marker schema, validation rules, and default values for batch mode story creation. Enables efficient creation of multiple stories from detected coverage gaps with shared metadata.

---

## Context Marker Schema

### Required Markers

| Marker | Format | Example | Required |
|--------|--------|---------|----------|
| `**Story ID:**` | STORY-NNN | STORY-089 | Yes |
| `**Epic ID:**` | EPIC-NNN | EPIC-015 | Yes |
| `**Feature Number:**` | X.Y | 1.3 | Yes |
| `**Feature Name:**` | String | Story Creation Integration | Yes |
| `**Feature Description:**` | String | Full description text | Yes |
| `**Priority:**` | Critical\|High\|Medium\|Low | Medium | Yes |
| `**Points:**` | Fibonacci | 5 | Yes |
| `**Sprint:**` | String | Sprint-1 or Backlog | Yes |
| `**Batch Mode:**` | true\|false | true | Yes |
| `**Batch Index:**` | Integer | 0 | Yes (in batch) |

### Optional Markers

| Marker | Format | Example | Default |
|--------|--------|---------|---------|
| `**Batch Total:**` | Integer | 5 | - |
| `**Created From:**` | String | /create-missing-stories | - |
| `**Parent Epic Title:**` | String | Epic Coverage Validation | - |

---

## Validation Rules

### Marker Presence Validation

```python
REQUIRED_MARKERS = [
    "Story ID",
    "Epic ID",
    "Feature Description",
    "Priority",
    "Points",
    "Sprint"
]

def validate_batch_markers(context):
    missing = []
    for marker in REQUIRED_MARKERS:
        pattern = f"\\*\\*{marker}:\\*\\*\\s*(.+)"
        if not regex.search(pattern, context):
            missing.append(marker)

    if missing:
        return {"valid": False, "missing": missing}
    return {"valid": True}
```

### Fallback to Interactive Mode (BR-002)

**When markers are missing:**
```
IF validation.valid == False:
    Display: "⚠️ Missing required batch markers: {missing}"
    Display: "Switching to interactive mode..."

    FOR marker in missing:
        IF marker == "Epic ID":
            AskUserQuestion("Which epic does this story belong to?")
        ELSE IF marker == "Priority":
            AskUserQuestion("What priority? [Critical/High/Medium/Low]")
            # Default to "Medium" if no response
        ELSE IF marker == "Points":
            AskUserQuestion("Story points? [1/2/3/5/8/13]")
            # Default to "5" if no response
        ELSE IF marker == "Sprint":
            AskUserQuestion("Which sprint? [Sprint name or 'Backlog']")
            # Default to "Backlog" if no response

    # Continue with gathered values
```

---

## Default Values

### Priority Defaults
- Default: `Medium`
- Used when: `**Priority:**` marker missing AND interactive mode skipped

### Points Defaults
- Default: `5` (standard integration story size)
- Used when: `**Points:**` marker missing AND interactive mode skipped
- Fibonacci sequence: 1, 2, 3, 5, 8, 13, 21

### Sprint Defaults
- Default: `Backlog`
- Used when: `**Sprint:**` marker missing AND interactive mode skipped

---

## Batch Mode Detection

### Detection Logic

```python
def is_batch_mode(context):
    """Check if batch mode is explicitly enabled."""
    batch_marker = regex.search(r"\*\*Batch Mode:\*\*\s*(true|false)", context)
    if batch_marker:
        return batch_marker.group(1).lower() == "true"
    return False

def get_batch_index(context):
    """Extract current batch index."""
    index_marker = regex.search(r"\*\*Batch Index:\*\*\s*(\d+)", context)
    if index_marker:
        return int(index_marker.group(1))
    return 0
```

### Batch Mode Behaviors

**When `**Batch Mode:** true`:**
1. Skip Phase 1 interactive questions (epic/sprint/priority/points selection)
2. Extract all metadata from context markers
3. Execute Phases 2-7 normally (requirements, tech spec, UI spec, file creation, linking, validation)
4. Skip Phase 8 next action AskUserQuestion
5. Return control to batch loop immediately

**When `**Batch Mode:** false` or not present:**
1. Execute full interactive workflow
2. Prompt for all metadata via AskUserQuestion

---

## Shared Metadata Collection

### For Batch Creation (2+ gaps)

**Prompt sequence:**
```
Display: "📋 Batch Story Creation Setup"
Display: "Found {N} gaps to fill. Let's configure shared settings."
Display: ""

# Sprint selection
AskUserQuestion:
  Question: "Which sprint should these stories go into?"
  Options:
    - "Backlog (default)"
    - "Current Sprint"
    - "Next Sprint"
    - "Specify sprint name..."

# Priority selection
AskUserQuestion:
  Question: "Default priority for these stories?"
  Options:
    - "Medium (default)"
    - "High"
    - "Low"
    - "Let me set individually..."

# Points selection
AskUserQuestion:
  Question: "Default story points?"
  Options:
    - "5 points (default for integration)"
    - "3 points (small)"
    - "8 points (large)"
    - "Let me estimate individually..."
```

**Apply to all stories:**
```python
for i, gap in enumerate(gaps):
    markers = generate_markers(
        gap=gap,
        sprint=shared_sprint,
        priority=shared_priority if not individual_priority else ask_for_gap(gap),
        points=shared_points if not individual_points else ask_for_gap(gap),
        batch_index=i,
        batch_total=len(gaps)
    )
    invoke_skill(markers)
```

---

## Error Handling

### Invalid Marker Format

| Error | Handling |
|-------|----------|
| Epic ID format invalid | Display error, prompt for correction |
| Priority not in allowed values | Default to "Medium", warn user |
| Points not Fibonacci | Round to nearest Fibonacci, warn user |
| Story ID already exists | Generate next available ID, warn user |

### Batch Failure Handling (BR-004)

```python
results = {"success": [], "failed": []}

for gap in gaps:
    try:
        story = create_story(gap)
        results["success"].append(story)
    except Exception as e:
        results["failed"].append({
            "gap": gap,
            "error": str(e)
        })
        # Continue to next story - don't stop batch

# Report results
Display: f"✅ Created {len(results['success'])} stories"
if results["failed"]:
    Display: f"⚠️ Failed to create {len(results['failed'])} stories"
    for failure in results["failed"]:
        Display: f"  - {failure['gap'].feature_title}: {failure['error']}"
```

---

## Integration with Story Discovery

**Reference:** `references/story-discovery.md`

**Step 1.0.1 in story-discovery.md handles batch mode:**
```
IF "**Batch Mode:** true" detected in conversation:
    # Extract all values from context markers
    extracted = extract_batch_markers(context)

    IF all required markers present:
        # Skip interactive prompts
        story_id = extracted["Story ID"]
        epic_id = extracted["Epic ID"]
        priority = extracted["Priority"]
        points = extracted["Points"]
        sprint = extracted["Sprint"]
        feature_description = extracted["Feature Description"]

        # Proceed directly to Phase 2
    ELSE:
        # Missing markers - fallback to interactive
        Log: "Batch mode requested but markers incomplete, switching to interactive"
        # Continue with normal interactive flow
```

---

## Examples

### Example 1: Complete Batch Context

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
**Batch Total:** 3
**Created From:** /create-missing-stories
```

### Example 2: Incomplete Markers (Triggers Fallback)

```
**Story ID:** STORY-089
**Epic ID:** EPIC-015
**Feature Description:** Some description
**Batch Mode:** true
```

**Result:** Missing Priority, Points, Sprint → Interactive fallback

---

**Created:** 2025-12-13 (STORY-088)
**Related:** AC#2 (Epic Context Auto-Population), BR-002 (Interactive Fallback)
