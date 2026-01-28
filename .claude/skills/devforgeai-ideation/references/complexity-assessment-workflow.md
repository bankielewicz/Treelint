# Phase 3: Complexity Assessment & Architecture Planning Workflow

Score project complexity (0-60) across 4 dimensions and determine appropriate architecture tier.

## TodoWrite - Phase Start

**At phase start, update todo list:**
```
TodoWrite([
  {"content": "Phase 3: Complexity Assessment", "status": "in_progress", "activeForm": "Calculating complexity score"}
])
```

## Overview

Phase 3 evaluates solution complexity to prevent over-engineering simple problems or under-architecting complex platforms. The scoring system produces a quantified complexity score (0-60) that maps to one of four architecture tiers.

**Duration:** 5-10 minutes
**Questions:** Scoring validation questions
**Output:** Complexity score (0-60), architecture tier (1-4), technology recommendations

---

## Step 3.1: Complexity Scoring

### Load Assessment Matrix

```
Read(file_path=".claude/skills/devforgeai-ideation/references/complexity-assessment-matrix.md")
```

This reference provides the comprehensive 0-60 scoring rubric with examples, edge case guidance, and technology recommendations per tier.

### Calculate Score Across 4 Dimensions

**Dimension 1: Functional Complexity (0-20 points)**

Based on Phase 2 functional requirements:

- **User roles:** Count from user stories
  - 1-2 roles: Low = 5 points
  - 3-5 roles: Medium = 10 points
  - 6+ roles: High = 15 points

- **Core entities:** Count from data model
  - 1-3 entities: Low = 5 points
  - 4-10 entities: Medium = 10 points
  - 11+ entities: High = 15 points

- **Integrations:** Count from integration requirements
  - 0-1 integration: Low = 3 points
  - 2-4 integrations: Medium = 7 points
  - 5+ integrations: High = 10 points

- **Workflow complexity:** Analyze user flows
  - Linear workflows: Low = 3 points
  - Branching workflows: Medium = 7 points
  - State machines/complex logic: High = 10 points

**Functional subtotal:** 0-20 points

**Dimension 2: Technical Complexity (0-20 points)**

Based on Phase 2 NFRs:

- **Data volume:** Expected data scale
  - <10k records: Low = 5 points
  - 10k-1M records: Medium = 10 points
  - >1M records: High = 15 points

- **Concurrency:** Simultaneous user target
  - <100 concurrent: Low = 5 points
  - 100-10k concurrent: Medium = 10 points
  - >10k concurrent: High = 15 points

- **Real-time requirements:** Synchronization needs
  - None/Async: Low = 3 points
  - Polling/periodic sync: Medium = 7 points
  - WebSockets/event-driven: High = 10 points

**Technical subtotal:** 0-20 points

**Dimension 3: Team/Organizational Complexity (0-10 points)**

Based on team context from Phase 1:

- **Team size:** Development team count
  - 1-3 developers: Low = 3 points
  - 4-10 developers: Medium = 6 points
  - 11+ developers: High = 10 points

- **Team distribution:** Co-location
  - Co-located: Low = 2 points
  - Remote same timezone: Medium = 5 points
  - Multi-timezone: High = 8 points

**Team/Org subtotal:** 0-10 points

**Dimension 4: Non-Functional Complexity (0-10 points)**

Based on Phase 2 NFRs:

- **Performance targets:** How aggressive
  - Moderate (<2s response): Low = 3 points
  - Standard (<500ms response): Medium = 6 points
  - High performance (<100ms): High = 10 points

- **Compliance requirements:** Regulatory burden
  - None: Low = 0 points
  - Standard (GDPR, basic security): Medium = 5 points
  - Strict (HIPAA, PCI-DSS, SOC2): High = 10 points

**NFR subtotal:** 0-10 points

### Calculate Total Score

```
total_score = functional + technical + team_org + nfr

# Validate range
assert 0 <= total_score <= 60
assert 0 <= functional <= 20
assert 0 <= technical <= 20
assert 0 <= team_org <= 10
assert 0 <= nfr <= 10
```

---

## Step 3.2: Architecture Tier Recommendation

Map complexity score to architecture tier:

**Tier 1: Simple Application (0-15 points)**
- **Architecture:** Monolithic
- **Layers:** 2-3 (API, Business Logic, Data)
- **Database:** Single database
- **Deployment:** Single server or serverless
- **Example:** Todo app, blog, portfolio site, simple CRUD app

**Recommended Technologies:**
- Backend: Express.js, Flask, Ruby on Rails, Laravel
- Frontend: React, Vue, Svelte
- Database: PostgreSQL, MySQL, SQLite
- Deployment: Vercel, Netlify, Heroku, single VPS

**Tier 2: Moderate Application (16-30 points)**
- **Architecture:** Modular Monolith
- **Layers:** 3-4 (API, Application, Domain, Infrastructure)
- **Database:** Primary + read replicas
- **Deployment:** Load-balanced multi-instance
- **Example:** E-commerce site, SaaS tool, internal platform

**Recommended Technologies:**
- Backend: NestJS, ASP.NET Core, Spring Boot, Django
- Frontend: React + state management (Zustand, Redux), Next.js
- Database: PostgreSQL + Redis cache
- Deployment: AWS ECS, Azure App Service, DigitalOcean

**Tier 3: Complex Platform (31-45 points)**
- **Architecture:** Microservices or Clean Architecture
- **Layers:** 4-5 with domain separation
- **Database:** Polyglot persistence (SQL + NoSQL)
- **Deployment:** Kubernetes, service mesh
- **Example:** Multi-tenant SaaS, marketplace, workflow automation

**Recommended Technologies:**
- Backend: Microservices (Node.js, .NET, Go)
- Frontend: React + micro-frontends, Next.js
- Database: PostgreSQL + MongoDB + Redis + message queue
- Deployment: Kubernetes (EKS, AKS, GKE), Istio service mesh

**Tier 4: Enterprise Platform (46-60 points)**
- **Architecture:** Distributed microservices with event-driven patterns
- **Layers:** 5+ with DDD, CQRS, event sourcing
- **Database:** Polyglot + event store + CQRS read models
- **Deployment:** Multi-region, auto-scaling, global CDN
- **Example:** Global fintech, streaming service, large-scale marketplace

**Recommended Technologies:**
- Backend: Event-driven microservices (Go, Rust, .NET)
- Frontend: Micro-frontends, edge rendering
- Database: PostgreSQL + Cassandra + Kafka + Redis
- Deployment: Kubernetes multi-cluster, service mesh, global load balancing

---

## Step 3.3: Validate Tier Recommendation

```
Question: "Based on requirements, this appears to be {Tier} with complexity score {X}/60. Does this match your expectations?"
Options:
  - "Yes, that's correct"
  - "No, it should be simpler (explain why)"
  - "No, it should be more complex (explain why)"
```

**If "No, it should be simpler":**
- Review requirements: Are some features optional for MVP?
- Re-score with reduced scope
- Propose deferring complex features to later releases

**If "No, it should be more complex":**
- Ask: "What additional complexity factors did we miss?"
- Probe for hidden requirements (compliance, scale, integrations)
- Re-score with complete requirements

---

## Output from Phase 3

**Complexity Assessment Report:**

```markdown
## Complexity Assessment

**Total Score:** {X}/60
**Architecture Tier:** {Simple|Moderate|Complex|Enterprise}

### Score Breakdown

- **Functional Complexity:** {score}/20
  - User roles: {count} ({Low|Medium|High})
  - Core entities: {count} ({Low|Medium|High})
  - Integrations: {count} ({Low|Medium|High})
  - Workflow complexity: {Linear|Branching|State Machine}

- **Technical Complexity:** {score}/20
  - Data volume: {estimate} ({Low|Medium|High})
  - Concurrency: {estimate} concurrent users ({Low|Medium|High})
  - Real-time: {None|Polling|Event-driven}

- **Team/Organizational Complexity:** {score}/10
  - Team size: {count} developers ({Low|Medium|High})
  - Distribution: {Co-located|Remote|Multi-timezone}

- **Non-Functional Complexity:** {score}/10
  - Performance: {<2s|<500ms|<100ms}
  - Compliance: {None|Standard|Strict}

### Architecture Tier Recommendation

**Tier {1-4}: {Name}**

**Recommended Architecture:**
- Pattern: {architecture pattern}
- Layers: {layer count and names}
- Database strategy: {database approach}
- Deployment: {deployment strategy}

**Recommended Technology Stack:**
- Backend: {recommendations}
- Frontend: {recommendations}
- Database: {recommendations}
- Infrastructure: {recommendations}
```

**Transition:** Proceed to Phase 4 (Epic & Feature Decomposition)

---

## Common Issues and Recovery

### Issue: Score Borderline Between Tiers

**Symptom:** Score is 15-16 (Tier 1/2 boundary) or 30-31 (Tier 2/3 boundary)

**Recovery:**
1. Review score components for accuracy
2. Consider project growth trajectory (2-year horizon)
3. Lean toward higher tier if growth expected
4. Use AskUserQuestion if uncertain:

```
Question: "Score is {X}, borderline between {Tier A} and {Tier B}. Consider 2-year growth. Which is more appropriate?"
Options:
  - "{Tier A}: {simpler architecture}"
  - "{Tier B}: {more scalable architecture}"
```

### Issue: Requirements Insufficient for Scoring

**Symptom:** Missing data needed for complexity calculation

**Recovery:**
1. Return to Phase 2 (Requirements Elicitation)
2. Ask specific questions for missing dimensions
3. Example: "How many concurrent users do you expect?" (needed for Technical Complexity)
4. Re-run scoring after requirements complete

---

## References Used in Phase 3

**Primary:**
- **complexity-assessment-matrix.md** (617 lines) - Complete 0-60 scoring rubric with examples

**On Error:**
- **error-handling.md** - Recovery for complexity assessment errors

---

## Success Criteria

Phase 3 complete when:
- [ ] Complexity scored across all 4 dimensions
- [ ] Total score in range 0-60
- [ ] Architecture tier determined (1-4)
- [ ] Tier matches score range
- [ ] Technology recommendations provided for tier
- [ ] User validated tier recommendation

**Token Budget:** ~3,000-5,000 tokens (load matrix, calculate, validate)

---

## TodoWrite - Phase Completion

**At phase end, mark as completed with complexity score:**
```
TodoWrite([
  {"content": "Phase 3: Complexity Assessment", "status": "completed", "activeForm": "Calculating complexity score"}
])
```

**Display complexity score summary before marking complete:**
```
Display: "✓ Complexity Score: {total_score}/60 → Tier {tier_number}: {tier_name}"
```

---

**Next Phase:** Phase 4 (Epic & Feature Decomposition)
