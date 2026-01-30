"""
Tests for validate-installation command (STORY-314).

Validates post-install checks for DevForgeAI projects.

6 Checks:
1. CLI availability (devforgeai-validate --version)
2. Context files (6 files in devforgeai/specs/context/)
3. Hook installation (.git/hooks/pre-commit exists)
4. PYTHONPATH configuration (CLI imports succeed)
5. Git repository (.git/ exists)
6. Settings file (.claude/settings.json exists)
"""

import pytest
from pathlib import Path
from unittest.mock import patch, MagicMock
import subprocess


# =============================================================================
# AC#1: All 6 checks pass on valid installation
# =============================================================================

class TestValidInstallation:
    """Tests for AC#1: All 6 checks pass on valid installation."""

    def test_validate_installation_all_checks_pass_returns_zero(self, tmp_path):
        """Test: Valid installation returns exit code 0."""
        # Arrange: Create valid installation structure
        _create_valid_installation(tmp_path)

        # Act
        from devforgeai_cli.commands.validate_installation import validate_installation_command
        exit_code = validate_installation_command(project_root=str(tmp_path))

        # Assert
        assert exit_code == 0

    def test_validate_installation_reports_6_of_6_passed(self, tmp_path, capsys):
        """Test: Valid installation reports '6/6 checks passed'."""
        # Arrange
        _create_valid_installation(tmp_path)

        # Act
        from devforgeai_cli.commands.validate_installation import validate_installation_command
        validate_installation_command(project_root=str(tmp_path))
        captured = capsys.readouterr()

        # Assert
        assert "6/6" in captured.out or "PASS" in captured.out

    def test_validate_installation_check_cli_available_success(self, tmp_path):
        """Test: CLI availability check passes when devforgeai-validate exists."""
        # Arrange
        _create_valid_installation(tmp_path)

        # Act
        from devforgeai_cli.commands.validate_installation import check_cli_available
        result = check_cli_available()

        # Assert
        assert result["passed"] is True
        assert "devforgeai-validate" in result["message"]

    def test_validate_installation_check_context_files_success(self, tmp_path):
        """Test: Context files check passes when all 6 files exist."""
        # Arrange
        _create_valid_installation(tmp_path)

        # Act
        from devforgeai_cli.commands.validate_installation import check_context_files
        result = check_context_files(project_root=str(tmp_path))

        # Assert
        assert result["passed"] is True
        assert "6/6" in result["message"]

    def test_validate_installation_check_hooks_success(self, tmp_path):
        """Test: Hook check passes when .git/hooks/pre-commit exists."""
        # Arrange
        _create_valid_installation(tmp_path)

        # Act
        from devforgeai_cli.commands.validate_installation import check_hooks_installed
        result = check_hooks_installed(project_root=str(tmp_path))

        # Assert
        assert result["passed"] is True

    def test_validate_installation_check_pythonpath_success(self):
        """Test: PYTHONPATH check passes when CLI imports succeed."""
        # Act
        from devforgeai_cli.commands.validate_installation import check_pythonpath
        result = check_pythonpath()

        # Assert
        assert result["passed"] is True

    def test_validate_installation_check_git_repo_success(self, tmp_path):
        """Test: Git check passes when .git/ directory exists."""
        # Arrange
        _create_valid_installation(tmp_path)

        # Act
        from devforgeai_cli.commands.validate_installation import check_git_repository
        result = check_git_repository(project_root=str(tmp_path))

        # Assert
        assert result["passed"] is True

    def test_validate_installation_check_settings_success(self, tmp_path):
        """Test: Settings check passes when .claude/settings.json exists."""
        # Arrange
        _create_valid_installation(tmp_path)

        # Act
        from devforgeai_cli.commands.validate_installation import check_settings_file
        result = check_settings_file(project_root=str(tmp_path))

        # Assert
        assert result["passed"] is True


# =============================================================================
# AC#2: Clear error for incomplete installation
# =============================================================================

class TestIncompleteInstallation:
    """Tests for AC#2: Clear error for incomplete installation."""

    def test_validate_installation_missing_context_files_returns_nonzero(self, tmp_path):
        """Test: Missing context files returns non-zero exit code."""
        # Arrange: Create installation missing context files
        _create_partial_installation(tmp_path, skip_context=True)

        # Act
        from devforgeai_cli.commands.validate_installation import validate_installation_command
        exit_code = validate_installation_command(project_root=str(tmp_path))

        # Assert
        assert exit_code != 0

    def test_validate_installation_missing_hooks_returns_nonzero(self, tmp_path):
        """Test: Missing hooks returns non-zero exit code."""
        # Arrange
        _create_partial_installation(tmp_path, skip_hooks=True)

        # Act
        from devforgeai_cli.commands.validate_installation import validate_installation_command
        exit_code = validate_installation_command(project_root=str(tmp_path))

        # Assert
        assert exit_code != 0

    def test_validate_installation_missing_git_returns_nonzero(self, tmp_path):
        """Test: Missing .git/ returns non-zero exit code."""
        # Arrange
        _create_partial_installation(tmp_path, skip_git=True)

        # Act
        from devforgeai_cli.commands.validate_installation import validate_installation_command
        exit_code = validate_installation_command(project_root=str(tmp_path))

        # Assert
        assert exit_code != 0

    def test_validate_installation_error_includes_to_fix_instruction(self, tmp_path, capsys):
        """Test: Error output includes 'To fix:' instructions."""
        # Arrange
        _create_partial_installation(tmp_path, skip_context=True)

        # Act
        from devforgeai_cli.commands.validate_installation import validate_installation_command
        validate_installation_command(project_root=str(tmp_path))
        captured = capsys.readouterr()

        # Assert
        assert "To fix:" in captured.out or "to fix" in captured.out.lower()

    def test_validate_installation_error_lists_missing_files(self, tmp_path, capsys):
        """Test: Error output lists which files are missing."""
        # Arrange
        _create_partial_installation(tmp_path, skip_context=True)

        # Act
        from devforgeai_cli.commands.validate_installation import validate_installation_command
        validate_installation_command(project_root=str(tmp_path))
        captured = capsys.readouterr()

        # Assert: Should mention at least one missing context file
        assert any(f in captured.out for f in [
            "tech-stack.md", "source-tree.md", "dependencies.md",
            "coding-standards.md", "architecture-constraints.md", "anti-patterns.md"
        ]) or "Missing" in captured.out

    def test_validate_installation_check_context_files_lists_missing(self, tmp_path):
        """Test: Context check result includes list of missing files."""
        # Arrange
        context_dir = tmp_path / "devforgeai" / "specs" / "context"
        context_dir.mkdir(parents=True)
        (context_dir / "tech-stack.md").write_text("# Tech Stack")
        # Only 1 of 6 files created

        # Act
        from devforgeai_cli.commands.validate_installation import check_context_files
        result = check_context_files(project_root=str(tmp_path))

        # Assert
        assert result["passed"] is False
        assert "missing" in result.get("details", "").lower() or len(result.get("missing", [])) > 0


# =============================================================================
# Business Rules: BR-001, BR-002, BR-003
# =============================================================================

class TestBusinessRules:
    """Tests for business rules from technical specification."""

    def test_br001_each_check_returns_pass_fail_with_reason(self, tmp_path):
        """BR-001: Each check must return pass/fail with clear reason."""
        # Arrange
        _create_valid_installation(tmp_path)

        # Act
        from devforgeai_cli.commands.validate_installation import check_git_repository
        result = check_git_repository(project_root=str(tmp_path))

        # Assert
        assert "passed" in result
        assert "message" in result
        assert isinstance(result["passed"], bool)
        assert isinstance(result["message"], str)

    def test_br002_failed_check_suggests_fix(self, tmp_path):
        """BR-002: Actionable fix must be suggested for each failure."""
        # Arrange: No .git directory
        tmp_path.mkdir(exist_ok=True)

        # Act
        from devforgeai_cli.commands.validate_installation import check_git_repository
        result = check_git_repository(project_root=str(tmp_path))

        # Assert
        assert result["passed"] is False
        assert "fix" in result.get("fix", "").lower() or "To fix" in result.get("message", "")

    def test_br003_exit_code_zero_only_when_all_pass(self, tmp_path):
        """BR-003: Exit code 0 only if all checks pass."""
        # Arrange: Valid installation
        _create_valid_installation(tmp_path)

        # Act
        from devforgeai_cli.commands.validate_installation import validate_installation_command
        exit_code = validate_installation_command(project_root=str(tmp_path))

        # Assert
        assert exit_code == 0

    def test_br003_exit_code_nonzero_when_any_fails(self, tmp_path):
        """BR-003: Any failure results in non-zero exit code."""
        # Arrange: Missing one component
        _create_partial_installation(tmp_path, skip_settings=True)

        # Act
        from devforgeai_cli.commands.validate_installation import validate_installation_command
        exit_code = validate_installation_command(project_root=str(tmp_path))

        # Assert
        assert exit_code != 0


# =============================================================================
# CLI Integration Tests
# =============================================================================

class TestCLIIntegration:
    """Tests for CLI entry point integration."""

    def test_cli_help_includes_validate_installation(self, capsys):
        """Test: CLI --help lists validate-installation command."""
        # This tests that cli.py was updated with the new subcommand
        import sys
        from io import StringIO

        with patch.object(sys, 'argv', ['devforgeai-validate', '--help']):
            with pytest.raises(SystemExit) as exc_info:
                from devforgeai_cli.cli import main
                main()

            # Should exit 0 (help displayed successfully)
            assert exc_info.value.code == 0

        captured = capsys.readouterr()
        assert "validate-installation" in captured.out


# =============================================================================
# Helper Functions
# =============================================================================

def _create_valid_installation(path: Path):
    """Create a complete valid DevForgeAI installation structure."""
    # Git directory
    git_dir = path / ".git"
    git_dir.mkdir(parents=True)

    # Git hooks
    hooks_dir = git_dir / "hooks"
    hooks_dir.mkdir()
    (hooks_dir / "pre-commit").write_text("#!/bin/bash\nexit 0")

    # Context files (6 files)
    context_dir = path / "devforgeai" / "specs" / "context"
    context_dir.mkdir(parents=True)
    for filename in [
        "tech-stack.md", "source-tree.md", "dependencies.md",
        "coding-standards.md", "architecture-constraints.md", "anti-patterns.md"
    ]:
        (context_dir / filename).write_text(f"# {filename}")

    # Settings file
    claude_dir = path / ".claude"
    claude_dir.mkdir(exist_ok=True)
    (claude_dir / "settings.json").write_text("{}")


def _create_partial_installation(
    path: Path,
    skip_context: bool = False,
    skip_hooks: bool = False,
    skip_git: bool = False,
    skip_settings: bool = False
):
    """Create an incomplete installation for error testing."""
    # Git directory (unless skipped)
    if not skip_git:
        git_dir = path / ".git"
        git_dir.mkdir(parents=True)

        if not skip_hooks:
            hooks_dir = git_dir / "hooks"
            hooks_dir.mkdir()
            (hooks_dir / "pre-commit").write_text("#!/bin/bash\nexit 0")

    # Context files (unless skipped)
    if not skip_context:
        context_dir = path / "devforgeai" / "specs" / "context"
        context_dir.mkdir(parents=True)
        for filename in [
            "tech-stack.md", "source-tree.md", "dependencies.md",
            "coding-standards.md", "architecture-constraints.md", "anti-patterns.md"
        ]:
            (context_dir / filename).write_text(f"# {filename}")

    # Settings file (unless skipped)
    if not skip_settings:
        claude_dir = path / ".claude"
        claude_dir.mkdir(exist_ok=True)
        (claude_dir / "settings.json").write_text("{}")
