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
  phase-init       Initialize phase state file for a story
  phase-check      Check if phase transition is allowed
  phase-complete   Mark a phase as complete
  phase-status     Display current phase status
  phase-record     Record subagent invocation for a phase
  ast-grep scan    Semantic code analysis with ast-grep or grep fallback

Based on industry research (SpecDriven AI, pre-commit patterns, DoD checkers).
"""

import sys
import argparse
from pathlib import Path


def main():
    """Main CLI entry point."""
    parser = argparse.ArgumentParser(
        prog='devforgeai-validate',
        description='DevForgeAI Workflow Validators',
        epilog='For detailed help: devforgeai-validate <command> --help'
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

    # ======================================================================
    # phase-init command (STORY-148)
    # ======================================================================
    phase_init_parser = subparsers.add_parser(
        'phase-init',
        help='Initialize phase state file for a story',
        description='Creates a new phase state file to track TDD workflow execution'
    )
    phase_init_parser.add_argument(
        'story_id',
        help='Story ID (format: STORY-XXX)'
    )
    phase_init_parser.add_argument(
        '--project-root',
        default='.',
        help='Project root directory (default: current directory)'
    )
    phase_init_parser.add_argument(
        '--format',
        choices=['text', 'json'],
        default='text',
        help='Output format (default: text)'
    )

    # ======================================================================
    # phase-check command (STORY-148)
    # ======================================================================
    phase_check_parser = subparsers.add_parser(
        'phase-check',
        help='Check if phase transition is allowed',
        description='Validates that phase transition follows sequential order and all subagents were invoked'
    )
    phase_check_parser.add_argument(
        'story_id',
        help='Story ID (format: STORY-XXX)'
    )
    phase_check_parser.add_argument(
        '--from',
        dest='from_phase',
        required=True,
        help='Source phase (e.g., 01)'
    )
    phase_check_parser.add_argument(
        '--to',
        dest='to_phase',
        required=True,
        help='Target phase (e.g., 02)'
    )
    phase_check_parser.add_argument(
        '--project-root',
        default='.',
        help='Project root directory'
    )
    phase_check_parser.add_argument(
        '--format',
        choices=['text', 'json'],
        default='text',
        help='Output format'
    )

    # ======================================================================
    # phase-complete command (STORY-148)
    # ======================================================================
    phase_complete_parser = subparsers.add_parser(
        'phase-complete',
        help='Mark a phase as complete',
        description='Updates phase status to completed and advances current phase'
    )
    phase_complete_parser.add_argument(
        'story_id',
        help='Story ID (format: STORY-XXX)'
    )
    phase_complete_parser.add_argument(
        '--phase',
        required=True,
        help='Phase to complete (e.g., 02)'
    )
    phase_complete_parser.add_argument(
        '--checkpoint-passed',
        action='store_true',
        default=True,
        help='Whether checkpoint validation passed'
    )
    phase_complete_parser.add_argument(
        '--checkpoint-failed',
        action='store_true',
        help='Mark checkpoint as failed'
    )
    phase_complete_parser.add_argument(
        '--project-root',
        default='.',
        help='Project root directory'
    )
    phase_complete_parser.add_argument(
        '--format',
        choices=['text', 'json'],
        default='text',
        help='Output format'
    )

    # ======================================================================
    # phase-status command (STORY-148)
    # ======================================================================
    phase_status_parser = subparsers.add_parser(
        'phase-status',
        help='Display current phase status',
        description='Shows workflow progress and phase completion status'
    )
    phase_status_parser.add_argument(
        'story_id',
        help='Story ID (format: STORY-XXX)'
    )
    phase_status_parser.add_argument(
        '--project-root',
        default='.',
        help='Project root directory'
    )
    phase_status_parser.add_argument(
        '--format',
        choices=['text', 'json'],
        default='text',
        help='Output format'
    )

    # ======================================================================
    # phase-record command (STORY-149 AC#4)
    # ======================================================================
    phase_record_parser = subparsers.add_parser(
        'phase-record',
        help='Record subagent invocation for a phase',
        description='Appends subagent to phase subagents_invoked list (idempotent)'
    )
    phase_record_parser.add_argument(
        'story_id',
        help='Story ID (format: STORY-XXX)'
    )
    phase_record_parser.add_argument(
        '--phase',
        required=True,
        help='Phase ID (01-10)'
    )
    phase_record_parser.add_argument(
        '--subagent',
        required=True,
        help='Subagent name that was invoked'
    )
    phase_record_parser.add_argument(
        '--project-root',
        default='.',
        help='Project root directory (default: current directory)'
    )
    phase_record_parser.add_argument(
        '--format',
        choices=['text', 'json'],
        default='text',
        help='Output format (default: text)'
    )

    # ======================================================================
    # phase-observe command (STORY-188)
    # ======================================================================
    phase_observe_parser = subparsers.add_parser(
        'phase-observe',
        help='Record workflow observation for a phase',
        description='Captures friction, gaps, successes, and patterns during TDD workflow execution'
    )
    phase_observe_parser.add_argument(
        'story_id',
        help='Story ID (format: STORY-XXX)'
    )
    phase_observe_parser.add_argument(
        '--phase',
        required=True,
        help='Phase ID (01-10)'
    )
    phase_observe_parser.add_argument(
        '--category',
        required=True,
        choices=['friction', 'gap', 'success', 'pattern'],
        help='Observation category'
    )
    phase_observe_parser.add_argument(
        '--note',
        required=True,
        help='Observation description'
    )
    phase_observe_parser.add_argument(
        '--severity',
        default='medium',
        choices=['low', 'medium', 'high'],
        help='Severity level (default: medium)'
    )
    phase_observe_parser.add_argument(
        '--project-root',
        default='.',
        help='Project root directory (default: current directory)'
    )
    phase_observe_parser.add_argument(
        '--format',
        choices=['text', 'json'],
        default='text',
        help='Output format (default: text)'
    )

    # ======================================================================
    # validate-installation command (STORY-314)
    # ======================================================================
    validate_install_parser = subparsers.add_parser(
        'validate-installation',
        help='Validate DevForgeAI installation completeness',
        description='Runs 6 checks to verify installation: CLI, context files, hooks, PYTHONPATH, Git, settings'
    )
    validate_install_parser.add_argument(
        '--project-root',
        default='.',
        help='Project root directory (default: current directory)'
    )
    validate_install_parser.add_argument(
        '--format',
        choices=['text', 'json'],
        default='text',
        help='Output format (default: text)'
    )

    # ======================================================================
    # ast-grep command (STORY-115)
    # ======================================================================
    astgrep_parser = subparsers.add_parser(
        'ast-grep',
        help='Semantic code analysis with ast-grep',
        description='Analyze code using ast-grep patterns or grep fallback'
    )
    astgrep_subparsers = astgrep_parser.add_subparsers(dest='ast_grep_subcommand')

    # ast-grep scan subcommand
    scan_parser = astgrep_subparsers.add_parser(
        'scan',
        help='Scan directory for code violations'
    )
    scan_parser.add_argument(
        'path',
        help='Directory to scan'
    )
    scan_parser.add_argument(
        '--category',
        choices=['security', 'anti-patterns', 'complexity', 'architecture'],
        help='Filter by rule category'
    )
    scan_parser.add_argument(
        '--language',
        choices=['python', 'csharp', 'typescript', 'javascript'],
        help='Filter by language'
    )
    scan_parser.add_argument(
        '--format',
        choices=['text', 'json', 'markdown'],
        default='text',
        help='Output format (default: text)'
    )
    scan_parser.add_argument(
        '--fallback',
        action='store_true',
        help='Force grep fallback mode (skip ast-grep)'
    )

    # ast-grep init subcommand (STORY-116)
    init_parser = astgrep_subparsers.add_parser(
        'init',
        help='Initialize ast-grep configuration directory'
    )
    init_parser.add_argument(
        '--force',
        action='store_true',
        help='Force overwrite existing configuration'
    )
    init_parser.add_argument(
        '--project-root',
        default='.',
        help='Project root directory (default: current directory)'
    )

    # ast-grep validate-config subcommand (STORY-116)
    validate_config_parser = astgrep_subparsers.add_parser(
        'validate-config',
        help='Validate ast-grep configuration'
    )
    validate_config_parser.add_argument(
        '--config',
        default=None,
        help='Path to sgconfig.yml (default: devforgeai/ast-grep/sgconfig.yml)'
    )
    validate_config_parser.add_argument(
        '--format',
        choices=['text', 'json'],
        default='text',
        help='Output format (default: text)'
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

        elif args.command == 'phase-init':
            from .commands.phase_commands import phase_init_command
            return phase_init_command(
                story_id=args.story_id,
                project_root=args.project_root,
                format=args.format
            )

        elif args.command == 'phase-check':
            from .commands.phase_commands import phase_check_command
            return phase_check_command(
                story_id=args.story_id,
                from_phase=args.from_phase,
                to_phase=args.to_phase,
                project_root=args.project_root,
                format=args.format
            )

        elif args.command == 'phase-complete':
            from .commands.phase_commands import phase_complete_command
            checkpoint = not args.checkpoint_failed if hasattr(args, 'checkpoint_failed') else True
            return phase_complete_command(
                story_id=args.story_id,
                phase=args.phase,
                checkpoint_passed=checkpoint,
                project_root=args.project_root,
                format=args.format
            )

        elif args.command == 'phase-status':
            from .commands.phase_commands import phase_status_command
            return phase_status_command(
                story_id=args.story_id,
                project_root=args.project_root,
                format=args.format
            )

        elif args.command == 'phase-record':
            from .commands.phase_commands import phase_record_command
            return phase_record_command(
                story_id=args.story_id,
                phase=args.phase,
                subagent=args.subagent,
                project_root=args.project_root,
                format=args.format
            )

        elif args.command == 'phase-observe':
            from .commands.phase_commands import phase_observe_command
            return phase_observe_command(
                story_id=args.story_id,
                phase=args.phase,
                category=args.category,
                note=args.note,
                severity=args.severity,
                project_root=args.project_root,
                format=args.format
            )

        elif args.command == 'validate-installation':
            from .commands.validate_installation import main as validate_installation_main
            return validate_installation_main(
                project_root=args.project_root,
                output_format=args.format
            )

        elif args.command == 'ast-grep':
            if args.ast_grep_subcommand == 'scan':
                from .validators.ast_grep_validator import AstGrepValidator
                from .validators.grep_fallback import GrepFallbackAnalyzer, log_fallback_warning

                validator = AstGrepValidator()

                # Check if we should use fallback mode
                use_fallback = args.fallback or validator.config.get('fallback_mode', False)

                if use_fallback or not validator.is_installed():
                    # Use grep fallback
                    log_fallback_warning()
                    analyzer = GrepFallbackAnalyzer()
                    violations = analyzer.analyze_directory(
                        args.path,
                        category=args.category,
                        language=args.language
                    )
                    output = analyzer.format_results(violations, format=args.format)
                    print(output)
                    return 0 if not violations else 1
                else:
                    # Validate installation and version
                    is_valid, violations = validator.validate(args.path)

                    # For now, just report validation status
                    # Full ast-grep integration will be in future stories
                    if args.format == 'json':
                        import json
                        print(json.dumps({"valid": is_valid, "violations": violations}, indent=2))
                    else:
                        if is_valid:
                            print("✓ ast-grep available and compatible")
                        else:
                            print("✗ ast-grep validation failed:")
                            for v in violations:
                                print(f"  {v['severity']}: {v['error']}")

                    return 0 if is_valid else 1

            elif args.ast_grep_subcommand == 'init':
                # STORY-116: Initialize ast-grep configuration
                from pathlib import Path
                from .ast_grep.config_init import ConfigurationInitializer

                project_root = Path(args.project_root).resolve()
                initializer = ConfigurationInitializer(project_root)
                result = initializer.initialize(force=args.force)

                if result.success:
                    print(f"SUCCESS: ast-grep configuration initialized")
                    if result.created_paths:
                        print(f"Created directories:")
                        for p in result.created_paths:
                            print(f"  - {p}")
                    print(f"Configuration file: {result.config_path}")
                    return 0
                else:
                    print(f"ERROR: {result.error}", file=sys.stderr)
                    return 1

            elif args.ast_grep_subcommand == 'validate-config':
                # STORY-116: Validate ast-grep configuration
                from pathlib import Path
                import json as json_module
                from .ast_grep.config_validator import ConfigurationValidator

                if args.config:
                    config_path = Path(args.config)
                else:
                    config_path = Path('devforgeai/ast-grep/sgconfig.yml')

                validator = ConfigurationValidator(config_path)
                result = validator.validate()

                if args.format == 'json':
                    output = {
                        'valid': result.valid,
                        'errors': [
                            {
                                'field': e.field,
                                'message': e.message,
                                'line': e.line
                            }
                            for e in result.errors
                        ],
                        'warnings': result.warnings
                    }
                    print(json_module.dumps(output, indent=2))
                else:
                    if result.valid:
                        print("VALID: Configuration is valid")
                        if result.warnings:
                            for warning in result.warnings:
                                print(f"  WARNING: {warning}")
                    else:
                        print("INVALID: Configuration has errors:")
                        for error in result.errors:
                            line_info = f" (line {error.line})" if error.line else ""
                            print(f"  {error.field.upper()}{line_info}: {error.message}")

                return 0 if result.valid else 1

            else:
                print(f"Unknown ast-grep subcommand: {args.ast_grep_subcommand}", file=sys.stderr)
                return 2

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
