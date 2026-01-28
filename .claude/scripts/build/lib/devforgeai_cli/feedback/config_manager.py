"""
Configuration manager for feedback system.

This module provides the main interface for loading, validating, and managing
feedback system configuration from YAML files.
"""

import os
import logging
import threading
from pathlib import Path
from typing import Optional, Dict, Any
from datetime import datetime

try:
    import yaml
except ImportError:
    yaml = None

from .config_models import FeedbackConfiguration, ConversationSettings, SkipTrackingSettings, TemplateSettings
from .config_defaults import DEFAULT_CONFIG_DICT, get_default_config
from .config_schema import get_schema
from .skip_tracker import SkipTracker
from .hot_reload import HotReloadManager, ConfigFileWatcher


class ConfigurationManager:
    """Manages feedback system configuration loading, validation, and updates.

    Handles YAML parsing, configuration validation, default merging, and
    hot-reload of configuration changes.
    """

    def __init__(
        self,
        config_file_path: Optional[Path] = None,
        logs_dir: Optional[Path] = None,
        enable_hot_reload: bool = True
    ):
        """Initialize the configuration manager.

        Args:
            config_file_path: Path to feedback.yaml configuration file.
                             Defaults to devforgeai/config/feedback.yaml
            logs_dir: Directory for logging configuration changes.
                     Defaults to devforgeai/logs
            enable_hot_reload: Whether to enable configuration hot-reload.
        """
        if config_file_path is None:
            config_file_path = Path("devforgeai/config/feedback.yaml")
        if logs_dir is None:
            logs_dir = Path("devforgeai/logs")

        self.config_file_path = config_file_path
        self.logs_dir = logs_dir
        self._config_errors_log = logs_dir / "config-errors.log"
        self._current_config: Optional[FeedbackConfiguration] = None
        self._skip_tracker: Optional[SkipTracker] = None
        self._hot_reload_manager: Optional[HotReloadManager] = None
        self._initialization_lock = threading.Lock()
        self._initialized = False
        self._debug = os.getenv("DEBUG_FEEDBACK_CONFIG", "false").lower() == "true"

        # Setup logging
        self._setup_logging()

        # Load initial configuration
        self._current_config = self.load_configuration()

        # Initialize skip tracker
        self._skip_tracker = SkipTracker(logs_dir / "feedback-skips.log")

        # Setup hot-reload if enabled
        if enable_hot_reload:
            self._hot_reload_manager = HotReloadManager(
                config_file_path,
                self._reload_config_callback
            )
            self._hot_reload_manager.start()
            self._hot_reload_manager.set_current_config(self._current_config)

        self._initialized = True

    def _setup_logging(self) -> None:
        """Setup logging for configuration operations."""
        self.logs_dir.mkdir(parents=True, exist_ok=True)

    def _log_error(self, message: str) -> None:
        """Log configuration error.

        Args:
            message: Error message to log.
        """
        try:
            with open(self._config_errors_log, 'a') as f:
                timestamp = datetime.now().isoformat()
                f.write(f"{timestamp}: {message}\n")
        except (IOError, OSError):
            # Silently fail if log write fails
            pass

        if self._debug:
            print(f"[CONFIG] {message}")

    def _ensure_config_directory(self) -> None:
        """Ensure config directory exists."""
        self.config_file_path.parent.mkdir(parents=True, exist_ok=True)

    def _load_yaml_file(self) -> Optional[Dict[str, Any]]:
        """Load and parse YAML configuration file.

        Returns:
            Parsed configuration dictionary or None if file doesn't exist.

        Raises:
            yaml.YAMLError: If YAML parsing fails.
            IOError: If file cannot be read.
        """
        if not self.config_file_path.exists():
            return None

        try:
            if yaml is None:
                self._log_error("PyYAML not available - cannot load configuration")
                return None

            with open(self.config_file_path, 'r') as f:
                content = yaml.safe_load(f)
                return content if isinstance(content, dict) else {}

        except yaml.YAMLError as e:
            self._log_error(f"YAML parsing error: {str(e)}")
            raise
        except (IOError, OSError) as e:
            self._log_error(f"File read error: {str(e)}")
            raise

    def _merge_nested_config(self, section_name: str, loaded_config: Dict[str, Any], merged: Dict[str, Any]) -> None:
        """Merge a nested configuration section with defaults.

        Args:
            section_name: Name of the configuration section to merge.
            loaded_config: Loaded configuration dictionary.
            merged: Target merged configuration dictionary to update in-place.
        """
        if section_name in loaded_config and isinstance(loaded_config[section_name], dict):
            merged_section = get_default_config()[section_name].copy()
            merged_section.update(loaded_config[section_name])
            merged[section_name] = merged_section

    def _merge_with_defaults(self, loaded_config: Optional[Dict[str, Any]]) -> Dict[str, Any]:
        """Merge loaded configuration with defaults.

        Args:
            loaded_config: Configuration loaded from file (may be None or partial).

        Returns:
            Complete configuration with defaults merged.
        """
        if loaded_config is None:
            return get_default_config()

        # Start with defaults
        merged = get_default_config()

        # Override with loaded values (shallow merge at top level)
        merged.update(loaded_config)

        # For nested objects, merge more carefully
        for section_name in ("conversation_settings", "skip_tracking", "templates"):
            self._merge_nested_config(section_name, loaded_config, merged)

        return merged

    def _parse_nested_settings(self, section_name: str, config_dict: Dict[str, Any], settings_class: type) -> Optional[Any]:
        """Parse a nested configuration section to a dataclass object.

        Args:
            section_name: Name of the configuration section.
            config_dict: Configuration dictionary.
            settings_class: Dataclass to instantiate (ConversationSettings, SkipTrackingSettings, or TemplateSettings).

        Returns:
            Instantiated settings object or None if section not found.
        """
        if section_name in config_dict:
            section_dict = config_dict[section_name]
            if isinstance(section_dict, dict):
                return settings_class(**section_dict)
        return None

    def _dict_to_configuration(self, config_dict: Dict[str, Any]) -> FeedbackConfiguration:
        """Convert configuration dictionary to FeedbackConfiguration object.

        Args:
            config_dict: Configuration dictionary.

        Returns:
            FeedbackConfiguration object.

        Raises:
            ValueError: If configuration is invalid.
        """
        try:
            # Parse nested objects
            conv_settings = self._parse_nested_settings("conversation_settings", config_dict, ConversationSettings)
            skip_settings = self._parse_nested_settings("skip_tracking", config_dict, SkipTrackingSettings)
            template_settings = self._parse_nested_settings("templates", config_dict, TemplateSettings)

            # Create configuration object
            config = FeedbackConfiguration(
                enabled=config_dict.get("enabled", True),
                trigger_mode=config_dict.get("trigger_mode", "failures-only"),
                operations=config_dict.get("operations"),
                conversation_settings=conv_settings,
                skip_tracking=skip_settings,
                templates=template_settings
            )

            return config

        except ValueError as e:
            self._log_error(f"Configuration validation error: {str(e)}")
            raise

    def load_configuration(self) -> FeedbackConfiguration:
        """Load and validate feedback configuration.

        Loads YAML file, validates structure, merges with defaults, and
        returns a FeedbackConfiguration object.

        Returns:
            FeedbackConfiguration object with loaded or default values.

        Performance:
            Should complete in <100ms.
        """
        try:
            # Load YAML file
            loaded_dict = self._load_yaml_file()

            # Merge with defaults
            merged_config = self._merge_with_defaults(loaded_dict)

            # Convert to configuration object (validates)
            config = self._dict_to_configuration(merged_config)

            return config

        except Exception as e:
            self._log_error(f"Failed to load configuration: {str(e)}")
            # Return default configuration on error
            return FeedbackConfiguration()

    def _reload_config_callback(self) -> FeedbackConfiguration:
        """Callback for hot-reload to load new configuration.

        Called when configuration file changes are detected.

        Returns:
            New FeedbackConfiguration object.
        """
        return self.load_configuration()

    def get_configuration(self) -> FeedbackConfiguration:
        """Get the current configuration.

        Returns:
            Current FeedbackConfiguration object.
        """
        with self._initialization_lock:
            if self._current_config is None:
                self._current_config = self.load_configuration()
            return self._current_config

    def update_configuration(self, config: FeedbackConfiguration) -> None:
        """Update the current configuration (for testing/reloading).

        Args:
            config: New FeedbackConfiguration object.
        """
        with self._initialization_lock:
            self._current_config = config

    def is_enabled(self) -> bool:
        """Check if feedback collection is enabled.

        Returns:
            True if enabled, False otherwise.
        """
        config = self.get_configuration()
        return config.enabled

    def get_trigger_mode(self) -> str:
        """Get the trigger mode.

        Returns:
            Trigger mode string (always, failures-only, specific-operations, never).
        """
        config = self.get_configuration()
        return config.trigger_mode

    def get_operations(self) -> Optional[list]:
        """Get the list of specific operations (if applicable).

        Returns:
            List of operation names or None.
        """
        config = self.get_configuration()
        return config.operations

    def _get_nested_config(self, attribute_name: str) -> Optional[Any]:
        """Get a nested configuration object by attribute name.

        Args:
            attribute_name: Name of the nested config attribute
                           (conversation_settings, skip_tracking, or templates).

        Returns:
            The nested configuration object or None if not found.
        """
        config = self.get_configuration()
        return getattr(config, attribute_name, None)

    def get_conversation_settings(self) -> Optional[ConversationSettings]:
        """Get conversation settings from current configuration.

        Returns:
            ConversationSettings object or None.
        """
        return self._get_nested_config("conversation_settings")

    def get_skip_tracking_settings(self) -> Optional[SkipTrackingSettings]:
        """Get skip tracking settings from current configuration.

        Returns:
            SkipTrackingSettings object or None.
        """
        return self._get_nested_config("skip_tracking")

    def get_template_settings(self) -> Optional[TemplateSettings]:
        """Get template settings from current configuration.

        Returns:
            TemplateSettings object or None.
        """
        return self._get_nested_config("templates")

    def get_skip_tracker(self) -> SkipTracker:
        """Get the skip tracker instance.

        Returns:
            SkipTracker object for tracking skips.
        """
        return self._skip_tracker

    def is_hot_reload_enabled(self) -> bool:
        """Check if hot-reload is active.

        Returns:
            True if hot-reload is running, False otherwise.
        """
        if self._hot_reload_manager is None:
            return False
        return self._hot_reload_manager.is_running()

    def start_hot_reload(self) -> None:
        """Start configuration hot-reload."""
        if self._hot_reload_manager is not None and not self._hot_reload_manager.is_running():
            self._hot_reload_manager.start()
            self._hot_reload_manager.set_current_config(self._current_config)

    def stop_hot_reload(self) -> None:
        """Stop configuration hot-reload."""
        if self._hot_reload_manager is not None:
            self._hot_reload_manager.stop()

    def shutdown(self) -> None:
        """Shutdown the configuration manager (stop hot-reload, etc.)."""
        self.stop_hot_reload()


# Global configuration manager instance
_global_config_manager: Optional[ConfigurationManager] = None
_global_manager_lock = threading.Lock()


def get_config_manager(
    config_file_path: Optional[Path] = None,
    logs_dir: Optional[Path] = None
) -> ConfigurationManager:
    """Get or create the global configuration manager.

    Uses singleton pattern to ensure only one manager exists.

    Args:
        config_file_path: Path to configuration file (used on first call only).
        logs_dir: Path to logs directory (used on first call only).

    Returns:
        ConfigurationManager instance.
    """
    global _global_config_manager

    if _global_config_manager is None:
        with _global_manager_lock:
            if _global_config_manager is None:
                _global_config_manager = ConfigurationManager(
                    config_file_path=config_file_path,
                    logs_dir=logs_dir
                )

    return _global_config_manager


def reset_config_manager() -> None:
    """Reset the global configuration manager (for testing)."""
    global _global_config_manager

    with _global_manager_lock:
        if _global_config_manager is not None:
            _global_config_manager.shutdown()
        _global_config_manager = None
