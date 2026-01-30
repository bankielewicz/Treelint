"""
Data models for feedback system
"""

from dataclasses import dataclass, field
from typing import List, Optional, Any
from datetime import datetime


@dataclass
class Question:
    """Represents a single feedback question"""
    question_id: str
    question_text: str
    response_type: str  # 'rating', 'multiple_choice', 'open_text'
    scale: Optional[str] = None  # For rating questions (e.g., "1-5")
    options: Optional[List[str]] = None  # For multiple choice
    response: Optional[Any] = None
    skip: bool = False

    def __post_init__(self):
        """Validate question configuration"""
        if self.response_type == 'rating' and self.scale is None:
            raise ValueError(f"Rating question {self.question_id} must have scale")

        if self.response_type == 'multiple_choice' and (self.options is None or len(self.options) < 2):
            raise ValueError(f"Multiple choice question {self.question_id} must have at least 2 options")


@dataclass
class FeedbackSession:
    """Represents a complete feedback session"""
    feedback_id: str
    timestamp: str  # ISO 8601
    story_id: str
    epic_id: Optional[str] = None
    workflow_type: str = ''  # 'dev', 'qa', 'orchestrate', 'release', etc.
    success_status: str = ''  # 'success', 'failed', 'partial'
    questions: List[dict] = field(default_factory=list)
    metadata: dict = field(default_factory=dict)

    def to_dict(self) -> dict:
        """Convert to dictionary for JSON serialization"""
        return {
            'feedback_id': self.feedback_id,
            'timestamp': self.timestamp,
            'story_id': self.story_id,
            'epic_id': self.epic_id,
            'workflow_type': self.workflow_type,
            'success_status': self.success_status,
            'questions': self.questions,
            'metadata': self.metadata,
        }

    @classmethod
    def from_dict(cls, data: dict) -> 'FeedbackSession':
        """Create FeedbackSession from dictionary"""
        return cls(
            feedback_id=data['feedback_id'],
            timestamp=data['timestamp'],
            story_id=data['story_id'],
            epic_id=data.get('epic_id'),
            workflow_type=data.get('workflow_type', ''),
            success_status=data.get('success_status', ''),
            questions=data.get('questions', []),
            metadata=data.get('metadata', {}),
        )
