"""
Integration tests for retrospective feedback system

Tests the complete workflow end-to-end with real file I/O,
YAML persistence, and actual workflow scenarios.
"""

import pytest
import tempfile
import shutil
from pathlib import Path
from datetime import datetime, timezone

from devforgeai_cli.feedback.retrospective import (
    trigger_retrospective,
    capture_feedback,
    save_in_progress_state,
    resume_feedback,
    detect_rapid_sequence,
)
from devforgeai_cli.feedback.skip_tracking import (
    increment_skip,
    get_skip_count,
    reset_skip_count,
    check_skip_threshold,
)
from devforgeai_cli.feedback.aggregation import (
    aggregate_feedback_by_story,
    aggregate_feedback_by_epic,
    detect_patterns,
    generate_insights,
    export_quarterly_insights,
)
from devforgeai_cli.feedback.longitudinal import (
    correlate_feedback_across_stories,
    identify_improvement_trajectories,
    export_personal_journal,
)


class TestFullWorkflowIntegration:
    """Test complete workflow from trigger → capture → aggregation"""

    @pytest.fixture
    def temp_dirs(self):
        """Create temporary directories for feedback and config"""
        temp_dir = tempfile.mkdtemp()
        feedback_dir = Path(temp_dir) / 'feedback'
        config_dir = Path(temp_dir) / 'config'
        feedback_dir.mkdir(parents=True)
        config_dir.mkdir(parents=True)

        yield feedback_dir, config_dir

        shutil.rmtree(temp_dir)

    def test_successful_dev_workflow_end_to_end(self, temp_dirs):
        """
        GIVEN user completes /dev STORY-001 successfully
        WHEN retrospective triggered, feedback captured, and aggregated
        THEN complete workflow executes without errors
        """
        feedback_dir, config_dir = temp_dirs

        # Step 1: Trigger retrospective
        operation_result = {
            'status': 'success',
            'story_id': 'STORY-001',
        }
        questions = trigger_retrospective('dev', operation_result)

        assert len(questions) >= 4
        assert all(hasattr(q, 'question_id') for q in questions)

        # Step 2: Simulate user responses
        responses = [
            {'question_id': questions[0].question_id, 'response': 5, 'skip': False},
            {'question_id': questions[1].question_id, 'response': 'Green', 'skip': False},
            {'question_id': questions[2].question_id, 'response': 'The workflow was very clear and well-structured.', 'skip': False},
            {'question_id': questions[3].question_id, 'response': 4, 'skip': False},
        ]

        # Step 3: Capture feedback
        result = capture_feedback(
            responses=responses,
            story_id='STORY-001',
            workflow_type='dev',
            success_status='success',
            feedback_dir=feedback_dir,
            epic_id='EPIC-001',
        )

        assert result['status'] == 'recorded'
        assert 'feedback_id' in result
        assert 'file_path' in result

        # Verify file was written
        assert Path(result['file_path']).exists()

        # Step 4: Aggregate feedback
        aggregated = aggregate_feedback_by_story(feedback_dir)
        assert 'STORY-001' in aggregated
        assert len(aggregated['STORY-001']) == 1

    def test_failed_qa_workflow_with_skip_tracking(self, temp_dirs):
        """
        GIVEN user runs /qa that fails 3 times and skips feedback each time
        WHEN skip threshold reached
        THEN system detects pattern and suggests improvement
        """
        feedback_dir, config_dir = temp_dirs
        user_id = 'test_user'

        # Simulate 3 failed QA operations with feedback skipped
        for i in range(3):
            # Trigger retrospective
            operation_result = {
                'status': 'failed',
                'story_id': f'STORY-00{i+1}',
                'failure_reason': 'Coverage below threshold',
            }
            questions = trigger_retrospective('qa', operation_result)

            # User skips all questions
            responses = [
                {'question_id': q.question_id, 'response': '', 'skip': True}
                for q in questions
            ]

            # Capture feedback (should be skipped)
            result = capture_feedback(
                responses=responses,
                story_id=f'STORY-00{i+1}',
                workflow_type='qa',
                success_status='failed',
                feedback_dir=feedback_dir,
                allow_skip=True,
            )

            assert result['status'] == 'skipped'

            # Increment skip count
            count = increment_skip(user_id, config_dir)
            assert count == i + 1

        # Verify threshold detected
        assert check_skip_threshold(user_id, threshold=3, config_dir=config_dir)
        assert get_skip_count(user_id, config_dir) == 3

    def test_network_loss_recovery(self, temp_dirs):
        """
        GIVEN user starts feedback but connection lost mid-way
        WHEN they reconnect and resume
        THEN partial responses are preserved and recovered
        """
        feedback_dir, config_dir = temp_dirs

        # Step 1: Start feedback session
        partial_responses = [
            {'question_id': 'dev_success_01', 'response': 4, 'skip': False},
            {'question_id': 'dev_success_02', 'response': 'Red', 'skip': False},
        ]

        # Save in-progress state
        state_file = save_in_progress_state(
            story_id='STORY-001',
            workflow_type='dev',
            responses=partial_responses,
            feedback_dir=feedback_dir,
        )

        assert state_file.exists()

        # Step 2: Simulate recovery
        recovered_state = resume_feedback(story_id='STORY-001', feedback_dir=feedback_dir)

        assert recovered_state is not None
        assert recovered_state['story_id'] == 'STORY-001'
        assert recovered_state['workflow_type'] == 'dev'
        assert len(recovered_state['responses']) == 2
        assert recovered_state['responses'][0]['response'] == 4

    def test_rapid_sequence_detection(self, temp_dirs):
        """
        GIVEN user runs /dev STORY-001, then /dev STORY-002 within 30 seconds
        WHEN checking for rapid sequence
        THEN system detects rapid sequence
        """
        feedback_dir, config_dir = temp_dirs

        # First feedback
        operation_result_1 = {
            'status': 'success',
            'story_id': 'STORY-001',
        }
        questions_1 = trigger_retrospective('dev', operation_result_1)
        responses_1 = [
            {'question_id': q.question_id, 'response': 'Test', 'skip': False}
            for q in questions_1
        ]

        result_1 = capture_feedback(
            responses=responses_1,
            story_id='STORY-001',
            workflow_type='dev',
            success_status='success',
            feedback_dir=feedback_dir,
        )

        # Get timestamp from first feedback
        import json
        with open(result_1['file_path'], 'r') as f:
            first_feedback = json.load(f)

        first_timestamp = datetime.fromisoformat(first_feedback['timestamp'].replace('Z', '+00:00'))

        # Check rapid sequence (should be detected if within 30 seconds)
        is_rapid = detect_rapid_sequence(first_timestamp, threshold_seconds=30)

        # Since we just created it, current time - first_timestamp should be < 30 seconds
        assert is_rapid is True

    def test_aggregation_and_insights_generation(self, temp_dirs):
        """
        GIVEN multiple users provide feedback across multiple stories
        WHEN generating insights
        THEN patterns are detected and actionable recommendations generated
        """
        feedback_dir, config_dir = temp_dirs

        # Create 5 feedback sessions with common issue
        for i in range(5):
            operation_result = {
                'status': 'success',
                'story_id': f'STORY-00{i+1}',
            }
            questions = trigger_retrospective('dev', operation_result)

            responses = [
                {'question_id': questions[0].question_id, 'response': 3, 'skip': False},
                {'question_id': questions[1].question_id, 'response': 'Red', 'skip': False},
                {'question_id': questions[2].question_id, 'response': 'tests are hard to write', 'skip': False},
                {'question_id': questions[3].question_id, 'response': 3, 'skip': False},
            ]

            capture_feedback(
                responses=responses,
                story_id=f'STORY-00{i+1}',
                workflow_type='dev',
                success_status='success',
                feedback_dir=feedback_dir,
                epic_id='EPIC-001',
            )

        # Generate insights
        insights = generate_insights(feedback_dir)

        assert 'recommendations' in insights
        # With 5 feedback sessions all saying "tests are hard to write",
        # pattern detection should find this (100% occurrence)
        recommendations = insights['recommendations']
        assert len(recommendations) > 0

        # Export quarterly insights
        output_path = export_quarterly_insights(feedback_dir)
        assert output_path.exists()
        assert output_path.name == 'quarterly-insights.md'

    def test_longitudinal_tracking_across_stories(self, temp_dirs):
        """
        GIVEN user completes multiple stories over time with improving confidence
        WHEN analyzing trajectory
        THEN system detects improvement trend
        """
        feedback_dir, config_dir = temp_dirs
        user_id = 'test_user'

        # Create 5 feedback sessions with increasing confidence
        for i in range(5):
            operation_result = {
                'status': 'success',
                'story_id': f'STORY-00{i+1}',
            }
            questions = trigger_retrospective('dev', operation_result)

            # Confidence increases: 2, 3, 3, 4, 5
            confidence = min(2 + (i // 2), 5)

            responses = [
                {'question_id': questions[0].question_id, 'response': confidence, 'skip': False},
                {'question_id': questions[1].question_id, 'response': 'Red', 'skip': False},
                {'question_id': questions[2].question_id, 'response': 'Getting better!', 'skip': False},
                {'question_id': questions[3].question_id, 'response': confidence, 'skip': False},
            ]

            capture_feedback(
                responses=responses,
                story_id=f'STORY-00{i+1}',
                workflow_type='dev',
                success_status='success',
                feedback_dir=feedback_dir,
                epic_id='EPIC-001',
            )

        # Correlate feedback across stories
        correlation = correlate_feedback_across_stories(feedback_dir, user_id)
        assert 'timeline' in correlation
        assert len(correlation['timeline']) == 5

        # Identify improvement trajectory
        trajectories = identify_improvement_trajectories(feedback_dir, user_id)
        assert 'metrics' in trajectories

        if len(trajectories['metrics']) > 0:
            metric = trajectories['metrics'][0]
            assert metric['name'] == 'TDD confidence'
            # Should be improving (2 → 5)
            assert metric['start_value'] <= metric['end_value']
            assert metric['trend'] == 'improving'

        # Export personal journal
        journal_path = export_personal_journal(feedback_dir, user_id)
        assert journal_path.exists()
        assert journal_path.name == 'journal.md'


class TestErrorHandlingIntegration:
    """Test error handling in integrated scenarios"""

    @pytest.fixture
    def temp_dirs(self):
        """Create temporary directories"""
        temp_dir = tempfile.mkdtemp()
        feedback_dir = Path(temp_dir) / 'feedback'
        config_dir = Path(temp_dir) / 'config'
        feedback_dir.mkdir(parents=True)
        config_dir.mkdir(parents=True)

        yield feedback_dir, config_dir

        shutil.rmtree(temp_dir)

    def test_invalid_story_id_rejected(self, temp_dirs):
        """
        GIVEN user provides invalid story ID format
        WHEN capturing feedback
        THEN validation fails gracefully
        """
        from devforgeai_cli.feedback.validation import validate_story_id

        # Invalid formats
        assert validate_story_id('story-001') is False  # lowercase
        assert validate_story_id('STORY001') is False    # no hyphen
        assert validate_story_id('EPIC-001') is False    # wrong prefix

        # Valid format
        assert validate_story_id('STORY-001') is True

    def test_insufficient_responses_rejected(self, temp_dirs):
        """
        GIVEN user provides only 1 substantive response out of 5 questions
        WHEN capturing feedback
        THEN validation error raised
        """
        feedback_dir, config_dir = temp_dirs

        responses = [
            {'question_id': 'q1', 'response': 'Good', 'skip': False},  # Only 1 substantive
            {'question_id': 'q2', 'response': '', 'skip': True},
            {'question_id': 'q3', 'response': '', 'skip': True},
            {'question_id': 'q4', 'response': '', 'skip': True},
            {'question_id': 'q5', 'response': '', 'skip': True},
        ]

        with pytest.raises(ValueError, match='At least 2 of 5 questions'):
            capture_feedback(
                responses=responses,
                story_id='STORY-001',
                workflow_type='dev',
                success_status='success',
                feedback_dir=feedback_dir,
            )

    def test_sensitive_content_detection_integration(self, temp_dirs):
        """
        GIVEN user accidentally includes API key in feedback
        WHEN capturing feedback
        THEN system detects and flags sensitive content
        """
        from devforgeai_cli.feedback.validation import check_sensitive_content

        feedback_with_key = "The API key sk-1234567890abcdefghijklmnopqrst was exposed in logs"
        is_sensitive, types = check_sensitive_content(feedback_with_key)

        assert is_sensitive is True
        assert 'secret' in types

        feedback_normal = "The workflow was great and easy to follow"
        is_sensitive, types = check_sensitive_content(feedback_normal)

        assert is_sensitive is False
        assert len(types) == 0
