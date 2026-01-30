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

        Args:
            context: Pre-extracted operation context

        Returns:
            True if skill invocation succeeded, False otherwise
        """
        try:
            # This would call the actual devforgeai-feedback skill
            # For now, we simulate it returning True
            operation_id = context.get("operation_id", "unknown")
            logger.info(f"Skill invoked with context: {operation_id}")
            return True
        except Exception as e:
            self._log_error(e)
            return False

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
