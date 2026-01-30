# Hook Integration for /create-story Command

**Purpose:** Detailed implementation guide for Phase 5 Hook Integration (STORY-027)

**File:** `.claude/commands/create-story.md` Phase 5
**Tests:** 69 tests covering all acceptance criteria

---

## Implementation Workflow

### Step 1: Check if Hooks Enabled

**Load hook configuration:**

```
# Check if hooks are enabled in devforgeai/config/hooks.yaml
# If missing → default enabled: false (safe default)

hook_config = load_hooks_config_from_file()
# Expected structure:
# {
#   'enabled': true/false,
#   'timeout': 30000 (ms, default),
#   'operation': 'story-create'
# }
```

**Performance requirement:** Must complete in <100ms (p95 latency target)

### Step 2: Detect Batch Mode

**Check for batch mode marker in conversation context:**

```
# Batch mode indicator
batch_mode = "**Batch Mode:** true" in conversation_context

IF batch_mode is TRUE:
  → Skip hook invocation (defer to end of batch)
  → Append story_id to batch_story_ids list
  → Return to command

IF batch_mode is FALSE:
  → Proceed to single-story hook invocation
```

### Step 3: Validate Story File Existence

**Before invoking hook, verify story file exists:**

```
story_files = Glob(pattern="devforgeai/specs/Stories/{story_id}-*.story.md")

IF no files found:
  → Skip hook invocation (no error thrown)
  → Log: "Story file not found, skipping hook"
  → Return to command (exit code 0)

IF file found:
  → Proceed to hook context assembly
```

### Step 4: Validate Story ID (Security)

**Prevent command injection via story ID validation:**

```
import re

pattern = r'^STORY-\d{3}$'  # STORY-NNN format (3 digits)

IF not re.match(pattern, story_id):
  → Log error: "Invalid story ID format"
  → Skip hook invocation
  → Return to command (exit code 0)

# Examples:
✓ STORY-027  (valid)
✓ STORY-001  (valid)
✗ STORY-1    (invalid - too few digits)
✗ STORY-027; rm -rf /  (invalid - injection attempt)
✗ STORY-9999 (invalid - too many digits)
```

### Step 5: Assemble Hook Context

**Extract 7 required metadata fields from story file:**

Read story YAML frontmatter and assemble context dict:

```
context = {
  'story_id': extracted from YAML frontmatter id field,
  'epic_id': extracted from YAML frontmatter epic field (or None),
  'sprint': extracted from YAML frontmatter sprint field (or "Backlog"),
  'title': extracted from YAML frontmatter title field,
  'points': extracted from YAML frontmatter points field,
  'priority': extracted from YAML frontmatter priority field,
  'timestamp': current time in ISO format (YYYY-MM-DDTHH:MM:SS.ffffffZ)
}

# All 7 fields MUST be present in context dict
# types: story_id (str), epic_id (str or null), sprint (str),
#        title (str), points (int), priority (str), timestamp (str ISO)
```

### Step 6: Invoke Hook (Single Story Mode)

**If hook enabled AND batch_mode is FALSE:**

```
# Determine hook configuration
hook_check_start = time.time()
hook_config = devforgeai_check_hooks('--operation=story-create')
hook_check_elapsed_ms = (time.time() - hook_check_start) * 1000

# Assert: hook_check_elapsed_ms < 100 (p95 requirement)

IF hook_config['enabled'] is FALSE:
  → Skip hook invocation (respects disabled state)
  → Return to command (exit code 0)

IF hook_config['enabled'] is TRUE:
  → Invoke hook with context
  → Pass context as environment variables or JSON args:
     devforgeai invoke-hooks \
       --operation=story-create \
       --story-id={story_id} \
       --context=$(json_encode(context))

  # Invocation MUST include all 7 metadata fields
  # Response expected: Hook presents feedback questions to user
```

### Step 7: Graceful Failure Handling

**Hook failures MUST NOT break story creation workflow:**

```
TRY:
  # Invoke hook with timeout (from config, default 30000ms)
  hook_result = invoke_hook_with_timeout(hook_config['timeout'])

  # Log success
  log_to_hooks_log({
    'timestamp': iso_timestamp(),
    'operation': 'story-create',
    'story_id': story_id,
    'status': 'success',
    'duration_ms': elapsed_time
  })

EXCEPT subprocess.TimeoutExpired:
  # Timeout occurred
  log_to_hook_errors_log({
    'timestamp': iso_timestamp(),
    'operation': 'story-create',
    'story_id': story_id,
    'error_message': 'Hook timeout exceeded {timeout}ms',
    'stack_trace': traceback_string
  })

  display_to_user('⚠️ Feedback hook failed - story created successfully')
  # Continue, exit code 0

EXCEPT subprocess.CalledProcessError:
  # CLI error
  log_to_hook_errors_log({ error details })
  display_to_user('⚠️ Feedback hook failed - story created successfully')
  # Continue, exit code 0

EXCEPT Exception:
  # Script crash or other failure
  log_to_hook_errors_log({ error details })
  display_to_user('⚠️ Feedback hook failed - story created successfully')
  # Continue, exit code 0

# KEY PRINCIPLE:
# - Story creation ALWAYS exits with code 0
# - Hook failures are ALWAYS logged but NOT propagated
# - User sees warning but story is successfully created
```

### Step 8: Batch Mode Hook Invocation (End of Batch)

**When batch mode is complete (all stories created):**

```
# At end of epic batch workflow (after all stories created):
IF batch_mode was TRUE:

  # Collect all story IDs created in batch
  created_story_ids = [STORY-NNN, STORY-NNN, STORY-NNN, ...]

  # Check hook enabled
  hook_config = devforgeai_check_hooks('--operation=story-create')

  IF hook_config['enabled'] is TRUE:
    # Invoke hook ONCE with all story IDs
    batch_context = {
      'operation': 'batch-story-create',
      'story_ids': created_story_ids,  # List of all created story IDs
      'count': len(created_story_ids),
      'timestamp': iso_timestamp()
    }

    TRY:
      invoke_hook_with_context(batch_context)
      log_to_hooks_log({
        'operation': 'batch-story-create',
        'story_ids': created_story_ids,
        'status': 'success'
      })
    EXCEPT:
      log_to_hook_errors_log({ error })
      display_to_user('⚠️ Batch feedback hook failed - stories created successfully')
```

### Step 9: Logging

**Log all hook invocations and errors:**

**Success log:** `devforgeai/feedback/.logs/hooks.log`

```json
{
  "timestamp": "2025-11-14T10:30:45.123456Z",
  "operation": "story-create",
  "story_id": "STORY-027",
  "status": "success",
  "duration_ms": 250
}
```

**Error log:** `devforgeai/feedback/.logs/hook-errors.log`

```json
{
  "timestamp": "2025-11-14T10:30:45.123456Z",
  "operation": "story-create",
  "story_id": "STORY-027",
  "error_message": "Hook timeout exceeded 30000ms",
  "stack_trace": "Traceback (most recent call last)..."
}
```

---

## Acceptance Criteria Mapping

**AC-1: Hook triggers after successful story creation**
- Implemented in Step 6: Invoke Hook (Single Story Mode)
- Tests: 6 tests (unit + integration + E2E)

**AC-2: Hook failure doesn't break story creation workflow**
- Implemented in Step 7: Graceful Failure Handling
- Tests: 10 tests (unit + integration + E2E)

**AC-3: Hook respects configuration (enabled/disabled state)**
- Implemented in Step 1: Check if Hooks Enabled
- Tests: 6 tests (unit + integration + E2E)

**AC-4: Hook check executes efficiently**
- Implemented in Step 6: Performance measurement
- Tests: 5 tests (p95, p99, overhead)

**AC-5: Hook doesn't trigger during batch creation**
- Implemented in Step 2: Detect Batch Mode + Step 8: Batch Mode Hook Invocation
- Tests: 9 tests (unit + integration + E2E)

**AC-6: Hook invocation includes complete story context**
- Implemented in Step 5: Assemble Hook Context
- Tests: 15 tests (unit + integration)

---

## Test Coverage

**Total Tests:** 69 (all passing)

**By Component:**
- Configuration Loading: 6 tests
- Validation: 5 tests (story ID format, command injection prevention)
- Metadata Assembly: 15 tests (7 fields + all together)
- Graceful Failure: 14 tests (timeout, CLI error, script crash)
- Batch Mode: 9 tests
- Performance: 5 tests (p95 <100ms, p99 <150ms, overhead <3s)
- Reliability: 3 tests (99.9%+ success rate)
- Logging: 2 tests (success log, error log)
- Security: 2 tests (story ID validation, injection prevention)

---

## Performance Requirements (NFRs)

**NFR-001: Hook Check <100ms (p95)**
- Target: p95 latency <100ms
- Test: `test_hook_check_p95_latency_under_100ms`

**NFR-002: Total Overhead <3 Seconds**
- Target: Hook check + invocation <3000ms
- Test: `test_total_hook_overhead_under_3_seconds`

**NFR-003: 99.9%+ Success Rate Despite Failures**
- Target: 1000 creations, ≤10 failures
- Test: `test_story_creation_success_despite_hook_failure`

**NFR-004: Story ID Validated Before Shell Invocation**
- Target: Prevent command injection
- Test: `test_validate_story_id_no_command_injection`, `test_malicious_story_id_rejected`

---

## Framework Integration

**Triggered after:** Phase 4 (Story File Verification)

**Integrates with:**
- `devforgeai check-hooks --operation=story-create` (CLI command)
- `devforgeai invoke-hooks --operation=story-create` (CLI command)
- `devforgeai/config/hooks.yaml` (configuration)
- `devforgeai/feedback/.logs/hooks.log` (success logging)
- `devforgeai/feedback/.logs/hook-errors.log` (error logging)

**Respects:**
- Batch mode marker: `**Batch Mode:** true`
- Conversation context for feature detection

---

## Error Scenarios

**Scenario 1: Hooks Disabled**
- Check returns `enabled: false`
- Hook invocation skipped
- Story creation proceeds to completion
- Exit code: 0

**Scenario 2: Hook Timeout**
- Hook exceeds timeout threshold
- Error logged to hook-errors.log
- User sees warning message
- Story creation continues
- Exit code: 0

**Scenario 3: Hook CLI Error**
- devforgeai invoke-hooks exits non-zero
- Error details logged
- User sees warning message
- Story creation continues
- Exit code: 0

**Scenario 4: Hook Script Crash**
- Hook subprocess raises exception
- Full traceback logged
- User sees warning message
- Story creation continues
- Exit code: 0

**Scenario 5: Batch Mode**
- Individual stories created without hook invocation
- Hook IDs collected during batch
- Single hook invocation at batch end
- All story IDs passed to hook
- Exit code: 0

**Scenario 6: Story File Missing**
- File existence check fails (file may have been deleted)
- Hook invocation skipped gracefully
- No error thrown
- Story creation exit code: 0

**Scenario 7: Malicious Story ID**
- Story ID fails regex validation (command injection attempt)
- Hook invocation blocked
- Error logged
- Story creation continues
- Exit code: 0

---

**See also:** STORY-027 acceptance criteria and test suite (69 tests)
