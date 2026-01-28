"""
Data models for feedback system configuration management.

This module provides dataclass definitions for all configuration structures,
including validation in __post_init__ methods.
"""

from dataclasses import dataclass, field, asdict
from typing import List, Optional, Set
from enum import Enum


class TriggerMode(Enum):
    """Trigger modes for feedback collection."""
    ALWAYS = "always"
    FAILURES_ONLY = "failures-only"
    SPECIFIC_OPS = "specific-operations"
    NEVER = "never"


class TemplateFormat(Enum):
    """Template format options for feedback collection."""
    STRUCTURED = "structured"
    FREE_TEXT = "free-text"


class TemplateTone(Enum):
    """Template tone options for feedback questions."""
    BRIEF = "brief"
    DETAILED = "detailed"


# Validation constants
VALID_TEMPLATE_FORMATS: Set[str] = {"structured", "free-text"}
VALID_TEMPLATE_TONES: Set[str] = {"brief", "detailed"}
VALID_TRIGGER_MODES: Set[str] = {"always", "failures-only", "specific-operations", "never"}


@dataclass
class ConversationSettings:
    """Conversation-level settings for feedback collection.

    Attributes:
        max_questions: Maximum number of questions per conversation. 0 = unlimited.
        allow_skip: Whether users can skip feedback questions.
    """
    max_questions: int = 5
    allow_skip: bool = True

    def __post_init__(self):
        """Validate conversation settings."""
        if not isinstance(self.max_questions, int) or self.max_questions < 0:
            raise ValueError(
                f"max_questions must be non-negative integer, got {self.max_questions}"
            )
        if not isinstance(self.allow_skip, bool):
            raise ValueError(
                f"allow_skip must be boolean, got {self.allow_skip}"
            )


@dataclass
class SkipTrackingSettings:
    """Skip tracking configuration.

    Attributes:
        enabled: Whether skip tracking is active.
        max_consecutive_skips: Maximum consecutive skips allowed. 0 = unlimited.
        reset_on_positive: Whether to reset counter on positive feedback.
    """
    enabled: bool = True
    max_consecutive_skips: int = 3
    reset_on_positive: bool = True

    def __post_init__(self):
        """Validate skip tracking settings."""
        if not isinstance(self.enabled, bool):
            raise ValueError(
                f"enabled must be boolean, got {self.enabled}"
            )
        if not isinstance(self.max_consecutive_skips, int) or self.max_consecutive_skips < 0:
            raise ValueError(
                f"max_consecutive_skips must be non-negative integer, got {self.max_consecutive_skips}"
            )
        if not isinstance(self.reset_on_positive, bool):
            raise ValueError(
                f"reset_on_positive must be boolean, got {self.reset_on_positive}"
            )


@dataclass
class TemplateSettings:
    """Template preferences for feedback collection.

    Attributes:
        format: Template format (structured or free-text).
        tone: Template tone (brief or detailed).
    """
    format: str = "structured"  # structured|free-text
    tone: str = "brief"  # brief|detailed

    def __post_init__(self):
        """Validate template settings."""
        if self.format not in VALID_TEMPLATE_FORMATS:
            raise ValueError(
                f"Invalid template format: '{self.format}'. "
                f"Must be one of: {', '.join(sorted(VALID_TEMPLATE_FORMATS))}"
            )

        if self.tone not in VALID_TEMPLATE_TONES:
            raise ValueError(
                f"Invalid template tone: '{self.tone}'. "
                f"Must be one of: {', '.join(sorted(VALID_TEMPLATE_TONES))}"
            )


@dataclass
class FeedbackConfiguration:
    """Complete feedback system configuration.

    Attributes:
        enabled: Master enable/disable switch for all feedback operations.
        trigger_mode: Determines when feedback is collected.
        operations: List of specific operations (only if trigger_mode is specific-operations).
        conversation_settings: Conversation-level settings.
        skip_tracking: Skip tracking configuration.
        templates: Template preferences.
    """
    enabled: bool = True
    trigger_mode: str = "failures-only"
    operations: Optional[List[str]] = None
    conversation_settings: Optional[ConversationSettings] = None
    skip_tracking: Optional[SkipTrackingSettings] = None
    templates: Optional[TemplateSettings] = None

    def _normalize_nested_objects(self) -> None:
        """Convert dict to dataclass objects if needed and ensure defaults."""
        if isinstance(self.conversation_settings, dict):
            self.conversation_settings = ConversationSettings(**self.conversation_settings)
        elif self.conversation_settings is None:
            self.conversation_settings = ConversationSettings()

        if isinstance(self.skip_tracking, dict):
            self.skip_tracking = SkipTrackingSettings(**self.skip_tracking)
        elif self.skip_tracking is None:
            self.skip_tracking = SkipTrackingSettings()

        if isinstance(self.templates, dict):
            self.templates = TemplateSettings(**self.templates)
        elif self.templates is None:
            self.templates = TemplateSettings()

    def _validate_trigger_mode(self) -> None:
        """Validate trigger_mode field."""
        if self.trigger_mode not in VALID_TRIGGER_MODES:
            raise ValueError(
                f"Invalid trigger_mode value: '{self.trigger_mode}'. "
                f"Must be one of: {', '.join(sorted(VALID_TRIGGER_MODES))}"
            )

    def _validate_enabled(self) -> None:
        """Validate enabled field."""
        if not isinstance(self.enabled, bool):
            raise ValueError(
                f"enabled must be boolean, got {self.enabled}"
            )

    def _validate_operations(self) -> None:
        """Validate operations field (required only for specific-operations mode)."""
        if self.trigger_mode == "specific-operations":
            if self.operations is None or not isinstance(self.operations, list) or len(self.operations) == 0:
                raise ValueError(
                    "operations list must be provided and non-empty when trigger_mode is 'specific-operations'"
                )

    def __post_init__(self):
        """Initialize nested objects with defaults and validate configuration."""
        self._normalize_nested_objects()
        self._validate_enabled()
        self._validate_trigger_mode()
        self._validate_operations()

    def to_dict(self) -> dict:
        """Convert configuration to dictionary."""
        return {
            "enabled": self.enabled,
            "trigger_mode": self.trigger_mode,
            "operations": self.operations,
            "conversation_settings": asdict(self.conversation_settings),
            "skip_tracking": asdict(self.skip_tracking),
            "templates": asdict(self.templates),
        }
