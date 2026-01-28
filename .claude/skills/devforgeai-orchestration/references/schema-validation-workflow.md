# Schema Validation Workflow

**Purpose:** Validate document schemas at skill handoff boundaries to prevent data format drift and ensure context completeness.

**Status:** LOCKED
**Version:** 1.0
**Last Updated:** 2026-01-23
**Reference:** STORY-301

---

## Overview

Schema validation executes automatically at workflow transition points:
- **Brainstorm → Ideation:** Validate brainstorm document before ideation processing
- **Ideation → Epic:** Validate ideation output before epic creation
- **Epic → Story:** Validate epic document before story decomposition

**Validation fails → HALT workflow with clear error message**
**Validation succeeds → Log success and proceed**

---

## Handoff Boundaries

### 1. Ideation Skill (Brainstorm Input)

**Trigger:** When devforgeai-ideation skill receives brainstorm document

**Validation Steps:**
```
Phase 0: Schema Validation (Pre-Flight)

1. Load brainstorm schema from skill-output-schemas.yaml
2. Extract YAML frontmatter from brainstorm document
3. Validate required_fields: id, title, status, problem_statement, created, updated
4. Validate id_pattern: ^BRAINSTORM-\d{3,}$
5. Count markdown sections matching required_sections headers
6. Verify min_sections (3 of 7) requirement met

IF validation_status == "FAILED":
    HALT workflow
    Display: "Schema validation failed for brainstorm document"
    Display: error_report with field-level errors
    Return

IF validation_status == "PASSED":
    Log: "Schema validation passed for brainstorm"
    Proceed to Phase 1
```

### 2. Create-Epic Command (Ideation Input)

**Trigger:** When /create-epic command receives ideation output

**Validation Steps:**
```
Phase 0: Schema Validation (Pre-Flight)

1. Load ideation schema from skill-output-schemas.yaml
2. Parse ideation output structure
3. Validate required_fields: features, total_points, priority_rankings
4. Validate features array has min_items (3)
5. Validate each feature has: name, description, estimated_points
6. Verify total_points equals sum of feature estimated_points

IF validation_status == "FAILED":
    HALT workflow
    Display: "Schema validation failed for ideation output"
    Display: error_report with field-level errors
    Return

IF validation_status == "PASSED":
    Log: "Schema validation passed for ideation"
    Proceed to epic creation
```

### 3. Story Creation Skill (Epic Input)

**Trigger:** When devforgeai-story-creation skill receives epic document

**Validation Steps:**
```
Phase 0: Schema Validation (Pre-Flight)

1. Load epic schema from skill-output-schemas.yaml
2. Extract YAML frontmatter from epic document
3. Validate required_fields: id, title, status, priority, total_points, target_date
4. Validate id_pattern: ^EPIC-\d{3,}$
5. Verify Features section exists with min 1 feature
6. Verify Dependencies section exists (may be empty)

IF validation_status == "FAILED":
    HALT workflow
    Display: "Schema validation failed for epic document"
    Display: error_report with field-level errors
    Return

IF validation_status == "PASSED":
    Log: "Schema validation passed for epic"
    Proceed to story decomposition
```

---

## Validation Algorithm

### Step-by-Step Validation Process

```
FUNCTION validate_document(document_path, document_type):
    # Step 1: Load appropriate schema
    schema = load_schema(document_type)  # brainstorm, ideation, or epic

    # Step 2: Check for legacy document (backward compatibility)
    IF document lacks format_version field:
        mode = "WARN"  # Legacy mode - non-blocking
    ELSE:
        mode = "STRICT"  # Standard mode - blocking

    # Step 3: Extract document content
    IF document_type in ["brainstorm", "epic"]:
        frontmatter = extract_yaml_frontmatter(document_path)
        body = extract_markdown_body(document_path)
    ELSE:
        content = parse_structured_output(document_path)

    # Step 4: Validate required fields
    errors = []
    FOR field in schema.required_fields:
        IF field not in frontmatter:
            errors.append({
                field_name: field,
                expected: "Field must be present",
                actual: "Field missing",
                error_message: f"Required field '{field}' is missing"
            })

    # Step 5: Validate patterns (if applicable)
    IF schema.id_pattern:
        IF not matches(frontmatter.id, schema.id_pattern):
            errors.append({
                field_name: "id",
                expected: schema.id_pattern,
                actual: frontmatter.id,
                error_message: f"ID '{frontmatter.id}' does not match pattern"
            })

    # Step 6: Validate sections/arrays (if applicable)
    IF schema.required_sections:
        # For brainstorm: count markdown sections
        section_count = count_sections(body, schema.required_sections)
        IF section_count < schema.min_sections:
            errors.append({
                field_name: "sections",
                expected: f"Minimum {schema.min_sections} sections",
                actual: f"Found {section_count} sections",
                error_message: f"Document has {section_count}/{schema.min_sections} required sections"
            })

    IF schema.features.min_items:
        # For ideation/epic: validate feature count
        feature_count = count_items(content.features)
        IF feature_count < schema.features.min_items:
            errors.append({
                field_name: "features",
                expected: f"Minimum {schema.features.min_items} features",
                actual: f"Found {feature_count} features",
                error_message: f"Document has {feature_count}/{schema.features.min_items} required features"
            })

    # Step 7: Determine validation status
    IF len(errors) > 0:
        IF mode == "WARN":
            validation_status = "WARN"
        ELSE:
            validation_status = "FAILED"
    ELSE:
        validation_status = "PASSED"

    # Step 8: Generate error report
    RETURN {
        document_type: document_type,
        validation_status: validation_status,
        errors: errors,
        recommended_action: generate_recommended_action(errors)
    }
```

---

## Auto-Trigger Integration

### Ideation Skill Integration

Add to `devforgeai-ideation/SKILL.md` Phase 0:

```markdown
## Phase 0: Pre-Flight Validation

### Step 0.1: Schema Validation (STORY-301)

Before processing brainstorm document:

1. Read schema: skill-output-schemas.yaml
2. Validate brainstorm document against brainstorm schema
3. IF validation_status == "FAILED": HALT with error report
4. IF validation_status == "PASSED": Log success, proceed

Display on success: "✓ Schema validation passed for brainstorm"
```

### Create-Epic Command Integration

Add to `create-epic.md` Phase 0:

```markdown
## Phase 0: Input Validation

### Step 0.1: Schema Validation (STORY-301)

Before creating epic:

1. Read schema: skill-output-schemas.yaml
2. Validate ideation output against ideation schema
3. IF validation_status == "FAILED": HALT with error report
4. IF validation_status == "PASSED": Log success, proceed

Display on success: "✓ Schema validation passed for ideation"
```

### Story Creation Skill Integration

Add to `devforgeai-story-creation/SKILL.md` Phase 0:

```markdown
## Phase 0: Pre-Flight Validation

### Step 0.1: Schema Validation (STORY-301)

Before decomposing epic:

1. Read schema: skill-output-schemas.yaml
2. Validate epic document against epic schema
3. IF validation_status == "FAILED": HALT with error report
4. IF validation_status == "PASSED": Log success, proceed

Display on success: "✓ Schema validation passed for epic"
```

---

## Success Logging Format

When validation passes, log the following format:

```
Schema validation passed for {document_type}
```

Examples:
- `Schema validation passed for brainstorm`
- `Schema validation passed for ideation`
- `Schema validation passed for epic`

This message is logged to:
1. Console output (visible to user)
2. Workflow state file (`devforgeai/workflows/{STORY_ID}-phase-state.json`)

---

## Failure Handling

When validation fails:

1. **HALT** the workflow immediately
2. **Display** structured error report (see validation-error-schema.md)
3. **Do NOT proceed** to main processing
4. **Provide** recommended_action for remediation

Example failure message:
```
❌ Schema validation FAILED for brainstorm document

Errors:
  • Field 'problem_statement' is missing (expected: Field must be present)
  • Found 2/3 required sections (expected: Minimum 3 sections)

Recommended Action:
  Add missing 'problem_statement' field to YAML frontmatter and ensure at least 3 of the 7 standard sections exist.
```

---

## Performance Requirements

- Validation completes in < 200ms per document (p95)
- Batch validation of 10 documents completes in < 2 seconds
- All errors collected before returning (no short-circuit on first error)

---

## References

- [skill-output-schemas.yaml](skill-output-schemas.yaml) - Schema definitions
- [validation-error-schema.md](validation-error-schema.md) - Error report structure
- [schema-backward-compatibility.md](schema-backward-compatibility.md) - Legacy document handling
