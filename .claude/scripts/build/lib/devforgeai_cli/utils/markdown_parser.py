#!/usr/bin/env python3
"""
Markdown Parser for DevForgeAI Story Files

Extracts sections, checklists, and content from markdown documents.
Pattern based on industry research (SpecDriven AI, GitHub DoD Checker).
"""

import re
from typing import Any, Dict, List, Optional, Tuple


def extract_section(content: str, section_name: str) -> Optional[str]:
    """
    Extract a section from markdown content.

    Args:
        content: Full markdown content
        section_name: Section header (without ## prefix)

    Returns:
        Section content (excluding header) or None if not found

    Example:
        >>> content = "## Introduction\\nText\\n## Details\\nMore text"
        >>> extract_section(content, "Details")
        'More text'
    """
    # Match section: ## {name} until next ## or end of file
    pattern = rf'^## {re.escape(section_name)}\s*\n(.*?)(?:\n##|\Z)'
    match = re.search(pattern, content, re.MULTILINE | re.DOTALL)

    if match:
        return match.group(1).strip()
    return None


def parse_checklist(section_content: str) -> List[Dict[str, Any]]:
    """
    Parse markdown checklist items from section content.

    Args:
        section_content: Content of a section containing checklist items

    Returns:
        List of dicts with: text, checked, line_number

    Example:
        >>> content = "- [x] Complete item\\n- [ ] Incomplete item"
        >>> parse_checklist(content)
        [
            {'text': 'Complete item', 'checked': True, 'line_number': 1},
            {'text': 'Incomplete item', 'checked': False, 'line_number': 2}
        ]
    """
    items = []

    # Match: - [x] or - [ ] followed by text
    pattern = r'^-\s*\[([ x])\]\s*(.+)$'

    for line_num, line in enumerate(section_content.split('\n'), start=1):
        match = re.match(pattern, line.strip())
        if match:
            checkbox_char = match.group(1)
            item_text = match.group(2).strip()

            items.append({
                'text': item_text,
                'checked': checkbox_char == 'x',
                'line_number': line_num,
                'raw_line': line
            })

    return items


def extract_all_sections(content: str) -> Dict[str, str]:
    """
    Extract all ## sections from markdown.

    Args:
        content: Full markdown content

    Returns:
        Dict mapping section names to section content
    """
    sections = {}

    # Find all ## headers
    pattern = r'^## (.+?)\s*$'
    matches = list(re.finditer(pattern, content, re.MULTILINE))

    for i, match in enumerate(matches):
        section_name = match.group(1).strip()
        start = match.end()

        # End is next section or end of file
        if i + 1 < len(matches):
            end = matches[i + 1].start()
        else:
            end = len(content)

        section_content = content[start:end].strip()
        sections[section_name] = section_content

    return sections


def find_checklist_item_context(content: str, item_text: str, context_lines: int = 3) -> Optional[str]:
    """
    Find checklist item and return surrounding context.

    Args:
        content: Full markdown content
        item_text: Text of the checklist item to find
        context_lines: Number of lines before/after to include

    Returns:
        Context around the item or None if not found
    """
    lines = content.split('\n')

    for i, line in enumerate(lines):
        if item_text in line and re.match(r'^-\s*\[([ x])\]', line.strip()):
            # Found the item, extract context
            start = max(0, i - context_lines)
            end = min(len(lines), i + context_lines + 1)

            context = '\n'.join(lines[start:end])
            return context

    return None


def extract_item_justification(impl_notes_content: str, item_text: str) -> Optional[str]:
    """
    Extract justification text following a checklist item.

    Args:
        impl_notes_content: Implementation Notes section content
        item_text: The checklist item to find justification for

    Returns:
        Justification text (lines following item until next item/section) or None

    Example:
        >>> content = '''
        ... - [ ] Item 1 - Deferred to STORY-042
        ...   **User Approved:** YES
        ...   **Rationale:** Complexity
        ... - [x] Item 2 - Completed
        ... '''
        >>> extract_item_justification(content, "Item 1")
        'Deferred to STORY-042\\n**User Approved:** YES\\n**Rationale:** Complexity'
    """
    lines = impl_notes_content.split('\n')

    for i, line in enumerate(lines):
        # Find the item
        if item_text in line and re.match(r'^-\s*\[([ x])\]', line.strip()):
            # Extract text from this line until next checklist item
            justification_lines = []

            # Get remainder of current line after checkbox
            match = re.match(r'^-\s*\[([ x])\]\s*(.+)$', line.strip())
            if match:
                remainder = match.group(2).strip()
                # Remove item text itself, keep justification part
                if ' - ' in remainder:
                    parts = remainder.split(' - ', 1)
                    if len(parts) > 1:
                        justification_lines.append(parts[1])

            # Get following lines until next item or section
            for j in range(i + 1, len(lines)):
                next_line = lines[j].strip()

                # Stop at next checklist item or section header
                if re.match(r'^-\s*\[([ x])\]', next_line) or re.match(r'^##', next_line):
                    break

                # Skip empty lines at start
                if not justification_lines and not next_line:
                    continue

                justification_lines.append(lines[j])

            if justification_lines:
                return '\n'.join(justification_lines).strip()

    return None


def count_lines_in_section(content: str, section_name: str) -> int:
    """
    Count number of lines in a section.

    Args:
        content: Full markdown content
        section_name: Name of section

    Returns:
        Number of lines in section
    """
    section = extract_section(content, section_name)
    if section:
        return len(section.split('\n'))
    return 0


# Module exports
__all__ = [
    'extract_section',
    'parse_checklist',
    'extract_all_sections',
    'find_checklist_item_context',
    'extract_item_justification',
    'count_lines_in_section'
]
