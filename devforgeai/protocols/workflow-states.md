# Workflow States Protocol

**Version:** 1.0
**Date:** 2025-11-20
**Status:** Active Protocol
**Applies To:** All DevForgeAI stories, epics, and sprints

---

## Purpose

This protocol defines the canonical workflow states used in the DevForgeAI framework for tracking story, epic, and sprint progression through the Software Development Lifecycle (SDLC). These states enforce quality gates, ensure traceability, and provide clear visibility into work progression.

**Problem Solved:**
- Inconsistent status terminology across stories
- Unclear progression criteria between states
- Missing audit trail for workflow transitions
- Ambiguous readiness for quality gates

---

## Core Principle

**Every artifact (story/epic/sprint) MUST be in exactly ONE workflow state at any given time.**

States are:
- ✅ **Mutually exclusive** - An artifact cannot be in multiple states simultaneously
- ✅ **Sequentially ordered** - Progression follows defined state transitions (see `state-transitions.md`)
- ✅ **Quality-gated** - Each transition requires specific criteria to be met (see `quality-gates.md`)
- ✅ **Auditable** - All transitions are recorded in Workflow History section

---

## Workflow States

### 1. Backlog

**Definition:** Story/epic has been created and documented but not yet ready for development.

**Characteristics:**
- All acceptance criteria defined
- Technical specification complete (if required)
- Priority assigned
- Story points estimated
- Dependencies identified (if any)

**Applicable To:**
- Stories
- Epics
- Sprints

**Entry Criteria:**
- Story file created with complete YAML frontmatter
- All mandatory sections present (Description, Acceptance Criteria)
- Format version specified

**Exit Criteria:**
- All dependencies in "Dev Complete" or "QA Approved" state
- Sprint assignment confirmed (if applicable)
- Technical unknowns resolved

**Next States:**
- → Ready for Dev (when dependencies complete)
- → Cancelled (if scope changes)

---

### 2. Ready for Dev

**Definition:** All prerequisites satisfied; story is ready to begin development in the next available capacity window.

**Characteristics:**
- Zero blocking dependencies
- All technical questions answered
- Development environment prepared
- Context files validated (if applicable)

**Applicable To:**
- Stories

**Entry Criteria:**
- Status = "Backlog" previously
- All `depends_on` stories in "QA Approved" or "Released" state
- Tech stack validated against `devforgeai/context/tech-stack.md`
- Sprint assignment confirmed (optional)

**Exit Criteria:**
- Developer assigned
- /dev command initiated

**Next States:**
- → In Development (when work begins)
- → Backlog (if dependencies regress)

---

### 3. In Development

**Definition:** Active development work in progress using TDD workflow (Red → Green → Refactor → Integration).

**Characteristics:**
- Feature branch exists (if using Git)
- Tests being written (TDD Red phase)
- Implementation in progress (TDD Green phase)
- Refactoring applied (TDD Refactor phase)

**Applicable To:**
- Stories

**Entry Criteria:**
- Status = "Ready for Dev" previously
- Developer assigned
- /dev STORY-ID command executed
- Pre-flight validation passed (Git, tech stack, context files)

**Exit Criteria:**
- All acceptance criteria implemented
- All tests passing (unit + integration)
- Code committed (or tracked via file-based method if Git unavailable)
- Deferral Challenge Checkpoint passed (RCA-006 safeguard)

**Next States:**
- → Dev Complete (when implementation done)
- → Blocked (if dependency discovered)
- → Backlog (if deprioritized)

---

### 4. Dev Complete

**Definition:** Implementation finished, all tests passing, code committed. Awaiting QA validation.

**Characteristics:**
- 100% acceptance criteria implemented
- All tests passing (unit + integration + regression)
- Code quality meets standards
- No known defects
- Deferred Definition of Done items documented (if any)

**Applicable To:**
- Stories

**Entry Criteria:**
- Status = "In Development" previously
- TDD workflow complete (Phases 0-5)
- All tests passing
- Git commit created (or file-based tracking complete)
- Dev result interpreter summary generated

**Exit Criteria:**
- Ready for QA validation
- Story file updated with implementation summary

**Next States:**
- → QA Approved (when QA validation passes)
- → In Development (if defects found requiring rework)

**Automated Transition:**
- /dev command automatically updates status from "In Development" → "Dev Complete" when TDD workflow completes successfully

---

### 5. QA Approved

**Definition:** Story has passed all quality gates (coverage, anti-patterns, spec compliance, quality metrics) and is ready for release.

**Characteristics:**
- Test coverage ≥95% (strict), ≥85% (standard), ≥80% (acceptable)
- Zero anti-pattern violations
- 100% spec compliance
- Code quality score ≥80/100
- All deferred DoD items justified and documented

**Applicable To:**
- Stories
- Epics (when all constituent stories QA approved)

**Entry Criteria:**
- Status = "Dev Complete" previously
- /qa STORY-ID deep validation executed
- All 5 quality gates passed:
  1. Gate 1: Test Coverage (95%/85%/80% thresholds)
  2. Gate 2: Anti-Pattern Detection (zero violations)
  3. Gate 3: Spec Compliance (100% AC implementation)
  4. Gate 4: Code Quality (≥80/100 score)
  5. Gate 5: Deferral Validation (all justified)

**Exit Criteria:**
- QA report generated
- Story file updated with QA validation history

**Next States:**
- → Released (when deployed to production)
- → Dev Complete (if regression discovered)

**Automated Transition:**
- /qa command (deep mode, PASSED result) automatically updates status from "Dev Complete" → "QA Approved"

---

### 6. Released

**Definition:** Story deployed to production environment and verified functional.

**Characteristics:**
- Deployed to production environment
- Smoke tests passed
- No rollback required
- Release documentation complete

**Applicable To:**
- Stories
- Epics (when all stories released)
- Sprints (when all stories released)

**Entry Criteria:**
- Status = "QA Approved" previously
- /release STORY-ID production executed
- Deployment successful
- Smoke tests passing

**Exit Criteria:**
- Production verification complete
- Release notes published
- Metrics baseline captured

**Next States:**
- → Closed (when accepted by stakeholders)
- → QA Approved (if rollback required)

**Automated Transition:**
- /release command automatically updates status from "QA Approved" → "Released" when production deployment succeeds

---

### 7. Blocked

**Definition:** Work cannot progress due to external dependency, technical blocker, or unresolved question.

**Characteristics:**
- Blocker clearly documented
- Blocker owner identified
- Blocker resolution timeline estimated
- Work paused

**Applicable To:**
- Stories
- Epics

**Entry Criteria:**
- Blocker identified during any state
- Blocker prevents progression
- Blocker documented in story file

**Exit Criteria:**
- Blocker resolved
- Ready to resume previous state

**Next States:**
- → [Previous State] (when blocker cleared)
- → Cancelled (if blocker unresolvable)

**Note:** Blocked is an **overlay state** - the story retains its previous state context (e.g., "In Development - Blocked")

---

### 8. Cancelled

**Definition:** Work discontinued; story will not be implemented.

**Characteristics:**
- Cancellation reason documented
- No further work planned
- Resources reallocated

**Applicable To:**
- Stories
- Epics
- Sprints

**Entry Criteria:**
- Business decision to cancel
- Duplicate work identified
- Requirements invalidated

**Exit Criteria:**
- None (terminal state)

**Next States:**
- None (terminal state)

**Note:** Cancelled stories remain in `devforgeai/specs/Stories/` for audit trail but are excluded from active sprint planning.

---

## State Persistence

### YAML Frontmatter

All states are stored in story/epic YAML frontmatter:

```yaml
---
id: STORY-XXX
title: Story Title
status: QA Approved  # Current workflow state
created: 2025-11-20
updated: 2025-11-20
qa_approved: 2025-11-20  # Optional: timestamp of QA approval
released: 2025-11-20     # Optional: timestamp of release
---
```

### Workflow History Section

All state transitions are recorded in the story's Workflow History section:

```markdown
## Workflow History

- **2025-11-20 14:30:** Status: Backlog → Ready for Dev
- **2025-11-20 15:00:** Status: Ready for Dev → In Development (TDD workflow started)
- **2025-11-20 17:45:** Status: In Development → Dev Complete (all tests passing, 37/37)
- **2025-11-20 18:15:** Status: Dev Complete → QA Approved (deep validation: 100/100 score)
```

---

## Special Cases

### Epic States

Epics inherit state from constituent stories:

| Epic State | Criteria |
|------------|----------|
| **Backlog** | Epic created, stories not yet created |
| **In Development** | ≥1 story in "In Development" state |
| **QA Approved** | ALL stories in "QA Approved" or "Released" state |
| **Released** | ALL stories in "Released" state |

### Sprint States

Sprints follow similar aggregation:

| Sprint State | Criteria |
|--------------|----------|
| **Planned** | Sprint created, stories assigned |
| **Active** | Sprint start date reached, ≥1 story in development |
| **Complete** | ALL stories in "QA Approved" or "Released" state |

---

## Framework Integration

### Commands That Update States

| Command | State Transition | Automatic? |
|---------|------------------|------------|
| `/dev STORY-ID` | In Development → Dev Complete | Yes (Phase 5) |
| `/qa STORY-ID deep` | Dev Complete → QA Approved | Yes (if PASSED) |
| `/release STORY-ID production` | QA Approved → Released | Yes (if successful) |
| `/orchestrate STORY-ID` | Backlog → Released | Yes (full workflow) |

### State Validation

All commands validate current state before execution:

- `/dev` requires: Backlog, Ready for Dev, or In Development
- `/qa` requires: Dev Complete
- `/release` requires: QA Approved

Invalid state transitions are blocked with clear error messages.

---

## State Transition Rules

See `devforgeai/protocols/state-transitions.md` for:
- Valid transition paths
- Forbidden transitions
- Rollback scenarios
- Emergency procedures

---

## Quality Gates

See `devforgeai/protocols/quality-gates.md` for:
- Gate criteria per state transition
- Measurement methods
- Pass/fail thresholds
- Gate exemption process (if any)

---

## Related Documentation

- **State Transitions:** `devforgeai/protocols/state-transitions.md` - Valid transition rules
- **Quality Gates:** `devforgeai/protocols/quality-gates.md` - Gate criteria per transition
- **Lean Orchestration:** `devforgeai/protocols/lean-orchestration-pattern.md` - Command responsibilities
- **Story Template:** `devforgeai/templates/story-template.md` - Story structure with states

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-11-20 | Initial protocol based on 60+ existing stories |
