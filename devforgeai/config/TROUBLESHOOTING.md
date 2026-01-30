# Feedback Configuration Troubleshooting Guide

Solutions to common configuration issues.

---

## Common Issues

### 1. Configuration Not Loading

**Symptom:** Feedback system using defaults despite configuration file existing.

**Possible Causes:**
- File in wrong location
- YAML syntax errors
- File permissions

**Solutions:**

**Check file location:**
```bash
ls -la devforgeai/config/feedback.yaml
```

Expected: `devforgeai/config/feedback.yaml` (note the `devforgeai` directory with leading dot)

**Check YAML syntax:**
```bash
python3 -c "import yaml; yaml.safe_load(open('devforgeai/config/feedback.yaml'))"
```

If syntax error:
```
Error: while parsing a block mapping
  in "<file>", line 5, column 3
```

Fix: Validate YAML at https://yaml-checker.com/

**Check file permissions:**
```bash
chmod 644 devforgeai/config/feedback.yaml
```

**Check logs:**
```bash
tail -20 devforgeai/logs/config-errors.log
```

---

### 2. Invalid Configuration Values

**Symptom:** Configuration rejected with validation error.

**Error Examples:**

#### Invalid trigger_mode
```
Error: Invalid trigger_mode value: 'invalid-mode'. Must be one of: always, failures-only, specific-operations, never
```

**Fix:**
```yaml
# Wrong
trigger_mode: invalid-mode

# Correct
trigger_mode: failures-only
```

#### Missing operations array
```
Error: trigger_mode is 'specific-operations' but operations array is missing
```

**Fix:**
```yaml
# Wrong
trigger_mode: specific-operations

# Correct
trigger_mode: specific-operations
operations:
  - qa
  - development
```

#### Invalid format/tone
```
Error: Invalid templates.format value: 'unknown'. Must be one of: structured, free-text
```

**Fix:**
```yaml
# Wrong
templates:
  format: unknown
  tone: verbose

# Correct
templates:
  format: structured
  tone: brief
```

---

### 3. Hot-Reload Not Working

**Symptom:** Configuration changes not detected after saving file.

**Possible Causes:**
- File watcher not started
- File system doesn't support inotify (WSL, network drives)
- Changes saved to wrong file

**Solutions:**

**Check file watcher is running:**
```bash
# Check debug logs
tail -f devforgeai/logs/debug.log | grep -i "watcher"
```

Expected: `[INFO] ConfigFileWatcher started, monitoring devforgeai/config/feedback.yaml`

**Enable debug mode:**
```bash
export DEBUG_FEEDBACK_CONFIG=1
```

**Force reload manually:**
If hot-reload not working, restart the process/skill that uses configuration.

**WSL/Network drive workaround:**
Use polling-based detection (automatically enabled if inotify fails).

---

### 4. Skip Tracking Not Working

**Symptom:** Skip counter not incrementing or feedback not pausing after max skips.

**Possible Causes:**
- Skip tracking disabled
- max_consecutive_skips set to 0 (unlimited)
- Skip counter file permissions

**Solutions:**

**Check skip tracking enabled:**
```yaml
skip_tracking:
  enabled: true  # Must be true
  max_consecutive_skips: 3  # Must be >0 for limit
```

**Check skip counter log:**
```bash
tail -f devforgeai/logs/feedback-skips.log
```

Expected after skip:
```
[2025-11-10 10:30:15] INFO: Skip counter incremented to 2/3
```

**Check file permissions:**
```bash
ls -la devforgeai/logs/feedback-skips.log
chmod 644 devforgeai/logs/feedback-skips.log
```

---

### 5. Feedback Still Collected Despite enabled: false

**Symptom:** Feedback prompts appear even with `enabled: false`.

**Possible Causes:**
- Configuration not loaded
- Old configuration cached
- Multiple configuration files

**Solutions:**

**Verify configuration loaded:**
```bash
# Check config-errors.log for load confirmation
tail devforgeai/logs/config-errors.log
```

Expected: `[INFO] Configuration loaded successfully`

**Check for multiple config files:**
```bash
find . -name "feedback*.yaml" -o -name "*feedback*.yaml"
```

Remove any duplicates, keep only `devforgeai/config/feedback.yaml`

**Force reload:**
```bash
# Touch file to trigger hot-reload
touch devforgeai/config/feedback.yaml
```

---

### 6. Extremely Slow Configuration Load

**Symptom:** Configuration takes >1 second to load (target: <100ms).

**Possible Causes:**
- Very large configuration file
- Slow file system (network drive)
- Schema validation overhead

**Solutions:**

**Check file size:**
```bash
wc -c devforgeai/config/feedback.yaml
```

Expected: <5 KB

If larger: Remove comments, simplify structure.

**Check file system:**
```bash
time cat devforgeai/config/feedback.yaml > /dev/null
```

If slow (>50ms): Move configuration to local disk.

**Benchmark load time:**
```bash
export DEBUG_FEEDBACK_CONFIG=1
# Check debug.log for load time
tail devforgeai/logs/debug.log | grep "loaded in"
```

Expected: `Configuration loaded in <100ms`

---

### 7. IDE Autocomplete Not Working

**Symptom:** No autocomplete suggestions when editing `feedback.yaml`.

**Possible Causes:**
- JSON Schema file missing
- Editor not configured
- YAML extension not installed

**Solutions:**

**Check schema file exists:**
```bash
ls -la devforgeai/config/feedback.schema.json
```

If missing, regenerate:
```bash
python3 << 'EOF'
import json, sys
sys.path.insert(0, '.claude/scripts')
from devforgeai_cli.feedback.config_schema import get_schema
with open('devforgeai/config/feedback.schema.json', 'w') as f:
    json.dump(get_schema(), f, indent=2)
EOF
```

**VS Code:**
1. Install "YAML" extension (Red Hat)
2. Add to `.vscode/settings.json`:
```json
{
  "yaml.schemas": {
    "devforgeai/config/feedback.schema.json": "devforgeai/config/feedback.yaml"
  }
}
```

**IntelliJ/PyCharm:**
1. Settings → Languages & Frameworks → Schemas and DTDs
2. Add mapping: `feedback.schema.json` → `feedback.yaml`

---

### 8. Concurrent Operations Causing Issues

**Symptom:** Skip counter incorrect with multiple parallel operations.

**Possible Causes:**
- Thread safety issue (should not happen - file a bug)
- Race condition in file writes

**Solutions:**

**Verify thread safety:**
```bash
# Run concurrent test
python3 -m pytest .claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py::TestEdgeCases::test_edge_case_concurrent_skip_tracking_updates -v
```

Expected: PASSED

If failed: File a bug report with logs from `devforgeai/logs/feedback-skips.log`

---

### 9. Configuration Changes Not Persisted

**Symptom:** Hot-reload works but changes lost after restart.

**Possible Causes:**
- Editing wrong file (temporary copy)
- File permissions prevent write

**Solutions:**

**Verify editing correct file:**
```bash
# Check which file is being monitored
tail devforgeai/logs/debug.log | grep "monitoring"
```

Expected: `monitoring devforgeai/config/feedback.yaml`

**Check write permissions:**
```bash
touch devforgeai/config/feedback.yaml
echo $?  # Should output: 0
```

If permission denied: `chmod 644 devforgeai/config/feedback.yaml`

---

### 10. Empty Configuration File Behavior

**Symptom:** Unsure what happens with empty `feedback.yaml`.

**Expected Behavior:** System uses defaults (same as no file).

**Verification:**
```bash
# Create empty file
echo "" > devforgeai/config/feedback.yaml

# Check logs
tail devforgeai/logs/config-errors.log
```

Expected: `[INFO] Configuration file empty, using defaults`

---

## Diagnostic Commands

### Check Configuration Status

```bash
# Show current configuration
python3 << 'EOF'
import sys
sys.path.insert(0, '.claude/scripts')
from devforgeai_cli.feedback.config_manager import ConfigurationManager
mgr = ConfigurationManager.get_instance()
config = mgr.get_configuration()
print(f"Enabled: {config.enabled}")
print(f"Trigger Mode: {config.trigger_mode.value}")
print(f"Max Questions: {config.conversation_settings.max_questions}")
EOF
```

### Validate Configuration File

```bash
# Validate YAML syntax and schema
python3 << 'EOF'
import sys, yaml, jsonschema
sys.path.insert(0, '.claude/scripts')
from devforgeai_cli.feedback.config_schema import get_schema

with open('devforgeai/config/feedback.yaml') as f:
    config = yaml.safe_load(f)

try:
    jsonschema.validate(config, get_schema())
    print("✅ Configuration valid")
except jsonschema.ValidationError as e:
    print(f"❌ Validation error: {e.message}")
EOF
```

### Check Skip Counter

```bash
# Display current skip count
python3 << 'EOF'
import sys
sys.path.insert(0, '.claude/scripts')
from devforgeai_cli.feedback.skip_tracker import SkipTracker
tracker = SkipTracker()
count = tracker.get_skip_count()
print(f"Current skip count: {count}")
EOF
```

### Test Hot-Reload

```bash
# Test hot-reload detection
echo "enabled: false" > devforgeai/config/feedback.yaml
echo "Waiting 5 seconds for hot-reload detection..."
sleep 5
tail -5 devforgeai/logs/config-errors.log
```

---

## Log File Locations

| Log File | Purpose |
|----------|---------|
| `devforgeai/logs/config-errors.log` | Configuration load errors and validation |
| `devforgeai/logs/feedback-skips.log` | Skip tracking events |
| `devforgeai/logs/debug.log` | Debug-level logging (enable with `DEBUG_FEEDBACK_CONFIG=1`) |
| `devforgeai/logs/audit.log` | Configuration change audit trail |

**View all logs:**
```bash
tail -f devforgeai/logs/*.log
```

---

## Performance Benchmarking

### Configuration Load Time

```bash
python3 << 'EOF'
import time, sys
sys.path.insert(0, '.claude/scripts')

start = time.perf_counter()
from devforgeai_cli.feedback.config_manager import ConfigurationManager
mgr = ConfigurationManager.get_instance()
config = mgr.get_configuration()
elapsed = (time.perf_counter() - start) * 1000

print(f"Load time: {elapsed:.2f}ms (target: <100ms)")
print(f"Status: {'✅ PASS' if elapsed < 100 else '❌ FAIL'}")
EOF
```

### Skip Counter Lookup Time

```bash
python3 << 'EOF'
import time, sys
sys.path.insert(0, '.claude/scripts')
from devforgeai_cli.feedback.skip_tracker import SkipTracker

tracker = SkipTracker()

start = time.perf_counter()
count = tracker.get_skip_count()
elapsed = (time.perf_counter() - start) * 1000

print(f"Skip lookup time: {elapsed:.2f}ms (target: <10ms)")
print(f"Status: {'✅ PASS' if elapsed < 10 else '❌ FAIL'}")
EOF
```

---

## Getting Help

If issues persist after trying solutions above:

1. **Check logs:**
   ```bash
   cat devforgeai/logs/config-errors.log
   ```

2. **Run tests:**
   ```bash
   python3 -m pytest .claude/scripts/devforgeai_cli/tests/feedback/test_configuration_management.py -v
   ```

3. **File a bug report** with:
   - Configuration file content (sanitized)
   - Log file excerpts
   - Test results
   - Environment (OS, Python version)

---

**Last Updated:** 2025-11-10
**Version:** 1.0
**Story:** STORY-011
