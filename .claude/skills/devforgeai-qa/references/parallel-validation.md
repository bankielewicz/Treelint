# Parallel Validation for QA Skill

**Story:** STORY-113
**Version:** 1.0
**Epic:** EPIC-017 (Parallel Task Orchestration)

---

## Overview

This reference defines the parallel validation pattern for the QA skill, enabling concurrent execution of test-automator, code-reviewer, and security-auditor subagents.

**Performance Impact:** 3x faster QA validation (90s sequential → 30s parallel)

**Key Principle:** One failed validator should not block two successful ones.

---

## Context Summary Passing (STORY-180)

### Generate Context Summary Before Invocation

To reduce token usage (~3K per subagent), generate a context summary once and pass to all validators:

```pseudocode
# Step 1: Generate context summary (one-time extraction)
context_summary = """
**Context Summary (do not re-read files):**
- tech-stack.md: Framework-agnostic, Markdown-based, no external deps
- anti-patterns.md: No Bash for file ops, no monolithic components
- architecture-constraints.md: Three-layer, single responsibility
- source-tree.md: Skills in .claude/skills/, agents in .claude/agents/
- dependencies.md: Zero external deps for core framework
- coding-standards.md: Direct instructions, not prose; YAML frontmatter required
"""

# Step 2: Pass summary to each validator (below)
```

### Token Savings with Context Summaries

| Validator | Without Summary | With Summary | Savings |
|-----------|-----------------|--------------|---------|
| test-automator | ~3K tokens | ~0.5K tokens | -2.5K |
| code-reviewer | ~3K tokens | ~0.5K tokens | -2.5K |
| security-auditor | ~3K tokens | ~0.5K tokens | -2.5K |
| **Total** | ~9K tokens | ~1.5K tokens | **-7.5K** |

---

## Adaptive Validator Selection (STORY-183)

### Validator Mapping by Story Type

Select validators based on story type extracted in Phase 0 (Step 0.6):

| Story Type | Validators | Count | Success Threshold | Token Savings |
|------------|------------|-------|-------------------|---------------|
| `documentation` | code-reviewer only | 1 | 1/1 (100%) | ~6K (-67%) |
| `refactor` | code-reviewer, security-auditor | 2 | 1/2 (50%) | ~3K (-33%) |
| `feature` | all 3 validators | 3 | 2/3 (66%) | 0 (full) |
| `bugfix` | all 3 validators | 3 | 2/3 (66%) | 0 (full) |
| (unknown/missing) | all 3 validators | 3 | 2/3 (66%) | 0 (default) |

### Selection Algorithm

```pseudocode
# Step 1: Get story type from Phase 0 (default: "feature" for unknown/missing)
story_type = $STORY_TYPE  # Set in Step 0.6

# Step 2: Select validators based on story type
IF story_type == "documentation":
    # Documentation stories: only code-reviewer (skip test-automator, security-auditor)
    validators = ["code-reviewer"]
    success_threshold = 1  # 1/1 required
    Display: "ℹ️ Documentation story - running code-reviewer only"

ELIF story_type == "refactor":
    # Refactor stories: skip test-automator (tests already exist)
    validators = ["code-reviewer", "security-auditor"]
    success_threshold = 1  # 1/2 required
    Display: "ℹ️ Refactor story - skipping test-automator (tests exist)"

ELSE:  # "feature", "bugfix", or fallback for unknown
    # Feature/bugfix: full validation suite with all 3 validators
    validators = ["test-automator", "code-reviewer", "security-auditor"]
    success_threshold = 2  # 2/3 required
    Display: "ℹ️ Feature/bugfix story - running all validators"

# Step 3: Display validator count for transparency
validator_count = len(validators)
Display: f"  Validators: {validator_count}"
Display: f"  Threshold: {success_threshold}/{validator_count}"
```

### Rationale by Story Type

**Documentation Stories:**
- documentation stories skip test-automator (no executable code)
- documentation stories skip security-auditor (no attack surface)
- documentation stories only run code-reviewer for quality review

**Refactor Stories:**
- refactor stories skip test-automator (tests already exist)
- refactor stories include code-reviewer (code quality critical)
- refactor stories include security-auditor (security review needed)

**Feature/Bugfix Stories:**
- feature stories run all 3 validators (full validation suite)
- bugfix stories run all 3 validators (comprehensive testing needed)
- New code requires all validators: test-automator, code-reviewer, security-auditor

---

## Parallel Invocation Pattern

### Single Message with 3 Task Calls (Default - Feature/Bugfix)

Execute ALL three validators in ONE message (parallel execution):

```pseudocode
# All 3 Task calls in a SINGLE message - they execute in parallel
Task(
    subagent_type="test-automator",
    prompt=f"""Analyze test coverage for {STORY_ID}. Check: test file existence, coverage percentage, assertion quality. Return: coverage metrics and gaps.

**Response Constraints:**
Return ONLY:
1. Status: PASS/FAIL
2. Coverage %: {{number}}
3. Key findings (max 3 bullets)
4. Blocking issues (if any)

Do NOT include: full analysis, code snippets, detailed recommendations.

{context_summary}""",
    description="Run test analysis",
    run_in_background=true
)

Task(
    subagent_type="code-reviewer",
    prompt=f"""Review code changes for {STORY_ID}. Check: code quality, maintainability, best practices. Return: review findings with severity.

**Response Constraints:**
Return ONLY:
1. Status: PASS/FAIL
2. Coverage %: {{number}}
3. Key findings (max 3 bullets)
4. Blocking issues (if any)

Do NOT include: full analysis, code snippets, detailed recommendations.

{context_summary}""",
    description="Review code",
    run_in_background=true
)

Task(
    subagent_type="security-auditor",
    prompt=f"""Scan code for {STORY_ID}. Check: OWASP Top 10, input validation, authentication. Return: security findings with severity.

**Response Constraints:**
Return ONLY:
1. Status: PASS/FAIL
2. Coverage %: {{number}}
3. Key findings (max 3 bullets)
4. Blocking issues (if any)

Do NOT include: full analysis, code snippets, detailed recommendations.

{context_summary}""",
    description="Security scan",
    run_in_background=true
)
```

**CRITICAL:** All 3 Task calls MUST be in the same message for parallel execution.

**NOTE:** Include `{context_summary}` in each prompt to avoid redundant context file reads.

---

## Result Aggregation

### Collect Results Using TaskOutput

After launching parallel tasks, collect results:

```pseudocode
# Wait for all results (blocking retrieval)
test_result = TaskOutput(task_id=test_task.id, block=true, timeout=120000)
review_result = TaskOutput(task_id=review_task.id, block=true, timeout=120000)
security_result = TaskOutput(task_id=security_task.id, block=true, timeout=120000)

# Aggregate using PartialResult model (from STORY-110)
partial_result = aggregate_parallel_results([test_result, review_result, security_result])
```

### PartialResult Model

Uses the `PartialResult` data model from error-handling-patterns.md (STORY-110):

```yaml
PartialResult:
  successes: List[TaskResult]       # Successfully completed validators
  failures: List[TaskFailure]       # Failed validator details
  total_tasks: 3                    # Always 3 for QA validation
  success_rate: Float               # 0.0-1.0 (e.g., 0.67 for 2/3)
```

---

## Success Threshold

### Adaptive Threshold Based on Validator Count (STORY-183)

Threshold adjusts based on story type and validator count.

**Threshold formula:** `validators_passed / validators_run >= success_threshold`

The pass ratio determines success based on story type requirements.

```pseudocode
# Get validator count and threshold from adaptive selection
validator_count = len(validators)  # 1, 2, or 3 based on story type
success_threshold = threshold_from_story_type  # Set in selection algorithm

# Calculate minimum success rate (threshold formula)
min_success_rate = success_threshold / validator_count  # pass ratio calculation

IF partial_result.success_rate < min_success_rate:
    Display: "⚠️ QA validation below threshold"
    Display: f"  Success rate: {partial_result.success_rate * 100}%"
    Display: f"  Required: 66% (2 of 3 validators)"
    Display: "  Failed validators:"
    FOR failure IN partial_result.failures:
        Display: f"    - {failure.task_id}: {failure.error_message}"
    HALT workflow

ELSE:
    Display: "✓ QA validation threshold met"
    Continue to result aggregation
```

### Threshold Rationale by Story Type (STORY-183)

**Feature/Bugfix Stories (3 validators, threshold: 2):**

| Scenario | Validators | Success Rate | Decision |
|----------|------------|--------------|----------|
| All pass | 3/3 | 100% | Continue (ideal) |
| 2 pass, 1 fail | 2/3 | 67% | Continue (acceptable) |
| 1 pass, 2 fail | 1/3 | 33% | **HALT** (below 66%) |
| All fail | 0/3 | 0% | **HALT** (critical failure) |

**Refactor Stories (2 validators, threshold: 1):**

| Scenario | Validators | Success Rate | Decision |
|----------|------------|--------------|----------|
| All pass | 2/2 | 100% | Continue (ideal) |
| 1 pass, 1 fail | 1/2 | 50% | Continue (threshold met) |
| All fail | 0/2 | 0% | **HALT** (critical failure) |

**Documentation Stories (1 validator, threshold: 1):**

| Scenario | Validators | Success Rate | Decision |
|----------|------------|--------------|----------|
| Pass | 1/1 | 100% | Continue |
| Fail | 0/1 | 0% | **HALT** |

---

## Display Format

### Parallel Validation Phase Display

```
✓ Phase 2 Complete: Parallel validation
  test-automator: [PASS ✓ / FAIL ✗]
  code-reviewer: [PASS ✓ / FAIL ✗]
  security-auditor: [PASS ✓ / FAIL ✗]
  Success rate: [X]% (threshold: 66%)
  Duration: [X]s (vs ~[3X]s sequential)
```

### Example Output (2 of 3 Pass)

```
✓ Phase 2 Complete: Parallel validation
  test-automator: PASS ✓ (coverage: 92%)
  code-reviewer: PASS ✓ (no critical findings)
  security-auditor: FAIL ✗ (timeout after 120s)
  Success rate: 67% (threshold: 66%)
  Duration: 32s (vs ~90s sequential)

⚠️ Note: security-auditor failed - results excluded from report
```

---

## Error Handling

### Integration with error-handling-patterns.md (STORY-110)

This reference uses error handling patterns defined in:
`.claude/skills/devforgeai-orchestration/references/error-handling-patterns.md`

**Key concepts applied:**
1. **Partial Failure Recovery** - Continue if success_rate >= 0.66
2. **Result Aggregation** - Use PartialResult model for mixed results
3. **Failure Logging** - Log failed validators with correlation ID

### Failure Classification

| Error Type | Retryable | Action |
|------------|-----------|--------|
| Timeout | Yes | Log, continue with 2/3 |
| TransientError | Yes | Log, continue with 2/3 |
| PermanentError | No | Log, continue with 2/3 |
| All Failed | - | HALT, suggest sequential fallback |

---

## Coverage WARN Escalation (ADR-010)

**CRITICAL:** test-automator returning WARN for coverage gaps → QA FAILED (not PASS WITH WARNINGS)

### Coverage Result Interpretation

| test-automator Result | Coverage Status | Phase 3 Outcome |
|----------------------|-----------------|-----------------|
| PASS (≥ thresholds) | Met | PASSED (if no other issues) |
| WARN (< thresholds) | Below | **FAILED** (blocking - ADR-010) |
| FAIL (execution error) | Unknown | Depends on 2/3 threshold |

### Escalation Logic

```pseudocode
# At Phase 3 Step 3.1 (Result Determination)
IF test_automator_result.status == "WARN":
    IF test_automator_result.reason == "coverage_below_threshold":
        # Coverage WARN is NOT a warning - it's a blocking FAILED
        coverage_blocking = true
        overall_status = "FAILED"  # Non-negotiable
        Display: "❌ Coverage below thresholds - QA FAILED (ADR-010)"
```

### Rationale

- **Coverage gaps cannot be deferred** - they must be remediated
- **test-automator WARN ≠ other validator WARN** - coverage is special case
- **Enforcement prevents "approved with warnings" for incomplete coverage**

**Reference:** `.claude/rules/workflow/qa-validation.md` (Coverage Threshold Enforcement section)

---

## Configuration Reference

### Loading Parallel Config

```pseudocode
Read(file_path="devforgeai/config/parallel-orchestration.yaml")

# Extract timeout settings
timeout_ms = config.profiles[active_profile].timeout_ms  # Default: 120000
max_concurrent_tasks = config.profiles[active_profile].max_concurrent_tasks  # Default: 4
```

### Config Integration

The QA skill respects `parallel-orchestration.yaml` for:
- `timeout_ms`: Maximum wait time per validator
- `min_success_rate`: Configured threshold (override possible)

---

## Phase Integration

### Where This Runs in QA Workflow

This parallel validation replaces the sequential Phase 2 (Anti-Pattern Detection) with a comprehensive parallel phase:

```
Phase 0.0-0.9: Pre-validation (unchanged)
    ↓
Phase 1: Coverage Analysis (unchanged, but can run in parallel with Phase 2)
    ↓
Phase 2: Parallel Validation ← THIS REFERENCE
    - test-automator (parallel)
    - code-reviewer (parallel)
    - security-auditor (parallel)
    ↓
Phase 3-7: Continue with aggregated results
```

---

## Related Documentation

- `error-handling-patterns.md` (STORY-110) - PartialResult model, failure handling
- `parallel-config.md` (STORY-108) - Configuration schema
- `task-result-aggregation.md` (STORY-112) - TaskOutput blocking retrieval

---

## Phase 2.2 Completion Checkpoint [MANDATORY - BLOCKS PHASE 2.3]

**Purpose:** Ensure all required validators (based on story type) were invoked before proceeding.

**Constitution Alignment:** Parallel tasks MUST be independent (architecture-constraints.md line 169)

### Validator Invocation Checklist (Adaptive - STORY-183)

Before proceeding to Phase 2.3, verify ALL required validators were invoked based on story type:

**For feature/bugfix stories (3 validators required):**
```
- [ ] test-automator subagent invoked? (verify Task() call in conversation)
- [ ] code-reviewer subagent invoked? (verify Task() call in conversation)
- [ ] security-auditor subagent invoked? (verify Task() call in conversation)
```

**For refactor stories (2 validators required):**
```
- [ ] code-reviewer subagent invoked? (verify Task() call in conversation)
- [ ] security-auditor subagent invoked? (verify Task() call in conversation)
```

**For documentation stories (1 validator required):**
```
- [ ] code-reviewer subagent invoked? (verify Task() call in conversation)
```

### Enforcement Logic (Adaptive)

```
# Get required validators from story type selection
required_validators = validators  # Set by adaptive selection algorithm
required_count = len(required_validators)

# Count actually invoked validators
invoked_count = count(invoked validators in required_validators)

IF invoked_count < required_count:
    Display: "❌ INCOMPLETE: Only {invoked_count}/{required_count} validators invoked"
    Display: "Missing: {list_missing_validators}"
    HALT: "All required validators MUST be invoked in a SINGLE message for parallel execution"

    AskUserQuestion:
        Question: "Invoke missing validators now?"
        Header: "Validator Gap"
        Options:
            - label: "Yes, invoke {missing} now"
              description: "Launch missing validator(s) before continuing"
            - label: "Continue with {invoked_count}/{required_count} (NOT RECOMMENDED)"
              description: "Proceed without all validators - may miss issues"
        multiSelect: false

    IF user chooses "Yes": Invoke missing validator(s)
    IF user chooses "Continue": Log warning, proceed with reduced coverage

IF invoked_count == required_count:
    Display: "✓ All {required_count} validators invoked"
    PROCEED to Phase 2.3
```

### Important Distinction (Adaptive - STORY-183)

| Concept | Applies To | Threshold |
|---------|------------|-----------|
| **Invocation** | Task() calls | N/N REQUIRED (N = validators for story type) |
| **Success Rate** | Validator RESULTS | Varies by story type (see threshold table) |

**Note:** The success threshold applies to RESULTS, not invocation. ALL required validators for the story type MUST be invoked even if some fail.

### Verification Test

```
# Test: Run QA and deliberately skip one validator
# Expected: HALT occurs with "Only 2/3 validators invoked" message
# Resolution: AskUserQuestion prompts for missing validator invocation
```

---

**Last Updated:** 2026-01-06
**Story:** STORY-113, STORY-126, STORY-183 (Adaptive Validator Selection)
