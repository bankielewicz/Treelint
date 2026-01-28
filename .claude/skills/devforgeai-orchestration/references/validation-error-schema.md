# Validation Error Schema

**Purpose:** Define the structure of error reports returned by schema validation.

**Status:** LOCKED
**Version:** 1.0
**Last Updated:** 2026-01-23
**Reference:** STORY-301

---

## Error Report Structure

When schema validation completes, it returns a structured error report with the following fields:

### Top-Level Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `document_type` | string | **Yes** | Type of document validated: `brainstorm`, `ideation`, or `epic` |
| `validation_status` | enum | **Yes** | Result status: `PASSED`, `FAILED`, or `WARN` |
| `errors` | array | **Yes** | Array of error objects (empty if validation passed) |
| `recommended_action` | string | **Yes** | Human-readable remediation guidance |

### Validation Status Enum

| Value | Description | Workflow Behavior |
|-------|-------------|-------------------|
| `PASSED` | All validations passed | Proceed to next phase |
| `FAILED` | One or more required validations failed | **HALT workflow** |
| `WARN` | Legacy document or optional warnings | Proceed with logged warnings |

### Error Object Structure

Each error in the `errors` array contains:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `field_name` | string | **Yes** | Name of the field that failed validation |
| `expected` | string | **Yes** | Expected value: the pattern, value, or condition that was expected |
| `actual` | string | **Yes** | Actual value: what was actually found in the document |
| `error_message` | string | **Yes** | Human-readable error description |

---

## Example Error Reports

### Example 1: Passed Validation

```json
{
  "document_type": "brainstorm",
  "validation_status": "PASSED",
  "errors": [],
  "recommended_action": "No action required. Document is valid."
}
```

### Example 2: Failed Validation (Missing Fields)

```json
{
  "document_type": "brainstorm",
  "validation_status": "FAILED",
  "errors": [
    {
      "field_name": "problem_statement",
      "expected": "Field must be present",
      "actual": "Field missing",
      "error_message": "Required field 'problem_statement' is missing from YAML frontmatter"
    },
    {
      "field_name": "updated",
      "expected": "Field must be present",
      "actual": "Field missing",
      "error_message": "Required field 'updated' is missing from YAML frontmatter"
    }
  ],
  "recommended_action": "Add missing required fields to YAML frontmatter: problem_statement, updated"
}
```

### Example 3: Failed Validation (Pattern Mismatch)

```json
{
  "document_type": "epic",
  "validation_status": "FAILED",
  "errors": [
    {
      "field_name": "id",
      "expected": "^EPIC-\\d{3,}$",
      "actual": "EPIC-42",
      "error_message": "ID 'EPIC-42' does not match required pattern. Must be EPIC-NNN with at least 3 digits."
    }
  ],
  "recommended_action": "Update the id field to use at least 3 digits (e.g., EPIC-042 instead of EPIC-42)"
}
```

### Example 4: Failed Validation (Insufficient Sections)

```json
{
  "document_type": "brainstorm",
  "validation_status": "FAILED",
  "errors": [
    {
      "field_name": "sections",
      "expected": "Minimum 3 sections",
      "actual": "Found 2 sections",
      "error_message": "Document has 2/3 required sections. Missing sections needed."
    }
  ],
  "recommended_action": "Add at least 1 more section from: Stakeholder Analysis, Problem Statement, Root Cause Analysis, Hypothesis Register, Impact-Effort Matrix, Solution Space, Next Steps"
}
```

### Example 5: Warning (Legacy Document)

```json
{
  "document_type": "epic",
  "validation_status": "WARN",
  "errors": [
    {
      "field_name": "format_version",
      "expected": "Field must be present",
      "actual": "Field missing",
      "error_message": "Document appears to be legacy (no format_version). Running in WARN mode."
    }
  ],
  "recommended_action": "Consider adding 'format_version: 2.0' to YAML frontmatter for full validation. Workflow will proceed with warnings."
}
```

### Example 6: Failed Validation (Feature Count)

```json
{
  "document_type": "ideation",
  "validation_status": "FAILED",
  "errors": [
    {
      "field_name": "features",
      "expected": "Minimum 3 features",
      "actual": "Found 2 features",
      "error_message": "Features array has 2/3 required items. Add at least 1 more feature."
    }
  ],
  "recommended_action": "Add at least 1 more feature to the features array. Each feature must have name, description, and estimated_points."
}
```

---

## Recommended Action Generation

The `recommended_action` field provides specific, actionable guidance based on the errors found:

### Rules for Generating Recommended Actions

1. **Missing Fields:** List all missing fields and their location (frontmatter/body)
2. **Pattern Mismatches:** Show the expected pattern with an example
3. **Count Violations:** Show current count vs required minimum with options
4. **Multiple Errors:** Combine into prioritized action list
5. **Legacy Documents:** Suggest adding format_version but note workflow proceeds

### Template Patterns

| Error Type | Recommended Action Template |
|------------|----------------------------|
| Missing required field | `Add missing required field '{field_name}' to {location}` |
| Pattern mismatch | `Update {field_name} to match pattern {pattern} (e.g., {example})` |
| Insufficient sections | `Add at least {needed} more section(s) from: {options}` |
| Insufficient features | `Add at least {needed} more feature(s) with name, description, and estimated_points` |
| Legacy document | `Consider adding 'format_version: {version}' to enable full validation` |

---

## Error Handling Behavior

### All Errors Collected

The validator **does not short-circuit** on the first error. It collects all validation errors before returning, providing a complete picture of issues to fix.

### YAML Parse Errors

If the document cannot be parsed (malformed YAML):

```json
{
  "document_type": "unknown",
  "validation_status": "FAILED",
  "errors": [
    {
      "field_name": "yaml_frontmatter",
      "expected": "Valid YAML between --- delimiters",
      "actual": "YAML parse error at line 5",
      "error_message": "YAML parse error: unexpected character at line 5, column 10"
    }
  ],
  "recommended_action": "Fix YAML syntax error at line 5. Ensure proper indentation and valid YAML structure."
}
```

### File Not Found

If the document file does not exist:

```json
{
  "document_type": "unknown",
  "validation_status": "FAILED",
  "errors": [
    {
      "field_name": "document_path",
      "expected": "File exists at path",
      "actual": "File not found",
      "error_message": "Document not found at specified path"
    }
  ],
  "recommended_action": "Verify the document path is correct and the file exists."
}
```

---

## Display Format

When displaying error reports to users, use this format:

### For FAILED Status

```
❌ Schema validation FAILED for {document_type} document

Errors:
  • {error_message_1} (expected: {expected_1}, found: {actual_1})
  • {error_message_2} (expected: {expected_2}, found: {actual_2})

Recommended Action:
  {recommended_action}
```

### For WARN Status

```
⚠️ Schema validation WARN for {document_type} document

Warnings:
  • {error_message_1}

Note: Workflow will proceed with degraded context preservation.
```

### For PASSED Status

```
✓ Schema validation passed for {document_type}
```

---

## References

- [skill-output-schemas.yaml](skill-output-schemas.yaml) - Schema definitions
- [schema-validation-workflow.md](schema-validation-workflow.md) - Validation workflow
- [schema-backward-compatibility.md](schema-backward-compatibility.md) - Legacy handling
