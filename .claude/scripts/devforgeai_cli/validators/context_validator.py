#!/usr/bin/env python3
"""
Context Files Validator

Validates all 6 DevForgeAI context files exist and are non-empty.
Quality gate before development begins.
"""

import sys
from pathlib import Path
from typing import Dict, List, Tuple


# The 6 required context files
REQUIRED_CONTEXT_FILES = [
    'tech-stack.md',
    'source-tree.md',
    'dependencies.md',
    'coding-standards.md',
    'architecture-constraints.md',
    'anti-patterns.md'
]


def check_context_files(directory: str = ".") -> Tuple[bool, List[Dict]]:
    """
    Check if all 6 context files exist and are non-empty.

    Args:
        directory: Project root directory

    Returns:
        Tuple of (all_valid, violations)

    Violations include:
    - CRITICAL: Missing context file
    - HIGH: Empty context file (placeholder)
    - MEDIUM: File exists but <100 bytes (likely incomplete)
    """
    violations = []
    project_root = Path(directory)
    context_dir = project_root / "devforgeai" / "context"

    # Check if context directory exists
    if not context_dir.exists():
        violations.append({
            'severity': 'CRITICAL',
            'file': 'devforgeai/specs/context/',
            'error': 'Context directory does not exist',
            'fix': 'Run /create-context to generate context files'
        })
        return False, violations

    # Check each required file
    for filename in REQUIRED_CONTEXT_FILES:
        file_path = context_dir / filename

        if not file_path.exists():
            violations.append({
                'severity': 'CRITICAL',
                'file': filename,
                'error': f'Context file missing',
                'path': str(file_path),
                'fix': f'Create {filename} or run /create-context'
            })

        elif file_path.stat().st_size == 0:
            violations.append({
                'severity': 'HIGH',
                'file': filename,
                'error': 'Context file is empty (placeholder)',
                'path': str(file_path),
                'fix': f'Populate {filename} with project-specific content'
            })

        elif file_path.stat().st_size < 100:
            violations.append({
                'severity': 'MEDIUM',
                'file': filename,
                'error': f'Context file is very small ({file_path.stat().st_size} bytes)',
                'path': str(file_path),
                'fix': f'Review and expand {filename} content'
            })

    is_valid = len([v for v in violations if v['severity'] in ['CRITICAL', 'HIGH']]) == 0

    return is_valid, violations


def validate_context(directory: str = ".", output_format: str = 'text') -> int:
    """
    Main validator entry point.

    Args:
        directory: Project root directory
        output_format: 'text' or 'json'

    Returns:
        Exit code: 0 = all files valid, 1 = violations found, 2 = error
    """
    try:
        is_valid, violations = check_context_files(directory)

        if output_format == 'json':
            import json
            result = {
                'valid': is_valid,
                'violations': violations,
                'directory': str(Path(directory).resolve())
            }
            print(json.dumps(result, indent=2))

        else:
            # Text output
            if is_valid:
                print(f"✅ All 6 context files validated")
                print(f"   Location: {Path(directory).resolve()}/devforgeai/specs/context/")
                return 0
            else:
                print(f"❌ CONTEXT VALIDATION FAILED\n")

                critical = [v for v in violations if v['severity'] == 'CRITICAL']
                high = [v for v in violations if v['severity'] == 'HIGH']
                medium = [v for v in violations if v['severity'] == 'MEDIUM']

                if critical:
                    print("CRITICAL - Missing Files:")
                    for v in critical:
                        print(f"  • {v['file']}")
                        print(f"    Error: {v['error']}")
                        print(f"    Fix: {v['fix']}")
                        print()

                if high:
                    print("HIGH - Empty Files:")
                    for v in high:
                        print(f"  • {v['file']}")
                        print(f"    Path: {v['path']}")
                        print(f"    Fix: {v['fix']}")
                        print()

                if medium:
                    print("MEDIUM - Incomplete Files:")
                    for v in medium:
                        print(f"  • {v['file']} ({v['error']})")
                        print()

                print("=" * 80)
                print("DEVELOPMENT BLOCKED - Context files required")
                print("=" * 80)
                print("\nAll 6 context files must exist:")
                for filename in REQUIRED_CONTEXT_FILES:
                    status = "✅" if not any(v['file'] == filename for v in critical + high) else "❌"
                    print(f"  {status} {filename}")
                print("\nTo create context files:")
                print("  Run: /create-context <project-name>")
                print()

                return 1

    except Exception as e:
        print(f"ERROR: Validation failed with exception: {e}", file=sys.stderr)
        return 2


if __name__ == '__main__':
    import argparse

    parser = argparse.ArgumentParser(
        description='Validate DevForgeAI context files exist and are non-empty'
    )
    parser.add_argument('--directory', default='.',
                        help='Project root directory (default: current directory)')
    parser.add_argument('--format', choices=['text', 'json'], default='text',
                        help='Output format (default: text)')

    args = parser.parse_args()

    exit_code = validate_context(args.directory, args.format)
    sys.exit(exit_code)
