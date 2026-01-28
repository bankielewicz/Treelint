# Field Mapping Guide

**Version:** 1.0
**Date:** 2025-11-10
**Status:** Active

This guide explains how to define field mappings in feedback templates to connect conversation responses with output sections.

---

## Overview

Field mappings define the relationship between:
- **Question IDs** - Identifiers for questions asked during retrospective conversation
- **Section Headers** - Markdown sections in the rendered output
- **Responses** - User answers collected during conversation

---

## Field Mapping Structure

### Basic Syntax

```yaml
field-mappings:
  field-name:
    question-id: "unique_question_identifier"
    section: "## Section Header"
    description: "Optional explanation of field purpose"
```

### Example

```yaml
field-mappings:
  what-went-well:
    question-id: "cmd_passed_01"
    section: "## What Went Well"
    description: "Positive aspects of the command execution"
```

**Result:** When conversation contains response for `cmd_passed_01`, it appears under `## What Went Well` in rendered output.

---

## Field Name

### Purpose

Internal identifier for the field mapping entry. Used for:
- Template maintenance and documentation
- Debugging field mapping issues
- Code readability

### Format

- **Pattern:** `lowercase-with-hyphens`
- **Length:** 3-30 characters
- **Characters:** Letters, numbers, hyphens only

### Examples

**✅ Valid:**
- `what-went-well`
- `root-cause-analysis`
- `suggestions-for-improvement`
- `efficiency-notes`

**❌ Invalid:**
- `WhatWentWell` (PascalCase not allowed)
- `what_went_well` (underscores not allowed)
- `what went well` (spaces not allowed)
- `1st-item` (cannot start with number)

### Best Practices

1. **Be descriptive** - Field name should indicate content type
2. **Use full words** - Avoid abbreviations unless very common
3. **Match section intent** - Align with section header meaning
4. **Consistent naming** - Similar fields across templates use same names

---

## Question ID

### Purpose

Unique identifier that links conversation responses to template sections.

### Format

**Pattern:** `{type}_{status}_{number}`

**Components:**
- `{type}`: Operation type abbreviation (3-8 letters)
- `{status}`: Status suffix (passed, failed, partial, generic)
- `{number}`: Sequential number (01, 02, 03... up to 99)

### Operation Type Abbreviations

| Operation Type | Abbreviation | Example Question IDs |
|----------------|--------------|----------------------|
| command | `cmd` | `cmd_passed_01`, `cmd_failed_02` |
| skill | `skill` | `skill_passed_01`, `skill_failed_02` |
| subagent | `subagent` | `subagent_passed_01`, `subagent_failed_02` |
| workflow | `workflow` | `workflow_passed_01`, `workflow_failed_02` |
| generic | `generic` | `generic_01`, `generic_02` |

### Examples

**Command Success:**
- `cmd_passed_01` - What went well?
- `cmd_passed_02` - What could improve?
- `cmd_passed_03` - Suggestions for next time?
- `cmd_passed_04` - Efficiency notes?

**Skill Failure:**
- `skill_failed_01` - What happened?
- `skill_failed_02` - Root cause analysis?
- `skill_failed_03` - Blockers encountered?
- `skill_failed_04` - Recovery steps taken?

**Generic:**
- `generic_01` - What went well?
- `generic_02` - What could improve?
- `generic_03` - Additional notes?

### Question ID Constraints

**Uniqueness:**
- Each question ID must be unique within a template
- Question IDs can be reused across different templates
- Reusing question IDs across templates enables cross-template analysis

**Stability:**
- Once assigned, question IDs should not change
- Changing question IDs breaks historical data analysis
- If question changes significantly, assign new ID and deprecate old one

**Numbering:**
- Start numbering at 01 (not 1 or 00)
- Use zero-padding for consistent string sorting
- Leave gaps for future questions (e.g., 01, 03, 05 allows inserting 02, 04 later)

---

## Section Header

### Purpose

Defines the markdown section where the response appears in rendered output.

### Format

**Pattern:** `## {Section Title}`

**Requirements:**
- Must start with `##` (level 2 header)
- Space after `##`
- Title Case or Sentence case (consistent within template)
- 3-50 characters (excluding `##`)

### Examples

**✅ Valid:**
- `## What Went Well`
- `## Root Cause Analysis`
- `## Suggestions for Improvement`
- `## Blockers Encountered`
- `## Recovery Steps Taken`
- `## How to Prevent Next Time`

**❌ Invalid:**
- `# What Went Well` (level 1 header - reserved for document title)
- `### What Went Well` (level 3 header - too deep)
- `What Went Well` (missing `##`)
- `##What Went Well` (missing space after `##`)
- `## WHAT WENT WELL` (all caps not recommended)

### Section Header Guidelines

1. **Descriptive** - Clear indication of content type
2. **Concise** - Short enough to scan quickly
3. **Consistent** - Use same phrasing across templates where applicable
4. **Actionable** - Frame as questions or outcomes when possible

### Common Section Headers

**Success Templates:**
- `## What Went Well` - Positive outcomes
- `## What Could Improve` - Areas for optimization
- `## Efficiency Notes` - Performance observations
- `## Suggestions for Next Time` - Recommendations

**Failure Templates:**
- `## What Happened` - Error description
- `## Root Cause Analysis` - Investigation findings
- `## Blockers Encountered` - Obstacles faced
- `## Recovery Steps Taken` - Actions attempted
- `## How to Prevent Next Time` - Prevention strategies

**Analysis Templates:**
- `## Key Findings` - Major discoveries
- `## Trends Observed` - Patterns identified
- `## Recommendations` - Suggested actions
- `## Next Steps` - Follow-up tasks

---

## Description (Optional)

### Purpose

Human-readable explanation of what the field collects. Used for:
- Template documentation
- Developer reference
- Question bank integration

### Format

- **Type:** String (plain text)
- **Length:** 10-200 characters
- **Style:** Sentence case, descriptive phrase

### Examples

**Good Descriptions:**
- `"Positive aspects of the command execution"`
- `"Areas where the workflow could be more efficient"`
- `"Specific recommendations for next execution"`
- `"Root cause investigation of the failure"`
- `"Obstacles that prevented completion"`

**Poor Descriptions:**
- `"Good stuff"` (too vague)
- `"What went well during the command that the user executed in the DevForgeAI framework"` (too verbose)
- `"Field 1"` (not descriptive)

### Best Practices

1. **Be specific** - Indicate what type of information to collect
2. **Keep it concise** - One sentence is sufficient
3. **Focus on purpose** - Explain why this field exists
4. **Avoid redundancy** - Don't repeat the section header exactly

---

## Complete Field Mapping Example

```yaml
field-mappings:
  what-went-well:
    question-id: "cmd_passed_01"
    section: "## What Went Well"
    description: "Positive aspects of the command execution"

  what-could-improve:
    question-id: "cmd_passed_02"
    section: "## What Could Improve"
    description: "Areas where the workflow could be more efficient"

  efficiency-notes:
    question-id: "cmd_passed_03"
    section: "## Efficiency Notes"
    description: "Performance observations and optimization opportunities"

  suggestions:
    question-id: "cmd_passed_04"
    section: "## Suggestions for Next Time"
    description: "Specific recommendations for next execution"
```

**Rendered Output:**

```markdown
---
operation: /dev STORY-042
type: command
status: passed
timestamp: 2025-11-10T10:30:00+00:00
---

# Retrospective: /dev STORY-042

## What Went Well
TDD workflow was clear and well-structured.

## What Could Improve
Initial git setup was confusing.

## Efficiency Notes
Test generation took 2 minutes, could be optimized.

## Suggestions for Next Time
Pre-create template directory structure.

## Context
(auto-populated)

## User Sentiment
(auto-populated)

## Actionable Insights
(auto-extracted)
```

---

## Auto-Populated Sections

Some sections are automatically generated by the template engine and don't require field mappings.

### Context Section

**Header:** `## Context`
**Content:** Auto-generated from operation metadata
**Source:** TodoWrite status, errors encountered, performance metrics

**Example:**
```markdown
## Context
- **TodoWrite Status:** 5 of 5 tasks completed
- **Errors Encountered:** No
- **Performance Metrics:** Execution time: 12m 34s, Token usage: 87,500
```

**Do NOT create field mapping** - Template engine adds this automatically.

### User Sentiment Section

**Header:** `## User Sentiment`
**Content:** Calculated from satisfaction rating question
**Source:** Response to question with `sentiment_rating` prefix

**Example:**
```markdown
## User Sentiment
Satisfied (4/5)
```

**How it works:**
- If conversation contains response with question_id starting with `sentiment_rating`
- Template engine parses numeric rating (1-5 scale)
- Converts to text: 1=Frustrated, 2=Dissatisfied, 3=Neutral, 4=Satisfied, 5=Delighted

**Do NOT create field mapping** - Template engine adds this automatically.

### Actionable Insights Section

**Header:** `## Actionable Insights`
**Content:** Extracted from suggestions using keyword patterns
**Source:** Responses containing action words (should, could, needs, must, recommend)

**Example:**
```markdown
## Actionable Insights
1. Pre-create template directory structure [Priority: Medium]
2. Optimize test generation performance [Priority: Low]
```

**How it works:**
- Template engine scans all responses for action keywords
- Extracts sentences containing those keywords
- Formats as numbered list with priority heuristics

**Do NOT create field mapping** - Template engine adds this automatically.

---

## Additional Feedback Section

**Header:** `## Additional Feedback`
**Content:** Responses not mapped to any field
**Source:** Conversation responses with question IDs not in field-mappings

**Example:**

Template defines mappings for:
- `cmd_passed_01`
- `cmd_passed_02`
- `cmd_passed_03`

Conversation contains responses for:
- `cmd_passed_01` ✅ Mapped
- `cmd_passed_02` ✅ Mapped
- `cmd_passed_03` ✅ Mapped
- `optional_feedback` ❌ Not mapped

**Rendered Output:**
```markdown
## Additional Feedback
- optional_feedback: User provided extra comments not covered by template
```

**Do NOT create field mapping** - Template engine adds this automatically for unmapped responses.

---

## Response Handling

### Missing Response

**Scenario:** Question ID in field mapping, but no response in conversation

**Example:**
```yaml
field-mappings:
  suggestions:
    question-id: "cmd_passed_04"
    section: "## Suggestions for Next Time"
```

Conversation has no `cmd_passed_04` response.

**Rendered Output:**
```markdown
## Suggestions for Next Time
No response provided
```

**Behavior:** Section appears with default text indicating no response.

### Empty Response

**Scenario:** Response exists but is empty string `""`

**Rendered Output:**
```markdown
## Suggestions for Next Time
No response provided
```

**Behavior:** Treated same as missing response.

### None Response

**Scenario:** Response is Python `None` value

**Rendered Output:**
```markdown
## Suggestions for Next Time
No response provided
```

**Behavior:** Treated same as missing response.

### Multiline Response

**Scenario:** Response contains line breaks

**Example:**
```
cmd_passed_01: "Point 1\nPoint 2\nPoint 3"
```

**Rendered Output:**
```markdown
## What Went Well
Point 1
Point 2
Point 3
```

**Behavior:** Line breaks preserved in markdown output.

### Special Characters

**Scenario:** Response contains markdown special characters (*, _, #, etc.)

**Example:**
```
cmd_passed_01: "Used *emphasis* and _underline_ formatting"
```

**Rendered Output:**
```markdown
## What Went Well
Used *emphasis* and _underline_ formatting
```

**Behavior:** Characters preserved as-is (not escaped in markdown body).

---

## Best Practices

### Ordering Field Mappings

1. **Logical flow** - Order mappings by natural question sequence
2. **Core first** - Most important questions at the top
3. **Optional last** - Less critical questions at the bottom
4. **Consistent structure** - Use same order across similar templates

**Example:**
```yaml
field-mappings:
  # Core questions
  what-went-well:
    ...
  what-went-poorly:
    ...

  # Supporting questions
  efficiency-notes:
    ...

  # Optional questions
  additional-comments:
    ...
```

### Reusing Question IDs

**When to reuse:**
- Same question asked across success/failure templates
- Common questions across operation types (command, skill, subagent)
- Generic questions applicable to all contexts

**Example:**
```yaml
# command-passed.yaml
field-mappings:
  suggestions:
    question-id: "cmd_passed_03"
    section: "## Suggestions for Next Time"

# command-failed.yaml
field-mappings:
  prevention:
    question-id: "cmd_failed_05"
    section: "## How to Prevent Next Time"
```

Note: Different question IDs because intent differs (optimization vs prevention).

**When NOT to reuse:**
- Questions with different semantic meaning
- Status-specific questions (success vs failure)
- Context-dependent questions

### Documenting Field Mappings

1. **Use descriptions** - Always provide description field
2. **Comment complex mappings** - Add YAML comments for non-obvious mappings
3. **Document question bank** - Maintain separate question bank documentation
4. **Version field mappings** - Track changes to mappings in template version

---

## Testing Field Mappings

### Manual Testing

1. **Create test conversation responses:**
```python
test_responses = {
    "cmd_passed_01": "Tests passed quickly",
    "cmd_passed_02": "Setup took too long",
    "cmd_passed_03": "Pre-create directories",
}
```

2. **Load template and map fields:**
```python
from devforgeai_cli.feedback.template_engine import select_template, map_fields

template = select_template("command", "passed", user_config, template_dir)
mapped_sections = map_fields(template, test_responses)
```

3. **Verify section mapping:**
```python
assert "## What Went Well" in mapped_sections
assert "Tests passed quickly" in mapped_sections["## What Went Well"]
```

### Automated Testing

Add test cases for each template:
```python
def test_command_passed_field_mappings():
    template = load_template("command-passed.yaml")
    responses = {
        "cmd_passed_01": "Success message",
        "cmd_passed_02": "Improvement note",
    }

    sections = map_fields(template, responses)

    assert len(sections) >= 2
    assert "## What Went Well" in sections
    assert sections["## What Went Well"] == "Success message"
```

---

## Common Issues

### Issue: Section Not Appearing

**Symptom:** Field mapping defined but section missing in rendered output

**Causes:**
1. Question ID mismatch (typo in question_id)
2. Response not in conversation
3. Section header format incorrect (missing `##`)

**Fix:**
- Verify question_id spelling
- Check conversation contains response for that question ID
- Ensure section header starts with `##`

### Issue: Response in Wrong Section

**Symptom:** Response appears under unexpected section header

**Causes:**
1. Multiple field mappings use same question ID
2. Section header mismatch between mapping and template sections list

**Fix:**
- Ensure each question ID appears once in field-mappings
- Verify section header matches exactly (case-sensitive)

### Issue: Default Text Appearing

**Symptom:** "No response provided" when response exists

**Causes:**
1. Question ID in conversation doesn't match field mapping
2. Response value is `None`, `""`, or whitespace-only

**Fix:**
- Match question IDs exactly (case-sensitive)
- Check response has actual content

---

## Migration Guide

### Adding New Field Mapping

1. **Choose question ID** - Follow naming pattern
2. **Define section header** - Use level 2 header (`##`)
3. **Add to field-mappings section:**
```yaml
field-mappings:
  # Existing mappings...

  new-field:
    question-id: "cmd_passed_05"
    section: "## New Section"
    description: "Purpose of new field"
```
4. **Update template sections list** - Add `## New Section` to list
5. **Test with sample data** - Verify rendering

### Removing Field Mapping

**Don't remove** - For backward compatibility, mark as deprecated instead:

```yaml
field-mappings:
  deprecated-field:
    question-id: "cmd_passed_old"
    section: "## Deprecated Section"
    description: "DEPRECATED: Use new-field instead"
```

This preserves historical data analysis while guiding users to new field.

### Renaming Section Header

1. **Update field mapping:**
```yaml
field-mappings:
  suggestions:
    question-id: "cmd_passed_03"
    section: "## Improvement Recommendations"  # Changed from "Suggestions for Next Time"
```

2. **Update template sections list** - Reflect new header
3. **Increment template version** - Major version bump if breaking change
4. **Test historical data** - Verify old rendered outputs still parse correctly

---

## References

- **Template Format Specification:** `template-format-specification.md`
- **Template Examples:** `template-examples.md`
- **User Customization Guide:** `user-customization-guide.md`
- **Template Engine Implementation:** `.claude/scripts/devforgeai_cli/feedback/template_engine.py`

---

**Version History:**
- **1.0 (2025-11-10):** Initial guide for STORY-010
