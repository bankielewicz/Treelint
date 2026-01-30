# Phase 05: Integration & Validation

**Entry Gate:**
```bash
devforgeai-validate phase-check ${STORY_ID} --from=4.5 --to=05

Examples (--project-root applies to phase-* commands only, not check-hooks/invoke-hooks):
 - Correct: devforgeai-validate phase-init ${STORY_ID} --project-root=.
 - Incorrect: python -m devforgeai.cli.devforgeai_validate phase-init ${STORY_ID} --project-root=.
# Exit code 0: Transition allowed
# Exit code 1: Phase 4.5 not complete - HALT
# Exit code 2: Missing subagents from Phase 4.5 - HALT
```

---

## Phase Workflow

**Purpose:** Cross-component testing and coverage validation

**Required Subagents:**
- integration-tester (Integration tests)

**Steps:**

0. **Anti-Gaming Validation** [MANDATORY - RUN FIRST]
   - Check for skip decorators
   - Check for assertion-less tests
   - Check for excessive mocking
   - HALT if gaming patterns detected (coverage scores would be invalid)

1. **Invoke integration tester**
   ```
   Task(
     subagent_type="integration-tester",
     description="Run integration tests for ${STORY_ID}",
     prompt="""
     Validate cross-component interactions.

     Story: ${STORY_FILE}
     Implementation: ${IMPL_FILES}

     Requirements:
     1. Test API contracts if applicable
     2. Test database transactions if applicable
     3. Test message flows if applicable
     4. Verify coverage thresholds met
        - Business logic: 95%
        - Application layer: 85%
        - Infrastructure: 80%

     **Response Constraints:**
     - Limit response to 500 words maximum
     - Use bullet points, not paragraphs
     - Only include actionable findings
     - No code snippets unless essential
     """
   )
   ```

2. **Validate coverage thresholds**
   ```bash
   # Run coverage analysis
   ${COVERAGE_COMMAND}
   # Verify: 95%/85%/80% thresholds met
   ```

3. **Update AC Checklist (integration items)**
   ```
   Edit(
     file_path="${STORY_FILE}",
     old_string="- [ ] Integration item",
     new_string="- [x] Integration item"
   )
   ```

**Reference:** `references/integration-testing.md` for complete workflow
    Read(file_path=".claude/skills/devforgeai-development/references/integration-testing.md")

---

## Phase 05 Validation Checkpoint

**Before proceeding to Phase 06, verify:**

- [ ] Anti-gaming validation PASSED
- [ ] integration-tester subagent invoked (check for Task() call in conversation)
- [ ] Coverage thresholds validated (95%/85%/80%)
- [ ] AC Checklist (integration items) updated ([ ] → [x])

### AC Checklist Update Verification (RCA-003)

After Step 3 completes, verify AC Checklist was actually updated:
```
Grep(pattern="- \\[x\\].*[Ii]ntegration", path="${STORY_FILE}")
# Should find checked integration-related items
# If no matches found: AC Checklist update was skipped - HALT
```

**IF Anti-Gaming validation FAILED:**
- HALT immediately
- Test gaming detected, coverage scores INVALID
- Fix: Remove skip decorators, add assertions, reduce mocking

**IF any other checkbox UNCHECKED:** HALT workflow

### Subagent Invocation Verification

FOR required_subagent in [integration-tester]:
  IF conversation contains Task(subagent_type="{required_subagent}"):
    mark_verified(required_subagent)
  ELSE:
    add_to_missing(required_subagent)

IF any check fails:
  Display: "Phase 05 incomplete: {missing items}"
  HALT (do not proceed to Phase 06)
  Prompt: "Complete missing items before proceeding"

IF all checks pass:
  Display: "Phase 05 validation passed - all mandatory steps completed"
  Proceed to Phase 06

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
  "id": "obs-05-{seq}",
  "phase": "05",
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
devforgeai-validate phase-complete ${STORY_ID} --phase=05 --checkpoint-passed
# Exit code 0: Phase complete, proceed to Phase 5.5
# Exit code 1: Cannot complete - coverage thresholds not met
```
