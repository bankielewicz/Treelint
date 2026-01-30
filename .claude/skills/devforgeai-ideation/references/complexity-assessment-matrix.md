# Complexity Assessment Matrix

Comprehensive scoring rubric to determine project complexity and recommend appropriate architecture tier.

## Table of Contents

1. [Complexity Scoring Overview](#complexity-scoring-overview)
2. [Scoring Dimensions](#scoring-dimensions)
3. [Architecture Tier Definitions](#architecture-tier-definitions)
4. [Technology Recommendations by Tier](#technology-recommendations-by-tier)
5. [Case Studies](#case-studies)

---

## Complexity Scoring Overview

**Scoring Range:** 0-60 points across 4 dimensions

**Architecture Tiers:**
- **Tier 1: Simple Application** (0-15 points)
- **Tier 2: Moderate Application** (16-30 points)
- **Tier 3: Complex Platform** (31-45 points)
- **Tier 4: Enterprise Platform** (46-60 points)

**Principle:** Match architecture complexity to project complexity. Over-engineering wastes resources; under-engineering creates technical debt.

---

## Scoring Dimensions

### Dimension 1: Functional Complexity (0-20 points)

#### 1.1 User Roles (0-10 points)

**Score 2: Single Role**
- Example: Personal todo app (only owner)
- Authentication: Optional
- Authorization: Not needed

**Score 5: 2-3 Roles**
- Example: Blog (reader, author, admin)
- Authentication: Required
- Authorization: Role-based (basic)

**Score 7: 4-6 Roles**
- Example: E-commerce (customer, seller, admin, support, warehouse)
- Authentication: Required with MFA for admin
- Authorization: Role-based with permissions

**Score 10: 7+ Roles with Hierarchies**
- Example: Enterprise ERP (multiple departments, managers, approvers, executives)
- Authentication: SSO + MFA
- Authorization: Role hierarchies + attribute-based

#### 1.2 Core Data Entities (0-10 points)

**Score 2: 1-3 Entities**
- Example: Todo app (User, Task)
- Relationships: Minimal (User → Tasks)
- Database: Single table set

**Score 5: 4-7 Entities**
- Example: Blog (User, Post, Comment, Category, Tag)
- Relationships: Moderate (many-to-many for tags)
- Database: Normalized schema

**Score 8: 8-15 Entities**
- Example: E-commerce (User, Product, Category, Order, OrderItem, Payment, Shipment, Review, Inventory)
- Relationships: Complex (multiple many-to-many, cascading deletes)
- Database: Normalized with indexes

**Score 10: 16+ Entities**
- Example: ERP (Sales, Inventory, Finance, HR, dozens of entities)
- Relationships: Highly complex (cross-module dependencies)
- Database: Polyglot persistence (SQL + NoSQL)

#### 1.3 External Integrations (0-10 points)

**Score 0: No Integrations**
- Self-contained application
- No external dependencies

**Score 3: 1-2 Simple Integrations**
- Example: Email service (SendGrid)
- Integration: REST API, synchronous
- Error handling: Basic retry logic

**Score 6: 3-5 Integrations (Mixed Complexity)**
- Example: Payment gateway (Stripe), email (SendGrid), analytics (Mixpanel)
- Integration: REST APIs, webhooks
- Error handling: Retry logic, idempotency

**Score 10: 6+ Integrations or Complex Integrations**
- Example: Legacy ERP, multiple third-party APIs, real-time data sync
- Integration: REST, SOAP, message queues, webhooks, polling
- Error handling: Circuit breakers, compensation logic, event sourcing

#### 1.4 Workflow Complexity (0-10 points)

**Score 2: Linear Workflows**
- Example: User registration → Email verification → Dashboard
- No branching or loops
- Single happy path

**Score 5: Branching Workflows**
- Example: Order fulfillment (conditional steps based on inventory, payment type, shipping method)
- Multiple paths based on conditions
- Error handling with retries

**Score 8: State Machine Workflows**
- Example: Loan approval (submitted → in review → approved/rejected → funded)
- Complex state transitions
- Business rules govern transitions
- Rollback/compensation logic

**Score 10: Multi-Entity Orchestration**
- Example: Supply chain (order triggers inventory, manufacturing, shipping, invoicing across systems)
- Long-running processes (hours/days)
- Saga patterns for distributed transactions
- Event-driven coordination

---

### Dimension 2: Technical Complexity (0-20 points)

#### 2.1 Data Volume (0-10 points)

**Score 2: Small Data**
- Records: < 10,000
- Storage: < 1 GB
- Database: SQLite or single PostgreSQL instance
- Queries: Simple, no optimization needed

**Score 5: Medium Data**
- Records: 10,000 - 1,000,000
- Storage: 1 GB - 100 GB
- Database: PostgreSQL/MySQL with indexes
- Queries: Optimized queries, covering indexes

**Score 8: Large Data**
- Records: 1M - 100M
- Storage: 100 GB - 10 TB
- Database: Sharding, read replicas, caching layer (Redis)
- Queries: Query optimization, materialized views, CQRS

**Score 10: Massive Data**
- Records: > 100M
- Storage: > 10 TB
- Database: Distributed databases (Cassandra, DynamoDB), data lakes
- Queries: Big data tools (Spark, Hadoop), real-time analytics

#### 2.2 Concurrent Users (0-10 points)

**Score 2: Low Concurrency**
- Users: < 100 concurrent
- Infrastructure: Single server
- Scaling: Vertical scaling sufficient

**Score 5: Medium Concurrency**
- Users: 100 - 1,000 concurrent
- Infrastructure: Load balancer + 2-3 app servers
- Scaling: Horizontal scaling (manual)

**Score 8: High Concurrency**
- Users: 1,000 - 10,000 concurrent
- Infrastructure: Auto-scaling groups, CDN, caching
- Scaling: Horizontal auto-scaling

**Score 10: Massive Concurrency**
- Users: > 10,000 concurrent
- Infrastructure: Multi-region, global CDN, edge computing
- Scaling: Horizontal auto-scaling, rate limiting, queue-based processing

#### 2.3 Real-Time Requirements (0-10 points)

**Score 0: No Real-Time**
- Batch processing acceptable
- User can refresh page
- Delay tolerance: Minutes/hours

**Score 3: Soft Real-Time (Polling)**
- Polling every 5-30 seconds
- Example: Email inbox (fetch new emails periodically)
- Technology: HTTP polling, short polling

**Score 7: Real-Time (WebSockets)**
- Latency: < 1 second
- Example: Chat application, live dashboards
- Technology: WebSockets, Server-Sent Events (SSE)

**Score 10: Hard Real-Time (Event-Driven)**
- Latency: < 100ms
- Example: Trading platform, multiplayer game, IoT device control
- Technology: WebSockets + message queues, event sourcing, CQRS

---

### Dimension 3: Team & Organizational Complexity (0-10 points)

#### 3.1 Team Size (0-5 points)

**Score 1: Solo Developer**
- 1 developer
- Simpler architecture, fewer abstractions
- Monolithic preferred

**Score 3: Small Team (2-5 developers)**
- 2-5 developers
- Modular monolith or simple microservices
- Co-located or remote

**Score 5: Large Team (6+ developers)**
- 6+ developers
- Microservices or modular architecture
- Multiple feature teams
- Distributed/remote

#### 3.2 Team Distribution (0-5 points)

**Score 1: Co-located**
- Same office, timezone
- Synchronous communication easy
- Minimal coordination overhead

**Score 3: Remote (Same Timezone)**
- Remote but same timezone
- Synchronous communication during work hours
- Coordination tools needed

**Score 5: Distributed (Multi-Timezone)**
- Multiple timezones (e.g., US + Europe + Asia)
- Asynchronous communication required
- High coordination overhead
- Documentation critical

---

### Dimension 4: Non-Functional Complexity (0-10 points)

#### 4.1 Performance Requirements (0-5 points)

**Score 1: Relaxed Performance**
- Response time: < 5 seconds acceptable
- Throughput: Low (< 10 req/sec)
- Example: Internal admin tool

**Score 3: Standard Performance**
- Response time: < 1 second
- Throughput: Medium (10-100 req/sec)
- Example: Business web app

**Score 5: High Performance**
- Response time: < 200ms
- Throughput: High (100-1000 req/sec)
- Example: E-commerce, SaaS platform, fintech

#### 4.2 Compliance & Regulatory (0-5 points)

**Score 0: No Special Compliance**
- Standard security practices
- No regulatory requirements

**Score 2: Basic Compliance (GDPR)**
- GDPR (EU users)
- Data privacy, right to deletion
- Audit logging

**Score 4: Industry Compliance (PCI-DSS, SOC 2)**
- PCI-DSS (credit card processing)
- SOC 2 (SaaS security certification)
- Regular audits required

**Score 5: Strict Compliance (HIPAA, Financial)**
- HIPAA (healthcare)
- Banking regulations (KYC, AML)
- Zero tolerance for violations
- Extensive audit trails

---

## Architecture Tier Definitions

### Tier 1: Simple Application (0-15 points)

**Characteristics:**
- Few user roles (1-2)
- Few entities (1-3)
- Low data volume (< 10k records)
- Low concurrency (< 100 users)
- No real-time requirements
- Solo or small team (1-3 developers)
- Relaxed performance
- No special compliance

**Recommended Architecture:**
- **Pattern:** Monolithic
- **Layers:** 2-3 (Presentation, Business Logic, Data Access)
- **Database:** Single SQL database (PostgreSQL, SQLite) or simple NoSQL (MongoDB)
- **Deployment:** Single server, serverless (Vercel, Netlify), or PaaS (Heroku)
- **Authentication:** Simple JWT or session-based
- **Caching:** Optional (in-memory caching)

**Examples:**
- Personal todo app
- Simple blog or portfolio
- Small business website
- Proof-of-concept MVP

**Technology Stack Suggestions:**
- **Backend:** Node.js + Express, Python + Flask, ASP.NET Core (minimal)
- **Frontend:** React (simple), Vue.js, vanilla JavaScript
- **Database:** PostgreSQL, SQLite, MongoDB
- **Hosting:** Vercel, Netlify, Heroku, Railway

---

### Tier 2: Moderate Application (16-30 points)

**Characteristics:**
- Multiple user roles (3-5)
- Moderate entities (4-10)
- Medium data volume (10k-1M records)
- Medium concurrency (100-1k users)
- Soft real-time (polling acceptable)
- Small-medium team (3-8 developers)
- Standard performance (< 1s response time)
- Basic compliance (GDPR)

**Recommended Architecture:**
- **Pattern:** Modular Monolith or Simple Microservices (2-3 services)
- **Layers:** 3-4 (API, Application, Domain, Infrastructure)
- **Database:** Primary SQL database + optional read replicas
- **Deployment:** Load-balanced (2-4 instances), containerized (Docker)
- **Authentication:** OAuth 2.0 or JWT with refresh tokens
- **Caching:** Redis for session storage and frequently accessed data
- **Background Jobs:** Redis/Celery/Hangfire for async processing

**Examples:**
- E-commerce site
- SaaS tool (single tenant)
- Internal business dashboard
- Content management system

**Technology Stack Suggestions:**
- **Backend:** .NET 8.0 (Clean Architecture), Node.js + NestJS, Python + FastAPI
- **Frontend:** React + TypeScript + Zustand/Redux, Next.js
- **Database:** PostgreSQL, MySQL, SQL Server
- **Caching:** Redis
- **Hosting:** Azure App Service, AWS Elastic Beanstalk, DigitalOcean

---

### Tier 3: Complex Platform (31-45 points)

**Characteristics:**
- Many user roles (6+) with hierarchies
- Many entities (11-20)
- Large data volume (1M-100M records)
- High concurrency (1k-10k users)
- Real-time features (WebSockets)
- Multiple integrations (5+)
- Medium-large team (8-15 developers)
- High performance (< 500ms)
- Industry compliance (SOC 2, PCI-DSS)

**Recommended Architecture:**
- **Pattern:** Microservices (5-10 services) or Clean Architecture with Domain-Driven Design
- **Layers:** 4-5 (API Gateway, Application, Domain, Infrastructure, Cross-Cutting Concerns)
- **Database:** Polyglot persistence (SQL for transactional, NoSQL for documents/cache, event store)
- **Deployment:** Kubernetes, service mesh (Istio), multi-region
- **Authentication:** OAuth 2.0 + OpenID Connect, SSO integration
- **Caching:** Distributed caching (Redis Cluster), CDN for static assets
- **Message Queues:** RabbitMQ, Kafka for event-driven communication
- **Background Jobs:** Dedicated worker services

**Examples:**
- Multi-tenant SaaS platform
- Marketplace platform (two-sided)
- Healthcare platform
- Financial services platform

**Technology Stack Suggestions:**
- **Backend:** .NET 8.0 (Microservices), Java + Spring Boot, Go
- **Frontend:** React + TypeScript + Redux Toolkit, Next.js (SSR)
- **Database:** PostgreSQL (primary), MongoDB (documents), Redis (cache), Elasticsearch (search)
- **Message Queue:** RabbitMQ, Kafka, Azure Service Bus
- **Hosting:** Kubernetes (AKS, EKS, GKE), Docker Swarm

---

### Tier 4: Enterprise Platform (46-60 points)

**Characteristics:**
- Complex role hierarchies (10+ roles)
- Extensive entities (20+ across domains)
- Massive data volume (> 100M records)
- Massive concurrency (> 10k users)
- Hard real-time requirements (< 100ms)
- Complex integrations (10+, legacy systems)
- Large distributed team (15+ developers)
- Extreme performance (< 200ms p95)
- Strict compliance (HIPAA, banking regulations)

**Recommended Architecture:**
- **Pattern:** Distributed Microservices + Event-Driven Architecture + CQRS + Event Sourcing
- **Layers:** Domain-Driven Design with bounded contexts
- **Database:** Polyglot persistence (SQL, NoSQL, graph, time-series, event store)
- **Deployment:** Multi-region Kubernetes, auto-scaling, chaos engineering
- **Authentication:** Enterprise SSO (SAML, OAuth 2.0), multi-factor authentication
- **Caching:** Multi-level caching (L1: in-memory, L2: Redis, L3: CDN)
- **Message Queues:** Kafka (event streaming), RabbitMQ (command/query)
- **API Gateway:** Kong, Apigee, AWS API Gateway with rate limiting
- **Observability:** Distributed tracing (Jaeger), centralized logging (ELK), metrics (Prometheus/Grafana)
- **Disaster Recovery:** Multi-region active-active, RPO < 1 hour, RTO < 4 hours

**Examples:**
- Global fintech platform (trading, banking)
- Streaming service (Netflix, Spotify scale)
- IoT platform (millions of devices)
- Large-scale social network

**Technology Stack Suggestions:**
- **Backend:** .NET 8.0 (Microservices + DDD), Java + Spring Boot, Go (high-performance services)
- **Frontend:** React + TypeScript + Redux Toolkit, Micro-frontends
- **Database:** PostgreSQL (transactional), Cassandra (high-write), Redis (cache), Elasticsearch (search), Neo4j (graph), InfluxDB (time-series)
- **Message Queue:** Kafka (event streaming), RabbitMQ/AWS SQS
- **Hosting:** Kubernetes (multi-region), Serverless (AWS Lambda, Azure Functions for edge computing)

---

## Technology Recommendations by Tier

### Backend Frameworks

| Tier | Lightweight | Moderate | Enterprise |
|------|-------------|----------|------------|
| **Tier 1** | Express.js, Flask | FastAPI, Koa | - |
| **Tier 2** | NestJS, FastAPI | ASP.NET Core, Django | - |
| **Tier 3** | - | NestJS + Microservices | .NET 8.0 + DDD, Spring Boot |
| **Tier 4** | - | - | .NET 8.0 + CQRS, Java + Spring Cloud |

### Frontend Frameworks

| Tier | Simple | Moderate | Complex |
|------|--------|----------|---------|
| **Tier 1** | Vanilla JS, Svelte | React, Vue.js | - |
| **Tier 2** | React + Context API | React + Zustand/Redux | Next.js |
| **Tier 3** | - | Next.js, React + Redux Toolkit | Next.js + SSR, Gatsby |
| **Tier 4** | - | - | Micro-frontends, Next.js + ISR |

### Databases

| Tier | Simple | Moderate | Complex |
|------|--------|----------|---------|
| **Tier 1** | SQLite, MongoDB | PostgreSQL | - |
| **Tier 2** | PostgreSQL, MySQL | PostgreSQL + Redis | PostgreSQL + Redis + Elasticsearch |
| **Tier 3** | - | PostgreSQL + Redis + MongoDB | Polyglot (SQL + NoSQL + Event Store) |
| **Tier 4** | - | - | Distributed SQL (CockroachDB), Cassandra, Kafka |

### Deployment

| Tier | Simple | Moderate | Complex |
|------|--------|----------|---------|
| **Tier 1** | Vercel, Netlify, Heroku | Railway, Render | - |
| **Tier 2** | Azure App Service, AWS Beanstalk | Docker + VM | - |
| **Tier 3** | - | Kubernetes (single region) | Kubernetes + Service Mesh |
| **Tier 4** | - | - | Multi-region Kubernetes + Auto-scaling |

---

## Case Studies

### Case Study 1: Todo App (Tier 1, Score: 8)

**Scoring:**
- User roles: 1 (owner) → 2 points
- Entities: 2 (User, Task) → 2 points
- Integrations: 0 → 0 points
- Workflow: Linear → 2 points
- Data volume: < 10k → 2 points
- Concurrency: < 100 → 0 points
- Real-time: None → 0 points
- Team: Solo → 0 points
- Performance: Relaxed → 0 points
- Compliance: None → 0 points

**Total: 8 points (Tier 1: Simple Application)**

**Recommended:**
- Architecture: Monolithic
- Backend: Node.js + Express or Python + Flask
- Frontend: React (simple, no state management library)
- Database: SQLite or PostgreSQL
- Deployment: Vercel (frontend) + Railway (backend)

---

### Case Study 2: E-commerce Site (Tier 2, Score: 24)

**Scoring:**
- User roles: 3 (customer, admin, support) → 5 points
- Entities: 9 (User, Product, Category, Order, OrderItem, Payment, Shipment, Review, Inventory) → 8 points
- Integrations: 3 (Stripe, SendGrid, Mixpanel) → 6 points
- Workflow: Branching (order fulfillment) → 5 points
- Data volume: 10k-1M products → 5 points
- Concurrency: 100-1k users → 5 points
- Real-time: Polling (inventory updates) → 3 points
- Team: 3-5 developers → 3 points
- Performance: Standard (< 1s) → 3 points
- Compliance: GDPR → 2 points

**Total: 24 points (Tier 2: Moderate Application)**

**Recommended:**
- Architecture: Modular Monolith with Clean Architecture
- Backend: .NET 8.0 or Node.js + NestJS
- Frontend: React + TypeScript + Zustand
- Database: PostgreSQL (primary) + Redis (caching)
- Deployment: Azure App Service or AWS Elastic Beanstalk

---

### Case Study 3: Multi-tenant SaaS (Tier 3, Score: 38)

**Scoring:**
- User roles: 6 (tenant admin, user, billing, support, super admin, API consumer) → 7 points
- Entities: 15 (across tenant management, core features, billing) → 8 points
- Integrations: 6 (payment, email, SMS, analytics, SSO, webhooks) → 10 points
- Workflow: State machines (subscription lifecycle, feature provisioning) → 8 points
- Data volume: 1M-10M records → 8 points
- Concurrency: 1k-10k users → 8 points
- Real-time: WebSockets (notifications, live updates) → 7 points
- Team: 10 developers → 5 points
- Performance: High (< 500ms) → 5 points
- Compliance: SOC 2 → 4 points

**Total: 38 points (Tier 3: Complex Platform)**

**Recommended:**
- Architecture: Microservices (6-8 services: API Gateway, Auth, Tenant Management, Core Features, Billing, Notifications)
- Backend: .NET 8.0 with Clean Architecture + DDD
- Frontend: Next.js + React + Redux Toolkit
- Database: PostgreSQL (tenants, users, billing) + MongoDB (feature data) + Redis (cache) + Elasticsearch (search)
- Message Queue: RabbitMQ or Kafka
- Deployment: Kubernetes (AKS, EKS) with Helm charts

---

### Case Study 4: Global Fintech Platform (Tier 4, Score: 52)

**Scoring:**
- User roles: 10+ (customers, merchants, admins, compliance, fraud analysts, support, API consumers, partners) → 10 points
- Entities: 25+ (accounts, transactions, cards, settlements, compliance records, fraud rules) → 10 points
- Integrations: 15+ (banking APIs, card networks, KYC providers, fraud detection, reporting) → 10 points
- Workflow: Complex orchestration (payment processing, settlement, compliance workflows) → 10 points
- Data volume: > 100M transactions → 10 points
- Concurrency: > 10k concurrent users → 10 points
- Real-time: Hard real-time (transaction processing < 100ms) → 10 points
- Team: 20+ developers → 5 points
- Performance: Extreme (< 200ms p95) → 5 points
- Compliance: Banking regulations (KYC, AML, PCI-DSS) → 5 points

**Total: 52 points (Tier 4: Enterprise Platform)**

**Recommended:**
- Architecture: Distributed Microservices + Event-Driven + CQRS + Event Sourcing
- Backend: .NET 8.0 (microservices) + Go (high-performance transaction processing)
- Frontend: Micro-frontends (React + Module Federation)
- Database: PostgreSQL (accounts, users), Cassandra (transactions, event sourcing), Redis (cache), Elasticsearch (search, fraud detection)
- Message Queue: Kafka (event streaming), RabbitMQ (commands)
- Deployment: Multi-region Kubernetes (active-active), auto-scaling, disaster recovery
- Observability: Distributed tracing (Jaeger), centralized logging (ELK), metrics (Prometheus/Grafana)

---

## Scoring Worksheet

Use this worksheet during Phase 3 to calculate complexity score:

```markdown
## Complexity Assessment Worksheet

### Functional Complexity (0-20)
- [ ] User Roles: ___ points (0-10)
- [ ] Core Entities: ___ points (0-10)
- [ ] Integrations: ___ points (0-10)
- [ ] Workflow Complexity: ___ points (0-10)
**Subtotal:** ___ / 20

### Technical Complexity (0-20)
- [ ] Data Volume: ___ points (0-10)
- [ ] Concurrent Users: ___ points (0-10)
- [ ] Real-Time Requirements: ___ points (0-10)
**Subtotal:** ___ / 20

### Team/Organizational Complexity (0-10)
- [ ] Team Size: ___ points (0-5)
- [ ] Team Distribution: ___ points (0-5)
**Subtotal:** ___ / 10

### Non-Functional Complexity (0-10)
- [ ] Performance Requirements: ___ points (0-5)
- [ ] Compliance & Regulatory: ___ points (0-5)
**Subtotal:** ___ / 10

---

**TOTAL COMPLEXITY SCORE:** ___ / 60

**RECOMMENDED TIER:** [Tier N: Name]

**RATIONALE:** [Key drivers of complexity score]
```

---

**Use this matrix in Phase 3 (Complexity Assessment) to systematically score project complexity and recommend the appropriate architecture tier.**
