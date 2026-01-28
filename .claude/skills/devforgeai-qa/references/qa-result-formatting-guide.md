# QA Result Formatting Guide

**Purpose:** Framework-aware guardrails for the `qa-result-interpreter` subagent

This reference document provides the subagent with DevForgeAI context to prevent "bull in china shop" behavior and ensure results are presented appropriately within the framework's governance model.

---

## DevForgeAI Context

### Story Workflow States

The QA validator operates on stories progressing through defined states:

```
Backlog → Architecture → Ready for Dev → In Development → Dev Complete →
QA In Progress → [QA Approved | QA Failed] → Releasing → Released
```

**Key State Transitions (where QA validator operates):**

- **In Development → Dev Complete (Light QA):**
  - Automatic, after each TDD phase (Red, Green, Refactor, Integration)
  - Blocks development on failure (prevents bad code from continuing)
  - Does NOT change story status on pass
  - Quick feedback loop (10 seconds to 2 minutes)

- **Dev Complete → QA In Progress (Deep QA):**
  - Manual, initiated by developer via `/qa` command
  - Comprehensive validation before approval
  - Changes story status: "QA Approved" or "QA Failed"
  - Quality gate (65 seconds to 5 minutes)

- **QA Failed → In Development (Retry Loop):**
  - Developer returns to implementation after QA failure
  - Up to 3 retry attempts (after 3, suggest story split)
  - Each retry follows Dev → QA cycle

### Quality Gates

Three critical gates enforce progression:

**Gate 1: Context Validation** (Architecture → Ready for Dev)
- All 6 context files exist and are non-empty
- Enforced by: devforgeai-architecture skill

**Gate 2: Test Passing** (Dev Complete → QA In Progress)
- Build succeeds, all tests pass
- Enforced by: devforgeai-development skill (light QA)

**Gate 3: QA Approval** (QA In Progress → Releasing) ← **Where result-interpreter operates**
- Deep validation passed
- Zero CRITICAL violations
- Zero HIGH violations (or approved exceptions)
- Coverage meets thresholds (95%/85%/80%)

**Gate 4: Release Readiness** (Releasing → Released)
- QA approved
- All workflow checkboxes complete
- Enforced by: devforgeai-release skill

---

## Validation Modes and Interpretation

### Light Validation Context

**When used:** During development (all story statuses except Dev Complete/QA states)

**Characteristics:**
- Fast feedback loop (10 seconds to 2 minutes)
- Minimal checks (build, tests, critical security only)
- Blocks immediately on failure (developer fixes in same cycle)
- Does NOT change story status
- Does NOT approve/fail story

**Result Interpretation:**
- PASS: "Continue development" (no status change)
- FAIL: "Fix issues immediately" (blocks development cycle)

**Display Style:**
- Brief summary (3-5 lines maximum)
- Focus on what's broken (not what's good)
- Action: Back to development immediately
- No "comprehensive results" needed

**Next Steps Recommendation:**
```
Light PASS:
  "Continue implementation. Run deep QA when story Dev Complete."

Light FAIL:
  "Fix issues and continue. Build must succeed before proceeding."
```

### Deep Validation Context

**When used:** After story implementation (status Dev Complete)

**Characteristics:**
- Comprehensive analysis (2-5 minutes)
- Full coverage validation (95%/85%/80% strict thresholds)
- Extensive anti-pattern detection (10+ categories)
- Changes story status: "QA Approved" or "QA Failed"
- Quality gate (blocks release if fail)

**Result Interpretation:**
- PASS: Story meets all gates, approved for release
- FAIL: Story has blocking issues, must fix before release

**Display Style:**
- Detailed results (show all metrics)
- Explain pass/fail in context of thresholds
- Show coverage by layer (business logic, application, infrastructure)
- List violations by severity
- Provide clear remediation steps

**Next Steps Recommendation:**
```
Deep PASS:
  "Story approved. Review report details, then: /release STORY-XXX"

Deep FAIL (by type):
  Coverage: "Add tests for uncovered code, then re-run: /qa STORY-XXX"
  Anti-pattern: "Fix violations, return to dev: /dev STORY-XXX"
  Compliance: "Add AC tests, fix API contracts, then: /qa STORY-XXX"
  Deferral: "Return to dev to resolve: /dev STORY-XXX"
```

---

## Framework Constraints

### 1. Coverage Thresholds (Strict, No Negotiation)

These thresholds are **immutable framework rules**:

```
Business Logic Layer:     95% minimum (strict)
Application Layer:        85% minimum (strict)
Infrastructure Layer:     80% minimum (strict)
Overall:                  80% minimum (strict)
```

**Display guidance:**
- Always show coverage breakdown by layer
- Show percentages vs. thresholds (what's the gap?)
- If below threshold, show what percentage increase needed
- Suggest test generation for quick improvements

**Never say:** "Coverage is close enough" or "95% is just a guideline"
**Always enforce:** Display exact gap and require specific actions to fix

### 2. Violation Severity Classification (Deterministic)

Violations ALWAYS classify as one of four severities:

**CRITICAL** (blocks approval immediately):
- Circular deferrals
- Security vulnerabilities (SQL injection, hardcoded secrets)
- Library substitution (tech-stack.md violated)
- Architecture layer violations (Domain → Infrastructure)
- Test failures

**HIGH** (blocks approval, must be resolved):
- Coverage below threshold
- Multi-level deferral chains (RCA-007)
- Invalid story references
- Unjustified deferrals
- Missing API contracts

**MEDIUM** (documented, may not block):
- Code smells (high complexity, duplication)
- Missing ADR for scope changes
- Unnecessary deferrals
- Maintainability issues

**LOW** (informational, no action required):
- Minor code quality issues
- Documentation gaps
- Formatting suggestions

**Display guidance:**
- Always list violations grouped by severity
- CRITICAL and HIGH count toward "approval gate"
- MEDIUM and LOW are "nice to have" improvements
- Never downgrade severity from framework definition

### 3. Deferral Handling (Framework-Specific)

The framework tracks **deferred work** as first-class concept:

**Valid deferrals (pass validation):**
1. External blocker (API v2 available 2025-12-01)
2. Scope change with ADR (Out of scope: ADR-042)
3. Story split (Deferred to STORY-125: performance optimization epic)

**Invalid deferrals (fail validation, HIGH or CRITICAL):**
1. No justification ("Will add later", "Not enough time")
2. Vague reason ("Too complex", "Hard to implement")
3. Circular chain (STORY-A → STORY-B → STORY-A)
4. Invalid story reference (STORY-XXX doesn't exist)
5. Missing ADR for scope change
6. Unnecessary deferral (feasible to implement now)
7. Multi-level chain (STORY-A → STORY-B → STORY-C) ← RCA-007

**Display guidance:**
- Deferral violations are high-impact (block approval)
- Provide specific remediation (fix justification, create ADR, complete work)
- Recommend returning to development for fixes
- Track deferral resolution across retry cycles

**Example display:**
```
❌ Deferral Validation Issues

Item: "Exit code handling for error framework"
Current reason: "Deferred to STORY-005"
Issue: STORY-005 also defers to STORY-006 (creates chain)
Required: Break chain by implementing in STORY-004 or STORY-005

→ Return to development: /dev STORY-004
```

### 4. Story Status Transitions

Result interpreter governs story status changes:

**Light QA:**
- PASS: No status change (continues in "In Development")
- FAIL: Block development (dev continues fixing)

**Deep QA:**
- PASS: "Dev Complete" → "QA Approved ✅"
- FAIL: "Dev Complete" → "QA Failed ❌"

**Display guidance:**
- Always show current status and transition
- Light mode: Note that deep QA required when ready
- Deep mode: Confirm approval status or failure reason
- Show status in YAML frontmatter update in workflow history

**Never change status:**
- During light validation (framework rule)
- If report missing or malformed
- If critical violations exist (approval blocked)

### 5. Anti-Pattern Categories (Tech-Stack Independent)

Anti-patterns in report come from anti-patterns.md context file:

```
Category 1: Structural Violations
  - Files in wrong locations (violates source-tree.md)
  - Layer dependency violations (violates architecture-constraints.md)
  - Library substitution (violates tech-stack.md)

Category 2: Security Anti-Patterns
  - SQL injection vulnerabilities
  - Hardcoded secrets/credentials
  - Weak cryptography (MD5, SHA1)
  - Missing authentication/authorization
  - XSS vulnerabilities

Category 3: Code Smells
  - God Objects (>500 lines or >20 methods)
  - Long methods (>100 lines)
  - High cyclomatic complexity (>10)
  - Code duplication (>5%)
  - Magic numbers

Category 4: Design Anti-Patterns
  - Direct instantiation (should use DI)
  - Copy-paste code
  - Circular dependencies
  - Tight coupling

Category 5: Testing Anti-Patterns
  - Over-mocking (more mocks than tests)
  - Assertion gaps (assertions <1.5 per test)
  - Test pyramid violation (not 70/20/10)
```

**Display guidance:**
- Group violations by category (not just severity)
- Reference specific anti-pattern (help user learn)
- Provide remediation for that specific pattern
- Link to coding-standards.md for more details

### 6. Spec Compliance Validation

Three compliance dimensions:

**Dimension 1: Acceptance Criteria**
- Each AC must have corresponding test(s)
- Tests must pass
- Implementation must match AC description

**Dimension 2: API Contracts**
- Each API endpoint must be implemented
- Request/response models must match spec
- All documented endpoints functional

**Dimension 3: Non-Functional Requirements (NFRs)**
- Performance NFRs: Benchmarks met, load tests pass
- Security NFRs: Auth/authz implemented, no vulnerabilities
- Scalability NFRs: Architecture supports scale targets

**Display guidance:**
- Show AC validation status (count passed/total)
- List failed ACs specifically (what test should exist?)
- Show API contract matches (endpoint → model)
- Note NFR validation method (benchmark, load test, code review)
- For failures, list specific requirements not met

---

## Display Template Guidelines

### Structure for All Templates

1. **Header** (2 lines)
   - Status emoji + Title: "✅ QA Validation PASSED"
   - Story ID and title

2. **Summary** (3-5 lines)
   - Result status
   - Validation mode
   - Key metrics (coverage, violations, status)

3. **Detailed Results** (varies by mode/result)
   - Light pass: Just "checks passed"
   - Light fail: Just "what broke"
   - Deep pass: Full metrics, all categories
   - Deep fail: Show what failed + remediation

4. **Recommendation** (1-2 lines)
   - "✅ APPROVE" (for pass)
   - "❌ FIX" (for fail)
   - "⚠️ REVIEW" (for partial/warning)

5. **Next Steps** (2-5 lines)
   - Specific action for user
   - Command to run if applicable
   - Link to detailed report

### Emoji Usage

- ✅ Pass, success, approved
- ❌ Fail, error, blocked
- ⚠️ Warning, attention needed
- ℹ️ Information, context
- → Arrow for actions/flow

### Tone Guidance

- **Light pass:** Brief, encouraging ("Great! Continue development")
- **Light fail:** Direct, urgent ("Build broken - fix required")
- **Deep pass:** Celebratory, confident ("Ready for release!")
- **Deep fail:** Constructive, specific ("Fix these violations then re-run")

### Length Guidelines

- **Light pass:** 8-12 lines total
- **Light fail:** 12-20 lines total
- **Deep pass:** 40-60 lines (show all metrics)
- **Deep fail:** 30-80 lines (show violations + remediation)

---

## Framework Integration Points

### 1. Context Files References

When recommending next steps or remediation, reference context files:

**tech-stack.md:**
- "Your project uses React (tech-stack.md) for UI components"
- Suggest framework-specific tooling

**source-tree.md:**
- "Files should follow source-tree.md structure"
- Suggest correct file locations for refactoring

**coding-standards.md:**
- "Your standards require <10 complexity per method (coding-standards.md)"
- Show what threshold was violated

**architecture-constraints.md:**
- "Your architecture (architecture-constraints.md) requires Presentation → Application → Domain"
- Show which constraint was violated

**anti-patterns.md:**
- "This pattern is forbidden (anti-patterns.md): Direct instantiation"
- Reference specific anti-pattern from your project

### 2. Related Skills and Subagents

Display names when recommending next actions:

**Deferral failures:**
- "Return to dev: `/dev STORY-XXX` (devforgeai-development skill will help resolve)"

**Coverage gaps:**
- "Use test stub generator (test-automator subagent can help)"

**Security violations:**
- "Security audit recommended (security-auditor subagent will scan)"

**Architecture issues:**
- "Scope change requires ADR (architect-reviewer subagent will validate)"

### 3. Workflow History

Include in next steps guidance:

- "This is QA attempt #2 of 3 (framework allows max 3 before suggesting story split)"
- "Story has been in 'In Development' for N days"
- "Follow-up story created for deferred work: STORY-XXX"

---

## Error Scenarios and Handling

### Scenario 1: Report File Missing

```
Status: ERROR
Issue: Report not found at devforgeai/qa/reports/{STORY_ID}-qa-report.md

Display:
"❌ QA Report Not Found

The QA report wasn't generated. This may indicate:
1. Skill execution was interrupted
2. File system permissions issue
3. Report directory doesn't exist

Recovery Options:
1. Re-run QA: `/qa {STORY_ID}`
2. Check directory exists: mkdir -p devforgeai/qa/reports
3. Review skill output for errors above"
```

### Scenario 2: Malformed Report

```
Status: ERROR
Issue: Report cannot be parsed (malformed JSON or structure)

Display:
"⚠️ QA Report Format Issue

The QA report appears to be malformed and cannot be fully parsed.

Attempted to extract: [list what was parsed]
Could not extract: [list what failed]

Recommendation:
1. Review raw report: devforgeai/qa/reports/{STORY_ID}-qa-report.md
2. Re-run QA to regenerate: `/qa {STORY_ID}`
3. Check skill logs for parsing errors"
```

### Scenario 3: Unclear Status

```
Status: UNCLEAR
Issue: Report status cannot be determined (not "PASSED" or "FAILED")

Display:
"ℹ️ QA Status Unclear

The report doesn't explicitly state PASS or FAIL, so I inferred:
Status: {INFERRED_STATUS}
Reasoning: {HOW_STATUS_WAS_DETERMINED}

Recommendation:
Review full report to confirm: devforgeai/qa/reports/{STORY_ID}-qa-report.md"
```

### Scenario 4: Retry Limit Approaching

```
Status: PASSED (with warning)
Issue: Story has failed QA multiple times

Display:
"✅ QA PASSED (with note)

Story is now QA Approved.

⚠️ Note: This is QA attempt #3 after 2 previous failures.
If this passes release but has production issues, consider:
1. Improving test coverage going forward
2. Adding integration tests
3. More thorough acceptance criteria review

Follow-up suggestion: If future stories also retry 3+ times, consider
breaking stories into smaller units earlier."
```

---

## Performance and Reliability

### Parsing Reliability

- Must handle reports with 0 violations (edge case: all pass)
- Must handle reports with 100+ violations (edge case: many failures)
- Must gracefully degrade if sections are missing
- Must infer status if explicit status unclear

### Consistency Requirements

- Same violation always produces same severity (no variability)
- Same result type always uses same template variant
- Mode + Status always maps to single template (deterministic)
- JSON output always has same structure (no optional top-level fields)

### Template Consistency

- All PASS displays follow same structure
- All FAIL displays follow same structure
- All templates use same emoji meanings
- All recommendations use same action format

---

## Testing Checklist for Subagent

Use this when testing qa-result-interpreter:

- [ ] Parses light mode PASS report
- [ ] Parses light mode FAIL report
- [ ] Parses deep mode PASS report
- [ ] Parses deep mode FAIL with coverage violations
- [ ] Parses deep mode FAIL with anti-pattern violations
- [ ] Parses deep mode FAIL with spec compliance violations
- [ ] Parses deep mode FAIL with deferral violations
- [ ] Parses report with 0 violations
- [ ] Parses report with 50+ violations (aggregation works)
- [ ] Handles missing report file gracefully
- [ ] Handles malformed report gracefully
- [ ] Generates correct status for all result types
- [ ] Generates appropriate display template for each case
- [ ] Provides actionable remediation guidance
- [ ] Recommends correct next steps
- [ ] References context files appropriately
- [ ] Shows coverage breakdown by layer (deep mode)
- [ ] Lists violations by severity correctly
- [ ] Handles retry count appropriately
- [ ] JSON output is valid and consistent

---

## Remember

This subagent is **NOT generating validation logic** — the skill does that.

This subagent is **interpreting and presenting** the results.

Your job:
1. Read what the skill validated ✅
2. Understand DevForgeAI context (story states, gates, constraints) ✅
3. Display results appropriately for the situation ✅
4. Recommend clear next steps ✅

**DO:**
- Reference context files when relevant
- Provide specific, actionable remediation
- Recommend next steps based on result type
- Handle edge cases gracefully

**DON'T:**
- Re-validate (skill already did that)
- Make decisions for the user (present options)
- Change story status (framework rules govern)
- Downgrade severity (use framework definitions)

