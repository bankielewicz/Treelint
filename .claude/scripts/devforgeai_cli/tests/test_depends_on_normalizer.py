#!/usr/bin/env python3
"""
Tests for Depends On Normalizer - STORY-090 AC#5

TDD tests for input normalization logic.
Run: pytest tests/test_depends_on_normalizer.py -v
"""

import pytest
from devforgeai_cli.utils.depends_on_normalizer import (
    normalize_depends_on,
    is_valid_story_id,
    validate_depends_on_input,
    STORY_ID_PATTERN
)


class TestIsValidStoryId:
    """Test STORY-ID format validation."""

    def test_valid_three_digit_story_id(self):
        assert is_valid_story_id("STORY-044") == True

    def test_valid_four_digit_story_id(self):
        assert is_valid_story_id("STORY-1234") == True

    def test_invalid_lowercase_story_id(self):
        assert is_valid_story_id("story-044") == False

    def test_invalid_insufficient_digits(self):
        assert is_valid_story_id("STORY-44") == False

    def test_invalid_missing_prefix(self):
        assert is_valid_story_id("044") == False

    def test_invalid_wrong_prefix(self):
        assert is_valid_story_id("EPIC-044") == False

    def test_invalid_extra_characters(self):
        assert is_valid_story_id("STORY-044a") == False

    def test_invalid_none(self):
        assert is_valid_story_id(None) == False

    def test_invalid_empty_string(self):
        assert is_valid_story_id("") == False


class TestNormalizeDependsOn:
    """Test normalize_depends_on function."""

    # Null/Empty handling
    def test_null_to_empty_array(self):
        assert normalize_depends_on(None) == []

    def test_empty_string_to_empty_array(self):
        assert normalize_depends_on("") == []

    def test_none_string_to_empty_array(self):
        assert normalize_depends_on("none") == []

    def test_none_uppercase_to_empty_array(self):
        assert normalize_depends_on("NONE") == []

    def test_null_string_to_empty_array(self):
        assert normalize_depends_on("null") == []

    def test_empty_brackets_to_empty_array(self):
        assert normalize_depends_on("[]") == []

    # Single string normalization
    def test_single_story_string_to_array(self):
        assert normalize_depends_on("STORY-044") == ["STORY-044"]

    def test_single_lowercase_converted_to_uppercase(self):
        assert normalize_depends_on("story-044") == ["STORY-044"]

    # Multiple items - comma separated
    def test_comma_separated_to_array(self):
        result = normalize_depends_on("STORY-044, STORY-045")
        assert result == ["STORY-044", "STORY-045"]

    def test_comma_no_space_to_array(self):
        result = normalize_depends_on("STORY-044,STORY-045")
        assert result == ["STORY-044", "STORY-045"]

    # Multiple items - space separated
    def test_space_separated_to_array(self):
        result = normalize_depends_on("STORY-044 STORY-045")
        assert result == ["STORY-044", "STORY-045"]

    # Array input
    def test_array_passthrough(self):
        result = normalize_depends_on(["STORY-044", "STORY-045"])
        assert result == ["STORY-044", "STORY-045"]

    def test_array_filters_invalid(self):
        result = normalize_depends_on(["STORY-044", "invalid", "STORY-045"])
        assert result == ["STORY-044", "STORY-045"]

    def test_empty_array_stays_empty(self):
        assert normalize_depends_on([]) == []

    # Edge cases
    def test_whitespace_trimmed(self):
        result = normalize_depends_on("  STORY-044  ,  STORY-045  ")
        assert result == ["STORY-044", "STORY-045"]

    def test_mixed_case_normalized(self):
        result = normalize_depends_on("Story-044, story-045, STORY-046")
        assert result == ["STORY-044", "STORY-045", "STORY-046"]


class TestValidateDependsOnInput:
    """Test validate_depends_on_input function."""

    def test_mixed_input_returns_both(self):
        valid, invalid = validate_depends_on_input("STORY-044, invalid, STORY-045")
        assert valid == ["STORY-044", "STORY-045"]
        assert invalid == ["invalid"]

    def test_all_valid_returns_empty_invalid(self):
        valid, invalid = validate_depends_on_input("STORY-044, STORY-045")
        assert valid == ["STORY-044", "STORY-045"]
        assert invalid == []

    def test_all_invalid_returns_empty_valid(self):
        valid, invalid = validate_depends_on_input("invalid, bad")
        assert valid == []
        assert invalid == ["invalid", "bad"]

    def test_none_returns_empty_lists(self):
        valid, invalid = validate_depends_on_input(None)
        assert valid == []
        assert invalid == []

    def test_empty_string_returns_empty_lists(self):
        valid, invalid = validate_depends_on_input("")
        assert valid == []
        assert invalid == []

    def test_list_with_mixed_entries(self):
        valid, invalid = validate_depends_on_input(["STORY-044", "bad", "STORY-045", ""])
        assert valid == ["STORY-044", "STORY-045"]
        assert invalid == ["bad"]


class TestStoryIdPattern:
    """Test the regex pattern directly."""

    def test_pattern_matches_three_digits(self):
        assert STORY_ID_PATTERN.match("STORY-044") is not None

    def test_pattern_matches_four_digits(self):
        assert STORY_ID_PATTERN.match("STORY-1234") is not None

    def test_pattern_rejects_two_digits(self):
        assert STORY_ID_PATTERN.match("STORY-44") is None

    def test_pattern_rejects_five_digits(self):
        assert STORY_ID_PATTERN.match("STORY-12345") is None

    def test_pattern_rejects_lowercase(self):
        assert STORY_ID_PATTERN.match("story-044") is None

    def test_pattern_rejects_no_hyphen(self):
        assert STORY_ID_PATTERN.match("STORY044") is None


if __name__ == '__main__':
    pytest.main([__file__, '-v'])
