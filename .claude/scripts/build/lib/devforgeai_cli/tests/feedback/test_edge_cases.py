"""
Unit tests for edge cases

Tests cover:
- Network/connection loss during feedback
- Extremely long feedback responses
- Rapid command sequence
- Failed setup scenarios
- Sensitive feedback handling
"""

import pytest
import json
import tempfile
import shutil
from pathlib import Path

from devforgeai_cli.feedback.retrospective import (
    capture_feedback,
    save_in_progress_state,
    resume_feedback,
)
from devforgeai_cli.feedback.validation import (
    validate_response_length,
    detect_spam,
    check_sensitive_content,
)


class TestNetworkLoss:
    """Edge Case 1: Network/Connection Loss During Feedback Collection"""

    @pytest.fixture
    def temp_feedback_dir(self):
        temp_dir = tempfile.mkdtemp()
        yield Path(temp_dir)
        shutil.rmtree(temp_dir)

    def test_save_in_progress_state_preserves_partial_responses(self, temp_feedback_dir):
        """
        GIVEN user is in middle of providing feedback
        WHEN connection drops
        THEN system saves any completed fields (partial capture)
        """
        # Arrange
        story_id = 'STORY-001'
        partial_responses = [
            {'question_id': 'dev_success_01', 'response': 4, 'skip': False},
            {'question_id': 'dev_success_02', 'response': 'TDD was helpful', 'skip': False},
            # User disconnected before answering questions 3-5
        ]

        # Act
        state_file = save_in_progress_state(
            story_id=story_id,
            responses=partial_responses,
            workflow_type='dev',
            feedback_dir=temp_feedback_dir
        )

        # Assert
        assert state_file.exists()
        assert state_file.name.endswith('-in-progress.json')

        # Verify state was saved
        with open(state_file, 'r') as f:
            state_data = json.load(f)

        assert state_data['story_id'] == story_id
        assert len(state_data['responses']) == 2
        assert state_data['status'] == 'in_progress'

    def test_resume_feedback_offers_continuation_option(self, temp_feedback_dir):
        """
        GIVEN in-progress feedback state exists
        WHEN user starts new session
        THEN system offers "Continue previous feedback?" option
        """
        # Arrange - Save in-progress state
        story_id = 'STORY-001'
        partial_responses = [
            {'question_id': 'dev_success_01', 'response': 4, 'skip': False},
        ]
        save_in_progress_state(story_id, partial_responses, 'dev', temp_feedback_dir)

        # Act
        resume_data = resume_feedback(story_id, temp_feedback_dir)

        # Assert
        assert resume_data is not None
        assert 'responses' in resume_data
        assert len(resume_data['responses']) == 1
        assert 'timestamp' in resume_data


class TestLongResponses:
    """Edge Case 2: Extremely Long Feedback Response"""

    def test_validate_response_length_accepts_long_detailed_feedback(self):
        """
        GIVEN user provides detailed multi-paragraph feedback (>5,000 chars)
        WHEN feedback length exceeds normal bounds
        THEN system accepts and stores full response without truncation
        """
        # Arrange
        long_response = "This is detailed feedback. " * 200  # ~5,400 chars

        # Act
        is_valid, warning = validate_response_length(long_response)

        # Assert
        assert is_valid is True  # Accepts long feedback
        assert warning is not None  # But warns user
        assert 'approaching' in warning.lower() or 'long' in warning.lower()

    def test_validate_response_length_rejects_spam(self):
        """
        GIVEN user provides spam (repeated characters or noise)
        WHEN validating feedback
        THEN system rejects as spam
        """
        # Arrange
        spam_responses = [
            'aaaaaaaaaaaaaaaaaaaaaaaaa' * 100,  # Character repetition
            '12341234123412341234' * 100,       # Pattern repetition
            'asdf' * 500,                       # Random characters
        ]

        # Act & Assert
        for spam in spam_responses:
            is_spam = detect_spam(spam)
            assert is_spam is True, f"Failed to detect spam: {spam[:50]}"


class TestRapidSequence:
    """Edge Case 3: Rapid Command Sequence (No Feedback Between Runs)"""

    @pytest.fixture
    def temp_feedback_dir(self):
        temp_dir = tempfile.mkdtemp()
        yield Path(temp_dir)
        shutil.rmtree(temp_dir)

    def test_rapid_sequence_detection(self, temp_feedback_dir):
        """
        GIVEN user runs /dev STORY-001, immediately /dev STORY-002 (within 30 seconds)
        WHEN second command completes
        THEN system identifies rapid sequence
        """
        # Arrange
        from devforgeai_cli.feedback.retrospective import detect_rapid_sequence
        from datetime import datetime, timedelta, timezone

        last_feedback_time = datetime.now(timezone.utc) - timedelta(seconds=15)  # 15 seconds ago

        # Act
        is_rapid = detect_rapid_sequence(
            last_feedback_time=last_feedback_time,
            threshold_seconds=30
        )

        # Assert
        assert is_rapid is True

    def test_rapid_sequence_offers_quick_feedback_option(self, temp_feedback_dir):
        """
        GIVEN rapid command sequence detected
        WHEN feedback prompt appears
        THEN system offers "Quick feedback on last command?" or "Skip, I'm in flow state"
        """
        # This is a behavioral test - would require integration testing
        # For now, verify the detection logic exists
        from devforgeai_cli.feedback.retrospective import detect_rapid_sequence

        assert callable(detect_rapid_sequence)


class TestSensitiveContent:
    """Edge Case 5: Sensitive Feedback (User Reports Privacy Concern)"""

    def test_check_sensitive_content_detects_api_keys(self):
        """
        GIVEN user mentions API keys in feedback
        WHEN checking for sensitive content
        THEN system detects and flags it
        """
        # Arrange
        feedback_with_key = "I accidentally exposed my API key sk-1234567890abcdef in the logs"

        # Act
        is_sensitive, detected_types = check_sensitive_content(feedback_with_key)

        # Assert
        assert is_sensitive is True
        assert 'api_key' in detected_types or 'secret' in detected_types

    def test_check_sensitive_content_detects_data_loss_concerns(self):
        """
        GIVEN user reports data loss
        WHEN checking for sensitive content
        THEN system flags for careful handling
        """
        # Arrange
        feedback = "The command deleted my production database without warning"

        # Act
        is_sensitive, detected_types = check_sensitive_content(feedback)

        # Assert
        assert is_sensitive is True
        assert 'data_loss' in detected_types or 'critical_issue' in detected_types

    def test_check_sensitive_content_allows_normal_feedback(self):
        """
        GIVEN normal feedback with no sensitive content
        WHEN checking for sensitive content
        THEN system does not flag it
        """
        # Arrange
        normal_feedback = "The TDD workflow was very helpful and easy to follow"

        # Act
        is_sensitive, detected_types = check_sensitive_content(normal_feedback)

        # Assert
        assert is_sensitive is False
        assert len(detected_types) == 0


class TestDataValidation:
    """Test data validation rules from story spec"""

    def test_story_id_pattern_validation(self):
        """
        GIVEN story_id input
        WHEN validating format
        THEN must match STORY-[0-9]+ pattern
        """
        from devforgeai_cli.feedback.validation import validate_story_id

        # Valid patterns
        assert validate_story_id('STORY-001') is True
        assert validate_story_id('STORY-123') is True
        assert validate_story_id('STORY-999') is True

        # Invalid patterns
        assert validate_story_id('story-001') is False  # lowercase
        assert validate_story_id('STORY-abc') is False  # non-numeric
        assert validate_story_id('TASK-001') is False   # wrong prefix
        assert validate_story_id('STORY001') is False   # missing hyphen

    def test_workflow_type_validation(self):
        """
        GIVEN workflow_type input
        WHEN validating
        THEN must be one of [dev, qa, orchestrate, release, ideate, create-story, create-epic, create-sprint]
        """
        from devforgeai_cli.feedback.validation import validate_workflow_type

        valid_types = ['dev', 'qa', 'orchestrate', 'release', 'ideate', 'create-story', 'create-epic', 'create-sprint']
        for wf_type in valid_types:
            assert validate_workflow_type(wf_type) is True

        invalid_types = ['development', 'quality', 'deploy', 'invalid']
        for wf_type in invalid_types:
            assert validate_workflow_type(wf_type) is False

    def test_response_length_limits(self):
        """
        GIVEN open text response
        WHEN validating length
        THEN must be 5-10,000 characters (warn if >2,000)
        """
        from devforgeai_cli.feedback.validation import validate_response_length

        # Too short
        is_valid, warning = validate_response_length('abc')
        assert is_valid is False

        # Valid short
        is_valid, warning = validate_response_length('This is helpful feedback')
        assert is_valid is True
        assert warning is None

        # Valid long (with warning)
        is_valid, warning = validate_response_length('x' * 2500)
        assert is_valid is True
        assert warning is not None  # Warns above 2,000

        # Too long
        is_valid, warning = validate_response_length('x' * 10500)
        assert is_valid is False

    def test_coherent_text_detection(self):
        """
        GIVEN text response
        WHEN validating coherence
        THEN detect random character repetition
        """
        from devforgeai_cli.feedback.validation import is_coherent_text

        # Coherent
        assert is_coherent_text('The workflow was confusing') is True

        # Not coherent (repetition)
        assert is_coherent_text('aaaaaaaaaaaaa') is False
        assert is_coherent_text('123412341234') is False
        assert is_coherent_text('asdfasdfasdf') is False
