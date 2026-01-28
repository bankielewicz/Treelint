# Hook Failures Troubleshooting Guide

**Diagnose and fix feedback hook issues**

---

## Overview

Feedback hooks are designed to fail gracefully. If hooks encounter problems, your command still completes successfully. This guide helps diagnose what happened and fix hook-related issues.

**Quick Principle:**
- ✅ Hook failures are never fatal
- ✅ Your primary operation always completes
- ✅ Hook issues are logged but not alarming
- ✅ You can continue working normally

---

## Symptom Index

Click the symptom closest to what you're experiencing:

### Hooks Never Appear

[→ Section: "Hooks Not Triggering"](#hooks-not-triggering)

### Hooks Keep Asking Me

[→ Section: "Hooks Too Frequent"](#hooks-too-frequent)

### Feedback Not Saving

[→ Section: "Feedback Not Saved"](#feedback-not-saved)

### Error Message After Command

[→ Section: "Error Messages"](#error-messages)

### CLI Not Found

[→ Section: "DevForgeAI CLI Missing"](#devforgeai-cli-missing)

---

## Detailed Troubleshooting

### Hooks Not Triggering

**Symptom:** You complete a command with `/dev` or `/create-context`, but no feedback prompt appears.

#### Step 1: Check Configuration

```bash
# View current hook configuration
devforgeai feedback-config view
```

**Expected output:**
```
enabled: true
trigger_on: all
skip_patterns: []
rate_limit:
  max_per_day: 3
  cooldown_minutes: 60
```

#### Step 2: Diagnose the Issue

| Config State | Likely Cause | Fix |
|---|---|---|
| `enabled: false` | Hooks globally disabled | Set `enabled: true` |
| Your operation in `skip_patterns` | Operation is skipped | Remove from skip_patterns |
| `trigger_on: failures-only` + success | Only fails trigger | Change to `trigger_on: all` |
| `trigger_on: success-only` + failure | Only success triggers | Change to `trigger_on: all` |
| `cooldown_minutes` hasn't elapsed | Rate limiting | Wait or reduce cooldown |
| `max_per_day` reached | Daily limit hit | Wait until next day or increase limit |

#### Step 3: Fix Configuration

**Example: Re-enable hooks for /dev**

```bash
# Reset configuration to defaults
devforgeai feedback-config reset

# Or specifically enable /dev
devforgeai feedback-config edit trigger_on all
devforgeai feedback-config edit skip_patterns ""
```

#### Step 4: Test

Try the command again:
```bash
/dev STORY-001
# At the end, feedback prompt should appear
```

---

### Hooks Too Frequent

**Symptom:** Feedback prompts appear too often, interrupting your workflow.

#### Step 1: Current Rate Limit

```bash
devforgeai feedback-config view rate_limit
```

**Output:**
```
rate_limit:
  max_per_day: 3
  cooldown_minutes: 60
```

#### Step 2: Reduce Frequency

**Option A: Increase cooldown between prompts**
```bash
# Wait 2 hours between feedback prompts
devforgeai feedback-config edit rate_limit.cooldown_minutes 120
```

**Option B: Reduce max per day**
```bash
# Maximum 1 feedback prompt per day
devforgeai feedback-config edit rate_limit.max_per_day 1
```

**Option C: Skip feedback for specific operations**
```bash
# Don't ask about /dev, but do ask about /create-context
devforgeai feedback-config edit skip_patterns "dev"
```

**Option D: Disable entirely**
```bash
# No feedback prompts at all
devforgeai feedback-config edit enabled false
```

#### Step 3: Verify

```bash
devforgeai feedback-config view
# Should show your new limits
```

---

### Feedback Not Saved

**Symptom:** You provided feedback, but later you don't see your responses saved.

#### Step 1: Check Directory Exists

```bash
ls -la devforgeai/feedback/sessions/
```

**Expected:** Directory should exist and be readable/writable.

**If missing:**
```bash
mkdir -p devforgeai/feedback/sessions/
chmod 755 devforgeai/feedback/sessions/
```

#### Step 2: Check Disk Space

```bash
df -h devforgeai/
```

**If disk full:** Delete old feedback or make space.

#### Step 3: Check File Permissions

```bash
ls -la devforgeai/feedback/sessions/*.json
# Should show -rw- permissions (writable by user)
```

**If not writable:**
```bash
chmod 644 devforgeai/feedback/sessions/*.json
```

#### Step 4: Check Recent Saves

```bash
# List all saved feedback sessions
ls -lt devforgeai/feedback/sessions/ | head

# View most recent feedback
cat devforgeai/feedback/sessions/$(ls -t devforgeai/feedback/sessions/ | head -1)
```

#### Step 5: Retry

Run a command again and provide feedback:
```bash
/dev STORY-001
# Provide feedback when prompted
```

Check if saved:
```bash
ls -lt devforgeai/feedback/sessions/ | head -1
```

---

### Error Messages

**Symptom:** You see an error message like "Optional feedback system unavailable, continuing..."

#### What This Means

The feedback system encountered an error, but your primary operation **succeeded anyway**. This is working as designed.

**Common causes:**
- devforgeai CLI not installed
- hooks.yaml corrupted/invalid
- Permission denied on feedback directory
- Disk full
- User cancelled feedback (Ctrl+C)

#### Step 1: Check devforgeai CLI

```bash
which devforgeai
# Should show path like: /usr/local/bin/devforgeai

# Or check version
devforgeai --version
# Should show: devforgeai version X.X.X
```

**If CLI not found:** See section "DevForgeAI CLI Missing" below.

#### Step 2: Check hooks.yaml

```bash
# View configuration
cat devforgeai/hooks/hooks.yaml

# Try to validate it
devforgeai feedback-config view
# Should display without errors
```

**If invalid YAML:**
```bash
# Reset to defaults
devforgeai feedback-config reset
```

#### Step 3: Check Permissions

```bash
# Verify directory is writable
test -w devforgeai/feedback/sessions/ && echo "OK" || echo "NOT WRITABLE"

# Fix if needed
chmod 755 devforgeai/feedback/sessions/
```

#### Step 4: Check Disk Space

```bash
# Show available space
df -h /
# Should have >100MB available
```

**If full:** Clean up old files or archive feedback.

---

### DevForgeAI CLI Missing

**Symptom:** Error like "devforgeai: command not found" or "check-hooks: command not found"

#### Step 1: Verify Installation

```bash
which devforgeai
# If nothing prints, CLI is not in PATH
```

#### Step 2: Check Installation Status

```bash
# Try to install
pip install devforgeai
# or
pip3 install devforgeai
```

#### Step 3: Verify After Install

```bash
devforgeai --version
# Should print version number
```

**If still not found:**

```bash
# Show Python path
python3 -m site --user-base
# Should contain devforgeai binary

# Try absolute path
~/.local/bin/devforgeai --version
```

#### Step 4: Add to PATH (if needed)

If devforgeai works but `which devforgeai` fails:

```bash
# Add to .bashrc or .zshrc
export PATH="$PATH:~/.local/bin"

# Then source it
source ~/.bashrc  # or ~/.zshrc
```

#### Step 5: Verify in New Terminal

```bash
devforgeai --version
# Should work now
```

---

## Common Scenarios

### Scenario 1: Using DevForgeAI Offline

**Problem:** No internet, hooks trying to connect?

**Answer:** Hooks don't need internet. Feedback is saved locally to `devforgeai/feedback/sessions/`. You can share feedback later when online.

**Solution:** If hooks are blocking:
```bash
# Disable hooks temporarily
devforgeai feedback-config edit enabled false

# Work offline normally
/dev STORY-001

# Re-enable when online
devforgeai feedback-config edit enabled true
```

### Scenario 2: Multiple Projects

**Problem:** Different feedback settings needed for different projects?

**Answer:** Each project has its own `devforgeai/hooks/hooks.yaml`. Create project-specific configurations:

```bash
# Project A - enable all feedback
cd /path/to/project-a
devforgeai feedback-config edit trigger_on all

# Project B - disable feedback
cd /path/to/project-b
devforgeai feedback-config edit enabled false
```

### Scenario 3: Sharing Feedback

**Problem:** Want to share your feedback with the DevForgeAI team?

**Answer:** Export and share:

```bash
# Export feedback
devforgeai export-feedback --output-dir=/tmp/feedback-export

# Review exported files (human-readable JSON)
cat /tmp/feedback-export/feedback-sessions.json

# Share via GitHub, email, etc.
```

### Scenario 4: Clearing Feedback History

**Problem:** Want to delete all saved feedback?

**Answer:**
```bash
# View what's saved
ls devforgeai/feedback/sessions/

# Delete specific session
rm devforgeai/feedback/sessions/2025-11-17-dev.json

# Delete all sessions
rm devforgeai/feedback/sessions/*

# Verify cleared
ls devforgeai/feedback/sessions/
# Should be empty
```

---

## Advanced Diagnosis

### Check Hook Logs

```bash
# View detailed hook execution log
cat devforgeai/feedback/.hook-execution.log
```

**Look for:**
- Exit codes from check-hooks and invoke-hooks
- Timestamps of hook calls
- Error messages from CLI

### Test Hook Manually

```bash
# Test hook check manually
devforgeai check-hooks --operation=dev --status=completed

# Check exit code
echo $?
# 0 = eligible, 1 = not eligible, other = error
```

### Debug Configuration

```bash
# View all current settings
devforgeai feedback-config view --verbose

# Show full hooks.yaml file
cat devforgeai/hooks/hooks.yaml

# Test YAML validity
devforgeai feedback-config validate
```

---

## Getting Help

If troubleshooting doesn't resolve your issue:

1. **Check the User Guide:** `devforgeai/guides/FEEDBACK_HOOKS_USER_GUIDE.md`
2. **Review Hook Pattern:** `devforgeai/protocols/hook-integration-pattern.md`
3. **Search Issues:** GitHub issues with "feedback hooks"
4. **Create New Issue:** Include:
   - Command you ran (`/dev`, `/create-context`)
   - Output of `devforgeai feedback-config view`
   - Error message (if any)
   - Output of `devforgeai --version`

---

## Checklists

### Quick Fix Checklist

- [ ] Run `devforgeai feedback-config view`
- [ ] Check if `enabled: true`
- [ ] Check if your operation isn't in `skip_patterns`
- [ ] Check if `cooldown_minutes` hasn't elapsed
- [ ] Run `devforgeai feedback-config reset` to restore defaults
- [ ] Try the command again

### Diagnosis Checklist

- [ ] Confirm hooks.yaml exists at `devforgeai/hooks/hooks.yaml`
- [ ] Verify devforgeai CLI installed: `which devforgeai`
- [ ] Check feedback directory: `ls devforgeai/feedback/sessions/`
- [ ] Verify disk space: `df -h /`
- [ ] Check permissions: `test -w devforgeai/feedback/sessions/`
- [ ] Test hook manually: `devforgeai check-hooks --operation=dev --status=completed`

### Clean Slate

If all else fails, reset to defaults:

```bash
# Reset all hook configuration
devforgeai feedback-config reset

# Delete problematic feedback files
rm -rf devforgeai/feedback/sessions/*

# Verify reset
devforgeai feedback-config view
# Should show defaults
```

Then try running a command again:
```bash
/dev STORY-001
```

---

## Glossary

| Term | Definition |
|------|-----------|
| **Hook** | Feedback system trigger that asks for your input after a command completes |
| **Eligible** | User is allowed to see feedback based on configuration and rate limits |
| **Trigger** | Event that causes hooks to check eligibility (command completion) |
| **Skip Pattern** | Operation name added to configuration to prevent hooks from triggering |
| **Rate Limit** | Configuration that limits how often feedback prompts appear |
| **Cooldown** | Minimum time between feedback prompts |
| **Session** | One feedback response saved to `devforgeai/feedback/sessions/` |

---

**Last Updated:** 2025-11-17
**Framework Version:** 3.0 (Phase 3 Complete)
**Related:** STORY-030 Wire hooks into /create-context command

