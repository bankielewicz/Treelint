---
name: devforgeai-qa
description: Validates code quality through hybrid progressive validation (light checks during development, deep analysis after completion). Enforces test coverage (95%/85%/80% strict thresholds), detects anti-patterns, validates spec compliance, and analyzes code quality metrics. Use when validating implementations, ensuring quality standards, or preparing for release.
tools: AskUserQuestion, Read, Write, Edit, Glob, Grep, Bash, Task
model: claude-opus-4-5-20251101
---

# DevForgeAI QA Skill

Quality validation enforcing architectural constraints, coverage thresholds, and code standards through progressive validation.

Do not skip any phases in the devforgeai-qa skill.

---

## EXECUTION MODEL: This Skill Expands Inline

**After invocation, YOU (Claude) execute these instructions phase by phase.**

**When you invoke this skill:**
1. This SKILL.md content is now in your conversation
2. You execute each phase sequentially
3. You display results as you work through phases
4. You complete with success/failure report

**Do NOT:**
- Wait passively for skill to "return results"
- Assume skill is executing elsewhere
- Stop workflow after invocation

**Proceed to "Parameter Extraction" section below and begin execution.**

---

## Parameter Extraction

Extracts story ID and mode (light/deep) from conversation context.

**See `references/parameter-extraction.md`** for extraction algorithm (YAML frontmatter, file reference, explicit statement, status inference).
    Read(file_path=".claude/skills/devforgeai-qa/references/parameter-extraction.md")

---

## CRITICAL: Definition of Done Protocol

**Deferral Validation CANNOT be skipped.**

Deferred DoD items MUST have user approval, story/ADR references, and deferral-validator subagent validation.

**PROHIBITED:** Autonomous deferrals, manual validation shortcuts, token optimization bypasses.

**Rationale:** RCA-007 - Manual validation missed STORY-004→005→006 chain, causing work loss.

**See `references/dod-protocol.md`** for protocol requirements and enforcement.
    Read(file_path=".claude/skills/devforgeai-qa/references/dod-protocol.md")

---

## Validation Modes

### Light (~10K tokens, 2-3 min)
- Build/syntax checks
- Test execution (100% pass required)
- Critical anti-patterns only
- Deferral validation (if deferrals exist)

### Deep (~35K tokens, 8-12 min)
- Complete coverage analysis (95%/85%/80% thresholds)
- Comprehensive anti-pattern detection
- Full spec compliance (AC, API, NFRs)
- Code quality metrics
- Security scanning (OWASP Top 10)
- Deferral validation (if deferrals exist)

---

## QA Workflow (5 Phases)

**EXECUTION STARTS HERE - You are now executing the skill's workflow.**

**Progressive Disclosure:** Workflow references are loaded when each phase executes (not before) to optimize token usage.

**IMPORTANT:** "On-demand" means "load when phase starts" - NOT "loading is optional."

**Execution Pattern:**
1. Reach phase (e.g., Phase 2: Analysis)
2. See "⚠️ CHECKPOINT" marker
3. Load reference file (REQUIRED)
4. Execute ALL steps from reference file
5. Complete phase marker write
6. Proceed to next phase

**IF you skip loading a reference:** You will execute the phase incorrectly and miss mandatory steps.

---

## Phase 0: Setup

**Purpose:** Initialize QA environment - validate CWD, create test isolation, acquire locks.

**Create execution tracker at Phase 0 start:**

```
TodoWrite({
  todos: [
    { content: "Phase 0: Setup", status: "in_progress", activeForm: "Running Phase 0: Setup" },
    { content: "Phase 1: Validation", status: "pending", activeForm: "Running Phase 1: Validation" },
    { content: "Phase 2: Analysis", status: "pending", activeForm: "Running Phase 2: Analysis" },
    { content: "Phase 3: Reporting", status: "pending", activeForm: "Running Phase 3: Reporting" },
    { content: "Phase 4: Cleanup", status: "pending", activeForm: "Running Phase 4: Cleanup" }
  ]
})
```

### Step 0.0: Session Checkpoint Detection [NEW - STORY-126]

**Purpose:** Detect interrupted QA sessions and offer resume capability.

**Constitution Alignment:** Skills MUST NOT assume state from previous invocations (architecture-constraints.md line 38)
    Read(file_path=".claude/skills/devforgeai-qa/references/deep-validation-workflow.md")

```
checkpoint_path = "devforgeai/qa/reports/{STORY_ID}/.qa-session-checkpoint.json"
Glob(pattern=checkpoint_path)

IF checkpoint found:
    Read(file_path=checkpoint_path)
    checkpoint = parse_json(file_content)

    IF checkpoint.can_resume == true:
        Display: "Found interrupted QA session for {STORY_ID}"
        Display: "  Last phase: {checkpoint.current_phase}"
        Display: "  Completed phases: {checkpoint.completed_phases}"
        Display: "  Started: {checkpoint.started_at}"

        AskUserQuestion:
            Question: "Resume from Phase {checkpoint.current_phase} or start fresh?"
            Header: "Resume Session"
            Options:
                - label: "Resume from Phase {current_phase}"
                  description: "Continue from last checkpoint, skip completed phases"
                - label: "Start fresh (discard checkpoint)"
                  description: "Delete checkpoint and run complete QA validation"
            multiSelect: false

        IF user chooses "Resume":
            # Load checkpoint state
            $RESUME_MODE = true
            $RESUME_PHASE = checkpoint.current_phase
            $COMPLETED_PHASES = checkpoint.completed_phases
            Display: "✓ Resuming from Phase {checkpoint.current_phase}"
            # Skip to RESUME_PHASE (pre-flight will validate markers)
        ELSE:
            # Delete checkpoint and start fresh
            Bash(command="rm {checkpoint_path}")
            $RESUME_MODE = false
            Display: "✓ Starting fresh QA validation"

ELSE:
    $RESUME_MODE = false
    Display: "✓ No interrupted session found - starting fresh"
```

### Step 0.1: Validate Project Root [MANDATORY - FIRST STEP]

```
# Check project marker file
result = Read(file_path="CLAUDE.md")

IF result.success:
    content = result.content
    IF content_contains("DevForgeAI") OR content_contains("devforgeai"):
        CWD_VALID = true
        Display: "✓ Project root validated"
    ELSE:
        CWD_VALID = false
        HALT: Use AskUserQuestion to get correct path
ELSE:
    # Try secondary markers
    dir_check = Glob(pattern=".claude/skills/*.md")
    IF dir_check.has_results:
        CWD_VALID = true
        Display: "✓ Project root validated via .claude/skills/ structure"
    ELSE:
        CWD_VALID = false
        HALT: Use AskUserQuestion: "Provide project root path?"
```

**CRITICAL:** Do NOT proceed if CWD validation fails.

### Step 0.2: Load Test Isolation Configuration

**Reference:** `references/test-isolation-service.md`
    Read(file_path=".claude/skills/devforgeai-qa/references/test-isolation-service.md")

```
Read(file_path="devforgeai/config/test-isolation.yaml")

IF file not found:
    Display: "ℹ️ Test isolation config not found, using defaults"
    config = {
        enabled: true,
        paths: {
            results_base: "tests/results",
            coverage_base: "tests/coverage",
            logs_base: "tests/logs"
        },
        directory: { auto_create: true, permissions: 755 },
        concurrency: { locking_enabled: true, lock_timeout_seconds: 300 }
    }
ELSE:
    config = parsed YAML content
    Display: "✓ Test isolation config loaded"
```

### Step 0.3: Create Story-Scoped Directories

```
story_paths = {
    results_dir: "{config.paths.results_base}/{STORY_ID}",
    coverage_dir: "{config.paths.coverage_base}/{STORY_ID}",
    logs_dir: "{config.paths.logs_base}/{STORY_ID}"
}

IF config.directory.auto_create:
    Bash(command="mkdir -p {story_paths.results_dir} {story_paths.coverage_dir} {story_paths.logs_dir}")
    Write(file_path="{story_paths.results_dir}/timestamp.txt", content="{ISO_8601_TIMESTAMP}")
    Display: "✓ Story directories created: {STORY_ID}"
```

### Step 0.4: Acquire Lock File

```
IF config.concurrency.locking_enabled:
    lock_file = "{story_paths.results_dir}/.qa-lock"

    IF exists(lock_file):
        lock_age = now() - file_mtime(lock_file)
        IF lock_age > config.concurrency.stale_lock_threshold_seconds:
            Display: "⚠️ Removing stale lock file"
            Remove(file_path=lock_file)
        ELSE:
            AskUserQuestion: "Lock exists. Wait/Force/Cancel?"

    Write(file_path=lock_file, content="timestamp: {ISO_8601}\nstory: {STORY_ID}")
    Display: "✓ Lock acquired for {STORY_ID}"
```

### Step 0.5: Load Deep Mode Workflow (Deep Mode Only)

```
IF mode == "deep":
    Read(file_path=".claude/skills/devforgeai-qa/references/deep-validation-workflow.md")
    Display: "✓ Deep validation workflow loaded"
```

### Step 0.6: Extract Story Type for Adaptive Validation (STORY-183)

**Purpose:** Extract story type from YAML frontmatter to select appropriate validators.

This step detects story type in Phase 0 for adaptive validator selection.

```
# Read story file and extract type from frontmatter
story_content = Read(file_path="devforgeai/specs/Stories/{STORY_ID}*.story.md")

# Extract story_type from YAML frontmatter (type: field)
story_type = grep_extract("^type:\s*(.+)$", story_content)

IF $STORY_TYPE is empty OR $STORY_TYPE not in ["feature", "bugfix", "refactor", "documentation"]:
    # Default to feature for unknown/missing types (full validation)
    $STORY_TYPE = "feature"
    Display: "ℹ️ Story type not specified - defaulting to 'feature' (full validation)"
ELSE:
    Display: "✓ Story type detected: {$STORY_TYPE}"

# Display which validators will run (adaptive selection preview)
IF $STORY_TYPE == "documentation":
    Display: "  → Validators: [code-reviewer] (1/1 threshold)"
ELIF $STORY_TYPE == "refactor":
    Display: "  → Validators: [code-reviewer, security-auditor] (1/2 threshold)"
ELSE:
    Display: "  → Validators: [test-automator, code-reviewer, security-auditor] (2/3 threshold)"

# Store for Phase 2 adaptive validator selection
# See references/parallel-validation.md for validator mapping
    Read(file_path=".claude/skills/devforgeai-qa/references/parallel-validation.md")

```

**Validator Selection by Story Type:**

| Story Type | Validators | Threshold | Rationale |
|------------|------------|-----------|-----------|
| `documentation` | code-reviewer only | 1/1 | No code tests needed |
| `refactor` | code-reviewer, security-auditor | 1/2 | Tests already exist |
| `feature`/`bugfix` | all 3 validators | 2/3 | Full validation suite |
| (unknown/missing) | all 3 validators | 2/3 | Conservative default |

**Phase 0 Completion Checklist:**
- [ ] CWD validated
- [ ] Test isolation config loaded
- [ ] Story directories created
- [ ] Lock acquired (if enabled)
- [ ] Deep workflow loaded (if deep mode)
- [ ] Story type extracted for adaptive validation

**Display:**
```
✓ Phase 0 Complete: Setup
  Project root: ✓ Validated
  Test isolation: ✓ Configured
  Lock: ✓ Acquired
  Mode: [light/deep]
```

### Phase 0 Marker Write

```
Write(file_path="devforgeai/qa/reports/{STORY_ID}/.qa-phase-0.marker",
      content="phase: 0\nstory_id: {STORY_ID}\nmode: {MODE}\ntimestamp: {ISO_8601}\nstatus: complete")

Display: "✓ Phase 0 marker written"
Display: "Phase 0 ✓ | Setup | Lock acquired"
```

**Update execution tracker:**

```
TodoWrite({
  todos: [
    { content: "Phase 0: Setup", status: "completed", activeForm: "Phase 0 complete" },
    { content: "Phase 1: Validation", status: "in_progress", activeForm: "Running Phase 1: Validation" },
    { content: "Phase 2: Analysis", status: "pending", activeForm: "Running Phase 2: Analysis" },
    { content: "Phase 3: Reporting", status: "pending", activeForm: "Running Phase 3: Reporting" },
    { content: "Phase 4: Cleanup", status: "pending", activeForm: "Running Phase 4: Cleanup" }
  ]
})
```

### Phase 0 Completion Enforcement

**Verify deep-validation-workflow.md was loaded (deep mode only):**

```
IF mode == "deep":
    IF "deep-validation-workflow.md" NOT loaded in conversation:
        Display: "❌ CRITICAL ERROR: Phase 0 Step 0.5 incomplete"
        Display: "   Deep validation workflow reference file was not loaded"
        Display: "   Load file: .claude/skills/devforgeai-qa/references/deep-validation-workflow.md"
        HALT: "Cannot proceed to Phase 1 without deep workflow reference"
        Instruction: "Load the reference file manually, then resume /qa {STORY_ID} deep"
    ELSE:
        Display: "✓ Deep mode workflow reference verified loaded"
```

This enforcement prevents Phase 1-3 from executing without complete initialization.

---

## Phase Marker Protocol [STORY-126 Enhancement]

**Purpose:** Write marker files after each phase completes to enable sequential verification.

**Constitution Alignment:** All-or-Nothing Principle (architecture-constraints.md line 246)

### Marker File Format

```yaml
# Location: devforgeai/qa/reports/{STORY_ID}/.qa-phase-{N}.marker
phase: {N}
story_id: {STORY_ID}
mode: {MODE}
timestamp: {ISO_8601}
status: complete
```

### Marker Write (End of Each Phase)

```
Write(file_path="devforgeai/qa/reports/{STORY_ID}/.qa-phase-{N}.marker",
      content="phase: {N}\nstory_id: {STORY_ID}\nmode: {MODE}\ntimestamp: {TIMESTAMP}\nstatus: complete")

Display: "✓ Phase {N} marker written"
```

### Pre-Flight Verification (Start of Phases 1-4)

```
Glob(pattern="devforgeai/qa/reports/{STORY_ID}/.qa-phase-{N-1}.marker")

IF NOT found:
    Display: "❌ Phase {N-1} marker not found"
    Display: "   Previous phase may not have completed"
    HALT: "Run phases in sequence. Start from Phase {N-1}"
ELSE:
    Read marker and verify story_id matches
    Display: "✓ Phase {N-1} verified complete"
```

---

## Phase 1: Validation

### Pre-Flight: Verify Phase 0 Complete

```
Glob(pattern="devforgeai/qa/reports/{STORY_ID}/.qa-phase-0.marker")

IF marker file NOT found:
    CRITICAL ERROR: "Phase 0 not verified complete"
    HALT: "Phase 1 cannot execute without Phase 0 completion"
    Display: "Previous phase (Phase 0) must complete successfully before starting Phase 1"
    Instruction: "Start workflow from Phase 0. Run setup first."
    Exit: Code 1 (phase sequencing violation)

Display: "✓ Phase 0 verified complete - Phase 1 preconditions met"
```

### ⚠️ CHECKPOINT: Phase 1 Reference Loading [MANDATORY]

**You MUST execute ALL steps before proceeding to phase content.**

**Step 1.0: Load Workflow Reference (REQUIRED)**
```
Read(file_path=".claude/skills/devforgeai-qa/references/traceability-validation-algorithm.md")
```

**This reference contains the complete workflow. Execute ALL steps from the reference file.**

**After loading:** Proceed to Step 1.1 (in reference file)

**IF you skip this step:** You will execute the phase incorrectly and miss mandatory steps.

---

**Purpose:** Execute tests, analyze coverage, validate traceability.

### Step 1.1: AC-DoD Traceability Validation

**Reference:** `references/traceability-validation-algorithm.md`
**Templates:** `assets/traceability-report-template.md`
    Read(file_path=".claude/skills/devforgeai-qa/references/traceability-validation-algorithm.md")
    Read(file_path=".claude/skills/devforgeai-qa/assets/traceability-report-template.md")

```
# Extract AC Requirements
ac_headers = grep count "^### AC#[0-9]+" story_file
FOR each AC section:
    Extract: Then/And clauses, bullet requirements, metrics
    Store: ac_requirements[]

# Extract DoD Items
dod_section = extract_between("^## Definition of Done", "^## Workflow")
FOR each subsection in [Implementation, Quality, Testing, Documentation]:
    Parse: checkbox lines
    Store: dod_items[]

# Map AC → DoD
FOR each ac_req in ac_requirements:
    best_match = find_best_dod_match(ac_keywords, dod_items)
    IF match_score >= 0.5:
        traceability_map[ac_req] = best_match
    ELSE:
        missing_traceability.append(ac_req)

# Calculate Score
traceability_score = ((total - missing.length) / total) × 100

IF traceability_score < 100:
    Display: "QA WORKFLOW HALTED - Fix traceability issues"
    EXIT
```

### Step 1.2: Test Coverage Analysis

**Reference:** `references/coverage-analysis-workflow.md`
**Guide:** `references/coverage-analysis.md`
    Read(file_path=".claude/skills/devforgeai-qa/references/coverage-analysis-workflow.md")
    Read(file_path=".claude/skills/devforgeai-qa/references/coverage-analysis.md")

Execute the 7-step coverage workflow:
1. Load coverage thresholds (95%/85%/80%)
2. Generate coverage reports (language-specific command)
3. Classify files by layer (Business Logic, Application, Infrastructure)
4. Calculate coverage percentage for each layer
5. Validate against thresholds
6. Identify coverage gaps with test suggestions
7. Analyze test quality (assertions, mocking, pyramid)

**Blocks on:** Business <95%, Application <85%, Overall <80%

**Display:**
```
✓ Phase 1 Complete: Validation
  Traceability: {traceability_score}%
  Business Logic: [X]% (threshold: 95%)
  Application: [X]% (threshold: 85%)
  Infrastructure: [X]% (threshold: 80%)
  Overall: [X]%
```

### Phase 1 Completion Checklist

**Before writing Phase 1 marker, verify you have:**

- [ ] Loaded traceability-validation-algorithm.md (Step 1.0)
- [ ] Validated AC-DoD traceability (Step 1.1)
- [ ] Executed test runner (Step 1.2)
- [ ] Analyzed coverage results (Step 1.3)
- [ ] Verified critical threshold (100% pass required)
- [ ] Displayed Phase 1 completion summary

**IF any checkbox unchecked:** HALT and complete missing steps before Phase 2.

**Display to user:**
```
✓ Phase 1 Complete: Validation | {traceability_score}% traceability
```

### Phase 1 Marker Write

```
Write(file_path="devforgeai/qa/reports/{STORY_ID}/.qa-phase-1.marker",
      content="phase: 1\nstory_id: {STORY_ID}\nmode: {MODE}\ntimestamp: {ISO_8601}\nstatus: complete")

Display: "✓ Phase 1 marker written"
Display: "Phase 1 ✓ | Validation | {traceability_score}% traceability"
```

**Update execution tracker:**

```
TodoWrite({
  todos: [
    { content: "Phase 0: Setup", status: "completed", activeForm: "Phase 0 complete" },
    { content: "Phase 1: Validation", status: "completed", activeForm: "Phase 1 complete" },
    { content: "Phase 2: Analysis", status: "in_progress", activeForm: "Running Phase 2: Analysis" },
    { content: "Phase 3: Reporting", status: "pending", activeForm: "Running Phase 3: Reporting" },
    { content: "Phase 4: Cleanup", status: "pending", activeForm: "Running Phase 4: Cleanup" }
  ]
})
```

---

## Phase 2: Analysis

### Pre-Flight: Verify Phase 1 Complete

```
Glob(pattern="devforgeai/qa/reports/{STORY_ID}/.qa-phase-1.marker")

IF marker file NOT found:
    CRITICAL ERROR: "Phase 1 not verified complete"
    HALT: "Phase 2 cannot execute without Phase 1 completion"
    Display: "Previous phase (Phase 1) must complete successfully before starting Phase 2"
    Instruction: "Start workflow from Phase 0. Run setup first."
    Exit: Code 1 (phase sequencing violation)

Display: "✓ Phase 1 verified complete - Phase 2 preconditions met"
```

### ⚠️ CHECKPOINT: Phase 2 Reference Loading [MANDATORY]

**You MUST execute ALL steps before proceeding to phase content.**

**Step 2.0: Load Workflow References (REQUIRED)**
```
Read(file_path=".claude/skills/devforgeai-qa/references/anti-pattern-detection-workflow.md")
Read(file_path=".claude/skills/devforgeai-qa/references/parallel-validation.md")
Read(file_path=".claude/skills/devforgeai-qa/references/spec-compliance-workflow.md")
Read(file_path=".claude/skills/devforgeai-qa/references/code-quality-workflow.md")
```

**These references contain the complete workflows. Execute ALL steps from the reference files.**

**After loading:** Proceed to Step 2.1 (Anti-Pattern Detection)

**IF you skip this step:** You will execute the phase incorrectly and miss mandatory steps.

---

**Purpose:** Detect anti-patterns, run parallel validators, check spec compliance, measure quality.

### Step 2.1: Anti-Pattern Detection

**Reference:** `references/anti-pattern-detection-workflow.md`
**Subagent:** anti-pattern-scanner
    Read(file_path=".claude/skills/devforgeai-qa/references/anti-pattern-detection-workflow.md")

```
# Load ALL 6 context files
Read: tech-stack.md, source-tree.md, dependencies.md,
      coding-standards.md, architecture-constraints.md, anti-patterns.md

# Invoke scanner
Task(subagent_type="anti-pattern-scanner",
     prompt="Scan {changed_files} for violations against 6 context files")

# Parse results
violations = parse_json_response()
```

**Blocks on:** CRITICAL (security, library substitution), HIGH (structure, layer)

### Step 2.2: Parallel Validation (Deep Mode Only)

**Reference:** `references/parallel-validation.md`
    Read(file_path=".claude/skills/devforgeai-qa/references/parallel-validation.md")

Execute 3 validators in SINGLE message (parallel):
```
Task(subagent_type="test-automator", prompt="Analyze test coverage...", description="Run tests")
Task(subagent_type="code-reviewer", prompt="Review code changes...", description="Review code")
Task(subagent_type="security-auditor", prompt="Scan for security issues...", description="Security scan")
```

**Success Threshold:** 66% (2 of 3 must pass)

### Step 2.3: Spec Compliance Validation

**Reference:** `references/spec-compliance-workflow.md`
**Subagent:** deferral-validator (MANDATORY if deferrals exist)
    Read(file_path=".claude/skills/devforgeai-qa/references/spec-compliance-workflow.md")

1. Validate story documentation (Implementation Notes, DoD Status, Test Results)
2. Validate acceptance criteria (tests exist and pass for each)
3. Validate deferred DoD items (invoke deferral-validator if deferrals exist)
4. Validate API contracts (endpoints match spec)
5. Validate NFRs (performance, security)
6. Generate traceability matrix

### Step 2.4: Code Quality Metrics

**Reference:** `references/code-quality-workflow.md`
    Read(file_path=".claude/skills/devforgeai-qa/references/code-quality-workflow.md")

1. Analyze cyclomatic complexity (tool: radon/complexity-report)
2. Calculate maintainability index (MI <70 = MEDIUM, <50 = HIGH)
3. Detect code duplication (jscpd, >20% = HIGH)
4. Measure documentation coverage (target: 80%)
5. Analyze dependency coupling

**Blocks on:** Duplication >20%, MI <50

**Display:**
```
✓ Phase 2 Complete: Analysis
  Anti-patterns: [X] CRITICAL, [X] HIGH, [X] MEDIUM
  Parallel validators: [X]/3 passed (threshold: 2/3)
  Spec compliance: [X]/[Y] criteria validated
  Quality metrics: Complexity avg [X], MI [X]%, Duplication [X]%
```

### Phase 2 Completion Checklist

**Before writing Phase 2 marker, verify you have:**

- [ ] Loaded anti-pattern-detection-workflow.md (Step 2.0)
- [ ] Invoked anti-pattern-scanner subagent (Step 2.1)
- [ ] Ran parallel validators (Step 2.2) - deep mode only
- [ ] Executed spec compliance validation (Step 2.3)
- [ ] Analyzed code quality metrics (Step 2.4)
- [ ] Checked blocking violations (CRITICAL/HIGH)
- [ ] Displayed Phase 2 completion summary

**IF any checkbox unchecked:** HALT and complete missing steps before Phase 3.

**Display to user:**
```
✓ Phase 2 Complete: Analysis | {validator_count}/3 validators
```

### Phase 2 Marker Write

```
Write(file_path="devforgeai/qa/reports/{STORY_ID}/.qa-phase-2.marker",
      content="phase: 2\nstory_id: {STORY_ID}\nmode: {MODE}\ntimestamp: {ISO_8601}\nstatus: complete")

Display: "✓ Phase 2 marker written"
Display: "Phase 2 ✓ | Analysis | {validator_count}/3 validators"
```

**Update execution tracker:**

```
TodoWrite({
  todos: [
    { content: "Phase 0: Setup", status: "completed", activeForm: "Phase 0 complete" },
    { content: "Phase 1: Validation", status: "completed", activeForm: "Phase 1 complete" },
    { content: "Phase 2: Analysis", status: "completed", activeForm: "Phase 2 complete" },
    { content: "Phase 3: Reporting", status: "in_progress", activeForm: "Running Phase 3: Reporting" },
    { content: "Phase 4: Cleanup", status: "pending", activeForm: "Running Phase 4: Cleanup" }
  ]
})
```

---

## Phase 3: Reporting

### Pre-Flight: Verify Phase 2 Complete

```
Glob(pattern="devforgeai/qa/reports/{STORY_ID}/.qa-phase-2.marker")

IF marker file NOT found:
    CRITICAL ERROR: "Phase 2 not verified complete"
    HALT: "Phase 3 cannot execute without Phase 2 completion"
    Display: "Previous phase (Phase 2) must complete successfully before starting Phase 3"
    Instruction: "Start workflow from Phase 0. Run setup first."
    Exit: Code 1 (phase sequencing violation)

Display: "✓ Phase 2 verified complete - Phase 3 preconditions met"
```

### ⚠️ CHECKPOINT: Phase 3 Reference Loading [MANDATORY]

**You MUST execute ALL steps before proceeding to phase content.**

**Step 3.0: Load Workflow Reference (REQUIRED)**
```
Read(file_path=".claude/skills/devforgeai-qa/references/qa-result-formatting-guide.md")
```

**This reference contains the complete workflow. Execute ALL steps from the reference file.**

**After loading:** Proceed to Step 3.1 (Result Determination)

**IF you skip this step:** You will execute the phase incorrectly and miss mandatory steps.

---

**Purpose:** Generate QA report, update story status, create gaps.json if failed.

### Step 3.1-3.3: Result Determination and Report Generation

**Purpose:** Determine QA result and generate reports.

**1. Determine Result:**

**CRITICAL (ADR-010):** Coverage below thresholds is a BLOCKING condition.
- Coverage gaps are NOT warnings - they trigger FAILED status
- test-automator WARN for coverage → escalates to FAILED here
- No deferral path exists for coverage gaps

```
# Coverage thresholds: Business 95%, Application 85%, Infrastructure 80%
# IMPORTANT: coverage < thresholds → FAILED (not PASS WITH WARNINGS)
IF any CRITICAL violations OR coverage < thresholds OR parallel < 66%:
    overall_status = "FAILED"
    # Coverage gap = FAILED (non-negotiable, per ADR-010)
ELIF any HIGH violations:
    overall_status = "PASS WITH WARNINGS"
    # HIGH violations (NOT coverage) allow approval with warnings
ELSE:
    overall_status = "PASSED"

Display: "Result determined: {overall_status}"
```

**2. Generate QA Report (Deep Mode Only):**
```
IF mode == "deep":
    Write(file_path="devforgeai/qa/reports/{STORY-ID}-qa-report.md",
          content=formatted_report)
    Display: "✓ QA report generated"
```

**3. Generate gaps.json (FAILED Only):**

**MANDATORY if overall_status == "FAILED":**
```
Write(file_path="devforgeai/qa/reports/{STORY-ID}-gaps.json",
      content=JSON containing:
        - story_id
        - qa_result: "FAILED"
        - coverage_gaps: [{file, layer, current, target, gap, suggested_tests}]
        - anti_pattern_violations: [{file, line, type, severity, remediation}]
        - deferral_issues: [{item, violation_type, severity, remediation}]
        - remediation_sequence: [{phase, name, target_files, gap_count}]
)

# Verify creation
Glob(pattern="devforgeai/qa/reports/{STORY-ID}-gaps.json")
IF NOT found:
    HALT: "gaps.json not created - required for /dev remediation mode"
```

### Step 3.3.5: MANDATORY gaps.json Creation BEFORE Status Transition [RCA-002]

**Purpose:** Ensure gaps.json exists BEFORE any status update to "QA Failed". This is a mandatory prerequisite for the atomic status update protocol.

**CRITICAL:** Create gaps.json BEFORE status update in Step 3.4. This ensures `/dev --fix` remediation mode has the required gap file regardless of how the failure was detected.

**Source:** RCA-002 discovered that gaps.json creation was conditional on deep mode, not status transition. This step links gaps.json creation to status="QA Failed" unconditionally.

**When overall_status == "FAILED":**
```
# MANDATORY: Write gaps.json BEFORE status Edit [RCA-002]
# Idempotent: Write() overwrites existing gaps.json (not append)

Write(file_path="devforgeai/qa/reports/{STORY-ID}-gaps.json",
      content=JSON containing:
        - story_id: "{STORY-ID}"
        - qa_timestamp: "{ISO_8601}"
        - overall_status: "FAILED"
        - violations: [
            {type, severity, message, remediation}
          ]
)

# Verify creation
Glob(pattern="devforgeai/qa/reports/{STORY-ID}-gaps.json")
IF NOT found:
    HALT: "gaps.json not created - cannot proceed to status update"

Display: "✓ gaps.json created (required for QA Failed status)"
```

**Idempotent Behavior:** Write() tool overwrites existing file. Each QA run produces fresh gaps.json with current violations only.

**Proceed to Step 3.4 only after gaps.json creation confirmed.**

### Step 3.4: Story File Update [Atomic Update Protocol - STORY-177]

**Purpose:** Update story YAML frontmatter status using atomic update protocol with QA results.

---

#### Atomic Update Protocol (STORY-177)

**CRITICAL:** Status updates MUST follow this 5-step atomic sequence to prevent YAML frontmatter divergence.

**Protocol Sequence:**
1. read current status from yaml frontmatter (capture for rollback)
2. edit yaml frontmatter status field (FIRST - before second edit)
3. grep verify new status in frontmatter (MANDATORY)
4. edit append record entry (ONLY after step 3 passes)
5. rollback: restore original status if verification fails (skip step 4)

---

**Step 1: Read Current Status (capture for rollback):**
```
Read(file_path="devforgeai/specs/Stories/{STORY-ID}.story.md")

# Extract and store original status for potential rollback
original_status = extract_status_from_yaml(file_content)
# Example: original_status = "Dev Complete"

Display: "✓ Original status captured: {original_status}"
```

**Step 2: Edit YAML Frontmatter Status (FIRST - yaml first):**
```
# Determine target status
IF overall_status == "PASSED" OR overall_status == "PASS WITH WARNINGS":
    target_status = "QA Approved"
ELSE:
    target_status = "QA Failed"

# Edit YAML frontmatter status FIRST (before Step 4)
Edit(
    file_path="devforgeai/specs/Stories/{STORY-ID}.story.md",
    old_string="status: {original_status}",
    new_string="status: {target_status}"
)

Display: "✓ YAML status edited: {original_status} → {target_status}"
```

**Step 3: Grep Verify New Status (MANDATORY):**
```
# Verify the status update succeeded using Grep
Grep(
    pattern="^status: {target_status}",
    path="devforgeai/specs/Stories/{STORY-ID}.story.md",
    output_mode="content"
)

IF grep_result.found == false:
    # ROLLBACK TRIGGERED - verification failed
    GOTO Step 5 (Rollback)
ELSE:
    # status before history - Grep verification complete, proceed to history
    Display: "✓ Status verification passed: {target_status} confirmed in frontmatter"
    # Proceed to Step 4
```

**Step 4: Edit Append History Entry (ONLY after verification succeeds):**
**Reference:** `.claude/references/changelog-update-guide.md`
    Read(file_path=".claude/references/changelog-update-guide.md")

```
# IF verification succeeds THEN append history
# do not append if fail - skip history on fail
# 3-step sequence: Edit status -> Grep verify -> Edit history
# This step executes ONLY if Step 3 verification passed
# History entry is CONDITIONAL on successful status update

Author: `claude/qa-result-interpreter`
Phase/Action: `QA Light` or `QA Deep`
Change: `{result}: Coverage {pct}%, {violations} violations`

# Append changelog entry using Edit tool
Edit(
    file_path="devforgeai/specs/Stories/{STORY-ID}.story.md",
    old_string="| {last_date} | {last_author} | {last_action} | {last_change} | {last_files} |",
    new_string="| {last_date} | {last_author} | {last_action} | {last_change} | {last_files} |\n| {current_timestamp} | claude/qa-result-interpreter | QA {MODE} | {overall_status}: Coverage {coverage}%, {violations} violations | - |"
)

# Update Current Status display
Edit(
    file_path="devforgeai/specs/Stories/{STORY-ID}.story.md",
    old_string="**Current Status:** {original_status}",
    new_string="**Current Status:** {target_status}"
)

Display: "✓ Change Log entry appended"
$STORY_FILE_UPDATED = true
```

**Step 5: Rollback Restore Original on Verification Failure:**
```
# This step executes ONLY if Step 3 verification FAILED
# Restores original status, no history append on rollback
# Use Edit to restore original value

Edit(
    file_path="devforgeai/specs/Stories/{STORY-ID}.story.md",
    old_string="status: {target_status}",
    new_string="status: {original_status}"
)
# Edit restores original value - rollback complete

Display: "❌ Status verification FAILED - rolled back to: {original_status}"
Display: "   No history entry appended (rollback scenario)"
Display: "   Manual intervention required"

HALT: "Atomic status update failed - divergence prevented by rollback"
```

---

#### Single Edit Sequence Optimization (AC#4)

**when possible use single Edit to combine YAML update and append in one sequence.**
**Use single Edit when:** file structure allows, both updates in proximity.
**Fallback to separate edits when:** file structure prevents combined edit.
**Optimization rationale:** token efficiency - reduces tool calls.
```
# Optimized: Single Edit for both status and history (reduces tool calls)
# Use when story file structure allows combined edit

Edit(
    file_path="devforgeai/specs/Stories/{STORY-ID}.story.md",
    old_string="status: {original_status}\n...\n| {last_changelog_row} |",
    new_string="status: {target_status}\n...\n| {last_changelog_row} |\n| {new_changelog_row} |"
)

# Still require Grep verification after combined edit
Grep(pattern="^status: {target_status}", path="...")
```

**Fallback:** Use separate Edits (Steps 2 and 4) when single Edit not possible due to file structure.

---

**Validation Checkpoint (Atomic Update):**
- [ ] Original status captured (Step 1)?
- [ ] YAML frontmatter Edit executed FIRST (Step 2)?
- [ ] Grep verification executed (Step 3)?
- [ ] Verification passed (no rollback triggered)?
- [ ] History entry appended AFTER verification (Step 4)?
- [ ] Change Log entry has `claude/qa-result-interpreter` author?

IF any checkbox unchecked: HALT with "Atomic update incomplete"
IF rollback triggered: HALT with "Atomic update failed - rolled back"

**This step is ATOMIC - do NOT proceed to Phase 4 until story file verified.**

### Step 3.5: Invoke qa-result-interpreter

```
Task(subagent_type="qa-result-interpreter",
     prompt="Format QA results for display: {qa_data}")
```

**Display:**
```
✓ Phase 3 Complete: Reporting
  Result: [PASSED ✅ / FAILED ❌ / PASS WITH WARNINGS ⚠️]
  Report: [path / Not generated (light mode)]
  Story status: [Updated to QA Approved / QA Failed]
```

### Phase 3 Completion Checklist

**Before writing Phase 3 marker, verify you have:**

- [ ] Loaded qa-result-formatting.md (Step 3.0)
- [ ] Aggregated results from Phases 1-2 (Step 3.1)
- [ ] Invoked qa-result-interpreter subagent (Step 3.2)
- [ ] Generated QA report (Step 3.3)
- [ ] Updated story file if applicable (Step 3.4)
- [ ] Displayed final QA status to user

**IF any checkbox unchecked:** HALT and complete missing steps before Phase 4.

**Display to user:**
```
✓ Phase 3 Complete: Reporting | {overall_status}
```

### Phase 3 Marker Write

```
Write(file_path="devforgeai/qa/reports/{STORY_ID}/.qa-phase-3.marker",
      content="phase: 3\nstory_id: {STORY_ID}\nmode: {MODE}\ntimestamp: {ISO_8601}\nstatus: complete")

Display: "✓ Phase 3 marker written"
Display: "Phase 3 ✓ | Reporting | {overall_status}"
```

**Update execution tracker:**

```
TodoWrite({
  todos: [
    { content: "Phase 0: Setup", status: "completed", activeForm: "Phase 0 complete" },
    { content: "Phase 1: Validation", status: "completed", activeForm: "Phase 1 complete" },
    { content: "Phase 2: Analysis", status: "completed", activeForm: "Phase 2 complete" },
    { content: "Phase 3: Reporting", status: "completed", activeForm: "Phase 3 complete" },
    { content: "Phase 4: Cleanup", status: "in_progress", activeForm: "Running Phase 4: Cleanup" }
  ]
})
```

---

## Phase 4: Cleanup

### Pre-Flight: Verify Phase 3 Complete

```
Glob(pattern="devforgeai/qa/reports/{STORY_ID}/.qa-phase-3.marker")

IF marker file NOT found:
    CRITICAL ERROR: "Phase 3 not verified complete"
    HALT: "Phase 4 cannot execute without Phase 3 completion"
    Display: "Previous phase (Phase 3) must complete successfully before starting Phase 4"
    Instruction: "Start workflow from Phase 0. Run setup first."
    Exit: Code 1 (phase sequencing violation)

Display: "✓ Phase 3 verified complete - Phase 4 preconditions met"
```

### ⚠️ CHECKPOINT: Phase 4 Reference Loading [MANDATORY]

**You MUST execute ALL steps before proceeding to phase content.**

**Step 4.0: Load Workflow Reference (REQUIRED)**
```
Read(file_path=".claude/skills/devforgeai-qa/references/feedback-hooks-workflow.md")
```

**This reference contains the complete workflow. Execute ALL steps from the reference file.**

**After loading:** Proceed to Step 4.1 (Release Lock File)

**IF you skip this step:** You will execute the phase incorrectly and miss mandatory steps.

---

**Purpose:** Release locks, invoke feedback hooks, display final summary.

### Step 4.1: Release Lock File

```
IF config.concurrency.locking_enabled:
    Remove(file_path="{story_paths.results_dir}/.qa-lock")
    Display: "✓ Lock released for {STORY_ID}"
```

### Step 4.2: Invoke Feedback Hooks

**Reference:** `references/feedback-hooks-workflow.md`
    Read(file_path=".claude/skills/devforgeai-qa/references/feedback-hooks-workflow.md")

```
# Map QA result to hook status
IF overall_status == "PASSED": STATUS = "success"
ELIF overall_status == "FAILED": STATUS = "failure"
ELSE: STATUS = "partial"

# Check and invoke hooks (non-blocking)
Bash(command="devforgeai-validate check-hooks --operation=qa --status=$STATUS")
IF exit_code == 0:
    Bash(command="devforgeai-validate invoke-hooks --operation=qa --story=$STORY_ID")
```

### Step 4.3: Execution Summary [MANDATORY - HALT ON INCOMPLETE]

**Purpose:** Enforce visibility of all phase executions before workflow completion.

**Constitution Alignment:** Quality gates MUST block on violations (architecture-constraints.md line 106)

Display the following summary (CANNOT be skipped):

```
╔══════════════════════════════════════════════════════════════╗
║                    QA EXECUTION SUMMARY                      ║
╠══════════════════════════════════════════════════════════════╣
║  Story: {STORY_ID}                                           ║
║  Mode: {MODE}                                                ║
╠══════════════════════════════════════════════════════════════╣
║  PHASE EXECUTION STATUS:                                     ║
║  - [x] Phase 0: Setup (Lock: {YES/NO})                       ║
║  - [x] Phase 1: Validation (Traceability: {score}%)          ║
║  - [x] Phase 2: Analysis (Validators: {count}/3)             ║
║  - [x] Phase 3: Reporting (Status: {status})                 ║
║  - [x] Phase 4: Cleanup (Hooks: {status})                    ║
╠══════════════════════════════════════════════════════════════╣
║  Story File Updated: {YES/NO}                                ║
║  Result: {PASSED/FAILED}                                     ║
╚══════════════════════════════════════════════════════════════╝
```

**Enforcement Logic:**

```
# Count unchecked phases
unchecked_count = count_unchecked_phases()

IF unchecked_count > 0:
    Display: "⚠️ WARNING: {unchecked_count} phases may have been skipped"

    AskUserQuestion:
        Question: "Phases appear incomplete. How should I proceed?"
        Header: "Incomplete Execution"
        Options:
            - label: "Re-run skipped phases now"
              description: "Return to first skipped phase and complete workflow"
            - label: "Continue with incomplete execution (NOT RECOMMENDED)"
              description: "Proceed despite missing phases - may cause issues"
            - label: "Abort QA validation"
              description: "Stop workflow and investigate manually"
        multiSelect: false

    IF user chooses "Re-run": GOTO first skipped phase
    IF user chooses "Continue": Log warning, proceed with caution
    IF user chooses "Abort": HALT workflow

IF unchecked_count == 0:
    Display: "✓ All phases complete - No skipped steps detected"
```

**Validation Checkpoint:**
- [ ] Execution summary displayed?
- [ ] All phases marked complete?
- [ ] Story file update confirmed?
- [ ] **IF QA FAILED: gaps.json exists?** [RCA-002]

**gaps.json Verification (Conditional):**
```
IF overall_status == "FAILED":
    gaps_file = Glob(pattern="devforgeai/qa/reports/{STORY-ID}-gaps.json")
    IF NOT gaps_file:
        Display: "❌ CRITICAL: gaps.json missing for failed QA"
        HALT: "Create gaps.json before completing QA workflow"
    ELSE:
        Display: "✓ gaps.json verified: {gaps_file}"
ELSE:
    # PASSED or PASS WITH WARNINGS - skip gaps.json check
    Display: "✓ gaps.json check skipped (QA passed)"
```

IF any checkbox unchecked: HALT with "Execution incomplete"

---

### Step 4.4: Display Final Summary

```
Display:
╔════════════════════════════════════════════════════════╗
║                    QA VALIDATION COMPLETE              ║
╠════════════════════════════════════════════════════════╣
║ Story: {STORY_ID}                                      ║
║ Mode: {mode}                                           ║
║ Result: {overall_status}                               ║
╠════════════════════════════════════════════════════════╣
║ Coverage:                                              ║
║   Business Logic: {biz}% | Application: {app}%         ║
║   Infrastructure: {infra}% | Overall: {overall}%       ║
╠════════════════════════════════════════════════════════╣
║ Violations: {critical} CRITICAL | {high} HIGH          ║
║             {medium} MEDIUM | {low} LOW                ║
╠════════════════════════════════════════════════════════╣
║ Next Steps:                                            ║
║   [If PASSED] Ready for /release {STORY_ID}            ║
║   [If FAILED] Run /dev {STORY_ID} for remediation      ║
╚════════════════════════════════════════════════════════╝
```

### Phase 4 Completion Checklist

**Before writing Phase 4 marker, verify you have:**

- [ ] Released lock file (Step 4.1)
- [ ] Cleaned up temporary files (Step 4.2)
- [ ] Archived session checkpoint (Step 4.3)
- [ ] Displayed cleanup confirmation

**IF any checkbox unchecked:** HALT and complete missing steps before QA completion.

**Display to user:**
```
✓ Phase 4 Complete: Cleanup | Complete
```

### Phase 4 Marker Write

```
Write(file_path="devforgeai/qa/reports/{STORY_ID}/.qa-phase-4.marker",
      content="phase: 4\nstory_id: {STORY_ID}\nmode: {MODE}\ntimestamp: {ISO_8601}\nstatus: complete")

Display: "✓ Phase 4 marker written"
Display: "Phase 4 ✓ | Cleanup | Complete"
Display: "✓ QA workflow complete - all 5 phase markers written"
```

**Update execution tracker:**

```
TodoWrite({
  todos: [
    { content: "Phase 0: Setup", status: "completed", activeForm: "Phase 0 complete" },
    { content: "Phase 1: Validation", status: "completed", activeForm: "Phase 1 complete" },
    { content: "Phase 2: Analysis", status: "completed", activeForm: "Phase 2 complete" },
    { content: "Phase 3: Reporting", status: "completed", activeForm: "Phase 3 complete" },
    { content: "Phase 4: Cleanup", status: "completed", activeForm: "Phase 4 complete" }
  ]
})
```

### Step 4.5: Marker Cleanup [CONDITIONAL - QA PASSED ONLY]

**Purpose:** Clean up marker files after successful QA validation to prevent file proliferation.

**Trigger:** Execute ONLY when overall_status == "PASSED"

```
IF overall_status == "PASSED":
    # Get all phase markers for this story
    Glob(pattern="devforgeai/qa/reports/{STORY_ID}/.qa-phase-*.marker")

    FOR each marker_file in results:
        Bash(command="rm {marker_file}")

    Display: "✓ Phase markers cleaned up for {STORY_ID}"

ELSE:
    Display: "⚠️ QA FAILED - Markers retained for debugging and resume capability"
```

**Rationale:**
- PASSED: Markers no longer needed - clean up to prevent file proliferation
- FAILED: Markers retained for debugging and to enable resume from last completed phase

---

## Automation Scripts

**6 Python scripts** in `scripts/`:
1. generate_coverage_report.py
2. detect_duplicates.py
3. analyze_complexity.py
4. security_scan.py
5. validate_spec_compliance.py
6. generate_test_stubs.py

**See `references/automation-scripts.md`** for usage.
    Read(file_path=".claude/skills/devforgeai-qa/references/automation-scripts.md")

---

## Subagents

| Subagent | Phase | Purpose |
|----------|-------|---------|
| anti-pattern-scanner | 2.1 | Detect 6 violation categories |
| test-automator | 2.2 | Coverage and quality analysis |
| code-reviewer | 2.2 | Code quality review |
| security-auditor | 2.2 | Security vulnerability scan |
| deferral-validator | 2.3 | Validate DoD deferrals |
| qa-result-interpreter | 3.5 | Format display output |

---

## Integration

**Invoked by:** devforgeai-development, /qa command, devforgeai-orchestration
**Invokes:** 6 subagents listed above
**Outputs to:** devforgeai-development (via gaps.json), devforgeai-release, user

---

## Success Criteria

**Light:** Build passes, tests pass, no CRITICAL, deferrals valid, <10K tokens
**Deep:** Coverage thresholds met, no CRITICAL/HIGH, spec compliant, quality acceptable, deferrals valid, status="QA Approved", <35K tokens

---

## Reference Files

**Single consolidated workflow (deep mode):** `references/deep-validation-workflow.md`
    Read(file_path=".claude/skills/devforgeai-qa/references/deep-validation-workflow.md")

**Individual references (21 total):**
- Workflows: parameter-extraction, dod-protocol, coverage-analysis-workflow, anti-pattern-detection-workflow, parallel-validation, spec-compliance-workflow, code-quality-workflow, report-generation, feedback-hooks-workflow, story-update-workflow, marker-operations
- Guides: coverage-analysis, anti-pattern-detection, deferral-decision-tree, language-specific-tooling, qa-result-formatting-guide, quality-metrics, security-scanning, spec-validation, traceability-validation-algorithm, test-isolation-service

---

**Token efficiency:** Entry ~1.5K, Light ~3.8K, Deep ~8K (improved via phase consolidation and single workflow file)
