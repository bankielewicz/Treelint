"""
DevForgeAI Retrospective Feedback System

This module provides post-operation retrospective conversation capabilities
for capturing structured user feedback after command/skill execution.
"""

__version__ = '0.1.0'

from .retrospective import (
    trigger_retrospective,
    capture_feedback,
    is_skip_selected,
)

from .skip_tracking import (
    increment_skip,
    get_skip_count,
    reset_skip_count,
)

from .question_router import (
    get_context_aware_questions,
)

__all__ = [
    'trigger_retrospective',
    'capture_feedback',
    'is_skip_selected',
    'increment_skip',
    'get_skip_count',
    'reset_skip_count',
    'get_context_aware_questions',
]
