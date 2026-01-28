# Technical Debt Update

**Phase:** 07 (Technical Debt Integration)
**Purpose:** Add skipped/deferred gaps to the technical debt register.

---

## Auto Mode (--add-to-debt Flag)

When `$ADD_TO_DEBT=true` (set by --add-to-debt flag), the AutoDebtAdder will:
- When add_to_debt=true, skip confirmation prompt - ALL selected gaps are automatically added
- **Source field** = "qa_remediation" (distinct from "qa_discovery" and "dev_phase_06")
- **Follow-up field** = Pre-populated with STORY-XXX IDs if --create-stories was also used
- **Summary message**: "Added N gap(s) to technical debt register (--add-to-debt mode)"

```
IF $ADD_TO_DEBT == true:
    skip_confirmation = true
    source_field = "qa_remediation"

    IF $CREATED_STORIES is not empty:
        # Combined flag mode - pre-populate Follow-up with story IDs
        for each gap in $DEFERRED_GAPS:
            matching_story = find_story_by_gap_id(gap.id)
            if matching_story:
                gap.follow_up = matching_story.story_id

    # Auto-add without prompting
    add_all_gaps_to_register($DEFERRED_GAPS)

    Display: "Added {len($DEFERRED_GAPS)} gap(s) to technical debt register (--add-to-debt mode)"
```

---

## Overview

Gaps that were filtered out due to `--min-severity` threshold are not lost.
They are automatically added to `devforgeai/technical-debt-register.md` for future processing.

This ensures:
1. No gaps are silently ignored
2. Low-priority issues are tracked
3. Debt reduction sprints have a backlog
4. Complete audit trail of QA findings

---

## Step 1: Check Configuration

```
config = Read(file_path="devforgeai/config/qa-remediation.yaml")

if not config.technical_debt.auto_add_skipped:
    log_info("Technical debt auto-add disabled in config")
    return  // Skip Phase 07
```

---

## Step 2: Read Current Register

```
register_path = config.technical_debt.register_path
// Default: "devforgeai/technical-debt-register.md"

register_content = Read(file_path=register_path)
```

### Expected Structure

```markdown
# Technical Debt Register

## Open Debt Items

(existing items here)

## In Progress Debt Items

(items being worked on)

## Resolved Debt Items

(completed items)

## Analysis

**Total Open Items:** X
**Total In Progress:** Y
**Last Analysis:** YYYY-MM-DD
```

---

## Step 3: Generate Debt Entries

For each gap in `$DEFERRED_GAPS`:

### Entry Template

```markdown
---
#### [GAP-{id}]: {description_summary}

**Date Added:** {YYYY-MM-DD}
**Type:** QA Gap ({gap_type_readable})
**Source Report:** {source_file}
**Original Severity:** {severity}
**Skipped Reason:** Below min-severity threshold (--min-severity {$MIN_SEVERITY})
**Follow-up:** Run `/review-qa-reports --min-severity LOW` to include
**Priority:** {mapped_priority}
**Status:** Open
**Resolution Target:** Next debt reduction sprint
**Estimated Effort:** {points} points

**Gap Details:**
- File: {file_path}
- Layer: {layer or "N/A"}
- Current: {current_value or "N/A"}
- Target: {target_value or "N/A"}
- Remediation: {remediation or "See gap file"}

---
```

### Type Mapping

| Gap Type | Readable Format |
|----------|-----------------|
| coverage_gap | Coverage Gap |
| anti_pattern | Anti-Pattern Violation |
| code_quality | Code Quality Issue |
| deferral | Deferral Issue |

### Priority Mapping

| Severity | Debt Priority |
|----------|---------------|
| MEDIUM | Medium |
| LOW | Low |

Note: CRITICAL and HIGH severities typically don't reach debt register
(they pass the default MEDIUM threshold).

---

## Step 4: Insert Entries

### Locate Insertion Point

Find the "## Open Debt Items" section:

```
insertion_marker = "## Open Debt Items\n\n"
insertion_index = register_content.find(insertion_marker)
if insertion_index == -1:
    // Section missing - append at end
    log_warning("Open Debt Items section not found, appending")
    insertion_index = len(register_content)
else:
    insertion_index += len(insertion_marker)
```

### Insert New Entries

```
new_entries = generate_all_debt_entries($DEFERRED_GAPS)

updated_content = (
    register_content[:insertion_index] +
    new_entries + "\n" +
    register_content[insertion_index:]
)
```

### Write Updated Register

```
Edit(file_path=register_path,
     old_string=register_content,
     new_string=updated_content)
```

---

## Step 5: Update Analysis Section

### Find Current Statistics

```
// Pattern: **Total Open Items:** X
current_total_match = regex.search(
    r"\*\*Total Open Items:\*\* (\d+)",
    register_content
)
current_total = int(current_total_match.group(1)) if current_total_match else 0
```

### Calculate New Statistics

```
new_total = current_total + len($DEFERRED_GAPS)
new_date = current_date()  // YYYY-MM-DD
```

### Update Statistics

```
Edit(file_path=register_path,
     old_string="**Total Open Items:** {current_total}",
     new_string="**Total Open Items:** {new_total}")

Edit(file_path=register_path,
     old_string="**Last Analysis:** {old_date}",
     new_string="**Last Analysis:** {new_date}")
```

---

## Step 6: Optional Analyzer Invocation

### Check Configuration

```
if config.technical_debt.invoke_analyzer:
    invoke_debt_analyzer()
```

### Analyzer Invocation

```
Task(
    subagent_type="technical-debt-analyzer",
    prompt="""
    Analyze the technical debt register and generate a trend report.

    Focus on:
    1. New items added today ({len($DEFERRED_GAPS)} items)
    2. Overall debt trends
    3. High-priority items requiring attention
    4. Recommendations for debt reduction sprint

    Register path: {register_path}
    """
)
```

---

## Example Generated Entry

```markdown
---
#### [GAP-015]: Style inconsistency in config_loader.py

**Date Added:** 2026-01-15
**Type:** QA Gap (Anti-Pattern Violation)
**Source Report:** devforgeai/qa/reports/STORY-082-gaps.json
**Original Severity:** LOW
**Skipped Reason:** Below min-severity threshold (--min-severity MEDIUM)
**Follow-up:** Run `/review-qa-reports --min-severity LOW` to include
**Priority:** Low
**Status:** Open
**Resolution Target:** Next debt reduction sprint
**Estimated Effort:** 1 points

**Gap Details:**
- File: src/config/config_loader.py
- Layer: N/A
- Current: Non-compliant naming convention
- Target: PEP8 compliance
- Remediation: Rename variables to snake_case

---
```

---

## Output Variables

| Variable | Description |
|----------|-------------|
| `$DEBT_ENTRIES_ADDED` | Count of entries added to register |
| `$REGISTER_UPDATED` | Boolean success flag |
| `$ANALYZER_INVOKED` | Boolean if analyzer was run |

---

## Error Handling

### Register Not Found

```
if not file_exists(register_path):
    log_error("Technical debt register not found at {register_path}")
    log_info("Create with: /document --type=debt-register")
    $REGISTER_UPDATED = False
    return
```

### Section Not Found

```
if "## Open Debt Items" not in register_content:
    log_warning("Adding missing 'Open Debt Items' section")
    // Append section with new entries
```

### Write Failure

```
try:
    Edit(file_path=register_path, ...)
except Error as e:
    log_error("Failed to update debt register: {e}")
    $REGISTER_UPDATED = False
    // Continue - include in final summary
```
