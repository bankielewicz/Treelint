"""
Context-aware question routing

Routes questions based on workflow type and success/failure status.
"""

import yaml
from pathlib import Path
from typing import List, Dict, Any

from .models import Question


def load_question_bank(bank_path: Path = None) -> Dict[str, Any]:
    """
    Load question bank from YAML file.

    Args:
        bank_path: Path to questions.yaml file

    Returns:
        Dict with workflow questions
    """
    if bank_path is None:
        # Default path
        bank_path = Path(__file__).parent.parent.parent.parent.parent / '.devforgeai' / 'feedback' / 'questions.yaml'

    if not bank_path.exists():
        # Return minimal default question bank
        return _get_default_question_bank()

    with open(bank_path, 'r') as f:
        question_bank = yaml.safe_load(f)

    return question_bank


def get_context_aware_questions(workflow_type: str, success_status: str) -> List[Question]:
    """
    Get context-aware questions based on workflow and status.

    Args:
        workflow_type: 'dev', 'qa', 'orchestrate', 'release', etc.
        success_status: 'success', 'failed', 'partial'

    Returns:
        List of 4-6 Question objects
    """
    question_bank = load_question_bank()

    workflows = question_bank.get('workflows', {})
    workflow_config = workflows.get(workflow_type, {})

    # Determine question set based on success status
    if success_status == 'failed':
        question_list = workflow_config.get('failure_questions', [])
    else:
        question_list = workflow_config.get('success_questions', [])

    # Convert to Question objects
    questions = []
    for q_data in question_list[:6]:  # Limit to 6 questions
        question = Question(
            question_id=q_data['id'],
            question_text=q_data['text'],
            response_type=q_data['type'],
            scale=q_data.get('scale'),
            options=q_data.get('options'),
        )
        questions.append(question)

    return questions


def _get_default_question_bank() -> Dict[str, Any]:
    """
    Return default question bank (used when questions.yaml doesn't exist).

    Returns:
        Dict with minimal question bank
    """
    return {
        'workflows': {
            'dev': {
                'success_questions': [
                    {
                        'id': 'dev_success_01',
                        'text': 'How confident do you feel about the TDD workflow?',
                        'type': 'rating',
                        'scale': '1-5'
                    },
                    {
                        'id': 'dev_success_02',
                        'text': 'Which phase was most challenging?',
                        'type': 'multiple_choice',
                        'options': ['Red', 'Green', 'Refactor', 'Integration']
                    },
                    {
                        'id': 'dev_success_03',
                        'text': 'What could we improve about the development workflow?',
                        'type': 'open_text'
                    },
                    {
                        'id': 'dev_success_04',
                        'text': 'How well did the framework guide you?',
                        'type': 'rating',
                        'scale': '1-5'
                    },
                ],
                'failure_questions': [
                    {
                        'id': 'dev_failure_01',
                        'text': 'What blocked you from completing the story?',
                        'type': 'open_text'
                    },
                    {
                        'id': 'dev_failure_02',
                        'text': 'What would help you complete this in the future?',
                        'type': 'open_text'
                    },
                    {
                        'id': 'dev_failure_03',
                        'text': 'How clear were the error messages?',
                        'type': 'rating',
                        'scale': '1-5'
                    },
                ]
            },
            'qa': {
                'success_questions': [
                    {
                        'id': 'qa_success_01',
                        'text': 'How clear were the quality metrics?',
                        'type': 'rating',
                        'scale': '1-5'
                    },
                    {
                        'id': 'qa_success_02',
                        'text': 'What could improve the validation process?',
                        'type': 'open_text'
                    },
                    {
                        'id': 'qa_success_03',
                        'text': 'Were coverage thresholds reasonable?',
                        'type': 'multiple_choice',
                        'options': ['Too strict', 'Just right', 'Too lenient']
                    },
                ],
                'failure_questions': [
                    {
                        'id': 'qa_failure_01',
                        'text': 'What made the quality check fail?',
                        'type': 'open_text'
                    },
                    {
                        'id': 'qa_failure_02',
                        'text': 'How can we make quality requirements clearer?',
                        'type': 'open_text'
                    },
                    {
                        'id': 'qa_failure_03',
                        'text': 'How helpful were the failure messages?',
                        'type': 'rating',
                        'scale': '1-5'
                    },
                ]
            },
            'orchestrate': {
                'success_questions': [
                    {
                        'id': 'orchestrate_success_01',
                        'text': 'How smooth was the full workflow integration?',
                        'type': 'rating',
                        'scale': '1-5'
                    },
                    {
                        'id': 'orchestrate_success_02',
                        'text': 'Which phase had the most friction?',
                        'type': 'multiple_choice',
                        'options': ['Dev', 'QA', 'Release', 'None']
                    },
                    {
                        'id': 'orchestrate_success_03',
                        'text': 'What could improve the orchestration workflow?',
                        'type': 'open_text'
                    },
                ],
                'failure_questions': [
                    {
                        'id': 'orchestrate_failure_01',
                        'text': 'At which phase did the workflow fail?',
                        'type': 'multiple_choice',
                        'options': ['Dev', 'QA', 'Release']
                    },
                    {
                        'id': 'orchestrate_failure_02',
                        'text': 'What would have helped prevent this failure?',
                        'type': 'open_text'
                    },
                    {
                        'id': 'orchestrate_failure_03',
                        'text': 'How well did the framework communicate the issue?',
                        'type': 'rating',
                        'scale': '1-5'
                    },
                ]
            },
            'release': {
                'success_questions': [
                    {
                        'id': 'release_success_01',
                        'text': 'How confident are you in the deployment?',
                        'type': 'rating',
                        'scale': '1-5'
                    },
                    {
                        'id': 'release_success_02',
                        'text': 'What could improve the release process?',
                        'type': 'open_text'
                    },
                ],
                'failure_questions': [
                    {
                        'id': 'release_failure_01',
                        'text': 'What caused the release to fail?',
                        'type': 'open_text'
                    },
                    {
                        'id': 'release_failure_02',
                        'text': 'How can we make releases more reliable?',
                        'type': 'open_text'
                    },
                ]
            }
        }
    }
