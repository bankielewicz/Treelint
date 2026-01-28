# Phase 1: Story Loading and Validation

This phase loads the story file and performs pre-execution validation checks before orchestration begins.

## Purpose

**Validate story is ready for orchestration:**
1. Story file exists and is properly formatted
2. Story status is valid for requested operation
3. Prerequisites for status transitions are met
4. Quality gates allow progression

**When executed:**
- Story Management Mode (standard orchestration)
- After checkpoint detection (Phase 0) determines starting point
- Before any skill invocation (Phase 2)

---

## Step 1: Load Story Document

### File Loading

```
Read(file_path="devforgeai/specs/Stories/{story_id}.story.md")
```

**Expected location:** `devforgeai/specs/Stories/{STORY-ID}.story.md` or `devforgeai/specs/Stories/{STORY-ID}-{slug}.story.md`

### Extract YAML Frontmatter

```yaml
---
id: STORY-042
title: User Login with Email Authentication
epic: EPIC-001
sprint: Sprint-3
status: Ready for Dev
points: 5
priority: High
assigned_to: dev-team
created: 2025-01-05
---
```

**Required fields:**
- `id` - Story identifier (STORY-NNN format)
- `title` - Brief story description
- `status` - Current workflow state (must be one of 11 valid states)
- `points` - Story point estimate
- `priority` - High, Medium, Low

**Optional fields:**
- `epic` - Parent epic ID
- `sprint` - Sprint assignment
- `assigned_to` - Team or individual
- `created` - Creation date
- `completed_date` - Completion date (when status = Released)

### Extract Content Sections

**Required sections:**
```markdown
## User Story
As a {role}, I want {feature}, so that {benefit}

## Acceptance Criteria
1. Given {precondition}, When {action}, Then {outcome}
2. Given {precondition}, When {action}, Then {outcome}
...

## Technical Specification
- API contracts
- Data models
- Business rules
- Dependencies

## Non-Functional Requirements
- Performance targets
- Security requirements
- Scalability goals

## Definition of Done
- [ ] All acceptance criteria implemented
- [ ] Unit tests written (95% coverage)
...

## Workflow Status
[Workflow history entries and checkpoints]
```

**Validation:**
- All required sections present (HALT if missing)
- Acceptance criteria has ≥1 criterion (HALT if missing)
- Definition of Done present (HALT if missing)

---

## Step 2: Validate Current State

### Valid Workflow States

**The 11 sequential states:**
```
Backlog → Architecture → Ready for Dev → In Development → Dev Complete →
QA In Progress → [QA Approved | QA Failed] → Releasing → Released
```

**Validation check:**
```
IF story.status NOT IN [11 valid states]:
  HALT: "Invalid story status: ${story.status}
         Valid states: Backlog, Architecture, Ready for Dev, In Development,
                      Dev Complete, QA In Progress, QA Approved, QA Failed,
                      Releasing, Released"
  Return: "INVALID_STATUS"
```

**See:** `state-transitions.md` for complete state definitions

---

### Verify Prerequisites for Transitions

**Example: Backlog → Ready for Dev**
```
Prerequisites:
- All 6 context files exist (devforgeai/specs/context/*.md)
- Context files non-empty (no placeholder content)

IF prerequisites NOT met:
  HALT: "Cannot transition Backlog → Ready for Dev
         Missing prerequisites:
         - Context files missing or incomplete"
  Return: "PREREQUISITES_NOT_MET"
```

**See:** `state-transitions.md` for prerequisites for all transitions

---

## Step 3: Validate Quality Gates

**Four quality gates** block story progression when requirements not met:

### Gate 1: Context Validation (Architecture → Ready for Dev)

**Requirements:**
- All 6 context files exist
- No placeholder content (TODO, TBD)
- Files non-empty

**Validation:**
```
Glob(pattern="devforgeai/specs/context/*.md")
Expected: 6 files (tech-stack, source-tree, dependencies, coding-standards, architecture-constraints, anti-patterns)

FOR each file:
  Read first 10 lines
  IF contains "TODO" OR "TBD" OR file is empty:
    HALT: "Context file incomplete: ${filename}"
```

**See:** `quality-gates.md` for complete requirements

---

### Gate 2: Test Passing (Dev Complete → QA In Progress)

**Requirements:**
- Build succeeds (no compilation errors)
- All tests pass (100% pass rate)
- Light QA validation passed

**Validation:**
```
Status check: Story status must be "Dev Complete"
Checkpoint check: DEV_COMPLETE checkpoint must exist in workflow history

IF requirements NOT met:
  HALT: "Cannot transition Dev Complete → QA In Progress
         Gate 2 requirements not met:
         - Build must succeed
         - All tests must pass
         - Light QA must pass"
```

**See:** `quality-gates.md` for test passing criteria

---

### Gate 3: QA Approval (QA Approved → Releasing)

**Requirements:**
- Deep QA validation PASSED
- Coverage meets thresholds (95%/85%/80%)
- Zero CRITICAL violations
- Zero HIGH violations (or approved exceptions)

**Validation:**
```
Status check: Story status must be "QA Approved"
Checkpoint check: QA_APPROVED checkpoint must exist
Report check: Read devforgeai/qa/reports/{story_id}-qa-report.md

IF requirements NOT met:
  HALT: "Cannot transition QA Approved → Releasing
         Gate 3 requirements not met:
         - Deep QA must pass
         - Coverage thresholds must be met
         - No CRITICAL/HIGH violations"
```

**See:** `quality-gates.md` for QA approval criteria

---

### Gate 4: Release Readiness (Releasing → Released)

**Requirements:**
- QA approved
- All workflow checkboxes complete
- No blocking dependencies

**Validation:**
```
Status check: Story status must be "Releasing"
QA check: QA_APPROVED checkpoint must exist
Staging check: STAGING_COMPLETE checkpoint must exist

IF requirements NOT met:
  HALT: "Cannot transition Releasing → Released
         Gate 4 requirements not met:
         - QA must be approved
         - Staging deployment must be complete"
```

**See:** `quality-gates.md` for release readiness criteria

---

## Output

**Validation Result Object:**
```json
{
  "story_loaded": true,
  "story_id": "STORY-042",
  "status": "Ready for Dev",
  "status_valid": true,
  "prerequisites_met": true,
  "quality_gate_passed": true,
  "ready_for_orchestration": true
}
```

**If validation fails:**
```json
{
  "story_loaded": true,
  "status_valid": false,
  "error": "INVALID_STATUS",
  "message": "Invalid story status: InProgress (expected valid state)",
  "ready_for_orchestration": false
}
```

---

## Error Handling

### Story File Not Found

```
Error: Read(file_path="devforgeai/specs/Stories/{story_id}.story.md") returns FileNotFoundError

Action:
  HALT: "Story ${story_id} not found at expected location.
         Use /create-story to create story first."
  Return: "STORY_NOT_FOUND"
```

---

### Missing YAML Frontmatter

```
Error: Story file exists but no YAML frontmatter detected

Action:
  HALT: "Story ${story_id} missing YAML frontmatter.
         Story file must begin with:
         ---
         id: STORY-XXX
         title: ...
         status: ...
         ---"
  Return: "INVALID_STORY_FORMAT"
```

---

### Missing Required Sections

```
Error: Story file missing required sections (Acceptance Criteria, Definition of Done)

Action:
  HALT: "Story ${story_id} incomplete.
         Missing required sections:
         - ${missing_sections}

         Use /create-story to generate complete story template."
  Return: "INCOMPLETE_STORY"
```

---

### Quality Gate Not Met

```
Error: Story attempting to transition but quality gate blocks

Action:
  HALT: "Quality Gate ${gate_number} blocks progression.
         Current status: ${current_status}
         Attempted transition: ${current_status} → ${next_status}

         Requirements not met:
         ${gate_requirements}

         See quality-gates.md for complete requirements."
  Return: "QUALITY_GATE_FAILED"
```

---

## Related Files

- **checkpoint-detection.md** - Phase 0 (executed before this phase)
- **skill-invocation.md** - Phase 2 (executed after this phase)
- **workflow-states.md** - Complete state definitions and descriptions
- **state-transitions.md** - Valid transitions and prerequisites
- **quality-gates.md** - All 4 gate requirements
- **troubleshooting.md** - Common story validation issues
