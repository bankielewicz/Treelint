"""
Phase Validation CLI Commands.

Provides CLI commands for phase state management in the
Phase Execution Enforcement System.

Commands:
- phase-init: Create state file (exit 0=created, 1=exists, 2=invalid ID)
- phase-check: Validate transition (exit 0=allowed, 1=blocked, 2=missing subagents)
- phase-complete: Mark phase done (exit 0=success, 1=incomplete)
- phase-status: Show current state (exit 0=success, 1=not found)
- phase-record: Record subagent invocation (exit 0=recorded, 1=not found, 2=error)
"""

import json
from pathlib import Path


def _get_valid_phases():
    """Get VALID_PHASES constant from phase_state module."""
    from ..phase_state import PhaseState
    return PhaseState.VALID_PHASES


def _get_phase_state(project_root: str):
    """
    Get PhaseState instance with graceful error handling.

    PhaseState is co-located in the same package for simple imports.

    Args:
        project_root: Path to the project root directory

    Returns:
        PhaseState instance for phase tracking

    Raises:
        ImportError: If phase_state.py module cannot be imported, with
                     helpful diagnostic message including:
                     - Original error details
                     - Expected module location
                     - Fix instructions
                     - Note about /dev workflow continuation
    """
    try:
        from ..phase_state import PhaseState
        return PhaseState(project_root=Path(project_root))
    except ImportError as e:
        raise ImportError(
            f"PhaseState module not found: {e}\n\n"
            "The phase_state.py module is required for phase tracking.\n"
            "Expected location: .claude/scripts/devforgeai_cli/phase_state.py\n\n"
            "To fix:\n"
            "  1. Ensure STORY-253 (PhaseState module) is implemented\n"
            "  2. Reinstall CLI using one of these methods:\n\n"
            "     # Using pipx (recommended for CLI tools):\n"
            "     pipx install -e .claude/scripts/ --force\n\n"
            "     # Using virtual environment:\n"
            "     python3 -m venv .venv && source .venv/bin/activate\n"
            "     pip install -e .claude/scripts/\n\n"
            "     # Direct pip (if not externally-managed):\n"
            "     pip install -e .claude/scripts/\n\n"
            "  3. Retry your command\n\n"
            "Note: The /dev workflow can continue without CLI-based phase\n"
            "enforcement if this module is unavailable. Phase tracking is\n"
            "optional and does not block story development."
        ) from e


def phase_init_command(
    story_id: str,
    project_root: str,
    format: str = "text"
) -> int:
    """
    Initialize phase state file for a story.

    Args:
        story_id: Story identifier (e.g., "STORY-001")
        project_root: Project root directory
        format: Output format ("text" or "json")

    Returns:
        Exit code: 0=created, 1=exists, 2=invalid ID
    """
    try:
        ps = _get_phase_state(project_root)
        state_path = ps._get_state_path(story_id)

        # Check if already exists
        if state_path.exists():
            if format == "json":
                print(json.dumps({
                    "success": False,
                    "error": "State file already exists",
                    "story_id": story_id,
                    "path": str(state_path)
                }))
            else:
                print(f"State file already exists for {story_id}")
                print(f"  Path: {state_path}")
            return 1

        # Create new state
        state = ps.create(story_id)

        if format == "json":
            print(json.dumps({
                "success": True,
                "story_id": story_id,
                "path": str(state_path),
                "current_phase": state["current_phase"]
            }))
        else:
            print(f"Created phase state for {story_id}")
            print(f"  Path: {state_path}")
            print(f"  Current phase: {state['current_phase']}")

        return 0

    except ValueError as e:
        if format == "json":
            print(json.dumps({
                "success": False,
                "error": str(e),
                "story_id": story_id
            }))
        else:
            print(f"ERROR: {e}")
        return 2

    except Exception as e:
        if format == "json":
            print(json.dumps({
                "success": False,
                "error": str(e),
                "story_id": story_id
            }))
        else:
            print(f"ERROR: {e}")
        return 2


def phase_check_command(
    story_id: str,
    from_phase: str,
    to_phase: str,
    project_root: str,
    format: str = "text"
) -> int:
    """
    Check if phase transition is allowed.

    Args:
        story_id: Story identifier
        from_phase: Source phase (e.g., "01")
        to_phase: Target phase (e.g., "02")
        project_root: Project root directory
        format: Output format

    Returns:
        Exit code: 0=allowed, 1=blocked, 2=missing subagents
    """
    try:
        ps = _get_phase_state(project_root)
        state = ps.read(story_id)

        if state is None:
            if format == "json":
                print(json.dumps({
                    "allowed": False,
                    "error": "State file not found",
                    "story_id": story_id
                }))
            else:
                print(f"State file not found for {story_id}")
            return 1

        # Rule 1: Previous phase must be completed
        if state["phases"][from_phase]["status"] != "completed":
            if format == "json":
                print(json.dumps({
                    "allowed": False,
                    "error": f"Phase {from_phase} not completed",
                    "story_id": story_id,
                    "from_phase": from_phase,
                    "to_phase": to_phase
                }))
            else:
                print(f"Phase {from_phase} not completed")
            return 1

        # Rule 2: Must be sequential (no skipping)
        # Use ordered VALID_PHASES list to handle decimal phases (4.5, 5.5)
        valid_phases = _get_valid_phases()
        try:
            from_idx = valid_phases.index(from_phase)
            to_idx = valid_phases.index(to_phase)
        except ValueError:
            if format == "json":
                print(json.dumps({
                    "allowed": False,
                    "error": f"Invalid phase: from='{from_phase}' or to='{to_phase}'",
                    "story_id": story_id
                }))
            else:
                print(f"Invalid phase: from='{from_phase}' or to='{to_phase}'")
            return 1

        if to_idx != from_idx + 1:
            expected = valid_phases[from_idx + 1] if from_idx + 1 < len(valid_phases) else "N/A"
            if format == "json":
                print(json.dumps({
                    "allowed": False,
                    "error": f"Cannot skip phases: {from_phase} -> {to_phase}, expected {expected}",
                    "story_id": story_id
                }))
            else:
                print(f"Cannot skip phases: {from_phase} -> {to_phase}")
            return 1

        # Rule 3: All required subagents must be invoked
        required = set(state["phases"][from_phase].get("subagents_required", []))
        invoked = set(state["phases"][from_phase].get("subagents_invoked", []))
        missing = required - invoked

        if missing:
            if format == "json":
                print(json.dumps({
                    "allowed": False,
                    "error": f"Missing subagents: {list(missing)}",
                    "story_id": story_id,
                    "missing_subagents": list(missing)
                }))
            else:
                print(f"Missing subagents for phase {from_phase}:")
                for agent in missing:
                    print(f"  - {agent}")
            return 2

        # Transition allowed
        if format == "json":
            print(json.dumps({
                "allowed": True,
                "story_id": story_id,
                "from_phase": from_phase,
                "to_phase": to_phase
            }))
        else:
            print(f"Transition allowed: {from_phase} -> {to_phase}")

        return 0

    except Exception as e:
        if format == "json":
            print(json.dumps({
                "allowed": False,
                "error": str(e),
                "story_id": story_id
            }))
        else:
            print(f"ERROR: {e}")
        return 1


def phase_complete_command(
    story_id: str,
    phase: str,
    checkpoint_passed: bool,
    project_root: str,
    format: str = "text"
) -> int:
    """
    Mark a phase as complete.

    Args:
        story_id: Story identifier
        phase: Phase to complete (e.g., "02")
        checkpoint_passed: Whether checkpoint validation passed
        project_root: Project root directory
        format: Output format

    Returns:
        Exit code: 0=success, 1=incomplete/error
    """
    try:
        ps = _get_phase_state(project_root)

        result = ps.complete_phase(story_id, phase, checkpoint_passed)

        if not result:
            if format == "json":
                print(json.dumps({
                    "success": False,
                    "error": "State file not found",
                    "story_id": story_id
                }))
            else:
                print(f"State file not found for {story_id}")
            return 1

        state = ps.read(story_id)

        if format == "json":
            print(json.dumps({
                "success": True,
                "story_id": story_id,
                "completed_phase": phase,
                "current_phase": state["current_phase"],
                "checkpoint_passed": checkpoint_passed
            }))
        else:
            print(f"Phase {phase} completed for {story_id}")
            print(f"  Current phase: {state['current_phase']}")
            print(f"  Checkpoint passed: {checkpoint_passed}")

        return 0

    except Exception as e:
        if format == "json":
            print(json.dumps({
                "success": False,
                "error": str(e),
                "story_id": story_id
            }))
        else:
            print(f"ERROR: {e}")
        return 1


def phase_status_command(
    story_id: str,
    project_root: str,
    format: str = "text"
) -> int:
    """
    Display current phase status.

    Args:
        story_id: Story identifier
        project_root: Project root directory
        format: Output format

    Returns:
        Exit code: 0=success, 1=not found
    """
    try:
        ps = _get_phase_state(project_root)
        state = ps.read(story_id)

        if state is None:
            if format == "json":
                print(json.dumps({
                    "found": False,
                    "error": "State file not found",
                    "story_id": story_id
                }))
            else:
                print(f"State file not found for {story_id}")
            return 1

        if format == "json":
            print(json.dumps(state, indent=2))
        else:
            print(f"Story: {state['story_id']}")
            print(f"Started: {state['workflow_started']}")
            print(f"Current Phase: {state['current_phase']}")
            print(f"Blocking: {state['blocking_status']}")
            print()
            print("Phase Status:")
            for phase_id, phase_data in state["phases"].items():
                status = phase_data["status"]
                marker = "x" if status == "completed" else " "
                print(f"  [{marker}] Phase {phase_id}: {status}")
                if phase_data.get("subagents_invoked"):
                    print(f"      Subagents: {', '.join(phase_data['subagents_invoked'])}")

        return 0

    except Exception as e:
        if format == "json":
            print(json.dumps({
                "found": False,
                "error": str(e),
                "story_id": story_id
            }))
        else:
            print(f"ERROR: {e}")
        return 1


def phase_record_command(
    story_id: str,
    phase: str,
    subagent: str,
    project_root: str,
    format: str = "text"
) -> int:
    """
    Record a subagent invocation for a phase.

    Args:
        story_id: Story identifier (e.g., "STORY-001")
        phase: Phase ID (e.g., "02")
        subagent: Subagent name that was invoked
        project_root: Project root directory
        format: Output format ("text" or "json")

    Returns:
        Exit code: 0=recorded, 1=not found, 2=error
    """
    try:
        ps = _get_phase_state(project_root)
        success = ps.record_subagent(story_id, phase, subagent)

        if not success:
            if format == "json":
                print(json.dumps({
                    "success": False,
                    "error": "State file not found",
                    "story_id": story_id
                }))
            else:
                print(f"State file not found for {story_id}")
            return 1

        if format == "json":
            print(json.dumps({
                "success": True,
                "story_id": story_id,
                "phase": phase,
                "subagent": subagent
            }))
        else:
            print(f"Recorded subagent '{subagent}' for {story_id} phase {phase}")

        return 0

    except Exception as e:
        if format == "json":
            print(json.dumps({
                "success": False,
                "error": str(e),
                "story_id": story_id
            }))
        else:
            print(f"ERROR: {e}")
        return 2


# =============================================================================
# STORY-188: Observation Constants
# =============================================================================

# Observation categories (AC-4)
VALID_CATEGORIES = ["friction", "gap", "success", "pattern"]

# Observation severities (AC-5)
VALID_SEVERITIES = ["low", "medium", "high"]


def phase_observe_command(
    story_id: str,
    phase: str,
    category: str,
    note: str,
    severity: str = "medium",
    project_root: str = ".",
    format: str = "text"
) -> int:
    """
    Record a workflow observation for a phase.

    Captures friction, gaps, successes, and patterns during
    TDD workflow execution for AI analysis.

    Args:
        story_id: Story identifier (e.g., "STORY-188")
        phase: Phase ID (e.g., "04")
        category: Observation category (friction, gap, success, pattern)
        note: Description of the observation
        severity: Severity level (low, medium, high). Default: medium
        project_root: Project root directory
        format: Output format ("text" or "json")

    Returns:
        Exit code: 0=recorded, 1=not found, 2=invalid input
    """
    try:
        # Validate category
        if category not in VALID_CATEGORIES:
            if format == "json":
                print(json.dumps({
                    "success": False,
                    "error": f"Invalid category: '{category}'. Must be one of: {VALID_CATEGORIES}",
                    "story_id": story_id
                }))
            else:
                print(f"ERROR: Invalid category '{category}'")
                print(f"  Valid categories: {', '.join(VALID_CATEGORIES)}")
            return 2

        # Validate severity
        if severity not in VALID_SEVERITIES:
            if format == "json":
                print(json.dumps({
                    "success": False,
                    "error": f"Invalid severity: '{severity}'. Must be one of: {VALID_SEVERITIES}",
                    "story_id": story_id
                }))
            else:
                print(f"ERROR: Invalid severity '{severity}'")
                print(f"  Valid severities: {', '.join(VALID_SEVERITIES)}")
            return 2

        # Validate note is not empty
        if not note or not note.strip():
            if format == "json":
                print(json.dumps({
                    "success": False,
                    "error": "Observation note cannot be empty",
                    "story_id": story_id
                }))
            else:
                print("ERROR: Observation note cannot be empty")
            return 2

        ps = _get_phase_state(project_root)

        # Add observation
        observation_id = ps.add_observation(
            story_id=story_id,
            phase_id=phase,
            category=category,
            note=note,
            severity=severity
        )

        if observation_id is None:
            if format == "json":
                print(json.dumps({
                    "success": False,
                    "error": "State file not found",
                    "story_id": story_id
                }))
            else:
                print(f"State file not found for {story_id}")
            return 1

        if format == "json":
            print(json.dumps({
                "success": True,
                "story_id": story_id,
                "phase": phase,
                "category": category,
                "severity": severity,
                "observation_id": observation_id
            }))
        else:
            print(f"Recorded observation for {story_id} phase {phase}")
            print(f"  Category: {category}")
            print(f"  Severity: {severity}")
            print(f"  ID: {observation_id}")

        return 0

    except ValueError as e:
        if format == "json":
            print(json.dumps({
                "success": False,
                "error": str(e),
                "story_id": story_id
            }))
        else:
            print(f"ERROR: {e}")
        return 2

    except Exception as e:
        if format == "json":
            print(json.dumps({
                "success": False,
                "error": str(e),
                "story_id": story_id
            }))
        else:
            print(f"ERROR: {e}")
        return 2
