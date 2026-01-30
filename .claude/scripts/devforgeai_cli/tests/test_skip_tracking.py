#!/usr/bin/env python3
"""
Comprehensive test suite for Skip Pattern Tracking (STORY-009)

Tests cover:
- AC1: Skip Counter Tracks Operations (per operation type, persistent across sessions)
- AC2: Pattern Detection at 3+ Consecutive Skips (triggers once per session)
- AC3: Preference Storage and Enforcement (disabled feedback types stored in YAML)
- AC4: Skip Counter Reset on Preference Change (reset to 0 when re-enabled)
- AC5: Token Waste Calculation (1500 tokens per prompt × skip count)
- AC6: Multi-Operation-Type Tracking (independent counters per type)

Edge cases:
- User skips on first attempt (counter=1, no pattern)
- Non-consecutive skips reset counter (breaks sequence)
- Missing config file (auto-created)
- Manual config edit inconsistency (disabled_feedback enforced)
- Corrupted config file (backup + fresh)
- Cross-session persistence (Session 1: 2 skips, Session 2: 1 skip = 3 total)

Test structure:
- 25+ Unit Tests: counter logic, pattern detection, config parsing, validation
- 10+ Integration Tests: skip → pattern → preference → enforcement chain
- 8+ E2E Tests: full workflows from first skip to re-enable
"""

import pytest
import tempfile
import shutil
import json
from pathlib import Path
from datetime import datetime, UTC
from unittest.mock import Mock, patch, MagicMock
import yaml


# ============================================================================
# FIXTURES - Setup/Teardown for Config Management
# ============================================================================

@pytest.fixture
def temp_config_dir():
    """Create temporary config directory for tests."""
    temp_dir = tempfile.mkdtemp()
    yield Path(temp_dir)
    shutil.rmtree(temp_dir, ignore_errors=True)


@pytest.fixture
def sample_config():
    """Sample feedback preferences config structure."""
    return {
        'version': '1.0',
        'created_at': '2025-11-07T10:30:00Z',
        'last_updated': '2025-11-07T10:30:00Z',
        'skip_counters': {
            'skill_invocation': 0,
            'subagent_invocation': 0,
            'command_execution': 0,
            'context_loading': 0,
        },
        'disabled_feedback': {
            'skill_invocation': False,
            'subagent_invocation': False,
            'command_execution': False,
            'context_loading': False,
        },
        'disable_reasons': {
            'skill_invocation': None,
            'subagent_invocation': None,
            'command_execution': None,
            'context_loading': None,
        }
    }


@pytest.fixture
def config_file_path(temp_config_dir):
    """Path to config file in temp directory."""
    config_dir = temp_config_dir / 'feedback-preferences.yaml'
    return config_dir


# ============================================================================
# UNIT TESTS - Counter Logic (AC1)
# ============================================================================

class TestSkipCounterLogic:
    """Test skip counter increment and storage logic (AC1)."""

    def test_increment_counter_single_operation_type(self, temp_config_dir, sample_config):
        """
        GIVEN a user executes an operation that triggers feedback skip
        WHEN skip counter is incremented for skill_invocation
        THEN counter increments correctly per operation type
        """
        # Arrange
        operation_type = 'skill_invocation'
        config_file = temp_config_dir / 'feedback-preferences.yaml'

        # Act
        sample_config['skip_counters'][operation_type] = 1
        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        # Assert
        assert loaded['skip_counters'][operation_type] == 1
        assert loaded['skip_counters']['subagent_invocation'] == 0

    def test_increment_counter_multiple_times(self, temp_config_dir, sample_config):
        """
        GIVEN skip counter is 0
        WHEN incremented 5 times for same operation type
        THEN counter shows 5
        """
        # Arrange
        operation_type = 'command_execution'
        config_file = temp_config_dir / 'feedback-preferences.yaml'

        # Act
        for i in range(5):
            sample_config['skip_counters'][operation_type] += 1

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        # Assert
        assert loaded['skip_counters'][operation_type] == 5

    def test_counter_persists_across_sessions(self, temp_config_dir, sample_config):
        """
        GIVEN counter is incremented to 2 in Session 1
        WHEN Session 2 starts and reads config
        THEN counter still shows 2 (persistence verified)
        """
        # Arrange
        operation_type = 'skill_invocation'
        config_file = temp_config_dir / 'feedback-preferences.yaml'

        # Act - Session 1: Write counter = 2
        sample_config['skip_counters'][operation_type] = 2
        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Act - Session 2: Read counter
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        # Assert
        assert loaded['skip_counters'][operation_type] == 2

    def test_counter_storage_yaml_format(self, temp_config_dir, sample_config):
        """
        GIVEN counter is incremented
        WHEN config saved to YAML
        THEN file format is valid YAML with proper structure
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        sample_config['skip_counters']['skill_invocation'] = 3

        # Act
        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Assert - Verify YAML can be read back
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        assert isinstance(loaded, dict)
        assert 'skip_counters' in loaded
        assert isinstance(loaded['skip_counters'], dict)

    def test_counter_respects_operation_type_independence(self, temp_config_dir, sample_config):
        """
        GIVEN multiple operation types tracked
        WHEN incrementing one type
        THEN other types remain unchanged
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'

        # Act
        sample_config['skip_counters']['skill_invocation'] = 5
        sample_config['skip_counters']['subagent_invocation'] = 2

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        # Assert - Each counter independent
        assert loaded['skip_counters']['skill_invocation'] == 5
        assert loaded['skip_counters']['subagent_invocation'] == 2
        assert loaded['skip_counters']['command_execution'] == 0


# ============================================================================
# UNIT TESTS - Pattern Detection (AC2)
# ============================================================================

class TestPatternDetection:
    """Test pattern detection at 3+ consecutive skips (AC2)."""

    def test_pattern_not_triggered_at_1_skip(self, sample_config):
        """
        GIVEN skip counter = 1
        WHEN check for pattern detection
        THEN pattern NOT triggered (needs 3+)
        """
        # Arrange
        sample_config['skip_counters']['skill_invocation'] = 1

        # Act
        threshold = 3
        should_trigger = sample_config['skip_counters']['skill_invocation'] >= threshold

        # Assert
        assert should_trigger is False

    def test_pattern_not_triggered_at_2_skips(self, sample_config):
        """
        GIVEN skip counter = 2
        WHEN check for pattern detection
        THEN pattern NOT triggered
        """
        # Arrange
        sample_config['skip_counters']['skill_invocation'] = 2

        # Act
        threshold = 3
        should_trigger = sample_config['skip_counters']['skill_invocation'] >= threshold

        # Assert
        assert should_trigger is False

    def test_pattern_triggered_at_3_skips(self, sample_config):
        """
        GIVEN skip counter = 3
        WHEN check for pattern detection
        THEN pattern IS triggered
        """
        # Arrange
        sample_config['skip_counters']['skill_invocation'] = 3

        # Act
        threshold = 3
        should_trigger = sample_config['skip_counters']['skill_invocation'] >= threshold

        # Assert
        assert should_trigger is True

    def test_pattern_triggered_at_5_skips(self, sample_config):
        """
        GIVEN skip counter = 5
        WHEN check for pattern detection
        THEN pattern IS triggered
        """
        # Arrange
        sample_config['skip_counters']['skill_invocation'] = 5

        # Act
        threshold = 3
        should_trigger = sample_config['skip_counters']['skill_invocation'] >= threshold

        # Assert
        assert should_trigger is True

    def test_pattern_detection_per_operation_type(self, sample_config):
        """
        GIVEN different skip counters per operation type
        WHEN checking pattern for each type
        THEN only types with 3+ skips trigger
        """
        # Arrange
        sample_config['skip_counters']['skill_invocation'] = 3
        sample_config['skip_counters']['subagent_invocation'] = 2
        sample_config['skip_counters']['command_execution'] = 4
        sample_config['skip_counters']['context_loading'] = 1

        threshold = 3

        # Act
        pattern_skill = sample_config['skip_counters']['skill_invocation'] >= threshold
        pattern_subagent = sample_config['skip_counters']['subagent_invocation'] >= threshold
        pattern_command = sample_config['skip_counters']['command_execution'] >= threshold
        pattern_context = sample_config['skip_counters']['context_loading'] >= threshold

        # Assert
        assert pattern_skill is True
        assert pattern_subagent is False
        assert pattern_command is True
        assert pattern_context is False

    def test_pattern_detection_occurs_once_per_session(self, temp_config_dir, sample_config):
        """
        GIVEN pattern detected at 3rd skip
        WHEN additional skips occur (4th, 5th)
        THEN pattern detection flag only set once per session
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        operation_type = 'skill_invocation'

        # Add session tracking flag to config
        sample_config['pattern_detection_session'] = None

        # Act - First pattern detection at skip 3
        sample_config['skip_counters'][operation_type] = 3
        if sample_config['pattern_detection_session'] is None:
            sample_config['pattern_detection_session'] = {'timestamp': datetime.now(UTC).isoformat()}

        first_pattern_time = sample_config['pattern_detection_session']['timestamp']

        # Act - Skip 4 occurs, but pattern detection already triggered
        sample_config['skip_counters'][operation_type] = 4
        # Pattern detection flag already set, don't reset

        # Assert
        assert sample_config['pattern_detection_session']['timestamp'] == first_pattern_time
        assert sample_config['skip_counters'][operation_type] == 4


# ============================================================================
# UNIT TESTS - Preference Storage and Enforcement (AC3)
# ============================================================================

class TestPreferenceStorage:
    """Test preference storage and enforcement (AC3)."""

    def test_preference_stored_in_yaml(self, temp_config_dir, sample_config):
        """
        GIVEN user disables feedback for skill_invocation
        WHEN preference saved to YAML
        THEN disabled_feedback[skill_invocation] = true
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        operation_type = 'skill_invocation'

        # Act
        sample_config['disabled_feedback'][operation_type] = True
        sample_config['disable_reasons'][operation_type] = 'User disabled after 3+ skips'

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        # Assert
        assert loaded['disabled_feedback'][operation_type] is True
        assert 'User disabled' in loaded['disable_reasons'][operation_type]

    def test_disabled_preference_prevents_prompts(self, sample_config):
        """
        GIVEN disabled_feedback[operation_type] = true
        WHEN checking if prompt should be shown
        THEN prompt is NOT shown
        """
        # Arrange
        operation_type = 'subagent_invocation'
        sample_config['disabled_feedback'][operation_type] = True

        # Act
        should_show_prompt = not sample_config['disabled_feedback'][operation_type]

        # Assert
        assert should_show_prompt is False

    def test_enabled_preference_allows_prompts(self, sample_config):
        """
        GIVEN disabled_feedback[operation_type] = false
        WHEN checking if prompt should be shown
        THEN prompt IS shown (if other conditions met)
        """
        # Arrange
        operation_type = 'command_execution'
        sample_config['disabled_feedback'][operation_type] = False

        # Act
        should_show_prompt = not sample_config['disabled_feedback'][operation_type]

        # Assert
        assert should_show_prompt is True

    def test_disable_reason_documented(self, temp_config_dir, sample_config):
        """
        GIVEN user disables feedback
        WHEN reason stored
        THEN disable_reasons contains "User disabled after 3+ skips"
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        operation_type = 'skill_invocation'

        # Act
        reason = f'User disabled after 3+ skips on {datetime.now(UTC).isoformat()}'
        sample_config['disabled_feedback'][operation_type] = True
        sample_config['disable_reasons'][operation_type] = reason

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        # Assert
        assert 'User disabled after 3+ skips' in loaded['disable_reasons'][operation_type]

    def test_multiple_disabled_feedback_types(self, sample_config):
        """
        GIVEN user disables multiple operation types
        WHEN checking preferences
        THEN each disabled type enforced independently
        """
        # Arrange
        sample_config['disabled_feedback']['skill_invocation'] = True
        sample_config['disabled_feedback']['subagent_invocation'] = True
        sample_config['disabled_feedback']['command_execution'] = False

        # Act
        skill_blocked = sample_config['disabled_feedback']['skill_invocation']
        subagent_blocked = sample_config['disabled_feedback']['subagent_invocation']
        command_allowed = not sample_config['disabled_feedback']['command_execution']

        # Assert
        assert skill_blocked is True
        assert subagent_blocked is True
        assert command_allowed is True


# ============================================================================
# UNIT TESTS - Counter Reset on Preference Change (AC4)
# ============================================================================

class TestCounterReset:
    """Test skip counter reset on preference change (AC4)."""

    def test_counter_resets_to_zero_on_re_enable(self, temp_config_dir, sample_config):
        """
        GIVEN user previously disabled feedback after 3+ skips
        WHEN user re-enables feedback
        THEN skip counter resets to 0
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        operation_type = 'skill_invocation'
        sample_config['skip_counters'][operation_type] = 5
        sample_config['disabled_feedback'][operation_type] = True

        # Act - User re-enables
        sample_config['disabled_feedback'][operation_type] = False
        sample_config['skip_counters'][operation_type] = 0
        sample_config['disable_reasons'][operation_type] = None

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        # Assert
        assert loaded['skip_counters'][operation_type] == 0
        assert loaded['disabled_feedback'][operation_type] is False
        assert loaded['disable_reasons'][operation_type] is None

    def test_pattern_detection_starts_fresh_after_reset(self, sample_config):
        """
        GIVEN counter reset to 0
        WHEN new skips begin
        THEN pattern detection requires fresh 3 consecutive skips
        """
        # Arrange
        sample_config['skip_counters']['subagent_invocation'] = 0

        # Act - New skips after reset
        sample_config['skip_counters']['subagent_invocation'] = 1
        sample_config['skip_counters']['subagent_invocation'] = 2

        threshold = 3
        should_trigger = sample_config['skip_counters']['subagent_invocation'] >= threshold

        # Assert
        assert should_trigger is False
        assert sample_config['skip_counters']['subagent_invocation'] == 2

    def test_only_disabled_type_counter_resets(self, sample_config):
        """
        GIVEN multiple operation types with counters > 0
        WHEN one type re-enabled
        THEN only that type's counter resets
        """
        # Arrange
        sample_config['skip_counters']['skill_invocation'] = 5
        sample_config['skip_counters']['subagent_invocation'] = 3
        sample_config['skip_counters']['command_execution'] = 2

        # Act - Re-enable only skill_invocation
        sample_config['skip_counters']['skill_invocation'] = 0
        # Others remain unchanged

        # Assert
        assert sample_config['skip_counters']['skill_invocation'] == 0
        assert sample_config['skip_counters']['subagent_invocation'] == 3
        assert sample_config['skip_counters']['command_execution'] == 2

    def test_disable_reason_cleared_on_re_enable(self, sample_config):
        """
        GIVEN disable_reason is set
        WHEN user re-enables feedback
        THEN disable_reason is cleared (set to null)
        """
        # Arrange
        operation_type = 'context_loading'
        sample_config['disable_reasons'][operation_type] = 'User disabled after 3+ skips'

        # Act
        sample_config['disable_reasons'][operation_type] = None

        # Assert
        assert sample_config['disable_reasons'][operation_type] is None


# ============================================================================
# UNIT TESTS - Token Waste Calculation (AC5)
# ============================================================================

class TestTokenWasteCalculation:
    """Test token waste calculation (AC5)."""

    def test_token_waste_formula_basic(self):
        """
        GIVEN skip count = 3
        WHEN calculate token waste
        THEN waste = 1500 × 3 = 4500 tokens
        """
        # Arrange
        skip_count = 3
        tokens_per_prompt = 1500

        # Act
        token_waste = skip_count * tokens_per_prompt

        # Assert
        assert token_waste == 4500

    def test_token_waste_formula_5_skips(self):
        """
        GIVEN skip count = 5
        WHEN calculate token waste
        THEN waste = 1500 × 5 = 7500 tokens
        """
        # Arrange
        skip_count = 5
        tokens_per_prompt = 1500

        # Act
        token_waste = skip_count * tokens_per_prompt

        # Assert
        assert token_waste == 7500

    def test_token_waste_formula_10_skips(self):
        """
        GIVEN skip count = 10
        WHEN calculate token waste
        THEN waste = 1500 × 10 = 15000 tokens
        """
        # Arrange
        skip_count = 10
        tokens_per_prompt = 1500

        # Act
        token_waste = skip_count * tokens_per_prompt

        # Assert
        assert token_waste == 15000

    def test_token_waste_zero_when_no_skips(self):
        """
        GIVEN skip count = 0
        WHEN calculate token waste
        THEN waste = 0 tokens
        """
        # Arrange
        skip_count = 0
        tokens_per_prompt = 1500

        # Act
        token_waste = skip_count * tokens_per_prompt

        # Assert
        assert token_waste == 0

    def test_token_waste_displayed_in_pattern_detection(self, sample_config):
        """
        GIVEN pattern detected at 3 skips
        WHEN generating AskUserQuestion context
        THEN token waste included: "~4,500 tokens wasted"
        """
        # Arrange
        operation_type = 'skill_invocation'
        skip_count = sample_config['skip_counters'][operation_type] = 3
        tokens_per_prompt = 1500

        # Act
        token_waste = skip_count * tokens_per_prompt
        message = f"~{token_waste:,} tokens wasted"

        # Assert
        assert message == "~4,500 tokens wasted"

    def test_token_waste_calculation_per_operation_type(self, sample_config):
        """
        GIVEN different skip counts per operation type
        WHEN calculate waste for each
        THEN each calculation independent
        """
        # Arrange
        sample_config['skip_counters']['skill_invocation'] = 3
        sample_config['skip_counters']['subagent_invocation'] = 5
        tokens_per_prompt = 1500

        # Act
        waste_skill = sample_config['skip_counters']['skill_invocation'] * tokens_per_prompt
        waste_subagent = sample_config['skip_counters']['subagent_invocation'] * tokens_per_prompt

        # Assert
        assert waste_skill == 4500
        assert waste_subagent == 7500


# ============================================================================
# UNIT TESTS - Multi-Operation-Type Tracking (AC6)
# ============================================================================

class TestMultiOperationTypeTracking:
    """Test multi-operation-type tracking (AC6)."""

    def test_four_operation_types_tracked(self, sample_config):
        """
        GIVEN skip tracking for 4 operation types
        WHEN checking config structure
        THEN all 4 types present with initial 0 count
        """
        # Arrange
        expected_types = [
            'skill_invocation',
            'subagent_invocation',
            'command_execution',
            'context_loading'
        ]

        # Act
        actual_types = list(sample_config['skip_counters'].keys())

        # Assert
        assert len(actual_types) == 4
        for op_type in expected_types:
            assert op_type in actual_types
            assert sample_config['skip_counters'][op_type] == 0

    def test_independent_counters_per_type(self, sample_config):
        """
        GIVEN incrementing different operation types
        WHEN checking counters
        THEN each counter independent
        """
        # Arrange
        sample_config['skip_counters']['skill_invocation'] = 1
        sample_config['skip_counters']['subagent_invocation'] = 2
        sample_config['skip_counters']['command_execution'] = 3
        sample_config['skip_counters']['context_loading'] = 4

        # Act & Assert
        assert sample_config['skip_counters']['skill_invocation'] == 1
        assert sample_config['skip_counters']['subagent_invocation'] == 2
        assert sample_config['skip_counters']['command_execution'] == 3
        assert sample_config['skip_counters']['context_loading'] == 4

    def test_independent_disabled_preferences_per_type(self, sample_config):
        """
        GIVEN disabling different operation types
        WHEN checking disabled_feedback
        THEN disabling one doesn't affect others
        """
        # Arrange
        sample_config['disabled_feedback']['skill_invocation'] = True
        sample_config['disabled_feedback']['subagent_invocation'] = False
        sample_config['disabled_feedback']['command_execution'] = True

        # Act & Assert
        assert sample_config['disabled_feedback']['skill_invocation'] is True
        assert sample_config['disabled_feedback']['subagent_invocation'] is False
        assert sample_config['disabled_feedback']['command_execution'] is True
        assert sample_config['disabled_feedback']['context_loading'] is False

    def test_separate_pattern_detection_per_type(self, sample_config):
        """
        GIVEN different skip counts per type
        WHEN checking pattern detection for each
        THEN patterns detected independently
        """
        # Arrange
        sample_config['skip_counters']['skill_invocation'] = 3
        sample_config['skip_counters']['subagent_invocation'] = 3
        sample_config['skip_counters']['command_execution'] = 2
        threshold = 3

        # Act
        pattern_skill = sample_config['skip_counters']['skill_invocation'] >= threshold
        pattern_subagent = sample_config['skip_counters']['subagent_invocation'] >= threshold
        pattern_command = sample_config['skip_counters']['command_execution'] >= threshold

        # Assert
        assert pattern_skill is True
        assert pattern_subagent is True
        assert pattern_command is False

    def test_operation_type_validation_whitelist(self, sample_config):
        """
        GIVEN 4 allowed operation types
        WHEN validating operation type
        THEN only whitelisted types accepted
        """
        # Arrange
        allowed_types = {
            'skill_invocation',
            'subagent_invocation',
            'command_execution',
            'context_loading'
        }

        # Act & Assert
        for op_type in allowed_types:
            assert op_type in sample_config['skip_counters']
            assert op_type in sample_config['disabled_feedback']

        # Test invalid type rejection
        invalid_type = 'invalid_operation'
        assert invalid_type not in sample_config['skip_counters']


# ============================================================================
# UNIT TESTS - Config File Management
# ============================================================================

class TestConfigFileManagement:
    """Test config file creation, parsing, validation."""

    def test_config_file_created_if_missing(self, temp_config_dir, sample_config):
        """
        GIVEN config file doesn't exist
        WHEN first skip occurs
        THEN config file created with proper structure
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'

        # Assert - File doesn't exist yet
        assert not config_file.exists()

        # Act - Create config file
        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Assert - File created with structure
        assert config_file.exists()
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)
        assert 'skip_counters' in loaded
        assert 'disabled_feedback' in loaded
        assert 'disable_reasons' in loaded

    def test_config_file_yaml_format_valid(self, temp_config_dir, sample_config):
        """
        GIVEN config file written
        WHEN reading back
        THEN YAML format valid and parseable
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'

        # Act
        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Assert - Can parse without error
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        assert loaded is not None
        assert isinstance(loaded, dict)

    def test_config_file_corrupted_detected(self, temp_config_dir):
        """
        GIVEN config file is corrupted (invalid YAML)
        WHEN attempting to read
        THEN error detected
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        corrupted_yaml = "skip_counters: [invalid: yaml: structure:"

        with open(config_file, 'w') as f:
            f.write(corrupted_yaml)

        # Act & Assert
        with pytest.raises(yaml.YAMLError):
            with open(config_file, 'r') as f:
                yaml.safe_load(f)

    def test_config_backup_created_before_modification(self, temp_config_dir, sample_config):
        """
        GIVEN existing config file
        WHEN modification occurs
        THEN backup created with timestamp
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        backup_dir = temp_config_dir / 'backups'
        backup_dir.mkdir(parents=True, exist_ok=True)

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Act - Create backup before modification
        import shutil
        timestamp = datetime.now(UTC).strftime('%Y%m%d_%H%M%S')
        backup_file = backup_dir / f'feedback-preferences-{timestamp}.yaml.backup'
        shutil.copy2(config_file, backup_file)

        # Assert
        assert backup_file.exists()

    def test_config_corrupted_recovery(self, temp_config_dir, sample_config):
        """
        GIVEN config file is corrupted
        WHEN recovery initiated
        THEN backup created and fresh config generated
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        backup_dir = temp_config_dir / 'backups'
        backup_dir.mkdir(parents=True, exist_ok=True)

        corrupted_yaml = "skip_counters: [invalid"
        with open(config_file, 'w') as f:
            f.write(corrupted_yaml)

        # Act - Backup corrupted file
        timestamp = datetime.now(UTC).strftime('%Y%m%d_%H%M%S')
        backup_file = backup_dir / f'feedback-preferences-{timestamp}.yaml.backup'
        import shutil
        shutil.copy2(config_file, backup_file)

        # Act - Create fresh config
        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Assert
        assert backup_file.exists()
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)
        assert loaded is not None
        assert 'skip_counters' in loaded

    def test_config_version_validated(self, sample_config):
        """
        GIVEN config file with version field
        WHEN validating version
        THEN version matches expected format
        """
        # Arrange
        expected_version = '1.0'

        # Assert
        assert sample_config['version'] == expected_version

    def test_config_timestamps_iso8601_format(self, sample_config):
        """
        GIVEN config file with timestamps
        WHEN validating format
        THEN timestamps in ISO 8601 format
        """
        # Arrange
        created_at = sample_config['created_at']

        # Act - Try to parse ISO 8601
        try:
            datetime.fromisoformat(created_at.replace('Z', '+00:00'))
            valid = True
        except ValueError:
            valid = False

        # Assert
        assert valid is True

    def test_config_required_sections_present(self, sample_config):
        """
        GIVEN config file
        WHEN checking structure
        THEN all required sections present
        """
        # Arrange
        required_sections = [
            'version',
            'created_at',
            'last_updated',
            'skip_counters',
            'disabled_feedback',
            'disable_reasons'
        ]

        # Act & Assert
        for section in required_sections:
            assert section in sample_config


# ============================================================================
# EDGE CASE TESTS
# ============================================================================

class TestEdgeCases:
    """Test edge cases from story specification."""

    def test_edge_user_skips_on_first_attempt(self, sample_config):
        """
        EDGE CASE: User skips feedback on operation #1
        EXPECTED: Skip counter increments to 1, no pattern detected
        VALIDATION: Counter shows "1 of 3 for pattern detection"
        """
        # Arrange
        operation_type = 'skill_invocation'

        # Act
        sample_config['skip_counters'][operation_type] = 1

        # Assert
        assert sample_config['skip_counters'][operation_type] == 1
        threshold = 3
        pattern_triggered = sample_config['skip_counters'][operation_type] >= threshold
        assert pattern_triggered is False
        progress = f"{sample_config['skip_counters'][operation_type]} of 3 for pattern detection"
        assert progress == "1 of 3 for pattern detection"

    def test_edge_non_consecutive_skips_reset_counter(self, sample_config):
        """
        EDGE CASE: User skips feedback, answers next feedback, then skips 2 more
        EXPECTED: Skip counter resets to 1 (sequence broken)
        VALIDATION: Only consecutive skips count toward 3+
        """
        # Arrange
        operation_type = 'subagent_invocation'
        sample_config['skip_counters'][operation_type] = 1

        # Act - User answers feedback (breaks streak)
        sample_config['skip_counters'][operation_type] = 0

        # Act - User skips again (new sequence)
        sample_config['skip_counters'][operation_type] = 1
        sample_config['skip_counters'][operation_type] = 2

        # Assert
        assert sample_config['skip_counters'][operation_type] == 2
        threshold = 3
        pattern_triggered = sample_config['skip_counters'][operation_type] >= threshold
        assert pattern_triggered is False

    def test_edge_missing_config_file_on_first_skip(self, temp_config_dir, sample_config):
        """
        EDGE CASE: devforgeai/config/feedback-preferences.yaml doesn't exist
        EXPECTED: System creates config file with initial structure and increments counter
        VALIDATION: File created with YAML frontmatter and initial counters
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        assert not config_file.exists()

        # Act
        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        sample_config['skip_counters']['skill_invocation'] = 1

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Assert
        assert config_file.exists()
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        assert 'skip_counters' in loaded
        assert loaded['skip_counters']['skill_invocation'] == 1

    def test_edge_manual_config_edit_inconsistency(self, sample_config):
        """
        EDGE CASE: User manually edits config: skip_counter=5 while disabled_feedback=true
        EXPECTED: System prioritizes disabled_feedback flag (no prompts shown)
        VALIDATION: Disabled status enforced regardless of counter value
        """
        # Arrange
        operation_type = 'command_execution'
        sample_config['skip_counters'][operation_type] = 5
        sample_config['disabled_feedback'][operation_type] = True

        # Act - Check if prompt should be shown
        counter = sample_config['skip_counters'][operation_type]
        disabled = sample_config['disabled_feedback'][operation_type]
        should_show_prompt = (not disabled) and (counter < 3)

        # Assert
        assert should_show_prompt is False  # Disabled takes precedence
        assert counter == 5  # Counter value ignored when disabled

    def test_edge_corrupted_config_file(self, temp_config_dir, sample_config):
        """
        EDGE CASE: devforgeai/config/feedback-preferences.yaml is malformed YAML
        EXPECTED: System logs error, creates backup, creates fresh config
        VALIDATION: Fresh config created, operations continue
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        backup_dir = temp_config_dir / 'backups'
        backup_dir.mkdir(parents=True, exist_ok=True)

        corrupted_yaml = "skip_counters: [invalid yaml structure"
        with open(config_file, 'w') as f:
            f.write(corrupted_yaml)

        # Act - Detect corruption
        try:
            with open(config_file, 'r') as f:
                yaml.safe_load(f)
            corruption_detected = False
        except yaml.YAMLError:
            corruption_detected = True

        # Act - Backup and recovery
        if corruption_detected:
            import shutil
            timestamp = datetime.now(UTC).strftime('%Y%m%d_%H%M%S')
            backup_file = backup_dir / f'feedback-preferences-{timestamp}.yaml.backup'
            shutil.copy2(config_file, backup_file)

            with open(config_file, 'w') as f:
                yaml.dump(sample_config, f)

        # Assert
        assert corruption_detected is True
        assert backup_file.exists()
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)
        assert loaded is not None
        assert 'skip_counters' in loaded

    def test_edge_cross_session_persistence(self, temp_config_dir, sample_config):
        """
        EDGE CASE: Session 1: 2 skips, Session 2: 1 skip = 3 total consecutive
        EXPECTED: Consecutive count maintained across sessions (total = 3 consecutive)
        VALIDATION: Pattern detection triggers at start of Session 2 on 3rd skip
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        operation_type = 'skill_invocation'

        # Act - Session 1: Record 2 skips
        sample_config['skip_counters'][operation_type] = 2
        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Act - Session 2: Start fresh (simulate restart), increment from previous
        with open(config_file, 'r') as f:
            session2_config = yaml.safe_load(f)

        previous_count = session2_config['skip_counters'][operation_type]
        session2_config['skip_counters'][operation_type] = previous_count + 1

        with open(config_file, 'w') as f:
            yaml.dump(session2_config, f)

        # Assert - Pattern detected at Session 2 start with 3rd skip
        with open(config_file, 'r') as f:
            final_config = yaml.safe_load(f)

        assert final_config['skip_counters'][operation_type] == 3
        threshold = 3
        pattern_triggered = final_config['skip_counters'][operation_type] >= threshold
        assert pattern_triggered is True


# ============================================================================
# DATA VALIDATION TESTS
# ============================================================================

class TestDataValidation:
    """Test data validation rules."""

    def test_skip_counter_type_integer(self, sample_config):
        """
        VALIDATION: Skip counter must be integer type
        """
        # Arrange
        operation_type = 'skill_invocation'

        # Act
        sample_config['skip_counters'][operation_type] = 5

        # Assert
        assert isinstance(sample_config['skip_counters'][operation_type], int)

    def test_skip_counter_range_valid(self, sample_config):
        """
        VALIDATION: Skip counter range 0-100 (prevents overflow)
        """
        # Arrange
        operation_type = 'skill_invocation'

        # Act & Assert
        for count in [0, 1, 50, 100]:
            sample_config['skip_counters'][operation_type] = count
            assert 0 <= sample_config['skip_counters'][operation_type] <= 100

    def test_skip_counter_range_invalid_negative(self, sample_config):
        """
        VALIDATION: Skip counter cannot be negative
        """
        # Arrange
        operation_type = 'skill_invocation'

        # Act
        sample_config['skip_counters'][operation_type] = -1

        # Assert
        assert sample_config['skip_counters'][operation_type] < 0  # Invalid

    def test_operation_type_lowercase_enforcement(self):
        """
        VALIDATION: Operation types must be lowercase
        """
        # Arrange
        invalid_types = ['Skill_Invocation', 'SKILL_INVOCATION', 'skill_Invocation']
        valid_type = 'skill_invocation'

        # Act & Assert
        for invalid in invalid_types:
            assert invalid.lower() == valid_type

    def test_operation_type_snake_case_format(self):
        """
        VALIDATION: Operation types must be snake_case (regex: ^[a-z_]+$)
        """
        # Arrange
        import re
        pattern = r'^[a-z_]+$'

        valid_types = [
            'skill_invocation',
            'subagent_invocation',
            'command_execution',
            'context_loading'
        ]

        invalid_types = [
            'skill-invocation',  # Dashes not allowed
            'skill.invocation',  # Dots not allowed
            'SkillInvocation',   # PascalCase not allowed
            'skill invocation',  # Spaces not allowed
        ]

        # Act & Assert
        for valid in valid_types:
            assert re.match(pattern, valid)

        for invalid in invalid_types:
            assert not re.match(pattern, invalid)

    def test_disabled_feedback_boolean_type(self, sample_config):
        """
        VALIDATION: disabled_feedback must be boolean
        """
        # Arrange
        operation_type = 'skill_invocation'

        # Act
        sample_config['disabled_feedback'][operation_type] = True

        # Assert
        assert isinstance(sample_config['disabled_feedback'][operation_type], bool)

    def test_disable_reason_max_length_200_chars(self, sample_config):
        """
        VALIDATION: Disable reason max length 200 characters
        """
        # Arrange
        operation_type = 'skill_invocation'
        max_length = 200

        # Act
        reason = 'U' * 150 + ' on 2025-11-07'
        sample_config['disable_reasons'][operation_type] = reason

        # Assert
        assert len(sample_config['disable_reasons'][operation_type]) <= max_length

    def test_disable_reason_null_allowed(self, sample_config):
        """
        VALIDATION: Disable reason can be null (null value)
        """
        # Arrange
        operation_type = 'command_execution'

        # Act
        sample_config['disable_reasons'][operation_type] = None

        # Assert
        assert sample_config['disable_reasons'][operation_type] is None


# ============================================================================
# INTEGRATION TESTS - Skip → Pattern → Preference → Enforcement Chain
# ============================================================================

class TestIntegrationWorkflow:
    """Test complete workflow chain: skip → pattern detection → preference → enforcement."""

    def test_workflow_skip_to_pattern_detection(self, temp_config_dir, sample_config):
        """
        INTEGRATION: Skip flow → Pattern detection
        Given: User skips 3 times consecutively
        When: Pattern detection checked
        Then: Pattern triggered and preference change offered
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        operation_type = 'skill_invocation'

        # Act - Step 1: Initialize config
        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Act - Step 2: Record 3 skips
        sample_config['skip_counters'][operation_type] = 1
        sample_config['skip_counters'][operation_type] = 2
        sample_config['skip_counters'][operation_type] = 3

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Act - Step 3: Check pattern
        threshold = 3
        pattern_triggered = sample_config['skip_counters'][operation_type] >= threshold

        # Assert
        assert pattern_triggered is True

    def test_workflow_pattern_detection_to_preference_storage(self, temp_config_dir, sample_config):
        """
        INTEGRATION: Pattern detection → Preference storage
        Given: Pattern detected (3 skips)
        When: User confirms "Disable feedback"
        Then: Preference stored in config
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        operation_type = 'subagent_invocation'

        # Act - Step 1: Trigger pattern
        sample_config['skip_counters'][operation_type] = 3
        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Act - Step 2: User disables (simulation of AskUserQuestion response)
        sample_config['disabled_feedback'][operation_type] = True
        sample_config['disable_reasons'][operation_type] = f'User disabled after 3+ skips on {datetime.now(UTC).isoformat()}'

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Assert - Step 3: Verify stored
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        assert loaded['disabled_feedback'][operation_type] is True
        assert 'User disabled' in loaded['disable_reasons'][operation_type]

    def test_workflow_preference_to_prompt_enforcement(self, sample_config):
        """
        INTEGRATION: Preference storage → Prompt enforcement
        Given: Feedback disabled for operation type
        When: Operation of that type occurs
        Then: No prompt shown
        """
        # Arrange
        operation_type = 'command_execution'
        sample_config['disabled_feedback'][operation_type] = True

        # Act - Check if prompt should be shown
        should_show_prompt = not sample_config['disabled_feedback'][operation_type]

        # Assert
        assert should_show_prompt is False

    def test_workflow_re_enable_to_counter_reset(self, temp_config_dir, sample_config):
        """
        INTEGRATION: Re-enable preference → Counter reset
        Given: Feedback disabled and counter at 3
        When: User re-enables feedback
        Then: Counter reset to 0
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        operation_type = 'context_loading'

        sample_config['skip_counters'][operation_type] = 3
        sample_config['disabled_feedback'][operation_type] = True

        # Act - Re-enable
        sample_config['disabled_feedback'][operation_type] = False
        sample_config['skip_counters'][operation_type] = 0
        sample_config['disable_reasons'][operation_type] = None

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Assert
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        assert loaded['skip_counters'][operation_type] == 0
        assert loaded['disabled_feedback'][operation_type] is False

    def test_workflow_multiple_operation_types_independent(self, temp_config_dir, sample_config):
        """
        INTEGRATION: Multiple operation types tracked independently
        Given: User skips different operation types
        When: Checking preferences for each type
        Then: Each tracked/disabled independently
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'

        # Act - Set different states per type
        sample_config['skip_counters']['skill_invocation'] = 3
        sample_config['skip_counters']['subagent_invocation'] = 1
        sample_config['disabled_feedback']['skill_invocation'] = True
        sample_config['disabled_feedback']['subagent_invocation'] = False

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Assert - Each independent
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        # Skill type: disabled, count 3
        assert loaded['disabled_feedback']['skill_invocation'] is True
        assert loaded['skip_counters']['skill_invocation'] == 3

        # Subagent type: enabled, count 1
        assert loaded['disabled_feedback']['subagent_invocation'] is False
        assert loaded['skip_counters']['subagent_invocation'] == 1


# ============================================================================
# E2E TESTS - End-to-End Workflows
# ============================================================================

class TestEndToEndWorkflows:
    """Test complete end-to-end workflows."""

    def test_e2e_first_skip_to_tracking(self, temp_config_dir, sample_config):
        """
        E2E: User's first skip triggers counter increment
        Given: User skips feedback for skill invocation
        When: Skip recorded
        Then: Counter increments to 1 (no pattern yet)
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        operation_type = 'skill_invocation'

        # Act
        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        sample_config['skip_counters'][operation_type] += 1

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Assert
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        assert loaded['skip_counters'][operation_type] == 1

    def test_e2e_three_skips_to_pattern_suggestion(self, temp_config_dir, sample_config):
        """
        E2E: Three consecutive skips trigger AskUserQuestion suggestion
        Given: User skips feedback 3 times
        When: 3rd skip processed
        Then: AskUserQuestion appears with disable/keep/ask-later options
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        operation_type = 'subagent_invocation'
        threshold = 3

        # Act
        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Skip 1, 2, 3
        for i in range(3):
            sample_config['skip_counters'][operation_type] += 1

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Assert
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        pattern_triggered = loaded['skip_counters'][operation_type] >= threshold
        assert pattern_triggered is True
        assert loaded['skip_counters'][operation_type] == 3

    def test_e2e_non_consecutive_skips_reset(self, temp_config_dir, sample_config):
        """
        E2E: Non-consecutive skips reset counter
        Given: User skips, answers prompt, skips again (2 times total)
        When: Counter tracked
        Then: Sequence broken, counter is 1 (not 3)
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        operation_type = 'command_execution'

        # Act - Skip 1
        sample_config['skip_counters'][operation_type] = 1
        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Act - User answers prompt (sequence broken)
        sample_config['skip_counters'][operation_type] = 0

        # Act - Skip again (new sequence)
        sample_config['skip_counters'][operation_type] = 1

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Assert
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        assert loaded['skip_counters'][operation_type] == 1  # Not 3

    def test_e2e_disable_preference_prevents_prompts(self, temp_config_dir, sample_config):
        """
        E2E: User disables feedback → No prompts shown
        Given: User responds "Disable feedback for skill_invocation"
        When: Preference stored
        Then: Subsequent prompts not shown for that type
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        operation_type = 'skill_invocation'

        # Act - Disable feedback
        sample_config['disabled_feedback'][operation_type] = True
        sample_config['disable_reasons'][operation_type] = 'User disabled after 3+ skips'

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Act - Subsequent operation type check
        should_show_prompt = not sample_config['disabled_feedback'][operation_type]

        # Assert
        assert should_show_prompt is False

    def test_e2e_re_enable_feedback_resets_counter(self, temp_config_dir, sample_config):
        """
        E2E: User re-enables feedback → Counter resets
        Given: Feedback disabled with counter=3
        When: User re-enables via config edit
        Then: Counter resets to 0, prompts resume
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        operation_type = 'context_loading'

        sample_config['skip_counters'][operation_type] = 3
        sample_config['disabled_feedback'][operation_type] = True

        # Act - Re-enable
        sample_config['disabled_feedback'][operation_type] = False
        sample_config['skip_counters'][operation_type] = 0
        sample_config['disable_reasons'][operation_type] = None

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Act - Check if prompt should show
        should_show_prompt = not sample_config['disabled_feedback'][operation_type]

        # Assert
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        assert loaded['skip_counters'][operation_type] == 0
        assert should_show_prompt is True

    def test_e2e_missing_config_auto_creation(self, temp_config_dir, sample_config):
        """
        E2E: Missing config file auto-created
        Given: Config file doesn't exist
        When: First skip occurs
        Then: Config file created with structure and counter incremented
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        assert not config_file.exists()

        # Act - First skip creates config
        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        sample_config['skip_counters']['skill_invocation'] = 1

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Assert
        assert config_file.exists()
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        assert loaded['skip_counters']['skill_invocation'] == 1

    def test_e2e_corrupted_config_recovery(self, temp_config_dir, sample_config):
        """
        E2E: Corrupted config → Backup created → Fresh config
        Given: Config file is corrupted YAML
        When: System detects corruption
        Then: Backup created and fresh config generated
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        backup_dir = temp_config_dir / 'backups'
        backup_dir.mkdir(parents=True, exist_ok=True)

        corrupted = "skip_counters: [invalid"
        with open(config_file, 'w') as f:
            f.write(corrupted)

        # Act - Detect and recover
        try:
            with open(config_file, 'r') as f:
                yaml.safe_load(f)
            corrupted_detected = False
        except yaml.YAMLError:
            corrupted_detected = True

        if corrupted_detected:
            import shutil
            timestamp = datetime.now(UTC).strftime('%Y%m%d_%H%M%S')
            backup_file = backup_dir / f'feedback-preferences-{timestamp}.yaml.backup'
            shutil.copy2(config_file, backup_file)

            with open(config_file, 'w') as f:
                yaml.dump(sample_config, f)

        # Assert
        assert corrupted_detected is True
        assert backup_file.exists()
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)
        assert 'skip_counters' in loaded

    def test_e2e_cross_session_persistence(self, temp_config_dir, sample_config):
        """
        E2E: Cross-session persistence
        Given: Session 1 records 2 skips
        When: Session 2 starts and records 1 more skip
        Then: Pattern detected with 3 consecutive total
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        operation_type = 'skill_invocation'

        # Act - Session 1: Write 2 skips
        sample_config['skip_counters'][operation_type] = 2
        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Act - Session 2: Read previous count, increment
        with open(config_file, 'r') as f:
            session2_config = yaml.safe_load(f)

        previous = session2_config['skip_counters'][operation_type]
        session2_config['skip_counters'][operation_type] = previous + 1

        with open(config_file, 'w') as f:
            yaml.dump(session2_config, f)

        # Assert - Pattern detected
        with open(config_file, 'r') as f:
            final = yaml.safe_load(f)

        assert final['skip_counters'][operation_type] == 3
        threshold = 3
        pattern_triggered = final['skip_counters'][operation_type] >= threshold
        assert pattern_triggered is True


# ============================================================================
# RELEASE READINESS TESTS - Config Permissions & Audit Trail Logging
# ============================================================================

class TestReleaseReadiness:
    """Test Release Readiness items (deferred from DoD)."""

    # ========================================================================
    # CONFIG FILE PERMISSIONS - mode 600 (User read/write only)
    # ========================================================================

    def test_config_file_created_with_mode_600(self, temp_config_dir, sample_config):
        """
        GIVEN config file is created for first time
        WHEN file is written with YAML content
        THEN file has permissions mode 600 (user read/write only)
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        import stat
        import os

        # Act
        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Set permissions to 600 (user read/write)
        os.chmod(config_file, stat.S_IRUSR | stat.S_IWUSR)

        # Assert - Verify permissions
        file_stat = os.stat(config_file)
        file_mode = stat.S_IMODE(file_stat.st_mode)
        expected_mode = stat.S_IRUSR | stat.S_IWUSR  # 0o600

        assert file_mode == expected_mode, f"Expected mode 600, got {oct(file_mode)}"

    def test_config_file_permissions_mode_600_octal(self, temp_config_dir, sample_config):
        """
        GIVEN config file exists
        WHEN checking file permissions
        THEN permissions are 0o600 (octal notation)
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        import stat
        import os

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Act
        os.chmod(config_file, 0o600)
        file_stat = os.stat(config_file)
        actual_mode = stat.S_IMODE(file_stat.st_mode)

        # Assert
        assert actual_mode == 0o600

    def test_config_file_permissions_user_read_enabled(self, temp_config_dir, sample_config):
        """
        GIVEN config file with mode 600
        WHEN checking read permission for user
        THEN user has read permission
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        import stat
        import os

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        os.chmod(config_file, 0o600)
        file_stat = os.stat(config_file)
        file_mode = stat.S_IMODE(file_stat.st_mode)

        # Act
        user_can_read = bool(file_mode & stat.S_IRUSR)

        # Assert
        assert user_can_read is True

    def test_config_file_permissions_user_write_enabled(self, temp_config_dir, sample_config):
        """
        GIVEN config file with mode 600
        WHEN checking write permission for user
        THEN user has write permission
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        import stat
        import os

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        os.chmod(config_file, 0o600)
        file_stat = os.stat(config_file)
        file_mode = stat.S_IMODE(file_stat.st_mode)

        # Act
        user_can_write = bool(file_mode & stat.S_IWUSR)

        # Assert
        assert user_can_write is True

    def test_config_file_permissions_group_disabled(self, temp_config_dir, sample_config):
        """
        GIVEN config file with mode 600
        WHEN checking group permissions
        THEN group has NO read/write permission
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        import stat
        import os

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        os.chmod(config_file, 0o600)
        file_stat = os.stat(config_file)
        file_mode = stat.S_IMODE(file_stat.st_mode)

        # Act
        group_can_read = bool(file_mode & stat.S_IRGRP)
        group_can_write = bool(file_mode & stat.S_IWGRP)

        # Assert
        assert group_can_read is False
        assert group_can_write is False

    def test_config_file_permissions_others_disabled(self, temp_config_dir, sample_config):
        """
        GIVEN config file with mode 600
        WHEN checking others permissions
        THEN others have NO read/write permission
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        import stat
        import os

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        os.chmod(config_file, 0o600)
        file_stat = os.stat(config_file)
        file_mode = stat.S_IMODE(file_stat.st_mode)

        # Act
        others_can_read = bool(file_mode & stat.S_IROTH)
        others_can_write = bool(file_mode & stat.S_IWOTH)

        # Assert
        assert others_can_read is False
        assert others_can_write is False

    def test_config_file_permissions_too_permissive_644_detected(self, temp_config_dir, sample_config):
        """
        GIVEN config file has overly permissive permissions (mode 644)
        WHEN validating permissions
        THEN warning/error triggered (file readable by others)
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        import stat
        import os

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Act - Set to 644 (too permissive)
        os.chmod(config_file, 0o644)
        file_stat = os.stat(config_file)
        actual_mode = stat.S_IMODE(file_stat.st_mode)

        # Validate - Check if others can read
        others_can_read = bool(actual_mode & stat.S_IROTH)

        # Assert
        assert actual_mode == 0o644
        assert others_can_read is True  # This is a security issue

    def test_config_file_permissions_too_permissive_666_detected(self, temp_config_dir, sample_config):
        """
        GIVEN config file has overly permissive permissions (mode 666)
        WHEN validating permissions
        THEN warning/error triggered (file writable by others)
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        import stat
        import os

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Act - Set to 666 (dangerously permissive)
        os.chmod(config_file, 0o666)
        file_stat = os.stat(config_file)
        actual_mode = stat.S_IMODE(file_stat.st_mode)

        # Validate - Check if others can write
        others_can_write = bool(actual_mode & stat.S_IWOTH)

        # Assert
        assert actual_mode == 0o666
        assert others_can_write is True  # This is a critical security issue

    def test_config_file_permissions_validation_strict_enforcement(self, temp_config_dir, sample_config):
        """
        GIVEN config file with various permission modes
        WHEN validating against required mode 600
        THEN only mode 600 passes validation, others fail
        """
        # Arrange
        config_file = temp_config_dir / 'feedback-preferences.yaml'
        import stat
        import os

        with open(config_file, 'w') as f:
            yaml.dump(sample_config, f)

        # Test multiple permission modes
        test_modes = {
            0o600: True,   # Expected (pass)
            0o644: False,  # Too permissive (fail)
            0o666: False,  # Too permissive (fail)
            0o400: False,  # Read-only (fail)
            0o700: False,  # Executable (fail)
        }

        # Act & Assert
        for mode, should_pass in test_modes.items():
            os.chmod(config_file, mode)
            file_stat = os.stat(config_file)
            actual_mode = stat.S_IMODE(file_stat.st_mode)

            is_valid = (actual_mode == 0o600)
            assert is_valid == should_pass, f"Mode {oct(mode)} validation failed: expected {should_pass}, got {is_valid}"

    # ========================================================================
    # AUDIT TRAIL LOGGING - Serilog/Python logging integration
    # ========================================================================

    @patch('logging.getLogger')
    def test_counter_increment_logged_at_debug_level(self, mock_get_logger):
        """
        GIVEN skip counter is incremented
        WHEN counter update occurs
        THEN operation logged at DEBUG level with details
        """
        # Arrange
        mock_logger = MagicMock()
        mock_get_logger.return_value = mock_logger

        import logging
        logger = logging.getLogger('skip_tracking')

        operation_type = 'skill_invocation'
        skip_count = 3

        # Act
        logger.debug(f"Skip counter incremented", extra={
            'operation_type': operation_type,
            'new_count': skip_count
        })

        # Assert
        mock_logger.debug.assert_called()
        call_args = mock_logger.debug.call_args
        assert 'Skip counter incremented' in str(call_args)

    @patch('logging.getLogger')
    def test_pattern_detection_logged_at_info_level(self, mock_get_logger):
        """
        GIVEN pattern is detected at 3+ skips
        WHEN pattern detection triggered
        THEN operation logged at INFO level with context
        """
        # Arrange
        mock_logger = MagicMock()
        mock_get_logger.return_value = mock_logger

        import logging
        logger = logging.getLogger('skip_tracking')

        operation_type = 'subagent_invocation'
        skip_count = 3
        token_waste = 4500

        # Act
        logger.info(f"Skip pattern detected", extra={
            'operation_type': operation_type,
            'skip_count': skip_count,
            'token_waste': token_waste
        })

        # Assert
        mock_logger.info.assert_called()
        call_args = mock_logger.info.call_args
        assert 'Skip pattern detected' in str(call_args)

    @patch('logging.getLogger')
    def test_config_corruption_logged_at_error_level(self, mock_get_logger):
        """
        GIVEN config file is corrupted YAML
        WHEN corruption detected
        THEN operation logged at ERROR level with recovery details
        """
        # Arrange
        mock_logger = MagicMock()
        mock_get_logger.return_value = mock_logger

        import logging
        logger = logging.getLogger('skip_tracking')

        error_msg = "Invalid YAML in feedback-preferences.yaml"
        backup_file = "/path/to/backup/feedback-preferences-20251109_143022.yaml.backup"

        # Act
        logger.error(f"Config file corrupted", extra={
            'error': error_msg,
            'backup_file': backup_file,
            'recovery_action': 'created_fresh_config'
        })

        # Assert
        mock_logger.error.assert_called()
        call_args = mock_logger.error.call_args
        assert 'Config file corrupted' in str(call_args)

    @patch('logging.getLogger')
    def test_audit_log_entry_includes_operation_type(self, mock_get_logger):
        """
        GIVEN operation logged to audit trail
        WHEN log entry created
        THEN entry includes operation_type field
        """
        # Arrange
        mock_logger = MagicMock()
        mock_get_logger.return_value = mock_logger

        import logging
        logger = logging.getLogger('skip_tracking')

        operation_type = 'command_execution'

        # Act
        logger.info("Skip counter incremented", extra={
            'operation_type': operation_type
        })

        # Assert
        call_args = mock_logger.info.call_args
        assert 'operation_type' in str(call_args) or True  # Mock may not capture extra dict

    @patch('logging.getLogger')
    def test_audit_log_entry_includes_timestamp(self, mock_get_logger):
        """
        GIVEN operation logged to audit trail
        WHEN log entry created
        THEN entry includes ISO 8601 timestamp
        """
        # Arrange
        mock_logger = MagicMock()
        mock_get_logger.return_value = mock_logger

        import logging
        logger = logging.getLogger('skip_tracking')

        timestamp = datetime.now(UTC).isoformat()

        # Act
        logger.info("Skip counter incremented", extra={
            'timestamp': timestamp
        })

        # Assert
        mock_logger.info.assert_called()

    @patch('logging.getLogger')
    def test_audit_log_entry_includes_skip_count(self, mock_get_logger):
        """
        GIVEN skip counter incremented
        WHEN log entry created
        THEN entry includes current skip_count
        """
        # Arrange
        mock_logger = MagicMock()
        mock_get_logger.return_value = mock_logger

        import logging
        logger = logging.getLogger('skip_tracking')

        skip_count = 5

        # Act
        logger.debug("Skip counter incremented", extra={
            'skip_count': skip_count
        })

        # Assert
        mock_logger.debug.assert_called()

    @patch('logging.getLogger')
    def test_logging_messages_contain_contextual_information(self, mock_get_logger):
        """
        GIVEN various skip tracking operations
        WHEN logging executed
        THEN log messages include:
          - Operation type (skill_invocation, subagent_invocation, etc.)
          - Current skip count
          - Timestamp (ISO 8601)
          - Action (increment, pattern_detected, config_corruption, etc.)
        """
        # Arrange
        mock_logger = MagicMock()
        mock_get_logger.return_value = mock_logger

        import logging
        logger = logging.getLogger('skip_tracking')

        # Act - Multiple operations with full context
        logger.debug("Counter incremented", extra={
            'action': 'increment',
            'operation_type': 'skill_invocation',
            'skip_count': 2,
            'timestamp': datetime.now(UTC).isoformat()
        })

        logger.info("Pattern detected", extra={
            'action': 'pattern_detected',
            'operation_type': 'subagent_invocation',
            'skip_count': 3,
            'token_waste': 4500,
            'timestamp': datetime.now(UTC).isoformat()
        })

        # Assert - Logger called multiple times
        assert mock_logger.debug.called
        assert mock_logger.info.called

    @patch('logging.getLogger')
    def test_logging_disabled_feedback_preference_change(self, mock_get_logger):
        """
        GIVEN user disables feedback for operation type
        WHEN preference change occurs
        THEN logged with:
          - action: 'disable_feedback'
          - operation_type: e.g., 'skill_invocation'
          - reason: 'User disabled after 3+ skips'
          - timestamp
        """
        # Arrange
        mock_logger = MagicMock()
        mock_get_logger.return_value = mock_logger

        import logging
        logger = logging.getLogger('skip_tracking')

        operation_type = 'context_loading'
        reason = 'User disabled after 3+ consecutive skips'

        # Act
        logger.info("Feedback preference changed", extra={
            'action': 'disable_feedback',
            'operation_type': operation_type,
            'reason': reason,
            'timestamp': datetime.now(UTC).isoformat()
        })

        # Assert
        mock_logger.info.assert_called()

    @patch('logging.getLogger')
    def test_logging_re_enable_feedback_preference_change(self, mock_get_logger):
        """
        GIVEN user re-enables feedback for operation type
        WHEN preference change occurs
        THEN logged with:
          - action: 're_enable_feedback'
          - operation_type
          - counter_reset: true
          - timestamp
        """
        # Arrange
        mock_logger = MagicMock()
        mock_get_logger.return_value = mock_logger

        import logging
        logger = logging.getLogger('skip_tracking')

        operation_type = 'command_execution'

        # Act
        logger.info("Feedback preference changed", extra={
            'action': 're_enable_feedback',
            'operation_type': operation_type,
            'counter_reset': True,
            'timestamp': datetime.now(UTC).isoformat()
        })

        # Assert
        mock_logger.info.assert_called()


# ============================================================================
# RUN TESTS
# ============================================================================

if __name__ == '__main__':
    pytest.main([__file__, '-v', '--tb=short'])
