# DevForgeAI Uninstall Recovery Guide

## Overview

If you need to recover from an uninstall operation, DevForgeAI provides automatic backups and recovery procedures. This guide covers scenarios where you may need to restore files after uninstallation.

---

## Recovery Scenarios

### Scenario 1: Reinstall DevForgeAI (Recommended)

**Situation:** You uninstalled DevForgeAI and want to reinstall it while keeping your work

**Steps:**

1. **Verify backup exists:**
   ```bash
   ls ~/devforgeai/backups/
   # Output: uninstall-2025-12-08T10-30-00Z/
   ```

2. **Reinstall framework:**
   ```bash
   devforgeai install
   # This will reinstall all framework files
   ```

3. **Your data is safe:**
   ```bash
   ls .ai_docs/        # All your stories, epics, sprints are still here
   ls devforgeai/adrs/custom/  # Your custom ADRs are preserved
   ```

**Result:** Framework reinstalled, all user content preserved

---

### Scenario 2: Restore Specific File from Backup

**Situation:** You accidentally deleted a file and want to recover it

**Steps:**

1. **Find the backup directory:**
   ```bash
   ls -la ~/devforgeai/backups/
   cd ~/devforgeai/backups/uninstall-2025-12-08T10-30-00Z/
   ```

2. **Browse backup contents:**
   ```bash
   ls -la
   # Shows all files that were in the installation at uninstall time
   ```

3. **Restore specific file:**
   ```bash
   # Copy from backup to current location
   cp ~/devforgeai/backups/uninstall-2025-12-08T10-30-00Z/.claude/skills/devforgeai-ideation/SKILL.md .claude/skills/devforgeai-ideation/SKILL.md
   ```

4. **Verify restoration:**
   ```bash
   ls -la .claude/skills/devforgeai-ideation/SKILL.md
   # File should now exist
   ```

**Result:** Specific file recovered from backup

---

### Scenario 3: Restore Entire Framework

**Situation:** Uninstall went wrong and you want to restore the complete framework

**Steps:**

1. **Locate backup:**
   ```bash
   BACKUP_DIR=~/devforgeai/backups/uninstall-2025-12-08T10-30-00Z
   ls $BACKUP_DIR
   ```

2. **Check what's in backup:**
   ```bash
   ls -la $BACKUP_DIR/.claude/
   ls -la $BACKUP_DIR/devforgeai/
   # Verify all framework files are there
   ```

3. **Restore framework directories:**
   ```bash
   # Restore .claude/ directory
   cp -r $BACKUP_DIR/.claude/ .

   # Restore devforgeai/ framework files (be careful not to overwrite user content)
   cp -r $BACKUP_DIR/devforgeai/context/ devforgeai/
   cp -r $BACKUP_DIR/devforgeai/protocols/ devforgeai/
   cp -r $BACKUP_DIR/devforgeai/qa/ devforgeai/

   # Restore CLAUDE.md
   cp $BACKUP_DIR/CLAUDE.md .
   ```

4. **Verify restoration:**
   ```bash
   ls -la .claude/skills/
   ls -la devforgeai/context/
   # Framework files should be restored
   ```

**Result:** Entire framework restored from backup

---

### Scenario 4: Restore User Content After Complete Uninstall

**Situation:** You ran `--complete` and removed all files including `.ai_docs/`

**Steps:**

1. **Locate backup:**
   ```bash
   BACKUP_DIR=~/devforgeai/backups/uninstall-2025-12-08T10-30-00Z
   ```

2. **Restore .ai_docs/:**
   ```bash
   # Check if backup contains your stories
   ls $BACKUP_DIR/.ai_docs/

   # Restore everything
   cp -r $BACKUP_DIR/.ai_docs/ .
   ```

3. **Verify restoration:**
   ```bash
   ls devforgeai/specs/Stories/
   # All your stories should be back
   ```

4. **Reinstall framework (if needed):**
   ```bash
   devforgeai install
   ```

**Result:** User content and framework both restored

---

## Backup Location & Structure

### Where Backups Are Stored

```
~/devforgeai/backups/
└── uninstall-2025-12-08T10-30-00Z/
    ├── .claude/                 # Framework files
    ├── devforgeai/             # Framework config
    ├── .ai_docs/                # User stories, epics, sprints
    ├── CLAUDE.md                # Framework config
    ├── uninstall-report.json    # Detailed report
    └── uninstall-report.txt     # Human-readable summary
```

### How to Find Newest Backup

```bash
# Show all uninstall backups (newest first)
ls -lt ~/devforgeai/backups/uninstall-* | head -5

# Get the newest backup directory
LATEST_BACKUP=$(ls -td ~/devforgeai/backups/uninstall-*/ | head -1)
echo $LATEST_BACKUP
```

### Check Backup Integrity

```bash
# Verify backup contains expected files
cd ~/devforgeai/backups/uninstall-2025-12-08T10-30-00Z/

# Should show framework files
test -d .claude/skills && echo "✓ Framework skills found"
test -d devforgeai/context && echo "✓ Context files found"

# Should show user content
test -d .ai_docs && echo "✓ User content found"
```

---

## Backup Metadata

The backup includes a detailed report:

### uninstall-report.json

Machine-readable report:

```json
{
  "status": "SUCCESS",
  "timestamp": "2025-12-08T10:30:00Z",
  "mode": "PRESERVE_USER_CONTENT",
  "dry_run": false,
  "files_removed": 45,
  "files_preserved": 131,
  "space_freed_mb": 2.3,
  "duration_seconds": 2.3,
  "backup_location": "/home/user/devforgeai/backups/uninstall-2025-12-08T10-30-00Z",
  "errors": [],
  "warnings": []
}
```

### uninstall-report.txt

Human-readable summary:

```
UNINSTALL REPORT
================
Date: 2025-12-08 10:30:00
Mode: PRESERVE_USER_CONTENT

Removed: 45 files (2.3 MB)
Preserved: 131 files
Errors: 0
Warnings: 0
Duration: 2.3 seconds

Backup: ~/devforgeai/backups/uninstall-2025-12-08T10-30-00Z/
```

---

## Cleanup: Deleting Old Backups

### View All Backups

```bash
du -sh ~/devforgeai/backups/*/
# Shows size of each backup
```

### Delete Specific Backup

```bash
rm -rf ~/devforgeai/backups/uninstall-2025-12-08T10-30-00Z/
```

### Delete Old Backups (Keep Last 3)

```bash
# List backups (oldest first)
ls -t ~/devforgeai/backups/uninstall-*/ | tail -n +4

# Delete all but last 3
ls -td ~/devforgeai/backups/uninstall-*/ | tail -n +4 | xargs rm -rf
```

### Free Up Disk Space

```bash
# Show backup directory size
du -sh ~/devforgeai/backups/

# Delete all backups (after confirming you don't need them)
rm -rf ~/devforgeai/backups/*
```

---

## Troubleshooting Recovery

### Issue: Backup Directory Not Found

**Cause:** Uninstall was done with `--skip-backup` flag

**Solution:**
```bash
# Check if backup directory exists
ls ~/devforgeai/backups/

# If empty or missing:
# 1. Check your Git history
git log --oneline -- .claude/ | head -5
git show COMMIT:.claude/skills/... > recovered-skill.md

# 2. Use Git to restore
git checkout HEAD~1 -- .claude/

# 3. If using version control, restore from last commit
git restore .claude/ devforgeai/ CLAUDE.md
```

### Issue: Backup Is Corrupted

**Cause:** Disk error or incomplete backup

**Solution:**
```bash
# Verify backup integrity
tar -tzf ~/devforgeai/backups/uninstall-2025-12-08T10-30-00Z.tar.gz > /dev/null

# If error: Try using Git instead
git log --all -- .claude/ | head -1
git show COMMIT_HASH:.claude/ > restore.tar
```

### Issue: User Content Was Lost

**Cause:** Used `--complete` mode or backup doesn't contain files

**Solution:**
```bash
# Check if files are in devforgeai/qa/reports/ (might have copies)
ls devforgeai/qa/reports/

# Check your Git history
git log --oneline -- .ai_docs/ | head -5

# Restore from Git
git checkout HEAD~1 -- .ai_docs/

# Check any remote repositories
git log --all --grep="story" | head -5
```

---

## Prevention: Best Practices

### Before Uninstalling

1. **Always run dry-run first:**
   ```bash
   devforgeai uninstall --dry-run
   ```

2. **Commit your work:**
   ```bash
   git add .
   git commit -m "Pre-uninstall checkpoint"
   ```

3. **External backup (for critical projects):**
   ```bash
   cp -r .ai_docs/ ~/backup-stories-2025-12-08/
   ```

### Keep Backups Around

```bash
# Don't delete uninstall backup immediately
# Keep for at least 1-2 weeks after uninstall
ls -lh ~/devforgeai/backups/

# Before deleting, verify you don't need it
tar -tzf ~/devforgeai/backups/uninstall-*/uninstall-backup.tar | grep -c ".ai_docs"
# If count > 0, backup is valuable
```

### Version Control Everything

```bash
# Make sure your .ai_docs/ is in Git
git add .ai_docs/
git commit -m "Save all stories, epics, sprints"

# In case you need to recover later
git log --oneline -- .ai_docs/ | head -10
```

---

## Getting Help

**If recovery doesn't work:**

1. **Check GitHub Issues:** https://github.com/anthropics/devforgeai/issues
2. **Check recovery documentation:** Read above scenarios
3. **Use Git for restoration:** Final fallback is Git history
4. **Manual restoration:** Copy files from backup directory manually

---

**Version:** 1.0
**Last Updated:** 2025-12-08
**Scope:** DevForgeAI Framework Uninstall Recovery
