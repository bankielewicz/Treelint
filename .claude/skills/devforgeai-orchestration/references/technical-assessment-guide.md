# Technical Assessment Guide

Guide for evaluating technical complexity, architecture impact, and risk assessment of epics and features.

---

## Overview

Technical assessment scores complexity on a 0-10 scale and evaluates architecture impact, technology requirements, integration complexity, and risk factors. This guide helps the architect-reviewer subagent and technical leads systematically assess epics before implementation.

**Key Principle:** Assessment validates alignment with existing architecture, identifies required ADRs, and flags risks early.

---

## Complexity Scoring Rubric (0-10 Scale)

### Score 0-2: Trivial Complexity

**Characteristics:**
- No new technologies required
- Uses existing frameworks/libraries only
- No architecture changes needed
- No external integrations
- Minimal data modeling (single entity type)
- Standard testing approaches
- <1 week estimated effort

**Examples:**
- Add new field to existing form
- Display new view with existing components
- Add validation rule to existing form
- Create basic status page

**Assessment Output:**
```
Overall Complexity: 1/10
Technology Stack: No new technologies
Architecture Impact: None
Key Risks: None
Prerequisites: None
```

---

### Score 3-4: Low Complexity

**Characteristics:**
- Existing technology stack only (no new libraries)
- Minor architecture adjustments
- Simple external integrations (read-only REST APIs)
- Basic data modeling (1-2 entities)
- Standard test coverage (unit + integration)
- 1-2 week estimated effort

**Examples:**
- CRUD operations on single entity (Users, Products)
- Read-only reports/dashboards (2-3 metrics)
- Form submission to existing system
- Basic search functionality

**Assessment Output:**
```
Overall Complexity: 3/10
Technology Stack: Existing stack only
Architecture Impact: Minor (new endpoints, no layer changes)
Integration: Simple REST API read (no polling/webhooks)
Key Risks: None
Prerequisites: Existing database schema
Estimated Effort: 1-2 weeks
```

---

### Score 5-6: Moderate Complexity

**Characteristics:**
- 1-2 new technologies (within approved tech-stack.md)
- New service/layer may be required
- Moderate external integrations (OAuth, webhooks, real-time updates)
- Complex data modeling (3-5 entities with relationships)
- Integration tests required
- Custom business logic (2-4 rules)
- 2-4 week estimated effort

**Examples:**
- User authentication with OAuth 2.0 provider
- Multi-entity transaction processing
- Real-time notifications system
- Advanced search with filters and sorting
- Multi-step workflow with business rules

**Assessment Output:**
```
Overall Complexity: 5/10
Technology Stack: 1 new library (oauth2-client, within approved list)
Architecture Impact: New service layer, extends existing REST API
Integration: OAuth 2.0 provider + webhook for notifications
Data Model: 5 entities (User, OAuth Provider, Tokens, Audit Log, Session)
Key Risks:
  - OAuth provider SLA risk (Medium)
  - Token expiration edge cases (Medium)
Business Logic: 3 rules (token refresh, session timeout, concurrent login limits)
Prerequisites: User entity already exists
Estimated Effort: 2-4 weeks
```

---

### Score 7-8: High Complexity

**Characteristics:**
- 3+ new technologies (may require ADR review)
- Significant architecture changes (new services, layer restructuring)
- Complex integrations (multiple external systems, data transformation)
- Advanced data modeling (6+ entities, complex relationships)
- E2E + performance + security testing required
- Complex business logic (5+ rules, branching)
- Advanced deployment requirements
- 3-6 week estimated effort

**Examples:**
- Real-time analytics dashboard (event collection + processing + visualization)
- Approval workflow with multi-level routing and escalation
- Payment processing with fraud detection
- Microservice decomposition (major architecture shift)
- Advanced permission system (RBAC + ABAC)

**Assessment Output:**
```
Overall Complexity: 7/10
Technology Stack:
  - Event streaming library (Kafka) - REQUIRES ADR
  - Analytics processing (Apache Spark) - REQUIRES ADR
  - Visualization library (D3.js) - approved
Architecture Impact:
  - New event collection service layer
  - New analytics processing service
  - Database schema changes (event tables, aggregates)
Integration:
  - Multiple event sources (web, mobile, backend)
  - Real-time data pipeline (Kafka → Spark → Dashboard)
  - Analytics database (separate from OLTP)
Data Model:
  - 8+ entities (Events, Aggregates, Metrics, Dashboards, Users, etc.)
  - Complex relationships (events → aggregates → metrics)
Key Risks:
  - Data pipeline latency (High impact if >1 hour)
  - Kafka cluster management (Medium probability)
  - Spark job failures and recovery (Medium probability)
Business Logic:
  - Event validation and deduplication (complex)
  - Real-time aggregation windows (complex)
  - Metric calculation formulas (5+ different metrics)
Prerequisites:
  - Event schema design (Phase 0)
  - Analytics database setup (Phase 0)
  - Kafka cluster provisioned (Phase 0)
Estimated Effort: 3-6 weeks
```

---

### Score 9-10: Very High Complexity

**Characteristics:**
- New tech stack or major framework adoption (requires ADR + governance)
- Major architecture redesign (microservices, event-driven, etc.)
- Complex distributed systems (data consistency, failure handling)
- 5+ external system integrations with data transformation
- Data migration from legacy system required
- Comprehensive test automation needed (unit + integration + E2E + chaos testing)
- Complex security/compliance implications
- High organizational impact
- 6+ week estimated effort

**Examples:**
- Migrate monolith to microservices
- Real-time distributed ledger system
- Complex data warehousing solution
- Multi-region, multi-cloud deployment
- AI/ML pipeline with model training and inference

**Assessment Output:**
```
Overall Complexity: 9/10
Technology Stack:
  - Kubernetes (orchestration) - NEW STACK
  - gRPC (service communication) - NEW STACK
  - Service mesh (Istio) - NEW STACK
  - REQUIRES ADR + FULL ARCHITECTURE REVIEW

Architecture Impact:
  - Complete architecture redesign
  - Monolith decomposition into 8+ microservices
  - New service discovery layer
  - New inter-service communication layer
  - Database partitioning strategy
  - Distributed transaction coordination

Integration:
  - 6+ external systems
  - Complex data transformation pipelines
  - Event-driven communication (async messaging)
  - Real-time synchronization requirements

Data Model:
  - Distributed database design
  - Event sourcing patterns
  - Data consistency strategy (eventual vs strong)
  - Schema versioning across services

Key Risks:
  - Distributed system complexity (High)
  - Data consistency across services (High)
  - Operational complexity (High)
  - Team knowledge gaps (Medium-High)
  - Vendor lock-in (Medium)

Security/Compliance:
  - Service-to-service authentication (mTLS)
  - Data encryption in transit and at rest
  - API gateway security
  - Audit trail across services

Prerequisites:
  - Architecture review and ADR approval
  - Team training on microservices patterns
  - Infrastructure provisioning (Kubernetes cluster)
  - Observability platform setup (logging, metrics, tracing)
  - Rollback and disaster recovery procedures

Estimated Effort: 6-12 weeks
```

---

## Assessment Dimensions

### Dimension 1: Technology Stack Impact

**Questions to Ask:**
1. Are new technologies required? (Y/N)
2. How many new technologies? (0, 1-2, 3+)
3. Are proposed technologies in approved tech-stack.md? (Y/N/Needs ADR)
4. Learning curve for team? (None, Low, Medium, High)
5. Long-term maintenance burden? (Low, Medium, High)
6. Vendor lock-in risk? (None, Medium, High)

**Scoring Adjustment:**
- All technologies approved in tech-stack.md: No complexity increase
- 1-2 new technologies: +1 complexity point (needs ADR)
- 3+ new technologies: +2-3 complexity points (major ADR required, architecture review)

**Framework Integration - MUST Check tech-stack.md:**

```markdown
## Technology Validation

### Step 1: Read tech-stack.md
Read(file_path="devforgeai/specs/context/tech-stack.md")

### Step 2: For each proposed technology
IF technology in tech-stack.md:
  ✅ APPROVED (no action)

IF technology NOT in tech-stack.md:
  ⚠️ REQUIRES ADR
  Action: Flag in assessment, require ADR creation before implementation

IF multiple new technologies (3+):
  🛑 MAJOR DECISION
  Action: Require full architecture review + governance approval
```

**Example Assessment:**

```
Proposed: "Use Redis for real-time caching"

Check: tech-stack.md contains?
- Redis: YES (approved in "Caching" section)
Result: ✅ No ADR needed, technology already approved

---

Proposed: "Use Elasticsearch for full-text search"

Check: tech-stack.md contains?
- Elasticsearch: NO (not mentioned)
Result: ⚠️ Requires ADR documenting decision to adopt Elasticsearch
  - Alternative considered: Database full-text search (MySQL FULLTEXT)
  - Why Elasticsearch: Superior scaling, better relevance tuning
  - Maintenance burden: Cluster management overhead
```

---

### Dimension 2: Architecture Changes

**Questions to Ask:**
1. New services/layers required? (Y/N)
2. Changes to existing architecture? (Y/N)
3. Impact on architecture-constraints.md rules? (None/Minor/Major)
4. Violates layer boundaries? (Y/N)
5. Creates circular dependencies? (Y/N)

**Scoring Adjustment:**
- No architecture changes: 0 points
- Minor changes (new endpoints, slight layer extension): +1 point
- New service layer: +2 points
- Major architecture restructuring: +3-4 points

**Framework Integration - MUST Check architecture-constraints.md:**

```markdown
## Architecture Validation

### Step 1: Read architecture-constraints.md
Read(file_path="devforgeai/specs/context/architecture-constraints.md")

### Step 2: Validate proposed architecture against constraints

FOR each proposed layer/service:
  IF violates layer boundaries:
    🛑 BLOCK - Cannot proceed
    Example violation: "Domain layer directly imports Infrastructure layer"
    Action: Require redesign

  IF creates circular dependencies:
    🛑 BLOCK - Cannot proceed
    Example: "Service A imports Service B, Service B imports Service A"
    Action: Require architectural redesign

  IF extends existing boundaries:
    ⚠️ FLAG - May need ADR if significant change
    Action: Document in assessment

  IF respects all constraints:
    ✅ APPROVED - Continue
```

**Example Assessment:**

```
Proposed: "Add new caching layer between API and Database"

Architecture Check:
- Current: Presentation → Application → Domain → Infrastructure (DB)
- Proposed: Presentation → Application → Caching → Domain → Infrastructure
- Violates boundaries? NO (caching is infrastructure concern)
- Creates circular deps? NO (one-direction dependency)
Result: ✅ APPROVED - Minor architecture change (+1 complexity)

---

Proposed: "Domain model directly queries database"

Architecture Check:
- Violates: Domain → Infrastructure (forbidden in constraints)
- Creates architectural violation: Business logic couples to data storage
Result: 🛑 BLOCKED - Redesign required
  Correct approach:
  - Domain defines interfaces (ports)
  - Infrastructure implements repositories (adapters)
  - Inject repository into domain
```

---

### Dimension 3: Integration Complexity

**Questions to Ask:**
1. External systems to integrate? (Y/N)
2. Number of integrations? (1, 2-3, 4+)
3. Integration type? (REST API, SOAP, gRPC, webhooks, real-time, etc.)
4. Data transformation required? (Y/N)
5. Error handling complexity? (Standard, Custom, Very complex)
6. Rate limiting/throttling needed? (Y/N)

**Scoring Adjustment:**
- No integrations: 0 points
- Simple REST API integration (read-only): +1 point
- OAuth/webhook integration: +1-2 points
- Multiple integrations (2-3): +2 points
- Complex integrations (4+, with data transformation): +3-4 points
- Real-time integrations (event streams, WebSockets): +2-3 points

**Example Assessment:**

```
Simple Integration:
"Payment processing with Stripe API"
- 1 external integration (Stripe)
- REST API (documented, standard)
- Standard error handling (card declined, insufficient funds)
- No data transformation (direct field mapping)
Complexity Impact: +1 point

---

Complex Integration:
"Real-time sync with 5 external vendors"
- 5 external integrations
- Mixed protocols (REST for 2, webhooks for 2, gRPC for 1)
- Complex data transformation (vendor schema → internal schema)
- Error handling: retry logic, exponential backoff, circuit breakers
- Real-time requirements: <1 second latency
- Rate limiting: Each vendor has different limits
Complexity Impact: +4 points (very complex)
```

---

### Dimension 4: Data Modeling Complexity

**Questions to Ask:**
1. New entity types required? (0, 1-2, 3-5, 6+)
2. Entity relationships? (Simple 1:1/1:M, Complex M:M with attributes, Graph/Network)
3. Schema changes to existing data? (None, Minor, Major, Migration required)
4. Data consistency requirements? (Eventual, Strong, ACID)
5. Data volume and performance? (< 1GB, 1-100GB, 100GB+)

**Scoring Adjustment:**
- Single entity type, no relationships: 0 points
- 2-3 entities, simple relationships: +1 point
- 4-5 entities, complex relationships: +2 points
- 6+ entities, advanced relationships: +2-3 points
- Major schema changes to existing data: +1-2 points (migration risk)
- Data consistency requirements beyond ACID: +1 point (distributed transactions, etc.)

**Example Assessment:**

```
Simple Data Model:
"Add new user preferences table"
- 1 new entity (UserPreference)
- Simple relationship (1:1 with User)
- No changes to existing schema
- Standard ACID compliance
Complexity Impact: +1 point

---

Complex Data Model:
"Event-driven system with aggregates"
- 8 entities: Event, Aggregate, EventStream, Snapshot, Journal, etc.
- Complex relationships: M:M with temporal aspects
- Event sourcing pattern (append-only log)
- CQRS pattern (command side ≠ read side)
- Eventual consistency (not ACID)
- Distributed transactions across services
Complexity Impact: +3 points (very complex)
```

---

### Dimension 5: Testing Complexity

**Questions to Ask:**
1. New test types needed? (Unit, Integration, E2E, Performance, Security, Chaos)
2. Mock/stub complexity? (Simple mocks, Complex async mocks, Distributed mocks)
3. Test infrastructure required? (None, Minimal, Significant, Complex)
4. Coverage requirements? (Standard 80%, High 95%, Very High with edge cases)

**Scoring Adjustment:**
- Standard unit + integration tests: 0 points
- E2E tests required: +1 point
- Performance/load testing: +1 point
- Security testing: +1 point
- Chaos engineering tests: +2 points
- Complex mock infrastructure: +1 point

---

### Dimension 6: Security Considerations

**Questions to Ask:**
1. Handles authentication/authorization? (Y/N)
2. Handles sensitive data? (PII, financial, health, other)
3. Compliance requirements? (GDPR, PCI DSS, HIPAA, SOC 2, other)
4. Cryptography required? (Y/N, what type)
5. OWASP Top 10 implications? (Which vulnerabilities are relevant)

**Scoring Adjustment:**
- No security concerns: 0 points
- Authentication/authorization additions: +1 point
- Sensitive data handling: +2 points
- Compliance requirements: +2-3 points
- Cryptographic implementation: +2-3 points
- High OWASP impact: +1-2 points

**Framework Integration - MUST Check anti-patterns.md:**

```markdown
## Security & Anti-Pattern Validation

### Step 1: Read anti-patterns.md
Read(file_path="devforgeai/specs/context/anti-patterns.md")

### Step 2: Check for forbidden security patterns

FORBIDDEN Pattern 1: Hardcoded Secrets
- Check: Code contains hardcoded API keys, passwords, credentials? (Y/N)
- If YES: 🛑 BLOCKED - Must use environment variables
- If NO: ✅ APPROVED

FORBIDDEN Pattern 2: SQL Concatenation (Injection Risk)
- Check: SQL queries built via string concatenation? (Y/N)
- If YES: 🛑 BLOCKED - Must use parameterized queries
- If NO: ✅ APPROVED

FORBIDDEN Pattern 3: Weak Cryptography
- Check: Uses MD5, SHA1, DES, or non-standard crypto? (Y/N)
- If YES: 🛑 BLOCKED - Must use SHA256+, AES-256+
- If NO: ✅ APPROVED

FORBIDDEN Pattern 4: Direct Instantiation (Auth)
- Check: Services directly instantiate auth/security components? (Y/N)
- If YES: 🛑 BLOCKED - Must use Dependency Injection
- If NO: ✅ APPROVED
```

---

### Dimension 7: Performance Requirements

**Questions to Ask:**
1. Performance targets specified? (Response time, throughput, scalability)
2. Measurable targets or vague ("fast", "scalable")? (Specific/Vague)
3. Current system meeting targets? (Y/N/Unknown)
4. Caching strategy required? (Y/N)
5. Monitoring/alerting needed? (Y/N)

**Scoring Adjustment:**
- No performance concerns: 0 points
- Performance targets specified (measurable): +1 point (not higher, good planning)
- Vague performance targets ("fast", "scalable"): +1 point (risk: unclear when done)
- Optimization required to existing system: +2 points (code changes, refactoring)
- Scalability to 10x current load: +2 points
- Real-time performance (<100ms): +2-3 points

---

## Risk Identification Matrix

### Risk Categories

**A. Technology Risks**

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|-----------|
| Unproven technology | Low-Medium | High | Proof-of-concept phase, fallback plan |
| Vendor lock-in | Medium | High | Multi-vendor evaluation, contract review |
| Deprecated libraries | Low | Medium | Dependency monitoring, upgrade plan |
| Learning curve | Medium | Medium | Team training, pair programming |
| License conflicts | Low | High | Legal review, approved list enforcement |

**Mitigation Example:**
```
Risk: "Redis not in approved tech-stack.md"
Probability: Medium (depends on tech decisions)
Impact: High (blocks implementation if unapproved)
Mitigation:
  1. Create ADR documenting Redis decision
  2. Get architecture review approval
  3. Add to tech-stack.md before implementation
  4. Fallback: Use database caching if Redis not approved
```

---

**B. Integration Risks**

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|-----------|
| Third-party API instability | Medium | High | SLA monitoring, circuit breakers, fallback |
| Rate limiting | Medium | Medium | Request queuing, backoff strategy |
| Data format changes | Low | Medium | Version pinning, deprecation monitoring |
| Authentication failures | Low-Medium | High | Token refresh retry, manual recovery |
| Webhook delivery failures | Medium | Medium | Retry with exponential backoff, DLQ |

**Mitigation Example:**
```
Risk: "Stripe API rate limiting"
Probability: Medium (high-traffic scenarios)
Impact: Medium (payment delays, user frustration)
Mitigation:
  1. Implement request queuing with priority
  2. Exponential backoff retry (max 3 attempts)
  3. Monitor API limits in production
  4. Alert when approaching limits
  5. Pre-staging: Load test against Stripe sandbox
```

---

**C. Data Risks**

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|-----------|
| Data loss during migration | Low | Critical | Backup strategy, dry-run before cutover |
| Data inconsistency | Low-Medium | High | Consistency checks, reconciliation process |
| Data privacy violations | Low | Critical | Data classification, encryption, audit trail |
| Storage capacity exceeded | Medium | High | Capacity planning, data archival strategy |
| Performance degradation | Medium | High | Indexing strategy, query optimization |

---

**D. Team & Organizational Risks**

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|-----------|
| Skill gaps | Medium | High | Training program, pair programming, hiring |
| Knowledge silos | Medium-High | High | Documentation, rotating assignments |
| Capacity constraints | Low-Medium | Medium | Resource planning, prioritization |
| Dependencies on key people | Medium | High | Cross-training, documentation |
| Scope creep | High | High | Clear requirements, change management |

---

## Context File Validation Process

### Process: Pre-Assessment Context Check

**IF context files exist in `devforgeai/specs/context/`:**

```markdown
1. Read all 6 context files:
   - tech-stack.md
   - architecture-constraints.md
   - dependencies.md
   - coding-standards.md
   - source-tree.md
   - anti-patterns.md

2. For each proposed technology/architecture:
   - VALIDATE against tech-stack.md (approved or needs ADR)
   - VALIDATE against architecture-constraints.md (no layer violations)
   - VALIDATE against dependencies.md (approved integrations)
   - VALIDATE against anti-patterns.md (no forbidden patterns)

3. If any validation FAILS:
   - FLAG in assessment as "REQUIRES ADR" or "BLOCKED"
   - Do NOT proceed with assumption of approval
   - Require explicit approval before implementation

4. Assessment includes:
   - ✅ Validated against all context files
   - ⚠️ Flagged for ADR (technology not approved)
   - 🛑 Blocked (violates constraints)
```

**IF context files DON'T exist:**

```markdown
1. Note in assessment:
   "Operating in greenfield mode - no context files to validate"

2. Recommendations:
   - Create context files before implementation
   - Document all technology decisions in ADRs
   - Establish architecture constraints early

3. Assessment based on:
   - Industry best practices
   - Technology maturity
   - Team capabilities
```

---

## Output Format for Architect-Reviewer Subagent

When assessing technical complexity, provide structured output:

```markdown
## Technical Assessment: [Epic/Feature Name]

### Overall Complexity Score

**Overall**: [Score]/10 ([Level]: Trivial/Low/Moderate/High/Very High)

**Justification**: [1-2 sentences explaining score]

### Assessment Breakdown

#### 1. Technology Stack

**Proposed Technologies:**
- {Tech 1}: {Purpose} [APPROVED / ⚠️ REQUIRES ADR / 🛑 NOT APPROVED]
- {Tech 2}: {Purpose} [Status]

**New Technologies Count**: [0 / 1-2 / 3+ ]

**Learning Curve**: [None / Low / Medium / High]

**Long-term Burden**: [Low / Medium / High]

[IF ADR REQUIRED]
**ADRs Needed:**
- ADR-NNN: Justify adoption of {Technology}
  - Alternative considered: {alternative}
  - Rationale: {decision reasoning}

---

#### 2. Architecture Impact

**Proposed Changes:**
[Description of architecture changes]

**Layer Impact:**
- [Layer 1]: [Changes made]
- [Layer 2]: [Changes made]

**Validation Against Constraints:**
- [Constraint 1]: ✅ COMPLIANT / ⚠️ NEEDS REVIEW / 🛑 VIOLATION
- [Constraint 2]: [Status]

[IF VIOLATIONS FOUND]
**🛑 Architecture Violations Detected:**
- Violation 1: [Description]
  Action: [Resolution steps]
- Violation 2: [Description]
  Action: [Resolution steps]

---

#### 3. Integration Complexity

**External Integrations:**
- {System 1}: [Type, complexity, error handling]
- {System 2}: [Type, complexity, error handling]

**Integration Type**: [Simple REST / OAuth / Webhooks / Real-time / Complex]

**Data Transformation**: [None / Simple / Complex]

**Complexity Impact**: +[X] points

---

#### 4. Data Model

**New Entities**: [0 / 1-2 / 3-5 / 6+]

**Entity List**:
- {Entity 1}: {Purpose} [{Relationships}]
- {Entity 2}: {Purpose} [{Relationships}]

**Schema Changes**: [None / Minor / Major / Migration required]

**Consistency Model**: [ACID / Eventual / Other]

**Complexity Impact**: +[X] points

---

#### 5. Testing Requirements

**New Test Types**:
- [ ] Unit tests
- [ ] Integration tests
- [ ] E2E tests
- [ ] Performance tests
- [ ] Security tests
- [ ] Chaos tests

**Mock Infrastructure**: [Simple / Complex / Significant]

**Complexity Impact**: +[X] points

---

#### 6. Security & Compliance

**Security Concerns**:
- [Concern 1]: [Description] [Mitigation]
- [Concern 2]: [Description] [Mitigation]

**Sensitive Data**: [None / PII / Financial / Health / Other]

**Compliance**: [GDPR / PCI DSS / HIPAA / SOC 2 / None]

**Cryptography Required**: [Y/N] [Type if yes]

**Validation Against anti-patterns.md**:
- No hardcoded secrets: ✅ / 🛑
- No SQL concatenation: ✅ / 🛑
- No weak cryptography: ✅ / 🛑
- Proper DI usage: ✅ / 🛑

[IF VIOLATIONS]
**🛑 Anti-Pattern Violations Detected:**
- [Violation]: [Resolution steps]

---

#### 7. Performance Requirements

**Targets**: [Specific metrics or Vague "fast/scalable"]

**Current State**: [System currently meets targets / Falls short / Unknown]

**Optimization Needed**: [Y/N]

**Scalability**: [Current / 2x / 5x / 10x capacity]

**Complexity Impact**: +[X] points

---

### Key Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|-----------|
| [Risk 1] | [H/M/L] | [H/M/L] | [Mitigation strategy] |
| [Risk 2] | [H/M/L] | [H/M/L] | [Mitigation strategy] |
| [Risk 3] | [H/M/L] | [H/M/L] | [Mitigation strategy] |

---

### Prerequisites

**Before Implementation**:
- [ ] [Prerequisite 1]
- [ ] [Prerequisite 2]
- [ ] [Prerequisite 3]

**Infrastructure Needed**:
- {Infrastructure 1}
- {Infrastructure 2}

**Team Preparation**:
- {Training or hiring}
- {Documentation}

---

### Recommendations

**IF Complexity ≤ 4**:
Proceed with feature, standard development process

**IF Complexity 5-6**:
- Consider spike/POC phase
- Allocate extra buffer for unknowns
- Schedule architecture review

**IF Complexity 7-8**:
- REQUIRES ADR for architecture changes
- REQUIRES team review and approval
- REQUIRES risk mitigation plan
- Consider 2-phase implementation (MVP + extended)

**IF Complexity 9-10**:
- 🛑 REQUIRES full architecture review
- 🛑 REQUIRES governance approval
- 🛑 REQUIRES executive stakeholder alignment
- Likely needs multiple epics vs single epic
- Consider phased delivery (18-24 months vs 6 months)

---

### Context File Alignment

**Validated Against:**
- [x] tech-stack.md (all technologies approved or flagged for ADR)
- [x] architecture-constraints.md (no violations detected)
- [x] dependencies.md (external integrations approved)
- [x] coding-standards.md (coding practices documented)
- [x] anti-patterns.md (no forbidden patterns)

[IF ANY FAILED]
**⚠️ Context File Issues:**
- [Issue]: [Resolution required]

---

### Approval Status

**Ready to Proceed**: [Y/N]

[IF N, list blockers]
**Blockers**:
- [Blocker 1]: [Resolution steps]

[IF Y, list approvals]
**Approvals**:
- [x] Architecture review
- [x] Technology decisions
- [x] ADRs created
- [x] Risk mitigation planned
```

---

## Self-Validation Checklist

**Before finalizing assessment:**
- [ ] Complexity score justified by assessment dimensions
- [ ] All technologies checked against tech-stack.md
- [ ] All architecture changes checked against architecture-constraints.md
- [ ] All integrations checked against dependencies.md
- [ ] Security patterns checked against anti-patterns.md
- [ ] All risks documented with probability/impact/mitigation
- [ ] Prerequisites clearly listed
- [ ] ADRs identified for all significant decisions

---

**Last Updated:** 2025-11-05
**Version:** 1.0
**Framework:** DevForgeAI Orchestration
