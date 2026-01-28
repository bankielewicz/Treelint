# Requirements Elicitation Guide

Comprehensive probing questions, user story templates, and NFR checklists to systematically extract complete requirements.

## Table of Contents

1. [Discovery Questions by Domain](#discovery-questions-by-domain)
2. [User Story Templates](#user-story-templates)
3. [NFR Checklists by Industry](#nfr-checklists-by-industry)
4. [Interview Techniques](#interview-techniques)
5. [Requirement Validation](#requirement-validation)

---

## Discovery Questions by Domain

### E-commerce

**Core Capabilities:**
- What products will you sell? (Physical goods, digital products, services, subscriptions)
- How will customers discover products? (Search, categories, recommendations, curated collections)
- What payment methods should be supported? (Credit cards, PayPal, crypto, buy-now-pay-later)
- How complex is shipping? (Single warehouse, multiple fulfillment centers, international shipping)
- What customer account features are needed? (Order history, wishlists, saved addresses, loyalty programs)
- What inventory management is required? (Real-time stock tracking, backorders, pre-orders)
- What marketing features are needed? (Coupons, promotions, email campaigns, abandoned cart recovery)

**Regulatory/Compliance:**
- Will you handle EU customers? (GDPR compliance required)
- Will you accept credit cards directly? (PCI-DSS compliance required)
- What refund/return policies apply? (Consumer protection laws vary by region)

### SaaS Applications

**Core Capabilities:**
- What is the multi-tenancy model? (Shared database, separate databases per tenant, hybrid)
- What are the subscription tiers? (Free, starter, professional, enterprise with different feature sets)
- How should usage be tracked? (API calls, storage, users, compute time)
- What billing model? (Monthly, annual, usage-based, tiered pricing)
- What admin capabilities are needed? (Tenant management, user management, usage analytics)
- What collaboration features? (Team workspaces, shared resources, permission management)
- What integration capabilities? (REST APIs, webhooks, OAuth, SSO)

**Scalability Considerations:**
- Expected number of tenants? (10s, 100s, 1000s, 10,000s+)
- Expected users per tenant? (1-10, 10-100, 100-1000, 1000+)
- What SLA guarantees? (99.9% uptime, response time guarantees)

### Fintech Platforms

**Core Capabilities:**
- What financial operations? (Payments, transfers, lending, investing, accounting)
- What account types? (Checking, savings, investment, credit)
- What transaction types? (P2P, P2B, B2B, international)
- What regulatory requirements? (KYC, AML, SOC2, banking regulations)
- What fraud prevention? (Transaction monitoring, risk scoring, identity verification)
- What reporting requirements? (Financial statements, tax documents, audit trails)

**Security & Compliance:**
- What data encryption requirements? (At rest, in transit, key management)
- What audit logging requirements? (All transactions, administrative actions, data access)
- What disaster recovery requirements? (RPO, RTO, backup frequency)
- What regulatory reporting? (SAR, CTR, OFAC screening)

### Healthcare Systems

**Core Capabilities:**
- What patient data will be stored? (Demographics, medical history, prescriptions, lab results, imaging)
- What clinical workflows? (Scheduling, check-in, charting, billing, referrals)
- What provider roles? (Physicians, nurses, admin staff, billing)
- What interoperability requirements? (HL7, FHIR, lab system integration, pharmacy integration)
- What telehealth features? (Video consultations, remote monitoring, e-prescriptions)

**Compliance (HIPAA):**
- What PHI will be handled? (Requires HIPAA compliance)
- What BAAs required? (Business Associate Agreements with vendors)
- What audit controls? (Access logs, modification tracking, breach detection)
- What patient rights? (Access, amendment, accounting of disclosures)

### Content Management Systems

**Core Capabilities:**
- What content types? (Articles, pages, media, documents, structured data)
- What authoring workflow? (Draft, review, approval, publish, archive)
- What versioning requirements? (Version history, rollback, diff viewing)
- What taxonomy/organization? (Categories, tags, hierarchies, relationships)
- What media management? (Images, videos, documents, digital assets)
- What publishing channels? (Web, mobile, API, RSS, social media)

**Personalization:**
- What personalization features? (User-based, role-based, geo-based, time-based)
- What A/B testing? (Content variants, performance tracking)
- What analytics? (Page views, engagement, conversion tracking)

### Workflow/Automation Tools

**Core Capabilities:**
- What processes to automate? (Approvals, notifications, data transformations, integrations)
- What workflow patterns? (Sequential, parallel, conditional, looping)
- What trigger types? (Manual, scheduled, event-based, webhook)
- What actions? (Email, API calls, database operations, file operations)
- What error handling? (Retry logic, error notifications, manual intervention)
- What monitoring? (Workflow status, execution history, performance metrics)

**Integration:**
- What systems to integrate? (CRMs, ERPs, email, cloud storage, databases)
- What authentication methods? (API keys, OAuth, SAML)
- What data transformations? (Mapping, filtering, aggregation)

### Marketplaces

**Core Capabilities:**
- What marketplace model? (Two-sided marketplace, multi-sided, aggregator)
- What seller onboarding? (Verification, approval process, seller profiles)
- What listing management? (Product creation, pricing, inventory, categories)
- What transaction model? (Commission, subscription, listing fees, freemium)
- What payment splitting? (Seller payouts, platform fees, escrow)
- What search/discovery? (Keyword search, filters, recommendations, featured listings)
- What trust/safety? (Reviews, ratings, dispute resolution, fraud detection)

**Regulatory:**
- What liability model? (Platform vs. seller responsibility)
- What content moderation? (Listings, reviews, seller conduct)
- What seller agreements? (Terms of service, commission structure)

### Internal Business Tools

**Core Capabilities:**
- What departments will use this? (Sales, operations, finance, HR, IT)
- What data sources? (Databases, APIs, spreadsheets, file systems)
- What reporting needs? (Dashboards, scheduled reports, ad-hoc queries, export formats)
- What automation? (Data sync, notifications, task assignment)
- What access control? (Department-based, role-based, user-based)

**Integration with Existing Systems:**
- What current tools must integrate? (ERP, CRM, HRMS, accounting)
- What data migration? (Historical data, ongoing sync)
- What authentication? (SSO, Active Directory, LDAP)

### IoT/Sensor Platforms

**Core Capabilities:**
- What device types? (Sensors, actuators, gateways, edge devices)
- What data ingestion? (MQTT, HTTP, CoAP, custom protocols)
- What data volume? (Events per second, data retention period)
- What real-time processing? (Alerting, anomaly detection, aggregation)
- What device management? (Provisioning, configuration, firmware updates, monitoring)
- What visualization? (Real-time dashboards, historical trends, geo-mapping)

**Edge vs. Cloud:**
- What processing at edge? (Filtering, aggregation, ML inference)
- What processing in cloud? (Long-term storage, complex analytics, ML training)
- What connectivity? (Always connected, intermittent, store-and-forward)

---

## User Story Templates

### E-commerce User Stories

**Product Browsing:**
```
As a shopper,
I want to browse products by category,
So that I can discover products that interest me.

Acceptance Criteria:
- Categories displayed in navigation menu
- Products listed with image, name, price
- Pagination or infinite scroll for large catalogs
- Category hierarchy (e.g., Electronics > Laptops > Gaming Laptops)
```

**Shopping Cart:**
```
As a shopper,
I want to add products to my cart and modify quantities,
So that I can purchase multiple items in a single transaction.

Acceptance Criteria:
- Add to cart button on product pages
- Cart displays item, quantity, unit price, subtotal
- Update quantity or remove items
- Cart persists across sessions (logged in users)
- Cart subtotal and estimated total displayed
```

**Checkout:**
```
As a shopper,
I want to complete checkout with my shipping and payment information,
So that I can receive my order.

Acceptance Criteria:
- Shipping address form with validation
- Payment method selection (credit card, PayPal, etc.)
- Order review before submission
- Order confirmation page with order number
- Confirmation email sent
```

### SaaS User Stories

**Tenant Onboarding:**
```
As a new customer,
I want to sign up for an account and create my workspace,
So that I can start using the platform.

Acceptance Criteria:
- Registration form (email, password, organization name)
- Email verification required
- Workspace created with default settings
- Redirect to onboarding tutorial
```

**User Invitation:**
```
As a workspace admin,
I want to invite team members to my workspace,
So that we can collaborate.

Acceptance Criteria:
- Invite form with email and role selection
- Invitation email sent with signup link
- Invited user can accept invitation
- Invited user added to workspace with assigned role
```

**Subscription Management:**
```
As a workspace admin,
I want to upgrade or downgrade my subscription plan,
So that I can adjust features and costs based on needs.

Acceptance Criteria:
- View current plan and usage
- Compare available plans
- Select new plan
- Payment processed (if upgrade)
- Plan change takes effect immediately or at next billing cycle
- Confirmation email sent
```

### Fintech User Stories

**Account Opening:**
```
As a new user,
I want to open an account and complete identity verification,
So that I can use financial services.

Acceptance Criteria:
- Account application form (personal information)
- Identity verification (document upload, KYC checks)
- Terms and conditions acceptance
- Account approval or rejection notification
- Account dashboard accessible after approval
```

**Fund Transfer:**
```
As an account holder,
I want to transfer funds to another account,
So that I can send money to others.

Acceptance Criteria:
- Transfer form (recipient, amount, description)
- Account balance validation
- Transfer confirmation with estimated time
- Transaction appears in history
- Notification to sender and recipient
```

**Transaction History:**
```
As an account holder,
I want to view my transaction history with filters and search,
So that I can track my financial activity.

Acceptance Criteria:
- Transactions listed (date, description, amount, balance)
- Filters (date range, type, status)
- Search by description or amount
- Export to CSV or PDF
- Pagination for large histories
```

### Healthcare User Stories

**Patient Appointment Scheduling:**
```
As a patient,
I want to schedule an appointment with my provider,
So that I can receive care.

Acceptance Criteria:
- View provider availability
- Select date and time
- Provide reason for visit
- Receive confirmation email and SMS
- Add to calendar option
```

**Clinical Charting:**
```
As a provider,
I want to document patient encounters with structured notes,
So that I maintain accurate medical records.

Acceptance Criteria:
- Patient chart accessible from schedule
- SOAP note template (Subjective, Objective, Assessment, Plan)
- ICD-10 code lookup for diagnoses
- CPT code lookup for procedures
- E-prescribing integration
- Save as draft or finalize
```

**Lab Results Review:**
```
As a patient,
I want to view my lab results online,
So that I can access my health information.

Acceptance Criteria:
- Lab results listed by date
- Results display with reference ranges
- Abnormal values flagged
- Download results as PDF
- Secure messaging to ask provider questions
```

### Content Management User Stories

**Content Creation:**
```
As a content author,
I want to create and edit articles with rich media,
So that I can publish engaging content.

Acceptance Criteria:
- Rich text editor with formatting
- Image upload and embedding
- Video embedding (YouTube, Vimeo)
- Link insertion
- SEO metadata (title, description, keywords)
- Save as draft or publish
```

**Content Workflow:**
```
As an editor,
I want to review and approve submitted content,
So that I ensure quality before publication.

Acceptance Criteria:
- View pending submissions
- Preview content
- Add comments for revision
- Approve or reject with feedback
- Author notified of decision
```

**Content Versioning:**
```
As a content manager,
I want to view version history and restore previous versions,
So that I can recover from errors or unwanted changes.

Acceptance Criteria:
- Version history listed (date, author, change summary)
- Side-by-side diff view
- Restore to previous version
- Current version preserved in history
```

### Workflow/Automation User Stories

**Workflow Creation:**
```
As a workflow designer,
I want to create automated workflows with triggers and actions,
So that I can automate repetitive tasks.

Acceptance Criteria:
- Visual workflow builder (drag-and-drop)
- Trigger types (manual, scheduled, event, webhook)
- Action types (email, API call, database, condition)
- Variable passing between steps
- Test workflow with sample data
- Activate workflow
```

**Workflow Monitoring:**
```
As a workflow administrator,
I want to monitor workflow executions and troubleshoot failures,
So that I ensure automation reliability.

Acceptance Criteria:
- Execution history (date, status, duration)
- Execution details (input, output, logs)
- Error messages for failures
- Retry failed executions
- Export logs
```

---

## NFR Checklists by Industry

### E-commerce NFRs

**Performance:**
- [ ] Page load time < 2 seconds (mobile 3G network)
- [ ] Product search results < 500ms
- [ ] Checkout flow < 1 second per step
- [ ] Support 10,000 concurrent users during peak (Black Friday)

**Security:**
- [ ] PCI-DSS compliance (if handling credit cards directly)
- [ ] HTTPS everywhere
- [ ] Secure session management
- [ ] XSS and CSRF protection
- [ ] Rate limiting on login and checkout

**Availability:**
- [ ] 99.9% uptime SLA
- [ ] Graceful degradation (show cached catalog if database slow)
- [ ] Multi-region deployment for global customers

**Scalability:**
- [ ] Horizontal scaling for web tier
- [ ] Database read replicas
- [ ] CDN for static assets and images
- [ ] Caching layer (Redis) for product catalog

### SaaS NFRs

**Performance:**
- [ ] API response time < 200ms (p95)
- [ ] Dashboard load time < 1 second
- [ ] Real-time updates < 100ms latency
- [ ] Support 1,000 tenants with 10,000 users each

**Security:**
- [ ] SOC 2 Type II compliance
- [ ] GDPR compliance (EU tenants)
- [ ] Data encryption at rest and in transit
- [ ] Multi-factor authentication (MFA) available
- [ ] SSO support (SAML, OAuth)
- [ ] Audit logging (all tenant administrative actions)

**Availability:**
- [ ] 99.95% uptime SLA
- [ ] Automated backups (daily, retained 30 days)
- [ ] Disaster recovery plan (RPO 1 hour, RTO 4 hours)

**Multi-tenancy:**
- [ ] Tenant data isolation (separate schemas or row-level security)
- [ ] Resource limits per tenant (prevent noisy neighbors)
- [ ] Custom domains per tenant

### Fintech NFRs

**Performance:**
- [ ] Transaction processing < 3 seconds
- [ ] Balance inquiries < 500ms
- [ ] Support 100,000 transactions per day

**Security:**
- [ ] PCI-DSS compliance (if applicable)
- [ ] SOC 2 Type II compliance
- [ ] Data encryption (AES-256 at rest, TLS 1.3 in transit)
- [ ] Multi-factor authentication (required)
- [ ] Fraud detection (transaction monitoring, velocity checks)
- [ ] OFAC screening (if international transactions)

**Compliance:**
- [ ] KYC (Know Your Customer) verification
- [ ] AML (Anti-Money Laundering) monitoring
- [ ] Suspicious Activity Reports (SAR) generation
- [ ] Currency Transaction Reports (CTR) for large transactions
- [ ] Audit trail (immutable transaction logs)
- [ ] Data retention (7 years for financial records)

**Availability:**
- [ ] 99.99% uptime SLA
- [ ] Multi-region active-active deployment
- [ ] Zero data loss (synchronous replication)
- [ ] Disaster recovery plan (RPO 0, RTO 1 hour)

### Healthcare NFRs

**Performance:**
- [ ] Patient lookup < 1 second
- [ ] Chart access < 2 seconds
- [ ] Support 1,000 concurrent providers

**Security (HIPAA):**
- [ ] PHI encryption at rest and in transit
- [ ] Role-based access control (physicians, nurses, admin, billing)
- [ ] Audit logging (all PHI access)
- [ ] Automatic session timeout (15 minutes inactivity)
- [ ] Password complexity requirements
- [ ] Multi-factor authentication (required for remote access)
- [ ] Business Associate Agreements (BAAs) with all vendors

**Compliance:**
- [ ] HIPAA Privacy Rule compliance
- [ ] HIPAA Security Rule compliance
- [ ] Breach notification procedures
- [ ] Patient rights (access, amendment, accounting of disclosures)
- [ ] Data retention (6 years minimum)

**Availability:**
- [ ] 99.9% uptime SLA
- [ ] Scheduled maintenance windows (off-hours)
- [ ] Backup and recovery (daily backups, retained 30 days)

**Interoperability:**
- [ ] HL7 v2.x support (lab results, ADT messages)
- [ ] FHIR API (patient demographics, observations)
- [ ] Direct messaging (secure provider communication)

### Content Management NFRs

**Performance:**
- [ ] Page render time < 1 second
- [ ] Content search < 500ms
- [ ] Support 100,000 page views per day

**Security:**
- [ ] Role-based access control (author, editor, admin)
- [ ] Content approval workflow
- [ ] Audit logging (content changes)

**Scalability:**
- [ ] CDN for static content
- [ ] Database caching for frequent queries
- [ ] Media offloading to cloud storage

**Availability:**
- [ ] 99.9% uptime SLA
- [ ] Scheduled content publication
- [ ] Draft/preview/publish workflow

---

## Interview Techniques

### Open-Ended Questions

**Start broad, narrow progressively:**
1. "Tell me about the business problem you're trying to solve."
2. "Who are the users and what are their goals?"
3. "Walk me through a typical user flow."
4. "What does success look like? How will you measure it?"

### Follow-Up Probes

**When user provides vague answers:**
- "Can you give me a specific example?"
- "What would happen if we didn't have this feature?"
- "How often does this scenario occur?"
- "What are the consequences of getting this wrong?"

### Prioritization Questions

**Separate must-have from nice-to-have:**
- "If you could only have 3 features, which would they be?"
- "What's the minimum functionality needed to launch?"
- "What can we defer to a later release?"

### Edge Case Discovery

**Uncover hidden complexity:**
- "What happens when [unusual scenario]?"
- "How should the system behave if [error condition]?"
- "What's the worst-case scenario?"
- "What volumes or scale do you expect in 1 year? 5 years?"

---

## Requirement Validation

### Completeness Checklist

- [ ] All user roles identified
- [ ] All core user flows documented
- [ ] Success metrics defined (quantifiable)
- [ ] All data entities identified
- [ ] All integrations documented
- [ ] Performance targets specified (not just "fast")
- [ ] Security requirements documented
- [ ] Compliance requirements documented
- [ ] Scalability requirements specified (user counts, data volume)
- [ ] Availability requirements specified (uptime SLA)

### Ambiguity Detection

**Watch for vague language:**
- "Fast" → Specify: < 100ms, < 500ms, < 2s
- "Scalable" → Specify: X concurrent users, Y requests/sec
- "Secure" → Specify: Authentication, authorization, encryption, compliance
- "Intuitive" → Specify: User flows, usability testing
- "Real-time" → Specify: < 100ms, < 1s, polling interval

**Red flags:**
- "Should be easy to implement" → Implies technical assumptions
- "Like [competitor]" → Requires detailed comparison
- "Users will want..." → Requires validation with actual users
- "Obviously..." → Indicates unstated assumption

### Testability Check

**Every requirement must be testable:**
- "User can log in" → ✅ Testable
- "System is fast" → ❌ Not testable (what is "fast"?)
- "API response < 500ms" → ✅ Testable
- "UI is user-friendly" → ❌ Not testable (subjective)
- "UI passes usability testing with 90% task completion" → ✅ Testable

---

## Common Requirement Anti-Patterns

### Anti-Pattern 1: Solution Masquerading as Requirement

**Wrong:** "The system should use React and Redux"
**Right:** "The system should provide a responsive web interface that works offline"

Technology choices belong in architecture phase, not requirements.

### Anti-Pattern 2: Vague Non-Functional Requirements

**Wrong:** "The system should be fast and scalable"
**Right:** "The system should support 10,000 concurrent users with API response times < 500ms (p95)"

All NFRs must be measurable.

### Anti-Pattern 3: Conflating User Goals with Implementation Details

**Wrong:** "User should click the 'Submit' button to process the order"
**Right:** "User should be able to place an order with selected items and shipping address"

Focus on what (user goal), not how (UI implementation).

### Anti-Pattern 4: Missing Acceptance Criteria

**Wrong:** "User can search products"
**Right:** "User can search products by keyword, see results in < 1 second, filter by category and price range"

Every requirement needs specific, testable acceptance criteria.

---

**Use this guide throughout Phase 1 (Discovery) and Phase 2 (Requirements Elicitation) to systematically extract complete, unambiguous requirements.**
