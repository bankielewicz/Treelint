---
description: Create user story with acceptance criteria and technical specification
argument-hint: [feature-description | epic-id]
model: opus
allowed-tools: Read, Glob, Grep, Skill, AskUserQuestion, TodoWrite
---

# /create-story - Create User Story

Transform feature → complete story with AC, tech spec, UI spec. Single mode or batch from epic. Invokes `devforgeai-story-creation` skill.

---

## Phase 0: Mode Detection (NEW - Batch Support)

**Parse argument and detect mode:**
```
ARG=$1

# Mode 1: Epic reference (epic-001, EPIC-001, Epic-001)
if ARG matches ^[Ee][Pp][Ii][Cc]-\d{3}$:
    MODE="EPIC_BATCH"
    EPIC_ID=$(normalize to EPIC-XXX format)
    → Proceed to Epic Batch Workflow (below)

# Mode 2: Feature description (10+ words)
elif word_count(ARG) >= 10:
    MODE="SINGLE_STORY"
    → Proceed to Phase 1 (Single Story Workflow)

# Mode 3: Ambiguous or no argument
else:
    AskUserQuestion: "Single story or batch from epic?"
    Based on response → MODE="SINGLE_STORY" or "EPIC_BATCH"
```

**If EPIC_BATCH, validate epic exists:**
```
Glob(pattern="devforgeai/specs/Epics/${EPIC_ID}*.epic.md")
if not found:
    AskUserQuestion: "Epic ${EPIC_ID} not found. Enter different ID or cancel?"
```

---

## Epic Batch Workflow (NEW)

**Triggered:** MODE="EPIC_BATCH"

1. Extract features from epic (Grep: "### Feature X.Y:")
2. Multi-select features (AskUserQuestion, multiSelect: true)
3. Batch metadata: sprint, priority
4. Loop: Gap-aware ID → Markers → Skill → Track
5. Summary: Created/failed counts, story list

**Context markers per story:**
```
**Story ID:** STORY-007
**Epic ID:** EPIC-001
**Feature Description:** {feature.description}
**Priority:** {priority}
**Points:** {feature.points}
**Sprint:** {sprint}
**Batch Mode:** true
```

**Completion summary:** Counts, story list, next action

---

## Phase 1: Single Story Workflow

**Triggered:** MODE="SINGLE_STORY"

### 1.1 Capture Feature Description

**Feature description from user:**
```
$ARGUMENTS
```

**If no arguments provided:**

Use AskUserQuestion to capture feature description:

```
AskUserQuestion(
  questions=[{
    question: "Please describe the feature you want to create a story for",
    header: "Feature description",
    options: [
      {
        label: "CRUD operation",
        description: "Create, read, update, or delete data (e.g., manage users, manage products)"
      },
      {
        label: "Authentication/Authorization",
        description: "Login, signup, password reset, permissions, access control"
      },
      {
        label: "Workflow/Process",
        description: "Multi-step process or state transitions (e.g., order processing, approvals)"
      },
      {
        label: "Reporting/Analytics",
        description: "Data visualization, reports, dashboards, charts"
      },
      {
        label: "Integration",
        description: "Connect to external services (payment gateway, email, etc.)"
      },
      {
        label: "Other",
        description: "Different type of feature - I'll describe it"
      }
    ],
    multiSelect: false
  }]
)
```

**Then ask for detailed description:**
```
"Please provide a detailed description of the {feature_type} feature:
- What should users be able to do?
- What is the expected outcome?
- Any specific requirements or constraints?"
```

**Wait for user response before proceeding to Phase 2.**

### 1.2 Validate Description

**Minimum requirements:**
- Description has at least 10 words
- Describes user capability or business feature (not purely technical implementation)

**If description too vague or technical:**
```
Prompt user: "Please describe the feature from a user perspective:
- What user need does this address?
- What actions will users perform?
- What value does this provide?"

Avoid implementation details like "implement REST API" or "create database table"
Focus on user-facing capabilities like "users can register accounts" or "admins can view reports"
```

---

## Phase 2: Invoke Story Creation Skill

### 2.1 Set Context for Skill

**Prepare context markers for skill execution:**

```
**Feature Description:** $ARGUMENTS (or user-provided description)

**Feature Type:** {CRUD|Authentication|Workflow|Reporting|Integration|Other}
```

### 2.2 Skill Invocation

**The devforgeai-story-creation skill handles complete workflow:**

- **Phase 1:** Story Discovery & Context (ID generation, epic/sprint, metadata)
- **Phase 2:** Requirements Analysis (requirements-analyst subagent, AC generation)
- **Phase 3:** Technical Specification (api-designer subagent, data models, business rules)
- **Phase 4:** UI Specification (components, mockups, interfaces, accessibility)
- **Phase 5:** Story File Creation (YAML + markdown construction)
- **Phase 6:** Epic/Sprint Linking (update parent documents)
- **Phase 7:** Self-Validation (quality checks, self-healing)
- **Phase 8:** Completion Report (summary, next actions)

**Expected interaction:**
- Skill asks 5-10 questions for epic/sprint association, priority, story points
- Skill invokes requirements-analyst subagent (generates user story + AC)
- Skill invokes api-designer subagent if API detected
- Skill may ask UI confirmation if UI components detected
- Skill validates story quality internally (Phase 7)
- Skill presents detailed summary (Phase 8)
- Skill asks user for next action (Phase 8)

**Invoke skill:**

```
Skill(command="devforgeai-story-creation")
```

**After skill invocation:**
- Skill's SKILL.md content expands inline in conversation
- **YOU execute the skill's workflow phases** (not waiting for external result)
- Follow the skill's instructions phase by phase
- Produce output as skill instructs

**The skill instructs you to:**
- Execute all 8 phases: Discovery, Requirements Analysis, Technical Spec, UI Spec, File Creation, Linking, Self-Validation, Completion Report
- Invoke requirements-analyst subagent (Phase 2) and api-designer subagent (Phase 3, if needed)
- Handle all user interactions (AskUserQuestion for metadata)
- Create story file with complete specifications
- Perform self-validation in Phase 7
- Present completion summary in Phase 8

---

## Phase 3: Verify Story Created

### 3.1 Check Skill Completion Status

**After skill returns control:**

Verify skill completed successfully by checking for story file:

```
# Find all story files
story_files = Glob(pattern="devforgeai/specs/Stories/STORY-*.story.md")

# Identify newest story (should be the one just created)
# Story files are named STORY-XXX-{slug}.story.md
# Find highest number
```

**Expected artifact:**
- 1 new story file in `devforgeai/specs/Stories/`
- Filename format: `STORY-XXX-{slug}.story.md`

### 3.2 Handle Incomplete Execution

**If no new story file detected:**

```
# Compare story count before and after skill execution
# If count unchanged:

Report: """
⚠️ Story Creation Incomplete

Expected story file not found in `devforgeai/specs/Stories/`

Possible causes:
1. Skill execution interrupted
2. User exited during discovery questions
3. File system write permissions issue
4. Validation failed and skill halted

Recommended actions:
- Re-run `/create-story [feature-description]` to retry
- Check `devforgeai/specs/Stories/` directory for partial files
- Verify write permissions to devforgeai/specs/ directory

If issue persists, create story manually or contact support.
"""

HALT - Do not proceed to Phase 4
```

**If story file found:**

```
✓ Skill completed successfully
✓ Story file created

→ Proceed to Phase 4
```

---

## Phase 4: Brief Confirmation

**Note:** Skill already presented detailed summary in Phase 8. This phase provides brief command-level confirmation only.

### 4.1 Read Story Frontmatter

**Read just the frontmatter for quick validation:**

```
# Get newest story file
newest_story = story_files[-1]  # Last in sorted list

# Read first 20 lines (frontmatter only)
Read(file_path=newest_story, limit=20)
```

**Extract:**
- Story ID (e.g., STORY-042)
- Title
- Epic association (if any)
- Sprint association
- Priority
- Story points

### 4.2 Brief Confirmation Message

**Present concise confirmation:**

```
✅ Story created successfully

**Story:** {story_id}
**Title:** {title}
**Epic:** {epic_id or "None"}
**Sprint:** {sprint_id or "Backlog"}
**Priority:** {priority}
**Points:** {points}
**Status:** Backlog

**File location:**
`devforgeai/specs/Stories/{story_id}-{slug}.story.md`

The devforgeai-story-creation skill has:
✓ Generated user story and acceptance criteria
✓ Created technical specification {and API contracts if applicable}
✓ Documented UI specification {if UI components detected}
✓ Defined non-functional requirements
✓ Validated story quality (Phase 7)
✓ Linked to epic/sprint {if applicable}

{The skill should have asked you for next action in Phase 8.}
```

---

## Phase 4.5: Context Preservation Validation (STORY-299)

**Invoke context-preservation-validator for provenance chain validation:**

```
Task(
  subagent_type="context-preservation-validator",
  description="Validate context preservation for story",
  prompt="Validate story-to-epic-to-brainstorm chain for ${story_file_path}. Trace full provenance and report chain status (intact/partial/broken)."
)
```

**Behavior:** Non-blocking by default. Displays warning if provenance chain incomplete or broken.

---

## Phase 5: Hook Integration (STORY-027)

**Purpose:** Integrate feedback hooks for story quality retrospection.

**Key Workflow:**
1. Check hooks enabled (`devforgeai/config/hooks.yaml`, default: disabled)
2. Detect batch mode (`**Batch Mode:** true` marker)
3. If batch: defer until all stories created (single invocation with all IDs)
4. If single: validate story ID (STORY-NNN regex), file exists
5. Assemble 7 metadata fields from story YAML (story_id, epic_id, sprint, title, points, priority, timestamp)
6. Invoke hook (timeout: 30s default, performance: <100ms p95 requirement)
7. Log to `devforgeai/feedback/.logs/hooks.log` (success) and `hook-errors.log` (errors)

**Critical Principles:**
- Failures never break exit code (always 0, story created successfully)
- Story ID validation prevents command injection (STORY-\d{3} format only)
- Batch mode invokes hook once at end with all created story IDs

**For implementation details, error scenarios, and 69 passing tests, see `.claude/commands/references/hook-integration-guide.md`**

---

## Phase 6: Next Steps

**Skill Phase 8 already asked user for next action.** This is a backup if user needs clarification.

**Common next actions:**
- `/create-story [description]` - Create another story
- `/dev {story_id}` - Start development (requires context files)
- Review story file in editor
- `/create-sprint` to assign story to sprint

---

## Error Handling

### Skill Invocation Failed
```
ERROR: devforgeai-story-creation skill invocation failed

Troubleshooting:
1. Verify: Glob(pattern=".claude/skills/devforgeai-story-creation/SKILL.md")
2. Restart terminal (skill may need re-registration)
3. Check skill frontmatter is valid YAML
```

### Story File Not Created
```
⚠️ Skill completed but story file not found

Possible causes: Skill error, permissions issue, user exit, validation failure

Actions:
- Check skill error messages
- Verify `devforgeai/specs/Stories/` is writable
- Re-run `/create-story [feature-description]`
```

### Insufficient Description
```
⚠️ Feature description too brief

Requirements: 10+ words, user-focused (not technical)

Good: "Admin users can manage product listings with images and pricing"
Bad: "Add products" or "Implement API endpoints"

Provide more detail and re-run.
```

---

## Command Complete

**This command delegates all implementation logic to the devforgeai-story-creation skill.**

**Command responsibilities:**
- ✅ Argument validation and capture
- ✅ Skill invocation with context markers
- ✅ Basic story file existence verification
- ✅ Brief completion confirmation
- ✅ Next steps guidance (backup to skill's Phase 8)

**Skill responsibilities:**
- ✅ Complete 8-phase story creation workflow
- ✅ User interaction (epic/sprint selection, metadata collection)
- ✅ Subagent orchestration (requirements-analyst, api-designer)
- ✅ Story file generation with all sections
- ✅ Epic/sprint linking
- ✅ Self-validation (Phase 7)
- ✅ Detailed completion report (Phase 8)
- ✅ Error handling and recovery

**Architecture principle:** Commands orchestrate, skills implement, references provide deep knowledge through progressive disclosure.

---

## Integration

**Workflow:** /ideate → /create-context → /create-sprint → **/create-story** → /create-ui → /dev → /qa → /release

**Used by:** Users, orchestration skill, development skill, sprint planning

**Triggers:** /create-ui, /dev, /create-sprint

---

## Success Criteria

Story created with:
- [ ] Valid story ID (STORY-NNN format)
- [ ] User story (As a/I want/So that)
- [ ] 3+ acceptance criteria (Given/When/Then)
- [ ] Technical specification (complete)
- [ ] UI specification (if applicable)
- [ ] Non-functional requirements (measurable)
- [ ] Edge cases documented
- [ ] Definition of Done (checkboxes)
- [ ] File written to devforgeai/specs/Stories/
- [ ] Epic/sprint linked (if applicable)
- [ ] Skill Phase 7 validation passed

---

## Performance

**Token Budget:** Command ~3.5K tokens (single), ~6K tokens (batch), Skill ~90K tokens/story (isolated)
**Execution Time:** Single 2-5 min, Batch 10-15 min for 5 stories (sequential)
**Character Budget:** 14,163 chars (94% of 15K limit) ✅

---

## References

**Skill documentation:**
- `.claude/skills/devforgeai-story-creation/SKILL.md` - 8-phase workflow overview

**Skill reference files (loaded by skill):**
- `references/story-discovery.md` - Phase 1: ID generation, context discovery
- `references/requirements-analysis.md` - Phase 2: User story, AC (with RCA-007 enhancements)
- `references/technical-specification-creation.md` - Phase 3: APIs, data models, business rules
- `references/ui-specification-creation.md` - Phase 4: Components, mockups, accessibility
- `references/story-file-creation.md` - Phase 5: Document assembly, template usage
- `references/epic-sprint-linking.md` - Phase 6: Parent document updates
- `references/story-validation-workflow.md` - Phase 7: Quality checks, self-healing
- `references/completion-report.md` - Phase 8: Summary generation, next actions

**Supporting references:**
- `references/story-examples.md` - 5 complete examples (CRUD, auth, workflow, reporting, integration)
- `references/acceptance-criteria-patterns.md` - Given/When/Then templates
- `references/story-structure-guide.md` - YAML frontmatter, section formatting
- `references/validation-checklists.md` - Quality validation procedures

**Template:**
- `assets/templates/story-template.md` - Base story template

---

**Command follows lean orchestration pattern: Argument validation → Skill invocation → Results verification → Completion confirmation**
