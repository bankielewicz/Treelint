"""
Tests for config_manager.py module.

Validates configuration loading, YAML parsing, default merging,
validation, hot-reload integration, and singleton pattern.
Target: 95% coverage of 161 statements.
"""

import pytest
import yaml
import threading
import time
from pathlib import Path
from unittest.mock import Mock, patch, MagicMock
from devforgeai_cli.feedback.config_manager import (
    ConfigurationManager,
    get_config_manager,
    reset_config_manager
)
from devforgeai_cli.feedback.config_models import FeedbackConfiguration, ConversationSettings, SkipTrackingSettings, TemplateSettings


@pytest.fixture
def temp_config_file(tmp_path):
    """Provide a temporary config file path."""
    return tmp_path / "feedback.yaml"


@pytest.fixture
def temp_logs_dir(tmp_path):
    """Provide a temporary logs directory."""
    return tmp_path / "logs"


@pytest.fixture
def manager(temp_config_file, temp_logs_dir):
    """Provide a fresh ConfigurationManager instance."""
    return ConfigurationManager(
        config_file_path=temp_config_file,
        logs_dir=temp_logs_dir,
        enable_hot_reload=False  # Disable hot-reload for most tests
    )


@pytest.fixture
def valid_yaml_content():
    """Provide valid YAML configuration content."""
    return """
enabled: true
trigger_mode: always
operations:
  - qa
  - dev
conversation_settings:
  max_questions: 10
  allow_skip: false
skip_tracking:
  enabled: true
  max_consecutive_skips: 5
  reset_on_positive: true
templates:
  format: free-text
  tone: detailed
"""


class TestConfigurationManagerInitialization:
    """Tests for ConfigurationManager initialization."""

    def test_init_with_custom_paths(self, temp_config_file, temp_logs_dir):
        """ConfigurationManager accepts custom paths."""
        manager = ConfigurationManager(
            config_file_path=temp_config_file,
            logs_dir=temp_logs_dir,
            enable_hot_reload=False
        )
        assert manager.config_file_path == temp_config_file
        assert manager.logs_dir == temp_logs_dir

    def test_init_with_default_paths(self):
        """ConfigurationManager uses defaults when paths not provided."""
        manager = ConfigurationManager(enable_hot_reload=False)
        assert manager.config_file_path == Path("devforgeai/config/feedback.yaml")
        assert manager.logs_dir == Path("devforgeai/logs")

    def test_init_creates_logs_directory(self, temp_config_file, temp_logs_dir):
        """Initialization creates logs directory if it doesn't exist."""
        manager = ConfigurationManager(
            config_file_path=temp_config_file,
            logs_dir=temp_logs_dir,
            enable_hot_reload=False
        )
        assert temp_logs_dir.exists()

    def test_init_loads_configuration(self, manager):
        """Initialization loads configuration (defaults if file missing)."""
        assert manager._current_config is not None
        assert isinstance(manager._current_config, FeedbackConfiguration)

    def test_init_creates_skip_tracker(self, manager):
        """Initialization creates skip tracker instance."""
        assert manager._skip_tracker is not None

    def test_init_hot_reload_disabled(self, manager):
        """Hot-reload is disabled when enable_hot_reload=False."""
        assert manager._hot_reload_manager is None


class TestYAMLLoading:
    """Tests for YAML file loading and parsing."""

    def test_load_yaml_file_not_exists(self, manager):
        """_load_yaml_file returns None when file doesn't exist."""
        result = manager._load_yaml_file()
        assert result is None

    def test_load_yaml_file_valid(self, temp_config_file, manager, valid_yaml_content):
        """_load_yaml_file parses valid YAML correctly."""
        temp_config_file.write_text(valid_yaml_content)
        result = manager._load_yaml_file()

        assert isinstance(result, dict)
        assert result["enabled"] is True
        assert result["trigger_mode"] == "always"

    def test_load_yaml_file_empty(self, temp_config_file, manager):
        """_load_yaml_file returns empty dict for empty file."""
        temp_config_file.write_text("")
        result = manager._load_yaml_file()
        assert result == {}

    def test_load_yaml_file_invalid_syntax(self, temp_config_file, manager):
        """_load_yaml_file raises YAMLError for invalid syntax."""
        temp_config_file.write_text("invalid: yaml: syntax: [")
        with pytest.raises(yaml.YAMLError):
            manager._load_yaml_file()

    def test_load_yaml_file_not_dict(self, temp_config_file, manager):
        """_load_yaml_file returns {} when YAML is not a dictionary."""
        temp_config_file.write_text("- list\n- items")
        result = manager._load_yaml_file()
        assert result == {}

    def test_load_yaml_file_permission_error(self, temp_config_file, manager):
        """_load_yaml_file raises IOError for permission errors."""
        temp_config_file.write_text("enabled: true")
        temp_config_file.chmod(0o000)  # Remove all permissions

        try:
            with pytest.raises((IOError, OSError)):
                manager._load_yaml_file()
        finally:
            temp_config_file.chmod(0o644)  # Restore permissions


class TestConfigurationMerging:
    """Tests for configuration merging with defaults."""

    def test_merge_with_defaults_none_config(self, manager):
        """_merge_with_defaults returns full defaults when config is None."""
        result = manager._merge_with_defaults(None)
        assert result["enabled"] is True
        assert result["trigger_mode"] == "failures-only"
        assert result["conversation_settings"]["max_questions"] == 5

    def test_merge_with_defaults_partial_config(self, manager):
        """_merge_with_defaults merges partial config with defaults."""
        partial = {"enabled": False}
        result = manager._merge_with_defaults(partial)

        assert result["enabled"] is False  # Override
        assert result["trigger_mode"] == "failures-only"  # Default
        assert result["conversation_settings"]["max_questions"] == 5  # Default

    def test_merge_with_defaults_override_trigger_mode(self, manager):
        """_merge_with_defaults allows overriding trigger_mode."""
        config = {"trigger_mode": "always"}
        result = manager._merge_with_defaults(config)
        assert result["trigger_mode"] == "always"

    def test_merge_with_defaults_nested_merge_conversation(self, manager):
        """_merge_with_defaults deeply merges conversation_settings."""
        config = {
            "conversation_settings": {
                "max_questions": 10
                # allow_skip missing - should get default
            }
        }
        result = manager._merge_with_defaults(config)

        assert result["conversation_settings"]["max_questions"] == 10  # Override
        assert result["conversation_settings"]["allow_skip"] is True  # Default

    def test_merge_with_defaults_nested_merge_skip_tracking(self, manager):
        """_merge_with_defaults deeply merges skip_tracking."""
        config = {
            "skip_tracking": {
                "enabled": False
                # Other fields missing - should get defaults
            }
        }
        result = manager._merge_with_defaults(config)

        assert result["skip_tracking"]["enabled"] is False  # Override
        assert result["skip_tracking"]["max_consecutive_skips"] == 3  # Default
        assert result["skip_tracking"]["reset_on_positive"] is True  # Default

    def test_merge_with_defaults_nested_merge_templates(self, manager):
        """_merge_with_defaults deeply merges templates."""
        config = {
            "templates": {
                "format": "free-text"
                # tone missing - should get default
            }
        }
        result = manager._merge_with_defaults(config)

        assert result["templates"]["format"] == "free-text"  # Override
        assert result["templates"]["tone"] == "brief"  # Default

    def test_merge_with_defaults_full_custom_config(self, manager):
        """_merge_with_defaults handles fully custom configuration."""
        config = {
            "enabled": False,
            "trigger_mode": "never",
            "operations": None,
            "conversation_settings": {
                "max_questions": 0,
                "allow_skip": False
            },
            "skip_tracking": {
                "enabled": False,
                "max_consecutive_skips": 10,
                "reset_on_positive": False
            },
            "templates": {
                "format": "free-text",
                "tone": "detailed"
            }
        }
        result = manager._merge_with_defaults(config)

        assert result["enabled"] is False
        assert result["trigger_mode"] == "never"
        assert result["conversation_settings"]["max_questions"] == 0
        assert result["templates"]["format"] == "free-text"


class TestConfigurationValidation:
    """Tests for configuration validation."""

    def test_dict_to_configuration_valid(self, manager):
        """_dict_to_configuration creates valid FeedbackConfiguration."""
        config_dict = {
            "enabled": True,
            "trigger_mode": "always",
            "operations": ["qa"],
            "conversation_settings": {"max_questions": 10, "allow_skip": True},
            "skip_tracking": {"enabled": True, "max_consecutive_skips": 5, "reset_on_positive": True},
            "templates": {"format": "structured", "tone": "brief"}
        }
        config = manager._dict_to_configuration(config_dict)

        assert isinstance(config, FeedbackConfiguration)
        assert config.enabled is True
        assert config.trigger_mode == "always"

    def test_dict_to_configuration_invalid_trigger_mode(self, manager):
        """_dict_to_configuration raises ValueError for invalid trigger_mode."""
        config_dict = {"trigger_mode": "invalid-mode"}
        with pytest.raises(ValueError) as exc_info:
            manager._dict_to_configuration(config_dict)
        assert "Invalid trigger_mode" in str(exc_info.value)

    def test_dict_to_configuration_missing_operations_for_specific(self, manager):
        """_dict_to_configuration raises ValueError when specific-operations missing operations."""
        config_dict = {
            "trigger_mode": "specific-operations",
            "operations": None
        }
        with pytest.raises(ValueError) as exc_info:
            manager._dict_to_configuration(config_dict)
        assert "operations list must be provided" in str(exc_info.value)


class TestConfigurationLoading:
    """Tests for load_configuration method (AC-1, AC-8)."""

    def test_load_configuration_file_not_exists(self, manager):
        """load_configuration returns defaults when file doesn't exist (AC-8)."""
        config = manager.load_configuration()

        assert isinstance(config, FeedbackConfiguration)
        assert config.enabled is True
        assert config.trigger_mode == "failures-only"
        assert config.conversation_settings.max_questions == 5

    def test_load_configuration_valid_file(self, temp_config_file, temp_logs_dir, valid_yaml_content):
        """load_configuration parses valid YAML file (AC-1)."""
        temp_config_file.write_text(valid_yaml_content)
        manager = ConfigurationManager(
            config_file_path=temp_config_file,
            logs_dir=temp_logs_dir,
            enable_hot_reload=False
        )

        config = manager.load_configuration()
        assert config.enabled is True
        assert config.trigger_mode == "always"
        assert config.conversation_settings.max_questions == 10

    def test_load_configuration_partial_file(self, temp_config_file, temp_logs_dir):
        """load_configuration merges partial config with defaults (AC-8)."""
        temp_config_file.write_text("enabled: false\n")
        manager = ConfigurationManager(
            config_file_path=temp_config_file,
            logs_dir=temp_logs_dir,
            enable_hot_reload=False
        )

        config = manager.load_configuration()
        assert config.enabled is False  # From file
        assert config.trigger_mode == "failures-only"  # Default
        assert config.conversation_settings.max_questions == 5  # Default

    def test_load_configuration_invalid_yaml_returns_defaults(self, temp_config_file, temp_logs_dir):
        """load_configuration returns defaults when YAML invalid (error handling)."""
        temp_config_file.write_text("invalid: [yaml: syntax")
        manager = ConfigurationManager(
            config_file_path=temp_config_file,
            logs_dir=temp_logs_dir,
            enable_hot_reload=False
        )

        # Should catch exception and return defaults
        config = manager.load_configuration()
        assert isinstance(config, FeedbackConfiguration)
        assert config.enabled is True  # Default
        assert config.trigger_mode == "failures-only"  # Default


class TestGetterMethods:
    """Tests for configuration getter methods."""

    def test_get_configuration(self, manager):
        """get_configuration returns current configuration."""
        config = manager.get_configuration()
        assert isinstance(config, FeedbackConfiguration)

    def test_is_enabled_true(self, temp_config_file, temp_logs_dir):
        """is_enabled returns True when enabled in config (AC-2)."""
        temp_config_file.write_text("enabled: true\n")
        manager = ConfigurationManager(
            config_file_path=temp_config_file,
            logs_dir=temp_logs_dir,
            enable_hot_reload=False
        )
        assert manager.is_enabled() is True

    def test_is_enabled_false(self, temp_config_file, temp_logs_dir):
        """is_enabled returns False when disabled in config (AC-2)."""
        temp_config_file.write_text("enabled: false\n")
        manager = ConfigurationManager(
            config_file_path=temp_config_file,
            logs_dir=temp_logs_dir,
            enable_hot_reload=False
        )
        assert manager.is_enabled() is False

    def test_get_trigger_mode(self, temp_config_file, temp_logs_dir):
        """get_trigger_mode returns configured trigger mode (AC-3)."""
        temp_config_file.write_text("trigger_mode: always\n")
        manager = ConfigurationManager(
            config_file_path=temp_config_file,
            logs_dir=temp_logs_dir,
            enable_hot_reload=False
        )
        assert manager.get_trigger_mode() == "always"

    def test_get_operations(self, temp_config_file, temp_logs_dir):
        """get_operations returns configured operations list."""
        temp_config_file.write_text("operations:\n  - qa\n  - dev\n")
        manager = ConfigurationManager(
            config_file_path=temp_config_file,
            logs_dir=temp_logs_dir,
            enable_hot_reload=False
        )
        assert manager.get_operations() == ["qa", "dev"]

    def test_get_conversation_settings(self, manager):
        """get_conversation_settings returns ConversationSettings object (AC-4)."""
        settings = manager.get_conversation_settings()
        assert isinstance(settings, ConversationSettings)
        assert settings.max_questions == 5

    def test_get_skip_tracking_settings(self, manager):
        """get_skip_tracking_settings returns SkipTrackingSettings object (AC-5)."""
        settings = manager.get_skip_tracking_settings()
        assert isinstance(settings, SkipTrackingSettings)
        assert settings.max_consecutive_skips == 3

    def test_get_template_settings(self, manager):
        """get_template_settings returns TemplateSettings object (AC-6)."""
        settings = manager.get_template_settings()
        assert isinstance(settings, TemplateSettings)
        assert settings.format == "structured"

    def test_get_skip_tracker(self, manager):
        """get_skip_tracker returns SkipTracker instance."""
        tracker = manager.get_skip_tracker()
        assert tracker is not None


class TestHotReloadIntegration:
    """Tests for hot-reload manager integration (AC-9)."""

    def test_init_hot_reload_enabled(self, temp_config_file, temp_logs_dir):
        """Hot-reload manager is created when enable_hot_reload=True."""
        manager = ConfigurationManager(
            config_file_path=temp_config_file,
            logs_dir=temp_logs_dir,
            enable_hot_reload=True
        )
        assert manager._hot_reload_manager is not None
        # Cleanup
        manager.shutdown()

    def test_is_hot_reload_enabled_true(self, temp_config_file, temp_logs_dir):
        """is_hot_reload_enabled returns True when hot-reload active."""
        manager = ConfigurationManager(
            config_file_path=temp_config_file,
            logs_dir=temp_logs_dir,
            enable_hot_reload=True
        )
        # Give watcher time to start
        time.sleep(0.1)
        assert manager.is_hot_reload_enabled() is True
        manager.shutdown()

    def test_is_hot_reload_enabled_false(self, manager):
        """is_hot_reload_enabled returns False when not enabled."""
        assert manager.is_hot_reload_enabled() is False

    def test_start_hot_reload(self, temp_config_file, temp_logs_dir):
        """start_hot_reload starts the watcher."""
        manager = ConfigurationManager(
            config_file_path=temp_config_file,
            logs_dir=temp_logs_dir,
            enable_hot_reload=True
        )
        manager.stop_hot_reload()
        assert manager.is_hot_reload_enabled() is False

        manager.start_hot_reload()
        time.sleep(0.1)
        assert manager.is_hot_reload_enabled() is True
        manager.shutdown()

    def test_stop_hot_reload(self, temp_config_file, temp_logs_dir):
        """stop_hot_reload stops the watcher."""
        manager = ConfigurationManager(
            config_file_path=temp_config_file,
            logs_dir=temp_logs_dir,
            enable_hot_reload=True
        )
        time.sleep(0.1)
        manager.stop_hot_reload()
        assert manager.is_hot_reload_enabled() is False

    def test_shutdown(self, temp_config_file, temp_logs_dir):
        """shutdown stops hot-reload."""
        manager = ConfigurationManager(
            config_file_path=temp_config_file,
            logs_dir=temp_logs_dir,
            enable_hot_reload=True
        )
        time.sleep(0.1)
        manager.shutdown()
        assert manager.is_hot_reload_enabled() is False


class TestErrorLogging:
    """Tests for error logging functionality."""

    def test_log_error_creates_log_file(self, manager, temp_logs_dir):
        """_log_error creates error log file."""
        manager._log_error("Test error message")
        error_log = temp_logs_dir / "config-errors.log"
        assert error_log.exists()

    def test_log_error_contains_timestamp(self, manager, temp_logs_dir):
        """Error log entries contain timestamp."""
        manager._log_error("Test error")
        error_log = temp_logs_dir / "config-errors.log"

        with open(error_log, 'r') as f:
            content = f.read()
            assert "T" in content  # ISO timestamp marker
            assert "Test error" in content

    def test_log_error_appends_to_file(self, manager, temp_logs_dir):
        """_log_error appends multiple errors to log."""
        manager._log_error("Error 1")
        manager._log_error("Error 2")

        error_log = temp_logs_dir / "config-errors.log"
        with open(error_log, 'r') as f:
            content = f.read()
            assert content.count("Error 1") == 1
            assert content.count("Error 2") == 1


class TestSingletonPattern:
    """Tests for global singleton pattern."""

    def test_get_config_manager_creates_instance(self):
        """get_config_manager creates ConfigurationManager instance."""
        reset_config_manager()
        manager = get_config_manager()
        assert isinstance(manager, ConfigurationManager)
        manager.shutdown()
        reset_config_manager()

    def test_get_config_manager_returns_same_instance(self):
        """get_config_manager returns same instance on multiple calls."""
        reset_config_manager()
        manager1 = get_config_manager()
        manager2 = get_config_manager()
        assert manager1 is manager2
        manager1.shutdown()
        reset_config_manager()

    def test_reset_config_manager_clears_global(self):
        """reset_config_manager clears global instance."""
        manager1 = get_config_manager()
        reset_config_manager()
        manager2 = get_config_manager()
        assert manager1 is not manager2
        manager2.shutdown()
        reset_config_manager()

    def test_get_config_manager_thread_safe(self):
        """get_config_manager is thread-safe (no duplicate instances)."""
        reset_config_manager()
        managers = []

        def get_manager():
            managers.append(get_config_manager())

        threads = [threading.Thread(target=get_manager) for _ in range(10)]
        for t in threads:
            t.start()
        for t in threads:
            t.join()

        # All threads should get the same instance
        assert all(m is managers[0] for m in managers)
        managers[0].shutdown()
        reset_config_manager()


class TestConfigurationUpdate:
    """Tests for configuration update functionality."""

    def test_update_configuration(self, manager):
        """update_configuration updates current config."""
        new_config = FeedbackConfiguration(enabled=False, trigger_mode="never")
        manager.update_configuration(new_config)

        assert manager.get_configuration().enabled is False
        assert manager.get_configuration().trigger_mode == "never"

    def test_update_configuration_thread_safe(self, manager):
        """update_configuration is thread-safe."""
        configs = [
            FeedbackConfiguration(enabled=True, trigger_mode="always"),
            FeedbackConfiguration(enabled=False, trigger_mode="never")
        ]

        def update_config(config):
            manager.update_configuration(config)
            time.sleep(0.001)

        threads = [threading.Thread(target=update_config, args=(configs[i % 2],)) for i in range(20)]
        for t in threads:
            t.start()
        for t in threads:
            t.join()

        # Should have one of the configs (not corrupted)
        final_config = manager.get_configuration()
        assert final_config.trigger_mode in ["always", "never"]
