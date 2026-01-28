# Phase 04: Refactoring

**Entry Gate:**
```bash
devforgeai-validate phase-check ${STORY_ID} --from=03 --to=04

Examples (--project-root applies to phase-* commands only, not check-hooks/invoke-hooks):
 - Correct: devforgeai-validate phase-init ${STORY_ID} --project-root=.
 - Incorrect: python -m devforgeai.cli.devforgeai_validate phase-init ${STORY_ID} --project-root=.
# Exit code 0: Transition allowed
# Exit code 1: Phase 03 not complete - HALT
# Exit code 2: Missing subagents from Phase 03 - HALT
```

---

## Phase Workflow

**Purpose:** Improve quality while keeping tests green

**Required Subagents:**
- refactoring-specialist (Code improvement)
- code-reviewer (Quality review)
- test-automator (Coverage remediation - conditional)

**Steps:**

1. **Invoke refactoring specialist**
   ```
   Task(
     subagent_type="refactoring-specialist",
     description="Refactor implementation for ${STORY_ID}",
     prompt="""
     Improve code quality while maintaining test success.

     Story: ${STORY_FILE}
     Implementation files: ${IMPL_FILES}

     Requirements:
     1. Reduce cyclomatic complexity if > 10
     2. Extract reusable methods
     3. Improve naming consistency
     4. Apply DRY principle
     5. Ensure all tests still pass after changes

     **Response Constraints:**
     - Limit response to 500 words maximum
     - Use bullet points, not paragraphs
     - Only include actionable findings
     - No code snippets unless essential
     """
   )
   ```

---

### Step 2a: Early Coverage Validation (STORY-262)

**Purpose:** Catch coverage gaps 5+ phases earlier by validating coverage thresholds after refactoring completes, before code review. Enables targeted remediation tests to be injected before advancing to Phase 05.

**Entry Condition:** Step 1 (refactoring-specialist) completed successfully.

**Coverage Thresholds (ADR-010):**
| Layer | Threshold | Comparison |
|-------|-----------|------------|
| Business Logic | >= 95% | greater than or equal to 95% |
| Application | >= 85% | greater than or equal to 85% |
| Infrastructure | >= 80% | greater than or equal to 80% |

**Step 2a.1: Parse Coverage Report**

Parse coverage report and extract percentage values (0.0-100.0) for each architectural layer:

```bash
# Execute coverage command
${TEST_COVERAGE_COMMAND}

# Parse coverage percentage from report
# Extract: business_logic_coverage, application_coverage, infrastructure_coverage
```

**Coverage Parsing Logic (COMP-001):**
- Read coverage report output (pytest-cov, jest --coverage, dotnet test)
- Extract percentage values as floats (0.0-100.0)
- Map source files to layer classification using source-tree.md patterns
- Handle partial coverage data gracefully (log missing layers)

**Step 2a.2: Threshold Comparison**

Compare parsed coverage against thresholds using >= operator (COMP-003):

```
coverage_passed = true
gaps = []

IF business_logic_coverage >= 95:
    log("Business logic: PASS")
ELSE:
    coverage_passed = false
    gaps.append({layer: "business_logic", actual: business_logic_coverage, threshold: 95})

IF application_coverage >= 85:
    log("Application: PASS")
ELSE:
    coverage_passed = false
    gaps.append({layer: "application", actual: application_coverage, threshold: 85})

IF infrastructure_coverage >= 80:
    log("Infrastructure: PASS")
ELSE:
    coverage_passed = false
    gaps.append({layer: "infrastructure", actual: infrastructure_coverage, threshold: 80})
```

**Step 2a.3: Conditional Remediation or Normal Flow**

```
IF coverage_passed == true:
    # Proceed with normal threshold flow - all layers meet requirements
    Display: "Coverage thresholds met. Proceeding to code review (Step 3)."
    SKIP remediation
    GOTO Step 2b

ELSE:
    # Coverage below threshold - trigger remediation injection
    Display: "Coverage gaps detected. Initiating remediation cycle."
    GOTO Step 2a.4 (Remediation)
```

**Step 2a.4: Remediation Injection (Conditional)**

When any layer's coverage is below threshold, invoke test-automator in remediation mode:

```
remediation_cycle = 0
REMEDIATION_MAX_CYCLES = 2

WHILE coverage_passed == false AND remediation_cycle < REMEDIATION_MAX_CYCLES:
    remediation_cycle += 1

    Task(subagent_type="test-automator",
      description="Generate remediation tests for coverage gaps",
      prompt="""
      MODE: REMEDIATION
      REMEDIATION_MODE=true

      Generate targeted tests for coverage gaps identified:
      ${gaps}

      Requirements:
      1. Target ONLY files/lines below threshold
      2. Generate tests that increase coverage for identified gaps
      3. Follow test naming: test_<function>_<scenario>_<expected>
      4. Prioritize by gap size (largest gaps first)
      5. Maximum 20 files per remediation cycle

      Coverage Gaps:
      ${JSON.stringify(gaps)}
      """
    )

    # Re-execute coverage check after remediation
    ${TEST_COVERAGE_COMMAND}

    # Re-evaluate thresholds
    [Re-run Step 2a.2 threshold comparison]

    IF coverage_passed == true:
        Display: "Remediation successful. Coverage thresholds now met."
        BREAK

# After 2 remediation cycles, if still failing, HALT
IF coverage_passed == false AND remediation_cycle >= REMEDIATION_MAX_CYCLES:
    Display: "HALT: Coverage below threshold after 2 failed cycles. Manual intervention required."
    Display: "  - Business Logic: ${business_logic_coverage}% (threshold: 95%)"
    Display: "  - Application: ${application_coverage}% (threshold: 85%)"
    Display: "  - Infrastructure: ${infrastructure_coverage}% (threshold: 80%)"
    HALT workflow
```

**Step 2a.5: Log Coverage Observation**

Record coverage check results in phase-state.json observations:

```json
{
  "id": "obs-04-coverage",
  "phase": "04",
  "category": "coverage_check",
  "note": "Early coverage validation completed",
  "coverage_percentage": {
    "business_logic": 96.5,
    "application": 87.2,
    "infrastructure": 82.1
  },
  "coverage_observation": {
    "business_logic": {"actual": 96.5, "threshold": 95, "passed": true},
    "application": {"actual": 87.2, "threshold": 85, "passed": true},
    "infrastructure": {"actual": 82.1, "threshold": 80, "passed": true}
  },
  "remediation_triggered": false,
  "remediation_cycles": 0,
  "severity": "low"
}
```

**Graceful Fallback (COMP-005):**

If coverage tool fails or is unavailable:
```
IF coverage_command_exit_code != 0 OR coverage_tool_unavailable:
    Display: "Warning: Coverage tool unavailable or failed. Skipping early coverage check."
    log_observation({
      "id": "obs-04-coverage-skip",
      "phase": "04",
      "category": "coverage_check_skipped",
      "note": "Coverage check skipped - tool failure or unavailable",
      "fallback": "Deferred to Phase 05 validation",
      "severity": "medium"
    })
    # Proceed with fail-safe fallback to Phase 05
    GOTO Step 2b
```

**Exit Condition:** Coverage thresholds met OR graceful fallback activated. Proceed to Step 2b.

---

2b. **Verify tests still GREEN**
   ```bash
   ${TEST_COMMAND}
   # Expected: All tests still PASS
   ```

3. **Invoke code reviewer**
   ```
   Task(
     subagent_type="code-reviewer",
     description="Review code for ${STORY_ID}",
     prompt="""
     Review implementation quality and security.

     Files: ${IMPL_FILES}

     Check:
     1. Code quality and maintainability
     2. Security vulnerabilities
     3. Pattern compliance
     4. Standards adherence

     **Response Constraints:**
     - Limit response to 500 words maximum
     - Use bullet points, not paragraphs
     - Only include actionable findings
     - No code snippets unless essential
     """
   )
   ```

4. **Anti-Gaming Validation** [NEW - BLOCKING]
   - Check for skip decorators
   - Check for empty tests
   - Check for excessive mocking (>2x assertions)
   - HALT if gaming patterns detected

5. **Light QA validation** [MANDATORY]
   ```
   Skill(command="qa --mode=light --story=${STORY_ID}")
   ```

6. **Update AC Checklist (quality items)**
   ```
   Edit(
     file_path="${STORY_FILE}",
     old_string="- [ ] Quality item",
     new_string="- [x] Quality item"
   )
   ```

**Reference:** `references/tdd-refactor-phase.md` for complete workflow
    Read(file_path=".claude/skills/devforgeai-development/references/tdd-refactor-phase.md")

---

## Phase 04 Validation Checkpoint

**Before proceeding to Phase 05, verify:**

- [ ] refactoring-specialist invoked (check for Task() call in conversation)
- [ ] Early coverage validation performed (Step 2a executed)
- [ ] Coverage thresholds verified (95%/85%/80%) or graceful fallback activated
- [ ] code-reviewer invoked (check for Task() call in conversation)
- [ ] Anti-gaming validation passed
- [ ] Light QA validation passed (check for devforgeai-qa --mode=light)
- [ ] AC Checklist (quality items) updated ([ ] → [x])

**IF any checkbox UNCHECKED:** HALT workflow

### AC Checklist Update Verification (RCA-003)

After Step 6 completes, verify AC Checklist was actually updated:
```
Grep(pattern="- \\[x\\].*[Qq]uality", path="${STORY_FILE}")
# Should find checked quality-related items
# If no matches found: AC Checklist update was skipped - HALT
```

### Subagent Invocation Verification

FOR required_subagent in [refactoring-specialist, code-reviewer, Light QA]:
  IF conversation contains Task(subagent_type="{required_subagent}"):
    mark_verified(required_subagent)
  ELSE:
    add_to_missing(required_subagent)

IF any check fails:
  Display: "Phase 04 incomplete: {missing items}"
  HALT (do not proceed to Phase 05)
  Prompt: "Complete missing items before proceeding"

IF all checks pass:
  Display: "Phase 04 validation passed - all mandatory steps completed"
  Proceed to Phase 05

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
  "id": "obs-04-{seq}",
  "phase": "04",
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
devforgeai-validate phase-complete ${STORY_ID} --phase=04 --checkpoint-passed
# Exit code 0: Phase complete, proceed to Phase 05
# Exit code 1: Cannot complete - quality issues detected
```
