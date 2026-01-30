"""
STORY-306: Subagent Enforcement Tests

Unit tests for SubagentEnforcementError and PHASE_REQUIRED_SUBAGENTS validation.
Tests the enforcement mechanism that prevents phases from completing without
required subagents being invoked.

Test Coverage:
- AC1: PHASE_REQUIRED_SUBAGENTS constant validation
- AC2: subagents_required population on state creation
- AC3: complete_phase blocking when missing subagents
- AC4: complete_phase success when all subagents invoked
- AC5: Escape hatch (checkpoint_passed=False)
- AC6: OR logic for Phase 03 subagents
- AC8: Backward compatibility for legacy state files
"""

import json
import tempfile
from pathlib import Path
from typing import Generator

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
    from devforgeai_cli.phase_state import PhaseState
    return PhaseState(project_root=temp_project_root)


# =============================================================================
# AC1: PHASE_REQUIRED_SUBAGENTS constant
# =============================================================================


class TestAC1PhaseRequiredSubagentsConstant:
    """Tests for AC1: PHASE_REQUIRED_SUBAGENTS constant validation."""

    def test_constant_exists(self):
        """
        Given: The phase_state module
        When: PHASE_REQUIRED_SUBAGENTS is imported
        Then: The constant exists and is a dictionary
        """
        from devforgeai_cli.phase_state import PHASE_REQUIRED_SUBAGENTS

        assert isinstance(PHASE_REQUIRED_SUBAGENTS, dict)

    def test_constant_contains_all_12_phases(self):
        """
        Given: PHASE_REQUIRED_SUBAGENTS constant
        When: Inspecting the keys
        Then: Contains all 12 valid phases (01-10, 4.5, 5.5)
        """
        from devforgeai_cli.phase_state import PHASE_REQUIRED_SUBAGENTS

        expected_phases = ["01", "02", "03", "04", "4.5", "05", "5.5",
                          "06", "07", "08", "09", "10"]

        for phase in expected_phases:
            assert phase in PHASE_REQUIRED_SUBAGENTS, f"Missing phase: {phase}"

    def test_phase_09_contains_framework_analyst(self):
        """
        Given: PHASE_REQUIRED_SUBAGENTS constant
        When: Inspecting Phase 09 entry
        Then: Contains 'framework-analyst'
        """
        from devforgeai_cli.phase_state import PHASE_REQUIRED_SUBAGENTS

        assert "framework-analyst" in PHASE_REQUIRED_SUBAGENTS["09"]

    def test_phase_03_uses_tuple_for_or_logic(self):
        """
        Given: PHASE_REQUIRED_SUBAGENTS constant
        When: Inspecting Phase 03 entry
        Then: Uses tuple for OR logic (backend-architect, frontend-developer)
        """
        from devforgeai_cli.phase_state import PHASE_REQUIRED_SUBAGENTS

        phase_03_requirements = PHASE_REQUIRED_SUBAGENTS["03"]

        # Find the OR group (tuple)
        or_group = None
        for requirement in phase_03_requirements:
            if isinstance(requirement, tuple):
                or_group = requirement
                break

        assert or_group is not None, "Phase 03 should have a tuple for OR logic"
        assert "backend-architect" in or_group
        assert "frontend-developer" in or_group

    def test_phase_01_contains_git_validator_and_tech_stack_detector(self):
        """
        Given: PHASE_REQUIRED_SUBAGENTS constant
        When: Inspecting Phase 01 entry
        Then: Contains git-validator and tech-stack-detector
        """
        from devforgeai_cli.phase_state import PHASE_REQUIRED_SUBAGENTS

        assert "git-validator" in PHASE_REQUIRED_SUBAGENTS["01"]
        assert "tech-stack-detector" in PHASE_REQUIRED_SUBAGENTS["01"]

    def test_phase_02_contains_test_automator(self):
        """
        Given: PHASE_REQUIRED_SUBAGENTS constant
        When: Inspecting Phase 02 entry
        Then: Contains test-automator
        """
        from devforgeai_cli.phase_state import PHASE_REQUIRED_SUBAGENTS

        assert "test-automator" in PHASE_REQUIRED_SUBAGENTS["02"]


# =============================================================================
# AC2: subagents_required populated on state creation
# =============================================================================


class TestAC2SubagentsRequiredPopulation:
    """Tests for AC2: subagents_required populated from PHASE_REQUIRED_SUBAGENTS."""

    def test_new_state_has_populated_subagents_required(self, phase_state):
        """
        Given: A new phase state is created
        When: create() is called
        Then: Each phase has non-empty subagents_required (where applicable)
        """
        state = phase_state.create("STORY-001")

        # Phase 02 should have test-automator
        assert "test-automator" in state["phases"]["02"]["subagents_required"]

    def test_phase_02_subagents_required_contains_test_automator(self, phase_state):
        """
        Given: A new phase state
        When: Inspecting phases.02.subagents_required
        Then: Contains 'test-automator'
        """
        state = phase_state.create("STORY-001")

        assert "test-automator" in state["phases"]["02"]["subagents_required"]

    def test_phase_09_subagents_required_contains_framework_analyst(self, phase_state):
        """
        Given: A new phase state
        When: Inspecting phases.09.subagents_required
        Then: Contains 'framework-analyst'
        """
        state = phase_state.create("STORY-001")

        assert "framework-analyst" in state["phases"]["09"]["subagents_required"]

    def test_phase_07_has_empty_subagents_required(self, phase_state):
        """
        Given: A new phase state
        When: Inspecting Phase 07 (no required subagents)
        Then: subagents_required is empty
        """
        state = phase_state.create("STORY-001")

        assert state["phases"]["07"]["subagents_required"] == []


# =============================================================================
# AC3: complete_phase blocks when missing subagents
# =============================================================================


class TestAC3BlockingOnMissingSubagents:
    """Tests for AC3: complete_phase blocks when required subagents not invoked."""

    def test_raises_subagent_enforcement_error_when_missing(self, phase_state):
        """
        Given: Phase 02 requires test-automator, not invoked
        When: complete_phase with checkpoint_passed=True
        Then: SubagentEnforcementError is raised
        """
        from devforgeai_cli.phase_state import SubagentEnforcementError

        phase_state.create("STORY-001")
        # Complete phase 01 first
        phase_state.record_subagent("STORY-001", "01", "git-validator")
        phase_state.record_subagent("STORY-001", "01", "tech-stack-detector")
        phase_state.complete_phase("STORY-001", "01", checkpoint_passed=True)

        # Try to complete phase 02 without invoking test-automator
        with pytest.raises(SubagentEnforcementError) as exc_info:
            phase_state.complete_phase("STORY-001", "02", checkpoint_passed=True)

        assert "test-automator" in str(exc_info.value)

    def test_error_message_identifies_missing_subagent(self, phase_state):
        """
        Given: Phase 02 missing test-automator
        When: SubagentEnforcementError is raised
        Then: Error message contains 'test-automator'
        """
        from devforgeai_cli.phase_state import SubagentEnforcementError

        phase_state.create("STORY-001")
        phase_state.record_subagent("STORY-001", "01", "git-validator")
        phase_state.record_subagent("STORY-001", "01", "tech-stack-detector")
        phase_state.complete_phase("STORY-001", "01", checkpoint_passed=True)

        with pytest.raises(SubagentEnforcementError) as exc_info:
            phase_state.complete_phase("STORY-001", "02", checkpoint_passed=True)

        error = exc_info.value
        assert "test-automator" in error.missing_subagents

    def test_error_has_story_id_phase_and_missing_attributes(self, phase_state):
        """
        Given: SubagentEnforcementError is raised
        When: Inspecting exception attributes
        Then: Contains story_id, phase, and missing_subagents
        """
        from devforgeai_cli.phase_state import SubagentEnforcementError

        phase_state.create("STORY-001")
        phase_state.record_subagent("STORY-001", "01", "git-validator")
        phase_state.record_subagent("STORY-001", "01", "tech-stack-detector")
        phase_state.complete_phase("STORY-001", "01", checkpoint_passed=True)

        with pytest.raises(SubagentEnforcementError) as exc_info:
            phase_state.complete_phase("STORY-001", "02", checkpoint_passed=True)

        error = exc_info.value
        assert error.story_id == "STORY-001"
        assert error.phase == "02"
        assert isinstance(error.missing_subagents, list)


# =============================================================================
# AC4: complete_phase succeeds when all subagents invoked
# =============================================================================


class TestAC4SuccessWhenSubagentsInvoked:
    """Tests for AC4: complete_phase succeeds when all required subagents invoked."""

    def test_phase_completes_when_required_subagents_recorded(self, phase_state):
        """
        Given: Phase 02 requires test-automator, invoked
        When: complete_phase with checkpoint_passed=True
        Then: Phase status is 'completed'
        """
        phase_state.create("STORY-001")
        phase_state.record_subagent("STORY-001", "01", "git-validator")
        phase_state.record_subagent("STORY-001", "01", "tech-stack-detector")
        phase_state.complete_phase("STORY-001", "01", checkpoint_passed=True)

        # Record test-automator for phase 02
        phase_state.record_subagent("STORY-001", "02", "test-automator")
        state = phase_state.complete_phase("STORY-001", "02", checkpoint_passed=True)

        assert state["phases"]["02"]["status"] == "completed"

    def test_current_phase_advances_after_success(self, phase_state):
        """
        Given: Phase 02 completed successfully
        When: complete_phase returns
        Then: current_phase advances to '03'
        """
        phase_state.create("STORY-001")
        phase_state.record_subagent("STORY-001", "01", "git-validator")
        phase_state.record_subagent("STORY-001", "01", "tech-stack-detector")
        phase_state.complete_phase("STORY-001", "01", checkpoint_passed=True)

        phase_state.record_subagent("STORY-001", "02", "test-automator")
        state = phase_state.complete_phase("STORY-001", "02", checkpoint_passed=True)

        assert state["current_phase"] == "03"


# =============================================================================
# AC5: Escape hatch (checkpoint_passed=False)
# =============================================================================


class TestAC5EscapeHatch:
    """Tests for AC5: Escape hatch allows completion without subagent validation."""

    def test_completes_without_subagents_when_checkpoint_passed_false(self, phase_state):
        """
        Given: Phase 02 missing required subagents
        When: complete_phase with checkpoint_passed=False
        Then: Phase completes without error
        """
        phase_state.create("STORY-001")
        phase_state.record_subagent("STORY-001", "01", "git-validator")
        phase_state.record_subagent("STORY-001", "01", "tech-stack-detector")
        phase_state.complete_phase("STORY-001", "01", checkpoint_passed=True)

        # Complete phase 02 without invoking test-automator using escape hatch
        state = phase_state.complete_phase("STORY-001", "02", checkpoint_passed=False)

        assert state["phases"]["02"]["status"] == "completed"

    def test_checkpoint_passed_stored_as_false(self, phase_state):
        """
        Given: Escape hatch used
        When: Phase completed with checkpoint_passed=False
        Then: checkpoint_passed is False in state file
        """
        phase_state.create("STORY-001")
        phase_state.record_subagent("STORY-001", "01", "git-validator")
        phase_state.record_subagent("STORY-001", "01", "tech-stack-detector")
        phase_state.complete_phase("STORY-001", "01", checkpoint_passed=True)

        state = phase_state.complete_phase("STORY-001", "02", checkpoint_passed=False)

        assert state["phases"]["02"]["checkpoint_passed"] is False


# =============================================================================
# AC6: OR logic for Phase 03 subagents
# =============================================================================


class TestAC6ORLogicPhase03:
    """Tests for AC6: OR logic for Phase 03 subagents."""

    def test_completes_with_backend_architect_only(self, phase_state):
        """
        Given: Phase 03 requires backend-architect OR frontend-developer
        When: Only backend-architect invoked (plus context-validator)
        Then: Phase 03 completes successfully
        """
        phase_state.create("STORY-001")

        # Complete phases 01-02 first
        phase_state.record_subagent("STORY-001", "01", "git-validator")
        phase_state.record_subagent("STORY-001", "01", "tech-stack-detector")
        phase_state.complete_phase("STORY-001", "01", checkpoint_passed=True)

        phase_state.record_subagent("STORY-001", "02", "test-automator")
        phase_state.complete_phase("STORY-001", "02", checkpoint_passed=True)

        # Complete phase 03 with only backend-architect
        phase_state.record_subagent("STORY-001", "03", "backend-architect")
        phase_state.record_subagent("STORY-001", "03", "context-validator")
        state = phase_state.complete_phase("STORY-001", "03", checkpoint_passed=True)

        assert state["phases"]["03"]["status"] == "completed"

    def test_completes_with_frontend_developer_only(self, phase_state):
        """
        Given: Phase 03 requires backend-architect OR frontend-developer
        When: Only frontend-developer invoked (plus context-validator)
        Then: Phase 03 completes successfully
        """
        phase_state.create("STORY-001")

        # Complete phases 01-02 first
        phase_state.record_subagent("STORY-001", "01", "git-validator")
        phase_state.record_subagent("STORY-001", "01", "tech-stack-detector")
        phase_state.complete_phase("STORY-001", "01", checkpoint_passed=True)

        phase_state.record_subagent("STORY-001", "02", "test-automator")
        phase_state.complete_phase("STORY-001", "02", checkpoint_passed=True)

        # Complete phase 03 with only frontend-developer
        phase_state.record_subagent("STORY-001", "03", "frontend-developer")
        phase_state.record_subagent("STORY-001", "03", "context-validator")
        state = phase_state.complete_phase("STORY-001", "03", checkpoint_passed=True)

        assert state["phases"]["03"]["status"] == "completed"

    def test_blocks_with_neither_or_subagent(self, phase_state):
        """
        Given: Phase 03 requires backend-architect OR frontend-developer
        When: Neither invoked (only context-validator)
        Then: SubagentEnforcementError raised
        """
        from devforgeai_cli.phase_state import SubagentEnforcementError

        phase_state.create("STORY-001")

        # Complete phases 01-02 first
        phase_state.record_subagent("STORY-001", "01", "git-validator")
        phase_state.record_subagent("STORY-001", "01", "tech-stack-detector")
        phase_state.complete_phase("STORY-001", "01", checkpoint_passed=True)

        phase_state.record_subagent("STORY-001", "02", "test-automator")
        phase_state.complete_phase("STORY-001", "02", checkpoint_passed=True)

        # Try to complete phase 03 with only context-validator
        phase_state.record_subagent("STORY-001", "03", "context-validator")

        with pytest.raises(SubagentEnforcementError):
            phase_state.complete_phase("STORY-001", "03", checkpoint_passed=True)


# =============================================================================
# AC8: Backward compatibility for legacy state files
# =============================================================================


class TestAC8BackwardCompatibility:
    """Tests for AC8: Backward compatibility for legacy state files."""

    def test_legacy_state_file_loads_successfully(self, temp_project_root):
        """
        Given: A legacy state file with empty subagents_required
        When: Loaded via read()
        Then: No error, state is returned
        """
        from devforgeai_cli.phase_state import PhaseState

        # Create legacy state file manually
        workflows_dir = temp_project_root / "devforgeai" / "workflows"
        workflows_dir.mkdir(parents=True, exist_ok=True)

        legacy_state = {
            "story_id": "STORY-001",
            "current_phase": "01",
            "workflow_started": "2026-01-01T00:00:00Z",
            "blocking_status": False,
            "phases": {
                f"{i:02d}": {
                    "status": "pending",
                    "subagents_required": [],  # Empty - legacy format
                    "subagents_invoked": []
                } for i in range(1, 11)
            },
            "validation_errors": [],
            "observations": []
        }

        state_path = workflows_dir / "STORY-001-phase-state.json"
        state_path.write_text(json.dumps(legacy_state, indent=2))

        ps = PhaseState(project_root=temp_project_root)
        state = ps.read("STORY-001")

        assert state is not None
        assert state["story_id"] == "STORY-001"

    def test_legacy_state_file_gets_subagents_required_populated(self, temp_project_root):
        """
        Given: A legacy state file with empty subagents_required
        When: Loaded via read()
        Then: subagents_required is populated from PHASE_REQUIRED_SUBAGENTS
        """
        from devforgeai_cli.phase_state import PhaseState

        # Create legacy state file manually
        workflows_dir = temp_project_root / "devforgeai" / "workflows"
        workflows_dir.mkdir(parents=True, exist_ok=True)

        legacy_state = {
            "story_id": "STORY-001",
            "current_phase": "01",
            "workflow_started": "2026-01-01T00:00:00Z",
            "blocking_status": False,
            "phases": {
                f"{i:02d}": {
                    "status": "pending",
                    "subagents_required": [],  # Empty - legacy format
                    "subagents_invoked": []
                } for i in range(1, 11)
            },
            "validation_errors": [],
            "observations": []
        }

        state_path = workflows_dir / "STORY-001-phase-state.json"
        state_path.write_text(json.dumps(legacy_state, indent=2))

        ps = PhaseState(project_root=temp_project_root)
        state = ps.read("STORY-001")

        # After loading, subagents_required should be populated
        assert "test-automator" in state["phases"]["02"]["subagents_required"]
        assert "framework-analyst" in state["phases"]["09"]["subagents_required"]


# =============================================================================
# SubagentEnforcementError Exception Tests
# =============================================================================


class TestSubagentEnforcementError:
    """Tests for SubagentEnforcementError exception class."""

    def test_exception_inherits_from_phase_state_error(self):
        """SubagentEnforcementError should inherit from PhaseStateError."""
        from devforgeai_cli.phase_state import (
            PhaseStateError,
            SubagentEnforcementError
        )

        assert issubclass(SubagentEnforcementError, PhaseStateError)

    def test_exception_stores_attributes(self):
        """SubagentEnforcementError stores story_id, phase, missing_subagents."""
        from devforgeai_cli.phase_state import SubagentEnforcementError

        error = SubagentEnforcementError(
            story_id="STORY-001",
            phase="02",
            missing_subagents=["test-automator", "code-reviewer"]
        )

        assert error.story_id == "STORY-001"
        assert error.phase == "02"
        assert error.missing_subagents == ["test-automator", "code-reviewer"]

    def test_exception_message_format(self):
        """SubagentEnforcementError message includes all details."""
        from devforgeai_cli.phase_state import SubagentEnforcementError

        error = SubagentEnforcementError(
            story_id="STORY-001",
            phase="02",
            missing_subagents=["test-automator"]
        )

        message = str(error)
        assert "STORY-001" in message
        assert "02" in message
        assert "test-automator" in message
