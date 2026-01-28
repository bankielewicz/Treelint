# Phase 03: Implementation (TDD Green)

**Entry Gate:**
```bash
devforgeai-validate phase-check ${STORY_ID} --from=02 --to=03

Examples (--project-root applies to phase-* commands only, not check-hooks/invoke-hooks):
 - Correct: devforgeai-validate phase-init ${STORY_ID} --project-root=.
 - Incorrect: python -m devforgeai.cli.devforgeai_validate phase-init ${STORY_ID} --project-root=.
# Exit code 0: Transition allowed
# Exit code 1: Phase 02 not complete - HALT
# Exit code 2: Missing subagents from Phase 02 - HALT
```

---

## Phase Workflow

**Purpose:** Minimal code to pass tests (Tests GREEN)

**Required Subagents:**
- backend-architect OR frontend-developer (Implementation)
- context-validator (Constraint validation)

**Steps:**

1. **Determine implementation subagent**
   - If backend story → backend-architect
   - If frontend story → frontend-developer
   - If full-stack → invoke both sequentially

2. **Implement minimal code to pass tests**
   ```
   Task(
     subagent_type="backend-architect",  # or frontend-developer
     description="Implement code for ${STORY_ID}",
     prompt="""
     Write minimal implementation to pass tests.

     Story: ${STORY_FILE}
     Test files: ${TEST_FILES}

     Requirements:
     1. Implement ONLY what tests require
     2. Follow tech-stack.md constraints
     3. Follow coding-standards.md patterns
     4. Follow source-tree.md for file placement
     5. No premature optimization

     **Response Constraints:**
     - Limit response to 500 words maximum
     - Use bullet points, not paragraphs
     - Only include actionable findings
     - No code snippets unless essential
     """
   )
   ```

3. **Run tests - verify GREEN state**
   ```bash
   ${TEST_COMMAND}
   # Expected: All tests PASS (green phase)
   ```

4. **Validate context constraints**
   ```
   Task(
     subagent_type="context-validator",
     description="Validate constraints for ${STORY_ID}",
     prompt="""
     Validate implementation against all 6 context files.

     **Response Constraints:**
     - Limit response to 500 words maximum
     - Use bullet points, not paragraphs
     - Only include actionable findings
     - No code snippets unless essential
     """
   )
   ```

5. **Update AC Checklist (implementation items)**
   ```
   Edit(
     file_path="${STORY_FILE}",
     old_string="- [ ] Implementation item",
     new_string="- [x] Implementation item"
   )
   ```

**Reference:** `references/tdd-green-phase.md` for complete workflow
    Read(file_path=".claude/skills/devforgeai-development/references/tdd-green-phase.md")

---

## Phase 03 Validation Checkpoint

**Before proceeding to Phase 04, verify:**

- [ ] backend-architect OR frontend-developer invoked (check for Task() call in conversation)
- [ ] All tests GREEN (passing)
- [ ] context-validator invoked (check for Task() call in conversation)
- [ ] AC Checklist (implementation items) updated ([ ] → [x])

**IF any checkbox UNCHECKED:** HALT workflow

### AC Checklist Update Verification (RCA-003)

After Step 5 completes, verify AC Checklist was actually updated:
```
Grep(pattern="- \\[x\\].*[Ii]mplementation", path="${STORY_FILE}")
# Should find checked implementation-related items
# If no matches found: AC Checklist update was skipped - HALT
```

### Subagent Invocation Verification

FOR required_subagent in [backend-architect OR frontend-developer, context-validator]:
  IF conversation contains Task(subagent_type="{required_subagent}"):
    mark_verified(required_subagent)
  ELSE:
    add_to_missing(required_subagent)

IF any check fails:
  Display: "Phase 03 incomplete: {missing items}"
  HALT (do not proceed to Phase 04)
  Prompt: "Complete missing items before proceeding"

IF all checks pass:
  Display: "Phase 03 validation passed - all mandatory steps completed"
  Proceed to Phase 04

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
  "id": "obs-03-{seq}",
  "phase": "03",
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
devforgeai-validate phase-complete ${STORY_ID} --phase=03 --checkpoint-passed
# Exit code 0: Phase complete, proceed to Phase 04
# Exit code 1: Cannot complete - tests not GREEN
```
