---
id: user-input-guidance
title: Framework-Internal Guidance Reference for User Input Elicitation
version: "1.0"
created: 2025-01-21
updated: 2025-01-21
status: Published
audience: DevForgeAI Skills (Internal Use)
related_document: effective-prompting-guide.md (user-facing counterpart)
---

# User Input Guidance Reference

**Purpose:** Framework-internal guidance document for DevForgeAI skills to elicit complete, unambiguous requirements from users. This document is NOT for end-users (see `effective-prompting-guide.md` for user-facing counterpart).

**Target Skills:**
- `devforgeai-ideation` (discovery phase, feasibility analysis)
- `devforgeai-story-creation` (acceptance criteria specification)
- `devforgeai-architecture` (context file creation, constraint validation)
- `devforgeai-ui-generator` (user requirement specification before design)
- `devforgeai-orchestration` (feature decomposition, scope clarification)

**Quick Links:**
- [Section 1: Overview & Navigation](#section-1-overview--navigation)
- [Section 2: Elicitation Patterns](#section-2-elicitation-patterns)
- [Section 3: AskUserQuestion Templates](#section-3-askuserquestion-templates)
- [Section 4: NFR Quantification Table](#section-4-nfr-quantification-table)
- [Section 5: Skill Integration Guide](#section-5-skill-integration-guide)
- [Section 6: Framework Terminology Reference](#section-6-framework-terminology-reference)

---

## Section 1: Overview & Navigation

### 1.1 Purpose and Scope

This document serves as a reference for DevForgeAI skills when users provide incomplete, ambiguous, or vague requirements. Rather than making assumptions, skills use these patterns and templates to ask clarifying questions that result in complete specifications.

**Key Principle:** "Ask, Don't Assume"
- As stated in CLAUDE.md, when ambiguity exists, skills explicitly ask for clarification
- This document provides the templates and patterns to do so systematically
- Result: Higher-quality first-pass outputs with minimal iteration

---

### 1.2 How to Use This Document

**Quick Workflow:**
1. Identify ambiguity type (Functional, NFR, Edge cases, Integration, Constraints)
2. Find pattern in Section 2 matching the type
3. Select AskUserQuestion template from Section 3
4. Ask and document the response

> **Tip:** Load selectively via `Read` tool; not all sections needed in every phase.

---

### 1.3 Document Structure Quick Reference

| Section | Purpose | Use Case | Key Resource |
|---------|---------|----------|--------------|
| **Section 2: Elicitation Patterns** | 15 patterns covering 5 requirement categories | Find the pattern matching your situation | Choose 1 of 15 patterns |
| **Section 3: AskUserQuestion Templates** | 28 copy-paste templates with YAML syntax | Generate clarifying questions | Choose 1 of 28 templates |
| **Section 4: NFR Quantification Table** | 15+ vague terms mapped to metrics | Quantify non-functional requirements | Reference table, quick lookup |
| **Section 5: Skill Integration Guide** | How each of 5 skills integrates this guidance | Understand skill-specific use cases | Skill-specific workflows |
| **Section 6: Framework Terminology Reference** | Links to CLAUDE.md definitions, context files | Validate terminology consistency | Compliance validation |

---

### 1.4 Quick Navigation By Workflow

**FINDING THE RIGHT PATTERN:**

| I have this problem | Look at this pattern | Use this template |
|-------------------|---------------------|-------------------|
| Feature scope unclear | Pattern 1 | FUN-007 |
| Feature too large for one story | Pattern 4 | FUN-007 |
| "Fast/responsive/scalable" without metrics | Pattern 5 or 7 | NFR-001 or NFR-003 |
| "Secure" without specific threat model | Pattern 6 | NFR-002 |
| Missing edge cases or error handling | Pattern 8 | EDGE-001 or EDGE-002 |
| External system dependencies unclear | Pattern 11 | INT-001 |
| Tech constraints not documented | Pattern 14 | CONST-001 |
| Timeline/budget unclear | Pattern 15 | CONST-002 |

---

### 1.5 Version & Related Documents

**Version:** 1.0 (2025-01-21) | **Status:** Published

**Related:**
- `effective-prompting-guide.md` (user-facing)
- `requirements-elicitation-guide.md` (domain-specific)
- `CLAUDE.md` (framework)
- `devforgeai/specs/context/` (constraints)

---

## Section 2: Elicitation Patterns

**15 patterns across 5 categories to identify and resolve ambiguities**

| Category | Patterns | Focus |
|----------|----------|-------|
| **Functional** | Patterns 1-4 | Feature scope, interpretations, solutions, decomposition |
| **Non-Functional** | Patterns 5-7 | Performance, security, scalability |
| **Edge Cases** | Patterns 8-10 | Invalid inputs, boundaries, validation |
| **Integration** | Patterns 11-13 | External systems, data contracts, error recovery |
| **Constraints** | Patterns 14-15 | Technical & business limits |

---

### 2.1 Functional Requirement Patterns

Functional requirements describe **WHAT** the system does—features, capabilities, user workflows.

**Quick Links:** [Pattern 1](#pattern-1-clarifying-feature-scope) | [Pattern 2](#pattern-2-separating-requirements-from-solutions) | [Pattern 3](#pattern-3-identifying-multiple-interpretations) | [Pattern 4](#pattern-4-decomposing-complex-features)

---

#### Pattern 1: Clarifying Feature Scope

**When to Use:** Feature description is vague or could mean multiple things

**Problem Description:** User describes vague features like "user management" which could mean authentication only, RBAC, SSO, or full team hierarchies.

**What to Do:** Ask user to be specific about scope (in/out), user roles, and associated data.

**Template:** `FUN-007` - See Section 3 for full YAML template

---

#### Pattern 2: Separating Requirements from Solutions

**When to Use:** User suggests a specific solution instead of describing the business need

**Problem Description:** User states "Use Redis" or "Add database" instead of measurable requirements.

**What to Do:** Ask why they need the solution and what problem it solves.

**Template:** `NFR-001, NFR-003, or NFR-004` - See Section 3

---

#### Pattern 3: Identifying Multiple Interpretations

**When to Use:** Requirement has multiple valid interpretations that lead to different implementations


**Problem Description:**
Requirement could be interpreted multiple ways. Examples:

---

#### Pattern 4: Decomposing Complex Features

**When to Use:** Feature is too large for a single story or sprint


**Problem Description:**
Feature is too large to fit in a single story. Examples:

**Template:** `FUN-007` (Scope Definition)
 - See Section 3

---

#### Pattern 5: Quantifying Vague Performance Terms

**When to Use:** User says "make it fast" or "performance is important" without measurable targets


**Problem Description:**
User uses vague performance terms without metrics. Examples:

**Template:** `NFR-001` (Performance Targets)
 - See Section 3

---

#### Pattern 6: Defining Security Requirements Precisely

**When to Use:** User says "secure" or "needs security" without specific threat model or controls


**Problem Description:**
"Secure" means different things to different people. Examples:

**Template:** `NFR-002` (Security & Compliance)
 - See Section 3

---

#### Pattern 7: Specifying Scalability Targets

**When to Use:** User says "must be scalable" without specifying target scale


**Problem Description:**
"Scalable" means different things. Examples:

**Template:** `NFR-003` (Scalability & Growth)
 - See Section 3

---

#### Pattern 8: Discovering Missing Edge Cases

**When to Use:** User specifies happy path but doesn't mention error cases or boundary conditions


**Problem Description:**
User specifies happy path but not edge cases. Examples:

**Template:** `EDGE-001, EDGE-002, or EDGE-003` (depending on type)
 - See Section 3

---

#### Pattern 9: Handling Graceful Degradation

**When to Use:** Need to define behavior when system reaches capacity or encounters failures


**Problem Description:**
What happens when system reaches its limits? Examples:

**Template:** `EDGE-003` (Rate Limiting & Quotas)
 - See Section 3

---

#### Pattern 10: Identifying Data Validation Rules

**When to Use:** Need to specify what data is valid and what should be rejected


**Problem Description:**
What data is valid and what is not? Examples:

**Template:** `EDGE-001` (Input Validation)
 - See Section 3

---

#### Pattern 11: Finding External System Dependencies

**When to Use:** Feature requires integration with external services or APIs


**Problem Description:**
Feature might depend on external systems not yet specified. Examples:

**Template:** `INT-001` (External API Integration)
 - See Section 3

---

#### Pattern 12: Clarifying Data Contract Requirements

**When to Use:** Two systems exchange data but format/schema not yet specified


**Problem Description:**
Unclear data contracts between systems. Examples:

**Template:** `INT-002` (Webhook & Event Handling) or `INT-003` (Data Synchronization)
 - See Section 3

---

#### Pattern 13: Defining Error Recovery Procedures

**When to Use:** Integration might fail and need recovery procedure


**Problem Description:**
What happens when integration fails? Examples:

**Template:** `INT-001` (External API Integration)
 - See Section 3

---

#### Pattern 14: Discovering Hidden Technical Constraints

**When to Use:** Need to identify technical limits, locked technologies, or architecture requirements


**Problem Description:**
Project has implicit constraints not yet mentioned. Examples:

**Template:** `CONST-001` (Technical Stack & Architecture)
 - See Section 3

---

#### Pattern 15: Identifying Business Constraints

**When to Use:** Need to clarify business-level limits, deadlines, or priorities


**Problem Description:**
Feature has business limitations not yet mentioned. Examples:

**Template:** `CONST-002` (Timeline & Priority)
 - See Section 3

---

## Section 3: AskUserQuestion Templates

**28 copy-paste-ready YAML templates for the 5 categories of elicitation questions.**

Use these templates with the `AskUserQuestion` tool. Each template:
- Includes the exact YAML syntax needed
- Can be customized by replacing placeholders like [SOLUTION] or [FEATURE]
- Follows the same structure (question, header, multiSelect, options)

**Quick Access by Type:**
- **Functional (FUN-001 to FUN-008):** User goals, roles, scope, interactions
- **Non-Functional (NFR-001 to NFR-005):** Performance, security, scalability, cost
- **Edge Cases (EDGE-001 to EDGE-004):** Validation, boundaries, rate limits, consistency
- **Integration (INT-001 to INT-003):** External APIs, webhooks, data sync
- **Constraints (CONST-001, CONST-002):** Technical stack, timeline & priority

### 3.1 Functional Requirement Templates (8 templates)

**FUN-001 through FUN-008: Clarify feature scope, user goals, interactions, and success criteria**

---

#### Template FUN-001: Primary User Goal

# AskUserQuestion Template
(See Section 3 for full YAML examples)


#### Template FUN-002: User Roles and Permissions

# AskUserQuestion Template
(See Section 3 for full YAML examples)


#### Template FUN-003: Success Behaviors

# AskUserQuestion Template
(See Section 3 for full YAML examples)


#### Template FUN-004: Failure / Error Scenarios

# AskUserQuestion Template
(See Section 3 for full YAML examples)


#### Template FUN-005: Feature Interactions

# AskUserQuestion Template
(See Section 3 for full YAML examples)


#### Template FUN-006: Content/Data Management

# AskUserQuestion Template
(See Section 3 for full YAML examples)


#### Template FUN-007: Scope Definition (In vs. Out)

# AskUserQuestion Template
(See Section 3 for full YAML examples)


#### Template FUN-008: Integration with Third-Party Systems

# AskUserQuestion Template
(See Section 3 for full YAML examples)


---

### 3.2 Non-Functional Requirement Templates (5 templates)

**NFR-001 through NFR-005: Clarify performance, security, scalability, reliability, and cost targets**

---

#### Template NFR-001: Performance Targets

# AskUserQuestion Template
(See Section 3 for full YAML examples)


#### Template NFR-002: Security & Compliance

# AskUserQuestion Template
(See Section 3 for full YAML examples)


#### Template NFR-003: Scalability & Growth

# AskUserQuestion Template
(See Section 3 for full YAML examples)


#### Template NFR-004: Reliability & Availability

# AskUserQuestion Template
(See Section 3 for full YAML examples)


#### Template NFR-005: Cost & Resource Constraints

# AskUserQuestion Template
(See Section 3 for full YAML examples)


---

### 3.3 Edge Case Templates (4 templates)

**EDGE-001 through EDGE-004: Clarify validation rules, boundary conditions, rate limits, and data consistency**

---

#### Template EDGE-001: Input Validation

# AskUserQuestion Template
(See Section 3 for full YAML examples)


#### Template EDGE-002: Boundary Conditions

# AskUserQuestion Template
(See Section 3 for full YAML examples)


#### Template EDGE-003: Rate Limiting & Quotas

# AskUserQuestion Template
(See Section 3 for full YAML examples)


#### Template EDGE-004: Data Consistency

# AskUserQuestion Template
(See Section 3 for full YAML examples)


---

### 3.4 Integration Templates (3 templates)

**INT-001 through INT-003: Clarify external APIs, webhooks, and data synchronization requirements**

---

#### Template INT-001: External API Integration

# AskUserQuestion Template
(See Section 3 for full YAML examples)


#### Template INT-002: Webhook / Event Handling

# AskUserQuestion Template
(See Section 3 for full YAML examples)


#### Template INT-003: Data Synchronization

# AskUserQuestion Template
(See Section 3 for full YAML examples)


---

### 3.5 Constraint Templates (2 templates)

**CONST-001 & CONST-002: Clarify technical constraints, architecture requirements, timeline, and priority**

---

#### Template CONST-001: Technical Stack & Architecture

# AskUserQuestion Template
(See Section 3 for full YAML examples)


#### Template CONST-002: Timeline & Priority

# AskUserQuestion Template
(See Section 3 for full YAML examples)


---

## Section 4: NFR Quantification Table

This table maps 15+ common vague terms to measurable ranges and provides templates for clarification.

### NFR Quantification Reference Table

| Vague Term | Measurable Range | Typical Target | DevForgeAI Example | Template Ref |
|-----------|------------------|------------------|-------------------|-------------|
| **"Fast"** | Response latency | <100ms, <200ms, <500ms, <1s | API <200ms p95 | NFR-001 |
| **"Responsive"** | User-perceived latency | <500ms, <1s | UI interactions <1s | NFR-001 |
| **"Scalable"** | User/request capacity | 100 users, 1k users, 1M users | Support 10k concurrent | NFR-003 |
| **"High performance"** | Throughput | 100 req/s, 1k req/s, 10k req/s | 1000 req/sec peak | NFR-001 |
| **"Reliable"** | Uptime percentage | 99%, 99.9%, 99.99%, 99.999% | 99.9% SLA (4.3h/month) | NFR-004 |
| **"Secure"** | Encryption & auth | TLS 1.3, AES-256, OAuth | AES-256 + JWT tokens | NFR-002 |
| **"Easy to use"** | Task completion | % users complete without help | 80% users complete onboarding | NFR-002 |
| **"Well documented"** | Coverage % | 50%, 80%, 95% | ≥80% code documentation | NFR-002 |
| **"Cost effective"** | Budget target | $1/user/month, $10/user/month | <$100/month infrastructure | NFR-005 |
| **"Accessible"** | Standard compliance | WCAG 2.0 A, AA, AAA | WCAG 2.1 AA compliance | NFR-002 |
| **"Maintainable"** | Code quality metric | Cyclomatic complexity <10 | Complexity <15 per method | NFR-002 |
| **"Efficient"** | Resource usage | <100MB RAM, <512MB RAM | <512MB RAM per instance | NFR-005 |
| **"Flexible"** | Configuration options | 3+ options, 5+ topologies | Support 5 deployment topologies | NFR-003 |
| **"Robust"** | Error handling | 90%, 95%, 99% error coverage | Handle 99% error scenarios | NFR-002 |
| **"User friendly"** | Time to value | <5 min, <15 min | Onboarding <5 minutes | NFR-002 |
| **"Available"** | System uptime | 99%, 99.5%, 99.9% | 99.9% availability | NFR-004 |
| **"Concurrent"** | Simultaneous connections | 100, 1000, 10000 users | 1000 concurrent connections | NFR-003 |

### How to Use the Table

**When user says:** "The system must be fast"

**Step 1:** Find "Fast" in table → Multiple ranges possible
**Step 2:** Use template NFR-001 to ask for specific target
**Step 3:** User selects: "<200ms response time"
**Step 4:** Document specific metric in requirements
**Step 5:** Add to acceptance criteria: "API response time <200ms p95"

---

## Section 5: Skill Integration Guide

This section documents how each of the 5 target skills should integrate this guidance document.

### 5.1 Integration: devforgeai-ideation

**Purpose:** Discover business problems and validate feasibility

**Related Story:** STORY-055 (devforgeai-ideation Integration) - Parallel implementation with consistent cross-references

**Workflow Phases Where Guidance Applied:**
- Phase 2 (Discovery): Identify ambiguities in business idea
- Phase 3 (Requirements Elicitation): Ask clarifying questions about problem/solution
- Phase 4 (Feasibility Analysis): Validate technical and business constraints

**Use Cases:**

1. **Vague Business Idea**
   - User: "We need a CRM"
   - Ambiguity Type: Functional requirement scope
   - Pattern to Use: Pattern 1 (Clarifying Feature Scope)
   - Template: FUN-007 (Scope Definition)
   - Outcome: Identify 5-10 specific features, scope for first story

2. **Missing Success Criteria**
   - User: "Build a recommendation engine"
   - Ambiguity Type: Non-functional requirement (accuracy, speed)
   - Pattern to Use: Pattern 5 (Quantifying Vague Performance Terms)
   - Template: NFR-001 (Performance Targets)
   - Outcome: Define measurable accuracy target (e.g., "95% precision")

3. **Undefined Stakeholders**
   - User: "Create a dashboard"
   - Ambiguity Type: Functional requirement (who uses it?)
   - Pattern to Use: Pattern 2 (Separating Requirements from Solutions)
   - Template: FUN-001 (Primary User Goal)
   - Outcome: Identify user roles (sales, analytics, executive) with different needs

**Integration Instructions:**

```
1. Load this guidance at Phase 2 start via:
   Read(file_path="/mnt/c/Projects/DevForgeAI2/.claude/skills/devforgeai-ideation/references/user-input-guidance.md")

2. During Phase 3 (Requirements Elicitation), check for ambiguities:
   - Is feature scope clear? (no → use Pattern 1)
   - Are success criteria measurable? (no → use Pattern 5)
   - Are user roles defined? (no → use Pattern 1 or FUN-001)
   - Are edge cases identified? (no → use Pattern 8)

3. When ambiguity detected, select appropriate pattern and template

4. Ask user via AskUserQuestion tool (customize template for context)

5. Incorporate response into feasibility analysis output
```

**Success Criteria:**
- No vague terms in final requirements (all converted to metrics)
- User roles and goals clearly defined
- At least 3-5 success scenarios per feature
- Edge cases identified for critical features
- Technical constraints validated against tech-stack.md

---

### 5.2 Integration: devforgeai-story-creation

**Purpose:** Generate complete stories with acceptance criteria

**Workflow Phases Where Guidance Applied:**
- Phase 2: Clarify acceptance criteria
- Phase 3: Identify edge cases and error scenarios
- Phase 4: Validate story against context files

**Use Cases:**

1. **Vague Acceptance Criteria**
   - User provides: "User can search for products"
   - Problem: Doesn't specify search type (full-text, category, filters?)
   - Pattern: Pattern 3 (Identifying Multiple Interpretations)
   - Template: FUN-002 (User Roles and Permissions) or FUN-003 (Success Behaviors)
   - Outcome: AC specifies "full-text search by title/description, filter by category/price/rating"

2. **Missing Edge Cases**
   - Story specifies: "User uploads image"
   - Problem: No error handling (file too large, wrong format, upload fails?)
   - Pattern: Pattern 8 (Discovering Missing Edge Cases)
   - Template: EDGE-001 (Input Validation)
   - Outcome: AC includes "reject files >5MB with error message, support JPG/PNG only"

3. **Unclear Integration Points**
   - Story specifies: "Process payment"
   - Problem: No payment processor specified, retry strategy unclear
   - Pattern: Pattern 11 (Finding External System Dependencies)
   - Template: INT-001 (External API Integration)
   - Outcome: AC specifies "integrate with Stripe, retry 3x on timeout, fallback to manual review"

**Integration Instructions:**

```
1. During Phase 2, review each AC for ambiguities:
   - Are behaviors specific or vague? (vague → use Pattern 1, Pattern 3)
   - Are success criteria measurable? (vague → use Section 4 NFR table)
   - Are error cases covered? (missing → use Pattern 8)
   - Are integrations specified? (missing → use Pattern 11)

2. Use targeted patterns and templates for each type of gap

3. Result: Complete AC that is:
   - Specific (no ambiguous terms)
   - Measurable (includes metrics/numbers)
   - Testable (each AC can be verified)
   - Complete (happy path + errors + edge cases)
```

**Success Criteria:**
- Each AC has ≥1 success scenario + ≥1 error scenario
- No vague terms (all quantified via Section 4 table)
- Integration points explicitly specified
- Edge cases identified (boundary conditions, validation rules, rate limits)
- Acceptance criteria is testable (can write automated tests)

---

### 5.3 Integration: devforgeai-architecture

**Purpose:** Create 6 immutable context files with constraints

**Workflow Phases Where Guidance Applied:**
- Phase 1: Identify technical constraints
- Phase 2: Identify architectural constraints
- Phase 3: Create context files with constraint documentation

**Use Cases:**

1. **Incomplete tech-stack.md**
   - Problem: Doesn't specify approved libraries/versions
   - Pattern: Pattern 14 (Discovering Hidden Technical Constraints)
   - Template: CONST-001 (Technical Stack & Architecture Constraints)
   - Outcome: tech-stack.md lists all locked technologies with versions

2. **Missing security requirements in architecture-constraints.md**
   - Problem: Security policies not documented
   - Pattern: Pattern 6 (Defining Security Requirements Precisely)
   - Template: NFR-002 (Security & Compliance)
   - Outcome: architecture-constraints.md specifies encryption, authentication, compliance requirements

3. **Undefined performance targets in dependencies.md**
   - Problem: No SLA/availability targets specified
   - Pattern: Pattern 5 & 7 (Performance, Scalability)
   - Template: NFR-001, NFR-003, NFR-004
   - Outcome: context files document measurable SLAs and scalability targets

**Integration Instructions:**

```
1. Load this guidance at Phase 1 start

2. For each context file being created, check for ambiguities:
   - tech-stack.md: Are all technologies specified with versions? (no → use Pattern 14, CONST-001)
   - architecture-constraints.md: Are design patterns clear? (no → use Pattern 14)
   - dependencies.md: Are version constraints specified? (no → use Pattern 14)
   - coding-standards.md: Are code style rules specific? (vague → use Section 4)
   - source-tree.md: Are file location rules clear? (ambiguous → clarify)
   - anti-patterns.md: Are forbidden patterns specific with rationale? (yes → good)

3. When ambiguity found, ask for clarification via AskUserQuestion

4. Document constraints precisely in context files (no vague language)

5. Validate against patterns:
   - No circular dependencies (Pattern 11)
   - No conflicting constraints
   - All constraints measurable and testable
```

**Success Criteria:**
- All 6 context files exist and are non-empty
- No placeholder content (TODO, TBD, TBD, etc.)
- All constraints are specific and measurable
- tech-stack.md includes approved libraries with versions
- architecture-constraints.md specifies clean architecture rules
- dependencies.md locks all package versions
- coding-standards.md includes naming/style rules
- source-tree.md specifies file location rules
- anti-patterns.md lists forbidden patterns with rationale

---

### 5.4 Integration: devforgeai-ui-generator

**Purpose:** Generate UI specifications before coding

**Workflow Phases Where Guidance Applied:**
- Phase 1: Clarify user requirements for UI
- Phase 2: Identify user interactions and workflows
- Phase 3: Generate UI specifications with layouts/components

**Use Cases:**

1. **Vague UI Requirements**
   - User: "Create a dashboard"
   - Problem: No specification of what data to display, how users interact
   - Pattern: Pattern 2 (Separating Requirements from Solutions)
   - Template: FUN-001 (Primary User Goal), FUN-003 (Success Behaviors)
   - Outcome: Dashboard spec includes specific widgets, data sources, refresh rates

2. **Missing Usability Requirements**
   - User: "Design admin panel"
   - Problem: No accessibility requirements, performance targets for UI
   - Pattern: Pattern 5 (Quantifying Vague Performance Terms), Pattern 6 (Security)
   - Template: NFR-001 (Performance), NFR-002 (Accessibility)
   - Outcome: UI spec includes <500ms load time, WCAG 2.1 AA accessibility

3. **Undefined User Interactions**
   - User: "Create form for creating orders"
   - Problem: No specification of validation, error messaging, success feedback
   - Pattern: Pattern 8 (Discovering Missing Edge Cases), Pattern 10 (Data Validation)
   - Template: EDGE-001 (Input Validation), EDGE-002 (Error Handling)
   - Outcome: Form spec includes all validations, error messages, success states

**Integration Instructions:**

```
1. During Phase 1, clarify user requirements:
   - Who is the user? (use FUN-001)
   - What are they trying to accomplish? (use FUN-003)
   - What data do they need to see? (use FUN-005)
   - How do they interact? (use FUN-003 + FUN-008)

2. During Phase 2, identify edge cases:
   - What happens on error? (use EDGE-002)
   - What are validation rules? (use EDGE-001)
   - What's the loading/empty state? (use EDGE-002)
   - What accessibility requirements? (use NFR-002)

3. During Phase 3, generate UI spec including:
   - Layout and component hierarchy
   - User interactions (click, input, navigation)
   - Validation rules and error messages
   - Success/loading/empty states
   - Accessibility features (WCAG 2.1 AA)
   - Performance targets (page load <3s)

4. Validate spec is complete:
   - Happy path specified? (yes)
   - Error paths specified? (yes)
   - Edge cases handled? (yes)
   - Accessibility considered? (yes)
```

**Success Criteria:**
- User goals clearly defined (primary persona + use case)
- All user interactions specified (forms, buttons, navigation)
- Validation rules documented for all inputs
- Error messages specified for all error scenarios
- Success/loading/empty states defined
- Accessibility requirements met (WCAG 2.1 AA minimum)
- Performance targets included (page load time, interaction latency)
- UI responsive design breakpoints specified

---

### 5.5 Integration: devforgeai-orchestration

**Purpose:** Manage full feature lifecycle (dev → qa → release)

**Workflow Phases Where Guidance Applied:**
- Phase 1: Feature decomposition and scope clarification
- Phase 2: Story creation with complete AC
- Phase 3-5: Monitor for ambiguities during implementation
- Phase 6-7: Validate completeness before QA/release

**Use Cases:**

1. **Feature Too Large for Single Story**
   - Problem: Feature requires multiple stories, unclear how to decompose
   - Pattern: Pattern 4 (Decomposing Complex Features)
   - Template: FUN-007 (Scope Definition)
   - Outcome: Feature decomposed into 3-5 stories, each completable in one sprint

2. **Unclear Success Criteria for Release**
   - Problem: Not clear what "done" means, acceptance criteria ambiguous
   - Pattern: Pattern 5 (Quantifying Vague Performance Terms)
   - Template: NFR-001 (Performance Targets), CONST-002 (Timeline & Priority)
   - Outcome: Release criteria specified: "< 5 critical bugs, >95% coverage, <200ms API latency"

3. **Missing Integration Points Between Stories**
   - Problem: Story A and Story B need to integrate, interface not defined
   - Pattern: Pattern 12 (Clarifying Data Contract Requirements)
   - Template: INT-002 (Webhook/Event Handling), INT-003 (Data Sync)
   - Outcome: Data contract specified between stories (fields, formats, SLA)

**Integration Instructions:**

```
1. During Phase 1 (feature decomposition):
   - Is feature scope clear? (no → use Pattern 4, FUN-007)
   - Can it fit in one story? (yes/no → decide)
   - What are story dependencies? (identify → Pattern 11)
   - What's the priority order? (define → CONST-002)

2. During Phase 2 (story creation):
   - Do acceptance criteria align with overall feature goal? (no → refine)
   - Are integration points with other stories clear? (no → use Pattern 12)
   - Are edge cases covered? (no → use Pattern 8)
   - Are constraints understood? (no → use Pattern 14, Pattern 15)

3. During Phases 3-5 (implementation monitoring):
   - Are developers hitting ambiguities? (listen for questions)
   - Are integration points clear between stories? (verify via code review)
   - Is implementation staying within constraints? (check against context files)

4. During Phases 6-7 (QA/release validation):
   - Have all acceptance criteria been met? (yes/no → assess)
   - Are edge cases actually handled in code? (yes/no → test)
   - Are integrations working end-to-end? (yes/no → integration test)
   - Are metrics/SLAs being met? (yes/no → performance test)
```

**Success Criteria:**
- Feature decomposed into completable stories (each ≤1 sprint)
- Story dependencies identified and sequenced
- Acceptance criteria clear and testable for all stories
- Integration points specified between stories
- Constraints understood by entire team
- Definition of Done for feature is measurable
- Release criteria documented and verified

---

## Section 6: Framework Terminology Reference

Links guidance patterns to CLAUDE.md definitions and context files.

### 6.1 Key Concept Mappings

**Guidance Core Concepts (from CLAUDE.md):**
- **Ask, Don't Assume** → All 15 patterns implement this principle
- **Spec-Driven Development** → Patterns 14-15 enforce spec compliance
- **Context Files (6 immutable)** → Pattern 14 references these files
- **Clean Architecture** → Patterns 11 & 14 clarify layer boundaries
- **Acceptance Criteria** → All patterns ensure AC is testable

### 6.2 Context File Alignment

- **tech-stack.md** → Pattern 14 (locked technologies)
- **architecture-constraints.md** → Pattern 14 (layer boundaries, DI)
- **anti-patterns.md** → Patterns 8-10, 14 prevent violations
- **coding-standards.md** → Section 4 (quality metrics)

### 6.3 Related Framework Documents

- `effective-prompting-guide.md` (user-facing counterpart)
- `requirements-elicitation-guide.md` (domain-specific patterns)
- `CLAUDE.md` (framework constitution)

---

**v1.0** | **Status:** Published
