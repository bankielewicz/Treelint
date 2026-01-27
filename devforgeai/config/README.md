# DevForgeAI Feedback Configuration

Configuration guide for the DevForgeAI feedback system.

---

## Overview

The feedback system uses a YAML-based configuration file to control when and how user feedback is collected. The configuration supports master enable/disable, trigger modes, question limits, skip tracking, and template preferences.

**Configuration File:** `devforgeai/config/feedback.yaml`

**JSON Schema:** `devforgeai/config/feedback.schema.json` (for IDE autocomplete)

---

## Quick Start

### Default Configuration (No File Required)

The feedback system works out-of-the-box with sensible defaults:

```yaml
enabled: true
trigger_mode: failures-only
operations: null
conversation_settings:
  max_questions: 5
  allow_skip: true
skip_tracking:
  enabled: true
  max_consecutive_skips: 3
  reset_on_positive: true
templates:
  format: structured
  tone: brief
```

**Default Behavior:**
- Feedback enabled
- Collected only on failures (not successes)
- Max 5 questions per conversation
- Users can skip feedback
- Skip counter resets after 3 consecutive skips
- Structured format with brief tone

---

## Configuration Examples

### Example 1: Disable All Feedback Collection

```yaml
# devforgeai/config/feedback.yaml
enabled: false
```

**Result:** No feedback collected, all other settings ignored.

**Use Case:** Temporary disable during development or CI/CD runs.

---

### Example 2: Always Collect Feedback

```yaml
# devforgeai/config/feedback.yaml
enabled: true
trigger_mode: always
conversation_settings:
  max_questions: 10
  allow_skip: true
skip_tracking:
  enabled: true
  max_consecutive_skips: 5
  reset_on_positive: true
templates:
  format: structured
  tone: detailed
```

**Result:** Feedback collected after every operation (success or failure).

**Use Case:** Active development, gathering comprehensive user feedback.

---

### Example 3: Collect Feedback Only for Specific Operations

```yaml
# devforgeai/config/feedback.yaml
enabled: true
trigger_mode: specific-operations
operations:
  - qa
  - deployment
  - release
conversation_settings:
  max_questions: 3
  allow_skip: false
skip_tracking:
  enabled: false
templates:
  format: free-text
  tone: detailed
```

**Result:** Feedback collected only for QA, deployment, and release operations.

**Use Case:** Focus feedback on critical operations, require responses (no skip).

---

### Example 4: Minimal Configuration (Failures Only, Brief)

```yaml
# devforgeai/config/feedback.yaml
enabled: true
trigger_mode: failures-only
conversation_settings:
  max_questions: 3
  allow_skip: true
skip_tracking:
  enabled: true
  max_consecutive_skips: 2
templates:
  format: structured
  tone: brief
```

**Result:** Collect feedback only on failures, keep questions brief, limit to 3 per session.

**Use Case:** Production environment, minimal user interruption.

---

## Configuration Schema

### Top-Level Settings

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | boolean | `true` | Master enable/disable switch for all feedback collection |
| `trigger_mode` | string | `failures-only` | When to collect feedback: `always`, `failures-only`, `specific-operations`, `never` |
| `operations` | array | `null` | List of operations to collect feedback for (only used if `trigger_mode: specific-operations`) |
| `conversation_settings` | object | See below | Question limits and skip permissions |
| `skip_tracking` | object | See below | Skip counter management |
| `templates` | object | See below | Feedback format and tone preferences |

### Conversation Settings

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_questions` | integer | `5` | Maximum questions per conversation (0 = unlimited) |
| `allow_skip` | boolean | `true` | Whether users can skip feedback questions |

### Skip Tracking Settings

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | boolean | `true` | Whether skip tracking is active |
| `max_consecutive_skips` | integer | `3` | Max consecutive skips before pausing feedback (0 = no limit) |
| `reset_on_positive` | boolean | `true` | Reset skip counter when positive feedback received (rating ≥ 4) |

### Template Settings

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `format` | string | `structured` | Feedback format: `structured` (predefined options) or `free-text` (open input) |
| `tone` | string | `brief` | Question tone: `brief` (≤50 chars, no context) or `detailed` (includes context and outcome) |

---

## Trigger Modes

### `always`
Collect feedback unconditionally after every operation.

**Example:**
```yaml
trigger_mode: always
```

**When to use:** Active development, comprehensive feedback gathering.

---

### `failures-only` (Default)
Collect feedback only when operations fail (test failures, validation errors).

**Example:**
```yaml
trigger_mode: failures-only
```

**When to use:** Production, minimize user interruption, focus on issues.

---

### `specific-operations`
Collect feedback only for specific operations (QA, deployment, etc.).

**Example:**
```yaml
trigger_mode: specific-operations
operations:
  - qa
  - development
  - release
```

**When to use:** Target critical operations, selective feedback.

---

### `never`
Never collect feedback (fastest path, no processing).

**Example:**
```yaml
trigger_mode: never
```

**When to use:** CI/CD environments, automated testing, no user present.

---

## Hot-Reload

Configuration changes are detected and applied **within 5 seconds** without restarting.

**How it works:**
1. Edit `devforgeai/config/feedback.yaml`
2. Save the file
3. System detects change within 5 seconds
4. New configuration loaded and applied
5. Feedback collection immediately reflects new settings

**Example:**
```bash
# Disable feedback temporarily
echo "enabled: false" > devforgeai/config/feedback.yaml

# Wait ~2 seconds
sleep 2

# Feedback collection now stopped (no restart needed)
```

**Error Handling:** If invalid configuration detected during reload, previous valid configuration is retained and error logged to `devforgeai/logs/config-errors.log`.

---

## Logging

Feedback system logs to 4 files in `devforgeai/logs/`:

| Log File | Purpose | Example Entry |
|----------|---------|---------------|
| `config-errors.log` | Configuration load errors and validation failures | `[2025-11-10 10:30:15] ERROR: Invalid trigger_mode: 'invalid'. Must be one of: always, failures-only, specific-operations, never` |
| `feedback-skips.log` | Skip tracking (increments, resets, limit reached) | `[2025-11-10 10:35:20] INFO: Skip counter incremented to 2/3` |
| `debug.log` | Debug-level logging (enable with `DEBUG_FEEDBACK_CONFIG=1`) | `[2025-11-10 10:40:25] DEBUG: Configuration loaded in 18ms` |
| `audit.log` | Configuration change audit trail | `[2025-11-10 10:45:30] AUDIT: Configuration updated by user, enabled: true -> false` |

**Enable debug mode:**
```bash
export DEBUG_FEEDBACK_CONFIG=1
```

---

## Validation

Configuration is validated against JSON Schema on load. Invalid configurations are rejected with clear error messages.

**Common validation errors:**

### Invalid trigger_mode
```
Error: Invalid trigger_mode value: 'invalid-mode'. Must be one of: always, failures-only, specific-operations, never
Fix: Set trigger_mode to a valid value
```

### Missing operations array
```
Error: trigger_mode is 'specific-operations' but operations array is missing
Fix: Add operations: [qa, development, ...] to configuration
```

### Invalid format
```
Error: Invalid templates.format value: 'unknown'. Must be one of: structured, free-text
Fix: Set templates.format to 'structured' or 'free-text'
```

---

## IDE Support

The JSON Schema file (`devforgeai/config/feedback.schema.json`) enables autocomplete and validation in modern editors:

**VS Code:**
1. Install YAML extension
2. Schema is auto-detected from `$schema` reference in YAML file

**IntelliJ/PyCharm:**
1. Settings → Languages & Frameworks → Schemas and DTDs → JSON Schema Mappings
2. Add mapping: `feedback.schema.json` → `feedback.yaml`

**Vim/Neovim:**
1. Install `coc-yaml` plugin
2. Schema auto-detected from file pattern

---

## Migration from Previous Versions

### From v0.x to v1.0

**Changes:**
- New configuration file location: `devforgeai/config/feedback.yaml` (previously: `devforgeai/feedback-config.yaml`)
- New schema with hot-reload support
- Skip tracking now optional (previously always enabled)

**Migration steps:**
1. Copy old config to new location:
   ```bash
   cp devforgeai/feedback-config.yaml devforgeai/config/feedback.yaml
   ```

2. Update schema references (if using IDE autocomplete)

3. Test configuration:
   ```bash
   # Validate configuration loads without errors
   tail -f devforgeai/logs/config-errors.log
   ```

---

## Troubleshooting

See `TROUBLESHOOTING.md` for common issues and solutions.

---

## Advanced Topics

### Thread Safety

The configuration system is fully thread-safe:
- Skip counter uses atomic operations with locks
- Hot-reload updates are atomic (all-or-nothing)
- Concurrent operations properly synchronized

### Performance

Configuration system is optimized for minimal overhead:
- Load time: <100ms (typical: <20ms)
- Hot-reload detection: ≤5 seconds (typical: <2 seconds)
- Skip counter lookup: <10ms (typical: <1ms)
- Per-feedback overhead: <50ms (typical: <10ms)

### Custom Templates

See `.claude/skills/devforgeai-feedback/templates/` for template customization options.

---

## References

- **JSON Schema:** `devforgeai/config/feedback.schema.json`
- **Implementation:** `.claude/scripts/devforgeai_cli/feedback/config_manager.py`
- **Story:** `devforgeai/specs/Stories/STORY-011-configuration-management.story.md`
- **Tests:** `.claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py`

---

**Last Updated:** 2025-11-10
**Version:** 1.0
**Story:** STORY-011
