---
id: error-handling
title: Error Handling and Recovery Procedures for Brainstorming Sessions
version: "1.0"
created: 2025-12-21
status: Published
audience: devforgeai-brainstorming skill (Internal Use)
---

# Error Handling and Recovery Procedures

**Purpose:** Define error scenarios, recovery procedures, and graceful degradation patterns for the brainstorming skill. Ensures session continuity and data preservation when issues occur.

**Quick Links:**
- [Section 1: Error Categories](#section-1-error-categories)
- [Section 2: Recovery Procedures](#section-2-recovery-procedures)
- [Section 3: Graceful Degradation](#section-3-graceful-degradation)
- [Section 4: User Communication Templates](#section-4-user-communication-templates)

---

## Section 1: Error Categories

### 1.1 Error Severity Levels

| Level | Description | Action | Example |
|-------|-------------|--------|---------|
| **CRITICAL** | Session cannot continue | Save state, exit gracefully | File system unavailable |
| **HIGH** | Feature unavailable, workaround exists | Use fallback, notify user | Research subagent timeout |
| **MEDIUM** | Minor issue, can retry | Retry operation, log issue | File write retry |
| **LOW** | Cosmetic or informational | Log and continue | Display formatting issue |

### 1.2 Error Types by Category

#### Session Errors

| Error | Cause | Severity | Recovery |
|-------|-------|----------|----------|
| Session abandoned | User closed terminal | HIGH | Auto-checkpoint on inactivity |
| Context window full | Too much conversation | HIGH | Offer checkpoint, continue in new session |
| Resume ID not found | Invalid --resume argument | MEDIUM | List available brainstorms |
| Checkpoint corrupted | Invalid JSON in checkpoint | MEDIUM | Offer fresh start or manual recovery |

#### File System Errors

| Error | Cause | Severity | Recovery |
|-------|-------|----------|----------|
| Write permission denied | Directory not writable | CRITICAL | Suggest alternative location |
| Directory not found | Output directory missing | MEDIUM | Create directory automatically |
| File already exists | Duplicate brainstorm ID | LOW | Increment ID and retry |
| Disk full | No storage space | CRITICAL | Notify user, no recovery |

#### Subagent Errors

| Error | Cause | Severity | Recovery |
|-------|-------|----------|----------|
| Subagent timeout | Task took too long | HIGH | Skip optional work, continue |
| Subagent failure | Internal error | MEDIUM | Retry once, then skip |
| Research unavailable | Network issues | MEDIUM | Skip research, use internal knowledge |

#### User Input Errors

| Error | Cause | Severity | Recovery |
|-------|-------|----------|----------|
| Invalid response | User input doesn't match options | LOW | Re-ask question |
| Empty response | User skipped required field | MEDIUM | Explain requirement, re-ask |
| Contradictory input | User gave conflicting answers | LOW | Clarify with follow-up |

---

## Section 2: Recovery Procedures

### 2.1 Session Errors

#### ERR-001: Session Abandoned

**Trigger:** No user response for extended period OR terminal closed

**Detection:**
```
IF no_user_response_for > 5_minutes:
  trigger_auto_checkpoint = true
```

**Recovery Procedure:**
1. Generate checkpoint with current progress
2. Save all collected data to checkpoint.json
3. Mark checkpoint.status = "auto_saved"
4. Add note: "Session saved automatically due to inactivity"

**User Communication:**
```
(On resume)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Session Recovered
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Your previous session was automatically saved.

Progress: Phases 1-{N} complete
Last activity: {timestamp}

Resuming from Phase {N+1}...
```

---

#### ERR-002: Context Window Full

**Trigger:** Estimated context usage > 85%

**Detection:**
```
IF estimated_tokens > 0.85 * max_context:
  trigger_mandatory_checkpoint = true
```

**Recovery Procedure:**
1. Warn user at 70% (optional checkpoint)
2. Force checkpoint at 85% (mandatory)
3. Generate complete checkpoint with all progress
4. Display clear resume instructions

**User Communication:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Context Limit Approaching
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

The conversation is reaching memory limits.
Your progress will be saved automatically.

To continue in a new session:
  /brainstorm --resume {BRAINSTORM_ID}

Saving checkpoint now...
```

---

#### ERR-003: Resume ID Not Found

**Trigger:** User runs `/brainstorm --resume BRAINSTORM-XXX` but ID doesn't exist

**Detection:**
```
checkpoint_path = f"devforgeai/specs/brainstorms/{brainstorm_id}.checkpoint.json"
IF NOT exists(checkpoint_path):
  error = "resume_id_not_found"
```

**Recovery Procedure:**
1. Search for available checkpoints
2. Search for completed brainstorm documents
3. Offer alternatives or fresh start

**User Communication:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Checkpoint Not Found
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Could not find checkpoint for: {BRAINSTORM_ID}

Available checkpoints:
  - BRAINSTORM-001 (Phase 3, 43% complete)
  - BRAINSTORM-002 (Phase 5, 71% complete)

Completed brainstorms (use /ideate instead):
  - BRAINSTORM-003 (Complete)

Options:
  1. Resume one of the available checkpoints
  2. Start a new brainstorm: /brainstorm
  3. Use completed brainstorm with /ideate
```

---

#### ERR-004: Checkpoint Corrupted

**Trigger:** Checkpoint file exists but JSON is invalid

**Detection:**
```
TRY:
  checkpoint = json.parse(Read(checkpoint_path))
EXCEPT JSONDecodeError:
  error = "checkpoint_corrupted"
```

**Recovery Procedure:**
1. Check for brainstorm.md file (may have partial content)
2. Offer to recover from document if exists
3. Offer fresh start with warning about lost progress

**User Communication:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Checkpoint Recovery Issue
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

The checkpoint file for {BRAINSTORM_ID} appears corrupted.

Found brainstorm document with partial content:
  devforgeai/specs/brainstorms/{BRAINSTORM_ID}.brainstorm.md
  - Sections present: 1, 2, 3 (Phases 1-3)
  - Sections incomplete: 4, 5, 6, 7

Options:
  1. Continue from Phase 4 (use existing document content)
  2. Start fresh brainstorm
  3. View existing document content
```

---

### 2.2 File System Errors

#### ERR-005: Write Permission Denied

**Trigger:** Cannot write to output directory

**Detection:**
```
TRY:
  Write(file_path=output_path, content=test)
EXCEPT PermissionError:
  error = "write_permission_denied"
```

**Recovery Procedure:**
1. Try alternative locations in order:
   - `devforgeai/specs/brainstorms/`
   - `./brainstorms/`
   - `/tmp/devforgeai/brainstorms/`
2. If all fail, display content for manual save

**User Communication:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Cannot Write to Output Directory
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Cannot write to: devforgeai/specs/brainstorms/

Trying alternative: ./brainstorms/

If this also fails, you can:
  1. Check directory permissions
  2. Create the directory manually
  3. Copy the content displayed below
```

---

#### ERR-006: Directory Not Found

**Trigger:** Output directory doesn't exist

**Detection:**
```
IF NOT exists("devforgeai/specs/brainstorms/"):
  error = "directory_not_found"
```

**Recovery Procedure:**
1. Create directory automatically
2. Create .gitkeep file
3. Continue with normal operation

**User Communication:**
```
Creating output directory: devforgeai/specs/brainstorms/
Directory created successfully. Continuing...
```

---

### 2.3 Subagent Errors

#### ERR-007: Subagent Timeout

**Trigger:** Task() call exceeds timeout threshold

**Detection:**
```
TRY:
  result = Task(subagent_type="stakeholder-analyst", timeout=60000)
EXCEPT TimeoutError:
  error = "subagent_timeout"
```

**Recovery Procedure:**
1. For optional subagents (internet-sleuth): Skip and continue
2. For required subagents (stakeholder-analyst): Retry once
3. If retry fails: Fall back to manual questioning

**User Communication:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Research Taking Longer Than Expected
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

The market research is taking too long.

Options:
  1. Wait a bit longer
  2. Skip research, continue with internal knowledge
  3. Try again later (checkpoint and resume)
```

---

#### ERR-008: Research Unavailable

**Trigger:** Network issues prevent internet-sleuth from working

**Detection:**
```
TRY:
  result = Task(subagent_type="internet-sleuth", ...)
EXCEPT NetworkError:
  error = "research_unavailable"
```

**Recovery Procedure:**
1. Skip research (it's optional)
2. Continue with Phase 3 using only user knowledge
3. Note in final document: "Market research was not conducted"

**User Communication:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Market Research Unavailable
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Unable to perform market research at this time.

Continuing without research. The brainstorm will be based
on the information you provide.

You can conduct market research separately and add it
to the document later if needed.
```

---

### 2.4 User Input Errors

#### ERR-009: Invalid Response

**Trigger:** User input doesn't match expected options

**Detection:**
```
IF user_response NOT IN valid_options AND user_response != "Other":
  error = "invalid_response"
```

**Recovery Procedure:**
1. Politely re-ask the question
2. Clarify the available options
3. Offer "Other" as escape hatch

**User Communication:**
```
I didn't quite understand that response.

Could you please choose one of these options:
  1. [Option 1]
  2. [Option 2]
  3. [Option 3]
  4. Other (type your own answer)
```

---

#### ERR-010: Contradictory Input

**Trigger:** User gives conflicting answers to related questions

**Detection:**
```
IF session.budget == "<$10K" AND session.timeline == "ASAP":
  IF session.scope == "Enterprise transformation":
    potential_contradiction = true
```

**Recovery Procedure:**
1. Acknowledge both inputs
2. Ask for clarification without judgment
3. Accept user's final answer

**User Communication:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Clarification Needed
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Earlier you mentioned:
  - Budget: <$10K
  - Timeline: ASAP (< 1 month)
  - Scope: Enterprise transformation

These seem challenging to align. Can you help me understand:
  1. Is the budget flexible?
  2. Is the timeline flexible?
  3. Can scope be reduced for initial phase?
  4. All are correct as stated (I understand the constraints)
```

---

## Section 3: Graceful Degradation

### 3.1 Degradation Strategies

| Feature | Full Mode | Degraded Mode | Trigger |
|---------|-----------|---------------|---------|
| Market research | internet-sleuth subagent | Manual questions only | Network error, timeout |
| Stakeholder analysis | stakeholder-analyst subagent | Manual questions | Subagent failure |
| Session persistence | JSON checkpoint | Display content for copy | File write failure |
| Resume capability | Automatic from checkpoint | Manual continuation | Corrupted checkpoint |

### 3.2 Feature Priority

When resources are limited, preserve these in order:

1. **User answers** - Never lose collected data
2. **Problem statement** - Core output
3. **Stakeholder map** - Critical for ideation
4. **Constraints** - Necessary for feasibility
5. **Priorities** - Important for scope
6. **Research data** - Nice to have, can skip
7. **Formatting** - Least critical

### 3.3 Minimal Viable Session

If multiple errors occur, ensure at minimum:

```
Required outputs:
  - problem_statement: captured
  - primary_stakeholder: identified
  - at_least_one_constraint: documented
  - session_summary: generated

Skip if necessary:
  - Detailed stakeholder analysis
  - Market research
  - Full hypothesis formation
  - Impact-effort matrix
```

---

## Section 4: User Communication Templates

### 4.1 Error Notification Format

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  [ERROR_TITLE]
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[Brief description of what happened]

[What is affected]

Options:
  1. [Recovery option 1]
  2. [Recovery option 2]
  3. [Alternative action]
```

### 4.2 Warning Notification Format

```
⚠ [WARNING_TITLE]

[What might happen]

Recommendation: [Suggested action]
```

### 4.3 Recovery Success Format

```
✓ [RECOVERY_ACTION] successful

[What was recovered]
[What's next]
```

### 4.4 Standard Error Messages

#### Session Management
```
SESSION_SAVED = "Session saved. Resume with: /brainstorm --resume {id}"
SESSION_RECOVERED = "Previous session recovered. Resuming from Phase {n}."
SESSION_NOT_FOUND = "No session found for {id}. Available: {list}"
CHECKPOINT_CREATED = "Checkpoint created: {path}"
```

#### File Operations
```
FILE_CREATED = "Created: {path}"
FILE_WRITE_FAILED = "Could not write to {path}. Trying alternative..."
DIRECTORY_CREATED = "Created directory: {path}"
```

#### Subagent Operations
```
RESEARCH_SKIPPED = "Market research skipped. Continuing with internal knowledge."
SUBAGENT_RETRY = "Retrying operation..."
SUBAGENT_FALLBACK = "Using fallback method instead."
```

---

## Section 5: Error Logging

### 5.1 What to Log

All errors should be logged with:
- Timestamp
- Error code
- Error message
- Context (phase, user action)
- Recovery action taken
- Outcome

### 5.2 Log Format

```json
{
  "timestamp": "2025-12-21T10:30:00Z",
  "session_id": "BRAINSTORM-001",
  "error_code": "ERR-007",
  "error_type": "subagent_timeout",
  "phase": 3,
  "context": "internet-sleuth research",
  "recovery_action": "skip_research",
  "outcome": "success",
  "user_notified": true
}
```

### 5.3 Log Location

Errors are logged to checkpoint file under `error_log` array for debugging:

```json
{
  "checkpoint_version": "1.0",
  "brainstorm_id": "BRAINSTORM-001",
  ...
  "error_log": [
    { "timestamp": "...", "error_code": "ERR-007", ... }
  ]
}
```

---

## Section 6: Testing Error Handling

### 6.1 Manual Test Scenarios

| Scenario | How to Trigger | Expected Behavior |
|----------|----------------|-------------------|
| Session timeout | Leave session idle 5+ min | Auto-checkpoint created |
| Resume not found | `/brainstorm --resume BRAINSTORM-999` | List available brainstorms |
| Network error | Disconnect network during research | Skip research, continue |
| Contradiction | Give conflicting budget/scope | Clarification question |

### 6.2 Recovery Verification

After any error recovery:
1. Verify user can continue session
2. Verify no data was lost
3. Verify checkpoint is valid (if created)
4. Verify user received clear communication

---

**Version:** 1.0 | **Status:** Published | **Created:** 2025-12-21
