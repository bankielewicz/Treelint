"""
HeadlessAnswerResolver service (STORY-098).

Main entry point for resolving AskUserQuestion prompts in headless mode.
Follows singleton pattern from feedback/config_manager.py.

AC#1: CI Answers Configuration File
- Loads from devforgeai/config/ci-answers.yaml with fallbacks
- Supports both nested (preferred) and flat (deprecated) formats

AC#3: Fail-on-Unanswered Mode
- Raises HeadlessResolutionError when fail_on_unanswered=true and no match

BR-002: Interactive mode ignores ci-answers.yaml
- Only resolves when is_headless_mode() returns True
"""
import logging
import os
import threading
from pathlib import Path
from typing import List, Optional

from .answer_models import HeadlessAnswerConfiguration, load_config
from .exceptions import HeadlessResolutionError, ConfigurationError
from .pattern_matcher import PromptPatternMatcher, MatchResult

logger = logging.getLogger(__name__)


class HeadlessAnswerResolver:
    """
    Resolves AskUserQuestion prompts from CI configuration.

    Singleton pattern for consistent configuration across invocations.
    """

    _instance: Optional["HeadlessAnswerResolver"] = None
    _lock = threading.Lock()

    def __init__(
        self,
        config_path: Optional[Path] = None,
        search_paths: Optional[List[Path]] = None,
    ):
        """
        Initialize resolver with configuration path.

        Args:
            config_path: Explicit path to ci-answers.yaml
            search_paths: List of paths to search for config (in order)
        """
        self._config_path = config_path
        self._search_paths = search_paths or self._default_search_paths()
        self._config: Optional[HeadlessAnswerConfiguration] = None
        self._matcher: Optional[PromptPatternMatcher] = None
        self._loaded = False

    @staticmethod
    def _default_search_paths() -> List[Path]:
        """Default paths to search for ci-answers.yaml."""
        cwd = Path.cwd()
        return [
            cwd / "devforgeai" / "config" / "ci-answers.yaml",
            cwd / "devforgeai" / "config" / "ci" / "ci-answers.yaml",
            Path.home() / "devforgeai" / "config" / "ci-answers.yaml",
        ]

    def _find_config_file(self) -> Optional[Path]:
        """Find ci-answers.yaml in search paths."""
        if self._config_path and self._config_path.exists():
            return self._config_path

        for path in self._search_paths:
            if path.exists():
                logger.debug(f"Found ci-answers.yaml at: {path}")
                return path

        return None

    @classmethod
    def get_instance(cls) -> "HeadlessAnswerResolver":
        """
        Get singleton instance.

        Thread-safe singleton pattern following config_manager.py.
        """
        if cls._instance is None:
            with cls._lock:
                if cls._instance is None:
                    cls._instance = cls()
        return cls._instance

    @classmethod
    def reset_instance(cls) -> None:
        """Reset singleton instance (for testing)."""
        with cls._lock:
            cls._instance = None

    def is_configured(self) -> bool:
        """Check if configuration file exists."""
        return self._find_config_file() is not None

    def is_headless_mode(self) -> bool:
        """
        Check if running in headless mode.

        Detects:
        - CI=true environment variable
        - DEVFORGEAI_HEADLESS=true environment variable
        - Non-interactive terminal (stdin not a tty)
        """
        if os.environ.get("CI") == "true":
            return True
        if os.environ.get("DEVFORGEAI_HEADLESS") == "true":
            return True
        # Check if stdin is a tty (interactive)
        try:
            return not os.isatty(0)
        except Exception:
            return False

    def load_configuration(self) -> HeadlessAnswerConfiguration:
        """
        Load configuration from file.

        AC#1: CI Answers Configuration File
        - Reads from configured path or search paths
        - Validates configuration on load (AC#5)

        Returns:
            HeadlessAnswerConfiguration object

        Raises:
            ConfigurationError: If config file not found or invalid
        """
        if self._loaded and self._config:
            return self._config

        config_path = self._find_config_file()
        if config_path is None:
            if self._config_path:
                raise ConfigurationError(f"Configuration file not found: {self._config_path}")
            raise ConfigurationError(
                "No ci-answers.yaml found in search paths: "
                + ", ".join(str(p) for p in self._search_paths)
            )

        self._config = load_config(config_path)
        self._loaded = True

        # Initialize pattern matcher
        self._matcher = PromptPatternMatcher(
            patterns={k: {"pattern": v.pattern, "answer": v.answer} for k, v in self._config.answers.items()},
            default_strategy=self._config.defaults.unknown_prompt,
            log_matches=self._config.headless_mode.log_matches,
        )

        logger.info(f"Loaded headless configuration from: {config_path}")
        return self._config

    def resolve(
        self, prompt_text: str, options: List[str]
    ) -> Optional[str]:
        """
        Resolve prompt to configured answer.

        AC#2: Answer Matching Logic
        - Matches prompt text against configured patterns
        - Returns first matching answer

        AC#3: Fail-on-Unanswered Mode
        - Raises HeadlessResolutionError if no match and fail_on_unanswered=true

        AC#4: Default Answer Fallback
        - Uses default strategy when no pattern matches

        Args:
            prompt_text: The AskUserQuestion prompt text
            options: Available answer options

        Returns:
            Selected answer string, or None if skip strategy

        Raises:
            HeadlessResolutionError: If no match and fail strategy
        """
        if not self._loaded:
            self.load_configuration()

        if self._matcher is None:
            raise ConfigurationError("Configuration not loaded")

        # Check fail_on_unanswered setting
        if self._config and self._config.headless_mode.fail_on_unanswered:
            # Use fail strategy if configured
            result = self._matcher.match(prompt_text)
            if result:
                return result.answer
            # No match - apply fail_on_unanswered
            raise HeadlessResolutionError(prompt_text)

        # Use default strategy
        result = self._matcher.match_with_fallback(prompt_text, options)
        return result.answer if result else None
