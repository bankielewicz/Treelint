---
id: EPIC-XXX
title: [Epic Title]
status: Planning
start_date: YYYY-MM-DD
target_date: YYYY-MM-DD
total_points: [Estimated Story Points]
created: YYYY-MM-DD
updated: YYYY-MM-DD
---

# Epic: [Epic Title]

## Business Goal

[Clear, concise statement of the business value this epic delivers. Answer: Why are we building this? What problem does it solve? What opportunity does it enable?]

## Success Metrics

[Quantifiable measures to determine if this epic is successful. All metrics should be measurable and time-bound.]

- **Metric 1:** [e.g., Increase user signups by 25% within 30 days of release]
- **Metric 2:** [e.g., Reduce customer support tickets by 40%]
- **Metric 3:** [e.g., Achieve 90% task completion rate for core user flow]

## Scope

### Overview
[High-level description of what this epic includes and excludes]

### Features

1. **[Feature 1 Name]**
   - Description: [Brief description]
   - User Value: [What value does this provide?]
   - Estimated Points: [X story points]

2. **[Feature 2 Name]**
   - Description: [Brief description]
   - User Value: [What value does this provide?]
   - Estimated Points: [X story points]

3. **[Feature 3 Name]**
   - Description: [Brief description]
   - User Value: [What value does this provide?]
   - Estimated Points: [X story points]

### Out of Scope

[Explicitly list what is NOT included in this epic to avoid scope creep]

- [Excluded item 1]
- [Excluded item 2]

## Target Sprints

**Estimated Duration:** [X sprints / Y weeks]

**Sprint Breakdown:**
- **Sprint 1:** [Feature 1] - [X story points]
- **Sprint 2:** [Feature 2] - [X story points]
- **Sprint 3:** [Feature 3] - [X story points]

## Dependencies

### External Dependencies
[What external factors must be in place before or during this epic?]

- **Dependency 1:** [e.g., API access from third-party service]
- **Dependency 2:** [e.g., Legal approval for data collection]

### Internal Dependencies
[What other epics or stories must complete before or during this epic?]

- **Dependency 1:** [e.g., EPIC-001 (User Authentication) must complete first]

### Blocking Issues
[Any known blockers that could delay this epic?]

- [Blocker 1: Description and mitigation plan]
- [Blocker 2: Description and mitigation plan]

## Stakeholders

- **Product Owner:** [Name / Role]
- **Tech Lead:** [Name / Role]
- **Design Lead:** [Name / Role]
- **Other Stakeholders:** [List key stakeholders who need to be informed or consulted]

## Requirements

### Functional Requirements

#### User Stories

**User Story 1:**
```
As a [user type],
I want to [action/capability],
So that [business value/benefit].
```

**Acceptance Criteria:**
- [ ] [Criterion 1]
- [ ] [Criterion 2]
- [ ] [Criterion 3]

**User Story 2:**
```
As a [user type],
I want to [action/capability],
So that [business value/benefit].
```

**Acceptance Criteria:**
- [ ] [Criterion 1]
- [ ] [Criterion 2]
- [ ] [Criterion 3]

[Continue for all major user stories]

### Non-Functional Requirements (NFRs)

#### Performance
- **Response Time:** [e.g., API endpoints must respond in < 500ms (p95)]
- **Throughput:** [e.g., Support 1,000 concurrent users]
- **Page Load Time:** [e.g., Initial page load < 2 seconds on 3G network]

#### Security
- **Authentication:** [e.g., Multi-factor authentication required for admin accounts]
- **Authorization:** [e.g., Role-based access control (RBAC) for feature access]
- **Data Encryption:** [e.g., All sensitive data encrypted at rest (AES-256) and in transit (TLS 1.3)]
- **Compliance:** [e.g., GDPR compliance for EU users, SOC 2 Type II]

#### Scalability
- **Horizontal Scaling:** [e.g., System must support horizontal scaling to handle traffic spikes]
- **Database:** [e.g., Support up to 10M records with sub-second query performance]
- **Caching:** [e.g., Implement caching layer (Redis) for frequently accessed data]

#### Availability
- **Uptime SLA:** [e.g., 99.9% uptime (< 8.76 hours downtime per year)]
- **Disaster Recovery:** [e.g., Daily backups with 1-hour RPO, 4-hour RTO]
- **Monitoring:** [e.g., Real-time monitoring with alerts for critical failures]

#### Usability
- **Accessibility:** [e.g., WCAG 2.1 AA compliance]
- **Mobile Support:** [e.g., Responsive design for mobile, tablet, desktop]
- **Browser Support:** [e.g., Support Chrome, Firefox, Safari, Edge (current and previous versions)]

### Data Requirements

#### Entities

**Entity 1: [Entity Name]**
- **Attributes:**
  - [attribute_name] (type, required/optional, constraints)
  - [attribute_name] (type, required/optional, constraints)
- **Relationships:**
  - [Relationship to other entities]
- **Indexes:** [Fields to index for performance]

**Entity 2: [Entity Name]**
- **Attributes:**
  - [attribute_name] (type, required/optional, constraints)
- **Relationships:**
  - [Relationship to other entities]
- **Indexes:** [Fields to index for performance]

[Continue for all major entities]

#### Data Volume
[Expected data volume and growth rate]
- **Initial:** [e.g., 10,000 records]
- **Year 1:** [e.g., 100,000 records]
- **Year 3:** [e.g., 1,000,000 records]

#### Data Retention
[How long should data be retained?]
- **Active Data:** [e.g., Indefinitely or until user deletes account]
- **Archived Data:** [e.g., Move to cold storage after 2 years]
- **Audit Logs:** [e.g., Retain for 7 years for compliance]

### Integration Requirements

#### External Systems

**Integration 1: [System Name]**
- **Purpose:** [Why are we integrating with this system?]
- **Type:** [REST API, SOAP, Message Queue, Webhook, etc.]
- **Authentication:** [API key, OAuth 2.0, SAML, etc.]
- **Data Flow:** [Direction: Inbound, Outbound, Bidirectional]
- **Frequency:** [Real-time, polling every X seconds, batch nightly]
- **Error Handling:** [Retry logic, dead letter queue, manual intervention]

**Integration 2: [System Name]**
- **Purpose:** [Why are we integrating with this system?]
- **Type:** [REST API, SOAP, Message Queue, Webhook, etc.]
- **Authentication:** [API key, OAuth 2.0, SAML, etc.]
- **Data Flow:** [Direction: Inbound, Outbound, Bidirectional]
- **Frequency:** [Real-time, polling every X seconds, batch nightly]
- **Error Handling:** [Retry logic, dead letter queue, manual intervention]

[Continue for all integrations]

## Architecture Considerations

### Complexity Tier
**Tier [N]: [Tier Name]**
- **Score:** [X/60 points from complexity assessment]
- **Rationale:** [Brief explanation of why this tier was selected]

### Recommended Architecture Pattern
[e.g., Monolithic, Modular Monolith, Microservices, Clean Architecture, Event-Driven]

**Justification:** [Why is this pattern appropriate for this epic?]

### Recommended Technology Stack

**Backend:**
- **Language/Framework:** [e.g., .NET 8.0, Node.js + NestJS, Python + FastAPI]
- **Database:** [e.g., PostgreSQL for primary data, Redis for caching]
- **Message Queue:** [e.g., RabbitMQ for async processing] (if applicable)

**Frontend:**
- **Framework:** [e.g., React + TypeScript]
- **State Management:** [e.g., Zustand, Redux Toolkit]
- **Styling:** [e.g., Tailwind CSS, Styled Components]

**Infrastructure:**
- **Hosting:** [e.g., Azure App Service, AWS Elastic Beanstalk, Kubernetes]
- **CI/CD:** [e.g., GitHub Actions, Azure DevOps]
- **Monitoring:** [e.g., Application Insights, Datadog]

### Technology Constraints

[Any locked technology choices from tech-stack.md or existing architecture]

- **Constraint 1:** [e.g., Must use Dapper for data access (locked in tech-stack.md)]
- **Constraint 2:** [e.g., Must integrate with existing Auth0 authentication]

## Risks & Constraints

### Technical Risks

**Risk 1: [Risk Name]**
- **Description:** [What could go wrong?]
- **Probability:** [Low / Medium / High]
- **Impact:** [Low / Medium / High / Critical]
- **Mitigation:** [How will we reduce or handle this risk?]

**Risk 2: [Risk Name]**
- **Description:** [What could go wrong?]
- **Probability:** [Low / Medium / High]
- **Impact:** [Low / Medium / High / Critical]
- **Mitigation:** [How will we reduce or handle this risk?]

### Business Risks

**Risk 1: [Risk Name]**
- **Description:** [What could go wrong?]
- **Probability:** [Low / Medium / High]
- **Impact:** [Low / Medium / High / Critical]
- **Mitigation:** [How will we reduce or handle this risk?]

### Constraints

**Constraint 1: [Constraint Name]**
- **Description:** [What is the constraint?]
- **Impact:** [How does this affect the epic?]
- **Mitigation:** [How will we work within this constraint?]

**Constraint 2: [Constraint Name]**
- **Description:** [What is the constraint?]
- **Impact:** [How does this affect the epic?]
- **Mitigation:** [How will we work within this constraint?]

## Assumptions

[Document any assumptions being made. These should be validated with stakeholders.]

1. [Assumption 1: e.g., Users have reliable internet connectivity]
2. [Assumption 2: e.g., Third-party API will maintain backward compatibility]
3. [Assumption 3: e.g., Legal team will approve data collection within 1 week]

## Next Steps

### Immediate Actions
1. **Architecture Skill:** Invoke `devforgeai-architecture` to create context files (tech-stack.md, source-tree.md, dependencies.md, etc.)
2. **Sprint Planning:** Use `devforgeai-orchestration` to create Sprint 1 plan and generate detailed stories
3. **Design:** Create wireframes/mockups for key user flows
4. **Technical Spike:** [If needed] Conduct technical spike to validate feasibility of [specific technology or approach]

### Pre-Development Checklist
- [ ] Architecture context files created and validated
- [ ] Sprint 1 stories created in `devforgeai/specs/Stories/`
- [ ] Designs approved by stakeholders
- [ ] External dependencies secured (API access, licenses, etc.)
- [ ] Development environment set up
- [ ] Team briefed on epic goals and architecture

### Development Workflow
Once ready, stories will progress through:
1. **Architecture** → devforgeai-architecture (if context changes needed)
2. **Ready for Dev** → devforgeai-development (TDD implementation)
3. **Dev Complete** → devforgeai-qa (quality validation)
4. **QA Approved** → devforgeai-release (deployment)

## Notes

[Any additional notes, clarifications, or open questions]

- [Note 1]
- [Note 2]
- [Open Question 1: Needs decision from stakeholders]

---

**Epic Status:**
- ⚪ **Planning** - Requirements being defined
- 🔵 **Architecture** - Context files being created
- 🟢 **In Progress** - Active development
- 🟡 **Blocked** - Waiting on dependency or decision
- ✅ **Completed** - All stories released to production
- ❌ **Cancelled** - Epic no longer being pursued

**Last Updated:** [Date] by [Name]
