# Memory File Schema

**Status:** LOCKED
**Version:** 1.0
**Created:** 2026-01-23
**Story:** STORY-303

---

## Purpose

Define YAML frontmatter schema for session memory files that persist workflow state across context window clears.

**Performance:** 39% improvement in context recovery (Anthropic research)

---

## File Location Pattern

`.claude/memory/sessions/{STORY_ID}-{workflow}-session.md`

**Examples:**
- `.claude/memory/sessions/STORY-303-dev-session.md`
- `.claude/memory/sessions/STORY-303-qa-session.md`
- `.claude/memory/sessions/STORY-303-release-session.md`

**Workflow Namespacing (BR-004):**
- No collision between /dev, /qa, /release for same story

---

## Required Fields

### story_id (Required)

- **Type:** String
- **Pattern:** `STORY-\d{3,4}` (e.g., STORY-303, STORY-1234)
- **Validation:** Must match pattern exactly
- **Example:** `story_id: "STORY-303"`

### epic_id (Required, Nullable)

- **Type:** String | null
- **Pattern:** `EPIC-\d{3}` or `null`
- **Validation:** If present, must match pattern
- **Example:** `epic_id: "EPIC-049"` or `epic_id: null`

### workflow_name (Required)

- **Type:** String
- **Enum:** `dev`, `qa`, `release`
- **Description:** Workflow type for namespacing (BR-004)
- **Example:** `workflow_name: "dev"`

### current_phase (Required)

- **Type:** String
- **Enum:** `01`, `02`, `03`, `04`, `05`, `06`, `07`, `08`, `09`, `10`
- **Description:** Last completed or in-progress phase
- **Example:** `current_phase: "03"`

### phase_progress (Required)

- **Type:** Float
- **Range:** `0.0` to `1.0`
- **Description:** Progress within current phase (0% to 100%)
- **Example:** `phase_progress: 0.6`

### decisions (Required)

- **Type:** Array of Decision objects
- **Constraints:** Can be empty array `[]`
- **Object Structure:**
  - `timestamp`: ISO 8601 datetime
  - `description`: Brief summary of decision
  - `rationale`: Why this decision was made
- **Example:**
```yaml
decisions:
  - timestamp: "2026-01-23T14:30:00Z"
    description: "Use pytest for testing"
    rationale: "Project already uses pytest; consistent with tech-stack"
```

### blockers (Required)

- **Type:** Array of Blocker objects
- **Constraints:** Can be empty array `[]`
- **Object Structure:**
  - `id`: Unique blocker identifier (e.g., "BLK-001")
  - `description`: What is blocking progress
  - `severity`: `low`, `medium`, `high`, `critical`
  - `resolution_status`: `open`, `resolved`, `deferred`
- **Example:**
```yaml
blockers:
  - id: "BLK-001"
    description: "Missing database credentials"
    severity: "high"
    resolution_status: "open"
```

### session_started (Required)

- **Type:** ISO 8601 timestamp
- **Format:** `YYYY-MM-DDTHH:MM:SSZ`
- **Description:** When this session began
- **Example:** `session_started: "2026-01-23T14:00:00Z"`

### last_updated (Required)

- **Type:** ISO 8601 timestamp
- **Format:** `YYYY-MM-DDTHH:MM:SSZ`
- **Constraint:** Must be >= session_started
- **Description:** Last memory file update time
- **Example:** `last_updated: "2026-01-23T14:45:00Z"`

---

## Complete Example

```yaml
---
story_id: "STORY-303"
epic_id: "EPIC-049"
workflow_name: "dev"
current_phase: "03"
phase_progress: 0.6
decisions:
  - timestamp: "2026-01-23T14:30:00Z"
    description: "Use pytest for testing"
    rationale: "Project already uses pytest; consistent with tech-stack"
  - timestamp: "2026-01-23T14:35:00Z"
    description: "Skip integration tests for now"
    rationale: "No external dependencies in this story"
blockers: []
session_started: "2026-01-23T14:00:00Z"
last_updated: "2026-01-23T14:45:00Z"
---

# Session Notes

Additional context and notes can be added in the body section.
```

---

## Validation Rules

1. **YAML Frontmatter:** File MUST start with `---` and end frontmatter with `---`
2. **Required Fields:** All 8 fields MUST be present
3. **story_id Pattern:** Must match `STORY-\d{3,4}` regex
4. **current_phase Enum:** Must be one of 01-10
5. **phase_progress Range:** Must be between 0.0 and 1.0 inclusive
6. **ISO 8601 Timestamps:** session_started and last_updated must be valid ISO 8601
7. **Temporal Consistency:** last_updated >= session_started

---

## Sample Templates

### Fresh Session Template

```yaml
---
story_id: "{STORY_ID}"
epic_id: null
workflow_name: "dev"
current_phase: "01"
phase_progress: 0.0
decisions: []
blockers: []
session_started: "{ISO_TIMESTAMP}"
last_updated: "{ISO_TIMESTAMP}"
---
```

### Mid-Session Template

```yaml
---
story_id: "{STORY_ID}"
epic_id: "{EPIC_ID}"
workflow_name: "dev"
current_phase: "03"
phase_progress: 0.5
decisions:
  - timestamp: "{ISO_TIMESTAMP}"
    description: "{DECISION}"
    rationale: "{RATIONALE}"
blockers: []
session_started: "{SESSION_START}"
last_updated: "{ISO_TIMESTAMP}"
---
```

---

## Reference

- **Operations:** See `memory-file-operations.md` for read/write functions
- **Integration:** Phase 01 preflight checks for memory file
- **Story:** STORY-303 (Memory Files for Cross-Session State)
