#!/usr/bin/env python3
"""
Git Availability Validator

Checks if directory is a Git repository.
Prevents RCA-006 errors by validating Git before workflow commands execute.

Uses proven pattern: git rev-parse --is-inside-work-tree
"""

import subprocess
import sys
from pathlib import Path
from typing import Tuple


def check_git(directory: str = ".") -> Tuple[bool, str]:
    """
    Check if directory is a Git repository.

    Args:
        directory: Directory to check (default: current directory)

    Returns:
        Tuple of (is_git_repo, message)

    Uses the proven pattern: git rev-parse --is-inside-work-tree
    This is the same pattern used in devforgeai-development skill Phase 0.
    """
    try:
        result = subprocess.run(
            ["git", "rev-parse", "--is-inside-work-tree"],
            cwd=directory,
            capture_output=True,
            text=True,
            timeout=5
        )

        if result.returncode == 0 and result.stdout.strip() == "true":
            return True, f"✅ Git repository detected: {Path(directory).resolve()}"
        else:
            return False, f"❌ Not a Git repository: {Path(directory).resolve()}"

    except subprocess.TimeoutExpired:
        return False, "❌ Git command timed out"

    except FileNotFoundError:
        return False, "❌ Git not installed or not in PATH"

    except Exception as e:
        return False, f"❌ Error checking Git: {e}"


def validate_git(directory: str = ".", output_format: str = 'text') -> int:
    """
    Main validator entry point.

    Args:
        directory: Directory to check
        output_format: 'text' or 'json'

    Returns:
        Exit code: 0 = Git available, 1 = Git not available, 2 = error
    """
    is_git, message = check_git(directory)

    if output_format == 'json':
        import json
        result = {
            'git_available': is_git,
            'message': message,
            'directory': str(Path(directory).resolve())
        }
        print(json.dumps(result, indent=2))
    else:
        print(message)

        if not is_git:
            print("\nDevForgeAI workflows require Git:")
            print("  - /dev (TDD development)")
            print("  - /qa (quality validation)")
            print("  - /release (deployment)")
            print("  - /orchestrate (full lifecycle)")
            print("\nTo initialize Git:")
            print("  git init")
            print("  git add .")
            print("  git commit -m 'Initial commit'")
            print()

    return 0 if is_git else 1


if __name__ == '__main__':
    import argparse

    parser = argparse.ArgumentParser(
        description='Check if directory is a Git repository'
    )
    parser.add_argument('--directory', default='.',
                        help='Directory to check (default: current directory)')
    parser.add_argument('--format', choices=['text', 'json'], default='text',
                        help='Output format (default: text)')

    args = parser.parse_args()

    exit_code = validate_git(args.directory, args.format)
    sys.exit(exit_code)
