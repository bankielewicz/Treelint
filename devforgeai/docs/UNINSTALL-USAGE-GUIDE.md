# DevForgeAI Uninstall Usage Guide

## Overview

The DevForgeAI uninstall system provides safe removal of the framework from your project while protecting your work (stories, epics, sprints, and custom configurations).

**Default Behavior:** Preserve user content (safest option)
**Explicit Choice:** Complete removal (removes everything)

---

## Basic Usage

### Uninstall with User Content Preservation (Default)

Remove DevForgeAI framework files but keep all your work:

```bash
devforgeai uninstall
```

**What happens:**
- Removes: `.claude/`, `devforgeai/framework files`, CLI binaries
- Preserves: `.ai_docs/`, custom ADRs, user-modified context files
- Creates backup of entire installation (for recovery)
- Displays confirmation prompt with file counts
- Shows backup location for later restoration

### Preview Changes (Dry-Run)

See exactly what will be removed without making changes:

```bash
devforgeai uninstall --dry-run
```

**Output:**
```
Dry-Run Mode: No files will be modified

Framework files to remove:
  - .claude/skills/  (45 files)
  - .claude/agents/  (20 files)
  - devforgeai/context/ (unmodified - removed)
  - CLAUDE.md (if unmodified)

User content to preserve:
  - .ai_docs/ (123 files)
  - devforgeai/adrs/custom/ (8 files)
  - devforgeai/config/ (3 files)

Disk space that would be freed: 2.3 MB
Backup would be created at: ~/devforgeai/backups/uninstall-2025-12-08T10-30-00/
```

### Skip Confirmation Prompt

Bypass the confirmation prompt (useful for scripting):

```bash
devforgeai uninstall --yes
```

**⚠️ Warning:** This immediately starts uninstall without confirmation

### Complete Removal

Remove **everything** including user content:

```bash
devforgeai uninstall --complete
```

**What happens:**
- Removes: All `.claude/`, `devforgeai/`, `.ai_docs/`
- Removes: All user stories, epics, sprints, ADRs
- Creates backup (can be restored later)
- **Irreversible:** Requires explicit `--complete` flag

---

## Confirmation Prompt

When you run uninstall without `--yes`, you'll see:

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  DEVFORGEAI UNINSTALL CONFIRMATION
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Files that will be removed (45 files, 2.3 MB):
  .claude/skills/devforgeai-ideation/
  .claude/agents/test-automator.md
  .claude/commands/dev.md
  [... 42 more files ...]

Files that will be preserved (131 files):
  devforgeai/specs/Stories/STORY-001.md
  devforgeai/specs/Epics/EPIC-001.md
  [... 129 more files ...]

Backup will be created at:
  ~/devforgeai/backups/uninstall-2025-12-08T10-30-00Z/

This operation will free 2.3 MB of disk space.
This action cannot be undone.

Are you sure? (y/n, default: n):
```

**Default:** No (safe choice)
**Type:** `y` or `yes` to confirm, `n` or `no` to cancel

---

## Backup & Recovery

### Automatic Backup

Before uninstalling, DevForgeAI creates a **complete backup** including:
- All framework files
- All user content
- Installation manifest
- Backup metadata

**Backup location:**
```
~/devforgeai/backups/uninstall-YYYY-MM-DD-HHmmss/
```

**Why backups are important:**
- If uninstall completes successfully but you want to reinstall
- If you need to recover a file after uninstall
- Automatic rollback if uninstall fails

### Skip Backup (Advanced Users Only)

If you're absolutely sure you don't need a backup:

```bash
devforgeai uninstall --skip-backup
```

**⚠️ Warning:** Without a backup, recovery requires manual restoration

---

## Understanding What Gets Removed

### Framework Files (Always Removed in Uninstall)

These are DevForgeAI components that get removed:

```
.claude/
├── skills/                  # AI skills (deleted)
├── agents/                  # Subagents (deleted)
├── commands/                # Slash commands (deleted)
├── memory/                  # Reference docs (deleted)
└── scripts/                 # Setup scripts (deleted)

devforgeai/
├── context/                 # Context files (deleted)
├── protocols/               # Framework patterns (deleted)
├── qa/                      # QA reports (deleted)
└── specs/                   # Specifications (deleted)

CLAUDE.md                     # Framework config (deleted if unmodified)
```

### User Content (Preserved by Default)

These are your files that get preserved:

```
.ai_docs/
├── Epics/                   # Your epics (preserved)
├── Sprints/                 # Your sprints (preserved)
├── Stories/                 # Your stories (preserved)
└── Research/                # Your notes (preserved)

devforgeai/
├── adrs/custom/             # Your architecture decisions (preserved)
└── config/                  # Your configurations (preserved)
```

### Smart Detection

DevForgeAI detects **user-modified files** and preserves them:

**Example:**
```
CLAUDE.md
  - Unmodified (from template) → REMOVED
  - Modified by you → PRESERVED
```

**Why?** If you've customized the framework config, keeping your version is safer.

---

## Special Cases

### Multiple Shell Integrations

If DevForgeAI is installed in multiple shells (bash, zsh, fish):

**Uninstall output:**
```
Removed from shells:
  ✓ bash (~/.bashrc)
  ✓ zsh (~/.zshrc)
  ✓ fish (~/.config/fish/config.fish)
```

### System PATH Installation

If DevForgeAI CLI is installed to system PATH (`/usr/local/bin`):

**Uninstall shows warning:**
```
⚠️ Manual cleanup needed for system PATH

The 'devforgeai' binary was installed to:
  /usr/local/bin/devforgeai

To remove manually, run:
  sudo rm /usr/local/bin/devforgeai
  sudo rm /usr/local/bin/devforgeai-validate
```

### npm Global Installation

If installed via npm globally:

```bash
npm list -g devforgeai  # Check if installed globally
npm uninstall -g devforgeai  # Remove global installation
```

---

## Uninstall Output Summary

After uninstall completes, you'll see:

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  UNINSTALL COMPLETE ✅
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Framework removed:  45 files
User content saved: 131 files
Disk space freed:   2.3 MB
Duration:           2.3 seconds

Backup location:
  ~/devforgeai/backups/uninstall-2025-12-08T10-30-00Z/

Report saved:
  ~/devforgeai/backups/uninstall-2025-12-08T10-30-00Z/uninstall-report.json
```

---

## Troubleshooting

### Uninstall Fails with "Permission Denied"

**Cause:** Some files are read-only
**Solution:**
```bash
# Grant write permission to current user
chmod -R u+w .claude devforgeai

# Then retry uninstall
devforgeai uninstall --yes
```

### Backup Creation Fails

**Cause:** Not enough disk space
**Solution:**
```bash
# Check available space
df -h

# Or skip backup (data loss risk!)
devforgeai uninstall --skip-backup --yes
```

### Cannot Remove Specific File

**Cause:** File is in use or locked
**Solution:**
```bash
# Close all applications using the file
# Then retry uninstall

# Or force removal (if you know what you're doing)
rm -rf .claude/skills/devforgeai-development/
```

### Want to Reinstall After Uninstall

**Steps:**
1. Uninstall creates backup automatically
2. Run: `devforgeai install`
3. All your `.ai_docs/` files will be preserved

---

## Best Practices

**Before Uninstalling:**
1. Run `devforgeai uninstall --dry-run` to preview
2. Backup your project files manually (if critical)
3. Commit your Git repository
4. Close any open editors

**Recommended Sequence:**
```bash
# 1. Preview what will be removed
devforgeai uninstall --dry-run

# 2. Commit current state
git add .
git commit -m "Pre-uninstall backup commit"

# 3. Perform uninstall with confirmation
devforgeai uninstall

# 4. Remove uninstall backup after confirmation (optional)
rm -rf ~/devforgeai/backups/uninstall-*/
```

---

## Advanced: Scripting Uninstall

For automation, combine flags:

```bash
# Complete removal with no prompts (careful!)
devforgeai uninstall --complete --yes --skip-backup

# Preserve mode with backup (safer)
devforgeai uninstall --yes

# Dry-run for testing
devforgeai uninstall --dry-run --complete
```

---

## Getting Help

**More Information:**
- DevForgeAI Docs: https://github.com/anthropics/devforgeai
- Recovery Guide: See `UNINSTALL-RECOVERY-GUIDE.md`
- GitHub Issues: Report problems at https://github.com/anthropics/devforgeai/issues

---

**Version:** 1.0
**Last Updated:** 2025-12-08
