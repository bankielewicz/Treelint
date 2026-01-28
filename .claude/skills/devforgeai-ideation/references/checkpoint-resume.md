# Resume Workflow for Ideation Sessions

**Status**: DRAFT (Full implementation in STORY-137)
**Version**: 0.1
**STORY**: STORY-136 (protocol), STORY-137 (resume implementation)

## Overview

This document describes how to resume an interrupted ideation session from a checkpoint file.

---

## Resume Flow

### Step 1: Detect Existing Checkpoint

```
Glob(pattern="devforgeai/temp/.ideation-checkpoint-*.yaml")

IF file exists:
    Display: "Previous session detected. Resume?"
    AskUserQuestion: Resume / Start Fresh
```

### Step 2: Read Checkpoint

```python
from checkpoint_protocol import ResumeService

# Read checkpoint file
checkpoint_content = Read(file_path=checkpoint_path)
checkpoint_data = yaml.safe_load(checkpoint_content)

# Extract resumable state
resume_service = ResumeService()
state = resume_service.extract_resumable_state(checkpoint_data)
```

### Step 3: Validate Checkpoint

```python
from checkpoint_protocol import CheckpointValidator

validator = CheckpointValidator()
validator.validate(checkpoint_data)  # Raises ValueError if invalid
```

### Step 4: Restore Context

```python
# Get resume information
session_id = state.session_id
current_phase = state.current_phase
brainstorm_context = state.brainstorm_context

# Continue from next phase
next_phase = current_phase + 1
```

### Step 5: Continue Ideation

Resume the ideation workflow from `next_phase` with restored context.

---

## ResumeService API

```python
class ResumeService:
    """Extracts resumable state from checkpoint data."""

    def extract_resumable_state(self, checkpoint_data: Dict[str, Any]) -> ResumeState:
        """
        Extract state needed to resume session.

        Args:
            checkpoint_data: Parsed checkpoint YAML

        Returns:
            ResumeState with session_id, current_phase, phase_completed,
            brainstorm_context, and convenience accessors
        """

class ResumeState:
    """Container for resumable session state."""

    @property
    def session_id(self) -> str: ...
    @property
    def current_phase(self) -> int: ...
    @property
    def phase_completed(self) -> bool: ...
    @property
    def brainstorm_context(self) -> Dict[str, Any]: ...

    # Convenience accessors
    @property
    def personas(self) -> List[Dict]: ...
    @property
    def requirements(self) -> List[Dict]: ...
    @property
    def complexity_score(self) -> Optional[int]: ...
```

---

## Error Handling

### Corrupted Checkpoint

```python
try:
    checkpoint_data = yaml.safe_load(content)
    validator.validate(checkpoint_data)
except (yaml.YAMLError, ValueError) as e:
    # Checkpoint corrupted - start fresh
    logger.warning(f"Checkpoint invalid: {e}")
    # Offer to start new session
```

### Stale Checkpoint

```python
# Check timestamp age
checkpoint_time = datetime.fromisoformat(checkpoint_data["timestamp"].replace("Z", "+00:00"))
age = datetime.now(timezone.utc) - checkpoint_time

if age > timedelta(days=7):
    # Warn user about stale checkpoint
    AskUserQuestion: "Checkpoint is {age.days} days old. Resume anyway?"
```

---

## STORY-137 Implementation Scope

The following will be implemented in STORY-137:

1. **Automatic checkpoint detection** on `/ideate` invocation
2. **Resume prompt** with session details display
3. **Context restoration** into conversation
4. **Phase continuation** from last completed phase
5. **Checkpoint cleanup** after successful completion
6. **Stale checkpoint handling** (>7 days warning)

---

## See Also

- `checkpoint-protocol.md` - Checkpoint creation and lifecycle
- `checkpoint-schema.yaml` - YAML schema definition
