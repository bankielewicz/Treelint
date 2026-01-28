#!/usr/bin/env python3
"""
Definition of Done (DoD) Validator

Prevents autonomous deferrals by validating user approval markers.

Based on research patterns:
- SpecDriven AI: spec_validator.py (spec-test traceability)
- GitHub DoD Checker: checkbox status validation
- Industry traceability: explicit approval markers

This validator catches the exact issue from tmp/output.md where Claude
marked DoD item as [x] but deferred implementation without user approval.
"""

import sys
from pathlib import Path
from typing import Dict, List, Tuple

from ..utils.story_analyzer import (
    load_story_file,
    extract_dod_items,
    extract_impl_notes_items,
    find_dod_impl_mismatch,
    check_user_approval_marker,
    has_implementation_notes,
    extract_story_references,
    extract_adr_references
)
from ..utils.markdown_parser import extract_item_justification, extract_section


class DoDValidator:
    """Validates Definition of Done completion and deferral justifications."""

    def __init__(self, project_root: str = "."):
        self.project_root = Path(project_root)

    def validate(self, story_file: str) -> Tuple[bool, List[Dict]]:
        """
        Validate DoD completion for a story file.

        Args:
            story_file: Path to story file (relative to project root or absolute)

        Returns:
            Tuple of (is_valid, violations)

        Violations include:
        - CRITICAL: Autonomous deferrals (no user approval)
        - CRITICAL: DoD [x] but missing from Implementation Notes
        - HIGH: Referenced stories don't exist
        - HIGH: Missing Implementation Notes section
        - MEDIUM: Deferred items with incomplete justifications
        """
        violations = []

        # Resolve story file path relative to project root
        story_path = Path(story_file)
        if not story_path.is_absolute():
            story_path = self.project_root / story_path

        try:
            frontmatter, content = load_story_file(str(story_path))
        except FileNotFoundError as e:
            return False, [{'severity': 'CRITICAL', 'error': str(e)}]

        story_id = frontmatter.get('id', 'UNKNOWN') if frontmatter else 'UNKNOWN'

        # Check 1: Implementation Notes section must exist
        if not has_implementation_notes(content):
            violations.append({
                'story_id': story_id,
                'severity': 'HIGH',
                'error': 'Implementation Notes section missing',
                'fix': 'Add ## Implementation Notes section to story file'
            })
            # Can't continue validation without Implementation Notes
            return False, violations

        # Check 2: Find DoD vs Implementation mismatches
        mismatches = find_dod_impl_mismatch(content)

        for mismatch in mismatches:
            item_text = mismatch['item']

            if not mismatch['impl_found']:
                # DoD item not in Implementation Notes
                violations.append({
                    'story_id': story_id,
                    'item': item_text,
                    'severity': 'CRITICAL',
                    'error': 'DoD item marked [x] but missing from Implementation Notes',
                    'dod_status': '[x]' if mismatch['dod_checked'] else '[ ]',
                    'impl_status': 'NOT FOUND',
                    'fix': f'Add "- [x] {item_text} - Completed: ..." to Implementation Notes'
                })

            elif mismatch['dod_checked'] and not mismatch['impl_checked']:
                # AUTONOMOUS DEFERRAL: DoD [x] but Impl [ ]
                # This is the exact issue from tmp/output.md

                impl_section = extract_section(content, "Implementation Notes")
                justification = extract_item_justification(impl_section, item_text)

                # Check for user approval marker
                has_approval, marker_type = check_user_approval_marker(justification)

                if not has_approval:
                    violations.append({
                        'story_id': story_id,
                        'item': item_text,
                        'severity': 'CRITICAL',
                        'error': 'AUTONOMOUS DEFERRAL DETECTED - DoD marked [x] but deferred without user approval',
                        'dod_status': '[x]',
                        'impl_status': '[ ]',
                        'justification': justification[:200] if justification else None,
                        'fix': 'Add user approval marker: "User approved: YES" OR STORY-XXX/ADR-XXX reference',
                        'violation_type': 'autonomous_deferral'
                    })
                else:
                    # Has approval marker, validate references exist
                    violations.extend(self._validate_references(
                        story_id, item_text, justification, marker_type
                    ))

        # Check 3: Validate all deferred items have justifications
        impl_items = extract_impl_notes_items(content)

        for impl_item in impl_items:
            if not impl_item['checked']:  # Deferred item
                item_text = impl_item['text'].split(' - ')[0].strip()

                impl_section = extract_section(content, "Implementation Notes")
                justification = extract_item_justification(impl_section, item_text)

                if not justification or len(justification.strip()) < 10:
                    violations.append({
                        'story_id': story_id,
                        'item': item_text,
                        'severity': 'MEDIUM',
                        'error': 'Deferred item has insufficient justification',
                        'justification': justification if justification else '(none)',
                        'fix': 'Add detailed justification explaining deferral reason'
                    })

        is_valid = len([v for v in violations if v['severity'] in ['CRITICAL', 'HIGH']]) == 0

        return is_valid, violations

    def _validate_references(self, story_id: str, item_text: str, justification: str, marker_type: str) -> List[Dict]:
        """
        Validate that referenced stories/ADRs exist.

        Args:
            story_id: Current story ID
            item_text: DoD item text
            justification: Justification text
            marker_type: Type of approval marker found

        Returns:
            List of violations if references don't exist
        """
        violations = []

        if marker_type == 'story_reference':
            story_refs = extract_story_references(justification)

            for ref in story_refs:
                if not self._story_exists(ref):
                    violations.append({
                        'story_id': story_id,
                        'item': item_text,
                        'severity': 'HIGH',
                        'error': f'Referenced story {ref} does not exist',
                        'fix': f'Create {ref} or update reference to existing story'
                    })

        if marker_type == 'adr_reference':
            adr_refs = extract_adr_references(justification)

            for ref in adr_refs:
                if not self._adr_exists(ref):
                    violations.append({
                        'story_id': story_id,
                        'item': item_text,
                        'severity': 'HIGH',
                        'error': f'Referenced ADR {ref} does not exist',
                        'fix': f'Create {ref} or update reference to existing ADR'
                    })

        return violations

    def _story_exists(self, story_id: str) -> bool:
        """Check if story file exists in project."""
        story_files = list(self.project_root.glob(f"devforgeai/specs/Stories/{story_id}*.story.md"))
        return len(story_files) > 0

    def _adr_exists(self, adr_id: str) -> bool:
        """Check if ADR file exists in project."""
        adr_files = list(self.project_root.glob(f"devforgeai/specs/adrs/{adr_id}*.md"))
        return len(adr_files) > 0


def validate_dod(story_file: str, output_format: str = 'text', project_root: str = '.') -> int:
    """
    Main validator entry point.

    Args:
        story_file: Path to story file
        output_format: 'text' or 'json'
        project_root: Project root directory

    Returns:
        Exit code: 0 = valid, 1 = violations found, 2 = error
    """
    validator = DoDValidator(project_root)

    try:
        is_valid, violations = validator.validate(story_file)

        if output_format == 'json':
            import json
            result = {
                'valid': is_valid,
                'violations': violations,
                'story_file': story_file
            }
            print(json.dumps(result, indent=2))

        else:
            # Text output
            if is_valid:
                print(f"✅ {Path(story_file).name}: All DoD items validated")
                return 0
            else:
                print(f"❌ VALIDATION FAILED: {Path(story_file).name}\n")

                # Group by severity
                critical = [v for v in violations if v['severity'] == 'CRITICAL']
                high = [v for v in violations if v['severity'] == 'HIGH']
                medium = [v for v in violations if v['severity'] == 'MEDIUM']

                if critical:
                    print("CRITICAL VIOLATIONS:")
                    for v in critical:
                        print(f"  • {v.get('item', v.get('story_id', 'N/A'))}")
                        print(f"    Error: {v['error']}")
                        if 'dod_status' in v:
                            print(f"    DoD: {v['dod_status']} | Impl: {v['impl_status']}")
                        if 'justification' in v and v['justification']:
                            print(f"    Found: {v['justification']}")
                        if 'fix' in v:
                            print(f"    Fix: {v['fix']}")
                        print()

                if high:
                    print("HIGH VIOLATIONS:")
                    for v in high:
                        print(f"  • {v.get('item', 'N/A')}")
                        print(f"    Error: {v['error']}")
                        print(f"    Fix: {v['fix']}")
                        print()

                if medium:
                    print("MEDIUM VIOLATIONS:")
                    for v in medium:
                        print(f"  • {v.get('item', v.get('story_id', 'N/A'))}")
                        print(f"    Error: {v['error']}")
                        if 'fix' in v:
                            print(f"    Fix: {v['fix']}")
                        print()

                print("=" * 80)
                print("GIT COMMIT BLOCKED - Fix violations before committing")
                print("=" * 80)
                print("\nRequired for all deferred DoD items:")
                print("  1. Reference to follow-up story (STORY-XXX), OR")
                print("  2. Reference to scope change ADR (ADR-XXX), OR")
                print("  3. External blocker: 'Blocked by: ... (external)', OR")
                print("  4. Explicit marker: 'User approved: [reason]'")
                print("\nTo fix:")
                print("  1. Re-run /dev command and answer AskUserQuestion for each deferral, OR")
                print("  2. Manually add approval marker to Implementation Notes")
                print()

                return 1

    except Exception as e:
        print(f"ERROR: Validation failed with exception: {e}", file=sys.stderr)
        return 2


if __name__ == '__main__':
    import argparse

    parser = argparse.ArgumentParser(
        description='Validate Definition of Done completion and detect autonomous deferrals'
    )
    parser.add_argument('story_file', help='Path to story file (.story.md)')
    parser.add_argument('--format', choices=['text', 'json'], default='text',
                        help='Output format (default: text)')
    parser.add_argument('--project-root', default='.',
                        help='Project root directory (default: current directory)')

    args = parser.parse_args()

    exit_code = validate_dod(args.story_file, args.format, args.project_root)
    sys.exit(exit_code)
