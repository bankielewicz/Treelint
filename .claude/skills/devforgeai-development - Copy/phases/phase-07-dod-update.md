# Phase 07: DoD Update Workflow

**Entry Gate:**
```bash
devforgeai-validate phase-check ${STORY_ID} --from=06 --to=07

Examples (--project-root applies to phase-* commands only, not check-hooks/invoke-hooks):
 - Correct: devforgeai-validate phase-init ${STORY_ID} --project-root=.
 - Incorrect: python -m devforgeai.cli.devforgeai_validate phase-init ${STORY_ID} --project-root=.
# Exit code 0: Transition allowed
# Exit code 1: Phase 06 not complete - HALT
# Exit code 2: Missing subagents from Phase 06 - HALT
```

---

## Phase Workflow

**Purpose:** Update DoD format for git commit - prepare documentation

**Required Subagents:** None (file operations only)

**Pre-Check: Implementation Notes Section**
```
Grep(pattern="^## Implementation Notes", path="${STORY_FILE}")

IF NOT found:
  # Auto-create section before Workflow Status
  Edit(
    file_path="${STORY_FILE}",
    old_string="## Workflow Status",
    new_string="## Implementation Notes\n\n**Developer:** DevForgeAI AI Agent\n**Implemented:** ${CURRENT_DATE}\n\n## Workflow Status"
  )
```

**Steps:**

1. **Mark completed items [x] in Definition of Done section**
   ```
   Edit(
     file_path="${STORY_FILE}",
     old_string="- [ ] {completed_item}",
     new_string="- [x] {completed_item}"
   )
   ```

2. **Add DoD items to Implementation Notes**
   - CRITICAL: Use FLAT LIST format
   - NO ### subsections under Implementation Notes
   ```
   Edit(
     file_path="${STORY_FILE}",
     old_string="## Implementation Notes\n\n**Developer:**",
     new_string="## Implementation Notes\n\n- [x] DoD item 1 completed\n- [x] DoD item 2 completed\n- [ ] DoD item 3 (DEFERRED: reason)\n\n**Developer:**"
   )
   ```

3. **Validate DoD format**
   ```bash
   devforgeai validate-dod ${STORY_FILE}
   # Exit code 0: Format valid
   # Exit code 1: Format invalid - fix and retry
   ```

4. **Update Workflow Status section**
   ```
   Edit(
     file_path="${STORY_FILE}",
     old_string="status: In Development",
     new_string="status: Dev Complete"
   )
   ```

5. **Final validation**
   ```bash
   devforgeai validate-dod ${STORY_FILE}
   # Exit code 0 required before proceeding
   ```

**Reference:** `references/dod-update-workflow.md` for complete workflow
    Read(file_path=".claude/skills/devforgeai-development/references/dod-update-workflow.md")

---

## Format Requirements

**CORRECT Implementation Notes Format:**
```markdown
## Implementation Notes

- [x] Unit tests written and passing
- [x] Implementation complete
- [x] Code review completed
- [ ] Performance testing (DEFERRED: infrastructure not ready)

**Developer:** DevForgeAI AI Agent
**Implemented:** 2025-12-25

### TDD Workflow Summary (optional subsection OK)
...
```

**INCORRECT Format (will fail validation):**
```markdown
## Implementation Notes

### Definition of Done - Completed Items  ← NO! This subsection causes failures
- [x] Unit tests...
```

---

## Validation Checkpoint

**Before proceeding to Phase 08, verify:**

- [ ] DoD items marked [x] in story file
- [ ] Implementation Notes flat list added (no ### subsections)
- [ ] DoD format validated (exit code 0)
- [ ] Workflow Status updated

**IF any checkbox UNCHECKED:** HALT - Git commit will FAIL without proper DoD

---

## Observation Capture

**Before exiting this phase, reflect:**
1. Did I encounter any friction? (unclear docs, missing tools, workarounds)
2. Did anything work particularly well? (constraints that helped, patterns that fit)
3. Did I notice any repeated patterns?
4. Are there gaps in tooling/docs?
5. Did I discover any bugs?

**If YES to any:** Append to phase-state.json `observations` array:
```json
{
  "id": "obs-07-{seq}",
  "phase": "07",
  "category": "{friction|success|pattern|gap|idea|bug}",
  "note": "{1-2 sentence description}",
  "files": ["{relevant files}"],
  "severity": "{low|medium|high}"
}
```

**Reference:** `references/observation-capture.md`
    Read(file_path=".claude/skills/devforgeai-development/references/observation-capture.md")

---

**Exit Gate:**
```bash
devforgeai-validate phase-complete ${STORY_ID} --phase=07 --checkpoint-passed
# Exit code 0: Phase complete, proceed to Phase 08
# Exit code 1: Cannot complete - DoD format invalid
```
