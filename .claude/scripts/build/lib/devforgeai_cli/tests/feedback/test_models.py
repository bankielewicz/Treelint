"""Unit tests for models.py data models.

Tests Question and FeedbackSession dataclasses.
"""

import pytest
from devforgeai_cli.feedback.models import Question, FeedbackSession


class TestQuestion:
    """Test Question dataclass."""

    def test_question_creation_valid(self):
        """Test creating a valid question."""
        q = Question(
            question_id='q1',
            question_text='How confident are you?',
            response_type='rating',
            scale='1-5'
        )
        assert q.question_id == 'q1'
        assert q.scale == '1-5'

    def test_rating_question_without_scale_raises_error(self):
        """Test that rating question without scale raises ValueError (line 24)."""
        with pytest.raises(ValueError, match="must have scale"):
            Question(
                question_id='q1',
                question_text='Rate this',
                response_type='rating'
                # Missing scale - should raise error
            )

    def test_multiple_choice_without_options_raises_error(self):
        """Test that multiple choice without options raises ValueError (line 27)."""
        with pytest.raises(ValueError, match="must have at least 2 options"):
            Question(
                question_id='q2',
                question_text='Choose one',
                response_type='multiple_choice'
                # Missing options - should raise error
            )

    def test_multiple_choice_with_one_option_raises_error(self):
        """Test that multiple choice with only 1 option raises ValueError (line 27)."""
        with pytest.raises(ValueError, match="must have at least 2 options"):
            Question(
                question_id='q3',
                question_text='Choose one',
                response_type='multiple_choice',
                options=['Only one option']  # Need at least 2
            )


class TestFeedbackSession:
    """Test FeedbackSession dataclass."""

    def test_feedback_session_creation(self):
        """Test creating a FeedbackSession."""
        session = FeedbackSession(
            feedback_id='fb-1',
            timestamp='2025-01-01T00:00:00Z',
            story_id='STORY-001',
            epic_id='EPIC-001',
            workflow_type='dev',
            success_status='success'
        )
        assert session.feedback_id == 'fb-1'
        assert session.story_id == 'STORY-001'

    def test_feedback_session_to_dict(self):
        """Test converting FeedbackSession to dict."""
        session = FeedbackSession(
            feedback_id='fb-1',
            timestamp='2025-01-01T00:00:00Z',
            story_id='STORY-001'
        )
        data = session.to_dict()

        assert data['feedback_id'] == 'fb-1'
        assert data['story_id'] == 'STORY-001'
        assert 'timestamp' in data

    def test_feedback_session_from_dict(self):
        """Test creating FeedbackSession from dict (line 58)."""
        data = {
            'feedback_id': 'fb-2',
            'timestamp': '2025-01-01T12:00:00Z',
            'story_id': 'STORY-002',
            'epic_id': 'EPIC-001',
            'workflow_type': 'qa',
            'success_status': 'failed',
            'questions': [{'question_id': 'q1', 'response': 3}],
            'metadata': {'duration': 120}
        }

        session = FeedbackSession.from_dict(data)

        assert session.feedback_id == 'fb-2'
        assert session.story_id == 'STORY-002'
        assert session.epic_id == 'EPIC-001'
        assert session.workflow_type == 'qa'
        assert session.success_status == 'failed'
        assert len(session.questions) == 1
        assert session.metadata['duration'] == 120
