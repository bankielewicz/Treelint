---
id: EPIC-XXX
title: [Epic Title - High-Level Business Initiative]
status: Planning
start_date: YYYY-MM-DD
target_date: YYYY-MM-DD
total_points: [Estimated story points for entire epic]
completed_points: 0
created: YYYY-MM-DD
owner: [Product Owner Name]
tech_lead: [Tech Lead Name]
team: [Team Name]
---

# Epic: [Title]

## Business Goal

[What business problem does this epic solve? What value does it deliver to users/business?]

**Example:**
Improve checkout experience to increase conversion rate and reduce cart abandonment. Enable customers to complete purchases faster with fewer steps and clearer payment options.

## Success Metrics

Define measurable outcomes that indicate epic success:

- **Metric 1:** [e.g., Reduce checkout time by 30% (from 5 min to 3.5 min)]
- **Metric 2:** [e.g., Increase conversion rate by 15% (from 20% to 23%)]
- **Metric 3:** [e.g., Support 10,000 concurrent checkouts without degradation]
- **Metric 4:** [e.g., Reduce cart abandonment rate by 20% (from 70% to 56%)]

**Measurement Plan:**
- How will metrics be tracked? [Analytics platform, custom instrumentation, etc.]
- Baseline values: [Current state measurements]
- Target values: [Desired end state]
- Review frequency: [Weekly, bi-weekly, end of epic]

## Scope

### In Scope

High-level features and capabilities included in this epic:

1. **Feature 1:** [e.g., One-click guest checkout]
   - Brief description
   - Business value

2. **Feature 2:** [e.g., Saved payment methods]
   - Brief description
   - Business value

3. **Feature 3:** [e.g., Multi-currency support]
   - Brief description
   - Business value

4. **Feature 4:** [e.g., Checkout progress indicator]
   - Brief description
   - Business value

### Out of Scope

Explicitly call out what is NOT included to prevent scope creep:

- ❌ [e.g., Cryptocurrency payment support - deferred to next epic]
- ❌ [e.g., Buy now, pay later integration - not in MVP]
- ❌ [e.g., Subscription/recurring payment - future epic]

## Target Sprints

Break down epic into sprint-sized iterations:

### Sprint 1 (SPRINT-XXX): [Sprint Name/Theme]
**Goal:** [What will be achieved in this sprint]
**Estimated Points:** [Points for this sprint]
**Features:**
- Feature 1: [Story list]
- Feature 2: [Story list]

**Key Deliverables:**
- [Deliverable 1]
- [Deliverable 2]

### Sprint 2 (SPRINT-XXX): [Sprint Name/Theme]
**Goal:** [What will be achieved in this sprint]
**Estimated Points:** [Points for this sprint]
**Features:**
- Feature 3: [Story list]
- Feature 4: [Story list]

### Sprint 3 (SPRINT-XXX): [Sprint Name/Theme]
**Goal:** [What will be achieved in this sprint]
**Estimated Points:** [Points for this sprint]
**Features:**
- Polish & optimization
- Performance testing
- Bug fixes

## User Stories

High-level user stories that will be broken down into detailed stories:

1. **As a** [user role], **I want** [capability], **so that** [benefit]
2. **As a** [user role], **I want** [capability], **so that** [benefit]
3. **As a** [user role], **I want** [capability], **so that** [benefit]

*Note: Each high-level user story will decompose into 1-5 detailed story documents.*

## Technical Considerations

### Architecture Impact
- [New services/components needed]
- [Changes to existing architecture]
- [Data model changes]
- [Integration points with external systems]

### Technology Decisions
- [New technologies/libraries to evaluate or adopt]
- [Technology upgrades required]
- [Infrastructure changes needed]

### Security & Compliance
- [Security requirements (PCI DSS for payments, GDPR for data, etc.)]
- [Compliance constraints]
- [Audit requirements]

### Performance Requirements
- [Response time targets]
- [Throughput requirements]
- [Scalability goals]

## Dependencies

### Internal Dependencies
- [ ] **Dependency 1:** [e.g., User authentication epic must complete first]
  - **Status:** [In Progress / Complete / Not Started]
  - **Impact if delayed:** [Description]

- [ ] **Dependency 2:** [e.g., Payment gateway integration]
  - **Status:** [In Progress / Complete / Not Started]
  - **Impact if delayed:** [Description]

### External Dependencies
- [ ] **External Dependency 1:** [e.g., Stripe API v3 access]
  - **Owner:** [External team/vendor]
  - **ETA:** [Date]
  - **Status:** [On track / At risk / Blocked]

- [ ] **External Dependency 2:** [e.g., PCI DSS certification]
  - **Owner:** [Compliance team]
  - **ETA:** [Date]
  - **Status:** [On track / At risk / Blocked]

## Risks & Mitigation

### Risk 1: [Risk Description]
- **Probability:** [High / Medium / Low]
- **Impact:** [High / Medium / Low]
- **Mitigation:** [How to reduce or avoid risk]
- **Contingency:** [What to do if risk materializes]

### Risk 2: [Risk Description]
- **Probability:** [High / Medium / Low]
- **Impact:** [High / Medium / Low]
- **Mitigation:** [How to reduce or avoid risk]
- **Contingency:** [What to do if risk materializes]

## Stakeholders

### Primary Stakeholders
- **Product Owner:** [Name] - [Responsibilities]
- **Tech Lead:** [Name] - [Responsibilities]
- **Engineering Manager:** [Name] - [Responsibilities]

### Additional Stakeholders
- **Business:** [Names/roles needing updates]
- **Marketing:** [Names/roles for launch coordination]
- **Customer Support:** [Names/roles for training]
- **Legal/Compliance:** [Names/roles for review]

## Communication Plan

### Status Updates
- **Frequency:** [Weekly / Bi-weekly]
- **Format:** [Email / Slack / Dashboard]
- **Audience:** [Who receives updates]

### Demos
- **Sprint demos:** [When and to whom]
- **Milestone demos:** [Major checkpoints]

### Escalation Path
1. Team → Tech Lead
2. Tech Lead → Engineering Manager
3. Engineering Manager → VP Engineering
4. VP Engineering → C-level

## Timeline

```
Epic Timeline:
════════════════════════════════════════════════════
Week 1-2:  Sprint 1 - Core checkout flow
Week 3-4:  Sprint 2 - Payment methods & currency
Week 5-6:  Sprint 3 - Polish, performance, release
════════════════════════════════════════════════════
Total Duration: 6 weeks
Target Release: [Date]
```

### Key Milestones
- [ ] **Milestone 1:** [Date] - [Deliverable/checkpoint]
- [ ] **Milestone 2:** [Date] - [Deliverable/checkpoint]
- [ ] **Milestone 3:** [Date] - [Deliverable/checkpoint]
- [ ] **Final Release:** [Date] - Production deployment

## Progress Tracking

### Sprint Summary

| Sprint | Status | Points | Stories | Completed | In Progress | Blocked |
|--------|--------|--------|---------|-----------|-------------|---------|
| SPRINT-001 | Not Started | 20 | 8 | 0 | 0 | 0 |
| SPRINT-002 | Not Started | 18 | 7 | 0 | 0 | 0 |
| SPRINT-003 | Not Started | 12 | 5 | 0 | 0 | 0 |
| **Total** | **0%** | **50** | **20** | **0** | **0** | **0** |

*Updated automatically as sprints progress*

### Burndown
- **Total Points:** 50
- **Completed:** 0
- **Remaining:** 50
- **Velocity:** [Avg points per sprint after first sprint]

## Retrospective (Post-Epic)

*To be completed after epic completes*

### What Went Well
- [Success 1]
- [Success 2]

### What Could Be Improved
- [Improvement 1]
- [Improvement 2]

### Lessons Learned
- [Lesson 1]
- [Lesson 2]

### Metrics Achieved
- **Metric 1:** [Actual vs Target]
- **Metric 2:** [Actual vs Target]
- **Metric 3:** [Actual vs Target]

### Recommendations for Future Epics
- [Recommendation 1]
- [Recommendation 2]

---

**Epic Template Version:** 1.0
**Last Updated:** 2025-10-30
