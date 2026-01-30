# Feedback Configuration Migration Guide

Guide for migrating between versions of the DevForgeAI feedback configuration system.

---

## Version History

| Version | Release Date | Key Changes |
|---------|--------------|-------------|
| **1.0** | 2025-11-10 | Initial release with YAML configuration, hot-reload, skip tracking |
| **0.x** | (Pre-release) | No formal configuration file |

---

## Migration: Pre-1.0 → v1.0

### Overview

Version 1.0 introduces a formal YAML-based configuration system replacing hardcoded defaults.

### Breaking Changes

**None.** Version 1.0 is backward compatible - if no configuration file exists, the system uses defaults matching pre-1.0 behavior.

### New Features in v1.0

1. **YAML Configuration File** - `devforgeai/config/feedback.yaml`
2. **Hot-Reload** - Configuration changes applied within 5 seconds
3. **Skip Tracking** - Track consecutive skips with configurable limits
4. **Trigger Modes** - Control when feedback is collected (always, failures-only, specific-operations, never)
5. **Template Preferences** - Choose format (structured/free-text) and tone (brief/detailed)
6. **JSON Schema** - IDE autocomplete support

### Migration Steps

#### Option 1: Use Defaults (No Action Required)

If you're satisfied with default behavior:

**No migration needed.** System uses defaults automatically:
```yaml
enabled: true
trigger_mode: failures-only
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

#### Option 2: Create Custom Configuration

To customize feedback behavior:

1. **Create configuration file:**
   ```bash
   mkdir -p devforgeai/config
   ```

2. **Create `devforgeai/config/feedback.yaml`:**
   ```yaml
   # Example custom configuration
   enabled: true
   trigger_mode: failures-only
   conversation_settings:
     max_questions: 3
     allow_skip: true
   skip_tracking:
     enabled: true
     max_consecutive_skips: 2
     reset_on_positive: true
   templates:
     format: structured
     tone: brief
   ```

3. **Test configuration:**
   ```bash
   # Verify no errors
   tail -f devforgeai/logs/config-errors.log
   ```

4. **(Optional) Set up IDE autocomplete:**
   - JSON Schema available at `devforgeai/config/feedback.schema.json`
   - See README.md for IDE setup instructions

### Configuration Mapping

**Pre-1.0 Hardcoded Behavior → v1.0 Configuration:**

| Pre-1.0 Behavior | v1.0 Configuration |
|------------------|-------------------|
| Always feedback enabled | `enabled: true` (default) |
| Feedback on failures only | `trigger_mode: failures-only` (default) |
| No question limit | `conversation_settings.max_questions: 0` (0 = unlimited) |
| Skip allowed | `conversation_settings.allow_skip: true` (default) |
| No skip tracking | `skip_tracking.enabled: false` |
| Structured format | `templates.format: structured` (default) |

---

## Migration: v1.0 → v1.1 (Future)

### Planned Changes (Not Yet Released)

Version 1.1 (planned release: 2025-Q2) will introduce:

1. **User-specific configurations** - Per-user override files
2. **Environment-based configs** - Development, staging, production profiles
3. **Advanced template customization** - Custom template fields
4. **Aggregation preferences** - Control how feedback is aggregated
5. **Export/import** - Configuration backup and restore

### Backward Compatibility

**v1.1 will be fully backward compatible with v1.0.** No migration required.

**Deprecated (but supported):**
- None planned

**Removed:**
- None planned

---

## Migration: v1.1 → v2.0 (Future)

### Planned Changes (Not Yet Released)

Version 2.0 (tentative release: 2025-Q4) may introduce:

1. **Breaking change:** Configuration file format upgrade
2. **Database-backed configuration** (optional, for multi-user environments)
3. **REST API** for configuration management
4. **Role-based permissions** for configuration access

### Migration Strategy (Draft)

**Automatic migration tool** will be provided:

```bash
# Planned migration command (not yet available)
devforgeai-cli upgrade-config --from=1.x --to=2.0
```

**Manual migration** (if needed):
1. Export v1.x configuration
2. Run conversion script
3. Import v2.0 configuration
4. Validate

**Backward compatibility:**
- v2.0 will read v1.x configuration files during transition period
- Deprecation warnings for 6 months before breaking changes

---

## Version Compatibility Matrix

| Feature | v0.x | v1.0 | v1.1 (Planned) | v2.0 (Planned) |
|---------|------|------|----------------|----------------|
| **YAML Configuration** | ❌ | ✅ | ✅ | ✅ |
| **Hot-Reload** | ❌ | ✅ | ✅ | ✅ |
| **Skip Tracking** | ❌ | ✅ | ✅ | ✅ |
| **JSON Schema** | ❌ | ✅ | ✅ | ✅ |
| **Per-user Configs** | ❌ | ❌ | ✅ (Planned) | ✅ |
| **Environment Profiles** | ❌ | ❌ | ✅ (Planned) | ✅ |
| **Database Backend** | ❌ | ❌ | ❌ | ✅ (Planned) |
| **REST API** | ❌ | ❌ | ❌ | ✅ (Planned) |

---

## Common Migration Scenarios

### Scenario 1: Disable Feedback Temporarily

**Pre-1.0:**
```python
# Had to modify source code or environment variable
FEEDBACK_ENABLED = False
```

**v1.0:**
```yaml
# devforgeai/config/feedback.yaml
enabled: false
```

**Change detected within 5 seconds, no restart needed.**

---

### Scenario 2: Limit Feedback Questions

**Pre-1.0:**
```python
# Hardcoded in source, required code change
MAX_QUESTIONS = 5
```

**v1.0:**
```yaml
# devforgeai/config/feedback.yaml
conversation_settings:
  max_questions: 3
```

**Dynamic configuration, no code changes.**

---

### Scenario 3: Collect Feedback Only for QA Operations

**Pre-1.0:**
```python
# Required conditional logic in source code
if operation_type == "qa":
    collect_feedback()
```

**v1.0:**
```yaml
# devforgeai/config/feedback.yaml
trigger_mode: specific-operations
operations:
  - qa
```

**Declarative configuration, no code changes.**

---

## Rollback Procedures

### Rollback: v1.0 → Pre-1.0

**Not recommended,** but if needed:

1. **Remove configuration file:**
   ```bash
   rm devforgeai/config/feedback.yaml
   ```

2. **System reverts to defaults** (same as pre-1.0 behavior)

3. **No data loss** - Skip tracking logs preserved

---

### Rollback: v1.1 → v1.0 (Future)

**Planned rollback procedure:**

1. **Downgrade package:**
   ```bash
   pip install devforgeai-cli==1.0.0
   ```

2. **Configuration auto-downgraded** (backward compatible)

3. **New v1.1 features disabled** but configuration still valid

---

## Data Migration

### Skip Tracking Data

**v1.0 skip tracking uses simple log file:**
- Location: `devforgeai/logs/feedback-skips.log`
- Format: Plain text log entries
- Migration: None needed (no breaking changes planned)

**v1.1 (planned):**
- May introduce SQLite database for skip tracking
- Automatic migration from log file to database
- Log file preserved as backup

---

## Testing After Migration

### Validation Checklist

After migrating to v1.0:

- [ ] Configuration file loads without errors
  ```bash
  tail devforgeai/logs/config-errors.log
  ```

- [ ] Hot-reload working (test by toggling `enabled` flag)
  ```bash
  echo "enabled: false" > devforgeai/config/feedback.yaml
  sleep 2
  # Verify feedback stopped
  ```

- [ ] Skip tracking functional
  ```bash
  tail -f devforgeai/logs/feedback-skips.log
  ```

- [ ] JSON Schema autocomplete working in IDE

- [ ] All tests passing
  ```bash
  python3 -m pytest .claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py -v
  ```

---

## Support

### Getting Help

If migration issues occur:

1. **Check logs:**
   ```bash
   cat devforgeai/logs/config-errors.log
   ```

2. **Run diagnostic:**
   ```bash
   # See TROUBLESHOOTING.md for diagnostic commands
   ```

3. **File migration bug report** with:
   - Version migrating from/to
   - Configuration file content
   - Log excerpts
   - Steps to reproduce

---

## Migration FAQ

### Q: Do I need to migrate if I don't have a configuration file?

**A:** No. Version 1.0 works out-of-the-box with sensible defaults. Migration only needed if you want custom configuration.

---

### Q: Will hot-reload work after migration?

**A:** Yes, immediately. No restart needed after creating configuration file.

---

### Q: Can I roll back to pre-1.0 behavior?

**A:** Yes. Delete configuration file or set `trigger_mode: failures-only` to match pre-1.0 behavior exactly.

---

### Q: What happens to existing skip tracking data?

**A:** Skip counter is fresh after upgrade (starts at 0). No historical skip data migrated from pre-1.0 (none existed).

---

### Q: Do I need to update my code?

**A:** No. Configuration system is entirely file-based, no code changes required.

---

### Q: Is there a migration tool?

**A:** Not needed for v1.0 migration (backward compatible). Future versions (v1.1+) may provide migration tools if breaking changes introduced.

---

## Resources

- **README:** `devforgeai/config/README.md` - Configuration guide with examples
- **Troubleshooting:** `devforgeai/config/TROUBLESHOOTING.md` - Common issues and solutions
- **Schema:** `devforgeai/config/feedback.schema.json` - JSON Schema for validation
- **Tests:** `.claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py` - Test suite

---

**Last Updated:** 2025-11-10
**Current Version:** 1.0
**Next Planned Version:** 1.1 (Q2 2025)
**Story:** STORY-011
