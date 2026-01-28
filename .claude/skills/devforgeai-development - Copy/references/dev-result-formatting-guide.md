# Dev Result Formatting Guide

**Purpose:** Framework-aware guardrails for the `dev-result-interpreter` subagent

This reference document provides the subagent with DevForgeAI context to prevent autonomous behavior and ensure development results are presented appropriately within the framework's governance model.

---

## DevForgeAI Context

### Story Workflow States

The development validator operates on stories progressing through defined states:

```
Backlog → Architecture → Ready for Dev → In Development → Dev Complete →
QA In Progress → [QA Approved | QA Failed] → Releasing → Released
```

**Key State Transitions (where dev-result-interpreter operates):**

- **Ready for Dev → In Development (Workflow Start):**
  - Story status changes when `/dev` command initiated
  - Phase 01 validation confirms story readiness
  - Framework checks: context files exist, story complete, no blockers

- **In Development → Dev Complete (TDD Cycle Complete):**
  - Workflow progression after Phase 08 (all TDD phases complete)
  - Must pass all tests (100% pass rate)
  - All acceptance criteria must have green tests
  - Changes story status to "Dev Complete"

- **Dev Complete → QA In Progress (Automatic):**
  - Triggered after Phase 08, before Phase 09 finalization
  - Story ready for deep QA validation
  - Next step: `/qa STORY-ID deep`

### TDD Phases and Dev Result Points

The development skill executes **8 phases** in sequence:

```
Phase 01: Pre-Flight (Git, context, tech detection)
Phase 02: Red (write failing tests from AC)
Phase 03: Green (implement code to pass tests)
Phase 04: Refactor (improve code quality)
Phase 05: Integration (cross-component tests)
Phase 05.5: Deferral Challenge (validate all deferrals with approval)
Phase 08: Git/Tracking (commit or file-based tracking)
Phase 09: Final Validation (Implementation Notes completion)
```

**Result Interpreter Operates At:**
- After Phase 09 completion
- Receives: Development summary with phases executed, tests passed/failed, deferrals status
- Returns: Formatted result for command display

### Quality Gates

Two critical gates enforce development workflow:

**Gate 1: Context Validation** (Phase 01)
- All 6 context files exist and are non-empty
- Tech stack detected and validated
- Git or file-based workflow confirmed
- Enforced by: devforgeai-development skill

**Gate 2: Test Passing** (Phase 03 completion)
- Build succeeds
- All tests pass (100% pass rate)
- Each acceptance criterion has ≥1 passing test
- Blocks advancement to Phase 04 if fails
- Enforced by: devforgeai-development skill

**Gate 3: DoD Completion** (Phase 09 completion)
- All Definition of Done items checked (complete = Phase 08)
- Incomplete items documented (with Phase references)
- Deferrals justified and approved
- Enforced by: devforgeai-development skill

---

## Framework Constraints

### 1. Story Status Transitions (Immutable)

Status transitions are deterministic and follow framework rules:

```
Ready for Dev → In Development (on /dev start)
In Development → Dev Complete (on Phase 09 success)
Dev Complete → QA In Progress (automatic after Phase 09)
```

**Display guidance:**
- Always show current status and transition
- Confirm new status after Phase 09 (Dev Complete)
- Note: QA In Progress is automatic, not user action
- Never display status as "Reviewing" or "Testing" (use framework states only)

**Never change status:**
- Before Phase 09 completion
- If tests fail (remains in Development)
- If deferrals not approved (remains in Development)

### 2. Test Requirements (Deterministic)

Tests must meet framework standards:

```
Test Pass Rate:         100% required (no skipped or failed tests allowed)
Test per AC:            ≥1 test per acceptance criterion
Test Framework:         From tech-stack.md (not substitutable)
Test Type Distribution: ~70% unit, ~20% integration, ~10% E2E
```

**Display guidance:**
- Always show test count (X tests passing)
- Show coverage gaps (if any ACs missing tests)
- Reference tech-stack.md for framework (e.g., "pytest (tech-stack.md)")
- If 100% pass rate not achieved, show which tests failed and why

**Never accept:**
- Skipped tests ("pending", "todo", "xfail")
- Commented-out tests
- 95% pass rate ("close enough")

### 3. Deferral Validation (RCA-006 - Framework-Specific)

Deferrals must meet strict validation criteria:

```
Valid Deferrals (pass Phase 05.5):
  1. External blocker (API v2 available 2025-12-01) ✅
  2. Scope change with ADR (Out of scope: ADR-042) ✅
  3. Story split (Deferred to STORY-125: performance epic) ✅
  4. User-approved (timestamp + approval note) ✅

Invalid Deferrals (fail Phase 05.5):
  1. No justification ("Will add later") ❌
  2. Vague reason ("Too complex", "Hard to implement") ❌
  3. Circular chain (STORY-A → STORY-B → STORY-A) ❌ CRITICAL
  4. Invalid story reference (STORY-XXX doesn't exist) ❌
  5. Missing ADR for scope change ❌
  6. Unnecessary deferral (feasible to implement now) ❌
  7. Multi-level chain (A → B → C) ❌ CRITICAL
```

**Display guidance:**
- If deferrals exist, show validation status
- CRITICAL deferrals block Dev Complete status
- Show which deferrals are approved vs pending
- Recommend returning to dev for deferral resolution

### 4. Definition of Done (Framework-Specific)

DoD items progress through TDD phases:

```
Phase 02 (Red):       Generate tests, count items, validate
Phase 03 (Green):     Implement business logic, mark items done
Phase 04 (Refactor):  Code quality items, complexity validation
Phase 05 (Integration): Cross-component tests, API contracts
Phase 05.5 (Deferrals): Validate all deferred items, get approvals
Phase 08 (Git):       Commit or file-based tracking
Phase 09 (Final):     Update Implementation Notes, confirm status
```

**Display guidance:**
- Show DoD completion percentage (X of Y items complete)
- Group by status (✅ Complete, ⏳ In Progress, ❌ Blocked)
- List blocked items (why blocked, what action needed)
- Note deferred items (when will they be completed)

### 5. Git Operations (RCA-008 - User Consent Required)

Git operations follow strict user approval pattern:

```
APPROVED Operations:
  ✅ git status (read-only)
  ✅ git diff (read-only)
  ✅ git log (read-only)
  ✅ git add [current story files] (≤5 files created in session)
  ✅ git commit -m "message" (TDD workflow commits)

DENIED Without Approval:
  ❌ git stash (hides files from user)
  ❌ git reset --hard (destroys uncommitted work)
  ❌ git branch -D [branch] (deletes branch)
  ❌ git push --force (rewrites history)
  ❌ git commit --amend (modifies existing commits)
  ❌ Affects >10 files (too risky without explicit consent)

User Consent Pattern:
  1. Display: "Git operation will [ACTION]"
  2. Show: Files affected, what will happen
  3. Ask: AskUserQuestion for approval
  4. Execute: Only if user explicitly approves
  5. Report: Confirm completion with results
```

**Display guidance:**
- Never perform risky git operations autonomously
- Always explain what git operation will do
- Show which files are affected
- Require explicit user approval for destructive ops

### 6. Anti-Pattern Enforcement (From anti-patterns.md)

Code must avoid project-specific anti-patterns:

```
Structural Violations:
  - Files in wrong locations (violates source-tree.md) → FAIL Phase 04
  - Layer dependency violations (violates architecture-constraints.md) → FAIL Phase 04
  - Library substitution (violates tech-stack.md) → FAIL Phase 04

Code Smells (Code Quality):
  - God Objects (>500 lines or >20 methods) → Warning in Phase 04
  - High complexity (>10 cyclomatic) → Refactor required in Phase 04
  - Code duplication (>5%) → Refactor required in Phase 04
  - Long methods (>100 lines) → Refactor required in Phase 04
```

**Display guidance:**
- Show which anti-patterns detected (if any)
- Reference specific anti-pattern from anti-patterns.md
- Recommend refactoring for Phase 04 violations
- FAIL development if structural violations found

---

## Display Template Guidelines

### Structure for All Templates

1. **Header** (2 lines)
   - Status emoji + Title: "✅ Development COMPLETE"
   - Story ID and title

2. **Summary** (3-5 lines)
   - Result status (Complete, Incomplete, Failed)
   - TDD phases executed
   - Test results (pass count, failures)
   - Deferral status (if any)

3. **Detailed Results** (varies by result type)
   - Phases completed (with phase summaries)
   - Test results breakdown (unit/integration/E2E)
   - DoD completion percentage
   - Any blockers or issues

4. **Deferrals Section** (if deferrals exist)
   - List of deferred items
   - Justification for each
   - Approval status
   - Follow-up story references

5. **Recommendation** (1-2 lines)
   - "✅ READY FOR QA" (for Dev Complete)
   - "❌ FIX ISSUES" (for failures)
   - "⚠️ APPROVE DEFERRALS" (for pending approvals)

6. **Next Steps** (2-5 lines)
   - Specific action for user
   - Command to run if applicable
   - Link to detailed implementation notes

### Emoji Usage

- ✅ Complete, success, approved, passing tests
- ❌ Fail, error, blocked, failing tests
- ⚠️ Warning, attention needed, incomplete
- ℹ️ Information, context, note
- → Arrow for actions/flow
- ⏳ In progress, pending

### Tone Guidance

- **Development Complete:** Celebratory, confident ("Ready for QA!")
- **Deferrals Pending:** Constructive, specific ("Defer after approval")
- **Tests Failing:** Direct, urgent ("Fix broken tests immediately")
- **Issues Found:** Helpful, actionable ("Return to Phase 04 for refactoring")

### Length Guidelines

- **Dev Complete (no deferrals):** 20-35 lines total
- **Dev Complete (with deferrals):** 35-50 lines total
- **Incomplete/Blocked:** 25-45 lines total
- **Failed:** 30-60 lines total

### Template Examples

**Dev Complete (No Deferrals):**
```
✅ Development COMPLETE

Story: STORY-042 - User Registration Form
Status transition: In Development → Dev Complete

Summary:
- All 8 TDD phases executed successfully
- 47 tests written and passing (100% pass rate)
- 8 of 8 Definition of Done items complete
- Ready for QA validation

Test Results:
- Unit tests: 35/35 passing
- Integration tests: 10/10 passing
- E2E tests: 2/2 passing
- Coverage: 96% (meets 95% threshold)

Phases Executed:
  Phase 02 (Red): 47 failing tests created ✅
  Phase 03 (Green): All tests now passing ✅
  Phase 04 (Refactor): Code simplified, complexity 8/10 ✅
  Phase 05 (Integration): Cross-component tests ✅
  Phase 05.5 (Deferrals): No deferrals ✅
  Phase 08 (Git): Changes committed ✅
  Phase 09 (Final): Implementation Notes updated ✅

Recommendation: ✅ READY FOR QA

Next Steps:
1. Review Implementation Notes: devforgeai/specs/Stories/STORY-042.story.md
2. Start QA validation: /qa STORY-042 deep
3. Target: QA approval within 5 minutes
```

**Dev Complete (With Deferrals):**
```
⚠️ Development COMPLETE (with Deferrals)

Story: STORY-043 - Payment Processing
Status transition: In Development → Dev Complete

Summary:
- 7 of 8 TDD phases executed
- 52 tests written and passing (100% pass rate)
- 6 of 8 Definition of Done items complete
- 2 items deferred (pending approval)

Test Results:
- Unit tests: 38/38 passing
- Integration tests: 12/12 passing
- E2E tests: 2/2 passing
- Coverage: 94% (below 95% threshold, deferred to Phase 04 refactor)

Deferrals Requiring Approval:
1. "Exit code handling for PCI compliance verification"
   - Reason: External dependency (PCI v4.1 June 2025)
   - Status: ⏳ PENDING USER APPROVAL
   - Deferred to: STORY-087 (PCI compliance epic)
   → Approve or return to dev: /dev STORY-043

2. "Coverage gap: Error recovery paths"
   - Reason: Test complexity requires investigation
   - Status: ⏳ PENDING USER APPROVAL
   - Deferred to: STORY-088 (error handling epic)
   → Approve or return to dev: /dev STORY-043

Definition of Done:
- ✅ Code written (all acceptance criteria implemented)
- ✅ Tests written (52 tests, 100% pass)
- ✅ Code reviewed (no violations)
- ⏳ Coverage 95%+ (currently 94%, 1 point gap)
- ⏳ No deferrals (2 pending approval)
- ✅ Committed (changes staged)

Recommendation: ⚠️ APPROVE DEFERRALS OR CONTINUE DEVELOPMENT

Next Steps:
1. Review deferrals above and decide:
   - Option A: Approve deferrals (create follow-up stories)
   - Option B: Return to dev to fix (run /dev STORY-043)
2. Once deferrals approved or fixed:
   - Status updates to Dev Complete
   - Run QA: /qa STORY-043 deep
```

**Development Failed:**
```
❌ Development FAILED

Story: STORY-044 - Async Job Processing
Status remains: In Development (not progressing)

Issue: Tests Failing in Phase 03

Problem:
- 3 of 45 tests failing (93% pass rate, need 100%)
- Failing tests: test_job_timeout, test_job_retry, test_job_cancel
- Error: "Timeout handler not implemented"

Failed Tests:
1. test_job_timeout
   Error: AssertionError: Expected timeout after 5s, got 8.2s
   Location: tests/async/test_timeout.py:42

2. test_job_retry
   Error: AsyncTimeout: Retry mechanism not responding
   Location: tests/async/test_retry.py:18

3. test_job_cancel
   Error: RuntimeError: Cancel signal not processed
   Location: tests/async/test_cancel.py:31

Recommendation: ❌ FIX TESTS AND CONTINUE

Next Steps:
1. Return to development to fix failing tests:
   /dev STORY-044
2. Focus on async task handling in Phase 03 (Green phase)
3. Ensure all 45 tests pass before Phase 04
4. Common causes to investigate:
   - Event loop timing in test setup
   - Async context manager cleanup
   - Signal handling in task handler
```

---

## Framework Integration Points

### 1. Context Files References

When recommending next steps, reference context files:

**tech-stack.md:**
- "Your project uses pytest (tech-stack.md) for testing"
- Suggest framework-specific best practices
- Never suggest substitute testing framework

**source-tree.md:**
- "Code should follow source-tree.md structure"
- Suggest correct file locations if violations found
- Reference layer organization for Phase 04 refactoring

**coding-standards.md:**
- "Your standards require <10 complexity per method (coding-standards.md)"
- Show what threshold was violated
- Reference for refactoring guidance

**architecture-constraints.md:**
- "Your architecture (architecture-constraints.md) requires Presentation → Application → Domain"
- Show which constraint was violated
- Reference for Phase 04 refactoring

**anti-patterns.md:**
- "This pattern is forbidden (anti-patterns.md): Direct instantiation"
- Reference specific anti-pattern from your project
- Suggest refactoring approach

### 2. Related Skills and Subagents

Display names when recommending next actions:

**Test failures:**
- "test-automator subagent can help generate stub tests"
- "Run test generator for uncovered code paths"

**Code quality issues:**
- "Refactoring-specialist subagent can help improve complexity"
- "Code reviewer subagent found style violations"

**Deferral validation:**
- "deferral-validator subagent will check approval status"
- "Follow-up story creation recommended: /create-story [description]"

**Architecture issues:**
- "architect-reviewer subagent validates scope changes"
- "Create ADR if scope change required: /rca [description]"

### 3. Workflow History Integration

Include in next steps guidance:

- "This is attempt #2 after previous failure"
- "Story has been in development for X hours"
- "7 TDD phases executed, 1 phase remaining"
- "Follow-up story created for deferred work: STORY-XXX"

---

## Error Scenarios and Handling

### Scenario 1: Implementation Notes Missing

```
Status: INCOMPLETE
Issue: Implementation Notes section not found in story file

Display:
"⚠️ Implementation Notes Incomplete

The story file exists but Implementation Notes are not populated.
This section must be completed before Phase 09 finalization.

Contents Expected:
- Business logic implementation summary
- File locations of key implementation
- Non-obvious design decisions
- Metrics: test count, coverage, complexity

Recovery Options:
1. Return to development: /dev STORY-ID
   (Phase 09 will update Implementation Notes)
2. Manually update story file if development is complete
3. Review Phase 09 requirements in skill documentation"
```

### Scenario 2: Partial Test Execution

```
Status: INCOMPLETE
Issue: Only Phase 02-2 completed, Phase 04+ not started

Display:
"⚠️ Development Incomplete

Story development stopped at Phase 03 (Green Phase).

Phases Executed:
  ✅ Phase 02 (Red): 35 tests created
  ✅ Phase 03 (Green): Tests now passing
  ❌ Phase 04 (Refactor): NOT STARTED
  ❌ Phase 05 (Integration): NOT STARTED
  ❌ Phase 05.5 (Deferrals): NOT STARTED
  ❌ Phase 08 (Git): NOT STARTED
  ❌ Phase 09 (Final): NOT STARTED

Recommendation: Continue development to complete all phases

Next Steps:
1. Return to development: /dev STORY-ID
   (Skill will detect checkpoint and resume at Phase 04)
2. Complete remaining phases (3, 4, 4.5, 5, 6)
3. Then proceed to QA"
```

### Scenario 3: Deferred Items Without Approval

```
Status: INCOMPLETE (Awaiting Approval)
Issue: 2 deferred items pending user approval

Display:
"⏳ Development Blocked on Deferrals

All code complete, tests passing, but 2 items deferred.
Framework requires user approval for all deferrals.

Deferred Items (Awaiting Approval):

1. "Database migration strategy for legacy data"
   Reason: Complex data transformation, needs spike
   Deferred to: STORY-089 (database refactor epic)
   Approval options:
   → Approve (creates follow-up story): Answer 'YES' in prompt
   → Return to dev to complete now: Answer 'NO' in prompt

2. "GraphQL API documentation and examples"
   Reason: Requires marketing/docs team coordination
   Deferred to: STORY-090 (API documentation epic)
   Approval options:
   → Approve (creates follow-up story): Answer 'YES' in prompt
   → Return to dev to complete now: Answer 'NO' in prompt

Recommendation: ⏳ REVIEW DEFERRALS

Next Steps:
1. Approve deferrals (follow-up stories will be created):
   → Choose 'YES' for each deferral in prompt
2. OR return to development to complete:
   → Choose 'NO' for each deferral
   → Run: /dev STORY-ID to continue
3. Once all deferrals approved/completed, status updates to Dev Complete
4. Then: /qa STORY-ID deep for QA validation"
```

### Scenario 4: Git Operation Failed

```
Status: FAILED
Issue: Git commit failed (possibly due to conflicts)

Display:
"❌ Git Commit Failed

Phase 08 attempted to commit changes but encountered an error.

Error Details:
Command: git commit -m "[STORY-045]: Implement payment retry logic..."
Error: CONFLICT: devforgeai/specs/context/tech-stack.md (modified by another process)

Possible Causes:
1. Another story/branch modified the same files
2. Pre-commit hook validation failed
3. File permissions issue (read-only file)

Recovery Options:
1. Check git status:
   git status

2. Review the conflict:
   git diff devforgeai/specs/context/tech-stack.md

3. Resolve the conflict and commit:
   git add [files]
   git commit -m 'message'

4. OR use file-based tracking (no git):
   Run: /dev STORY-ID
   (Skill will detect git conflict and use file-based tracking)

Recommendation: ❌ RESOLVE GIT CONFLICT

Next Steps:
1. Understand the conflict (see error details above)
2. Choose resolution:
   - Option A: Resolve manually and commit
   - Option B: Return to dev for file-based tracking
3. Once resolved, development status: Dev Complete"
```

---

## Performance and Reliability

### Parsing Reliability

- Must handle development with 0 deferrals (edge case: clean completion)
- Must handle development with 50+ line Implementation Notes (summary only)
- Must handle partial phase execution (checkpoints)
- Must handle git vs file-based tracking (both valid paths)
- Must gracefully degrade if Implementation Notes missing

### Consistency Requirements

- Same test failure always produces same severity/action
- Same status transition always maps to correct template
- Same phase completion level always shows same summary
- Deferrals always show approval status consistently
- Status never changes without explicit framework rule

### Template Consistency

- All Dev Complete displays follow same structure
- All Incomplete displays show which phases missing
- All Deferred displays show approval options
- All Failed displays show recovery steps
- All templates use same emoji meanings and action format

---

## Testing Checklist for Subagent

Use this when testing dev-result-interpreter:

- [ ] Parses development COMPLETE result (no deferrals)
- [ ] Parses development COMPLETE result (with deferrals)
- [ ] Parses development INCOMPLETE (partial phases)
- [ ] Parses development FAILED (test failures)
- [ ] Parses development FAILED (git conflict)
- [ ] Parses result with 0 deferrals
- [ ] Parses result with 1-5 deferrals (normal case)
- [ ] Parses result with 50+ lines Implementation Notes (summarizes)
- [ ] Handles missing Implementation Notes gracefully
- [ ] Handles missing Deferrals section gracefully
- [ ] Generates correct status for all result types
- [ ] Generates appropriate display template for each case
- [ ] Provides actionable remediation guidance
- [ ] Recommends correct next steps (dev/qa/fix)
- [ ] References context files appropriately
- [ ] Shows phase completion accurately
- [ ] Lists test results correctly (unit/integration/E2E)
- [ ] Shows deferral approval options (if needed)
- [ ] Handles checkpoint resume scenarios
- [ ] JSON output is valid and consistent
- [ ] Displays match framework states (In Development, Dev Complete, etc.)

---

## Remember

This subagent is **NOT executing development** — the skill does that.

This subagent is **interpreting and presenting** the results.

Your job:
1. Read what the skill executed ✅
2. Understand DevForgeAI context (TDD phases, gates, constraints) ✅
3. Display results appropriately for the situation ✅
4. Recommend clear next steps (continue dev, fix issues, proceed to QA) ✅

**DO:**
- Reference context files when relevant
- Provide specific, actionable remediation
- Recommend next steps based on result type
- Handle edge cases gracefully
- Show phase progression clearly

**DON'T:**
- Re-execute development (skill already did that)
- Make decisions for the user (present options, ask for approval)
- Change story status autonomously (framework rules govern)
- Downgrade severity (use framework definitions)
- Operate without framework guardrails
