#!/usr/bin/env python3
"""
Context Validation Script

Validates that all 6 required DevForgeAI context files exist and contain valid content.
Part of the DevForgeAI UI Generator skill.

Usage:
    python validate_context.py

Exit Codes:
    0 - All context files valid
    1 - Missing or invalid context files
"""

import os
import sys
from pathlib import Path
from typing import List, Tuple


# Required context files
REQUIRED_CONTEXT_FILES = [
    "tech-stack.md",
    "source-tree.md",
    "dependencies.md",
    "coding-standards.md",
    "architecture-constraints.md",
    "anti-patterns.md",
]

# Placeholder content to detect
PLACEHOLDER_MARKERS = ["TODO", "TBD", "[PLACEHOLDER]", "FIXME", "XXX"]


def validate_context() -> Tuple[bool, List[str]]:
    """
    Validate all required context files.

    Returns:
        Tuple of (success: bool, errors: List[str])
    """
    # Determine project root (3 levels up from script location)
    script_dir = Path(__file__).resolve().parent
    project_root = script_dir.parent.parent.parent.parent
    context_dir = project_root / "devforgeai" / "context"

    errors = []

    # Check if context directory exists
    if not context_dir.exists():
        errors.append(f"Context directory does not exist: {context_dir}")
        return False, errors

    # Check each required file
    for filename in REQUIRED_CONTEXT_FILES:
        file_path = context_dir / filename

        # Check file exists
        if not file_path.exists():
            errors.append(f"Missing required file: {filename}")
            continue

        # Check file is not empty
        try:
            content = file_path.read_text(encoding="utf-8")

            if not content.strip():
                errors.append(f"File is empty: {filename}")
                continue

            # Check for placeholder content
            for marker in PLACEHOLDER_MARKERS:
                if marker in content:
                    errors.append(
                        f"File contains placeholder '{marker}': {filename}"
                    )
                    break

        except Exception as e:
            errors.append(f"Error reading {filename}: {e}")

    success = len(errors) == 0
    return success, errors


def main():
    """Main entry point."""
    print("🔍 Validating DevForgeAI context files...")
    print()

    success, errors = validate_context()

    if success:
        print("✅ All 6 context files are valid!")
        print()
        print("Context files validated:")
        for filename in REQUIRED_CONTEXT_FILES:
            print(f"  ✓ {filename}")
        return 0
    else:
        print("❌ Context validation failed!")
        print()
        print("Errors found:")
        for error in errors:
            print(f"  ✗ {error}")
        print()
        print("Resolution:")
        print("  Please invoke the 'devforgeai-architecture' skill to create")
        print("  or update the required context files.")
        return 1


if __name__ == "__main__":
    sys.exit(main())
