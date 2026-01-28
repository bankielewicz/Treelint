#!/usr/bin/env python3
"""
DevForgeAI check-hooks CLI Command

Validates whether hooks should trigger based on:
- Global enabled/disabled status
- Trigger rules (all/failures-only/none)
- Operation-specific overrides
- Circular invocation detection

Exit Codes:
  0 - Hooks should trigger
  1 - Hooks should not trigger (or disabled/missing config)
  2 - Error (invalid arguments or config error)

Story: STORY-021 - Implement devforgeai check-hooks CLI command
"""

import os
import logging
import sys
from pathlib import Path
from typing import Dict, Optional, Any
import yaml

# Exit codes (exported for use in tests)
EXIT_CODE_TRIGGER = 0
EXIT_CODE_DONT_TRIGGER = 1
EXIT_CODE_ERROR = 2

# Configure logger - uses logger hierarchy for DevForgeAI CLI context
logger = logging.getLogger("devforgeai_cli.commands.check_hooks")


class CheckHooksValidator:
    """Validator for hook configuration and trigger rules."""

    # Valid trigger_on values
    VALID_TRIGGER_ON = {"all", "failures-only", "none"}
    # Valid status values
    VALID_STATUSES = {"success", "failure", "partial"}
    # Valid hook_type values (STORY-185)
    VALID_HOOK_TYPES = {"user", "ai", "all"}

    def __init__(self, config: Dict[str, Any], hook_type: str = "all"):
        """
        Initialize validator with configuration.

        Args:
            config: Hook configuration dictionary
            hook_type: Filter hooks by type ('user', 'ai', or 'all') (STORY-185)
        """
        self.config = config or {}
        self.hook_type = hook_type  # STORY-185: Store hook_type for filtering
        self.enabled = self.config.get("enabled", False)
        self.global_rules = self.config.get("global_rules") or {}
        self.operations = self.config.get("operations") or {}
        self.hooks = self._filter_hooks_by_type()  # STORY-185: Filter hooks

    def _filter_hooks_by_type(self) -> list:
        """
        Filter hooks list by hook_type (STORY-185: AC-3).

        Returns:
            Filtered list of hooks matching hook_type, or all hooks if type is 'all'
        """
        hooks = self.config.get("hooks", [])
        if self.hook_type == "all":
            return hooks
        return [h for h in hooks if h.get("hook_type") == self.hook_type]

    def _is_valid_enum(self, value: str, allowed_set: set, field_name: str) -> bool:
        """
        Validate that value is in allowed set of enum values.

        Helper method to reduce duplication in enum validation.

        Args:
            value: Value to validate
            allowed_set: Set of allowed enum values
            field_name: Name of field being validated (for error messages)

        Returns:
            True if valid, False otherwise
        """
        return value in allowed_set

    def validate_status(self, status: str) -> bool:
        """
        Validate that status is one of the allowed values.

        Args:
            status: Status value to validate

        Returns:
            True if valid, False otherwise
        """
        return self._is_valid_enum(status, self.VALID_STATUSES, "status")

    def validate_trigger_on(self, trigger_on: str) -> bool:
        """
        Validate that trigger_on is one of the allowed values.

        Args:
            trigger_on: Trigger rule to validate

        Returns:
            True if valid, False otherwise
        """
        return self._is_valid_enum(trigger_on, self.VALID_TRIGGER_ON, "trigger_on")

    def validate(self) -> None:
        """
        Validate the entire configuration schema.

        Raises:
            ValueError: If configuration is invalid
        """
        # Check global_rules trigger_on value
        if self.global_rules:
            trigger_on = self.global_rules.get("trigger_on")
            if trigger_on and not self.validate_trigger_on(trigger_on):
                raise ValueError(f"Invalid trigger_on value: {trigger_on}")

        # Check operation-specific trigger_on values
        for op_name, op_config in self.operations.items():
            if isinstance(op_config, dict):
                trigger_on = op_config.get("trigger_on")
                if trigger_on and not self.validate_trigger_on(trigger_on):
                    raise ValueError(
                        f"Invalid trigger_on value for operation '{op_name}': {trigger_on}"
                    )

    def get_trigger_rule(self, operation: str) -> Optional[str]:
        """
        Get the trigger rule for an operation (with fallback to global).

        Args:
            operation: Operation name

        Returns:
            Trigger rule string (all/failures-only/none) or None
        """
        # Check for operation-specific override
        if operation in self.operations:
            op_config = self.operations[operation]
            if isinstance(op_config, dict) and "trigger_on" in op_config:
                return op_config["trigger_on"]

        # Fall back to global rule
        if self.global_rules and "trigger_on" in self.global_rules:
            return self.global_rules["trigger_on"]

        # Default: don't trigger
        return None

    def should_trigger(self, operation: str, status: str) -> bool:
        """
        Determine if hook should trigger based on rules.

        Args:
            operation: Operation name
            status: Operation status (success/failure/partial)

        Returns:
            True if hook should trigger, False otherwise
        """
        trigger_rule = self.get_trigger_rule(operation)

        if trigger_rule is None:
            return False

        if trigger_rule == "all":
            # Trigger on any status
            return True
        elif trigger_rule == "failures-only":
            # Trigger only on failure or partial (not success)
            return status in {"failure", "partial"}
        elif trigger_rule == "none":
            # Never trigger
            return False
        else:
            # Invalid trigger rule - don't trigger as safe default
            logger.warning(f"Invalid trigger_on rule: {trigger_rule}")
            return False


def load_config(config_path: Optional[str] = None) -> Optional[Dict[str, Any]]:
    """
    Load hook configuration from YAML file.

    Args:
        config_path: Path to hooks.yaml config file.
                    If None, uses default: devforgeai/config/hooks.yaml

    Returns:
        Configuration dictionary or None if file not found/invalid
    """
    if config_path is None:
        config_path = "devforgeai/config/hooks.yaml"

    config_path = str(config_path)

    # Check if file exists
    if not os.path.exists(config_path):
        logger.warning(f"Hooks config not found at {config_path}, assuming disabled")
        return None

    try:
        with open(config_path, "r") as f:
            config = yaml.safe_load(f)

        if config is None:
            logger.warning(f"Empty hooks config file: {config_path}")
            return None

        return config

    except Exception as e:
        # Consolidated exception handling for YAML parsing, file I/O, and unexpected errors
        logger.error(f"Failed to load hooks config from {config_path}: {e}")
        return None


def _validate_required_string_arg(arg_value: Any, arg_name: str) -> Optional[str]:
    """
    Validate that argument is a non-empty string.

    Helper to reduce duplication in argument validation.

    Args:
        arg_value: Argument value to validate
        arg_name: Name of argument for error messages

    Returns:
        Stripped string if valid, None if invalid
    """
    if not arg_value or not isinstance(arg_value, str) or not arg_value.strip():
        logger.error(f"Invalid {arg_name}: {arg_name} is required and must be non-empty")
        return None
    return arg_value.strip()


def check_hooks_command(
    operation: str,
    status: str,
    config_path: Optional[str] = None,
    hook_type: str = "all",
) -> int:
    """
    Main check-hooks command implementation.

    Determines if hooks should trigger based on configuration and rules.

    Args:
        operation: Operation name (e.g., 'dev', 'qa', 'release')
        status: Operation status ('success', 'failure', or 'partial')
        config_path: Path to hooks.yaml config file (optional)
        hook_type: Hook type filter ('user', 'ai', or 'all') (STORY-185)

    Returns:
        Exit code:
          0 - Hooks should trigger
          1 - Hooks should not trigger
          2 - Error (invalid arguments)
    """
    # AC7: Check for circular invocation
    if os.environ.get("DEVFORGEAI_HOOK_ACTIVE"):
        logger.warning("Circular invocation detected (DEVFORGEAI_HOOK_ACTIVE set), skipping hook")
        return EXIT_CODE_DONT_TRIGGER

    # AC6: Validate arguments - operation must be non-empty string
    operation = _validate_required_string_arg(operation, "operation")
    if operation is None:
        return EXIT_CODE_ERROR

    # AC6: Validate arguments - status must be non-empty string
    status = _validate_required_string_arg(status, "status")
    if status is None:
        return EXIT_CODE_ERROR

    # Validate status against allowed values
    if status not in CheckHooksValidator.VALID_STATUSES:
        logger.error(
            f"Invalid status: '{status}' must be one of {CheckHooksValidator.VALID_STATUSES}"
        )
        return EXIT_CODE_ERROR

    # STORY-185: Validate hook_type against allowed values
    if hook_type not in CheckHooksValidator.VALID_HOOK_TYPES:
        logger.error(
            f"Invalid hook_type: '{hook_type}' must be one of {CheckHooksValidator.VALID_HOOK_TYPES}"
        )
        return EXIT_CODE_ERROR

    # Load configuration
    config = load_config(config_path)

    # AC5: Handle missing config
    if config is None:
        # Config not found or empty - treat as disabled
        return EXIT_CODE_DONT_TRIGGER

    # AC1: Check if hooks are enabled
    if not config.get("enabled", False):
        logger.warning("Hooks are disabled in configuration")
        return EXIT_CODE_DONT_TRIGGER

    # Create validator with hook_type filtering (STORY-185: AC-3)
    try:
        validator = CheckHooksValidator(config, hook_type=hook_type)
    except Exception as e:
        logger.error(f"Failed to initialize hooks validator: {e}")
        return EXIT_CODE_ERROR

    # AC2 & AC3: Evaluate trigger rules and operation-specific overrides
    if validator.should_trigger(operation, status):
        return EXIT_CODE_TRIGGER
    else:
        return EXIT_CODE_DONT_TRIGGER


def _create_argument_parser() -> "argparse.ArgumentParser":
    """
    Create and configure argument parser for check-hooks command.

    Returns:
        Configured ArgumentParser instance
    """
    import argparse

    parser = argparse.ArgumentParser(
        description="Check if hooks should trigger for an operation",
        prog="devforgeai check-hooks",
    )

    parser.add_argument(
        "--operation",
        required=True,
        help="Operation name (e.g., dev, qa, release)",
    )

    parser.add_argument(
        "--status",
        required=True,
        choices=["success", "failure", "partial"],
        help="Operation status",
    )

    parser.add_argument(
        "--config",
        default=None,
        help="Path to hooks.yaml config file (default: devforgeai/config/hooks.yaml)",
    )

    # STORY-185: Add --type argument for hook type filtering
    parser.add_argument(
        "--type",
        type=str,
        choices=["user", "ai", "all"],
        default="all",
        help="Hook type to check (user, ai, or all)",
    )

    return parser


def main():
    """CLI entry point for check-hooks command."""
    parser = _create_argument_parser()
    args = parser.parse_args()

    exit_code = check_hooks_command(
        operation=args.operation,
        status=args.status,
        config_path=args.config,
        hook_type=args.type,  # STORY-185: Pass hook_type from CLI
    )

    sys.exit(exit_code)


if __name__ == "__main__":
    main()
