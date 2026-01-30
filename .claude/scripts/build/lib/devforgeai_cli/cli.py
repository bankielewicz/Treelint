#!/usr/bin/env python3
"""
DevForgeAI CLI - Main Entry Point

Command-line interface for DevForgeAI workflow validators.

Commands:
  validate-dod     Validate Definition of Done completion
  check-git        Check Git repository availability
  validate-context Validate context files exist
  check-hooks      Check if hooks should trigger for an operation
  invoke-hooks     Invoke devforgeai-feedback skill for operation

Based on industry research (SpecDriven AI, pre-commit patterns, DoD checkers).
"""

import sys
import argparse
from pathlib import Path


def main():
    """Main CLI entry point."""
    parser = argparse.ArgumentParser(
        prog='devforgeai',
        description='DevForgeAI Workflow Validators',
        epilog='For detailed help: devforgeai <command> --help'
    )

    parser.add_argument('--version', action='version', version='%(prog)s 0.1.0')

    subparsers = parser.add_subparsers(dest='command', help='Available commands')

    # ======================================================================
    # validate-dod command
    # ======================================================================
    dod_parser = subparsers.add_parser(
        'validate-dod',
        help='Validate Definition of Done completion',
        description='Detects autonomous deferrals and validates user approval markers'
    )
    dod_parser.add_argument(
        'story_file',
        help='Path to story file (.story.md)'
    )
    dod_parser.add_argument(
        '--format',
        choices=['text', 'json'],
        default='text',
        help='Output format (default: text)'
    )
    dod_parser.add_argument(
        '--project-root',
        default='.',
        help='Project root directory (default: current directory)'
    )

    # ======================================================================
    # check-git command
    # ======================================================================
    git_parser = subparsers.add_parser(
        'check-git',
        help='Check if directory is a Git repository',
        description='Validates Git availability for DevForgeAI workflows'
    )
    git_parser.add_argument(
        '--directory',
        default='.',
        help='Directory to check (default: current directory)'
    )
    git_parser.add_argument(
        '--format',
        choices=['text', 'json'],
        default='text',
        help='Output format (default: text)'
    )

    # ======================================================================
    # validate-context command
    # ======================================================================
    context_parser = subparsers.add_parser(
        'validate-context',
        help='Validate context files exist',
        description='Checks all 6 DevForgeAI context files are present and non-empty'
    )
    context_parser.add_argument(
        '--directory',
        default='.',
        help='Project root directory (default: current directory)'
    )
    context_parser.add_argument(
        '--format',
        choices=['text', 'json'],
        default='text',
        help='Output format (default: text)'
    )

    # ======================================================================
    # check-hooks command
    # ======================================================================
    hooks_parser = subparsers.add_parser(
        'check-hooks',
        help='Check if hooks should trigger for an operation',
        description='Validates hook configuration and determines trigger status'
    )
    hooks_parser.add_argument(
        '--operation',
        required=True,
        help='Operation name (e.g., dev, qa, release)'
    )
    hooks_parser.add_argument(
        '--status',
        required=True,
        choices=['success', 'failure', 'partial'],
        help='Operation status'
    )
    hooks_parser.add_argument(
        '--config',
        default=None,
        help='Path to hooks.yaml config file (default: devforgeai/config/hooks.yaml)'
    )

    # ======================================================================
    # invoke-hooks command
    # ======================================================================
    invoke_hooks_parser = subparsers.add_parser(
        'invoke-hooks',
        help='Invoke devforgeai-feedback skill for operation',
        description='Extracts operation context and invokes devforgeai-feedback skill for retrospective feedback'
    )
    invoke_hooks_parser.add_argument(
        '--operation',
        required=True,
        help='Operation name (e.g., dev, qa, release)'
    )
    invoke_hooks_parser.add_argument(
        '--story',
        default=None,
        help='Story ID (format: STORY-NNN)'
    )
    invoke_hooks_parser.add_argument(
        '--verbose',
        action='store_true',
        help='Enable verbose logging'
    )

    # Parse arguments
    args = parser.parse_args()

    # Show help if no command specified
    if not args.command:
        parser.print_help()
        return 0

    # Execute command
    try:
        if args.command == 'validate-dod':
            from .validators.dod_validator import validate_dod
            return validate_dod(args.story_file, args.format, args.project_root)

        elif args.command == 'check-git':
            from .validators.git_validator import validate_git
            return validate_git(args.directory, args.format)

        elif args.command == 'validate-context':
            from .validators.context_validator import validate_context
            return validate_context(args.directory, args.format)

        elif args.command == 'check-hooks':
            from .commands.check_hooks import check_hooks_command
            return check_hooks_command(
                operation=args.operation,
                status=args.status,
                config_path=args.config
            )

        elif args.command == 'invoke-hooks':
            from .commands.invoke_hooks import invoke_hooks_command
            return invoke_hooks_command(
                operation=args.operation,
                story_id=args.story,
                verbose=args.verbose
            )

        else:
            print(f"Unknown command: {args.command}", file=sys.stderr)
            return 2

    except KeyboardInterrupt:
        print("\nInterrupted by user", file=sys.stderr)
        return 130

    except Exception as e:
        print(f"ERROR: {e}", file=sys.stderr)
        return 2


if __name__ == '__main__':
    sys.exit(main())
