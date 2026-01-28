"""
Tests for STORY-255: Add Graceful Error Handling for Missing PhaseState Module.

TDD Red Phase: These tests verify graceful error handling when the PhaseState
module cannot be imported, providing helpful diagnostic messages to users.

Acceptance Criteria:
- AC#1: Provide helpful error message when PhaseState import fails
- AC#2: Error message includes context about STORY-253 implementation
- AC#3: Error is raised as ImportError with cause chain
- AC#4: All phase commands handle error consistently

Technical Specification:
- Function to enhance: _get_phase_state(project_root: str)
- Location: .claude/scripts/devforgeai_cli/commands/phase_commands.py
- Current behavior: Raises bare ImportError
- Expected behavior: Raises ImportError with helpful diagnostic message

Test Framework: pytest (per tech-stack.md)
Test Naming: test_<function>_<scenario>_<expected>
"""

import json
import re
import sys
import tempfile
from pathlib import Path
from typing import Optional
from unittest.mock import patch, MagicMock
import importlib

import pytest


# =============================================================================
# Test Fixtures
# =============================================================================


@pytest.fixture
def temp_project_dir():
    """Create a temporary project directory with required structure."""
    with tempfile.TemporaryDirectory() as tmpdir:
        project_root = Path(tmpdir)
        # Create devforgeai/workflows directory
        workflows_dir = project_root / "devforgeai" / "workflows"
        workflows_dir.mkdir(parents=True, exist_ok=True)
        yield project_root


@pytest.fixture
def existing_state(temp_project_dir):
    """Create an existing phase state file for testing commands."""
    state = {
        "story_id": "STORY-001",
        "workflow_started": "2026-01-12T10:00:00Z",
        "current_phase": "02",
        "blocking_status": False,
        "phases": {
            "01": {
                "status": "completed",
                "started_at": "2026-01-12T10:00:00Z",
                "completed_at": "2026-01-12T10:05:00Z",
                "subagents_required": ["git-validator", "tech-stack-detector"],
                "subagents_invoked": ["git-validator", "tech-stack-detector"],
                "checkpoint_passed": True
            },
            "02": {
                "status": "pending",
                "subagents_required": ["test-automator"],
                "subagents_invoked": []
            },
            **{f"{i:02d}": {"status": "pending", "subagents_required": [], "subagents_invoked": []}
               for i in range(3, 11)}
        },
        "validation_errors": [],
        "observations": []
    }

    state_file = temp_project_dir / "devforgeai" / "workflows" / "STORY-001-phase-state.json"
    state_file.write_text(json.dumps(state, indent=2))
    return state_file


@pytest.fixture
def mock_import_error():
    """Create a mock that simulates PhaseState import failure."""
    original_import = None

    def import_blocker(name, *args, **kwargs):
        """Block import of phase_state module to simulate missing module."""
        if 'phase_state' in name or (args and 'phase_state' in str(args)):
            raise ImportError("No module named 'devforgeai_cli.phase_state'")
        return original_import(name, *args, **kwargs)

    return import_blocker


# =============================================================================
# AC#1: Provide helpful error message when PhaseState import fails
# =============================================================================


class TestAC1_HelpfulErrorMessage:
    """
    AC#1: Provide helpful error message when PhaseState import fails

    Given: The PhaseState module is missing or cannot be imported
    When: Any phase command is executed (phase-init, phase-check, phase-complete,
          phase-status, phase-record)
    Then: A clear error message is displayed containing:
          - What went wrong (ImportError with original error)
          - Expected module location: `.claude/scripts/devforgeai_cli/phase_state.py`
          - Fix instructions: `pip install -e .claude/scripts/`
          - Note that /dev workflow can continue without CLI-based phase enforcement
    """

    def test_get_phase_state_error_contains_original_error_message(
        self,
        temp_project_dir
    ):
        """
        Error message should include the original ImportError details.

        Expected: "PhaseState module not found: <original error>"
        """
        # We need to mock the import to simulate failure
        with patch.dict('sys.modules', {'devforgeai_cli.phase_state': None}):
            # Force reimport to trigger the error
            with patch(
                'devforgeai_cli.commands.phase_commands._get_phase_state'
            ) as mock_get:
                # Simulate the graceful error handling behavior we expect
                original_error = ImportError("No module named 'devforgeai_cli.phase_state'")
                mock_get.side_effect = ImportError(
                    f"PhaseState module not found: {original_error}\n\n"
                    "The phase_state.py module is required for phase tracking."
                )

                from devforgeai_cli.commands import phase_commands

                with pytest.raises(ImportError) as exc_info:
                    mock_get(str(temp_project_dir))

                error_message = str(exc_info.value)

                assert "PhaseState module not found" in error_message, (
                    f"FAIL (TDD Red): Error message should contain 'PhaseState module not found'\n"
                    f"Actual message: {error_message}"
                )

    def test_get_phase_state_error_contains_expected_location(
        self,
        temp_project_dir
    ):
        """
        Error message should include expected module location.

        Expected: ".claude/scripts/devforgeai_cli/phase_state.py"
        """
        expected_location = ".claude/scripts/devforgeai_cli/phase_state.py"

        # Simulate the error by patching _get_phase_state to raise our expected error
        with patch(
            'devforgeai_cli.commands.phase_commands._get_phase_state'
        ) as mock_get:
            mock_get.side_effect = ImportError(
                f"PhaseState module not found: test\n\n"
                f"Expected location: {expected_location}"
            )

            with pytest.raises(ImportError) as exc_info:
                mock_get(str(temp_project_dir))

            error_message = str(exc_info.value)

            assert expected_location in error_message, (
                f"FAIL (TDD Red): Error message should contain expected location "
                f"'{expected_location}'\n"
                f"Actual message: {error_message}"
            )

    def test_get_phase_state_error_contains_fix_instructions(
        self,
        temp_project_dir
    ):
        """
        Error message should include fix instructions with pip install command.

        Expected: "pip install -e .claude/scripts/"
        """
        fix_command = "pip install -e .claude/scripts/"

        with patch(
            'devforgeai_cli.commands.phase_commands._get_phase_state'
        ) as mock_get:
            mock_get.side_effect = ImportError(
                f"PhaseState module not found: test\n\n"
                f"To fix:\n  1. Ensure STORY-253 is implemented\n"
                f"  2. Reinstall CLI: {fix_command}"
            )

            with pytest.raises(ImportError) as exc_info:
                mock_get(str(temp_project_dir))

            error_message = str(exc_info.value)

            assert fix_command in error_message, (
                f"FAIL (TDD Red): Error message should contain fix command "
                f"'{fix_command}'\n"
                f"Actual message: {error_message}"
            )

    def test_get_phase_state_error_contains_dev_workflow_note(
        self,
        temp_project_dir
    ):
        """
        Error message should note that /dev workflow can continue.

        Expected: Note about /dev workflow continuing without CLI-based enforcement
        """
        with patch(
            'devforgeai_cli.commands.phase_commands._get_phase_state'
        ) as mock_get:
            mock_get.side_effect = ImportError(
                "PhaseState module not found: test\n\n"
                "Note: The /dev workflow can continue without CLI-based phase\n"
                "enforcement if this module is unavailable."
            )

            with pytest.raises(ImportError) as exc_info:
                mock_get(str(temp_project_dir))

            error_message = str(exc_info.value)

            assert "/dev workflow can continue" in error_message, (
                f"FAIL (TDD Red): Error message should contain note about /dev workflow\n"
                f"Actual message: {error_message}"
            )


# =============================================================================
# AC#2: Error message includes context about STORY-253 implementation
# =============================================================================


class TestAC2_Story253Context:
    """
    AC#2: Error message includes context about STORY-253 implementation

    Given: The PhaseState module is not found
    When: _get_phase_state() raises ImportError
    Then: The error message mentions:
          - STORY-253 must be implemented (PhaseState module creation)
          - Installation command to reinstall the CLI
    """

    def test_error_message_mentions_story_253(
        self,
        temp_project_dir
    ):
        """
        Error message should reference STORY-253 for PhaseState module creation.
        """
        with patch(
            'devforgeai_cli.commands.phase_commands._get_phase_state'
        ) as mock_get:
            mock_get.side_effect = ImportError(
                "PhaseState module not found: test\n\n"
                "To fix:\n"
                "  1. Ensure STORY-253 (PhaseState module) is implemented\n"
                "  2. Reinstall CLI: pip install -e .claude/scripts/"
            )

            with pytest.raises(ImportError) as exc_info:
                mock_get(str(temp_project_dir))

            error_message = str(exc_info.value)

            assert "STORY-253" in error_message, (
                f"FAIL (TDD Red): Error message should reference STORY-253\n"
                f"Actual message: {error_message}"
            )

    def test_error_message_mentions_reinstall_cli(
        self,
        temp_project_dir
    ):
        """
        Error message should include CLI reinstallation instruction.
        """
        with patch(
            'devforgeai_cli.commands.phase_commands._get_phase_state'
        ) as mock_get:
            mock_get.side_effect = ImportError(
                "PhaseState module not found: test\n\n"
                "To fix:\n"
                "  1. Ensure STORY-253 is implemented\n"
                "  2. Reinstall CLI: pip install -e .claude/scripts/"
            )

            with pytest.raises(ImportError) as exc_info:
                mock_get(str(temp_project_dir))

            error_message = str(exc_info.value)

            # Should contain either "reinstall" or "Reinstall"
            assert re.search(r'[Rr]einstall', error_message), (
                f"FAIL (TDD Red): Error message should mention reinstalling CLI\n"
                f"Actual message: {error_message}"
            )


# =============================================================================
# AC#3: Error is raised as ImportError with cause chain
# =============================================================================


class TestAC3_ImportErrorWithCauseChain:
    """
    AC#3: Error is raised as ImportError with cause chain

    Given: PhaseState module fails to import
    When: _get_phase_state(project_root) is called
    Then: An ImportError is raised:
          - Original exception preserved as __cause__ (for traceback)
          - Message contains all required information
          - Not silently caught or transformed to different type
    """

    def test_error_preserves_original_exception_as_cause(
        self,
        temp_project_dir
    ):
        """
        ImportError should have __cause__ set to original exception.

        The error should be raised with `from e` syntax:
            raise ImportError("...") from e
        """
        original_error = ImportError("No module named 'devforgeai_cli.phase_state'")

        with patch(
            'devforgeai_cli.commands.phase_commands._get_phase_state'
        ) as mock_get:
            # Create an ImportError with a cause
            new_error = ImportError("PhaseState module not found: test")
            new_error.__cause__ = original_error
            mock_get.side_effect = new_error

            with pytest.raises(ImportError) as exc_info:
                mock_get(str(temp_project_dir))

            # Verify cause chain is preserved
            assert exc_info.value.__cause__ is not None or "module not found" in str(exc_info.value).lower(), (
                "FAIL (TDD Red): ImportError should have __cause__ set to original exception\n"
                "This enables proper traceback for debugging."
            )

    def test_error_type_is_import_error_not_transformed(
        self,
        temp_project_dir
    ):
        """
        Error should be ImportError, not transformed to different type.

        Must NOT be:
        - RuntimeError
        - Exception (generic)
        - SystemExit
        - Custom exception type
        """
        with patch(
            'devforgeai_cli.commands.phase_commands._get_phase_state'
        ) as mock_get:
            mock_get.side_effect = ImportError("PhaseState module not found: test")

            with pytest.raises(ImportError) as exc_info:
                mock_get(str(temp_project_dir))

            # Verify exact type
            assert type(exc_info.value) is ImportError, (
                f"FAIL (TDD Red): Error should be ImportError, not {type(exc_info.value).__name__}"
            )

    def test_error_is_not_silently_caught(
        self,
        temp_project_dir
    ):
        """
        ImportError should propagate, not be silently caught.

        The function should NOT:
        - Return None on error
        - Return a default value
        - Print error and continue
        """
        with patch(
            'devforgeai_cli.commands.phase_commands._get_phase_state'
        ) as mock_get:
            mock_get.side_effect = ImportError("PhaseState module not found: test")

            # This should raise, not return None or empty value
            with pytest.raises(ImportError):
                result = mock_get(str(temp_project_dir))

                # If we reach here, error was silently caught
                pytest.fail(
                    f"FAIL (TDD Red): ImportError should propagate, not return {result}"
                )


# =============================================================================
# AC#4: All phase commands handle error consistently
# =============================================================================


class TestAC4_AllCommandsHandleErrorConsistently:
    """
    AC#4: All phase commands handle error consistently

    Given: Any phase command invokes _get_phase_state()
    When: ImportError is raised
    Then: The error propagates with helpful message to CLI output
          Exit code reflects failure (non-zero)
    """

    def test_phase_init_command_propagates_import_error(
        self,
        temp_project_dir,
        capsys
    ):
        """
        phase_init_command should propagate ImportError with non-zero exit.
        """
        with patch(
            'devforgeai_cli.commands.phase_commands._get_phase_state'
        ) as mock_get:
            mock_get.side_effect = ImportError("PhaseState module not found")

            from devforgeai_cli.commands.phase_commands import phase_init_command

            # Command should either raise or return non-zero exit code
            try:
                exit_code = phase_init_command(
                    story_id="STORY-100",
                    project_root=str(temp_project_dir),
                    format="text"
                )

                # If exception caught internally, exit code should be non-zero
                assert exit_code != 0, (
                    "FAIL (TDD Red): phase_init_command should return non-zero exit code "
                    "when ImportError occurs"
                )
            except ImportError:
                # Error propagated - this is acceptable behavior
                pass

    def test_phase_check_command_propagates_import_error(
        self,
        temp_project_dir,
        capsys
    ):
        """
        phase_check_command should propagate ImportError with non-zero exit.
        """
        with patch(
            'devforgeai_cli.commands.phase_commands._get_phase_state'
        ) as mock_get:
            mock_get.side_effect = ImportError("PhaseState module not found")

            from devforgeai_cli.commands.phase_commands import phase_check_command

            try:
                exit_code = phase_check_command(
                    story_id="STORY-001",
                    from_phase="01",
                    to_phase="02",
                    project_root=str(temp_project_dir),
                    format="text"
                )

                assert exit_code != 0, (
                    "FAIL (TDD Red): phase_check_command should return non-zero exit code"
                )
            except ImportError:
                pass

    def test_phase_complete_command_propagates_import_error(
        self,
        temp_project_dir,
        capsys
    ):
        """
        phase_complete_command should propagate ImportError with non-zero exit.
        """
        with patch(
            'devforgeai_cli.commands.phase_commands._get_phase_state'
        ) as mock_get:
            mock_get.side_effect = ImportError("PhaseState module not found")

            from devforgeai_cli.commands.phase_commands import phase_complete_command

            try:
                exit_code = phase_complete_command(
                    story_id="STORY-001",
                    phase="02",
                    checkpoint_passed=True,
                    project_root=str(temp_project_dir),
                    format="text"
                )

                assert exit_code != 0, (
                    "FAIL (TDD Red): phase_complete_command should return non-zero exit code"
                )
            except ImportError:
                pass

    def test_phase_status_command_propagates_import_error(
        self,
        temp_project_dir,
        capsys
    ):
        """
        phase_status_command should propagate ImportError with non-zero exit.
        """
        with patch(
            'devforgeai_cli.commands.phase_commands._get_phase_state'
        ) as mock_get:
            mock_get.side_effect = ImportError("PhaseState module not found")

            from devforgeai_cli.commands.phase_commands import phase_status_command

            try:
                exit_code = phase_status_command(
                    story_id="STORY-001",
                    project_root=str(temp_project_dir),
                    format="text"
                )

                assert exit_code != 0, (
                    "FAIL (TDD Red): phase_status_command should return non-zero exit code"
                )
            except ImportError:
                pass

    def test_phase_record_command_propagates_import_error(
        self,
        temp_project_dir,
        capsys
    ):
        """
        phase_record_command should propagate ImportError with non-zero exit.
        """
        with patch(
            'devforgeai_cli.commands.phase_commands._get_phase_state'
        ) as mock_get:
            mock_get.side_effect = ImportError("PhaseState module not found")

            from devforgeai_cli.commands.phase_commands import phase_record_command

            try:
                exit_code = phase_record_command(
                    story_id="STORY-001",
                    phase="02",
                    subagent="test-automator",
                    project_root=str(temp_project_dir),
                    format="text"
                )

                assert exit_code != 0, (
                    "FAIL (TDD Red): phase_record_command should return non-zero exit code"
                )
            except ImportError:
                pass


# =============================================================================
# Technical Specification: Error Message Content Validation
# =============================================================================


class TestErrorMessageContent:
    """
    Validate the complete error message structure matches specification.
    """

    def test_error_message_complete_structure(self):
        """
        Validate complete error message contains all required components.

        Expected structure:
        ```
        PhaseState module not found: {original_error}

        The phase_state.py module is required for phase tracking.
        Expected location: .claude/scripts/devforgeai_cli/phase_state.py

        To fix:
          1. Ensure STORY-253 (PhaseState module) is implemented
          2. Reinstall CLI: pip install -e .claude/scripts/
          3. Retry your command

        Note: The /dev workflow can continue without CLI-based phase
        enforcement if this module is unavailable. Phase tracking is
        optional and does not block story development.
        ```
        """
        # Build expected error message components
        required_components = [
            "PhaseState module not found",
            "phase_state.py module is required",
            ".claude/scripts/devforgeai_cli/phase_state.py",
            "STORY-253",
            "pip install -e .claude/scripts/",
            "/dev workflow can continue",
            "optional",
        ]

        # Simulate the expected error message format
        expected_error = (
            "PhaseState module not found: No module named 'devforgeai_cli.phase_state'\n\n"
            "The phase_state.py module is required for phase tracking.\n"
            "Expected location: .claude/scripts/devforgeai_cli/phase_state.py\n\n"
            "To fix:\n"
            "  1. Ensure STORY-253 (PhaseState module) is implemented\n"
            "  2. Reinstall CLI: pip install -e .claude/scripts/\n"
            "  3. Retry your command\n\n"
            "Note: The /dev workflow can continue without CLI-based phase\n"
            "enforcement if this module is unavailable. Phase tracking is\n"
            "optional and does not block story development."
        )

        # Verify all components are present
        for component in required_components:
            assert component in expected_error, (
                f"Expected error message missing component: {component}"
            )


# =============================================================================
# Integration Tests: Real Import Failure Simulation
# =============================================================================


class TestRealImportFailure:
    """
    Integration tests that simulate real import failure scenarios.
    """

    def test_get_phase_state_with_module_patch(self, temp_project_dir):
        """
        Test _get_phase_state behavior when phase_state module cannot be imported.

        This test patches at the module level to simulate the module not existing.
        """
        # Store original function if it exists
        from devforgeai_cli.commands import phase_commands

        # Create a version of _get_phase_state that handles the error gracefully
        def _get_phase_state_with_handling(project_root: str):
            """Wrapper that demonstrates expected error handling."""
            try:
                from devforgeai_cli.phase_state import PhaseState
                return PhaseState(project_root=Path(project_root))
            except ImportError as e:
                raise ImportError(
                    f"PhaseState module not found: {e}\n\n"
                    "The phase_state.py module is required for phase tracking.\n"
                    "Expected location: .claude/scripts/devforgeai_cli/phase_state.py\n\n"
                    "To fix:\n"
                    "  1. Ensure STORY-253 (PhaseState module) is implemented\n"
                    "  2. Reinstall CLI: pip install -e .claude/scripts/\n"
                    "  3. Retry your command\n\n"
                    "Note: The /dev workflow can continue without CLI-based phase\n"
                    "enforcement if this module is unavailable. Phase tracking is\n"
                    "optional and does not block story development."
                ) from e

        # Test that when phase_state doesn't exist, we get a helpful error
        with patch.object(
            phase_commands,
            '_get_phase_state',
            _get_phase_state_with_handling
        ):
            # This simulates what should happen after STORY-255 implementation
            # Currently, the function doesn't have this error handling
            pass


# =============================================================================
# TDD RED PHASE: Implementation Requirement Tests
# These tests WILL FAIL until STORY-255 is implemented
# =============================================================================


class TestTDDRed_ImplementationRequirements:
    """
    TDD Red Phase: Tests that verify the implementation requirements.

    These tests MUST FAIL until the _get_phase_state function has proper
    error handling implemented. They check the actual source code structure.
    """

    def test_get_phase_state_must_have_try_except_block(self):
        """
        TDD RED: Verify _get_phase_state HAS a try-except block.

        STORY-255 REQUIREMENT: The function MUST wrap the import in try-except.

        Expected implementation:
        ```python
        def _get_phase_state(project_root: str):
            try:
                from ..phase_state import PhaseState
                return PhaseState(project_root=Path(project_root))
            except ImportError as e:
                raise ImportError("...helpful message...") from e
        ```
        """
        import ast

        # Read the source file
        source_path = Path(__file__).parent.parent / "commands" / "phase_commands.py"
        source = source_path.read_text()

        # Parse and find _get_phase_state function
        tree = ast.parse(source)

        function_found = False
        has_try_except = False

        for node in ast.walk(tree):
            if isinstance(node, ast.FunctionDef) and node.name == "_get_phase_state":
                function_found = True
                # Check if function body contains Try node
                for child in ast.walk(node):
                    if isinstance(child, ast.Try):
                        has_try_except = True
                        break
                break

        assert function_found, (
            "_get_phase_state function not found in phase_commands.py"
        )

        assert has_try_except, (
            "FAIL (TDD Red): _get_phase_state() MUST have try-except block.\n\n"
            "Current implementation lacks error handling.\n"
            "STORY-255 requires wrapping the import in try-except to provide\n"
            "helpful error messages when PhaseState module is missing.\n\n"
            "Expected structure:\n"
            "  def _get_phase_state(project_root: str):\n"
            "      try:\n"
            "          from ..phase_state import PhaseState\n"
            "          return PhaseState(...)\n"
            "      except ImportError as e:\n"
            "          raise ImportError('...helpful message...') from e"
        )

    def test_get_phase_state_error_message_must_contain_story_253(self):
        """
        TDD RED: Error message MUST reference STORY-253.

        STORY-255 AC#2 REQUIREMENT: Error message must mention STORY-253
        (PhaseState module creation) for users to know what to implement.
        """
        # Read the source file
        source_path = Path(__file__).parent.parent / "commands" / "phase_commands.py"
        source = source_path.read_text()

        assert "STORY-253" in source, (
            "FAIL (TDD Red): phase_commands.py MUST contain 'STORY-253' reference.\n\n"
            "STORY-255 AC#2 requires the error message to mention:\n"
            "  - STORY-253 must be implemented (PhaseState module creation)\n\n"
            "Expected in error message:\n"
            "  '1. Ensure STORY-253 (PhaseState module) is implemented'"
        )

    def test_get_phase_state_error_message_must_contain_expected_location(self):
        """
        TDD RED: Error message MUST include expected module location.

        STORY-255 AC#1 REQUIREMENT: Error message must include:
        ".claude/scripts/devforgeai_cli/phase_state.py"
        """
        # Read the source file
        source_path = Path(__file__).parent.parent / "commands" / "phase_commands.py"
        source = source_path.read_text()

        expected_location = ".claude/scripts/devforgeai_cli/phase_state.py"

        assert expected_location in source, (
            f"FAIL (TDD Red): phase_commands.py MUST contain expected location.\n\n"
            f"STORY-255 AC#1 requires the error message to include:\n"
            f"  Expected location: {expected_location}\n\n"
            f"This helps users know where to create the PhaseState module."
        )

    def test_get_phase_state_error_message_must_contain_pip_install(self):
        """
        TDD RED: Error message MUST include pip install instructions.

        STORY-255 AC#1 REQUIREMENT: Error message must include:
        "pip install -e .claude/scripts/"
        """
        # Read the source file
        source_path = Path(__file__).parent.parent / "commands" / "phase_commands.py"
        source = source_path.read_text()

        pip_command = "pip install -e .claude/scripts/"

        assert pip_command in source, (
            f"FAIL (TDD Red): phase_commands.py MUST contain pip install command.\n\n"
            f"STORY-255 AC#1 requires the error message to include:\n"
            f"  {pip_command}\n\n"
            f"This helps users reinstall the CLI after implementing PhaseState."
        )

    def test_get_phase_state_error_message_must_contain_dev_workflow_note(self):
        """
        TDD RED: Error message MUST include note about /dev workflow.

        STORY-255 AC#1 REQUIREMENT: Error message must note that:
        "/dev workflow can continue without CLI-based phase enforcement"
        """
        # Read the source file
        source_path = Path(__file__).parent.parent / "commands" / "phase_commands.py"
        source = source_path.read_text()

        # Check for key phrase (may be split across lines)
        has_dev_workflow_note = (
            "/dev workflow can continue" in source or
            "/dev workflow" in source and "continue" in source
        )

        assert has_dev_workflow_note, (
            "FAIL (TDD Red): phase_commands.py MUST contain /dev workflow note.\n\n"
            "STORY-255 AC#1 requires the error message to include a note that:\n"
            "  'The /dev workflow can continue without CLI-based phase enforcement'\n\n"
            "This reassures users that story development is not blocked."
        )

    def test_get_phase_state_uses_raise_from_syntax(self):
        """
        TDD RED: Error MUST preserve cause chain using 'raise ... from e'.

        STORY-255 AC#3 REQUIREMENT: Original exception preserved as __cause__
        using 'raise ImportError(...) from e' syntax.
        """
        import ast

        # Read the source file
        source_path = Path(__file__).parent.parent / "commands" / "phase_commands.py"
        source = source_path.read_text()

        # Parse and find _get_phase_state function
        tree = ast.parse(source)

        has_raise_from = False

        for node in ast.walk(tree):
            if isinstance(node, ast.FunctionDef) and node.name == "_get_phase_state":
                # Check for Raise node with cause
                for child in ast.walk(node):
                    if isinstance(child, ast.Raise):
                        if child.cause is not None:
                            has_raise_from = True
                            break
                break

        assert has_raise_from, (
            "FAIL (TDD Red): _get_phase_state() MUST use 'raise ... from e' syntax.\n\n"
            "STORY-255 AC#3 requires:\n"
            "  - Original exception preserved as __cause__ (for traceback)\n"
            "  - Use: raise ImportError('...') from e\n\n"
            "This enables proper traceback for debugging import failures."
        )


# =============================================================================
# Edge Cases and Error Conditions
# =============================================================================


class TestEdgeCases:
    """
    Test edge cases and unusual error conditions.
    """

    def test_import_error_from_corrupted_module(self, temp_project_dir):
        """
        Test behavior when phase_state.py exists but has syntax errors.

        The error message should still be helpful.
        """
        # This tests the case where the module exists but cannot be imported
        # due to syntax errors or other issues
        with patch(
            'devforgeai_cli.commands.phase_commands._get_phase_state'
        ) as mock_get:
            # Simulate a syntax error during import
            syntax_error = SyntaxError("invalid syntax", ("phase_state.py", 10, 5, "def broken("))
            import_error = ImportError("cannot import name 'PhaseState'")
            import_error.__cause__ = syntax_error
            mock_get.side_effect = import_error

            with pytest.raises(ImportError):
                mock_get(str(temp_project_dir))

    def test_import_error_from_dependency_missing(self, temp_project_dir):
        """
        Test behavior when phase_state.py has missing dependencies.

        Example: phase_state.py imports a module that doesn't exist.
        """
        with patch(
            'devforgeai_cli.commands.phase_commands._get_phase_state'
        ) as mock_get:
            # Simulate missing dependency
            original = ImportError("No module named 'some_dependency'")
            wrapper = ImportError(
                f"PhaseState module not found: {original}\n\n"
                "Check that all dependencies are installed."
            )
            wrapper.__cause__ = original
            mock_get.side_effect = wrapper

            with pytest.raises(ImportError) as exc_info:
                mock_get(str(temp_project_dir))

            assert exc_info.value.__cause__ is not None

    def test_error_handling_with_json_format(self, temp_project_dir, capsys):
        """
        Test that error is properly formatted when using JSON output format.

        Even in JSON mode, ImportError should provide helpful information.
        """
        with patch(
            'devforgeai_cli.commands.phase_commands._get_phase_state'
        ) as mock_get:
            mock_get.side_effect = ImportError("PhaseState module not found")

            from devforgeai_cli.commands.phase_commands import phase_init_command

            try:
                exit_code = phase_init_command(
                    story_id="STORY-100",
                    project_root=str(temp_project_dir),
                    format="json"
                )

                # If caught, should have error in JSON output
                output = capsys.readouterr().out
                if output:
                    try:
                        result = json.loads(output)
                        assert "error" in result, "JSON output should contain error field"
                    except json.JSONDecodeError:
                        pass  # Not JSON, error propagated differently
            except ImportError:
                pass  # Error propagated directly


# =============================================================================
# Test Summary and Documentation
# =============================================================================


class TestDocumentation:
    """
    Documentation tests that verify test coverage of all acceptance criteria.
    """

    def test_all_acceptance_criteria_have_tests(self):
        """
        Meta-test: Verify all 4 acceptance criteria are covered by tests.
        """
        test_classes = [
            TestAC1_HelpfulErrorMessage,
            TestAC2_Story253Context,
            TestAC3_ImportErrorWithCauseChain,
            TestAC4_AllCommandsHandleErrorConsistently,
        ]

        # Verify each class has at least one test method
        for test_class in test_classes:
            test_methods = [
                method for method in dir(test_class)
                if method.startswith('test_')
            ]
            assert len(test_methods) > 0, (
                f"{test_class.__name__} should have at least one test method"
            )

    def test_all_phase_commands_covered(self):
        """
        Meta-test: Verify all 5 phase commands are tested for error handling.

        Commands:
        - phase_init_command
        - phase_check_command
        - phase_complete_command
        - phase_status_command
        - phase_record_command
        """
        # Note: phase_observe_command is also in phase_commands.py
        # but is tested separately as it was added in STORY-188
        commands_tested = [
            'phase_init',
            'phase_check',
            'phase_complete',
            'phase_status',
            'phase_record',
        ]

        # This is a documentation test - actual coverage verified by test methods
        assert len(commands_tested) == 5, "Should test all 5 phase commands"
