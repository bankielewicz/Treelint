# Phase 02: Test-First Design (TDD Red)

**Entry Gate:**
```bash
devforgeai-validate phase-check ${STORY_ID} --from=01 --to=02

Examples (--project-root applies to phase-* commands only, not check-hooks/invoke-hooks):
 - Correct: devforgeai-validate phase-init ${STORY_ID} --project-root=.
 - Incorrect: python -m devforgeai.cli.devforgeai_validate phase-init ${STORY_ID} --project-root=.
# Exit code 0: Transition allowed
# Exit code 1: Phase 01 not complete - HALT
# Exit code 2: Missing subagents from Phase 01 - HALT
```

---

## Phase Workflow

**Purpose:** Write failing tests from acceptance criteria

**Required Subagents:**
- test-automator (Test generation)

**Steps:**

1. **Generate failing tests from AC**
   ```
   Task(
     subagent_type="test-automator",
     description="Generate failing tests for ${STORY_ID}",
     prompt="""
     Generate failing tests from acceptance criteria.

     Story: ${STORY_FILE}

     Requirements:
     1. Read story file acceptance criteria
     2. Generate tests that will FAIL initially
     3. Follow test naming: test_<function>_<scenario>_<expected>
     4. Use project's test framework (from tech-stack.md)
     5. Return test files and run command

     **Response Constraints:**
     - Limit response to 500 words maximum
     - Use bullet points, not paragraphs
     - Only include actionable findings
     - No code snippets unless essential
     """
   )
   ```

1.5. **Distinguish Test Output Based on Implementation Type**
   ```
   # Determine output type based on story implementation
   IF story modifies Slash Command (.md files):
       output_type = "Test Specification Document"
       Display: "Test Specification Generated for Slash Command"
       # Note: Specification validates structure, not executable

   ELIF story modifies Code (Python/JS/C#/etc):
       output_type = "Executable unit tests"
       Display: "Executable Tests Generated for Code implementation"
   ```

2. **Run tests - verify RED state**
   ```bash
   # Run generated tests
   ${TEST_COMMAND}
   # Expected: All tests FAIL (red phase)
   ```

3. **Verify tests fail for expected reasons**
   - Not import errors
   - Not configuration errors
   - Failures are business logic (expected)

4. **Tech Spec Coverage Validation**
   - Verify all technical spec sections have tests
   - User approval if gaps detected

5. **Update AC Checklist (test items)**
   ```
   Edit(
     file_path="${STORY_FILE}",
     old_string="- [ ] Test item",
     new_string="- [x] Test item"
   )
   ```

**Reference:** `references/tdd-red-phase.md` for complete workflow
    Read(file_path=".claude/skills/devforgeai-development/references/tdd-red-phase.md")

---

## Validation Checkpoint

**Before proceeding to Phase 03, verify:**

- [ ] test-automator subagent invoked
- [ ] Tech Spec Coverage Validation completed
- [ ] AC Checklist (test items) updated ([ ] → [x])

**IF any checkbox UNCHECKED:** HALT workflow

### AC Checklist Update Verification (RCA-003)

After Step 5 completes, verify AC Checklist was actually updated:
```
Grep(pattern="- \\[x\\].*test", path="${STORY_FILE}")
# Should find checked test-related items
# If no matches found: AC Checklist update was skipped - HALT
```

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
  "id": "obs-02-{seq}",
  "phase": "02",
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
devforgeai-validate phase-complete ${STORY_ID} --phase=02 --checkpoint-passed
# Exit code 0: Phase complete, proceed to Phase 03
# Exit code 1: Cannot complete - tests not in RED state
```
