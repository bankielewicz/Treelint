---
description: Transform business problems into structured brainstorm sessions
argument-hint: [optional-topic] | --resume BRAINSTORM-ID
model: opus
allowed-tools: Read, Write, Edit, Glob, Skill, AskUserQuestion, TodoWrite, Task
---

# /brainstorm - Business Analyst Discovery Session

Transform vague business problems into structured, AI-consumable brainstorm documents
that feed into `/ideate` for formal requirements.

---

## Quick Reference

```bash
# Start new brainstorm
/brainstorm

# Start with topic
/brainstorm "improve customer onboarding"

# Resume previous session
/brainstorm --resume BRAINSTORM-001
```

---

## Command Workflow

### Phase 0: Argument Parsing

```
IF $1 == "--resume":
  BRAINSTORM_ID = $2
  MODE = "resume"
ELSE IF $1 is provided:
  TOPIC = $1
  MODE = "new"
ELSE:
  MODE = "new"
  TOPIC = null (will ask)
```

### Phase 1: Resume Check (if --resume)

```
IF MODE == "resume":
  checkpoint_file = "devforgeai/specs/brainstorms/${BRAINSTORM_ID}.checkpoint.json"

  IF checkpoint_file exists:
    Read(file_path=checkpoint_file)
    Display: "Resuming ${BRAINSTORM_ID} from Phase ${checkpoint.progress.current_phase}"
  ELSE:
    Display: "No checkpoint found for ${BRAINSTORM_ID}"
    AskUserQuestion: "Start fresh or check for brainstorm document?"
```

### Phase 2: Invoke Skill

```
Display:
"━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  DevForgeAI Brainstorming Session
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Mode: ${MODE}
Topic: ${TOPIC || 'To be discovered'}

Delegating to devforgeai-brainstorming skill..."

Skill(command="devforgeai-brainstorming")
```

### Phase 3: Display Results

```
# Skill returns structured summary
Display skill output (complete or checkpoint)
```

---

## Error Handling

### No Topic Provided
Skill will ask for topic in Phase 1.

### Resume Failed
```
ERROR: Cannot resume BRAINSTORM-${ID}

Possible causes:
- Checkpoint file not found
- Brainstorm was completed (use /ideate instead)

Options:
- /brainstorm (start new)
- /ideate (if brainstorm is complete)
```

---

## Integration

**Invokes:** devforgeai-brainstorming skill
**Outputs to:** devforgeai/specs/brainstorms/
**Feeds into:** /ideate command
