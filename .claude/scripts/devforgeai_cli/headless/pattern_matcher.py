"""
Pattern matching for AskUserQuestion prompts (STORY-098).

AC#2: Answer Matching Logic
- Regex pattern matching with case insensitivity
- First match wins
- Logging of matches when enabled

AC#4: Default Answer Fallback
- first_option: Use first available option
- skip: Return None (no resolution)
- fail: Raise HeadlessResolutionError
"""
import logging
import re
from dataclasses import dataclass
from typing import Dict, List, Optional, Pattern

from .exceptions import HeadlessResolutionError

logger = logging.getLogger(__name__)


@dataclass
class MatchResult:
    """Result of pattern matching."""

    key: str
    answer: str
    pattern: str
    is_default: bool = False


class PromptPatternMatcher:
    """
    Matches AskUserQuestion prompts to configured answers using regex patterns.

    NFR-001: Answer resolution time < 10ms per prompt lookup
    - Pre-compiles regex patterns on initialization
    - Uses case-insensitive matching
    """

    def __init__(
        self,
        patterns: Dict[str, dict],
        default_strategy: str = "fail",
        log_matches: bool = False,
    ):
        """
        Initialize pattern matcher.

        Args:
            patterns: Dict mapping key to {pattern, answer} entries
            default_strategy: What to do when no match found (fail|first_option|skip)
            log_matches: Whether to log match selections
        """
        self._patterns = patterns
        self._default_strategy = default_strategy
        self._log_matches = log_matches
        self._compiled: Dict[str, Pattern] = self._compile_patterns()

    def _compile_patterns(self) -> Dict[str, Pattern]:
        """Pre-compile regex patterns for performance."""
        compiled = {}
        for key, entry in self._patterns.items():
            pattern_str = entry.get("pattern", "") if isinstance(entry, dict) else str(entry)
            try:
                compiled[key] = re.compile(pattern_str, re.IGNORECASE)
            except re.error as e:
                logger.warning(f"Invalid regex pattern for '{key}': {pattern_str} - {e}")
                # Use literal match as fallback
                compiled[key] = re.compile(re.escape(pattern_str), re.IGNORECASE)
        return compiled

    def match(self, prompt_text: str) -> Optional[MatchResult]:
        """
        Match prompt to first matching pattern.

        AC#2: Answer Matching Logic
        - Returns first matching pattern (order matters)
        - Logs selection if log_matches enabled

        Args:
            prompt_text: The AskUserQuestion prompt text

        Returns:
            MatchResult if match found, None otherwise
        """
        for key, pattern in self._compiled.items():
            if pattern.search(prompt_text):
                entry = self._patterns[key]
                answer = entry.get("answer", "") if isinstance(entry, dict) else str(entry)

                if self._log_matches:
                    logger.info(f"CI Mode: Selected '{answer}' for prompt matching pattern '{key}'")

                return MatchResult(
                    key=key,
                    answer=answer,
                    pattern=entry.get("pattern", "") if isinstance(entry, dict) else str(entry),
                    is_default=False,
                )

        return None

    def match_with_fallback(
        self, prompt_text: str, options: List[str]
    ) -> Optional[MatchResult]:
        """
        Match prompt with fallback to default strategy.

        AC#4: Default Answer Fallback
        - first_option: Use first option from provided list
        - skip: Return None
        - fail: Raise HeadlessResolutionError

        Args:
            prompt_text: The AskUserQuestion prompt text
            options: Available answer options

        Returns:
            MatchResult if match found or default used, None if skip strategy

        Raises:
            HeadlessResolutionError: If fail strategy and no match
        """
        # Try exact/regex match first
        result = self.match(prompt_text)
        if result:
            return result

        # Apply default strategy
        if self._default_strategy == "first_option":
            if options:
                if self._log_matches:
                    logger.warning(
                        f"Using default answer for unmatched prompt: '{prompt_text[:50]}...'"
                    )
                return MatchResult(
                    key="_default",
                    answer=options[0],
                    pattern="",
                    is_default=True,
                )
            # No options available, fall through to fail
            self._default_strategy = "fail"

        if self._default_strategy == "skip":
            if self._log_matches:
                logger.warning(
                    f"Skipping unmatched prompt (skip strategy): '{prompt_text[:50]}...'"
                )
            return None

        # fail strategy (default)
        raise HeadlessResolutionError(prompt_text)
