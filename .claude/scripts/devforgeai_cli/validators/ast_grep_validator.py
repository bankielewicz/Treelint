"""
AstGrepValidator - ast-grep installation detection and management.

Handles installation detection, version compatibility checking,
interactive prompts, and pip-based installation for STORY-115.
"""

import os
import sys
import re
import shutil
import subprocess
import logging
from pathlib import Path
from typing import Tuple, List, Dict, Optional
from enum import Enum
from dataclasses import dataclass

try:
    import yaml
except ImportError:
    yaml = None  # YAML is optional for basic functionality

logger = logging.getLogger(__name__)


# =============================================================================
# Enums and Data Classes
# =============================================================================

class InstallAction(Enum):
    """User choices for missing ast-grep dependency."""
    INSTALL_NOW = "install"
    USE_FALLBACK = "fallback"
    SKIP = "skip"


@dataclass
class VersionInfo:
    """Parsed semantic version information."""
    major: int
    minor: int
    patch: int
    raw: str

    def is_compatible(self, min_version: str = "0.40.0", max_version: str = "1.0.0") -> bool:
        """
        Check if version is within compatible range [min_version, max_version).

        Args:
            min_version: Minimum version (inclusive)
            max_version: Maximum version (exclusive)

        Returns:
            True if version is compatible
        """
        min_ver = parse_version(min_version)
        max_ver = parse_version(max_version)

        if not min_ver or not max_ver:
            return False

        # Check minimum (inclusive)
        if self.major < min_ver.major:
            return False
        if self.major == min_ver.major and self.minor < min_ver.minor:
            return False
        if self.major == min_ver.major and self.minor == min_ver.minor and self.patch < min_ver.patch:
            return False

        # Check maximum (exclusive)
        if self.major > max_ver.major:
            return False
        if self.major == max_ver.major and self.minor >= max_ver.minor:
            return False

        return True


# =============================================================================
# Utility Functions
# =============================================================================

def parse_version(version_string: str) -> Optional[VersionInfo]:
    """
    Parse semantic version string without external libraries.

    Args:
        version_string: Version string like "0.40.0" or "v0.45.0-beta"

    Returns:
        VersionInfo object or None if parsing fails
    """
    if not version_string:
        return None

    # Remove 'v' prefix if present
    version_string = version_string.strip()
    if version_string.startswith('v'):
        version_string = version_string[1:]

    # Extract major.minor.patch (ignore prerelease/build metadata)
    pattern = r'^(\d+)\.(\d+)\.(\d+)'
    match = re.match(pattern, version_string)

    if not match:
        return None

    try:
        major = int(match.group(1))
        minor = int(match.group(2))
        patch = int(match.group(3))
        return VersionInfo(major=major, minor=minor, patch=patch, raw=version_string)
    except (ValueError, IndexError):
        return None


def detect_headless_mode() -> bool:
    """
    Detect if running in CI/headless environment.

    Checks for common CI environment variables and terminal availability.

    Returns:
        True if headless mode detected
    """
    # CI environment variables
    if os.environ.get("CI") == "true":
        return True
    if os.environ.get("DEVFORGEAI_HEADLESS") == "true":
        return True
    if os.environ.get("GITHUB_ACTIONS") == "true":
        return True

    # Non-interactive terminal
    try:
        return not os.isatty(sys.stdin.fileno())
    except Exception:
        return False


def load_config(config_path: Optional[str] = None) -> Dict:
    """
    Load configuration with defaults.

    Args:
        config_path: Path to config.yaml file

    Returns:
        Configuration dictionary with defaults
    """
    default_config = {
        'fallback_mode': False,
        'min_version': '0.40.0',
        'max_version': '1.0.0',
        'allow_auto_install': False
    }

    if not config_path:
        config_path = "devforgeai/ast-grep/config.yaml"

    config_file = Path(config_path)
    if not config_file.exists():
        return default_config

    if not yaml:
        logger.warning("PyYAML not available, using default configuration")
        return default_config

    try:
        with open(config_file, 'r') as f:
            user_config = yaml.safe_load(f) or {}
            # Merge with defaults
            return {**default_config, **user_config}
    except Exception as e:
        logger.warning(f"Failed to load config from {config_path}: {e}")
        return default_config


# =============================================================================
# Main Validator Class
# =============================================================================

class AstGrepValidator:
    """Manages ast-grep installation and validation."""

    def __init__(self, config_path: Optional[str] = None, interactive: bool = True):
        """
        Initialize validator.

        Args:
            config_path: Path to devforgeai/ast-grep/config.yaml
            interactive: Whether to show prompts (False for CI/headless)
        """
        self.config = load_config(config_path)
        self.interactive = interactive and not detect_headless_mode()
        self._ast_grep_available = None  # Lazy initialization

    def is_installed(self) -> bool:
        """
        Check if ast-grep is available on system.

        Returns:
            True if ast-grep found in PATH
        """
        return shutil.which("sg") is not None or shutil.which("ast-grep") is not None

    def get_version(self) -> Optional[VersionInfo]:
        """
        Get installed ast-grep version.

        Returns:
            VersionInfo object or None if detection fails
        """
        ast_grep_cmd = shutil.which("sg") or shutil.which("ast-grep")
        if not ast_grep_cmd:
            return None

        try:
            result = subprocess.run(
                [ast_grep_cmd, "--version"],
                capture_output=True,
                text=True,
                timeout=5
            )

            if result.returncode == 0:
                # Parse version from output like "ast-grep 0.45.0"
                version_match = re.search(r'(\d+\.\d+\.\d+)', result.stdout)
                if version_match:
                    return parse_version(version_match.group(1))

            return None
        except Exception as e:
            logger.debug(f"Failed to get ast-grep version: {e}")
            return None

    def check_version_compatibility(self) -> Tuple[bool, str]:
        """
        Verify version meets requirements.

        Returns:
            Tuple of (is_compatible, message)
        """
        version = self.get_version()

        if not version:
            return False, "Could not determine ast-grep version"

        min_version = self.config.get('min_version', '0.40.0')
        max_version = self.config.get('max_version', '1.0.0')

        if version.is_compatible(min_version, max_version):
            return True, f"ast-grep version {version.raw} is compatible"
        else:
            return False, f"ast-grep version {version.raw} is not compatible (requires >={min_version}, <{max_version})"

    def install_via_pip(self) -> Tuple[bool, str]:
        """
        Install ast-grep-cli via pip.

        Returns:
            Tuple of (success, message)
        """
        try:
            result = subprocess.run(
                [sys.executable, "-m", "pip", "install", "ast-grep-cli>=0.40.0,<1.0.0"],
                capture_output=True,
                text=True,
                timeout=120
            )

            if result.returncode == 0:
                return True, "Successfully installed ast-grep-cli"
            else:
                return False, f"Installation failed: {result.stderr}"

        except subprocess.TimeoutExpired:
            return False, "Installation timeout after 120 seconds"
        except Exception as e:
            return False, f"Installation error: {str(e)}"

    def prompt_missing_dependency(self) -> InstallAction:
        """
        Display interactive prompt for missing ast-grep.

        Returns:
            User's chosen action
        """
        if not self.interactive:
            # In headless mode, use config or default to fallback
            if self.config.get('fallback_mode', False):
                return InstallAction.USE_FALLBACK
            return InstallAction.USE_FALLBACK

        print("\n" + "=" * 60)
        print("  ast-grep Not Found")
        print("=" * 60)
        print("\nast-grep provides semantic code analysis (90-95% accuracy).")
        print("Without it, grep-based fallback will be used (60-75% accuracy).\n")
        print("Options:")
        print("  1) Install now (pip install ast-grep-cli)")
        print("  2) Use fallback (grep-based analysis)")
        print("  3) Skip\n")

        while True:
            try:
                choice = input("Select option [1-3]: ").strip()

                if choice == "1":
                    return InstallAction.INSTALL_NOW
                elif choice == "2":
                    return InstallAction.USE_FALLBACK
                elif choice == "3":
                    return InstallAction.SKIP
                else:
                    print("Invalid choice. Please enter 1, 2, or 3.")
            except (EOFError, KeyboardInterrupt):
                print("\nOperation cancelled.")
                return InstallAction.SKIP

    def validate(self, target_path: str) -> Tuple[bool, List[Dict]]:
        """
        Main validation entry point.

        Args:
            target_path: Directory to scan

        Returns:
            Tuple of (success, violations)
        """
        violations = []

        # Check if ast-grep is installed
        if not self.is_installed():
            # Handle missing dependency
            action = self.prompt_missing_dependency()

            if action == InstallAction.INSTALL_NOW:
                success, message = self.install_via_pip()
                if not success:
                    violations.append({
                        "severity": "HIGH",
                        "error": "ast-grep installation failed",
                        "fix": message,
                        "analysis_method": "none"
                    })
                    return False, violations

            elif action == InstallAction.SKIP:
                violations.append({
                    "severity": "INFO",
                    "error": "ast-grep not available, scan skipped",
                    "fix": "Install ast-grep-cli to enable semantic analysis",
                    "analysis_method": "none"
                })
                return True, violations

            # If USE_FALLBACK, will continue to use grep fallback below

        # Check version compatibility
        if self.is_installed():
            is_compatible, message = self.check_version_compatibility()
            if not is_compatible:
                violations.append({
                    "severity": "MEDIUM",
                    "error": "ast-grep version incompatible",
                    "fix": message,
                    "analysis_method": "version-check"
                })

        # For now, return success (actual scanning will be implemented in integration)
        return True, violations
