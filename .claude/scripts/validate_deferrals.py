#!/usr/bin/env python3
"""
Lightweight Deferral Format Validator (Hybrid Approach - Layer 1)

Quick format validation for deferred Definition of Done items.
Part of three-layer defense architecture:
  - Layer 1 (this script): Fast format validation (~200 tokens, <100ms)
  - Layer 2 (task file): Interactive user approval checkpoint
  - Layer 3 (subagent): Comprehensive AI analysis (feasibility, circular deps)

This script provides FAST FEEDBACK by catching basic format errors instantly,
allowing comprehensive validation to be handled by AI subagent (deferral-validator).

Usage:
    # Format-only validation (non-blocking warnings)
    python validate_deferrals.py --story-file STORY-006.story.md --format-only

    # Quiet mode (for automation)
    python validate_deferrals.py --story-file STORY-006.story.md --quiet

Exit Codes:
    0 - Format valid OR warnings only (format-only mode)
    1 - Format invalid (strict mode, not used in hybrid approach)
    2 - Configuration error (file not found, parse error)

Design Philosophy:
    - Simple and fast (regex pattern matching only)
    - Non-blocking in hybrid architecture (warnings, not errors)
    - Delegates complex analysis to deferral-validator subagent
    - Follows DevForgeAI validation script patterns (Color class, argparse)

Related:
    - RCA-006 Recommendation 3 (automated validation)
    - Native Tools Efficiency Analysis (40-73% token savings)
    - .claude/agents/deferral-validator.md (comprehensive validation)
"""

import argparse
import re
import sys
from pathlib import Path
from typing import List, Tuple
from dataclasses import dataclass


class Color:
    """
    ANSI color codes for terminal output.

    Pattern from DevForgeAI framework validation scripts:
    - .claude/skills/devforgeai-architecture/scripts/validate_all_context.py
    - .claude/skills/devforgeai-qa/scripts/security_scan.py
    """
    RED = '\033[91m'
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    CYAN = '\033[96m'
    BOLD = '\033[1m'
    END = '\033[0m'


@dataclass
class DoDItem:
    """
    Definition of Done item from story file.

    Attributes:
        text: Item description text
        line_num: Line number in DoD section (1-indexed)
        context: Next 3 lines after item (for justification detection)
    """
    text: str
    line_num: int
    context: str


class FormatValidator:
    """
    Lightweight format validator for deferral justifications.

    Validates ONLY format compliance:
    - Incomplete items have justification text
    - Justification matches expected patterns

    Does NOT validate (delegated to deferral-validator subagent):
    - Story references exist (AI checks file system)
    - ADR references exist (AI checks file system)
    - Circular deferrals (AI analyzes chains)
    - Implementation feasibility (AI analyzes tech spec)
    """

    # Valid deferral patterns (case-insensitive)
    VALID_PATTERNS = [
        r'Deferred to STORY-\d+:',     # Story split deferral
        r'Blocked by:',                # External blocker
        r'Out of scope: ADR-\d+'       # Scope change with ADR
    ]

    def __init__(self, story_file: Path):
        """
        Initialize validator.

        Args:
            story_file: Path to story markdown file
        """
        self.story_file = story_file

    def validate(self) -> Tuple[bool, List[str]]:
        """
        Run format validation.

        Returns:
            Tuple of (is_valid, violations_list)
            - is_valid: True if all incomplete items have valid format
            - violations_list: List of violation messages (empty if valid)
        """
        # Step 1: Read story file
        story_content = self._read_story()

        # Step 2: Extract DoD section
        dod_section = self._extract_dod_section(story_content)

        if not dod_section:
            # No DoD section found - unusual but not an error for this script
            # (Comprehensive validation in subagent will catch this)
            return True, []

        # Step 3: Extract incomplete items
        incomplete_items = self._extract_incomplete_items(dod_section)

        if not incomplete_items:
            # All items complete - valid
            return True, []

        # Step 4: Check format for each incomplete item
        violations = []
        for item in incomplete_items:
            if not self._has_valid_format(item):
                violations.append(
                    f"Line {item.line_num}: '{self._truncate(item.text, 60)}'\n"
                    f"  Missing justification. Expected one of:\n"
                    f"    - Deferred to STORY-XXX: [reason]\n"
                    f"    - Blocked by: [external dependency]\n"
                    f"    - Out of scope: ADR-XXX"
                )

        is_valid = len(violations) == 0
        return is_valid, violations

    def _read_story(self) -> str:
        """
        Read story file content using Python stdlib.

        Note: Uses Python file I/O (not Bash cat) per native tools efficiency guidance.
        While this script is invoked via Bash, it uses Python stdlib internally,
        which is equivalent to native tool efficiency.

        Returns:
            Story file content as string

        Raises:
            SystemExit(2): If file not found or read error
        """
        try:
            with open(self.story_file, 'r', encoding='utf-8') as f:
                return f.read()
        except FileNotFoundError:
            print(
                f"{Color.RED}ERROR: Story file not found: {self.story_file}{Color.END}",
                file=sys.stderr
            )
            sys.exit(2)
        except Exception as e:
            print(
                f"{Color.RED}ERROR reading file: {e}{Color.END}",
                file=sys.stderr
            )
            sys.exit(2)

    def _extract_dod_section(self, content: str) -> str:
        """
        Extract Definition of Done section from story content.

        Searches for markdown section between:
        - Start: ## Definition of Done
        - End: Next ## heading (or end of file)

        Args:
            content: Full story file content

        Returns:
            DoD section content (empty string if not found)
        """
        # Match DoD section (until next ## heading)
        match = re.search(
            r'## Definition of Done\n(.*?)(?:\n## |\Z)',
            content,
            re.DOTALL
        )
        return match.group(1).strip() if match else ""

    def _extract_incomplete_items(self, dod_section: str) -> List[DoDItem]:
        """
        Extract all incomplete (- [ ]) items with context.

        Each item includes the next 3 lines as context for justification detection.

        Args:
            dod_section: Definition of Done section content

        Returns:
            List of DoDItem objects for incomplete items
        """
        lines = dod_section.split('\n')
        items = []

        for i, line in enumerate(lines):
            if line.strip().startswith('- [ ]'):
                # Extract item text (after "- [ ]" marker)
                text = line.strip()[6:].strip()

                # Get next 3 lines as context (for justification detection)
                context_lines = lines[i+1:min(i+4, len(lines))]
                context = '\n'.join(context_lines)

                items.append(DoDItem(
                    text=text,
                    line_num=i+1,
                    context=context
                ))

        return items

    def _has_valid_format(self, item: DoDItem) -> bool:
        """
        Check if item has valid deferral justification format.

        Valid formats (in context lines following item):
        - "Deferred to STORY-XXX: [reason]"
        - "Blocked by: [external dependency]"
        - "Out of scope: ADR-XXX"

        Args:
            item: DoDItem to validate

        Returns:
            True if valid format found in context, False otherwise
        """
        for pattern in self.VALID_PATTERNS:
            if re.search(pattern, item.context, re.IGNORECASE):
                return True
        return False

    @staticmethod
    def _truncate(text: str, max_len: int) -> str:
        """Truncate text to max length with ellipsis."""
        return text if len(text) <= max_len else text[:max_len] + "..."


def main():
    """Main entry point for validation script."""
    parser = argparse.ArgumentParser(
        description="Lightweight deferral format validator (Hybrid Approach - Layer 1)",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
    # Format-only validation (non-blocking)
    python validate_deferrals.py --story-file devforgeai/specs/Stories/STORY-006.story.md --format-only

    # Quiet mode (automation/git hooks)
    python validate_deferrals.py --story-file devforgeai/specs/Stories/STORY-006.story.md --quiet

Exit Codes:
    0 - Validation passed (or warnings only in format-only mode)
    1 - Validation failed (strict mode, not used in hybrid architecture)
    2 - Configuration error (file not found, parse error)

Integration:
    - Invoked by /dev command Phase 2.5a (Layer 1 quick check)
    - Followed by interactive checkpoint (Layer 2)
    - Followed by deferral-validator subagent (Layer 3)
        """
    )

    parser.add_argument(
        "--story-file",
        type=Path,
        required=True,
        help="Path to story file to validate (devforgeai/specs/Stories/STORY-XXX.story.md)"
    )

    parser.add_argument(
        "--format-only",
        action="store_true",
        help="Only validate format (non-blocking, always exit 0 with warnings)"
    )

    parser.add_argument(
        "--quiet",
        action="store_true",
        help="Suppress output (only return exit code)"
    )

    args = parser.parse_args()

    # Validate story file exists (early check)
    if not args.story_file.exists():
        if not args.quiet:
            print(
                f"{Color.RED}ERROR: Story file not found: {args.story_file}{Color.END}",
                file=sys.stderr
            )
        sys.exit(2)

    # Run validation
    validator = FormatValidator(args.story_file)
    is_valid, violations = validator.validate()

    # Display results (unless quiet mode)
    if not args.quiet:
        if is_valid:
            print(f"{Color.GREEN}✓ Deferral format validation PASSED{Color.END}")
            print(f"  All incomplete DoD items have basic justification format")
        else:
            print(f"{Color.YELLOW}⚠️  Format issues detected ({len(violations)} items){Color.END}\n")
            for violation in violations:
                print(f"{Color.YELLOW}{violation}{Color.END}\n")

            if args.format_only:
                print(f"{Color.CYAN}Note: Format-only mode - warnings only (non-blocking){Color.END}")
                print(f"{Color.CYAN}Interactive checkpoint will guide you through resolution.{Color.END}")

    # Exit code handling
    if args.format_only:
        # Format-only mode: Always exit 0 (warnings only, non-blocking)
        sys.exit(0)
    else:
        # Strict mode: Exit 1 on violations (not used in hybrid approach)
        sys.exit(0 if is_valid else 1)


if __name__ == "__main__":
    main()
