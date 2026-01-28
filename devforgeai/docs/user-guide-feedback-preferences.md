# User Guide: Managing Feedback Preferences

**STORY-009:** Skip Pattern Tracking - User Guide

## Overview

This guide explains how to manage your feedback preferences in DevForgeAI. When you skip feedback prompts repeatedly, the system detects a pattern and offers to disable feedback for that operation type, helping you focus on your work while reclaiming wasted tokens.

## Quick Start

### What Happens When You Skip Feedback

Every time you skip a feedback prompt (by selecting "Skip" or pressing the skip button), the system counts the consecutive skips for that operation type.

**Skip Counter Behavior:**
- **Skip 1:** Skip counter increments to 1 (no action)
- **Skip 2:** Skip counter increments to 2 (no action)
- **Skip 3:** Skip counter reaches 3 → **Pattern detected!** AskUserQuestion appears

**When Pattern is Detected (3+ Consecutive Skips):**

A dialog appears:
```
You've skipped feedback for [operation_type] 3 consecutive times.
Feedback prompts for [operation_type] have wasted ~4,500 tokens so far.

Would you like to disable feedback for this operation type?
A) Yes, disable feedback
B) Keep feedback enabled
C) Ask me later
```

### Option A: Yes, Disable Feedback

**What happens:**
- Feedback for this operation type is **permanently disabled**
- No more prompts for this operation type
- Reclaim ~1,500 tokens per skipped prompt going forward
- Skip counter resets to 0

**Example:**
If you disabled `skill_invocation` feedback after 3 skips:
- You saved 4,500 tokens already (3 × 1,500)
- Future sessions: 0 more prompts for skill invocation
- Total savings: 4,500+ tokens

**To re-enable later:** See "Re-enabling Feedback" section below

### Option B: Keep Feedback Enabled

**What happens:**
- Feedback continues showing
- Skip counter keeps incrementing
- You can skip more and see the suggestion again in a future session
- Useful if you want to reconsider later

**When to choose this:**
- Feedback is occasionally valuable
- You want feedback for some situations but not all
- You're not ready to disable yet

### Option C: Ask Me Later

**What happens:**
- Feedback continues showing
- Skip counter **resets to 0** (fresh start)
- If you skip 3 more times, the dialog will appear again
- Useful if you want to test disabling later

**When to choose this:**
- You're on a deadline and need to decide later
- You want to gather more data first
- You're not sure about permanently disabling

---

## Operation Types

The system tracks feedback for 4 operation types:

### 1. skill_invocation

**Triggered when:** You invoke a DevForgeAI skill (e.g., `Skill(command="devforgeai-development")`)

**Example feedback:** "Running development skill for STORY-042..."

**Common reason to disable:** You invoke skills frequently and don't need reminders

---

### 2. subagent_invocation

**Triggered when:** A subagent is invoked to specialize a task (e.g., test-automator generates tests)

**Example feedback:** "Running test-automator subagent..."

**Common reason to disable:** Subagents are normal workflow, don't need constant notifications

---

### 3. command_execution

**Triggered when:** You run a DevForgeAI command (e.g., `/dev STORY-001`)

**Example feedback:** "Executing /dev workflow..."

**Common reason to disable:** Commands run frequently, you know what you're doing

---

### 4. context_loading

**Triggered when:** The system loads context files or story documents

**Example feedback:** "Loading story context for STORY-042..."

**Common reason to disable:** Context loading happens automatically, doesn't need approval

---

## Viewing Current Preferences

### Check Your Config File

Your feedback preferences are stored in a YAML file:

```
Location: devforgeai/config/feedback-preferences.yaml
```

### View with Cat

```bash
cat devforgeai/config/feedback-preferences.yaml
```

### Example Output

```yaml
---
version: "1.0"
created_at: "2025-11-07T10:30:00Z"
last_updated: "2025-11-09T14:45:32Z"
---

skip_counters:
  skill_invocation: 0
  subagent_invocation: 2
  command_execution: 0
  context_loading: 3

disabled_feedback:
  skill_invocation: false
  subagent_invocation: false
  command_execution: false
  context_loading: true

disable_reasons:
  skill_invocation: null
  subagent_invocation: null
  command_execution: null
  context_loading: "User disabled after 3+ consecutive skips on 2025-11-09T14:45:32Z"
```

### Understanding Your Config

**skip_counters:**
- Shows how many consecutive times you've skipped each operation type
- `skill_invocation: 0` = You haven't skipped (or reset after disabling)
- `subagent_invocation: 2` = You've skipped 2 consecutive times (below 3+ threshold)

**disabled_feedback:**
- Shows which operation types have feedback disabled
- `false` = Feedback is ENABLED (will show prompts)
- `true` = Feedback is DISABLED (no prompts shown)

**disable_reasons:**
- Why each operation type was disabled (audit trail)
- `null` = Not disabled (or enabled by default)
- String value = Timestamp when user disabled it

---

## Disabling Feedback

### Method 1: Via AskUserQuestion (Recommended)

When the pattern detection dialog appears after 3 consecutive skips:

```
Would you like to disable feedback for [operation_type]?
A) Yes, disable feedback        ← Select this
B) Keep feedback enabled
C) Ask me later
```

**Result:**
- Feedback automatically disabled
- Config file updated
- Counter resets to 0

---

### Method 2: Manual Edit (Advanced)

You can directly edit the config file to disable feedback:

**Step 1: Open the config file**
```bash
nano devforgeai/config/feedback-preferences.yaml
```

**Step 2: Find the operation type to disable**
```yaml
disabled_feedback:
  skill_invocation: false   ← Change this to true
  subagent_invocation: false
  command_execution: false
  context_loading: true
```

**Step 3: Change false → true**
```yaml
disabled_feedback:
  skill_invocation: true    ← Now disabled
  subagent_invocation: false
  command_execution: false
  context_loading: true
```

**Step 4: Add disable reason (optional, but recommended)**
```yaml
disable_reasons:
  skill_invocation: "User manually disabled feedback on 2025-11-09T15:00:00Z"
  ...
```

**Step 5: Update last_updated timestamp**
```yaml
last_updated: "2025-11-09T15:00:00Z"  ← Update to current time
```

**Step 6: Save file**
```
Press Ctrl+X, then Y, then Enter (in nano)
```

**Result:**
- Feedback disabled for that operation type
- Changes take effect immediately
- Counter resets to 0 on next skip

---

## Re-enabling Feedback

If you later decide you want feedback again:

### Method 1: Via AskUserQuestion

The system will prompt you again after 3 more consecutive skips (if you selected "Ask me later" before).

---

### Method 2: Manual Re-enable (Recommended)

**Step 1: Open the config file**
```bash
nano devforgeai/config/feedback-preferences.yaml
```

**Step 2: Change disabled status to false**
```yaml
disabled_feedback:
  skill_invocation: true    ← Change this to false to re-enable
  subagent_invocation: false
  command_execution: false
  context_loading: false
```

**Step 3: Clear the disable reason**
```yaml
disable_reasons:
  skill_invocation: null    ← Set to null (was: "User manually disabled...")
  ...
```

**Step 4: Reset the skip counter**
```yaml
skip_counters:
  skill_invocation: 0       ← Reset to 0 (fresh start)
  ...
```

**Step 5: Update last_updated timestamp**
```yaml
last_updated: "2025-11-09T15:15:00Z"  ← Current time
```

**Step 6: Save file**
```
Press Ctrl+X, then Y, then Enter (in nano)
```

**Result:**
- Feedback re-enabled for that operation type
- Counter resets to 0
- Pattern detection starts fresh (requires 3 new skips to trigger)

---

## Resetting Skip Counters

### When to Reset

You might want to reset counters if:
- You accidentally skipped multiple times
- You want a fresh start
- Feedback circumstances have changed

### How to Reset

**Step 1: Open config file**
```bash
nano devforgeai/config/feedback-preferences.yaml
```

**Step 2: Reset the counter**
```yaml
skip_counters:
  skill_invocation: 0       ← Reset to 0
  subagent_invocation: 0    ← Reset to 0
  command_execution: 0      ← Reset to 0
  context_loading: 0        ← Reset to 0
```

**Step 3: Update timestamp**
```yaml
last_updated: "2025-11-09T15:20:00Z"  ← Current time
```

**Step 4: Save file**

**Result:**
- All skip counters reset to 0
- Pattern detection will require 3 skips again
- Your decision history preserved (disable_reasons unchanged)

---

## Understanding Token Waste

### What is Token Waste?

When you skip a feedback prompt, the system showed you something you didn't want to see. That feedback interaction consumed tokens that could have been used for model work.

**Example:**
- Feedback prompt shown: ~1,500 tokens consumed
- You skip it: Those 1,500 tokens "wasted" (not useful to you)
- Wasted 3 times: 4,500 tokens total

### Token Waste Calculation

```
Token Waste = 1,500 tokens per prompt × Number of skips

Skip 1:  1,500 × 1 = 1,500 tokens
Skip 2:  1,500 × 2 = 3,000 tokens
Skip 3:  1,500 × 3 = 4,500 tokens (pattern detected!)
```

### How This Helps

By disabling feedback you skip repeatedly, you:
1. **Save tokens:** No more useless prompts
2. **Improve performance:** Faster workflow without interruptions
3. **Keep tokens available:** Reclaim 1,500+ tokens per skipped prompt

---

## FAQ

### Q: If I disable feedback, can I re-enable it?

**A:** Yes, absolutely. You can re-enable feedback anytime by editing your config file (see "Re-enabling Feedback" section) or by waiting for the next pattern detection dialog.

---

### Q: Will disabling feedback cause problems?

**A:** No. Feedback prompts are optional recommendations. Disabling them just removes the interruption. The system continues working normally.

**Example:** If you disable skill_invocation feedback:
- Skills still run normally
- You just don't see reminder prompts
- You can still manually check skill progress

---

### Q: What if I accidentally disable important feedback?

**A:** You can re-enable it (see "Re-enabling Feedback" section). The system is designed for reversibility.

**Recommendation:** Before permanently disabling a frequently-used operation type, consider:
1. First choose "Ask me later" (resets counter)
2. See if behavior changes
3. If you still skip, then disable

---

### Q: Can I delete my config file?

**A:** You can, but not recommended. Deleting it:
- Loses your disable preferences
- Loses your skip history
- System creates a fresh one on next skip

**Better approach:** Reset counters manually (see "Resetting Skip Counters") if you want a fresh start.

---

### Q: What happens if I manually set skip_counter to a high number?

**A:** The skip counter is used for pattern detection. Setting it to 3+ will:
- Trigger pattern detection next time you skip (or on next startup)
- System will think you've skipped that many times
- AskUserQuestion will appear with the actual counter value

**Not recommended** - just use the normal skip mechanism or "Ask me later" option.

---

### Q: How often are patterns detected?

**A:** Pattern detection occurs once per operation type per session:
- When you reach 3 consecutive skips
- Dialog appears automatically
- You make a choice (disable/keep/ask-later)
- Pattern considered "handled" for that session

If you skip again in a future session and disabled feedback, no dialog appears (feedback is disabled). If you didn't disable it, a new dialog will appear on next 3+ skips.

---

### Q: Why is it 3 skips and not 2 or 5?

**A:** The threshold of 3 is chosen because:
- **Too low (1-2):** False positives, disrupts workflow
- **Just right (3):** High confidence you're intentionally skipping, before too much waste
- **Too high (5+):** Already wasted 7,500+ tokens before asking

Research shows 3 is the sweet spot for user experience and token efficiency.

---

## Troubleshooting

### Problem: Config File Missing

**Symptom:** You don't see `devforgeai/config/feedback-preferences.yaml`

**Cause:** File is created on first skip

**Solution:** Skip a feedback prompt. The system will create the file automatically.

---

### Problem: Config File Corrupted

**Symptom:** YAML parsing error when you try to open config file

**Cause:** Accidental edits or file corruption

**Solution:**
1. System automatically creates `.yaml.backup` file
2. Delete the corrupted config file
3. Skip feedback - system creates fresh config
4. Your previous preferences are lost (start fresh)

---

### Problem: "Feedback still showing despite disabled=true"

**Symptom:** You set `disabled_feedback: skill_invocation: true` but feedback still appears

**Cause:** File permissions or file not saved properly

**Solution:**
1. Open config file: `nano devforgeai/config/feedback-preferences.yaml`
2. Verify `disabled_feedback: skill_invocation: true` is set
3. Verify file is saved (Ctrl+X, Y, Enter)
4. Check file permissions: `ls -l devforgeai/config/feedback-preferences.yaml`
5. Should show: `-rw-------` (only user can read/write)
6. If permissions wrong, run: `chmod 600 devforgeai/config/feedback-preferences.yaml`

---

## Best Practices

1. **Review before disabling:** If you're not sure, choose "Ask me later" first
2. **Keep audit trail:** Don't delete `disable_reasons` entries (helps you remember why)
3. **Regular review:** Every quarter, check if disabled feedback is still needed
4. **Safe edits:** Always back up config file before manual edits
   ```bash
   cp devforgeai/config/feedback-preferences.yaml \
      devforgeai/config/feedback-preferences.yaml.my_backup
   ```

---

## Related Documentation

- **Config Schema Reference:** `config-schema-reference.md` (detailed field definitions)
- **Token Waste Formula:** `token-waste-formula.md` (how waste is calculated)
- **Skip Event Schema:** `skip-event-schema.md` (technical event structure)
- **Developer Guide:** `developer-guide-operation-types.md` (for admins/developers)

---

## Support

For issues or questions:
1. Check the FAQ section above
2. Review config file manually (see "Viewing Current Preferences")
3. Check logs: `devforgeai/logs/skip-pattern-detection.log`
4. Reset to defaults: Delete config file and re-create (via skip)

