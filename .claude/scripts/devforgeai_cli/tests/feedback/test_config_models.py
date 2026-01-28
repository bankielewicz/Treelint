"""
Tests for config_models.py module.

Validates data models, enums, validation logic, and type checking.
Target: 95% coverage of 85 statements.
"""

import pytest
from devforgeai_cli.feedback.config_models import (
    TriggerMode,
    TemplateFormat,
    TemplateTone,
    VALID_TEMPLATE_FORMATS,
    VALID_TEMPLATE_TONES,
    VALID_TRIGGER_MODES,
    ConversationSettings,
    SkipTrackingSettings,
    TemplateSettings,
    FeedbackConfiguration,
)


class TestEnumsAndConstants:
    """Tests for enum definitions and validation constants."""

    def test_trigger_mode_enum_values(self):
        """TriggerMode enum has all required values."""
        assert TriggerMode.ALWAYS.value == "always"
        assert TriggerMode.FAILURES_ONLY.value == "failures-only"
        assert TriggerMode.SPECIFIC_OPS.value == "specific-operations"
        assert TriggerMode.NEVER.value == "never"

    def test_template_format_enum_values(self):
        """TemplateFormat enum has structured and free-text."""
        assert TemplateFormat.STRUCTURED.value == "structured"
        assert TemplateFormat.FREE_TEXT.value == "free-text"

    def test_template_tone_enum_values(self):
        """TemplateTone enum has brief and detailed."""
        assert TemplateTone.BRIEF.value == "brief"
        assert TemplateTone.DETAILED.value == "detailed"

    def test_valid_template_formats_constant(self):
        """VALID_TEMPLATE_FORMATS contains correct values."""
        assert VALID_TEMPLATE_FORMATS == {"structured", "free-text"}

    def test_valid_template_tones_constant(self):
        """VALID_TEMPLATE_TONES contains correct values."""
        assert VALID_TEMPLATE_TONES == {"brief", "detailed"}

    def test_valid_trigger_modes_constant(self):
        """VALID_TRIGGER_MODES contains all trigger modes."""
        assert VALID_TRIGGER_MODES == {
            "always",
            "failures-only",
            "specific-operations",
            "never"
        }


class TestConversationSettings:
    """Tests for ConversationSettings dataclass."""

    def test_conversation_settings_default_values(self):
        """ConversationSettings has correct defaults."""
        settings = ConversationSettings()
        assert settings.max_questions == 5
        assert settings.allow_skip is True

    def test_conversation_settings_custom_values(self):
        """ConversationSettings accepts custom values."""
        settings = ConversationSettings(max_questions=10, allow_skip=False)
        assert settings.max_questions == 10
        assert settings.allow_skip is False

    def test_conversation_settings_zero_max_questions(self):
        """ConversationSettings allows 0 (unlimited)."""
        settings = ConversationSettings(max_questions=0)
        assert settings.max_questions == 0

    def test_conversation_settings_invalid_max_questions_negative(self):
        """Negative max_questions raises ValueError."""
        with pytest.raises(ValueError) as exc_info:
            ConversationSettings(max_questions=-1)
        assert "must be non-negative integer" in str(exc_info.value)

    def test_conversation_settings_invalid_max_questions_not_int(self):
        """Non-integer max_questions raises ValueError."""
        with pytest.raises(ValueError) as exc_info:
            ConversationSettings(max_questions="invalid")
        # Type error happens before validation
        pass

    def test_conversation_settings_invalid_allow_skip_not_bool(self):
        """Non-boolean allow_skip raises ValueError."""
        with pytest.raises(ValueError) as exc_info:
            ConversationSettings(allow_skip="invalid")
        # Type error happens before validation
        pass


class TestSkipTrackingSettings:
    """Tests for SkipTrackingSettings dataclass."""

    def test_skip_tracking_settings_default_values(self):
        """SkipTrackingSettings has correct defaults."""
        settings = SkipTrackingSettings()
        assert settings.enabled is True
        assert settings.max_consecutive_skips == 3
        assert settings.reset_on_positive is True

    def test_skip_tracking_settings_custom_values(self):
        """SkipTrackingSettings accepts custom values."""
        settings = SkipTrackingSettings(
            enabled=False,
            max_consecutive_skips=5,
            reset_on_positive=False
        )
        assert settings.enabled is False
        assert settings.max_consecutive_skips == 5
        assert settings.reset_on_positive is False

    def test_skip_tracking_settings_zero_max_skips(self):
        """SkipTrackingSettings allows 0 (unlimited)."""
        settings = SkipTrackingSettings(max_consecutive_skips=0)
        assert settings.max_consecutive_skips == 0

    def test_skip_tracking_settings_invalid_enabled_not_bool(self):
        """Non-boolean enabled raises ValueError."""
        with pytest.raises(ValueError) as exc_info:
            SkipTrackingSettings(enabled="invalid")
        pass

    def test_skip_tracking_settings_invalid_max_skips_negative(self):
        """Negative max_consecutive_skips raises ValueError."""
        with pytest.raises(ValueError) as exc_info:
            SkipTrackingSettings(max_consecutive_skips=-1)
        assert "must be non-negative integer" in str(exc_info.value)

    def test_skip_tracking_settings_invalid_reset_on_positive_not_bool(self):
        """Non-boolean reset_on_positive raises ValueError."""
        with pytest.raises(ValueError) as exc_info:
            SkipTrackingSettings(reset_on_positive="invalid")
        pass


class TestTemplateSettings:
    """Tests for TemplateSettings dataclass."""

    def test_template_settings_default_values(self):
        """TemplateSettings has correct defaults."""
        settings = TemplateSettings()
        assert settings.format == "structured"
        assert settings.tone == "brief"

    def test_template_settings_custom_values_structured_brief(self):
        """TemplateSettings accepts structured + brief."""
        settings = TemplateSettings(format="structured", tone="brief")
        assert settings.format == "structured"
        assert settings.tone == "brief"

    def test_template_settings_custom_values_free_text_detailed(self):
        """TemplateSettings accepts free-text + detailed."""
        settings = TemplateSettings(format="free-text", tone="detailed")
        assert settings.format == "free-text"
        assert settings.tone == "detailed"

    def test_template_settings_invalid_format(self):
        """Invalid format raises ValueError with helpful message."""
        with pytest.raises(ValueError) as exc_info:
            TemplateSettings(format="invalid")
        error_msg = str(exc_info.value)
        assert "Invalid template format: 'invalid'" in error_msg
        assert "Must be one of:" in error_msg

    def test_template_settings_invalid_tone(self):
        """Invalid tone raises ValueError with helpful message."""
        with pytest.raises(ValueError) as exc_info:
            TemplateSettings(tone="invalid")
        error_msg = str(exc_info.value)
        assert "Invalid template tone: 'invalid'" in error_msg
        assert "Must be one of:" in error_msg


class TestFeedbackConfiguration:
    """Tests for main FeedbackConfiguration dataclass."""

    def test_feedback_configuration_default_values(self):
        """FeedbackConfiguration has correct defaults."""
        config = FeedbackConfiguration()
        assert config.enabled is True
        assert config.trigger_mode == "failures-only"
        assert config.operations is None
        assert isinstance(config.conversation_settings, ConversationSettings)
        assert isinstance(config.skip_tracking, SkipTrackingSettings)
        assert isinstance(config.templates, TemplateSettings)

    def test_feedback_configuration_custom_values(self):
        """FeedbackConfiguration accepts all custom values."""
        config = FeedbackConfiguration(
            enabled=False,
            trigger_mode="always",
            operations=["qa", "dev"],
            conversation_settings=ConversationSettings(max_questions=10),
            skip_tracking=SkipTrackingSettings(enabled=False),
            templates=TemplateSettings(format="free-text", tone="detailed")
        )
        assert config.enabled is False
        assert config.trigger_mode == "always"
        assert config.operations == ["qa", "dev"]
        assert config.conversation_settings.max_questions == 10
        assert config.skip_tracking.enabled is False
        assert config.templates.format == "free-text"

    def test_feedback_configuration_normalize_conversation_settings_dict(self):
        """Nested dict for conversation_settings is converted to dataclass."""
        config = FeedbackConfiguration(
            conversation_settings={"max_questions": 8, "allow_skip": False}
        )
        assert isinstance(config.conversation_settings, ConversationSettings)
        assert config.conversation_settings.max_questions == 8
        assert config.conversation_settings.allow_skip is False

    def test_feedback_configuration_normalize_skip_tracking_dict(self):
        """Nested dict for skip_tracking is converted to dataclass."""
        config = FeedbackConfiguration(
            skip_tracking={"enabled": False, "max_consecutive_skips": 10, "reset_on_positive": False}
        )
        assert isinstance(config.skip_tracking, SkipTrackingSettings)
        assert config.skip_tracking.enabled is False
        assert config.skip_tracking.max_consecutive_skips == 10

    def test_feedback_configuration_normalize_templates_dict(self):
        """Nested dict for templates is converted to dataclass."""
        config = FeedbackConfiguration(
            templates={"format": "free-text", "tone": "detailed"}
        )
        assert isinstance(config.templates, TemplateSettings)
        assert config.templates.format == "free-text"
        assert config.templates.tone == "detailed"

    def test_feedback_configuration_normalize_none_conversation_settings(self):
        """None conversation_settings is replaced with defaults."""
        config = FeedbackConfiguration(conversation_settings=None)
        assert isinstance(config.conversation_settings, ConversationSettings)
        assert config.conversation_settings.max_questions == 5

    def test_feedback_configuration_normalize_none_skip_tracking(self):
        """None skip_tracking is replaced with defaults."""
        config = FeedbackConfiguration(skip_tracking=None)
        assert isinstance(config.skip_tracking, SkipTrackingSettings)
        assert config.skip_tracking.enabled is True

    def test_feedback_configuration_normalize_none_templates(self):
        """None templates is replaced with defaults."""
        config = FeedbackConfiguration(templates=None)
        assert isinstance(config.templates, TemplateSettings)
        assert config.templates.format == "structured"

    def test_feedback_configuration_invalid_enabled_not_bool(self):
        """Non-boolean enabled raises ValueError."""
        with pytest.raises(ValueError) as exc_info:
            FeedbackConfiguration(enabled="invalid")
        pass

    def test_feedback_configuration_invalid_trigger_mode(self):
        """Invalid trigger_mode raises ValueError with helpful message."""
        with pytest.raises(ValueError) as exc_info:
            FeedbackConfiguration(trigger_mode="invalid-mode")
        error_msg = str(exc_info.value)
        assert "Invalid trigger_mode value: 'invalid-mode'" in error_msg
        assert "Must be one of:" in error_msg

    def test_feedback_configuration_trigger_mode_always(self):
        """Trigger mode 'always' is valid."""
        config = FeedbackConfiguration(trigger_mode="always")
        assert config.trigger_mode == "always"

    def test_feedback_configuration_trigger_mode_failures_only(self):
        """Trigger mode 'failures-only' is valid."""
        config = FeedbackConfiguration(trigger_mode="failures-only")
        assert config.trigger_mode == "failures-only"

    def test_feedback_configuration_trigger_mode_specific_operations(self):
        """Trigger mode 'specific-operations' is valid."""
        config = FeedbackConfiguration(
            trigger_mode="specific-operations",
            operations=["qa"]
        )
        assert config.trigger_mode == "specific-operations"

    def test_feedback_configuration_trigger_mode_never(self):
        """Trigger mode 'never' is valid."""
        config = FeedbackConfiguration(trigger_mode="never")
        assert config.trigger_mode == "never"

    def test_feedback_configuration_specific_operations_requires_operations(self):
        """specific-operations mode requires operations list."""
        with pytest.raises(ValueError) as exc_info:
            FeedbackConfiguration(
                trigger_mode="specific-operations",
                operations=None
            )
        error_msg = str(exc_info.value)
        assert "operations list must be provided" in error_msg

    def test_feedback_configuration_specific_operations_requires_non_empty(self):
        """specific-operations mode requires non-empty operations list."""
        with pytest.raises(ValueError) as exc_info:
            FeedbackConfiguration(
                trigger_mode="specific-operations",
                operations=[]
            )
        error_msg = str(exc_info.value)
        assert "non-empty" in error_msg

    def test_feedback_configuration_operations_with_other_modes(self):
        """Operations can be None for non-specific-operations modes."""
        config = FeedbackConfiguration(
            trigger_mode="always",
            operations=None
        )
        assert config.operations is None

    def test_feedback_configuration_to_dict(self):
        """to_dict() converts configuration to dictionary."""
        config = FeedbackConfiguration()
        result = config.to_dict()

        assert isinstance(result, dict)
        assert result["enabled"] is True
        assert result["trigger_mode"] == "failures-only"
        assert result["operations"] is None
        assert isinstance(result["conversation_settings"], dict)
        assert isinstance(result["skip_tracking"], dict)
        assert isinstance(result["templates"], dict)

    def test_feedback_configuration_to_dict_nested_structure(self):
        """to_dict() includes all nested fields."""
        config = FeedbackConfiguration()
        result = config.to_dict()

        # Check conversation_settings nested fields
        assert result["conversation_settings"]["max_questions"] == 5
        assert result["conversation_settings"]["allow_skip"] is True

        # Check skip_tracking nested fields
        assert result["skip_tracking"]["enabled"] is True
        assert result["skip_tracking"]["max_consecutive_skips"] == 3
        assert result["skip_tracking"]["reset_on_positive"] is True

        # Check templates nested fields
        assert result["templates"]["format"] == "structured"
        assert result["templates"]["tone"] == "brief"

    def test_feedback_configuration_to_dict_custom_values(self):
        """to_dict() correctly serializes custom values."""
        config = FeedbackConfiguration(
            enabled=False,
            trigger_mode="specific-operations",
            operations=["qa", "dev"],
            conversation_settings=ConversationSettings(max_questions=10, allow_skip=False),
            skip_tracking=SkipTrackingSettings(enabled=False, max_consecutive_skips=5, reset_on_positive=False),
            templates=TemplateSettings(format="free-text", tone="detailed")
        )
        result = config.to_dict()

        assert result["enabled"] is False
        assert result["trigger_mode"] == "specific-operations"
        assert result["operations"] == ["qa", "dev"]
        assert result["conversation_settings"]["max_questions"] == 10
        assert result["skip_tracking"]["enabled"] is False
        assert result["templates"]["format"] == "free-text"
