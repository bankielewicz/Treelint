"""Comprehensive unit tests for validation.py module.

Tests all validation functions for complete coverage.
"""

import pytest
from devforgeai_cli.feedback.validation import (
    validate_response_length,
    detect_spam,
    is_coherent_text,
    check_sensitive_content,
    validate_story_id,
    validate_workflow_type,
)


class TestValidateResponseLength:
    """Test validate_response_length function."""

    def test_valid_length_no_warning(self):
        """Test response within valid range with no warning."""
        response = "This is a normal response"
        is_valid, warning = validate_response_length(response)
        assert is_valid is True
        assert warning is None

    def test_too_short_response(self):
        """Test response below minimum length."""
        response = "Hi"
        is_valid, warning = validate_response_length(response, min_length=5)
        assert is_valid is False
        assert "too short" in warning

    def test_too_long_response(self):
        """Test response exceeding maximum length."""
        response = "x" * 11000
        is_valid, warning = validate_response_length(response, max_length=10000)
        assert is_valid is False
        assert "too long" in warning

    def test_warning_threshold_triggered(self):
        """Test response triggering warning threshold."""
        response = "x" * 2500
        is_valid, warning = validate_response_length(response, warn_threshold=2000)
        assert is_valid is True
        assert warning is not None
        assert "long" in warning


class TestDetectSpam:
    """Test detect_spam function."""

    def test_empty_text_is_spam(self):
        """Test that empty text is detected as spam (line 47)."""
        assert detect_spam("") is True

    def test_character_repetition_detected(self):
        """Test detection of character repetition."""
        # Need more than 10 chars with 3 or fewer unique chars
        assert detect_spam("aaaaaaaaaaa") is True  # 11 a's (only 1 unique char)
        assert detect_spam("abababababab") is True  # 12 chars (only 2 unique chars)
        assert detect_spam("111111111111") is True  # 12 1's (only 1 unique char)

    def test_pattern_repetition_detected(self):
        """Test detection of pattern repetition (lines 61-70)."""
        # Pattern "123" repeated
        assert detect_spam("123123123123123123") is True
        # Pattern "abc" repeated
        assert detect_spam("abcabcabcabcabcabcabcabc") is True

    def test_low_word_count_spam(self):
        """Test detection of low word count with long text."""
        # More than 50 chars but less than 5 words
        # Need to have 4 or fewer words with more than 50 chars
        assert detect_spam("aaa bbb ccc ddd" + "x" * 40) is True  # 4 words, 55 chars total

    def test_high_non_alphanumeric_ratio(self):
        """Test detection of high non-alphanumeric ratio."""
        # Mostly special characters
        assert detect_spam("!@#$%^&*()_+{}|:<>?[]\\;',./~`") is True

    def test_valid_text_not_spam(self):
        """Test that valid text is not flagged as spam."""
        assert detect_spam("This is a coherent piece of feedback text") is False
        assert detect_spam("I found the documentation unclear and would like more examples") is False


class TestIsCoherentText:
    """Test is_coherent_text function."""

    def test_short_text_always_coherent(self):
        """Test that short text (<5 chars) is always considered coherent (line 84)."""
        assert is_coherent_text("hi") is True
        assert is_coherent_text("ok") is True
        assert is_coherent_text("yes") is True

    def test_single_character_repetition(self):
        """Test detection of single character repetition."""
        assert is_coherent_text("aaaaaaaaaa") is False
        assert is_coherent_text("1111111111") is False

    def test_pattern_repetition_incoherent(self):
        """Test detection of repeated patterns."""
        # Pattern "123" repeated 3+ times
        assert is_coherent_text("123123123") is False
        # Pattern "ab" repeated 3+ times
        assert is_coherent_text("ababababab") is False

    def test_partial_pattern_match_at_end(self):
        """Test detection of pattern with partial match at end."""
        # Pattern "abc" repeated 3 times plus partial "ab"
        assert is_coherent_text("abcabcabcab") is False

    def test_coherent_text_accepted(self):
        """Test that truly coherent text is accepted."""
        assert is_coherent_text("This is coherent text") is True
        assert is_coherent_text("I would like to provide feedback on the TDD workflow") is True


class TestCheckSensitiveContent:
    """Test check_sensitive_content function."""

    def test_api_key_pattern_detected(self):
        """Test detection of API key pattern (secret)."""
        feedback = "The command exposed my key: sk-proj1234567890abcdefghij1234567890"
        is_sensitive, types = check_sensitive_content(feedback)
        assert is_sensitive is True
        assert 'secret' in types

    def test_api_key_word_detected(self):
        """Test detection of 'api key' mention."""
        feedback = "My API_KEY was visible in the output"
        is_sensitive, types = check_sensitive_content(feedback)
        assert is_sensitive is True
        assert 'api_key' in types

    def test_data_loss_concern_detected(self):
        """Test detection of data loss concerns."""
        feedback = "I experienced data loss during the migration"
        is_sensitive, types = check_sensitive_content(feedback)
        assert is_sensitive is True
        assert 'data_loss' in types

    def test_security_breach_detected(self):
        """Test detection of security breach (lines 129-130)."""
        feedback = "There was a security breach in the system"
        is_sensitive, types = check_sensitive_content(feedback)
        assert is_sensitive is True
        assert 'critical_issue' in types

    def test_vulnerability_without_api_key(self):
        """Test vulnerability detection when no API key mentioned."""
        feedback = "I found a vulnerability in the input validation"
        is_sensitive, types = check_sensitive_content(feedback)
        assert is_sensitive is True
        assert 'critical_issue' in types

    def test_non_sensitive_feedback(self):
        """Test that normal feedback is not flagged as sensitive."""
        feedback = "The documentation could be clearer"
        is_sensitive, types = check_sensitive_content(feedback)
        assert is_sensitive is False
        assert len(types) == 0

    def test_multiple_sensitive_types(self):
        """Test detection of multiple sensitive types."""
        feedback = "API key exposed and data loss occurred in production database"
        is_sensitive, types = check_sensitive_content(feedback)
        assert is_sensitive is True
        assert 'api_key' in types
        assert 'data_loss' in types


class TestValidateStoryId:
    """Test validate_story_id function."""

    def test_valid_story_ids(self):
        """Test that valid story IDs are accepted."""
        assert validate_story_id("STORY-001") is True
        assert validate_story_id("STORY-123") is True
        assert validate_story_id("STORY-999") is True
        assert validate_story_id("STORY-1") is True

    def test_invalid_story_ids(self):
        """Test that invalid story IDs are rejected."""
        assert validate_story_id("story-001") is False  # lowercase
        assert validate_story_id("STORY-abc") is False  # non-numeric
        assert validate_story_id("TASK-001") is False   # wrong prefix
        assert validate_story_id("STORY001") is False   # missing hyphen
        assert validate_story_id("STORY-") is False     # missing number


class TestValidateWorkflowType:
    """Test validate_workflow_type function."""

    def test_valid_workflow_types(self):
        """Test that all valid workflow types are accepted."""
        valid_types = [
            'dev', 'qa', 'orchestrate', 'release', 'ideate',
            'create-story', 'create-epic', 'create-sprint'
        ]
        for workflow_type in valid_types:
            assert validate_workflow_type(workflow_type) is True

    def test_invalid_workflow_types(self):
        """Test that invalid workflow types are rejected."""
        assert validate_workflow_type('invalid') is False
        assert validate_workflow_type('deploy') is False
        assert validate_workflow_type('build') is False
        assert validate_workflow_type('') is False
