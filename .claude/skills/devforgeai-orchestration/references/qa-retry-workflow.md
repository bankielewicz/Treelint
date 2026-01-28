# Phase 3.5: QA Failure Recovery with Retry Loop

When QA fails, this phase manages the retry workflow with maximum 3 attempts.

## Purpose

**Handle QA validation failures intelligently:**
1. Categorize failure type (deferral, coverage, anti-pattern, compliance)
2. Track retry attempts (max 3)
3. Prevent infinite retry loops
4. Offer targeted recovery strategies per failure type
5. Create follow-up stories for deferred work
6. Update workflow history with all retry attempts

**This phase moves 134 lines of QA retry business logic FROM /orchestrate command TO skill** for proper orchestration layer separation (lean orchestration pattern).

**Enhanced by RCA-006:** Deferral-specific handling, max 3 attempt enforcement, follow-up story automation.

---

## Overview

### Retry Strategy

**Maximum 3 attempts** to pass QA validation:
- **Attempt 1:** Initial QA run (Phase 3)
- **Attempt 2:** After development fixes (if retry chosen)
- **Attempt 3:** Final retry (if needed)
- **After 3 failures:** HALT orchestration, recommend story split

**Loop prevention:** Strict attempt counting prevents infinite retries.

---

### Failure Types

1. **Deferral violations** - Deferred DoD items without justification
2. **Coverage gaps** - Coverage below thresholds (95%/85%/80%)
3. **Anti-pattern violations** - CRITICAL/HIGH severity violations
4. **Compliance failures** - Spec, performance, security issues

**Each type** gets tailored recovery options.

---

## Step 1: Detect QA Validation Result

### Parse QA Skill Result

```
Phase 3 executes QA skill:
  Skill(command="devforgeai-qa")

QA skill returns result in conversation context

Parse result:
  IF qa_status == "PASSED" OR qa_status == "QA Approved":
    Display: "✅ QA Validation PASSED"
    Create checkpoint: "QA_APPROVED"
    Return: "QA_PASSED"
    → Proceed to Phase 4 (Staging Release)

  ELSE:
    # QA validation failed
    Display: "❌ QA Validation FAILED - Analyzing failure..."

    Read QA report:
      Read(file_path="devforgeai/qa/reports/{STORY_ID}-qa-report.md")

    Parse report sections:
      - Overall status (should be FAILED)
      - Failure reason summary
      - Violations by severity (CRITICAL, HIGH, MEDIUM, LOW)
      - Specific issues list

    Continue to Step 2 (categorize failure)
```

---

## Step 2: Categorize QA Failure Type

### Deferral Violations

```
Grep(
  pattern="Deferral Validation.*FAILED|deferral.*violation",
  path="devforgeai/qa/reports/{STORY_ID}-qa-report.md",
  output_mode="content",
  -i=true
)

IF deferral violations found:
  failure_type = "deferral"

  Extract deferral details:
    - Deferred items count
    - Violation types (circular, invalid ref, unjustified)
    - Specific items with violations
    - Remediation guidance from deferral-validator subagent

  Example violations:
    - "Unit testing deferred without story reference"
    - "Documentation deferred with circular dependency"
    - "Integration tests deferred without ADR justification"
```

---

### Coverage Violations

```
Grep(
  pattern="Coverage.*below.*threshold|Coverage.*FAILED",
  path=QA_report,
  output_mode="content"
)

IF coverage violations found:
  failure_type = "coverage"

  Extract coverage details:
    - Business logic coverage (threshold 95%)
    - Application coverage (threshold 85%)
    - Infrastructure coverage (threshold 80%)
    - Specific uncovered files/functions

  Example violations:
    - "Business logic: 88% (below 95% threshold)"
    - "Uncovered: src/payment/processor.py lines 45-78"
```

---

### Anti-Pattern Violations

```
Grep(
  pattern="CRITICAL.*violation|Anti-Pattern.*CRITICAL",
  path=QA_report,
  output_mode="content"
)

IF anti-pattern violations found:
  failure_type = "anti-pattern"

  Extract anti-pattern details:
    - Pattern types (SQL injection, hardcoded secrets, layer violations)
    - Code locations
    - Remediation steps

  Example violations:
    - "SQL concatenation detected in src/db/query.py:34"
    - "Hardcoded API key in src/config.py:12"
    - "God Object: UserManager (687 lines, limit 500)"
```

---

### Compliance Violations

```
IF none of above categories match:
  failure_type = "compliance"

  Extract general violations:
    - Acceptance criteria not validated
    - API contracts missing
    - NFRs not met

  Example violations:
    - "Acceptance criteria 3: No corresponding test found"
    - "NFR: Response time >100ms (actual 245ms)"
```

**Display:** `"Failure type: {failure_type}"`

---

## Step 3: Count QA Retry Attempts

### Parse Workflow History

```
Read story file workflow history section

Grep(pattern="QA Attempt [0-9]+", file=story_workflow_history)

Count matches:
  qa_attempts_history = match_count

Calculate current attempt:
  qa_attempt_number = qa_attempts_history + 1

Display:
  "📊 QA Attempt {qa_attempt_number} failed
   Previous attempts: {qa_attempts_history}
   Max attempts allowed: 3"

Log current attempt:
  current_qa_attempt = qa_attempt_number
```

**Purpose:** Track how many times QA has failed to prevent infinite retries.

---

## Step 4: Loop Prevention (Max 3 Attempts)

### Max Retries Exceeded

```
IF current_qa_attempt >= 3:
  # Maximum retry attempts exceeded - story likely too large

  Display:
  "❌ QA FAILED 3 TIMES - Maximum Retry Attempts Exceeded

  Story: {STORY_ID}
  Title: {STORY_TITLE}
  Failure Type: {failure_type}
  Status: QA Failed (max retries)

  This indicates:
  - Story scope too large for single sprint
  - DoD items not properly estimated during planning
  - Systemic issues with story decomposition

  🔴 ORCHESTRATION HALTED

  Recommended Actions:
  1. Split story into 2-3 smaller stories (1-2 sprints each)
  2. Review Definition of Done items for proper estimation
  3. Escalate external blockers to leadership
  4. Run /dev {STORY_ID} manually to address issues

  Latest QA Report: devforgeai/qa/reports/{STORY_ID}-qa-report.md
  Violations:
  {violations_summary}"

  # Update story with max retries status
  Append to workflow history:
    "### QA Max Retries Exceeded - {timestamp}
     - Attempts: 3
     - Final failure: {failure_type}
     - Status: QA Failed (requires story split)
     - Action required: Manual intervention"

  Return: {
    "status": "QA_MAX_RETRIES_EXCEEDED",
    "attempts": current_qa_attempt,
    "failure_type": failure_type,
    "violations": violations_summary,
    "recommendation": "Split story into smaller units"
  }

  HALT orchestration (no more retries)

ELSE:
  # Within retry limit, proceed to recovery strategy
  Display: "Retry attempt {current_qa_attempt} of 3 - determining recovery strategy..."
  Continue to Step 5
```

**Critical:** Prevents infinite retry loops. After 3 failures, orchestration HALTS permanently for that story.

---

## Step 5: Determine Recovery Strategy Based on Failure Type

### Deferral Failure Recovery

```
IF failure_type == "deferral":
  AskUserQuestion:
    question: "QA failed due to deferred DoD items (attempt {current_qa_attempt}/3). How should we proceed?"
    header: "Deferral Failure"
    options:
      - label: "Return to dev, fix deferrals, retry QA"
        description: "Go back to development to complete or properly justify deferred items, then automatically retry QA validation"
      - label: "Create follow-up stories for deferred items"
        description: "Generate tracking stories for deferred work, document deferrals as justified (story remains QA Failed)"
      - label: "Manual resolution - I'll fix it"
        description: "Stop orchestration, I'll handle deferral issues manually via /dev command"
    multiSelect: false

  recovery_strategy = user_response
```

**Options explained:**
1. **Return to dev** - Automatic retry (recommended)
2. **Create follow-up** - Track deferrals as technical debt
3. **Manual resolution** - User handles outside orchestration

---

### Coverage Failure Recovery

```
IF failure_type == "coverage":
  AskUserQuestion:
    question: "QA failed: Coverage below thresholds (attempt {current_qa_attempt}/3). Fix and retry?"
    header: "Coverage Failure"
    options:
      - label: "Return to dev, add missing tests"
        description: "Go back to development to write tests for uncovered code, then retry QA"
      - label: "Request coverage exception"
        description: "Document justification for below-threshold coverage (requires approval)"
      - label: "Manual fix - I'll handle it"
        description: "Stop orchestration, I'll write tests manually"
    multiSelect: false

  recovery_strategy = user_response
```

---

### Anti-Pattern Violation Recovery

```
IF failure_type == "anti-pattern":
  AskUserQuestion:
    question: "QA found anti-pattern violations (attempt {current_qa_attempt}/3). Fix and retry?"
    header: "Anti-Pattern"
    options:
      - label: "Return to dev, refactor code"
        description: "Go back to development to fix anti-pattern violations (SQL injection, layer violations, etc.)"
      - label: "Create ADR, request exception"
        description: "Document architectural decision for exception approval"
      - label: "Manual fix - I'll handle it"
        description: "Stop orchestration, I'll fix violations manually"
    multiSelect: false

  recovery_strategy = user_response
```

---

### General Compliance Failure (HALTS - No Retry Offered)

```
IF failure_type == "compliance":
  # General compliance failures require manual intervention

  Display:
  "❌ QA Validation Failed

  Story: {STORY_ID}
  Failure Type: {failure_type}
  Attempt: {current_qa_attempt}/3

  Violations Summary:
  - CRITICAL: {critical_count}
  - HIGH: {high_count}
  - MEDIUM: {medium_count}
  - LOW: {low_count}

  Review detailed report: devforgeai/qa/reports/{STORY_ID}-qa-report.md

  Orchestration HALTED - Manual intervention required.

  To resume:
  1. Run /dev {STORY_ID} to fix violations
  2. Run /orchestrate {STORY_ID} to resume from QA validation"

  Return: {
    "status": "QA_FAILED_MANUAL",
    "failure_type": failure_type,
    "attempt": current_qa_attempt,
    "violations": violations_summary
  }

  HALT orchestration (manual fix required)
```

**Rationale:** General compliance failures are complex and require human analysis. No automatic retry offered.

---

## Step 6: Execute Recovery Action

### Option A: Return to Development (Automatic Retry)

**Most common path** - Automatic fix and retry loop:

```
IF recovery_strategy CONTAINS "Return to dev":
  Display:
  "🔄 Returning to Phase 2 (Development) to fix {failure_type} issues...

  Orchestration will:
  1. Re-invoke devforgeai-development skill
  2. Dev skill will address: {failure_description}
  3. After dev complete, automatically retry QA
  4. If QA passes: Continue to staging release
  5. If QA fails again: Repeat recovery (attempt {current_qa_attempt + 1}/3)"

  # Set context markers for development skill
  **Development Mode:** fix_qa_failures
  **QA Failure Type:** {failure_type}
  **Issues to Fix:** {violations_summary}
  **Orchestration Retry:** Attempt {current_qa_attempt}

  # Re-invoke development skill
  Skill(command="devforgeai-development")

  # Development skill executes TDD workflow to fix issues
  # Uses context markers to understand what to fix
  # Returns: "Dev Complete" status with fixes applied

  Display:
  "✅ Development fixes complete
   📋 Changes: {dev_changes_summary}
   🔍 Retrying QA validation (attempt {current_qa_attempt + 1})..."

  # Re-invoke QA skill for retry validation
  **Story ID:** {STORY_ID}
  **Validation Mode:** deep
  **Retry Attempt:** {current_qa_attempt + 1}

  Skill(command="devforgeai-qa")

  # QA skill executes comprehensive validation
  # Returns: "PASSED" or "FAILED"

  IF QA result == "PASSED":
    Display: "✅ QA PASSED after {current_qa_attempt} retries!"
    Create checkpoint: "QA_APPROVED"
    Track retry in history:
      "QA Attempt {current_qa_attempt + 1}: PASSED (after {current_qa_attempt} failures)"
    Return: "QA_PASSED_AFTER_RETRY"
    → Proceed to Phase 4 (Staging Release)

  ELSE:
    # QA failed again, loop back
    Display: "❌ QA still failing after retry (attempt {current_qa_attempt + 1})"
    # Recursive: Jump back to Step 3 (check attempt count, may hit max 3)
    GOTO Step 3: Count QA Retry Attempts
```

**Key features:**
- Automatic dev → QA cycle
- User informed at each step
- Recursive loop with max 3 limit
- Clear progress tracking

---

### Option B: Create Follow-Up Stories (Deferral Tracking)

**Deferral-specific path** - Create stories to track deferred work:

```
IF recovery_strategy CONTAINS "Create follow-up":
  Display:
  "📝 Creating Follow-Up Stories for Deferred DoD Items...

  This will:
  1. Create tracking stories for each deferred item
  2. Link tracking stories to original story
  3. Mark deferrals as justified (with story references)
  4. Original story remains QA Failed (requires decision)"

  # Extract deferred items from story
  Read story Definition of Done section
  deferred_items = parse items marked [x] in DoD but [ ] in Implementation

  created_stories = []
  FOR each deferred_item in deferred_items:
    AskUserQuestion:
      question: "Create follow-up story for: '{deferred_item.description}'?"
      header: "Follow-Up Story"
      options:
        - label: "Yes - Create tracking story"
          description: "Generate story to track this deferred work"
        - label: "Skip this item"
          description: "Don't create tracking story for this one"
      multiSelect: false

    IF user selects "Yes":
      # Set context for story creation
      **Feature Description:** {deferred_item.description}
      **Parent Story:** {STORY_ID} (deferred work from)
      **Epic:** {current_story.epic}
      **Sprint:** Backlog (for future planning)
      **Story Type:** deferral_tracking

      # Invoke story creation skill
      Skill(command="devforgeai-story-creation")

      # Story created, capture ID
      new_story_id = extract from skill result
      created_stories.append(new_story_id)

      # Update original story with tracking reference
      Edit original story DoD section:
        old_string: "{deferred_item}: [Deferred reason]"
        new_string: "{deferred_item}: [Deferred - Tracked in {new_story_id}]"

  Display:
  "✅ Follow-Up Stories Created

  Original Story: {STORY_ID}
  Status: QA Failed (deferred items now tracked)
  Follow-up Stories Created: {len(created_stories)}
  {FOR story in created_stories:}
    - {story}: {story.title}
  {END FOR}

  Options:
  1. Accept story with justified deferrals (create ADR, update QA report)
  2. Continue orchestration to staging (with deferrals justified)
  3. Complete deferred work now (run /dev {STORY_ID})"

  Return: {
    "status": "QA_FAILED_DEFERRALS_TRACKED",
    "created_stories": created_stories,
    "original_story_status": "QA Failed",
    "deferrals_justified": true
  }

  HALT orchestration (user decides next step: accept or complete work)
```

**Key features:**
- Interactive story creation for each deferred item
- Automatic linking (tracking story → original story)
- DoD section updated with references
- User chooses what to track

---

### Option C: Manual Resolution (User Handles)

**User intervention path** - Stop orchestration, user fixes manually:

```
IF recovery_strategy CONTAINS "Manual":
  Display:
  "⏸️  Orchestration Paused - Manual Intervention Required

  Story: {STORY_ID}
  Status: QA Failed (attempt {current_qa_attempt}/3)
  Failure Type: {failure_type}

  QA Report: devforgeai/qa/reports/{STORY_ID}-qa-report.md

  Next Steps:
  1. Review QA report and violations
  2. Fix issues manually
  3. Run: /dev {STORY_ID} (to apply fixes)
  4. Resume: /orchestrate {STORY_ID} (will retry QA automatically)

  Checkpoint: QA_FAILED saved (orchestration will resume from QA)"

  # Update story workflow history
  Append:
    "### QA Attempt {current_qa_attempt}: User Chose Manual Fix - {timestamp}
     - Failure: {failure_type}
     - Violations: {violations_summary}
     - Next: Manual /dev required
     - Resume: /orchestrate will retry QA"

  Return: {
    "status": "QA_FAILED_MANUAL",
    "failure_type": failure_type,
    "attempt": current_qa_attempt,
    "resume_instruction": "/orchestrate {STORY_ID}"
  }

  HALT orchestration (user handles manually)
```

**Key features:**
- Clear resume instructions
- Checkpoint saved (orchestration knows where to resume)
- User has full control

---

### Option D: Request Exception (Coverage/Anti-Pattern Only)

**Exception approval path** - Document justification via ADR:

```
IF recovery_strategy CONTAINS "exception":
  Display:
  "📋 Exception Request Process

  To request exception for {failure_type}:
  1. Create ADR documenting justification:
     File: devforgeai/specs/adrs/ADR-{next_number}-{failure_type}-exception-{STORY_ID}.md
  2. Document:
     - Why exception needed (business justification)
     - Risk assessment (what could go wrong)
     - Mitigation plan (how to address in future)
     - Approval required (tech lead sign-off)
  3. After ADR approved:
     - Update QA report with exception reference
     - Continue orchestration: /orchestrate {STORY_ID}

  Orchestration HALTED pending exception approval."

  Return: {
    "status": "QA_FAILED_EXCEPTION_REQUESTED",
    "failure_type": failure_type,
    "adr_required": true
  }

  HALT orchestration (exception approval needed)
```

**Available for:**
- Coverage exceptions (below threshold but justified)
- Anti-pattern exceptions (architectural decision)

**Not available for:**
- Deferral violations (must create tracking stories)
- Compliance violations (must fix)

---

## Step 7: Track Retry Iteration in Workflow History

### Workflow History Entry Template

**Append comprehensive retry entry:**

```markdown
### QA Attempt {current_qa_attempt}: {RESULT} - {timestamp}

**Failure Type:** {failure_type}
**Violations:**
- CRITICAL: {critical_count}
- HIGH: {high_count}
- MEDIUM: {medium_count}
- LOW: {low_count}

**Recovery Strategy Chosen:** {recovery_strategy}

{IF retry path:}
**Actions Taken:**
1. Returned to Phase 2 (Development)
2. Development skill invoked with fix instructions
3. Issues addressed: {fix_summary}
4. QA retry initiated

**QA Retry Result:** {PASSED/FAILED}

{IF passed:}
✅ QA Approved after {current_qa_attempt} attempts
   Proceeding to staging release

{IF failed again:}
❌ QA still failing, attempt {current_qa_attempt + 1} will follow

{IF follow-up stories created:}
**Follow-Up Stories:**
{FOR story in created_stories:}
  - {story.id}: {story.title} (tracks: {deferred_item})
{END FOR}

{IF manual fix:}
**Status:** User chose manual resolution
**Next:** Run /dev {STORY_ID} to fix, then /orchestrate {STORY_ID} to resume
```

### Edit Operation

```
Edit(file_path="devforgeai/specs/Stories/{STORY_ID}.story.md")

Find "## Workflow Status" section
Prepend new entry (reverse chronological - latest first)

old_string: "## Workflow Status\n\n"
new_string: "## Workflow Status\n\n{new_entry}\n\n"
```

**Purpose:** Complete audit trail of all QA attempts, recovery decisions, and outcomes.

---

## Retry Loop Flow Diagram

```
Phase 3: QA Validation
  ↓
QA FAILED
  ↓
Step 1: Detect result ────────┐
  ↓                            │
Step 2: Categorize failure     │
  ↓                            │
Step 3: Count attempts         │
  ↓                            │
Step 4: Check max 3?           │
  ├─ YES (≥3) → HALT           │
  └─ NO (<3) → Continue        │
       ↓                       │
Step 5: User chooses recovery  │
  ├─ Return to dev ──────┐    │
  ├─ Create follow-up → HALT   │
  ├─ Manual → HALT             │
  └─ Exception → HALT          │
       ↓                       │
Step 6: Execute recovery       │
  ↓                            │
Re-invoke Dev Skill            │
  ↓                            │
Re-invoke QA Skill             │
  ↓                            │
QA PASSED? ───────────────────┘
  ├─ YES → Phase 4 (Release)
  └─ NO → Loop back to Step 1 (increment attempt)
```

**Max loop iterations:** 3

---

## Templates

### Template 1: QA Attempt Entry (Retry Path)

```markdown
### QA Attempt 2: FAILED - 2025-01-06T15:30:45Z

**Failure Type:** coverage
**Violations:**
- CRITICAL: 0
- HIGH: 1 (Coverage below threshold: Business logic 88% vs 95%)
- MEDIUM: 2
- LOW: 1

**Recovery Strategy Chosen:** Return to dev, add missing tests

**Actions Taken:**
1. Returned to Phase 2 (Development)
2. Development skill invoked with fix instructions
3. Issues addressed: Added 12 unit tests for payment processing module
4. QA retry initiated

**QA Retry Result:** PASSED

✅ QA Approved after 2 attempts
   Coverage improved: 88% → 96%
   Proceeding to staging release
```

---

### Template 2: QA Attempt Entry (Follow-Up Stories)

```markdown
### QA Attempt 1: FAILED - 2025-01-06T14:45:22Z

**Failure Type:** deferral
**Violations:**
- CRITICAL: 0
- HIGH: 3 (Deferred DoD items without story references)
- MEDIUM: 0
- LOW: 0

**Deferred Items:**
- Unit testing for edge cases
- Integration tests with payment gateway
- Performance testing under load

**Recovery Strategy Chosen:** Create follow-up stories for deferred items

**Follow-Up Stories Created:**
- STORY-043: Unit testing edge cases (tracks deferred: unit testing)
- STORY-044: Payment gateway integration tests (tracks deferred: integration tests)
- STORY-045: Performance load testing (tracks deferred: performance testing)

**Status:** QA Failed (deferrals now justified with tracking stories)
**Next:** User decision - Accept with deferrals OR complete deferred work
```

---

### Template 3: QA Attempt Entry (Manual Fix)

```markdown
### QA Attempt 1: User Chose Manual Fix - 2025-01-06T14:50:10Z

**Failure Type:** anti-pattern
**Violations:**
- CRITICAL: 2 (SQL concatenation, hardcoded secret)
- HIGH: 1 (God Object: UserManager 687 lines)
- MEDIUM: 3
- LOW: 2

**Status:** Orchestration paused, manual intervention required

**User Actions Required:**
1. Fix SQL concatenation in src/db/query.py:34
2. Move hardcoded API key to environment variable
3. Refactor UserManager (split into 3 smaller classes)

**Next Steps:**
1. Run /dev STORY-042 to apply fixes
2. Run /orchestrate STORY-042 to resume from QA validation

**Resume:** /orchestrate will detect checkpoint and retry QA
```

---

### Template 4: Max Retries Exceeded

```markdown
### QA Max Retries Exceeded - 2025-01-06T16:15:33Z

**Attempts:** 3
**Final Failure:** coverage
**Status:** QA Failed (requires story split)
**Action Required:** Manual intervention

**Attempt Summary:**
- Attempt 1: Coverage 88% (failed) → Fixed, retried
- Attempt 2: Coverage 91% (failed on different module) → Fixed, retried
- Attempt 3: Coverage 89% (failed again) → MAX RETRIES

**Root Cause Analysis:**
Story scope too large. Coverage gaps keep appearing in different modules.

**Recommended Actions:**
1. Split STORY-042 into 3 smaller stories:
   - STORY-046: User login (core functionality)
   - STORY-047: Session management (separate concern)
   - STORY-048: OAuth integration (separate concern)
2. Each new story will be easier to test comprehensively
3. Re-estimate original story (was 8 points, split into 3×3 = 9 points)

**Escalation:** Tech lead review required
```

---

## Error Handling

### Development Skill Fails During Retry

```
Error: Dev skill invoked but fails to complete

Action:
  Display: "Development skill failed during retry attempt {current_qa_attempt}.
           Error: {dev_skill_error}

           Orchestration HALTED.

           Review development skill output for issues.
           Fix manually: /dev {STORY_ID}"

  Append to workflow history:
    "### Development Retry Failed - {timestamp}
     - Attempt: {current_qa_attempt}
     - Dev skill error: {error_message}
     - Status: Orchestration halted"

  Return: "DEV_RETRY_FAILED"
  HALT orchestration
```

---

### QA Skill Fails to Execute

```
Error: QA skill invoked but returns error (not FAILED result, but execution error)

Action:
  Display: "QA skill execution error during retry {current_qa_attempt}.
           Error: {qa_skill_error}

           This is not a validation failure (which can retry).
           This is a skill execution error (requires investigation).

           Orchestration HALTED.

           Possible causes:
           - QA scripts missing dependencies
           - Report generation failed
           - File permission issues"

  Return: "QA_EXECUTION_ERROR"
  HALT orchestration
```

---

### Invalid Recovery Strategy

```
Error: User selects recovery option but skill cannot execute it

Action:
  Display: "Selected recovery strategy cannot be executed.
           Strategy: {recovery_strategy}
           Reason: {why_invalid}

           Falling back to manual resolution."

  Proceed with manual resolution path (Option C)
```

---

## Related Files

- **skill-invocation.md** - Phase 2, invokes QA skill that can fail
- **story-status-update.md** - Phase 3A, updates status after QA passes/fails
- **orchestration-finalization.md** - Phase 6, includes retry summary in completion report
- **deferred-tracking.md** - Phase 4.5, tracks technical debt from deferrals
- **quality-gates.md** - Gate 3 (QA Approval) that can block
- **troubleshooting.md** - Common QA retry issues
