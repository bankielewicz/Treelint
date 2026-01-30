"""
Skip tracking functionality.

Tracks when users skip feedback and triggers suggestions after 3+ consecutive skips.
"""

import yaml
import os
import logging
from pathlib import Path
from typing import Optional, Callable


# Configure logging
logger = logging.getLogger(__name__)


# ============================================================================
# CONFIG FILE I/O (Private)
# ============================================================================

def _get_config_file(config_dir: Optional[Path] = None) -> Path:
    """
    Get feedback config file path.

    Args:
        config_dir: Config directory (default: devforgeai/config)

    Returns:
        Path to feedback-preferences.yaml (per STORY-009 specification)
    """
    if config_dir is None:
        config_dir = Path.cwd() / 'devforgeai' / 'config'

    config_dir.mkdir(parents=True, exist_ok=True)
    return config_dir / 'feedback-preferences.yaml'


def _load_config(config_file: Path) -> dict:
    """
    Load config from YAML file.

    Args:
        config_file: Path to config file

    Returns:
        Config dictionary (defaults to empty skip_counts if missing)
    """
    if not config_file.exists():
        return {'skip_counts': {}}

    with open(config_file, 'r') as f:
        config = yaml.safe_load(f)

    return config or {'skip_counts': {}}


def _save_config(config: dict, config_file: Path) -> None:
    """
    Save config to YAML file with proper permissions.

    Sets file permissions to mode 600 (user-readable/writable only).

    Args:
        config: Config dictionary
        config_file: Path to config file
    """
    with open(config_file, 'w') as f:
        yaml.safe_dump(config, f, default_flow_style=False)

    # Set file permissions to 600 (user read/write only)
    try:
        os.chmod(config_file, 0o600)
        logger.debug(f"Set config file permissions to 600: {config_file}")
    except OSError as e:
        logger.warning(f"Could not set config file permissions: {e}")


def validate_config_permissions(config_file: Path) -> bool:
    """
    Validate that config file has appropriate permissions (mode 600).

    Args:
        config_file: Path to config file

    Returns:
        True if permissions are 600, False otherwise
    """
    if not config_file.exists():
        return True  # File doesn't exist yet

    try:
        file_stat = config_file.stat()
        # Extract permission bits
        permissions = file_stat.st_mode & 0o777
        is_valid = permissions == 0o600

        if not is_valid:
            logger.warning(
                f"Config file has insecure permissions: "
                f"{oct(permissions)} (should be 0o600)"
            )

        return is_valid
    except OSError as e:
        logger.warning(f"Could not validate config file permissions: {e}")
        return False


def _apply_config_modification(config_file: Path, modifier_fn: Callable[[dict], dict]) -> None:
    """
    Apply modification to config atomically (DRY helper).

    Encapsulates read-modify-write pattern to reduce duplication.

    Args:
        config_file: Path to config file
        modifier_fn: Function that receives config dict and returns modified dict
    """
    config = _load_config(config_file)
    modified_config = modifier_fn(config)
    _save_config(modified_config, config_file)


# ============================================================================
# SKIP COUNTER OPERATIONS (Public)
# ============================================================================

def increment_skip(operation_type: str, config_dir: Optional[Path] = None) -> int:
    """
    Increment skip count for operation type.

    Args:
        operation_type: Operation type (e.g., 'skill_invocation', 'subagent_invocation')
        config_dir: Config directory

    Returns:
        New skip count
    """
    config_file = _get_config_file(config_dir)

    def modify_config(config):
        if 'skip_counts' not in config:
            config['skip_counts'] = {}

        current_count = config['skip_counts'].get(operation_type, 0)
        new_count = current_count + 1
        config['skip_counts'][operation_type] = new_count
        return config

    _apply_config_modification(config_file, modify_config)

    # Reload to get updated count
    config = _load_config(config_file)
    return config['skip_counts'][operation_type]


def get_skip_count(operation_type: str, config_dir: Optional[Path] = None) -> int:
    """
    Get current skip count for operation type.

    Args:
        operation_type: Operation type (e.g., 'skill_invocation', 'subagent_invocation')
        config_dir: Config directory

    Returns:
        Current skip count
    """
    config_file = _get_config_file(config_dir)
    config = _load_config(config_file)
    return config.get('skip_counts', {}).get(operation_type, 0)


def reset_skip_count(operation_type: str, config_dir: Optional[Path] = None) -> None:
    """
    Reset skip count for operation type to 0.

    Args:
        operation_type: Operation type (e.g., 'skill_invocation', 'subagent_invocation')
        config_dir: Config directory
    """
    config_file = _get_config_file(config_dir)

    def modify_config(config):
        if 'skip_counts' not in config:
            config['skip_counts'] = {}

        config['skip_counts'][operation_type] = 0
        return config

    _apply_config_modification(config_file, modify_config)


def check_skip_threshold(operation_type: str, threshold: int = 3, config_dir: Optional[Path] = None) -> bool:
    """
    Check if operation type has reached skip threshold.

    Args:
        operation_type: Operation type (e.g., 'skill_invocation', 'subagent_invocation')
        threshold: Skip threshold (default: 3)
        config_dir: Config directory

    Returns:
        True if threshold reached, False otherwise
    """
    count = get_skip_count(operation_type, config_dir)
    return count >= threshold


# ============================================================================
# PUBLIC API (Export for external use)
# ============================================================================

__all__ = [
    'increment_skip',
    'get_skip_count',
    'reset_skip_count',
    'check_skip_threshold',
    'validate_config_permissions',
]

