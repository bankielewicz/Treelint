# Pre-Phase 04: Refactoring Planning

**Trigger:** `pre-phase-planning.yaml` has `phases.04.enabled: true`

**Purpose:** Create refactoring plan from Phase 03 observations before refactoring-specialist executes in Phase 04.

**Status:** Conditional (disabled by default)
**Story:** STORY-FEEDBACK-004
**Implementation Date:** 2026-01-29

---

## Overview

This pre-phase analyzes implementation observations from Phase 03 to identify refactoring opportunities. The resulting plan guides refactoring-specialist to prioritize improvements that address real friction points rather than speculative optimizations.

**Benefits:**
- Targeted refactoring based on actual implementation experience
- Addresses complexity hotspots identified during coding
- Prevents over-engineering (refactor only what matters)
- Documents refactoring rationale for code review

---

## Prerequisites

**Before executing this pre-phase:**
1. Phase 03 (Implementation) completed successfully
2. Pre-phase planning enabled in config (`phases.04.enabled: true`)
3. Tests passing (Green state achieved)

---

## Workflow

### Step 1: Read Phase 03 Observations

```
Read(file_path="devforgeai/feedback/ai-analysis/${STORY_ID}/phase-03-backend-architect.json")
```

Extract:
- Code smells identified during implementation
- Complexity concerns (methods > 10 CC)
- Duplication introduced
- Design pattern deviations
- Any HIGH/MEDIUM severity observations

### Step 2: Read Implementation Plan (if exists)

```
Glob(pattern="devforgeai/feedback/ai-analysis/${STORY_ID}/pre-03-impl-plan.json")

IF file exists:
    Read(file_path="devforgeai/feedback/ai-analysis/${STORY_ID}/pre-03-impl-plan.json")
    Compare actual implementation vs planned structure
```

### Step 3: Analyze Created Files

```
Glob(pattern="${FILES_CREATED_IN_PHASE_03}")

FOR each file:
    Analyze:
    - Line count (> 500 = candidate for extraction)
    - Cyclomatic complexity per method
    - Code duplication percentage
    - Naming quality
```

### Step 4: Generate Refactoring Plan

Create prioritized refactoring plan:

```json
{
  "story_id": "${STORY_ID}",
  "timestamp": "${ISO_8601_TIMESTAMP}",
  "refactoring_targets": [
    {
      "file": "src/services/order_service.py",
      "method": "process_order",
      "current_complexity": 15,
      "target_complexity": 8,
      "refactoring_type": "Extract Method",
      "priority": "high",
      "observation_reference": "obs-03-002",
      "description": "Extract validation logic to separate method"
    },
    {
      "file": "src/repositories/order_repository.py",
      "issue": "Duplication with user_repository.py",
      "refactoring_type": "Extract Superclass",
      "priority": "medium",
      "lines_affected": 25,
      "description": "Create BaseRepository with shared query logic"
    }
  ],
  "no_refactoring_needed": [
    {
      "file": "src/models/order.py",
      "reason": "Clean implementation, complexity within limits"
    }
  ],
  "phase_03_observations_addressed": [
    {
      "id": "obs-03-002",
      "category": "warning",
      "note": "process_order complexity at 15",
      "refactoring_target": "src/services/order_service.py:process_order"
    }
  ],
  "estimated_effort_minutes": 20,
  "test_risk": "low"
}
```

### Step 5: Write to Disk

```
Write(
  file_path="devforgeai/feedback/ai-analysis/${STORY_ID}/pre-04-refactor-plan.json",
  content=${refactor_plan_json}
)
```

### Step 6: Return Summary to Orchestrator

```json
{
  "pre_phase": "04",
  "status": "complete",
  "output_file": "devforgeai/feedback/ai-analysis/${STORY_ID}/pre-04-refactor-plan.json",
  "summary": {
    "refactoring_targets": 2,
    "high_priority": 1,
    "medium_priority": 1,
    "estimated_effort_minutes": 20,
    "observations_addressed": 1
  }
}
```

---

## Error Handling

**If Phase 03 observations not found:**
```
Log warning: "No Phase 03 observations - analyzing code directly"
Perform static analysis on story files
Generate plan based on code analysis only
```

**If no refactoring needed:**
```
Return:
{
  "pre_phase": "04",
  "status": "complete",
  "summary": {
    "refactoring_targets": 0,
    "message": "Code meets quality standards - no refactoring required"
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
- **Anti-Patterns:** `devforgeai/specs/context/anti-patterns.md`
- **Refactoring Patterns:** `.claude/skills/devforgeai-development/references/refactoring-patterns.md`
- **Observation Protocol:** `.claude/skills/devforgeai-development/references/observation-write-protocol.md`
