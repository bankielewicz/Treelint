"""
Tests for STORY-254: Update phase_commands.py Import.

TDD Red Phase: These tests verify the import refactoring from sys.path
manipulation to relative imports (from ..phase_state import PhaseState).

Acceptance Criteria:
- AC#1: Relative import replaces sys.path manipulation
- AC#2: Function behavior remains unchanged
- AC#3: All phase commands still work correctly

Technical Specification Requirements:
- SVC-001: Replace sys.path manipulation with relative import (Critical)
- SVC-002: Function returns PhaseState instance unchanged (Critical)
- SVC-003: All 6 phase commands work after refactor (Critical)
- SVC-004: Remove unused sys import if applicable (Low)
- BR-001: Import path must be `from ..phase_state` (Critical)
- NFR-001: Import latency < 5ms (Medium)
- NFR-002: Function reduced from 8 lines to 3 lines (High)

Test Framework: pytest (per tech-stack.md)
Test Naming: test_<function>_<scenario>_<expected>
"""

import ast
import inspect
import json
import re
import sys
import tempfile
import time
from pathlib import Path
from typing import Callable, List
from unittest.mock import patch, MagicMock

import pytest


# =============================================================================
# Test Fixtures
# =============================================================================


@pytest.fixture
def phase_commands_source() -> str:
    """Read the source code of phase_commands.py."""
    source_path = Path(__file__).parent.parent / "commands" / "phase_commands.py"
    return source_path.read_text()


@pytest.fixture
def get_phase_state_function_source(phase_commands_source: str) -> str:
    """Extract the _get_phase_state function source code."""
    # Parse the AST to find the function
    tree = ast.parse(phase_commands_source)

    for node in ast.walk(tree):
        if isinstance(node, ast.FunctionDef) and node.name == "_get_phase_state":
            # Get the line range
            start_line = node.lineno
            end_line = node.end_lineno

            # Extract lines
            lines = phase_commands_source.split('\n')
            function_lines = lines[start_line - 1:end_line]
            return '\n'.join(function_lines)

    raise ValueError("_get_phase_state function not found in phase_commands.py")


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


# =============================================================================
# AC#1: Relative import replaces sys.path manipulation
# =============================================================================


class TestAC1_RelativeImport:
    """
    AC#1: Relative import replaces sys.path manipulation

    Given: STORY-253 has been completed and PhaseState is available
    When: The _get_phase_state() function in phase_commands.py is updated
    Then: The function uses `from ..phase_state import PhaseState` instead of
          sys.path.insert() and `from installer.phase_state import PhaseState`
    """

    def test_get_phase_state_uses_relative_import_statement(
        self,
        get_phase_state_function_source: str
    ):
        """
        SVC-001 / BR-001: Import uses `from ..phase_state` relative path.

        The function should contain:
            from ..phase_state import PhaseState

        NOT:
            from installer.phase_state import PhaseState
        """
        # Check for relative import pattern
        relative_import_pattern = r'from\s+\.\.phase_state\s+import\s+PhaseState'
        has_relative_import = bool(
            re.search(relative_import_pattern, get_phase_state_function_source)
        )

        assert has_relative_import, (
            "FAIL (TDD Red): _get_phase_state() should use relative import "
            "'from ..phase_state import PhaseState' but does not.\n"
            f"Current function:\n{get_phase_state_function_source}"
        )

    def test_get_phase_state_no_sys_path_insert(
        self,
        get_phase_state_function_source: str
    ):
        """
        SVC-001: No sys.path.insert() calls in the function.

        The function should NOT contain:
            sys.path.insert(...)
        """
        sys_path_pattern = r'sys\.path\.insert'
        has_sys_path_insert = bool(
            re.search(sys_path_pattern, get_phase_state_function_source)
        )

        assert not has_sys_path_insert, (
            "FAIL (TDD Red): _get_phase_state() should NOT use sys.path.insert() "
            "but it does.\n"
            f"Current function:\n{get_phase_state_function_source}"
        )

    def test_get_phase_state_no_installer_import_path(
        self,
        get_phase_state_function_source: str
    ):
        """
        SVC-001: No 'from installer.phase_state' import path.

        The function should NOT contain:
            from installer.phase_state import PhaseState
        """
        installer_import_pattern = r'from\s+installer\.phase_state\s+import'
        has_installer_import = bool(
            re.search(installer_import_pattern, get_phase_state_function_source)
        )

        assert not has_installer_import, (
            "FAIL (TDD Red): _get_phase_state() should NOT use "
            "'from installer.phase_state import PhaseState' "
            "but should use relative import instead.\n"
            f"Current function:\n{get_phase_state_function_source}"
        )

    def test_get_phase_state_no_installer_path_construction(
        self,
        get_phase_state_function_source: str
    ):
        """
        SVC-001: No installer path construction (no Path(project_root) / "installer").

        The function should NOT contain:
            installer_path = Path(project_root) / "installer"
        """
        path_construction_pattern = r'installer_path\s*=.*["\']installer["\']'
        has_path_construction = bool(
            re.search(path_construction_pattern, get_phase_state_function_source)
        )

        assert not has_path_construction, (
            "FAIL (TDD Red): _get_phase_state() should NOT construct installer path "
            "but it does.\n"
            f"Current function:\n{get_phase_state_function_source}"
        )


# =============================================================================
# AC#2: Function behavior remains unchanged
# =============================================================================


class TestAC2_FunctionBehaviorUnchanged:
    """
    AC#2: Function behavior remains unchanged

    Given: The refactored _get_phase_state() function
    When: Called with a valid project_root parameter
    Then: It returns a PhaseState instance with the same behavior as before
    """

    def test_get_phase_state_returns_phase_state_instance(self, temp_project_dir):
        """
        SVC-002: Function returns PhaseState instance unchanged.

        _get_phase_state(project_root) should return a PhaseState instance.
        """
        # Import the module to test
        from devforgeai_cli.commands.phase_commands import _get_phase_state

        result = _get_phase_state(str(temp_project_dir))

        # The result should be a PhaseState instance
        # We check the class name since import paths may differ
        assert result.__class__.__name__ == "PhaseState", (
            f"FAIL (TDD Red): _get_phase_state() should return PhaseState instance, "
            f"got {type(result).__name__}"
        )

    def test_get_phase_state_sets_project_root(self, temp_project_dir):
        """
        SVC-002: PhaseState instance has correct project_root.
        """
        from devforgeai_cli.commands.phase_commands import _get_phase_state

        result = _get_phase_state(str(temp_project_dir))

        # PhaseState should have project_root attribute
        assert hasattr(result, 'project_root'), (
            "FAIL (TDD Red): PhaseState instance should have project_root attribute"
        )
        assert result.project_root == Path(temp_project_dir), (
            f"FAIL (TDD Red): PhaseState.project_root should be {temp_project_dir}, "
            f"got {result.project_root}"
        )

    def test_get_phase_state_can_create_workflow_state(self, temp_project_dir):
        """
        SVC-002: Returned PhaseState can create workflow state files.
        """
        from devforgeai_cli.commands.phase_commands import _get_phase_state

        ps = _get_phase_state(str(temp_project_dir))

        # Should be able to create a state
        state = ps.create("STORY-099")

        assert state is not None
        assert state["story_id"] == "STORY-099"
        assert state["current_phase"] == "01"


# =============================================================================
# AC#3: All phase commands still work correctly
# =============================================================================


class TestAC3_AllPhaseCommandsWork:
    """
    AC#3: All phase commands still work correctly

    Given: The updated import in _get_phase_state()
    When: All phase command functions are invoked
    Then: All commands execute without import errors and maintain existing functionality

    Commands tested:
    - phase_init_command
    - phase_check_command
    - phase_complete_command
    - phase_status_command
    - phase_record_command
    - phase_observe_command
    """

    def test_phase_init_command_works_after_import_refactor(self, temp_project_dir):
        """
        SVC-003: phase_init command works with relative import.
        """
        from devforgeai_cli.commands.phase_commands import phase_init_command

        # Should not raise ImportError
        exit_code = phase_init_command(
            story_id="STORY-100",
            project_root=str(temp_project_dir),
            format="text"
        )

        assert exit_code == 0, (
            f"FAIL (TDD Red): phase_init_command should return 0, got {exit_code}"
        )

        # State file should be created
        state_file = temp_project_dir / "devforgeai" / "workflows" / "STORY-100-phase-state.json"
        assert state_file.exists(), "State file should be created"

    def test_phase_check_command_works_after_import_refactor(
        self,
        temp_project_dir,
        existing_state
    ):
        """
        SVC-003: phase_check command works with relative import.
        """
        from devforgeai_cli.commands.phase_commands import phase_check_command

        # Should not raise ImportError
        exit_code = phase_check_command(
            story_id="STORY-001",
            from_phase="01",
            to_phase="02",
            project_root=str(temp_project_dir),
            format="text"
        )

        assert exit_code == 0, (
            f"FAIL (TDD Red): phase_check_command should return 0, got {exit_code}"
        )

    def test_phase_complete_command_works_after_import_refactor(
        self,
        temp_project_dir,
        existing_state
    ):
        """
        SVC-003: phase_complete command works with relative import.
        """
        from devforgeai_cli.commands.phase_commands import phase_complete_command

        # Should not raise ImportError
        exit_code = phase_complete_command(
            story_id="STORY-001",
            phase="02",
            checkpoint_passed=True,
            project_root=str(temp_project_dir),
            format="text"
        )

        assert exit_code == 0, (
            f"FAIL (TDD Red): phase_complete_command should return 0, got {exit_code}"
        )

    def test_phase_status_command_works_after_import_refactor(
        self,
        temp_project_dir,
        existing_state
    ):
        """
        SVC-003: phase_status command works with relative import.
        """
        from devforgeai_cli.commands.phase_commands import phase_status_command

        # Should not raise ImportError
        exit_code = phase_status_command(
            story_id="STORY-001",
            project_root=str(temp_project_dir),
            format="text"
        )

        assert exit_code == 0, (
            f"FAIL (TDD Red): phase_status_command should return 0, got {exit_code}"
        )

    def test_phase_record_command_works_after_import_refactor(
        self,
        temp_project_dir,
        existing_state
    ):
        """
        SVC-003: phase_record command works with relative import.
        """
        from devforgeai_cli.commands.phase_commands import phase_record_command

        # Should not raise ImportError
        exit_code = phase_record_command(
            story_id="STORY-001",
            phase="02",
            subagent="test-automator",
            project_root=str(temp_project_dir),
            format="text"
        )

        assert exit_code == 0, (
            f"FAIL (TDD Red): phase_record_command should return 0, got {exit_code}"
        )

    def test_phase_observe_command_works_after_import_refactor(
        self,
        temp_project_dir,
        existing_state
    ):
        """
        SVC-003: phase_observe command works with relative import.
        """
        from devforgeai_cli.commands.phase_commands import phase_observe_command

        # Should not raise ImportError
        exit_code = phase_observe_command(
            story_id="STORY-001",
            phase="02",
            category="success",
            note="Test observation",
            severity="medium",
            project_root=str(temp_project_dir),
            format="text"
        )

        assert exit_code == 0, (
            f"FAIL (TDD Red): phase_observe_command should return 0, got {exit_code}"
        )


# =============================================================================
# Technical Specification: Non-Functional Requirements
# =============================================================================


class TestNFR_NonFunctionalRequirements:
    """
    Non-Functional Requirements from Technical Specification.
    """

    def test_get_phase_state_function_line_count_reduced(
        self,
        get_phase_state_function_source: str
    ):
        """
        NFR-002: Function reduced from original sys.path approach.

        STORY-254: Reduced from 8 lines (sys.path) to 3 lines (relative import)
        STORY-255: Added graceful error handling (~20 additional lines for
                   helpful diagnostic message when PhaseState module missing)

        The refactored function should have:
        - Relative import (from ..phase_state) - STORY-254
        - try/except error handling with helpful message - STORY-255
        - Comprehensive docstring documenting the ImportError behavior
        """
        # Count non-empty, non-comment lines
        lines = get_phase_state_function_source.split('\n')
        code_lines = [
            line for line in lines
            if line.strip() and not line.strip().startswith('#')
            and not line.strip().startswith('"""')
        ]

        # STORY-255 adds error handling which increases line count legitimately:
        # - ~18 lines for docstring
        # - ~4 lines for try block (def, try, import, return)
        # - ~15 lines for except block (error message)
        # Total: ~35-40 lines maximum
        max_expected_lines = 40  # With error handling from STORY-255
        min_expected_lines = 15  # Minimum for proper error handling

        actual_line_count = len(code_lines)

        # Verify function has error handling (STORY-255) but isn't bloated
        assert actual_line_count <= max_expected_lines, (
            f"_get_phase_state() should have ≤{max_expected_lines} lines "
            f"(with STORY-255 error handling), but has {actual_line_count}.\n"
            f"Current function:\n{get_phase_state_function_source}"
        )

        # Verify function has minimum content for error handling
        assert actual_line_count >= min_expected_lines, (
            f"_get_phase_state() should have ≥{min_expected_lines} lines "
            f"(must include STORY-255 error handling), but has {actual_line_count}.\n"
            f"Verify try/except block and helpful error message are present."
        )

    def test_get_phase_state_import_latency(self, temp_project_dir):
        """
        NFR-001: Import latency < 5ms per import.

        The function should execute quickly without sys.path overhead.
        """
        from devforgeai_cli.commands.phase_commands import _get_phase_state

        # Warm up (first import may be slower)
        _get_phase_state(str(temp_project_dir))

        # Measure subsequent calls
        iterations = 10
        start_time = time.perf_counter()

        for _ in range(iterations):
            _get_phase_state(str(temp_project_dir))

        elapsed_ms = ((time.perf_counter() - start_time) / iterations) * 1000

        # Allow 5ms per call
        assert elapsed_ms < 5.0, (
            f"FAIL: _get_phase_state() took {elapsed_ms:.2f}ms per call, "
            f"expected < 5ms"
        )


# =============================================================================
# Technical Specification: SVC-004 Sys Import Removal
# =============================================================================


class TestSVC004_SysImportRemoval:
    """
    SVC-004: Remove unused sys import if applicable (Low priority).

    If sys is only used for sys.path.insert() in _get_phase_state(),
    and that usage is removed, check if sys import can be removed.
    """

    def test_sys_import_usage_check(self, phase_commands_source: str):
        """
        SVC-004: Check if sys is still needed after refactoring.

        After removing sys.path.insert(), verify sys usage:
        - If sys is NOT used anywhere else, import should be removed
        - If sys IS used elsewhere, import should remain
        """
        # Count sys usages (excluding the import statement itself)
        # Pattern matches sys.something usage
        sys_usage_pattern = r'(?<!import\s)sys\.\w+'
        sys_usages = re.findall(sys_usage_pattern, phase_commands_source)

        # Import statement pattern
        sys_import_pattern = r'^import\s+sys\s*$|^from\s+sys\s+import'
        has_sys_import = bool(
            re.search(sys_import_pattern, phase_commands_source, re.MULTILINE)
        )

        # If there are no sys usages (after refactoring), import should be removed
        # For TDD Red, we expect sys.path.insert to still exist
        if len(sys_usages) == 0 and has_sys_import:
            pytest.fail(
                "FAIL (TDD Red): sys is imported but not used. "
                "Remove 'import sys' if sys.path.insert() has been removed."
            )

        # Note: This test passes if either:
        # 1. sys IS used (current state before refactoring)
        # 2. sys import is removed (after refactoring if not needed)


# =============================================================================
# Integration Test: Import Chain Verification
# =============================================================================


class TestImportChainVerification:
    """
    Verify the import chain works correctly after refactoring.
    """

    def test_phase_state_module_importable_from_parent_package(self):
        """
        Verify PhaseState can be imported via relative path from commands module.

        This tests that the package structure supports:
            from ..phase_state import PhaseState

        When executed from .claude/scripts/devforgeai_cli/commands/phase_commands.py
        """
        # Test file path: .claude/scripts/devforgeai_cli/tests/test_phase_commands_import.py
        # devforgeai_cli path: .claude/scripts/devforgeai_cli/
        # phase_state.py should be at: .claude/scripts/devforgeai_cli/phase_state.py

        # The package structure should be:
        # devforgeai_cli/
        #   __init__.py
        #   phase_state.py      <- PhaseState lives here (STORY-253 creates this)
        #   commands/
        #     __init__.py
        #     phase_commands.py <- imports from ..phase_state
        #   tests/
        #     test_phase_commands_import.py <- This file

        # Check that phase_state.py exists at the correct location
        # Path(__file__).parent = tests/
        # Path(__file__).parent.parent = devforgeai_cli/
        phase_state_path = Path(__file__).parent.parent / "phase_state.py"

        assert phase_state_path.exists(), (
            f"FAIL (TDD Red): PhaseState module not found at expected location.\n"
            f"Expected: {phase_state_path}\n"
            "The package structure must support: from ..phase_state import PhaseState\n"
            "STORY-253 must be completed first to create phase_state.py in devforgeai_cli/"
        )

    def test_package_init_files_exist(self):
        """
        Verify __init__.py files exist for proper package structure.
        """
        # Path(__file__).parent = tests/
        # Path(__file__).parent.parent = devforgeai_cli/
        devforgeai_cli_path = Path(__file__).parent.parent

        required_init_files = [
            devforgeai_cli_path / "__init__.py",           # devforgeai_cli/__init__.py
            devforgeai_cli_path / "commands" / "__init__.py",  # devforgeai_cli/commands/__init__.py
        ]

        missing = []
        for init_file in required_init_files:
            if not init_file.exists():
                missing.append(str(init_file))

        assert not missing, (
            f"FAIL: Missing __init__.py files for package structure:\n"
            + "\n".join(missing)
        )


# =============================================================================
# Error Handling Tests
# =============================================================================


class TestImportErrorHandling:
    """
    Test error handling when import fails.
    """

    def test_import_error_is_clear_when_phase_state_missing(self):
        """
        When PhaseState module is not found, error should be clear.

        This test documents expected behavior - if the relative import
        fails, Python should raise ImportError with a clear message.
        """
        # This test validates that if phase_state.py doesn't exist,
        # the import will fail with ImportError, not some obscure error
        # from sys.path manipulation

        # We can't easily test this without mocking, but we document
        # the expected behavior: ImportError raised immediately (fail-fast)
        pass  # Documentation-only test


# =============================================================================
# Code Style Tests
# =============================================================================


class TestCodeStyle:
    """
    Verify code follows Python best practices (PEP 328 for imports).
    """

    def test_no_magic_path_strings(self, get_phase_state_function_source: str):
        """
        Function should not contain hardcoded path strings like "installer".
        """
        # Magic strings that indicate old implementation
        magic_strings = ['installer', 'phase_state.py']

        found = []
        for magic in magic_strings:
            if f'"{magic}"' in get_phase_state_function_source:
                found.append(magic)
            if f"'{magic}'" in get_phase_state_function_source:
                found.append(magic)

        assert not found, (
            f"FAIL (TDD Red): _get_phase_state() contains magic path strings: {found}\n"
            "Use relative imports instead of path string manipulation."
        )
