"""
Longitudinal feedback tracking (AC6)
"""

import json
from pathlib import Path
from typing import Dict, List, Any
from datetime import datetime


def correlate_feedback_across_stories(feedback_dir: Path, user_id: str) -> Dict[str, Any]:
    """
    Correlate feedback across stories for a user.

    Returns:
        Dict with 'timeline' list showing progression
    """
    all_feedback = []

    for story_dir in feedback_dir.iterdir():
        if not story_dir.is_dir():
            continue

        for feedback_file in story_dir.glob('*-retrospective.json'):
            with open(feedback_file, 'r') as f:
                feedback_data = json.load(f)
                all_feedback.append(feedback_data)

    # Sort by timestamp
    all_feedback.sort(key=lambda x: x['timestamp'])

    timeline = []
    for fb in all_feedback:
        timeline.append({
            'timestamp': fb['timestamp'],
            'story_id': fb['story_id'],
            'workflow_type': fb['workflow_type'],
            'success_status': fb['success_status'],
        })

    return {'timeline': timeline}


def identify_improvement_trajectories(feedback_dir: Path, user_id: str) -> Dict[str, Any]:
    """
    Identify improvement trajectories over time.

    Returns:
        Dict with 'metrics' list showing trends
    """
    correlation = correlate_feedback_across_stories(feedback_dir, user_id)

    # Extract confidence ratings over time (dev_success_01)
    confidence_values = []

    for story_dir in feedback_dir.iterdir():
        if not story_dir.is_dir():
            continue

        for feedback_file in sorted(story_dir.glob('*-retrospective.json')):
            with open(feedback_file, 'r') as f:
                feedback_data = json.load(f)

                for question in feedback_data.get('questions', []):
                    if question.get('question_id', '').endswith('_01') and isinstance(question.get('response'), int):
                        confidence_values.append(question['response'])

    if len(confidence_values) < 2:
        return {'metrics': []}

    # Determine trend
    trend = 'improving' if confidence_values[-1] >= confidence_values[0] else 'declining'

    metrics = [{
        'name': 'TDD confidence',
        'trend': trend,
        'start_value': confidence_values[0],
        'end_value': confidence_values[-1],
        'data_points': len(confidence_values)
    }]

    return {'metrics': metrics}


def export_personal_journal(feedback_dir: Path, user_id: str) -> Path:
    """
    Export personal retrospective journal.

    Returns:
        Path to journal.md
    """
    correlation = correlate_feedback_across_stories(feedback_dir, user_id)
    trajectories = identify_improvement_trajectories(feedback_dir, user_id)

    user_dir = feedback_dir / user_id
    user_dir.mkdir(parents=True, exist_ok=True)

    journal_path = user_dir / 'journal.md'

    content = "# Retrospective Journal\n\n"
    content += f"**User:** {user_id}\n\n"
    content += "## Timeline\n\n"

    for entry in correlation['timeline']:
        content += f"- **{entry['story_id']}** ({entry['timestamp'][:10]}) - {entry['workflow_type']} - {entry['success_status']}\n"

    content += "\n## Improvement Trajectory\n\n"

    for metric in trajectories.get('metrics', []):
        content += f"- **{metric['name']}**: {metric['trend']} (from {metric['start_value']} to {metric['end_value']})\n"

    with open(journal_path, 'w') as f:
        f.write(content)

    return journal_path
