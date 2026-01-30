# Phase 0: Checkpoint Detection and Recovery

Orchestration workflows can be interrupted and resumed. This phase detects existing checkpoints and resumes from the last successful stage.

## Purpose

When the `/orchestrate` command is run, this phase checks for:
1. Existing checkpoints (DEV_COMPLETE, QA_APPROVED, STAGING_COMPLETE, PRODUCTION_COMPLETE)
2. Current story status
3. Last successful phase

**Key benefit:** Enables resume functionality - workflows don't restart from scratch after interruption.

**This phase moved FROM /orchestrate command TO skill** for proper orchestration layer separation (lean orchestration pattern).

---

## Checkpoint Types

### PRODUCTION_COMPLETE

**Meaning:** Story successfully deployed to production

**Created by:** devforgeai-release skill (Phase 5: Production Release)

**Marker in workflow history:**
```
Checkpoint: PRODUCTION_COMPLETE
Timestamp: {ISO 8601}
```

**Action when detected:**
- `checkpoint_status = "PRODUCTION_COMPLETE"`
- `orchestration_result = "ALREADY_RELEASED"`
- **HALT**: "Story ${STORY_ID} already released (status: Released)"
- Return early with completion status (no further phases needed)

---

### STAGING_COMPLETE

**Meaning:** Story successfully deployed to staging environment

**Created by:** devforgeai-release skill (Phase 4: Staging Release)

**Marker in workflow history:**
```
Checkpoint: STAGING_COMPLETE
Timestamp: {ISO 8601}
```

**Action when detected:**
- `checkpoint_status = "STAGING_COMPLETE"`
- `starting_phase = 5` (Phase 5: Production Release)
- `skip_phases = [1, 2, 3, 4]`
- `resume_message = "Resuming from Staging Complete checkpoint"`

**Resume workflow:**
- Skip Development (Phase 2)
- Skip QA Validation (Phase 3)
- Skip Staging Release (Phase 4)
- Execute Production Release (Phase 5)

---

### QA_APPROVED

**Meaning:** Story passed QA validation, ready for deployment

**Created by:** devforgeai-qa skill (Deep validation passed)

**Marker in workflow history:**
```
Checkpoint: QA_APPROVED
Timestamp: {ISO 8601}
```

**Action when detected:**
- `checkpoint_status = "QA_APPROVED"`
- `starting_phase = 4` (Phase 4: Staging Release)
- `skip_phases = [1, 2, 3]`
- `resume_message = "Resuming from QA Approved checkpoint"`

**Resume workflow:**
- Skip Development (Phase 2)
- Skip QA Validation (Phase 3)
- Execute Staging Release (Phase 4)
- Execute Production Release (Phase 5)

**Precedence:** If both QA_APPROVED and STAGING_COMPLETE exist, STAGING_COMPLETE takes priority (later in workflow).

---

### DEV_COMPLETE

**Meaning:** Development completed, implementation finished

**Created by:** devforgeai-development skill (TDD cycle complete)

**Marker in workflow history:**
```
Checkpoint: DEV_COMPLETE
Timestamp: {ISO 8601}
```

**Action when detected:**
- `checkpoint_status = "DEV_COMPLETE"`
- `starting_phase = 3` (Phase 3: QA Validation)
- `skip_phases = [1, 2]`
- `resume_message = "Resuming from Dev Complete checkpoint"`

**Resume workflow:**
- Skip Development (Phase 2)
- Execute QA Validation (Phase 3)
- Execute Staging Release (Phase 4)
- Execute Production Release (Phase 5)

**Precedence:** If DEV_COMPLETE and later checkpoints (QA_APPROVED, STAGING_COMPLETE) exist, later checkpoints take priority.

---

## Detection Logic

### Step 1: Load Story Document

```
Read(file_path="devforgeai/specs/Stories/{story_id}.story.md")

Extract YAML frontmatter:
- id, title, epic, sprint
- status (current workflow state)
- points, priority, assigned_to
- created_date, completed_date (if exists)

Extract workflow history section:
- All previous phase results
- Checkpoint entries (if any)
- QA attempt history
- Orchestration timeline (if previous run)
```

**Purpose:** Load complete story context for checkpoint detection.

---

### Step 2: Detect Orchestration Checkpoints

**Search sequence (reverse chronological - latest first):**

```
# Check 1: PRODUCTION_COMPLETE (highest priority)
Grep(pattern="Checkpoint: PRODUCTION_COMPLETE", file=story_workflow_history)
IF found:
  checkpoint_status = "PRODUCTION_COMPLETE"
  orchestration_result = "ALREADY_RELEASED"
  HALT: "Story ${STORY_ID} already released (status: Released)"
  Return early with completion status

# Check 2: STAGING_COMPLETE
Grep(pattern="Checkpoint: STAGING_COMPLETE", file=story_workflow_history)
IF found:
  checkpoint_status = "STAGING_COMPLETE"
  starting_phase = 5  # Phase 5: Production Release
  skip_phases = [1, 2, 3, 4]
  resume_message = "Resuming from Staging Complete checkpoint"

# Check 3: QA_APPROVED
Grep(pattern="Checkpoint: QA_APPROVED", file=story_workflow_history)
IF found AND checkpoint_status NOT "STAGING_COMPLETE":
  checkpoint_status = "QA_APPROVED"
  starting_phase = 4  # Phase 4: Staging Release
  skip_phases = [1, 2, 3]
  resume_message = "Resuming from QA Approved checkpoint"

# Check 4: DEV_COMPLETE
Grep(pattern="Checkpoint: DEV_COMPLETE", file=story_workflow_history)
IF found AND checkpoint_status NOT IN ["QA_APPROVED", "STAGING_COMPLETE"]:
  checkpoint_status = "DEV_COMPLETE"
  starting_phase = 3  # Phase 3: QA Validation
  skip_phases = [1, 2]
  resume_message = "Resuming from Dev Complete checkpoint"

# Check 5: No checkpoints
IF no checkpoints found:
  checkpoint_status = "NONE"
  starting_phase = 2  # Phase 2: Development
  skip_phases = []
  resume_message = "Starting full orchestration from development"
```

**Priority order:** PRODUCTION_COMPLETE > STAGING_COMPLETE > QA_APPROVED > DEV_COMPLETE > NONE

---

### Step 3: Validate Current Story State

**Determines if orchestration can proceed based on current story status.**

#### Valid Starting States (Orchestration Allowed)

**Status: "Backlog"**
```
Action: Start from Phase 2 (Development)
Message: "Story in Backlog, beginning development phase"
Checkpoints: Ignored (no dev work done yet)
```

**Status: "Ready for Dev"**
```
Action: Start from Phase 2 (Development)
Message: "Story ready for dev, beginning development phase"
Checkpoints: Ignored (no dev work done yet)
```

**Status: "Dev Complete"**
```
Action: Start from Phase 3 (QA Validation)
Message: "Development complete, beginning QA validation"
Checkpoints: Use DEV_COMPLETE checkpoint if exists (takes precedence)
```

**Status: "QA Failed"**
```
Action: Restart from Phase 2 (Development)
Message: "QA failed previously, restarting from development"
Reason: QA failure requires fixing implementation
Checkpoints: Ignored (must redo dev work)
```

**Status: "QA Approved"**
```
Action: Start from Phase 4 (Staging Release)
Message: "QA approved, beginning staging deployment"
Checkpoints: Use QA_APPROVED checkpoint if exists (takes precedence)
```

---

#### Blocked States (Orchestration Cannot Proceed)

**Status: "In Development"**
```
HALT: "Story in In Development status. Manual development in progress.
       Cannot auto-orchestrate while dev active.

       Options:
       - Complete /dev manually, then run /orchestrate
       - Cancel /dev, then run /orchestrate (will restart dev)"

Return: "ORCHESTRATION_BLOCKED_DEV_IN_PROGRESS"
```

**Reason:** Manual /dev command currently executing. Auto-orchestration would conflict with manual workflow.

---

**Status: "QA In Progress"**
```
HALT: "Story in QA In Progress status. Manual QA validation running.
       Cannot auto-orchestrate while QA active.

       Options:
       - Complete /qa manually, then run /orchestrate
       - Cancel /qa, then run /orchestrate (will restart QA)"

Return: "ORCHESTRATION_BLOCKED_QA_IN_PROGRESS"
```

**Reason:** Manual /qa command currently executing. Auto-orchestration would conflict.

---

**Status: "Releasing"**
```
HALT: "Story in Releasing status. Manual deployment in progress.
       Cannot auto-orchestrate while release active.

       Options:
       - Complete /release manually
       - Wait for deployment to finish"

Return: "ORCHESTRATION_BLOCKED_RELEASE_IN_PROGRESS"
```

**Reason:** Manual /release command currently executing. Auto-orchestration would conflict.

---

#### Complete States (Orchestration Not Needed)

**Status: "Released"**
```
MESSAGE: "Story ${STORY_ID} already released. No orchestration needed.
          Deployment complete: {completed_date}
          Monitor production metrics."

Return: "ALREADY_RELEASED"
```

**Reason:** Story complete. Nothing to orchestrate.

---

### Step 4: Determine Final Starting Phase

**Reconcile checkpoint vs status:**

```
IF checkpoint_status != "NONE":
  # Checkpoint takes precedence over status
  final_starting_phase = starting_phase from checkpoint

  Display:
  "📍 Checkpoint Detected: {checkpoint_status}
   Resuming orchestration from Phase {final_starting_phase}
   Skipping phases: {skip_phases}
   {resume_message}"

ELSE:
  # No checkpoint, use status-based determination
  final_starting_phase = determined from status validation (Step 3)

  Display:
  "🚀 Starting Orchestration: {STORY_ID}
   Beginning from Phase {final_starting_phase}
   Story status: {current_status}"

Set orchestration context:
  orchestration_state = {
    "story_id": story_id,
    "starting_phase": final_starting_phase,
    "skip_phases": skip_phases,
    "checkpoint": checkpoint_status,
    "story_status": current_status,
    "resume_mode": checkpoint_status != "NONE"
  }

Return: orchestration_state to workflow controller
```

---

## Resume Strategy

### Full Workflow (No Checkpoints)

**Phases executed:** 2 → 3 → 4 → 5
1. Development (devforgeai-development)
2. QA Validation (devforgeai-qa)
3. Staging Release (devforgeai-release --env=staging)
4. Production Release (devforgeai-release --env=production)

---

### Resume from DEV_COMPLETE

**Phases executed:** 3 → 4 → 5
1. ~~Development~~ (skipped)
2. QA Validation (devforgeai-qa)
3. Staging Release (devforgeai-release --env=staging)
4. Production Release (devforgeai-release --env=production)

**Use case:** Development completed, but QA/deployment interrupted.

---

### Resume from QA_APPROVED

**Phases executed:** 4 → 5
1. ~~Development~~ (skipped)
2. ~~QA Validation~~ (skipped)
3. Staging Release (devforgeai-release --env=staging)
4. Production Release (devforgeai-release --env=production)

**Use case:** QA passed, but deployment interrupted.

---

### Resume from STAGING_COMPLETE

**Phases executed:** 5
1. ~~Development~~ (skipped)
2. ~~QA Validation~~ (skipped)
3. ~~Staging Release~~ (skipped)
4. Production Release (devforgeai-release --env=production)

**Use case:** Staging deployed successfully, only production deployment remaining.

---

## Output

**Orchestration State Object:**
```json
{
  "story_id": "STORY-042",
  "starting_phase": 3,
  "skip_phases": [1, 2],
  "checkpoint": "DEV_COMPLETE",
  "story_status": "Dev Complete",
  "resume_mode": true
}
```

**This state** is passed to the workflow controller (orchestration skill main logic) to determine which phases to execute.

---

## Error Handling

### Missing Story File

**Error:** Story file not found at `devforgeai/specs/Stories/{story_id}.story.md`

**Action:**
```
HALT: "Story ${story_id} not found.
       Expected location: devforgeai/specs/Stories/${story_id}.story.md

       Options:
       - Verify story ID spelling
       - Use /create-story to create story first"

Return: "STORY_NOT_FOUND"
```

---

### Malformed Workflow History

**Error:** Story file exists but workflow history section missing or corrupted

**Action:**
```
WARNING: "Workflow history section missing or malformed in ${story_id}.
          Cannot detect checkpoints. Starting from beginning."

checkpoint_status = "NONE"
starting_phase = 2
Proceed with full orchestration
```

**Reason:** Non-fatal error. Story may be newly created without history yet.

---

### Conflicting Status and Checkpoint

**Error:** Checkpoint says DEV_COMPLETE but status is "Backlog"

**Action:**
```
WARNING: "Checkpoint/status mismatch detected.
          Checkpoint: ${checkpoint_status}
          Status: ${current_status}

          Using checkpoint (higher reliability).
          Story status may need manual correction."

Use checkpoint to determine starting phase
Log inconsistency for review
```

**Reason:** Trust checkpoint over status (checkpoints are immutable, status can be manually changed).

---

## Related Files

- **story-validation.md** - Phase 1 story file validation
- **skill-invocation.md** - Phase 2 skill coordination
- **orchestration-finalization.md** - Phase 6 creates checkpoints
- **workflow-states.md** - Complete state definitions
- **troubleshooting.md** - Common checkpoint issues
