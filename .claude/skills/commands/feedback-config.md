---
description: View and edit feedback system configuration
argument-hint: [view|edit|reset] [field] [value]
model: opus
allowed-tools: Skill
---

# /feedback-config - Feedback Configuration Management

View, edit, and reset feedback system configuration settings.

---

## Quick Reference

```bash
# View current configuration
/feedback-config view

# Edit retention period
/feedback-config edit retention_days 30

# Enable auto-trigger
/feedback-config edit auto_trigger_enabled True

# Change export format
/feedback-config edit export_format csv

# Reset to defaults
/feedback-config reset

# No parameter
/feedback-config
 - Use AskUserQuestion tool to ask questions, if no parameter is passed.
```

---

## Command Workflow

### Subcommands

**view** - Display current configuration
**edit** - Modify a configuration field
**reset** - Restore default configuration

---

### View Subcommand

**Syntax:** `/feedback-config view`

**Displays:**
- retention_days: Days to keep feedback (1-3650)
- auto_trigger_enabled: Automatic feedback triggering (True/False)
- export_format: Default export format (json/csv/markdown)
- include_metadata: Include metadata in exports (True/False)
- search_enabled: Enable search functionality (True/False)

**Response (JSON):**
```json
{
  "status": "success",
  "config": {
    "retention_days": 90,
    "auto_trigger_enabled": true,
    "export_format": "json",
    "include_metadata": true,
    "search_enabled": true
  },
  "message": "Current feedback configuration loaded"
}
```

---

### Edit Subcommand

**Syntax:** `/feedback-config edit [field] [value]`

**Valid Fields:**
- `retention_days` - Integer (1-3650)
- `auto_trigger_enabled` - Boolean (True/False)
- `export_format` - Enum (json, csv, markdown)
- `include_metadata` - Boolean (True/False)
- `search_enabled` - Boolean (True/False)

**Validation Rules:**

1. **retention_days**
   - Type: Integer
   - Range: 1 to 3650 (1 day to 10 years)
   - Rejects: Negative numbers, zero, >3650

2. **Booleans** (auto_trigger_enabled, include_metadata, search_enabled)
   - Type: Boolean (exact match)
   - Valid: "True" or "False" (case-sensitive)
   - Rejects: "true", "false", "1", "0", "yes", "no"

3. **export_format**
   - Type: Enum
   - Valid: "json", "csv", "markdown"
   - Rejects: Any other value (e.g., "xml", "txt")

**Response (Success):**
```json
{
  "status": "success",
  "field": "retention_days",
  "value": 30,
  "message": "Configuration updated: retention_days = 30"
}
```

**Response (Error):**
```json
{
  "status": "error",
  "message": "retention_days must be between 1 and 3650 (received: -5)",
  "suggested_action": "Check value constraints and retry"
}
```

---

### Reset Subcommand

**Syntax:** `/feedback-config reset`

**Action:** Restore all fields to default values

**Default Configuration:**
```yaml
retention_days: 90
auto_trigger_enabled: true
export_format: json
include_metadata: true
search_enabled: true
```

**Response:**
```json
{
  "status": "success",
  "config": {
    "retention_days": 90,
    "auto_trigger_enabled": true,
    "export_format": "json",
    "include_metadata": true,
    "search_enabled": true
  },
  "message": "Configuration reset to defaults"
}
```

---

## Configuration File

**Location:** `devforgeai/feedback/config.yaml`

**Format:** YAML

**Example:**
```yaml
retention_days: 90
auto_trigger_enabled: true
export_format: json
include_metadata: true
search_enabled: true
```

**Persistence:** Changes saved immediately on `edit` command

---

## Error Handling

### Invalid Field Name
```
Error: Invalid field name: max_entries
Valid fields: retention_days, auto_trigger_enabled, export_format,
              include_metadata, search_enabled
Action: Use one of the valid field names
```

### Invalid Value Type
```
Error: retention_days must be between 1 and 3650 (received: abc)
Action: Provide integer value within valid range
```

### Invalid Boolean Value
```
Error: auto_trigger_enabled must be exactly 'True' or 'False' (received: true)
Action: Use capital 'True' or 'False'
```

### Invalid Enum Value
```
Error: export_format must be 'json', 'csv', or 'markdown' (received: xml)
Action: Use one of the supported formats
```

### Config File Corrupted
```
Error: Configuration file corrupted: invalid YAML syntax
Action: Run '/feedback-config reset' to restore defaults
```

---

## Examples

### Example 1: View Current Settings
```bash
/feedback-config view
```

**Output:**
```
Current Configuration:
  retention_days: 90
  auto_trigger_enabled: true
  export_format: json
  include_metadata: true
  search_enabled: true
```

### Example 2: Change Retention Period
```bash
/feedback-config edit retention_days 30
```

**Output:**
```
✓ Updated retention_days = 30
```

### Example 3: Disable Auto-Trigger
```bash
/feedback-config edit auto_trigger_enabled False
```

**Output:**
```
✓ Updated auto_trigger_enabled = False
```

### Example 4: Change Export Format
```bash
/feedback-config edit export_format csv
```

**Output:**
```
✓ Updated export_format = csv
```

### Example 5: Reset to Defaults
```bash
/feedback-config reset
```

**Output:**
```
✓ Configuration reset to defaults
  retention_days: 90
  auto_trigger_enabled: true
  export_format: json
  include_metadata: true
  search_enabled: true
```

---

## Troubleshooting

### Issue: "Invalid field name"

**Symptoms:** Error when editing configuration field

**Cause:** Field name not in whitelist (security protection)

**Resolution:** Use exact field names:
- retention_days (not retention-days, retentionDays)
- auto_trigger_enabled (not auto_trigger)
- export_format (not format, export-format)
- include_metadata (not metadata)
- search_enabled (not search)

### Issue: "Must be exactly 'True' or 'False'"

**Symptoms:** Boolean fields reject lowercase values

**Cause:** Strict boolean validation (prevents ambiguity)

**Resolution:**
```bash
# ❌ Wrong
/feedback-config edit auto_trigger_enabled true   # lowercase rejected
/feedback-config edit auto_trigger_enabled 1      # number rejected
/feedback-config edit auto_trigger_enabled yes    # string rejected

# ✅ Correct
/feedback-config edit auto_trigger_enabled True   # exact capital
/feedback-config edit auto_trigger_enabled False  # exact capital
```

### Issue: "Configuration file corrupted"

**Symptoms:** View command returns error about corrupted file

**Cause:** Invalid YAML syntax in config.yaml

**Resolution:**
```bash
# Option 1: Reset to defaults
/feedback-config reset

# Option 2: Manually fix YAML file
# Edit devforgeai/feedback/config.yaml
# Ensure valid YAML syntax (no tabs, proper indentation)
```

### Issue: Config changes not persisting

**Symptoms:** Edit command succeeds but view shows old values

**Cause:** Permission issue or file lock

**Resolution:**
```bash
# Check file permissions
ls -la devforgeai/feedback/config.yaml

# Fix permissions if needed
chmod 644 devforgeai/feedback/config.yaml

# Verify changes
/feedback-config view
```

---

## Integration

**File Updated:** `devforgeai/feedback/config.yaml`

**Used By:**
- `/feedback` - Respects auto_trigger_enabled
- `/feedback-search` - Respects search_enabled
- `/export-feedback` - Uses export_format and include_metadata defaults

**Configuration Scope:** Global (affects all feedback operations)

---

## Advanced Usage

### Batch Configuration Updates

```bash
# Update multiple fields sequentially
/feedback-config edit retention_days 30
/feedback-config edit export_format csv
/feedback-config edit include_metadata False
```

### Configuration Workflow

```bash
# 1. View current settings
/feedback-config view

# 2. Make changes
/feedback-config edit retention_days 60

# 3. Verify changes
/feedback-config view

# 4. If mistake, reset
/feedback-config reset
```

### Configuration Backup

```bash
# Manual backup before changes
cp devforgeai/feedback/config.yaml devforgeai/feedback/config.yaml.backup

# Restore if needed
cp devforgeai/feedback/config.yaml.backup devforgeai/feedback/config.yaml
```

---

## Security

**Field Name Whitelist:** Only 5 predefined fields can be edited (prevents arbitrary config modification)

**Value Validation:** All values validated against type and range constraints

**No Path Traversal:** Config file path is hardcoded (cannot write to arbitrary locations)

**No Injection:** No shell commands executed with user input

---

## Related Commands

- `/feedback` - Manual feedback trigger
- `/feedback-search` - Search feedback
- `/export-feedback` - Export feedback
- `/orchestrate` - Full lifecycle (auto-configures feedback)

---

## See Also

- devforgeai-feedback skill (SKILL.md)
- STORY-011: Configuration Management
- devforgeai/feedback/config.yaml (configuration file)
- .claude/scripts/devforgeai_cli/feedback/config_manager.py (implementation)
