"""
Configuration models for headless answer resolution (STORY-098).

Follows patterns from feedback/config_manager.py and config_models.py.
"""
import logging
import re
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, Optional, Any, List

import yaml

from .exceptions import ConfigurationError

logger = logging.getLogger(__name__)

# Valid values for defaults.unknown_prompt
VALID_DEFAULT_STRATEGIES = ["fail", "first_option", "skip"]


@dataclass
class AnswerEntry:
    """Single answer configuration with pattern and answer."""

    pattern: str
    answer: str
    description: Optional[str] = None

    def __post_init__(self):
        if not self.pattern:
            raise ValueError("AnswerEntry requires a pattern")
        if not self.answer:
            raise ValueError("AnswerEntry requires an answer")


@dataclass
class HeadlessModeSettings:
    """Headless mode configuration settings."""

    enabled: bool = True
    fail_on_unanswered: bool = True
    log_matches: bool = True

    def __post_init__(self):
        if not isinstance(self.enabled, bool):
            raise TypeError(f"enabled must be boolean, got {type(self.enabled).__name__}")
        if not isinstance(self.fail_on_unanswered, bool):
            raise TypeError(
                f"fail_on_unanswered must be boolean, got {type(self.fail_on_unanswered).__name__}"
            )
        if not isinstance(self.log_matches, bool):
            raise TypeError(f"log_matches must be boolean, got {type(self.log_matches).__name__}")


@dataclass
class DefaultSettings:
    """Default behavior settings for unmatched prompts."""

    unknown_prompt: str = "fail"

    def __post_init__(self):
        if self.unknown_prompt not in VALID_DEFAULT_STRATEGIES:
            raise ValueError(
                f"unknown_prompt must be one of {VALID_DEFAULT_STRATEGIES}, "
                f"got '{self.unknown_prompt}'"
            )


@dataclass
class HeadlessAnswerConfiguration:
    """Complete headless answer configuration."""

    headless_mode: HeadlessModeSettings
    answers: Dict[str, AnswerEntry]
    defaults: DefaultSettings

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "HeadlessAnswerConfiguration":
        """Create configuration from dictionary."""
        # Parse headless_mode section
        hm_data = data.get("headless_mode", {})
        headless_mode = HeadlessModeSettings(
            enabled=hm_data.get("enabled", True),
            fail_on_unanswered=hm_data.get("fail_on_unanswered", True),
            log_matches=hm_data.get("log_matches", True),
        )

        # Parse answers section
        answers_data = data.get("answers", {})
        answers = {}
        for key, entry in answers_data.items():
            if isinstance(entry, dict):
                # Validate regex pattern
                pattern = entry.get("pattern", "")
                try:
                    re.compile(pattern, re.IGNORECASE)
                except re.error as e:
                    logger.warning(f"Invalid regex pattern for '{key}': {pattern} - {e}")

                answers[key] = AnswerEntry(
                    pattern=pattern,
                    answer=entry.get("answer", ""),
                    description=entry.get("description"),
                )

        # Parse defaults section
        defaults_data = data.get("defaults", {})
        defaults = DefaultSettings(unknown_prompt=defaults_data.get("unknown_prompt", "fail"))

        return cls(headless_mode=headless_mode, answers=answers, defaults=defaults)


def _detect_flat_format(data: Dict[str, Any]) -> bool:
    """Detect if config is in legacy flat format."""
    flat_keys = {
        "test_failure_action",
        "deferral_strategy",
        "priority_default",
        "technology_choice",
        "circular_dependency_action",
        "git_conflict_strategy",
        "custom_answers",
    }
    return bool(flat_keys.intersection(data.keys()))


def _migrate_flat_to_nested(data: Dict[str, Any]) -> Dict[str, Any]:
    """Migrate flat format to nested format with deprecation warning."""
    logger.warning(
        "Flat ci-answers.yaml format is deprecated. "
        "Please migrate to nested format with headless_mode, answers, and defaults sections."
    )

    # Map flat keys to patterns
    key_to_pattern = {
        "test_failure_action": "Tests failed.*How should we proceed",
        "deferral_strategy": "Do you approve this deferral",
        "priority_default": "What is the story priority",
        "technology_choice": "technology.*not specified",
        "circular_dependency_action": "circular dependency",
        "git_conflict_strategy": "Git conflict",
    }

    answers = {}
    for key, pattern in key_to_pattern.items():
        if key in data:
            answers[key] = {"pattern": pattern, "answer": str(data[key])}

    # Handle custom_answers if present
    custom = data.get("custom_answers", {})
    if isinstance(custom, dict):
        for pattern, answer in custom.items():
            safe_key = re.sub(r"[^a-zA-Z0-9_]", "_", pattern)[:50]
            answers[f"custom_{safe_key}"] = {"pattern": pattern, "answer": str(answer)}

    return {
        "headless_mode": {"enabled": True, "fail_on_unanswered": True, "log_matches": True},
        "answers": answers,
        "defaults": {"unknown_prompt": "fail"},
    }


def load_config(config_path: Path) -> HeadlessAnswerConfiguration:
    """
    Load and validate configuration from YAML file.

    AC#5: Answer Validation on Load
    - Validates YAML syntax
    - Validates required fields
    - Validates field types and enum values

    Args:
        config_path: Path to ci-answers.yaml file

    Returns:
        HeadlessAnswerConfiguration object

    Raises:
        ConfigurationError: If YAML is malformed
        ValueError: If required fields are missing or invalid
        KeyError: If required sections are missing
    """
    if not config_path.exists():
        raise ConfigurationError(f"Configuration file not found: {config_path}")

    try:
        with open(config_path, "r") as f:
            data = yaml.safe_load(f)
    except yaml.YAMLError as e:
        line = getattr(e, "problem_mark", None)
        line_num = line.line + 1 if line else None
        raise ConfigurationError(f"YAML parsing error: {e}", line_number=line_num)

    if data is None:
        raise ConfigurationError("Empty configuration file")

    # Check for flat format and migrate
    if _detect_flat_format(data):
        data = _migrate_flat_to_nested(data)

    # Validate required sections for nested format
    if "defaults" not in data:
        raise ValueError("Configuration missing required 'defaults' section")

    return HeadlessAnswerConfiguration.from_dict(data)
