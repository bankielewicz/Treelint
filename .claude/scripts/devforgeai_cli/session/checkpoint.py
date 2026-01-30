"""
Session Checkpoint Management for DevForgeAI CLI.

Enables resuming TDD implementation across context window resets.
Checkpoints are written at each phase completion and read by /resume-dev
for automatic phase detection.

STORY-120: Session Checkpoint Protocol

Based on patterns from: devforgeai_cli/validators/dod_validator.py
"""

import json
import os
import re
import shutil
from datetime import datetime, timezone
from pathlib import Path
from typing import Dict, Optional, Any


# ============================================================================
# CONSTANTS
# ============================================================================

CHECKPOINT_FILENAME = "checkpoint.json"
SESSIONS_DIR = "devforgeai/sessions"

# Phase 0-7 maps to TDD workflow phases (AC#1: phases 0-7)
PHASE_NAMES: Dict[int, str] = {
    0: "Pre-Flight",
    1: "Red (Test Generation)",
    2: "Green (Implementation)",
    3: "Refactor",
    4: "Integration",
    5: "Deferral Challenge",
    6: "DoD Update",
    7: "Git Workflow",
}

PHASE_PROGRESS: Dict[int, int] = {
    0: 10,
    1: 20,
    2: 30,
    3: 40,
    4: 50,
    5: 60,
    6: 75,
    7: 100,
}

REQUIRED_FIELDS = [
    "story_id",
    "phase",
    "phase_name",
    "timestamp",
    "progress_percentage",
    "dod_completion",
    "next_action",
]

# Validation patterns
STORY_ID_PATTERN = re.compile(r"^STORY-\d{3,}$")


# ============================================================================
# HELPER FUNCTIONS
# ============================================================================

def _get_sessions_dir() -> str:
    """Get sessions directory from environment or default."""
    return os.environ.get("DEVFORGEAI_SESSIONS_DIR", SESSIONS_DIR)


def _validate_story_id(story_id: str) -> bool:
    """Validate story ID format (STORY-NNN)."""
    if not story_id or not isinstance(story_id, str):
        return False
    return bool(STORY_ID_PATTERN.match(story_id))


def _validate_phase(phase: int) -> bool:
    """Validate phase is in valid range (0-7)."""
    if not isinstance(phase, int):
        return False
    return 0 <= phase <= 7


def _validate_dod_completion(dod_completion: Any) -> bool:
    """Validate dod_completion has required keys."""
    if not isinstance(dod_completion, dict):
        return False
    required_keys = ["implementation", "quality", "testing", "documentation"]
    return all(key in dod_completion for key in required_keys)


def _get_checkpoint_path(story_id: str) -> Path:
    """Get standardized checkpoint file path."""
    sessions_dir = _get_sessions_dir()
    return Path(sessions_dir) / story_id / CHECKPOINT_FILENAME


# ============================================================================
# PUBLIC FUNCTIONS
# ============================================================================

def write_checkpoint(
    story_id: str,
    phase: int,
    progress: Dict[str, Any],
) -> bool:
    """
    Write checkpoint to session directory.

    Args:
        story_id: Story identifier (STORY-NNN format)
        phase: Current phase number (0-7)
        progress: Checkpoint data dict containing dod_completion and next_action

    Returns:
        True on success, False on failure

    Raises:
        ValueError: If story_id or phase are invalid (optional behavior)
    """
    # Validate inputs
    if not _validate_story_id(story_id):
        return False

    if not _validate_phase(phase):
        return False

    # Extract or default dod_completion
    dod_completion = progress.get("dod_completion", {
        "implementation": [0, 0],
        "quality": [0, 0],
        "testing": [0, 0],
        "documentation": [0, 0],
    })

    if not _validate_dod_completion(dod_completion):
        return False

    # Build checkpoint data
    # Use progress_percentage from input if provided, else default from PHASE_PROGRESS
    progress_pct = progress.get("progress_percentage", PHASE_PROGRESS.get(phase, 0))

    checkpoint_data = {
        "story_id": story_id,
        "phase": phase,
        "phase_name": progress.get("phase_name", PHASE_NAMES.get(phase, f"Phase {phase}")),
        "timestamp": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "progress_percentage": progress_pct,
        "dod_completion": dod_completion,
        "last_action": progress.get("last_action", ""),
        "next_action": progress.get("next_action", f"Phase {phase + 1}"),
    }

    try:
        # Get checkpoint path
        checkpoint_path = _get_checkpoint_path(story_id)

        # Create directory if not exists
        checkpoint_path.parent.mkdir(parents=True, exist_ok=True)

        # Write JSON with atomic pattern (write to temp, then rename)
        temp_path = checkpoint_path.with_suffix(".tmp")
        with open(temp_path, "w", encoding="utf-8") as f:
            json.dump(checkpoint_data, f, indent=2)

        # Atomic rename
        shutil.move(str(temp_path), str(checkpoint_path))

        return True

    except (IOError, OSError, json.JSONDecodeError) as e:
        # Log error but don't crash
        # Graceful failure - checkpoint is optional for workflow
        return False


def read_checkpoint(story_id: str) -> Optional[Dict[str, Any]]:
    """
    Read checkpoint from session directory.

    Args:
        story_id: Story identifier (STORY-NNN format)

    Returns:
        Checkpoint data dict if valid, None if missing or corrupted

    AC#5: Graceful fallback if missing/corrupted
    """
    # Validate input
    if not _validate_story_id(story_id):
        return None

    try:
        checkpoint_path = _get_checkpoint_path(story_id)

        # Check if file exists
        if not checkpoint_path.exists():
            return None

        # Read and parse JSON
        with open(checkpoint_path, "r", encoding="utf-8") as f:
            data = json.load(f)

        # Validate required fields (AC#5: graceful fallback)
        if not isinstance(data, dict):
            return None

        for field in REQUIRED_FIELDS:
            if field not in data:
                return None

        # Validate specific field types
        if not isinstance(data.get("phase"), int):
            return None

        if not _validate_dod_completion(data.get("dod_completion")):
            return None

        return data

    except (IOError, OSError, json.JSONDecodeError, KeyError, TypeError):
        # Corrupted or unreadable - return None for graceful fallback
        return None


def delete_checkpoint(story_id: str) -> bool:
    """
    Delete checkpoint file and clean up empty directory.

    Args:
        story_id: Story identifier (STORY-NNN format)

    Returns:
        True on success (including if file didn't exist), False on error

    AC#4: Checkpoint cleaned up on Released status
    """
    # Validate input (but still succeed if invalid - idempotent)
    if not _validate_story_id(story_id):
        return True  # Nothing to delete for invalid ID

    try:
        checkpoint_path = _get_checkpoint_path(story_id)

        # Remove checkpoint file if exists
        if checkpoint_path.exists():
            checkpoint_path.unlink()

        # Remove empty parent directory
        session_dir = checkpoint_path.parent
        if session_dir.exists() and session_dir.is_dir():
            try:
                # Only remove if empty
                session_dir.rmdir()
            except OSError:
                # Directory not empty - that's fine
                pass

        return True

    except (IOError, OSError) as e:
        # Return False only on actual errors
        return False
