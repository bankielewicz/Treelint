# User Interaction Patterns

AskUserQuestion templates and best practices for the ideation workflow.

## Overview

The ideation skill uses **10-60 strategic questions** across 6 phases to discover requirements without making assumptions. This reference documents proven question patterns, best practices, and common scenarios.

**Core Principle:** "Ask, Don't Assume" - Every ambiguity resolved through explicit user input.

---

## AskUserQuestion Best Practices

### Pattern 1: Quantifying Vague Requirements

**Problem:** User provides vague terms like "fast", "scalable", "secure"

**Template:**

```
# Instead of accepting "the system should be fast"
Question: "What does 'fast' mean for this project?"
Header: "Performance target"
Options:
  - "High performance (<100ms API response, sub-second page loads)"
  - "Standard performance (<500ms API response, <3s page loads)"
  - "Moderate performance (functional but not optimized)"
multiSelect: false
```

**Examples by domain:**

**Performance:**
```
Question: "What response time is acceptable?"
Options:
  - "Real-time (<100ms, high-frequency trading)"
  - "Interactive (<500ms, e-commerce checkout)"
  - "Standard (<2s, content browsing)"
  - "Batch processing (seconds to minutes acceptable)"
```

**Scalability:**
```
Question: "What scale do you expect?"
Options:
  - "Small (<1k users, single server)"
  - "Medium (1k-10k users, load-balanced)"
  - "Large (10k-100k users, distributed)"
  - "Massive (100k+ users, global CDN)"
```

**Security:**
```
Question: "What security level is required?"
Options:
  - "Public data (minimal security)"
  - "User data (authentication + encryption)"
  - "Sensitive data (RBAC + audit logging + compliance)"
  - "Highly sensitive (multi-factor + zero-trust architecture)"
```

---

### Pattern 2: MVP Scope Negotiation

**Problem:** User wants comprehensive solution but has timeline/budget constraints

**Template:**

```
Question: "Full feature set would take ~{estimate}. What's minimum for initial release?"
Header: "MVP definition"
Options:
  - "Core workflow only ({reduced_estimate})"
  - "Core + 2 secondary features ({mid_estimate})"
  - "Full feature set required ({full_estimate})"
  - "Help me prioritize features"
multiSelect: false
```

**Follow-up if "Help me prioritize":**

```
Question: "Which features deliver the most user value?"
Options: {List all features with estimated effort}
  - "Feature A: {description} (Effort: {Small|Medium|Large})"
  - "Feature B: {description} (Effort: {Small|Medium|Large})"
  ...
multiSelect: true
```

**Apply 80/20 rule:**
- Identify 20% of features that deliver 80% of value
- Propose as MVP
- Defer remaining features to Release 2

---

### Pattern 3: Technology Preference Discovery

**Problem:** Need to understand team expertise for technology recommendations

**Template:**

```
Question: "Does your team have experience with any of these technologies?"
Header: "Team expertise"
Options:
  - "React (frontend framework)"
  - "Node.js (backend)"
  - "Python (backend)"
  - "C#/.NET (backend)"
  - "PostgreSQL (database)"
  - "MongoDB (database)"
  - "AWS (cloud platform)"
  - "Azure (cloud platform)"
  - "None of the above"
multiSelect: true
```

**Use responses to:**
- Weight technology recommendations toward team expertise
- Identify learning curve requirements
- Estimate ramp-up time for new technologies
- Flag skill gaps as risks

---

### Pattern 4: Compliance Uncertainty Resolution

**Problem:** User unsure what compliance standards apply

**Template:**

```
Question: "What type of data will this system handle?"
Header: "Data sensitivity"
Options:
  - "Health information (HIPAA required)"
  - "Payment data (PCI-DSS required)"
  - "Personal data - EU users (GDPR required)"
  - "Personal data - California users (CCPA required)"
  - "Financial data (SOC2, financial regulations)"
  - "Public/non-sensitive data (standard security)"
multiSelect: true
```

**Follow-up for each regulation:**

```
If HIPAA selected:
    Question: "What HIPAA requirements apply?"
    Options:
      - "Store PHI (Protected Health Information)"
      - "Transmit PHI"
      - "Both store and transmit PHI"
      - "Just general healthcare data (not PHI)"

If PCI-DSS selected:
    Question: "How will payment data be handled?"
    Options:
      - "Store credit card numbers (requires full PCI-DSS compliance)"
      - "Tokenize via gateway (reduced compliance scope)"
      - "Never touch card data (payment iframe/redirect)"
```

---

### Pattern 5: Feature Priority Trade-offs

**Problem:** User wants all features but has limited resources

**Template:**

```
Question: "All features are valuable. If forced to choose, which are must-have for launch?"
Header: "Must-have features"
Options: {List all features}
  - "Feature A: {description}"
  - "Feature B: {description}"
  - "Feature C: {description}"
multiSelect: true
```

**Categorize responses:**
- **Must-have:** Selected features (P0)
- **Should-have:** Not selected but important (P1)
- **Could-have:** Nice to have (P2)
- **Won't-have:** Explicitly deferred (Future scope)

---

### Pattern 6: Integration Complexity Probing

**Problem:** User says "integrate with third-party API" without details

**Template:**

```
Question: "What data flows with {integration}?"
Header: "Integration pattern"
Options:
  - "One-way: Send data to service (webhook, API call)"
  - "One-way: Receive data from service (polling, webhook)"
  - "Two-way: Bidirectional synchronization (real-time updates)"
multiSelect: false
```

**Follow-up:**

```
Question: "What happens if {integration} is unavailable?"
Header: "Failure handling"
Options:
  - "Critical - Application cannot function (must retry/queue)"
  - "Important - Application degraded (fallback behavior)"
  - "Optional - Application works without it (skip gracefully)"
```

---

### Pattern 7: Brownfield Constraint Discovery

**Problem:** Adding features to existing system with unknown constraints

**Template:**

```
Question: "Are there any constraints from the existing system we should know about?"
Header: "Existing constraints"
Options:
  - "Must use current tech stack (no new languages/frameworks)"
  - "Must maintain API compatibility (no breaking changes)"
  - "Must work with existing database schema"
  - "Must support existing user base (migration complexity)"
  - "No major constraints - greenfield within existing codebase"
multiSelect: true
```

**For each constraint:**
```
Question: "Regarding '{constraint}', what are the specifics?"
# Free-text response to understand details
```

---

### Pattern 8: Risk Probability Assessment

**Problem:** User identifies risk but can't estimate probability

**Template:**

```
Question: "How likely is '{risk}' to occur?"
Header: "Risk probability"
Options:
  - "Low (<30% chance, hasn't happened before)"
  - "Medium (30-70% chance, happens occasionally)"
  - "High (>70% chance, likely will happen)"
  - "Uncertain - need to research"
```

**If "Uncertain":**
- Document as assumption
- Flag for validation during architecture phase
- Recommend researching during sprint planning

---

## Question Flow Strategy

### Progressive Narrowing

**Level 1: Broad Discovery (Phase 1)**
- 3-5 questions
- Open-ended or category selection
- Establishes project type, problem, users, goals

**Level 2: Feature Discovery (Phase 2)**
- 10-30 questions
- Domain-specific feature probing
- Functional requirements, data model, integrations

**Level 3: Detail Extraction (Phase 2-5)**
- 10-40 questions
- Validation rules, business rules, edge cases
- NFRs, constraints, risks, feasibility

**Level 4: Validation (Phases 3-6)**
- 5-10 questions
- Confirm complexity score, validate epics, resolve conflicts
- Determine next action

**Total:** 10-60 questions (adaptive based on complexity)

---

## Multi-Select Usage

**Use multiSelect: true when:**
- Multiple valid answers (e.g., "Select all user roles")
- Options not mutually exclusive
- Combination of features needed

**Use multiSelect: false when:**
- Single choice required (e.g., "Performance tier")
- Mutually exclusive options (e.g., "Greenfield OR Brownfield")
- Priority selection (e.g., "Most important feature")

---

## Question Timing

### When to Ask Questions

**Phase 1:** 5-10 questions (project type, problem, users, goals, scope)
**Phase 2:** 10-60 questions (features, data, integrations, NFRs) - **Most questions here**
**Phase 3:** 1-3 questions (validate complexity tier)
**Phase 4:** 3-8 questions (prioritize epics, validate feature breakdown)
**Phase 5:** 2-5 questions (constraints, timeline, risk assessment)
**Phase 6:** 1-2 questions (requirements spec, next action)

### Avoid Question Fatigue

**If question count >50:**
- Batch related questions (ask 3-5 at once with multiSelect)
- Use defaults for low-priority decisions
- Skip optional questions if user indicates urgency
- Provide "Help me decide" option to use reasonable defaults

**Example batch:**
```
Question: "Select all that apply to this project:"
Options:
  - "Authentication required"
  - "Payment processing needed"
  - "Email notifications required"
  - "File upload/storage needed"
  - "Search functionality needed"
  - "Analytics/reporting needed"
multiSelect: true
```

---

## Error Recovery Patterns

### Incomplete User Answers

**Symptom:** User says "I don't know" or provides vague answer

**Recovery:**

```
# Provide examples of good answers
Explain: "Instead of 'fast', specify: 'API responses under 500ms for 95th percentile'"

# Offer to defer decision
Question: "Would you like to defer this decision to the architecture phase?"
Options:
  - "Yes - document as assumption, decide later"
  - "No - let me provide specifics now"

# If defer:
Document as ASSUMPTION with validation flag in requirements spec
```

### Conflicting Answers

**Symptom:** User selects contradictory options

**Recovery:**

```
Question: "You selected both '{option_a}' and '{option_b}' which may conflict. How to resolve?"
Header: "Conflict resolution"
Options:
  - "Keep {option_a}, remove {option_b}"
  - "Keep {option_b}, remove {option_a}"
  - "Both needed - explain how they work together"
```

---

## References Used

**Loaded during questioning:**
- **requirements-elicitation-guide.md** - Domain-specific question libraries
- **domain-specific-patterns.md** - Feature option lists per domain
- **complexity-assessment-matrix.md** - Complexity validation questions

---

## Success Criteria

User interaction successful when:
- [ ] All ambiguities resolved through questions
- [ ] User understands trade-offs presented
- [ ] Decisions documented in requirements
- [ ] No assumptions without validation flags
- [ ] Question fatigue avoided (<60 questions total)
- [ ] User feels heard and involved

**Token Budget:** ~15,000-40,000 tokens (10-60 AskUserQuestion interactions across all phases)

---

**This reference supports all 6 phases of ideation workflow with proven question patterns.**
