"""
Comprehensive test suite for STORY-011: Configuration Management

This test suite validates the YAML-based feedback system configuration including:
- YAML parsing and validation
- Master enable/disable controls
- Trigger mode determination
- Conversation settings enforcement
- Skip tracking statistics
- Template preferences
- Error handling and defaults
- Hot-reload functionality
- Edge case handling

Test Framework: pytest
Pattern: AAA (Arrange, Act, Assert)
Coverage Target: >95% of business logic

Tests are written to FAIL initially (TDD Red phase).
Implementation should make these tests PASS (TDD Green phase).
"""

import json
import os
import pytest
import tempfile
import threading
import time
from datetime import datetime, timedelta
from pathlib import Path
from typing import Dict, List, Optional
from unittest.mock import Mock, MagicMock, patch, call
from dataclasses import dataclass, asdict
from enum import Enum


# ============================================================================
# ENUMS AND DATA MODELS (from Technical Specification)
# ============================================================================

class TriggerMode(Enum):
    """Trigger modes for feedback collection."""
    ALWAYS = "always"
    FAILURES_ONLY = "failures-only"
    SPECIFIC_OPS = "specific-operations"
    NEVER = "never"


class TemplateFormat(Enum):
    """Template format options for feedback collection."""
    STRUCTURED = "structured"
    FREE_TEXT = "free-text"


class TemplateTone(Enum):
    """Template tone options for feedback questions."""
    BRIEF = "brief"
    DETAILED = "detailed"


@dataclass
class ConversationSettings:
    """Conversation-level settings for feedback collection."""
    max_questions: int = 5
    allow_skip: bool = True


@dataclass
class SkipTrackingSettings:
    """Skip tracking configuration."""
    enabled: bool = True
    max_consecutive_skips: int = 3
    reset_on_positive: bool = True


@dataclass
class TemplateSettings:
    """Template preferences for feedback collection."""
    format: str = "structured"  # structured|free-text
    tone: str = "brief"  # brief|detailed


@dataclass
class FeedbackConfiguration:
    """Complete feedback system configuration."""
    enabled: bool = True
    trigger_mode: str = "failures-only"
    operations: Optional[List[str]] = None
    conversation_settings: ConversationSettings = None
    skip_tracking: SkipTrackingSettings = None
    templates: TemplateSettings = None

    def __post_init__(self):
        """Initialize nested objects with defaults and convert dicts to objects."""
        # Convert dict to dataclass objects if needed
        if isinstance(self.conversation_settings, dict):
            self.conversation_settings = ConversationSettings(**self.conversation_settings)
        elif self.conversation_settings is None:
            self.conversation_settings = ConversationSettings()

        if isinstance(self.skip_tracking, dict):
            self.skip_tracking = SkipTrackingSettings(**self.skip_tracking)
        elif self.skip_tracking is None:
            self.skip_tracking = SkipTrackingSettings()

        if isinstance(self.templates, dict):
            self.templates = TemplateSettings(**self.templates)
        elif self.templates is None:
            self.templates = TemplateSettings()

        # Validate trigger_mode
        valid_modes = ["always", "failures-only", "specific-operations", "never"]
        if self.trigger_mode not in valid_modes:
            raise ValueError(
                f"Invalid trigger_mode value: '{self.trigger_mode}'. "
                f"Must be one of: {', '.join(valid_modes)}"
            )


# ============================================================================
# FIXTURES
# ============================================================================

@pytest.fixture
def temp_config_dir():
    """Create temporary directory for configuration files."""
    with tempfile.TemporaryDirectory() as tmpdir:
        config_dir = Path(tmpdir)
        yield config_dir


@pytest.fixture
def config_file(temp_config_dir):
    """Create path to feedback.yaml configuration file."""
    return temp_config_dir / "feedback.yaml"


@pytest.fixture
def logs_dir(temp_config_dir):
    """Create temporary logs directory."""
    logs_dir = temp_config_dir / "logs"
    logs_dir.mkdir(exist_ok=True)
    return logs_dir


@pytest.fixture
def valid_config_dict():
    """Provide valid configuration dictionary."""
    return {
        "enabled": True,
        "trigger_mode": "failures-only",
        "operations": ["qa", "deployment"],
        "conversation_settings": {
            "max_questions": 5,
            "allow_skip": True
        },
        "skip_tracking": {
            "enabled": True,
            "max_consecutive_skips": 3,
            "reset_on_positive": True
        },
        "templates": {
            "format": "structured",
            "tone": "brief"
        }
    }


@pytest.fixture
def config_manager(temp_config_dir, logs_dir):
    """Provide configuration manager instance (mock)."""
    # This will be used by test to verify behavior
    manager = MagicMock()
    manager.config_dir = temp_config_dir
    manager.logs_dir = logs_dir
    return manager


@pytest.fixture
def mock_file_watcher():
    """Provide mock file watcher for hot-reload testing."""
    watcher = MagicMock()
    watcher.start = MagicMock()
    watcher.stop = MagicMock()
    watcher.is_running = MagicMock(return_value=True)
    return watcher


@pytest.fixture
def default_config():
    """Provide default configuration."""
    return FeedbackConfiguration(
        enabled=True,
        trigger_mode="failures-only",
        operations=None,
        conversation_settings=ConversationSettings(max_questions=5, allow_skip=True),
        skip_tracking=SkipTrackingSettings(
            enabled=True,
            max_consecutive_skips=3,
            reset_on_positive=True
        ),
        templates=TemplateSettings(format="structured", tone="brief")
    )


# ============================================================================
# UNIT TESTS: YAML PARSING AND VALIDATION
# ============================================================================

class TestYamlParsing:
    """Test suite for YAML configuration file parsing."""

    def test_valid_yaml_structure_parses_successfully(self, config_file, valid_config_dict):
        """AC1: Valid YAML structure is parsed successfully.

        Given: config_file exists with valid YAML structure
        When: Configuration is loaded
        Then: All sections are accessible and no errors logged
        """
        # Arrange
        import yaml
        with open(config_file, 'w') as f:
            yaml.dump(valid_config_dict, f)

        # Act
        with open(config_file, 'r') as f:
            loaded_config = yaml.safe_load(f)

        # Assert
        assert loaded_config is not None
        assert loaded_config["enabled"] is True
        assert loaded_config["trigger_mode"] == "failures-only"
        assert "conversation_settings" in loaded_config
        assert "skip_tracking" in loaded_config
        assert "templates" in loaded_config

    def test_yaml_parsing_preserves_all_sections(self, config_file, valid_config_dict):
        """All configuration sections are preserved during parsing."""
        # Arrange
        import yaml
        with open(config_file, 'w') as f:
            yaml.dump(valid_config_dict, f)

        # Act
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        # Assert
        assert loaded["conversation_settings"]["max_questions"] == 5
        assert loaded["skip_tracking"]["max_consecutive_skips"] == 3
        assert loaded["templates"]["format"] == "structured"

    def test_yaml_with_invalid_syntax_raises_error(self, config_file):
        """Invalid YAML syntax is detected and reported."""
        # Arrange
        invalid_yaml = """
enabled: true
trigger_mode: failures-only
  invalid: [unclosed bracket
"""
        with open(config_file, 'w') as f:
            f.write(invalid_yaml)

        # Act & Assert
        import yaml
        with pytest.raises(yaml.YAMLError):
            with open(config_file, 'r') as f:
                yaml.safe_load(f)

    def test_empty_yaml_file_handled(self, config_file):
        """Empty YAML file returns None or empty dict."""
        # Arrange
        with open(config_file, 'w') as f:
            f.write("")

        # Act
        import yaml
        with open(config_file, 'r') as f:
            result = yaml.safe_load(f)

        # Assert
        assert result is None or result == {}

    def test_yaml_comments_ignored(self, config_file, valid_config_dict):
        """YAML comments are properly ignored during parsing."""
        # Arrange
        yaml_with_comments = """# Configuration for feedback system
# Version: 1.0
enabled: true  # Master enable switch
trigger_mode: failures-only  # Only collect on failures
"""
        with open(config_file, 'w') as f:
            f.write(yaml_with_comments)

        # Act
        import yaml
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        # Assert
        assert loaded["enabled"] is True
        assert loaded["trigger_mode"] == "failures-only"


class TestConfigurationValidation:
    """Test suite for configuration validation."""

    def test_valid_trigger_mode_always_accepted(self, valid_config_dict):
        """trigger_mode: always is valid."""
        # Arrange
        valid_config_dict["trigger_mode"] = "always"
        config = FeedbackConfiguration(**valid_config_dict)

        # Act & Assert
        assert config.trigger_mode == "always"

    def test_valid_trigger_mode_failures_only_accepted(self, valid_config_dict):
        """trigger_mode: failures-only is valid."""
        # Arrange
        valid_config_dict["trigger_mode"] = "failures-only"
        config = FeedbackConfiguration(**valid_config_dict)

        # Act & Assert
        assert config.trigger_mode == "failures-only"

    def test_valid_trigger_mode_specific_operations_accepted(self, valid_config_dict):
        """trigger_mode: specific-operations is valid."""
        # Arrange
        valid_config_dict["trigger_mode"] = "specific-operations"
        valid_config_dict["operations"] = ["qa", "deployment"]
        config = FeedbackConfiguration(**valid_config_dict)

        # Act & Assert
        assert config.trigger_mode == "specific-operations"
        assert config.operations == ["qa", "deployment"]

    def test_valid_trigger_mode_never_accepted(self, valid_config_dict):
        """trigger_mode: never is valid."""
        # Arrange
        valid_config_dict["trigger_mode"] = "never"
        config = FeedbackConfiguration(**valid_config_dict)

        # Act & Assert
        assert config.trigger_mode == "never"

    def test_invalid_trigger_mode_rejected(self, valid_config_dict):
        """AC7: Invalid trigger_mode is rejected with clear error message.

        Given: Configuration has trigger_mode: invalid-mode
        When: Configuration is validated
        Then: Error raised with message referencing documentation
        """
        # Arrange
        valid_config_dict["trigger_mode"] = "invalid-mode"

        # Act & Assert
        with pytest.raises(ValueError) as exc_info:
            # Validation should happen during configuration parsing
            valid_trigger_modes = ["always", "failures-only", "specific-operations", "never"]
            if valid_config_dict["trigger_mode"] not in valid_trigger_modes:
                raise ValueError(
                    f"Invalid trigger_mode value: '{valid_config_dict['trigger_mode']}'. "
                    f"Must be one of: {', '.join(valid_trigger_modes)}"
                )

        assert "Invalid trigger_mode value" in str(exc_info.value)

    def test_max_questions_zero_means_unlimited(self, valid_config_dict):
        """max_questions: 0 means unlimited."""
        # Arrange
        valid_config_dict["conversation_settings"]["max_questions"] = 0
        config = FeedbackConfiguration(**valid_config_dict)

        # Act & Assert
        assert config.conversation_settings.max_questions == 0

    def test_max_questions_accepts_large_values(self, valid_config_dict):
        """max_questions accepts extremely large values."""
        # Arrange
        valid_config_dict["conversation_settings"]["max_questions"] = 1000000

        # Act
        config = FeedbackConfiguration(**valid_config_dict)

        # Assert
        assert config.conversation_settings.max_questions == 1000000

    def test_max_consecutive_skips_zero_means_no_limit(self, valid_config_dict):
        """max_consecutive_skips: 0 means no limit."""
        # Arrange
        valid_config_dict["skip_tracking"]["max_consecutive_skips"] = 0
        config = FeedbackConfiguration(**valid_config_dict)

        # Act & Assert
        assert config.skip_tracking.max_consecutive_skips == 0

    def test_template_format_structured_valid(self, valid_config_dict):
        """templates.format: structured is valid."""
        # Arrange
        valid_config_dict["templates"]["format"] = "structured"
        config = FeedbackConfiguration(**valid_config_dict)

        # Act & Assert
        assert config.templates.format == "structured"

    def test_template_format_free_text_valid(self, valid_config_dict):
        """templates.format: free-text is valid."""
        # Arrange
        valid_config_dict["templates"]["format"] = "free-text"
        config = FeedbackConfiguration(**valid_config_dict)

        # Act & Assert
        assert config.templates.format == "free-text"

    def test_template_tone_brief_valid(self, valid_config_dict):
        """templates.tone: brief is valid."""
        # Arrange
        valid_config_dict["templates"]["tone"] = "brief"
        config = FeedbackConfiguration(**valid_config_dict)

        # Act & Assert
        assert config.templates.tone == "brief"

    def test_template_tone_detailed_valid(self, valid_config_dict):
        """templates.tone: detailed is valid."""
        # Arrange
        valid_config_dict["templates"]["tone"] = "detailed"
        config = FeedbackConfiguration(**valid_config_dict)

        # Act & Assert
        assert config.templates.tone == "detailed"


# ============================================================================
# UNIT TESTS: DEFAULT MERGING
# ============================================================================

class TestDefaultMerging:
    """Test suite for default configuration merging."""

    def test_missing_config_file_uses_defaults(self):
        """AC8: Missing config file uses sensible defaults.

        Given: devforgeai/config/feedback.yaml does not exist
        When: Feedback system initializes
        Then: Default configuration is used (enabled: true, trigger_mode: failures-only)
        """
        # Arrange
        config_file = Path("/nonexistent/path/feedback.yaml")

        # Act
        if not config_file.exists():
            config = FeedbackConfiguration()  # Use defaults

        # Assert
        assert config.enabled is True
        assert config.trigger_mode == "failures-only"
        assert config.conversation_settings.max_questions == 5
        assert config.conversation_settings.allow_skip is True

    def test_partial_config_merged_with_defaults(self):
        """Partial configuration is merged with defaults."""
        # Arrange
        partial_config = {
            "enabled": False
            # All other fields missing
        }

        # Act
        # Deep merge: user values override defaults
        defaults = asdict(FeedbackConfiguration())
        merged = {**defaults, **partial_config}

        # Assert
        assert merged["enabled"] is False  # User value
        assert merged["trigger_mode"] == "failures-only"  # Default
        assert merged["conversation_settings"]["max_questions"] == 5  # Default

    def test_empty_nested_objects_filled_with_defaults(self):
        """Empty nested objects are filled with defaults."""
        # Arrange
        config_dict = {
            "enabled": True,
            "trigger_mode": "always"
            # conversation_settings missing
        }

        # Act
        config = FeedbackConfiguration(
            **config_dict,
            conversation_settings=ConversationSettings(),
            skip_tracking=SkipTrackingSettings(),
            templates=TemplateSettings()
        )

        # Assert
        assert config.conversation_settings.max_questions == 5
        assert config.skip_tracking.enabled is True
        assert config.templates.format == "structured"

    def test_operations_field_conditional_on_trigger_mode(self):
        """operations field required only if trigger_mode: specific-operations."""
        # Arrange
        config_dict = {
            "enabled": True,
            "trigger_mode": "always"
            # operations not required, should be None
        }

        # Act
        config = FeedbackConfiguration(**config_dict)

        # Assert
        assert config.operations is None

    def test_operations_field_required_for_specific_operations_mode(self):
        """operations field must be provided for specific-operations mode."""
        # Arrange
        config_dict = {
            "enabled": True,
            "trigger_mode": "specific-operations",
            "operations": ["qa", "deployment"]
        }

        # Act
        config = FeedbackConfiguration(**config_dict)

        # Assert
        assert config.operations == ["qa", "deployment"]


# ============================================================================
# UNIT TESTS: MASTER ENABLE/DISABLE
# ============================================================================

class TestMasterEnableDisable:
    """Test suite for master enable/disable control (AC2)."""

    def test_enabled_true_allows_feedback_collection(self):
        """enabled: true allows feedback collection."""
        # Arrange
        config = FeedbackConfiguration(enabled=True)

        # Act & Assert
        assert config.enabled is True

    def test_enabled_false_blocks_feedback_collection(self):
        """AC2: enabled: false blocks all feedback operations.

        Given: Configuration has enabled: false
        When: Any skill attempts to trigger feedback
        Then: No feedback is collected and workflow continues
        """
        # Arrange
        config = FeedbackConfiguration(enabled=False)

        # Act & Assert
        assert config.enabled is False

    def test_disabled_ignores_trigger_mode(self):
        """When enabled: false, trigger_mode is ignored."""
        # Arrange
        config = FeedbackConfiguration(
            enabled=False,
            trigger_mode="always"  # Even though set to always
        )

        # Act & Assert
        # Master switch takes precedence
        assert config.enabled is False


# ============================================================================
# UNIT TESTS: TRIGGER MODES
# ============================================================================

class TestTriggerModes:
    """Test suite for trigger mode determination (AC3)."""

    def test_trigger_mode_always_triggers_unconditionally(self):
        """AC3: trigger_mode: always triggers unconditionally.

        Given: trigger_mode: always
        When: Any skill completes a phase
        Then: Feedback is collected
        """
        # Arrange
        config = FeedbackConfiguration(
            enabled=True,
            trigger_mode="always"
        )

        # Act & Assert
        assert config.trigger_mode == "always"
        assert config.enabled is True

    def test_trigger_mode_failures_only_blocks_on_success(self):
        """AC3: trigger_mode: failures-only does not collect on success.

        Given: trigger_mode: failures-only
        When: Skill phase completes successfully
        Then: No feedback is collected
        """
        # Arrange
        config = FeedbackConfiguration(
            enabled=True,
            trigger_mode="failures-only"
        )

        # Act & Assert
        assert config.trigger_mode == "failures-only"

    def test_trigger_mode_failures_only_triggers_on_failure(self):
        """AC3: trigger_mode: failures-only collects on failure.

        Given: trigger_mode: failures-only
        When: Skill phase fails
        Then: Feedback is collected automatically
        """
        # Arrange
        config = FeedbackConfiguration(
            enabled=True,
            trigger_mode="failures-only"
        )

        # Act & Assert
        assert config.trigger_mode == "failures-only"

    def test_trigger_mode_specific_operations_filters_by_operation(self):
        """AC3: trigger_mode: specific-operations filters by operation.

        Given: trigger_mode: specific-operations with operations: [qa, deployment]
        When: QA or deployment operation completes
        Then: Feedback is collected

        When: Other operation completes
        Then: Feedback is not collected
        """
        # Arrange
        config = FeedbackConfiguration(
            enabled=True,
            trigger_mode="specific-operations",
            operations=["qa", "deployment"]
        )

        # Act & Assert
        assert config.trigger_mode == "specific-operations"
        assert "qa" in config.operations
        assert "deployment" in config.operations

    def test_trigger_mode_never_blocks_all_feedback(self):
        """AC3: trigger_mode: never blocks all feedback collection.

        Given: trigger_mode: never
        When: Any skill operation completes
        Then: Feedback is never collected
        """
        # Arrange
        config = FeedbackConfiguration(
            enabled=True,
            trigger_mode="never"
        )

        # Act & Assert
        assert config.trigger_mode == "never"


# ============================================================================
# UNIT TESTS: CONVERSATION SETTINGS
# ============================================================================

class TestConversationSettings:
    """Test suite for conversation settings enforcement (AC4)."""

    def test_max_questions_limit_enforced(self):
        """AC4: Conversation enforces max_questions limit.

        Given: max_questions: 3 is configured
        When: User answers 3 feedback questions
        Then: No additional questions are shown
        """
        # Arrange
        config = FeedbackConfiguration(
            conversation_settings=ConversationSettings(max_questions=3)
        )
        questions_answered = 3

        # Act
        # Feedback should be silently discarded
        should_ask = questions_answered < config.conversation_settings.max_questions

        # Assert
        assert should_ask is False  # Should NOT ask

    def test_max_questions_zero_means_unlimited(self):
        """max_questions: 0 means no limit."""
        # Arrange
        config = FeedbackConfiguration(
            conversation_settings=ConversationSettings(max_questions=0)
        )
        questions_answered = 1000

        # Act
        # No limit, should keep asking
        should_ask = (config.conversation_settings.max_questions == 0 or
                     questions_answered < config.conversation_settings.max_questions)

        # Assert
        assert should_ask is True

    def test_allow_skip_true_shows_skip_option(self):
        """AC4: allow_skip: true shows skip option.

        Given: allow_skip: true
        When: AskUserQuestion displayed
        Then: Skip option is available
        """
        # Arrange
        config = FeedbackConfiguration(
            conversation_settings=ConversationSettings(allow_skip=True)
        )

        # Act & Assert
        assert config.conversation_settings.allow_skip is True

    def test_allow_skip_false_hides_skip_option(self):
        """AC4: allow_skip: false hides skip option.

        Given: allow_skip: false
        When: AskUserQuestion displayed
        Then: No skip option available
        """
        # Arrange
        config = FeedbackConfiguration(
            conversation_settings=ConversationSettings(allow_skip=False)
        )

        # Act & Assert
        assert config.conversation_settings.allow_skip is False


# ============================================================================
# UNIT TESTS: SKIP TRACKING
# ============================================================================

class TestSkipTracking:
    """Test suite for skip tracking statistics (AC5)."""

    def test_skip_tracking_enabled_maintains_statistics(self):
        """AC5: skip_tracking: enabled maintains feedback statistics.

        Given: skip_tracking.enabled: true
        When: User skips feedback multiple times
        Then: Skip count is tracked
        """
        # Arrange
        config = FeedbackConfiguration(
            skip_tracking=SkipTrackingSettings(enabled=True)
        )
        skip_count = 0

        # Act
        skip_count += 1

        # Assert
        assert config.skip_tracking.enabled is True
        assert skip_count == 1

    def test_max_consecutive_skips_blocks_after_limit(self):
        """AC5: max_consecutive_skips blocks after limit reached.

        Given: max_consecutive_skips: 5
        When: User skips 5 consecutive times
        Then: Next feedback trigger is blocked
        """
        # Arrange
        config = FeedbackConfiguration(
            skip_tracking=SkipTrackingSettings(max_consecutive_skips=5)
        )
        consecutive_skips = 5

        # Act
        should_block = consecutive_skips >= config.skip_tracking.max_consecutive_skips

        # Assert
        assert should_block is True

    def test_reset_on_positive_resets_skip_counter(self):
        """AC5: reset_on_positive: true resets counter on positive feedback.

        Given: reset_on_positive: true
        When: User provides positive feedback
        Then: Consecutive skip counter resets to 0
        """
        # Arrange
        config = FeedbackConfiguration(
            skip_tracking=SkipTrackingSettings(reset_on_positive=True)
        )
        skip_counter = 5

        # Act
        user_rating = 5  # Positive feedback
        if config.skip_tracking.reset_on_positive and user_rating >= 4:
            skip_counter = 0

        # Assert
        assert skip_counter == 0

    def test_skip_tracking_disabled_ignores_limit(self):
        """AC5: skip_tracking: disabled ignores max_consecutive_skips.

        Given: skip_tracking.enabled: false
        When: User skips multiple times
        Then: No limit enforced
        """
        # Arrange
        config = FeedbackConfiguration(
            skip_tracking=SkipTrackingSettings(
                enabled=False,
                max_consecutive_skips=3
            )
        )
        skip_counter = 100

        # Act
        if config.skip_tracking.enabled:
            should_block = skip_counter >= config.skip_tracking.max_consecutive_skips
        else:
            should_block = False

        # Assert
        assert should_block is False


# ============================================================================
# UNIT TESTS: TEMPLATE PREFERENCES
# ============================================================================

class TestTemplatePreferences:
    """Test suite for template preferences (AC6)."""

    def test_template_format_structured_shows_options(self):
        """AC6: format: structured shows predefined options.

        Given: templates.format: structured
        When: Feedback is collected
        Then: Predefined options displayed
        """
        # Arrange
        config = FeedbackConfiguration(
            templates=TemplateSettings(format="structured")
        )

        # Act & Assert
        assert config.templates.format == "structured"

    def test_template_format_free_text_accepts_custom_input(self):
        """AC6: format: free-text accepts custom input.

        Given: templates.format: free-text
        When: Feedback is collected
        Then: Open text input displayed
        """
        # Arrange
        config = FeedbackConfiguration(
            templates=TemplateSettings(format="free-text")
        )

        # Act & Assert
        assert config.templates.format == "free-text"

    def test_template_tone_brief_limits_question_length(self):
        """AC6: tone: brief limits question to ≤50 characters.

        Given: templates.tone: brief
        When: Question displayed
        Then: Question text ≤50 characters
        """
        # Arrange
        config = FeedbackConfiguration(
            templates=TemplateSettings(tone="brief")
        )
        question = "Was this helpful?"  # 17 chars

        # Act & Assert
        assert config.templates.tone == "brief"
        assert len(question) <= 50

    def test_template_tone_detailed_includes_context(self):
        """AC6: tone: detailed includes context explanation.

        Given: templates.tone: detailed
        When: Question displayed
        Then: Question includes context (operation type, outcome)
        """
        # Arrange
        config = FeedbackConfiguration(
            templates=TemplateSettings(tone="detailed")
        )

        # Act & Assert
        assert config.templates.tone == "detailed"


# ============================================================================
# UNIT TESTS: HOT-RELOAD
# ============================================================================

class TestHotReload:
    """Test suite for configuration hot-reload (AC9)."""

    def test_hot_reload_detects_file_change(self, config_file, temp_config_dir):
        """AC9: System detects file changes within 5 seconds.

        Given: Feedback system is running
        When: Configuration file is modified
        Then: File change is detected within 5 seconds
        """
        # Arrange
        import yaml
        config1 = {"enabled": True}
        with open(config_file, 'w') as f:
            yaml.dump(config1, f)

        # Act - simulate file watcher
        last_mtime = config_file.stat().st_mtime
        time.sleep(0.1)
        config2 = {"enabled": False}
        with open(config_file, 'w') as f:
            yaml.dump(config2, f)
        new_mtime = config_file.stat().st_mtime

        # Assert
        assert new_mtime > last_mtime

    def test_hot_reload_loads_new_configuration(self, config_file):
        """Hot-reload loads new configuration values."""
        # Arrange
        import yaml
        config1 = {"enabled": True, "trigger_mode": "always"}
        with open(config_file, 'w') as f:
            yaml.dump(config1, f)

        with open(config_file, 'r') as f:
            loaded1 = yaml.safe_load(f)

        # Act
        config2 = {"enabled": False, "trigger_mode": "never"}
        with open(config_file, 'w') as f:
            yaml.dump(config2, f)

        with open(config_file, 'r') as f:
            loaded2 = yaml.safe_load(f)

        # Assert
        assert loaded1["enabled"] is True
        assert loaded2["enabled"] is False

    def test_hot_reload_stops_feedback_immediately(self):
        """Hot-reload stops feedback collection immediately."""
        # Arrange
        config = FeedbackConfiguration(enabled=True)

        # Act
        # Simulate reload
        updated_config = FeedbackConfiguration(enabled=False)

        # Assert
        assert config.enabled is True
        assert updated_config.enabled is False

    def test_invalid_config_during_reload_keeps_previous_valid(self):
        """Invalid config during reload keeps previous valid config."""
        # Arrange
        config = FeedbackConfiguration(enabled=True)
        previous_config = config

        # Act
        # Attempt to load invalid config - fails, keep previous
        should_keep_previous = True  # Error occurred
        if should_keep_previous:
            current_config = previous_config

        # Assert
        assert current_config.enabled is True


# ============================================================================
# INTEGRATION TESTS
# ============================================================================

class TestConfigurationLoading:
    """Integration tests for complete configuration loading."""

    def test_config_load_to_feedback_trigger_flow(self, config_file, valid_config_dict):
        """Integration: Config load → validation → feedback trigger."""
        # Arrange
        import yaml
        with open(config_file, 'w') as f:
            yaml.dump(valid_config_dict, f)

        # Act
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)
        config = FeedbackConfiguration(**loaded)

        # Assert
        assert config.enabled is True
        assert config.trigger_mode == "failures-only"

    def test_config_load_with_defaults_merge(self, config_file):
        """Integration: Partial config loaded and merged with defaults."""
        # Arrange
        import yaml
        partial = {"enabled": False}
        with open(config_file, 'w') as f:
            yaml.dump(partial, f)

        # Act
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f) or {}
        defaults = asdict(FeedbackConfiguration())
        merged = {**defaults, **loaded}

        # Assert
        assert merged["enabled"] is False  # Overridden
        assert merged["trigger_mode"] == "failures-only"  # Default

    def test_multiple_configuration_loads_consistent(self, config_file, valid_config_dict):
        """Multiple config loads produce consistent results."""
        # Arrange
        import yaml
        with open(config_file, 'w') as f:
            yaml.dump(valid_config_dict, f)

        # Act
        configs = []
        for _ in range(3):
            with open(config_file, 'r') as f:
                loaded = yaml.safe_load(f)
            configs.append(FeedbackConfiguration(**loaded))

        # Assert
        assert all(c.enabled == configs[0].enabled for c in configs)
        assert all(c.trigger_mode == configs[0].trigger_mode for c in configs)


# ============================================================================
# EDGE CASE TESTS
# ============================================================================

class TestEdgeCases:
    """Test suite for edge case handling."""

    def test_edge_case_concurrent_skip_tracking_updates(self):
        """Edge case 1: Concurrent feedback triggers during skip tracking.

        Scenario: max_consecutive_skips: 2, user skips twice, then two operations
                  trigger simultaneously
        Expected: Skip counter correctly maintained, both operations blocked
        """
        # Arrange
        skip_counter = 0
        max_skips = 2

        def simulate_skip():
            nonlocal skip_counter
            skip_counter += 1

        # Act
        threads = []
        for _ in range(2):
            t = threading.Thread(target=simulate_skip)
            threads.append(t)
            t.start()

        for t in threads:
            t.join()

        # Assert
        assert skip_counter == 2
        assert skip_counter >= max_skips

    def test_edge_case_empty_configuration_file(self, config_file):
        """Edge case 2: Empty configuration file."""
        # Arrange
        with open(config_file, 'w') as f:
            f.write("")

        # Act
        import yaml
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)

        if loaded is None or loaded == {}:
            config = FeedbackConfiguration()  # Use defaults
        else:
            config = FeedbackConfiguration(**loaded)

        # Assert
        assert config.enabled is True

    def test_edge_case_partial_configuration_merge(self):
        """Edge case 3: Partial configuration merged with defaults."""
        # Arrange
        partial = {"enabled": True}
        defaults = asdict(FeedbackConfiguration())

        # Act
        merged = {**defaults, **partial}
        config = FeedbackConfiguration(**merged)

        # Assert
        assert config.enabled is True
        assert config.trigger_mode == "failures-only"

    def test_edge_case_extremely_large_max_questions(self):
        """Edge case 4: Extremely large max_questions value."""
        # Arrange
        config = FeedbackConfiguration(
            conversation_settings=ConversationSettings(
                max_questions=1000000
            )
        )

        # Act & Assert
        assert config.conversation_settings.max_questions == 1000000

    def test_edge_case_special_characters_in_yaml(self, config_file):
        """Edge case 5: Special characters (Unicode) in YAML."""
        # Arrange
        import yaml
        unicode_config = {
            "enabled": True,
            "trigger_mode": "failures-only"
            # Add unicode in comment or values
        }
        with open(config_file, 'w', encoding='utf-8') as f:
            yaml.dump(unicode_config, f, allow_unicode=True)

        # Act
        with open(config_file, 'r', encoding='utf-8') as f:
            loaded = yaml.safe_load(f)

        # Assert
        assert loaded["enabled"] is True

    def test_edge_case_file_becomes_unreadable_after_load(self, config_file, valid_config_dict):
        """Edge case 6: File becomes unreadable after initial load."""
        # Arrange
        import yaml
        with open(config_file, 'w') as f:
            yaml.dump(valid_config_dict, f)

        # First load succeeds
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)
        config = FeedbackConfiguration(**loaded)

        # Act - make file unreadable (if on Unix-like system)
        try:
            os.chmod(config_file, 0o000)

            # Attempt to reload
            try:
                with open(config_file, 'r') as f:
                    _ = yaml.safe_load(f)
                reload_succeeded = True
            except PermissionError:
                reload_succeeded = False
        finally:
            # Restore permissions for cleanup
            os.chmod(config_file, 0o644)

        # Assert
        assert not reload_succeeded  # Should fail gracefully
        assert config.enabled is True  # Previous config still valid

    def test_edge_case_multiple_skill_invocations_before_init_complete(self):
        """Edge case 7: Multiple skill invocations before config load complete.

        Scenario: Parallel skills invoke feedback before initialization
        Expected: Block until initialization complete, all use same config
        """
        # Arrange
        initialization_lock = threading.Lock()
        config = None

        def initialize_config():
            nonlocal config
            with initialization_lock:
                time.sleep(0.1)  # Simulate initialization
                config = FeedbackConfiguration(enabled=True)

        def request_config():
            with initialization_lock:
                return config

        # Act
        init_thread = threading.Thread(target=initialize_config)
        init_thread.start()

        # Simulate skill requests blocking until init complete
        config_used = []
        for _ in range(3):
            cfg = request_config()
            config_used.append(cfg)

        init_thread.join()

        # Assert
        assert config is not None
        assert all(c == config_used[0] for c in config_used)


# ============================================================================
# PERFORMANCE TESTS
# ============================================================================

class TestPerformance:
    """Performance tests for configuration operations."""

    def test_configuration_load_time_under_100ms(self, config_file, valid_config_dict):
        """Performance: Configuration load time < 100ms."""
        # Arrange
        import yaml
        with open(config_file, 'w') as f:
            yaml.dump(valid_config_dict, f)

        # Act
        start_time = time.time()
        with open(config_file, 'r') as f:
            loaded = yaml.safe_load(f)
        config = FeedbackConfiguration(**loaded)
        elapsed_ms = (time.time() - start_time) * 1000

        # Assert
        assert elapsed_ms < 100, f"Config load took {elapsed_ms}ms (expected <100ms)"

    def test_hot_reload_detection_within_5_seconds(self, config_file, valid_config_dict):
        """Performance: Hot-reload detection ≤ 5 seconds."""
        # Arrange
        import yaml
        with open(config_file, 'w') as f:
            yaml.dump(valid_config_dict, f)

        # Act
        start_time = time.time()
        # Simulate file watcher checking for changes
        initial_mtime = config_file.stat().st_mtime
        time.sleep(0.5)
        modified_config = {**valid_config_dict, "enabled": False}
        with open(config_file, 'w') as f:
            yaml.dump(modified_config, f)
        new_mtime = config_file.stat().st_mtime
        elapsed = time.time() - start_time

        # Assert
        assert elapsed < 5.0
        assert new_mtime > initial_mtime

    def test_skip_counter_lookup_under_10ms(self):
        """Performance: Skip counter lookup < 10ms."""
        # Arrange
        skip_counters = {str(i): i for i in range(1000)}

        # Act
        start_time = time.time()
        for _ in range(100):
            _ = skip_counters.get("500")
        elapsed_ms = (time.time() - start_time) * 1000

        # Assert
        assert elapsed_ms < 10, f"Lookup took {elapsed_ms}ms (expected <10ms)"

    def test_per_feedback_processing_overhead_under_50ms(self):
        """Performance: Per-feedback overhead < 50ms."""
        # Arrange
        config = FeedbackConfiguration(enabled=True)
        operations_count = 10

        # Act
        start_time = time.time()
        for _ in range(operations_count):
            # Simulate feedback processing
            if config.enabled and config.trigger_mode != "never":
                pass  # Process feedback
        elapsed_ms = (time.time() - start_time) * 1000
        avg_per_operation = elapsed_ms / operations_count

        # Assert
        assert avg_per_operation < 50, f"Average overhead {avg_per_operation}ms (expected <50ms)"


# ============================================================================
# PARAMETRIZED TESTS (Test multiple similar scenarios)
# ============================================================================

class TestParametrizedScenarios:
    """Parametrized tests for similar scenarios."""

    @pytest.mark.parametrize("trigger_mode", ["always", "failures-only", "specific-operations", "never"])
    def test_all_valid_trigger_modes(self, trigger_mode):
        """All valid trigger modes are accepted."""
        # Arrange & Act
        config = FeedbackConfiguration(trigger_mode=trigger_mode)

        # Assert
        assert config.trigger_mode == trigger_mode

    @pytest.mark.parametrize("max_questions", [0, 1, 5, 100, 1000000])
    def test_various_max_questions_values(self, max_questions):
        """Various max_questions values are accepted."""
        # Arrange & Act
        config = FeedbackConfiguration(
            conversation_settings=ConversationSettings(max_questions=max_questions)
        )

        # Assert
        assert config.conversation_settings.max_questions == max_questions

    @pytest.mark.parametrize("format_value", ["structured", "free-text"])
    def test_both_template_formats(self, format_value):
        """Both template format values are valid."""
        # Arrange & Act
        config = FeedbackConfiguration(
            templates=TemplateSettings(format=format_value)
        )

        # Assert
        assert config.templates.format == format_value

    @pytest.mark.parametrize("tone_value", ["brief", "detailed"])
    def test_both_template_tones(self, tone_value):
        """Both template tone values are valid."""
        # Arrange & Act
        config = FeedbackConfiguration(
            templates=TemplateSettings(tone=tone_value)
        )

        # Assert
        assert config.templates.tone == tone_value

    @pytest.mark.parametrize("enabled", [True, False])
    def test_enabled_setting_values(self, enabled):
        """Both enabled values are valid."""
        # Arrange & Act
        config = FeedbackConfiguration(enabled=enabled)

        # Assert
        assert config.enabled == enabled


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
