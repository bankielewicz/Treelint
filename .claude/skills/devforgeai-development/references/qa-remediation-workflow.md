# QA Remediation Workflow

**Purpose:** Targeted development workflow when gaps.json exists from previous QA failure.

**When to use:** `$REMEDIATION_MODE == true` (set by Step h.1. in preflight-validation.md)

**Token Cost:** ~3,000 tokens when loaded

---

## Overview

When QA fails and generates gaps.json, `/dev` enters **Remediation Mode** - a targeted workflow that focuses TDD phases on specific gaps rather than full story implementation.

**Key Differences from Normal TDD:**

| Aspect | Normal TDD | Remediation Mode |
|--------|------------|------------------|
| Phase 02 (Red) | Generate tests for ALL AC | Generate tests for GAP FILES only |
| Phase 03 (Green) | Implement ALL story features | Fix COVERAGE GAPS only |
| Phase 04 (Refactor) | General code quality | Fix ANTI-PATTERN VIOLATIONS only |
| Phase 06 | Validate all deferrals | Resolve DEFERRAL ISSUES from gaps.json |
| Scope | Full story | Gaps from QA report |
| Subagent context | Story spec | gaps.json + story spec |

---

## Prerequisites

**Required variables from Step h.1.:**
- `$REMEDIATION_MODE = true`
- `$QA_COVERAGE_GAPS` - Array of coverage gap objects
- `$QA_ANTIPATTERN_GAPS` - Array of anti-pattern violations
- `$QA_DEFERRAL_GAPS` - Array of deferral issues
- `$QA_FAILURE_REPORT` - Path to QA report

**Required files:**
- `devforgeai/qa/reports/{STORY_ID}-gaps.json` - Structured gap data
- `devforgeai/qa/reports/{STORY_ID}-qa-report.md` - Human-readable report

---

## Phase 02R: Targeted Test Generation (Red Phase - Remediation)

**Purpose:** Generate tests ONLY for coverage gaps identified in gaps.json.

### Step 1R.1: Display Gap Summary

```
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  Phase 02R: Targeted Test Generation (REMEDIATION MODE)"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""
Display: "Coverage Gaps to Address: {$QA_COVERAGE_GAPS.count}"
Display: ""

FOR EACH gap in $QA_COVERAGE_GAPS:
    Display: "📉 {gap.file}"
    Display: "   Current: {gap.current_coverage}% | Target: {gap.target_coverage}%"
    Display: "   Gap: {gap.gap_percentage}% ({gap.uncovered_line_count} lines)"
    Display: "   Suggested Tests:"
    FOR EACH test in gap.suggested_tests:
        Display: "     • {test}"
    Display: ""
```

### Step 1R.2: Invoke Test-Automator with Gap Context

**Pass structured gap data to test-automator:**

```
Task(
    subagent_type="test-automator",
    description="Generate targeted tests for coverage gaps",
    prompt="Generate tests to close coverage gaps for {STORY_ID}.

            MODE: REMEDIATION (targeted, not full coverage)

            COVERAGE GAPS TO ADDRESS:
            {json.dumps($QA_COVERAGE_GAPS, indent=2)}

            CONTEXT FILES (already read by skill, summary provided):
            - tech-stack.md: {technology summary}
            - coding-standards.md: {test patterns}

            STORY SPEC (from preflight):
            Story ID: {STORY_ID}
            Story Title: {story_title}

            INSTRUCTIONS:
            1. For EACH file in coverage_gaps:
               - Analyze the suggested_tests descriptions
               - Generate specific test cases for each suggestion
               - Target the uncovered scenarios

            2. Test naming: test_{scenario_from_suggestion}
               Example: 'Test rollback on corrupted backup' → test_rollback_corrupted_backup()

            3. Focus on:
               - Error handling paths (often uncovered)
               - Edge cases mentioned in suggestions
               - Business logic branches

            4. Do NOT generate tests for:
               - Files not in coverage_gaps array
               - Already-covered scenarios
               - Infrastructure that passes threshold

            OUTPUT:
            - Write test files to appropriate test directory
            - Return JSON with files created and test counts

            EXPECTED RESULT:
            - Each gap file should gain {gap.gap_percentage}%+ coverage
            - Suggested tests converted to executable test cases"
)
```

### Step 1R.3: Validate Test Generation

```
# Parse test-automator response
test_result = parse_subagent_response()

Display: ""
Display: "Tests Generated:"
FOR EACH file in test_result.files_created:
    Display: "   ✓ {file.path} ({file.test_count} tests)"

# Run tests to verify they fail (Red phase)
Bash(command="{$TEST_COMMAND}", description="Run generated tests (expect failures)")

IF tests_fail:
    Display: "✅ Phase 02R Complete - Tests properly fail (Red phase successful)"
ELSE:
    Display: "⚠️  Warning: Some tests pass immediately - verify coverage targets"
```

---

## Phase 03R: Targeted Implementation (Green Phase - Remediation)

**Purpose:** Implement ONLY code needed to pass the new tests and close coverage gaps.

### Step 2R.1: Focus on Gap Files

```
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  Phase 03R: Targeted Implementation (REMEDIATION MODE)"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""
Display: "Files to Modify (coverage gaps only):"

FOR EACH gap in $QA_COVERAGE_GAPS:
    Display: "   • {gap.file}"
```

### Step 2R.2: Implement Missing Coverage

```
# For each gap file, implement the missing code paths

FOR EACH gap in $QA_COVERAGE_GAPS:
    # Read the file
    Read(file_path=gap.file)

    # Invoke backend-architect for implementation
    Task(
        subagent_type="backend-architect",
        description="Implement missing coverage for {gap.file}",
        prompt="Implement code to satisfy failing tests for {gap.file}.

                CONTEXT:
                - Current coverage: {gap.current_coverage}%
                - Target coverage: {gap.target_coverage}%
                - Uncovered lines: {gap.uncovered_line_count}

                SUGGESTED SCENARIOS TO IMPLEMENT:
                {json.dumps(gap.suggested_tests)}

                CONSTRAINTS:
                - Follow tech-stack.md (no library substitution)
                - Follow coding-standards.md patterns
                - Keep cyclomatic complexity < 10

                GOAL: Make the new tests pass while maintaining existing functionality.

                OUTPUT: Write implementation to {gap.file}"
    )
```

### Step 2R.3: Verify Tests Pass

```
# Run all tests
Bash(command="{$TEST_COMMAND}", description="Run tests (should pass)")

IF all_tests_pass:
    Display: "✅ Phase 03R Complete - All tests passing"
ELSE:
    Display: "❌ Some tests still failing - review implementation"
    # List failing tests
    # Return to Step 2R.2
```

---

## Phase 04R: Anti-Pattern Resolution (Refactor Phase - Remediation)

**Purpose:** Fix ONLY anti-pattern violations identified in gaps.json.

### Step 3R.1: Check Anti-Pattern Gaps

```
IF $QA_ANTIPATTERN_GAPS.count == 0:
    Display: "✓ No anti-pattern violations to resolve"
    # Skip to Phase 05R
    GOTO Phase 05R

Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  Phase 04R: Anti-Pattern Resolution (REMEDIATION MODE)"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""
Display: "Anti-Pattern Violations to Fix: {$QA_ANTIPATTERN_GAPS.count}"
Display: ""

FOR EACH violation in $QA_ANTIPATTERN_GAPS:
    Display: "⚠️  {violation.file}:{violation.line}"
    Display: "   Type: {violation.type} ({violation.severity})"
    Display: "   Issue: {violation.message}"
    Display: "   Fix: {violation.remediation}"
    Display: ""
```

### Step 3R.2: Invoke Refactoring Specialist

```
Task(
    subagent_type="refactoring-specialist",
    description="Fix anti-pattern violations",
    prompt="Fix anti-pattern violations identified by QA.

            VIOLATIONS TO FIX:
            {json.dumps($QA_ANTIPATTERN_GAPS, indent=2)}

            CONSTRAINTS:
            - Keep all tests passing
            - Follow coding-standards.md patterns
            - Do NOT change public API signatures (unless required for fix)

            FOR EACH VIOLATION:
            1. Read the file at violation.file
            2. Navigate to violation.line
            3. Apply violation.remediation
            4. Verify tests still pass

            OUTPUT: Summary of changes made"
)

# Verify tests still pass after refactoring
Bash(command="{$TEST_COMMAND}", description="Verify tests pass after refactoring")

Display: "✅ Phase 04R Complete - Anti-patterns resolved"
```

---

## Phase 05R: Coverage Verification

**Purpose:** Verify coverage gaps are closed before proceeding.

```
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  Phase 05R: Coverage Verification (REMEDIATION MODE)"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""

# Run coverage report
Bash(command="{$TEST_COVERAGE_COMMAND}", description="Run tests with coverage")

# Check each gap file
all_gaps_closed = true

FOR EACH gap in $QA_COVERAGE_GAPS:
    # Parse coverage for this file
    new_coverage = extract_coverage_for_file(gap.file)

    IF new_coverage >= gap.target_coverage:
        Display: "✅ {gap.file}: {new_coverage}% (was {gap.current_coverage}%)"
    ELSE:
        Display: "❌ {gap.file}: {new_coverage}% (need {gap.target_coverage}%)"
        all_gaps_closed = false

IF all_gaps_closed:
    Display: ""
    Display: "✅ ALL COVERAGE GAPS CLOSED"
    Display: ""
    # Continue to Phase 06R
ELSE:
    Display: ""
    Display: "⚠️  Some coverage gaps remain"
    Display: ""

    AskUserQuestion(
        questions=[{
            question: "Coverage gaps not fully closed. How should we proceed?",
            header: "Coverage",
            multiSelect: false,
            options: [
                {
                    label: "Generate more tests",
                    description: "Return to Phase 02R with remaining gaps"
                },
                {
                    label: "Accept current coverage",
                    description: "Proceed with current coverage (may fail QA again)"
                },
                {
                    label: "Investigate gaps",
                    description: "Show remaining uncovered lines for analysis"
                }
            ]
        }]
    )

    # Handle response
```

---

## Phase 06R: Deferral Resolution

**Purpose:** Resolve deferral issues identified in gaps.json.

```
IF $QA_DEFERRAL_GAPS.count == 0:
    Display: "✓ No deferral issues to resolve"
    # Skip to Phase 08R
    GOTO Phase 08R

Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  Phase 06R: Deferral Resolution (REMEDIATION MODE)"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""

FOR EACH deferral in $QA_DEFERRAL_GAPS:
    Display: "📋 {deferral.item}"
    Display: "   Issue: {deferral.violation_type}"
    Display: "   Current: {deferral.current_reason}"
    Display: "   Required: {deferral.remediation}"
    Display: ""

# Use existing qa-deferral-recovery.md workflow for resolution
# (This workflow is already implemented in DevForgeAI)

INVOKE: qa-deferral-recovery.md workflow with $QA_DEFERRAL_GAPS
```

---

## Phase 08R: Commit and Complete

**Purpose:** Commit remediation changes and prepare for QA re-run.

```
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  Phase 08R: Commit Remediation (REMEDIATION MODE)"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""

# Show what was changed
Bash(command="git diff --stat", description="Show changes made")

# Commit with remediation context
commit_message = """fix({STORY_ID}): Close coverage gaps from QA

Remediation for QA failure:
- Coverage gaps closed: {$QA_COVERAGE_GAPS.count} files
- Anti-patterns fixed: {$QA_ANTIPATTERN_GAPS.count} violations
- Deferrals resolved: {$QA_DEFERRAL_GAPS.count} issues

Gaps addressed:
{FOR EACH gap in $QA_COVERAGE_GAPS:
  - {gap.file}: {gap.current_coverage}% → {new_coverage}%
}

Triggered by: devforgeai/qa/reports/{STORY_ID}-gaps.json
"""

Bash(command="git add . && git commit -m '{commit_message}'", description="Commit remediation")

Display: ""
Display: "✅ REMEDIATION COMPLETE"
Display: ""
Display: "Gaps addressed:"
Display: "  • Coverage: {$QA_COVERAGE_GAPS.count} files improved"
Display: "  • Anti-patterns: {$QA_ANTIPATTERN_GAPS.count} violations fixed"
Display: "  • Deferrals: {$QA_DEFERRAL_GAPS.count} issues resolved"
Display: ""
Display: "Next steps:"
Display: "  1. Run: /qa {STORY_ID}"
Display: "  2. If QA passes, gaps.json will be archived to resolved/"
Display: "  3. Story can then proceed to release"
Display: ""
```

---

## Summary

**Remediation Mode Workflow:**

```
Step h.1.: Load gaps.json → Set $REMEDIATION_MODE = true
    ↓
Phase 02R: Generate tests for $QA_COVERAGE_GAPS files only
    ↓
Phase 03R: Implement code to pass new tests
    ↓
Phase 04R: Fix $QA_ANTIPATTERN_GAPS violations
    ↓
Phase 05R: Verify all coverage gaps closed
    ↓
Phase 06R: Resolve $QA_DEFERRAL_GAPS issues
    ↓
Phase 08R: Commit remediation
    ↓
/qa STORY-ID: Re-run QA to validate
    ↓
If PASS: gaps.json archived to resolved/
```

**Token Efficiency:**
- Normal TDD: ~40,000 tokens (full story implementation)
- Remediation Mode: ~15,000 tokens (targeted fixes only)
- **Savings:** ~62% fewer tokens for QA failure recovery

---

## References

- `preflight-validation.md` - Step h.1. loads gaps.json
- `qa-deferral-recovery.md` - Existing deferral resolution workflow
- `devforgeai/qa/reports/{STORY-ID}-gaps.json` - Structured gap data
- `test-automator.md` - Subagent for targeted test generation
- `refactoring-specialist.md` - Subagent for anti-pattern fixes
