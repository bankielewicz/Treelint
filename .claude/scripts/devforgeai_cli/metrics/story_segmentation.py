"""
Story point segmentation and averages module.

AC#3: Segment metrics by story points (1, 2, 3, 5, 8 - Fibonacci-based).

STORY-227: Calculate Workflow Success Metrics
"""
from typing import Any, Dict, List, Optional


# Valid Fibonacci-based story points
VALID_STORY_POINTS: List[int] = [1, 2, 3, 5, 8]


def get_valid_story_points() -> List[int]:
    """
    Return the list of valid story point values.

    Returns:
        [1, 2, 3, 5, 8] (Fibonacci-based story points)
    """
    return VALID_STORY_POINTS.copy()


def is_valid_story_point(point: Any) -> bool:
    """
    Check if a value is a valid story point.

    Args:
        point: The value to check.

    Returns:
        True if point is in [1, 2, 3, 5, 8], False otherwise.
        Returns False for None.
    """
    if point is None:
        return False
    return point in VALID_STORY_POINTS


def segment_metrics_by_story_points(
    workflow_metrics: List[Dict[str, Any]]
) -> Dict[int, List[Dict[str, Any]]]:
    """
    Segment metrics by story points.

    Args:
        workflow_metrics: List of workflow metric entries with 'story_points' field.

    Returns:
        Dictionary with valid story points as keys:
        {
            1: [stories with 1 point...],
            2: [stories with 2 points...],
            3: [],  # empty if no stories
            5: [],
            8: []
        }
        - Only includes valid points (1, 2, 3, 5, 8)
        - Excludes stories with missing/invalid story_points
        - Returns structure with empty lists for empty input
    """
    # Initialize result with empty lists for each valid point
    result: Dict[int, List[Dict[str, Any]]] = {
        point: [] for point in VALID_STORY_POINTS
    }

    if not workflow_metrics:
        return result

    for metric in workflow_metrics:
        story_points = metric.get("story_points")

        # Skip if story_points is missing, None, or invalid
        if not is_valid_story_point(story_points):
            continue

        # Add to appropriate segment
        result[story_points].append(metric)

    return result


def calculate_segment_averages(
    segments: Dict[int, List[Dict[str, Any]]]
) -> Dict[int, Dict[str, Optional[float]]]:
    """
    Calculate averages per segment.

    Args:
        segments: Dictionary of story point segments from segment_metrics_by_story_points.

    Returns:
        Dictionary with averages per segment:
        {
            1: {"avg_completion_rate": float, "avg_error_rate": float},
            2: {...},
            ...
        }
        - Returns 0.0 for empty segments
    """
    result: Dict[int, Dict[str, Optional[float]]] = {}

    for point in VALID_STORY_POINTS:
        stories = segments.get(point, [])

        if not stories:
            result[point] = {
                "avg_completion_rate": 0.0,
                "avg_error_rate": 0.0,
            }
        else:
            # Calculate averages
            total_completion = sum(
                story.get("completion_rate", 0.0) for story in stories
            )
            total_error = sum(
                story.get("error_rate", 0.0) for story in stories
            )

            avg_completion = total_completion / len(stories)
            avg_error = total_error / len(stories)

            # Round to 2 decimal places
            result[point] = {
                "avg_completion_rate": round(avg_completion, 2),
                "avg_error_rate": round(avg_error, 2),
            }

    return result


def get_segmentation_summary(
    workflow_metrics: List[Dict[str, Any]]
) -> Dict[str, Any]:
    """
    Return summary of story point segmentation.

    Args:
        workflow_metrics: List of workflow metric entries.

    Returns:
        Dictionary containing:
        - total_stories: int (all stories including invalid)
        - segmented_stories: int (stories with valid points)
        - excluded_stories: int (stories without valid points)
        - segments: Dict[int, List[Dict]] (from segment_metrics_by_story_points)
        - averages_by_segment: Dict[int, Dict] (from calculate_segment_averages)
    """
    if not workflow_metrics:
        return {
            "total_stories": 0,
            "segmented_stories": 0,
            "excluded_stories": 0,
            "segments": {point: [] for point in VALID_STORY_POINTS},
            "averages_by_segment": {
                point: {"avg_completion_rate": 0.0, "avg_error_rate": 0.0}
                for point in VALID_STORY_POINTS
            },
        }

    # Segment the metrics
    segments = segment_metrics_by_story_points(workflow_metrics)

    # Count segmented stories
    segmented_count = sum(len(stories) for stories in segments.values())

    # Calculate total and excluded
    total_stories = len(workflow_metrics)
    excluded_stories = total_stories - segmented_count

    # Calculate averages
    averages = calculate_segment_averages(segments)

    return {
        "total_stories": total_stories,
        "segmented_stories": segmented_count,
        "excluded_stories": excluded_stories,
        "segments": segments,
        "averages_by_segment": averages,
    }
