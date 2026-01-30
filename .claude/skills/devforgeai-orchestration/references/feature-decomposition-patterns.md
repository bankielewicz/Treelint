# Feature Decomposition Patterns

Guide for breaking down epics into independently valuable features that map to user-facing capabilities.

---

## Overview

Feature decomposition is the critical bridge between high-level epics (business initiatives spanning 2-6 sprints) and detailed user stories (work units implementable in 1-3 days). This reference guide helps the requirements-analyst subagent and epic creators systematically decompose epics into well-scoped features.

**Key Principle:** Each feature should be independently implementable, incrementally valuable, and mappable to 3-8 user stories.

---

## Decomposition Methodology

### Core Principles

**1. Independence Principle**
- Features should have minimal dependencies on each other
- Ideally, features can be developed and released independently
- If Feature B blocks Feature A, reconsider the decomposition

**2. Incremental Value Principle**
- Each feature delivers measurable user value
- Users can adopt and benefit from features incrementally
- Avoid technical-only features (e.g., "Database schema updates")

**3. Right-Sizing Principle**
- Each feature fits within 1-2 sprints of work (15-25 story points)
- If larger → decompose further
- If smaller → combine with related feature

**4. Testability Principle**
- Each feature has clear acceptance criteria
- Features can be demonstrated independently
- Coverage and quality can be measured per-feature

### Decomposition Process (5 Steps)

**Step 1: Extract Capabilities from Epic Goal**

```
Epic Goal: "Improve checkout experience to increase conversion rate"

Capabilities:
- Allow guests to check out without account
- Let users save payment methods
- Display progress through checkout steps
- Support multiple currencies
- Recover from checkout errors gracefully
```

**Step 2: Group Related Capabilities**

```
Feature 1: Guest Checkout
├─ Guest checkout form (minimal data)
├─ Guest account creation option
└─ Email confirmation of order

Feature 2: Payment Method Management
├─ Save payment methods to account
├─ Select from saved methods
├─ Update/delete payment methods
└─ Add new payment method during checkout

Feature 3: Checkout Experience
├─ Visual progress indicator
├─ Clear step labels and guidance
├─ Error recovery with context
└─ Order summary at each step

Feature 4: Multi-Currency Support
├─ Detect user location/preference
├─ Display prices in selected currency
├─ Real-time exchange rate updates
└─ Payment processing in multiple currencies
```

**Step 3: Validate Independence**

Check dependency matrix:
```
                 F1   F2   F3   F4
Feature 1 (Guest) -   Hard Soft  Soft
Feature 2 (Payment) Soft  -   Soft  Hard
Feature 3 (UX)    Soft  Soft  -    Soft
Feature 4 (Multi)  Soft  Hard  Soft  -

Key: Hard = blocks other feature, Soft = nice-to-have ordering
```

**If many Hard dependencies:** Reconsider grouping

**Step 4: Right-Size Each Feature**

Estimate story count per feature:
```
Feature 1 (Guest): 4 stories, 10 points, 1 sprint
Feature 2 (Payment): 5 stories, 13 points, 1-2 sprints
Feature 3 (UX): 4 stories, 8 points, 1 sprint
Feature 4 (Multi): 4 stories, 12 points, 1-2 sprints

Total: 17 stories, 43 points, 3 sprints
```

**If feature is 20+ points → split further**

**Step 5: Define Feature Acceptance Criteria**

For each feature, define what "done" looks like:
```
Feature 1: Guest Checkout
[ ] Guest checkout form displays without login
[ ] Email validation works (confirmation email sent)
[ ] Order confirmation page shows for guests
[ ] Guest can track order without account
[ ] All at <3s page load time
```

---

## Decomposition Patterns by Epic Type

### 1. CRUD Application Epics

**Pattern:** Entity management features covering Create, Read, Update, Delete, List, Search operations

**Example Epic:** User Management System

**Decomposition:**
```
Feature 1: User Creation & Registration
├─ User registration form with validation
├─ Email verification workflow
├─ Initial profile setup
└─ Account activation

Feature 2: User Profile Management
├─ View user profile information
├─ Edit personal information
├─ Change password
└─ Update profile picture

Feature 3: User Search & Directory
├─ Search users by name/email
├─ Filter by department/role
├─ Browse user directory
└─ View user details

Feature 4: User Deactivation & Cleanup
├─ Deactivate user account
├─ Archive user data
├─ Reassign user responsibilities
└─ Account recovery option

Feature 5: User Permissions Management
├─ Assign roles to users
├─ Grant/revoke permissions
├─ View user permissions
└─ Audit permission changes
```

**Complexity:**
- Feature 1: Moderate (email integration required)
- Features 2-3: Simple (CRUD operations)
- Features 4-5: Moderate (business logic complex)

**Duration:** 3-4 sprints

---

### 2. Authentication/Authorization Epics

**Pattern:** Identity features covering registration, authentication, session management, permission models

**Example Epic:** Enterprise User Authentication

**Decomposition:**
```
Feature 1: Basic Email/Password Authentication
├─ User registration with email
├─ Login with credentials
├─ Password reset via email
└─ Session management

Feature 2: Multi-Factor Authentication
├─ TOTP (authenticator app) setup
├─ Backup codes generation
├─ MFA enforcement policies
└─ Device trust management

Feature 3: Single Sign-On (SSO) Integration
├─ OAuth 2.0 provider integration (Google, Microsoft)
├─ SAML 2.0 support for enterprises
├─ Automatic account linking
└─ SSO session management

Feature 4: Role-Based Access Control (RBAC)
├─ Define system roles
├─ Assign roles to users
├─ Permission inheritance hierarchy
└─ Role-based view filtering

Feature 5: Advanced Authorization
├─ Attribute-Based Access Control (ABAC)
├─ Fine-grained permission management
├─ Time-based access restrictions
└─ Resource-level permissions
```

**Complexity:**
- Feature 1: Moderate (security critical)
- Feature 2: High (integration with TOTP libraries)
- Feature 3: High (external provider APIs)
- Features 4-5: Moderate-High (complex business rules)

**Duration:** 4-5 sprints (security-critical, requires thorough testing)

**Framework Integration:**
- MUST check anti-patterns.md for forbidden patterns (hardcoded secrets, weak cryptography)
- MUST check tech-stack.md for approved authentication libraries
- Security-auditor subagent required for validation

---

### 3. API Development Epics

**Pattern:** REST/GraphQL endpoint features covering resources, operations, versioning, documentation

**Example Epic:** REST API v2 Development

**Decomposition:**
```
Feature 1: Core Resource Endpoints
├─ GET /users (list users with pagination)
├─ GET /users/{id} (fetch user details)
├─ POST /users (create new user)
├─ PUT /users/{id} (update user)
├─ DELETE /users/{id} (delete user)
└─ Error handling and validation

Feature 2: Advanced Querying
├─ Filter by field values
├─ Sort by multiple fields
├─ Search full-text capability
├─ Partial responses (field selection)
└─ Cursor-based pagination

Feature 3: API Versioning & Compatibility
├─ Version routing (/v2/users vs /v1/users)
├─ Deprecation headers for v1
├─ Backward compatibility layer
└─ Migration guide documentation

Feature 4: API Documentation & Discovery
├─ OpenAPI/Swagger specification
├─ API documentation portal
├─ Interactive API explorer
└─ SDK/client library generation

Feature 5: API Security & Rate Limiting
├─ API key authentication
├─ OAuth 2.0 token validation
├─ Rate limiting per client
└─ DDoS protection headers
```

**Complexity:**
- Feature 1: Simple (standard REST patterns)
- Feature 2: Moderate (query parser complexity)
- Feature 3: Moderate (backward compatibility logic)
- Feature 4: Simple (documentation generation)
- Feature 5: High (security implementation)

**Duration:** 3-4 sprints

**Framework Integration:**
- api-designer subagent validates API contract in Feature 1
- security-auditor validates Feature 5 against OWASP

---

### 4. Reporting/Analytics Epics

**Pattern:** Data features covering collection, processing, visualization, export

**Example Epic:** Analytics Dashboard

**Decomposition:**
```
Feature 1: Data Collection & Instrumentation
├─ Event tracking framework
├─ Page view analytics
├─ User action logging
├─ Event validation and schema
└─ Real-time event ingestion

Feature 2: Data Processing & Aggregation
├─ Daily aggregations (rollups)
├─ Real-time metrics calculation
├─ Data deduplication
├─ Historical data retention policies
└─ Data quality checks

Feature 3: Dashboard Visualization
├─ Key metrics widgets (KPIs)
├─ Time-series charts
├─ Funnel analysis charts
├─ Custom dashboard layouts
└─ Drill-down capabilities

Feature 4: Report Generation & Export
├─ Scheduled report generation
├─ Export to CSV/PDF/Excel
├─ Email delivery of reports
├─ Custom report builder
└─ Historical report archives

Feature 5: Advanced Analytics
├─ Cohort analysis
├─ User segmentation
├─ Retention analysis
└─ Predictive trending
```

**Complexity:**
- Feature 1: Moderate (event schema design)
- Feature 2: High (data pipeline complexity)
- Feature 3: Moderate (charting library integration)
- Feature 4: Simple (report formatting)
- Feature 5: High (statistical analysis)

**Duration:** 4-5 sprints

**Framework Integration:**
- Dependencies: Feature 1 must complete before Feature 2
- Feature 2 prerequisite for Features 3-5

---

### 5. Workflow/Process Epics

**Pattern:** Stage features covering initiation, processing, approval, completion

**Example Epic:** Approval Workflow System

**Decomposition:**
```
Feature 1: Request Creation & Submission
├─ Request form with validation
├─ Attachment support
├─ Request draft saving
├─ Notification to approvers
└─ Request confirmation email

Feature 2: Approval Process Management
├─ View pending approvals
├─ Approve/reject with comments
├─ Multi-level approval routing
├─ Approval history tracking
└─ Comment/discussion threads

Feature 3: Escalation & Delegation
├─ Escalation to manager
├─ Delegation to colleague
├─ Escalation timeout rules
├─ Delegation restrictions
└─ Audit trail of delegations

Feature 4: Completion & Documentation
├─ Auto-execution after approval
├─ Completion notification
├─ Delivery/implementation tracking
├─ Completion feedback form
└─ Archive approved requests

Feature 5: Reporting & Analytics
├─ Approval metrics dashboard
├─ Bottleneck identification
├─ SLA tracking and alerts
├─ Export approval reports
└─ Trend analysis
```

**Complexity:**
- Feature 1: Simple (form processing)
- Feature 2: Moderate (workflow engine integration)
- Feature 3: Moderate-High (routing rules)
- Feature 4: Simple (post-approval actions)
- Feature 5: Moderate (reporting logic)

**Duration:** 3-4 sprints

**Dependencies:**
- Feature 1 → Feature 2 (cannot approve before creating)
- Feature 2 → Features 3-4 (sequential workflow stages)

---

### 6. E-Commerce Epics

**Pattern:** Transaction features covering catalog, cart, checkout, payment, fulfillment

**Example Epic:** E-Commerce Platform

**Decomposition:**
```
Feature 1: Product Catalog Management
├─ Add/edit/delete products
├─ Product categorization
├─ Inventory management
├─ Product variants (size, color)
└─ Stock status indicators

Feature 2: Shopping Cart & Wishlist
├─ Add/remove items from cart
├─ Update quantities
├─ Wishlist functionality
├─ Cart persistence across sessions
└─ Cart sharing feature

Feature 3: Checkout Flow
├─ Shipping address entry
├─ Shipping method selection
├─ Billing address (same/different)
├─ Order review page
└─ Order confirmation

Feature 4: Payment Processing
├─ Credit/debit card payments
├─ PayPal integration
├─ Stripe payment gateway
├─ PCI compliance
└─ Payment error handling

Feature 5: Order Management & Fulfillment
├─ Order status tracking
├─ Shipment notifications
├─ Delivery tracking integration
├─ Return request processing
└─ Return shipping labels

Feature 6: Customer Account Management
├─ Order history
├─ Saved addresses
├─ Saved payment methods
├─ Account preferences
└─ Download invoices
```

**Complexity:**
- Feature 1: Simple (CRUD operations)
- Feature 2: Simple-Moderate (session management)
- Feature 3: Moderate (multi-step form validation)
- Feature 4: High (payment integration, security-critical)
- Feature 5: Moderate (external shipping API)
- Feature 6: Simple (user data management)

**Duration:** 4-5 sprints

**Dependencies:**
- Feature 1 → Features 2-3 (need products before shopping)
- Feature 3 → Feature 4 (checkout before payment)
- Feature 4 → Feature 5 (payment before fulfillment)

**Framework Integration:**
- Feature 4: MUST check tech-stack.md for approved payment gateways
- Feature 4: security-auditor validation (PCI compliance)
- Feature 5: check dependencies.md for shipping integrations

---

## Feature Characteristics Matrix

### What Makes a Good Feature

| Characteristic | Good ✅ | Poor ❌ |
|---|---|---|
| **Scope** | 1-2 sprints (13-20 pts) | <5 pts or >25 pts |
| **Value** | User-facing benefit | Technical-only work |
| **Independence** | Can be developed separately | Tightly coupled |
| **Testing** | Clear acceptance criteria | Vague success conditions |
| **Demonstration** | Shippable demo possible | Requires other features |
| **Documentation** | User story per capability | Abstract requirements |

### Feature Size Guidelines

**Small Feature (1 Sprint, 8-13 Points)**
- Affects single entity type (Users, Products, Orders)
- Single view/UI component
- Minimal external integrations
- Standard error handling
- Example: "Edit user profile"

**Medium Feature (1-2 Sprints, 13-21 Points)**
- Affects 2-3 entity types with relationships
- Multi-step workflow (3-4 steps)
- One external integration (payment, SMS, etc.)
- Custom business logic
- Example: "Complete checkout with payment"

**Large Feature (2-3 Sprints, 21-30 Points)**
- Affects 4+ related entity types
- Complex workflow with branching (5+ steps)
- Multiple external integrations
- Advanced search/filtering
- Example: "Approval workflow with routing"

**Very Large (>30 Points) - SHOULD SPLIT**
- Likely over-scoped
- Consider breaking into smaller features
- May indicate multiple epics needed

---

## Framework Integration Checklist

### Before Finalizing Feature List

**✅ Technology Alignment**
- [ ] Proposed technologies appear in tech-stack.md (or approved for ADR)
- [ ] External integrations in dependencies.md (payment gateways, APIs, etc.)
- [ ] No unapproved library substitutions

**✅ Architecture Compliance**
- [ ] Features respect architecture-constraints.md layer boundaries
- [ ] No cross-layer violations (Domain → Infrastructure, etc.)
- [ ] Integration points clearly defined

**✅ Anti-Pattern Prevention**
- [ ] No God Object features (single 500+ line class)
- [ ] No direct instantiation (DI required)
- [ ] No hardcoded secrets or SQL concatenation
- [ ] No copy-paste patterns

**✅ Security Considerations**
- [ ] Features handling sensitive data flagged for security-auditor
- [ ] Authentication/authorization requirements documented
- [ ] PCI/GDPR/compliance requirements noted

**✅ Dependency Mapping**
- [ ] Feature dependencies form valid DAG (no cycles)
- [ ] Critical path clearly identified
- [ ] Parallel-able features isolated

---

## Feature Naming Conventions

### What Makes Good Feature Names

**Pattern:** `[Action] [Subject] [with Context]`

**Good Names (User-Focused, Action-Oriented):**
- ✅ "User Registration with Email Verification"
- ✅ "Add Items to Shopping Cart"
- ✅ "Guest Checkout Without Account"
- ✅ "Save Payment Methods for Reuse"
- ✅ "Track Shipment Status in Real-Time"
- ✅ "Request Approval with Multi-Level Routing"
- ✅ "Search Products with Filters and Sorting"

**Bad Names (Technical, Vague, Passive):**
- ❌ "Backend API Endpoints" (technical, no user benefit)
- ❌ "Database Schema Updates" (implementation detail)
- ❌ "Refactor Authentication Module" (not a feature)
- ❌ "Improve Performance" (vague, not specific)
- ❌ "Fix Bugs in Checkout" (bug fix, not feature)
- ❌ "Integrate Payment Gateway" (implementation-focused)
- ❌ "Build Reporting System" (too large, not specific)

### Naming Rules

1. **Start with user action verb**: Add, View, Create, Update, Search, Request, Track, etc.
2. **Include subject/object**: What is being acted upon (Cart, Payment, User, Order)
3. **Add context if needed**: "with email verification", "in real-time", "across devices"
4. **No technical jargon**: Users don't care about APIs, schemas, or databases
5. **Keep concise**: 3-8 words maximum

---

## Complexity Estimation Framework

### Simple Features (1 Sprint, 8-13 Points)

**Characteristics:**
- Uses existing technology only
- Single entity type
- Minimal external integrations
- Standard patterns
- No architectural changes

**Examples:**
- CRUD operations on single entity (Users, Products)
- Read-only views/reports
- Configuration/settings changes
- Basic validation forms

**Tech Stack Check:**
- All technologies in tech-stack.md
- No new libraries or frameworks

---

### Moderate Features (1-2 Sprints, 13-21 Points)

**Characteristics:**
- Mostly existing technology
- 2-3 entity types with relationships
- One external integration required
- Custom business logic (2-3 business rules)
- No architecture changes

**Examples:**
- Multi-entity CRUD workflows
- Payment processing
- Basic reporting (2-3 metrics)
- Simple approval workflows (2-level)

**Tech Stack Check:**
- May require 1 new library (must be in dependencies.md or approved)
- No significant architectural changes

---

### Complex Features (2-3 Sprints, 21-30 Points)

**Characteristics:**
- May require 1-2 new technologies
- 4+ entity types with complex relationships
- 2-3 external integrations
- Advanced business logic (5+ business rules)
- Minor architecture extensions (new service, API endpoint)

**Examples:**
- Complex workflows with branching/routing
- Real-time analytics dashboards
- Multi-vendor payment integration
- Advanced permission systems

**Tech Stack Check:**
- May require new technology (needs ADR)
- May require new service/component
- architect-reviewer validation required

---

### Very High Complexity (>30 Points) - LIKELY OVER-SCOPED

**Do Not Use as Single Feature**

**Action:**
1. Break into 2-3 smaller features
2. Identify critical path features
3. Consider if this requires multiple epics

**Examples of Over-Scoped Features:**
- "Build complete e-commerce platform" (multiple epics needed)
- "Implement microservices architecture" (multiple epics)
- "Migrate legacy system with 0 downtime" (major epic)

---

## Common Decomposition Mistakes & Corrections

### Mistake 1: Vertical Slices (Technical Stack) Instead of Horizontal (User Value)

❌ **Wrong Decomposition:**
```
Feature 1: Database Schema Design
Feature 2: Backend API Implementation
Feature 3: Frontend UI Components
Feature 4: Integration Testing
Feature 5: Deployment Automation
```
*Problem: No feature delivers user value independently; all features required for single capability*

✅ **Correct Decomposition:**
```
Feature 1: User Registration with Email Verification
Feature 2: User Profile Management
Feature 3: User Search Functionality
Feature 4: Permission-Based Access Control
Feature 5: User Activity Auditing
```
*Each feature delivers distinct user value*

---

### Mistake 2: All-or-Nothing Dependencies

❌ **Wrong:**
```
Feature 1: Payment Gateway Setup
Feature 2: Checkout Flow
Feature 3: Shipping Integration
Feature 4: Order Fulfillment

All depend on all predecessors
```
*Problem: Cannot release Feature 2 without Features 3-4*

✅ **Correct:**
```
Feature 1: Basic Checkout Flow (hardcoded shipping, mock payment)
Feature 2: Payment Integration (upgrade from mock)
Feature 3: Real Shipping Methods (upgrade from hardcoded)
Feature 4: Order Fulfillment Automation

Can release Feature 1 as MVP with Feature 2-4 as enhancements
```

---

### Mistake 3: Features Too Large

❌ **Wrong:**
```
Feature: "Complete User Management System" (50+ points)
├─ Registration, login, profile, permissions, audit
└─ 8+ sprints to complete
```
*Problem: Large features are hard to estimate, test, and demonstrate*

✅ **Correct:**
```
Feature 1: User Registration (8 points)
Feature 2: User Profile Management (8 points)
Feature 3: User Permissions (13 points)
Feature 4: User Activity Audit (8 points)

Each feature independently shippable
```

---

### Mistake 4: Features Too Small

❌ **Wrong:**
```
Feature 1: Add Email Field to Form (2 points)
Feature 2: Add Validation to Email (2 points)
Feature 3: Send Confirmation Email (2 points)
Feature 4: Verify Email Token (2 points)

Total: 4 features, 8 points (should be 1 feature)
```
*Problem: Overhead of managing 4 separate feature tracks*

✅ **Correct:**
```
Feature: User Registration with Email Verification (8 points)
├─ Email field in form
├─ Email validation
├─ Confirmation email sending
└─ Token verification

All in one coherent feature
```

---

### Mistake 5: Technical Features, Not User-Facing

❌ **Wrong:**
```
Feature 1: Refactor Authentication Module
Feature 2: Migrate to New ORM
Feature 3: Optimize Database Queries
Feature 4: Upgrade Dependencies
```
*Problem: No direct user value; users don't care about technical work*

✅ **Correct:**
```
Feature 1: Login with Single Sign-On (user value: convenience)
Feature 2: Reduce Login Time by 50% (user value: performance)
Feature 3: Remember Device for 30 Days (user value: less logins)

Technical work supports these user features but shouldn't be separate features
```

---

## Output Format for Requirements-Analyst Subagent

When decomposing an epic into features, provide structured output:

```markdown
## Epic Decomposition: [Epic Name]

### Overall Structure
- **Total Features**: [N] features
- **Total Estimated Points**: [Total] story points
- **Projected Duration**: [N] sprints
- **MVP Features**: [List features for minimum viable product]

### Feature List

### Feature 1: [Feature Name]

**Description**: [1-2 sentences of what users can do]

**User Value**: [Benefit to end user, not technical benefit]

**Complexity**: [Simple / Moderate / Complex]

**Estimated Points**: [Story points - use Fibonacci: 3, 5, 8, 13, 20]

**Estimated Duration**: [1 sprint / 1-2 sprints / 2-3 sprints]

**Dependencies**: [None / List prerequisite features with brief reason]

**Technology Requirements**: [Any new tech? Check against tech-stack.md]

**Architecture Impact**: [None / Describe any architectural changes needed]

**Acceptance Criteria Summary**: [2-3 high-level success criteria]

**Stories Needed**: [Estimated count: 3-6 stories per feature]

---

### Feature [N]: [Feature Name]

[Repeat structure above]

---

### Feature Dependencies Diagram

```
Feature 1 (Registration)
  ↓
Feature 2 (Profile) ← Feature 3 (Search)
  ↓
Feature 4 (Permissions)
  ↓
Feature 5 (Audit)
```

### MVP Release Plan

**Phase 1 (Sprint 1):** Features [1-2] - Core functionality
**Phase 2 (Sprint 2):** Features [3-4] - Enhanced capabilities
**Phase 3 (Sprint 3):** Feature [5] - Advanced features

### Technical Assessment

**Technology Alignment:**
- [ ] All proposed technologies in tech-stack.md
- [ ] External integrations in dependencies.md
- [ ] No library substitutions

**Architecture Compliance:**
- [ ] Respects architecture-constraints.md layers
- [ ] No cross-layer violations
- [ ] Clear integration points

**Anti-Pattern Check:**
- [ ] No forbidden patterns from anti-patterns.md
- [ ] No God Objects (features split accordingly)
- [ ] Clear DI rather than direct instantiation

### Context File References

[List any relevant context files and how features align with them]
```

---

## References for Subagent Invocation

**Use this guide when:**
- Breaking down epic into 3-8 features
- Validating feature independence and sizing
- Estimating feature complexity
- Planning MVP vs extended releases
- Checking framework alignment (tech-stack, architecture, anti-patterns)

**Self-validation before finalizing:**
- [ ] 3-8 features per epic (optimal range)
- [ ] Each feature 1-2 sprints (8-21 points)
- [ ] Features form valid DAG (no circular dependencies)
- [ ] Each feature has distinct user value
- [ ] Features respect tech-stack.md constraints
- [ ] Features comply with architecture-constraints.md
- [ ] No forbidden patterns from anti-patterns.md

**When to escalate:**
- Feature >25 points → suggest split
- Feature <3 points → suggest merge with related feature
- Circular dependencies → block, require redesign
- Technology not in tech-stack.md → flag for ADR
- Architecture violation → block, require redesign

---

**Last Updated:** 2025-11-05
**Version:** 1.0
**Framework:** DevForgeAI Orchestration
