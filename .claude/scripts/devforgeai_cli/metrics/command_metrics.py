"""
Per-command metrics calculation module.

AC#1: Calculate completion rate, error rate, and retry rate per command type.

STORY-227: Calculate Workflow Success Metrics
"""
from typing import Any, Dict, List


def calculate_completion_rate(data: List[Dict[str, Any]], command_type: str) -> float:
    """
    Calculate completion rate for a specific command type.

    Args:
        data: List of command execution entries with 'command' and 'status' fields.
        command_type: The command type to filter (e.g., "/dev", "/qa").

    Returns:
        Percentage of completed executions (0-100).
        Returns 0.0 for empty data or nonexistent command.
    """
    if not data:
        return 0.0

    # Filter for the specific command type
    command_entries = [entry for entry in data if entry.get("command") == command_type]

    if not command_entries:
        return 0.0

    # Count completed entries
    completed_count = sum(
        1 for entry in command_entries if entry.get("status") == "completed"
    )

    # Calculate percentage
    return (completed_count / len(command_entries)) * 100.0


def calculate_error_rate(data: List[Dict[str, Any]], command_type: str) -> float:
    """
    Calculate error rate for a specific command type.

    Args:
        data: List of command execution entries with 'command' and 'status' fields.
        command_type: The command type to filter (e.g., "/dev", "/qa").

    Returns:
        Percentage of failed executions (0-100).
        Returns 0.0 for empty data or nonexistent command.
    """
    if not data:
        return 0.0

    # Filter for the specific command type
    command_entries = [entry for entry in data if entry.get("command") == command_type]

    if not command_entries:
        return 0.0

    # Count error entries
    error_count = sum(
        1 for entry in command_entries if entry.get("status") == "error"
    )

    # Calculate percentage
    return round((error_count / len(command_entries)) * 100.0, 2)


def calculate_retry_rate(data: List[Dict[str, Any]], command_type: str) -> float:
    """
    Calculate retry rate for a specific command type.

    Args:
        data: List of command execution entries with 'command' and 'retry_count' fields.
        command_type: The command type to filter (e.g., "/dev", "/qa").

    Returns:
        Percentage of executions that had retries (retry_count > 0).
        Returns 0.0 for empty data or nonexistent command.
    """
    if not data:
        return 0.0

    # Filter for the specific command type
    command_entries = [entry for entry in data if entry.get("command") == command_type]

    if not command_entries:
        return 0.0

    # Count entries with retries
    retry_count = sum(
        1 for entry in command_entries if entry.get("retry_count", 0) > 0
    )

    # Calculate percentage
    return round((retry_count / len(command_entries)) * 100.0, 2)


def calculate_per_command_metrics(data: List[Dict[str, Any]]) -> Dict[str, Dict[str, Any]]:
    """
    Calculate metrics for all command types in data.

    Args:
        data: List of command execution entries.

    Returns:
        Dictionary with metrics per command type:
        {
            "/dev": {
                "completion_rate": float,
                "error_rate": float,
                "retry_rate": float,
                "total_executions": int
            },
            ...
        }
        Returns empty dict for empty data.
    """
    if not data:
        return {}

    # Extract unique command types
    command_types = set(entry.get("command") for entry in data if entry.get("command"))

    result: Dict[str, Dict[str, Any]] = {}

    for command_type in command_types:
        # Count total executions for this command
        command_entries = [
            entry for entry in data if entry.get("command") == command_type
        ]

        result[command_type] = {
            "completion_rate": calculate_completion_rate(data, command_type),
            "error_rate": calculate_error_rate(data, command_type),
            "retry_rate": calculate_retry_rate(data, command_type),
            "total_executions": len(command_entries),
        }

    return result
