"""
Tests for config_schema.py module.

Validates JSON Schema structure and accessor functionality.
Target: 100% coverage of 4 statements (module-level dict + 1 function).
"""

import pytest
from devforgeai_cli.feedback.config_schema import FEEDBACK_CONFIG_SCHEMA, get_schema


class TestConfigSchema:
    """Tests for JSON Schema structure and access."""

    def test_feedback_config_schema_global_defined(self):
        """Global FEEDBACK_CONFIG_SCHEMA is defined and non-empty."""
        assert isinstance(FEEDBACK_CONFIG_SCHEMA, dict)
        assert len(FEEDBACK_CONFIG_SCHEMA) > 0

    def test_get_schema_returns_dict(self):
        """get_schema() returns a dictionary."""
        schema = get_schema()
        assert isinstance(schema, dict)

    def test_get_schema_returns_copy(self):
        """get_schema() returns a copy, not reference."""
        schema1 = get_schema()
        schema1["title"] = "Modified"  # Modify copy
        schema2 = get_schema()

        # schema2 should have original value
        assert schema2["title"] == "Feedback Configuration Schema"
        # FEEDBACK_CONFIG_SCHEMA should be unchanged
        assert FEEDBACK_CONFIG_SCHEMA["title"] == "Feedback Configuration Schema"

    def test_get_schema_has_required_structure(self):
        """Schema contains required JSON Schema fields."""
        schema = get_schema()
        assert "$schema" in schema
        assert "title" in schema
        assert "type" in schema
        assert "properties" in schema

    def test_get_schema_json_schema_draft_07(self):
        """Schema specifies JSON Schema draft 07."""
        schema = get_schema()
        assert schema["$schema"] == "http://json-schema.org/draft-07/schema#"

    def test_get_schema_type_is_object(self):
        """Schema type is 'object' for configuration dict."""
        schema = get_schema()
        assert schema["type"] == "object"

    def test_get_schema_has_enabled_property(self):
        """Schema defines 'enabled' property."""
        schema = get_schema()
        assert "enabled" in schema["properties"]
        assert schema["properties"]["enabled"]["type"] == "boolean"

    def test_get_schema_has_trigger_mode_property(self):
        """Schema defines 'trigger_mode' property with enum."""
        schema = get_schema()
        assert "trigger_mode" in schema["properties"]
        trigger_mode = schema["properties"]["trigger_mode"]
        assert trigger_mode["type"] == "string"
        assert "enum" in trigger_mode
        assert set(trigger_mode["enum"]) == {"always", "failures-only", "specific-operations", "never"}

    def test_get_schema_has_operations_property(self):
        """Schema defines 'operations' property (array or null)."""
        schema = get_schema()
        assert "operations" in schema["properties"]
        operations = schema["properties"]["operations"]
        assert set(operations["type"]) == {"array", "null"}

    def test_get_schema_has_conversation_settings(self):
        """Schema defines 'conversation_settings' nested object."""
        schema = get_schema()
        assert "conversation_settings" in schema["properties"]
        conv = schema["properties"]["conversation_settings"]
        assert conv["type"] == "object"
        assert "max_questions" in conv["properties"]
        assert "allow_skip" in conv["properties"]

    def test_get_schema_has_skip_tracking(self):
        """Schema defines 'skip_tracking' nested object."""
        schema = get_schema()
        assert "skip_tracking" in schema["properties"]
        skip = schema["properties"]["skip_tracking"]
        assert skip["type"] == "object"
        assert "enabled" in skip["properties"]
        assert "max_consecutive_skips" in skip["properties"]
        assert "reset_on_positive" in skip["properties"]

    def test_get_schema_has_templates(self):
        """Schema defines 'templates' nested object."""
        schema = get_schema()
        assert "templates" in schema["properties"]
        templates = schema["properties"]["templates"]
        assert templates["type"] == "object"
        assert "format" in templates["properties"]
        assert "tone" in templates["properties"]

    def test_get_schema_has_default_values(self):
        """Schema includes default values at root level."""
        schema = get_schema()
        assert "default" in schema
        defaults = schema["default"]
        assert defaults["enabled"] is True
        assert defaults["trigger_mode"] == "failures-only"
        assert "conversation_settings" in defaults
        assert "skip_tracking" in defaults
        assert "templates" in defaults

    def test_get_schema_no_additional_properties(self):
        """Schema disallows additional properties at root."""
        schema = get_schema()
        assert schema["additionalProperties"] is False

    def test_get_schema_template_format_enum(self):
        """Template format has correct enum values."""
        schema = get_schema()
        template_format = schema["properties"]["templates"]["properties"]["format"]
        assert set(template_format["enum"]) == {"structured", "free-text"}

    def test_get_schema_template_tone_enum(self):
        """Template tone has correct enum values."""
        schema = get_schema()
        template_tone = schema["properties"]["templates"]["properties"]["tone"]
        assert set(template_tone["enum"]) == {"brief", "detailed"}
