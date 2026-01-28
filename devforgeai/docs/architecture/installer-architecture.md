# DevForgeAI Installer Architecture

> **Version:** 1.0.0
> **Last Updated:** 2025-01-11
> **Status:** Reference Documentation

## Table of Contents

1. [Overview](#overview)
2. [Architecture Diagram](#architecture-diagram)
3. [Module Reference](#module-reference)
4. [Installation Modes](#installation-modes)
5. [Core Workflows](#core-workflows)
6. [Data Flow](#data-flow)
7. [Exit Codes](#exit-codes)
8. [Security Considerations](#security-considerations)
9. [Design Principles](#design-principles)

---

## Overview

The DevForgeAI installer is a **modular, multi-mode installation system** located in `/installer/` that supports various deployment scenarios from CI/CD pipelines to interactive desktop installations.

### Key Capabilities

- **5 Installation Modes**: Fresh install, upgrade, rollback, validate, uninstall
- **4 User Interfaces**: CLI, Wizard, Silent (CI/CD), GUI configurations
- **Cross-Platform**: Windows (MSI/NSIS), Linux (DEB/RPM), macOS (PKG)
- **Offline Support**: Air-gapped installation with bundled dependencies
- **Atomic Transactions**: Backup-first approach with automatic rollback on failure

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          ENTRY POINTS                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│  __main__.py          CLI entry point (python -m installer)                  │
│  install()            Main orchestrator function                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                       INSTALLATION MODES                                     │
├──────────────────┬─────────────────┬──────────────────┬─────────────────────┤
│  Wizard Mode     │  Silent Mode    │  Offline Mode    │  CLI Mode           │
│  (wizard.py)     │  (silent.py)    │  (offline.py)    │  (install.py)       │
│  - Interactive   │  - CI/CD ready  │  - Air-gapped    │  - Standard CLI     │
│  - 6-step wizard │  - YAML config  │  - Bundled       │  - Force option     │
│  - Component     │  - JSON output  │  - SHA256 verify │  - Auto-detect      │
│    selection     │  - Dry-run      │  - Delta updates │    mode             │
├──────────────────┴─────────────────┴──────────────────┴─────────────────────┤
│                           CORE MODULES                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│  deploy.py       File deployment (.claude/, devforgeai/) with exclusions     │
│  backup.py       Timestamped backups with SHA256 integrity hashes            │
│  rollback.py     Restore from backup, handles list/verify operations         │
│  version.py      Semantic versioning, mode detection (fresh/upgrade/etc.)    │
│  validate.py     Installation validation (dirs, files, CLI, version.json)    │
│  merge.py        CLAUDE.md merge (preserve user content + framework updates) │
├─────────────────────────────────────────────────────────────────────────────┤
│                      CONFIGURATION & GENERATION                              │
├─────────────────────────────────────────────────────────────────────────────┤
│  installer_mode_config.py   Mode-specific configs (cli/wizard/silent/gui)    │
│  installer_generator.py     OS-specific installers (MSI/NSIS/DEB/RPM/PKG)    │
│  variables.py               Template variable substitution                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                           SERVICES                                           │
├─────────────────────────────────────────────────────────────────────────────┤
│  services/                                                                   │
│  ├─ rollback_service.py     AC#4/AC#8 compliant rollback (security-hardened) │
│  ├─ backup_service.py       Backup creation and verification                 │
│  ├─ install_logger.py       Structured logging with ISO 8601 timestamps      │
│  ├─ lock_file_manager.py    Prevents concurrent installations                │
│  ├─ error_categorizer.py    Error classification and recovery hints          │
│  └─ error_recovery_orchestrator.py  Recovery workflow orchestration          │
├─────────────────────────────────────────────────────────────────────────────┤
│                        SUPPORTING MODULES                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│  network.py      Network availability detection                              │
│  checksum.py     SHA256 file/bundle verification                             │
│  bundle.py       Bundle structure verification                               │
│  exit_codes.py   Standardized exit codes (0=success, 1-5=various failures)   │
│  preflight.py    Pre-installation validation                                 │
│  platform_detector.py  OS/environment detection                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Module Reference

### Entry Points

#### `__main__.py`
CLI entry point enabling `python -m installer` invocation.

```bash
# Usage
python -m installer install <target_path> [--force]
python -m installer wizard <target_path>
python -m installer validate <target_path>
python -m installer rollback <target_path>
python -m installer uninstall <target_path>
```

#### `install.py` - Main Orchestrator

The central orchestration module coordinating the complete installation workflow.

**Key Functions:**
- `install(target_path, source_path, mode, force)` - Main entry point
- `_detect_installation_mode()` - Auto-detects fresh vs upgrade
- `_handle_rollback_mode()` - Rollback workflow
- `_handle_uninstall_mode()` - Uninstall workflow
- `_handle_claude_md_merge()` - CLAUDE.md preservation

**Installation Modes:**
| Mode | Description |
|------|-------------|
| `fresh_install` | No existing installation detected |
| `patch_upgrade` | 1.0.0 → 1.0.1 (patch version change) |
| `minor_upgrade` | 1.0.0 → 1.1.0 (backward compatible) |
| `major_upgrade` | 1.0.0 → 2.0.0 (breaking changes) |
| `reinstall` | Same version re-installation |
| `downgrade` | Source version older than installed |

---

### Installation Mode Modules

#### `wizard.py` (STORY-247)

Interactive CLI wizard with 7-step installation flow.

**Steps:**
1. **Welcome** - Framework info display
2. **License** - MIT license acceptance (requires "accept")
3. **Path Selection** - Target directory with write validation
4. **Component Selection** - Core (required), CLI, Templates, Examples
5. **Configuration** - Git integration options
6. **Installation** - Progress display with real-time updates
7. **Completion** - Summary and next steps

**Business Rules:**
- BR-001: Core Framework cannot be deselected
- BR-002: License requires 'accept' text (case-insensitive)
- BR-003: Path validated for write permissions (`os.access`)
- BR-004: Git options disabled if Git unavailable (`shutil.which`)
- BR-005: Progress callback invoked per step

**Components:**
```python
DEFAULT_COMPONENTS = [
    Component(id="core", name="Core Framework", size_mb=2.5, required=True),
    Component(id="cli", name="CLI Tools", size_mb=0.5, required=False),
    Component(id="templates", name="Templates", size_mb=1.0, required=False),
    Component(id="examples", name="Examples", size_mb=3.0, required=False),
]
```

#### `silent.py` (STORY-249)

Configuration-driven silent installation for CI/CD pipelines.

**Features:**
- YAML configuration files (AC#1)
- Environment variable configuration (AC#2)
- No interactive prompts (AC#3)
- Structured logging with ISO 8601 timestamps (AC#4)
- CI/CD exit codes (AC#5)
- Dry-run mode (AC#6)
- Idempotency checking (AC#7)
- JSON progress output (AC#8)

**Configuration Sources (Priority Order):**
1. Environment variables (`DEVFORGEAI_*`)
2. YAML config file
3. Python dict

**Environment Variables:**
```bash
DEVFORGEAI_TARGET=/opt/project
DEVFORGEAI_COMPONENTS=core,cli
DEVFORGEAI_LOG_FILE=install.log
DEVFORGEAI_INIT_GIT=true
DEVFORGEAI_DRY_RUN=false
```

**Example YAML Config:**
```yaml
target: /opt/devforgeai
components:
  - core
  - cli
options:
  initialize_git: true
  create_backup: true
  run_validation: true
  dry_run: false
log_file: install.log
```

#### `offline.py` (STORY-250)

Air-gapped installation support with bundled dependencies.

**Classes:**

| Class | Purpose |
|-------|---------|
| `OfflineBundler` | Creates tar.gz bundles with SHA256 checksums |
| `BundleVerifier` | Verifies bundle integrity before installation |
| `OfflineInstaller` | Extracts and installs from offline bundles |

**Bundle Structure:**
```
bundle.tar.gz
├── manifest.yaml     # SHA256 checksums for all files
├── metadata.json     # Version, creation date, components
├── install.py        # Standalone extraction script
└── payload/          # Framework files
    ├── .claude/
    └── devforgeai/
```

**Security Features:**
- Path traversal protection (CVE-2007-4559 mitigation)
- SHA256 integrity verification
- Absolute path validation before extraction

**Incremental Updates:**
```python
bundler = OfflineBundler(source_dir=Path("."), output=Path("delta.tar.gz"))
bundler.create_incremental_bundle(base_version="1.0.0", base_bundle=Path("base.tar.gz"))
```

#### `installer_mode_config.py` (STORY-243)

Generates mode-specific installer configurations.

**Supported Modes:**
| Mode | Description |
|------|-------------|
| `cli` | Interactive command-line with progress spinner |
| `wizard` | 6-step guided installation |
| `silent` | Automated CI/CD installation |
| `gui` | Graphical desktop installation |

**Standard Wizard Steps:**
```python
WIZARD_STEPS = [
    InstallerStep(id="welcome", title="Welcome", type="info"),
    InstallerStep(id="license", title="License Agreement", type="license"),
    InstallerStep(id="path", title="Installation Path", type="path_select"),
    InstallerStep(id="components", title="Component Selection", type="component_select"),
    InstallerStep(id="install", title="Installing", type="progress"),
    InstallerStep(id="complete", title="Complete", type="complete"),
]
```

#### `installer_generator.py` (STORY-242)

Generates OS-specific installer configurations.

**Platform Support:**

| Platform | Format | Tool Required | Build Command |
|----------|--------|---------------|---------------|
| Windows | MSI (.wxs) | WiX Toolset | `candle + light` |
| Windows | NSIS (.nsi) | NSIS | `makensis` |
| Linux (Debian) | DEB | dpkg-deb | `dpkg-deb --build` |
| Linux (RHEL) | RPM (.spec) | rpmbuild | `rpmbuild -bb` |
| macOS | PKG | pkgbuild | `pkgbuild + productbuild` |

**Usage:**
```python
generator = InstallerGenerator(output_dir=Path("./installers"))
configs = generator.generate_all(package_info={
    "name": "DevForgeAI",
    "version": "1.0.0",
    "publisher": "DevForgeAI Team",
    "description": "AI-powered development framework"
})
```

---

### Core Modules

#### `deploy.py`

Framework file deployment with exclusions and permissions management.

**Source Mapping:**
```
src/claude/      → target/.claude/     (~370 files)
src/devforgeai/  → target/devforgeai/  (~80 files)
```

**Exclusion Patterns:**
```python
EXCLUDE_PATTERNS = {
    "*.backup", "*.bak",           # Backup files
    "__pycache__", "*.pyc",        # Python cache
    ".pytest_cache", ".coverage",  # Test artifacts
    "*-phase-state.lock",          # Workflow locks
    "settings.local.json",         # Local settings
}

NO_DEPLOY_DIRS = {
    "devforgeai/qa/reports",       # Generated reports
    "devforgeai/RCA",              # Root cause analysis
    "devforgeai/workflows",        # Session-specific
    "tests", "build", "dist",      # Development dirs
}
```

**Preserved Paths (User Configs):**
```python
PRESERVE_PATHS = {
    "devforgeai/config/hooks.yaml",
    "devforgeai/feedback/config.yaml",
    "devforgeai/specs/context/",
}
```

**Permissions:**
| Type | Mode | Description |
|------|------|-------------|
| Directories | 755 | rwxr-xr-x |
| Shell scripts (*.sh) | 755 | rwxr-xr-x |
| Regular files | 644 | rw-r--r-- |

#### `backup.py`

Timestamped backup creation with integrity verification.

**Backup Location:**
```
.backups/devforgeai-upgrade-YYYYMMDD-HHMMSS-μs/
├── .claude/
├── devforgeai/
├── CLAUDE.md
└── manifest.json
```

**Manifest Schema:**
```json
{
  "created_at": "2025-01-11T12:00:00Z",
  "reason": "upgrade",
  "from_version": "1.0.0",
  "to_version": "1.1.0",
  "files_backed_up": 450,
  "total_size_mb": 5.2,
  "backup_integrity_hash": "sha256:abc123..."
}
```

**Integrity Verification:**
```python
result = verify_backup_integrity(backup_path)
# Returns: {valid, file_count, manifest_file_count, hash_matches, errors}
```

#### `version.py`

Semantic versioning and installation mode detection.

**Version File Locations:**
- Installed: `devforgeai/.version.json`
- Source: `src/devforgeai/version.json`

**Version Comparison:**
```python
mode = compare_versions("1.0.0", "1.1.0")  # Returns: "minor_upgrade"
```

**Mode Detection Logic:**
```
installed == None         → fresh_install
source == installed       → reinstall
source < installed        → downgrade
source.major > installed  → major_upgrade
source.minor > installed  → minor_upgrade
source.patch > installed  → patch_upgrade
```

#### `validate.py`

Installation validation and health checks.

**Validation Checks:**
1. Directory structure (7 required directories)
2. Critical files (≥11 commands, ≥10 skills, ≥3 protocols)
3. CLAUDE.md presence
4. version.json schema validation
5. CLI installation check

**Required Directories:**
```python
required_dirs = [
    ".claude/skills",
    ".claude/agents",
    ".claude/commands",
    ".claude/memory",
    "devforgeai/protocols",
    "devforgeai/context",
    "devforgeai/adrs",
]
```

---

### Services

#### `services/rollback_service.py`

Security-hardened rollback operations (AC#4, AC#8 compliant).

**Security Features:**
- Path boundary validation (`os.path.abspath` + `startswith`)
- Prevents arbitrary file deletion
- All delete operations validated against `installation_root`

**Rollback Workflow:**
1. Display "Rolling back installation..."
2. Restore all files from backup directory
3. Remove partially copied files not in backup
4. Clean up empty directories
5. Display "Rollback complete. System restored to pre-installation state."
6. Return exit code 3

**Usage:**
```python
service = RollbackService(logger=logger, installation_root=target_path)
result = service.rollback(backup_dir=backup_path, target_dir=target_path)
# Returns: RollbackResult(exit_code=3, files_restored=N, files_removed=M, ...)
```

#### `services/install_logger.py`

Structured logging with ISO 8601 timestamps.

**Log Format:**
```
YYYY-MM-DDTHH:MM:SSZ [LEVEL] module: message
```

#### `services/lock_file_manager.py`

Prevents concurrent installations using file-based locking.

#### `services/error_categorizer.py`

Error classification with recovery hints.

#### `services/error_recovery_orchestrator.py`

Recovery workflow orchestration for failed installations.

---

## Installation Modes

### Mode Comparison

| Feature | CLI | Wizard | Silent | Offline |
|---------|-----|--------|--------|---------|
| Interactive | Yes | Yes | No | No |
| Component Selection | No | Yes | Config | Config |
| Progress Display | Basic | Rich | JSON | Basic |
| Network Required | Yes | Yes | No | No |
| CI/CD Ready | Limited | No | Yes | Yes |
| Configuration Source | Args | Interactive | YAML/Env | Bundle |

---

## Core Workflows

### Fresh Installation Flow

```
┌─────────────┐    ┌─────────────┐    ┌──────────────┐    ┌─────────────┐
│  Validate   │    │   Create    │    │   Deploy     │    │  Validate   │
│   Source    │───▶│   Backup    │───▶│   Files      │───▶│ Installation│
│             │    │  (safety)   │    │  + Merge     │    │             │
└─────────────┘    └─────────────┘    └──────────────┘    └─────────────┘
                                            │
                                            ▼
                                     ┌──────────────┐
                                     │   Update     │
                                     │ version.json │
                                     └──────────────┘
```

### Upgrade Flow (with Rollback)

```
┌─────────────┐    ┌─────────────┐    ┌──────────────┐
│   Detect    │    │   Create    │    │   Deploy     │
│   Version   │───▶│   Backup    │───▶│   Files      │
│             │    │  (atomic)   │    │  + Merge     │
└─────────────┘    └─────────────┘    └──────────────┘
                          │                  │
                          │    On Failure    │
                          │◀────────────────▶│
                          │     Rollback     │
                          │  (auto-restore)  │
                          └──────────────────┘
```

### Offline Installation Flow

```
┌─────────────┐    ┌─────────────┐    ┌──────────────┐    ┌─────────────┐
│   Verify    │    │   Verify    │    │   Extract    │    │  Validate   │
│   Bundle    │───▶│  Checksums  │───▶│   Payload    │───▶│   Install   │
│  Exists     │    │   (SHA256)  │    │   Files      │    │             │
└─────────────┘    └─────────────┘    └──────────────┘    └─────────────┘
       │
       │ On Corruption
       ▼
┌─────────────┐
│  Exit Code  │
│      5      │
└─────────────┘
```

---

## Data Flow

### File Deployment Flow

```
Source (src/)
    │
    ├── claude/           ──────────────────▶  .claude/
    │   ├── skills/                              ├── skills/
    │   ├── agents/                              ├── agents/
    │   ├── commands/                            ├── commands/
    │   └── memory/                              └── memory/
    │
    └── devforgeai/       ──────────────────▶  devforgeai/
        ├── protocols/                           ├── protocols/
        ├── context/      (preserved if exists)  ├── context/
        └── config/       (preserved if exists)  └── config/

CLAUDE.md                 ──── merge ────────▶  CLAUDE.md
                          (preserve user content)
```

---

## Exit Codes

| Code | Constant | Meaning | Recovery Action |
|------|----------|---------|-----------------|
| 0 | `SUCCESS` | Installation successful | None |
| 1 | `CONFIG_ERROR` | Configuration invalid | Fix config file |
| 2 | `PREFLIGHT_ERROR` | Pre-flight check failed | Check prerequisites |
| 3 | `ROLLBACK_OCCURRED` | Installation rolled back | Check logs, retry |
| 4 | `VALIDATION_FAILED` | Post-install validation failed | Manual verification |
| 5 | `BUNDLE_CORRUPTION` | Offline bundle corrupted | Re-download bundle |

---

## Security Considerations

### Path Traversal Prevention

All file operations validate paths against the installation root:

```python
def _validate_path_within_root(self, path: Path) -> bool:
    path_abs = os.path.abspath(path)
    root_abs = os.path.abspath(self.installation_root)
    return path_abs.startswith(root_abs + os.sep) or path_abs == root_abs
```

### Sensitive File Exclusions

Files with potential secrets are never deployed:

```python
EXCLUDE_PATTERNS = {
    "settings.json.glm",        # Contains hardcoded API token
    "settings.json.anthropic",  # Platform-specific settings
    "settings.local.json",      # Local settings
}
```

### Bundle Integrity Verification

Offline bundles are verified before extraction:

1. Bundle format validation (valid tar.gz)
2. Manifest presence check
3. SHA256 checksum verification for all files
4. Path traversal check during extraction

### Race Condition Prevention

Backup directories use microsecond timestamps and exclusive creation:

```python
timestamp = now.strftime("%Y%m%d-%H%M%S-%f")  # Microseconds for uniqueness
backup_path.mkdir(parents=False, exist_ok=False)  # Fails if exists
```

---

## Design Principles

### 1. Atomic Transactions

Every modification is preceded by a backup. On failure, automatic rollback restores the previous state.

### 2. Security-First

- Path validation on all file operations
- No hardcoded secrets in deployed files
- Input sanitization on all user inputs
- CVE-2007-4559 mitigation in bundle extraction

### 3. Zero External Dependencies

Core modules use only Python standard library (`pathlib`, `shutil`, `hashlib`, `json`, `subprocess`).

### 4. Multi-Modal Support

Same codebase supports CLI, wizard, silent (CI/CD), and offline installation modes.

### 5. Idempotency

Silent mode checks for existing installations before re-installing, supporting safe CI/CD reruns.

### 6. Graceful Degradation

Optional components (Python CLI, Git integration) fail gracefully without blocking core installation.

### 7. Cross-Platform Compatibility

- OS detection for permissions, paths, and CLI checks
- Native installer generation for Windows, Linux, macOS
- Platform-specific default paths

---

## Related Documentation

- [Uninstall Usage Guide](../UNINSTALL-USAGE-GUIDE.md)
- [Uninstall Recovery Guide](../UNINSTALL-RECOVERY-GUIDE.md)
- [External Project Setup Guide](../EXTERNAL-PROJECT-SETUP-GUIDE.md)
- [Framework Maintainer Guide](../FRAMEWORK-MAINTAINER-GUIDE.md)

---

## Changelog

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2025-01-11 | Initial documentation |
