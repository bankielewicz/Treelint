"""
Tests for phase CLI commands.

TDD Red Phase: These tests verify the phase-init, phase-check,
phase-complete, and phase-status CLI commands.

Exit Codes:
- phase-init: 0=created, 1=exists, 2=invalid ID
- phase-check: 0=allowed, 1=blocked, 2=missing subagents
- phase-complete: 0=success, 1=incomplete
- phase-status: 0=success, 1=not found
"""

import json
import sys
import tempfile
from pathlib import Path
from unittest.mock import patch, MagicMock

import pytest

# Add parent directory to path
sys.path.insert(0, str(Path(__file__).parent.parent))
sys.path.insert(0, str(Path(__file__).parent.parent.parent.parent.parent))


@pytest.fixture
def temp_project_dir():
    """Create a temporary project directory."""
    with tempfile.TemporaryDirectory() as tmpdir:
        project_root = Path(tmpdir)
        # Create devforgeai/workflows directory
        workflows_dir = project_root / "devforgeai" / "workflows"
        workflows_dir.mkdir(parents=True, exist_ok=True)
        yield project_root


@pytest.fixture
def existing_state(temp_project_dir):
    """Create an existing phase state file."""
    state = {
        "story_id": "STORY-001",
        "workflow_started": "2025-12-24T10:00:00Z",
        "current_phase": "02",
        "phases": {
            "01": {
                "status": "completed",
                "started_at": "2025-12-24T10:00:00Z",
                "completed_at": "2025-12-24T10:05:00Z",
                "subagents_required": ["git-validator", "tech-stack-detector"],
                "subagents_invoked": ["git-validator", "tech-stack-detector"],
                "checkpoint_passed": True
            },
            "02": {"status": "pending", "subagents_required": ["test-automator"], "subagents_invoked": []},
            "03": {"status": "pending", "subagents_required": [], "subagents_invoked": []},
            "04": {"status": "pending", "subagents_required": [], "subagents_invoked": []},
            "05": {"status": "pending", "subagents_required": [], "subagents_invoked": []},
            "06": {"status": "pending", "subagents_required": [], "subagents_invoked": []},
            "07": {"status": "pending", "subagents_required": [], "subagents_invoked": []},
            "08": {"status": "pending", "subagents_required": [], "subagents_invoked": []},
            "09": {"status": "pending", "subagents_required": [], "subagents_invoked": []},
            "10": {"status": "pending", "subagents_required": [], "subagents_invoked": []}
        },
        "validation_errors": [],
        "blocking_status": False
    }

    state_file = temp_project_dir / "devforgeai" / "workflows" / "STORY-001-phase-state.json"
    state_file.write_text(json.dumps(state, indent=2))
    return state_file


# =============================================================================
# phase-init Tests
# =============================================================================


class TestPhaseInitCommand:
    """Tests for phase-init command."""

    def test_phase_init_creates_state_file(self, temp_project_dir):
        """Test phase-init creates a new state file."""
        from commands.phase_commands import phase_init_command

        exit_code = phase_init_command("STORY-100", str(temp_project_dir), format="text")

        assert exit_code == 0
        state_file = temp_project_dir / "devforgeai" / "workflows" / "STORY-100-phase-state.json"
        assert state_file.exists()

    def test_phase_init_returns_1_if_exists(self, temp_project_dir, existing_state):
        """Test phase-init returns 1 if state file already exists."""
        from commands.phase_commands import phase_init_command

        exit_code = phase_init_command("STORY-001", str(temp_project_dir), format="text")

        assert exit_code == 1

    def test_phase_init_returns_2_for_invalid_id(self, temp_project_dir):
        """Test phase-init returns 2 for invalid story ID."""
        from commands.phase_commands import phase_init_command

        exit_code = phase_init_command("INVALID-001", str(temp_project_dir), format="text")

        assert exit_code == 2

    def test_phase_init_json_output(self, temp_project_dir, capsys):
        """Test phase-init with JSON output format."""
        from commands.phase_commands import phase_init_command

        exit_code = phase_init_command("STORY-100", str(temp_project_dir), format="json")

        assert exit_code == 0
        output = capsys.readouterr().out
        result = json.loads(output)
        assert result["success"] is True
        assert result["story_id"] == "STORY-100"
        assert "path" in result


# =============================================================================
# phase-check Tests
# =============================================================================


class TestPhaseCheckCommand:
    """Tests for phase-check command."""

    def test_phase_check_allows_valid_transition(self, temp_project_dir, existing_state):
        """Test phase-check allows valid phase transition."""
        from commands.phase_commands import phase_check_command

        # Current phase is 02, should allow 01->02 check
        exit_code = phase_check_command(
            "STORY-001",
            from_phase="01",
            to_phase="02",
            project_root=str(temp_project_dir),
            format="text"
        )

        assert exit_code == 0

    def test_phase_check_blocks_skip(self, temp_project_dir, existing_state):
        """Test phase-check blocks phase skipping."""
        from commands.phase_commands import phase_check_command

        exit_code = phase_check_command(
            "STORY-001",
            from_phase="02",
            to_phase="04",  # Trying to skip phase 03
            project_root=str(temp_project_dir),
            format="text"
        )

        assert exit_code == 1

    def test_phase_check_reports_missing_subagents(self, temp_project_dir):
        """Test phase-check returns 2 when subagents are missing."""
        from commands.phase_commands import phase_check_command

        # Create state with phase 01 incomplete (missing subagents)
        state = {
            "story_id": "STORY-002",
            "workflow_started": "2025-12-24T10:00:00Z",
            "current_phase": "01",
            "phases": {
                "01": {
                    "status": "completed",
                    "subagents_required": ["git-validator", "tech-stack-detector"],
                    "subagents_invoked": ["git-validator"],  # Missing tech-stack-detector
                    "checkpoint_passed": True
                },
                "02": {"status": "pending", "subagents_required": ["test-automator"], "subagents_invoked": []},
                **{f"{i:02d}": {"status": "pending", "subagents_required": [], "subagents_invoked": []} for i in range(3, 11)}
            },
            "validation_errors": [],
            "blocking_status": False
        }

        state_file = temp_project_dir / "devforgeai" / "workflows" / "STORY-002-phase-state.json"
        state_file.write_text(json.dumps(state, indent=2))

        exit_code = phase_check_command(
            "STORY-002",
            from_phase="01",
            to_phase="02",
            project_root=str(temp_project_dir),
            format="text"
        )

        assert exit_code == 2

    def test_phase_check_json_output(self, temp_project_dir, existing_state, capsys):
        """Test phase-check with JSON output format."""
        from commands.phase_commands import phase_check_command

        exit_code = phase_check_command(
            "STORY-001",
            from_phase="01",
            to_phase="02",
            project_root=str(temp_project_dir),
            format="json"
        )

        assert exit_code == 0
        output = capsys.readouterr().out
        result = json.loads(output)
        assert result["allowed"] is True


# =============================================================================
# phase-complete Tests
# =============================================================================


class TestPhaseCompleteCommand:
    """Tests for phase-complete command."""

    def test_phase_complete_success(self, temp_project_dir, existing_state):
        """Test phase-complete marks phase as complete."""
        from commands.phase_commands import phase_complete_command

        exit_code = phase_complete_command(
            "STORY-001",
            phase="02",
            checkpoint_passed=True,
            project_root=str(temp_project_dir),
            format="text"
        )

        assert exit_code == 0

        # Verify state was updated
        state_file = temp_project_dir / "devforgeai" / "workflows" / "STORY-001-phase-state.json"
        state = json.loads(state_file.read_text())
        assert state["phases"]["02"]["status"] == "completed"
        assert state["current_phase"] == "03"

    def test_phase_complete_returns_1_for_wrong_phase(self, temp_project_dir, existing_state):
        """Test phase-complete returns 1 when completing wrong phase."""
        from commands.phase_commands import phase_complete_command

        # Current phase is 02, can't complete 03
        exit_code = phase_complete_command(
            "STORY-001",
            phase="03",
            checkpoint_passed=True,
            project_root=str(temp_project_dir),
            format="text"
        )

        assert exit_code == 1

    def test_phase_complete_not_found(self, temp_project_dir):
        """Test phase-complete returns 1 for non-existent state."""
        from commands.phase_commands import phase_complete_command

        exit_code = phase_complete_command(
            "STORY-999",
            phase="01",
            checkpoint_passed=True,
            project_root=str(temp_project_dir),
            format="text"
        )

        assert exit_code == 1


# =============================================================================
# phase-status Tests
# =============================================================================


class TestPhaseStatusCommand:
    """Tests for phase-status command."""

    def test_phase_status_success(self, temp_project_dir, existing_state, capsys):
        """Test phase-status displays current state."""
        from commands.phase_commands import phase_status_command

        exit_code = phase_status_command(
            "STORY-001",
            project_root=str(temp_project_dir),
            format="text"
        )

        assert exit_code == 0
        output = capsys.readouterr().out
        assert "STORY-001" in output
        assert "Phase 02" in output or "02" in output

    def test_phase_status_not_found(self, temp_project_dir):
        """Test phase-status returns 1 for non-existent state."""
        from commands.phase_commands import phase_status_command

        exit_code = phase_status_command(
            "STORY-999",
            project_root=str(temp_project_dir),
            format="text"
        )

        assert exit_code == 1

    def test_phase_status_json_output(self, temp_project_dir, existing_state, capsys):
        """Test phase-status with JSON output format."""
        from commands.phase_commands import phase_status_command

        exit_code = phase_status_command(
            "STORY-001",
            project_root=str(temp_project_dir),
            format="json"
        )

        assert exit_code == 0
        output = capsys.readouterr().out
        result = json.loads(output)
        assert result["story_id"] == "STORY-001"
        assert result["current_phase"] == "02"
        assert "phases" in result
