#!/usr/bin/env python3
r"""
Depends On Field Normalizer

Normalizes various input formats for the depends_on field to array format.
Supports STORY-090 v2.2 template format.

Validation: STORY-ID format ^STORY-\d{3,4}$
"""

import re
from typing import List, Tuple, Union


# Valid STORY-ID regex pattern
STORY_ID_PATTERN = re.compile(r'^STORY-\d{3,4}$')


def normalize_depends_on(value: Union[str, List, None]) -> List[str]:
    """
    Normalize depends_on input to array format.

    Args:
        value: Input value (string, list, None, or other)

    Returns:
        List of validated STORY-IDs (empty list if none valid)

    Examples:
        >>> normalize_depends_on(None)
        []
        >>> normalize_depends_on("")
        []
        >>> normalize_depends_on("STORY-044")
        ['STORY-044']
        >>> normalize_depends_on("STORY-044, STORY-045")
        ['STORY-044', 'STORY-045']
        >>> normalize_depends_on(["STORY-044", "STORY-045"])
        ['STORY-044', 'STORY-045']
    """
    # Handle null/None
    if value is None:
        return []

    # Handle empty string or "none" variants
    if isinstance(value, str):
        stripped = value.strip().lower()
        if stripped in ('', 'none', 'null', '[]'):
            return []
        return _parse_string_input(value)

    # Handle list input
    if isinstance(value, list):
        return _validate_list(value)

    # Unknown type - return empty
    return []


def _parse_string_input(value: str) -> List[str]:
    """Parse comma/space-separated string into validated list."""
    parts = re.split(r'[,\s]+', value.strip())
    validated = []
    for part in parts:
        cleaned = part.strip().upper()
        if cleaned and is_valid_story_id(cleaned):
            validated.append(cleaned)
    return validated


def _validate_list(values: List) -> List[str]:
    """Validate list of values, filtering invalid entries."""
    validated = []
    for value in values:
        if isinstance(value, str):
            cleaned = value.strip().upper()
            if is_valid_story_id(cleaned):
                validated.append(cleaned)
    return validated


def is_valid_story_id(value: str) -> bool:
    r"""
    Check if string is valid STORY-ID format.

    Args:
        value: String to validate

    Returns:
        True if matches ^STORY-\d{3,4}$
    """
    if not value or not isinstance(value, str):
        return False
    return bool(STORY_ID_PATTERN.match(value))


def validate_depends_on_input(value: Union[str, List, None]) -> Tuple[List[str], List[str]]:
    """
    Validate input and return both valid and invalid entries.

    Args:
        value: Input value to validate

    Returns:
        Tuple of (valid_ids, invalid_entries)
    """
    if value is None:
        return [], []

    if isinstance(value, str):
        stripped = value.strip().lower()
        if stripped in ('', 'none', 'null', '[]'):
            return [], []

        parts = re.split(r'[,\s]+', value.strip())
        valid, invalid = [], []

        for part in parts:
            cleaned = part.strip()
            if cleaned:
                upper = cleaned.upper()
                if is_valid_story_id(upper):
                    valid.append(upper)
                else:
                    invalid.append(cleaned)
        return valid, invalid

    if isinstance(value, list):
        valid, invalid = [], []
        for item in value:
            if isinstance(item, str):
                cleaned = item.strip()
                if cleaned:
                    upper = cleaned.upper()
                    if is_valid_story_id(upper):
                        valid.append(upper)
                    else:
                        invalid.append(cleaned)
        return valid, invalid

    return [], []


__all__ = [
    'normalize_depends_on',
    'is_valid_story_id',
    'validate_depends_on_input',
    'STORY_ID_PATTERN'
]
