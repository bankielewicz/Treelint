#!/usr/bin/env python3
"""
Validate Installation Command for DevForgeAI CLI.

STORY-314: Add Post-Install Validation Command

Provides comprehensive post-installation validation that checks:
1. CLI availability (devforgeai-validate --version)
2. Context files (6 files in devforgeai/specs/context/)
3. Hook installation (.git/hooks/pre-commit exists)
4. PYTHONPATH configuration
5. Git repository (.git/ exists)
6. Settings file (.claude/settings.json exists)

Each check returns a ValidationResult with pass/fail status,
descriptive message, and actionable fix instruction for failures.
"""

import subprocess
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import List, Optional


# =============================================================================
# Constants
# =============================================================================

# The 6 required context files
REQUIRED_CONTEXT_FILES = [
    'tech-stack.md',
    'source-tree.md',
    'dependencies.md',
    'coding-standards.md',
    'architecture-constraints.md',
    'anti-patterns.md'
]


# =============================================================================
# Data Classes
# =============================================================================

@dataclass
class ValidationResult:
    """Result of a single validation check."""
    passed: bool
    message: str
    fix_instruction: str = ""

    def __post_init__(self):
        """Ensure fix_instruction has proper format for failures."""
        if not self.passed and self.fix_instruction and not self.fix_instruction.startswith("To fix:"):
            self.fix_instruction = f"To fix: {self.fix_instruction}"


@dataclass
class InstallationValidationResult:
    """Result of complete installation validation."""
    success: bool
    passed_count: int
    failed_count: int
    checks: List[ValidationResult] = field(default_factory=list)
    exit_code: int = 0
    summary: str = ""

    def __post_init__(self):
        """Generate summary and exit code from results."""
        if not self.summary:
            if self.success:
                self.summary = f"PASS ({self.passed_count}/{self.passed_count + self.failed_count} checks passed)"
            else:
                self.summary = f"FAIL ({self.passed_count}/{self.passed_count + self.failed_count} checks passed)"

        if self.exit_code == 0 and not self.success:
            self.exit_code = 1

    def format_output(self) -> str:
        """Format validation results for display."""
        lines = []
        lines.append("DevForgeAI Installation Validation")
        lines.append("=" * 38)
        lines.append("")

        for check in self.checks:
            marker = "[✓]" if check.passed else "[✗]"
            lines.append(f"{marker} {check.message}")
            if not check.passed and check.fix_instruction:
                lines.append(f"    {check.fix_instruction}")

        lines.append("")
        lines.append(f"Result: {self.summary}")

        return "\n".join(lines)


# =============================================================================
# Individual Check Functions
# =============================================================================

def check_cli_available() -> ValidationResult:
    """
    Check 1: Verify devforgeai-validate CLI is installed and working.

    Runs: devforgeai-validate --version

    Returns:
        ValidationResult with pass/fail and version info or fix instruction.
    """
    try:
        result = subprocess.run(
            ["devforgeai-validate", "--version"],
            capture_output=True,
            text=True,
            timeout=10
        )

        if result.returncode == 0:
            version_output = result.stdout.strip()
            return ValidationResult(
                passed=True,
                message=f"CLI available: {version_output}"
            )
        else:
            return ValidationResult(
                passed=False,
                message="CLI not installed",
                fix_instruction="To fix: pip install -e .claude/scripts/"
            )

    except FileNotFoundError:
        return ValidationResult(
            passed=False,
            message="CLI not installed",
            fix_instruction="To fix: pip install -e .claude/scripts/"
        )

    except subprocess.TimeoutExpired:
        return ValidationResult(
            passed=False,
            message="CLI command timed out",
            fix_instruction="To fix: Check for hanging processes and reinstall: pip install -e .claude/scripts/"
        )

    except Exception as e:
        return ValidationResult(
            passed=False,
            message=f"CLI check error: {e}",
            fix_instruction="To fix: pip install -e .claude/scripts/"
        )


def check_context_files(project_root: str) -> ValidationResult:
    """
    Check 2: Verify all 6 context files exist.

    Checks: devforgeai/specs/context/{tech-stack,source-tree,...}.md

    Args:
        project_root: Path to project root directory.

    Returns:
        ValidationResult with count of present files and missing list.
    """
    root_path = Path(project_root)
    context_dir = root_path / "devforgeai" / "specs" / "context"

    present_files = []
    missing_files = []

    for filename in REQUIRED_CONTEXT_FILES:
        file_path = context_dir / filename
        if file_path.exists():
            present_files.append(filename)
        else:
            missing_files.append(filename)

    present_count = len(present_files)
    total_count = len(REQUIRED_CONTEXT_FILES)

    if present_count == total_count:
        return ValidationResult(
            passed=True,
            message=f"Context files: {present_count}/{total_count} present"
        )
    else:
        missing_str = ", ".join(missing_files)
        return ValidationResult(
            passed=False,
            message=f"Context files: {present_count}/{total_count} present. Missing: {missing_str}",
            fix_instruction="To fix: Run /create-context to generate missing context files"
        )


def check_hooks_installed(project_root: str) -> ValidationResult:
    """
    Check 3: Verify Git hooks are installed.

    Checks: .git/hooks/pre-commit exists

    Args:
        project_root: Path to project root directory.

    Returns:
        ValidationResult indicating hook installation status.
    """
    root_path = Path(project_root)
    hooks_dir = root_path / ".git" / "hooks"
    pre_commit = hooks_dir / "pre-commit"

    if pre_commit.exists():
        return ValidationResult(
            passed=True,
            message="Hooks: pre-commit installed"
        )
    elif not (root_path / ".git").exists():
        return ValidationResult(
            passed=False,
            message="Hooks not installed (no .git directory)",
            fix_instruction="To fix: Initialize Git first (git init), then run: bash .claude/scripts/install_hooks.sh"
        )
    else:
        return ValidationResult(
            passed=False,
            message="Hooks not installed",
            fix_instruction="To fix: Run: bash .claude/scripts/install_hooks.sh"
        )


def check_pythonpath() -> ValidationResult:
    """
    Check 4: Verify PYTHONPATH is configured correctly.

    Tests that devforgeai_cli module can be imported successfully.

    Returns:
        ValidationResult indicating PYTHONPATH configuration status.
    """
    try:
        # Try to import the CLI module - if we're running, it's working
        import devforgeai_cli
        return ValidationResult(
            passed=True,
            message="PYTHONPATH: configured correctly"
        )
    except ImportError:
        return ValidationResult(
            passed=False,
            message="PYTHONPATH not configured",
            fix_instruction="To fix: See coding-standards.md for PYTHONPATH configuration. "
                          "Run: export PYTHONPATH=\".:$PYTHONPATH\""
        )


def check_git_repository(project_root: str) -> ValidationResult:
    """
    Check 5: Verify project is a Git repository.

    Checks: .git/ directory exists

    Args:
        project_root: Path to project root directory.

    Returns:
        ValidationResult indicating Git repository status.
    """
    root_path = Path(project_root)
    git_dir = root_path / ".git"

    if git_dir.exists() and git_dir.is_dir():
        return ValidationResult(
            passed=True,
            message="Git repository: initialized"
        )
    else:
        return ValidationResult(
            passed=False,
            message="Not a Git repository",
            fix_instruction="To fix: Run: git init"
        )


def check_settings_file(project_root: str) -> ValidationResult:
    """
    Check 6: Verify Claude settings file exists.

    Checks: .claude/settings.json exists

    Args:
        project_root: Path to project root directory.

    Returns:
        ValidationResult indicating settings file status.
    """
    root_path = Path(project_root)
    settings_file = root_path / ".claude" / "settings.json"

    if settings_file.exists():
        return ValidationResult(
            passed=True,
            message="Settings file: present"
        )
    else:
        return ValidationResult(
            passed=False,
            message="Settings missing",
            fix_instruction="To fix: Run installer to create .claude/settings.json"
        )


# =============================================================================
# Main Command Function
# =============================================================================

def validate_installation_command(project_root: str) -> InstallationValidationResult:
    """
    Run all 6 installation validation checks.

    Args:
        project_root: Path to project root directory.

    Returns:
        InstallationValidationResult with all check results.
    """
    checks = []

    # Run all 6 checks
    checks.append(check_cli_available())
    checks.append(check_context_files(project_root))
    checks.append(check_hooks_installed(project_root))
    checks.append(check_pythonpath())
    checks.append(check_git_repository(project_root))
    checks.append(check_settings_file(project_root))

    # Calculate totals
    passed_count = sum(1 for check in checks if check.passed)
    failed_count = sum(1 for check in checks if not check.passed)
    success = failed_count == 0

    return InstallationValidationResult(
        success=success,
        passed_count=passed_count,
        failed_count=failed_count,
        checks=checks,
        exit_code=0 if success else 1
    )


# =============================================================================
# CLI Entry Point (for standalone use)
# =============================================================================

def main(project_root: str = ".", output_format: str = "text") -> int:
    """
    Main entry point for validate-installation command.

    Args:
        project_root: Project root directory.
        output_format: Output format ('text' or 'json').

    Returns:
        Exit code: 0 if all checks pass, 1 if any fail.
    """
    result = validate_installation_command(project_root)

    if output_format == "json":
        import json
        output = {
            "success": result.success,
            "passed_count": result.passed_count,
            "failed_count": result.failed_count,
            "exit_code": result.exit_code,
            "summary": result.summary,
            "checks": [
                {
                    "passed": check.passed,
                    "message": check.message,
                    "fix_instruction": check.fix_instruction
                }
                for check in result.checks
            ]
        }
        print(json.dumps(output, indent=2))
    else:
        print(result.format_output())

    return result.exit_code


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(
        description="Validate DevForgeAI installation"
    )
    parser.add_argument(
        "--project-root",
        default=".",
        help="Project root directory (default: current directory)"
    )
    parser.add_argument(
        "--format",
        choices=["text", "json"],
        default="text",
        help="Output format (default: text)"
    )

    args = parser.parse_args()

    exit_code = main(args.project_root, args.format)
    sys.exit(exit_code)
