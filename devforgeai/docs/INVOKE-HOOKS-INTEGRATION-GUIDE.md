# Integration Guide: devforgeai invoke-hooks Command

**Version:** 1.0
**Created:** 2025-11-13
**Story:** STORY-022

---

## Overview

The `devforgeai invoke-hooks` command provides centralized hook invocation for DevForgeAI operations. This guide explains how to integrate invoke-hooks into commands, workflows, and custom scripts.

---

## Command Syntax

```bash
devforgeai invoke-hooks --operation <operation-name> [--story <STORY-ID>] [--verbose]
```

**Arguments:**
- `--operation` (required): Operation name (e.g., `dev`, `qa`, `release`, `orchestrate`)
- `--story` (optional): Story ID in format `STORY-NNN`
- `--verbose` (optional): Enable verbose logging

**Exit Codes:**
- `0`: Success (hook invocation completed)
- `1`: Failure (error occurred, check logs)

---

## Integration Pattern

### Pattern 1: Post-Operation Hook (Recommended)

Call invoke-hooks **after** an operation completes successfully:

```bash
#!/bin/bash
# Example: /dev command integration

# 1. Run development workflow
devforgeai-development-logic "$STORY_ID"
DEV_RESULT=$?

if [ $DEV_RESULT -eq 0 ]; then
    echo "Development complete, invoking hooks..."

    # 2. Check if hooks are configured
    devforgeai check-hooks --operation=dev --status=completed

    if [ $? -eq 0 ]; then
        # 3. Invoke hooks (non-blocking)
        devforgeai invoke-hooks --operation=dev --story="$STORY_ID"

        # Note: Hook failures don't affect parent command
        # Exit code from invoke-hooks is logged but not propagated
    fi
fi

exit $DEV_RESULT
```

### Pattern 2: Conditional Hook Invocation

Only invoke hooks when certain conditions are met:

```bash
# Only invoke for specific statuses
if [ "$STORY_STATUS" = "Dev Complete" ]; then
    devforgeai invoke-hooks --operation=dev --story="$STORY_ID"
fi
```

### Pattern 3: Background Invocation

Run invoke-hooks in the background (non-blocking):

```bash
# Invoke in background, don't wait for completion
devforgeai invoke-hooks --operation=dev --story="$STORY_ID" &

# Continue with parent command
echo "Hook invocation started in background"
```

---

## Integration by Command

### /dev Command Integration

**Location:** `.claude/commands/dev.md`

**Integration Point:** After Phase 5 (Git Workflow complete)

**Implementation:**
```bash
# After story status updated to "Dev Complete"
devforgeai check-hooks --operation=dev --status=completed
if [ $? -eq 0 ]; then
    devforgeai invoke-hooks --operation=dev --story="$STORY_ID"
fi
```

### /qa Command Integration

**Location:** `.claude/commands/qa.md`

**Integration Point:** After QA validation complete (pass or fail)

**Implementation:**
```bash
# After QA report generated
STATUS=$(grep "^Status:" devforgeai/qa/reports/$STORY_ID-qa-report.md | awk '{print $2}')

devforgeai check-hooks --operation=qa --status="$STATUS"
if [ $? -eq 0 ]; then
    devforgeai invoke-hooks --operation=qa --story="$STORY_ID"
fi
```

### /release Command Integration

**Location:** `.claude/commands/release.md`

**Integration Point:** After deployment complete (staging or production)

**Implementation:**
```bash
# After release to environment
ENVIRONMENT="$1"  # staging or production

devforgeai check-hooks --operation=release --status=completed
if [ $? -eq 0 ]; then
    devforgeai invoke-hooks --operation=release --story="$STORY_ID" --verbose
fi
```

### /orchestrate Command Integration

**Location:** `.claude/commands/orchestrate.md`

**Integration Point:** After each workflow phase complete

**Implementation:**
```bash
# After dev phase
devforgeai invoke-hooks --operation=dev --story="$STORY_ID"

# After qa phase
devforgeai invoke-hooks --operation=qa --story="$STORY_ID"

# After release phase
devforgeai invoke-hooks --operation=release --story="$STORY_ID"
```

---

## Python API Integration

For Python-based workflows, use the hooks module directly:

```python
from devforgeai_cli.hooks import invoke_hooks

# Simple invocation
success = invoke_hooks('dev', 'STORY-001')

if success:
    print("Hook invocation completed")
else:
    print("Hook invocation failed (check logs)")

# The function returns:
# - True: Hook invocation succeeded
# - False: Hook invocation failed (error logged, graceful degradation)
```

---

## Context Extraction

invoke-hooks automatically extracts operation context including:

- **Operation ID:** Unique identifier (`dev-STORY-001-20251113-143022`)
- **Todos:** TodoWrite data (status, content, completed/pending)
- **Errors:** Error messages and truncated stack traces
- **Timing:** Start time, end time, duration in seconds
- **Status:** Operation status (completed, failed, etc.)

**Context Size:** Capped at 50KB. Large contexts are summarized automatically.

---

## Secret Sanitization

invoke-hooks automatically sanitizes 54+ secret patterns before logging or passing to feedback skill:

**Categories:**
- API Keys (sk-*, AKIA*, etc.)
- Passwords (password:, passwd:, pwd=)
- OAuth Tokens (access_token, refresh_token)
- AWS Credentials (AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY)
- Database URLs (postgresql://, mongodb://)
- GCP Keys (GCP_SERVICE_ACCOUNT_KEY, GOOGLE_CLOUD_API_KEY)
- GitHub Tokens (ghp_*, GITHUB_PAT)
- SSH Keys (-----BEGIN RSA PRIVATE KEY-----)
- JWT Tokens (eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...)
- PII (SSN, credit card numbers)

**Sanitization:** All secrets replaced with `***` before logging or skill invocation.

---

## Timeout Protection

invoke-hooks implements 30-second timeout:

- **Default:** 30 seconds
- **Behavior:** Aborts invocation after timeout, returns exit code 1
- **Logging:** Logs "Feedback hook timeout after 30s"
- **Non-blocking:** Parent command continues regardless

---

## Circular Invocation Protection

invoke-hooks detects and blocks circular invocations:

- **Detection:** `DEVFORGEAI_HOOK_ACTIVE` environment variable
- **Behavior:** Immediate return with exit code 1
- **Logging:** Logs "Circular invocation detected, aborting"
- **Protection:** Prevents infinite feedback loops

---

## Error Handling

invoke-hooks implements graceful degradation:

- **All errors caught:** No exceptions propagated to parent command
- **Logging:** Full error context logged with stack traces
- **Exit codes:** Always returns 0 (success) or 1 (failure)
- **Parent continuation:** Parent command continues regardless of hook result

---

## Configuration

Hook behavior is configured via `devforgeai/config/hooks.yaml`:

```yaml
hooks:
  enabled: true
  operations:
    dev:
      enabled: true
      mode: "interactive"
    qa:
      enabled: true
      mode: "interactive"
    release:
      enabled: true
      mode: "interactive"
```

Check configuration with:
```bash
devforgeai check-hooks --operation=dev --status=completed
```

---

## Testing Integration

### Unit Tests

Test hook invocation in isolation:
```python
from unittest.mock import patch
from devforgeai_cli.hooks import invoke_hooks

@patch('devforgeai_cli.hooks.HookInvocationService.invoke_feedback_skill')
def test_invoke_hooks(mock_skill):
    mock_skill.return_value = True

    result = invoke_hooks('dev', 'STORY-001')

    assert result == True
    mock_skill.assert_called_once()
```

### Integration Tests

Test end-to-end invocation:
```bash
# Run manual test suite
bash .claude/scripts/devforgeai_cli/tests/manual_test_invoke_hooks.sh
```

---

## Troubleshooting

See: [Troubleshooting Guide](devforgeai/docs/INVOKE-HOOKS-TROUBLESHOOTING.md)

---

## References

- **Story:** STORY-022 (invoke-hooks implementation)
- **CLI Module:** `.claude/scripts/devforgeai_cli/hooks.py`
- **Tests:** `.claude/scripts/devforgeai_cli/tests/test_invoke_hooks.py`
- **Configuration:** `devforgeai/config/hooks.yaml`

---

**This guide satisfies STORY-022 DoD requirement: "Integration guide updated (how commands call invoke-hooks)"**
