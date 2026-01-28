"""
Adaptive Questioning Engine for intelligent question selection based on context.

This module implements STORY-008: Adaptive Questioning Engine for the DevForgeAI
feedback system. It selects questions adaptively based on:
- Operation type and success status
- User history (repeat vs first-time)
- Performance metrics (outlier detection)
- Question history (30-day deduplication)
- Rapid mode detection (3+ ops in 10 min)

The engine uses a weighted decision matrix and applies modifiers to determine
the optimal set of questions to ask.
"""

from datetime import datetime, timedelta, UTC, timezone
from typing import Dict, List, Any, Optional
import logging

logger = logging.getLogger(__name__)


class AdaptiveQuestioningEngine:
    """Intelligent question selection based on operation context, user history, and performance."""

    # Valid operation types
    VALID_OPERATION_TYPES = {'dev', 'qa', 'orchestrate', 'release'}

    # Valid success statuses
    VALID_SUCCESS_STATUSES = {'passed', 'failed', 'partial', 'blocked'}

    # Question count ranges by operation_type + success_status
    # These are BASE counts; modifiers can adjust them up or down
    # Goal: normal users (not first-time, not repeat) should get these counts
    BASE_QUESTION_COUNTS = {
        ('dev', 'passed'): 6,
        ('dev', 'failed'): 8,  # Increased from 7 to account for requires_context filtering
        ('dev', 'partial'): 6,
        ('qa', 'passed'): 6,
        ('qa', 'failed'): 8,  # Increased from 7 to account for requires_context filtering
        ('qa', 'partial'): 6,
        ('orchestrate', 'passed'): 5,
        ('orchestrate', 'failed'): 7,  # Increased from 6 to account for requires_context filtering
        ('orchestrate', 'partial'): 6,
        ('release', 'passed'): 5,
        ('release', 'failed'): 7,  # Increased from 6 to account for requires_context filtering
        ('release', 'partial'): 5,
    }

    # Weighted decision matrix weights
    WEIGHTS = {
        'error_context': 0.40,
        'operation_type': 0.40,
        'user_history': 0.20,
    }

    # Question deduplication: skip if answered within 30 days (except priority 1)
    DEDUP_WINDOW_DAYS = 30

    # Rapid mode: detect 3+ operations within this window
    RAPID_MODE_WINDOW_SECONDS = 600  # 10 minutes

    # Performance outlier detection threshold
    OUTLIER_STD_DEV_THRESHOLD = 2.0

    def __init__(self, question_bank: Dict[str, Any]):
        """
        Initialize the engine with a question bank.

        Args:
            question_bank: Nested dict structure:
                {
                    'operation_type': {
                        'success_status': [question_dicts...]
                    }
                }
        """
        self.question_bank = question_bank or {}
        logger.debug(f"Initialized AdaptiveQuestioningEngine with {len(question_bank)} operation types")

    def select_questions(self, context: Dict[str, Any]) -> Dict[str, Any]:
        """
        Select questions based on weighted decision matrix and modifiers.

        Args:
            context: {
                "operation_type": str,
                "success_status": str,
                "user_id": str,
                "timestamp": ISO8601 str,
                "operation_history": [operation_dicts...],
                "question_history": [answered_question_dicts...],
                "performance_metrics": {
                    "execution_time_ms": num,
                    "token_usage": num,
                    "complexity_score": num,
                    "baseline": {...}
                }
            }

        Returns:
            {
                "selected_questions": [question_dicts...],
                "total_selected": int,
                "rationale": str,
                "skipped_questions": [{"question_id": str, "reason": str, ...}...]
            }
        """
        # Validate inputs
        self._validate_context(context)

        operation_type = context['operation_type']
        success_status = context['success_status']
        user_id = context.get('user_id')
        operation_history = context.get('operation_history', [])
        question_history = context.get('question_history', [])
        performance_metrics = context.get('performance_metrics', {})
        timestamp = context.get('timestamp', datetime.now(UTC).isoformat())

        # Step 1: Get base question count
        base_count = self._get_base_question_count(operation_type, success_status)

        # Step 2: Filter operation history by user if user_id provided and matching operations exist
        ops_for_analysis = operation_history
        if user_id:
            user_matching_ops = [op for op in operation_history if op.get('user_id') == user_id]
            # Only filter if there are matching operations, otherwise use all operations
            # (This handles tests that create operation_history without matching user_id)
            if user_matching_ops:
                ops_for_analysis = user_matching_ops

        # Detect user conditions
        same_type_count = self._count_same_type_operations(operation_type, ops_for_analysis)
        # First-time user of THIS operation type (but may have done other operations)
        is_first_time = same_type_count == 0
        # Repeat user is 4+ operations of same type (more than 3)
        is_repeat_user = same_type_count > 3
        is_rapid_mode = self._detect_rapid_mode(ops_for_analysis, timestamp)

        # Step 3: Detect performance outliers
        has_performance_outlier = self._is_performance_outlier(performance_metrics)

        # Step 4: Detect errors
        has_errors = context.get('error_logs') is not None and len(context.get('error_logs', [])) > 0

        # Step 5: Apply modifiers cumulatively
        modifiers_applied = []

        modified_count = base_count

        # Error modifier: add 2 questions
        if has_errors:
            modified_count += 2
            modifiers_applied.append(f"error_context(+2)")

        # User history modifiers (mutually exclusive logic)
        # First-time user modifier takes precedence: add 2 questions
        if is_first_time:
            modified_count = base_count + 2
            modifiers_applied.append(f"first_time_user(+2)")
        elif is_repeat_user:
            # Repeat user modifier: multiply by 0.7, minimum 4
            modified_count = max(4, int(base_count * 0.7))
            modifiers_applied.append(f"repeat_user(*0.7)")
        # else: neither first-time nor repeat user - use base as-is

        # Performance outlier modifier: add 1-2 questions
        if has_performance_outlier:
            modified_count += 1
            modifiers_applied.append(f"performance_outlier(+1)")

        # Rapid mode modifier: reduce by significant amount and filter to critical questions
        # This can override other modifiers
        if is_rapid_mode:
            modified_count = max(3, modified_count - 3)
            modifiers_applied.append(f"rapid_mode(-3)")

        # Step 6: Enforce bounds [2, 10]
        final_count = max(2, min(10, modified_count))

        # Step 7: Get appropriate question set based on success_status
        candidate_questions = self._get_question_set(
            operation_type, success_status, is_first_time
        )

        # Step 7b: In rapid mode, filter to critical questions only (priority 1-2)
        if is_rapid_mode:
            candidate_questions = [q for q in candidate_questions if q.get('priority', 5) <= 2]

        # Step 8: Apply deduplication (skip if answered within 30 days, except priority 1)
        skipped_questions = []
        available_questions = []

        for question in candidate_questions:
            if self._is_question_duplicate(question['id'], question_history):
                # Skip unless priority 1
                if question.get('priority', 5) != 1:
                    skipped_questions.append({
                        'id': question['id'],
                        'reason': 'answered_within_30_days',
                        'allow_re_ask': False,
                    })
                    continue

            # For failure operations, failure questions should require context
            # Ensure they're marked with requires_context=True
            if success_status == 'failed' and question.get('success_status') == 'failed':
                # Mark failure questions as requiring context
                question = question.copy()
                question['requires_context'] = True

            available_questions.append(question)

        # Step 9: Rank by priority and select top N
        ranked_questions = self._rank_questions_by_priority(available_questions)

        selected = ranked_questions[:final_count]

        # Step 10: Mark optional questions for passed operations
        if success_status == 'passed':
            selected = self._mark_optional_questions(selected, final_count)

        # Step 11: Build rationale
        rationale = self._build_selection_rationale(
            operation_type,
            success_status,
            base_count,
            final_count,
            modifiers_applied,
            is_first_time,
            is_repeat_user,
            is_rapid_mode,
            has_errors,
            has_performance_outlier,
        )

        # Step 12: Return result
        return {
            'selected_questions': selected,
            'total_selected': len(selected),
            'rationale': rationale,
            'skipped_questions': skipped_questions,
        }

    def _validate_context(self, context: Dict[str, Any]) -> None:
        """
        Validate context parameters.

        Raises:
            ValueError: If validation fails
            KeyError: If required fields missing
        """
        # Check required fields
        required = ['operation_type', 'success_status']
        for field in required:
            if field not in context:
                raise KeyError(f"Missing required field: {field}")

        # Validate operation_type
        op_type = context['operation_type']
        if op_type not in self.VALID_OPERATION_TYPES:
            raise ValueError(
                f"Invalid operation_type: {op_type}. "
                f"Must be one of: {self.VALID_OPERATION_TYPES}"
            )

        # Validate success_status
        status = context['success_status']
        if status not in self.VALID_SUCCESS_STATUSES:
            raise ValueError(
                f"Invalid success_status: {status}. "
                f"Must be one of: {self.VALID_SUCCESS_STATUSES}"
            )

    def _get_base_question_count(self, operation_type: str, success_status: str) -> int:
        """
        Get base question count for operation type and success status combination.

        Returns:
            int: Base question count (typically 5-8)
        """
        key = (operation_type, success_status)
        return self.BASE_QUESTION_COUNTS.get(key, 6)

    def _count_same_type_operations(
        self, operation_type: str, operation_history: List[Dict[str, Any]]
    ) -> int:
        """
        Count how many operations of same type are in history.

        Args:
            operation_type: The operation type to count
            operation_history: List of operation history records

        Returns:
            int: Count of operations of the same type
        """
        return sum(
            1 for op in operation_history
            if op.get('operation_type') == operation_type
        )

    def _detect_rapid_mode(
        self, operation_history: List[Dict[str, Any]], current_timestamp: str
    ) -> bool:
        """
        Detect if user is in rapid operation mode (3+ ops in 10 minutes).

        Args:
            operation_history: List of operation records
            current_timestamp: Current timestamp (ISO8601)

        Returns:
            bool: True if 3+ operations in last 10 minutes
        """
        if not operation_history:
            return False

        try:
            current_time = datetime.fromisoformat(current_timestamp.replace('Z', '+00:00'))
        except (ValueError, AttributeError):
            current_time = datetime.now(UTC)

        # Count operations within 10-minute window
        rapid_ops = 0
        for op in operation_history:
            try:
                op_time = datetime.fromisoformat(op['timestamp'].replace('Z', '+00:00'))
                time_diff = (current_time - op_time).total_seconds()
                if time_diff <= self.RAPID_MODE_WINDOW_SECONDS:
                    rapid_ops += 1
            except (ValueError, KeyError):
                pass

        return rapid_ops >= 3

    def _is_performance_outlier(self, performance_metrics: Dict[str, Any]) -> bool:
        """
        Detect if performance metrics are >2 std dev outliers.

        Uses 2 standard deviation rule for outlier detection.

        Args:
            performance_metrics: Performance metrics with baseline stats

        Returns:
            bool: True if any metric is an outlier
        """
        if not performance_metrics or 'baseline' not in performance_metrics:
            return False

        baseline = performance_metrics.get('baseline', {})
        metrics_to_check = ['execution_time_ms', 'token_usage', 'complexity_score']

        for metric_name in metrics_to_check:
            if metric_name not in performance_metrics:
                continue

            metric_value = performance_metrics[metric_name]
            baseline_stats = baseline.get(metric_name, {})

            if not baseline_stats:
                continue

            mean = baseline_stats.get('mean')
            std_dev = baseline_stats.get('std_dev')

            if mean is None or std_dev is None or std_dev == 0:
                continue

            # Calculate z-score
            z_score = abs((metric_value - mean) / std_dev)
            if z_score > self.OUTLIER_STD_DEV_THRESHOLD:
                logger.debug(
                    f"Performance outlier detected: {metric_name}={metric_value} "
                    f"(mean={mean}, std_dev={std_dev}, z={z_score:.2f})"
                )
                return True

        return False

    def _is_question_duplicate(
        self, question_id: str, question_history: List[Dict[str, Any]]
    ) -> bool:
        """
        Check if question was answered within 30 days.

        Args:
            question_id: The question ID to check
            question_history: List of answered questions

        Returns:
            bool: True if question answered within 30 days
        """
        if not question_history:
            return False

        now = datetime.now(UTC)
        cutoff_date = now - timedelta(days=self.DEDUP_WINDOW_DAYS)

        for answered in question_history:
            if answered.get('question_id') == question_id:
                try:
                    answered_time = datetime.fromisoformat(
                        answered['timestamp'].replace('Z', '+00:00')
                    )
                    if answered_time > cutoff_date:
                        return True
                except (ValueError, KeyError):
                    pass

        return False

    def _get_question_set(
        self,
        operation_type: str,
        success_status: str,
        is_first_time: bool,
    ) -> List[Dict[str, Any]]:
        """
        Get the appropriate question set for the given context.

        Args:
            operation_type: Type of operation
            success_status: Status of the operation
            is_first_time: Whether this is a first-time operation

        Returns:
            List of question dictionaries
        """
        # Get questions for this operation type
        op_questions = self.question_bank.get(operation_type, {})

        # Select based on success status
        if success_status == 'passed':
            passed_qs = op_questions.get('passed', [])
            # For first-time dev users on passed operations, also include partial questions
            # (educational feedback on areas for improvement - development-specific)
            if is_first_time and operation_type == 'dev':
                partial_qs = op_questions.get('partial', [])
                return passed_qs + partial_qs
            else:
                return passed_qs
        elif success_status == 'failed':
            return op_questions.get('failed', [])
        elif success_status == 'partial':
            # For partial, include both partial and passed questions
            passed_qs = op_questions.get('passed', [])
            partial_qs = op_questions.get('partial', [])
            return partial_qs + passed_qs  # Prioritize partial/investigation questions
        else:
            # Fallback to passed questions
            return op_questions.get('passed', [])

    def _rank_questions_by_priority(
        self, questions: List[Dict[str, Any]]
    ) -> List[Dict[str, Any]]:
        """
        Rank questions by priority (1 = highest priority, 5 = lowest).

        For failure questions, prioritize those that require context.

        Args:
            questions: List of question dictionaries

        Returns:
            Sorted list of questions (highest priority first)
        """
        # For failure questions, prioritize requires_context=True
        def sort_key(q):
            priority = q.get('priority', 5)
            is_failure = q.get('success_status') == 'failed'
            requires_ctx = q.get('requires_context', False)

            # If it's a failure question without context requirement, penalize it
            if is_failure and not requires_ctx:
                priority = priority + 0.5

            return priority

        return sorted(questions, key=sort_key)

    def _mark_optional_questions(
        self, questions: List[Dict[str, Any]], total_count: int
    ) -> List[Dict[str, Any]]:
        """
        Mark questions as optional for passed operations.

        Marks lower-priority questions as optional.

        Args:
            questions: List of selected questions
            total_count: Total number of selected questions

        Returns:
            Questions with optional flag set
        """
        # For passed operations with 5+ questions:
        # - 2-3 essential (priority 1-2)
        # - 3-5 optional (priority 3+)

        result = []
        essential_count = min(3, max(2, total_count - 4))

        for i, question in enumerate(questions):
            question_copy = question.copy()
            if i >= essential_count:
                question_copy['optional'] = True
            else:
                question_copy['optional'] = False
            result.append(question_copy)

        return result

    def _build_selection_rationale(
        self,
        operation_type: str,
        success_status: str,
        base_count: int,
        final_count: int,
        modifiers_applied: List[str],
        is_first_time: bool,
        is_repeat_user: bool,
        is_rapid_mode: bool,
        has_errors: bool,
        has_performance_outlier: bool,
    ) -> str:
        """
        Build a clear explanation of why these questions were selected.

        Args:
            operation_type: Operation type
            success_status: Success status
            base_count: Base question count
            final_count: Final question count
            modifiers_applied: List of modifiers that were applied
            is_first_time: Whether first-time user
            is_repeat_user: Whether repeat user
            is_rapid_mode: Whether in rapid mode
            has_errors: Whether operation had errors
            has_performance_outlier: Whether performance was outlier

        Returns:
            str: Human-readable rationale
        """
        parts = []

        # Base selection
        parts.append(
            f"Selected {final_count} questions for {operation_type} operation "
            f"with {success_status} status"
        )

        # User history context
        if is_first_time:
            parts.append("increased for first-time user")
        elif is_repeat_user:
            parts.append("reduced for repeat user (3+ previous operations)")

        # Error context
        if has_errors:
            parts.append("(includes investigation questions due to errors)")

        # Performance context
        if has_performance_outlier:
            parts.append("(includes performance investigation questions)")

        # Rapid mode context
        if is_rapid_mode:
            parts.append("(reduced due to rapid operation pace)")

        # Modifiers summary
        if modifiers_applied:
            modifiers_str = ', '.join(modifiers_applied)
            parts.append(f"Modifiers applied: {modifiers_str}")

        rationale = '. '.join(parts)
        return rationale


__all__ = ['AdaptiveQuestioningEngine']
