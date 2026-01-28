# Workflow States Reference

Complete reference for all 11 story workflow states in the DevForgeAI orchestration system.

## State Definitions

### 1. Backlog

**Description:** Story created but not yet started

**Characteristics:**
- Story has been defined with acceptance criteria
- Not yet assigned to active sprint (or assigned but sprint not started)
- Awaiting prioritization and capacity allocation
- No work has begun

**Entry Conditions:**
- Story document created with required sections
- Has at least one acceptance criterion
- Has technical specification (optional at this stage)

**Exit Conditions:**
- Story assigned to active sprint
- Acceptance criteria validated
- Dependencies identified and resolved
- Transitions to: **Architecture**

**Activities in this State:**
- Stakeholder review and approval
- Story point estimation
- Dependency identification
- Priority assignment

**Who's Responsible:**
- Product Owner (prioritization)
- Tech Lead (technical feasibility review)
- Development Team (estimation)

---

### 2. Architecture

**Description:** Creating/validating architectural context files

**Characteristics:**
- Context files are being created or validated
- devforgeai-architecture skill is invoked
- Architectural decisions are being documented
- ADRs (Architecture Decision Records) may be created

**Entry Conditions:**
- Story in Backlog state
- Story assigned to sprint
- Has clear acceptance criteria

**Exit Conditions:**
- All 6 context files exist:
  - tech-stack.md
  - source-tree.md
  - dependencies.md
  - coding-standards.md
  - architecture-constraints.md
  - anti-patterns.md
- Files are populated (not placeholder content)
- No architecture violations detected
- Transitions to: **Ready for Dev**

**Activities in this State:**
- Create/update context files
- Document architectural constraints
- Define layer boundaries
- Specify technology stack
- Create ADRs for major decisions
- Validate architecture against story requirements

**Who's Responsible:**
- devforgeai-architecture skill (automated)
- Architect (review and approval)
- Tech Lead (decision-making)

**Typical Duration:** Minutes to hours (if context exists, quick validation; if creating from scratch, longer)

---

### 3. Ready for Dev

**Description:** Context complete, waiting for development to start

**Characteristics:**
- All architectural prerequisites met
- Context files validated and current
- Story ready for developer to begin TDD workflow
- Waiting state (no active work)

**Entry Conditions:**
- Architecture state complete
- All 6 context files exist and valid
- No architecture violations

**Exit Conditions:**
- Developer begins work
- Transitions to: **In Development**

**Activities in this State:**
- Developer reviews story and context
- Developer reviews acceptance criteria
- Developer reviews technical specification
- Developer prepares development environment

**Who's Responsible:**
- Developer (starts work when ready)
- Tech Lead (available for questions)

**Typical Duration:** Minutes to hours (depends on developer availability)

---

### 4. In Development

**Description:** Active TDD implementation in progress

**Characteristics:**
- Developer executing TDD workflow
- devforgeai-development skill orchestrating 6 phases
- Light QA validation running automatically during phases
- Active coding, testing, refactoring

**Entry Conditions:**
- Ready for Dev state
- Developer ready to start
- Context files loaded

**Exit Conditions:**
- All 6 TDD phases complete:
  1. Context Validation
  2. Test-First (Red)
  3. Implementation (Green)
  4. Refactor
  5. Integration
  6. Git workflow
- All tests passing
- Build succeeds
- Light validation passed
- Transitions to: **Dev Complete**

**Activities in this State:**
- Write failing tests (Red)
- Implement code to pass tests (Green)
- Refactor for quality
- Integration testing
- Light QA validation (automatic)
- Code review
- Git commits

**Who's Responsible:**
- Developer (primary)
- devforgeai-development skill (orchestration)
- devforgeai-qa skill (light validation)

**Typical Duration:** Hours to days (depends on story complexity and points)

**Can Transition to Blocked:** Yes, if external dependency discovered

---

### 5. Dev Complete

**Description:** Development finished, ready for comprehensive QA

**Characteristics:**
- All development work complete
- All tests passing
- Build succeeds
- Light validation passed
- Awaiting deep QA validation

**Entry Conditions:**
- In Development state complete
- All TDD phases finished
- No test failures
- No build errors

**Exit Conditions:**
- Deep QA validation initiated
- Transitions to: **QA In Progress**

**Activities in this State:**
- Developer reviews code for final cleanup
- Developer ensures all tests documented
- Developer verifies acceptance criteria covered
- Prepare for QA validation

**Who's Responsible:**
- Developer (final verification)
- Orchestration (triggers QA automatically)

**Typical Duration:** Minutes (brief transition state)

---

### 6. QA In Progress

**Description:** Comprehensive quality validation running

**Characteristics:**
- devforgeai-qa skill executing deep validation
- 5 comprehensive validation phases running:
  1. Test Coverage Analysis
  2. Anti-Pattern Detection
  3. Spec Compliance Validation
  4. Code Quality Metrics
  5. QA Report Generation
- Automated analysis in progress

**Entry Conditions:**
- Dev Complete state
- All tests passing
- Build succeeds

**Exit Conditions:**
- QA validation complete
- QA report generated
- Transitions to: **QA Approved** (if PASS) or **QA Failed** (if FAIL)

**Activities in this State:**
- Coverage analysis
- Anti-pattern scanning
- Spec compliance checking
- Quality metrics calculation
- Security scanning
- Report generation

**Who's Responsible:**
- devforgeai-qa skill (automated)
- Quality tools (coverage, complexity, security scanners)

**Typical Duration:** Minutes (automated analysis)

---

### 7. QA Failed

**Description:** Quality violations detected, requires fixes

**Characteristics:**
- QA validation found violations
- Action items documented in QA report
- Developer must address issues
- Cannot proceed to release until fixed

**Entry Conditions:**
- QA In Progress state
- Deep validation result: FAIL
- Violations detected (critical, high, or coverage below threshold)

**Exit Conditions:**
- Developer ready to fix violations
- Transitions to: **In Development** (to fix issues)

**Activities in this State:**
- Developer reviews QA report
- Developer analyzes violations
- Developer prioritizes action items
- Developer plans fixes
- Developer starts fixing (transitions to In Development)

**Who's Responsible:**
- Developer (fix violations)
- Tech Lead (guidance for complex issues)
- QA report (provides fix guidance)

**Typical Duration:** Minutes to hours (review, then fix)

**Quality Violations Examples:**
- Coverage below thresholds (95%/85%/80%)
- Critical security vulnerabilities (SQL injection, XSS)
- Architecture violations (layer boundaries crossed)
- High complexity (methods >10, classes >50)
- Missing tests for acceptance criteria

---

### 8. QA Approved

**Description:** All quality gates passed, ready for release

**Characteristics:**
- Deep validation PASSED
- All quality metrics met
- Zero critical violations
- Zero high violations (or approved exceptions)
- Production-ready code

**Entry Conditions:**
- QA In Progress state
- Deep validation result: PASS
- Coverage meets thresholds (95%/85%/80%)
- No critical/high violations
- All acceptance criteria validated

**Exit Conditions:**
- Release initiated
- Transitions to: **Releasing**

**Activities in this State:**
- Review QA report for any medium/low recommendations
- Plan release timing
- Coordinate with stakeholders
- Prepare release notes
- Wait for release trigger

**Who's Responsible:**
- Product Owner (release approval)
- Tech Lead (technical approval)
- DevOps (release coordination)

**Typical Duration:** Minutes to days (depends on release cadence - immediate vs. coordinated sprint release)

**Quality Assurance:**
- ✅ Coverage: Business Logic ≥95%, Application ≥85%, Infrastructure ≥80%
- ✅ Zero CRITICAL violations
- ✅ Zero HIGH violations
- ✅ All acceptance criteria validated
- ✅ API contracts match spec
- ✅ NFRs validated
- ✅ Code quality metrics within thresholds

---

### 9. Releasing

**Description:** Deployment to production in progress

**Characteristics:**
- devforgeai-release skill executing deployment
- Release pipeline running
- Deployment to production environment
- Active deployment process

**Entry Conditions:**
- QA Approved state
- Release initiated
- All workflow checkboxes complete
- No blocking dependencies

**Exit Conditions:**
- Deployment successful
- Health checks pass
- Release verified
- Transitions to: **Released**

**Activities in this State:**
- Create release branch/tag
- Run deployment pipeline
- Execute database migrations (if needed)
- Deploy to production
- Run post-deployment health checks
- Verify deployment success
- Update release notes

**Who's Responsible:**
- devforgeai-release skill (automated)
- DevOps (monitoring)
- Infrastructure (deployment pipeline)

**Typical Duration:** Minutes (automated deployment)

**Rollback Triggers:**
- Deployment failure
- Health check failure
- Critical errors in production
- Rollback plan executed

---

### 10. Released

**Description:** Successfully deployed to production, story complete

**Characteristics:**
- Deployed to production environment
- All workflow stages complete
- Story closed
- Success state (terminal)

**Entry Conditions:**
- Releasing state
- Deployment successful
- Health checks passed
- Production verification complete

**Exit Conditions:**
- None (terminal state)
- Story lifecycle complete

**Activities in this State:**
- Monitor production for issues
- Update sprint progress
- Update epic progress
- Close story
- Celebrate success 🎉
- Post-release retrospective (if needed)

**Who's Responsible:**
- DevOps (production monitoring)
- Product Owner (stakeholder communication)
- Team (retrospective)

**Typical Duration:** N/A (permanent state)

**Release Information Documented:**
- Version number
- Environment (production)
- Timestamp
- Deployment notes
- Release notes

---

### 11. Blocked

**Description:** Waiting for external dependency or decision

**Characteristics:**
- Story cannot proceed due to external blocker
- Work paused
- Waiting for resolution
- Can occur from any state

**Entry Conditions:**
- External dependency identified
- Blocker prevents progress
- Cannot be resolved by team independently

**Exit Conditions:**
- Blocker resolved
- Transitions to: Previous state before blocking

**Activities in this State:**
- Document blocker details
- Identify owner of blocker
- Escalate if needed
- Create workaround if possible
- Track blocker status
- Notify stakeholders

**Who's Responsible:**
- Tech Lead (escalation, resolution coordination)
- Product Owner (priority decisions)
- External teams (if dependency on other teams)

**Typical Duration:** Variable (hours to weeks, depends on blocker)

**Common Blockers:**
- External API not ready
- Third-party service unavailable
- Dependency on another team's story
- Infrastructure/environment issues
- Stakeholder decision needed
- Regulatory/compliance approval pending

**Resolution Strategies:**
- Wait for dependency
- Create mock/stub to unblock
- De-prioritize story, start different story
- Escalate to leadership
- Implement workaround
- Negotiate timeline with external team

---

## State Transition Diagram

```
Backlog
  ↓
Architecture (context files created)
  ↓
Ready for Dev (waiting for developer)
  ↓
In Development (TDD workflow) ←──┐
  ↓                                │
Dev Complete                       │
  ↓                                │
QA In Progress                     │
  ↓           ↓                    │
QA Approved   QA Failed ──────────┘
  ↓
Releasing
  ↓
Released (COMPLETE)

                Blocked
                  ↓
        (Can occur from any state)
                  ↓
        (Returns to previous state)
```

## State Duration Expectations

| State | Typical Duration | Automated |
|-------|------------------|-----------|
| Backlog | Days to weeks | No |
| Architecture | Minutes to hours | Yes |
| Ready for Dev | Minutes to hours | No |
| In Development | Hours to days | Partial |
| Dev Complete | Minutes | No |
| QA In Progress | Minutes | Yes |
| QA Failed | Hours | No |
| QA Approved | Minutes to days | No |
| Releasing | Minutes | Yes |
| Released | Permanent | N/A |
| Blocked | Variable | No |

## State Categories

**Waiting States (Manual Trigger Required):**
- Backlog
- Ready for Dev
- QA Approved
- Blocked

**Active States (Work in Progress):**
- Architecture
- In Development
- QA In Progress
- Releasing

**Terminal States (Complete):**
- Released

**Failure States (Require Action):**
- QA Failed
- Blocked

## Monitoring and Reporting

### Story Status Dashboard

Track stories across all states:

```
Sprint SPRINT-001 Progress
═════════════════════════════
Backlog:        3 stories
Architecture:   0 stories
Ready for Dev:  2 stories
In Development: 4 stories
Dev Complete:   1 story
QA In Progress: 1 story
QA Failed:      0 stories
QA Approved:    2 stories
Releasing:      1 story
Released:       5 stories
Blocked:        1 story
─────────────────────────────
Total:         20 stories
Velocity:      25 points (5 released)
```

### State Transition Metrics

Track transition times for process improvement:

- Backlog → Released: 3 days average
- In Development → Dev Complete: 4 hours average
- Dev Complete → QA Approved: 10 minutes average
- QA Approved → Released: 1 day average (coordinated release)

### Bottleneck Identification

Monitor for states with long duration:
- Stories stuck in Blocked: Escalate
- QA Failed → In Development cycles: Review quality practices
- Long In Development: Review story point estimates

---

**Reference this document when:**
- Understanding story lifecycle
- Determining valid state transitions
- Troubleshooting workflow issues
- Training new team members
- Optimizing development process
