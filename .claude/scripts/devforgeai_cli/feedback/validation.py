"""
Feedback validation utilities
"""

import re
from typing import Tuple, List, Optional


def validate_response_length(response: str, min_length: int = 5, max_length: int = 10000, warn_threshold: int = 2000) -> Tuple[bool, Optional[str]]:
    """
    Validate response length.

    Args:
        response: Response text
        min_length: Minimum length (default: 5)
        max_length: Maximum length (default: 10000)
        warn_threshold: Threshold for warning (default: 2000)

    Returns:
        Tuple of (is_valid, warning_message)
    """
    length = len(response)

    if length < min_length:
        return False, f"Response too short (minimum {min_length} characters)"

    if length > max_length:
        return False, f"Response too long (maximum {max_length} characters)"

    if length > warn_threshold:
        return True, f"Response is long ({length} chars). Consider being more concise."

    return True, None


def detect_spam(text: str) -> bool:
    """
    Detect spam or noise in feedback.

    Args:
        text: Text to check

    Returns:
        True if spam detected, False otherwise
    """
    if len(text) == 0:
        return True

    # Check for character repetition (e.g., "aaaaaaa")
    if len(set(text)) <= 3 and len(text) > 10:
        return True

    # Check for pattern repetition (e.g., "123412341234")
    if len(text) > 20:
        for pattern_len in range(3, 10):
            pattern = text[:pattern_len]
            if text == pattern * (len(text) // pattern_len):
                return True

    # Check for low word count
    words = text.split()
    if len(words) < 5 and len(text) > 50:
        return True

    # Check for non-alphanumeric ratio
    alphanumeric_count = sum(c.isalnum() or c.isspace() for c in text)
    if len(text) > 20 and alphanumeric_count / len(text) < 0.1:
        return True

    return False


def is_coherent_text(text: str) -> bool:
    """
    Check if text is coherent (not random repetition).

    Args:
        text: Text to check

    Returns:
        True if coherent, False otherwise
    """
    if len(text) < 5:
        return True

    # Check for single character repetition
    if len(set(text)) == 1:
        return False

    # Check for pattern repetition (check all possible pattern lengths)
    for pattern_len in range(2, min(len(text) // 3 + 1, 10)):
        pattern = text[:pattern_len]
        repetitions = len(text) // pattern_len
        # Check if text consists of repeated pattern
        if repetitions >= 3:
            reconstructed = pattern * repetitions
            # Also check for partial match at end
            if text == reconstructed or text == reconstructed + pattern[:len(text) - len(reconstructed)]:
                return False

    return True


def check_sensitive_content(feedback: str) -> Tuple[bool, List[str]]:
    """
    Check if feedback contains sensitive content.

    Args:
        feedback: Feedback text

    Returns:
        Tuple of (is_sensitive, detected_types)
    """
    detected_types = []
    text_lower = feedback.lower()

    # Check for API keys or secrets (specific patterns first)
    if re.search(r'sk-[a-zA-Z0-9]{20,}', feedback):
        detected_types.append('secret')
    elif re.search(r'(api[_\s-]?key)\s*[:=]?\s*[\w-]+', text_lower):
        detected_types.append('api_key')

    # Check for data loss concerns
    if any(phrase in text_lower for phrase in ['data loss', 'deleted', 'lost data', 'production database']):
        detected_types.append('data_loss')

    # Check for critical issues (but not if it's just the word "exposed" from "api key exposed")
    if any(phrase in text_lower for phrase in ['security breach', 'vulnerability']):
        if 'api_key' not in detected_types and 'secret' not in detected_types:
            detected_types.append('critical_issue')

    return len(detected_types) > 0, detected_types


def validate_story_id(story_id: str) -> bool:
    """
    Validate story ID format.

    Args:
        story_id: Story ID to validate

    Returns:
        True if valid, False otherwise
    """
    pattern = r'^STORY-\d+$'
    return bool(re.match(pattern, story_id))


def validate_workflow_type(workflow_type: str) -> bool:
    """
    Validate workflow type.

    Args:
        workflow_type: Workflow type to validate

    Returns:
        True if valid, False otherwise
    """
    valid_types = [
        'dev', 'qa', 'orchestrate', 'release', 'ideate',
        'create-story', 'create-epic', 'create-sprint'
    ]
    return workflow_type in valid_types
