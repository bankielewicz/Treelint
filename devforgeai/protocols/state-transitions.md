# State Transitions Protocol

**Version:** 1.0
**Date:** 2025-11-20
**Status:** Active Protocol
**Applies To:** All DevForgeAI stories, epics, and sprints

**Related:**
- **Workflow States:** `devforgeai/protocols/workflow-states.md` - State definitions
- **Quality Gates:** `devforgeai/protocols/quality-gates.md` - Gate criteria per transition

---

## Purpose

This protocol defines the **valid state transition paths** for DevForgeAI workflow states, ensuring stories progress through the SDLC in a controlled, auditable manner with appropriate quality gates enforced at each transition.

**Problem Solved:**
- Uncontrolled state changes bypassing quality gates
- Stories jumping from Backlog directly to Released
- Regression without proper rollback procedures
- Inconsistent progression paths across stories

---

## Core Principles

### 1. Sequential Progression

Stories MUST progress through states in defined order:

```
Backlog → Ready for Dev → In Development → Dev Complete → QA Approved → Released
```

**Exception:** Blocked state can overlay any state temporarily.

### 2. Quality-Gated Transitions

Every forward transition MUST pass quality gates (see `quality-gates.md`):

- Backlog → Ready for Dev: Dependencies complete
- Ready for Dev → In Development: Pre-flight validation
- In Development → Dev Complete: All tests passing
- Dev Complete → QA Approved: 5 quality gates passed
- QA Approved → Released: Production deployment successful

### 3. Rollback Allowed

Backward transitions (rollbacks) are permitted for defect correction:

- Released → QA Approved (production rollback)
- QA Approved → Dev Complete (regression found)
- Dev Complete → In Development (defect requiring rework)

### 4. Audit Trail Required

ALL transitions MUST be recorded in Workflow History:

```markdown
## Workflow History

- **2025-11-20 14:30:** Status: Backlog → Ready for Dev (dependencies complete)
- **2025-11-20 15:00:** Status: Ready for Dev → In Development (/dev executed)
```

---

## Valid Transitions

### Forward Transitions (Normal Flow)

#### T1: Backlog → Ready for Dev

**Trigger:** All blocking dependencies resolved

**Quality Gate:** Gate 0 - Dependency Readiness
- All `depends_on` stories in "QA Approved" or "Released" state
- Tech stack validated against context files
- Technical unknowns resolved

**Automated:** No (manual status update)

**Command:** None (manual review process)

**Rollback:** Ready for Dev → Backlog (if dependency regresses)

---

#### T2: Ready for Dev → In Development

**Trigger:** Developer starts work using /dev command

**Quality Gate:** Gate 1 - Pre-Flight Validation
- Git repository available (or file-based tracking ready)
- Tech stack detection successful
- Context files validated
- Story file readable

**Automated:** Yes (by /dev command Phase 0)

**Command:** `/dev STORY-ID`

**Rollback:** In Development → Ready for Dev (if work abandoned)

**Validation:**
```bash
# Pre-flight checks (devforgeai-development skill Phase 0)
1. Git availability check
2. Tech stack detection
3. Context file validation
4. Story file load
```

---

#### T3: In Development → Dev Complete

**Trigger:** TDD workflow completes successfully

**Quality Gate:** Gate 2 - Implementation Complete
- All acceptance criteria implemented
- All tests passing (unit + integration)
- Code committed (or file-based tracking complete)
- Deferral Challenge Checkpoint passed (RCA-006)

**Automated:** Yes (by /dev command Phase 5)

**Command:** `/dev STORY-ID` (automatic transition at Phase 5 completion)

**Rollback:** Dev Complete → In Development (if defect found before QA)

**Validation:**
```bash
# TDD completion criteria
1. Red phase: Tests written and failing
2. Green phase: Tests passing
3. Refactor phase: Code quality improved
4. Integration phase: Cross-component tests passing
5. Git/tracking: Changes committed
```

---

#### T4: Dev Complete → QA Approved

**Trigger:** QA validation passes all 5 quality gates

**Quality Gate:** Gate 3 - QA Validation (5 sub-gates)
1. **Coverage Gate:** Test coverage ≥95%/85%/80% (strict/standard/acceptable)
2. **Anti-Pattern Gate:** Zero violations detected
3. **Spec Compliance Gate:** 100% acceptance criteria implemented
4. **Quality Gate:** Code quality score ≥80/100
5. **Deferral Gate:** All deferred DoD items justified

**Automated:** Yes (by /qa command when deep mode passes)

**Command:** `/qa STORY-ID deep`

**Rollback:** QA Approved → Dev Complete (if regression discovered)

**Validation:**
```bash
# QA validation phases (devforgeai-qa skill)
1. Phase 1: Test coverage analysis
2. Phase 2: Anti-pattern detection
3. Phase 3: Spec compliance check
4. Phase 4: Code quality metrics
5. Phase 5: QA report generation
6. Phase 6: Feedback hooks (optional)
7. Phase 7: Story status update (automatic)
```

---

#### T5: QA Approved → Released

**Trigger:** Production deployment successful

**Quality Gate:** Gate 4 - Release Readiness
- Deployment to production successful
- Smoke tests passing
- No critical errors in logs
- Rollback plan prepared

**Automated:** Yes (by /release command when production deployment succeeds)

**Command:** `/release STORY-ID production`

**Rollback:** Released → QA Approved (if production issues require rollback)

**Validation:**
```bash
# Release validation
1. Deployment strategy selected (blue-green, canary, rolling)
2. Pre-deployment checks passed
3. Deployment executed successfully
4. Smoke tests passed
5. Health checks green
```

---

### Backward Transitions (Rollback Flow)

#### R1: Released → QA Approved

**Trigger:** Production issue requiring rollback

**Reason:** Critical defect discovered in production

**Process:**
1. Execute rollback deployment
2. Update story status to "QA Approved"
3. Document rollback reason in Workflow History
4. Create hotfix story (if needed)

**Automated:** Partial (deployment rollback automatic, status update manual)

**Command:** `/release STORY-ID rollback`

---

#### R2: QA Approved → Dev Complete

**Trigger:** Regression discovered during release preparation or production monitoring

**Reason:** Quality gate violation found after initial approval

**Process:**
1. Document regression details
2. Update story status to "Dev Complete"
3. Re-run /dev to fix regression
4. Re-run /qa to re-validate

**Automated:** No (manual status update)

**Command:** Manual update (then `/dev STORY-ID` to fix)

---

#### R3: Dev Complete → In Development

**Trigger:** Defect found during QA preparation requiring code changes

**Reason:** Implementation defect or missing acceptance criteria

**Process:**
1. Document defect
2. Update story status to "In Development"
3. Fix implementation
4. Re-run TDD workflow
5. Re-transition to Dev Complete

**Automated:** No (manual status update)

**Command:** Manual update (then `/dev STORY-ID` to rework)

---

#### R4: In Development → Ready for Dev

**Trigger:** Work abandoned or deprioritized mid-development

**Reason:** Business decision to pause work

**Process:**
1. Stash or commit incomplete work (with user approval - RCA-008)
2. Update story status to "Ready for Dev"
3. Document pause reason in Workflow History

**Automated:** No (manual status update with git workflow)

**Command:** Manual update (with user approval for git operations)

---

#### R5: Ready for Dev → Backlog

**Trigger:** Dependency regression or technical blocker discovered

**Reason:** Blocking dependency moved back to earlier state

**Process:**
1. Identify regressed dependency
2. Update story status to "Backlog"
3. Document blocker in Workflow History
4. Monitor dependency for resolution

**Automated:** No (manual status update)

**Command:** Manual update

---

### Special Transitions (Overlay States)

#### B1: Any State → Blocked

**Trigger:** External blocker prevents progression

**Reason:** Dependency, technical issue, or unanswered question

**Process:**
1. Document blocker details (description, owner, timeline)
2. Add "Blocked" overlay to current state
3. Update Workflow History with blocker reason
4. Track blocker resolution

**Automated:** No (manual status update)

**Command:** Manual update

**Note:** State becomes "In Development - Blocked", "QA Approved - Blocked", etc.

---

#### B2: Blocked → [Previous State]

**Trigger:** Blocker resolved

**Reason:** Dependency complete, question answered, technical issue fixed

**Process:**
1. Verify blocker resolution
2. Remove "Blocked" overlay
3. Resume work in previous state
4. Document resolution in Workflow History

**Automated:** No (manual status update)

**Command:** Manual update

---

#### C1: Any State → Cancelled

**Trigger:** Business decision to discontinue work

**Reason:** Duplicate work, requirements invalidated, or scope change

**Process:**
1. Document cancellation reason
2. Update story status to "Cancelled"
3. Archive story (remains in devforgeai/specs/Stories/ for audit)
4. Reallocate resources

**Automated:** No (manual status update)

**Command:** Manual update

**Note:** Terminal state - no transitions out of Cancelled

---

## Transition Matrix

| From State | To State | Valid? | Trigger | Automated? | Quality Gate |
|------------|----------|--------|---------|------------|--------------|
| Backlog | Ready for Dev | ✅ | Dependencies complete | No | Gate 0: Dependency Readiness |
| Ready for Dev | In Development | ✅ | /dev executed | Yes | Gate 1: Pre-Flight |
| In Development | Dev Complete | ✅ | TDD complete | Yes | Gate 2: Implementation |
| Dev Complete | QA Approved | ✅ | /qa deep passed | Yes | Gate 3: QA (5 sub-gates) |
| QA Approved | Released | ✅ | /release production | Yes | Gate 4: Release |
| Released | QA Approved | ✅ | Rollback | Partial | None (rollback) |
| QA Approved | Dev Complete | ✅ | Regression | No | None (rollback) |
| Dev Complete | In Development | ✅ | Defect fix | No | None (rollback) |
| In Development | Ready for Dev | ✅ | Work paused | No | None (rollback) |
| Ready for Dev | Backlog | ✅ | Dependency regress | No | None (rollback) |
| Any | Blocked | ✅ | Blocker found | No | None (overlay) |
| Blocked | Previous | ✅ | Blocker resolved | No | None (overlay) |
| Any | Cancelled | ✅ | Business decision | No | None (terminal) |
| Backlog | QA Approved | ❌ | INVALID | - | Gate skipped |
| In Development | Released | ❌ | INVALID | - | Gates skipped |
| Cancelled | Any | ❌ | INVALID (terminal) | - | - |

---

## Forbidden Transitions

The following transitions are **EXPLICITLY FORBIDDEN** as they bypass quality gates:

### F1: Backlog → Dev Complete
**Reason:** Skips TDD workflow and implementation validation

### F2: Backlog → QA Approved
**Reason:** Skips development and testing entirely

### F3: Backlog → Released
**Reason:** Skips entire SDLC

### F4: In Development → QA Approved
**Reason:** Skips Dev Complete state and associated verification

### F5: In Development → Released
**Reason:** Skips QA validation and release process

### F6: Dev Complete → Released
**Reason:** Skips QA validation (all 5 gates)

### F7: Cancelled → [Any State]
**Reason:** Terminal state - work discontinued

**Enforcement:** Commands validate current state before execution and reject invalid transitions with error messages:

```
Error: Invalid state transition
Current state: Backlog
Requested transition: → Released
Reason: FORBIDDEN - Skips quality gates (Gate 1, 2, 3, 4)

Valid next states from Backlog:
- Ready for Dev (when dependencies complete)
- Cancelled (if work discontinued)
```

---

## Automated vs Manual Transitions

### Automated Transitions (Command-Driven)

These transitions are automatically executed by DevForgeAI commands:

| Transition | Command | Phase | Condition |
|------------|---------|-------|-----------|
| Ready for Dev → In Development | /dev | Phase 0 | Pre-flight passed |
| In Development → Dev Complete | /dev | Phase 5 | TDD complete |
| Dev Complete → QA Approved | /qa deep | Phase 7 | All 5 gates passed |
| QA Approved → Released | /release | Phase 5 | Deployment successful |

**Benefits:**
- Consistent state updates
- Automatic Workflow History entries
- Quality gate enforcement
- Reduced human error

---

### Manual Transitions (User-Driven)

These transitions require manual story file updates:

| Transition | Reason | Process |
|------------|--------|---------|
| Backlog → Ready for Dev | Dependencies complete | Edit status in YAML frontmatter |
| [Any] → Blocked | External blocker | Add blocker details, update status |
| Blocked → [Previous] | Blocker resolved | Remove blocker, restore status |
| [Any] → Cancelled | Business decision | Update status, document reason |
| Released → QA Approved | Rollback (partial) | /release rollback + manual status update |
| QA Approved → Dev Complete | Regression | Manual status update, then /dev |
| Dev Complete → In Development | Defect rework | Manual status update, then /dev |

**Process for Manual Update:**
1. Edit `devforgeai/specs/Stories/STORY-XXX.story.md`
2. Update `status:` field in YAML frontmatter
3. Add Workflow History entry with timestamp, transition, and reason
4. Save file

---

## Workflow History Documentation

ALL transitions MUST be documented in the Workflow History section with:

1. **Timestamp** (ISO 8601 format or human-readable)
2. **Transition** (From State → To State)
3. **Reason/Context** (Why transition occurred)
4. **Additional Details** (Optional: command used, validation results, etc.)

### Example Format

```markdown
## Workflow History

- **2025-11-20 14:00:00:** Status: Backlog (Story created)
- **2025-11-20 14:30:00:** Status: Backlog → Ready for Dev (STORY-042 dependency complete)
- **2025-11-20 15:00:00:** Status: Ready for Dev → In Development (/dev STORY-043 executed, pre-flight passed)
- **2025-11-20 17:45:00:** Status: In Development → Dev Complete (TDD cycle complete: 37/37 tests passing, all ACs implemented)
- **2025-11-20 18:15:00:** Status: Dev Complete → QA Approved (/qa STORY-043 deep: 100/100 quality score, zero violations)
- **2025-11-20 18:45:00:** Status: QA Approved → Released (/release STORY-043 production: deployment successful, smoke tests passed)
```

### Rollback Example

```markdown
## Workflow History

- **2025-11-20 18:45:00:** Status: QA Approved → Released (production deployment)
- **2025-11-20 19:30:00:** Status: Released → QA Approved (ROLLBACK: critical defect in user authentication, rolled back to v1.0.0)
- **2025-11-20 20:00:00:** Status: QA Approved → Dev Complete (regression analysis complete, fix required)
- **2025-11-20 20:15:00:** Status: Dev Complete → In Development (/dev STORY-043 re-executed for defect fix)
- **2025-11-20 21:00:00:** Status: In Development → Dev Complete (defect fixed, 40/40 tests passing including regression test)
- **2025-11-20 21:30:00:** Status: Dev Complete → QA Approved (/qa deep re-validation: 100/100 score)
- **2025-11-20 22:00:00:** Status: QA Approved → Released (re-deployment successful, regression test green)
```

---

## Epic and Sprint State Transitions

### Epic States

Epics aggregate constituent story states:

```
Epic Backlog → Epic In Development → Epic QA Approved → Epic Released
```

**Transition Rules:**
- **Backlog → In Development:** When first constituent story enters "In Development"
- **In Development → QA Approved:** When ALL constituent stories reach "QA Approved" or "Released"
- **QA Approved → Released:** When ALL constituent stories reach "Released"

**Rollback:**
- If ANY story rolls back from "Released" to earlier state, Epic also rolls back

---

### Sprint States

Sprints aggregate assigned story states:

```
Sprint Planned → Sprint Active → Sprint Complete
```

**Transition Rules:**
- **Planned → Active:** When sprint start date reached OR first story enters "In Development"
- **Active → Complete:** When ALL assigned stories reach "QA Approved" or "Released"

**Note:** Sprints do not transition to "Released" - they remain "Complete" after all stories are QA approved.

---

## Emergency Procedures

### Emergency Rollback (Production Critical)

**Scenario:** Critical production defect requiring immediate rollback

**Process:**
1. Execute `/release STORY-ID rollback` (automatic deployment rollback)
2. Manually update story status: Released → QA Approved
3. Document incident in Workflow History
4. Create hotfix story (if needed)
5. Follow normal workflow for fix (Dev → QA → Release)

**Bypass Allowed:** None - all transitions must follow normal flow

---

### State Corruption Recovery

**Scenario:** Story status incorrectly set (e.g., manual edit bypassed quality gates)

**Process:**
1. Identify correct state based on implementation status
2. Review Workflow History for last valid transition
3. Correct status in YAML frontmatter
4. Add Workflow History entry documenting correction
5. Re-run appropriate command to validate state (e.g., /qa to validate "QA Approved")

**Example:**
```markdown
## Workflow History

- **2025-11-20 18:00:** Status: Dev Complete → QA Approved (INVALID - manual edit bypassed /qa validation)
- **2025-11-20 18:15:** Status: QA Approved → Dev Complete (CORRECTION - state corrupted, reverted to last valid state)
- **2025-11-20 18:20:** Status: Dev Complete → QA Approved (/qa deep executed: 100/100 score, legitimate transition)
```

---

## Related Documentation

- **Workflow States:** `devforgeai/protocols/workflow-states.md` - State definitions
- **Quality Gates:** `devforgeai/protocols/quality-gates.md` - Gate criteria
- **Lean Orchestration:** `devforgeai/protocols/lean-orchestration-pattern.md` - Command responsibilities
- **RCA-006:** `devforgeai/RCA/RCA-006-autonomous-deferrals.md` - Deferral validation pattern
- **RCA-008:** `devforgeai/RCA/RCA-008-autonomous-git-stashing.md` - Git workflow safeguards

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-11-20 | Initial protocol based on workflow patterns from 60+ stories |
