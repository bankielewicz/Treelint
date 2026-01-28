# Phase 10: Result Interpretation

**Entry Gate:**
```bash
devforgeai-validate phase-check ${STORY_ID} --from=09 --to=10

Examples (--project-root applies to phase-* commands only, not check-hooks/invoke-hooks):
 - Correct: devforgeai-validate phase-init ${STORY_ID} --project-root=.
 - Incorrect: python -m devforgeai.cli.devforgeai_validate phase-init ${STORY_ID} --project-root=.
# Exit code 0: Transition allowed
# Exit code 1: Phase 09 not complete - HALT
```

---

## Phase Workflow

**Purpose:** Generate user-facing display template and structured result summary

**Required Subagents:**
- dev-result-interpreter (Result formatting)

**Execution:** After Phase 09, before returning to /dev command

**Steps:**

a. **Invoke dev-result-interpreter Subagent**
   ```
   Task(
     subagent_type="dev-result-interpreter",
     description="Interpret dev workflow results for ${STORY_ID}",
     prompt="""
     Interpret development workflow results for story ${STORY_ID}.

     Story file: ${STORY_FILE}

     Task:
     1. Read story file and extract:
        - Current status (from YAML frontmatter)
        - TDD phases completed (from Status History or Implementation Notes)
        - Test results (passing count, coverage %)
        - DoD completion status (from Implementation Notes)
        - Deferred items (from Implementation Notes)

     2. Determine overall result:
        - SUCCESS: status="Dev Complete", all tests passing
        - INCOMPLETE: status="In Development", some work remaining
        - FAILURE: workflow error or blocking issue

     3. Generate display template appropriate for result type

     4. Provide next step recommendations based on story state

     Return structured JSON with:
     - status: "success|incomplete|failure"
     - display.template: "..." (formatted display text)
     - display.next_steps: [...] (actionable recommendations)
     - story_status: "..." (current story status)
     - tdd_phases_completed: [...] (phases finished)
     - workflow_summary: "..." (brief summary)
     """
   )
   ```

b. **Receive Structured Result**
   ```json
   {
     "status": "success",
     "display": {
       "template": "╔═══════════...║  DEVELOPMENT COMPLETE ✅  ║...",
       "next_steps": [
         "Run QA validation: /qa ${STORY_ID}",
         "Or run full orchestration: /orchestrate ${STORY_ID}",
         "Review implementation: Read story file Implementation Notes"
       ]
     },
     "story_status": "Dev Complete",
     "tdd_phases_completed": ["Phase 01", "Phase 02", "Phase 03", "Phase 04", "Phase 05", "Phase 06", "Phase 07", "Phase 08", "Phase 09", "Phase 10"],
     "workflow_summary": "TDD cycle complete, X/Y tests passing (Z%)"
   }
   ```

c. **Return Result to Command**
   - Skill returns the result object to /dev command
   - Command displays result.display.template
   - No further processing needed in skill

**Reference:** `references/dev-result-formatting-guide.md`
    Read(file_path=".claude/skills/devforgeai-development/references/dev-result-formatting-guide.md")

---

## Progress Indicator

Display at start of Phase 10:
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 10/10: Result Interpretation (95% → 100% complete)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

## Validation Checkpoint

**Before returning to /dev command, verify:**

- [ ] dev-result-interpreter subagent invoked
- [ ] Structured result returned to command

**IF any checkbox UNCHECKED:** HALT - Cannot complete workflow without result interpretation

---

**Exit Gate:**
```bash
devforgeai-validate phase-complete ${STORY_ID} --phase=10 --checkpoint-passed
# Exit code 0: Workflow complete, return result to command
# Exit code 1: Cannot complete - result interpretation failed
```

**After Phase 10:**
```bash
# Archive the completed phase state file
devforgeai-validate phase-archive ${STORY_ID}
# Moves state file to devforgeai/workflows/completed/
```

---

## Integration Notes

- dev-result-interpreter operates in isolated context (8K tokens max)
- Reference guide provides framework constraints
- Subagent generates templates matching original /dev output format
- No business logic in command (follows lean orchestration pattern)

**Benefits:**
- 184 lines extracted from /dev command
- Token efficiency: Display generation in isolated subagent context
- Maintainability: Single source of truth for display templates
- Reusability: Pattern applicable to other development commands
