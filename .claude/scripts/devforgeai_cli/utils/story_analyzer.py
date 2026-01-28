#!/usr/bin/env python3
"""
Story Analyzer for DevForgeAI Story Files

High-level analysis functions for story validation.
Combines markdown and YAML parsing for story-specific operations.
"""

import re
from pathlib import Path
from typing import Dict, List, Optional, Tuple

from .markdown_parser import (
    extract_section,
    parse_checklist,
    extract_item_justification
)
from .yaml_parser import parse_frontmatter, extract_story_id


def load_story_file(story_path: str) -> Tuple[Optional[Dict], str]:
    """
    Load story file and parse frontmatter + content.

    Args:
        story_path: Path to story file

    Returns:
        Tuple of (frontmatter, content)

    Raises:
        FileNotFoundError if story doesn't exist
    """
    path = Path(story_path)
    if not path.exists():
        raise FileNotFoundError(f"Story file not found: {story_path}")

    content = path.read_text(encoding='utf-8')
    frontmatter, body = parse_frontmatter(content)

    return frontmatter, content


def extract_dod_items(story_content: str) -> List[Dict]:
    """
    Extract all Definition of Done checklist items.

    Args:
        story_content: Full story file content

    Returns:
        List of DoD items with text, checked status, line number
    """
    dod_section = extract_section(story_content, "Definition of Done")

    if not dod_section:
        return []

    return parse_checklist(dod_section)


def extract_impl_notes_items(story_content: str) -> List[Dict]:
    """
    Extract all Implementation Notes checklist items.

    Args:
        story_content: Full story file content

    Returns:
        List of Implementation Notes items with text, checked status
    """
    impl_section = extract_section(story_content, "Implementation Notes")

    if not impl_section:
        return []

    # Implementation Notes can have subsections, extract from full section
    return parse_checklist(impl_section)


def find_dod_impl_mismatch(story_content: str) -> List[Dict]:
    """
    Find mismatches between DoD and Implementation Notes status.

    Args:
        story_content: Full story file content

    Returns:
        List of mismatches with item text, dod_checked, impl_checked

    Example mismatch (autonomous deferral):
        DoD: [x] Item completed
        Impl: [ ] Item - Deferred to STORY-XXX
    """
    dod_items = extract_dod_items(story_content)
    impl_items = extract_impl_notes_items(story_content)

    # Create lookup for impl items by text (extract base text before " - ")
    impl_lookup = {}
    for item in impl_items:
        # Extract base text (everything before " - " if present)
        base_text = item['text'].split(' - ')[0].strip()
        impl_lookup[base_text] = item

    mismatches = []

    for dod_item in dod_items:
        dod_text = dod_item['text'].strip()

        # Try exact match first
        impl_item = impl_lookup.get(dod_text)

        # If no exact match, try partial match
        if not impl_item:
            for impl_key, impl_val in impl_lookup.items():
                # Match if either contains the other (handles variations)
                if (impl_key in dod_text or dod_text in impl_key) and len(impl_key) > 5:
                    impl_item = impl_val
                    break

        if not impl_item:
            # DoD item not found in Implementation Notes
            mismatches.append({
                'item': dod_text,
                'dod_checked': dod_item['checked'],
                'impl_found': False,
                'impl_checked': None
            })
        elif dod_item['checked'] != impl_item['checked']:
            # Status mismatch
            mismatches.append({
                'item': dod_text,
                'dod_checked': dod_item['checked'],
                'impl_found': True,
                'impl_checked': impl_item['checked']
            })

    return mismatches


def check_user_approval_marker(justification_text: str) -> Tuple[bool, Optional[str]]:
    """
    Check if justification contains user approval marker.

    Args:
        justification_text: Text following deferred item

    Returns:
        Tuple of (has_approval, marker_type)
        marker_type: "user_approved", "story_reference", "adr_reference", "external_blocker", None

    Valid approval markers:
    - "User approved:" or "User Approved:"
    - STORY-XXX reference
    - ADR-XXX reference
    - "Blocked by: ... (external)"
    """
    if not justification_text:
        return False, None

    # Check for explicit user approval
    if re.search(r'User [Aa]pproved:', justification_text):
        return True, "user_approved"

    # Check for AskUserQuestion mention
    if 'AskUserQuestion' in justification_text:
        return True, "user_approved"

    # Check for story reference
    if re.search(r'STORY-\d+', justification_text):
        return True, "story_reference"

    # Check for ADR reference
    if re.search(r'ADR-\d+', justification_text):
        return True, "adr_reference"

    # Check for external blocker
    if 'Blocked by:' in justification_text and '(external)' in justification_text:
        return True, "external_blocker"

    return False, None


def extract_story_references(text: str) -> List[str]:
    """
    Extract all STORY-XXX references from text.

    Args:
        text: Text to search

    Returns:
        List of story IDs (e.g., ["STORY-042", "STORY-100"])
    """
    return re.findall(r'STORY-\d+', text)


def extract_adr_references(text: str) -> List[str]:
    """
    Extract all ADR-XXX references from text.

    Args:
        text: Text to search

    Returns:
        List of ADR IDs (e.g., ["ADR-001", "ADR-023"])
    """
    return re.findall(r'ADR-\d+', text)


def has_implementation_notes(story_content: str) -> bool:
    """
    Check if story has Implementation Notes section.

    Args:
        story_content: Full story file content

    Returns:
        True if Implementation Notes section exists
    """
    impl_section = extract_section(story_content, "Implementation Notes")
    return impl_section is not None and len(impl_section.strip()) > 0


def count_deferred_items(impl_notes_content: str) -> int:
    """
    Count number of deferred items in Implementation Notes.

    Args:
        impl_notes_content: Implementation Notes section content

    Returns:
        Number of items marked [ ] (incomplete/deferred)
    """
    items = parse_checklist(impl_notes_content)
    return sum(1 for item in items if not item['checked'])


# Module exports
__all__ = [
    'load_story_file',
    'extract_dod_items',
    'extract_impl_notes_items',
    'find_dod_impl_mismatch',
    'check_user_approval_marker',
    'extract_story_references',
    'extract_adr_references',
    'has_implementation_notes',
    'count_deferred_items'
]
