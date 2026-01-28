# Feature Prioritization Matrix

**Project:** [Project Name]
**Date Created:** YYYY-MM-DD
**Last Updated:** YYYY-MM-DD
**Created By:** [Name]

---

## Instructions

This matrix helps prioritize features based on two dimensions:
1. **Value** - How much value does this feature provide to users/business?
2. **Effort** - How much development effort is required?

**Prioritization Strategy:**
- **Quick Wins (High Value, Low Effort)** → Build First (MVP)
- **Strategic (High Value, High Effort)** → Plan for Later Phases
- **Fill-Ins (Low Value, Low Effort)** → Consider if Resources Allow
- **Time Sinks (Low Value, High Effort)** → Avoid or Defer Indefinitely

---

## Feature List with Scoring

### Scoring Rubric

#### Value Score (1-10)

**Business Value:**
- 10: Critical for business success, direct revenue impact
- 7-9: High value, significant user/business benefit
- 4-6: Medium value, nice-to-have improvement
- 1-3: Low value, minimal impact

**User Value:**
- 10: Solves critical user pain point, users can't live without it
- 7-9: Addresses major user need, significant satisfaction increase
- 4-6: Nice-to-have, improves experience
- 1-3: Minor improvement, few users benefit

#### Effort Score (1-10)

**Development Complexity:**
- 1-3: Low effort (1-3 days, single developer, straightforward implementation)
- 4-6: Medium effort (1-2 weeks, requires coordination, moderate complexity)
- 7-9: High effort (3-6 weeks, multiple developers, complex implementation)
- 10: Very high effort (>6 weeks, architectural changes, high risk)

**Risk/Uncertainty:**
- Add +2 if integration with external systems required
- Add +2 if requires new technology/framework learning
- Add +2 if dependencies on other teams/features
- Add +1 if uncertain requirements

---

## Features

| # | Feature Name | Description | Value (1-10) | Effort (1-10) | Priority | Category | MVP? |
|---|--------------|-------------|--------------|---------------|----------|----------|------|
| 1 | [Feature 1] | [Brief description] | [Score] | [Score] | [Calculated] | [Category] | [Y/N] |
| 2 | [Feature 2] | [Brief description] | [Score] | [Score] | [Calculated] | [Category] | [Y/N] |
| 3 | [Feature 3] | [Brief description] | [Score] | [Score] | [Calculated] | [Category] | [Y/N] |

**Priority Calculation:**
- **Priority Score = Value / Effort**
- Higher score = Higher priority
- Scores > 2.0 = High Priority (Quick Wins or Strategic)
- Scores 1.0-2.0 = Medium Priority
- Scores < 1.0 = Low Priority

---

## Detailed Feature Analysis

### Feature 1: [Feature Name]

**Description:** [Detailed description of what this feature does]

**Value Score: [X]/10**
- **Business Value:** [X/10] - [Rationale]
- **User Value:** [X/10] - [Rationale]
- **Average:** [X/10]

**Effort Score: [Y]/10**
- **Development Time:** [Est. X days/weeks]
- **Developers Required:** [X developers]
- **Complexity:** [Low / Medium / High]
- **External Dependencies:** [None / List]
- **Risk Factors:** [List any technical risks]
- **Total Effort:** [Y/10]

**Priority Score: [Value/Effort] = [Z]**

**Category:** [Quick Win / Strategic / Fill-In / Time Sink]

**Recommendation:**
- [ ] **Build for MVP** (Quick Win)
- [ ] **Build in Phase 2** (Strategic)
- [ ] **Build if Resources Allow** (Fill-In)
- [ ] **Defer Indefinitely** (Time Sink)

**Dependencies:**
- [Dependency 1: Must have Feature X first]
- [Dependency 2: Requires API access from System Y]

**User Stories:**
1. As a [user], I want to [action], so that [benefit]
2. As a [user], I want to [action], so that [benefit]

**Acceptance Criteria:**
- [ ] [Criterion 1]
- [ ] [Criterion 2]
- [ ] [Criterion 3]

---

[Repeat for all features]

---

## Prioritization Matrix (Visual)

```
        High Value
            │
            │
  STRATEGIC │  QUICK WINS
            │
  (Plan)    │  (Build Now)
            │
────────────┼────────────── Low Effort → High Effort
            │
  TIME SINKS│  FILL-INS
            │
  (Avoid)   │  (Maybe Later)
            │
        Low Value
```

### Quadrant 1: Quick Wins (High Value, Low Effort)
**Build First - MVP**

| Feature # | Feature Name | Value | Effort | Priority Score |
|-----------|--------------|-------|--------|----------------|
| [X] | [Feature Name] | [X] | [Y] | [Z] |
| [X] | [Feature Name] | [X] | [Y] | [Z] |

**Recommendation:** Build all these features in MVP.

---

### Quadrant 2: Strategic (High Value, High Effort)
**Plan for Later Phases**

| Feature # | Feature Name | Value | Effort | Priority Score |
|-----------|--------------|-------|--------|----------------|
| [X] | [Feature Name] | [X] | [Y] | [Z] |
| [X] | [Feature Name] | [X] | [Y] | [Z] |

**Recommendation:** Plan these features for Phase 2 or 3. High value justifies the effort, but MVP should focus on Quick Wins first.

---

### Quadrant 3: Fill-Ins (Low Value, Low Effort)
**Consider if Resources Allow**

| Feature # | Feature Name | Value | Effort | Priority Score |
|-----------|--------------|-------|--------|----------------|
| [X] | [Feature Name] | [X] | [Y] | [Z] |
| [X] | [Feature Name] | [X] | [Y] | [Z] |

**Recommendation:** Build these if time permits after Quick Wins are complete. Low effort means low risk.

---

### Quadrant 4: Time Sinks (Low Value, High Effort)
**Avoid or Defer Indefinitely**

| Feature # | Feature Name | Value | Effort | Priority Score |
|-----------|--------------|-------|--------|----------------|
| [X] | [Feature Name] | [X] | [Y] | [Z] |
| [X] | [Feature Name] | [X] | [Y] | [Z] |

**Recommendation:** Avoid building these features. Effort doesn't justify the value. Re-evaluate if user feedback changes the value assessment.

---

## MVP Feature Set

Based on prioritization, the MVP should include:

### Must-Have (Quick Wins)
1. [Feature X] - Priority: [Z]
2. [Feature Y] - Priority: [Z]
3. [Feature Z] - Priority: [Z]

**Estimated MVP Effort:** [Total effort score or weeks]

### Should-Have (If Time Permits)
1. [Feature A] - Priority: [Z]
2. [Feature B] - Priority: [Z]

**Estimated Additional Effort:** [Total effort score or weeks]

### Won't Have (Deferred to Later Phases)
1. [Feature C] - Deferred to Phase 2
2. [Feature D] - Deferred to Phase 3

---

## Phase Planning

### Phase 1 (MVP) - [Timeline: X weeks]
**Goal:** [Primary goal - e.g., "Validate core user flow and gather feedback"]

**Features:**
1. [Feature 1] - [Effort: X days]
2. [Feature 2] - [Effort: X days]
3. [Feature 3] - [Effort: X days]

**Total Effort:** [X weeks]

**Success Metrics:**
- [Metric 1: e.g., "100 users sign up in first 30 days"]
- [Metric 2: e.g., "70% task completion rate"]

---

### Phase 2 - [Timeline: Y weeks]
**Goal:** [e.g., "Enhance functionality based on MVP feedback"]

**Features:**
1. [Feature 4] - [Effort: X days]
2. [Feature 5] - [Effort: X days]

**Total Effort:** [Y weeks]

**Success Metrics:**
- [Metric 1]
- [Metric 2]

---

### Phase 3 - [Timeline: Z weeks]
**Goal:** [e.g., "Scale and optimize for growth"]

**Features:**
1. [Feature 6] - [Effort: X days]
2. [Feature 7] - [Effort: X days]

**Total Effort:** [Z weeks]

**Success Metrics:**
- [Metric 1]
- [Metric 2]

---

## Assumptions & Risks

### Assumptions
[Document assumptions made during prioritization]
1. [Assumption 1: e.g., "Users will adopt MVP despite missing advanced features"]
2. [Assumption 2: e.g., "External API will be available by MVP launch"]

### Risks
[Document risks that could affect priorities]
1. [Risk 1: e.g., "If competitor launches similar feature, Priority Feature X may need to be expedited"]
2. [Risk 2: e.g., "If technical spike reveals high complexity, Effort scores may need adjustment"]

---

## Re-Evaluation Criteria

**Triggers for Re-Prioritization:**
- User feedback after MVP launch
- Competitive landscape changes
- Technical feasibility discoveries
- Resource availability changes
- Business strategy shifts

**Review Schedule:**
- After MVP launch (30 days)
- End of each phase
- Quarterly (ongoing)

---

## Stakeholder Input

[Document stakeholder input on priorities]

### Product Owner Input
**Top Priorities:**
1. [Feature X] - [Rationale]
2. [Feature Y] - [Rationale]

### Engineering Lead Input
**Technical Considerations:**
1. [Feature A] - [Should be built early because it establishes foundation]
2. [Feature B] - [Should be deferred due to technical complexity]

### Design Lead Input
**UX Considerations:**
1. [Feature C] - [Critical for user experience]
2. [Feature D] - [Can be simplified or deferred]

### Sales/Customer Success Input
**Market Feedback:**
1. [Feature E] - [Customers are asking for this]
2. [Feature F] - [Not a common request]

---

## Decision Log

[Track decisions made about feature prioritization]

| Date | Decision | Rationale | Decider |
|------|----------|-----------|---------|
| YYYY-MM-DD | [Feature X moved from Phase 2 to MVP] | [Customer feedback showed high demand] | [Product Owner] |
| YYYY-MM-DD | [Feature Y deferred indefinitely] | [Low user interest validated by survey] | [Product Owner] |

---

## Appendix: Scoring Examples

### Example 1: User Registration

**Description:** Allow users to create account with email/password

**Value Score: 9/10**
- Business Value: 10/10 (Critical - can't have users without registration)
- User Value: 8/10 (Essential but not delightful)
- Average: 9/10

**Effort Score: 3/10**
- Development Time: 2-3 days
- Complexity: Low (standard pattern)
- Dependencies: None
- Risk: Low
- Total: 3/10

**Priority Score: 9/3 = 3.0 (High)**

**Category:** Quick Win → **Build for MVP**

---

### Example 2: Advanced Analytics Dashboard

**Description:** Comprehensive analytics with custom reports, exports, visualizations

**Value Score: 7/10**
- Business Value: 8/10 (Valuable for enterprise customers)
- User Value: 6/10 (Power users love it, casual users don't need it)
- Average: 7/10

**Effort Score: 9/10**
- Development Time: 4-6 weeks
- Complexity: High (data aggregation, visualization library, export formats)
- Dependencies: Requires stable data pipeline
- Risk: Medium (performance at scale)
- Total: 9/10

**Priority Score: 7/9 = 0.78 (Low)**

**Category:** Time Sink → **Defer to Phase 3** (or build simplified version as Fill-In)

---

## Template Usage Notes

1. **Gather Input:** Involve stakeholders (Product, Engineering, Design, Sales) in scoring
2. **Be Honest:** Resist temptation to inflate Value scores or underestimate Effort
3. **Iterate:** Scores will change as you learn more - update regularly
4. **Validate Assumptions:** Test value assumptions with user research when possible
5. **Balance Portfolio:** MVP should have mostly Quick Wins, plus 1-2 Strategic features if essential

---

**This prioritization matrix should be revisited:**
- After completing user research
- After MVP launch (validate assumptions)
- Quarterly (as business priorities evolve)

**Last Updated:** [Date] by [Name]
