"""
Failure mode identification and ranking module.

AC#2: Identify and rank most common failure modes from error entries.

STORY-227: Calculate Workflow Success Metrics
"""
from collections import Counter
from typing import Any, Dict, List


# Category mapping for error types
CATEGORY_MAP: Dict[str, str] = {
    "test_failure": "testing",
    "coverage_gap": "quality",
    "validation_failure": "validation",
    "timeout": "infrastructure",
}


def identify_failure_modes(error_entries: List[Dict[str, Any]]) -> List[str]:
    """
    Return list of unique failure modes (error_type values).

    Args:
        error_entries: List of error entries with 'error_type' field.

    Returns:
        List of unique failure mode strings.
        Skips entries without error_type field.
        Returns empty list for empty input.
    """
    if not error_entries:
        return []

    # Extract unique error_type values, skipping entries without the field
    failure_modes = set()
    for entry in error_entries:
        error_type = entry.get("error_type")
        if error_type is not None:
            failure_modes.add(error_type)

    return list(failure_modes)


def rank_failure_modes(error_entries: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
    """
    Return failure modes ranked by frequency (most common first).

    Args:
        error_entries: List of error entries with 'error_type' field.

    Returns:
        List of dicts, each containing:
        - error_type: str
        - count: int
        - percentage: float (0-100)
        Returns empty list for empty input.
    """
    if not error_entries:
        return []

    # Count occurrences of each error_type (skip entries without error_type)
    error_types = [
        entry.get("error_type")
        for entry in error_entries
        if entry.get("error_type") is not None
    ]

    if not error_types:
        return []

    # Count frequencies
    counter = Counter(error_types)
    total_errors = len(error_types)

    # Sort by count (descending) and create result
    ranked = []
    for error_type, count in counter.most_common():
        percentage = (count / total_errors) * 100.0
        ranked.append({
            "error_type": error_type,
            "count": count,
            "percentage": percentage,
        })

    return ranked


def categorize_failure_mode(error_type: str) -> str:
    """
    Categorize error types into categories.

    Args:
        error_type: The error type string to categorize.

    Returns:
        Category string:
        - "test_failure" -> "testing"
        - "coverage_gap" -> "quality"
        - "validation_failure" -> "validation"
        - "timeout" -> "infrastructure"
        - anything else -> "unknown"
    """
    return CATEGORY_MAP.get(error_type, "unknown")


def get_failure_mode_summary(error_entries: List[Dict[str, Any]]) -> Dict[str, Any]:
    """
    Return summary of failure modes.

    Args:
        error_entries: List of error entries with 'error_type' field.

    Returns:
        Dictionary containing:
        - total_errors: int
        - unique_failure_modes: int
        - ranked_modes: List[Dict] (from rank_failure_modes)
        - by_category: Dict[str, int] (count per category)
    """
    if not error_entries:
        return {
            "total_errors": 0,
            "unique_failure_modes": 0,
            "ranked_modes": [],
            "by_category": {},
        }

    # Get failure modes and ranked list
    failure_modes = identify_failure_modes(error_entries)
    ranked_modes = rank_failure_modes(error_entries)

    # Count by category
    by_category: Dict[str, int] = {}
    for entry in error_entries:
        error_type = entry.get("error_type")
        if error_type is not None:
            category = categorize_failure_mode(error_type)
            by_category[category] = by_category.get(category, 0) + 1

    # Count total errors (only entries with error_type)
    total_errors = sum(
        1 for entry in error_entries if entry.get("error_type") is not None
    )

    return {
        "total_errors": total_errors,
        "unique_failure_modes": len(failure_modes),
        "ranked_modes": ranked_modes,
        "by_category": by_category,
    }
