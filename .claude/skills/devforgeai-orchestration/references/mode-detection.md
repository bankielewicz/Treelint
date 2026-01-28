# Mode Detection Logic

This file contains the complete logic for detecting which orchestration mode to execute based on conversation context markers.

## Purpose

The DevForgeAI orchestration skill operates in multiple modes to handle different workflow entry points:
1. **Epic Creation Mode** - Creating new epics from requirements
2. **Sprint Planning Mode** - Planning sprints with story selection
3. **Story Management Mode** - Standard story lifecycle orchestration
4. **Default Mode** - Fallback behavior when mode cannot be determined

The skill searches the conversation for explicit context markers to determine which workflow to execute.

---

## How Mode Detection Works

**Detection sequence:**
1. Check for Epic Creation markers first (highest priority)
2. Check for Sprint Planning markers second
3. Check for Story Management markers third
4. Fall back to Default Mode if no markers found

**Context markers** are explicit statements in the conversation formatted as:
```
**Command:** create-epic
**Epic name:** User Authentication System
```

The skill uses Grep to search the conversation for these patterns.

---

## Epic Creation Mode

### Detection Markers

**Required markers:**
- `**Command:** create-epic`
- `**Epic name:** {name}` (must be present)

### Entry Point

**Phase 4A** - Epic Creation Workflow (7-phase comprehensive process)

### Workflow Summary

1. Epic Discovery - Generate EPIC-ID, check duplicates
2. Context Gathering - Goal, timeline, priority, stakeholders, success criteria
3. Feature Decomposition - requirements-analyst subagent, 3-8 features
4. Technical Assessment - architect-reviewer subagent, complexity scoring
5. Epic File Creation - Populate epic-template.md, write to disk
6. Optional Requirements Spec - Detailed requirements document
7. Validation & Self-Healing - 9 validation checks, auto-correct issues

### Subagents Used

- **requirements-analyst** - Feature decomposition (Phase 3), optional requirements spec (Phase 6)
- **architect-reviewer** - Technical assessment and complexity scoring (Phase 4)

### Output Files

- Epic document: `devforgeai/specs/Epics/{EPIC-ID}.epic.md`
- Optional requirements spec: `devforgeai/specs/requirements/{EPIC-ID}-requirements.md`
- Structured completion summary (JSON for command display)

### Reference Files Loaded Progressively

- `epic-management.md` - Phases 1-2 (epic discovery and context gathering)
- `feature-decomposition-patterns.md` - Phase 3 (patterns by domain)
- `technical-assessment-guide.md` - Phase 4 (complexity scoring rubric)
- `epic-template.md` - Phase 5 (epic document template)
- `epic-validation-checklist.md` - Phase 7 (validation procedures)

### Example Detection

```
Conversation contains:
**Command:** create-epic
**Epic name:** User Authentication System

Result: Epic Creation Mode detected → Execute Phase 4A workflow
```

---

## Sprint Planning Mode

### Detection Markers

**Required markers:**
- `**Command:** create-sprint` OR `**Command:** plan-sprint`
- `**Sprint Name:** {name}` (required)
- `**Selected Stories:** {story-ids}` (required, comma-separated)

### Entry Point

**Phase 3** - Sprint Planning Workflow

### Workflow Summary

1. Sprint Discovery - Calculate next sprint number
2. Story Validation - Verify all stories exist and are in Backlog status
3. Capacity Validation - Calculate total story points, check 20-40 point range
4. Sprint Creation - Generate sprint document with YAML + markdown
5. Story Updates - Update all stories to "Ready for Dev" status, add sprint references
6. Summary Report - Return structured JSON with sprint details

### Subagents Used

- **sprint-planner** - Complete sprint creation workflow (all phases)

### Output Files

- Sprint document: `devforgeai/specs/Sprints/Sprint-{N}.md`
- Story files updated with sprint references and status changes
- Sprint summary for command display

### Reference Files Loaded

- `sprint-planning-guide.md` - Capacity guidelines, date calculations, status transition rules

### Example Detection

```
Conversation contains:
**Command:** create-sprint
**Sprint Name:** Sprint-3
**Selected Stories:** STORY-001, STORY-002, STORY-003

Result: Sprint Planning Mode detected → Execute Phase 3 workflow
```

---

## Story Management Mode

### Detection Markers

**Required marker:**
- `**Story ID:** STORY-NNN` (where NNN is numeric)

**Exclusion:**
- No Epic Creation markers present
- No Sprint Planning markers present

### Entry Point

**Phase 0** - Checkpoint Detection (if resuming workflow)
**Phase 1** - Load and Validate Story (standard entry)

### Workflow Summary

This is the default orchestration mode for standard story lifecycle management:
1. Checkpoint Detection - Check for existing checkpoints (DEV_COMPLETE, QA_APPROVED, STAGING_COMPLETE)
2. Story Validation - Load and validate story file
3. Skill Invocation - Coordinate devforgeai-architecture, development, qa, release skills
4. Status Transitions - Update story status through 11 workflow states
5. Quality Gate Enforcement - Block transitions when requirements not met

### Behavior

**Automatic skill coordination** based on story status:
- Status = "Backlog" → Invoke devforgeai-architecture
- Status = "Architecture" → Validate context files
- Status = "Ready for Dev" → Invoke devforgeai-development
- Status = "Dev Complete" → Invoke devforgeai-qa
- Status = "QA Approved" → Invoke devforgeai-release

**Checkpoint resume:**
- DEV_COMPLETE checkpoint → Resume from QA phase
- QA_APPROVED checkpoint → Resume from Release (staging)
- STAGING_COMPLETE checkpoint → Resume from Release (production)

### Example Detection

```
Conversation contains:
**Story ID:** STORY-042

Result: Story Management Mode detected → Execute Phase 0/1 workflow
```

---

## Default Mode (No Mode Markers)

### When Triggered

**No explicit mode markers** found in conversation:
- No `**Command:** create-epic`
- No `**Command:** create-sprint`
- No `**Story ID:** STORY-NNN`

### Fallback Logic

The skill attempts to infer intent from conversation context:

```
IF conversation contains "epic" OR "Epic":
  → Assume story management mode (user wants to link story to epic)
  → Prompt user for **Story ID:** if not provided

IF conversation contains "sprint" OR "Sprint":
  → Assume story management mode (user wants to link story to sprint)
  → Prompt user for **Story ID:** if not provided

IF Story ID pattern detected (STORY-\d+):
  → Assume story management mode
  → Extract story ID from pattern

OTHERWISE:
  → HALT with clear error message
```

### Error Message

```
Cannot determine orchestration mode. Please set explicit context marker:

For Epic Creation:
  **Command:** create-epic
  **Epic name:** {name}

For Sprint Planning:
  **Command:** create-sprint
  **Sprint Name:** {name}
  **Selected Stories:** {story-ids}

For Story Management:
  **Story ID:** STORY-NNN
```

---

## Mode Priority

When multiple markers are present, the skill uses this priority order:

1. **Epic Creation Mode** (highest priority)
   - If `**Command:** create-epic` is found, ignore all other markers

2. **Sprint Planning Mode**
   - If `**Command:** create-sprint` is found AND no epic marker

3. **Story Management Mode**
   - If `**Story ID:**` is found AND no epic/sprint markers

4. **Default Mode** (lowest priority)
   - If no explicit markers found, attempt inference

---

## Troubleshooting

### Issue: Mode Detection Fails

**Symptom:** Skill defaults to wrong mode or HALTS with "Cannot determine mode"

**Cause:** Missing or malformed context markers

**Solution:**
1. Check for exact marker format: `**Marker Name:** value` (bold markdown, colon, space)
2. Verify required markers present for intended mode
3. Ensure no typos in marker names (case-sensitive)
4. Add explicit markers if relying on inference

### Issue: Wrong Mode Detected

**Symptom:** Epic mode triggered when Sprint mode intended

**Cause:** Multiple markers present, priority rules applied

**Solution:**
1. Remove conflicting markers (only include markers for intended mode)
2. Check marker priority order (Epic > Sprint > Story)
3. Use explicit `**Command:**` marker to force mode

### Issue: Story ID Not Detected

**Symptom:** Default mode triggered instead of Story Management mode

**Cause:** Story ID marker malformed or pattern not matched

**Solution:**
1. Use exact format: `**Story ID:** STORY-042`
2. Ensure "STORY-" prefix (case-sensitive)
3. Verify numeric suffix (e.g., STORY-042, not STORY-ABC)
4. Check for extra spaces or characters

---

## Implementation Notes

**How the skill performs detection:**

1. **Grep search** - Skill uses Grep tool to search conversation for markers:
   ```
   Grep(pattern="\\*\\*Command:\\*\\*\\s+create-epic")
   Grep(pattern="\\*\\*Epic name:\\*\\*")
   ```

2. **Pattern matching** - Extract values from marker patterns:
   ```
   Pattern: **Epic name:** User Authentication System
   Extract: "User Authentication System"
   ```

3. **Mode selection** - Based on priority order, select first matching mode

4. **Validation** - Verify all required markers for selected mode are present

5. **Execution** - Branch to appropriate phase/workflow

**Performance:**
- Mode detection completes in <50ms (typical)
- No LLM calls required (pattern matching only)
- Deterministic behavior (no ambiguity)

---

## Related Files

- **checkpoint-detection.md** - Phase 0 checkpoint recovery (Story Management mode)
- **epic-management.md** - Epic creation workflow details (Epic Creation mode)
- **sprint-planning-guide.md** - Sprint planning workflow details (Sprint Planning mode)
- **story-validation.md** - Story file validation (Story Management mode)
- **troubleshooting.md** - Common issues and solutions
