"""
DevForgeAI Workflow Success Metrics package.

Provides functions for calculating per-command metrics,
failure mode analysis, and story point segmentation.

STORY-227: Calculate Workflow Success Metrics
"""
from devforgeai_cli.metrics.command_metrics import (
    calculate_completion_rate,
    calculate_error_rate,
    calculate_retry_rate,
    calculate_per_command_metrics,
)
from devforgeai_cli.metrics.failure_modes import (
    identify_failure_modes,
    rank_failure_modes,
    categorize_failure_mode,
    get_failure_mode_summary,
)
from devforgeai_cli.metrics.story_segmentation import (
    get_valid_story_points,
    is_valid_story_point,
    segment_metrics_by_story_points,
    calculate_segment_averages,
    get_segmentation_summary,
)

__all__ = [
    # AC#1: Per-command metrics
    "calculate_completion_rate",
    "calculate_error_rate",
    "calculate_retry_rate",
    "calculate_per_command_metrics",
    # AC#2: Failure mode identification
    "identify_failure_modes",
    "rank_failure_modes",
    "categorize_failure_mode",
    "get_failure_mode_summary",
    # AC#3: Story segmentation
    "get_valid_story_points",
    "is_valid_story_point",
    "segment_metrics_by_story_points",
    "calculate_segment_averages",
    "get_segmentation_summary",
]
