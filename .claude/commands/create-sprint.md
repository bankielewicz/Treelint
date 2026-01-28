---
description: Create sprint plan with story selection
argument-hint: [sprint-name]
model: opus
allowed-tools: Read, Glob, AskUserQuestion, Skill
---

# Create Sprint Command

Creates a new 2-week sprint with interactive story selection, capacity validation, and automatic story status updates to "Ready for Dev".

---

## Quick Reference

```bash
# Create sprint with name
/create-sprint "User Authentication Sprint"

# Create sprint (will prompt for name)
/create-sprint
```

**Sprint Planning Workflow:**
1. Select epic linkage (optional)
2. Select stories from Backlog
3. Set sprint metadata (name, dates, duration)
4. Validate capacity (20-40 points recommended)
5. Create sprint file and update story statuses

---

## Command Workflow

### Phase 0: Argument Validation and Epic Discovery

**Parse sprint name from arguments:**

```
SPRINT_NAME = $1

IF SPRINT_NAME is empty:
    AskUserQuestion:
        Question: "What is the sprint name or theme?"
        Header: "Sprint Name"
        Options:
            - "Sprint [auto-number]" (default pattern)
            - "Custom name" (specify below)

    Extract SPRINT_NAME from response
```

**Discover available epics:**

```
Glob(pattern="devforgeai/specs/Epics/*.epic.md")

IF epics found:
    Read each epic's YAML frontmatter (id, title, status)

    AskUserQuestion:
        Question: "Link this sprint to an epic?"
        Header: "Epic"
        multiSelect: false
        Options:
            - ${FOR each epic: "${epic.id}: ${epic.title}"}
            - "Multiple epics (cross-epic sprint)"
            - "No epic (standalone sprint)"

    Extract EPIC_ID from response

ELSE:
    Display: "No epics found. Creating standalone sprint."
    EPIC_ID = "Standalone"
```

---

### Phase 1: Story Discovery and Selection

**Find available Backlog stories:**

```
Glob(pattern="devforgeai/specs/Stories/*.story.md")

backlog_stories = []

FOR each story_file:
    Read YAML frontmatter

    IF status == "Backlog":
        backlog_stories.append({
            id: story.id,
            title: story.title,
            points: story.points,
            priority: story.priority,
            epic: story.epic
        })
```

**Validate stories available:**

```
IF backlog_stories is empty:
    Display:
    ⚠️ No Stories Available for Sprint

    Action Required:
    1. Create stories: /create-story [description]
    2. Or move stories back to Backlog status

    Cannot proceed without Backlog stories.

    HALT
```

**Present story selection:**

```
Group stories by epic and priority

AskUserQuestion:
    Question: "Select stories for sprint (comma-separated IDs):"
    Header: "Story Selection"
    multiSelect: true
    Options:
        ${FOR each epic_group:
            Epic: ${epic_name}
            ${FOR each story in epic_group sorted by priority:
                - "${story.id}: ${story.title} (${story.points} pts) - ${story.priority}"
            }
        }

Extract SELECTED_STORY_IDS from response (comma-separated list)
```

**Calculate initial capacity:**

```
total_points = SUM(selected_stories.points)

IF total_points > 40:
    AskUserQuestion:
        Question: "Selected stories total ${total_points} points (over recommended 40). Proceed?"
        Header: "Over Capacity"
        Options:
            - "Proceed anyway (team has high velocity)"
            - "Remove lowest priority stories"
            - "Let me adjust selection"

    IF response == "adjust":
        Return to story selection

ELIF total_points < 20:
    AskUserQuestion:
        Question: "Selected stories total ${total_points} points (below typical 20). Add more?"
        Header: "Under Capacity"
        Options:
            - "Proceed anyway (partial sprint/holidays)"
            - "Add more stories"
            - "Let me adjust selection"

    IF response == "add":
        Return to story selection
```

---

### Phase 2: Sprint Metadata Collection

**Collect sprint details:**

```
AskUserQuestion:
    Question: "Sprint start date?"
    Header: "Start Date"
    Options:
        - "Today (${today_date})"
        - "Tomorrow (${tomorrow_date})"
        - "Next Monday (${next_monday})"
        - "Custom date (specify)"

Extract START_DATE from response


AskUserQuestion:
    Question: "Sprint duration?"
    Header: "Duration"
    Options:
        - "2 weeks / 14 days (standard)"
        - "1 week / 7 days (short sprint)"
        - "3 weeks / 21 days (extended)"
        - "Custom (specify days)"

Extract DURATION_DAYS from response
```

**Calculate end date:**

```
END_DATE = START_DATE + DURATION_DAYS
```

**Confirm sprint plan:**

```
Display summary:
📋 Sprint Plan Summary

  Sprint Name: ${SPRINT_NAME}
  Start Date: ${START_DATE}
  End Date: ${END_DATE}
  Duration: ${DURATION_DAYS} days
  Epic: ${EPIC_ID}
  Stories: ${SELECTED_STORY_IDS.length} (${total_points} points)

AskUserQuestion:
    Question: "Create sprint with these parameters?"
    Header: "Confirmation"
    Options:
        - "Yes - create sprint"
        - "No - adjust parameters"

IF response == "adjust":
    Return to Phase 0
```

---

### Phase 3: Invoke Orchestration Skill

**Set context markers for skill:**

```
**Operation:** plan-sprint
**Sprint Name:** ${SPRINT_NAME}
**Selected Stories:** ${SELECTED_STORY_IDS}
**Duration:** ${DURATION_DAYS} days
**Start Date:** ${START_DATE}
**Epic:** ${EPIC_ID}
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

**The skill instructs you to (Phase 3 - Sprint Planning):**

1. Extract sprint parameters from context markers
2. Validate selected stories exist and status == Backlog
3. Invoke `sprint-planner` subagent (isolated context):
   - Discover next sprint number
   - Calculate capacity and end date
   - Generate sprint document (YAML + markdown)
   - Write to devforgeai/specs/Sprints/Sprint-N.md
   - Update story statuses: Backlog → Ready for Dev
   - Add sprint references to stories
   - Add workflow history entries
4. Return structured summary with sprint details and next steps

**Token efficiency:**
- Skill execution: ~40K tokens (isolated context)
- Subagent execution: ~35K tokens (isolated context)
- Main conversation: ~5K tokens (command overhead only)

---

### Phase 4: Display Results

**Output skill result:**

```
${skill_result.display}
```

**The skill returns a pre-formatted display:**

```
✅ Sprint Created Successfully

Sprint Details:
  📋 SPRINT-N: ${sprint_name}
  📅 ${start_date} to ${end_date} (${duration} days)
  🎯 Epic: ${epic}
  📊 Capacity: ${points} points (${status})
  📝 Stories: ${count} selected

Stories Added:
  ✓ STORY-001: Title (5 pts) - HIGH
  ✓ STORY-002: Title (8 pts) - HIGH
  ...

Sprint File: devforgeai/specs/Sprints/Sprint-N.md

Next Steps:
  1. Review sprint goals
  2. Begin development: /dev STORY-001
  3. Track progress: /sprint-status
```

---

### Phase N: Feedback Hook Integration

**Collect feedback after sprint creation (non-blocking):**

```
# Check hooks enabled
Execute: devforgeai-validate check-hooks --operation=create-sprint --status=success

# Conditional invocation (non-blocking)
IF check-hooks exit == 0:
    Execute: devforgeai-validate invoke-hooks --operation=create-sprint --sprint-name="${SPRINT_NAME}" --story-count=${STORY_COUNT} --capacity=${CAPACITY_POINTS}

    IF invoke-hooks fails:
        Log to: devforgeai/feedback/logs/hook-errors.log
        Display: "⚠️ Feedback collection failed (sprint creation succeeded)"
```

**Features:**
- Non-blocking (sprint always succeeds)
- Shell-escaped: `"${SPRINT_NAME}"` prevents injection
- Empty sprint: `--story-count=0 --capacity=0` allowed
- **NFR-001:** check-hooks <100ms | **NFR-002:** invoke-hooks <3s | **NFR-003:** Total <3.5s

---

## Error Handling

### Error: No Arguments Provided

```
IF $1 is empty AND user cancels AskUserQuestion:
    Display: "Usage: /create-sprint [sprint-name]"
    HALT
```

### Error: No Epics Found

```
Display: "No epics found. Creating standalone sprint."
EPIC_ID = "Standalone"
Continue with story selection
```

### Error: No Backlog Stories

```
Display:
⚠️ No stories in Backlog

Action: Create stories first with /create-story

HALT
```

### Error: Story Selection Cancelled

```
IF user cancels story selection:
    Display: "Sprint creation cancelled."
    HALT
```

### Error: Skill Execution Failed

```
IF skill returns error:
    Display:
    ❌ Sprint Creation Failed

    Cause: ${skill_error.message}

    ${skill_error.details}

    Recovery: ${skill_error.recovery_steps}

    HALT
```

---

## Success Criteria

- [x] Sprint file created in `devforgeai/specs/Sprints/Sprint-N.md`
- [x] YAML frontmatter valid and complete
- [x] Selected stories linked to sprint
- [x] Story statuses updated to "Ready for Dev"
- [x] Sprint references added to story files
- [x] Workflow history entries added
- [x] Capacity calculated and validated
- [x] Epic linkage established (if selected)
- [x] Token usage ~5K (command overhead)
- [x] Character count ~8K (53% of budget)

---

## Integration

**Prerequisites:**
- At least 1 story with status = "Backlog"
- (Optional) Epic exists in `devforgeai/specs/Epics/`
- DevForgeAI context files exist (6 context files)

**Invokes:**
- `devforgeai-orchestration` skill (Phase 3: Sprint Planning Workflow)

**Skill invokes:**
- `sprint-planner` subagent (isolated context)

**Creates:**
- Sprint file: `devforgeai/specs/Sprints/Sprint-N.md`

**Updates:**
- Story files: status, sprint reference, workflow history

**Enables:**
- `/dev STORY-ID` - Stories now Ready for Dev
- `/sprint-status` - Track sprint progress
- `/close-sprint` - Complete sprint retrospective

**Related Commands:**
- `/create-epic` - Create epic before sprint (recommended)
- `/create-story` - Add stories to backlog
- `/dev` - Start story development
- `/orchestrate` - Full story lifecycle

---

## Performance

**Token Budget:**
- Command overhead: ~3K tokens
- User interaction: ~2K tokens
- Skill execution: ~40K tokens (isolated)
- Subagent (sprint-planner): ~35K tokens (isolated)
- **Total main conversation:** ~5K tokens

**Character Budget:**
- Current: ~8,000 characters
- Limit: 15,000 characters
- Usage: 53% ✅ COMPLIANT

**Execution Time:**
- User interaction: 2-5 minutes
- Sprint creation: 30-60 seconds
- **Total:** 3-6 minutes

---

## Architecture (Post-Refactoring 2025-11-05)

**Command (250 lines - Lean Orchestration):**
- Phase 0: User interaction (epic, stories, metadata via AskUserQuestion)
- Phase 3: Skill invocation with context markers
- Phase 4: Result display

**Skill (devforgeai-orchestration - Phase 3):**
- Step 1: Extract sprint parameters from conversation
- Step 2: Invoke sprint-planner subagent
- Step 3: Process subagent result
- Step 4: Return formatted summary

**Subagent (sprint-planner - NEW):**
- Phase 1: Sprint discovery (calculate next sprint number)
- Phase 2: Story validation (verify Backlog status)
- Phase 3: Metrics calculation (capacity, dates)
- Phase 4: Document generation (YAML + markdown)
- Phase 5: Story updates (status, references, history)
- Phase 6: Summary report (structured JSON)

**Reference File:**
- `.claude/skills/devforgeai-orchestration/references/sprint-planning-guide.md`
- Provides capacity guidelines, status transitions, file structures

**Token Efficiency:**
- Before: ~12K tokens in main conversation
- After: ~5K tokens in main conversation
- **Savings:** 58% reduction (7K tokens)

**Character Reduction:**
- Before: 497 lines, 12,525 chars (84% of budget)
- After: 250 lines, 8,000 chars (53% of budget)
- **Reduction:** 50% line reduction, 36% character reduction

---

## Notes

**Design Philosophy:**
- Commands orchestrate user interaction and delegate to skills
- Skills coordinate workflow phases and invoke subagents
- Subagents execute specialized tasks in isolated contexts
- Reference files provide framework guardrails

**Framework Integration:**
- Respects 11-state workflow (Backlog → Ready for Dev transition)
- Enforces capacity planning (20-40 points for 2-week sprints)
- Maintains workflow history (timestamp, status, actions)
- Links epic hierarchy (epic → sprint → stories)

**When to Use:**
- Starting new 2-week sprint
- Selecting stories for coordinated development
- Planning feature batch for release
- Setting team velocity and capacity goals

**When NOT to Use:**
- For single-story work (use `/dev STORY-ID` directly)
- Mid-sprint (wait for current sprint to complete)
- Without backlog stories (create stories first with `/create-story`)

**Best Practices:**
- Link sprints to epics for traceability
- Balance story points (20-40 ideal for 2-week sprint)
- Prioritize HIGH stories first
- Review sprint goals before starting
- Update sprint progress regularly
