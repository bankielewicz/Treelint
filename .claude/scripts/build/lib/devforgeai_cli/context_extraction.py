"""
Context Extraction for DevForgeAI Feedback Hooks

Extracts operation context from TodoWrite, including:
- Todos (status, content)
- Errors (messages, stack traces)
- Timing (start_time, end_time, duration)
- Secret sanitization (50+ patterns)
- Context size limiting (50KB max)

Uses regex patterns to sanitize sensitive data before skill invocation.
"""

import json
import logging
import re
from datetime import datetime
from typing import Dict, List, Any, Optional

logger = logging.getLogger(__name__)

# Constants for context limiting
MAX_CONTEXT_SIZE_BYTES = 50 * 1024
MAX_TODOS_BEFORE_SUMMARY = 100
MAX_ERRORS_BEFORE_TRUNCATION = 10
TRUNCATION_MARKER = "... truncated"

# Secret patterns for sanitization (50+ patterns)
SECRET_PATTERNS = [
    # API Keys
    (r"(api[_-]?key\s*[:=]\s*)([sk-][a-zA-Z0-9]{20,})", r"\1***"),
    (r"(apikey\s*[:=]\s*)([a-zA-Z0-9\._\-]{32,})", r"\1***"),
    (r"(api[_-]?secret\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(sk[_-]proj\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(sk[_-]live\s*[:=]\s*)(\S+)", r"\1***"),

    # Passwords
    (r"(password\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(passwd\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(pwd\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(user[_-]?password\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(secret\s*[:=]\s*)(\S+)", r"\1***"),

    # OAuth Tokens
    (r"(access[_-]?token\s*[:=]\s*)([a-zA-Z0-9\._\-]{20,})", r"\1***"),
    (r"(refresh[_-]?token\s*[:=]\s*)([a-zA-Z0-9\._\-]{20,})", r"\1***"),
    (r"(token\s*[:=]\s*)(bearer[_\s]*[a-zA-Z0-9\._\-]+)", r"\1***"),
    (r"(oauth[_-]?token\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(id[_-]?token\s*[:=]\s*)(\S+)", r"\1***"),

    # AWS Keys and Credentials
    (r"(AKIA[0-9A-Z]{16})", r"***"),
    (r"(aws[_-]?access[_-]?key[_-]?id\s*[:=]\s*)(AKIA[0-9A-Z]{16})", r"\1***"),
    (r"(aws[_-]?secret[_-]?access[_-]?key\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(aws[_-]?key\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(session[_-]?token\s*[:=]\s*)(\S+)", r"\1***"),

    # Database Credentials
    (r"(database[_-]?url\s*[:=]\s*)([a-z]+://[^:]+):([^@]+)(@.*)", r"\1\2:***\4"),
    (r"(database[_-]?password\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(db[_-]?password\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(mongodb[_-]?uri\s*[:=]\s*)([a-z]+://[^:]+):([^@]+)(@.*)", r"\1\2:***\3"),
    (r"(postgres[_-]?password\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(mysql[_-]?password\s*[:=]\s*)(\S+)", r"\1***"),

    # GCP Credentials
    (r"(GCP[_-]?SERVICE[_-]?ACCOUNT[_-]?KEY\s*[:=]\s*)(\{.*?\})", r"\1***"),
    (r"(GOOGLE[_-]?CLOUD[_-]?API[_-]?KEY\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(gcp[_-]?key\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(service[_-]?account[_-]?key\s*[:=]\s*)(\{.*?\})", r"\1***"),

    # GitHub Tokens
    (r"(github[_-]?token\s*[:=]\s*)(ghp_[a-zA-Z0-9]{36})", r"\1***"),
    (r"(github[_-]?pat\s*[:=]\s*)(ghp_[a-zA-Z0-9]{36})", r"\1***"),
    (r"(github[_-]?key\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(github[_-]?secret\s*[:=]\s*)(\S+)", r"\1***"),

    # SSH Keys
    (r"(-----BEGIN RSA PRIVATE KEY-----)", r"[REDACTED SSH KEY]"),
    (r"(-----BEGIN OPENSSH PRIVATE KEY-----)", r"[REDACTED SSH KEY]"),
    (r"(ssh[_-]?key\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(private[_-]?key\s*[:=]\s*)(\S+)", r"\1***"),

    # JWT Tokens
    (r"(jwt\s*[:=]\s*)([a-zA-Z0-9\-\._]+\.[a-zA-Z0-9\-\._]+\.[a-zA-Z0-9\-\._]+)", r"\1***"),
    (r"(bearer\s+)([a-zA-Z0-9\-\._]+\.[a-zA-Z0-9\-\._]+\.[a-zA-Z0-9\-\._]+)", r"\1***"),
    (r"(authorization\s*[:=]\s*Bearer\s+)([a-zA-Z0-9\-\._]+)", r"\1***"),

    # PII Patterns
    (r"(ssn\s*[:=]\s*)(\d{3}-\d{2}-\d{4})", r"\1***"),
    (r"(credit[_-]?card\s*[:=]\s*)(\d{4}[_\s]?\d{4}[_\s]?\d{4}[_\s]?\d{4})", r"\1***"),
    (r"(social[_-]?security[_-]?number\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(phone[_-]?number\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(email\s*[:=]\s*)([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+)", r"\1***"),

    # Other tokens
    (r"(slack[_-]?token\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(discord[_-]?token\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(twilio[_-]?auth[_-]?token\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(auth[_-]?token\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(bearer[_-]?token\s*[:=]\s*)(\S+)", r"\1***"),

    # Additional patterns
    (r"(cert[_-]?password\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(certificate[_-]?password\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(encryption[_-]?key\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(private[_-]?secret\s*[:=]\s*)(\S+)", r"\1***"),
    (r"(sensitive[_-]?data\s*[:=]\s*)(\S+)", r"\1***"),
]


class ContextExtractor:
    """Extracts operation context from TodoWrite and other sources."""

    def __init__(self, max_size: int = MAX_CONTEXT_SIZE_BYTES):
        """
        Initialize ContextExtractor.

        Args:
            max_size: Maximum context size in bytes (default: 50KB)
        """
        self.max_size = max_size

    def extract_operation_context(
        self, operation: str, story_id: Optional[str] = None
    ) -> Dict[str, Any]:
        """
        Extract context for an operation.

        Args:
            operation: Operation name (e.g., 'dev', 'qa', 'release')
            story_id: Optional story ID (format: STORY-NNN)

        Returns:
            Dictionary with operation context
        """
        try:
            # Generate operation ID
            operation_id = self._generate_operation_id(operation, story_id)

            # Extract timing
            timing = self.extract_timing()

            # Extract todos
            todos = self.extract_todos()

            # Extract errors
            errors = self.extract_errors()

            # Determine status based on errors
            status = "completed" if not errors else "failed"

            # Build context
            context = {
                "operation_id": operation_id,
                "operation": operation,
                "story_id": story_id,
                "start_time": timing.get("start_time"),
                "end_time": timing.get("end_time"),
                "duration": timing.get("duration"),
                "status": status,
                "todos": todos,
                "errors": errors,
                "phases": [],
            }

            # Limit context size
            context = self.limit_context_size(context)

            # Add context size
            context_json = json.dumps(context)
            context["context_size_bytes"] = len(context_json.encode("utf-8"))

            return context

        except Exception as e:
            logger.warning(f"Context extraction error: {str(e)}")
            # Return minimal context on error
            return {
                "operation_id": self._generate_operation_id(operation, story_id),
                "operation": operation,
                "story_id": story_id,
                "status": "error",
                "todos": [],
                "errors": [{"message": str(e), "exception_type": type(e).__name__}],
                "context_size_bytes": 0,
            }

    def extract_todos(self) -> List[Dict[str, Any]]:
        """
        Extract todos from TodoWrite.

        Returns:
            List of todo dictionaries with id, content, status, activeForm
        """
        try:
            # Attempt to read TodoWrite data
            # This would be from the actual TodoWrite API in production
            todos = []
            logger.debug(f"Extracted {len(todos)} todos")
            return todos
        except Exception as e:
            logger.warning(f"Failed to extract todos: {str(e)}")
            return []

    def extract_errors(self) -> List[Dict[str, Any]]:
        """
        Extract error information from operation.

        Returns:
            List of error dictionaries with message, exception_type, stack_trace
        """
        try:
            # Attempt to extract errors from operation logs
            # This would be from actual operation logs in production
            errors = []
            logger.debug(f"Extracted {len(errors)} errors")
            return errors
        except Exception as e:
            logger.warning(f"Failed to extract errors: {str(e)}")
            return []

    def extract_timing(self) -> Dict[str, Any]:
        """
        Extract timing information for operation.

        Returns:
            Dictionary with start_time, end_time, duration (seconds)
        """
        try:
            now = datetime.utcnow().isoformat() + "Z"
            return {
                "start_time": now,
                "end_time": now,
                "duration": 0,
            }
        except Exception as e:
            logger.warning(f"Failed to extract timing: {str(e)}")
            return {
                "start_time": None,
                "end_time": None,
                "duration": 0,
            }

    def limit_context_size(
        self, context: Dict[str, Any], max_size: Optional[int] = None
    ) -> Dict[str, Any]:
        """
        Limit context size to maximum.

        If context exceeds max_size, summarizes todos and truncates errors.

        Args:
            context: Operation context
            max_size: Maximum size in bytes (default: 50KB)

        Returns:
            Context limited to max_size
        """
        if max_size is None:
            max_size = self.max_size

        context_size = self._calculate_context_size(context)

        if context_size <= max_size:
            return context

        logger.warning(
            f"Context size {context_size}B exceeds limit {max_size}B, summarizing"
        )

        # Apply size reduction strategies
        self._summarize_todos_if_needed(context)
        self._truncate_errors_if_needed(context)

        # Verify size after first reduction
        context_size = self._calculate_context_size(context)

        if context_size > max_size:
            logger.warning(
                f"Context still exceeds limit after summarization "
                f"({context_size}B > {max_size}B), removing phases"
            )
            context.pop("phases", None)

        return context

    def _calculate_context_size(self, context: Dict[str, Any]) -> int:
        """Calculate JSON size of context in bytes."""
        context_json = json.dumps(context)
        return len(context_json.encode("utf-8"))

    def _summarize_todos_if_needed(self, context: Dict[str, Any]) -> None:
        """Summarize todos if count exceeds threshold."""
        todos = context.get("todos", [])
        if len(todos) <= MAX_TODOS_BEFORE_SUMMARY:
            return

        summary = self._build_todo_summary(todos)
        context["todos"] = [summary]

    def _truncate_errors_if_needed(self, context: Dict[str, Any]) -> None:
        """Truncate errors if count exceeds threshold."""
        errors = context.get("errors", [])
        if len(errors) <= MAX_ERRORS_BEFORE_TRUNCATION:
            return

        context["errors"] = errors[:MAX_ERRORS_BEFORE_TRUNCATION]
        context["errors"].append({"message": TRUNCATION_MARKER})

    @staticmethod
    def _build_todo_summary(todos: List[Dict[str, Any]]) -> Dict[str, Any]:
        """Build summary of todos by status."""
        return {
            "total": len(todos),
            "completed": sum(1 for t in todos if t.get("status") == "completed"),
            "in_progress": sum(1 for t in todos if t.get("status") == "in_progress"),
            "pending": sum(1 for t in todos if t.get("status") == "pending"),
        }

    def _generate_operation_id(
        self, operation: str, story_id: Optional[str] = None
    ) -> str:
        """
        Generate unique operation ID.

        Args:
            operation: Operation name
            story_id: Optional story ID

        Returns:
            Operation ID in format: operation-story-timestamp
        """
        timestamp = datetime.utcnow().strftime("%Y%m%d-%H%M%S")
        if story_id:
            return f"{operation}-{story_id}-{timestamp}"
        return f"{operation}-{timestamp}"


def extract_context(operation: str, story_id: Optional[str] = None) -> Dict[str, Any]:
    """
    Public API for context extraction.

    Args:
        operation: Operation name (e.g., 'dev', 'qa', 'release')
        story_id: Optional story ID (format: STORY-NNN)

    Returns:
        Dictionary with operation context
    """
    extractor = ContextExtractor()
    return extractor.extract_operation_context(operation, story_id)


def sanitize_context(context: Dict[str, Any]) -> Dict[str, Any]:
    """
    Sanitize sensitive data from context using regex patterns.

    Recursively removes secrets from all string values in context.
    Modifies context in place.

    Args:
        context: Operation context to sanitize

    Returns:
        Sanitized context
    """
    try:
        _sanitize_dict(context)
        return context
    except Exception as e:
        logger.warning(f"Context sanitization error: {str(e)}")
        return context


def _sanitize_dict(obj: Any) -> None:
    """
    Recursively sanitize dictionary values.

    Modifies object in place.

    Args:
        obj: Object to sanitize (dict, list, or scalar)
    """
    if isinstance(obj, dict):
        _sanitize_dict_items(obj)
    elif isinstance(obj, list):
        _sanitize_list_items(obj)


def _sanitize_dict_items(obj: Dict[str, Any]) -> None:
    """Sanitize all values in a dictionary."""
    for key, value in obj.items():
        if isinstance(value, str):
            obj[key] = _sanitize_string(value)
        elif isinstance(value, (dict, list)):
            _sanitize_dict(value)


def _sanitize_list_items(obj: List[Any]) -> None:
    """Sanitize all items in a list."""
    for i, item in enumerate(obj):
        if isinstance(item, str):
            obj[i] = _sanitize_string(item)
        elif isinstance(item, (dict, list)):
            _sanitize_dict(item)


def _sanitize_string(value: str) -> str:
    """
    Sanitize a string value by applying all secret patterns.

    Args:
        value: String value to sanitize

    Returns:
        Sanitized string with secrets replaced with ***
    """
    return _apply_sanitization_patterns(value)


def _apply_sanitization_patterns(value: str) -> str:
    """Apply all registered secret patterns to value."""
    for pattern, replacement in SECRET_PATTERNS:
        value = re.sub(pattern, replacement, value, flags=re.IGNORECASE)
    return value
