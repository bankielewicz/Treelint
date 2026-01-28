"""
Unit tests for context-aware question routing (AC5)

Tests cover:
- AC5: Context-aware question routing by workflow type
- Question adaptation based on success/failure
- Cultural appropriateness validation
"""

import pytest
from devforgeai_cli.feedback.question_router import (
    get_context_aware_questions,
    load_question_bank,
)
from devforgeai_cli.feedback.models import Question


class TestContextAwareRouting:
    """AC5: Context-Aware Question Routing"""

    def test_get_questions_for_dev_success(self):
        """
        GIVEN a successful /dev workflow completion
        WHEN get_context_aware_questions is called with workflow_type='dev' and success_status='success'
        THEN it returns dev-specific success questions
        """
        # Arrange
        workflow_type = 'dev'
        success_status = 'success'

        # Act
        questions = get_context_aware_questions(workflow_type, success_status)

        # Assert
        assert questions is not None
        assert len(questions) >= 4
        assert all(isinstance(q, Question) for q in questions)

        # Check for dev-specific content
        question_texts = [q.question_text.lower() for q in questions]
        assert any('tdd' in text or 'test' in text or 'development' in text for text in question_texts)

    def test_get_questions_for_qa_success(self):
        """
        GIVEN a successful /qa workflow completion
        WHEN get_context_aware_questions is called with workflow_type='qa'
        THEN it returns qa-specific questions (coverage, validation)
        """
        # Arrange
        workflow_type = 'qa'
        success_status = 'success'

        # Act
        questions = get_context_aware_questions(workflow_type, success_status)

        # Assert
        assert questions is not None
        question_texts = [q.question_text.lower() for q in questions]
        assert any('coverage' in text or 'quality' in text or 'validation' in text for text in question_texts)

    def test_get_questions_for_orchestrate_success(self):
        """
        GIVEN a successful /orchestrate workflow completion
        WHEN get_context_aware_questions is called
        THEN it returns orchestration-specific questions
        """
        # Arrange
        workflow_type = 'orchestrate'
        success_status = 'success'

        # Act
        questions = get_context_aware_questions(workflow_type, success_status)

        # Assert
        assert questions is not None
        question_texts = [q.question_text.lower() for q in questions]
        assert any('workflow' in text or 'phases' in text or 'integration' in text for text in question_texts)

    def test_get_questions_for_failure_differ_from_success(self):
        """
        GIVEN failure vs success status
        WHEN get_context_aware_questions is called
        THEN failure questions differ from success questions
        """
        # Arrange
        workflow_type = 'dev'

        # Act
        success_questions = get_context_aware_questions(workflow_type, 'success')
        failure_questions = get_context_aware_questions(workflow_type, 'failed')

        # Assert
        success_ids = [q.question_id for q in success_questions]
        failure_ids = [q.question_id for q in failure_questions]

        # At least some questions should be different
        assert success_ids != failure_ids
        assert len(set(success_ids) & set(failure_ids)) < len(success_ids)  # Some overlap OK, but not all

    def test_questions_have_appropriate_response_types(self):
        """
        GIVEN context-aware questions
        WHEN examining response types
        THEN questions use appropriate types (rating, multiple_choice, open_text)
        """
        # Arrange
        workflow_type = 'dev'
        success_status = 'success'

        # Act
        questions = get_context_aware_questions(workflow_type, success_status)

        # Assert
        response_types = [q.response_type for q in questions]
        valid_types = ['rating', 'multiple_choice', 'open_text']

        assert all(rt in valid_types for rt in response_types)
        # Should have a mix of types
        assert len(set(response_types)) >= 2


class TestCulturalAppropriateness:
    """AC5: Cultural appropriateness validation"""

    def test_questions_never_blame_user(self):
        """
        GIVEN all workflow questions
        WHEN examining question text
        THEN questions never blame user (no "You failed", "You missed", etc.)
        """
        # Arrange
        workflows = ['dev', 'qa', 'orchestrate', 'release']
        statuses = ['success', 'failed', 'partial']

        blame_phrases = ['you failed', 'you missed', 'your fault', 'your mistake', 'you should have']

        # Act & Assert
        for workflow in workflows:
            for status in statuses:
                questions = get_context_aware_questions(workflow, status)
                for question in questions:
                    text_lower = question.question_text.lower()
                    for blame_phrase in blame_phrases:
                        assert blame_phrase not in text_lower, \
                            f"Question '{question.question_text}' contains blame phrase '{blame_phrase}'"

    def test_questions_focus_on_framework_improvement(self):
        """
        GIVEN failure questions
        WHEN examining question text
        THEN questions frame issues as framework improvements ("How can we make this clearer?")
        """
        # Arrange
        workflow_type = 'dev'
        success_status = 'failed'

        # Act
        questions = get_context_aware_questions(workflow_type, success_status)

        # Assert
        improvement_indicators = ['how can we', 'what would help', 'what could', 'suggestions', 'improvements']
        question_texts = [q.question_text.lower() for q in questions]

        # At least one question should focus on improvements
        assert any(
            any(indicator in text for indicator in improvement_indicators)
            for text in question_texts
        ), "No questions focus on framework improvements"

    def test_questions_avoid_jargon(self):
        """
        GIVEN all questions
        WHEN examining question text
        THEN questions use clear English (not overly technical jargon)
        """
        # Arrange
        workflow_type = 'dev'
        success_status = 'success'

        # Act
        questions = get_context_aware_questions(workflow_type, success_status)

        # Assert - Questions should be understandable
        # Check that question text is reasonably short (not overly verbose)
        for question in questions:
            assert len(question.question_text) < 200, \
                f"Question too long (may be too complex): '{question.question_text}'"

            # Should not use ALL CAPS for entire question (aggressive tone)
            assert not question.question_text.isupper(), \
                f"Question uses ALL CAPS: '{question.question_text}'"


class TestQuestionBankLoading:
    """Test question bank YAML loading"""

    def test_load_question_bank_returns_structured_data(self):
        """
        GIVEN question bank YAML file
        WHEN load_question_bank is called
        THEN it returns structured dictionary with workflows
        """
        # Act
        question_bank = load_question_bank()

        # Assert
        assert question_bank is not None
        assert 'workflows' in question_bank
        assert isinstance(question_bank['workflows'], dict)

        # Check required workflows exist
        required_workflows = ['dev', 'qa', 'orchestrate', 'release']
        for workflow in required_workflows:
            assert workflow in question_bank['workflows']

    def test_question_bank_has_success_and_failure_questions(self):
        """
        GIVEN question bank
        WHEN examining workflow questions
        THEN each workflow has both success_questions and failure_questions
        """
        # Act
        question_bank = load_question_bank()

        # Assert
        workflows = question_bank['workflows']
        for workflow_name, workflow_config in workflows.items():
            assert 'success_questions' in workflow_config, \
                f"Workflow '{workflow_name}' missing success_questions"
            assert 'failure_questions' in workflow_config, \
                f"Workflow '{workflow_name}' missing failure_questions"
            assert len(workflow_config['success_questions']) >= 3, \
                f"Workflow '{workflow_name}' needs at least 3 success questions"
            assert len(workflow_config['failure_questions']) >= 3, \
                f"Workflow '{workflow_name}' needs at least 3 failure questions"

    def test_question_bank_questions_have_required_fields(self):
        """
        GIVEN question bank
        WHEN examining individual questions
        THEN each question has id, text, type (and scale/options if applicable)
        """
        # Act
        question_bank = load_question_bank()

        # Assert
        workflows = question_bank['workflows']
        for workflow_name, workflow_config in workflows.items():
            for question_type in ['success_questions', 'failure_questions']:
                for question in workflow_config[question_type]:
                    assert 'id' in question, f"Question missing 'id' in {workflow_name}/{question_type}"
                    assert 'text' in question, f"Question missing 'text' in {workflow_name}/{question_type}"
                    assert 'type' in question, f"Question missing 'type' in {workflow_name}/{question_type}"

                    # If rating, must have scale
                    if question['type'] == 'rating':
                        assert 'scale' in question, f"Rating question missing 'scale' in {workflow_name}"

                    # If multiple_choice, must have options
                    if question['type'] == 'multiple_choice':
                        assert 'options' in question, f"Multiple choice question missing 'options' in {workflow_name}"
                        assert len(question['options']) >= 2, f"Multiple choice needs at least 2 options"
