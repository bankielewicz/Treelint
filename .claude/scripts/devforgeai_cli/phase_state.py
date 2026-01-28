"""
PhaseState Module - Workflow Phase Tracking for DevForgeAI CLI

This module provides the PhaseState class for managing workflow phase tracking
state files during /dev workflow execution. It enforces sequential phase
execution and provides atomic, concurrent-safe file operations.

STORY-253: Create PhaseState Module in Correct Location
Source RCA: RCA-001-phase-state-module-missing.md

Usage:
    from devforgeai_cli.phase_state import PhaseState

    ps = PhaseState(project_root=Path("/path/to/project"))
    state = ps.create(story_id="STORY-001")
    state = ps.complete_phase(story_id="STORY-001", phase="01", checkpoint_passed=True)

Platform Support:
    - Windows 10+: Uses msvcrt for file locking
    - macOS 11+: Uses fcntl for file locking
    - Linux (Ubuntu/Debian/RHEL): Uses fcntl for file locking
    - WSL 1/2: Uses fcntl for file locking
"""

from __future__ import annotations

import json
import logging
import os
import re
import shutil
import tempfile
import time
import uuid
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, List, Optional, Union

# Platform-aware file locking imports
if os.name == 'posix':
    import fcntl
    _HAS_FCNTL = True
else:
    _HAS_FCNTL = False

if os.name == 'nt':
    try:
        import msvcrt
        _HAS_MSVCRT = True
    except ImportError:
        _HAS_MSVCRT = False
else:
    _HAS_MSVCRT = False

logger = logging.getLogger(__name__)


# =============================================================================
# Custom Exceptions
# =============================================================================


class PhaseStateError(Exception):
    """Base exception for PhaseState errors."""
    pass


class PhaseNotFoundError(PhaseStateError):
    """Raised when an invalid phase ID is provided."""

    def __init__(self, phase_id: str, message: Optional[str] = None):
        self.phase_id = phase_id
        if message is None:
            message = (
                f"Invalid phase_id: '{phase_id}'. "
                "Valid phases are '01' through '10'."
            )
        super().__init__(message)


class StateFileCorruptionError(PhaseStateError):
    """Raised when a state file contains malformed JSON."""

    def __init__(self, story_id: str, original_error: Optional[Exception] = None):
        self.story_id = story_id
        self.original_error = original_error
        message = (
            f"State file for '{story_id}' is corrupted. "
            "Recovery: Delete the file and re-run the workflow."
        )
        super().__init__(message)


class PhaseTransitionError(PhaseStateError):
    """Raised when attempting to complete phases out of order."""

    def __init__(
        self,
        story_id: str,
        current_phase: str,
        attempted_phase: str
    ):
        self.story_id = story_id
        self.current_phase = current_phase
        self.attempted_phase = attempted_phase
        message = (
            f"Cannot complete phase '{attempted_phase}' for '{story_id}'. "
            f"Current phase is '{current_phase}'. "
            "Phases must be completed sequentially."
        )
        super().__init__(message)


class LockTimeoutError(PhaseStateError):
    """Raised when file lock acquisition times out."""

    def __init__(self, file_path: Union[str, Path], timeout: float):
        self.file_path = str(file_path)
        self.timeout = timeout
        message = (
            f"Failed to acquire lock on '{file_path}' "
            f"after {timeout} seconds."
        )
        super().__init__(message)


class SubagentEnforcementError(PhaseStateError):
    """Raised when required subagents not invoked before phase completion.

    STORY-306: Subagent Enforcement in Phase State Completion
    """

    def __init__(self, story_id: str, phase: str, missing_subagents: List[str]):
        self.story_id = story_id
        self.phase = phase
        self.missing_subagents = missing_subagents
        message = (
            f"Cannot complete phase '{phase}' for '{story_id}'. "
            f"Missing required subagents: {', '.join(missing_subagents)}"
        )
        super().__init__(message)


# =============================================================================
# Required Subagents Per Phase (STORY-306)
# =============================================================================

# Tuple indicates OR logic: any one subagent in tuple satisfies requirement
# Matches SKILL.md Required Subagents Per Phase table (lines 167-181)
PHASE_REQUIRED_SUBAGENTS: Dict[str, List[Union[str, tuple]]] = {
    "01": ["git-validator", "tech-stack-detector"],
    "02": ["test-automator"],
    "03": [("backend-architect", "frontend-developer"), "context-validator"],  # tuple = OR
    "04": ["refactoring-specialist", "code-reviewer"],
    "4.5": ["ac-compliance-verifier"],
    "05": ["integration-tester"],
    "5.5": ["ac-compliance-verifier"],
    "06": [],  # deferral-validator is conditional
    "07": [],  # no required subagents (file operations)
    "08": [],  # no required subagents (git operations)
    "09": ["framework-analyst"],  # RCA-027 fix
    "10": ["dev-result-interpreter"],
}


# =============================================================================
# PhaseState Class
# =============================================================================


class PhaseState:
    """
    Manages workflow phase tracking state files for DevForgeAI.

    Provides atomic, concurrent-safe operations for creating, reading,
    and updating phase state during /dev workflow execution.

    Attributes:
        project_root: Path to the project root directory
        workflows_dir: Path to the workflows directory (project_root/devforgeai/workflows)

    Example:
        >>> ps = PhaseState(project_root=Path("/my/project"))
        >>> state = ps.create(story_id="STORY-001")
        >>> print(state["current_phase"])  # "01"
    """

    # Valid phase IDs (includes decimal phases 4.5 and 5.5 for AC verification)
    VALID_PHASES: List[str] = [
        "01", "02", "03", "04", "4.5", "05", "5.5",
        "06", "07", "08", "09", "10"
    ]

    # Valid observation categories
    VALID_CATEGORIES: List[str] = ["friction", "gap", "success", "pattern"]

    # Valid observation severities
    VALID_SEVERITIES: List[str] = ["low", "medium", "high"]

    # Story ID pattern
    STORY_ID_PATTERN: re.Pattern = re.compile(r'^STORY-\d{3}$')

    # Lock timeout in seconds
    LOCK_TIMEOUT: float = 5.0

    # Maximum note length
    MAX_NOTE_LENGTH: int = 1000

    def __init__(self, project_root: Path) -> None:
        """
        Initialize PhaseState with project root directory.

        Args:
            project_root: Path to the project root directory

        Example:
            >>> ps = PhaseState(project_root=Path("/my/project"))
            >>> ps.workflows_dir
            PosixPath('/my/project/devforgeai/workflows')
        """
        self.project_root = Path(project_root)
        self.workflows_dir = self.project_root / "devforgeai" / "workflows"
        logger.debug(f"PhaseState initialized: project_root={project_root}")

    def _validate_story_id(self, story_id: str) -> None:
        """
        Validate story ID format and check for path traversal.

        Args:
            story_id: Story identifier to validate

        Raises:
            ValueError: If story_id is invalid or contains path traversal
        """
        # Check for path traversal attempts
        if '..' in story_id or '/' in story_id or '\\' in story_id:
            raise ValueError(
                f"Invalid story_id: '{story_id}'. "
                "Must match pattern STORY-XXX (e.g., STORY-001)"
            )

        # Check for null bytes (security)
        if '\x00' in story_id:
            raise ValueError(
                f"Invalid story_id: '{story_id}'. "
                "Must match pattern STORY-XXX (e.g., STORY-001)"
            )

        # Check pattern match
        if not self.STORY_ID_PATTERN.match(story_id):
            raise ValueError(
                f"Invalid story_id: '{story_id}'. "
                "Must match pattern STORY-XXX (e.g., STORY-001)"
            )

    def _validate_phase_id(self, phase_id: str) -> None:
        """
        Validate phase ID is in valid range.

        Args:
            phase_id: Phase identifier to validate

        Raises:
            PhaseNotFoundError: If phase_id is not valid
        """
        if phase_id not in self.VALID_PHASES:
            raise PhaseNotFoundError(phase_id)

    def _get_state_path(self, story_id: str) -> Path:
        """
        Get the path to the state file for a story.

        Args:
            story_id: Story identifier

        Returns:
            Path to the state file

        Example:
            >>> ps = PhaseState(project_root=Path("/project"))
            >>> ps._get_state_path("STORY-001")
            PosixPath('/project/devforgeai/workflows/STORY-001-phase-state.json')
        """
        return self.workflows_dir / f"{story_id}-phase-state.json"

    def _create_initial_state(self, story_id: str) -> Dict[str, Any]:
        """
        Create the initial state dictionary for a new story.

        Args:
            story_id: Story identifier

        Returns:
            Initial state dictionary
        """
        now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

        phases = {}
        for phase in self.VALID_PHASES:
            # Populate subagents_required from constant (AC2 - STORY-306)
            # Convert tuples to lists for JSON serialization
            required = []
            for item in PHASE_REQUIRED_SUBAGENTS.get(phase, []):
                if isinstance(item, tuple):
                    required.append(list(item))  # OR group as list
                else:
                    required.append(item)

            phases[phase] = {
                "status": "pending",
                "subagents_required": required,
                "subagents_invoked": []
            }

        return {
            "story_id": story_id,
            "current_phase": "01",
            "workflow_started": now,
            "blocking_status": False,
            "phases": phases,
            "validation_errors": [],
            "observations": []
        }

    def _acquire_lock(self, fd: int, timeout: float) -> bool:
        """
        Acquire an exclusive lock on a file descriptor.

        Uses platform-aware locking:
        - Unix (Linux/macOS): fcntl.flock with LOCK_EX | LOCK_NB
        - Windows: msvcrt.locking with LK_NBLCK
        - Fallback: No locking (last-write-wins)

        Args:
            fd: File descriptor
            timeout: Maximum time to wait for lock

        Returns:
            True if lock acquired, False if locking not available

        Raises:
            LockTimeoutError: If lock cannot be acquired within timeout
        """
        start_time = time.time()

        while True:
            elapsed = time.time() - start_time
            if elapsed >= timeout:
                raise LockTimeoutError("<file>", timeout)

            try:
                if _HAS_FCNTL:
                    fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
                    return True
                elif _HAS_MSVCRT:
                    msvcrt.locking(fd, msvcrt.LK_NBLCK, 1)
                    return True
                else:
                    # No locking available - proceed without lock
                    logger.warning("File locking not available on this platform")
                    return False
            except (IOError, OSError):
                # Lock is held by another process, wait and retry
                time.sleep(0.1)

    def _release_lock(self, fd: int) -> None:
        """
        Release a lock on a file descriptor.

        Args:
            fd: File descriptor
        """
        try:
            if _HAS_FCNTL:
                fcntl.flock(fd, fcntl.LOCK_UN)
            elif _HAS_MSVCRT:
                msvcrt.locking(fd, msvcrt.LK_UNLCK, 1)
        except (IOError, OSError):
            pass  # Ignore errors during unlock

    def _atomic_write(self, path: Path, data: Dict[str, Any]) -> None:
        """
        Write data to file atomically using temp file + rename.

        Args:
            path: Target file path
            data: Dictionary to write as JSON
        """
        # Ensure parent directory exists
        path.parent.mkdir(parents=True, exist_ok=True)

        # Write to temp file in same directory
        fd, temp_path = tempfile.mkstemp(
            dir=str(path.parent),
            suffix='.tmp',
            prefix=path.stem
        )

        try:
            with os.fdopen(fd, 'w') as f:
                json.dump(data, f, indent=2)

            # Atomic rename
            shutil.move(temp_path, str(path))

            # Set permissions (0644)
            try:
                os.chmod(str(path), 0o644)
            except OSError:
                pass  # Ignore permission errors on some platforms

        except Exception:
            # Clean up temp file on error
            if os.path.exists(temp_path):
                os.unlink(temp_path)
            raise

    def _read_state(self, path: Path) -> Dict[str, Any]:
        """
        Read and parse state file with validation.

        Args:
            path: Path to state file

        Returns:
            Parsed state dictionary

        Raises:
            StateFileCorruptionError: If JSON is malformed or empty
        """
        story_id = path.stem.replace("-phase-state", "")

        try:
            content = path.read_text()

            # Check for empty file
            if not content.strip():
                raise StateFileCorruptionError(story_id)

            state = json.loads(content)

            # Backward compatibility: ensure all VALID_PHASES exist
            # (handles state files created before decimal phases 4.5/5.5 were added)
            self._ensure_phases_exist(state)

            return state

        except json.JSONDecodeError as e:
            raise StateFileCorruptionError(story_id, e)

    def _ensure_phases_exist(self, state: Dict[str, Any]) -> None:
        """
        Ensure all VALID_PHASES have entries in the phases dictionary.

        Adds missing phase entries with default values for backward compatibility
        with state files created before decimal phases (4.5, 5.5) were added.

        Also populates empty subagents_required from PHASE_REQUIRED_SUBAGENTS
        for legacy state files (AC8 - STORY-306).

        Args:
            state: State dictionary to update in-place
        """
        phases = state.get("phases", {})
        for phase in self.VALID_PHASES:
            if phase not in phases:
                # Create new phase entry with populated subagents_required
                required = []
                for item in PHASE_REQUIRED_SUBAGENTS.get(phase, []):
                    if isinstance(item, tuple):
                        required.append(list(item))
                    else:
                        required.append(item)

                phases[phase] = {
                    "status": "pending",
                    "subagents_required": required,
                    "subagents_invoked": []
                }
            else:
                # Migrate legacy: populate empty subagents_required (AC8)
                if not phases[phase].get("subagents_required"):
                    required = []
                    for item in PHASE_REQUIRED_SUBAGENTS.get(phase, []):
                        if isinstance(item, tuple):
                            required.append(list(item))
                        else:
                            required.append(item)
                    phases[phase]["subagents_required"] = required

        state["phases"] = phases

    def create(self, story_id: str) -> Dict[str, Any]:
        """
        Create a new phase state file for a story (idempotent).

        If a state file already exists, returns the existing state
        without modification.

        Args:
            story_id: Story identifier (must match STORY-XXX pattern)

        Returns:
            The state dictionary (new or existing)

        Raises:
            ValueError: If story_id is invalid

        Example:
            >>> ps = PhaseState(project_root=Path("/project"))
            >>> state = ps.create("STORY-001")
            >>> state["current_phase"]
            '01'
        """
        self._validate_story_id(story_id)

        path = self._get_state_path(story_id)

        # Idempotent: return existing state if file exists
        if path.exists():
            logger.debug(f"State file already exists for {story_id}")
            return self._read_state(path)

        # Create new state
        state = self._create_initial_state(story_id)
        self._atomic_write(path, state)

        logger.info(f"Created phase state for {story_id}")
        return state

    def read(self, story_id: str) -> Optional[Dict[str, Any]]:
        """
        Read the phase state for a story.

        Args:
            story_id: Story identifier

        Returns:
            State dictionary if file exists, None otherwise

        Raises:
            ValueError: If story_id is invalid
            StateFileCorruptionError: If state file is malformed

        Example:
            >>> ps = PhaseState(project_root=Path("/project"))
            >>> state = ps.read("STORY-001")
            >>> state is None  # If file doesn't exist
            True
        """
        self._validate_story_id(story_id)

        path = self._get_state_path(story_id)

        if not path.exists():
            return None

        return self._read_state(path)

    def complete_phase(
        self,
        story_id: str,
        phase: str,
        checkpoint_passed: bool
    ) -> Dict[str, Any]:
        """
        Complete a phase with sequential enforcement.

        A phase can only be completed if it is the current phase.
        After completion, current_phase advances to the next phase
        (except at phase 10).

        Args:
            story_id: Story identifier
            phase: Phase ID to complete (must be current phase)
            checkpoint_passed: Whether the phase checkpoint passed

        Returns:
            Updated state dictionary

        Raises:
            ValueError: If story_id is invalid
            PhaseNotFoundError: If phase is invalid
            PhaseTransitionError: If phase is not current phase
            FileNotFoundError: If state file doesn't exist

        Example:
            >>> ps = PhaseState(project_root=Path("/project"))
            >>> state = ps.complete_phase("STORY-001", "01", checkpoint_passed=True)
            >>> state["current_phase"]
            '02'
        """
        self._validate_story_id(story_id)
        self._validate_phase_id(phase)

        path = self._get_state_path(story_id)

        if not path.exists():
            raise FileNotFoundError(f"State file not found for {story_id}")

        state = self._read_state(path)
        current = state["current_phase"]

        # Sequential enforcement
        if phase != current:
            raise PhaseTransitionError(story_id, current, phase)

        # Subagent enforcement (AC3, AC4, AC5, AC6 - STORY-306)
        # Skip validation when checkpoint_passed=False (escape hatch - AC5)
        if checkpoint_passed:
            required = state["phases"][phase].get("subagents_required", [])
            invoked = set(state["phases"][phase].get("subagents_invoked", []))
            missing = []

            for requirement in required:
                if isinstance(requirement, list):
                    # OR logic (AC6): any one subagent in list satisfies requirement
                    if not any(subagent_name in invoked for subagent_name in requirement):
                        missing.append(f"({' OR '.join(requirement)})")
                else:
                    # Simple requirement: subagent must be in invoked set
                    if requirement not in invoked:
                        missing.append(requirement)

            if missing:
                logger.warning(
                    f"Phase {phase} completion blocked for {story_id}: "
                    f"Missing subagents: {', '.join(missing)}"
                )
                raise SubagentEnforcementError(story_id, phase, missing)
        else:
            # Escape hatch used - log for audit trail
            logger.info(
                f"Phase {phase} completed via escape hatch for {story_id} "
                "(checkpoint_passed=False)"
            )

        # Update phase status
        now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
        state["phases"][phase]["status"] = "completed"
        state["phases"][phase]["completed_at"] = now
        state["phases"][phase]["checkpoint_passed"] = checkpoint_passed

        # Advance to next phase (unless at phase 10)
        phase_idx = self.VALID_PHASES.index(phase)
        if phase_idx < len(self.VALID_PHASES) - 1:
            state["current_phase"] = self.VALID_PHASES[phase_idx + 1]

        self._atomic_write(path, state)

        logger.info(f"Completed phase {phase} for {story_id}")
        return state

    def record_subagent(
        self,
        story_id: str,
        phase: str,
        subagent: str
    ) -> Dict[str, Any]:
        """
        Record a subagent invocation for a phase (idempotent).

        If the subagent is already recorded, does nothing.
        Also sets the phase started_at timestamp if not already set.

        Args:
            story_id: Story identifier
            phase: Phase ID
            subagent: Subagent name

        Returns:
            Updated state dictionary

        Raises:
            ValueError: If story_id is invalid
            PhaseNotFoundError: If phase is invalid
            FileNotFoundError: If state file doesn't exist

        Example:
            >>> ps = PhaseState(project_root=Path("/project"))
            >>> state = ps.record_subagent("STORY-001", "02", "test-automator")
            >>> "test-automator" in state["phases"]["02"]["subagents_invoked"]
            True
        """
        self._validate_story_id(story_id)
        self._validate_phase_id(phase)

        path = self._get_state_path(story_id)

        if not path.exists():
            raise FileNotFoundError(f"State file not found for {story_id}")

        state = self._read_state(path)

        # Idempotent: only add if not already present
        if subagent not in state["phases"][phase]["subagents_invoked"]:
            state["phases"][phase]["subagents_invoked"].append(subagent)

        # Set started_at if not present
        if "started_at" not in state["phases"][phase]:
            now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
            state["phases"][phase]["started_at"] = now

        self._atomic_write(path, state)

        logger.debug(f"Recorded subagent {subagent} for {story_id} phase {phase}")
        return state

    def add_observation(
        self,
        story_id: str,
        phase_id: str,
        category: str,
        note: str,
        severity: str
    ) -> str:
        """
        Add a workflow observation.

        Args:
            story_id: Story identifier
            phase_id: Phase ID (01-10)
            category: Observation category (friction, gap, success, pattern)
            note: Observation note (1-1000 characters, non-empty)
            severity: Observation severity (low, medium, high)

        Returns:
            Generated observation ID (format: obs-{phase_id}-{8-char-hex})

        Raises:
            ValueError: If any parameter is invalid
            PhaseNotFoundError: If phase_id is invalid
            FileNotFoundError: If state file doesn't exist

        Example:
            >>> ps = PhaseState(project_root=Path("/project"))
            >>> obs_id = ps.add_observation(
            ...     story_id="STORY-001",
            ...     phase_id="04",
            ...     category="friction",
            ...     note="Test took longer than expected",
            ...     severity="medium"
            ... )
            >>> obs_id.startswith("obs-04-")
            True
        """
        self._validate_story_id(story_id)
        self._validate_phase_id(phase_id)

        # Validate category
        if category not in self.VALID_CATEGORIES:
            raise ValueError(
                f"Invalid category: '{category}'. "
                f"Valid categories: {', '.join(self.VALID_CATEGORIES)}"
            )

        # Validate severity
        if severity not in self.VALID_SEVERITIES:
            raise ValueError(
                f"Invalid severity: '{severity}'. "
                f"Valid severities: {', '.join(self.VALID_SEVERITIES)}"
            )

        # Validate note
        note = note.strip() if note else ""
        if not note:
            raise ValueError("Observation note cannot be empty")

        if len(note) > self.MAX_NOTE_LENGTH:
            raise ValueError(
                f"Observation note too long ({len(note)} chars). "
                f"Maximum length is {self.MAX_NOTE_LENGTH} characters."
            )

        path = self._get_state_path(story_id)

        if not path.exists():
            raise FileNotFoundError(f"State file not found for {story_id}")

        state = self._read_state(path)

        # Generate unique observation ID
        hex_suffix = uuid.uuid4().hex[:8]
        obs_id = f"obs-{phase_id}-{hex_suffix}"

        # Create observation object
        now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
        observation = {
            "id": obs_id,
            "phase": phase_id,
            "category": category,
            "note": note,
            "severity": severity,
            "timestamp": now
        }

        state["observations"].append(observation)
        self._atomic_write(path, state)

        logger.info(f"Added observation {obs_id} for {story_id}")
        return obs_id
