# RCA Parsing Workflow (Phases 1-5)

Detailed pseudocode for parsing RCA documents and extracting recommendations.

---

## Reusable Helpers

**Helper: validate_enum(value, valid_values, default, field_name, context)**
```
# Generic enum validation with warning on invalid values
IF value NOT IN valid_values:
    Display: "Warning: Invalid ${field_name} '${value}'${context}, defaulting to ${default}"
    RETURN default
RETURN value
```

**Helper: format_effort_estimate(effort_hours)**
```
# Reusable effort formatting - returns "Nh" or "N/A"
RETURN "${effort_hours}h" IF effort_hours ELSE "N/A"
```

---

## Phase 1: Locate RCA File (Prerequisite)

```
# Find RCA file matching ID
Glob(pattern="devforgeai/RCA/${RCA_ID}*.md")

IF no files found:
    Display: "RCA not found: ${RCA_ID}"
    Display: "Available RCAs:"
    Glob(pattern="devforgeai/RCA/RCA-*.md")
    HALT

RCA_FILE = first matching file
Display: "Parsing: ${RCA_FILE}"
```

---

## Phase 2: Parse Frontmatter (AC#1)

```
# Read RCA file content
Read(file_path="${RCA_FILE}")

# Extract YAML frontmatter between --- markers
FRONTMATTER_PATTERN = content between first "---" and second "---"

# Parse required fields
rca_document = {
    id: extract_field("id"),           # Format: RCA-NNN
    title: extract_field("title"),
    date: extract_field("date"),       # Format: YYYY-MM-DD
    severity: extract_field("severity"), # Enum: CRITICAL|HIGH|MEDIUM|LOW
    status: extract_field("status"),   # Enum: OPEN|IN_PROGRESS|RESOLVED
    reporter: extract_field("reporter"),
    recommendations: []
}

# Validate enums using reusable helper
rca_document.severity = validate_enum(severity, VALID_PRIORITIES, DEFAULT_PRIORITY, "severity", "")
rca_document.status = validate_enum(status, VALID_STATUSES, DEFAULT_STATUS, "status", "")

# Edge case: Missing frontmatter
IF no frontmatter found:
    Display: "Warning: No frontmatter found, extracting ID from filename"
    rca_document.id = extract_id_from_filename(RCA_FILE)
```

**Helper: extract_field(field_name)**
```
# Pattern: "field_name: value" or "field_name: 'value'"
Grep(pattern="^${field_name}:", path="${RCA_FILE}")
RETURN value after colon, trimmed
```

---

## Phase 3: Extract Recommendations (AC#2, AC#3, AC#4)

```
# Find all recommendation sections: ### REC-N: PRIORITY - Title
Grep(pattern="^### REC-[0-9]+:", path="${RCA_FILE}", output_mode="content")

FOR each recommendation_header in matches:
    # Parse header: ### REC-1: HIGH - Fix Database Connection
    rec = {
        id: extract "REC-N" from header,
        priority: extract priority (CRITICAL|HIGH|MEDIUM|LOW),
        title: extract title after " - ",
        description: "",
        effort_hours: null,
        effort_points: null,
        success_criteria: []
    }

    # Validate priority using reusable helper
    rec.priority = validate_enum(priority, VALID_PRIORITIES, DEFAULT_PRIORITY, "priority", " for ${rec.id}")

    # Extract description (content between this header and next ### or end)
    rec.description = extract_section_content(rec.id)

    # Extract effort estimate (AC#3)
    effort_line = Grep(pattern="\\*\\*Effort Estimate:\\*\\*", section_content)
    IF effort_line found:
        IF contains "hours":
            rec.effort_hours = parse_int(hours_value)
        ELIF contains "story points" OR contains "points":
            rec.effort_points = parse_int(points_value)
            # BR-003: Convert story points to hours using constant
            rec.effort_hours = rec.effort_points * STORY_POINTS_TO_HOURS

    # Extract success criteria (AC#4)
    IF section contains "**Success Criteria:**":
        criteria_section = content after "**Success Criteria:**" until next "**" or "###"
        FOR each line matching "- [ ]" OR "- [x]" pattern:
            rec.success_criteria.append(checkbox_text)

    rca_document.recommendations.append(rec)

Display: "Found ${rca_document.recommendations.length} recommendations"
```

---

## Phase 4: Filter and Sort (AC#5)

```
# BR-001: Filter by effort threshold
IF EFFORT_THRESHOLD > 0:
    filtered_recommendations = []
    FOR rec in rca_document.recommendations:
        IF rec.effort_hours >= EFFORT_THRESHOLD:
            filtered_recommendations.append(rec)
        ELSE:
            Display: "Filtered out: ${rec.id} (effort: ${rec.effort_hours}h < threshold: ${EFFORT_THRESHOLD}h)"

    rca_document.recommendations = filtered_recommendations

# BR-002: Sort by priority using PRIORITY_ORDER constant
rca_document.recommendations.sort(key=lambda r: PRIORITY_ORDER[r.priority])

Display: "After filtering: ${rca_document.recommendations.length} recommendations"
```

---

## Phase 5: Display Results

```
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  RCA Document: ${rca_document.id}"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""
Display: "**Title:** ${rca_document.title}"
Display: "**Date:** ${rca_document.date}"
Display: "**Severity:** ${rca_document.severity}"
Display: "**Status:** ${rca_document.status}"
Display: "**Reporter:** ${rca_document.reporter}"
Display: ""

IF rca_document.recommendations.length == 0:
    Display: "No recommendations found (or all filtered out)."
ELSE:
    Display: "## Recommendations (${rca_document.recommendations.length})"
    Display: ""

    FOR rec in rca_document.recommendations:
        Display: "### ${rec.id}: ${rec.priority} - ${rec.title}"
        Display: ""
        Display: "${rec.description}"
        Display: ""

        IF rec.effort_hours:
            Display: "**Effort:** ${rec.effort_hours} hours"
            IF rec.effort_points:
                Display: " (${rec.effort_points} story points)"

        IF rec.success_criteria.length > 0:
            Display: ""
            Display: "**Success Criteria:**"
            FOR criterion in rec.success_criteria:
                Display: "- [ ] ${criterion}"

        Display: ""
        Display: "---"
        Display: ""

Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
```

---

## Edge Cases Handled

| Edge Case | Behavior |
|-----------|----------|
| Missing frontmatter | Extract ID from filename, log warning |
| No recommendations | Return empty array, display message |
| Missing effort estimate | Return null for effort_hours |
| Malformed priority | Default to MEDIUM, log warning |
| Malformed severity | Default to MEDIUM, log warning |
| Malformed status | Default to OPEN, log warning |
| Story points format | Convert to hours (1 point = 4 hours) per BR-003 |
| All filtered out | Return empty array, display filter message |
