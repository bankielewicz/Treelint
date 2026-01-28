# Configuration Schema Reference

**STORY-009:** Skip Pattern Tracking - Feedback Preferences Configuration Schema

## Overview

This document defines the complete YAML schema for `feedback-preferences.yaml`, the configuration file that stores skip counter state, disabled feedback preferences, and audit trails for the skip pattern tracking system.

## File Location

```
devforgeai/config/feedback-preferences.yaml
```

**File Permissions:** Mode 600 (user read/write only, no group/other access)

## Complete Schema

### Version and Metadata

```yaml
---
version: "1.0"
created_at: "2025-11-07T10:30:00Z"
last_updated: "2025-11-09T14:45:32Z"
---
```

**Fields:**
- `version` (string): Schema version. Current version: "1.0"
  - Required for future schema migrations
  - Must match code version to prevent incompatibility
  - Format: Semantic versioning (MAJOR.MINOR)

- `created_at` (ISO 8601 timestamp): When config file was first created
  - Example: "2025-11-07T10:30:00Z"
  - Used for audit trail and age tracking
  - Never updated after creation

- `last_updated` (ISO 8601 timestamp): When config was last modified
  - Example: "2025-11-09T14:45:32Z"
  - Updated whenever any skip counter, disabled flag, or disable reason changes
  - Helps detect stale configurations

---

### Skip Counters Section

```yaml
skip_counters:
  skill_invocation: 0
  subagent_invocation: 0
  command_execution: 0
  context_loading: 0
```

**Purpose:** Track number of consecutive skips per operation type

**Fields:**
- `skill_invocation` (integer, 0-100): Skips when skill is invoked
- `subagent_invocation` (integer, 0-100): Skips when subagent is invoked
- `command_execution` (integer, 0-100): Skips during command execution feedback
- `context_loading` (integer, 0-100): Skips during context loading feedback

**Constraints:**
- Type: Integer (non-negative)
- Range: 0-100 (prevents overflow)
- Default: 0 (new operation types start at 0)
- Update frequency: Incremented on each skip (immediately written to disk)

**Validation Rules:**
1. Value must be an integer (no strings like "1")
2. Value must be >= 0 (no negative counts)
3. Value must be <= 100 (prevents overflow)
4. Each operation type is tracked independently (disabling one doesn't affect others)

---

### Disabled Feedback Section

```yaml
disabled_feedback:
  skill_invocation: false
  subagent_invocation: false
  command_execution: false
  context_loading: false
```

**Purpose:** Track which feedback types are disabled by the user

**Fields:**
- `skill_invocation` (boolean): Whether skill invocation feedback is disabled
- `subagent_invocation` (boolean): Whether subagent invocation feedback is disabled
- `command_execution` (boolean): Whether command execution feedback is disabled
- `context_loading` (boolean): Whether context loading feedback is disabled

**Constraints:**
- Type: Boolean (true/false only, YAML native booleans)
- Default: false (feedback enabled by default)
- Persistence: Value persists across sessions until user changes it

**Enforcement:**
- When `disabled_feedback[operation_type] = true`:
  - NO feedback prompts shown for that operation type
  - Skip counter still increments (for tracking purposes)
  - Pattern detection doesn't trigger (disabled types are not prompted)

- When `disabled_feedback[operation_type] = false`:
  - Normal feedback behavior
  - Pattern detection triggers at 3+ consecutive skips
  - AskUserQuestion appears with disable/keep/ask-later options

**Validation Rules:**
1. Value must be boolean (true/false)
2. Case-sensitive: `true` not `True` (YAML spec)
3. Consistency check: Must have matching entry in disable_reasons section

---

### Disable Reasons Section

```yaml
disable_reasons:
  skill_invocation: null
  subagent_invocation: "User disabled after 3+ consecutive skips on 2025-11-09T14:45:32Z"
  command_execution: null
  context_loading: null
```

**Purpose:** Document WHY each feedback type was disabled (audit trail)

**Fields:**
- `skill_invocation` (string or null): Reason skill_invocation feedback is disabled
- `subagent_invocation` (string or null): Reason subagent_invocation feedback is disabled
- `command_execution` (string or null): Reason command_execution feedback is disabled
- `context_loading` (string or null): Reason context_loading feedback is disabled

**Field Values:**
- `null` (no reason, feedback is enabled)
- String with reason and timestamp:
  - Format: "User action [reason] on [ISO 8601 timestamp]"
  - Example: "User disabled after 3+ consecutive skips on 2025-11-09T14:45:32Z"
  - Example: "User manually disabled feedback on 2025-11-09T15:00:00Z"
  - Example: "Auto-disabled due to 10+ token waste on 2025-11-09T14:50:00Z"

**Constraints:**
- Type: String or null
- Max length: 200 characters
- Format: Reason + timestamp (for audit trail)
- Updated when: disabled_feedback flag is changed

**Validation Rules:**
1. If `disabled_feedback[type] = true`, `disable_reasons[type]` must NOT be null
2. If `disabled_feedback[type] = false`, `disable_reasons[type]` must be null
3. String must not exceed 200 characters
4. Timestamp must be ISO 8601 format

---

## Complete Example

```yaml
---
version: "1.0"
created_at: "2025-11-07T10:30:00Z"
last_updated: "2025-11-09T14:45:32Z"
---

# Skip counters per operation type
skip_counters:
  skill_invocation: 0
  subagent_invocation: 2
  command_execution: 0
  context_loading: 3

# Disabled feedback types
disabled_feedback:
  skill_invocation: false
  subagent_invocation: false
  command_execution: false
  context_loading: true

# Disable reasons (audit trail)
disable_reasons:
  skill_invocation: null
  subagent_invocation: null
  command_execution: null
  context_loading: "User disabled after 3+ consecutive skips on 2025-11-09T14:45:32Z"
```

**Interpretation:**
- Skill invocation: Enabled, 0 skips
- Subagent invocation: Enabled, 2 skips (below 3+ threshold)
- Command execution: Enabled, 0 skips
- Context loading: DISABLED, 3 skips (disabled after pattern detected)

---

## Schema Validation Rules

### Required Fields (Must Always Exist)

```
version
created_at
last_updated
skip_counters.*
disabled_feedback.*
disable_reasons.*
```

All 4 operation types must have entries in all 3 sections.

### Immutable Fields (Never Change)

```
version (no downgrades)
created_at (timestamp is fixed)
```

### Mutable Fields (Can Change)

```
last_updated (changes on every modification)
skip_counters (incrementing on skips)
disabled_feedback (changes on enable/disable)
disable_reasons (changes with disabled_feedback)
```

---

## File Format Details

### YAML Syntax

- **Frontmatter markers:** Three dashes `---` at start and after metadata
- **Indentation:** 2 spaces (not tabs)
- **Booleans:** `true`/`false` (lowercase, YAML native)
- **Null values:** `null` (YAML native null)
- **Timestamps:** ISO 8601 UTC (e.g., "2025-11-09T14:45:32Z")

### Example YAML Parsing

```python
import yaml

# Load config
with open('devforgeai/config/feedback-preferences.yaml', 'r') as f:
    config = yaml.safe_load(f)

# Access fields
version = config['version']
skip_count = config['skip_counters']['skill_invocation']
is_disabled = config['disabled_feedback']['subagent_invocation']
reason = config['disable_reasons']['command_execution']
```

---

## Data Types and Constraints Summary

| Field | Type | Range/Values | Default | Immutable |
|-------|------|--------------|---------|-----------|
| `version` | string | "1.0" | N/A | Yes |
| `created_at` | ISO 8601 | Any valid timestamp | Current UTC | Yes |
| `last_updated` | ISO 8601 | Any valid timestamp | Current UTC | No |
| `skip_counters[type]` | integer | 0-100 | 0 | No |
| `disabled_feedback[type]` | boolean | true/false | false | No |
| `disable_reasons[type]` | string/null | <200 chars | null | No |

---

## File Creation

### Automatic Creation (First Skip)

When a user skips feedback for the first time, the system automatically creates `feedback-preferences.yaml` with:

```yaml
---
version: "1.0"
created_at: "2025-11-09T14:45:00Z"
last_updated: "2025-11-09T14:45:00Z"
---

skip_counters:
  skill_invocation: 1
  subagent_invocation: 0
  command_execution: 0
  context_loading: 0

disabled_feedback:
  skill_invocation: false
  subagent_invocation: false
  command_execution: false
  context_loading: false

disable_reasons:
  skill_invocation: null
  subagent_invocation: null
  command_execution: null
  context_loading: null
```

**File Permissions:** Automatically set to mode 600 (user read/write only)

### Corruption Recovery

If the config file is corrupted (invalid YAML):
1. System logs error
2. Creates backup: `devforgeai/config/feedback-preferences-{timestamp}.yaml.backup`
3. Creates fresh config (same as above)
4. User notified (non-blocking notification)

---

## Backward Compatibility

### Schema Version "1.0"

Current production version. Supports:
- 4 operation types (skill_invocation, subagent_invocation, command_execution, context_loading)
- Skip counter tracking per type
- Disabled feedback flags per type
- Disable reasons per type

### Future Versions

If new operation types are added:
1. Schema version increments (e.g., "1.1")
2. New operation type added to all 3 sections
3. Existing configs migrated automatically
4. Backward compatibility maintained (old configs still work)

---

## Related Documentation

- **User Guide:** `user-guide-feedback-preferences.md`
- **Developer Guide:** `developer-guide-operation-types.md`
- **Skip Event Schema:** `skip-event-schema.md`
- **Token Waste Formula:** `token-waste-formula.md`
