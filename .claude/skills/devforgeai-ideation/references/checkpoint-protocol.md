# Checkpoint Protocol for Ideation Sessions

**Status**: LOCKED
**Version**: 1.0
**STORY**: STORY-136

## Overview

File-based checkpoint protocol for preserving ideation session state across context window boundaries.

---

## Checkpoint Lifecycle

### Phase Boundaries

Checkpoints are created/updated at each ideation phase boundary:

| Phase | Trigger | Data Captured |
|-------|---------|---------------|
| 1 | Brainstorm complete | problem_statement |
| 2 | Personas defined | personas array |
| 3 | Requirements gathered | requirements array |
| 4 | Complexity assessed | complexity_score |
| 5 | Epics generated | epics array |

### File Location

```
devforgeai/temp/.ideation-checkpoint-{session_id}.yaml
```

- **Hidden file**: Prefix `.` prevents accidental modification
- **Session-scoped**: UUID v4 session ID ensures uniqueness
- **Temp directory**: Cleaned up after session completion

---

## Core Components

### SessionIdGenerator

Generates UUID v4 session identifiers for checkpoint correlation.

```python
from checkpoint_protocol import SessionIdGenerator

generator = SessionIdGenerator()
session_id = generator.generate()
# Returns: "550e8400-e29b-41d4-a716-446655440000"
```

### TimestampGenerator

Generates ISO 8601 timestamps with millisecond precision.

```python
from checkpoint_protocol import TimestampGenerator

generator = TimestampGenerator()
timestamp = generator.generate()
# Returns: "2025-12-25T15:30:45.123Z"
```

### CheckpointService

Main orchestrator for checkpoint lifecycle management.

```python
from checkpoint_protocol import CheckpointService

service = CheckpointService(write_tool=mock_write_tool)

# Create initial checkpoint
service.create_checkpoint({
    "session_id": session_id,
    "timestamp": timestamp,
    "current_phase": 1,
    "phase_completed": True,
    "brainstorm_context": {...}
})

# Update at phase boundary
service.update_checkpoint(session_id, phase=2, context=updated_context)
```

---

## Validation

### Session ID Validation

- UUID v4 format: 8-4-4-4-12 hexadecimal pattern
- Case-insensitive matching
- Version bit verification

### Timestamp Validation

- ISO 8601 format: `YYYY-MM-DDTHH:MM:SS.fffZ`
- Millisecond precision required
- UTC timezone (Z suffix) required

### Path Validation

- Must start with `devforgeai/temp/`
- Hidden file prefix (`.`) required
- No directory traversal (`..`)

### Complexity Score Validation

- Integer range: 0-60 (inclusive)
- Maps to ideation complexity rubric

---

## Error Handling

### Graceful Degradation (BR-004)

Write failures MUST NOT crash the ideation session:

```python
try:
    checkpoint_service.create_checkpoint(data)
except IOError as e:
    # Log warning, continue session
    logger.warning(f"Checkpoint write failed: {e}")
    # Session continues without checkpoint
```

### Security

- **SecretScanner**: Detects API keys, passwords, secrets before write
- **PathValidator**: Prevents directory traversal attacks

---

## Business Rules

| Rule | Description |
|------|-------------|
| BR-001 | Use Write tool ONLY (no Bash for file ops) |
| BR-002 | Path MUST be `devforgeai/temp/.ideation-checkpoint-{session_id}.yaml` |
| BR-003 | Session ID generated once at session start and reused |
| BR-004 | Write failures MUST NOT crash session (graceful degradation) |

---

## Schema Reference

See: `checkpoint-schema.yaml` for complete YAML schema definition.

---

## Resume Protocol (STORY-137)

For resuming from checkpoint, see: `checkpoint-resume.md` (to be created in STORY-137)

**Basic Resume Flow:**

1. Read checkpoint file using `Read(file_path=...)`
2. Parse YAML using `yaml.safe_load()`
3. Extract `ResumeService.extract_resumable_state()`
4. Continue from `current_phase + 1`
