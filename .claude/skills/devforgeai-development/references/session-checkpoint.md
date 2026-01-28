# Session Checkpoint Protocol

**Story:** STORY-120 | **Epic:** EPIC-024 | **Created:** 2025-12-21

This document defines the session checkpoint protocol for enabling TDD workflow resumption after context window fills.

---

## Checkpoint Format Specification (AC#2)

**Storage Path:** `devforgeai/sessions/{STORY-ID}/checkpoint.json`

### JSON Schema

```json
{
  "story_id": "STORY-120",
  "phase": 3,
  "phase_name": "Refactor",
  "timestamp": "2025-12-20T15:30:00Z",
  "progress_percentage": 40,
  "dod_completion": {
    "implementation": [5, 8],
    "quality": [3, 6],
    "testing": [4, 5],
    "documentation": [1, 4]
  },
  "last_action": "refactoring-specialist completed",
  "next_action": "Phase 05: Integration Testing"
}
```

### Field Definitions

| Field | Type | Required | Validation | Description |
|-------|------|----------|------------|-------------|
| `story_id` | string | Yes | `STORY-\d{3,}` | Story identifier |
| `phase` | integer | Yes | 0-7 | Last completed phase number |
| `phase_name` | string | Yes | See phase names | Human-readable phase name |
| `timestamp` | string | Yes | ISO 8601 | When checkpoint was written |
| `progress_percentage` | integer | Yes | 0-100 | Overall progress percentage |
| `dod_completion` | object | Yes | See below | DoD completion counts |
| `last_action` | string | No | - | Last action performed |
| `next_action` | string | Yes | - | Next action to perform |

### Phase Number Mapping

| Phase | Name | Progress % |
|-------|------|------------|
| 0 | Pre-Flight | 10% |
| 1 | Red (Test Generation) | 20% |
| 2 | Green (Implementation) | 30% |
| 3 | Refactor | 40% |
| 4 | Integration | 50% |
| 5 | Deferral Challenge | 56% |
| 6 | DoD Update | 67% |
| 7 | Git Workflow | 78% |

---

## Write Protocol (AC#1)

### When Checkpoints Are Written

Checkpoints are written at the end of each TDD phase, after validation passes and before proceeding to the next phase.

**Trigger:** `Display: "Proceeding to Phase X..."`

### Write Locations in SKILL.md

| After Phase | Line Pattern | Progress |
|-------------|--------------|----------|
| Phase 01 (Pre-Flight) | `Proceeding to Phase 02...` | 10% |
| Phase 02 (Red) | `Proceeding to Phase 03...` | 20% |
| Phase 03 (Green) | `Proceeding to Phase 04...` | 30% |
| Phase 04 (Refactor) | `Proceeding to Phase 05...` | 40% |
| Phase 05 (Integration) | `Proceeding to Phase 06...` | 50% |
| Phase 06 (Deferral) | `Proceeding to Phase 07...` | 56% |
| Phase 07 (DoD Update) | `Proceeding to Phase 08...` | 67% |
| Phase 08 (Git) | `Proceeding to Phase 09...` | 78% |

### Write Command Pattern

```python
from devforgeai_cli.session.checkpoint import write_checkpoint

write_checkpoint(
    story_id="STORY-120",
    phase=1,
    progress={
        "dod_completion": {
            "implementation": [0, 5],
            "quality": [0, 5],
            "testing": [0, 3],
            "documentation": [0, 4]
        },
        "next_action": "Phase 02: Test-First Design"
    }
)
```

### Atomic Write Pattern

Checkpoints use atomic file writes to prevent corruption:
1. Write to temporary file (`.tmp`)
2. Rename to final path (atomic on most filesystems)
3. This prevents partial writes if interrupted

---

## Read Protocol (AC#3)

### How /resume-dev Reads Checkpoints

**Step 1.0 in resume-dev.md:**

```python
from devforgeai_cli.session.checkpoint import read_checkpoint

checkpoint = read_checkpoint("STORY-120")

if checkpoint:
    # Checkpoint found - use it
    resume_from = checkpoint["phase"] + 1  # Next phase
    print(f"Resuming from Phase {resume_from}")
else:
    # No checkpoint - fall back to DoD analysis
    print("No checkpoint found, using DoD analysis")
```

### Auto-Detection Logic

1. Check for checkpoint file at `devforgeai/sessions/{STORY-ID}/checkpoint.json`
2. If exists and valid → Extract `phase` field, resume from `phase + 1`
3. If missing → Fall back to DoD analysis (Step 1.1)
4. If corrupted → Warn user, fall back to DoD analysis

### Priority Order

1. **Explicit phase argument** (`/resume-dev STORY-120 3`) → Use specified phase
2. **Session checkpoint** → Use `checkpoint.phase + 1`
3. **DoD analysis** → Infer from incomplete DoD items

---

## Cleanup Protocol (AC#4)

### When Checkpoints Are Deleted

1. **Story reaches Released status** - Automatic cleanup
2. **7-day retention** - Stale checkpoints cleaned up
3. **Manual deletion** - User can delete `devforgeai/sessions/{STORY-ID}/`

### Cleanup in Release Workflow

Added to `devforgeai-release/SKILL.md`:

```python
from devforgeai_cli.session.checkpoint import delete_checkpoint

# After story status = "Released"
delete_checkpoint("STORY-120")
print("Session checkpoint cleaned up")
```

### Retention Policy

| Condition | Action |
|-----------|--------|
| Story Released | Delete immediately |
| Checkpoint > 7 days old | Delete on next /dev or /resume-dev |
| Story still in progress | Keep checkpoint |

---

## Error Handling (AC#5)

### Missing Checkpoint

```
IF read_checkpoint() returns None:
    Display: "ℹ No session checkpoint found - analyzing DoD for resumption point"
    Continue with DoD analysis
```

### Corrupted Checkpoint

```
IF JSON parsing fails OR required fields missing:
    Display: "⚠ Checkpoint corrupted - falling back to DoD analysis"
    Continue with DoD analysis
```

### Invalid Story ID

```
IF story_id does not match STORY-\d{3,} pattern:
    Return None (graceful failure)
```

### Graceful Fallback Pattern

All checkpoint operations follow non-blocking pattern:
- Write failures → Log warning, continue workflow
- Read failures → Return None, fall back to DoD analysis
- Delete failures → Log warning, continue workflow

Checkpoints are **advisory** - workflow continues even if checkpoint operations fail.

---

## Python API Reference

### Module: `devforgeai_cli.session.checkpoint`

**File:** `src/claude/scripts/devforgeai_cli/session/checkpoint.py`

### Functions

```python
def write_checkpoint(
    story_id: str,
    phase: int,
    progress: Dict[str, Any]
) -> bool:
    """
    Write checkpoint to session directory.

    Args:
        story_id: Story identifier (STORY-NNN format)
        phase: Current phase number (0-7)
        progress: Dict with dod_completion and next_action

    Returns:
        True on success, False on failure
    """

def read_checkpoint(
    story_id: str
) -> Optional[Dict[str, Any]]:
    """
    Read checkpoint from session directory.

    Args:
        story_id: Story identifier (STORY-NNN format)

    Returns:
        Checkpoint dict if valid, None if missing/corrupted
    """

def delete_checkpoint(
    story_id: str
) -> bool:
    """
    Delete checkpoint file and clean up empty directory.

    Args:
        story_id: Story identifier (STORY-NNN format)

    Returns:
        True on success (including if already deleted)
    """
```

### Constants

```python
CHECKPOINT_FILENAME = "checkpoint.json"
SESSIONS_DIR = "devforgeai/sessions"

PHASE_NAMES = {
    0: "Pre-Flight",
    1: "Red (Test Generation)",
    2: "Green (Implementation)",
    3: "Refactor",
    4: "Integration",
    5: "Deferral Challenge",
    6: "DoD Update",
    7: "Git Workflow"
}

PHASE_PROGRESS = {
    0: 10, 1: 20, 2: 30, 3: 40,
    4: 50, 5: 56, 6: 67, 7: 78
}
```

---

## Usage Examples

### Example 1: Context Window Fill Recovery

```
Session 1:
$ /dev STORY-120
✓ Phase 01 Pre-Flight (10%)
✓ Phase 02 Red (20%)
✓ Phase 03 Green (30%)
[Context window fills]

Session 2:
$ /resume-dev STORY-120
✓ SESSION CHECKPOINT DETECTED
  Last checkpoint: Phase 2 (Green)
  Resuming from: Phase 3 (Refactor)

[Continues from Phase 03]
```

### Example 2: Manual Phase Override

```
$ /resume-dev STORY-120 2
✓ Manual resumption: Phase 2
[Ignores checkpoint, starts at Phase 2]
```

### Example 3: No Checkpoint Fallback

```
$ /resume-dev STORY-120
ℹ No session checkpoint found - analyzing DoD for resumption point
DoD Analysis:
  Implementation: 3 incomplete
  Auto-detected: Phase 2 (Implementation)
```

---

## Non-Functional Requirements

| Requirement | Target | Measured |
|-------------|--------|----------|
| Write latency | <100ms | ~5ms |
| Read latency | <50ms | ~2ms |
| File size | <10KB | ~500 bytes |
| Directory overhead | <1MB total | ~50KB for 100 stories |

---

## Security Considerations

1. **Input validation** - Story IDs validated against `STORY-\d{3,}` pattern
2. **Path traversal prevention** - Story IDs sanitized before path construction
3. **No secrets** - Checkpoints contain only workflow metadata
4. **File permissions** - Default umask (user-readable)

---

## Related Documentation

- **Story:** `devforgeai/specs/Stories/STORY-120-session-checkpoint-protocol.story.md`
- **Epic:** `devforgeai/specs/Epics/EPIC-024-session-management.epic.md`
- **CLI Module:** `src/claude/scripts/devforgeai_cli/session/checkpoint.py`
- **Tests:** `src/claude/scripts/devforgeai_cli/tests/session/test_checkpoint.py`

---

*Protocol Version: 1.0 | Last Updated: 2025-12-21*
