"""
JSON Schema definition for feedback configuration validation.

This module provides the JSON schema used to validate configuration files
against the expected structure.
"""

from typing import Dict, Any


# JSON Schema for feedback configuration
FEEDBACK_CONFIG_SCHEMA: Dict[str, Any] = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "title": "Feedback Configuration Schema",
    "description": "Schema for DevForgeAI feedback system configuration (YAML-based)",
    "type": "object",
    "properties": {
        "enabled": {
            "type": "boolean",
            "description": "Master enable/disable switch for feedback collection",
            "default": True
        },
        "trigger_mode": {
            "type": "string",
            "enum": ["always", "failures-only", "specific-operations", "never"],
            "description": "Determines when feedback is collected",
            "default": "failures-only"
        },
        "operations": {
            "type": ["array", "null"],
            "items": {
                "type": "string"
            },
            "description": "List of operations to collect feedback for (only used if trigger_mode is 'specific-operations')",
            "default": None
        },
        "conversation_settings": {
            "type": "object",
            "description": "Conversation-level settings for feedback collection",
            "properties": {
                "max_questions": {
                    "type": "integer",
                    "minimum": 0,
                    "description": "Maximum number of questions per conversation (0 = unlimited)",
                    "default": 5
                },
                "allow_skip": {
                    "type": "boolean",
                    "description": "Whether users can skip feedback questions",
                    "default": True
                }
            },
            "additionalProperties": False,
            "default": {
                "max_questions": 5,
                "allow_skip": True
            }
        },
        "skip_tracking": {
            "type": "object",
            "description": "Skip tracking configuration",
            "properties": {
                "enabled": {
                    "type": "boolean",
                    "description": "Whether skip tracking is active",
                    "default": True
                },
                "max_consecutive_skips": {
                    "type": "integer",
                    "minimum": 0,
                    "description": "Maximum consecutive skips allowed (0 = unlimited)",
                    "default": 3
                },
                "reset_on_positive": {
                    "type": "boolean",
                    "description": "Whether to reset skip counter on positive feedback",
                    "default": True
                }
            },
            "additionalProperties": False,
            "default": {
                "enabled": True,
                "max_consecutive_skips": 3,
                "reset_on_positive": True
            }
        },
        "templates": {
            "type": "object",
            "description": "Template preferences for feedback collection",
            "properties": {
                "format": {
                    "type": "string",
                    "enum": ["structured", "free-text"],
                    "description": "Template format for feedback questions",
                    "default": "structured"
                },
                "tone": {
                    "type": "string",
                    "enum": ["brief", "detailed"],
                    "description": "Tone for feedback questions",
                    "default": "brief"
                }
            },
            "additionalProperties": False,
            "default": {
                "format": "structured",
                "tone": "brief"
            }
        }
    },
    "additionalProperties": False,
    "required": [],  # All fields are optional with defaults
    "default": {
        "enabled": True,
        "trigger_mode": "failures-only",
        "operations": None,
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
}


def get_schema() -> Dict[str, Any]:
    """Get a copy of the feedback configuration schema.

    Returns:
        Dictionary containing the JSON schema.
    """
    return FEEDBACK_CONFIG_SCHEMA.copy()
