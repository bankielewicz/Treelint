"""
Default configuration values for feedback system.

This module provides centralized default values and initial configuration.
"""

from typing import Dict, Any


# Default configuration as dictionary
DEFAULT_CONFIG_DICT: Dict[str, Any] = {
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


def get_default_config() -> Dict[str, Any]:
    """Get a copy of the default configuration.

    Returns:
        Dictionary containing default configuration values.
    """
    return DEFAULT_CONFIG_DICT.copy()


def get_default_nested_config(section: str) -> Dict[str, Any]:
    """Get default configuration for a specific nested section.

    Args:
        section: Name of the configuration section ('conversation_settings', 'skip_tracking', 'templates').

    Returns:
        Dictionary containing defaults for the specified section.

    Raises:
        ValueError: If section is not recognized.
    """
    if section not in DEFAULT_CONFIG_DICT:
        raise ValueError(
            f"Unknown configuration section: {section}. "
            f"Valid sections: {', '.join(DEFAULT_CONFIG_DICT.keys())}"
        )

    return DEFAULT_CONFIG_DICT[section].copy() if isinstance(DEFAULT_CONFIG_DICT[section], dict) else DEFAULT_CONFIG_DICT[section]
