"""
Headless mode answer resolution for CI/CD pipelines (STORY-098).

This package provides configuration-based answer resolution for AskUserQuestion
prompts when running in headless mode (CI=true or -p flag).
"""

from .exceptions import HeadlessResolutionError, ConfigurationError
from .answer_models import (
    AnswerEntry,
    HeadlessModeSettings,
    DefaultSettings,
    HeadlessAnswerConfiguration,
    load_config,
)
from .pattern_matcher import PromptPatternMatcher, MatchResult
from .answer_resolver import HeadlessAnswerResolver

__all__ = [
    "HeadlessResolutionError",
    "ConfigurationError",
    "AnswerEntry",
    "HeadlessModeSettings",
    "DefaultSettings",
    "HeadlessAnswerConfiguration",
    "load_config",
    "PromptPatternMatcher",
    "MatchResult",
    "HeadlessAnswerResolver",
]
