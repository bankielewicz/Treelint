"""Feature flag implementation for feedback system.

This module handles checking whether feedback collection is enabled
and provides graceful degradation when disabled.
"""

import os
from pathlib import Path
from typing import Optional

try:
    import yaml
    YAML_AVAILABLE = True
except ImportError:
    YAML_AVAILABLE = False


def should_enable_feedback() -> bool:
    """Check if feedback system is enabled.

    Checks in order:
    1. Environment variable DEVFORGEAI_DISABLE_FEEDBACK
    2. Config file devforgeai/feedback/config.yaml
    3. Default: True (opt-in)

    Returns:
        bool: True if feedback should be collected, False otherwise
    """
    # Check environment variable (highest priority)
    if os.getenv('DEVFORGEAI_DISABLE_FEEDBACK', '').lower() == 'true':
        return False

    # Check config file
    config_path = Path('devforgeai/feedback/config.yaml')
    if config_path.exists() and YAML_AVAILABLE:
        try:
            with open(config_path) as f:
                config = yaml.safe_load(f)
                if config:
                    # Check enable_feedback flag
                    if config.get('enable_feedback') is False:
                        return False

                    # Check mode (disabled is equivalent to enable_feedback: false)
                    if config.get('mode') == 'disabled':
                        return False
        except Exception:
            # If config can't be read, continue to default
            pass

    # Default: enabled (opt-in)
    return True


def get_collection_mode() -> str:
    """Get the feedback collection mode.

    Returns:
        str: Collection mode ('all', 'failures_only', or 'disabled')
    """
    if not should_enable_feedback():
        return 'disabled'

    config_path = Path('devforgeai/feedback/config.yaml')
    if config_path.exists() and YAML_AVAILABLE:
        try:
            with open(config_path) as f:
                config = yaml.safe_load(f)
                if config:
                    return config.get('mode', 'all')
        except Exception:
            pass

    # Default: collect after all operations
    return 'all'


def should_collect_for_operation(workflow_type: str, success_status: str) -> bool:
    """Determine if feedback should be collected for this operation.

    Args:
        workflow_type: Type of workflow (dev, qa, orchestrate, etc.)
        success_status: Outcome (success, failed, partial)

    Returns:
        bool: True if feedback should be collected
    """
    if not should_enable_feedback():
        return False

    mode = get_collection_mode()

    if mode == 'disabled':
        return False
    elif mode == 'failures_only':
        return success_status in ['failed', 'partial']
    elif mode == 'all':
        return True
    else:
        # Unknown mode, default to all
        return True


def trigger_retrospective_if_enabled(
    workflow_type: str,
    story_id: str,
    success_status: str
) -> Optional[dict]:
    """Trigger feedback collection if enabled.

    This is the main entry point for feedback collection. It checks
    feature flags and collection mode before triggering retrospective.

    Args:
        workflow_type: Type of workflow that completed
        story_id: Story ID associated with operation
        success_status: Outcome of operation

    Returns:
        dict | None: Feedback data if collected, None if skipped
    """
    if not should_collect_for_operation(workflow_type, success_status):
        return None

    # Import here to avoid circular dependency
    from .retrospective import trigger_retrospective

    try:
        return trigger_retrospective(workflow_type, story_id, success_status)
    except Exception as e:
        # Graceful degradation: log error, return None
        import logging
        logging.warning(f"Feedback collection failed: {e}")
        return None


# Example usage in command implementations:
#
# from .feedback.feature_flag import trigger_retrospective_if_enabled
#
# # After operation completes:
# feedback = trigger_retrospective_if_enabled(
#     workflow_type='dev',
#     story_id='STORY-042',
#     success_status='success'
# )
#
# # Command continues regardless of feedback result
# # feedback will be None if:
# # - Feature disabled
# # - Collection mode is failures_only and operation succeeded
# # - Feedback collection failed (graceful degradation)
