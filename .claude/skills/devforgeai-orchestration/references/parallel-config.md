# Parallel Configuration Reference

**Story:** STORY-108
**Version:** 1.0

## Quick Reference

**Config Location:** `devforgeai/config/parallel-orchestration.yaml`

**Profile Presets:**
| Profile | Concurrent Tasks | Timeout | Use Case |
|---------|-----------------|---------|----------|
| **pro** | 4 | 120000ms (2min) | Claude Pro subscribers |
| **max** | 6 | 180000ms (3min) | Power users |
| **api** | 8 | 300000ms (5min) | API/Enterprise users |
| **custom** | User-defined | User-defined | Non-standard configs |

---

## Config Loading Pattern

Skills load configuration using the Read tool:

```markdown
Read(file_path="devforgeai/config/parallel-orchestration.yaml")
```

**Example: Extract Profile Settings**
```bash
# Extract default profile
DEFAULT_PROFILE=$(grep "^default_profile:" config.yaml | awk '{print $2}' | tr -d '"')

# Extract max_concurrent_tasks for profile
MAX_TASKS=$(grep -A 10 "^  $DEFAULT_PROFILE:" config.yaml | grep "max_concurrent_tasks:" | head -1 | awk '{print $2}')

# Extract timeout_ms for profile
TIMEOUT_MS=$(grep -A 10 "^  $DEFAULT_PROFILE:" config.yaml | grep "timeout_ms:" | head -1 | awk '{print $2}')
```

---

## Validation Rules

Validate config values using Grep patterns:

### BR-001: max_concurrent_tasks Range (1-10)

```bash
MAX_TASKS=$(grep -A 10 "^  pro:" parallel-orchestration.yaml | grep "max_concurrent_tasks:" | head -1 | awk '{print $2}')

if [ "$MAX_TASKS" -lt 1 ] || [ "$MAX_TASKS" -gt 10 ]; then
  echo "ERROR: max_concurrent_tasks must be 1-10 (got: $MAX_TASKS)"
  exit 1
fi
```

**Rationale:** Maps to Anthropic subscription tier rate limits.

### BR-002: timeout_ms Range (1000-600000)

```bash
TIMEOUT=$(grep -A 10 "^  pro:" parallel-orchestration.yaml | grep "timeout_ms:" | head -1 | awk '{print $2}')

if [ "$TIMEOUT" -lt 1000 ] || [ "$TIMEOUT" -gt 600000 ]; then
  echo "ERROR: timeout_ms must be 1000-600000 (got: $TIMEOUT)"
  exit 1
fi
```

**Rationale:** Prevents indefinite hangs while allowing long operations.

### BR-003: Preset Locking

Preset profiles (pro/max/api) have locked defaults. Modifications trigger warnings.

---

## Default Fallback

When config file is missing or invalid, use default "pro" profile:

```yaml
# Default fallback values
default_profile: "pro"
max_concurrent_tasks: 4
timeout_ms: 120000
retry:
  max_attempts: 3
  backoff_ms: 1000
```

**Pattern:**
```bash
CONFIG_FILE="devforgeai/config/parallel-orchestration.yaml"

if [ ! -f "$CONFIG_FILE" ]; then
    echo "WARNING: Config not found, using default 'pro' profile"
    MAX_TASKS=4
    TIMEOUT_MS=120000
    RETRY_ATTEMPTS=3
    RETRY_BACKOFF=1000
fi
```

---

## Timeout Handling Pattern

Monitor long-running tasks and terminate gracefully when timeout_ms exceeded.

### Background Task with Timeout

```markdown
# Start task with 2-minute timeout (120000ms)
Bash(command="npm test", run_in_background=true, timeout=120000)

# Check task status
TaskOutput(task_id="...", timeout=120000)
```

### Timeout Behavior

1. **Task runs** up to `timeout_ms` milliseconds
2. **If timeout exceeded**, task receives SIGTERM (graceful shutdown)
3. **If still running after 5s**, task receives SIGKILL (force kill via KillShell)
4. **Error logged** with timeout details
5. **Skill receives** timeout error and can retry or fail gracefully

### Graceful Termination with KillShell

When a task exceeds `timeout_ms`, use KillShell for termination:

```markdown
# Terminate stuck background task
KillShell(shell_id="task_id")
```

**Logger Pattern:**
```python
logger.error(f"Task exceeded timeout ({timeout_ms}ms), terminated gracefully")
logger.warning(f"Task {task_id} killed after timeout")
```

### Example: Task with Timeout Monitoring

```bash
#!/bin/bash
# Start background task
TASK_ID=$(start_background_task "long_running_command")
START_TIME=$(date +%s%3N)
TIMEOUT_MS=120000

while task_is_running "$TASK_ID"; do
    ELAPSED=$(($(date +%s%3N) - START_TIME))
    if [ "$ELAPSED" -gt "$TIMEOUT_MS" ]; then
        echo "ERROR: Task exceeded timeout (${TIMEOUT_MS}ms)"
        kill_task "$TASK_ID"  # KillShell equivalent
        exit 1
    fi
    sleep 1
done
```

---

## Troubleshooting

### Config Not Loaded

**Symptom:** Skills report using default settings despite config file existing

**Solutions:**
1. Verify file path: `devforgeai/config/parallel-orchestration.yaml` (not `devforgeai/`)
2. Check YAML syntax: `python3 -c "import yaml; yaml.safe_load(open('...'))`
3. Ensure file is readable: `ls -la devforgeai/config/parallel-orchestration.yaml`

### Validation Errors

**Symptom:** `scripts/validate-parallel-config.sh` reports errors

**Common Issues:**
- **max_concurrent_tasks out of range:** Must be 1-10
- **timeout_ms out of range:** Must be 1000-600000
- **Missing required field:** Check version, default_profile, profiles sections

### Preset Profile Override Warning

**Symptom:** `WARNING: Preset profile 'pro' has non-standard values`

**Fix:** Use 'custom' profile for non-standard configurations instead of modifying preset profiles.

---

## Related Documentation

- [EPIC-017](devforgeai/specs/Epics/EPIC-017-parallel-task-orchestration.epic.md) - Parallel Task Orchestration
- [Tech Stack](devforgeai/specs/context/tech-stack.md) - Framework constraints
- [Anthropic Rate Limits](https://docs.anthropic.com/en/docs/rate-limits) - API limits reference
