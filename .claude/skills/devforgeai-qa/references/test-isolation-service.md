# Test Isolation Service Reference

Provides story-scoped test output isolation for concurrent QA validations.

**Configuration File:** `devforgeai/config/test-isolation.yaml`
**Schema:** `devforgeai/config/test-isolation.schema.json`
**Story:** STORY-092 (Story-Scoped Test Isolation)

---

## Overview

The Test Isolation Service ensures that test results, coverage reports, and logs are written to story-specific directories, preventing data corruption when multiple QA validations run concurrently.

**Directory Structure:**
```
tests/
├── results/
│   ├── STORY-091/
│   │   ├── test-results.xml
│   │   ├── test-output.log
│   │   └── timestamp.txt
│   └── STORY-092/
│       └── ...
├── coverage/
│   ├── STORY-091/
│   │   └── coverage.json
│   └── STORY-092/
│       └── ...
└── logs/
    └── STORY-NNN/
        └── test-output.log
```

---

## Path Resolution Algorithm

### Step 1: Load Configuration

```
Read(file_path="devforgeai/config/test-isolation.yaml")

IF file not found:
    Use defaults:
        results_base = "tests/results"
        coverage_base = "tests/coverage"
        logs_base = "tests/logs"
```

### Step 2: Generate Story-Scoped Paths

```python
def resolve_story_paths(story_id: str, config: dict) -> dict:
    """
    Generate all story-scoped paths for a given story ID.

    Args:
        story_id: Story identifier (e.g., "STORY-092")
        config: Loaded test-isolation.yaml configuration

    Returns:
        Dictionary with results_dir, coverage_dir, logs_dir paths
    """
    paths = config.get("paths", {})

    results_base = paths.get("results_base", "tests/results")
    coverage_base = paths.get("coverage_base", "tests/coverage")
    logs_base = paths.get("logs_base", "tests/logs")

    return {
        "results_dir": f"{results_base}/{story_id}",
        "coverage_dir": f"{coverage_base}/{story_id}",
        "logs_dir": f"{logs_base}/{story_id}"
    }
```

**Example:**
```
Input: story_id = "STORY-092"
Output: {
    "results_dir": "tests/results/STORY-092",
    "coverage_dir": "tests/coverage/STORY-092",
    "logs_dir": "tests/logs/STORY-092"
}
```

### Step 3: Validate Story ID

```python
def validate_story_id(story_id: str) -> bool:
    """
    Validate story ID format to prevent path traversal attacks.

    Valid: STORY-001, STORY-092, STORY-1234
    Invalid: ../STORY-001, STORY-001/../../etc, STORY 001
    """
    import re
    pattern = r'^STORY-\d{1,4}$'
    return bool(re.match(pattern, story_id))
```

---

## Directory Creation Logic

### Step 1: Check Configuration

```
IF config.directory.auto_create == false:
    Skip directory creation
    Return (directories must exist or test commands will fail)
```

### Step 2: Create Directories

```bash
# Linux/Mac
mkdir -p tests/results/{STORY_ID}
mkdir -p tests/coverage/{STORY_ID}
mkdir -p tests/logs/{STORY_ID}

# Apply permissions (Linux/Mac only)
chmod 755 tests/results/{STORY_ID}
chmod 755 tests/coverage/{STORY_ID}
chmod 755 tests/logs/{STORY_ID}
```

```bash
# Windows (PowerShell)
New-Item -ItemType Directory -Force -Path "tests\results\{STORY_ID}"
New-Item -ItemType Directory -Force -Path "tests\coverage\{STORY_ID}"
New-Item -ItemType Directory -Force -Path "tests\logs\{STORY_ID}"
# Note: Windows ignores Unix permissions
```

### Step 3: Validate Creation

```
IF directory creation failed:
    Display: "ERROR: Failed to create test isolation directories"
    Display: "Check write permissions on tests/ directory"
    HALT workflow
```

### Step 4: Write Timestamp

```
Write(file_path="tests/results/{STORY_ID}/timestamp.txt",
      content="{ISO_8601_TIMESTAMP}")

# Example: 2025-12-16T10:30:00Z
```

---

## Lock File Protocol

### Purpose

Prevents concurrent QA validations from corrupting each other's test outputs when two developers run `/qa STORY-091` and `/qa STORY-092` simultaneously with overlapping file writes.

### Lock File Location

```
tests/results/{STORY_ID}/.qa-lock
```

### Lock Acquisition Algorithm

```python
def acquire_lock(story_id: str, config: dict) -> bool:
    """
    Acquire exclusive lock for story test outputs.

    Returns True if lock acquired, False if timeout exceeded.
    """
    if not config.get("concurrency", {}).get("locking_enabled", True):
        return True  # Locking disabled, proceed without lock

    lock_file = f"tests/results/{story_id}/.qa-lock"
    lock_timeout = config["concurrency"].get("lock_timeout_seconds", 300)
    stale_threshold = config["concurrency"].get("stale_lock_threshold_seconds", 3600)

    start_time = now()

    while elapsed(start_time) < lock_timeout:
        if not exists(lock_file):
            # Create lock file
            write(lock_file, f"timestamp: {iso_now()}\nstory: {story_id}\npid: {pid()}")
            return True

        # Check if existing lock is stale
        lock_age = now() - file_mtime(lock_file)
        if lock_age > stale_threshold:
            # Remove stale lock and retry
            remove(lock_file)
            continue

        # Wait and retry
        sleep(5)

    return False  # Timeout exceeded
```

### Lock File Format

```yaml
timestamp: 2025-12-16T10:30:00Z
story: STORY-092
pid: 12345
hostname: dev-machine
```

### Lock Release

```
# At end of QA workflow (success or failure):
Remove(file_path="tests/results/{STORY_ID}/.qa-lock")
```

### Error Handling

```
IF lock acquisition failed (timeout):
    Display: "WARNING: Could not acquire lock for {STORY_ID}"
    Display: "Another QA validation may be running"
    AskUserQuestion:
        - "Wait and retry" - Wait 60 seconds and try again
        - "Force proceed" - Continue without lock (risk of data corruption)
        - "Cancel" - Abort QA validation
```

---

## Platform-Specific Notes

### Linux/Mac

- Full support for Unix permissions (755)
- `chmod` commands execute normally
- Lock files use file modification time for staleness detection

### Windows

- Permissions parameter ignored (Windows uses ACLs)
- Use PowerShell `New-Item` for directory creation
- Lock file staleness uses `(Get-Item).LastWriteTime`

### WSL (Windows Subsystem for Linux)

- Behaves like Linux for file operations
- Permissions may not persist across Windows/WSL boundary
- Recommended: Use native Linux paths within WSL

---

## Integration Points

### QA Skill (SKILL.md)

Test isolation integrates via three new phases:

```
Phase 0.5: Load Test Isolation Configuration
Phase 0.6: Create Story-Scoped Directories
Phase 0.7: Acquire Lock File (if enabled)
...
Final: Release Lock File
```

### Coverage Analysis Workflow

Test commands modified to output to story-scoped paths:

```bash
# Before (centralized)
pytest --cov=src --cov-report=json

# After (story-scoped)
pytest --cov=src --cov-report=json:tests/coverage/{STORY_ID}/coverage.json \
       --junitxml=tests/results/{STORY_ID}/test-results.xml
```

### QA Report Generation

Reports reference story-scoped paths:

```markdown
**Coverage Data Location:** `tests/coverage/{STORY_ID}/`
**Test Results Location:** `tests/results/{STORY_ID}/`
```

---

## Troubleshooting

### "Directory creation failed"

**Cause:** Insufficient permissions on `tests/` directory
**Solution:** `chmod 755 tests/` or run as user with write access

### "Lock timeout exceeded"

**Cause:** Previous QA validation crashed without releasing lock
**Solution:**
1. Check if another QA is actually running: `ps aux | grep qa`
2. If not, remove stale lock: `rm tests/results/STORY-NNN/.qa-lock`

### "Permission denied on Windows"

**Cause:** Windows ACLs preventing directory creation
**Solution:** Run terminal as Administrator or adjust folder permissions

### "Coverage file not found"

**Cause:** Test command used wrong output path
**Solution:** Verify test command includes story-scoped path parameters

---

## Configuration Reference

See `devforgeai/config/test-isolation.yaml` for full configuration options.

**Key settings:**
- `enabled`: Master switch (true/false)
- `paths.results_base`: Base directory for test results
- `paths.coverage_base`: Base directory for coverage
- `directory.auto_create`: Auto-create directories
- `concurrency.locking_enabled`: Enable file locking
- `cleanup.enabled`: Auto-cleanup old results

---

## Session Checkpoint Protocol [STORY-126 Enhancement]

**Purpose:** Persist workflow state to survive context clears and enable resume capability.

**Constitution Alignment:** Skills MUST NOT assume state from previous invocations (architecture-constraints.md line 38)

### Checkpoint File Location

```
devforgeai/qa/reports/{STORY_ID}/.qa-session-checkpoint.json
```

### Checkpoint File Format

```json
{
  "schema_version": "1.0",
  "story_id": "STORY-126",
  "skill": "devforgeai-qa",
  "mode": "deep",
  "started_at": "2025-12-23T16:00:00Z",
  "last_updated": "2025-12-23T16:15:00Z",
  "current_phase": 2,
  "completed_phases": [0, 1],
  "phase_details": {
    "0": {"status": "complete", "lock_acquired": true},
    "1": {"status": "complete", "traceability_score": 100, "coverage": 95},
    "2": {"status": "in_progress", "validators_invoked": ["code-reviewer", "security-auditor"], "validators_pending": ["test-automator"]}
  },
  "can_resume": true,
  "expiry": "2025-12-24T16:00:00Z"
}
```

### Checkpoint Write (End of Each Phase)

```
Read existing checkpoint (if any)
Update checkpoint with:
  - current_phase = {N+1}
  - completed_phases.append({N})
  - phase_details[{N}] = {completion_data}
  - last_updated = {TIMESTAMP}
  - expiry = {TIMESTAMP + 24 hours}

Write(file_path="devforgeai/qa/reports/{STORY_ID}/.qa-session-checkpoint.json",
      content={updated_checkpoint_json})

Display: "✓ Checkpoint saved at Phase {N}"
```

### Checkpoint Detection (Phase 0 Step 0.0)

```
Glob(pattern="devforgeai/qa/reports/{STORY_ID}/.qa-session-checkpoint.json")

IF checkpoint found:
    Read and parse JSON
    IF checkpoint.can_resume AND NOT expired:
        AskUserQuestion: "Resume from Phase {current_phase} or start fresh?"
    ELSE:
        Delete expired checkpoint, start fresh
```

### Checkpoint Cleanup

Checkpoints are automatically cleaned up when:
1. QA completes successfully (PASSED)
2. User chooses "Start fresh"
3. Checkpoint is expired (24 hours old)

### Session Checkpoint vs Phase Markers

| Feature | Session Checkpoint | Phase Markers |
|---------|-------------------|---------------|
| Purpose | Resume from interruption | Sequential verification |
| Format | JSON with state details | Simple YAML status |
| Cleanup | On completion or fresh start | On QA PASSED only |
| Contains | Full workflow state | Phase completion flag |
