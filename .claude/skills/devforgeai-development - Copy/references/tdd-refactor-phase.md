# Phase 04: Refactor (Refactor Phase)

**Purpose:** Improve code quality while keeping all tests green.

**Execution Order:** After Phase 03 (Green phase) - tests are passing

**Expected Outcome:** Improved code quality, all tests still GREEN

**Token Cost:** ~1,200 tokens in skill context (~60,000 combined in isolated subagent contexts)

---

## Phase Progress Indicator

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 04/10: Refactoring (30% → 40% complete)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Display this indicator at the start of Phase 04 execution.**

---

## Overview

The Refactor phase improves code quality without changing behavior. Tests remain green throughout.

**Core Principle:** Improve structure, maintain behavior.

---

## Story Type Skip Check [MANDATORY FIRST] (STORY-126)

**Purpose:** Skip Phase 04 for `bugfix` story types (minimal changes preferred).

**When to execute:** Before any Phase 04 processing.

```
# Check if Phase 04 should be skipped based on story type
# $STORY_TYPE set in Phase 01.6.5 (Pre-Flight Validation)

IF $STORY_TYPE == "bugfix":
    Display: ""
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: "  ℹ️  SKIPPING PHASE 04: Story Type 'bugfix'"
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: ""
    Display: "Reason: Bugfix stories require minimal, targeted changes."
    Display: "        Refactoring may introduce scope creep or unrelated changes."
    Display: ""
    Display: "Proceeding to Phase 05 (Integration Testing)..."
    Display: ""

    # Skip Phase 04 entirely
    GOTO Phase 05

    RETURN
```

---

## Phase 04: Refactor (Refactor Phase)

**Delegate refactoring to refactoring-specialist and code-reviewer subagents.**

### Step 1: Invoke refactoring-specialist Subagent

```
Task(
  subagent_type="refactoring-specialist",
  description="Refactor code while keeping tests green",
  prompt="Refactor the implementation from Phase 03 to improve code quality.

  Implementation files and tests available in conversation.

  Context files to enforce:
  - devforgeai/specs/context/anti-patterns.md (check for violations)
  - devforgeai/specs/context/coding-standards.md (apply patterns)
  - devforgeai/specs/context/architecture-constraints.md (maintain layer boundaries)

  Refactoring targets:
  1. Anti-pattern violations (God objects, tight coupling, magic numbers)
  2. Code complexity (methods >50 lines, cyclomatic complexity >10)
  3. Code duplication (DRY principle violations)
  4. Naming improvements (clarity, consistency)
  5. Performance optimizations (if low-hanging fruit)

  Requirements:
  - Keep tests GREEN throughout (run {TEST_COMMAND} after each change)
  - Use native tools (Edit for modifications, not sed)
  - Make incremental changes (one refactoring at a time)
  - HALT if tests break

  Test command: {TEST_COMMAND}

  Return:
  - Refactorings applied (list with rationale)
  - Files modified
  - Test status after each refactoring (must stay GREEN)"
)
```

### Step 2: Parse Subagent Response

```javascript
result = extract_from_subagent_output(response)

refactorings_applied = result["refactorings"]
tests_green = result["tests_remained_green"]

Display: "✓ Phase 04 (Refactor): Code improved by refactoring-specialist"
Display: "  - Refactorings applied: {len(refactorings_applied)}"

FOR refactoring in refactorings_applied:
    Display: "    • {refactoring['type']}: {refactoring['rationale']}"

IF tests_green:
    Display: "  - Tests: ✅ GREEN (all passing after refactoring)"
ELSE:
    Display: "  - Tests: ❌ BROKEN during refactoring"
    HALT development
```

### Step 3: Invoke code-reviewer Subagent

```
Task(
  subagent_type="code-reviewer",
  description="Review refactored code for quality",
  prompt="Perform comprehensive code review of the refactored implementation.

  Code and tests available in conversation.

  Review checklist:
  1. Code quality (readability, maintainability, simplicity)
  2. Security (no vulnerabilities, input validation, secrets management)
  3. Best practices (SOLID principles, design patterns)
  4. Test coverage (all paths covered, edge cases tested)
  5. Documentation (public APIs documented, complex logic explained)
  6. Performance (no obvious bottlenecks)
  7. Context file compliance (tech-stack.md, coding-standards.md, etc.)

  Provide feedback organized by priority:
  - CRITICAL (must fix before commit)
  - HIGH (should fix now)
  - MEDIUM (should fix soon)
  - LOW (nice to have)

  Return:
  - Issues found (by priority)
  - Positive observations
  - Recommendations"
)
```

### Step 4: Parse Code Review Response

```javascript
result = extract_from_subagent_output(response)

critical_issues = result["issues"]["critical"]
high_issues = result["issues"]["high"]

Display: "✓ Code review by code-reviewer complete"

IF len(critical_issues) > 0:
    Display: "  - CRITICAL issues: {len(critical_issues)} (must fix)"

    FOR issue in critical_issues:
        Display: "    🚨 {issue['description']}"

    # Re-invoke refactoring-specialist to fix critical issues
    Display: "Re-invoking refactoring-specialist to fix critical issues..."
    # [Task call with critical issues in prompt]

ELIF len(high_issues) > 0:
    Display: "  - HIGH issues: {len(high_issues)} (should fix)"

    FOR issue in high_issues:
        Display: "    ⚠️ {issue['description']}"

    # Ask user if they want to fix now
    AskUserQuestion:
        question: "{len(high_issues)} high-priority issues found. Fix now?"
        header: "Code Review"
        options:
            - label: "Fix now"
              description: "Address issues before proceeding"
            - label: "Continue"
              description: "Accept issues, proceed to Phase 05"
        multiSelect: false

ELSE:
    Display: "  - No critical or high issues found ✅"
    Display: "  Ready for Step 4 (Anti-Gaming Validation)"
```

---

### Step 4: Anti-Gaming Validation [MANDATORY - NEW]

**Purpose:** Detect test gaming patterns that artificially inflate coverage/pass rates.

**CRITICAL:** This step MUST complete BEFORE Light QA (Step 5). Test gaming undermines TDD integrity.

**Why This Matters:**
- Skip decorators hide failing tests (inflate pass rate)
- Empty tests pass without validating anything (inflate coverage)
- TODO placeholders indicate incomplete tests (inflate test count)
- Excessive mocking bypasses real behavior testing (fake coverage)

#### 4.1 Parse Gaming Validation from code-reviewer

The code-reviewer subagent (Step 3) now includes anti-gaming validation. Extract result:

```javascript
gaming_result = code_reviewer_response.gaming_validation

Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  Step 4: Anti-Gaming Validation"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""

IF gaming_result.status == "FAIL":
    Display: "🚨 TEST GAMING DETECTED - WORKFLOW HALTED"
    Display: ""
    Display: "The following gaming patterns were found:"
    Display: ""

    FOR each violation in gaming_result.violations:
        Display: "  [{violation.severity}] {violation.type}:"
        Display: "    {violation.message}"
        Display: "    Files: {violation.files}"
        Display: ""

    Display: "Gaming Pattern Counts:"
    Display: "  - Skip Decorators: {gaming_result.skip_decorators_found}"
    Display: "  - Empty Tests: {gaming_result.empty_tests_found}"
    Display: "  - TODO Placeholders: {gaming_result.todo_placeholders_found}"
    Display: "  - Excessive Mocking: {len(gaming_result.excessive_mocking_files)} files"
    Display: ""

    HALT: "Fix all test gaming violations before proceeding. Tests must be authentic."

ELSE:
    Display: "✓ Anti-gaming validation PASSED"
    Display: "  - No skip decorators detected"
    Display: "  - No empty tests detected"
    Display: "  - No TODO placeholders in tests"
    Display: "  - Mock ratios acceptable (≤2× test count)"
    Display: ""
    Display: "Test suite is authentic. Proceeding to Light QA..."
```

#### 4.2 Gaming Patterns That Block Workflow

| Pattern | Severity | Detection | Action |
|---------|----------|-----------|--------|
| Skip decorators | CRITICAL | `@skip`, `@Ignore`, `test.skip`, `xit(` | HALT |
| Empty tests | CRITICAL | Tests with no assertions or only `pass` | HALT |
| TODO placeholders | CRITICAL | `TODO`, `FIXME`, `NotImplementedError` | HALT |
| Excessive mocking | CRITICAL | Mocks > 2× test count | HALT |
| Stub-only tests | CRITICAL | Setup without real assertions | HALT |

#### 4.3 Resolution Requirements

Before workflow can continue:
1. **Remove ALL skip decorators** (or create ADR explaining why skip is necessary for specific test)
2. **Add real assertions to ALL empty tests** (tests must verify actual behavior)
3. **Implement ALL TODO placeholders** (no incomplete test stubs)
4. **Reduce mock count to ≤2× test count** (test real behavior, not mocks)
5. **Ensure tests verify real behavior**, not just that mocks return expected values

#### 4.4 Checkpoint Verification

```
IF gaming_result.violations is NOT empty:
    HALT: "Phase 04 blocked by test gaming. {len(gaming_result.violations)} violations found."
    Display: "Fix violations and re-run /dev workflow from Phase 04"
    DO NOT PROCEED to Step 5 (Light QA)

ELSE:
    Display: "✓ Anti-gaming validation passed"
    Mark Step 4 complete in progress tracker
    PROCEED to Step 5 (Light QA Validation)
```

---

### Step 5: Invoke Light QA Validation [MANDATORY]

**Purpose:** Intermediate quality gate to validate refactored code before proceeding to integration testing.

**Why Mandatory:** Light QA catches issues early (build failures, test regressions, anti-patterns) before expensive integration testing phase. Prevents propagating refactoring errors.

**Timing:** After refactoring complete and code review passed, BEFORE Phase 05 (Integration)

**Invoke devforgeai-qa skill in light mode:**

```
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  Phase 04 Step 5: Light QA Validation (Intermediate Quality Gate)"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""
Display: "Running light validation after refactoring..."
Display: ""
Display: "**Validation Mode:** light"
Display: "**Story ID:** {STORY_ID}"
Display: ""

# Invoke QA skill
Skill(command="devforgeai-qa")
```

**What Light QA validates:**
- Build succeeds (no syntax errors introduced)
- All tests pass (no regressions from refactoring)
- No anti-patterns introduced (quick scan)
- Code quality metrics acceptable

**Expected outcomes:**

**If Light QA PASSES:**
```
Display: ""
Display: "✅ STEP 5 COMPLETE: Light QA Validation PASSED"
Display: ""
Display: "Refactoring validated successfully:"
Display: "  - Build: ✅ Succeeds"
Display: "  - Tests: ✅ All passing"
Display: "  - Anti-patterns: ✅ None detected"
Display: "  - Code quality: ✅ Acceptable"
Display: ""
Display: "Ready for Phase 05 (Integration Testing)"
Display: ""

# Proceed to Phase 05
```

**If Light QA FAILS:**
```
Display: ""
Display: "❌ STEP 5 FAILED: Light QA Validation FAILED"
Display: ""
Display: "Refactoring introduced issues:"
Display: "  Review Light QA report above for violations"
Display: ""
Display: "Common issues:"
Display: "  - Test regressions (refactoring broke tests)"
Display: "  - Anti-patterns introduced (code smells)"
Display: "  - Build failures (syntax errors)"
Display: ""
Display: "Fix violations and re-run Phase 04 refactoring"
Display: ""

HALT development
DO NOT proceed to Phase 05 until Light QA passes
```

**Note:** The devforgeai-qa skill will detect "light" mode from conversation context. See SKILL.md for mode detection pattern.

---

## Subagents Invoked

**refactoring-specialist:**
- Applies systematic refactoring patterns
- Maintains green tests throughout
- Targets anti-patterns, complexity, duplication

**code-reviewer:**
- Comprehensive quality review
- Security vulnerability detection
- Best practices enforcement
- Context file compliance verification

**devforgeai-qa (light mode):** [NEW - Step 5]
- Validates build succeeds after refactoring
- Confirms all tests still pass (no regressions)
- Quick anti-pattern scan
- Code quality metrics check
- HALTS if violations detected

---

## ✅ PHASE 04 COMPLETION CHECKPOINT

**Before proceeding to Phase 05 (Integration Testing), verify ALL steps executed:**

### Mandatory Steps Executed

- [ ] **Step 1:** refactoring-specialist subagent invoked
  - Verification: Refactorings applied and listed
  - Output: Refactoring types and rationales displayed

- [ ] **Step 2:** Refactoring response parsed
  - Verification: Files modified shown, tests remained GREEN

- [ ] **Step 3:** code-reviewer subagent invoked
  - Verification: Comprehensive code review performed
  - Verification: Anti-gaming validation included in response

- [ ] **Step 4:** Anti-Gaming Validation [MANDATORY - NEW]
  - Verification: gaming_validation.status extracted from code-reviewer response
  - Verification: No skip decorators, empty tests, TODO placeholders, excessive mocking
  - Output: Gaming validation PASSED or HALTED with violations list
  - HALT IF: gaming_result.status == "FAIL"

- [ ] **Step 5:** Light QA validation executed [MANDATORY]
  - Verification: devforgeai-qa skill invoked in light mode
  - Verification: Build succeeds, tests pass, no anti-patterns
  - Output: Light QA PASSED message displayed

- [ ] **Step 6:** AC Verification Checklist updated (Phase 04 items) [NEW - RCA-011]
  - Verification: All Phase 04 AC items checked off (quality/refactoring items)
  - Output: "AC Progress: X/Y items complete" displayed
  - Graceful: Skipped if story doesn't have AC Checklist section

### Success Criteria

- [ ] Code quality improved (complexity reduced, duplication removed)
- [ ] All tests still GREEN (no regressions)
- [ ] No CRITICAL issues from code review
- [ ] Anti-patterns removed
- [ ] Code follows coding-standards.md
- [ ] Light QA validation passed
- [ ] AC Checklist updated (Phase 04 items checked)
- [ ] Ready for integration testing

### Checkpoint Validation

**IF ANY ITEM UNCHECKED:**
```
❌ PHASE 3 INCOMPLETE - Review missing steps above
⚠️  DO NOT PROCEED TO PHASE 4 until all checkpoints pass

Most commonly missed:
  - Step 5 (Light QA) ← Was buried in refactoring-patterns.md, now explicit
  - Code review CRITICAL issues ← Must fix before advancing
  - Tests still GREEN ← Refactoring must not break tests

Proceeding without Step 5 results in:
  - Refactoring errors not caught until Deep QA
  - Anti-patterns escaping to integration testing
  - Build failures discovered late
```

**IF ALL ITEMS CHECKED:**
```
✅ PHASE 3 COMPLETE - Refactor Phase Done

Refactorings: Applied and validated
Tests: All GREEN (no regressions)
Code Review: Passed (no CRITICAL issues)
Light QA: PASSED (build, tests, anti-patterns validated)

Code quality improved. Ready for integration testing.

**Update Progress Tracker:**
Mark "Execute Phase 04" todo as "completed"

**See Also:**
- `integration-testing.md` - Phase 05 workflow (cross-component testing)
- `refactoring-patterns.md` - Detailed refactoring catalog
- `code-reviewer` subagent - Code review specialist
- `devforgeai-qa` skill (light mode) - Intermediate quality gate

Next: Load integration-testing.md and execute Phase 05 (Integration & Validation)
```

---

## Common Refactorings

**See `references/refactoring-patterns.md` for comprehensive patterns:**
- Extract Method (long methods)
- Extract Class (God objects)
- Introduce Parameter Object (long parameter lists)
- Replace Magic Number with Constant
- Consolidate Duplicate Code
- Simplify Conditional Expressions
