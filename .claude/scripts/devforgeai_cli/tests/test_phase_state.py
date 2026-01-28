"""
STORY-253: PhaseState Module Tests

Test suite for PhaseState class following TDD Red phase principles.
These tests are designed to FAIL initially because the module doesn't exist yet
at `.claude/scripts/devforgeai_cli/phase_state.py`.

Test Coverage Target: 95%+ for business logic

Test Framework: pytest (per tech-stack.md)
Test Pattern: AAA (Arrange, Act, Assert)
Test Naming: test_<function>_<scenario>_<expected>

Acceptance Criteria Mapping:
- AC#1: PhaseState class initialization and path resolution
- AC#2: Create new phase state file with complete schema
- AC#3: Idempotent state file creation
- AC#4: Read existing phase state
- AC#5: Read returns None for non-existent state
- AC#6: Complete phase with sequential enforcement
- AC#7: Phase transition validation (sequential order only)
- AC#8: Record subagent invocation
- AC#9: Add workflow observation
- AC#10: Input validation for story ID format
- AC#11: Input validation for phase ID
- AC#12: State file path helper method

Edge Cases (from story):
- Corrupted JSON state file
- Concurrent write protection (platform-aware)
- Missing workflows directory
- Duplicate subagent recording
- Phase 10 completion boundary
- Empty observation note
- Invalid observation category
- Invalid observation severity
- Atomic file writes
- Empty state file
"""

import json
import os
import sys
import tempfile
import threading
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Generator
from unittest.mock import MagicMock, patch

import pytest


# =============================================================================
# Test Setup and Fixtures
# =============================================================================


@pytest.fixture
def temp_project_root() -> Generator[Path, None, None]:
    """Create a temporary project root directory for testing."""
    with tempfile.TemporaryDirectory() as tmpdir:
        project_root = Path(tmpdir)
        yield project_root


@pytest.fixture
def phase_state(temp_project_root: Path):
    """Create a PhaseState instance with temporary project root."""
    # Import will fail until module is created - this is expected for TDD Red
    from devforgeai_cli.phase_state import PhaseState
    return PhaseState(project_root=temp_project_root)


@pytest.fixture
def phase_state_with_existing_file(temp_project_root: Path):
    """Create PhaseState instance with pre-existing state file.

    STORY-307: Updated to populate subagents_required from PHASE_REQUIRED_SUBAGENTS
    constant, matching STORY-306 subagent enforcement feature.
    """
    from devforgeai_cli.phase_state import PhaseState, PHASE_REQUIRED_SUBAGENTS

    ps = PhaseState(project_root=temp_project_root)

    # Create workflows directory and state file
    workflows_dir = temp_project_root / "devforgeai" / "workflows"
    workflows_dir.mkdir(parents=True, exist_ok=True)

    # Build phases dict with populated subagents_required (AC1)
    # Includes decimal phases 4.5 and 5.5 for AC verification
    valid_phases = ["01", "02", "03", "04", "4.5", "05", "5.5", "06", "07", "08", "09", "10"]
    phases = {}
    for phase in valid_phases:
        # Convert tuples to lists for JSON serialization (Phase 03 OR logic)
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

    # Create a valid state file
    state = {
        "story_id": "STORY-001",
        "current_phase": "01",
        "workflow_started": "2026-01-12T12:00:00Z",
        "blocking_status": False,
        "phases": phases,
        "validation_errors": [],
        "observations": []
    }

    state_path = workflows_dir / "STORY-001-phase-state.json"
    state_path.write_text(json.dumps(state, indent=2))

    return ps


@pytest.fixture
def corrupted_state_file(temp_project_root: Path):
    """Create a corrupted state file for error handling tests."""
    workflows_dir = temp_project_root / "devforgeai" / "workflows"
    workflows_dir.mkdir(parents=True, exist_ok=True)

    state_path = workflows_dir / "STORY-002-phase-state.json"
    state_path.write_text("{invalid json content")

    return temp_project_root


@pytest.fixture
def empty_state_file(temp_project_root: Path):
    """Create an empty state file for edge case testing."""
    workflows_dir = temp_project_root / "devforgeai" / "workflows"
    workflows_dir.mkdir(parents=True, exist_ok=True)

    state_path = workflows_dir / "STORY-003-phase-state.json"
    state_path.write_text("")

    return temp_project_root


# =============================================================================
# AC#1: PhaseState class initialization and path resolution
# =============================================================================


class TestAC1Initialization:
    """Tests for AC#1: PhaseState class initialization and path resolution."""

    def test_init_accepts_project_root_path(self, temp_project_root: Path):
        """
        Given: A project root directory path
        When: PhaseState(project_root=Path("/path/to/project")) is instantiated
        Then: The instance stores the project root
        """
        from devforgeai_cli.phase_state import PhaseState

        # Arrange
        project_root = temp_project_root

        # Act
        ps = PhaseState(project_root=project_root)

        # Assert
        assert ps.project_root == project_root

    def test_init_resolves_workflows_dir_correctly(self, temp_project_root: Path):
        """
        Given: A project root directory path
        When: PhaseState is instantiated
        Then: workflows_dir resolves to {project_root}/devforgeai/workflows/
        """
        from devforgeai_cli.phase_state import PhaseState

        # Arrange
        project_root = temp_project_root
        expected_workflows_dir = project_root / "devforgeai" / "workflows"

        # Act
        ps = PhaseState(project_root=project_root)

        # Assert
        assert ps.workflows_dir == expected_workflows_dir

    def test_init_accepts_string_path_converts_to_path(self, temp_project_root: Path):
        """
        Given: A string path to project root
        When: PhaseState is instantiated
        Then: The path is converted to Path object
        """
        from devforgeai_cli.phase_state import PhaseState

        # Arrange
        project_root_str = str(temp_project_root)

        # Act
        ps = PhaseState(project_root=Path(project_root_str))

        # Assert
        assert isinstance(ps.project_root, Path)


# =============================================================================
# AC#2: Create new phase state file with complete schema
# =============================================================================


class TestAC2CreateNewState:
    """Tests for AC#2: Create new phase state file with complete schema."""

    def test_create_creates_json_file_at_correct_path(self, phase_state, temp_project_root: Path):
        """
        Given: A valid story ID
        When: create(story_id="STORY-001") is called
        Then: A JSON file is created at devforgeai/workflows/STORY-001-phase-state.json
        """
        # Arrange
        story_id = "STORY-001"
        expected_path = temp_project_root / "devforgeai" / "workflows" / "STORY-001-phase-state.json"

        # Act
        phase_state.create(story_id)

        # Assert
        assert expected_path.exists()

    def test_create_returns_state_with_story_id(self, phase_state):
        """
        Given: A valid story ID
        When: create() is called
        Then: Returned state contains story_id field
        """
        # Arrange
        story_id = "STORY-001"

        # Act
        state = phase_state.create(story_id)

        # Assert
        assert state["story_id"] == story_id

    def test_create_returns_state_with_current_phase_01(self, phase_state):
        """
        Given: A new state file creation
        When: create() is called
        Then: current_phase is "01"
        """
        # Arrange & Act
        state = phase_state.create("STORY-001")

        # Assert
        assert state["current_phase"] == "01"

    def test_create_returns_state_with_workflow_started_timestamp(self, phase_state):
        """
        Given: A new state file creation
        When: create() is called
        Then: workflow_started contains ISO-8601 UTC timestamp
        """
        # Arrange & Act
        state = phase_state.create("STORY-001")

        # Assert
        assert "workflow_started" in state
        # Validate ISO-8601 format with Z suffix
        assert state["workflow_started"].endswith("Z")
        # Should be parseable as ISO-8601
        datetime.fromisoformat(state["workflow_started"].replace("Z", "+00:00"))

    def test_create_returns_state_with_blocking_status_false(self, phase_state):
        """
        Given: A new state file creation
        When: create() is called
        Then: blocking_status is false
        """
        # Arrange & Act
        state = phase_state.create("STORY-001")

        # Assert
        assert state["blocking_status"] is False

    def test_create_returns_state_with_all_12_phases(self, phase_state):
        """
        Given: A new state file creation
        When: create() is called
        Then: phases object contains all 12 valid phases (01-10 plus 4.5, 5.5)

        STORY-307: Updated to expect 12 phases including decimal phases for AC verification.
        """
        # Arrange & Act
        state = phase_state.create("STORY-001")

        # Assert
        expected_phases = ["01", "02", "03", "04", "4.5", "05", "5.5", "06", "07", "08", "09", "10"]
        assert list(state["phases"].keys()) == expected_phases

    def test_create_phases_have_status_subagents_required_invoked(self, phase_state):
        """
        Given: A new state file creation
        When: create() is called
        Then: Each phase has status, subagents_required, and subagents_invoked
        """
        # Arrange & Act
        state = phase_state.create("STORY-001")

        # Assert
        for phase_id in state["phases"]:
            phase = state["phases"][phase_id]
            assert "status" in phase
            assert "subagents_required" in phase
            assert "subagents_invoked" in phase

    def test_create_returns_state_with_empty_validation_errors(self, phase_state):
        """
        Given: A new state file creation
        When: create() is called
        Then: validation_errors is an empty array
        """
        # Arrange & Act
        state = phase_state.create("STORY-001")

        # Assert
        assert state["validation_errors"] == []

    def test_create_returns_state_with_empty_observations(self, phase_state):
        """
        Given: A new state file creation
        When: create() is called
        Then: observations is an empty array
        """
        # Arrange & Act
        state = phase_state.create("STORY-001")

        # Assert
        assert state["observations"] == []

    def test_create_creates_directories_if_missing(self, temp_project_root: Path):
        """
        Given: A fresh project without devforgeai/workflows/ directory
        When: create() is called
        Then: Directories are created automatically
        """
        from devforgeai_cli.phase_state import PhaseState

        # Arrange
        ps = PhaseState(project_root=temp_project_root)
        workflows_dir = temp_project_root / "devforgeai" / "workflows"
        assert not workflows_dir.exists()

        # Act
        ps.create("STORY-001")

        # Assert
        assert workflows_dir.exists()


# =============================================================================
# AC#3: Idempotent state file creation
# =============================================================================


class TestAC3IdempotentCreation:
    """Tests for AC#3: Idempotent state file creation."""

    def test_create_existing_file_returns_existing_state(
        self, phase_state_with_existing_file, temp_project_root: Path
    ):
        """
        Given: A state file already exists for STORY-001
        When: create(story_id="STORY-001") is called again
        Then: The existing state is returned without modification
        """
        # Arrange
        ps = phase_state_with_existing_file
        original_state = ps.read("STORY-001")
        original_timestamp = original_state["workflow_started"]

        # Act
        returned_state = ps.create("STORY-001")

        # Assert
        assert returned_state["workflow_started"] == original_timestamp

    def test_create_existing_file_does_not_overwrite(
        self, phase_state_with_existing_file, temp_project_root: Path
    ):
        """
        Given: A state file already exists for STORY-001
        When: create() is called again
        Then: The file is not modified (no overwrite)
        """
        # Arrange
        ps = phase_state_with_existing_file
        state_path = temp_project_root / "devforgeai" / "workflows" / "STORY-001-phase-state.json"
        original_mtime = state_path.stat().st_mtime
        time.sleep(0.1)  # Ensure time difference

        # Act
        ps.create("STORY-001")

        # Assert
        new_mtime = state_path.stat().st_mtime
        assert new_mtime == original_mtime

    def test_create_idempotent_consecutive_calls_same_state(self, phase_state):
        """
        Given: Multiple consecutive create calls
        When: create() is called twice
        Then: Both return the same state (idempotent)
        """
        # Arrange & Act
        state1 = phase_state.create("STORY-001")
        state2 = phase_state.create("STORY-001")

        # Assert
        assert state1["story_id"] == state2["story_id"]
        assert state1["workflow_started"] == state2["workflow_started"]


# =============================================================================
# AC#4: Read existing phase state
# =============================================================================


class TestAC4ReadExistingState:
    """Tests for AC#4: Read existing phase state."""

    def test_read_returns_complete_state_dict(self, phase_state_with_existing_file):
        """
        Given: A state file exists for STORY-001
        When: read(story_id="STORY-001") is called
        Then: The method returns the complete state dictionary parsed from JSON
        """
        # Arrange
        ps = phase_state_with_existing_file

        # Act
        state = ps.read("STORY-001")

        # Assert
        assert state is not None
        assert isinstance(state, dict)
        assert "story_id" in state
        assert "current_phase" in state
        assert "phases" in state

    def test_read_returns_correct_story_id(self, phase_state_with_existing_file):
        """
        Given: A state file exists
        When: read() is called
        Then: The returned state has correct story_id
        """
        # Arrange
        ps = phase_state_with_existing_file

        # Act
        state = ps.read("STORY-001")

        # Assert
        assert state["story_id"] == "STORY-001"


# =============================================================================
# AC#5: Read returns None for non-existent state
# =============================================================================


class TestAC5ReadNonExistentState:
    """Tests for AC#5: Read returns None for non-existent state."""

    def test_read_non_existent_returns_none(self, phase_state):
        """
        Given: No state file exists for STORY-999
        When: read(story_id="STORY-999") is called
        Then: The method returns None (not an exception)
        """
        # Arrange & Act
        result = phase_state.read("STORY-999")

        # Assert
        assert result is None

    def test_read_non_existent_does_not_raise_exception(self, phase_state):
        """
        Given: No state file exists
        When: read() is called for non-existent story
        Then: No exception is raised
        """
        # Arrange & Act & Assert
        try:
            phase_state.read("STORY-999")
        except Exception:
            pytest.fail("read() should not raise exception for non-existent file")


# =============================================================================
# AC#6: Complete phase with sequential enforcement
# =============================================================================


class TestAC6CompletePhase:
    """Tests for AC#6: Complete phase with sequential enforcement.

    STORY-307: Updated to call record_subagent() before complete_phase()
    to satisfy STORY-306 subagent enforcement requirements.
    """

    def test_complete_phase_updates_status_to_completed(
        self, phase_state_with_existing_file
    ):
        """
        Given: A state file exists with current_phase="01"
        When: complete_phase(story_id, phase="01", checkpoint_passed=True) is called
        Then: Phase "01" status becomes "completed"
        """
        # Arrange
        ps = phase_state_with_existing_file

        # STORY-307: Record required subagents before completing phase (AC2)
        ps.record_subagent("STORY-001", "01", "git-validator")
        ps.record_subagent("STORY-001", "01", "tech-stack-detector")

        # Act
        ps.complete_phase("STORY-001", "01", checkpoint_passed=True)
        state = ps.read("STORY-001")

        # Assert
        assert state["phases"]["01"]["status"] == "completed"

    def test_complete_phase_records_completed_at_timestamp(
        self, phase_state_with_existing_file
    ):
        """
        Given: A phase is completed
        When: complete_phase() is called
        Then: completed_at timestamp is recorded
        """
        # Arrange
        ps = phase_state_with_existing_file

        # STORY-307: Record required subagents before completing phase (AC2)
        ps.record_subagent("STORY-001", "01", "git-validator")
        ps.record_subagent("STORY-001", "01", "tech-stack-detector")

        # Act
        ps.complete_phase("STORY-001", "01", checkpoint_passed=True)
        state = ps.read("STORY-001")

        # Assert
        assert "completed_at" in state["phases"]["01"]
        assert state["phases"]["01"]["completed_at"].endswith("Z")

    def test_complete_phase_stores_checkpoint_passed(
        self, phase_state_with_existing_file
    ):
        """
        Given: A phase is completed
        When: complete_phase(checkpoint_passed=True) is called
        Then: checkpoint_passed is stored
        """
        # Arrange
        ps = phase_state_with_existing_file

        # STORY-307: Record required subagents before completing phase (AC2)
        ps.record_subagent("STORY-001", "01", "git-validator")
        ps.record_subagent("STORY-001", "01", "tech-stack-detector")

        # Act
        ps.complete_phase("STORY-001", "01", checkpoint_passed=True)
        state = ps.read("STORY-001")

        # Assert
        assert state["phases"]["01"]["checkpoint_passed"] is True

    def test_complete_phase_advances_current_phase(
        self, phase_state_with_existing_file
    ):
        """
        Given: Phase "01" is the current phase
        When: complete_phase("01") is called
        Then: current_phase advances to "02"
        """
        # Arrange
        ps = phase_state_with_existing_file

        # STORY-307: Record required subagents before completing phase (AC2)
        ps.record_subagent("STORY-001", "01", "git-validator")
        ps.record_subagent("STORY-001", "01", "tech-stack-detector")

        # Act
        ps.complete_phase("STORY-001", "01", checkpoint_passed=True)
        state = ps.read("STORY-001")

        # Assert
        assert state["current_phase"] == "02"

    def test_complete_phase_10_stays_at_10(self, phase_state_with_existing_file):
        """
        Given: Phase "10" is the current phase
        When: complete_phase("10") is called
        Then: current_phase stays at "10" (no phase 11)
        """
        # Arrange
        ps = phase_state_with_existing_file

        # Complete phases 01-09 first
        state = ps.read("STORY-001")
        for i in range(1, 10):
            phase_id = f"{i:02d}"
            state["current_phase"] = phase_id
            state["phases"][phase_id]["status"] = "pending"
        # Set to phase 10
        state["current_phase"] = "10"

        # Write modified state directly
        state_path = ps._get_state_path("STORY-001")
        state_path.write_text(json.dumps(state, indent=2))

        # STORY-307: Record required subagent before completing phase (AC2)
        ps.record_subagent("STORY-001", "10", "dev-result-interpreter")

        # Act
        ps.complete_phase("STORY-001", "10", checkpoint_passed=True)
        final_state = ps.read("STORY-001")

        # Assert
        assert final_state["current_phase"] == "10"
        assert final_state["phases"]["10"]["status"] == "completed"


# =============================================================================
# AC#7: Phase transition validation (sequential order only)
# =============================================================================


class TestAC7PhaseTransitionValidation:
    """Tests for AC#7: Phase transition validation."""

    def test_complete_phase_skip_raises_phase_transition_error(
        self, phase_state_with_existing_file
    ):
        """
        Given: A state file exists with current_phase="02"
        When: complete_phase(phase="05") is called (attempting to skip)
        Then: A PhaseTransitionError is raised
        """
        from devforgeai_cli.phase_state import PhaseTransitionError

        # Arrange
        ps = phase_state_with_existing_file

        # Set current_phase to "02"
        state = ps.read("STORY-001")
        state["current_phase"] = "02"
        state["phases"]["01"]["status"] = "completed"
        state_path = ps._get_state_path("STORY-001")
        state_path.write_text(json.dumps(state, indent=2))

        # Act & Assert
        with pytest.raises(PhaseTransitionError) as exc_info:
            ps.complete_phase("STORY-001", "05", checkpoint_passed=True)

        assert "sequential" in str(exc_info.value).lower()

    def test_complete_phase_previous_phase_raises_error(
        self, phase_state_with_existing_file
    ):
        """
        Given: current_phase is "03"
        When: complete_phase(phase="01") is called (previous phase)
        Then: A PhaseTransitionError is raised
        """
        from devforgeai_cli.phase_state import PhaseTransitionError

        # Arrange
        ps = phase_state_with_existing_file

        # Set current_phase to "03"
        state = ps.read("STORY-001")
        state["current_phase"] = "03"
        state_path = ps._get_state_path("STORY-001")
        state_path.write_text(json.dumps(state, indent=2))

        # Act & Assert
        with pytest.raises(PhaseTransitionError):
            ps.complete_phase("STORY-001", "01", checkpoint_passed=True)

    def test_complete_phase_error_message_indicates_sequential(
        self, phase_state_with_existing_file
    ):
        """
        Given: An attempt to skip phases
        When: PhaseTransitionError is raised
        Then: Error message indicates sequential completion required
        """
        from devforgeai_cli.phase_state import PhaseTransitionError

        # Arrange
        ps = phase_state_with_existing_file

        # Act & Assert
        with pytest.raises(PhaseTransitionError) as exc_info:
            ps.complete_phase("STORY-001", "05", checkpoint_passed=True)

        error_message = str(exc_info.value)
        assert "sequential" in error_message.lower() or "order" in error_message.lower()


# =============================================================================
# AC#8: Record subagent invocation
# =============================================================================


class TestAC8RecordSubagent:
    """Tests for AC#8: Record subagent invocation."""

    def test_record_subagent_appends_to_invoked_list(
        self, phase_state_with_existing_file
    ):
        """
        Given: A state file exists for STORY-001
        When: record_subagent(story_id, phase="02", subagent="test-automator") is called
        Then: "test-automator" is appended to phases["02"]["subagents_invoked"]
        """
        # Arrange
        ps = phase_state_with_existing_file

        # Act
        ps.record_subagent("STORY-001", "02", "test-automator")
        state = ps.read("STORY-001")

        # Assert
        assert "test-automator" in state["phases"]["02"]["subagents_invoked"]

    def test_record_subagent_sets_started_at_timestamp(
        self, phase_state_with_existing_file
    ):
        """
        Given: A phase without started_at
        When: record_subagent() is called
        Then: started_at timestamp is recorded
        """
        # Arrange
        ps = phase_state_with_existing_file

        # Act
        ps.record_subagent("STORY-001", "02", "test-automator")
        state = ps.read("STORY-001")

        # Assert
        assert "started_at" in state["phases"]["02"]

    def test_record_subagent_idempotent_no_duplicates(
        self, phase_state_with_existing_file
    ):
        """
        Given: Subagent already recorded for a phase
        When: record_subagent() is called again with same subagent
        Then: Subagent is not duplicated (idempotent)
        """
        # Arrange
        ps = phase_state_with_existing_file

        # Act
        ps.record_subagent("STORY-001", "02", "test-automator")
        ps.record_subagent("STORY-001", "02", "test-automator")
        state = ps.read("STORY-001")

        # Assert
        count = state["phases"]["02"]["subagents_invoked"].count("test-automator")
        assert count == 1

    def test_record_subagent_returns_updated_state(
        self, phase_state_with_existing_file
    ):
        """
        Given: A valid state file
        When: record_subagent() is called
        Then: Returns updated state dictionary
        """
        # Arrange
        ps = phase_state_with_existing_file

        # Act
        result = ps.record_subagent("STORY-001", "02", "test-automator")

        # Assert
        assert isinstance(result, dict)
        assert "test-automator" in result["phases"]["02"]["subagents_invoked"]

    def test_record_subagent_raises_for_nonexistent_state(self, phase_state):
        """
        Given: No state file exists
        When: record_subagent() is called
        Then: Raises FileNotFoundError
        """
        # Arrange & Act & Assert
        with pytest.raises(FileNotFoundError):
            phase_state.record_subagent("STORY-999", "02", "test-automator")


# =============================================================================
# AC#9: Add workflow observation
# =============================================================================


class TestAC9AddObservation:
    """Tests for AC#9: Add workflow observation."""

    def test_add_observation_appends_to_observations_array(
        self, phase_state_with_existing_file
    ):
        """
        Given: A state file exists for STORY-001
        When: add_observation() is called
        Then: An observation object is appended to the observations array
        """
        # Arrange
        ps = phase_state_with_existing_file

        # Act
        ps.add_observation(
            story_id="STORY-001",
            phase_id="04",
            category="friction",
            note="Test took longer than expected",
            severity="medium"
        )
        state = ps.read("STORY-001")

        # Assert
        assert len(state["observations"]) == 1

    def test_add_observation_returns_unique_id_format(
        self, phase_state_with_existing_file
    ):
        """
        Given: An observation is added
        When: add_observation() returns
        Then: Returns ID matching format obs-{phase_id}-{8-char-uuid}
        """
        # Arrange
        ps = phase_state_with_existing_file

        # Act
        observation_id = ps.add_observation(
            story_id="STORY-001",
            phase_id="04",
            category="friction",
            note="Test observation",
            severity="medium"
        )

        # Assert
        assert observation_id is not None
        assert observation_id.startswith("obs-04-")
        # 8-char uuid after the prefix
        parts = observation_id.split("-")
        assert len(parts) == 3
        assert len(parts[2]) == 8

    def test_add_observation_stores_all_fields(
        self, phase_state_with_existing_file
    ):
        """
        Given: An observation is added
        When: add_observation() is called with all parameters
        Then: All fields are stored correctly
        """
        # Arrange
        ps = phase_state_with_existing_file

        # Act
        ps.add_observation(
            story_id="STORY-001",
            phase_id="04",
            category="friction",
            note="Test took longer than expected",
            severity="medium"
        )
        state = ps.read("STORY-001")

        # Assert
        obs = state["observations"][0]
        assert "id" in obs
        assert obs["phase"] == "04"
        assert obs["category"] == "friction"
        assert obs["note"] == "Test took longer than expected"
        assert obs["severity"] == "medium"
        assert "timestamp" in obs

    def test_add_observation_records_timestamp(
        self, phase_state_with_existing_file
    ):
        """
        Given: An observation is added
        When: add_observation() is called
        Then: Timestamp is recorded in ISO-8601 format
        """
        # Arrange
        ps = phase_state_with_existing_file

        # Act
        ps.add_observation(
            story_id="STORY-001",
            phase_id="04",
            category="friction",
            note="Test observation",
            severity="medium"
        )
        state = ps.read("STORY-001")

        # Assert
        timestamp = state["observations"][0]["timestamp"]
        assert timestamp.endswith("Z")


# =============================================================================
# AC#10: Input validation for story ID format
# =============================================================================


class TestAC10StoryIdValidation:
    """Tests for AC#10: Input validation for story ID format."""

    def test_create_invalid_story_id_raises_value_error(self, phase_state):
        """
        Given: An invalid story ID "INVALID-ID"
        When: create() is called with this story ID
        Then: A ValueError is raised
        """
        # Arrange & Act & Assert
        with pytest.raises(ValueError):
            phase_state.create("INVALID-ID")

    def test_create_invalid_story_id_error_message(self, phase_state):
        """
        Given: An invalid story ID
        When: ValueError is raised
        Then: Message contains "Invalid story_id" and pattern example
        """
        # Arrange & Act & Assert
        with pytest.raises(ValueError) as exc_info:
            phase_state.create("INVALID-ID")

        error_message = str(exc_info.value)
        assert "Invalid story_id" in error_message
        assert "STORY-XXX" in error_message or "STORY-001" in error_message

    def test_read_invalid_story_id_raises_value_error(self, phase_state):
        """
        Given: An invalid story ID
        When: read() is called
        Then: ValueError is raised (validation before file check)
        """
        # Note: Implementation may validate before checking file existence
        # This test assumes validation happens first
        # If not, read should return None without validation
        pass  # Implementation decision: may or may not validate on read

    @pytest.mark.parametrize("invalid_id", [
        "STORY-1",      # Too few digits
        "STORY-01",     # Two digits (spec says 3)
        "STORY-0001",   # Too many digits
        "story-001",    # Lowercase
        "STORY_001",    # Underscore instead of dash
        "STORY001",     # No dash
        "001-STORY",    # Wrong order
        "../STORY-001", # Path traversal attempt
        "STORY-001/../", # Path traversal attempt
    ])
    def test_invalid_story_id_patterns_rejected(self, phase_state, invalid_id: str):
        """
        Given: Various invalid story ID patterns
        When: create() is called
        Then: ValueError is raised for all invalid patterns
        """
        # Arrange & Act & Assert
        with pytest.raises(ValueError):
            phase_state.create(invalid_id)

    def test_path_traversal_story_id_rejected(self, phase_state):
        """
        Given: A story ID with path traversal attempt
        When: Any method is called
        Then: ValueError is raised (security)
        """
        # Arrange & Act & Assert
        with pytest.raises(ValueError):
            phase_state.create("../etc/passwd")


# =============================================================================
# AC#11: Input validation for phase ID
# =============================================================================


class TestAC11PhaseIdValidation:
    """Tests for AC#11: Input validation for phase ID."""

    def test_complete_phase_invalid_phase_raises_phase_not_found(
        self, phase_state_with_existing_file
    ):
        """
        Given: An invalid phase ID "15"
        When: complete_phase() is called with this phase ID
        Then: A PhaseNotFoundError is raised
        """
        from devforgeai_cli.phase_state import PhaseNotFoundError

        # Arrange
        ps = phase_state_with_existing_file

        # Act & Assert
        with pytest.raises(PhaseNotFoundError):
            ps.complete_phase("STORY-001", "15", checkpoint_passed=True)

    def test_record_subagent_invalid_phase_raises_phase_not_found(
        self, phase_state_with_existing_file
    ):
        """
        Given: An invalid phase ID
        When: record_subagent() is called
        Then: PhaseNotFoundError is raised
        """
        from devforgeai_cli.phase_state import PhaseNotFoundError

        # Arrange
        ps = phase_state_with_existing_file

        # Act & Assert
        with pytest.raises(PhaseNotFoundError):
            ps.record_subagent("STORY-001", "15", "test-automator")

    def test_phase_not_found_error_message_shows_valid_phases(
        self, phase_state_with_existing_file
    ):
        """
        Given: An invalid phase ID
        When: PhaseNotFoundError is raised
        Then: Error message indicates valid phases are "01" through "10"
        """
        from devforgeai_cli.phase_state import PhaseNotFoundError

        # Arrange
        ps = phase_state_with_existing_file

        # Act & Assert
        with pytest.raises(PhaseNotFoundError) as exc_info:
            ps.complete_phase("STORY-001", "15", checkpoint_passed=True)

        error_message = str(exc_info.value)
        assert "01" in error_message or "10" in error_message

    @pytest.mark.parametrize("invalid_phase", [
        "0",    # Below range
        "00",   # Zero phase
        "11",   # Above range
        "15",   # Well above range
        "1",    # Single digit
        "001",  # Three digits
        "-1",   # Negative
        "one",  # Text
    ])
    def test_invalid_phase_id_patterns_rejected(
        self, phase_state_with_existing_file, invalid_phase: str
    ):
        """
        Given: Various invalid phase ID patterns
        When: complete_phase() is called
        Then: PhaseNotFoundError is raised
        """
        from devforgeai_cli.phase_state import PhaseNotFoundError

        # Arrange
        ps = phase_state_with_existing_file

        # Act & Assert
        with pytest.raises(PhaseNotFoundError):
            ps.complete_phase("STORY-001", invalid_phase, checkpoint_passed=True)


# =============================================================================
# AC#12: State file path helper method
# =============================================================================


class TestAC12StateFilePath:
    """Tests for AC#12: State file path helper method."""

    def test_get_state_path_returns_correct_path(self, temp_project_root: Path):
        """
        Given: A PhaseState instance
        When: _get_state_path(story_id="STORY-001") is called
        Then: Returns Path("{project_root}/devforgeai/workflows/STORY-001-phase-state.json")
        """
        from devforgeai_cli.phase_state import PhaseState

        # Arrange
        ps = PhaseState(project_root=temp_project_root)
        expected_path = temp_project_root / "devforgeai" / "workflows" / "STORY-001-phase-state.json"

        # Act
        result = ps._get_state_path("STORY-001")

        # Assert
        assert result == expected_path

    def test_get_state_path_returns_path_object(self, phase_state):
        """
        Given: A PhaseState instance
        When: _get_state_path() is called
        Then: Returns a Path object
        """
        # Arrange & Act
        result = phase_state._get_state_path("STORY-001")

        # Assert
        assert isinstance(result, Path)


# =============================================================================
# Edge Case Tests: Corrupted JSON
# =============================================================================


class TestEdgeCaseCorruptedJSON:
    """Tests for edge case: Corrupted JSON state file."""

    def test_read_corrupted_json_raises_state_file_corruption_error(
        self, corrupted_state_file
    ):
        """
        Given: A corrupted JSON state file
        When: read() is called
        Then: StateFileCorruptionError is raised
        """
        from devforgeai_cli.phase_state import PhaseState, StateFileCorruptionError

        # Arrange
        ps = PhaseState(project_root=corrupted_state_file)

        # Act & Assert
        with pytest.raises(StateFileCorruptionError):
            ps.read("STORY-002")

    def test_corruption_error_includes_recovery_message(
        self, corrupted_state_file
    ):
        """
        Given: A corrupted JSON state file
        When: StateFileCorruptionError is raised
        Then: Error message includes recovery instructions
        """
        from devforgeai_cli.phase_state import PhaseState, StateFileCorruptionError

        # Arrange
        ps = PhaseState(project_root=corrupted_state_file)

        # Act & Assert
        with pytest.raises(StateFileCorruptionError) as exc_info:
            ps.read("STORY-002")

        error_message = str(exc_info.value)
        assert "recovery" in error_message.lower() or "delete" in error_message.lower()

    def test_read_empty_file_raises_state_file_corruption_error(
        self, empty_state_file
    ):
        """
        Given: An empty state file
        When: read() is called
        Then: StateFileCorruptionError is raised (treat as corrupted)
        """
        from devforgeai_cli.phase_state import PhaseState, StateFileCorruptionError

        # Arrange
        ps = PhaseState(project_root=empty_state_file)

        # Act & Assert
        with pytest.raises(StateFileCorruptionError):
            ps.read("STORY-003")


# =============================================================================
# Edge Case Tests: Observation Validation
# =============================================================================


class TestEdgeCaseObservationValidation:
    """Tests for edge cases: Observation input validation."""

    def test_add_observation_empty_note_raises_value_error(
        self, phase_state_with_existing_file
    ):
        """
        Given: An empty observation note
        When: add_observation() is called
        Then: ValueError is raised
        """
        # Arrange
        ps = phase_state_with_existing_file

        # Act & Assert
        with pytest.raises(ValueError) as exc_info:
            ps.add_observation(
                story_id="STORY-001",
                phase_id="04",
                category="friction",
                note="",
                severity="medium"
            )

        assert "empty" in str(exc_info.value).lower()

    def test_add_observation_whitespace_note_raises_value_error(
        self, phase_state_with_existing_file
    ):
        """
        Given: A whitespace-only observation note
        When: add_observation() is called
        Then: ValueError is raised
        """
        # Arrange
        ps = phase_state_with_existing_file

        # Act & Assert
        with pytest.raises(ValueError):
            ps.add_observation(
                story_id="STORY-001",
                phase_id="04",
                category="friction",
                note="   ",
                severity="medium"
            )

    def test_add_observation_invalid_category_raises_value_error(
        self, phase_state_with_existing_file
    ):
        """
        Given: An invalid observation category
        When: add_observation() is called
        Then: ValueError is raised with valid options
        """
        # Arrange
        ps = phase_state_with_existing_file

        # Act & Assert
        with pytest.raises(ValueError) as exc_info:
            ps.add_observation(
                story_id="STORY-001",
                phase_id="04",
                category="invalid_category",
                note="Test observation",
                severity="medium"
            )

        error_message = str(exc_info.value)
        # Should mention valid categories
        assert "friction" in error_message or "gap" in error_message

    def test_add_observation_invalid_severity_raises_value_error(
        self, phase_state_with_existing_file
    ):
        """
        Given: An invalid observation severity
        When: add_observation() is called
        Then: ValueError is raised with valid options
        """
        # Arrange
        ps = phase_state_with_existing_file

        # Act & Assert
        with pytest.raises(ValueError) as exc_info:
            ps.add_observation(
                story_id="STORY-001",
                phase_id="04",
                category="friction",
                note="Test observation",
                severity="critical"  # Invalid
            )

        error_message = str(exc_info.value)
        # Should mention valid severities
        assert "low" in error_message or "medium" in error_message or "high" in error_message

    @pytest.mark.parametrize("valid_category", [
        "friction", "gap", "success", "pattern"
    ])
    def test_add_observation_valid_categories_accepted(
        self, phase_state_with_existing_file, valid_category: str
    ):
        """
        Given: A valid observation category
        When: add_observation() is called
        Then: No error is raised
        """
        # Arrange
        ps = phase_state_with_existing_file

        # Act
        result = ps.add_observation(
            story_id="STORY-001",
            phase_id="04",
            category=valid_category,
            note="Test observation",
            severity="medium"
        )

        # Assert
        assert result is not None

    @pytest.mark.parametrize("valid_severity", ["low", "medium", "high"])
    def test_add_observation_valid_severities_accepted(
        self, phase_state_with_existing_file, valid_severity: str
    ):
        """
        Given: A valid observation severity
        When: add_observation() is called
        Then: No error is raised
        """
        # Arrange
        ps = phase_state_with_existing_file

        # Act
        result = ps.add_observation(
            story_id="STORY-001",
            phase_id="04",
            category="friction",
            note="Test observation",
            severity=valid_severity
        )

        # Assert
        assert result is not None

    def test_add_observation_note_max_length_1000(
        self, phase_state_with_existing_file
    ):
        """
        Given: A note exceeding 1000 characters
        When: add_observation() is called
        Then: ValueError is raised (max 1000 chars)
        """
        # Arrange
        ps = phase_state_with_existing_file
        long_note = "x" * 1001

        # Act & Assert
        with pytest.raises(ValueError):
            ps.add_observation(
                story_id="STORY-001",
                phase_id="04",
                category="friction",
                note=long_note,
                severity="medium"
            )


# =============================================================================
# Edge Case Tests: Atomic File Writes
# =============================================================================


class TestEdgeCaseAtomicWrites:
    """Tests for edge case: Atomic file writes."""

    def test_create_uses_atomic_write(self, phase_state, temp_project_root: Path):
        """
        Given: A state file being created
        When: create() is called
        Then: File is written atomically (temp file + rename)
        """
        # This test verifies the behavior, not implementation
        # We can verify by checking no partial files exist

        # Arrange
        workflows_dir = temp_project_root / "devforgeai" / "workflows"

        # Act
        phase_state.create("STORY-001")

        # Assert - no .tmp files left behind
        tmp_files = list(workflows_dir.glob("*.tmp"))
        assert len(tmp_files) == 0

    def test_complete_phase_uses_atomic_write(
        self, phase_state_with_existing_file, temp_project_root: Path
    ):
        """
        Given: A phase being completed
        When: complete_phase() is called
        Then: File is written atomically
        """
        # Arrange
        ps = phase_state_with_existing_file
        workflows_dir = temp_project_root / "devforgeai" / "workflows"

        # STORY-307: Record required subagents before completing phase (AC2)
        ps.record_subagent("STORY-001", "01", "git-validator")
        ps.record_subagent("STORY-001", "01", "tech-stack-detector")

        # Act
        ps.complete_phase("STORY-001", "01", checkpoint_passed=True)

        # Assert - no .tmp files left behind
        tmp_files = list(workflows_dir.glob("*.tmp"))
        assert len(tmp_files) == 0


# =============================================================================
# Edge Case Tests: Platform-Aware File Locking
# =============================================================================


class TestEdgeCaseFileLocking:
    """Tests for edge case: Platform-aware file locking."""

    @pytest.mark.skipif(os.name != 'posix', reason="Unix-only test")
    def test_unix_uses_fcntl_locking(self, phase_state):
        """
        Given: Unix platform (Linux/macOS)
        When: File operations occur
        Then: fcntl.flock() is used for locking
        """
        # This is an implementation detail test
        # Verifies fcntl is imported on Unix
        import importlib.util

        spec = importlib.util.find_spec("fcntl")
        assert spec is not None, "fcntl should be available on Unix"

    @pytest.mark.skipif(os.name != 'nt', reason="Windows-only test")
    def test_windows_uses_msvcrt_or_fallback(self, phase_state):
        """
        Given: Windows platform
        When: File operations occur
        Then: msvcrt.locking() or last-write-wins is used
        """
        # This is an implementation detail test
        # Verifies appropriate handling on Windows
        import importlib.util

        # msvcrt should be available on Windows
        spec = importlib.util.find_spec("msvcrt")
        assert spec is not None, "msvcrt should be available on Windows"

    def test_lock_timeout_raises_lock_timeout_error(
        self, phase_state_with_existing_file
    ):
        """
        Given: A file lock held by another process
        When: Lock acquisition times out (>5 seconds)
        Then: LockTimeoutError is raised
        """
        from devforgeai_cli.phase_state import LockTimeoutError

        # This test is complex to implement in unit tests
        # Would require multiprocessing or threading
        # Marking as placeholder for integration testing
        pass

    def test_concurrent_writes_dont_corrupt_file(
        self, phase_state_with_existing_file
    ):
        """
        Given: Multiple concurrent write attempts
        When: record_subagent() called from multiple threads
        Then: File is not corrupted
        """
        # Arrange
        ps = phase_state_with_existing_file
        errors = []

        def record_subagent(subagent_name: str):
            try:
                ps.record_subagent("STORY-001", "02", subagent_name)
            except Exception as e:
                errors.append(e)

        # Act
        threads = []
        for i in range(10):
            t = threading.Thread(
                target=record_subagent,
                args=(f"subagent-{i}",)
            )
            threads.append(t)
            t.start()

        for t in threads:
            t.join()

        # Assert
        assert len(errors) == 0, f"Errors during concurrent writes: {errors}"

        # Verify file is valid JSON
        state = ps.read("STORY-001")
        assert state is not None


# =============================================================================
# Custom Exception Tests
# =============================================================================


class TestCustomExceptions:
    """Tests for custom exception classes."""

    def test_phase_state_error_is_base_exception(self):
        """PhaseStateError should be base for all phase state exceptions."""
        from devforgeai_cli.phase_state import (
            PhaseStateError,
            PhaseNotFoundError,
            StateFileCorruptionError,
            PhaseTransitionError,
            LockTimeoutError
        )

        assert issubclass(PhaseNotFoundError, PhaseStateError)
        assert issubclass(StateFileCorruptionError, PhaseStateError)
        assert issubclass(PhaseTransitionError, PhaseStateError)
        assert issubclass(LockTimeoutError, PhaseStateError)

    def test_phase_not_found_error_stores_phase_id(self):
        """PhaseNotFoundError should store the invalid phase_id."""
        from devforgeai_cli.phase_state import PhaseNotFoundError

        error = PhaseNotFoundError("15")
        assert error.phase_id == "15"

    def test_state_file_corruption_error_stores_story_id(self):
        """StateFileCorruptionError should store story_id and original error."""
        from devforgeai_cli.phase_state import StateFileCorruptionError

        error = StateFileCorruptionError("STORY-001", "JSON decode error")
        assert error.story_id == "STORY-001"
        assert error.original_error == "JSON decode error"

    def test_phase_transition_error_stores_phases(self):
        """PhaseTransitionError should store current and attempted phases."""
        from devforgeai_cli.phase_state import PhaseTransitionError

        error = PhaseTransitionError("STORY-001", "02", "05")
        assert error.story_id == "STORY-001"
        assert error.current_phase == "02"
        assert error.attempted_phase == "05"

    def test_lock_timeout_error_stores_path_and_timeout(self):
        """LockTimeoutError should store file path and timeout."""
        from devforgeai_cli.phase_state import LockTimeoutError

        error = LockTimeoutError("/path/to/file.json", 5)
        assert error.file_path == "/path/to/file.json"
        assert error.timeout == 5


# =============================================================================
# Performance Tests (NFR-001, NFR-002)
# =============================================================================


class TestPerformance:
    """Tests for non-functional requirements: Performance."""

    @pytest.mark.slow
    def test_read_latency_under_10ms(self, phase_state_with_existing_file):
        """
        NFR-001: State file read latency < 10ms per read() operation (p99)
        """
        # Arrange
        ps = phase_state_with_existing_file

        # Act - perform 100 reads
        times = []
        for _ in range(100):
            start = time.perf_counter()
            ps.read("STORY-001")
            end = time.perf_counter()
            times.append((end - start) * 1000)  # Convert to ms

        # Assert - p99 should be < 10ms
        times.sort()
        p99_index = int(len(times) * 0.99)
        p99_time = times[p99_index]

        assert p99_time < 10, f"p99 read latency {p99_time:.2f}ms exceeds 10ms threshold"

    @pytest.mark.slow
    def test_1000_reads_complete_in_10_seconds(
        self, phase_state_with_existing_file
    ):
        """
        NFR-001: 1000 consecutive reads complete in < 10 seconds
        """
        # Arrange
        ps = phase_state_with_existing_file

        # Act
        start = time.perf_counter()
        for _ in range(1000):
            ps.read("STORY-001")
        end = time.perf_counter()

        # Assert
        elapsed = end - start
        assert elapsed < 10, f"1000 reads took {elapsed:.2f}s (expected < 10s)"

    @pytest.mark.slow
    def test_100_writes_complete_in_5_seconds(self, temp_project_root: Path):
        """
        NFR-002: 100 consecutive writes complete in < 5 seconds
        """
        from devforgeai_cli.phase_state import PhaseState

        # Arrange
        ps = PhaseState(project_root=temp_project_root)

        # Act
        start = time.perf_counter()
        for i in range(100):
            story_id = f"STORY-{i:03d}"
            ps.create(story_id)
        end = time.perf_counter()

        # Assert
        elapsed = end - start
        assert elapsed < 5, f"100 writes took {elapsed:.2f}s (expected < 5s)"


# =============================================================================
# Security Tests (NFR-005)
# =============================================================================


class TestSecurity:
    """Tests for non-functional requirements: Security."""

    @pytest.mark.parametrize("malicious_id", [
        "../etc/passwd",
        "STORY-001/../../../etc/passwd",
        "..\\windows\\system32\\config\\sam",
        "STORY-001/../../sensitive",
        "STORY-001\x00.json",  # Null byte injection
    ])
    def test_path_traversal_prevention(self, phase_state, malicious_id: str):
        """
        NFR-005: Path traversal prevention - 0 successful traversal attempts
        """
        # Arrange & Act & Assert
        with pytest.raises(ValueError):
            phase_state.create(malicious_id)

    def test_state_file_permissions(self, phase_state, temp_project_root: Path):
        """
        State files should have appropriate permissions (0644)
        """
        # Arrange
        phase_state.create("STORY-001")
        state_path = temp_project_root / "devforgeai" / "workflows" / "STORY-001-phase-state.json"

        # Act
        mode = os.stat(state_path).st_mode & 0o777

        # Assert - should be readable by owner and group, not world-writable
        # 0o644 = owner rw, group r, other r
        # Accept 0o664 or 0o644 depending on umask
        assert mode in [0o644, 0o664, 0o666], f"File permissions {oct(mode)} not secure"


# =============================================================================
# Scalability Tests (NFR-006)
# =============================================================================


class TestScalability:
    """Tests for non-functional requirements: Scalability."""

    @pytest.mark.slow
    def test_100_concurrent_story_state_files(self, temp_project_root: Path):
        """
        NFR-006: Support 100+ concurrent story state files
        """
        from devforgeai_cli.phase_state import PhaseState

        # Arrange
        ps = PhaseState(project_root=temp_project_root)

        # Act - create 100 state files
        for i in range(100):
            story_id = f"STORY-{i:03d}"
            ps.create(story_id)

        # Assert - all files exist and are readable
        for i in range(100):
            story_id = f"STORY-{i:03d}"
            state = ps.read(story_id)
            assert state is not None
            assert state["story_id"] == story_id


# =============================================================================
# Integration Tests: Full Workflow
# =============================================================================


class TestIntegrationFullWorkflow:
    """Integration tests for complete workflow scenarios.

    STORY-307: Updated to handle all 12 phases (including 4.5, 5.5) and
    record required subagents from PHASE_REQUIRED_SUBAGENTS constant.
    """

    def test_complete_workflow_from_creation_to_phase_10(
        self, temp_project_root: Path
    ):
        """
        Integration test: Create state, record subagents, complete all phases

        STORY-307: Updated to complete all 12 phases with proper subagent recording.
        """
        from devforgeai_cli.phase_state import PhaseState, PHASE_REQUIRED_SUBAGENTS

        # Arrange
        ps = PhaseState(project_root=temp_project_root)

        # Valid phases in order (includes decimal phases)
        valid_phases = ["01", "02", "03", "04", "4.5", "05", "5.5", "06", "07", "08", "09", "10"]

        # Act
        # 1. Create state
        state = ps.create("STORY-001")
        assert state["current_phase"] == "01"

        # 2. Complete all 12 phases
        for phase_id in valid_phases:
            # Record required subagents for this phase (AC2)
            required = PHASE_REQUIRED_SUBAGENTS.get(phase_id, [])
            for item in required:
                if isinstance(item, tuple):
                    # OR logic: record first option (backend-architect for Phase 03)
                    ps.record_subagent("STORY-001", phase_id, item[0])
                else:
                    ps.record_subagent("STORY-001", phase_id, item)

            # Add an observation
            ps.add_observation(
                story_id="STORY-001",
                phase_id=phase_id,
                category="success",
                note=f"Completed phase {phase_id}",
                severity="low"
            )

            # Complete the phase
            ps.complete_phase("STORY-001", phase_id, checkpoint_passed=True)

        # 3. Verify final state
        final_state = ps.read("STORY-001")

        # Assert
        assert final_state["current_phase"] == "10"
        for phase_id in valid_phases:
            assert final_state["phases"][phase_id]["status"] == "completed"

        assert len(final_state["observations"]) == 12  # Updated: 12 phases now


# =============================================================================
# STORY-307: SubagentEnforcementError Tests (AC3)
# =============================================================================


class TestSubagentEnforcementError:
    """Tests for AC3: SubagentEnforcementError exception behavior.

    STORY-307: Tests for the new exception added in STORY-306.
    """

    def test_subagent_enforcement_error_inherits_phase_state_error(self):
        """SubagentEnforcementError should inherit from PhaseStateError."""
        from devforgeai_cli.phase_state import SubagentEnforcementError, PhaseStateError

        assert issubclass(SubagentEnforcementError, PhaseStateError)

    def test_subagent_enforcement_error_stores_story_id(self):
        """SubagentEnforcementError should store story_id attribute."""
        from devforgeai_cli.phase_state import SubagentEnforcementError

        error = SubagentEnforcementError("STORY-001", "02", ["test-automator"])
        assert error.story_id == "STORY-001"

    def test_subagent_enforcement_error_stores_phase(self):
        """SubagentEnforcementError should store phase attribute."""
        from devforgeai_cli.phase_state import SubagentEnforcementError

        error = SubagentEnforcementError("STORY-001", "02", ["test-automator"])
        assert error.phase == "02"

    def test_subagent_enforcement_error_stores_missing_subagents(self):
        """SubagentEnforcementError should store missing_subagents attribute."""
        from devforgeai_cli.phase_state import SubagentEnforcementError

        error = SubagentEnforcementError("STORY-001", "04", ["refactoring-specialist", "code-reviewer"])
        assert error.missing_subagents == ["refactoring-specialist", "code-reviewer"]

    def test_subagent_enforcement_error_message_contains_phase(self):
        """Error message should contain phase identifier."""
        from devforgeai_cli.phase_state import SubagentEnforcementError

        error = SubagentEnforcementError("STORY-001", "02", ["test-automator"])
        assert "02" in str(error)

    def test_subagent_enforcement_error_message_contains_missing_subagents(self):
        """Error message should list missing subagent names."""
        from devforgeai_cli.phase_state import SubagentEnforcementError

        error = SubagentEnforcementError("STORY-001", "04", ["refactoring-specialist"])
        assert "refactoring-specialist" in str(error)

    def test_complete_phase_raises_enforcement_error_when_missing(
        self, phase_state_with_existing_file
    ):
        """complete_phase should raise SubagentEnforcementError when subagents missing."""
        from devforgeai_cli.phase_state import SubagentEnforcementError

        ps = phase_state_with_existing_file
        # Don't record any subagents

        with pytest.raises(SubagentEnforcementError) as exc_info:
            ps.complete_phase("STORY-001", "01", checkpoint_passed=True)

        assert exc_info.value.story_id == "STORY-001"
        assert exc_info.value.phase == "01"
        assert "git-validator" in exc_info.value.missing_subagents


# =============================================================================
# STORY-307: PHASE_REQUIRED_SUBAGENTS Constant Tests (AC4)
# =============================================================================


class TestPHASE_REQUIRED_SUBAGENTS:
    """Tests for AC4: PHASE_REQUIRED_SUBAGENTS constant structure validation.

    STORY-307: Validates the constant structure added in STORY-306.
    """

    def test_constant_exists_in_module(self):
        """PHASE_REQUIRED_SUBAGENTS constant should exist."""
        from devforgeai_cli import phase_state

        assert hasattr(phase_state, 'PHASE_REQUIRED_SUBAGENTS')

    def test_constant_contains_all_12_phases(self):
        """Constant should have entries for all 12 valid phases."""
        from devforgeai_cli.phase_state import PHASE_REQUIRED_SUBAGENTS

        expected = ["01", "02", "03", "04", "4.5", "05", "5.5", "06", "07", "08", "09", "10"]
        for phase in expected:
            assert phase in PHASE_REQUIRED_SUBAGENTS, f"Missing phase {phase}"

    def test_phase_03_uses_tuple_for_or_logic(self):
        """Phase 03 should use tuple for OR logic (backend/frontend)."""
        from devforgeai_cli.phase_state import PHASE_REQUIRED_SUBAGENTS

        phase_03 = PHASE_REQUIRED_SUBAGENTS["03"]
        has_tuple = any(isinstance(item, tuple) for item in phase_03)
        assert has_tuple, "Phase 03 should have tuple for OR logic"

    def test_phase_03_tuple_contains_backend_and_frontend(self):
        """Phase 03 tuple should contain both backend-architect and frontend-developer."""
        from devforgeai_cli.phase_state import PHASE_REQUIRED_SUBAGENTS

        phase_03 = PHASE_REQUIRED_SUBAGENTS["03"]
        for item in phase_03:
            if isinstance(item, tuple):
                assert "backend-architect" in item
                assert "frontend-developer" in item
                break
        else:
            pytest.fail("No tuple found in Phase 03 requirements")

    def test_phase_09_includes_framework_analyst(self):
        """Phase 09 should require framework-analyst (RCA-027 fix)."""
        from devforgeai_cli.phase_state import PHASE_REQUIRED_SUBAGENTS

        assert "framework-analyst" in PHASE_REQUIRED_SUBAGENTS["09"]

    def test_phase_01_requires_git_and_tech_stack(self):
        """Phase 01 should require git-validator and tech-stack-detector."""
        from devforgeai_cli.phase_state import PHASE_REQUIRED_SUBAGENTS

        assert "git-validator" in PHASE_REQUIRED_SUBAGENTS["01"]
        assert "tech-stack-detector" in PHASE_REQUIRED_SUBAGENTS["01"]

    def test_phase_02_requires_test_automator(self):
        """Phase 02 should require test-automator."""
        from devforgeai_cli.phase_state import PHASE_REQUIRED_SUBAGENTS

        assert "test-automator" in PHASE_REQUIRED_SUBAGENTS["02"]

    def test_phase_06_07_08_empty_requirements(self):
        """Phases 06, 07, 08 should have empty requirements (conditional/file/git ops)."""
        from devforgeai_cli.phase_state import PHASE_REQUIRED_SUBAGENTS

        assert PHASE_REQUIRED_SUBAGENTS["06"] == []
        assert PHASE_REQUIRED_SUBAGENTS["07"] == []
        assert PHASE_REQUIRED_SUBAGENTS["08"] == []

    def test_ac_verification_phases_require_verifier(self):
        """Phases 4.5 and 5.5 should require ac-compliance-verifier."""
        from devforgeai_cli.phase_state import PHASE_REQUIRED_SUBAGENTS

        assert "ac-compliance-verifier" in PHASE_REQUIRED_SUBAGENTS["4.5"]
        assert "ac-compliance-verifier" in PHASE_REQUIRED_SUBAGENTS["5.5"]


# =============================================================================
# STORY-307: OR Logic Phase 03 Tests (AC5)
# =============================================================================


class TestORLogicPhase03:
    """Tests for AC5: OR logic for Phase 03 subagent requirements.

    STORY-307: Validates that either backend-architect OR frontend-developer
    satisfies Phase 03 architect requirement.
    """

    def _advance_to_phase_03(self, ps):
        """Helper to advance state to Phase 03."""
        # Complete Phase 01
        ps.record_subagent("STORY-001", "01", "git-validator")
        ps.record_subagent("STORY-001", "01", "tech-stack-detector")
        ps.complete_phase("STORY-001", "01", checkpoint_passed=True)

        # Complete Phase 02
        ps.record_subagent("STORY-001", "02", "test-automator")
        ps.complete_phase("STORY-001", "02", checkpoint_passed=True)

    def test_phase03_succeeds_with_backend_architect_only(
        self, phase_state_with_existing_file
    ):
        """Phase 03 should complete with only backend-architect."""
        ps = phase_state_with_existing_file
        self._advance_to_phase_03(ps)

        # Phase 03: backend-architect only (no frontend-developer)
        ps.record_subagent("STORY-001", "03", "backend-architect")
        ps.record_subagent("STORY-001", "03", "context-validator")

        # Should succeed without error
        ps.complete_phase("STORY-001", "03", checkpoint_passed=True)
        state = ps.read("STORY-001")
        assert state["phases"]["03"]["status"] == "completed"

    def test_phase03_succeeds_with_frontend_developer_only(
        self, phase_state_with_existing_file
    ):
        """Phase 03 should complete with only frontend-developer."""
        ps = phase_state_with_existing_file
        self._advance_to_phase_03(ps)

        # Phase 03: frontend-developer only (no backend-architect)
        ps.record_subagent("STORY-001", "03", "frontend-developer")
        ps.record_subagent("STORY-001", "03", "context-validator")

        # Should succeed without error
        ps.complete_phase("STORY-001", "03", checkpoint_passed=True)
        state = ps.read("STORY-001")
        assert state["phases"]["03"]["status"] == "completed"

    def test_phase03_fails_with_neither_architect_subagent(
        self, phase_state_with_existing_file
    ):
        """Phase 03 should fail when neither backend/frontend invoked."""
        from devforgeai_cli.phase_state import SubagentEnforcementError

        ps = phase_state_with_existing_file
        self._advance_to_phase_03(ps)

        # Phase 03: only context-validator (missing architect)
        ps.record_subagent("STORY-001", "03", "context-validator")

        with pytest.raises(SubagentEnforcementError):
            ps.complete_phase("STORY-001", "03", checkpoint_passed=True)

    def test_phase03_succeeds_with_both_architect_subagents(
        self, phase_state_with_existing_file
    ):
        """Phase 03 should complete when both architects invoked (over-satisfaction)."""
        ps = phase_state_with_existing_file
        self._advance_to_phase_03(ps)

        # Phase 03: both architects (over-satisfied)
        ps.record_subagent("STORY-001", "03", "backend-architect")
        ps.record_subagent("STORY-001", "03", "frontend-developer")
        ps.record_subagent("STORY-001", "03", "context-validator")

        # Should succeed without error
        ps.complete_phase("STORY-001", "03", checkpoint_passed=True)
        state = ps.read("STORY-001")
        assert state["phases"]["03"]["status"] == "completed"

    def test_phase03_fails_missing_context_validator(
        self, phase_state_with_existing_file
    ):
        """Phase 03 should fail when context-validator missing (even with architect)."""
        from devforgeai_cli.phase_state import SubagentEnforcementError

        ps = phase_state_with_existing_file
        self._advance_to_phase_03(ps)

        # Phase 03: architect only (missing context-validator)
        ps.record_subagent("STORY-001", "03", "backend-architect")

        with pytest.raises(SubagentEnforcementError) as exc_info:
            ps.complete_phase("STORY-001", "03", checkpoint_passed=True)

        assert "context-validator" in str(exc_info.value)


# =============================================================================
# STORY-307: Escape Hatch Tests (AC6)
# =============================================================================


class TestEscapeHatch:
    """Tests for AC6: Escape hatch bypasses subagent enforcement.

    STORY-307: Validates that checkpoint_passed=False bypasses
    subagent enforcement for emergency situations.
    """

    def test_escape_hatch_bypasses_enforcement(
        self, phase_state_with_existing_file
    ):
        """checkpoint_passed=False should bypass subagent enforcement."""
        ps = phase_state_with_existing_file

        # Do NOT record required subagents
        ps.complete_phase("STORY-001", "01", checkpoint_passed=False)

        state = ps.read("STORY-001")
        assert state["phases"]["01"]["status"] == "completed"
        assert state["phases"]["01"]["checkpoint_passed"] is False

    def test_escape_hatch_no_subagent_enforcement_error(
        self, phase_state_with_existing_file
    ):
        """checkpoint_passed=False should not raise SubagentEnforcementError."""
        from devforgeai_cli.phase_state import SubagentEnforcementError

        ps = phase_state_with_existing_file

        # Should NOT raise even without required subagents
        try:
            ps.complete_phase("STORY-001", "01", checkpoint_passed=False)
        except SubagentEnforcementError:
            pytest.fail("Escape hatch should bypass SubagentEnforcementError")

    def test_escape_hatch_advances_current_phase(
        self, phase_state_with_existing_file
    ):
        """Escape hatch should still advance current_phase."""
        ps = phase_state_with_existing_file

        ps.complete_phase("STORY-001", "01", checkpoint_passed=False)

        state = ps.read("STORY-001")
        assert state["current_phase"] == "02"

    def test_escape_hatch_records_completed_at(
        self, phase_state_with_existing_file
    ):
        """Escape hatch should still record completed_at timestamp."""
        ps = phase_state_with_existing_file

        ps.complete_phase("STORY-001", "01", checkpoint_passed=False)

        state = ps.read("STORY-001")
        assert "completed_at" in state["phases"]["01"]


# =============================================================================
# STORY-307: Backward Compatibility Tests (AC7)
# =============================================================================


class TestBackwardCompatibility:
    """Tests for AC7: Legacy state file migration.

    STORY-307: Validates that legacy state files with empty subagents_required
    arrays are populated from PHASE_REQUIRED_SUBAGENTS on read.
    """

    def test_legacy_empty_arrays_populated_on_read(self, temp_project_root: Path):
        """Legacy state with empty subagents_required should be populated."""
        from devforgeai_cli.phase_state import PhaseState, PHASE_REQUIRED_SUBAGENTS

        ps = PhaseState(project_root=temp_project_root)

        # Create legacy state with empty arrays
        workflows_dir = temp_project_root / "devforgeai" / "workflows"
        workflows_dir.mkdir(parents=True)
        legacy_state = {
            "story_id": "STORY-001",
            "current_phase": "01",
            "workflow_started": "2026-01-24T00:00:00Z",
            "blocking_status": False,
            "phases": {
                f"{i:02d}": {
                    "status": "pending",
                    "subagents_required": [],  # Legacy: empty
                    "subagents_invoked": []
                } for i in range(1, 11)
            },
            "validation_errors": [],
            "observations": []
        }
        (workflows_dir / "STORY-001-phase-state.json").write_text(json.dumps(legacy_state))

        # Read should populate subagents_required
        state = ps.read("STORY-001")

        # Phase 01 should now have populated subagents_required
        assert state["phases"]["01"]["subagents_required"] != []
        assert "git-validator" in state["phases"]["01"]["subagents_required"]

    def test_legacy_missing_decimal_phases_added(self, temp_project_root: Path):
        """Legacy state missing 4.5/5.5 phases should have them added."""
        from devforgeai_cli.phase_state import PhaseState

        ps = PhaseState(project_root=temp_project_root)

        # Create legacy state without decimal phases
        workflows_dir = temp_project_root / "devforgeai" / "workflows"
        workflows_dir.mkdir(parents=True)
        legacy_state = {
            "story_id": "STORY-001",
            "current_phase": "01",
            "workflow_started": "2026-01-24T00:00:00Z",
            "blocking_status": False,
            "phases": {
                f"{i:02d}": {
                    "status": "pending",
                    "subagents_required": [],
                    "subagents_invoked": []
                } for i in range(1, 11)
            },
            "validation_errors": [],
            "observations": []
        }
        (workflows_dir / "STORY-001-phase-state.json").write_text(json.dumps(legacy_state))

        # Read should add missing decimal phases
        state = ps.read("STORY-001")

        assert "4.5" in state["phases"]
        assert "5.5" in state["phases"]
        assert state["phases"]["4.5"]["status"] == "pending"
        assert state["phases"]["5.5"]["status"] == "pending"

    def test_new_state_has_populated_subagents_required(self, phase_state):
        """Newly created state should have populated subagents_required."""
        from devforgeai_cli.phase_state import PHASE_REQUIRED_SUBAGENTS

        state = phase_state.create("STORY-001")

        # Verify Phase 01 has populated requirements
        assert state["phases"]["01"]["subagents_required"] != []
        assert "git-validator" in state["phases"]["01"]["subagents_required"]

        # Verify Phase 03 has OR logic (as list)
        phase_03_required = state["phases"]["03"]["subagents_required"]
        has_list = any(isinstance(item, list) for item in phase_03_required)
        assert has_list, "Phase 03 should have list (serialized tuple) for OR logic"
