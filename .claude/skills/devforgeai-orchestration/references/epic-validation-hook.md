# Epic Validation Hook - STORY-089 AC#1

## Purpose

Validates epic structure after `/create-epic` generates an epic file, ensuring all epics meet quality standards before workflow continues.

## Integration Point

**Phase 4A.6** in devforgeai-orchestration skill (after epic file creation, before completion summary).

## Validation Rules

### Feature Validation
- Epic MUST have at least one feature defined
- Each feature MUST have unique identifier (`Feature N:` format)
- Feature descriptions MUST be non-empty (minimum 10 characters)

### Frontmatter Validation
- Required fields: `title`, `status`, `priority`
- Either `epic_id` or `id` field must be present
- Valid YAML syntax (no tabs, proper indentation)

## Invocation

```bash
devforgeai/traceability/epic-validator.sh --validate-epic <epic-file>
```

## Exit Codes

| Code | Meaning | Workflow Action |
|------|---------|-----------------|
| 0 | Validation passed | Continue to completion |
| 1 | Validation failed | Display errors, suggest fixes |
| 2 | File not found | Error state |
| 3 | Internal error | Error state |

## Error Handling

Non-blocking by default. Validation failures display:
1. Specific error message
2. Line number (if applicable)
3. Fix suggestion with example

## Example Output

```
Validation PASSED: devforgeai/specs/Epics/EPIC-015.epic.md
```

```
Validation FAILED for: devforgeai/specs/Epics/EPIC-TEST.epic.md
  - Feature 1 description too short (5 chars, minimum 10 characters required)
  - Missing required field: priority

Fix suggestions:
  - Feature descriptions should be at least 10 characters
  - Ensure frontmatter has: title, status, priority
```

## Configuration

Thresholds loaded from: `devforgeai/traceability/thresholds.json`

```json
{
  "validation": {
    "min_feature_description_length": 10,
    "required_frontmatter_fields": ["title", "status", "priority"]
  }
}
```

## Performance Target

Single epic validation: <50ms (p95)
