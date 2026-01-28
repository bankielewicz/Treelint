# Resume Logic for Ideation Sessions

**Status**: LOCKED
**Version**: 1.0
**STORY**: STORY-137
**Depends On**: STORY-136 (Checkpoint Protocol)

## Overview

Resume-from-checkpoint logic that enables users to continue ideation sessions after disconnects or context window clears.

---

## Resume Workflow (Phase 1 Step 0)

**CRITICAL**: Checkpoint detection MUST occur BEFORE any user prompts in Phase 1.

### Step 0.1: Detect Existing Checkpoints

```
# CheckpointDetector: Detect checkpoint files using Glob
checkpoints = Glob(pattern="devforgeai/temp/.ideation-checkpoint-*.yaml")

IF checkpoints is empty:
    # No checkpoints - proceed with fresh session
    GOTO Phase 1 Step 1 (normal ideation start)
ELSE:
    # Checkpoints found - present resume choice
    GOTO Step 0.2 (Load and Sort Checkpoints)
```

### Step 0.2: Load and Sort Checkpoints

```
# Load each checkpoint to extract metadata for sorting
checkpoint_metadata = []

FOR each checkpoint_path in checkpoints:
    content = Read(file_path=checkpoint_path)
    parsed = yaml.safe_load(content)

    checkpoint_metadata.append({
        "path": checkpoint_path,
        "session_id": parsed.get("session_id"),
        "timestamp": parsed.get("timestamp"),
        "current_phase": parsed.get("current_phase"),
        "problem_preview": parsed.get("brainstorm_context", {}).get("problem_statement", "")[:50]
    })

# Sort by timestamp (newest first)
checkpoint_metadata.sort(key=lambda x: x["timestamp"], reverse=True)
```

### Step 0.3: Present Resume Choice

**Single Checkpoint:**
```
IF len(checkpoint_metadata) == 1:
    checkpoint = checkpoint_metadata[0]

    AskUserQuestion(
        questions=[{
            "question": "Resume previous ideation session?",
            "header": "Resume",
            "options": [
                {
                    "label": f"Resume from checkpoint ({checkpoint['current_phase']}/6 phases complete, {checkpoint['timestamp']})",
                    "description": f"Continue: {checkpoint['problem_preview']}..."
                },
                {
                    "label": "Start fresh (discard checkpoint)",
                    "description": "Begin a new ideation session from scratch"
                }
            ],
            "multiSelect": false
        }]
    )
```

**Multiple Checkpoints:**
```
IF len(checkpoint_metadata) > 1:
    options = []

    FOR i, checkpoint in enumerate(checkpoint_metadata):
        options.append({
            "label": f"Session {i+1}: {checkpoint['current_phase']}/6 phases ({checkpoint['timestamp']})",
            "description": f"{checkpoint['problem_preview']}..."
        })

    options.append({
        "label": "Start fresh (discard all checkpoints)",
        "description": "Begin a new ideation session from scratch"
    })

    AskUserQuestion(
        questions=[{
            "question": "Multiple sessions found. Which would you like to resume?",
            "header": "Sessions",
            "options": options,
            "multiSelect": false
        }]
    )
```

---

## Checkpoint Loading and Validation

### CheckpointLoader

```
FUNCTION load_checkpoint(checkpoint_path):
    # Step 1: Read file content
    content = Read(file_path=checkpoint_path)

    # Step 2: Parse YAML
    TRY:
        parsed = yaml.safe_load(content)
    EXCEPT yaml.YAMLError as e:
        Display: "⚠️ Warning: Checkpoint file is malformed"
        Display: f"Error: {e}"
        RETURN { "valid": false, "error": "malformed_yaml", "offer_fresh_start": true }

    # Step 3: Validate required fields
    REQUIRED_FIELDS = ["session_id", "timestamp", "current_phase", "brainstorm_context"]
    missing_fields = []

    FOR field in REQUIRED_FIELDS:
        IF field not in parsed:
            missing_fields.append(field)

    IF missing_fields:
        Display: f"⚠️ Warning: Checkpoint missing required fields: {missing_fields}"
        RETURN { "valid": false, "error": "missing_fields", "missing": missing_fields, "offer_fresh_start": true }

    # Step 4: Validate field types
    IF not isinstance(parsed["current_phase"], int):
        RETURN { "valid": false, "error": "invalid_type", "field": "current_phase", "offer_fresh_start": true }

    IF parsed["current_phase"] < 1 or parsed["current_phase"] > 6:
        RETURN { "valid": false, "error": "invalid_phase", "offer_fresh_start": true }

    # Step 5: Return validated checkpoint
    RETURN { "valid": true, "data": parsed }
```

### Graceful Failure Handling

```
IF load_result["valid"] == false:
    Display: "⚠️ Checkpoint validation failed"
    Display: f"Reason: {load_result['error']}"

    IF load_result.get("offer_fresh_start"):
        AskUserQuestion(
            questions=[{
                "question": "Checkpoint is invalid. Start a fresh session?",
                "header": "Invalid",
                "options": [
                    {"label": "Start fresh", "description": "Begin a new ideation session"},
                    {"label": "Try another checkpoint", "description": "Select a different checkpoint file"}
                ],
                "multiSelect": false
            }]
        )
```

---

## Phase Replay Engine

### Display Previous Answers

```
FUNCTION display_phase_answers(checkpoint_data, phase_number):
    context = checkpoint_data["brainstorm_context"]

    SWITCH phase_number:
        CASE 1:
            Display: "## Phase 1: Problem Statement"
            Display: f"**Previous Answer:** {context.get('problem_statement', 'N/A')}"

        CASE 2:
            Display: "## Phase 2: Personas"
            FOR persona in context.get('personas', []):
                Display: f"- {persona.get('name')}: {persona.get('description')}"

        CASE 3:
            Display: "## Phase 3: Requirements"
            FOR req in context.get('requirements', []):
                Display: f"- {req}"

        CASE 4:
            Display: "## Phase 4: Complexity Assessment"
            Display: f"**Score:** {context.get('complexity_score', 'N/A')}/60"

        CASE 5:
            Display: "## Phase 5: Epic Generation"
            FOR epic in context.get('epics', []):
                Display: f"- {epic.get('title')}"
```

### Keep or Update Choice

```
FUNCTION ask_keep_or_update(phase_number):
    AskUserQuestion(
        questions=[{
            "question": f"Keep Phase {phase_number} answers or update them?",
            "header": "Phase Data",
            "options": [
                {
                    "label": "Keep these answers",
                    "description": "Use saved data and skip to next phase"
                },
                {
                    "label": "Update answers",
                    "description": "Re-enter answers for this phase"
                }
            ],
            "multiSelect": false
        }]
    )

    IF user_choice == "Keep these answers":
        RETURN "keep"
    ELSE:
        RETURN "update"
```

---

## Resume from Last Incomplete Phase

### Resume Orchestration

```
FUNCTION resume_from_checkpoint(checkpoint_data):
    session_id = checkpoint_data["session_id"]
    current_phase = checkpoint_data["current_phase"]
    phase_completed = checkpoint_data.get("phase_completed", false)
    brainstorm_context = checkpoint_data["brainstorm_context"]

    # Determine resume point
    IF phase_completed:
        resume_phase = current_phase + 1
    ELSE:
        resume_phase = current_phase

    Display: f"📋 Resuming session {session_id}"
    Display: f"   Last completed phase: {current_phase}"
    Display: f"   Resuming from phase: {resume_phase}"

    # Replay completed phases (optional review)
    FOR phase in range(1, resume_phase):
        display_phase_answers(checkpoint_data, phase)
        choice = ask_keep_or_update(phase)

        IF choice == "update":
            # Re-execute phase with fresh user input
            EXECUTE_PHASE(phase)
            # Update checkpoint after re-execution
            UPDATE_CHECKPOINT(session_id, phase, new_context)

    # Continue from resume_phase
    RETURN {
        "session_id": session_id,
        "resume_phase": resume_phase,
        "brainstorm_context": brainstorm_context,
        "personas": brainstorm_context.get("personas", []),
        "requirements": brainstorm_context.get("requirements", [])
    }
```

---

## Multi-Checkpoint Selection

### MultiCheckpointSelector

```
FUNCTION select_from_multiple_checkpoints(checkpoint_metadata):
    # checkpoint_metadata is already sorted (newest first)

    Display: "📚 Multiple Ideation Sessions Found"
    Display: ""

    FOR i, checkpoint in enumerate(checkpoint_metadata):
        Display: f"**Session {i+1}:**"
        Display: f"  - Timestamp: {checkpoint['timestamp']}"
        Display: f"  - Phases Complete: {checkpoint['current_phase']}/6"
        Display: f"  - Problem: {checkpoint['problem_preview']}..."
        Display: ""

    # Build options for AskUserQuestion
    options = []
    FOR i, checkpoint in enumerate(checkpoint_metadata):
        options.append({
            "label": f"Session {i+1} ({checkpoint['current_phase']}/6 phases)",
            "description": f"{checkpoint['timestamp']}: {checkpoint['problem_preview']}..."
        })

    options.append({
        "label": "Start fresh",
        "description": "Discard all checkpoints and begin new session"
    })

    AskUserQuestion(
        questions=[{
            "question": "Which session would you like to resume?",
            "header": "Select",
            "options": options,
            "multiSelect": false
        }]
    )

    IF user_choice == "Start fresh":
        RETURN { "action": "fresh_start" }
    ELSE:
        selected_index = parse_selection(user_choice)  # Extract session number
        RETURN {
            "action": "resume",
            "checkpoint_path": checkpoint_metadata[selected_index]["path"]
        }
```

---

## Business Rules

| Rule | Description |
|------|-------------|
| BR-001 | Use Glob tool for checkpoint detection (NOT Bash) |
| BR-002 | Use Read tool for checkpoint loading (NOT Bash) |
| BR-003 | Detection MUST occur before any user prompts in Phase 1 |
| BR-004 | User MUST be given choice to resume or start fresh (no auto-resume) |
| BR-005 | Invalid checkpoints MUST NOT crash session - offer fresh start |
| BR-006 | Phase replay MUST allow user to update answers (not forced to keep) |
| BR-007 | Multiple checkpoints sorted by timestamp (newest first) |

---

## Edge Cases

| Scenario | Handling |
|----------|----------|
| Checkpoint deleted between detection and load | Graceful failure, offer fresh start |
| Checkpoint from different framework version | Version check, warn if incompatible |
| Very old checkpoint (>30 days) | Warn user, offer to discard |
| Checkpoint with partial phase data | Resume from last fully completed phase |
| User cancels resume selection | Return to fresh session without error |
| Concurrent session creates checkpoint | Include in selection list |

---

## Integration with SKILL.md

Add to Phase 1 of devforgeai-ideation SKILL.md:

```markdown
## Phase 1: Discovery

### Step 0: Checkpoint Resume Check (BEFORE user interaction)

See: references/resume-logic.md

1. Detect checkpoints using Glob
2. If found, present resume choice
3. If resume selected, load and validate checkpoint
4. Continue from appropriate phase

### Step 1: Problem Statement (original Phase 1)
...
```

---

## Schema Reference

See: `checkpoint-schema.yaml` for complete checkpoint YAML schema.
See: `checkpoint-protocol.md` for checkpoint creation/update logic.
