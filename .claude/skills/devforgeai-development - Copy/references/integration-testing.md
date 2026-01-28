# Phase 05: Integration & Validation

**Purpose:** Create and execute integration tests for cross-component interactions.

**Execution Order:** After Phase 04 (Refactor phase) - code quality improved

**Expected Outcome:** All tests GREEN, coverage thresholds met, build succeeds

**Token Cost:** ~1,000 tokens in skill context (~40,000 in isolated subagent context)

---

## Phase Progress Indicator

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 05/9: Integration Testing (44% → 56% complete)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Display this indicator at the start of Phase 05 execution.**

---

## Overview

Phase 05 validates that the implementation integrates correctly with other components and meets quality thresholds.

**Core Principle:** Verify system-level behavior, not just unit behavior.

---

## Story Type Skip Check [MANDATORY FIRST] (STORY-126)

**Purpose:** Skip Phase 05 for `documentation` story types (no runtime code).

**When to execute:** Before any Phase 05 processing.

```
# Check if Phase 05 should be skipped based on story type
# $STORY_TYPE set in Phase 01.6.5 (Pre-Flight Validation)

IF $STORY_TYPE == "documentation":
    Display: ""
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: "  ℹ️  SKIPPING PHASE 05: Story Type 'documentation'"
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: ""
    Display: "Reason: Documentation stories have no runtime code to test."
    Display: "        Integration tests validate cross-component behavior,"
    Display: "        which doesn't exist for documentation-only changes."
    Display: ""
    Display: "Proceeding to Phase 06 (Deferral Challenge)..."
    Display: ""

    # Skip Phase 05 entirely
    GOTO Phase 06

    RETURN
```

### Phase Skip Decision Matrix

| Story Type | Phase 02 (Red) | Phase 04 (Refactor) | Phase 05 (Integration) |
|------------|----------------|---------------------|------------------------|
| `feature` | Execute | Execute | Execute |
| `documentation` | Execute | Execute | **SKIP** |
| `bugfix` | Execute | **SKIP** | Execute |
| `refactor` | **SKIP** | Execute | Execute |

**Reference:** `.claude/skills/devforgeai-story-creation/references/story-type-classification.md`

---

## Phase 05: Integration & Validation

**Delegate integration testing to integration-tester subagent.**

### Phase 05.0: Anti-Gaming Validation [MANDATORY - RUN FIRST - NEW]

**Purpose:** Prevent coverage gaming BEFORE test execution.

**CRITICAL:** This step MUST complete BEFORE running any tests. Coverage metrics are meaningless if tests are gamed.

#### 0.1 Why This Runs First

Coverage metrics are only valid if tests are authentic:
- **Skipped tests** don't run but aren't counted as failures → inflates pass rate
- **Empty tests** pass automatically without validating anything → inflates coverage
- **TODO placeholders** indicate incomplete test stubs → inflates test count
- **Over-mocked tests** don't test real behavior → fake coverage

By validating BEFORE test execution, we ensure coverage scores reflect real testing quality.

#### 0.2 Execute Gaming Detection via integration-tester

The integration-tester subagent (invoked in Step 1 below) now includes Phase 05.0gaming validation FIRST.

**Gaming detection happens automatically inside integration-tester subagent:**

```
gaming_scan = integration_tester.step_0_gaming_validation()

IF gaming_scan.status == "BLOCKED":
    Display:
    ```
    ══════════════════════════════════════════════════════
    🚨 PHASE 4 BLOCKED - TEST GAMING DETECTED
    ══════════════════════════════════════════════════════
    Cannot calculate coverage with gaming patterns present.

    Violations found:
    - Skip Decorators: {count} files
    - Empty Tests: {count} files
    - TODO Placeholders: {count} files
    - Excessive Mocking: {count} files

    See violation details below.
    ══════════════════════════════════════════════════════
    ```

    FOR each violation in gaming_scan.violations:
        Display: "  {violation.type}: {violation.files}"

    Display: ""
    Display: "ACTION REQUIRED:"
    Display: "1. Remove all @skip/@Ignore decorators"
    Display: "2. Add real assertions to empty tests"
    Display: "3. Implement TODO/FIXME placeholders"
    Display: "4. Reduce mock usage to ≤2× test count"
    Display: ""

    HALT: "Phase 05 cannot proceed. Test gaming invalidates coverage metrics."
    DO NOT execute Step 1 (integration testing)
```

#### 0.3 Proceed Only on PASS

```
IF gaming_scan.status == "PASS":
    Display: "✓ Anti-gaming validation passed - coverage will be authentic"
    Display: "  - No skip decorators detected"
    Display: "  - No empty tests detected"
    Display: "  - No TODO placeholders in tests"
    Display: "  - Mock ratios acceptable (≤2× test count)"
    Display: ""
    Display: "Proceeding to Step 1 (Integration Testing)..."

    PROCEED to Step 1
```

**Integration:** The integration-tester subagent runs Phase 05.0automatically as its first action. No separate invocation needed.

---

### Step 1: Invoke integration-tester Subagent [MANDATORY]

**Story-Scoped Test Outputs (STORY-092):**
```
# Get story-scoped paths from Phase 0.5 (test_isolation_paths)
results_dir = "tests/results/{STORY_ID}"   # e.g., tests/results/STORY-092
coverage_dir = "tests/coverage/{STORY_ID}" # e.g., tests/coverage/STORY-092
```

```
Task(
  subagent_type="integration-tester",
  description="Run full test suite with coverage",
  prompt="Execute comprehensive integration testing and validation.

  Implementation and tests available in conversation.

  **Story-Scoped Output Paths (STORY-092):**
  - Results directory: {results_dir}
  - Coverage directory: {coverage_dir}

  Test execution (use story-scoped paths):
  1. Run full test suite with story-scoped outputs:
     - Python: pytest --cov=src --cov-report=json:{coverage_dir}/coverage.json --junitxml={results_dir}/test-results.xml
     - Node.js: npm test -- --coverage --coverageDirectory={coverage_dir}
     - .NET: dotnet test --results-directory={results_dir}
     - Go: go test ./... -coverprofile={coverage_dir}/coverage.out
  2. Validate coverage meets thresholds:
     - Business logic: 95% minimum
     - Application layer: 85% minimum
     - Infrastructure: 80% minimum
  3. Check for regressions (existing tests still pass)
  4. Validate build succeeds: {BUILD_COMMAND}
  5. Run linter if available
  6. Check for integration issues (cross-component interactions)

  Context files:
  - devforgeai/specs/context/coding-standards.md (coverage requirements)
  - devforgeai/specs/context/architecture-constraints.md (layer boundaries)
  - devforgeai/config/test-isolation.yaml (test output path configuration)

  Return:
  - Test results (total, passed, failed, coverage %)
  - Coverage by layer (business/application/infrastructure)
  - Build status (success/failure)
  - Linter issues (if applicable)
  - Integration issues found
  - Test output paths (for concurrent story verification)"
)
```

### Step 2: Parse Subagent Response [MANDATORY]

```javascript
result = extract_from_subagent_output(response)

test_results = result["test_results"]
coverage = result["coverage"]
build_status = result["build_status"]

Display: "✓ Phase 05 (Integration): Full validation by integration-tester"
Display: "  - Tests: {test_results['passed']}/{test_results['total']} passing"
Display: "  - Coverage: {coverage['overall']}%"
Display: "    • Business logic: {coverage['business']}%"
Display: "    • Application: {coverage['application']}%"
Display: "    • Infrastructure: {coverage['infrastructure']}%"
Display: "  - Build: {build_status}"

# Validate coverage thresholds
IF coverage['business'] < 95:
    Display: "  ⚠️ Business logic coverage below 95% threshold"
    coverage_issues = true

IF coverage['application'] < 85:
    Display: "  ⚠️ Application coverage below 85% threshold"
    coverage_issues = true

IF coverage['infrastructure'] < 80:
    Display: "  ⚠️ Infrastructure coverage below 80% threshold"
    coverage_issues = true

IF coverage_issues:
    # Ask user how to proceed
    AskUserQuestion:
        question: "Coverage below thresholds. How to proceed?"
        header: "Coverage"
        options:
            - label: "Add more tests now"
              description: "Re-invoke test-automator to fill coverage gaps"
            - label: "Defer to QA"
              description: "Mark as incomplete DoD item, address in QA phase"
            - label: "Accept (document why)"
              description: "Justify why lower coverage acceptable for this story"
        multiSelect: false

    # Handle user response appropriately

IF test_results['failed'] > 0:
    Display: "❌ {test_results['failed']} tests failing"
    HALT development

IF build_status != "SUCCESS":
    Display: "❌ Build failed"
    HALT development

Display: "✓ Ready for Phase 08 (Git Workflow / Change Tracking)"
```

---

## Subagents Invoked

**integration-tester:**
- Runs full test suite with coverage analysis
- Validates coverage thresholds (95%/85%/80%)
- Checks build success
- Detects integration issues
- Reports linter violations

---

## Success Criteria

Phase 05 succeeds when:
- [ ] All tests pass (100% pass rate)
- [ ] Coverage meets thresholds:
  - Business logic: ≥95%
  - Application layer: ≥85%
  - Infrastructure: ≥80%
- [ ] Build succeeds
- [ ] No integration issues detected
- [ ] Linter passes (if applicable)

---

## Coverage Threshold Enforcement

**Coverage below thresholds triggers user decision:**

**Option 1: Add more tests now**
- Re-invoke test-automator subagent
- Focus on uncovered code paths
- Run coverage again after new tests added

**Option 2: Defer to QA**
- Mark as incomplete DoD item: "Test coverage: {X}% (below {threshold}%) - Deferred to STORY-XXX"
- Create follow-up story for coverage improvement
- Document in Implementation Notes

**Option 3: Accept with justification**
- Document why lower coverage acceptable (e.g., generated code, trivial getters/setters)
- Add to Implementation Notes
- Proceed to Phase 08

---

## ✅ PHASE 4 COMPLETION CHECKPOINT

**Before proceeding to Phase 05.5 (Deferral Challenge), verify ALL steps executed:**

### Mandatory Steps Executed

- [ ] **Phase 05.0:** Anti-Gaming Validation PASSED [MANDATORY - NEW]
  - Verification: integration-tester ran gaming validation BEFORE tests
  - Verification: No skip decorators, empty tests, TODO placeholders, excessive mocking
  - Output: "✓ Anti-gaming validation passed" displayed
  - HALT IF: gaming_scan.status == "BLOCKED"

- [ ] **Step 1:** integration-tester subagent invoked
  - Verification: Full test suite executed with coverage (AFTER Phase 05.0PASS)
  - Output: Test results and coverage percentages displayed

- [ ] **Step 2:** Integration test results parsed and validated
  - Verification: Coverage thresholds checked (95%/85%/80%)
  - Verification: Build status confirmed
  - Output: Test results, coverage by layer, build status displayed

### Success Criteria

- [ ] All tests pass (100% pass rate)
- [ ] Coverage meets thresholds:
  - [ ] Business logic: ≥95%
  - [ ] Application layer: ≥85%
  - [ ] Infrastructure: ≥80%
- [ ] Build succeeds
- [ ] No integration issues detected
- [ ] Linter passes (if applicable)
- [ ] Ready for deferral validation

### Checkpoint Validation

**IF ANY ITEM UNCHECKED:**
```
❌ PHASE 4 INCOMPLETE - Review missing steps above
⚠️  DO NOT PROCEED TO PHASE 4.5 until all checkpoints pass

Common issues:
  - Coverage below thresholds ← Add more tests or justify lower coverage
  - Integration test failures ← Fix API contracts, DI config, timing issues
  - Build failures ← Check compilation errors, dependencies

If coverage below thresholds:
  - Option 1: Add more tests now (re-invoke test-automator)
  - Option 2: Defer to QA (mark as incomplete DoD item)
  - Option 3: Accept with justification (document why acceptable)
```

**IF ALL ITEMS CHECKED:**
```
✅ PHASE 4 COMPLETE - Integration Testing Done

Tests: {passed}/{total} passing (100%)
Coverage: Business {X}%, Application {Y}%, Infrastructure {Z}%
Build: SUCCESS
Integration: No issues detected

All quality thresholds met. Ready for deferral validation.

**Update Progress Tracker:**
Mark "Execute Phase 05" todo as "completed"

**See Also:**
- `phase-4.5-deferral-challenge.md` - Phase 05.5 workflow (deferral validation)
- `integration-tester` subagent - Integration testing specialist
- `deferral-budget-enforcement.md` - Phase 08 deferral budget limits

Next: Load phase-4.5-deferral-challenge.md and execute Phase 05.5 (Deferral Challenge)
```

---

### Step 3: Update AC Verification Checklist (Phase 05 Items) [NEW - RCA-011]

**Purpose:** Check off AC items related to integration testing (real-time progress tracking)

**Execution:** After integration-tester completes, before Phase 05 checkpoint

**Load AC Checklist Update Workflow:**
```
Read(file_path=".claude/skills/devforgeai-development/references/ac-checklist-update-workflow.md")
```

**Identify Phase 05 AC Items:**
```
Grep(pattern="Phase.*: 4", path="${STORY_FILE}", output_mode="content", -B=1)
```

**Common Phase 05 items:**
- [ ] Integration tests passing
- [ ] Cross-component tests passing
- [ ] Performance targets met (response time, throughput)
- [ ] Coverage thresholds met (95%/85%/80%)
- [ ] All scenarios tested
- [ ] Token efficiency verified

**Update Procedure:** Batch-update all Phase 05 items

**Display:** "Phase 05 AC Checklist: ✓ {count} items checked | AC Progress: {X}/{Y}"

**Performance:** ~30-60 seconds

---

## Common Issues

**Issue 1: Integration test failures**
- Check API contracts match specification
- Verify dependency injection configuration
- Validate database schema matches models
- Check for timing issues (async operations)

**Issue 2: Build failures**
- Check for compilation errors
- Verify dependencies installed ({INSTALL_COMMAND})
- Check for syntax errors missed by tests

**Issue 3: Coverage gaps**
- Identify uncovered branches (if/else, switch)
- Add tests for edge cases
- Check for unreachable code (dead code elimination)
