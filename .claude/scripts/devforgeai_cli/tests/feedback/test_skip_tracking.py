"""
Unit tests for skip tracking functionality

Tests cover:
- Skip counter increment/reset
- 3+ consecutive skips detection
- Configuration storage
"""

import pytest
import tempfile
import shutil
from pathlib import Path

from devforgeai_cli.feedback.skip_tracking import (
    increment_skip,
    get_skip_count,
    reset_skip_count,
    check_skip_threshold,
)


class TestSkipTracking:
    """Test skip tracking functionality"""

    @pytest.fixture
    def temp_config_dir(self):
        """Create temporary config directory for tests"""
        temp_dir = tempfile.mkdtemp()
        yield Path(temp_dir)
        shutil.rmtree(temp_dir)

    def test_increment_skip_increases_count(self, temp_config_dir):
        """
        GIVEN user skips feedback
        WHEN increment_skip is called
        THEN skip count increases by 1
        """
        # Arrange
        operation_type = 'skill_invocation'

        # Act
        count1 = increment_skip(operation_type, config_dir=temp_config_dir)
        count2 = increment_skip(operation_type, config_dir=temp_config_dir)
        count3 = increment_skip(operation_type, config_dir=temp_config_dir)

        # Assert
        assert count1 == 1
        assert count2 == 2
        assert count3 == 3

    def test_get_skip_count_returns_current_count(self, temp_config_dir):
        """
        GIVEN skip count has been incremented
        WHEN get_skip_count is called
        THEN it returns current count
        """
        # Arrange
        operation_type = 'skill_invocation'
        increment_skip(operation_type, config_dir=temp_config_dir)
        increment_skip(operation_type, config_dir=temp_config_dir)

        # Act
        count = get_skip_count(operation_type, config_dir=temp_config_dir)

        # Assert
        assert count == 2

    def test_get_skip_count_returns_zero_for_new_operation_type(self, temp_config_dir):
        """
        GIVEN operation type with no skip history
        WHEN get_skip_count is called
        THEN it returns 0
        """
        # Arrange
        operation_type = 'subagent_invocation'

        # Act
        count = get_skip_count(operation_type, config_dir=temp_config_dir)

        # Assert
        assert count == 0

    def test_reset_skip_count_resets_to_zero(self, temp_config_dir):
        """
        GIVEN skip count is 5
        WHEN reset_skip_count is called
        THEN count resets to 0
        """
        # Arrange
        operation_type = 'command_execution'
        increment_skip(operation_type, config_dir=temp_config_dir)
        increment_skip(operation_type, config_dir=temp_config_dir)
        increment_skip(operation_type, config_dir=temp_config_dir)
        increment_skip(operation_type, config_dir=temp_config_dir)
        increment_skip(operation_type, config_dir=temp_config_dir)

        # Act
        reset_skip_count(operation_type, config_dir=temp_config_dir)
        count = get_skip_count(operation_type, config_dir=temp_config_dir)

        # Assert
        assert count == 0

    def test_check_skip_threshold_returns_true_at_3_skips(self, temp_config_dir):
        """
        GIVEN operation type has skipped 3+ consecutive times
        WHEN check_skip_threshold is called
        THEN it returns True (trigger suggestion)
        """
        # Arrange
        operation_type = 'context_loading'
        increment_skip(operation_type, config_dir=temp_config_dir)
        increment_skip(operation_type, config_dir=temp_config_dir)
        increment_skip(operation_type, config_dir=temp_config_dir)

        # Act
        reached_threshold = check_skip_threshold(operation_type, threshold=3, config_dir=temp_config_dir)

        # Assert
        assert reached_threshold is True

    def test_check_skip_threshold_returns_false_below_threshold(self, temp_config_dir):
        """
        GIVEN operation type has skipped 2 times (below threshold)
        WHEN check_skip_threshold is called
        THEN it returns False
        """
        # Arrange
        operation_type = 'skill_invocation'
        increment_skip(operation_type, config_dir=temp_config_dir)
        increment_skip(operation_type, config_dir=temp_config_dir)

        # Act
        reached_threshold = check_skip_threshold(operation_type, threshold=3, config_dir=temp_config_dir)

        # Assert
        assert reached_threshold is False

    def test_skip_tracking_persists_across_sessions(self, temp_config_dir):
        """
        GIVEN skip count has been incremented
        WHEN new session starts and checks skip count
        THEN count persists from previous session
        """
        # Arrange
        operation_type = 'subagent_invocation'
        increment_skip(operation_type, config_dir=temp_config_dir)
        increment_skip(operation_type, config_dir=temp_config_dir)

        # Act - Simulate new session (re-read from disk)
        count = get_skip_count(operation_type, config_dir=temp_config_dir)

        # Assert
        assert count == 2

        # Verify config file exists
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        assert config_file.exists()
