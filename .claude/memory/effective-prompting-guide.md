---
id: effective-prompting-guide
title: Effective Prompting Guide for DevForgeAI
version: "1.0"
created: 2025-01-20
updated: 2025-01-20
status: Published
---

# Effective Prompting Guide for DevForgeAI

## Table of Contents

- [Introduction](#introduction)
- [Quick Reference Checklist](#quick-reference-checklist)
- [Core Principles](#core-principles)
- [Command-Specific Guidance](#command-specific-guidance)
  - [/ideate](#ideate-transform-business-ideas-into-requirements)
  - [/create-context](#create-context-generate-architectural-constraints)
  - [/create-story](#create-story-generate-user-stories-with-acceptance-criteria)
  - [/create-epic](#create-epic-create-feature-groupings)
  - [/create-sprint](#create-sprint-plan-2-week-iterations)
  - [/create-ui](#create-ui-generate-ui-specifications)
  - [/dev](#dev-implement-stories-with-tdd)
  - [/qa](#qa-validate-story-implementations)
  - [/release](#release-deploy-to-production)
  - [/orchestrate](#orchestrate-full-lifecycle-automation)
  - [/create-agent](#create-agent-create-custom-subagents)
- [Before and After Examples](#before-and-after-examples)
- [Common Pitfalls and Mitigation](#common-pitfalls-and-mitigation)
- [Framework Integration](#framework-integration)
- [Terminology Reference](#terminology-reference)

---

## Introduction

Clear, complete input is the foundation of effective DevForgeAI usage. When you provide well-structured requirements and context, DevForgeAI can generate high-quality specifications, implementations, and validations on the first pass. Incomplete or ambiguous input leads to multiple clarification questions, reduced token efficiency, and lower-quality outputs.

**Why Clear Input Matters:**

DevForgeAI uses advanced AI reasoning to interpret your intent, but it operates best with explicit context. Unlike human colleagues who can ask follow-up questions continuously, DevForgeAI has a single opportunity to understand your requirements within each command invocation. When key information is missing, the framework's safety mechanisms trigger AskUserQuestion prompts to clarify ambiguities—this is beneficial for correctness but increases iteration cycles.

Research within the DevForgeAI project shows:
- **Incomplete input (vague/ambiguous requirements):** Average 5-8 clarifying questions per story
- **Complete input (specific/detailed requirements):** Average 0-2 clarifying questions per story
- **Well-structured input (with examples/constraints):** 90%+ first-pass success rate

This guide teaches the patterns and structures that lead to high-quality, first-pass outputs with minimal clarification.

**The "Ask, Don't Assume" Principle:**

DevForgeAI's CLAUDE.md instruction set emphasizes "Ask, Don't Assume"—when ambiguity exists, the framework explicitly asks you for clarification rather than making assumptions. This is a feature, not a limitation. By understanding what the framework asks about, you can provide that information upfront in future prompts, eliminating iterations.

This guide shows you how to identify and communicate these details proactively.

---

## Quick Reference Checklist

Use this 1-page checklist before invoking any DevForgeAI command. ✓ all items for optimal results.

### Universal Requirements (All Commands)

- [ ] **Specific Intent:** What exactly do you want to accomplish? (Not: "build API" → Yes: "create REST endpoint for user authentication with JWT tokens")
- [ ] **Context:** What is the broader goal? (Epic, sprint, project goal that this story/task serves)
- [ ] **Success Criteria:** How will you know it's complete? (Measurable, testable outcomes)
- [ ] **Constraints:** What must/must not be done? (Technology stack, performance targets, security requirements)

### Story & Feature Requests (/create-story, /create-epic, /ideate)

- [ ] **User Persona:** Who needs this? (Developer, product owner, end-user, etc.)
- [ ] **Benefit/Value:** Why do they need it? (Saves time, reduces errors, enables new capability)
- [ ] **Acceptance Criteria:** What behavior demonstrates success? (At least 3-5 testable scenarios)
- [ ] **Domain Context:** What project/system does this belong to? (Provides technology/architecture context)

### Development Tasks (/dev, /create-ui)

- [ ] **Story ID/Reference:** Which story are you implementing? (Enables context file lookup)
- [ ] **Technology Stack:** What languages/frameworks are locked? (Prevents conflicts)
- [ ] **Testing Requirements:** How will you validate? (Unit, integration, E2E expectations)
- [ ] **Scope Boundaries:** What's IN scope vs OUT of scope? (Prevents feature creep)

### Validation & Release (/qa, /release)

- [ ] **Story/Build Reference:** What are you validating? (Specific story ID or build version)
- [ ] **Environment:** Where are you validating? (Development, staging, production)
- [ ] **Success Metrics:** What defines a passing validation? (Coverage %, test pass rate, etc.)
- [ ] **Rollback Plan:** How will you handle failures? (Revert steps, fallback strategy)

### Planning Tasks (/create-context, /create-sprint)

- [ ] **Project Scope:** What is being built? (Product description, key features)
- [ ] **Technical Constraints:** What tech stack, security, performance targets? (From tech-stack.md, requirements)
- [ ] **Timeline:** When is this needed? (Sprint length, release dates)
- [ ] **Success Definition:** What does successful project completion look like?

---

## Core Principles

### Principle 1: Completeness Over Brevity

**Pattern:** More specific detail = Higher quality output

```
❌ BEFORE (Incomplete):
/create-story "Add user authentication"

✅ AFTER (Complete):
/create-story "Implement JWT-based authentication for REST API allowing users to register with email/password and login with automatic token refresh every 24 hours"
```

**Why:** The complete version provides:
- Technology choice (JWT)
- Integration point (REST API)
- User workflows (register, login)
- Non-functional requirement (24-hour token refresh)
- Measurable completion (all workflows present)

### Principle 2: Context Before Requests

Provide context BEFORE stating what you want. Context answers "why"—implementation follows "how."

```
❌ BEFORE (No context):
"Create a caching layer"

✅ AFTER (With context):
"API responses currently take 5-10 seconds. Users report slow dashboard. Need caching layer for user profile data (updated <4x daily). Target: <500ms response time for 80% of requests."
```

**Why:** Context enables:
- Appropriate solution selection (cache layer vs. database optimization vs. algorithm improvement)
- Scope negotiation (if target is unrealistic)
- Trade-off decisions (cost vs. performance, complexity vs. feature)

### Principle 3: Examples Over Abstraction

Show specific examples rather than abstract requirements.

```
❌ BEFORE (Abstract):
"Create a fast, scalable API"

✅ AFTER (With examples):
"Create REST endpoint accepting request like: { userId: '123', action: 'purchase' }. Response in <200ms for 1000 concurrent requests. Store results in PostgreSQL with index on userId."
```

**Why:** Examples provide:
- Concrete data structures (request/response format)
- Measurable metrics (200ms, 1000 concurrent)
- Technology choices (PostgreSQL with indexing)
- No ambiguity about "fast" or "scalable"

### Principle 4: Constraints Clarify Scope

Explicitly state what must/must not happen. Constraints eliminate edge cases.

```
❌ BEFORE (No constraints):
"Build image upload feature"

✅ AFTER (With constraints):
"Build image upload feature. Requirements: Max 5MB files, JPG/PNG only, store in S3, users can upload max 100 images per day, must validate on both client and server."
```

**Why:** Constraints define:
- File size limits (prevents storage overrun)
- Format restrictions (prevents unsupported file handling)
- Storage location (clarifies infrastructure)
- Rate limits (prevents abuse)
- Validation approach (full security picture)

---

## Command-Specific Guidance

### /ideate: Transform Business Ideas into Requirements

**Purpose:** Discover and refine business problems, validate feasibility, and structure requirements before architecture.

**Best For:**
- Starting new projects or features from a high-level idea
- Validating if an idea is feasible before investing in design
- Understanding what users/stakeholders actually need

**Required Inputs:**

1. **Business Problem/Idea** (100+ words)
   - What is the opportunity or problem you're solving?
   - Who experiences this problem?
   - What value would solving it provide?

2. **Current State** (optional but recommended, 50+ words)
   - How do users currently handle this?
   - What gaps or pain points exist?

3. **Constraints or Preferences** (if known)
   - Technology preferences?
   - Budget/resource constraints?
   - Timeline expectations?

**What Makes Input Complete:**

Input is "complete" for /ideate when:
- You can articulate the problem in 1-2 sentences
- You can explain who benefits and why (2-3 sentences)
- You've identified key success metrics (3-5 measurable outcomes)
- You've acknowledged any known constraints

**Examples:**

❌ **BEFORE (Incomplete):**
```
/ideate "We need better project management"
```
→ Too vague. "Better" is undefined. No problem articulation.
→ Missing: Current state, metrics, constraints
→ Expected questions: What specifically is broken? Who uses it? What's the budget?

✅ **AFTER (Complete):**
```
/ideate "Our 50-person development team spends 2-3 hours daily in status meetings across 5 different tools. We want a single dashboard showing: (1) sprint progress for all teams, (2) blockers requiring immediate attention, (3) integration with Jira for ticket data. Success = 50% reduction in status meeting time within 6 weeks. Tech: modern web app, integrate with existing Jira instance. Budget: under 100K USD."
```

**See Also:**
- `/create-story` for turning refined ideas into implementable stories
- `/create-epic` for grouping related stories

---

### /create-context: Generate Architectural Constraints

**Purpose:** Create the 6 immutable constraint files (tech-stack.md, source-tree.md, dependencies.md, coding-standards.md, architecture-constraints.md, anti-patterns.md) that guide all development.

**Best For:**
- Starting new projects (greenfield)
- Establishing architectural decisions before development
- Ensuring consistency across team implementations

**Required Inputs:**

1. **Project Name/Description** (50+ words)
   - What is being built?
   - Who are the users?
   - What's the primary use case?

2. **Technology Stack** (specific choices)
   - Language(s): Python, Node.js, C#, Go, etc.
   - Frameworks: Express, Django, ASP.NET, etc.
   - Database: PostgreSQL, MongoDB, Redis, etc.
   - Other key technologies: Docker, Kubernetes, AWS, etc.

3. **Non-Functional Requirements**
   - Performance targets: response time, throughput, scalability
   - Security: authentication method, encryption, compliance
   - Availability: uptime SLA, disaster recovery
   - Maintenance: logging, monitoring, alerting

**What Makes Input Complete:**

Input is "complete" for /create-context when:
- You can describe the project in 2-3 sentences
- You've chosen specific technologies (not generic categories)
- You've defined 3-5 measurable NFRs
- You understand your architectural constraints (and can articulate them)

**Examples:**

❌ **BEFORE (Incomplete):**
```
/create-context "Build a web application"
```
→ Too generic. What kind of app? What tech?
→ Missing: tech stack, NFRs, architecture approach
→ Expected questions: Language? Framework? Performance targets?

✅ **AFTER (Complete):**
```
/create-context "E-commerce platform for selling digital products. 100,000 concurrent users, $500K annual revenue. Tech stack: Node.js backend (Express), React frontend, PostgreSQL database, Redis cache, Docker containers, AWS deployment. NFRs: 99.9% uptime SLA, <200ms response time for product catalog, PCI compliance for payments, autoscaling 50-10,000 containers based on load."
```

**See Also:**
- CLAUDE.md for architecture principles
- tech-stack.md pattern (post-creation)

---

### /create-story: Generate User Stories with Acceptance Criteria

**Purpose:** Transform requirements into structured user stories with testable acceptance criteria and technical specifications.

**Best For:**
- Converting validated ideas (from /ideate) into implementable work
- Breaking down features into 5-13 point stories
- Establishing Definition of Done before development

**Required Inputs:**

1. **Feature Description** (100+ words)
   - As a [user persona], I want [capability], so that [value/benefit]
   - Avoid technical jargon; focus on user intent

2. **Acceptance Criteria** (3-7 scenarios)
   - Given [precondition], When [action], Then [expected result]
   - Each should be independently testable

3. **Context** (if not obvious)
   - Which epic does this serve?
   - Are there dependencies on other stories?
   - What is the business priority?

**What Makes Input Complete:**

Input is "complete" for /create-story when:
- You can state the user persona and their goal in 1 sentence
- You can write 3-5 Given/When/Then scenarios
- You can articulate why this story matters (business value)
- You've identified any dependencies or constraints

**Examples:**

❌ **BEFORE (Incomplete):**
```
/create-story "Add password reset"
```
→ Missing user perspective. What's the workflow?
→ Missing acceptance criteria. How do we verify it works?
→ Result: 5 AskUserQuestion prompts for clarification

✅ **AFTER (Complete):**
```
/create-story "As a logged-out user, I want to reset my password via email link so I can regain access if I've forgotten my password.

Acceptance Criteria:
1. Given I'm on the login page, When I click 'Forgot Password', Then I see an email input field and submit button
2. Given I enter a registered email, When I submit, Then I receive an email within 2 minutes with a password reset link
3. Given I click the password reset link, When I enter a new password (8+ chars, 1 uppercase, 1 number), Then my password is updated and I can login
4. Given I enter a weak password, When I submit, Then I see validation error with password requirements
5. Given the reset link is >1 hour old, When I click it, Then I see 'Link expired' and option to request new link

Dependencies: User authentication system (STORY-001 must be complete)
Priority: High - Password resets are essential for user retention"
```

**See Also:**
- `/create-epic` for grouping multiple related stories
- `/create-context` for establishing acceptance criteria patterns

---

### /create-epic: Create Feature Groupings

**Purpose:** Group multiple related stories under a business initiative/feature.

**Best For:**
- Large features requiring 5+ stories
- Coordinating work across multiple teams
- Establishing business-level progress tracking

**Required Inputs:**

1. **Epic Name** (3-5 words describing the feature)
   - Clear, stakeholder-friendly language
   - Example: "User Authentication System", "Payment Processing Integration"

2. **Epic Goal** (50+ words)
   - What business outcome does this achieve?
   - Who benefits and how?

3. **Stories** (if known, can be refined during ideation)
   - List of features/components that comprise the epic
   - Example: Login, Registration, Password Reset, 2FA, Audit Logging

**What Makes Input Complete:**

Input is "complete" for /create-epic when:
- You can state the epic goal in 2-3 sentences
- You can list 4-8 component stories
- You can define epic-level success criteria (features, metrics, timeline)

**Examples:**

❌ **BEFORE (Incomplete):**
```
/create-epic "Authentication"
```
→ Too terse. What's included? What's the scope?
→ Missing: goal, stories, success metrics
→ Result: Multiple clarifications needed

✅ **AFTER (Complete):**
```
/create-epic "User Authentication System

Goal: Enable secure user account creation and authentication, supporting 100,000 concurrent users with <200ms login time and 99.9% uptime.

Stories:
1. User Registration (email/password validation, email confirmation)
2. Login (password verification, JWT token generation, automatic refresh)
3. Password Reset (email-based recovery, token expiration)
4. Two-Factor Authentication (TOTP support, SMS backup)
5. Session Management (token expiration, forced logout, device tracking)
6. Audit Logging (track all auth events for compliance)

Success Criteria:
- All stories complete and merged (6 points total)
- 95%+ test coverage
- <200ms login time for 95th percentile
- Security audit passed (OWASP Top 10 validated)
- Zero critical vulnerabilities"
```

**See Also:**
- `/create-story` for individual story details
- `/create-sprint` for assigning stories to sprint

---

### /create-sprint: Plan 2-Week Iterations

**Purpose:** Select stories, validate capacity, and establish sprint goals.

**Best For:**
- Planning 2-week development iterations
- Coordinating team capacity and dependencies
- Establishing sprint priorities

**Required Inputs:**

1. **Sprint Number/Name**
   - Example: "Sprint 1", "Sprint-03-Q1-2025"

2. **Stories to Include** (list story IDs)
   - Which stories from backlog are being tackled?
   - Should total 30-50 points for typical team

3. **Team Capacity** (if relevant)
   - How many people? Skill levels?
   - Any planned time off?

**What Makes Input Complete:**

Input is "complete" for /create-sprint when:
- You've identified which backlog stories to include
- You've estimated total points (and confirmed within team capacity)
- You've identified any blockers or dependencies

**Examples:**

❌ **BEFORE (Incomplete):**
```
/create-sprint Sprint-001
```
→ Which stories? What's the goal? How many points?
→ Missing: story list, capacity, priorities
→ Result: Clarification questions about story selection

✅ **AFTER (Complete):**
```
/create-sprint "Sprint-001 (Jan 20-31, 2025)

Stories:
- STORY-001: User Registration (8 points) - Foundation
- STORY-002: User Login (8 points) - Foundation
- STORY-003: Password Reset (5 points) - High value
- STORY-004: Email Integration (8 points) - Blocker for auth stories

Team Capacity: 40 points (4 developers × 10 points each)
Total Sprint Load: 29 points (87% utilization, 11-point buffer)

Dependencies:
- Email service must be configured before STORY-004
- Database migrations must be reviewed before STORY-001

Goals:
- Complete core authentication flow (register, login, password reset)
- Integrate email service
- 95%+ test coverage
- 0 security vulnerabilities

Priority: Auth is critical path—all stories must complete"
```

**See Also:**
- `/dev` for implementing stories in the sprint
- Capacity planning guidelines in CLAUDE.md

---

### /create-ui: Generate UI Specifications

**Purpose:** Design and specify user interface components with accessibility and responsive design.

**Best For:**
- Web, mobile, or desktop interface design
- Interactive workflows requiring user input
- Accessibility and responsive design validation

**Required Inputs:**

1. **Story or Component Description**
   - Which story's UI are you designing? (reference STORY-ID)
   - What user workflows need UI? (e.g., "login form", "product catalog")

2. **User Actions/Workflows**
   - What can users do? (click buttons, enter text, view data)
   - What information do they need to see?

3. **Design Preferences** (optional)
   - Color scheme, branding guidelines?
   - Framework preferences? (React, Vue, Angular)
   - Accessibility requirements (WCAG 2.1 AA, ARIA labels)

**What Makes Input Complete:**

Input is "complete" for /create-ui when:
- You can describe the user workflow in 3-5 steps
- You've identified required data fields/display elements
- You've specified accessibility needs

**Examples:**

❌ **BEFORE (Incomplete):**
```
/create-ui "Create login page"
```
→ What form fields? Error handling? Mobile responsive?
→ Missing: workflow, fields, accessibility requirements

✅ **AFTER (Complete):**
```
/create-ui STORY-002

Workflow:
1. User enters email address
2. User enters password
3. User clicks "Login" button
4. System shows loading indicator
5. On success: redirect to dashboard
6. On error: show error message with retry option

Form Fields:
- Email (text input, validation: RFC 5322 format)
- Password (password input, "forgot password?" link)
- Remember me (checkbox, optional)

Error Scenarios:
- Invalid email format: "Please enter a valid email"
- Wrong password: "Email or password is incorrect" (don't reveal which)
- Account locked: "Too many attempts. Try again in 15 minutes"
- Server error: "Login temporarily unavailable. Try again soon"

Accessibility:
- WCAG 2.1 AA compliant
- All form labels with associated <label> tags
- Error messages announced to screen readers
- Keyboard navigation (Tab order: email → password → submit)
- Minimum 44px touch targets for mobile

Responsive:
- Mobile (320px): Single column, full-width inputs
- Tablet (768px): Centered form, 400px width
- Desktop (1200px): Centered form, 400px width, remember-me on same row
"
```

**See Also:**
- `/dev` for implementing the UI
- Component-specific guidance in devforgeai-ui-generator documentation

---

### /dev: Implement Stories with TDD

**Purpose:** Implement a story following Test-Driven Development (Red → Green → Refactor cycle).

**Best For:**
- Building features from stories with full test coverage
- Ensuring code quality, maintainability, and security
- Automated validation of implementation

**Required Inputs:**

1. **Story ID**
   - Which story are you implementing? (e.g., STORY-001)
   - The story file should be ready with acceptance criteria

2. **No Other Inputs Required**
   - Story file provides all context
   - Tech stack from context files
   - Acceptance criteria from story definition

**What Makes Input Complete:**

Input is "complete" for /dev when:
- You've identified the story ID
- The story file exists with acceptance criteria
- You have a Git repository initialized (recommended)

**Examples:**

❌ **BEFORE (Incomplete):**
```
/dev "Implement user registration"
```
→ Which story? Where are acceptance criteria?
→ No story file, can't proceed
→ Result: Story lookup failures, halted workflow

✅ **AFTER (Complete):**
```
/dev STORY-001
```
→ Loads STORY-001 from devforgeai/specs/Stories/
→ Extracts acceptance criteria
→ Validates tech stack from context files
→ Begins TDD workflow: Red → Green → Refactor → Integration

**Workflow:**
1. Red phase: Generate failing tests from acceptance criteria
2. Green phase: Write minimal code to pass tests
3. Refactor phase: Improve code quality, patterns, performance
4. Integration phase: Validate cross-component interactions
5. Complete: Story marked "Dev Complete", ready for QA

**See Also:**
- `/qa` for validating implementation
- CLAUDE.md for test-driven development principles

---

### /qa: Validate Story Implementations

**Purpose:** Validate that completed implementations meet acceptance criteria, quality standards, and security requirements.

**Best For:**
- Post-implementation validation (after /dev)
- Security and quality auditing
- Coverage threshold validation

**Required Inputs:**

1. **Story ID**
   - Which story are you validating? (e.g., STORY-001)

2. **Validation Mode** (optional, defaults to "light")
   - `light`: Quick validation (build, tests, basic patterns)
   - `deep`: Comprehensive validation (coverage, security, architecture)

**What Makes Input Complete:**

Input is "complete" for /qa when:
- You've identified the story ID
- You've optionally specified validation mode

**Validation Procedures:**

Light Mode (during development):
- Build succeeds
- All tests pass (100%)
- Basic pattern checks (no obvious code smells)
- No CRITICAL violations

Deep Mode (before release):
- Light mode + coverage analysis
- Security audit (OWASP Top 10)
- Architecture validation (context files)
- Performance testing
- Accessibility validation (if UI)

**Examples:**

❌ **BEFORE (Incomplete):**
```
/qa "Validate the registration feature"
```
→ Which story? What validation level?
→ Missing: story ID, validation mode

✅ **AFTER (Complete):**
```
/qa STORY-001 light
```
→ Runs light validation on STORY-001
→ Checks: build, tests, basic patterns
→ Result: Pass/Fail with remediation guidance

✅ **FOR RELEASE:**
```
/qa STORY-001 deep
```
→ Runs comprehensive validation
→ Checks: coverage (95%+ target), security (OWASP), architecture
→ Result: Pass/Fail with detailed analysis

**See Also:**
- `/dev` for initial implementation
- `/release` for production deployment
- `/orchestrate` for full lifecycle

---

### /release: Deploy to Production

**Purpose:** Automated deployment with validation, health checks, and rollback capability.

**Best For:**
- Moving validated stories to production
- Multi-environment deployment (staging → production)
- Automated rollback if issues detected

**Required Inputs:**

1. **Story ID** (or Build ID)
   - What are you releasing? (e.g., STORY-001)

2. **Target Environment** (optional, defaults to staging)
   - `staging`: Pre-production environment (for smoke testing)
   - `production`: Live environment (user-facing)

3. **Deployment Strategy** (optional, defaults to rolling)
   - `rolling`: Gradual rollout
   - `blue-green`: Instant switch between versions
   - `canary`: Staged rollout to subset of users
   - `recreate`: Stop old, start new (downtime)

**What Makes Input Complete:**

Input is "complete" for /release when:
- You've identified the story/build ID
- You've specified target environment
- You've selected deployment strategy

**Pre-Release Checks:**
- QA approved (story status = "QA Approved")
- All tests passing
- No critical/blocking issues
- Rollback plan documented

**Examples:**

❌ **BEFORE (Incomplete):**
```
/release "Deploy the new feature"
```
→ Which feature? Which environment? Rollback plan?
→ Missing: story ID, environment, strategy

✅ **AFTER (Complete):**
```
/release STORY-001 staging rolling
```
→ Deploys STORY-001 to staging environment
→ Uses rolling deployment (gradual rollout)
→ Runs smoke tests post-deployment
→ Reports health metrics (error rate, response time)
→ If failures detected: Automatic rollback to previous version

✅ **FOR PRODUCTION:**
```
/release STORY-001 production blue-green
```
→ Deploys STORY-001 to production using blue-green
→ Instant switch from old version to new
→ Monitors health for 5 minutes
→ If issues: Instant rollback to previous version
→ If healthy: Old version decommissioned

---

### Example 3: QA Validation

**❌ BEFORE (Incomplete):**
```
/qa STORY-001
```
→ "What validation level? Light or deep? What environment?"
→ Ambiguous → Multiple questions

✅ **AFTER (Complete):**
```
/qa STORY-001 deep
```
→ Runs comprehensive validation
→ Checks: coverage, security, architecture, performance
→ Clear validation target

---

### Example 4: Epic Creation

**❌ BEFORE (Ineffective):**
```
/create-epic "Build authentication"
```
→ Too vague, missing scope and value

✅ **AFTER (Effective):**
```
/create-epic "User Authentication System

Business Goal: Enable secure user account creation and login, supporting 100K concurrent users, <200ms response time, 99.9% uptime.

Stories:
1. User Registration (email+password validation)
2. User Login (JWT token generation)
3. Password Reset (email-based recovery)
4. Session Management (token expiration, revocation)

Success: All stories complete, 95%+ test coverage, zero security vulnerabilities"
```

---

### Example 5: Sprint Planning

**❌ BEFORE (Incomplete):**
```
/create-sprint Sprint-001
```
→ "Which stories? Capacity check?"

✅ **AFTER (Complete):**
```
/create-sprint "Sprint-001 (Jan 20-31)

Stories: STORY-001 (8 pts), STORY-002 (8 pts), STORY-003 (5 pts)
Total: 21 points (40-point team capacity)
Goal: Complete core registration/login workflows"
```

**See Also:**
- `/qa` for pre-release validation
- `/orchestrate` for coordinating dev → qa → release

---

### /orchestrate: Full Lifecycle Automation

**Purpose:** Automate complete story lifecycle (development → QA → release) in one invocation.

**Best For:**
- Simple stories with straightforward implementations
- Validating that end-to-end workflow functions
- High-confidence, low-risk changes

**Required Inputs:**

1. **Story ID**
   - Which story for full lifecycle? (e.g., STORY-001)

2. **No Other Inputs**
   - Everything else is automated

**What Makes Input Complete:**

Input is "complete" for /orchestrate when:
- You've identified the story ID
- Story is in "Ready for Dev" status
- No exceptional circumstances (no custom validation needed)

**Workflow Executed:**
1. /dev STORY-ID (Red → Green → Refactor → Integration)
2. /qa STORY-ID light (Intermediate validation)
3. /qa STORY-ID deep (Comprehensive validation before release)
4. /release STORY-ID staging (Deploy to staging, smoke tests)
5. /release STORY-ID production (Deploy to production)

**Examples:**

❌ **BEFORE (Using Step-by-Step):**
```
/dev STORY-001
[Wait for completion]
/qa STORY-001 light
[Wait for result]
/qa STORY-001 deep
[Wait for result]
/release STORY-001 staging
[Wait for validation]
/release STORY-001 production
[Wait for deployment]
```
→ 5 separate invocations, lots of waiting
→ 2-3 hours total time
→ Manual coordination

✅ **AFTER (Using Orchestration):**
```
/orchestrate STORY-001
```
→ Single invocation
→ Entire lifecycle automated
→ Progress updates as each phase completes
→ Automatic rollback if validation fails
→ <1 hour total time
→ Zero manual coordination

**See Also:**
- `/dev`, `/qa`, `/release` for individual phase control
- Story workflow states in CLAUDE.md

---

### /create-agent: Create Custom Subagents

**Purpose:** Generate specialized Claude Code subagents for domain-specific tasks (code analysis, testing, architecture review, etc.).

**Best For:**
- Extending DevForgeAI with custom AI agents
- Implementing domain-specific validation
- Building specialized code analysis tools

**Required Inputs:**

1. **Agent Purpose** (50+ words)
   - What problem does this agent solve?
   - What's its primary responsibility?
   - What inputs/outputs does it handle?

2. **Agent Capabilities** (list 3-5 abilities)
   - What specific tasks can it perform?
   - Examples: code analysis, test generation, architecture review, etc.

3. **Tool Access Required** (if known)
   - Which Claude Code tools does it need?
   - File access (Read, Write, Edit)?
   - Tool execution (Bash for specific commands)?

**What Makes Input Complete:**

Input is "complete" for /create-agent when:
- You can describe the agent's purpose in 2-3 sentences
- You can list 3-5 specific capabilities
- You can identify required tool access

**Examples:**

❌ **BEFORE (Incomplete):**
```
/create-agent "Code analyzer"
```
→ What kind of code? What's it analyzing? What's the output?
→ Missing: purpose, capabilities, tool access

✅ **AFTER (Complete):**
```
/create-agent "Python Test Coverage Analyzer

Purpose: Analyze Python test suites to identify coverage gaps and suggest missing test cases for business logic. Helps developers achieve 95%+ code coverage with minimal test bloat.

Capabilities:
1. Parse Python source code and test files to extract coverage data
2. Identify uncovered code paths (lines, branches, conditions)
3. Suggest minimal test cases to achieve coverage targets
4. Detect code patterns indicating testability issues (tight coupling, static methods)
5. Generate test stubs for high-risk uncovered code

Tool Access Required:
- Read: Python source files, test files, coverage reports
- Bash: pytest with coverage plugin
- Output: Structured report with coverage gaps and test recommendations

Input Format:
- Path to Python source tree
- Path to test directory
- Target coverage threshold (default: 95%)

Output Format:
- Coverage summary (current % per file)
- Gap analysis (lines/branches not covered)
- Test recommendations (code to test, suggested test approach)
- Risk assessment (which uncovered code is highest risk)
"
```

**See Also:**
- `.claude/agents/` directory for existing agent patterns
- CLAUDE.md for agent development guidelines

---

## Before and After Examples

### Example 1: Feature Request - User Authentication

**❌ BEFORE (Ineffective):**
```
/create-story "Add user login"

**Result:**
- 5 AskUserQuestion prompts
- Questions: "What login method? Email+password or social? JWT or sessions? How long tokens last? Remember me option?"
- Total iterations: 3 back-and-forths
- Time investment: 45 minutes
- Quality: Story missing key implementation details
```

**✅ AFTER (Effective):**
```
/create-story "Implement email+password login for REST API

As a registered user, I want to login with my email and password to access my account and receive a JWT token for API requests.

Acceptance Criteria:
1. User submits valid email+password → receive JWT token valid for 24 hours
2. User submits invalid credentials → receive 'Email or password incorrect' error
3. User receives token → can use token in Authorization header to access protected endpoints
4. User's token expires after 24 hours → must login again
5. User can request token refresh → receive new token before expiration

Technical Requirements:
- Use bcrypt for password hashing
- JWT tokens (RS256 algorithm)
- Store tokens in database (no in-memory)
- Rate limit: max 5 failed attempts per IP per hour
- Log all authentication events for audit trail

**Result:**
- 0 AskUserQuestion prompts (fully specified)
- Immediate story creation
- All implementation details clear
- Total time: 10 minutes
- Quality: High - developers have all context needed
```

**Improvement Metrics:**
- Questions reduced: 5 → 0 (100% improvement)
- Time saved: 35 minutes per story (43% reduction)
- First-pass implementation success: Increased to 95%+

---

### Example 1b: API Design - Before/After

**❌ BEFORE (Ineffective):**
```
/create-story "Add API endpoint for getting user data"

**Result:**
- "What HTTP method? GET? POST? What data should the response include? Should it be paginated?"
- Multiple clarifications needed
```

**✅ AFTER (Effective):**
```
/create-story "Add GET /api/v1/users/{id} endpoint returning { id, email, name, createdAt } JSON response. Authorization: Bearer token required. Error responses: 401 (unauthorized), 404 (user not found), 500 (server error) with standard error format { code, message }"
```

---

### Example 1c: UI Feature - Before/After

**❌ BEFORE (Ineffective):**
```
/create-story "Add user profile page"

**Result:**
- "What information should be displayed? Can users edit their profile? Mobile responsive?"
```

✅ **AFTER (Effective):**
```
/create-story "Create user profile page showing: email, name, bio (editable), avatar (read-only). Allow name/bio editing. Changes saved to database with confirmation message. Mobile responsive (stacked layout below 768px). Accessibility: WCAG 2.1 AA compliant"
```

---

### Example 2: Development Task - Implementation

**❌ BEFORE (Ineffective):**
```
/dev "Implement authentication"

**Result:**
- Tech-stack validation failure: "What language? Framework? Database?"
- Cannot proceed without context
- Framework halts workflow
```

**✅ AFTER (Effective):**
```
/dev STORY-002

**Workflow:**
- Loads story file (STORY-002)
- Extracts acceptance criteria (all specified in story)
- Validates tech-stack.md (Node.js + Express + PostgreSQL)
- Begins TDD: Red phase (test generation from AC)
- Proceeds through Green → Refactor → Integration
- Completes with full test coverage (95%+)
- Story status: "Dev Complete"

**Result:**
- Zero clarification questions
- Clear acceptance criteria guide implementation
- Tech stack known from context files
- Tests validate against AC directly
- No ambiguity about done
```

---

### Example 3: Release Deployment

**❌ BEFORE (Ineffective):**
```
/release STORY-001

**Result:**
- "What environment? Deployment strategy? Rollback plan?"
- Multiple clarification prompts
- Uncertain about deployment approach
- Manual coordination needed
```

✅ **AFTER (Effective):**
```
/release STORY-001 production blue-green

**Workflow:**
- Deploy to production using blue-green strategy
- Run smoke tests on new version
- If tests pass: Switch traffic to new version
- If tests fail: Automatic rollback to previous version
- Monitor for 5 minutes post-switch
- All clear: Old version decommissioned

**Result:**
- Zero clarifications (strategy specified)
- Clear rollback plan (automatic)
- Reduced deployment risk
- Audit trail of deployment
```

---

## Common Pitfalls and Mitigation

### Pitfall 1: Vague Success Criteria

**Problem:** "Build a fast API" doesn't define "fast" or measurable success.

**Mitigation:**
```
❌ "Build a fast API"
✅ "Build REST API with <200ms response time for 95th percentile requests, supporting 1,000 concurrent users"
```

---

### Pitfall 2: Missing User Context

**Problem:** Implementing features without understanding who uses them or why.

**Mitigation:**
```
❌ "Add email notifications"
✅ "As a project manager, I want email notifications when team members complete tasks so I can track project progress without checking the app constantly"
```

---

### Pitfall 3: Incomplete Acceptance Criteria

**Problem:** AC that cannot be objectively verified: "System should be reliable."

**Mitigation:**
```
❌ AC: "System is reliable"
✅ AC: "System has 99.9% uptime (monthly), zero critical bugs after release, <5-minute recovery from failures"
```

---

### Pitfall 4: Forgetting Dependencies

**Problem:** Implementing STORY-003 when STORY-001 (prerequisite) isn't complete.

**Mitigation:**
```
❌ No dependency tracking
✅ Story explicitly states: "Depends on: STORY-001 (user authentication). Cannot start until STORY-001 status = 'Dev Complete'"
```

---

### Pitfall 5: Technology Constraints Ignored

**Problem:** Suggesting React when tech-stack.md specifies Vue.

**Mitigation:**
```
Read devforgeai/specs/context/tech-stack.md BEFORE proposing technologies.
If conflict detected: Use AskUserQuestion to resolve.
```

---

### Pitfall 6: No Test Coverage Consideration

**Problem:** Implementing features without planning how to test them.

**Mitigation:**
```
❌ "Implement password reset feature" (no testing plan)
✅ "Implement password reset with: (1) Unit tests for token generation, (2) Integration tests for email sending, (3) E2E tests for full reset workflow. Target: 95%+ code coverage"
```

---

### Pitfall 7: Ambiguous Scope Boundaries

**Problem:** Feature creep when scope is unclear: "Add user profile" could include profile photo, social links, bio, verification, etc.

**Mitigation:**
```
❌ "Add user profile"
✅ "Add user profile with: (1) Display name, (2) Bio (max 500 chars), (3) Avatar upload (JPG/PNG, max 5MB). OUT of scope: social media links, verification, activity feed"
```

---

### Pitfall 8: Ignoring Performance Targets

**Problem:** Building features without performance requirements, leading to slow systems.

**Mitigation:**
```
❌ "Build search feature"
✅ "Build search feature: (1) Return results in <500ms for 10,000-item catalog, (2) Support full-text search with typo tolerance, (3) Pagination (20 results per page)"
```

---

### Pitfall 9: Missing Security Specifications

**Problem:** Forgetting to mention authentication, authorization, data validation, encryption.

**Mitigation:**
```
❌ "Add file upload"
✅ "Add file upload: (1) Max 5MB, JPG/PNG only, (2) Validate on server (not just client), (3) Store in S3 (encrypted), (4) Rate limit: 100 uploads per user per day, (5) Require authentication"
```

---

### Pitfall 10: Lack of Measurable Definition of Done

**Problem:** "Feature is done" is subjective without clear completion criteria.

**Mitigation:**
```
❌ DoD: "Feature is implemented and tested"
✅ DoD: "(1) All AC passing, (2) 95%+ code coverage, (3) Zero critical/high security issues, (4) Performance targets met, (5) Backward compatibility maintained, (6) Documentation updated, (7) QA approved"
```

---

## Framework Integration

### How This Guide Fits DevForgeAI

This guide complements framework documentation:

| Document | Purpose | Audience |
|----------|---------|----------|
| **effective-prompting-guide.md** (this file) | How to provide effective input to DevForgeAI commands | Users (developers, PMs, stakeholders) |
| **CLAUDE.md** | Framework rules, quality gates, workflow states | AI agents, advanced users |
| **user-input-guidance.md** | Framework-internal requirements for AskUserQuestion | Development agents |
| **claude-code-terminal-expert/SKILL.md** | Claude Code Terminal features + [user input guidance](../skills/claude-code-terminal-expert/SKILL.md#how-devforgeai-skills-work-with-user-input) | Users asking about Claude Code capabilities |
| **skills/** | Command implementations (how each /command works internally) | AI agents executing commands |

### Reading This Guide

**For first-time users:**
1. Start with "Introduction" section
2. Review "Quick Reference Checklist"
3. Read command-specific guidance for commands you'll use
4. Keep "Common Pitfalls" as reference

**For experienced users:**
1. Skim "Before and After Examples" for patterns
2. Reference specific command sections as needed
3. Review "Framework Integration" for deeper understanding

**For framework developers:**
1. Study "Core Principles" for why certain input is needed
2. Review "Command-Specific Guidance" for documentation completeness
3. Reference "Common Pitfalls" for user education content

### Navigation

- **Want to start a project?** → /ideate, /create-context, /create-epic
- **Want to define work?** → /create-story, /create-sprint
- **Want to implement?** → /dev
- **Want to validate?** → /qa
- **Want to deploy?** → /release
- **Want full automation?** → /orchestrate
- **Want to design UI?** → /create-ui
- **Want custom agents?** → /create-agent

---

## Terminology Reference

**Acceptance Criteria (AC):** Testable scenarios that define "done" for a user story. Format: "Given [precondition], When [action], Then [expected result]"

**Backlog:** Prioritized list of work items waiting to be included in a sprint.

**Definition of Done (DoD):** Checklist of quality requirements that all stories must meet (test coverage, documentation, security validation, etc.).

**Epic:** Large feature grouping multiple related stories (e.g., "User Authentication System").

**QA (Quality Assurance):** Validation that implementation meets acceptance criteria, quality standards, and security requirements.

**Ready for Dev:** Story status indicating story is well-defined and ready for development.

**Skill:** Inline prompt expansion providing workflow instructions (Red → Green → Refactor for /dev, etc.).

**Sprint:** 2-week iteration of work (typically 30-50 story points).

**Story (User Story):** Unit of work with acceptance criteria, usually 5-13 story points.

**Tech-Stack:** LOCKED constraint file defining project technologies (language, framework, database, etc.).

**TDD (Test-Driven Development):** Development approach: Red (failing tests) → Green (pass tests) → Refactor (improve quality).

**User Persona:** Representative user type (e.g., "project manager", "end-user", "admin").

**Workflow State:** Story status in lifecycle (Backlog → Ready for Dev → In Development → Dev Complete → QA → Released).

---

**Last Updated:** 2025-01-20
**Version:** 1.0
**Status:** Published for STORY-052 implementation
