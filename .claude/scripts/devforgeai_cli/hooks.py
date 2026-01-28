"""
DevForgeAI Feedback Hook Service

Handles invocation of devforgeai-feedback skill with operation context.
Implements graceful degradation, timeout protection, and circular invocation guards.

Features:
- Context extraction and sanitization
- 30-second timeout protection
- Circular invocation detection via DEVFORGEAI_HOOK_ACTIVE env var
- Graceful error handling (no exceptions to caller)
- Comprehensive logging
"""

import logging
import os
import threading
import traceback
from typing import Optional, Dict, Any

from .context_extraction import extract_context, sanitize_context

# Configure logging
logger = logging.getLogger(__name__)

# Constants for timeout protection
TIMEOUT_SECONDS = 30
HOOK_ACTIVE_ENV_VAR = "DEVFORGEAI_HOOK_ACTIVE"


class HookInvocationService:
    """Service for invoking devforgeai-feedback skill with operation context."""

    def __init__(self, timeout: int = TIMEOUT_SECONDS):
        """
        Initialize HookInvocationService.

        Args:
            timeout: Timeout in seconds (default: 30)
        """
        self.timeout = timeout
        self._timeout_occurred = False
        self._timer = None

    def invoke(self, operation: str, story_id: Optional[str] = None) -> bool:
        """
        Invoke feedback hook for an operation.

        Args:
            operation: Operation name (e.g., 'dev', 'qa', 'release')
            story_id: Optional story ID (format: STORY-NNN)

        Returns:
            True if hook invocation succeeded, False otherwise
        """
        try:
            self._log_invocation_start(operation, story_id)

            # Check for circular invocation
            if self.check_circular_invocation():
                logger.error("Circular invocation detected, aborting")
                return False

            # Set hook active flag for nested calls
            self.set_hook_active()

            try:
                return self._process_hook_invocation(operation, story_id)

            finally:
                # Clear hook active flag
                self._clear_hook_active()

        except Exception as e:
            self._log_error(e)
            return False

    def _process_hook_invocation(
        self, operation: str, story_id: Optional[str]
    ) -> bool:
        """Process the actual hook invocation workflow."""
        # Extract context from operation
        context = extract_context(operation, story_id)
        self._log_context_extraction(context)

        # Sanitize context (remove secrets)
        context = sanitize_context(context)

        # Invoke feedback skill with timeout
        return self._invoke_skill_with_timeout(operation, context)

    @staticmethod
    def _log_invocation_start(operation: str, story_id: Optional[str]) -> None:
        """Log the start of hook invocation."""
        logger.info(f"Invoking feedback hook: operation={operation}, story={story_id}")

    @staticmethod
    def _log_context_extraction(context: Dict[str, Any]) -> None:
        """Log context extraction completion with metrics."""
        logger.info(
            f"Context extracted: {context.get('context_size_bytes', 0)}B, "
            f"{len(context.get('todos', []))} todos, "
            f"{len(context.get('errors', []))} errors"
        )

    @staticmethod
    def _log_error(error: Exception) -> None:
        """Log error with stack trace."""
        logger.error(f"Hook invocation failed: {str(error)}")
        logger.debug(traceback.format_exc())

    def check_circular_invocation(self) -> bool:
        """
        Check if we're in a circular invocation (already inside a hook).

        Returns:
            True if circular invocation detected, False otherwise
        """
        hook_active = os.environ.get(HOOK_ACTIVE_ENV_VAR)
        return hook_active in ("1", "true", "True")

    def set_hook_active(self) -> None:
        """Set DEVFORGEAI_HOOK_ACTIVE environment variable to prevent re-entry."""
        os.environ[HOOK_ACTIVE_ENV_VAR] = "1"

    def _clear_hook_active(self) -> None:
        """Clear DEVFORGEAI_HOOK_ACTIVE environment variable."""
        if HOOK_ACTIVE_ENV_VAR in os.environ:
            del os.environ[HOOK_ACTIVE_ENV_VAR]

    def invoke_feedback_skill(self, context: Dict[str, Any]) -> bool:
        """
        Invoke devforgeai-feedback skill with pre-populated context.

        Prints structured output to stdout containing operation context and
        instructions for Claude to invoke the devforgeai-feedback skill.

        Output Format (v1.0):
        ==============================================================
          FEEDBACK HOOK TRIGGERED
        ==============================================================
          Operation: {operation}
          Operation ID: {operation_id}
          Story ID: {story_id or "N/A"}
          Status: {status}
          Duration: {duration_ms}ms
          Todos: {count} items ({completed} completed, ...)
          Errors: {error_count}

          Action Required: Invoke devforgeai-feedback skill
          Context: operation={operation}, story={story_id}, status={status}
        ==============================================================

        Args:
            context: Pre-extracted operation context dictionary containing:
                - operation_id: Unique operation identifier
                - operation: Operation type (dev, qa, release)
                - story_id: Story reference (STORY-NNN) or None
                - status: Operation status (completed, failed, error)
                - duration_ms: Operation duration in milliseconds
                - todos: List of todo items with status
                - errors: List of error dictionaries

        Returns:
            True if output generation succeeded, False on any exception
        """
        try:
            # Validate context is a dictionary
            if context is None or not isinstance(context, dict):
                error_msg = ("Invalid context: expected dict, got " +
                           f"{type(context).__name__ if context is not None else 'None'}")
                logger.error(error_msg)
                # Log error details at debug level for debugging
                logger.debug(f"Context validation error - Traceback: {error_msg}")
                return False

            # Extract context fields with safe defaults
            operation_id = self._escape_value(context.get("operation_id", "unknown"))
            operation = self._escape_value(context.get("operation", "unknown"))
            story_id = context.get("story_id")
            story_id_display = self._escape_value(story_id) if story_id else "N/A"
            story_id_context = self._escape_value(story_id) if story_id else "unassigned"
            status = self._escape_value(context.get("status", "unknown"))
            duration_ms = context.get("duration_ms", 0)

            # Calculate todos summary
            todos_list = context.get("todos", [])
            if not isinstance(todos_list, list):
                todos_list = []
            todos_count = len(todos_list)
            completed = sum(1 for t in todos_list
                          if isinstance(t, dict) and t.get("status") == "completed")
            in_progress = sum(1 for t in todos_list
                            if isinstance(t, dict) and t.get("status") == "in_progress")
            pending = sum(1 for t in todos_list
                        if isinstance(t, dict) and t.get("status") == "pending")

            # Calculate errors count
            errors_list = context.get("errors", [])
            if not isinstance(errors_list, list):
                errors_list = []
            error_count = len(errors_list)

            # Build and print structured output
            delimiter = "=" * 62
            output_lines = [
                delimiter,
                "  FEEDBACK HOOK TRIGGERED",
                delimiter,
                f"  Operation: {operation}",
                f"  Operation ID: {operation_id}",
                f"  Story ID: {story_id_display}",
                f"  Status: {status}",
                f"  Duration: {duration_ms}ms",
                f"  Todos: {todos_count} items ({completed} completed, "
                f"{in_progress} in progress, {pending} pending)",
                f"  Errors: {error_count}",
                "",
                "  Action Required: Invoke devforgeai-feedback skill",
                f"  Context: operation={operation}, story={story_id_context}, status={status}",
                delimiter,
            ]

            # Print to stdout
            print("\n".join(output_lines))

            # Log success
            logger.info(f"Feedback hook output generated for operation: {operation_id}")

            return True

        except Exception as e:
            logger.error(f"Failed to generate feedback hook output: {str(e)}")
            logger.debug(traceback.format_exc())
            return False

    @staticmethod
    def _escape_value(value: Any) -> str:
        """
        Escape special characters in context values for safe output.

        Handles newlines, tabs, quotes, and other special characters
        to ensure parsable output format.

        Args:
            value: The value to escape (converted to string if needed)

        Returns:
            Escaped string safe for structured output
        """
        if value is None:
            return ""
        str_value = str(value)
        # Escape special characters
        str_value = str_value.replace("\\", "\\\\")  # Backslash first
        str_value = str_value.replace("\n", "\\n")   # Newlines
        str_value = str_value.replace("\r", "\\r")   # Carriage returns
        str_value = str_value.replace("\t", "\\t")   # Tabs
        str_value = str_value.replace('"', '\\"')    # Double quotes
        return str_value

    def _invoke_skill_with_timeout(self, operation: str, context: Dict[str, Any]) -> bool:
        """
        Invoke skill with timeout protection.

        Args:
            operation: Operation name
            context: Pre-extracted operation context

        Returns:
            True if skill invocation succeeded, False on timeout or error
        """
        self._timeout_occurred = False

        # Create and start skill invocation thread
        thread = self._create_skill_thread(context)
        thread.start()

        # Wait for completion with timeout
        thread.join(timeout=self.timeout)

        # Check if thread is still alive (timeout occurred)
        if thread.is_alive():
            self._timeout_occurred = True
            logger.error(f"Feedback hook timeout after {self.timeout}s")
            return False

        return True

    def _create_skill_thread(self, context: Dict[str, Any]) -> threading.Thread:
        """Create a daemon thread for skill invocation."""
        def skill_thread():
            try:
                self.invoke_feedback_skill(context)
            except Exception:
                pass

        return threading.Thread(target=skill_thread, daemon=True)


def invoke_hooks(operation: str, story_id: Optional[str] = None) -> bool:
    """
    Public API for invoking feedback hooks.

    Args:
        operation: Operation name (e.g., 'dev', 'qa', 'release')
        story_id: Optional story ID (format: STORY-NNN)

    Returns:
        True if hook invocation succeeded, False otherwise
    """
    service = HookInvocationService(timeout=TIMEOUT_SECONDS)
    return service.invoke(operation, story_id)
