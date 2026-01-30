#!/usr/bin/env python3
"""
Release Notes Generator

Generates release notes from story documents and templates.
Updates CHANGELOG.md with new release entry.

Usage:
    python release_notes_generator.py --story STORY-001 --version v1.2.3

Exit Codes:
    0: Success - Release notes generated
    1: Failure - Generation failed

Examples:
    # Basic release notes generation
    python release_notes_generator.py --story STORY-001 --version v1.2.3

    # With QA and metrics reports
    python release_notes_generator.py --story STORY-001 --version v1.2.3 \\
        --qa-report devforgeai/qa/reports/STORY-001-qa-report.md \\
        --metrics-report metrics.json

    # Custom template
    python release_notes_generator.py --story STORY-001 --version v1.2.3 \\
        --template custom-template.md
"""

import argparse
import json
import logging
import re
import sys
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Any

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    datefmt='%Y-%m-%d %H:%M:%S'
)
logger = logging.getLogger(__name__)


def parse_yaml_frontmatter(content: str) -> Dict[str, Any]:
    """
    Parse YAML frontmatter from markdown file.

    Args:
        content: File content with YAML frontmatter

    Returns:
        Dictionary of frontmatter key-value pairs
    """
    # Match YAML frontmatter between --- delimiters
    match = re.match(r'^---\s*\n(.*?)\n---\s*\n', content, re.DOTALL)

    if not match:
        return {}

    yaml_content = match.group(1)
    frontmatter = {}

    # Simple YAML parser (handles basic key: value pairs)
    for line in yaml_content.split('\n'):
        if ':' in line:
            key, value = line.split(':', 1)
            frontmatter[key.strip()] = value.strip()

    return frontmatter


def extract_acceptance_criteria(content: str) -> List[str]:
    """
    Extract acceptance criteria from story document.

    Args:
        content: Story document content

    Returns:
        List of acceptance criteria
    """
    criteria = []

    # Find acceptance criteria section
    match = re.search(r'## Acceptance Criteria\s*\n(.*?)(?=\n##|\Z)', content, re.DOTALL)

    if not match:
        return criteria

    section = match.group(1)

    # Extract list items or numbered items
    for line in section.split('\n'):
        line = line.strip()
        if line.startswith('- ') or line.startswith('* ') or re.match(r'^\d+\.', line):
            # Remove list markers
            criterion = re.sub(r'^[-*]\s+|\d+\.\s+', '', line)
            if criterion:
                criteria.append(criterion)

    return criteria


def load_story_document(story_id: str) -> Dict[str, Any]:
    """
    Load and parse story document.

    Args:
        story_id: Story identifier

    Returns:
        Story data dictionary

    Raises:
        FileNotFoundError: If story file not found
    """
    story_path = Path(f'devforgeai/specs/Stories/{story_id}.story.md')

    if not story_path.exists():
        raise FileNotFoundError(f"Story file not found: {story_path}")

    with open(story_path, 'r') as f:
        content = f.read()

    # Parse frontmatter
    frontmatter = parse_yaml_frontmatter(content)

    # Extract acceptance criteria
    acceptance_criteria = extract_acceptance_criteria(content)

    # Extract changes section
    changes_match = re.search(r'## Changes\s*\n(.*?)(?=\n##|\Z)', content, re.DOTALL)
    changes = changes_match.group(1).strip() if changes_match else ""

    return {
        'id': story_id,
        'title': frontmatter.get('title', 'Unknown'),
        'status': frontmatter.get('status', 'Unknown'),
        'epic': frontmatter.get('epic', 'N/A'),
        'sprint': frontmatter.get('sprint', 'N/A'),
        'created': frontmatter.get('created', 'Unknown'),
        'acceptance_criteria': acceptance_criteria,
        'changes': changes,
        'content': content
    }


def load_qa_report(qa_report_path: str) -> Dict[str, Any]:
    """
    Load and parse QA report.

    Args:
        qa_report_path: Path to QA report file

    Returns:
        QA report data dictionary
    """
    report_file = Path(qa_report_path)

    if not report_file.exists():
        logger.warning(f"QA report not found: {qa_report_path}")
        return {'status': 'N/A', 'coverage': 'N/A'}

    with open(report_file, 'r') as f:
        content = f.read()

    # Extract QA status
    status_match = re.search(r'Status:\s*\*\*(\w+)\*\*', content)
    status = status_match.group(1) if status_match else 'Unknown'

    # Extract coverage percentage
    coverage_match = re.search(r'Coverage:\s*(\d+)%', content)
    coverage = coverage_match.group(1) if coverage_match else 'N/A'

    return {
        'status': status,
        'coverage': coverage
    }


def load_metrics_report(metrics_report_path: str) -> Dict[str, Any]:
    """
    Load and parse metrics report.

    Args:
        metrics_report_path: Path to metrics JSON file

    Returns:
        Metrics data dictionary
    """
    report_file = Path(metrics_report_path)

    if not report_file.exists():
        logger.warning(f"Metrics report not found: {metrics_report_path}")
        return {}

    try:
        with open(report_file, 'r') as f:
            metrics = json.load(f)
        return metrics
    except json.JSONDecodeError:
        logger.warning(f"Invalid JSON in metrics report: {metrics_report_path}")
        return {}


def load_template(template_path: Optional[str] = None) -> str:
    """
    Load release notes template.

    Args:
        template_path: Optional custom template path

    Returns:
        Template content
    """
    if template_path:
        template_file = Path(template_path)
    else:
        # Default template location
        template_file = Path('.claude/skills/devforgeai-release/assets/templates/release-notes-template.md')

    if not template_file.exists():
        logger.warning(f"Template not found: {template_file}")
        logger.warning("Using default template")
        return get_default_template()

    with open(template_file, 'r') as f:
        return f.read()


def get_default_template() -> str:
    """
    Get default release notes template.

    Returns:
        Default template content
    """
    return """# Release Notes: {{VERSION}}

**Release Date:** {{DATE}}
**Story:** {{STORY_ID}} - {{STORY_TITLE}}
**Status:** {{STATUS}}

## Summary

{{SUMMARY}}

## Changes

{{CHANGES}}

## Acceptance Criteria

{{ACCEPTANCE_CRITERIA}}

## QA Validation

- **Status:** {{QA_STATUS}}
- **Coverage:** {{COVERAGE}}%

## Deployment Details

- **Version:** {{VERSION}}
- **Deployed By:** {{DEPLOYER}}
- **Deployment Strategy:** {{DEPLOYMENT_STRATEGY}}
- **Deployed At:** {{DEPLOYED_AT}}

## Rollback Information

- **Previous Version:** {{PREVIOUS_VERSION}}
- **Rollback Command:** `{{ROLLBACK_COMMAND}}`

---

*Generated by DevForgeAI Release Skill*
"""


def populate_template(template: str, data: Dict[str, str]) -> str:
    """
    Populate template with data.

    Args:
        template: Template content with {{VARIABLE}} placeholders
        data: Dictionary of variable values

    Returns:
        Populated template content
    """
    result = template

    # Replace all {{VARIABLE}} placeholders
    for key, value in data.items():
        placeholder = f'{{{{{key}}}}}'
        result = result.replace(placeholder, str(value))

    return result


def generate_changelog_entry(
    version: str,
    story_id: str,
    story_title: str,
    changes: str,
    deployment_strategy: str,
    coverage: str
) -> str:
    """
    Generate changelog entry.

    Args:
        version: Release version
        story_id: Story identifier
        story_title: Story title
        changes: Changes description
        deployment_strategy: Deployment strategy used
        coverage: Test coverage percentage

    Returns:
        Changelog entry markdown
    """
    today = datetime.now().strftime('%Y-%m-%d')

    # Determine change type (Added, Changed, Fixed, etc.)
    change_type = "Changed"
    if "add" in story_title.lower() or "new" in story_title.lower():
        change_type = "Added"
    elif "fix" in story_title.lower() or "bug" in story_title.lower():
        change_type = "Fixed"
    elif "remove" in story_title.lower() or "delete" in story_title.lower():
        change_type = "Removed"

    entry = f"""## [{version}] - {today}

### {change_type}
- **{story_id}**: {story_title}
{changes if changes else "  - See story document for details"}

### Deployment
- Story: {story_id}
- QA Coverage: {coverage}%
- Deployment: {deployment_strategy} strategy
- Released: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}

"""
    return entry


def update_changelog(changelog_entry: str, changelog_path: str = 'CHANGELOG.md'):
    """
    Update CHANGELOG.md with new entry.

    Args:
        changelog_entry: Changelog entry content
        changelog_path: Path to CHANGELOG.md file
    """
    changelog_file = Path(changelog_path)

    if not changelog_file.exists():
        logger.warning(f"CHANGELOG.md not found at {changelog_path}")
        logger.warning("Creating new CHANGELOG.md")

        with open(changelog_file, 'w') as f:
            f.write("# Changelog\n\n")
            f.write("All notable changes to this project will be documented in this file.\n\n")
            f.write("## [Unreleased]\n\n")
            f.write(changelog_entry)

        logger.info(f"✓ Created CHANGELOG.md with entry")
    else:
        # Read existing changelog
        with open(changelog_file, 'r') as f:
            content = f.read()

        # Insert after [Unreleased] section
        if '## [Unreleased]' in content:
            updated_content = content.replace(
                '## [Unreleased]',
                f'## [Unreleased]\n\n{changelog_entry}'
            )
        else:
            # Append at beginning after header
            lines = content.split('\n')
            header_end = 0
            for i, line in enumerate(lines):
                if line.startswith('## '):
                    header_end = i
                    break

            lines.insert(header_end, changelog_entry)
            updated_content = '\n'.join(lines)

        # Write updated changelog
        with open(changelog_file, 'w') as f:
            f.write(updated_content)

        logger.info(f"✓ Updated CHANGELOG.md")


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description='Generate release notes from story documents and templates',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Basic release notes generation
  python release_notes_generator.py --story STORY-001 --version v1.2.3

  # With QA and metrics reports
  python release_notes_generator.py --story STORY-001 --version v1.2.3 \\
      --qa-report devforgeai/qa/reports/STORY-001-qa-report.md \\
      --metrics-report metrics.json

  # Custom output location
  python release_notes_generator.py --story STORY-001 --version v1.2.3 \\
      --output /path/to/release-notes.md
        """
    )

    parser.add_argument(
        '--story',
        required=True,
        help='Story ID (e.g., STORY-001)'
    )

    parser.add_argument(
        '--version',
        required=True,
        help='Release version (e.g., v1.2.3)'
    )

    parser.add_argument(
        '--qa-report',
        help='Path to QA report file'
    )

    parser.add_argument(
        '--metrics-report',
        help='Path to metrics report JSON file'
    )

    parser.add_argument(
        '--template',
        help='Path to custom release notes template'
    )

    parser.add_argument(
        '--output',
        help='Output path for release notes (default: devforgeai/releases/release-{version}.md)'
    )

    parser.add_argument(
        '--deployment-strategy',
        default='rolling',
        help='Deployment strategy used (default: rolling)'
    )

    parser.add_argument(
        '--previous-version',
        help='Previous version for rollback information'
    )

    parser.add_argument(
        '--update-changelog',
        action='store_true',
        help='Update CHANGELOG.md with release entry'
    )

    args = parser.parse_args()

    try:
        # Load story document
        logger.info(f"Loading story document: {args.story}")
        story = load_story_document(args.story)

        # Load QA report
        qa_data = {'status': 'N/A', 'coverage': 'N/A'}
        if args.qa_report:
            logger.info(f"Loading QA report: {args.qa_report}")
            qa_data = load_qa_report(args.qa_report)

        # Load metrics report
        metrics_data = {}
        if args.metrics_report:
            logger.info(f"Loading metrics report: {args.metrics_report}")
            metrics_data = load_metrics_report(args.metrics_report)

        # Load template
        logger.info("Loading release notes template")
        template = load_template(args.template)

        # Prepare data for template
        acceptance_criteria_text = '\n'.join([f'- {criterion}' for criterion in story['acceptance_criteria']])

        template_data = {
            'VERSION': args.version,
            'DATE': datetime.now().strftime('%Y-%m-%d'),
            'STORY_ID': story['id'],
            'STORY_TITLE': story['title'],
            'STATUS': 'Released',
            'SUMMARY': story['title'],
            'CHANGES': story['changes'] if story['changes'] else 'See story document for detailed changes.',
            'ACCEPTANCE_CRITERIA': acceptance_criteria_text if acceptance_criteria_text else 'See story document.',
            'QA_STATUS': qa_data['status'],
            'COVERAGE': qa_data['coverage'],
            'DEPLOYER': 'DevForgeAI Release Skill',
            'DEPLOYMENT_STRATEGY': args.deployment_strategy,
            'DEPLOYED_AT': datetime.now().strftime('%Y-%m-%d %H:%M:%S'),
            'PREVIOUS_VERSION': args.previous_version if args.previous_version else 'N/A',
            'ROLLBACK_COMMAND': f'./rollback_automation.sh --platform kubernetes --deployment myapp --version {args.previous_version}' if args.previous_version else 'N/A'
        }

        # Add metrics data if available
        if metrics_data:
            template_data['ERROR_RATE'] = metrics_data.get('metrics', {}).get('error_rate', 'N/A')
            template_data['RESPONSE_TIME'] = metrics_data.get('metrics', {}).get('response_time_p95', 'N/A')

        # Populate template
        logger.info("Generating release notes")
        release_notes = populate_template(template, template_data)

        # Determine output path
        if args.output:
            output_path = Path(args.output)
        else:
            output_dir = Path('devforgeai/releases')
            output_dir.mkdir(parents=True, exist_ok=True)
            output_path = output_dir / f'release-{args.version}.md'

        # Write release notes
        with open(output_path, 'w') as f:
            f.write(release_notes)

        logger.info(f"✓ Release notes generated: {output_path}")

        # Update CHANGELOG.md if requested
        if args.update_changelog:
            logger.info("Updating CHANGELOG.md")
            changelog_entry = generate_changelog_entry(
                args.version,
                story['id'],
                story['title'],
                story['changes'],
                args.deployment_strategy,
                qa_data['coverage']
            )
            update_changelog(changelog_entry)

        logger.info("✓ Release notes generation complete")
        sys.exit(0)

    except FileNotFoundError as e:
        logger.error(str(e))
        sys.exit(1)
    except Exception as e:
        logger.error(f"Unexpected error: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == '__main__':
    main()
