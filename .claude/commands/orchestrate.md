---
description: Execute full story lifecycle end-to-end
argument-hint: [STORY-ID]
model: opus
allowed-tools: Read, Glob, Skill, AskUserQuestion
---

# /orchestrate - Complete Story Lifecycle Orchestration

Execute automated end-to-end workflow: Development → QA → Staging → Production

Orchestrates story from "Ready for Dev" through full deployment with automatic checkpoint resume and QA retry handling.

---

## Quick Reference

```bash
# Full orchestration (dev → QA → staging → production)
/orchestrate STORY-042

# Resume from checkpoint (auto-detected)
/orchestrate STORY-042    # Skill detects previous progress and resumes
```

---

## Command Workflow

### Phase 0: Argument Validation and Story Loading

**Validate story ID format:**
```
STORY_ID = $1

IF $1 is empty OR does NOT match pattern "STORY-[0-9]+":
  AskUserQuestion:
    Question: "Story ID format invalid. What story should I orchestrate?"
    Header: "Story ID"
    Options:
      - label: "List stories in Ready for Dev status"
        description: "Show stories ready to begin development"
      - label: "List stories in Dev Complete status"
        description: "Show stories ready for QA validation"
      - label: "List stories in QA Approved status"
        description: "Show stories ready for release"
      - label: "Show correct /orchestrate syntax"
        description: "Display usage examples"
    multiSelect: false

  Extract STORY_ID from user response OR exit if cancelled
```

**Verify story file exists:**
```
Glob(pattern="devforgeai/specs/Stories/${STORY_ID}*.story.md")

IF no matches found:
  AskUserQuestion:
    Question: "Story ${STORY_ID} not found. What should I do?"
    Header: "Story Not Found"
    Options:
      - label: "List all available stories"
        description: "Show all stories in devforgeai/specs/Stories/"
      - label: "Cancel orchestration"
        description: "Exit command"
    multiSelect: false

  IF user selects "List":
    Glob(pattern="devforgeai/specs/Stories/*.story.md")
    Display story list

  Exit command
```

**Load story via @file reference:**
```
@devforgeai/specs/Stories/${STORY_ID}*.story.md

(Story content now loaded in conversation context)
```

**Validation summary:**
```
Display:
"✓ Story ID: ${STORY_ID}
 ✓ Story file loaded
 ✓ Starting orchestration...
"
```

---

### Phase 1: Invoke Orchestration Skill

**Set context markers for skill:**
```
Display:
"━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  DevForgeAI Story Lifecycle Orchestration
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

**Command:** orchestrate
**Story ID:** ${STORY_ID}
**Auto-Resume:** Enabled

Delegating to devforgeai-orchestration skill...
"
```

**Invoke skill:**
```
Skill(command="devforgeai-orchestration")
```

**After skill invocation:**
- Skill's SKILL.md content expands inline in conversation
- **YOU execute the skill's workflow phases** (not waiting for external result)
- Follow the skill's instructions phase by phase
- Produce output as skill instructs

**The skill instructs you to execute orchestration workflow:**
- **Phase 0:** Load story, detect checkpoints, determine starting phase
- **Phase 1:** Validate story and workflow state
- **Phase 2:** Invoke devforgeai-development skill (execute TDD workflow)
- **Phase 3:** Invoke devforgeai-qa skill (execute deep validation)
- **Phase 3.5:** Handle QA failures with intelligent retry (max 3 attempts)
  - Deferral-specific recovery options
  - Follow-up story creation
  - Loop prevention (suggests story split after 3 failures)
- **Phase 4:** Invoke devforgeai-release skill (execute staging deployment)
- **Phase 5:** Invoke devforgeai-release skill (execute production deployment)
- **Phase 6:** Finalize workflow, update story status, generate summary

**Skill returns:** Structured summary with orchestration results

---

### Phase 2: Display Orchestration Results

**Receive result from skill and display:**
```
Skill returns structured summary:
{
  "status": "success|halted|max_retries|already_released|blocked",
  "story_id": "STORY-NNN",
  "final_status": "Released|QA Failed|Dev Complete",
  "summary_message": "Complete message for user with metrics",
  "next_steps": [...]
}

Display orchestration results:
  {result.summary_message}

Next Steps:
  {FOR step in result.next_steps:}
    - {step}
  {END FOR}
```

---

### Phase 3: Handle Orchestration Outcomes

**Success (status = "success"):**
```
Display result.summary_message

Example:
"🎉 Story STORY-042 Orchestration Complete!

✅ Development: Code implemented, 45 tests passing
✅ QA: All quality gates passed
✅ Staging: Deployed and validated successfully
✅ Production: Live with green health checks

Duration: 75 minutes
Status: Released

Monitor production metrics for 24 hours."
```

**QA Max Retries (status = "max_retries"):**
```
Display:
"❌ Orchestration Halted - QA Max Retries Exceeded

Story: {result.story_id}
Status: QA Failed (3 attempts)

This indicates story scope is too large.

Recommended Actions:
1. Split story into 2-3 smaller stories
2. Review DoD items for proper estimation
3. Escalate blockers to leadership

QA Report: devforgeai/qa/reports/{STORY_ID}-qa-report.md"
```

**User Halted (status = "halted"):**
```
Display:
"⏸️  Orchestration Paused - User Intervention

Story: {result.story_id}
Reason: {result.halt_reason}

Resume: /orchestrate {STORY_ID} (will continue from checkpoint)"
```

**Already Released (status = "already_released"):**
```
Display:
"✅ Story Already Released

Story: {result.story_id}
Status: Released
Completed: {result.completion_date}

No orchestration needed."
```

**Blocked (status = "blocked"):**
```
Display:
"🚫 Orchestration Blocked

Story: {result.story_id}
Reason: {result.block_reason}

{result.resolution_guidance}"
```

---

## Error Handling

### Story ID Invalid
```
ERROR: Invalid story ID format

Expected: STORY-NNN (e.g., STORY-001, STORY-042)
Received: '$1'

Usage: /orchestrate [STORY-ID]

Examples:
  /orchestrate STORY-001
  /orchestrate STORY-042
```

### Story File Not Found
```
ERROR: Story file not found

Path: devforgeai/specs/Stories/${STORY_ID}*.story.md

Available stories:
{Glob(pattern="devforgeai/specs/Stories/*.story.md")}

Use /create-story to create new story.
```

### Orchestration Skill Failed
```
ERROR: Orchestration skill execution failed

Story: ${STORY_ID}
Error: {skill_error_message}

This may indicate:
- Invalid story state
- Missing prerequisites (context files)
- System issue

Review error above and retry or contact support.
```

---

## Success Criteria

- [ ] Story progresses through all required phases
- [ ] Checkpoints detected and resume works correctly
- [ ] QA failures handled with retry logic (in skill)
- [ ] All quality gates enforced
- [ ] Workflow history updated with complete timeline
- [ ] Story status = "Released" on successful completion
- [ ] Token usage: Command ~2.5K overhead + skill in isolated context

---

## Integration

**Invoked by:** User via `/orchestrate [STORY-ID]` command

**Invokes:** `devforgeai-orchestration` skill which coordinates:
- devforgeai-development (Phase 2: TDD implementation)
- devforgeai-qa (Phase 3: Quality validation)
- devforgeai-qa retry handling (Phase 3.5: Intelligent retry with loop prevention)
- devforgeai-release (Phases 4-5: Staging and production deployment)

**Updates:**
- Story status (workflow state transitions)
- Story workflow history (timeline, checkpoints, phase results)
- Story YAML frontmatter (completed_date, orchestration metadata)

**Quality Gates Enforced:**
- Gate 1: Context Validation (before development)
- Gate 2: Test Passing (before QA)
- Gate 3: QA Approval (before release)
- Gate 4: Release Readiness (deployment validation)

---

## Performance

**Token Budget:**
- Command overhead: ~2.5K tokens (argument validation + skill invocation + display)
- Skill execution (isolated context): ~155K-175K tokens total
  - Development: ~85K
  - QA (deep): ~65K
  - QA retry (if needed): ~20K additional
  - Release (staging + production): ~40K
- **Main conversation impact:** ~2.5K (83% reduction from 15K original)

**Execution Time:**
- Typical (no retries): 60-90 minutes
  - Development: 30-45 min
  - QA: 10-15 min
  - Staging: 5-10 min
  - Production: 10-15 min
- With retries: Add 40-60 min per QA retry cycle
- Complex stories: Up to 2 hours

**Character Budget:**
- Target: ~9,000 characters (60% of 15K limit)
- Down from: 15,012 characters (100% of limit, over by 12)
- Savings: ~6,000 characters (40% reduction)

---

## Usage Examples

**Full orchestration:**
```
/orchestrate STORY-042
→ Executes: Dev → QA → Staging → Production
→ Result: Story released to production
```

**Resume from checkpoint:**
```
/orchestrate STORY-042  # Auto-detects previous progress
→ Skill resumes from last checkpoint
→ Skips completed phases
```

**After QA failure:**
```
/orchestrate STORY-042
→ Skill offers retry options
→ Coordinates dev fix → QA retry
→ Continues to release if QA passes
```

---

## What the Skill Handles

The devforgeai-orchestration skill executes complete workflow coordination:

**Phases:**
- Phase 0: Checkpoint detection (NEW - was in command)
- Phase 1: Story validation
- Phase 2: Development (invokes devforgeai-development)
- Phase 3: QA (invokes devforgeai-qa)
- Phase 3.5: QA retry with loop prevention (NEW - was in command, max 3 attempts)
- Phase 4: Staging release (invokes devforgeai-release)
- Phase 5: Production release (invokes devforgeai-release)
- Phase 6: Finalization (NEW - was in command)

**Key Features:**
- Automatic checkpoint resume (Phase 0)
- QA retry coordination (Phase 3.5)
- Loop prevention (max 3 QA attempts)
- Follow-up story creation for deferrals
- Complete workflow history tracking
- Structured summary generation

---

## Checkpoint Resume Capability

The skill automatically detects and resumes from checkpoints:

| Checkpoint | Starting Phase | Skips |
|------------|----------------|-------|
| **None** | Phase 2 (Development) | None (full cycle) |
| **DEV_COMPLETE** | Phase 3 (QA) | Phase 2 |
| **QA_APPROVED** | Phase 4 (Staging) | Phases 2-3 |
| **STAGING_COMPLETE** | Phase 5 (Production) | Phases 2-4 |
| **PRODUCTION_COMPLETE** | Skip (already done) | All (exit with message) |

**Checkpoint precedence:** Checkpoint status overrides story status field

---

## QA Retry Handling (Phase 3.5)

The skill now handles QA failures intelligently:

**Retry Loop:**
1. QA fails → Skill categorizes failure type
2. Skill counts retry attempts (from workflow history)
3. If < 3 attempts: User chooses recovery strategy
4. If "retry": Skill re-invokes dev → QA automatically
5. If QA passes: Continue to staging
6. If QA fails again: Loop repeats (up to 3 total attempts)
7. If 3 attempts reached: Halt with story split recommendation

**Recovery Options:**
- **Retry:** Automatic dev fix → QA retry cycle
- **Follow-ups:** Create tracking stories for deferred items
- **Manual:** User fixes via /dev, then re-run /orchestrate
- **Exception:** Create ADR for coverage/anti-pattern exceptions

**This was 134 lines in command, now coordinated by skill.**

---

## Error Recovery

### Manual Phase Execution (If Orchestration Halted)

```bash
# Development only
/dev STORY-042

# QA only
/qa STORY-042

# Staging only
/release STORY-042 staging

# Production only
/release STORY-042 production

# Resume orchestration
/orchestrate STORY-042    # Auto-resumes from checkpoint
```

### Common Error Scenarios

**Scenario 1: QA Fails Due to Deferrals**
- Skill presents 3 options (retry, follow-ups, manual)
- User chooses path
- Skill coordinates recovery
- Command displays result

**Scenario 2: Staging Deployment Fails**
- Skill creates STAGING_FAILED checkpoint
- Orchestration halts
- User fixes deployment config
- Run: `/orchestrate STORY-042` (resumes from staging)

**Scenario 3: Manual Process Running**
- Story status: "In Development"
- Skill blocks orchestration (manual process detected)
- User completes manual process or cancels
- Re-run: `/orchestrate STORY-042`

---

## Architecture (Lean Orchestration Pattern)

**This command exemplifies lean orchestration:**

**Command responsibilities (ONLY):**
1. ✅ Parse arguments (story ID validation)
2. ✅ Load context (story file via @file)
3. ✅ Set markers (explicit context for skill)
4. ✅ Invoke skill (single delegation point)
5. ✅ Display results (output what skill returns)

**Command does NOT:**
- ❌ Parse checkpoints (skill Phase 0 does this)
- ❌ Coordinate retries (skill Phase 3.5 does this)
- ❌ Update story status (skill Phase 6 does this)
- ❌ Determine starting phase (skill Phase 0 does this)
- ❌ Count retry attempts (skill Phase 3.5 does this)

**Benefits:**
- Command loads quickly (~2.5K tokens vs ~4K before)
- Skill contains all orchestration logic (proper layer)
- Easy to maintain (single source of truth)
- Budget compliant (60% usage vs 100% before)

---

## Related Commands

**Workflow commands:**
- `/dev [STORY-ID]` - Development only (no QA or release)
- `/qa [STORY-ID]` - QA validation only
- `/release [STORY-ID] [env]` - Release only (staging or production)
- `/orchestrate [STORY-ID]` - Complete lifecycle (dev → QA → release)

**Planning commands:**
- `/create-story [description]` - Create story before orchestration
- `/create-sprint [name]` - Plan sprint before development

**Framework maintenance:**
- `/audit-budget` - Check command character budgets
- `/audit-deferrals` - Check story deferral violations

---

## Notes

**Lean Orchestration Applied:**
- Checkpoint detection: Skill Phase 0 (was in command)
- QA retry coordination: Skill Phase 3.5 (was in command)
- Finalization: Skill Phase 6 (was in command)
- Command delegates, skill coordinates, clean separation

**Refactored:** 2025-11-06 (599 → 527 lines, 15,012 → 14,422 chars)
**Reduction:** 12% lines, 4% characters
**Budget:** 96% (was 100% over limit)
**Pattern:** Lean orchestration (234 lines business logic extracted to skill)

---

**Version:** 2.0 - Lean Orchestration | **Refactored:** 2025-11-06 | **Pattern:** Phase 3 Lean Orchestration
