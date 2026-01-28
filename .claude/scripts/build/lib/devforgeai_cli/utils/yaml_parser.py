#!/usr/bin/env python3
"""
YAML Parser for DevForgeAI Story Files

Extracts and validates YAML frontmatter from markdown documents.
"""

import re
import yaml
from typing import Dict, List, Optional, Tuple


def parse_frontmatter(content: str) -> Tuple[Optional[Dict], str]:
    """
    Extract YAML frontmatter from markdown content.

    Args:
        content: Full markdown content

    Returns:
        Tuple of (frontmatter_dict, remaining_content)
        frontmatter_dict is None if no frontmatter found

    Example:
        >>> content = '''---
        ... id: STORY-001
        ... title: Test
        ... ---
        ... # Story content'''
        >>> fm, body = parse_frontmatter(content)
        >>> fm['id']
        'STORY-001'
    """
    # Match YAML frontmatter: ---\n{yaml}\n---
    pattern = r'^---\s*\n(.*?)\n---\s*\n(.*)$'
    match = re.match(pattern, content, re.DOTALL)

    if not match:
        return None, content

    try:
        frontmatter_text = match.group(1)
        remaining_content = match.group(2)

        frontmatter = yaml.safe_load(frontmatter_text)

        return frontmatter, remaining_content

    except yaml.YAMLError as e:
        # Invalid YAML
        raise ValueError(f"Invalid YAML frontmatter: {e}")


def validate_story_frontmatter(frontmatter: Dict) -> Tuple[bool, List[str]]:
    """
    Validate DevForgeAI story frontmatter has required fields.

    Args:
        frontmatter: Parsed YAML frontmatter dict

    Returns:
        Tuple of (is_valid, error_messages)

    Required fields for story:
    - id: STORY-XXX format
    - title: Non-empty string
    - status: Valid workflow state
    """
    errors = []

    # Required fields
    required_fields = ['id', 'title', 'status']

    for field in required_fields:
        if field not in frontmatter:
            errors.append(f"Missing required field: {field}")
        elif not frontmatter[field]:
            errors.append(f"Field {field} is empty")

    # Validate id format
    if 'id' in frontmatter:
        story_id = frontmatter['id']
        if not re.match(r'^STORY-\d+$', story_id):
            errors.append(f"Invalid id format: '{story_id}' (expected STORY-NNN)")

    # Validate status
    if 'status' in frontmatter:
        valid_statuses = [
            'Backlog', 'Ready for Dev', 'Architecture', 'In Development',
            'Dev Complete', 'QA In Progress', 'QA Approved', 'QA Failed',
            'Releasing', 'Released'
        ]
        if frontmatter['status'] not in valid_statuses:
            errors.append(f"Invalid status: '{frontmatter['status']}'")

    return len(errors) == 0, errors


def extract_story_id(content: str) -> Optional[str]:
    """
    Extract story ID from frontmatter.

    Args:
        content: Full markdown content

    Returns:
        Story ID (e.g., "STORY-001") or None
    """
    frontmatter, _ = parse_frontmatter(content)

    if frontmatter and 'id' in frontmatter:
        return frontmatter['id']

    return None


def has_valid_frontmatter(content: str) -> bool:
    """
    Check if content has valid YAML frontmatter.

    Args:
        content: Full markdown content

    Returns:
        True if valid frontmatter exists
    """
    try:
        frontmatter, _ = parse_frontmatter(content)
        return frontmatter is not None
    except ValueError:
        return False


# Module exports
__all__ = [
    'parse_frontmatter',
    'validate_story_frontmatter',
    'extract_story_id',
    'has_valid_frontmatter'
]
