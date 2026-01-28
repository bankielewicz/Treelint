# Quality Gates Protocol

**Version:** 1.0
**Date:** 2025-11-20
**Status:** Active Protocol
**Applies To:** All DevForgeAI state transitions

**Related:**
- **Workflow States:** `devforgeai/protocols/workflow-states.md` - State definitions
- **State Transitions:** `devforgeai/protocols/state-transitions.md` - Transition rules

---

## Purpose

This protocol defines the **quality gates** that must be passed for each DevForgeAI workflow state transition, ensuring consistent quality standards, preventing technical debt, and enforcing the zero-defect principle in the framework.

**Problem Solved:**
- Inconsistent quality standards across stories
- Technical debt accumulation from rushed transitions
- Missing validation steps during state changes
- Unclear pass/fail criteria for quality checks

---

## Core Principles

### 1. Zero-Defect Progression

Stories MUST meet ALL quality gate criteria before advancing to the next state. No exceptions for "we'll fix it later."

### 2. Automated Validation

Quality gates are enforced by DevForgeAI commands (`/dev`, `/qa`, `/release`) with automated measurement and objective pass/fail criteria.

### 3. Deferral Justification

If a Definition of Done item cannot be met, it MUST:
- Have a **justified technical reason** (not convenience)
- Be **documented** in the story file
- Have a **follow-up story** created (if required)
- Pass **deferral validation** (see RCA-006)

### 4. Progressive Rigor

Quality gates become more rigorous as stories progress:
- Early gates focus on readiness and structure
- Middle gates focus on implementation quality
- Late gates focus on production readiness

---

## Quality Gates Overview

| Gate # | Name | State Transition | Enforcement | Automated? |
|--------|------|------------------|-------------|------------|
| Gate 0 | Dependency Readiness | Backlog → Ready for Dev | Manual review | No |
| Gate 1 | Pre-Flight Validation | Ready for Dev → In Development | /dev Phase 0 | Yes |
| Gate 2 | Implementation Complete | In Development → Dev Complete | /dev Phase 5 | Yes |
| Gate 3 | QA Validation (5 sub-gates) | Dev Complete → QA Approved | /qa Phases 1-5 | Yes |
| Gate 4 | Release Readiness | QA Approved → Released | /release Phases 1-4 | Yes |

---

## Gate 0: Dependency Readiness

**Transition:** Backlog → Ready for Dev

**Purpose:** Ensure all prerequisites are satisfied before development begins

**Enforcement:** Manual review by team/developer

**Automated:** No

---

### Gate 0 Criteria

#### G0.1: Blocking Dependencies Complete

**Requirement:** All stories listed in `depends_on` YAML field MUST be in "QA Approved" or "Released" state.

**Measurement:**
```bash
# Manual check
1. Read story YAML frontmatter
2. Extract depends_on: [STORY-XXX, STORY-YYY]
3. Verify each dependency status = "QA Approved" or "Released"
```

**Pass Criteria:** 100% of dependencies in approved/released state

**Fail Action:** Story remains in "Backlog" until dependencies complete

---

#### G0.2: Tech Stack Validation

**Requirement:** All technologies used in the story MUST be listed in `devforgeai/context/tech-stack.md`.

**Measurement:**
```bash
# Manual check
1. Read story technical specification
2. Extract technologies mentioned (languages, frameworks, libraries)
3. Verify each technology exists in tech-stack.md
```

**Pass Criteria:** 100% of technologies are approved in context files

**Fail Action:** Update tech-stack.md or revise story specification

---

#### G0.3: Technical Unknowns Resolved

**Requirement:** All technical questions, unknowns, or blockers MUST be answered/resolved.

**Measurement:**
```bash
# Manual check
1. Review story description and ACs
2. Verify no "TBD" or "Unknown" placeholders exist
3. Confirm all architectural decisions made
```

**Pass Criteria:** Zero unresolved technical questions

**Fail Action:** Research and document answers, or create spike story

---

## Gate 1: Pre-Flight Validation

**Transition:** Ready for Dev → In Development

**Purpose:** Ensure development environment is ready and story is actionable

**Enforcement:** `/dev` command Phase 0 (Pre-Flight Validation)

**Automated:** Yes

---

### Gate 1 Criteria

#### G1.1: Git Repository Available

**Requirement:** Git repository MUST be available (or file-based tracking fallback confirmed).

**Measurement:**
```bash
# Automated check (devforgeai-development skill Phase 0)
git rev-parse --is-inside-work-tree 2>/dev/null
# Exit code 0 = Git available
# Exit code != 0 = Fallback to file-based tracking
```

**Pass Criteria:** Git available OR file-based tracking confirmed

**Fail Action:** /dev command provides fallback instructions

**Deferral Allowed:** Yes (file-based tracking is valid alternative per RCA-008)

---

#### G1.2: Tech Stack Detection

**Requirement:** Project technology stack MUST be detectable and valid.

**Measurement:**
```bash
# Automated check (tech-stack-detector subagent)
1. Detect languages (package.json → Node.js, requirements.txt → Python, etc.)
2. Detect frameworks (React, .NET, etc.)
3. Validate against devforgeai/context/tech-stack.md
```

**Pass Criteria:** Tech stack detected and matches context file

**Fail Action:** /dev command fails with tech stack mismatch error

---

#### G1.3: Context Files Validated

**Requirement:** All 6 context files MUST exist and be readable.

**Measurement:**
```bash
# Automated check (context-validator subagent)
for file in tech-stack.md source-tree.md dependencies.md \
            coding-standards.md architecture-constraints.md anti-patterns.md; do
  [ -f "devforgeai/context/$file" ] || echo "MISSING: $file"
done
```

**Pass Criteria:** All 6 context files present and readable

**Fail Action:** /dev command fails with missing context file error

**Deferral Allowed:** No (context files are mandatory for framework operation)

---

#### G1.4: Story File Readable

**Requirement:** Story file MUST be loadable and parseable.

**Measurement:**
```bash
# Automated check (/dev command Phase 0)
Read(file_path="devforgeai/specs/Stories/STORY-XXX.story.md")
# Success = file exists and is readable
# Failure = file missing or corrupted
```

**Pass Criteria:** Story file loads successfully

**Fail Action:** /dev command fails with file not found error

---

## Gate 2: Implementation Complete

**Transition:** In Development → Dev Complete

**Purpose:** Ensure implementation is functionally complete with all tests passing

**Enforcement:** `/dev` command Phase 5 (Git/Tracking & Finalization)

**Automated:** Yes

---

### Gate 2 Criteria

#### G2.1: All Acceptance Criteria Implemented

**Requirement:** 100% of acceptance criteria MUST be implemented (or justifiably deferred).

**Measurement:**
```bash
# Automated check (devforgeai-development skill Phase 5)
1. Parse all ACs from story file
2. Verify implementation exists for each AC
3. Check for deferred items in Definition of Done
4. Validate deferrals meet RCA-006 criteria
```

**Pass Criteria:**
- 100% ACs implemented, OR
- Deferred ACs have justified technical reason + follow-up story

**Fail Action:** /dev command reports missing ACs and blocks transition

**Deferral Allowed:** Yes (with RCA-006 justification)

---

#### G2.2: All Tests Passing

**Requirement:** 100% of tests (unit + integration + regression) MUST pass.

**Measurement:**
```bash
# Automated check (TDD Green & Integration phases)
# Example for Python:
pytest tests/ --tb=short
# Exit code 0 = all tests passing

# Example for Node.js:
npm test
# Exit code 0 = all tests passing
```

**Pass Criteria:** Test exit code = 0 (all tests passed)

**Fail Action:** /dev command fails with test failure summary

**Deferral Allowed:** No (tests MUST pass - zero tolerance for failing tests)

---

#### G2.3: Code Committed (or Tracked)

**Requirement:** All changes MUST be committed to Git (or tracked via file-based method).

**Measurement:**
```bash
# Automated check (/dev command Phase 5)
# Git method:
git diff --exit-code  # Exit code 0 = no uncommitted changes

# File-based method:
# Verify implementation files exist and are tracked in story file
```

**Pass Criteria:**
- Git: All changes committed, OR
- File-based: Implementation files documented in story

**Fail Action:** /dev command prompts for commit or file-based tracking

**Deferral Allowed:** No (tracking is mandatory for audit trail)

---

#### G2.4: Deferral Challenge Checkpoint

**Requirement:** All deferred Definition of Done items MUST pass deferral validation (RCA-006).

**Measurement:**
```bash
# Automated check (devforgeai-development skill Phase 4.5)
1. Extract all deferred DoD items from story
2. Validate each deferral has:
   - Justified technical reason (not "ran out of time")
   - Follow-up story reference (if required)
   - No circular deferrals (story A defers to B, B defers to A)
```

**Pass Criteria:**
- Zero deferrals, OR
- All deferrals meet RCA-006 justification criteria

**Fail Action:** /dev command challenges unjustified deferrals

**Deferral Allowed:** Yes (with proper justification)

**Reference:** `devforgeai/RCA/RCA-006-autonomous-deferrals.md`

---

## Gate 3: QA Validation (5 Sub-Gates)

**Transition:** Dev Complete → QA Approved

**Purpose:** Ensure implementation meets quality standards for production readiness

**Enforcement:** `/qa STORY-ID deep` command (Phases 1-5)

**Automated:** Yes

---

### Gate 3.1: Test Coverage Gate

**Requirement:** Test coverage MUST meet minimum thresholds based on mode.

**Measurement:**
```bash
# Automated check (devforgeai-qa skill Phase 1)
# Example for Python:
pytest --cov=src --cov-report=term-missing
# Parse coverage percentage from output

# Example for Node.js:
npm test -- --coverage
# Parse coverage from JSON report
```

**Pass Criteria (Tiered):**
- **Strict Mode:** ≥95% coverage (critical features, security, data integrity)
- **Standard Mode:** ≥85% coverage (most features)
- **Acceptable Mode:** ≥80% coverage (non-critical features)

**Fail Action:** /qa command reports coverage gaps and blocks transition

**Deferral Allowed:** No (coverage is measurable and achievable)

---

### Gate 3.2: Anti-Pattern Detection Gate

**Requirement:** ZERO anti-pattern violations detected.

**Measurement:**
```bash
# Automated check (devforgeai-qa skill Phase 2)
1. Read devforgeai/context/anti-patterns.md
2. Scan codebase for each anti-pattern
3. Report violations with file:line references
```

**Anti-Patterns Checked:**
- God objects (classes > 300 lines)
- Circular dependencies
- Magic numbers (hardcoded constants)
- Code duplication (DRY violations)
- Nested conditionals (> 3 levels)
- Long methods (> 50 lines)
- Framework-specific anti-patterns (from context file)

**Pass Criteria:** Zero violations detected

**Fail Action:** /qa command reports violations with remediation guidance

**Deferral Allowed:** No (anti-patterns are code quality issues, not features)

---

### Gate 3.3: Spec Compliance Gate

**Requirement:** 100% of acceptance criteria MUST be verifiably implemented.

**Measurement:**
```bash
# Automated check (devforgeai-qa skill Phase 3)
1. Parse acceptance criteria from story file
2. For each AC, verify:
   - Test exists covering AC
   - Implementation matches AC description
   - AC marked as complete (not deferred without justification)
```

**Pass Criteria:** 100% ACs implemented or justifiably deferred

**Fail Action:** /qa command reports missing/incomplete ACs

**Deferral Allowed:** Yes (with RCA-006 justification)

---

### Gate 3.4: Code Quality Gate

**Requirement:** Code quality score MUST be ≥80/100.

**Measurement:**
```bash
# Automated check (devforgeai-qa skill Phase 4)
# Composite score from:
1. Cyclomatic complexity (weight: 30%)
   - Score = 100 - (avg_complexity - 5) * 10
   - Target: avg complexity ≤ 10
2. Code duplication (weight: 20%)
   - Score = 100 - (duplication_pct * 5)
   - Target: < 5% duplication
3. Documentation coverage (weight: 20%)
   - Score = docstring_coverage_pct
   - Target: ≥ 80% functions documented
4. Naming conventions (weight: 15%)
   - Score = 100 - (violations * 2)
   - Target: < 10 naming violations
5. Code organization (weight: 15%)
   - Score = 100 - (structure_issues * 5)
   - Target: proper layering, separation of concerns
```

**Pass Criteria:** Composite score ≥ 80/100

**Fail Action:** /qa command reports quality metrics and improvement recommendations

**Deferral Allowed:** No (quality is measurable and achievable through refactoring)

---

### Gate 3.5: Deferral Validation Gate

**Requirement:** All deferred Definition of Done items MUST have:
1. Justified technical reason (not convenience)
2. Follow-up story reference (if work required)
3. No circular deferrals

**Measurement:**
```bash
# Automated check (deferral-validator subagent)
1. Extract deferred DoD items from story
2. For each deferral:
   - Verify justification is technical (not "time constraint")
   - Verify follow-up story exists (if applicable)
   - Verify no circular deferral chain
3. Calculate deferral severity (NONE/LOW/MEDIUM/HIGH)
```

**Pass Criteria:**
- Deferral severity ≤ MEDIUM, AND
- All deferrals have technical justification, AND
- No circular deferrals

**Fail Action:** /qa command challenges invalid deferrals

**Deferral Allowed:** N/A (this IS the deferral validation gate)

**Reference:** `devforgeai/RCA/RCA-006-autonomous-deferrals.md`

---

## Gate 4: Release Readiness

**Transition:** QA Approved → Released

**Purpose:** Ensure deployment to production is safe and successful

**Enforcement:** `/release STORY-ID production` command (Phases 1-4)

**Automated:** Yes

---

### Gate 4 Criteria

#### G4.1: Deployment Strategy Validated

**Requirement:** Appropriate deployment strategy MUST be selected and validated.

**Measurement:**
```bash
# Automated check (devforgeai-release skill Phase 1)
1. Detect deployment environment (Kubernetes, Docker, serverless, etc.)
2. Select deployment strategy:
   - Blue-Green (zero-downtime, instant rollback)
   - Canary (gradual rollout, risk mitigation)
   - Rolling (progressive updates)
   - Recreate (simple, downtime acceptable)
3. Validate strategy is appropriate for story type
```

**Pass Criteria:** Valid deployment strategy selected and validated

**Fail Action:** /release command fails with strategy validation error

---

#### G4.2: Pre-Deployment Checks Passed

**Requirement:** All pre-deployment validations MUST pass.

**Measurement:**
```bash
# Automated check (devforgeai-release skill Phase 2)
1. Environment health check (CPU, memory, disk space)
2. Dependency availability (databases, APIs, services)
3. Configuration validation (env vars, secrets)
4. Rollback plan prepared (previous version available)
5. Monitoring/alerting configured
```

**Pass Criteria:** All pre-deployment checks return success

**Fail Action:** /release command fails with pre-deployment error details

---

#### G4.3: Deployment Successful

**Requirement:** Deployment to production MUST complete without errors.

**Measurement:**
```bash
# Automated check (devforgeai-release skill Phase 3)
# Example for Kubernetes:
kubectl rollout status deployment/app-name
# Exit code 0 = deployment successful

# Example for Docker:
docker ps | grep app-name
# Container running = deployment successful
```

**Pass Criteria:** Deployment command exit code = 0

**Fail Action:** /release command triggers automatic rollback

---

#### G4.4: Smoke Tests Passed

**Requirement:** Post-deployment smoke tests MUST pass.

**Measurement:**
```bash
# Automated check (devforgeai-release skill Phase 4)
1. Health endpoint check (HTTP 200 response)
2. Critical path verification (login, core features work)
3. Database connectivity check
4. External API integration check
5. No critical errors in logs (last 5 minutes)
```

**Pass Criteria:** All smoke tests pass within 5 minutes of deployment

**Fail Action:** /release command triggers automatic rollback

---

## Quality Gate Exemptions

### When Are Exemptions Allowed?

**NEVER for:**
- Gate 2.2: All Tests Passing (zero tolerance for failing tests)
- Gate 3.2: Anti-Pattern Detection (zero tolerance for code quality violations)

**CONDITIONALLY for:**
- Gate 3.1: Test Coverage (if deferred with technical justification per RCA-006)
- Gate 3.3: Spec Compliance (if ACs deferred with technical justification)
- Gate 3.5: Deferral Validation (if deferral severity ≤ MEDIUM)

**Process for Exemption:**
1. Developer documents technical reason in story Definition of Done
2. Deferral validator subagent evaluates justification
3. If justified: exemption granted, follow-up story created
4. If not justified: /qa command challenges deferral, blocks transition

---

## Quality Gate Failure Handling

### Automated Failure Response

When a quality gate fails, the enforcing command:

1. **Blocks state transition** - Story remains in current state
2. **Generates detailed report** - Specific failures with file:line references
3. **Provides remediation guidance** - Steps to resolve each failure
4. **Calculates quality score** - Overall quality metric (0-100)
5. **Returns to user** - Clear error message with next steps

### Example Failure Output

```
❌ QA VALIDATION FAILED - Story remains in "Dev Complete" state

Quality Score: 72/100 (threshold: 80/100)

Gate Failures:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

❌ Gate 3.1: Test Coverage (FAILED)
   Current: 78% | Required: 85% (Standard Mode)
   Missing coverage:
   - src/services/auth.py:45-67 (login method)
   - src/services/auth.py:89-102 (token validation)

   Remediation: Add tests covering 7% gap (est. 3 test cases)

❌ Gate 3.4: Code Quality (FAILED)
   Score: 72/100 | Required: 80/100
   Issues:
   - Cyclomatic complexity: 12.3 (target: ≤10)
     - src/handlers/request.py:process_request() complexity=15
   - Code duplication: 8% (target: <5%)
     - src/utils/validation.py duplicates src/utils/helpers.py

   Remediation: Refactor complex methods, extract duplicated code

✅ Gate 3.2: Anti-Pattern Detection (PASSED)
✅ Gate 3.3: Spec Compliance (PASSED)
✅ Gate 3.5: Deferral Validation (PASSED)

Next Steps:
1. Fix test coverage gaps (add 3 tests)
2. Refactor process_request() method to reduce complexity
3. Extract duplicated validation code to shared module
4. Re-run: /qa STORY-XXX deep
```

---

## Quality Metrics Dashboard

### Per-Story Quality Summary

Each story tracks quality metrics in QA Validation section:

```markdown
## QA Validation

**Validation Mode:** Deep
**Timestamp:** 2025-11-20 18:15:00
**Overall Quality Score:** 100/100 ✅

### Gate Results

| Gate | Status | Score | Details |
|------|--------|-------|---------|
| 3.1: Test Coverage | ✅ PASSED | 98% | 95%+ threshold (Strict Mode) |
| 3.2: Anti-Patterns | ✅ PASSED | 100% | Zero violations |
| 3.3: Spec Compliance | ✅ PASSED | 100% | All 7 ACs implemented |
| 3.4: Code Quality | ✅ PASSED | 95/100 | Above 80/100 threshold |
| 3.5: Deferral Validation | ✅ PASSED | N/A | Zero deferrals |

**Recommendation:** ✅ APPROVED FOR RELEASE
```

---

## Framework Integration

### Commands That Enforce Quality Gates

| Command | Gates Enforced | Phase |
|---------|----------------|-------|
| `/dev STORY-ID` | Gate 1 (Pre-Flight), Gate 2 (Implementation) | Phases 0, 5 |
| `/qa STORY-ID deep` | Gate 3 (QA - all 5 sub-gates) | Phases 1-5 |
| `/release STORY-ID production` | Gate 4 (Release Readiness) | Phases 1-4 |
| `/orchestrate STORY-ID` | All gates (Gate 1-4) | Full workflow |

### Subagents That Measure Quality

| Subagent | Purpose | Gates Measured |
|----------|---------|----------------|
| `git-validator` | Git availability check | G1.1 |
| `tech-stack-detector` | Tech stack validation | G1.2 |
| `context-validator` | Context files validation | G1.3 |
| `test-automator` | Test execution and coverage | G2.2, G3.1 |
| `code-analyzer` | Anti-pattern detection | G3.2 |
| `deferral-validator` | Deferral justification | G2.4, G3.5 |
| `integration-tester` | Integration test execution | G2.2 |
| `deployment-engineer` | Deployment and smoke tests | G4.1-G4.4 |

---

## Related Documentation

- **Workflow States:** `devforgeai/protocols/workflow-states.md` - State definitions
- **State Transitions:** `devforgeai/protocols/state-transitions.md` - Transition rules
- **Lean Orchestration:** `devforgeai/protocols/lean-orchestration-pattern.md` - Command architecture
- **RCA-006:** `devforgeai/RCA/RCA-006-autonomous-deferrals.md` - Deferral validation pattern
- **Anti-Patterns:** `devforgeai/context/anti-patterns.md` - Framework anti-patterns
- **Coding Standards:** `devforgeai/context/coding-standards.md` - Code quality standards

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-11-20 | Initial protocol based on QA validation patterns from 60+ stories |
