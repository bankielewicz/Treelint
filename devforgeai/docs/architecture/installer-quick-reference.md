# Installer Quick Reference

> One-page reference for DevForgeAI installer commands and configurations.

## CLI Commands

```bash
# Standard installation
python -m installer install /path/to/project

# Force installation (skip confirmations)
python -m installer install /path/to/project --force

# Interactive wizard
python -m installer wizard /path/to/project

# Validate existing installation
python -m installer validate /path/to/project

# Rollback to previous version
python -m installer rollback /path/to/project

# Uninstall framework
python -m installer uninstall /path/to/project
```

## Exit Codes

| Code | Meaning |
|:----:|---------|
| 0 | Success |
| 1 | Configuration error |
| 2 | Pre-flight check failed |
| 3 | Rollback occurred |
| 4 | Validation failed |
| 5 | Bundle corruption (offline) |

## Environment Variables (Silent Mode)

```bash
export DEVFORGEAI_TARGET=/opt/project
export DEVFORGEAI_COMPONENTS=core,cli,templates
export DEVFORGEAI_LOG_FILE=install.log
export DEVFORGEAI_INIT_GIT=true
export DEVFORGEAI_DRY_RUN=false
```

## Silent Mode YAML Config

```yaml
# install-config.yaml
target: /opt/devforgeai
components:
  - core      # Required
  - cli       # Optional
  - templates # Optional
  - examples  # Optional
options:
  initialize_git: true
  create_backup: true
  run_validation: true
  dry_run: false
log_file: install.log
```

## Installation Modes

| Mode | Trigger |
|------|---------|
| `fresh_install` | No existing installation |
| `patch_upgrade` | X.Y.Z → X.Y.Z+1 |
| `minor_upgrade` | X.Y.Z → X.Y+1.Z |
| `major_upgrade` | X.Y.Z → X+1.Y.Z |
| `reinstall` | Same version |
| `downgrade` | Source < Installed |

## Components

| ID | Name | Required | Size |
|----|------|:--------:|-----:|
| `core` | Core Framework | Yes | 2.5 MB |
| `cli` | CLI Tools | No | 0.5 MB |
| `templates` | Templates | No | 1.0 MB |
| `examples` | Examples | No | 3.0 MB |

## Directory Structure (Post-Install)

```
project/
├── .claude/
│   ├── skills/      # 10+ skills
│   ├── agents/      # Subagent definitions
│   ├── commands/    # 11+ commands
│   └── memory/      # Reference docs
├── devforgeai/
│   ├── protocols/   # 3+ protocols
│   ├── context/     # User configs (preserved)
│   ├── config/      # Framework configs
│   └── .version.json
├── .backups/        # Timestamped backups
└── CLAUDE.md        # Project instructions
```

## Backup Location

```
.backups/devforgeai-upgrade-YYYYMMDD-HHMMSS-μs/
├── .claude/
├── devforgeai/
├── CLAUDE.md
└── manifest.json
```

## Offline Bundle Creation

```python
from installer.offline import OfflineBundler

bundler = OfflineBundler(
    source_dir=Path("."),
    output=Path("devforgeai-bundle.tar.gz")
)
bundler.create_bundle()
```

## Offline Installation

```python
from installer.offline import OfflineInstaller

installer = OfflineInstaller(
    bundle_path=Path("devforgeai-bundle.tar.gz"),
    target=Path("/opt/project")
)
result = installer.install()
```

## OS-Specific Installer Generation

```python
from installer.installer_generator import InstallerGenerator

gen = InstallerGenerator(output_dir=Path("./installers"))

# Generate all platforms
configs = gen.generate_all(package_info={
    "name": "DevForgeAI",
    "version": "1.0.0",
    "publisher": "DevForgeAI Team"
})

# Generate specific platform
config = gen.generate(
    platform="windows",
    format="msi",
    package_info={...}
)
```

## Validation Thresholds

| Resource | Minimum |
|----------|--------:|
| Commands | 11 |
| Skills | 10 |
| Protocols | 3 |

## Preserved Paths (Not Overwritten)

```
devforgeai/config/hooks.yaml
devforgeai/feedback/config.yaml
devforgeai/specs/context/*
```

## File Permissions

| Type | Mode |
|------|------|
| Directories | 755 |
| Scripts (*.sh) | 755 |
| Regular files | 644 |

---

> **Full Documentation:** [installer-architecture.md](./installer-architecture.md)
