# Graceful Degradation Policy

Policy for handling feedback system failures without blocking command execution.

---

## Core Principle

**The feedback system NEVER blocks command execution.**

Commands must succeed even if:
- Feedback directory is not writable
- JSON response is invalid
- Network interruption occurs
- Storage is full
- Configuration is missing

---

## Failure Scenarios and Handling

### Scenario 1: Feedback Directory Not Writable

**Cause:**
- Incorrect file permissions
- Disk full
- Read-only filesystem

**Detection:**
```python
import os
from pathlib import Path

feedback_dir = Path('devforgeai/feedback')

try:
    test_file = feedback_dir / '.write_test'
    test_file.touch()
    test_file.unlink()
except (PermissionError, OSError) as e:
    # Directory not writable
    pass
```

**Behavior:**
1. Log warning to console
2. Skip feedback collection
3. Command continues normally
4. Return success status

**Error message:**
```
⚠️  Warning: Feedback directory not writable (devforgeai/feedback)
    Feedback collection skipped, command will continue.
    Fix: Check file permissions or disk space.
```

**Recovery:**
```bash
# Check permissions
ls -la devforgeai/feedback/

# Fix if needed
chmod 755 devforgeai/feedback/

# Check disk space
df -h .
```

---

### Scenario 2: Invalid JSON Response

**Cause:**
- User enters malformed JSON
- Encoding issue
- Special characters not escaped

**Detection:**
```python
import json

try:
    data = json.loads(user_response)
except json.JSONDecodeError as e:
    # Invalid JSON
    pass
```

**Behavior:**
1. Reject invalid response
2. Re-prompt user with error message
3. Allow retry (up to 3 attempts)
4. Offer skip option
5. Command continues regardless

**Error message:**
```
❌ Invalid response format. Please try again.
   (Attempt 2 of 3)

Your response:
_
```

**After 3 failures:**
```
⚠️  Unable to collect valid response after 3 attempts.
    Feedback will be skipped for this question.

Continue with remaining questions? [Yes] [Skip all]
```

**Recovery:**
- User corrects input
- User skips question
- User skips entire feedback session
- Command proceeds in all cases

---

### Scenario 3: Network Interruption During Collection

**Cause:**
- Terminal connection lost
- SSH session dropped
- Process killed

**Detection:**
```python
import signal

def handle_interrupt(signum, frame):
    """Handle SIGINT (Ctrl+C) gracefully"""
    save_partial_feedback()
    print("\n⚠️  Feedback interrupted. Partial responses saved.")
    exit(0)

signal.signal(signal.SIGINT, handle_interrupt)
```

**Behavior:**
1. Save all collected responses so far
2. Mark session as "partial"
3. Offer continue-previous option on next operation

**Partial save:**
```json
{
  "feedback_id": "uuid-here",
  "timestamp": "2025-01-09T14:30:22Z",
  "story_id": "STORY-042",
  "workflow_type": "dev",
  "success_status": "success",
  "session_status": "partial",
  "questions": [
    {
      "question_id": "dev_success_01",
      "response": 5,
      "skip": false
    },
    {
      "question_id": "dev_success_02",
      "response": null,
      "skip": false,
      "interrupted": true
    }
  ]
}
```

**Next operation:**
```
Would you like to complete the interrupted feedback session from STORY-042?
(2 questions remaining)

[Yes] [No, start fresh] [Skip feedback]
```

**Recovery:**
- User continues previous session
- User starts fresh
- User skips
- Command proceeds in all cases

---

### Scenario 4: Storage Full

**Cause:**
- Disk quota exceeded
- Partition full

**Detection:**
```python
import shutil

def check_storage():
    stat = shutil.disk_usage('devforgeai/feedback')
    available_mb = stat.free / (1024 * 1024)
    return available_mb > 10  # Need at least 10 MB
```

**Behavior:**
1. Check available storage before collecting
2. If insufficient, skip collection with warning
3. Command continues

**Error message:**
```
⚠️  Warning: Insufficient disk space for feedback collection.
    Available: 3 MB, Required: 10 MB
    Feedback skipped, command will continue.
    Fix: Free up disk space or clean old feedback.
```

**Recovery:**
```bash
# Check disk space
df -h devforgeai/

# Clean old feedback (12+ months)
python3 devforgeai/scripts/cleanup-old-feedback.py --older-than=12m

# Or manually remove old backups
rm devforgeai/backups/feedback/feedback_backup_202401*.tar.gz
```

---

### Scenario 5: Configuration Missing

**Cause:**
- First run (no config created yet)
- Config deleted
- Migration from older version

**Detection:**
```python
config_path = Path('devforgeai/feedback/config.yaml')
if not config_path.exists():
    # No config found
    pass
```

**Behavior:**
1. Create default configuration
2. Use sensible defaults
3. Proceed with feedback collection
4. Command continues normally

**Default config created:**
```yaml
# devforgeai/feedback/config.yaml
enable_feedback: true
mode: all  # all, failures_only, disabled
max_skip_count: 3
retention_months: 12
anonymize_after: true
```

**Console message:**
```
ℹ️  Feedback system initialized with default configuration.
   Config: devforgeai/feedback/config.yaml
```

**Recovery:**
- Automatic (no user action needed)
- User can customize config after creation

---

### Scenario 6: Question Bank Missing

**Cause:**
- Corrupted installation
- File deleted accidentally
- Git checkout issue

**Detection:**
```python
questions_file = Path('devforgeai/feedback/questions.yaml')
if not questions_file.exists():
    # Question bank missing
    pass
```

**Behavior:**
1. Log error
2. Skip feedback collection
3. Command continues
4. Suggest reinstallation

**Error message:**
```
❌ Error: Feedback question bank not found.
   Expected: devforgeai/feedback/questions.yaml
   Feedback skipped, command will continue.
   Fix: Reinstall framework or restore from backup.
```

**Recovery:**
```bash
# Restore from git
git checkout devforgeai/feedback/questions.yaml

# Or reinstall framework
pip install --upgrade --force-reinstall devforgeai
```

---

### Scenario 7: Schema Validation Failure

**Cause:**
- Response doesn't match schema
- New question type added
- Version mismatch

**Detection:**
```python
import jsonschema

schema = load_schema('devforgeai/feedback/schema.json')
try:
    jsonschema.validate(feedback_data, schema)
except jsonschema.ValidationError as e:
    # Schema validation failed
    pass
```

**Behavior:**
1. Log validation error
2. Save feedback with validation_failed flag
3. Continue command execution
4. Flag for manual review

**Saved feedback:**
```json
{
  "feedback_id": "uuid",
  "validation_status": "failed",
  "validation_error": "Response type mismatch",
  "data": { ... }
}
```

**Console message:**
```
⚠️  Feedback validation failed but has been saved for review.
    Command will continue normally.
```

**Recovery:**
- Maintainer reviews flagged feedback
- Updates schema if needed
- Fixes question definition
- Migrates old feedback if necessary

---

## Testing Graceful Degradation

### Test Suite

```bash
#!/bin/bash
# Test graceful degradation

echo "=== Testing Graceful Degradation ==="

# Test 1: Directory not writable
echo -e "\nTest 1: Read-only feedback directory"
chmod 444 devforgeai/feedback/
/dev STORY-001
# Expected: Warning logged, feedback skipped, command succeeds
chmod 755 devforgeai/feedback/

# Test 2: Disk full (simulate with quota)
echo -e "\nTest 2: Disk full"
# (Requires specific setup, skip in automated tests)

# Test 3: Interruption (Ctrl+C)
echo -e "\nTest 3: Interrupted feedback"
timeout 5s /dev STORY-001 || true
# Expected: Partial feedback saved

# Test 4: Missing config
echo -e "\nTest 4: Missing configuration"
mv devforgeai/feedback/config.yaml /tmp/config.yaml.backup
/dev STORY-001
# Expected: Default config created, command succeeds
mv /tmp/config.yaml.backup devforgeai/feedback/config.yaml

# Test 5: Missing question bank
echo -e "\nTest 5: Missing question bank"
mv devforgeai/feedback/questions.yaml /tmp/questions.yaml.backup
/dev STORY-001
# Expected: Error logged, feedback skipped, command succeeds
mv /tmp/questions.yaml.backup devforgeai/feedback/questions.yaml

echo -e "\n=== All tests completed ==="
echo "Review console output to verify graceful degradation behavior."
```

### Verification Checklist

After each test:
- [ ] Command completed successfully (exit code 0)
- [ ] Appropriate error/warning message displayed
- [ ] Feedback gracefully skipped or partially saved
- [ ] No exceptions thrown
- [ ] No data corruption
- [ ] Recovery instructions provided

---

## Logging

### Log Levels

**INFO:** Normal operation
```
[INFO] Feedback collected for STORY-042 (4/4 questions answered)
```

**WARNING:** Non-critical issue, degraded mode
```
[WARNING] Feedback directory not writable, collection skipped
```

**ERROR:** Critical issue, unable to collect
```
[ERROR] Question bank not found, feedback system disabled
```

### Log Location

```
devforgeai/feedback/system.log
```

### Log Format

```
2025-01-09 14:30:22 [INFO] Feedback session started for STORY-042
2025-01-09 14:30:45 [WARNING] Storage low (5 MB available), partial collection
2025-01-09 14:31:02 [INFO] Feedback saved (3/5 questions answered)
```

---

## Monitoring

### Health Checks

**Daily health check script:**

```bash
#!/bin/bash
# Check feedback system health

echo "=== Feedback System Health Check ==="

# Check directory permissions
if [ -w devforgeai/feedback ]; then
    echo "✅ Directory writable"
else
    echo "❌ Directory not writable"
fi

# Check disk space
AVAILABLE=$(df devforgeai | tail -1 | awk '{print $4}')
if [ $AVAILABLE -gt 10000 ]; then
    echo "✅ Disk space sufficient ($AVAILABLE KB)"
else
    echo "⚠️  Disk space low ($AVAILABLE KB)"
fi

# Check required files
FILES=("schema.json" "questions.yaml" "config.yaml")
for file in "${FILES[@]}"; do
    if [ -f "devforgeai/feedback/$file" ]; then
        echo "✅ $file present"
    else
        echo "❌ $file missing"
    fi
done

# Check recent errors
ERRORS=$(grep -c "ERROR" devforgeai/feedback/system.log 2>/dev/null || echo 0)
echo "Errors in last 24h: $ERRORS"
```

### Alerting

**Set up alerts for:**
- 3+ consecutive feedback failures
- Storage below 50 MB
- Missing critical files
- Validation failures >10% of sessions

---

## Recovery Procedures

### Full System Recovery

```bash
#!/bin/bash
# Restore feedback system to working state

echo "=== Feedback System Recovery ==="

# 1. Restore from backup
LATEST_BACKUP=$(ls -t devforgeai/backups/feedback/*.tar.gz | head -1)
if [ -n "$LATEST_BACKUP" ]; then
    echo "Restoring from $LATEST_BACKUP"
    tar -xzf "$LATEST_BACKUP"
fi

# 2. Restore files from git
git checkout devforgeai/feedback/schema.json
git checkout devforgeai/feedback/questions.yaml

# 3. Reset permissions
chmod 755 devforgeai/feedback
chmod 644 devforgeai/feedback/*.json
chmod 644 devforgeai/feedback/*.yaml
chmod 644 devforgeai/feedback/*.md

# 4. Create default config if missing
if [ ! -f devforgeai/feedback/config.yaml ]; then
    cat > devforgeai/feedback/config.yaml << 'EOF'
enable_feedback: true
mode: all
max_skip_count: 3
retention_months: 12
anonymize_after: true
EOF
fi

# 5. Verify integrity
python3 devforgeai/scripts/validate-feedback-integrity.py

echo "=== Recovery complete ==="
echo "Run health check: devforgeai/scripts/check-feedback-health.sh"
```

---

## Best Practices

### For Developers

1. **Always check feedback system availability**
   ```python
   if feedback_system_available():
       collect_feedback()
   # Continue command regardless
   ```

2. **Use try-except blocks**
   ```python
   try:
       save_feedback(data)
   except Exception as e:
       log_warning(f"Feedback save failed: {e}")
       # Command continues
   ```

3. **Provide clear error messages**
   - What went wrong
   - Impact (feedback skipped, command continues)
   - How to fix

4. **Test failure scenarios**
   - Simulate each scenario
   - Verify graceful degradation
   - Confirm command success

### For Users

1. **Monitor warnings**
   - Act on storage warnings promptly
   - Fix permission issues
   - Report persistent errors

2. **Keep backups**
   - Weekly backups automatic
   - Don't delete recent backups
   - Verify backup integrity

3. **Report issues**
   - File bug if feedback consistently fails
   - Include error messages
   - Describe steps to reproduce

---

## Summary

**Key guarantees:**
- ✅ Commands always succeed (feedback optional)
- ✅ Errors logged and communicated clearly
- ✅ Partial data saved when possible
- ✅ Recovery procedures documented
- ✅ User control maintained

**Failure modes handled:**
- Directory not writable
- Invalid responses
- Network interruption
- Storage full
- Missing configuration
- Missing question bank
- Schema validation failure

**All failure modes result in:** Warning logged + Feedback skipped + Command succeeds
