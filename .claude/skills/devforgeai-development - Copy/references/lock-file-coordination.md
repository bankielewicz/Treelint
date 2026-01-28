# Lock File Coordination Reference (STORY-096)

Complete guide for git commit lock coordination during Phase 08. Serializes git commits across parallel worktrees to prevent git index lock conflicts.

---

## Phase Progress Indicator

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 08/9: Git Workflow - Lock Coordination Step
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Display this indicator at the start of lock coordination.**

---

## Overview

Lock file coordination prevents race conditions when multiple parallel story worktrees attempt git commits simultaneously.

**Lock File Location:** `devforgeai/.locks/git-commit.lock`

**Lock File Format:**
```
pid: 12345
story_id: STORY-096
timestamp: 2025-12-17T10:30:00.123456Z
hostname: workstation-1
```

**Module:** `src/lock_file_coordinator.py`

---

## Step 5.0: Lock Acquisition Before Git Commit [STORY-096]

**Purpose:** Serialize git commits across parallel story worktrees.

**Execution:** BEFORE git add (Step 5.1 in current workflow)

### Step 5.0.1: Acquire Lock

**AC#1 Implementation:**

```
# Set lock acquisition parameters
STORY_ID = "${STORY_ID}"
TIMEOUT_SECONDS = 600  # 10 minutes (AC#4)
PROGRESS_INTERVAL = 5  # seconds (AC#2)

# Attempt lock acquisition
Display: ""
Display: "⏳ Acquiring git commit lock..."

result = Bash(command="python3 src/lock_file_coordinator.py acquire --story-id {STORY_ID} --timeout {TIMEOUT_SECONDS}")

# Parse result
lock_result = JSON.parse(result.stdout)

IF lock_result.success == true:
    Display: "✓ Git commit lock acquired"
    IF lock_result.stale_removed == true:
        Display: "  ℹ️  Removed stale lock from previous crashed process"
    PROCEED to Step 5.1 (git add)

ELIF lock_result.status == "timeout":
    # AC#4: Timeout prompt
    GOTO Step 5.0.4 (Timeout Prompt)

ELSE:
    Display: "❌ Failed to acquire lock: {lock_result.error_message}"
    HALT workflow
```

---

### Step 5.0.2: Wait with Progress Display

**AC#2 Implementation:**

When lock is held by another story, display progress updates every 5 seconds:

```
Progress Display Format:
"Waiting for git lock (held by {holder_story_id} PID {holder_pid})... {elapsed}s"

Example:
"Waiting for git lock (held by STORY-037 PID 12345)... 15s"
"Waiting for git lock (held by STORY-037 PID 12345)... 20s"
"Waiting for git lock (held by STORY-037 PID 12345)... 25s"
```

**Progress callback is automatic when using the Python module with progress_interval parameter.**

---

### Step 5.0.3: Stale Lock Detection

**AC#3 Implementation:**

A lock is considered stale when BOTH conditions are met:
1. The process holding the lock is no longer running (PID dead)
2. The lock is older than 5 minutes

```
Stale Detection Logic:
IF lock_file exists:
    pid = parse_lock_file().pid
    timestamp = parse_lock_file().timestamp

    pid_alive = os.kill(pid, 0)  # Signal 0 checks existence
    lock_age = now() - timestamp

    IF pid_alive == false AND lock_age > 5 minutes:
        # Lock is stale - auto-remove
        remove(lock_file)
        log("Removed stale lock (PID {pid} not running)")
        RETRY acquisition
```

**Why both conditions?**
- Young locks with dead PIDs might be from very recent crashes (give time for cleanup)
- Old locks with alive PIDs are legitimately held by long-running commits

---

### Step 5.0.4: Timeout Prompt (10 minutes)

**AC#4 Implementation:**

When lock wait exceeds 10 minutes, prompt user for action:

```
AskUserQuestion(
    questions=[{
        question: "Git commit lock wait has exceeded 10 minutes. The lock is held by {holder_story_id} (PID {holder_pid}). How would you like to proceed?",
        header: "Lock Timeout",
        multiSelect: false,
        options: [
            {
                label: "Continue waiting",
                description: "Keep waiting for the lock to be released. The other story may complete soon."
            },
            {
                label: "Force acquire lock (risky)",
                description: "⚠️ RISKY: Remove existing lock and proceed. Only use if you're certain the other process has crashed."
            },
            {
                label: "Abort",
                description: "Cancel commit operation. Your changes are preserved but not committed."
            }
        ]
    }]
)
```

**Handle user choice:**

```
IF user_choice == "Continue waiting":
    Display: "Continuing to wait for lock..."
    # Reset timeout and retry acquisition
    GOTO Step 5.0.1 with timeout reset

ELIF user_choice == "Force acquire lock":
    Display: "⚠️  Force acquiring lock (removing existing lock held by {holder_story_id})"
    result = Bash(command="python3 src/lock_file_coordinator.py acquire --story-id {STORY_ID} --force")

    IF result.success:
        Display: "✓ Lock force acquired"
        PROCEED to Step 5.1 (git add)
    ELSE:
        Display: "❌ Force acquire failed: {result.error_message}"
        HALT workflow

ELIF user_choice == "Abort":
    Display: ""
    Display: "═══════════════════════════════════════════════════════════"
    Display: "  Git Commit Aborted"
    Display: "═══════════════════════════════════════════════════════════"
    Display: ""
    Display: "Your changes are preserved but not committed."
    Display: "The other story ({holder_story_id}) still holds the lock."
    Display: ""
    Display: "Options:"
    Display: "  1. Wait for other story to complete, then re-run /dev {STORY_ID}"
    Display: "  2. Check status: python3 src/lock_file_coordinator.py status"
    Display: ""

    HALT workflow with exit_code=0 (clean abort)
```

---

## Step 5.3: Lock Release After Commit [STORY-096]

**AC#5 Implementation:**

**Purpose:** Release lock for next waiting story.

**Execution:** AFTER git commit completes (success OR failure)

```
# Lock release - ALWAYS runs (try/finally pattern)
try:
    # Step 5.1: git add
    Bash(command="git add {files}")

    # Step 5.2: git commit
    Bash(command="git commit -m '{message}'")

    # Mark success
    commit_success = true

finally:
    # Step 5.3: Release lock (always)
    Bash(command="python3 src/lock_file_coordinator.py release --story-id {STORY_ID}")
    Display: "✓ Git commit lock released"

IF commit_success:
    Display: "✓ Changes committed successfully"
ELSE:
    Display: "❌ Commit failed (lock released for other stories)"
```

**Critical:** Lock MUST be released even if commit fails to prevent deadlocks.

---

## CLI Reference

**Acquire lock:**
```bash
python3 src/lock_file_coordinator.py acquire --story-id STORY-096 --timeout 600
```

**Release lock:**
```bash
python3 src/lock_file_coordinator.py release --story-id STORY-096
```

**Check lock status:**
```bash
python3 src/lock_file_coordinator.py status
```

**Force acquire (risky):**
```bash
python3 src/lock_file_coordinator.py acquire --story-id STORY-096 --force
```

---

## Directory Structure

```
devforgeai/
└── .locks/
    └── git-commit.lock    # Active lock file (when held)
```

**Directory Creation:** Created automatically on first lock acquisition.

**Permissions:**
- Directory: 700 (drwx------)
- Lock file: 600 (-rw-------)

---

## Error Handling

### Lock Acquisition Timeout

```
Status: timeout
Action: Present AC#4 prompt (Continue/Force/Abort)
```

### Lock Acquisition Error

```
Status: error
Action: Display error message, suggest manual status check
```

### Lock Release by Wrong Story

```
Error: PermissionError - Lock not held by this story
Action: Log warning, continue (non-blocking)
```

### Stale Lock Detected

```
Status: Automatically removed
Action: Log removal, retry acquisition
```

---

## Integration Points

**Phase 08 Workflow:**
1. Pre-Commit DoD Validation (Phase 08.0)
2. **Lock Acquisition (Phase 08.0.5)** ← NEW
3. Git Add (Step 5.1)
4. Git Commit (Step 5.2)
5. **Lock Release (Step 5.3)** ← NEW
6. AC Checklist Updates
7. Completion

**Dependencies:**
- STORY-091 (Git Worktree Auto-Management) - provides parallel worktree infrastructure
- Python 3.x with standard library (no external dependencies)

---

## Performance Requirements

| Metric | Target | Validation |
|--------|--------|------------|
| Lock acquisition (no contention) | < 100ms | NFR-001 |
| Wait loop overhead | < 10ms/iteration | NFR |
| Stale detection | < 500ms | NFR |
| Lock release | < 50ms | NFR |

---

## Security Considerations

- Lock file permissions: 600 (owner only read/write)
- PID validation prevents spoofing (os.kill checks)
- Force acquire logs warning for audit trail
- Hostname tracked for distributed scenario debugging

---

**Story:** STORY-096 - Lock File Coordination for Critical Operations
**Epic:** EPIC-010 - Parallel Story Development
**Created:** 2025-12-17
