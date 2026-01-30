"""
Skip tracking system for feedback collection.

This module provides atomic operations for tracking consecutive skips,
with thread-safe operations and resetting on positive feedback.
"""

import os
import threading
from pathlib import Path
from typing import Optional, Dict
from datetime import datetime


class SkipTracker:
    """Tracks consecutive skips for feedback collection.

    Provides thread-safe atomic operations for incrementing skip counters,
    checking limits, and resetting based on positive feedback.
    """

    # Default log path for skip tracking
    DEFAULT_SKIP_LOG_PATH = Path("devforgeai/logs/feedback-skips.log")
    # Default rating threshold for "positive" feedback
    DEFAULT_RATING_THRESHOLD = 4

    def __init__(self, skip_log_path: Optional[Path] = None):
        """Initialize the skip tracker.

        Args:
            skip_log_path: Path to skip tracking log file.
                          Defaults to devforgeai/logs/feedback-skips.log
        """
        if skip_log_path is None:
            skip_log_path = self.DEFAULT_SKIP_LOG_PATH

        self.skip_log_path = skip_log_path
        self._skip_counters: Dict[str, int] = {}
        self._lock = threading.Lock()
        self._load_existing_counters()

    def _load_existing_counters(self) -> None:
        """Load skip counters from log file if it exists."""
        if self.skip_log_path.exists():
            try:
                with open(self.skip_log_path, 'r') as f:
                    for line in f:
                        line = line.strip()
                        if not line or line.startswith("#"):
                            continue
                        parts = line.split(":")
                        if len(parts) >= 3:
                            # Format: timestamp:operation:count
                            operation = parts[1].strip()
                            try:
                                count = int(parts[2].split(",")[0].strip())
                                self._skip_counters[operation] = count
                            except (ValueError, IndexError):
                                pass
            except (IOError, OSError):
                # File read error - continue with empty counters
                pass

    def _ensure_log_directory(self) -> None:
        """Ensure the logs directory exists."""
        self.skip_log_path.parent.mkdir(parents=True, exist_ok=True)

    def _log_skip_operation(self, operation: str, count: int, action: str) -> None:
        """Log a skip operation to the tracking file.

        Args:
            operation: Name of the operation.
            count: Current skip count.
            action: Action performed (skip, reset, block).
        """
        self._ensure_log_directory()
        timestamp = datetime.now().isoformat()
        try:
            with open(self.skip_log_path, 'a') as f:
                f.write(f"{timestamp}: {operation}: {count}, action={action}\n")
        except (IOError, OSError):
            # Silently fail if log write fails
            pass

    def increment_skip(self, operation: str) -> int:
        """Increment skip counter for an operation (thread-safe).

        Args:
            operation: Name of the operation that was skipped.

        Returns:
            Updated skip count for the operation.
        """
        with self._lock:
            current = self._skip_counters.get(operation, 0)
            current += 1
            self._skip_counters[operation] = current
            self._log_skip_operation(operation, current, "skip")
            return current

    def get_skip_count(self, operation: str) -> int:
        """Get current skip count for an operation (thread-safe).

        Args:
            operation: Name of the operation.

        Returns:
            Current skip count (0 if never skipped).
        """
        with self._lock:
            return self._skip_counters.get(operation, 0)

    def reset_skip_counter(self, operation: str) -> None:
        """Reset skip counter for an operation (thread-safe).

        Args:
            operation: Name of the operation.
        """
        with self._lock:
            if operation in self._skip_counters:
                self._skip_counters[operation] = 0
                self._log_skip_operation(operation, 0, "reset")

    def check_skip_limit(self, operation: str, max_consecutive_skips: int) -> bool:
        """Check if skip limit has been reached (thread-safe).

        Args:
            operation: Name of the operation.
            max_consecutive_skips: Maximum allowed consecutive skips.
                                   0 = unlimited.

        Returns:
            True if limit reached (should block), False otherwise.
                Returns False if max_consecutive_skips is 0 (unlimited).
        """
        if max_consecutive_skips == 0:
            # Unlimited skips
            return False

        with self._lock:
            count = self._skip_counters.get(operation, 0)
            if count >= max_consecutive_skips:
                self._log_skip_operation(operation, count, "block")
                return True
            return False

    def reset_on_positive(self, operation: str, rating: int, rating_threshold: Optional[int] = None) -> None:
        """Reset skip counter if positive feedback received (thread-safe).

        Args:
            operation: Name of the operation.
            rating: User's feedback rating/score.
            rating_threshold: Rating value above which is considered positive.
                            Defaults to DEFAULT_RATING_THRESHOLD.
        """
        if rating_threshold is None:
            rating_threshold = self.DEFAULT_RATING_THRESHOLD

        if rating >= rating_threshold:
            self.reset_skip_counter(operation)

    def get_all_counters(self) -> Dict[str, int]:
        """Get copy of all skip counters (thread-safe).

        Returns:
            Dictionary of operation -> skip count.
        """
        with self._lock:
            return self._skip_counters.copy()

    def clear_all_counters(self) -> None:
        """Clear all skip counters (thread-safe).

        Used for testing and reset scenarios.
        """
        with self._lock:
            self._skip_counters.clear()
