# Schema Backward Compatibility

**Purpose:** Define handling for legacy documents created before schema validation was implemented.

**Status:** LOCKED
**Version:** 1.0
**Last Updated:** 2026-01-23
**Reference:** STORY-301

---

## Overview

Documents created before STORY-301 schema validation may not include all required fields or sections. This document defines how the validator handles such "legacy" documents to ensure backward compatibility while encouraging adoption of the new schema standards.

---

## Legacy Document Detection

### Detection Logic

A document is considered **legacy** if:

```
IF document lacks 'format_version' field in YAML frontmatter:
    document_mode = "LEGACY"
    validation_mode = "WARN"  # Non-blocking
ELSE:
    document_mode = "STANDARD"
    validation_mode = "STRICT"  # Blocking on failure
```

### Missing Format Version Field

The `format_version` field is the primary indicator of schema-aware documents:

| Field Present | Document Mode | Validation Mode |
|---------------|---------------|-----------------|
| `format_version` present | STANDARD | STRICT (blocking) |
| `format_version` missing | LEGACY | WARN (non-blocking) |

---

## Legacy Behavior

### WARN Mode (Non-Blocking)

When a legacy document is detected:

1. **Validation runs** - All schema checks execute
2. **Errors logged as warnings** - Missing fields/sections noted
3. **Workflow proceeds** - Document is not rejected
4. **Degraded context warning** - User informed of potential context loss

### Validation Mode Enum

| Mode | Behavior | Use Case |
|------|----------|----------|
| `STRICT` | Validation failure = HALT | Documents with `format_version` |
| `WARN` | Validation failure = Log warning, continue | Legacy documents |
| `FAIL` | Validation failure = HALT (same as STRICT) | Explicit strict mode |

---

## Optional Sections Handling

### Legacy Documents

For legacy documents (without `format_version`), optional sections **do not block** workflow:

```
IF document_mode == "LEGACY":
    # Required fields still checked
    FOR field in required_fields:
        IF field missing:
            log_warning(f"Required field '{field}' missing")

    # Optional sections NOT blocking
    FOR section in optional_sections:
        IF section missing:
            log_info(f"Optional section '{section}' not present")

    # Only required fields can fail, and in WARN mode they don't block
    validation_status = "WARN"
    workflow_action = "PROCEED"
```

### Standard Documents

For standard documents (with `format_version`), missing required fields or insufficient sections **block** workflow:

```
IF document_mode == "STANDARD":
    # All validations blocking
    FOR field in required_fields:
        IF field missing:
            add_error(f"Required field '{field}' missing")

    FOR section in required_sections:
        IF section missing AND count < min_required:
            add_error(f"Insufficient sections")

    IF errors.length > 0:
        validation_status = "FAILED"
        workflow_action = "HALT"
```

---

## Only Required Fields Block

Even in WARN mode, the validator distinguishes between:

### Truly Required (Always Checked)

These fields are fundamental to document identity and must exist even in legacy mode:

| Document Type | Always Required |
|---------------|-----------------|
| Brainstorm | `id`, `title`, `status` |
| Ideation | `features` (array exists) |
| Epic | `id`, `title`, `status` |

### Conditionally Required (WARN in Legacy Mode)

The validator will report specific fields that are missing and list each missing field in the warning output.

These fields trigger warnings but don't block in legacy mode:

| Document Type | Warned if Missing |
|---------------|------------------|
| Brainstorm | `problem_statement`, `created`, `updated` |
| Brainstorm | Section count < 3 |
| Ideation | `total_points`, `priority_rankings` |
| Epic | `priority`, `total_points`, `target_date` |

---

## Workflow Proceeding Behavior

### When Workflow Proceeds with Warnings

```
Display:
    "⚠️ Schema validation completed with warnings (legacy document)"
    ""
    "Warnings:"
    "  • Missing 'format_version' - document treated as legacy"
    "  • Missing field: problem_statement"
    "  • Found 2/3 required sections"
    ""
    "Note: Workflow proceeding with degraded context preservation."
    "Consider updating document to include format_version for full validation."
```

### Log Entry Format

Warnings are logged to workflow state:

```json
{
  "phase": "00",
  "step": "schema_validation",
  "document_type": "brainstorm",
  "validation_status": "WARN",
  "legacy_mode": true,
  "warnings": [
    "Missing format_version field - legacy mode active",
    "Missing field: problem_statement",
    "Found 2/3 required sections (minimum 3)"
  ],
  "workflow_action": "proceed"
}
```

---

## Migration Guidance

### Upgrading Legacy Documents

To upgrade a legacy document to standard mode:

1. **Add format_version to frontmatter:**
   ```yaml
   ---
   id: BRAINSTORM-001
   title: My Brainstorm
   status: Active
   format_version: "2.0"  # Add this line
   # ... other fields
   ---
   ```

2. **Add missing required fields:**
   - Brainstorm: `problem_statement`, `created`, `updated`
   - Epic: `priority`, `total_points`, `target_date`

3. **Ensure minimum sections (for brainstorms):**
   - At least 3 of 7 standard sections required

### Gradual Adoption Strategy

1. **Phase 1 (Current):** WARN mode for all legacy documents
2. **Phase 2 (Future):** Optional strict mode via configuration
3. **Phase 3 (Future):** Default strict mode with opt-out

---

## Example Legacy Document Handling

### Input: Legacy Brainstorm

```markdown
---
id: BRAINSTORM-001
title: User Authentication
status: Active
# Note: No format_version, created, updated, or problem_statement
---

## Problem Statement
Users cannot log in securely.

## Solution Space
Implement OAuth2 with MFA.
```

### Validation Result

```json
{
  "document_type": "brainstorm",
  "validation_status": "WARN",
  "errors": [
    {
      "field_name": "format_version",
      "expected": "Field must be present",
      "actual": "Field missing",
      "error_message": "Document appears to be legacy (no format_version). Running in WARN mode."
    },
    {
      "field_name": "problem_statement",
      "expected": "Field must be present in frontmatter",
      "actual": "Field missing",
      "error_message": "Required field 'problem_statement' missing from YAML frontmatter"
    },
    {
      "field_name": "sections",
      "expected": "Minimum 3 sections",
      "actual": "Found 2 sections",
      "error_message": "Document has 2/3 required sections"
    }
  ],
  "recommended_action": "Consider adding 'format_version: 2.0' to enable full validation. Missing fields: problem_statement in frontmatter. Add 1 more section from standard list."
}
```

### Workflow Behavior

- **Status:** WARN (not FAILED)
- **Action:** Proceed with warnings
- **User Message:** Degraded context preservation notification

---

## Configuration (Future)

In a future enhancement, legacy mode behavior could be configurable:

```yaml
# devforgeai/config/validation.yaml
schema_validation:
  legacy_mode: "warn"        # "warn" (default), "strict", "skip"
  strict_mode_date: null     # Date after which all docs must pass strict
  migration_warnings: true   # Show migration guidance
```

---

## References

- [skill-output-schemas.yaml](skill-output-schemas.yaml) - Schema definitions
- [schema-validation-workflow.md](schema-validation-workflow.md) - Validation workflow
- [validation-error-schema.md](validation-error-schema.md) - Error report structure
