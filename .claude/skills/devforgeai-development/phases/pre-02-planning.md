# Pre-Phase 02: API Specification Planning

**Trigger:** `pre-phase-planning.yaml` has `phases.02.enabled: true`

**Purpose:** Create API specification before test-automator generates tests in Phase 02.

**Status:** Conditional (disabled by default)
**Story:** STORY-FEEDBACK-004
**Implementation Date:** 2026-01-29

---

## Overview

This pre-phase extracts and documents the API specification (types, methods, error contracts) from the story's technical specification. The resulting document guides test-automator in Phase 02 to generate tests that accurately reflect the intended API.

**Benefits:**
- Tests align with intended API (not inferred)
- Clear contract documentation before implementation
- Reduces test-implementation misalignment
- Enables course correction before coding

---

## Prerequisites

**Before executing this pre-phase:**
1. Story file exists with Technical Specification section
2. Pre-phase planning enabled in config (`phases.02.enabled: true`)
3. Phase 01 (Preflight) completed successfully

---

## Workflow

### Step 1: Read Story Technical Specification

```
Read(file_path="devforgeai/specs/Stories/${STORY_ID}.story.md")
```

Extract from Technical Specification section:
- API endpoints (if applicable)
- Data models (entities, fields, types)
- Method signatures (functions, parameters, return types)
- State machines / enums
- Error conditions and exception types
- Business rules and validation constraints

### Step 2: Read Context Files

```
Read(file_path="devforgeai/specs/context/coding-standards.md")
Read(file_path="devforgeai/specs/context/architecture-constraints.md")
```

Extract:
- Naming conventions for APIs
- Layer separation rules
- Error handling patterns
- Required design patterns

### Step 3: Generate API Specification

Create specification document:

```json
{
  "story_id": "${STORY_ID}",
  "timestamp": "${ISO_8601_TIMESTAMP}",
  "api_contract": {
    "types": [
      {
        "name": "TypeName",
        "kind": "enum|class|interface|struct",
        "values": ["value1", "value2"],
        "fields": [
          {"name": "fieldName", "type": "string", "required": true}
        ]
      }
    ],
    "methods": [
      {
        "name": "methodName",
        "signature": "pub fn method_name(&self, param: Type) -> Result<Output, Error>",
        "description": "What this method does",
        "parameters": [
          {"name": "param", "type": "Type", "description": "Parameter purpose"}
        ],
        "returns": {"type": "Result<Output, Error>", "description": "Success/error cases"},
        "throws": ["ErrorType1", "ErrorType2"]
      }
    ],
    "errors": [
      {
        "name": "ErrorTypeName",
        "variants": ["Variant1", "Variant2"],
        "description": "When this error occurs"
      }
    ]
  },
  "test_patterns": ["unit", "integration", "error_handling", "edge_cases"],
  "notes": "Additional context for test generation"
}
```

### Step 4: Write to Disk

```
Write(
  file_path="devforgeai/feedback/ai-analysis/${STORY_ID}/pre-02-api-spec.json",
  content=${api_spec_json}
)
```

### Step 5: Return Summary to Orchestrator

Return summary of API contract for Phase 02 context:

```json
{
  "pre_phase": "02",
  "status": "complete",
  "output_file": "devforgeai/feedback/ai-analysis/${STORY_ID}/pre-02-api-spec.json",
  "summary": {
    "types_defined": 3,
    "methods_defined": 5,
    "errors_defined": 2,
    "test_patterns": ["unit", "integration"]
  }
}
```

---

## Error Handling

**If Technical Specification section missing:**
```
Log warning: "Story lacks Technical Specification - using AC only"
Generate minimal spec from Acceptance Criteria
Continue with warning flag
```

**If story file not found:**
```
HALT: "Cannot execute pre-02-planning: Story file not found"
Return error to orchestrator
```

**If write fails:**
```
Log error: "Failed to write pre-02 spec: {error}"
Continue with warning (non-blocking)
Return spec in response for orchestrator to handle
```

---

## Performance Requirements

**NFR-001:** Pre-phase execution
- Total execution time: < 30 seconds
- Model used: haiku (per config defaults)

---

## Output Schema

See: `.claude/skills/devforgeai-development/references/observation-write-protocol.md`

---

## References

- **Config:** `devforgeai/config/pre-phase-planning.yaml`
- **Coding Standards:** `devforgeai/specs/context/coding-standards.md`
- **Architecture Constraints:** `devforgeai/specs/context/architecture-constraints.md`
- **Observation Protocol:** `.claude/skills/devforgeai-development/references/observation-write-protocol.md`
