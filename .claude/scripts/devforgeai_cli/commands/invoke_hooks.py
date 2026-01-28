"""
DevForgeAI invoke-hooks CLI Command

Handles the 'devforgeai invoke-hooks' command for triggering feedback hooks.

Command: devforgeai invoke-hooks --operation <op> [--story <story>] [--verbose]

Arguments:
  --operation: Operation name (required, e.g., dev, qa, release)
  --story: Story ID (optional, format: STORY-NNN)
  --verbose: Verbose logging output (optional, flag)

Exit Codes:
  0: Success
  1: Failure
"""

import logging
import re
from typing import Optional

# Exit codes
EXIT_CODE_SUCCESS = 0
EXIT_CODE_FAILURE = 1

# Validation constants
STORY_ID_PATTERN = r"^STORY-\d{3,}$"

# Configure logging
logger = logging.getLogger(__name__)


def invoke_hooks_command(
    operation: str, story_id: Optional[str] = None, verbose: bool = False
) -> int:
    """
    CLI command handler for 'devforgeai invoke-hooks'.

    Args:
        operation: Operation name (required)
        story_id: Optional story ID (format: STORY-NNN)
        verbose: Enable verbose logging

    Returns:
        Exit code (0 for success, 1 for failure)
    """
    _configure_logging(verbose)

    try:
        # Validate arguments
        if not _validate_operation(operation):
            return EXIT_CODE_FAILURE

        story_id = _validate_and_normalize_story_id(story_id)

        # Import here to avoid circular imports
        from ..hooks import invoke_hooks

        # Invoke the hook and return result
        return _execute_hook_invocation(operation, story_id, invoke_hooks)

    except Exception as e:
        logger.error(f"Unexpected error in invoke-hooks command: {str(e)}")
        logger.debug("", exc_info=True)
        return EXIT_CODE_FAILURE


def _configure_logging(verbose: bool) -> None:
    """Configure logging level based on verbosity flag."""
    level = logging.DEBUG if verbose else logging.INFO
    logging.basicConfig(level=level)


def _validate_operation(operation: str) -> bool:
    """
    Validate operation argument.

    Args:
        operation: Operation name to validate

    Returns:
        True if valid, False otherwise
    """
    if not operation:
        logger.error("--operation argument is required")
        return False
    return True


def _validate_and_normalize_story_id(story_id: Optional[str]) -> Optional[str]:
    """
    Validate and normalize story ID.

    Args:
        story_id: Story ID to validate

    Returns:
        Normalized story ID or None if invalid
    """
    if not story_id:
        return None

    if not _validate_story_id_format(story_id):
        logger.warning(f"Invalid story ID format: {story_id}, continuing with story_id=None")
        return None

    return story_id


def _execute_hook_invocation(
    operation: str,
    story_id: Optional[str],
    invoke_hooks_fn,
) -> int:
    """
    Execute hook invocation and return appropriate exit code.

    Args:
        operation: Operation name
        story_id: Optional story ID
        invoke_hooks_fn: Function to invoke hooks

    Returns:
        Exit code (0 for success, 1 for failure)
    """
    success = invoke_hooks_fn(operation, story_id)

    if success:
        logger.info(f"Feedback hook completed successfully: {operation}")
        return EXIT_CODE_SUCCESS
    else:
        logger.error(f"Feedback hook failed: {operation}")
        return EXIT_CODE_FAILURE


def _validate_story_id_format(story_id: str) -> bool:
    """
    Validate story ID format (STORY-NNN where N is digit).

    Args:
        story_id: Story ID to validate

    Returns:
        True if valid format, False otherwise
    """
    if not story_id:
        return False

    return bool(re.match(STORY_ID_PATTERN, story_id))
