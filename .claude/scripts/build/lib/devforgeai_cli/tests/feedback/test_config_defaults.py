"""
Tests for config_defaults.py module.

Validates default configuration values, accessors, and copy semantics.
Target: 100% coverage of 8 statements.
"""

import pytest
from devforgeai_cli.feedback.config_defaults import (
    DEFAULT_CONFIG_DICT,
    get_default_config,
    get_default_nested_config
)


class TestConfigDefaults:
    """Tests for default configuration access."""

    def test_default_config_dict_exists(self):
        """DEFAULT_CONFIG_DICT is defined and non-empty."""
        assert isinstance(DEFAULT_CONFIG_DICT, dict)
        assert len(DEFAULT_CONFIG_DICT) > 0

    def test_default_config_dict_has_enabled(self):
        """Defaults include enabled field with True."""
        assert "enabled" in DEFAULT_CONFIG_DICT
        assert DEFAULT_CONFIG_DICT["enabled"] is True

    def test_default_config_dict_has_trigger_mode(self):
        """Defaults include trigger_mode with failures-only."""
        assert "trigger_mode" in DEFAULT_CONFIG_DICT
        assert DEFAULT_CONFIG_DICT["trigger_mode"] == "failures-only"

    def test_default_config_dict_has_operations(self):
        """Defaults include operations field with None."""
        assert "operations" in DEFAULT_CONFIG_DICT
        assert DEFAULT_CONFIG_DICT["operations"] is None

    def test_default_config_dict_has_nested_sections(self):
        """Defaults include all three nested configuration sections."""
        assert "conversation_settings" in DEFAULT_CONFIG_DICT
        assert "skip_tracking" in DEFAULT_CONFIG_DICT
        assert "templates" in DEFAULT_CONFIG_DICT

    def test_default_config_dict_conversation_settings(self):
        """Conversation settings have correct default values."""
        conv_settings = DEFAULT_CONFIG_DICT["conversation_settings"]
        assert conv_settings["max_questions"] == 5
        assert conv_settings["allow_skip"] is True

    def test_default_config_dict_skip_tracking(self):
        """Skip tracking has correct default values."""
        skip_settings = DEFAULT_CONFIG_DICT["skip_tracking"]
        assert skip_settings["enabled"] is True
        assert skip_settings["max_consecutive_skips"] == 3
        assert skip_settings["reset_on_positive"] is True

    def test_default_config_dict_templates(self):
        """Templates have correct default values."""
        template_settings = DEFAULT_CONFIG_DICT["templates"]
        assert template_settings["format"] == "structured"
        assert template_settings["tone"] == "brief"

    def test_get_default_config_returns_dict(self):
        """get_default_config() returns a dictionary."""
        config = get_default_config()
        assert isinstance(config, dict)

    def test_get_default_config_returns_copy(self):
        """get_default_config() returns a copy, not reference."""
        config1 = get_default_config()
        config1["enabled"] = False  # Modify copy
        config2 = get_default_config()

        # config2 should still have original value
        assert config2["enabled"] is True
        # DEFAULT_CONFIG_DICT should be unchanged
        assert DEFAULT_CONFIG_DICT["enabled"] is True

    def test_get_default_config_has_all_fields(self):
        """Returned config has all required fields."""
        config = get_default_config()
        assert "enabled" in config
        assert "trigger_mode" in config
        assert "operations" in config
        assert "conversation_settings" in config
        assert "skip_tracking" in config
        assert "templates" in config

    def test_get_default_nested_config_valid_section(self):
        """Get default for specific valid section."""
        conv_settings = get_default_nested_config("conversation_settings")
        assert isinstance(conv_settings, dict)
        assert "max_questions" in conv_settings
        assert "allow_skip" in conv_settings

    def test_get_default_nested_config_all_sections(self):
        """All nested sections are accessible."""
        # Conversation settings
        conv = get_default_nested_config("conversation_settings")
        assert "max_questions" in conv
        assert "allow_skip" in conv

        # Skip tracking
        skip = get_default_nested_config("skip_tracking")
        assert "enabled" in skip
        assert "max_consecutive_skips" in skip
        assert "reset_on_positive" in skip

        # Templates
        templates = get_default_nested_config("templates")
        assert "format" in templates
        assert "tone" in templates

    def test_get_default_nested_config_invalid_section(self):
        """Invalid section raises ValueError with helpful message."""
        with pytest.raises(ValueError) as exc_info:
            get_default_nested_config("invalid_section")

        error_msg = str(exc_info.value)
        assert "Unknown configuration section: invalid_section" in error_msg
        assert "Valid sections:" in error_msg

    def test_get_default_nested_config_returns_copy(self):
        """Returned section is a copy, not reference."""
        config1 = get_default_nested_config("conversation_settings")
        config1["max_questions"] = 999  # Modify copy
        config2 = get_default_nested_config("conversation_settings")

        # config2 should have original value
        assert config2["max_questions"] == 5
        # DEFAULT_CONFIG_DICT should be unchanged
        assert DEFAULT_CONFIG_DICT["conversation_settings"]["max_questions"] == 5
