---
id: session-checkpoint-workflow
title: Session Checkpoint and Resume Workflow
version: "1.0"
created: 2025-12-21
status: Published
---

# Session Checkpoint and Resume Workflow

Handle session continuity through checkpoint generation, context monitoring, and resume capabilities.

## Overview

| Attribute | Value |
|-----------|-------|
| **Purpose** | Enable brainstorm sessions to survive context window limits |
| **Trigger** | Context usage > 70% OR user requests checkpoint |
| **Output** | BRAINSTORM-{NNN}.checkpoint.json |
| **Resume** | /brainstorm --resume BRAINSTORM-{NNN} |

---

## Section 1: Context Window Monitoring

### 1.1 When to Check

Check context usage at these phase transitions:
- After Phase 1 (Stakeholder Discovery)
- After Phase 3 (Opportunity Mapping) - Often heaviest with research
- After Phase 5 (Hypothesis Formation)
- Before Phase 7 (Synthesis) - Ensure room for final output

### 1.2 Threshold Levels

| Usage Level | Action |
|-------------|--------|
| < 70% | Continue normally |
| 70-85% | Offer optional checkpoint |
| > 85% | Mandatory checkpoint |

### 1.3 Context Check Logic

```
FUNCTION check_context_window():
  # Estimate context usage (approximation)
  estimated_tokens = count_session_tokens()
  threshold_70 = 0.70 * MAX_CONTEXT_TOKENS
  threshold_85 = 0.85 * MAX_CONTEXT_TOKENS

  IF estimated_tokens > threshold_85:
    RETURN "mandatory_checkpoint"
  ELSE IF estimated_tokens > threshold_70:
    RETURN "optional_checkpoint"
  ELSE:
    RETURN "continue"
```

---

## Section 2: Checkpoint Prompt

### 2.1 Optional Checkpoint (70%)

```
AskUserQuestion:
  questions:
    - question: "Context window is approximately {PERCENT}% full. Would you like to:"
      header: "Session"
      multiSelect: false
      options:
        - label: "Continue in this session"
          description: "Proceed to next phase"
        - label: "Save and continue later"
          description: "Create checkpoint and exit"
```

### 2.2 Mandatory Checkpoint (85%)

```
Display:
"━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Context Limit Approaching
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

The conversation is reaching memory limits.
Your progress will be saved automatically.

To continue in a new session:
  /brainstorm --resume {BRAINSTORM_ID}

Saving checkpoint now..."

# Generate checkpoint without asking
generate_checkpoint()
```

---

## Section 3: Checkpoint JSON Schema

### 3.1 Full Schema

```json
{
  "checkpoint_version": "1.0",
  "brainstorm_id": "BRAINSTORM-001",
  "session_number": 1,
  "created_at": "2025-12-21T10:30:00Z",
  "status": "in_progress",

  "progress": {
    "current_phase": 4,
    "phases_completed": [1, 2, 3],
    "phases_remaining": [4, 5, 6, 7],
    "completion_percentage": 43
  },

  "context_state": {
    "estimated_tokens_used": 45000,
    "estimated_tokens_remaining": 35000,
    "reason_for_checkpoint": "user_requested"
  },

  "work_in_flight": {
    "phase_in_progress": 4,
    "phase_name": "Constraint Discovery",
    "questions_asked": 3,
    "questions_remaining": 5,
    "partial_responses": [
      {
        "question": "What is the budget range?",
        "answer": "$50K-$100K",
        "timestamp": "2025-12-21T10:25:00Z"
      }
    ],
    "warning": "Phase 4 incomplete. Resume from question 4."
  },

  "completed_outputs": {
    "topic": {
      "value": "Order processing automation",
      "type": "problem"
    },
    "stakeholder_map": {
      "status": "complete",
      "primary_count": 3,
      "secondary_count": 5,
      "conflicts_count": 1
    },
    "problem_statement": {
      "status": "complete",
      "value": "Operations team spends 45 minutes per order..."
    },
    "root_causes": {
      "status": "complete",
      "depth": 5,
      "root_cause": "No central IT strategy"
    },
    "current_state": {
      "status": "complete",
      "type": "manual",
      "duration": "hours",
      "error_rate": "15-30%"
    },
    "pain_points": {
      "status": "complete",
      "count": 4,
      "critical_count": 1
    },
    "opportunities": {
      "status": "complete",
      "count": 8,
      "research_conducted": true
    },
    "constraints": {
      "status": "partial",
      "budget": "complete",
      "timeline": "complete",
      "resources": "pending",
      "technical": "pending",
      "organizational": "pending"
    }
  },

  "resume_instructions": {
    "for_claude": [
      "1. Read BRAINSTORM-001 partial document for context",
      "2. Resume Phase 4 (Constraint Discovery) from question 4",
      "3. Work in flight: Budget and timeline captured",
      "4. Remaining: Resources, technical, organizational constraints",
      "5. User confirmed budget $50K-$100K before session ended"
    ],
    "for_user": [
      "Run: /brainstorm --resume BRAINSTORM-001",
      "Session will continue from Constraint Discovery phase",
      "Your previous answers have been saved"
    ]
  },

  "error_log": []
}
```

### 3.2 Status Values

| Field | Values |
|-------|--------|
| `status` | "in_progress", "complete", "error" |
| `reason_for_checkpoint` | "user_requested", "context_limit", "auto_save", "error" |
| `completed_outputs.*.status` | "complete", "partial", "pending" |

---

## Section 4: Checkpoint Generation

### 4.1 Generation Logic

```
FUNCTION generate_checkpoint():
  checkpoint = {
    checkpoint_version: "1.0",
    brainstorm_id: session.brainstorm_id,
    session_number: session.session_number + 1,
    created_at: now(),
    status: "in_progress"
  }

  # Calculate progress
  phases_completed = get_completed_phases()
  checkpoint.progress = {
    current_phase: session.current_phase,
    phases_completed: phases_completed,
    phases_remaining: [p for p in [1,2,3,4,5,6,7] if p not in phases_completed],
    completion_percentage: (len(phases_completed) / 7) * 100
  }

  # Capture work in flight
  checkpoint.work_in_flight = {
    phase_in_progress: session.current_phase,
    phase_name: get_phase_name(session.current_phase),
    questions_asked: session.questions_asked_in_phase,
    questions_remaining: estimate_remaining_questions(),
    partial_responses: session.partial_responses
  }

  # Capture completed outputs
  checkpoint.completed_outputs = serialize_session_data()

  # Generate resume instructions
  checkpoint.resume_instructions = generate_resume_instructions()

  RETURN checkpoint
```

### 4.2 Write Checkpoint

```
FUNCTION save_checkpoint(checkpoint):
  path = f"devforgeai/specs/brainstorms/{checkpoint.brainstorm_id}.checkpoint.json"

  TRY:
    Write(file_path=path, content=json.stringify(checkpoint, indent=2))
    session.checkpoint_file = path
    RETURN true
  EXCEPT:
    # Log error but don't fail session
    checkpoint.error_log.append({
      timestamp: now(),
      error: "Failed to write checkpoint",
      path: path
    })
    RETURN false
```

---

## Section 5: Resume Detection

### 5.1 Command Parsing

```
FUNCTION parse_resume_command(args):
  IF args contains "--resume":
    resume_id = args.after("--resume")
    IF resume_id is valid format (BRAINSTORM-\d{3}):
      RETURN {mode: "resume", brainstorm_id: resume_id}
    ELSE:
      RETURN {mode: "error", message: "Invalid brainstorm ID format"}
  ELSE:
    RETURN {mode: "new"}
```

### 5.2 Resume Flow

```
FUNCTION resume_session(brainstorm_id):
  checkpoint_path = f"devforgeai/specs/brainstorms/{brainstorm_id}.checkpoint.json"

  IF NOT exists(checkpoint_path):
    # Check for completed document
    doc_path = f"devforgeai/specs/brainstorms/{brainstorm_id}*.brainstorm.md"
    IF exists(doc_path):
      Display: "This brainstorm is complete. Use /ideate instead."
      RETURN null
    ELSE:
      Display: "No checkpoint or document found for {brainstorm_id}"
      list_available_brainstorms()
      RETURN null

  # Load checkpoint
  TRY:
    checkpoint = json.parse(Read(file_path=checkpoint_path))
  EXCEPT JSONDecodeError:
    Display: "Checkpoint file is corrupted."
    offer_recovery_options()
    RETURN null

  # Validate checkpoint
  IF checkpoint.checkpoint_version != "1.0":
    Display: "Checkpoint version incompatible."
    RETURN null

  # Restore session
  session = restore_session_from_checkpoint(checkpoint)

  # Display resume info
  Display:
  "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Session Restored
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Brainstorm: {brainstorm_id}
  Progress: {completion_percentage}% complete

  Completed Phases:
    {list completed phases with ✓}

  Resuming: Phase {current_phase} ({phase_name})
  "

  RETURN session
```

### 5.3 Restore Session Data

```
FUNCTION restore_session_from_checkpoint(checkpoint):
  session = new Session()

  session.brainstorm_id = checkpoint.brainstorm_id
  session.session_number = checkpoint.session_number
  session.current_phase = checkpoint.progress.current_phase

  # Restore completed outputs
  FOR output_name, output_data in checkpoint.completed_outputs:
    IF output_data.status == "complete":
      session[output_name] = output_data.value

  # Restore partial work
  session.partial_responses = checkpoint.work_in_flight.partial_responses
  session.questions_asked_in_phase = checkpoint.work_in_flight.questions_asked

  RETURN session
```

---

## Section 6: Resume Instructions Generation

### 6.1 For Claude (New Session)

```
FUNCTION generate_claude_instructions(checkpoint):
  instructions = []

  # Basic context
  instructions.append(f"Brainstorm ID: {checkpoint.brainstorm_id}")
  instructions.append(f"Resume from Phase {checkpoint.progress.current_phase}")

  # What's completed
  FOR output in checkpoint.completed_outputs WHERE output.status == "complete":
    instructions.append(f"✓ {output_name}: {output.summary}")

  # What's in flight
  IF checkpoint.work_in_flight:
    instructions.append(f"⚠ Phase {checkpoint.work_in_flight.phase_in_progress} incomplete")
    instructions.append(f"  Questions asked: {checkpoint.work_in_flight.questions_asked}")
    FOR response in checkpoint.work_in_flight.partial_responses:
      instructions.append(f"  - {response.question}: {response.answer}")

  # What's remaining
  instructions.append("Remaining phases:")
  FOR phase in checkpoint.progress.phases_remaining:
    instructions.append(f"  - Phase {phase}: {get_phase_name(phase)}")

  RETURN instructions
```

### 6.2 For User

```
FUNCTION generate_user_instructions(checkpoint):
  instructions = [
    f"Run: /brainstorm --resume {checkpoint.brainstorm_id}",
    f"Progress: {checkpoint.progress.completion_percentage}% complete",
    f"Resume from: Phase {checkpoint.progress.current_phase} ({get_phase_name(checkpoint.progress.current_phase)})",
    "Your previous answers have been saved"
  ]
  RETURN instructions
```

---

## Section 7: Checkpoint Display

### 7.1 Checkpoint Created

```
Display:
"━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Brainstorm Session Checkpointed
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Progress: {completion_percentage}% complete (Phases 1-{last_complete} of 7)
Checkpoint: devforgeai/specs/brainstorms/{brainstorm_id}.checkpoint.json

Completed:
  ✓ Phase 1: Stakeholder Discovery
  ✓ Phase 2: Problem Exploration
  ✓ Phase 3: Opportunity Mapping

Remaining:
  ○ Phase 4: Constraint Discovery (in progress)
  ○ Phase 5: Hypothesis Formation
  ○ Phase 6: Prioritization
  ○ Phase 7: Handoff Synthesis

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Resume Instructions
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

In your next Claude session, run:
  /brainstorm --resume {brainstorm_id}

Your progress has been saved. The session will continue
from Phase 4 (Constraint Discovery).
"
```

---

## Section 8: Error Recovery

### 8.1 Checkpoint Not Found

```
FUNCTION handle_checkpoint_not_found(brainstorm_id):
  Display: "Checkpoint not found for {brainstorm_id}"

  # List available options
  available = list_available_checkpoints()
  completed = list_completed_brainstorms()

  Display:
  "Available checkpoints:
    {list checkpoints with progress}

  Completed brainstorms (use /ideate instead):
    {list completed}

  Options:
    1. Resume one of the available checkpoints
    2. Start a new brainstorm: /brainstorm
    3. Use completed brainstorm with /ideate"
```

### 8.2 Corrupted Checkpoint

```
FUNCTION handle_corrupted_checkpoint(brainstorm_id):
  # Check for partial document
  doc_path = find_partial_document(brainstorm_id)

  IF doc_path exists:
    sections = analyze_document_sections(doc_path)

    Display:
    "Checkpoint corrupted, but found partial document:
      {doc_path}

    Sections present: {list complete sections}
    Sections incomplete: {list incomplete sections}

    Options:
      1. Continue from last complete section
      2. Start fresh brainstorm
      3. View existing document"

    AskUserQuestion:
      questions:
        - question: "How would you like to proceed?"
          header: "Recovery"
          options:
            - label: "Continue from {last_complete}"
              description: "Resume from Phase {n}"
            - label: "Start fresh"
              description: "New brainstorm session"
  ELSE:
    Display: "No recovery data found. Starting fresh."
```

---

## Common Issues and Recovery

| Issue | Symptom | Recovery |
|-------|---------|----------|
| Checkpoint write fails | Permission denied | Save to alternate location |
| Resume ID not found | No checkpoint file | List available, offer fresh start |
| Checkpoint corrupted | JSON parse error | Check for partial doc, offer recovery |
| Version mismatch | Old checkpoint format | Attempt migration or fresh start |
| Session data lost | Incomplete restore | Note gaps, continue from safe point |

---

## Success Criteria

- [ ] Context window monitored at phase transitions
- [ ] Checkpoint offered at 70% threshold
- [ ] Checkpoint mandatory at 85% threshold
- [ ] Checkpoint file generated with all required fields
- [ ] Resume instructions generated for user and Claude
- [ ] Resume flow restores session correctly
- [ ] Partial work preserved and continued
- [ ] Error recovery handles common failures

---

**Version:** 1.0 | **Status:** Published | **Created:** 2025-12-21
