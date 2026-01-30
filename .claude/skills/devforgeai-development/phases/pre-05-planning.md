# Pre-Phase 05: Integration Planning

**Trigger:** `pre-phase-planning.yaml` has `phases.05.enabled: true`

**Purpose:** Create integration test plan from Phase 04 observations before integration-tester executes in Phase 05.

**Status:** Conditional (disabled by default)
**Story:** STORY-FEEDBACK-004
**Implementation Date:** 2026-01-29

---

## Overview

This pre-phase analyzes refactoring observations from Phase 04 and the overall implementation to identify integration test priorities. The resulting plan guides integration-tester to focus on cross-component interactions that matter most.

**Benefits:**
- Integration tests target actual component boundaries
- Coverage based on implementation reality (not theory)
- Prioritizes critical user journeys
- Documents integration scope for QA

---

## Prerequisites

**Before executing this pre-phase:**
1. Phase 04 (Refactoring) completed successfully
2. Pre-phase planning enabled in config (`phases.05.enabled: true`)
3. All unit tests passing

---

## Workflow

### Step 1: Read Phase 04 Observations

```
Read(file_path="devforgeai/feedback/ai-analysis/${STORY_ID}/phase-04-code-reviewer.json")
Read(file_path="devforgeai/feedback/ai-analysis/${STORY_ID}/phase-04-refactoring-specialist.json")
```

Extract:
- Component boundaries identified
- Cross-component dependencies
- Integration points refactored
- Any warnings about integration risks

### Step 2: Read All Prior Observations

```
Glob(pattern="devforgeai/feedback/ai-analysis/${STORY_ID}/*.json")

FOR each observation_file:
    Read and aggregate observations
    Identify patterns across phases
    Note repeated friction points
```

### Step 3: Analyze Story Technical Specification

```
Read(file_path="devforgeai/specs/Stories/${STORY_ID}.story.md")
```

Extract:
- API endpoints to test end-to-end
- Database operations requiring transaction tests
- External service integrations
- Critical user journeys

### Step 4: Generate Integration Plan

Create integration test scope:

```json
{
  "story_id": "${STORY_ID}",
  "timestamp": "${ISO_8601_TIMESTAMP}",
  "integration_scope": {
    "api_contracts": [
      {
        "endpoint": "POST /api/orders",
        "test_type": "contract",
        "priority": "high",
        "validates": ["request schema", "response schema", "error codes"]
      }
    ],
    "database_transactions": [
      {
        "operation": "create_order_with_inventory_update",
        "test_type": "transaction",
        "priority": "high",
        "validates": ["atomicity", "rollback on error"]
      }
    ],
    "component_interactions": [
      {
        "source": "OrderService",
        "target": "InventoryService",
        "test_type": "integration",
        "priority": "medium",
        "validates": ["stock reservation", "stock release on cancel"]
      }
    ],
    "user_journeys": [
      {
        "name": "Complete purchase flow",
        "steps": ["register", "login", "add to cart", "checkout", "payment"],
        "test_type": "e2e",
        "priority": "high",
        "estimated_duration_ms": 5000
      }
    ]
  },
  "excluded_from_integration": [
    {
      "component": "EmailNotificationService",
      "reason": "Mock in integration tests, verify in separate e2e suite"
    }
  ],
  "prior_observations_addressed": [
    {
      "id": "obs-04-001",
      "category": "gap",
      "note": "No transaction boundary tests",
      "addressed_by": "database_transactions scope"
    }
  ],
  "estimated_test_count": 8,
  "estimated_duration_minutes": 15
}
```

### Step 5: Write to Disk

```
Write(
  file_path="devforgeai/feedback/ai-analysis/${STORY_ID}/pre-05-integration-plan.json",
  content=${integration_plan_json}
)
```

### Step 6: Return Summary to Orchestrator

```json
{
  "pre_phase": "05",
  "status": "complete",
  "output_file": "devforgeai/feedback/ai-analysis/${STORY_ID}/pre-05-integration-plan.json",
  "summary": {
    "api_contracts": 1,
    "database_transactions": 1,
    "component_interactions": 1,
    "user_journeys": 1,
    "estimated_test_count": 8,
    "observations_addressed": 1
  }
}
```

---

## Error Handling

**If Phase 04 observations not found:**
```
Log warning: "No Phase 04 observations - inferring from story spec"
Generate plan from technical specification and AC
Continue with limited scope
```

**If no integration tests needed:**
```
Return:
{
  "pre_phase": "05",
  "status": "complete",
  "summary": {
    "integration_scope": "minimal",
    "message": "Story is self-contained unit - minimal integration tests"
  }
}
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
- **Integration Testing:** `.claude/skills/devforgeai-development/references/integration-testing.md`
- **Architecture Constraints:** `devforgeai/specs/context/architecture-constraints.md`
- **Observation Protocol:** `.claude/skills/devforgeai-development/references/observation-write-protocol.md`
