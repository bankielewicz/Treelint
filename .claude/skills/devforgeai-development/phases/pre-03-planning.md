# Pre-Phase 03: Implementation Planning

**Trigger:** `pre-phase-planning.yaml` has `phases.03.enabled: true`

**Purpose:** Create implementation plan from API spec and Phase 02 observations before backend-architect implements in Phase 03.

**Status:** Conditional (disabled by default)
**Story:** STORY-FEEDBACK-004
**Implementation Date:** 2026-01-29

---

## Overview

This pre-phase synthesizes the API specification (from pre-02) and test observations (from Phase 02) into a concrete implementation plan. The plan guides backend-architect to write code that:
1. Passes all generated tests
2. Follows the API contract exactly
3. Addresses any observations from test generation

**Benefits:**
- Implementation aligned with tests (not divergent)
- Course correction based on Phase 02 friction/gaps
- Clear file structure before coding
- Pattern guidance specific to this story

---

## Prerequisites

**Before executing this pre-phase:**
1. Phase 02 (Test-First) completed successfully
2. Pre-phase planning enabled in config (`phases.03.enabled: true`)
3. Optional: pre-02-api-spec.json exists (if pre-02 was enabled)

---

## Workflow

### Step 1: Read API Specification (if exists)

```
Glob(pattern="devforgeai/feedback/ai-analysis/${STORY_ID}/pre-02-api-spec.json")

IF file exists:
    Read(file_path="devforgeai/feedback/ai-analysis/${STORY_ID}/pre-02-api-spec.json")
    $API_SPEC = parsed content
ELSE:
    $API_SPEC = null
    Log: "No pre-02 API spec - inferring from tests"
```

### Step 2: Read Phase 02 Observations

```
Read(file_path="devforgeai/feedback/ai-analysis/${STORY_ID}/phase-02-test-automator.json")
```

Extract:
- Test file locations
- API assumptions made by tests
- Any HIGH/MEDIUM severity observations
- Friction points or gaps identified

### Step 3: Read Story Technical Specification

```
Read(file_path="devforgeai/specs/Stories/${STORY_ID}.story.md")
```

Extract:
- Files to create/modify
- Implementation patterns required
- Business rules to enforce
- Error handling requirements

### Step 4: Read Context Files

```
Read(file_path="devforgeai/specs/context/source-tree.md")
Read(file_path="devforgeai/specs/context/anti-patterns.md")
```

Extract:
- Valid file locations for new code
- Anti-patterns to avoid

### Step 5: Generate Implementation Plan

Create plan aligned with test expectations:

```json
{
  "story_id": "${STORY_ID}",
  "timestamp": "${ISO_8601_TIMESTAMP}",
  "implementation_guidance": {
    "files_to_create": [
      {
        "path": "src/module/file.py",
        "purpose": "Main implementation",
        "layer": "domain|application|infrastructure"
      }
    ],
    "api_alignment": [
      {
        "test_expects": "start() method returns Result<(), WatcherError>",
        "implement_as": "pub fn start(&self) -> Result<(), WatcherError>",
        "notes": "Ensure return type matches test assertions"
      }
    ],
    "patterns_to_follow": [
      "Repository pattern for data access",
      "Dependency injection for all services",
      "Result type for error handling"
    ],
    "anti_patterns_to_avoid": [
      "God Object (keep classes < 500 lines)",
      "Direct instantiation (use DI)",
      "SQL concatenation (use parameterized queries)"
    ],
    "error_handling": [
      {
        "error_type": "WatcherError",
        "variants": ["DirectoryNotFound", "InitializationError"],
        "test_coverage": "tests/test_watcher_errors.py"
      }
    ]
  },
  "prior_observations_addressed": [
    {
      "id": "obs-02-001",
      "category": "friction",
      "note": "Unclear naming convention",
      "addressed_by": "Using PascalCase for types per coding-standards.md"
    }
  ],
  "estimated_files": 3,
  "estimated_lines": 250
}
```

### Step 6: Write to Disk

```
Write(
  file_path="devforgeai/feedback/ai-analysis/${STORY_ID}/pre-03-impl-plan.json",
  content=${impl_plan_json}
)
```

### Step 7: Return Summary to Orchestrator

```json
{
  "pre_phase": "03",
  "status": "complete",
  "output_file": "devforgeai/feedback/ai-analysis/${STORY_ID}/pre-03-impl-plan.json",
  "summary": {
    "files_to_create": 3,
    "api_alignments": 5,
    "observations_addressed": 2,
    "patterns_required": ["repository", "di", "result-type"]
  }
}
```

---

## Error Handling

**If Phase 02 observations not found:**
```
Log warning: "No Phase 02 observations found - proceeding without course correction"
Generate plan from story spec and API spec only
Continue with warning flag
```

**If both API spec and observations missing:**
```
Log warning: "No prior phase artifacts - using story spec only"
Generate basic plan from technical specification
Continue with limited guidance
```

**If write fails:**
```
Log error: "Failed to write pre-03 plan: {error}"
Continue with warning (non-blocking)
Return plan in response for orchestrator to handle
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
- **Source Tree:** `devforgeai/specs/context/source-tree.md`
- **Anti-Patterns:** `devforgeai/specs/context/anti-patterns.md`
- **Observation Protocol:** `.claude/skills/devforgeai-development/references/observation-write-protocol.md`
