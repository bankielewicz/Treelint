"""
Retrospective conversation core module

Handles triggering retrospectives, capturing feedback, and managing feedback sessions.
"""

import json
import uuid
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import List, Dict, Any, Optional

from .models import Question, FeedbackSession
from .question_router import get_context_aware_questions


def trigger_retrospective(workflow_type: str, operation_result: Dict[str, Any]) -> List[Question]:
    """
    Trigger retrospective conversation after operation completion.

    Args:
        workflow_type: Type of workflow ('dev', 'qa', 'orchestrate', etc.)
        operation_result: Dict with keys 'status', 'story_id', optional 'failure_reason'

    Returns:
        List of Question objects (4-6 questions)
    """
    success_status = operation_result.get('status', 'success')

    # Get context-aware questions based on workflow and status
    questions = get_context_aware_questions(workflow_type, success_status)

    return questions


def capture_feedback(
    responses: List[Dict[str, Any]],
    story_id: str,
    workflow_type: str,
    success_status: str,
    feedback_dir: Optional[Path] = None,
    epic_id: Optional[str] = None,
    allow_skip: bool = True,
) -> Dict[str, Any]:
    """
    Capture and store feedback responses.

    Args:
        responses: List of dicts with 'question_id', 'response', 'skip'
        story_id: Story ID (e.g., 'STORY-001')
        workflow_type: Workflow type
        success_status: 'success', 'failed', or 'partial'
        feedback_dir: Directory to store feedback (default: devforgeai/feedback)
        epic_id: Optional epic ID
        allow_skip: Whether to allow full skip (all questions skipped)

    Returns:
        Dict with 'status' ('recorded' or 'skipped'), 'feedback_id', 'file_path', 'message'
    """
    # Check if all skipped
    all_skipped = all(r.get('skip', False) for r in responses)

    if all_skipped and allow_skip:
        return {
            'status': 'skipped',
            'message': 'No problem, thanks for using DevForgeAI!'
        }

    # Validate minimum responses (at least 2 of 5 substantive responses)
    substantive_responses = [
        r for r in responses
        if not r.get('skip', False) and r.get('response') and len(str(r['response'])) > 10
    ]

    if len(substantive_responses) < 2 and len(responses) >= 5:
        raise ValueError("At least 2 of 5 questions must have substantive responses (>10 chars)")

    # Generate feedback ID and timestamp
    feedback_id = f"fb-{uuid.uuid4()}"
    timestamp = datetime.now(timezone.utc).isoformat().replace('+00:00', 'Z')

    # Calculate metadata
    total_questions = len(responses)
    answered = sum(1 for r in responses if not r.get('skip', False))
    skipped = total_questions - answered

    metadata = {
        'duration_seconds': 0,  # Would be calculated in real implementation
        'total_questions': total_questions,
        'answered': answered,
        'skipped': skipped,
    }

    # Create feedback session
    feedback_session = FeedbackSession(
        feedback_id=feedback_id,
        timestamp=timestamp,
        story_id=story_id,
        epic_id=epic_id,
        workflow_type=workflow_type,
        success_status=success_status,
        questions=responses,
        metadata=metadata,
    )

    # Determine feedback directory
    if feedback_dir is None:
        feedback_dir = Path.cwd() / 'devforgeai' / 'feedback'

    # Create story-specific directory
    story_feedback_dir = feedback_dir / story_id
    story_feedback_dir.mkdir(parents=True, exist_ok=True)

    # Generate filename with timestamp
    filename = f"{timestamp[:10]}-retrospective.json"
    file_path = story_feedback_dir / filename

    # Write feedback to JSON file
    with open(file_path, 'w') as f:
        json.dump(feedback_session.to_dict(), f, indent=2)

    return {
        'status': 'recorded',
        'feedback_id': feedback_id,
        'file_path': str(file_path),
        'message': '✅ Feedback recorded'
    }


def is_skip_selected(user_response: str) -> bool:
    """
    Check if user selected to skip feedback.

    Args:
        user_response: User's response text

    Returns:
        True if user wants to skip, False otherwise
    """
    skip_phrases = [
        'skip',
        'no thanks',
        'not now',
        'later',
        'decline',
    ]

    response_lower = user_response.lower().strip()

    return any(phrase in response_lower for phrase in skip_phrases)


def save_in_progress_state(
    story_id: str,
    responses: List[Dict[str, Any]],
    workflow_type: str,
    feedback_dir: Path,
) -> Path:
    """
    Save in-progress feedback state (for network loss recovery).

    Args:
        story_id: Story ID
        responses: Partial responses completed so far
        workflow_type: Workflow type
        feedback_dir: Feedback directory

    Returns:
        Path to saved state file
    """
    story_feedback_dir = feedback_dir / story_id
    story_feedback_dir.mkdir(parents=True, exist_ok=True)

    state_data = {
        'story_id': story_id,
        'workflow_type': workflow_type,
        'responses': responses,
        'status': 'in_progress',
        'timestamp': datetime.now(timezone.utc).isoformat().replace('+00:00', 'Z'),
    }

    state_file = story_feedback_dir / f"{state_data['timestamp'][:19]}-in-progress.json"

    with open(state_file, 'w') as f:
        json.dump(state_data, f, indent=2)

    return state_file


def resume_feedback(story_id: str, feedback_dir: Path) -> Optional[Dict[str, Any]]:
    """
    Resume in-progress feedback session.

    Args:
        story_id: Story ID
        feedback_dir: Feedback directory

    Returns:
        In-progress state data if found, None otherwise
    """
    story_feedback_dir = feedback_dir / story_id

    if not story_feedback_dir.exists():
        return None

    # Find in-progress files
    in_progress_files = list(story_feedback_dir.glob('*-in-progress.json'))

    if not in_progress_files:
        return None

    # Load most recent
    most_recent = sorted(in_progress_files)[-1]

    with open(most_recent, 'r') as f:
        state_data = json.load(f)

    return state_data


def detect_rapid_sequence(last_feedback_time: datetime, threshold_seconds: int = 30) -> bool:
    """
    Detect if commands are being run in rapid sequence.

    Args:
        last_feedback_time: Timestamp of last feedback
        threshold_seconds: Threshold in seconds (default: 30)

    Returns:
        True if rapid sequence detected, False otherwise
    """
    time_since_last = datetime.now(timezone.utc) - last_feedback_time
    return time_since_last.total_seconds() < threshold_seconds
