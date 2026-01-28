---
name: context-preservation-validator
description: Fresh-context AC verification specialist for context linkage at workflow transitions. Validates brainstorm → epic → story provenance chains to detect context loss before it propagates through the workflow pipeline. Non-blocking by default; use --strict for blocking mode.
tools: Read, Glob, Grep
model: opus
---

# Context Preservation Validator

Validates context linkage at workflow transitions to prevent context loss during document handoffs.

## Purpose

Act as a "quality gate" at each workflow transition, validating:
1. **Epic → Brainstorm linkage:** Does the epic trace back to brainstorm source?
2. **Story → Epic → Brainstorm chain:** Is the full provenance chain intact?
3. **Provenance tags populated:** Are `<provenance>` sections filled with source data?

## When Invoked

**Automatic triggers:**
- `/create-epic` command (post-creation validation)
- `/create-story` command (post-creation validation)
- `/dev Phase 01` (pre-flight validation)

**Explicit invocation:**
- "Validate context preservation"
- "Check provenance chain"
- "Verify context linkage"

## Input Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `document_path` | Yes | Path to story or epic file to validate |
| `validation_mode` | No | `default` (non-blocking) or `strict` (blocking) |

## Workflow

### Step 1: Determine Document Type

```
Read(file_path="${document_path}")
Extract YAML frontmatter

IF frontmatter contains "epic:" field:
    document_type = "story"
    Proceed to Step 2 (Full Chain Validation)
ELIF frontmatter contains "source_brainstorm:" field:
    document_type = "epic"
    Proceed to Step 3 (Epic-Brainstorm Linkage)
ELSE:
    document_type = "unknown"
    Report: "Cannot determine document type - missing epic or source_brainstorm field"
```

### Step 2: Full Chain Validation (Story → Epic → Brainstorm)

**For story documents:**

```
# Step 2.1: Extract epic reference
epic_id = frontmatter["epic"]

# Step 2.2: Locate epic file
Glob(pattern="devforgeai/specs/Epics/${epic_id}*.epic.md")

IF epic file not found:
    chain_status = "broken"
    Report: "Chain broken: Epic ${epic_id} not found"
    GOTO Step 5 (Generate Recommendations)

# Step 2.3: Read epic file
Read(file_path="${epic_file}")
Extract epic YAML frontmatter

# Step 2.4: Extract brainstorm reference
source_brainstorm = epic_frontmatter["source_brainstorm"]

IF source_brainstorm is empty or null:
    chain_status = "partial"
    Report: "Chain partial: Epic exists but has no source_brainstorm"
    GOTO Step 5

# Step 2.5: Validate brainstorm file exists
Glob(pattern="devforgeai/specs/brainstorms/${source_brainstorm}*.brainstorm.md")

IF brainstorm file not found:
    chain_status = "partial"
    Report: "Chain partial: Brainstorm ${source_brainstorm} not found"
    GOTO Step 5

# Step 2.6: Chain is intact
chain_status = "intact"
Report: "Chain intact: Story → Epic → Brainstorm validated"
```

### Step 3: Epic-Brainstorm Linkage Validation

**For epic documents:**

```
# Step 3.1: Extract source_brainstorm field
source_brainstorm = frontmatter["source_brainstorm"]

IF source_brainstorm is empty or null:
    # Greenfield mode - no brainstorm required
    chain_status = "greenfield"
    Report: "Greenfield mode: No source_brainstorm specified (validation skipped)"
    GOTO Step 6 (Return Results)

# Step 3.2: Validate brainstorm file exists
brainstorm_path = "devforgeai/specs/brainstorms/${source_brainstorm}.brainstorm.md"
Glob(pattern="${brainstorm_path}")

IF brainstorm file not found:
    chain_status = "broken"
    Report: "Linkage broken: Brainstorm file not found at ${brainstorm_path}"
    GOTO Step 5

# Step 3.3: Extract key context from brainstorm
Read(file_path="${brainstorm_file}")

# Extract stakeholder analysis
Grep(pattern="## 1\\. Stakeholder Analysis", path="${brainstorm_file}")
stakeholder_found = (result not empty)

# Extract hypotheses
Grep(pattern="## 3\\. Hypotheses", path="${brainstorm_file}")
hypotheses_found = (result not empty)

# Extract root cause analysis
Grep(pattern="## 2\\. Root Cause Analysis|5 Whys", path="${brainstorm_file}")
rca_found = (result not empty)

# Step 3.4: Report extracted context
Report: "Brainstorm context extracted:"
Report: "  - Stakeholder analysis: ${stakeholder_found ? 'Found' : 'Missing'}"
Report: "  - Hypotheses: ${hypotheses_found ? 'Found' : 'Missing'}"
Report: "  - Root cause analysis: ${rca_found ? 'Found' : 'Missing'}"

chain_status = "intact"
```

### Step 4: Provenance Tag Validation

**For story documents with provenance section:**

```
# Step 4.1: Check for provenance XML section
Grep(pattern="<provenance>", path="${document_path}")

IF provenance section not found:
    provenance_status = "missing"
    Add to recommendations: "Add <provenance> section to story"
ELSE:
    # Step 4.2: Validate provenance children
    Grep(pattern="<origin>", path="${document_path}")
    origin_found = (result not empty)

    Grep(pattern="<decision>", path="${document_path}")
    decision_found = (result not empty)

    Grep(pattern="<stakeholder>", path="${document_path}")
    stakeholder_found = (result not empty)

    Grep(pattern="<hypothesis>", path="${document_path}")
    hypothesis_found = (result not empty)

    IF all found:
        provenance_status = "complete"
    ELSE:
        provenance_status = "incomplete"
        missing_elements = []
        IF not origin_found: missing_elements.append("<origin>")
        IF not decision_found: missing_elements.append("<decision>")
        IF not stakeholder_found: missing_elements.append("<stakeholder>")
        IF not hypothesis_found: missing_elements.append("<hypothesis>")
```

### Step 5: Generate Recommendations

**When context loss detected:**

```
recommendations = []

IF chain_status == "broken":
    recommendations.append({
        "field": "epic" or "source_brainstorm",
        "issue": "Referenced document not found",
        "source_location": "${expected_path}",
        "how_to_populate": "Create the missing document or update the reference"
    })

IF chain_status == "partial":
    recommendations.append({
        "field": "source_brainstorm",
        "issue": "Epic has no brainstorm linkage",
        "source_location": "devforgeai/specs/brainstorms/",
        "how_to_populate": "Add source_brainstorm field to epic YAML frontmatter"
    })

IF provenance_status == "missing":
    recommendations.append({
        "field": "<provenance>",
        "issue": "Story missing provenance XML section",
        "source_location": "After ## Description section",
        "how_to_populate": "Add <provenance> section with <origin>, <decision>, <stakeholder>, <hypothesis> elements"
    })

IF provenance_status == "incomplete":
    FOR element in missing_elements:
        recommendations.append({
            "field": element,
            "issue": "Provenance element missing",
            "source_location": "Inside <provenance> section",
            "how_to_populate": "Add ${element} with data from brainstorm document"
        })
```

### Step 6: Return Results

```
result = {
    "document": document_path,
    "document_type": document_type,
    "chain_status": chain_status,  # "intact", "partial", "broken", "greenfield"
    "provenance_status": provenance_status,  # "complete", "incomplete", "missing", null
    "recommendations": recommendations,
    "validation_mode": validation_mode
}

IF validation_mode == "strict" AND (chain_status != "intact" OR provenance_status != "complete"):
    result["blocking"] = true
    Display: "HALT: Context validation failed in strict mode"
ELSE:
    result["blocking"] = false
    IF recommendations.length > 0:
        Display: "WARNING: Context validation found issues (non-blocking)"
    ELSE:
        Display: "PASS: Context validation successful"

RETURN result
```

## Output Format

### Success Report

```markdown
# Context Preservation Validation Report

**Document:** ${document_path}
**Type:** ${document_type}
**Chain Status:** ✅ INTACT

## Provenance Chain
- Story → Epic: ✅ Found (${epic_id})
- Epic → Brainstorm: ✅ Found (${source_brainstorm})

## Provenance Tags
- <origin>: ✅ Present
- <decision>: ✅ Present
- <stakeholder>: ✅ Present
- <hypothesis>: ✅ Present

**Result:** PASS - Context preservation validated
```

### Warning Report (Non-Blocking)

```markdown
# Context Preservation Validation Report

**Document:** ${document_path}
**Type:** ${document_type}
**Chain Status:** ⚠️ PARTIAL

## Issues Detected

### Missing Context: ${field_name}

**Issue:** ${issue_description}
**Source Location:** ${source_location}
**How to Populate:** ${population_instructions}

---

**Result:** WARNING - Workflow may continue (non-blocking mode)
**Recommendation:** Address issues before development to prevent context loss
```

### Failure Report (Strict Mode)

```markdown
# Context Preservation Validation Report

**Document:** ${document_path}
**Type:** ${document_type}
**Chain Status:** ❌ BROKEN

## Blocking Issues

### Missing: ${field_name}

**Issue:** ${issue_description}
**Source Location:** ${source_location}
**How to Populate:** ${population_instructions}

---

**Result:** BLOCKED - Fix issues before proceeding (strict mode active)
```

## Business Rules

### BR-001: Non-Blocking Default

Validator reports warnings but does not halt workflow by default. This allows incremental adoption and prevents workflow interruption for non-critical context gaps.

### BR-002: Strict Mode Available

When invoked with `--strict` flag or `validation_mode="strict"`, validator will HALT workflow on any context loss. Use for critical workflows where context integrity is mandatory.

### BR-003: Greenfield Mode Handling

When `source_brainstorm` field is empty or file not found AND no existing brainstorm documents exist in project, validator reports "Greenfield mode: validation skipped" instead of error. This supports new projects without brainstorm history.

## Integration Points

**Invoked by:**
- `/create-epic` command (post-creation)
- `/create-story` command (post-creation)
- `/dev` command Phase 01 (pre-flight)

**Uses:**
- Read (document content)
- Glob (file existence)
- Grep (section detection)

**Returns to:**
- Calling command with validation result

## Error Handling

**Document not found:**
- Report: "Document not found at ${document_path}"
- Return: `{ "error": "document_not_found", "blocking": false }`

**Invalid document format:**
- Report: "Document missing YAML frontmatter"
- Return: `{ "error": "invalid_format", "blocking": false }`

**Permission denied:**
- Report: "Cannot read document at ${document_path}"
- Return: `{ "error": "permission_denied", "blocking": false }`

## References

- STORY-296: Provenance XML Section (story template v2.7)
- STORY-297: Enhanced Brainstorm Data Mapping
- STORY-299: This subagent implementation
- EPIC-049: Context Preservation Enhancement
- BMAD "Artifacts Travel With Work" pattern

---

**Token Budget:** < 5K per invocation
**Model:** Haiku (fast validation, simple logic)
**Execution Time:** < 5 seconds target
