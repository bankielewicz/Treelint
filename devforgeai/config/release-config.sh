#!/usr/bin/env bash
#
# DevForgeAI Release Configuration
#
# This file contains customizable settings for the release automation script.
# Sourced by scripts/release.sh during execution.

# =============================================================================
# GENERAL SETTINGS
# =============================================================================

# Dry run mode (simulate without external changes)
# Override with: bash scripts/release.sh --dry-run
DRY_RUN="${DRY_RUN:-false}"

# Auto-confirm mode (skip interactive prompts for CI)
# Override with: bash scripts/release.sh --yes
AUTO_YES="${AUTO_YES:-false}"

# =============================================================================
# CHECKSUM CONFIGURATION
# =============================================================================

# Checksum algorithm (sha256 or sha512)
# Only SHA-256 and SHA-512 are allowed for security
CHECKSUM_ALGORITHM="${CHECKSUM_ALGORITHM:-sha256}"

# Validate checksum algorithm
case "$CHECKSUM_ALGORITHM" in
    sha256|sha512)
        # Valid
        ;;
    *)
        echo "ERROR: Invalid CHECKSUM_ALGORITHM: $CHECKSUM_ALGORITHM"
        echo "Allowed values: sha256, sha512"
        exit 1
        ;;
esac

# =============================================================================
# NPM REGISTRY CONFIGURATION
# =============================================================================

# NPM registry URL
# Default: Public npm registry
# Alternative: Private registry (e.g., https://registry.company.com)
NPM_REGISTRY="${NPM_REGISTRY:-https://registry.npmjs.org}"

# Validate NPM registry URL format
if [[ ! "$NPM_REGISTRY" =~ ^https?:// ]]; then
    echo "ERROR: Invalid NPM_REGISTRY URL: $NPM_REGISTRY"
    echo "Must start with http:// or https://"
    exit 1
fi

# =============================================================================
# SYNC EXCLUSION PATTERNS (CFG-001)
# =============================================================================

# Patterns to exclude when syncing .claude/ → src/claude/
# Format: Bash array of glob patterns
CLAUDE_EXCLUDE_PATTERNS=(
    "*.backup*"        # Backup files (.backup, .md.backup, etc.)
    "__pycache__/"     # Python bytecode cache directories
    "*.pyc"            # Python bytecode files
    ".DS_Store"        # macOS metadata files
    "*.log"            # Log files
    ".env"             # Environment files (security)
    "*.key"            # Key files (security)
    "secrets/"         # Secrets directory (security)
)

# Patterns to exclude when syncing devforgeai/ → src/devforgeai/
DEVFORGEAI_EXCLUDE_PATTERNS=(
    "backups/"           # Backup directory
    "qa/reports/"        # Generated QA reports
    "feedback/sessions/" # Feedback session data
    "*.log"              # Log files
    "*.backup*"          # Backup files
    ".env"               # Environment files (security)
    "*.key"              # Key files (security)
    "secrets/"           # Secrets directory (security)
)

# =============================================================================
# GITHUB RELEASE CONFIGURATION
# =============================================================================

# GitHub repository (detected automatically from git remote)
# Override if needed: GITHUB_REPO="owner/repo"
GITHUB_REPO="${GITHUB_REPO:-}"

# Default branch for releases
DEFAULT_BRANCH="${DEFAULT_BRANCH:-main}"

# =============================================================================
# VALIDATION THRESHOLDS
# =============================================================================

# Minimum number of checksum entries required
# Framework has 100+ files, set minimum to 50 for validation
MIN_CHECKSUM_ENTRIES="${MIN_CHECKSUM_ENTRIES:-50}"

# =============================================================================
# LOGGING CONFIGURATION
# =============================================================================

# Release log directory
RELEASE_LOG_DIR="${RELEASE_LOG_DIR:-devforgeai/releases}"

# Log level (INFO, DEBUG, ERROR)
LOG_LEVEL="${LOG_LEVEL:-INFO}"

# =============================================================================
# PERFORMANCE TUNING
# =============================================================================

# Use rsync for sync operations (faster for large file counts)
# Falls back to cp if rsync not available
USE_RSYNC="${USE_RSYNC:-true}"

# Parallel checksum generation (if supported)
PARALLEL_CHECKSUMS="${PARALLEL_CHECKSUMS:-false}"

# =============================================================================
# SECURITY SETTINGS
# =============================================================================

# GPG sign git tags (requires GPG key configured)
GPG_SIGN_TAGS="${GPG_SIGN_TAGS:-false}"

# GPG sign commits
GPG_SIGN_COMMITS="${GPG_SIGN_COMMITS:-false}"

# =============================================================================
# CI/CD INTEGRATION
# =============================================================================

# Detect CI environment
if [[ -n "${CI:-}" ]] || [[ -n "${GITHUB_ACTIONS:-}" ]]; then
    # Running in CI - use CI-friendly defaults
    AUTO_YES=true
    NO_COLOR=1
fi

# =============================================================================
# VALIDATION
# =============================================================================

# Validate configuration on source
echo "✓ Release configuration loaded"
