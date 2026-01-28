# Memory File Operations

**Status:** LOCKED
**Version:** 1.0
**Created:** 2026-01-23
**Story:** STORY-303

---

## Purpose

Specification for memory file read/write operations enabling session state persistence and recovery.

---

## File Path Pattern

**Pattern:** `.claude/memory/sessions/{STORY_ID}-{workflow}-session.md`

**Examples:**
- `.claude/memory/sessions/STORY-303-dev-session.md`
- `.claude/memory/sessions/STORY-303-qa-session.md`
- `.claude/memory/sessions/STORY-303-release-session.md`

**Workflow Namespacing (BR-004):**
- `/dev` workflow uses `{STORY_ID}-dev-session.md`
- `/qa` workflow uses `{STORY_ID}-qa-session.md`
- `/release` workflow uses `{STORY_ID}-release-session.md`
- No collision between workflows for same story

---

## write_session_state() Function

### Purpose

Create or update memory file with current workflow state.

### Triggers

Memory file is written/updated on:
1. **Phase completion:** After any phase completes successfully
2. **Key decision:** When a decision is recorded (technology choice, approach selection)
3. **Blocker encounter:** When a blocking issue is detected

### Specification

```
write_session_state(
  story_id: String,          # STORY-NNN format
  workflow: String,          # dev|qa|release
  current_phase: String,     # 01-10
  phase_progress: Float,     # 0.0-1.0
  decisions: Array,          # Array of Decision objects
  blockers: Array            # Array of Blocker objects
) -> Result
```

### Implementation Pattern

1. **Construct file path:**
   ```
   path = ".claude/memory/sessions/{story_id}-{workflow}-session.md"
   ```

2. **Build YAML frontmatter:**
   ```yaml
   ---
   story_id: "{story_id}"
   epic_id: "{epic_id or null}"
   workflow_name: "{workflow}"
   current_phase: "{current_phase}"
   phase_progress: {phase_progress}
   decisions: {decisions array}
   blockers: {blockers array}
   session_started: "{session_started or now}"
   last_updated: "{now}"
   ---
   ```

3. **Atomic write (NFR-004):**
   - Write to temporary file first: `{path}.tmp`
   - Validate YAML syntax
   - Rename temp file to final path (atomic operation)
   - If rename fails, delete temp file

4. **Non-blocking (BR-001):**
   - Write operation should not block workflow
   - Log error if write fails, do not interrupt workflow

### Example Usage

```
# After Phase 02 completion
write_session_state(
  story_id="STORY-303",
  workflow="dev",
  current_phase="02",
  phase_progress=1.0,
  decisions=[
    {timestamp: "2026-01-23T14:30:00Z", description: "Use shell tests", rationale: "Framework feature"}
  ],
  blockers=[]
)
```

---

## read_session_state() Function

### Purpose

Read existing memory file for session recovery.

### Specification

```
read_session_state(
  story_id: String,
  workflow: String
) -> SessionState | null
```

### Returns

- **SessionState object** if valid memory file exists
- **null** if no memory file found or file is corrupted

### Implementation Pattern

1. **Construct file path:**
   ```
   path = ".claude/memory/sessions/{story_id}-{workflow}-session.md"
   ```

2. **Check file existence:**
   ```
   IF NOT file_exists(path):
     RETURN null
   ```

3. **Read and parse YAML:**
   ```
   content = Read(file_path=path)
   frontmatter = parse_yaml_frontmatter(content)
   ```

4. **Validate schema:**
   ```
   IF NOT validate_schema(frontmatter):
     handle_corrupted_file(path)
     RETURN null
   ```

5. **Return SessionState:**
   ```
   RETURN SessionState(
     story_id=frontmatter.story_id,
     epic_id=frontmatter.epic_id,
     workflow_name=frontmatter.workflow_name,
     current_phase=frontmatter.current_phase,
     phase_progress=frontmatter.phase_progress,
     decisions=frontmatter.decisions,
     blockers=frontmatter.blockers,
     session_started=frontmatter.session_started,
     last_updated=frontmatter.last_updated
   )
   ```

---

## Session Recovery Workflow

### Detection

On workflow start, check for existing memory file:

```
state = read_session_state(story_id, workflow)

IF state is not null:
  # Previous session found
  Display: "Previous session found at Phase {state.current_phase} ({state.phase_progress * 100}% complete)"

  # User confirmation prompt (BR-002: opt-in recovery)
  AskUserQuestion:
    Question: "Resume from previous session?"
    Options:
      - "Yes, resume from Phase {state.current_phase}"
      - "No, start fresh session"

  IF user selects "Yes, resume":
    Display: "Resuming from Phase {state.current_phase} ({state.phase_progress * 100}% complete)"
    SET current_phase = state.current_phase
    # Skip completed phases, continue from current_phase

  ELSE:
    # Start fresh - handled by backward compatibility logic

ELSE:
  # No previous state found
  # Fresh session start (AC#4)
```

### Skip Completed Phases

When resuming:

```
# Determine which phases to skip
completed_phase = parse_int(state.current_phase)

FOR phase in [1..completed_phase - 1]:
  Display: "Phase 0{phase}: Already completed (bypassed)"

# Start execution from current_phase
Display: "Continuing from Phase 0{completed_phase}..."
```

---

## Fresh Session Start (Backward Compatibility)

### Condition

No memory file exists for story (legacy workflow or first execution):

```
state = read_session_state(story_id, workflow)

IF state is null:
  # Fresh session - no previous state found
  Display: "Starting fresh session (no previous state found)"

  # Initialize new memory file
  write_session_state(
    story_id=story_id,
    workflow=workflow,
    current_phase="01",
    phase_progress=0.0,
    decisions=[],
    blockers=[]
  )

  # Begin from Phase 01
  SET current_phase = "01"
```

### Legacy Workflow Handling

For existing workflows without memory files:
- Workflow proceeds normally from Phase 01
- New memory file created at first phase completion
- No disruption to initial run or legacy stories

---

## handle_corrupted_file() Function

### Purpose

Gracefully handle corrupted memory files (BR-003).

### Triggers

Corruption detected when:
- Invalid YAML syntax (parse error)
- Missing required fields (schema validation failure)
- Truncated file (incomplete frontmatter, missing `---` delimiter)
- Malformed data types (e.g., phase_progress not a float)

### Specification

```
handle_corrupted_file(
  file_path: String
) -> void
```

### Implementation Pattern

1. **Log warning:**
   ```
   Display: "Warning: Memory file corrupted, starting fresh session"
   ```

2. **Preserve corrupted file (rename with timestamp):**
   ```
   timestamp = now().format("YYYYMMDD-HHmmss")
   corrupted_path = "{file_path}.corrupted.{timestamp}"
   rename(file_path, corrupted_path)
   Display: "Corrupted file preserved: {corrupted_path}"
   ```

3. **Create fresh memory file:**
   ```
   write_session_state(
     story_id=extracted_story_id,
     workflow=extracted_workflow,
     current_phase="01",
     phase_progress=0.0,
     decisions=[],
     blockers=[]
   )
   ```

4. **Continue from Phase 01:**
   ```
   SET current_phase = "01"
   Display: "Starting from Phase 01"
   ```

### Corruption Scenarios

| Scenario | Detection | Handling |
|----------|-----------|----------|
| Invalid YAML syntax | YAML parse error | Preserve + restart |
| Missing required field | Schema validation fail | Preserve + restart |
| Truncated file (incomplete) | Missing closing `---` | Preserve + restart |
| Invalid data type | Type validation fail | Preserve + restart |
| Empty file | No frontmatter found | Preserve + restart |

---

## Atomic Write Pattern (NFR-004)

### Purpose

Prevent partial writes that could corrupt memory files.

### Implementation

```
1. Write content to temporary file:
   temp_path = "{target_path}.tmp"
   Write(file_path=temp_path, content=yaml_content)

2. Validate temporary file:
   IF parse_yaml(temp_path) fails:
     delete(temp_path)
     RETURN error

3. Atomic rename:
   rename(temp_path, target_path)
   # Rename is atomic on most filesystems

4. Cleanup on error:
   IF rename fails:
     delete(temp_path)
     RETURN error
```

### Crash Recovery

If session crashes during write:
- Temp file `.tmp` may exist alongside original
- On next session start, delete orphaned `.tmp` files
- Original file remains intact (no partial writes)

---

## Reference

- **Schema:** See `memory-file-schema.md` for field definitions
- **Phase 01 Integration:** Session recovery check added to preflight
- **Business Rules:** BR-001 (non-blocking), BR-002 (opt-in), BR-003 (preserve), BR-004 (namespace)
- **NFRs:** NFR-004 (atomic), NFR-005 (corruption detection)
