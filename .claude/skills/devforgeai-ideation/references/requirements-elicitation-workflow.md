# Phase 2: Requirements Elicitation Workflow

Systematic questioning to extract functional and non-functional requirements through progressive discovery.

## Overview

Phase 2 uses structured questioning to uncover detailed requirements across functional capabilities, data needs, integrations, and non-functional requirements (NFRs). This phase employs **10-60 progressive questions** to transform vague ideas into specific, testable requirements.

**Duration:** 15-45 minutes
**Questions:** 10-60 progressive questions (adaptive based on complexity)
**Output:** Complete requirements list (functional, NFRs, data models, integrations)

---

## Questioning Strategy

### Progressive Narrowing Technique

**Level 1: Broad Discovery (3-5 questions)**
- What major feature categories are needed?
- What types of users interact with the system?
- What high-level workflows must be supported?

**Level 2: Feature-Specific Probing (5-20 questions)**
- For each feature category, drill into specifics
- What exactly should this feature do?
- What data does it need?
- What are edge cases?

**Level 3: Detail Extraction (10-40 questions)**
- How should validation work?
- What are the business rules?
- What happens when X fails?
- How do users recover from errors?

**Adaptive Questioning:**
- Simple projects (Tier 1): 10-15 questions total
- Standard projects (Tier 2): 20-30 questions
- Complex projects (Tier 3): 35-50 questions
- Enterprise projects (Tier 4): 50-60 questions

---

## Step 2.1: Functional Requirements Discovery

### Load Domain-Specific Patterns

```
Read(file_path=".claude/skills/devforgeai-ideation/references/requirements-elicitation-guide.md")
```

This reference provides comprehensive probing questions for:
- E-commerce platforms
- SaaS applications
- Fintech systems
- Healthcare platforms
- Content management systems
- Workflow/automation tools

### Capture Requirements as User Stories

Use the standard user story format:

```markdown
As a [user type],
I want to [action/capability],
So that [business value/benefit].
```

**Example questioning approach:**

```
Question: "What capabilities should users have in the system?"
Options: [Domain-specific feature list from requirements-elicitation-guide.md]
multiSelect: true
```

**For each selected capability:**
- Probe: "What specific actions should users be able to perform?"
- Probe: "What data do they need to see?"
- Probe: "What decisions do they need to make?"
- Probe: "What outcomes should they achieve?"

**Document:** 10-50 user stories depending on complexity

---

## Step 2.2: Data Requirements Discovery

### Identify Core Entities

```
Question: "What are the main data entities this system will manage?"
Options:
  - "Users/Accounts"
  - "Products/Inventory"
  - "Orders/Transactions"
  - "Content/Documents"
  - "Events/Activities"
  - "Files/Media"
  - "Messages/Notifications"
multiSelect: true
```

### Probe Entity Attributes and Relationships

**For each entity selected:**

```
Question: "What information needs to be stored about {entity}?"
# Free-text or multiSelect based on domain

Question: "How does {entity_1} relate to {entity_2}?"
Options:
  - "One {entity_1} has many {entity_2}"
  - "Many {entity_1} have many {entity_2}"
  - "One {entity_1} has one {entity_2}"
  - "No direct relationship"
```

**Document:**
- Entity name and purpose
- Attributes (fields/properties)
- Relationships (one-to-many, many-to-many, one-to-one)
- Constraints (required fields, validation rules, uniqueness)

---

## Step 2.3: Integration Requirements

```
Question: "Does this system need to integrate with external services?"
Options:
  - "Payment gateway (Stripe, PayPal, Square, etc.)"
  - "Email service (SendGrid, AWS SES, Mailgun, etc.)"
  - "Authentication provider (Auth0, OAuth, SAML, etc.)"
  - "Cloud storage (S3, Azure Blob, Google Cloud Storage, etc.)"
  - "Analytics (Google Analytics, Mixpanel, Amplitude, etc.)"
  - "SMS/notifications (Twilio, SNS, etc.)"
  - "Maps/geolocation (Google Maps, Mapbox, etc.)"
  - "No external integrations"
multiSelect: true
```

**For each integration:**

```
Question: "What data flows with {integration}?"
Options:
  - "One-way: Send data to service"
  - "One-way: Receive data from service"
  - "Two-way: Bidirectional synchronization"
```

**Document:**
- Integration service/API
- Data exchanged (format, protocol)
- Authentication method (API key, OAuth, etc.)
- Failure handling (retry logic, fallback)

---

## Step 2.4: Non-Functional Requirements (NFRs)

### Performance Requirements

```
Question: "What are the performance requirements?"
Options:
  - "High performance (<100ms response time, >10k concurrent users)"
  - "Standard performance (<500ms response time, 1k-10k users)"
  - "Moderate performance (<2s response time, <1k users)"
  - "Performance not critical (internal tool, low usage)"
```

**Quantify specifics:**
- API response time targets (p50, p95, p99)
- Page load time targets
- Concurrent user targets
- Database query performance
- Background job processing time

### Security Requirements

```
Question: "What security requirements apply?"
Options:
  - "Authentication required (user login)"
  - "Authorization/role-based access control (RBAC)"
  - "Data encryption (at rest and in transit)"
  - "Compliance (GDPR, HIPAA, SOC2, PCI-DSS)"
  - "Audit logging"
  - "Standard security practices"
multiSelect: true
```

**Document:**
- Authentication method (username/password, SSO, MFA)
- Authorization model (roles, permissions, resource-based)
- Encryption requirements (algorithms, key management)
- Compliance standards (specific regulations)
- Audit trail requirements (what to log, retention)

### Scalability Requirements

```
Question: "What scalability is needed?"
Options:
  - "Small scale (100s of users, single server)"
  - "Medium scale (1000s of users, horizontal scaling)"
  - "Large scale (10k+ concurrent users, multi-region)"
  - "Massive scale (millions of users, global CDN)"
```

**Quantify:**
- Expected user count (current and 2-year projection)
- Geographic distribution (single region, multi-region, global)
- Growth rate (users/month, data volume growth)
- Peak usage patterns (time of day, seasonality)

### Availability Requirements

```
Question: "What availability is required?"
Options:
  - "High availability (99.9% uptime, 24/7 monitoring)"
  - "Business hours only (99% uptime during work hours)"
  - "Best effort (no SLA)"
```

**Document:**
- Uptime SLA (99%, 99.9%, 99.99%)
- Acceptable downtime window (maintenance windows)
- Monitoring requirements (alerting, dashboards)
- Disaster recovery (RTO, RPO)

---

## Output from Phase 2

**Requirements List (structured):**

```markdown
# Functional Requirements

## User Stories
1. As a {persona}, I want to {action}, so that {benefit}
2. [... 10-50 user stories ...]

## Features Breakdown
- Feature Category 1
  - Capability 1
  - Capability 2
- Feature Category 2
  - Capability 3

# Data Requirements

## Entities
1. {Entity 1}: {Purpose}
   - Attributes: {field list}
   - Relationships: {related entities}
2. [... all entities ...]

# Integration Requirements

1. {Integration 1}: {Purpose}
   - Protocol: {REST, GraphQL, SOAP}
   - Authentication: {method}
   - Data flow: {one-way, two-way}

# Non-Functional Requirements

## Performance
- API response: <{X}ms (p95)
- Page load: <{Y} seconds
- Concurrent users: {N}+

## Security
- Authentication: {method}
- Authorization: {model}
- Encryption: {standards}
- Compliance: {regulations}

## Scalability
- Expected users: {current} → {2-year projection}
- Geographic: {regions}
- Growth: {rate}

## Availability
- SLA: {percentage} uptime
- Monitoring: {requirements}
- DR: RTO={X}, RPO={Y}
```

**Transition:** Proceed to Phase 3 (Complexity Assessment)

---

## Common Issues and Recovery

### Issue: Too Many Requirements (>100 user stories)

**Symptom:** Discovery uncovered massive scope

**Recovery:**
1. Group requirements into themes
2. Prioritize by business value
3. Propose phased implementation (MVP + Releases 2-3)
4. Use AskUserQuestion to validate prioritization
5. Document deferred requirements in "Future Scope"

### Issue: Vague NFRs ("Fast", "Scalable", "Secure")

**Symptom:** User cannot quantify performance/security/scalability

**Recovery:**
1. Provide examples of quantified NFRs
2. Ask for order of magnitude ("100s, 1000s, or 10,000s of users?")
3. Compare to known systems ("Like Gmail scale or like startup MVP scale?")
4. Document as ASSUMPTION with validation flag
5. Plan to refine during architecture phase

### Issue: Conflicting Requirements

**Symptom:** Requirement A contradicts Requirement B

**Recovery:**
```
AskUserQuestion(
    question: "Requirement '{A}' conflicts with '{B}'. How should we resolve?",
    header: "Conflict resolution",
    options: [
        "Keep {A}, remove {B}",
        "Keep {B}, remove {A}",
        "Modify both to be compatible",
        "Mark as constraint to resolve in architecture"
    ]
)
```

---

## References Used in Phase 2

**Primary:**
- **requirements-elicitation-guide.md** (659 lines) - Domain-specific question patterns, user story templates, NFR checklists

**On Error:**
- **error-handling.md** - Recovery procedures for incomplete answers

---

## Success Criteria

Phase 2 complete when:
- [ ] 10-50 user stories documented
- [ ] All NFRs quantified (no vague terms)
- [ ] Data entities identified with attributes
- [ ] Integrations specified with protocols
- [ ] All ambiguities resolved via AskUserQuestion
- [ ] Requirements complete enough for complexity scoring

**Token Budget:** ~8,000-25,000 tokens (10-60 AskUserQuestion interactions)

---

**Next Phase:** Phase 3 (Complexity Assessment)
