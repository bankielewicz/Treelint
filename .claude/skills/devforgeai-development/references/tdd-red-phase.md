# Phase 02: Test-First Design (Red Phase)

**Purpose:** Write failing tests from acceptance criteria before implementation.

**Execution Order:** After Phase 01 (Pre-Flight Validation) completes

**Expected Outcome:** All tests RED (failing), ready for implementation

**Token Cost:** ~800 tokens in skill context (~40,000 in isolated subagent context)

---

## Phase Progress Indicator

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 02/10: Test-First Design - Red Phase (10% → 20% complete)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Display this indicator at the start of Phase 02 execution.**

---

## Overview

The Red phase is the foundation of TDD: create tests that define expected behavior before writing implementation code.

**Core Principle:** Tests document requirements and guide implementation.

---

## Story Type Skip Check [MANDATORY FIRST] (STORY-126)

**Purpose:** Skip Phase 02 for `refactor` story types (tests already exist).

**When to execute:** Before any Phase 02 processing.

```
# Check if Phase 02 should be skipped based on story type
# $STORY_TYPE set in Phase 01.6.5 (Pre-Flight Validation)

IF $STORY_TYPE == "refactor":
    Display: ""
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: "  ℹ️  SKIPPING PHASE 02: Story Type 'refactor'"
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: ""
    Display: "Reason: Refactor stories work with existing tests."
    Display: "        Tests should already be GREEN before refactoring begins."
    Display: ""
    Display: "Proceeding to Phase 03 (Green - Implementation)..."
    Display: ""

    # Skip Phase 02 entirely
    GOTO Phase 03

    RETURN
```

---

## Remediation Mode Check [MANDATORY AFTER SKIP CHECK]

**CRITICAL:** Before executing normal Phase 02, check if remediation mode is active.

```
# Check remediation mode flag from Phase 01 Step h.1.
IF $REMEDIATION_MODE == true:

    Display: ""
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: "  🔧 REMEDIATION MODE - Phase 02R (Targeted Tests)"
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: ""

    # Load remediation workflow instead of normal Phase 02
    Read(file_path=".claude/skills/devforgeai-development/references/qa-remediation-workflow.md")

    # Execute Phase 02R: Targeted Test Generation from qa-remediation-workflow.md
    # This generates tests ONLY for $QA_COVERAGE_GAPS files, not full story

    RETURN after Phase 02R completes
    # Do NOT execute normal Phase 02 below

ELSE:
    # Normal Phase 02 - proceed with full story test generation
    Display: "Proceeding with normal Phase 02 (full story test generation)"
```

**Why this matters:**
- In remediation mode, test-automator receives `$QA_COVERAGE_GAPS` with specific files/suggestions
- In normal mode, test-automator receives full story acceptance criteria
- Wrong mode = wasted tokens + wrong test scope

---

## Phase 02: Test-First Design (Red Phase)

**Delegate test generation to test-automator subagent.**

### Step 0: Read source-tree.md Context [MANDATORY - STORY-206]

**Purpose:** Extract test directory context from source-tree.md for defense in depth validation.

**Why this step exists (Defense in Depth):**
1. **Layer 1 (This Step):** Skill reads source-tree.md and sets context markers
2. **Layer 2 (test-automator):** Subagent reads source-tree.md independently (STORY-203)
3. **Redundant validation:** If either layer fails, the other catches violations

```
Read(file_path="devforgeai/specs/context/source-tree.md")
```

**Extract test directory based on module path pattern:**

| Module Path | Expected Test Directory | Rule Source |
|-------------|------------------------|-------------|
| `installer/*` | `installer/tests/` | source-tree.md lines 384-443 |
| `.claude/scripts/devforgeai_cli/*` | `.claude/scripts/devforgeai_cli/tests/` | source-tree.md lines 299-309 |
| `src/*` | `tests/` (default) | source-tree.md lines 352 |

**Pattern Extraction Logic:**
```
# Determine module under test from story Technical Specification
$MODULE_PATH = story.technical_specification.components[0].file_path

# Extract test directory per source-tree.md rules
IF $MODULE_PATH starts with "installer/":
    $TEST_DIRECTORY = "installer/tests/"
ELIF $MODULE_PATH starts with ".claude/scripts/devforgeai_cli/":
    $TEST_DIRECTORY = ".claude/scripts/devforgeai_cli/tests/"
ELIF $MODULE_PATH starts with "src/":
    $TEST_DIRECTORY = "tests/"
ELSE:
    $TEST_DIRECTORY = "tests/"  # Default fallback
```

**Set context markers BEFORE test-automator invocation:**

```markdown
**Module Under Test:** ${MODULE_PATH}
**Expected Test Directory:** ${TEST_DIRECTORY} (per source-tree.md)
**Constraint:** All generated tests must be placed in ${TEST_DIRECTORY}
```

---

### Step 1: Invoke test-automator Subagent [MANDATORY]

```
Task(
  subagent_type="test-automator",
  description="Generate failing tests from acceptance criteria",
  prompt="Generate comprehensive test suite for this story.

  Story content is already loaded in conversation (via @file reference from /dev command).

  Extract from story:
  1. Acceptance criteria (Given/When/Then scenarios)
  2. Technical specification (API contracts, data models, business rules)
  3. Non-functional requirements (performance, security)

  Context files available:
  - devforgeai/specs/context/source-tree.md (test file placement rules)
  - devforgeai/specs/context/coding-standards.md (test patterns, AAA format)
  - devforgeai/specs/context/tech-stack.md (test framework: {TEST_FRAMEWORK})

  **source-tree.md Context for test file placement:**
  - Module Under Test: ${MODULE_PATH}
  - Expected Test Directory: ${TEST_DIRECTORY} (per source-tree.md)
  - Constraint: All generated tests must be placed in ${TEST_DIRECTORY}

  Generate tests that:
  1. Cover all acceptance criteria
  2. Follow AAA pattern (Arrange, Act, Assert)
  3. Use test framework: {TEST_FRAMEWORK}
  4. Place tests according to source-tree.md rules in ${TEST_DIRECTORY}
  5. Initially FAIL (Red phase of TDD)

  Test command to verify: {TEST_COMMAND}

  Return:
  - Test files created (paths and content summary)
  - Test count (unit/integration/e2e)
  - Initial test run status (all should fail)"
)
```

### Step 2: Parse Subagent Response [MANDATORY]

```javascript
result = extract_from_subagent_output(response)

tests_created = result["test_files"]
test_count = result["test_count"]

Display: "✓ Phase 02 (Red): Tests generated by test-automator"
Display: "  - Unit tests: {test_count['unit']}"
Display: "  - Integration tests: {test_count['integration']}"
Display: "  - Files created: {len(tests_created)}"

FOR file in tests_created:
    Display: "    • {file['path']}"
```

### Step 3: Verify Tests Fail (Red Phase) [MANDATORY]

```
Bash(command=TEST_COMMAND)

IF all tests fail (as expected):
    Display: "✓ RED phase confirmed - all tests failing as expected"
    Display: "  Ready for Phase 03 (Green) - implementation"

ELIF some tests pass:
    Display: "⚠️ Warning: Some tests passing unexpectedly"
    Display: "  This may indicate existing implementation or incorrect test design"
    Display: "  Proceeding to Phase 03..."

ELSE (tests not runnable):
    Display: "❌ ERROR: Tests not runnable"
    Display: "  Review test-automator output for errors"
    HALT development
```

---

### Step 4: Technical Specification Coverage Validation & Deferral Pre-Approval [MANDATORY]

**Purpose:** Ensure ALL components in Technical Specification have corresponding tests, and obtain explicit user approval for any coverage gaps.

**Why this step exists:** Prevents autonomous deferrals where implementation details are skipped without user knowledge, leading to minimal stub implementations that pass tests but don't match the story specification.

**Execution:** After tests generated (Step 1-3), before proceeding to Phase 03 (Green).

---

#### 4.1 Extract Technical Specification Components

Parse the story file's Technical Specification section to identify all required components.

**Story file already loaded:** The @file reference from /dev command loaded story content into conversation context.

**RCA-006 Phase 2: Dual Format Support**

**Detect format version:**
```python
# Check frontmatter for format_version
frontmatter = extract_yaml_frontmatter(story_content)
format_version = frontmatter.get("format_version", "1.0")

Display: f"Detected story format: v{format_version}"
```

**Parse accordingly:**

**If format_version == "2.0" (Structured YAML):**
```python
# Parse YAML directly (95%+ accuracy)
tech_spec_match = re.search(r"## Technical Specification\s+```yaml\s+(.*?)\s+```",
                             story_content, re.DOTALL)
tech_spec = yaml.safe_load(tech_spec_match.group(1))
components = tech_spec["technical_specification"]["components"]

# Direct extraction (no pattern matching needed)
for component in components:
    component_type = component["type"]          # Service, Worker, API, etc.
    component_name = component["name"]          # AlertDetectionWorker
    file_path = component["file_path"]          # src/Workers/AlertDetectionWorker.cs
    requirements = component.get("requirements", [])  # Structured requirements

    # Add to checklist
    component_checklist.append({
        "type": component_type,
        "name": component_name,
        "path": file_path,
        "requirements": requirements
    })

# Also extract business_rules and non_functional_requirements
business_rules = tech_spec["technical_specification"].get("business_rules", [])
nfrs = tech_spec["technical_specification"].get("non_functional_requirements", [])
```

**Benefit:** Deterministic parsing, 95%+ accuracy, no ambiguity

**If format_version == "1.0" (Freeform Text) - LEGACY:**

Use existing freeform parsing (below). Extract components from these subsections:

1. **File Structure** - Parse directory tree for file paths
   ```
   Example:
   src/
   └── Workers/
       ├── AlertDetectionWorker.cs
       └── EmailSenderWorker.cs

   Components identified: 2 workers
   ```

2. **Service Implementation Pattern** - Extract classes/methods from code examples
   ```
   Example:
   "AlertDetectionWorker.cs - Poll database for alerts"

   Components identified: AlertDetectionWorker class with polling logic
   ```

3. **Configuration Requirements** - Identify config files needed
   ```
   Example:
   "appsettings.json must contain ConnectionStrings.OmniWatchDb"

   Components identified: appsettings.json file, ConnectionStrings section
   ```

4. **Logging Requirements** - Identify logging setup
   ```
   Example:
   "Configure Serilog with File, EventLog, Database sinks"

   Components identified: Serilog configuration, 3 sinks
   ```

5. **Data Models** - Extract entities/tables
   ```
   Example:
   "Alert table with Id, Message, Severity, CreatedAt"

   Components identified: Alert entity, database schema
   ```

6. **Business Rules** - Extract numbered rules requiring validation
   ```
   Example:
   "Rule 1: Alert severity must be Info/Warning/Error"

   Components identified: Severity validation logic
   ```

**Build component checklist:**
```python
TECH_SPEC_COMPONENTS = [
    {
        "type": "Worker",
        "name": "AlertDetectionWorker",
        "file": "src/Workers/AlertDetectionWorker.cs",
        "requirements": [
            "Must poll database for alerts",
            "Must run continuous loop with cancellation",
            "Must handle exceptions without stopping"
        ]
    },
    {
        "type": "Configuration",
        "name": "appsettings.json",
        "file": "src/Project.Service/appsettings.json",
        "requirements": [
            "Must contain ConnectionStrings.OmniWatchDb",
            "Must contain AlertingService.PollingIntervalSeconds"
        ]
    },
    {
        "type": "Logging",
        "name": "Serilog",
        "file": "Program.cs",
        "requirements": [
            "Must configure Serilog with File sink",
            "Must configure Serilog with EventLog sink",
            "Must configure Serilog with Database sink"
        ]
    }
]
```

---

#### 4.2 Compare Generated Tests vs. Technical Specification

For each component in `TECH_SPEC_COMPONENTS`, check if tests exist.

**Scan test files generated in Steps 1-2:**
```
Glob(pattern="tests/**/*.cs")  # or *.py, *.js based on tech stack
```

**For each test file, search for component coverage:**
```
Read(file_path="{test_file}")
Grep(pattern="AlertDetectionWorker", path="{test_file}")
```

**Build coverage map:**
```python
COVERAGE_MAP = {
    "AlertDetectionWorker": {
        "tests_found": [
            "AlertDetectionWorkerTests.cs: StartAsync_WithValidConfig_ShouldStartPolling"
        ],
        "requirements_covered": [
            "✅ Must start polling (interface test exists)"
        ],
        "requirements_missing": [
            "❌ Must run continuous loop with cancellation",
            "❌ Must handle exceptions without stopping"
        ],
        "coverage_percentage": 33  # 1 of 3 requirements covered
    },
    "appsettings.json": {
        "tests_found": [],
        "requirements_covered": [],
        "requirements_missing": [
            "❌ Must contain ConnectionStrings.OmniWatchDb",
            "❌ Must contain AlertingService.PollingIntervalSeconds"
        ],
        "coverage_percentage": 0  # 0 of 2 requirements covered
    },
    "Serilog": {
        "tests_found": [],
        "requirements_covered": [],
        "requirements_missing": [
            "❌ Must configure File sink",
            "❌ Must configure EventLog sink",
            "❌ Must configure Database sink"
        ],
        "coverage_percentage": 0  # 0 of 3 requirements covered
    }
}
```

**Calculate overall coverage:**
```python
TOTAL_REQUIREMENTS = 8
REQUIREMENTS_COVERED = 1
COVERAGE_GAP_COUNT = 7
COVERAGE_PERCENTAGE = 12.5%
```

---

#### 4.3 Present Coverage Analysis to User

**If coverage gaps detected (COVERAGE_GAP_COUNT > 0):**

Display comprehensive analysis:
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  🔍 TECHNICAL SPECIFICATION COVERAGE ANALYSIS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Story: {STORY_ID}
Phase: 02 (RED - Test Generation)

Technical Specification Components: 3
Total Requirements: 8
Tests Generated: 1
Coverage: 12.5% ⚠️

COVERAGE GAPS DETECTED: 7 requirements lack tests

Gap Summary:
1. AlertDetectionWorker (2 gaps)
   ✅ Polling starts (interface test exists)
   ❌ Continuous loop with cancellation (NO TEST)
   ❌ Exception handling (NO TEST)

2. appsettings.json (2 gaps)
   ❌ ConnectionStrings.OmniWatchDb (NO TEST)
   ❌ AlertingService.PollingIntervalSeconds (NO TEST)

3. Serilog Configuration (3 gaps)
   ❌ File sink configured (NO TEST)
   ❌ EventLog sink configured (NO TEST)
   ❌ Database sink configured (NO TEST)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

⚠️ PHASE 02 INCOMPLETE: Technical specification not fully covered by tests

Proceeding to Phase 03 with these gaps will result in:
• Minimal implementations (stubs that pass interface tests only)
• Deferred work accumulating silently
• Technical debt not documented

USER DECISION REQUIRED for each gap (next step).
```

---

#### 4.4 Request User Decision for EACH Gap

**MANDATORY:** Use AskUserQuestion for EVERY coverage gap.

**Strategy:** Batch gaps by component to reduce question count (recommended: max 5 questions total).

**For each component with gaps:**

```python
AskUserQuestion(
    questions=[
        {
            "question": "AlertDetectionWorker has 2 missing tests. How should we proceed?",
            "header": "Worker Tests",
            "multiSelect": False,
            "options": [
                {
                    "label": "Generate tests now",
                    "description": "Add tests for continuous loop and exception handling (~15 min). RECOMMENDED - Prevents technical debt."
                },
                {
                    "label": "Defer to follow-up story",
                    "description": "Skip tests now, create tracking story for later implementation. Creates technical debt."
                },
                {
                    "label": "Remove from scope",
                    "description": "Update story to remove these requirements. Requires ADR for scope change."
                }
            ]
        }
    ]
)
```

**Capture user response:**
```python
USER_DECISION = {
    "component": "AlertDetectionWorker",
    "gaps": [
        "Continuous loop with cancellation",
        "Exception handling"
    ],
    "decision": "generate_tests_now" | "defer" | "remove_scope",
    "timestamp": "2025-11-07T10:30:00Z"
}
```

---

#### 4.5 Process User Decision

**Decision 1: Generate tests now**

```python
if USER_DECISION["decision"] == "generate_tests_now":
    # Re-invoke test-automator with specific requirements
    Task(
        subagent_type="test-automator",
        description="Generate missing worker tests",
        prompt=f"""
        Generate additional tests for AlertDetectionWorker:

        Required tests:
        1. Test: Worker runs in continuous loop until cancellation
           - Verify: while (!cancellationToken.IsCancellationRequested) executes
           - Verify: Worker stops when cancellation requested

        2. Test: Worker handles exceptions without stopping
           - Verify: Exception in polling doesn't crash worker
           - Verify: Worker continues after exception
           - Verify: Exception logged

        Follow AAA pattern, use existing test file structure.
        Add to existing test suite in tests/Workers/AlertDetectionWorkerTests.cs
        """
    )

    # Add to Phase 02 test suite
    # Continue to next gap
```

**Decision 2: Defer to follow-up story**

```python
elif USER_DECISION["decision"] == "defer":
    # Ask for follow-up story reference
    AskUserQuestion(
        questions=[
            {
                "question": "Which follow-up story will handle AlertDetectionWorker tests?",
                "header": "Deferral Tracking",
                "multiSelect": False,
                "options": [
                    {
                        "label": "Create new story",
                        "description": "Auto-generate STORY-XXX for deferred work"
                    },
                    {
                        "label": "Existing story",
                        "description": "I'll provide story ID (STORY-XXX)"
                    }
                ]
            }
        ]
    )

    if response == "create_new_story":
        FOLLOW_UP_STORY_ID = generate_next_story_id()  # e.g., STORY-003

        # Create tracking story
        Skill(command="devforgeai-story-creation")
        # With context: Deferred work from STORY-002

    elif response == "existing_story":
        # Ask for story ID
        FOLLOW_UP_STORY_ID = user_input("Enter story ID: ")

    # Document deferral
    DEFERRAL_RECORD = {
        "component": "AlertDetectionWorker",
        "requirements": [
            "Continuous loop with cancellation",
            "Exception handling"
        ],
        "reason": "User deferred to follow-up story",
        "follow_up_story": FOLLOW_UP_STORY_ID,
        "approved_by": "user",
        "approved_at": "2025-11-07T10:30:00Z"
    }

    # Add to Phase 06 tracking
    # Continue to next gap
```

**Decision 3: Remove from scope**

```python
elif USER_DECISION["decision"] == "remove_scope":
    # Require ADR for scope change
    Display: """
    ⚠️ SCOPE CHANGE REQUIRES ADR

    Removing requirements from Technical Specification is an architectural decision.

    Next steps:
    1. Create ADR documenting why requirements removed
    2. Update story Technical Specification section
    3. Update Definition of Done checklist

    Proceed with ADR creation? (Y/n)
    """

    if user_confirms():
        # Guide ADR creation
        Display: """
        Create ADR with:
        - Title: "Remove AlertDetectionWorker continuous loop requirement"
        - Context: Why requirement was in original spec
        - Decision: Why removing it now
        - Consequences: Impact on system behavior

        After ADR created, update story file manually.
        """

        # HALT Phase 02, wait for manual updates
        raise RequiresManualIntervention(
            "Scope change requires ADR creation and story update. "
            "Re-run /dev after updates complete."
        )
```

---

#### 4.6 Repeat for All Gaps

**Process EACH component with gaps:**

```python
for component in COVERAGE_MAP:
    if component["requirements_missing"]:
        user_decision = ask_user_question_for_component(component)
        process_decision(user_decision)

        # Update coverage map after processing
        COVERAGE_MAP[component]["decision"] = user_decision
```

**Track decisions:**
```python
DECISIONS_LOG = [
    {
        "component": "AlertDetectionWorker",
        "decision": "generate_tests_now",
        "timestamp": "2025-11-07T10:30:00Z"
    },
    {
        "component": "appsettings.json",
        "decision": "defer",
        "follow_up_story": "STORY-003",
        "timestamp": "2025-11-07T10:32:00Z"
    },
    {
        "component": "Serilog",
        "decision": "defer",
        "follow_up_story": "STORY-003",
        "timestamp": "2025-11-07T10:33:00Z"
    }
]
```

---

#### 4.7 Validate All Gaps Addressed

**Enforcement check:**

```python
unapproved_gaps = [
    gap for gap in COVERAGE_MAP.values()
    if gap["requirements_missing"] and "decision" not in gap
]

if unapproved_gaps:
    raise ValidationError(
        f"❌ CANNOT PROCEED TO PHASE 03: {len(unapproved_gaps)} unapproved coverage gaps\n\n"
        f"All gaps must have user decision (generate/defer/remove).\n\n"
        f"Gaps without decisions:\n" +
        "\n".join([f"- {gap['name']}" for gap in unapproved_gaps])
    )
```

**Success message:**
```
✅ STEP 4 COMPLETE: Technical Specification Coverage Validated

All {COVERAGE_GAP_COUNT} gaps addressed:
- {generate_count} tests generated
- {defer_count} deferred to follow-up stories
- {remove_count} removed from scope (ADR required)

Decisions documented in workflow history.
Proceeding to Phase 03 (GREEN - Implementation)...
```

---

#### 4.8 Document Decisions in Story File

**Update story workflow history:**

```markdown
## Workflow History

### 2025-11-07 10:30 - Phase 02 (RED) - Coverage Validation
- Technical Specification components: 3
- Total requirements: 8
- Coverage gaps detected: 7

**User Decisions:**
1. AlertDetectionWorker (2 gaps)
   - Decision: Generate tests now
   - Tests added: Continuous loop test, Exception handling test
   - Duration: 15 minutes

2. appsettings.json (2 gaps)
   - Decision: Defer to STORY-003
   - Reason: Configuration setup deferred to infrastructure story
   - Follow-up story: STORY-003

3. Serilog (3 gaps)
   - Decision: Defer to STORY-003
   - Reason: Logging setup deferred to infrastructure story
   - Follow-up story: STORY-003

**Phase 02 Result:** PASSED with 1 deferral decision (deferred 5/8 requirements)
```

---

#### 4.9 Special Case: Zero Gaps

**If COVERAGE_GAP_COUNT == 0:**

```
✅ EXCELLENT: Technical Specification 100% Covered

All components in Technical Specification have corresponding tests:
- AlertDetectionWorker (3/3 requirements tested)
- appsettings.json (2/2 requirements tested)
- Serilog (3/3 requirements tested)

No deferrals needed. Proceeding to Phase 03...
```

**Skip Steps 4.4-4.7** (no user interaction needed, proceed directly to Phase 03)

---

#### Step 4 Success Criteria

- [ ] All Technical Specification components extracted
- [ ] Coverage map built comparing tests vs. requirements
- [ ] User presented with clear coverage analysis
- [ ] User made decision for EVERY coverage gap
- [ ] All decisions documented in workflow history
- [ ] Zero unapproved gaps remaining
- [ ] Phase 02 completes successfully

---

#### Implementation Notes

**Token budget:** Step 4 adds ~5-10K tokens to Phase 02 (coverage analysis + user interaction)

**Time impact:** Adds 10-20 minutes to Phase 02 (user decision time)

**User experience:**
- **Positive:** Full transparency, explicit control
- **Negative:** More questions to answer
- **Mitigation:** Batch questions by component (max 5 total)

**Edge cases:**
- Technical Specification empty/incomplete → HALT, require story update
- User selects "defer" for ALL gaps → Allow but warn about 100% deferral
- User cancels mid-step → Save partial decisions, allow resume

---

## Subagents Invoked

**test-automator:**
- Generates failing tests from acceptance criteria
- Places tests according to source-tree.md
- Follows coding-standards.md test patterns
- Uses appropriate test framework from tech-stack.md

---

### Step 5: Update AC Verification Checklist (Phase 02 Items) [NEW - RCA-011]

**Purpose:** Check off AC items related to test generation (real-time progress tracking)

**Execution:** After Step 4 completes, before Phase 02 checkpoint validation

**Load AC Checklist Update Workflow:**
```
Read(file_path=".claude/skills/devforgeai-development/references/ac-checklist-update-workflow.md")
```

**Identify Phase 02 AC Items:**

Search story file for AC Verification Checklist section:
```
Grep(pattern="Phase.*: 1", path="${STORY_FILE}", output_mode="content", -B=1)
```

**Common Phase 02 items:**
- [ ] Unit tests ≥{N} generated
- [ ] Integration tests ≥{N} generated
- [ ] All ACs have corresponding tests
- [ ] Test files created in correct location
- [ ] Edge case tests included
- [ ] Test coverage ≥{N}% planned

**Update Procedure:**

```
FOR each item with "**Phase:** 1" marker:
  Validate item completion:
    - Check test count matches target
    - Verify test files exist
    - Confirm coverage planning done

  IF item completed:
    Edit(
      file_path="${STORY_FILE}",
      old_string="- [ ] {item_text} - **Phase:** 1",
      new_string="- [x] {item_text} - **Phase:** 1"
    )

    Display: "  ✓ AC item checked: {item_brief}"
```

**Display Progress:**
```
Display: "
Phase 02 AC Checklist Update:
  ✓ {count} items checked
  - {item 1 summary}
  - {item 2 summary}
  ...

AC Progress: {checked}/{total} items complete ({percentage}%)
"
```

**Graceful Skip:**
```
IF AC Verification Checklist section not found in story:
  Display: "ℹ️ Story uses DoD-only tracking (AC Checklist not present)"
  Skip AC checklist updates
  Continue to Phase 02 Checkpoint
```

**Performance:** ~30-60 seconds for 3-5 items (Edit operations + validation)

---

## ✅ PHASE 02 COMPLETION CHECKPOINT

**Before proceeding to Phase 03 (Implementation - Green Phase), verify ALL steps executed:**

### Mandatory Steps Executed

- [ ] **Step 1:** test-automator subagent invoked
  - Verification: Tests generated for all acceptance criteria
  - Output: Test file paths and test counts displayed

- [ ] **Step 2:** Subagent response parsed and displayed
  - Verification: Test counts shown (unit/integration/e2e)
  - Output: Files created listed

- [ ] **Step 3:** Tests verified RED (all failing as expected)
  - Verification: Executed {TEST_COMMAND}, confirmed all tests fail
  - Output: Red phase confirmed message displayed

- [ ] **Step 4:** Technical Specification Coverage Validation complete
  - [ ] 4.1: Tech Spec components extracted
  - [ ] 4.2: Coverage analysis generated (tests vs. components)
  - [ ] 4.3: Coverage gaps presented to user
  - [ ] 4.4: User decisions captured for ALL gaps (generate/defer/remove)
  - [ ] 4.5-4.6: Decisions processed and documented
  - [ ] 4.7: All gaps addressed (zero unapproved gaps)
  - [ ] 4.8: Decisions documented in story file

- [ ] **Step 5:** AC Verification Checklist updated (Phase 02 items) [NEW - RCA-011]
  - Verification: All Phase 02 AC items checked off (test generation items)
  - Output: "AC Progress: X/Y items complete" displayed
  - Graceful: Skipped if story doesn't have AC Checklist section

### Success Criteria

- [ ] All acceptance criteria have failing tests
- [ ] Tests follow AAA pattern (from coding-standards.md)
- [ ] Tests placed correctly (per source-tree.md)
- [ ] All tests FAIL initially (Red phase verified)
- [ ] Test command executable ({TEST_COMMAND} runs)
- [ ] Technical Specification 100% covered OR gaps explicitly approved by user
- [ ] Zero autonomous deferrals (all coverage decisions user-controlled)

### Checkpoint Validation

**IF ANY ITEM UNCHECKED:**
```
❌ PHASE 02 INCOMPLETE - Review missing steps above
⚠️  DO NOT PROCEED TO PHASE 03 until all checkpoints pass

Most commonly missed:
  - Step 4 (Tech Spec Coverage Validation) ← 620 lines, often skipped
  - User approval for coverage gaps ← Required for EVERY gap
  - Documentation of decisions ← Must update story file

Proceeding without Step 4 results in:
  - Minimal stubs that pass tests but lack full implementation
  - Deferred work accumulating silently without tracking
  - Technical debt not documented
```

**IF ALL ITEMS CHECKED:**
```
✅ PHASE 02 COMPLETE - Test-First Design (Red Phase) Done

Tests generated: {test_count} tests
Coverage: {AC_count} acceptance criteria
Tech Spec: {component_count} components validated
Gaps: {gap_count} addressed with user approval

All tests are RED (failing). Ready to implement.

**Update Progress Tracker:**
Mark "Execute Phase 02" todo as "completed"

**See Also:**
- `tdd-green-phase.md` - Phase 03 workflow (minimal implementation)
- `tdd-patterns.md` - Comprehensive TDD guidance
- `test-automator` subagent - Test generation specialist

Next: Load tdd-green-phase.md and execute Phase 03 (Implementation - Green Phase)
```
