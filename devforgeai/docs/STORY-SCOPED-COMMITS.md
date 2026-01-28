# Story-Scoped Pre-Commit Validation

**Version:** 1.0
**Created:** 2025-12-22
**Story:** STORY-121

---

## Overview

DevForgeAI's pre-commit hook validates all staged story files before each commit. When working on multiple stories simultaneously, validation errors in unrelated stories can block your commits.

**Story-scoped validation** allows you to scope pre-commit validation to a specific story, bypassing validation of other staged stories.

**Use Cases:**
- Working on STORY-120 while STORY-115 has known validation errors
- Committing fixes to a single story in a batch of staged files
- Parallel story development where stories have different completion states

---

## Basic Usage

### Syntax

```bash
DEVFORGEAI_STORY=STORY-NNN git commit -m "message"
```

### Example

```bash
# Validate only STORY-120, ignore validation errors in other staged stories
DEVFORGEAI_STORY=STORY-120 git commit -m "feat(STORY-120): implement user login"
```

### Console Output (Scoped)

```
🔍 DevForgeAI Validators Running...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Scoped to: STORY-120
  📋 Validating: devforgeai/specs/Stories/STORY-120-user-login.story.md
     ✅ Passed

✅ All validators passed - commit allowed
```

---

## Examples

### Scenario 1: Single Story Development

You're actively working on STORY-120 and have STORY-115 staged (incomplete):

```bash
# Stage your changes
git add devforgeai/specs/Stories/STORY-120-user-login.story.md

# Commit with scoping (STORY-115 errors ignored)
DEVFORGEAI_STORY=STORY-120 git commit -m "feat(STORY-120): add acceptance criteria"
```

### Scenario 2: Multiple Stories, One Ready

You have three stories staged: STORY-120 (ready), STORY-121 (WIP), STORY-122 (WIP):

```bash
# Only validate STORY-120
DEVFORGEAI_STORY=STORY-120 git commit -m "feat(STORY-120): complete implementation"

# Later, validate STORY-121
DEVFORGEAI_STORY=STORY-121 git commit -m "feat(STORY-121): add tests"
```

### Scenario 3: Shell Alias for Convenience

Add to your `.bashrc` or `.zshrc`:

```bash
alias gcs='DEVFORGEAI_STORY=$1 git commit'

# Usage
gcs STORY-120 -m "feat: implement login"
```

Or with a function:

```bash
gcs() {
    DEVFORGEAI_STORY=$1 git commit "${@:2}"
}

# Usage
gcs STORY-120 -m "feat: implement login"
```

---

## Troubleshooting

### Invalid Format Warning

**Symptom:**
```
WARNING: Invalid DEVFORGEAI_STORY format: story-120
Expected: STORY-NNN (e.g., STORY-120)
Falling back to unscoped validation...
```

**Cause:** Story ID must be uppercase `STORY-NNN` format.

**Fix:** Use uppercase: `DEVFORGEAI_STORY=STORY-120`

### Scoped Story Not Found

**Symptom:** "No story files to validate" when scoped.

**Cause:** The specified story file is not in the staging area.

**Fix:** Verify with `git diff --cached --name-only | grep STORY-120`

### Validation Still Blocks

**Symptom:** Commit blocked even with scoping.

**Cause:** The scoped story itself has validation errors.

**Fix:** Scoping only filters WHICH stories are validated, not WHETHER they pass.

---

## Best Practices

1. **Use scoping sparingly** - It's for unblocking, not bypassing validation
2. **Fix other stories soon** - Don't let validation errors accumulate
3. **Audit regularly** - Run `/audit-deferrals` to track deferred items
4. **Document why** - If scoping around a known issue, document it in Implementation Notes

---

## Technical Details

### Environment Variable

| Property | Value |
|----------|-------|
| Name | `DEVFORGEAI_STORY` |
| Format | `STORY-NNN` (3+ digits, uppercase) |
| Scope | Single command only (not persistent) |
| Default | Unset (validates all stories) |

### Validation Rules

- Format must match `^STORY-[0-9]{3,}$`
- Lowercase or mixed case is rejected
- Invalid format falls back to unscoped validation
- Empty string (`DEVFORGEAI_STORY=""`) treated as unset

### How Scoping Works

```bash
if [ -n "$DEVFORGEAI_STORY" ]; then
    # Scoped mode: grep for the specific story ID
    STORY_FILES=$(git diff --cached --name-only | grep "${DEVFORGEAI_STORY}" | grep -v '^tests/')
else
    # Unscoped mode: validate all .story.md files
    STORY_FILES=$(git diff --cached --name-only | grep '\.story\.md$' | grep -v '^tests/')
fi
```

**Key Points:**
- Both modes exclude test fixtures in `tests/` directory
- Scoped mode uses substring match on story ID (allows partial paths)
- Unscoped mode validates ALL story files (original behavior)
- Invalid format shows warning and falls back to unscoped

---

## Related Documentation

- [Pre-Commit Hook Installation](./../INSTALL-HOOKS.md)
- [DoD Validator](../../.claude/scripts/devforgeai_cli/README.md)
- [Quality Gates](../../.claude/rules/core/quality-gates.md)
- [Story Lifecycle](./story-lifecycle.md)
