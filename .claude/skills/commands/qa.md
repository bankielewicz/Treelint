---
description: Run QA validation on story implementation
argument-hint: [STORY-ID] [mode]
# Mode: 'deep' or 'light' (no -- prefix)
model: opus
allowed-tools: AskUserQuestion, Read, Write, Edit, Glob, Grep, Skill, Bash
execution-mode: immediate
---

# /qa - Quality Assurance Validation Command

Execute QA validation on story implementation. Supports light validation during development and deep validation after completion.

Do not skip any phases nor skip the devforgeai-qa skill.

---

## Quick Reference

```bash
# Light validation during development (10K tokens, ~1 min)
/qa STORY-001 light

# Deep validation after implementation (65K tokens, ~5 min)
/qa STORY-001 deep

# Default mode (inferred from story status)
/qa STORY-001
```

---

## Command Workflow

### Phase 0: Plan Mode Detection and Argument Validation

**Step 0.0: Plan Mode Auto-Exit [execution-mode: immediate]**

This command has `execution-mode: immediate` in frontmatter. If plan mode is currently active, auto-exit plan mode before proceeding:

```
IF plan mode is active:
    Display: "Note: /qa is an execution command. Exiting plan mode automatically."
    ExitPlanMode()
```

---

**Step 0.1: Validate Project Root [MANDATORY]**

Before ANY Glob or Read operations:

```
result = Read(file_path="CLAUDE.md")

IF result.success AND content_contains("DevForgeAI"):
    Display: "✓ Project root validated"
ELSE:
    Display: "❌ Not in DevForgeAI project root"
    Display: "   Expected: CLAUDE.md with DevForgeAI configuration"
    HALT: Use AskUserQuestion to get correct path or ask user to navigate
```

**Proceed to argument validation only after CWD check passes.**

---

**Validate story ID format:**
```
IF $1 is empty OR does NOT match pattern "STORY-[0-9]+":
  AskUserQuestion:
    Question: "Story ID '$1' doesn't match format. What story should I validate?"
    Header: "Story ID"
    Options:
      - "List stories in Dev Complete status"
      - "List stories in In Development status"
      - "Show /qa command syntax"
    multiSelect: false

  Extract STORY_ID from user response OR exit if cancelled
```

**Validate story file exists:**
```
Glob(pattern="devforgeai/specs/Stories/${STORY_ID}*.story.md")

IF no matches found:
  AskUserQuestion:
    Question: "Story ${STORY_ID} not found. What should I do?"
    Header: "Story not found"
    Options:
      - "List all available stories"
      - "Cancel command"
    multiSelect: false
```

**Load story via @file reference:**
```
@devforgeai/specs/Stories/{STORY_ID}.story.md

(Story content now available in conversation context)
```

**Parse validation mode:**
```
IF $2 provided:
  IF $2 in ["deep", "light"]:
    MODE = $2

  ELSE IF $2 starts with "--mode=":
    EXTRACTED_MODE = substring after "--mode="
    IF EXTRACTED_MODE in ["deep", "light"]:
      MODE = EXTRACTED_MODE
      Note: "Positional argument preferred: /qa STORY-001 deep"
    ELSE:
      AskUserQuestion for valid mode

  ELSE:
    AskUserQuestion for valid mode (unknown value provided)

ELSE:
  # Infer from story status
  Extract story status from YAML frontmatter

  IF status == "Dev Complete":
    MODE = "deep"
  ELSE IF status == "In Development":
    MODE = "light"
  ELSE:
    AskUserQuestion for mode selection
```

**Validation summary:**
```
✓ Story ID: ${STORY_ID}
✓ Validation mode: ${MODE}
✓ Proceeding with QA validation...
```

---

### Phase 1: Invoke QA Skill

Set explicit context markers for skill:

```
**Story ID:** ${STORY_ID}
**Validation Mode:** ${MODE}

Invoke skill:
Skill(command="devforgeai-qa")
```

**After skill invocation:**
- Skill's SKILL.md content expands inline in conversation
- **YOU execute the skill's workflow phases** (not waiting for external result)
- Follow the skill's instructions phase by phase
- Produce output as skill instructs

**The skill instructs you to:**
- Extract story from conversation context (YAML frontmatter)
- Execute mode-specific validation (light or deep)
- Generate QA report in devforgeai/qa/reports/
- Invoke qa-result-interpreter subagent to format results
- Return structured result summary with display template, violations, next steps

---

### Phase 2: Display Results

Receive result from skill and output display:

```
# Skill returns structured result object with:
# - display.template (formatted QA results)
# - next_steps (recommendations)
# - hook_status (triggered/skipped/failed from Phase 6)
# - story_update (completed/skipped from Phase 7)

# Command simply outputs what skill prepared
output result.display.template as-is

# Display next steps
Display: ""
Display: result.next_steps
```

**Note:** The devforgeai-qa skill now handles:
- Phase 6: Feedback hook invocation (non-blocking)
- Phase 7: Story file updates (deep mode pass only)

See `.claude/skills/devforgeai-qa/SKILL.md` and reference files for implementation details.

---

## Error Handling

### Story ID Invalid
```
Error: Story ID format error
Usage: /qa STORY-001 [mode]
Example: /qa STORY-001 deep
```

### Story File Not Found
```
Error: Story file not found
Path: devforgeai/specs/Stories/{STORY_ID}.story.md

Available stories:
[List from Glob result]

Action: Verify story ID and try again
```

### Invalid Mode Specified
```
Error: Invalid validation mode: {PROVIDED_MODE}

Valid modes:
- light: Quick validation during development
- deep: Comprehensive validation after completion

Usage: /qa STORY-001 [light|deep]
```

### QA Skill Execution Failed
```
Error: QA validation failed to complete

Story: {STORY_ID}
Mode: {MODE}

Skill output:
{Display skill error output above}

Actions:
1. Check context files exist: ls devforgeai/specs/context/
2. Verify build: [language-specific build command]
3. Verify tests: [language-specific test command]
4. Try again: /qa {STORY_ID} {MODE}
```

---

## Success Criteria

- [ ] Story ID validated and file found
- [ ] Validation mode determined (explicit or inferred)
- [ ] QA skill invoked successfully
- [ ] Skill returns valid result structure
- [ ] Display template generated correctly
- [ ] Results displayed to user
- [ ] Next steps recommended based on result
- [ ] Story file updated (deep mode pass only)
- [ ] QA Validation History section added (deep mode pass only)
- [ ] Status updated to "QA Approved" (deep mode pass only)
- [ ] Token usage within budget (<15K for light, <70K for deep)

---

## Integration with Framework

**Invoked by:**
- Developer: `/qa STORY-ID` or `/qa STORY-ID mode`
- devforgeai-development skill (light mode after implementation phases)
- devforgeai-orchestration skill (deep mode for quality gates)

**Invokes:**
- devforgeai-qa skill (actual validation logic)
- qa-result-interpreter subagent (via skill, for result display)

**Result handling:**
- Light mode pass: No status change, continue development
- Light mode fail: Block development, fix required
- Deep mode pass: Status → "QA Approved", ready for release
- Deep mode fail: Status → "QA Failed", return to development

**Quality gates affected:**
- Gate 2 (Test Passing): Light QA enforces
- Gate 3 (QA Approval): Deep QA enforces ← This command

---

## Related Commands

- `/dev STORY-ID` - Return to development after QA failure
- `/release STORY-ID` - Deploy QA-approved story
- `/board` - View sprint progress
- `/orchestrate STORY-ID` - Full lifecycle (dev → qa → release)

---

## Performance Targets

**Light Mode:**
- Execution time: <2 minutes
- Token usage: <15K
- Result summary: 8-12 lines

**Deep Mode:**
- Execution time: <5 minutes
- Token usage: <70K
- Result summary: 40-80 lines

---

## Implementation Notes

**Architecture (Refactored 2025-11-05, Enhanced 2025-11-14 - STORY-034):**

This command follows **lean orchestration** pattern:
- **Argument validation:** Minimal (30 lines)
- **Story loading:** Via @file reference
- **Skill invocation:** Simple (1 line)
- **Result display:** Pass-through (no parsing)
- **Error handling:** Minimal (skill communicates errors)

**Total: 307 lines (vs 509 before STORY-034) = 39.7% reduction**

**Business logic location:**
- Validation logic: devforgeai-qa skill (Phases 1-5)
- Result interpretation: qa-result-interpreter subagent (Phase 5 Step 6)
- Display generation: qa-result-interpreter subagent
- Feedback hooks: devforgeai-qa skill (Phase 6)
- Story file updates: devforgeai-qa skill (Phase 7)
- Command: Pure orchestration (validate → invoke → display)

**Token efficiency:**
- Command overhead: ~2.5K tokens (down from ~8K)
- Skill validation: ~15K (light) or ~65K (deep) in isolated context
- Main conversation: ~3K (display only, no parsing)
- **Savings vs pre-refactoring: 69%**

**STORY-034 Refactoring (2025-11-14):**
- Moved Phase 4 (feedback hooks) from command to skill as Phase 6
- Moved Phase 5 (story updates) from command to skill as Phase 7
- Created 2 reference files (feedback-hooks-workflow.md, story-update-workflow.md)
- Reduced command from 509 → 307 lines (40% reduction)
- Reduced character budget from 13,775 → 8,172 chars (41% reduction)
- Achieved 100% lean orchestration pattern compliance
- Command now has exactly 3 phases (Phase 0, 1, 2)

