"""
Unit tests for retrospective conversation functionality (AC1, AC2, AC3)

Tests cover:
- AC1: Retrospective triggered at operation completion
- AC2: Failed command with root cause feedback
- AC3: User opts out of feedback
"""

import pytest
import json
import uuid
from datetime import datetime
from pathlib import Path
import tempfile
import shutil

from devforgeai_cli.feedback.retrospective import (
    trigger_retrospective,
    capture_feedback,
    is_skip_selected,
)
from devforgeai_cli.feedback.models import FeedbackSession, Question


class TestRetrospectiveTrigger:
    """AC1: Retrospective Triggered at Operation Completion"""

    def test_trigger_retrospective_success_returns_questions(self):
        """
        GIVEN a successful /dev command completion
        WHEN trigger_retrospective is called with workflow_type='dev' and success_status='success'
        THEN it returns 4-6 targeted questions
        """
        # Arrange
        workflow_type = 'dev'
        operation_result = {'status': 'success', 'story_id': 'STORY-001'}

        # Act
        questions = trigger_retrospective(workflow_type, operation_result)

        # Assert
        assert questions is not None
        assert 4 <= len(questions) <= 6
        assert all(isinstance(q, Question) for q in questions)
        assert all(hasattr(q, 'question_id') for q in questions)
        assert all(hasattr(q, 'question_text') for q in questions)
        assert all(hasattr(q, 'response_type') for q in questions)

    def test_trigger_retrospective_failure_returns_failure_questions(self):
        """
        GIVEN a failed /qa command (AC2)
        WHEN trigger_retrospective is called with success_status='failed'
        THEN it returns failure-specific questions
        """
        # Arrange
        workflow_type = 'qa'
        operation_result = {'status': 'failed', 'story_id': 'STORY-002', 'failure_reason': 'Coverage below threshold'}

        # Act
        questions = trigger_retrospective(workflow_type, operation_result)

        # Assert
        assert questions is not None
        assert len(questions) >= 3  # At least 3 failure-specific questions
        assert any('block' in q.question_text.lower() or 'fail' in q.question_text.lower() for q in questions)
        assert any('help' in q.question_text.lower() or 'improve' in q.question_text.lower() for q in questions)


class TestFeedbackCapture:
    """AC1: Feedback capture and storage"""

    @pytest.fixture
    def temp_feedback_dir(self):
        """Create temporary feedback directory for tests"""
        temp_dir = tempfile.mkdtemp()
        yield Path(temp_dir)
        shutil.rmtree(temp_dir)

    def test_capture_feedback_stores_json_correctly(self, temp_feedback_dir):
        """
        GIVEN user responses to retrospective questions
        WHEN capture_feedback is called
        THEN feedback is stored in devforgeai/feedback/{STORY-ID}/{timestamp}-retrospective.json
        AND returns confirmation message
        """
        # Arrange
        story_id = 'STORY-001'
        responses = [
            {'question_id': 'dev_success_01', 'response': 4, 'skip': False},
            {'question_id': 'dev_success_02', 'response': 'Green', 'skip': False},
        ]

        # Act
        result = capture_feedback(
            responses=responses,
            story_id=story_id,
            workflow_type='dev',
            success_status='success',
            feedback_dir=temp_feedback_dir
        )

        # Assert
        assert result['status'] == 'recorded'
        assert 'feedback_id' in result
        assert 'file_path' in result

        # Verify file was created
        story_feedback_dir = temp_feedback_dir / story_id
        assert story_feedback_dir.exists()

        json_files = list(story_feedback_dir.glob('*-retrospective.json'))
        assert len(json_files) == 1

        # Verify JSON structure
        with open(json_files[0], 'r') as f:
            feedback_data = json.load(f)

        assert feedback_data['story_id'] == story_id
        assert feedback_data['workflow_type'] == 'dev'
        assert feedback_data['success_status'] == 'success'
        assert 'feedback_id' in feedback_data
        assert 'timestamp' in feedback_data
        assert len(feedback_data['questions']) == 2
        assert feedback_data['metadata']['total_questions'] == 2
        assert feedback_data['metadata']['answered'] == 2
        assert feedback_data['metadata']['skipped'] == 0

    def test_capture_feedback_validates_required_fields(self, temp_feedback_dir):
        """
        GIVEN incomplete feedback (AC2: at least 2 of 5 required)
        WHEN capture_feedback is called with only 1 substantive response
        THEN it raises ValueError
        """
        # Arrange - Only 1 substantive response, not enough
        story_id = 'STORY-002'
        responses = [
            {'question_id': 'dev_failure_01', 'response': 'Git issue', 'skip': False},
            {'question_id': 'dev_failure_02', 'response': '', 'skip': True},
            {'question_id': 'dev_failure_03', 'response': '', 'skip': True},
            {'question_id': 'dev_failure_04', 'response': '', 'skip': True},
            {'question_id': 'dev_failure_05', 'response': '', 'skip': True},
        ]

        # Act & Assert
        with pytest.raises(ValueError, match="At least 2 of 5 questions must have substantive responses"):
            capture_feedback(
                responses=responses,
                story_id=story_id,
                workflow_type='dev',
                success_status='failed',
                feedback_dir=temp_feedback_dir
            )

    def test_capture_feedback_accepts_valid_partial_completion(self, temp_feedback_dir):
        """
        GIVEN partial feedback with 2 substantive responses
        WHEN capture_feedback is called
        THEN it accepts and stores the feedback
        """
        # Arrange - 2 substantive responses (minimum required)
        story_id = 'STORY-003'
        responses = [
            {'question_id': 'qa_failure_01', 'response': 'Coverage metrics unclear', 'skip': False},
            {'question_id': 'qa_failure_02', 'response': 'Better documentation would help', 'skip': False},
            {'question_id': 'qa_failure_03', 'response': '', 'skip': True},
        ]

        # Act
        result = capture_feedback(
            responses=responses,
            story_id=story_id,
            workflow_type='qa',
            success_status='failed',
            feedback_dir=temp_feedback_dir
        )

        # Assert
        assert result['status'] == 'recorded'
        assert 'feedback_id' in result


class TestUserOptOut:
    """AC3: User Opts Out of Feedback"""

    def test_is_skip_selected_returns_true_for_skip_option(self):
        """
        GIVEN user selects "Skip feedback" option
        WHEN is_skip_selected is called
        THEN it returns True
        """
        # Arrange
        user_response = 'Skip feedback'

        # Act
        result = is_skip_selected(user_response)

        # Assert
        assert result is True

    def test_is_skip_selected_returns_true_for_decline_variations(self):
        """
        GIVEN user declines feedback with various phrases
        WHEN is_skip_selected is called
        THEN it returns True
        """
        # Arrange
        decline_phrases = [
            'Skip',
            'No thanks',
            'Not now',
            'Later',
            'Decline',
        ]

        # Act & Assert
        for phrase in decline_phrases:
            assert is_skip_selected(phrase) is True

    def test_is_skip_selected_returns_false_for_normal_responses(self):
        """
        GIVEN user provides normal feedback response
        WHEN is_skip_selected is called
        THEN it returns False
        """
        # Arrange
        normal_responses = [
            'The TDD workflow was helpful',
            '4',
            'Green phase',
        ]

        # Act & Assert
        for response in normal_responses:
            assert is_skip_selected(response) is False

    @pytest.fixture
    def temp_feedback_dir(self):
        """Create temporary feedback directory for tests"""
        temp_dir = tempfile.mkdtemp()
        yield Path(temp_dir)
        shutil.rmtree(temp_dir)

    def test_capture_feedback_respects_skip_without_storing(self, temp_feedback_dir):
        """
        GIVEN user skips all questions
        WHEN capture_feedback is called
        THEN it does NOT create feedback file
        AND returns skip status
        """
        # Arrange
        story_id = 'STORY-004'
        responses = [
            {'question_id': 'dev_success_01', 'response': '', 'skip': True},
            {'question_id': 'dev_success_02', 'response': '', 'skip': True},
            {'question_id': 'dev_success_03', 'response': '', 'skip': True},
            {'question_id': 'dev_success_04', 'response': '', 'skip': True},
        ]

        # Act
        result = capture_feedback(
            responses=responses,
            story_id=story_id,
            workflow_type='dev',
            success_status='success',
            feedback_dir=temp_feedback_dir,
            allow_skip=True
        )

        # Assert
        assert result['status'] == 'skipped'
        assert 'message' in result
        assert 'thanks' in result['message'].lower()

        # Verify no file was created
        story_feedback_dir = temp_feedback_dir / story_id
        if story_feedback_dir.exists():
            json_files = list(story_feedback_dir.glob('*-retrospective.json'))
            assert len(json_files) == 0


class TestTimestampAndMetadata:
    """Test ISO 8601 timestamps and metadata generation"""

    @pytest.fixture
    def temp_feedback_dir(self):
        temp_dir = tempfile.mkdtemp()
        yield Path(temp_dir)
        shutil.rmtree(temp_dir)

    def test_capture_feedback_generates_iso8601_timestamp(self, temp_feedback_dir):
        """
        GIVEN feedback capture
        WHEN feedback is stored
        THEN timestamp is in ISO 8601 format
        """
        # Arrange
        story_id = 'STORY-005'
        responses = [
            {'question_id': 'dev_success_01', 'response': 4, 'skip': False},
            {'question_id': 'dev_success_02', 'response': 'Refactor', 'skip': False},
        ]

        # Act
        result = capture_feedback(
            responses=responses,
            story_id=story_id,
            workflow_type='dev',
            success_status='success',
            feedback_dir=temp_feedback_dir
        )

        # Assert
        story_feedback_dir = temp_feedback_dir / story_id
        json_files = list(story_feedback_dir.glob('*-retrospective.json'))

        with open(json_files[0], 'r') as f:
            feedback_data = json.load(f)

        # Verify ISO 8601 format
        timestamp = feedback_data['timestamp']
        try:
            datetime.fromisoformat(timestamp.replace('Z', '+00:00'))
        except ValueError:
            pytest.fail(f"Timestamp '{timestamp}' is not valid ISO 8601")

        # Verify metadata completeness
        metadata = feedback_data['metadata']
        assert 'duration_seconds' in metadata
        assert 'total_questions' in metadata
        assert 'answered' in metadata
        assert 'skipped' in metadata
        assert metadata['total_questions'] == metadata['answered'] + metadata['skipped']
