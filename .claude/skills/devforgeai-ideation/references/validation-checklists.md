# Ideation Skill Validation Checklists

This reference provides comprehensive validation checklists for Phase 6.4 (Self-Validation) of the ideation workflow. Use these checklists to ensure all artifacts meet quality standards before completing ideation phase.

---

## Artifact Creation Checklist

Verify all required artifacts have been created in correct locations.

### Epic Documents

- [ ] At least 1 epic document created
- [ ] All epic files stored in `devforgeai/specs/Epics/` directory
- [ ] Epic IDs follow naming convention: `EPIC-001`, `EPIC-002`, etc. (sequential numbering)
- [ ] All epic files have `.epic.md` extension
- [ ] Epic count matches planned epic count from Phase 4 decomposition
- [ ] No placeholder or template epic files (all must be fully generated)

**Validation Command:**
```
epic_files = Glob(pattern="devforgeai/specs/Epics/EPIC-*.epic.md")
epic_count = len(epic_files)

# Verify count matches expected
if epic_count == 0:
    ERROR: No epic documents created
elif epic_count < planned_epic_count:
    WARNING: Only {epic_count}/{planned_epic_count} epics created
else:
    ✓ All {epic_count} epics created
```

### Requirements Specification

- [ ] Requirements spec created in `devforgeai/specs/requirements/` directory
- [ ] File naming follows convention: `{project-name}-requirements.md`
- [ ] File is non-empty (at least 1000 characters for valid spec)
- [ ] Markdown formatting is valid (no broken tables or lists)

**Validation Command:**
```
req_files = Glob(pattern="devforgeai/specs/requirements/*.md")

if len(req_files) == 0:
    ERROR: No requirements specification created
else:
    req_file = req_files[0]
    Read(file_path=req_file, limit=50)  # Check first 50 lines
    # Verify content structure
```

---

## Epic Content Quality Checklist

Validate each epic document has complete, high-quality content.

### YAML Frontmatter (Required Fields)

For each epic, verify frontmatter contains:

- [ ] `id` field: Matches filename (EPIC-001, EPIC-002, etc.)
- [ ] `title` field: Clear, descriptive epic title (not generic)
- [ ] `business-value` field: Quantified business value statement
- [ ] `status` field: Set to "Backlog" initially
- [ ] `priority` field: High, Medium, or Low
- [ ] `created` field: Date in YYYY-MM-DD format
- [ ] `estimated-points` field: Story point estimate (optional but recommended)

**Validation Command:**
```
For each epic_file:
    Read(file_path=epic_file, limit=15)  # Read frontmatter only

    Verify YAML contains:
    - id: EPIC-NNN
    - title: "..."
    - business-value: "..."
    - status: "Backlog"
    - priority: "High|Medium|Low"
    - created: "YYYY-MM-DD"
```

### Epic Description & Context

- [ ] Epic description present (minimum 100 words)
- [ ] Business context explains "why" this epic matters
- [ ] Target users/stakeholders identified
- [ ] Problem statement clear and specific (not vague)
- [ ] Solution overview describes "what" will be built (not "how")

**Quality Indicators:**
- ✅ Good: "Enable customers to track order status in real-time, reducing support calls by 30%"
- ❌ Bad: "Improve user experience"

### Success Metrics & KPIs

- [ ] At least 1 success metric defined per epic
- [ ] Metrics are **measurable** (quantified with numbers)
- [ ] Metrics tied to business outcomes (revenue, cost, time, satisfaction)
- [ ] Baseline and target values specified where applicable

**Examples of Good Metrics:**
- "Reduce average order processing time from 5 minutes to 2 minutes"
- "Increase conversion rate from 2% to 3.5%"
- "Achieve 99.9% uptime for payment processing"
- "Decrease customer support tickets by 40%"

**Examples of Bad Metrics (Too Vague):**
- "Improve performance" (not measurable)
- "Better user experience" (subjective)
- "Faster processing" (no target)

### Features Breakdown

- [ ] At least 2 features defined per epic
- [ ] Each feature has clear name/title
- [ ] Each feature has priority (High, Medium, Low)
- [ ] Each feature has brief description
- [ ] Features are user-facing capabilities (not technical tasks)
- [ ] Features decompose epic into manageable units

**Validation Logic:**
```
For each epic:
    Count features

    if feature_count == 0:
        ERROR: Epic has no features
    elif feature_count == 1:
        WARNING: Consider breaking epic into multiple features
    else:
        ✓ Epic has {feature_count} features
```

### Acceptance Criteria

- [ ] Epic-level acceptance criteria present
- [ ] Criteria follow Given/When/Then format (or similar structured format)
- [ ] At least 3 acceptance criteria per epic
- [ ] Criteria are **testable** (can verify pass/fail)
- [ ] Criteria cover main success scenarios
- [ ] Criteria include edge cases or error scenarios

**Quality Check:**
```
For each acceptance criterion:
    - Must include "Given" (precondition)
    - Must include "When" (action/trigger)
    - Must include "Then" (expected outcome)

    Example:
    ✅ "Given user is logged in, When they view order history, Then they see last 10 orders with status"
    ❌ "User can view orders" (not testable, no structure)
```

### Dependencies & Risks

- [ ] Dependencies section present
- [ ] External dependencies identified (other epics, systems, teams)
- [ ] Technical dependencies noted (infrastructure, tools)
- [ ] Risks section present with at least 1 risk
- [ ] Each risk has likelihood (Low/Medium/High) and impact (Low/Medium/High)
- [ ] Mitigation strategy provided for High-impact risks

### Timeline & Milestones

- [ ] Estimated timeline provided (weeks or sprints)
- [ ] Key milestones identified
- [ ] Milestones tied to feature delivery or acceptance criteria completion

---

## Requirements Specification Quality Checklist

Validate the requirements specification document has comprehensive, well-structured content.

### Document Structure

- [ ] Table of contents or clear section headers
- [ ] Project overview/vision section
- [ ] Problem statement section
- [ ] Functional requirements section
- [ ] Non-functional requirements section
- [ ] Data models section
- [ ] Integration requirements section
- [ ] User workflows section (optional but recommended)
- [ ] Constraints and assumptions section
- [ ] Out-of-scope section
- [ ] Complexity assessment section

### Project Overview & Vision

- [ ] 1-3 paragraph project overview
- [ ] Clear vision statement (what success looks like)
- [ ] Target audience/users identified
- [ ] Business goals articulated
- [ ] Project type specified (greenfield, brownfield, modernization)

### Problem Statement

- [ ] Current pain points documented
- [ ] Impact of problems quantified (time, cost, user satisfaction)
- [ ] Root causes identified (not just symptoms)
- [ ] Stakeholder perspectives captured

### Functional Requirements

- [ ] Requirements categorized by domain or feature area
- [ ] At least 5 functional requirements documented
- [ ] Each requirement follows user story format: "As a [role], I want [action], so that [benefit]"
- [ ] Each requirement has unique ID (e.g., FR-001, FR-002)
- [ ] Priority assigned to each requirement (Must Have, Should Have, Could Have, Won't Have - MoSCoW)
- [ ] Acceptance criteria defined for each requirement
- [ ] No ambiguous terms without definition

**Quality Indicators:**
```
For each functional requirement:
    - Has unique ID: FR-NNN
    - Has role: "As a [role]"
    - Has action: "I want [action]"
    - Has benefit: "So that [benefit]"
    - Has priority: Must|Should|Could|Won't
    - Has acceptance criteria (at least 1)
```

### Non-Functional Requirements (NFRs)

- [ ] Performance requirements quantified
  - Response time targets (e.g., <500ms for API calls)
  - Throughput targets (e.g., 1000 requests/second)
  - Resource usage limits (e.g., <512MB memory per container)
- [ ] Security requirements specified
  - Authentication method (e.g., OAuth2, JWT)
  - Authorization model (e.g., RBAC, ABAC)
  - Data encryption (at rest and in transit)
  - Compliance standards (GDPR, HIPAA, PCI-DSS, SOC2)
- [ ] Scalability requirements measurable
  - User base targets (e.g., 10,000 concurrent users)
  - Data volume targets (e.g., 100GB initial, 10TB at scale)
  - Geographic distribution (single region, multi-region, global)
- [ ] Availability requirements defined
  - Uptime SLA (e.g., 99.9%, 99.99%)
  - Recovery Time Objective (RTO)
  - Recovery Point Objective (RPO)
  - Maintenance windows
- [ ] Usability requirements specified
  - Accessibility standards (WCAG 2.1 Level AA)
  - Browser support (Chrome, Firefox, Safari, Edge versions)
  - Mobile responsiveness
  - Internationalization/localization

**No Vague NFRs Allowed:**
- ❌ "System should be fast" → ✅ "API response time <500ms for 95th percentile"
- ❌ "Secure authentication" → ✅ "OAuth2 with JWT, refresh tokens, MFA support"
- ❌ "Highly available" → ✅ "99.9% uptime SLA, <1 hour RTO, <5 minute RPO"

### Data Models & Entities

- [ ] Core entities identified (nouns in user stories)
- [ ] At least 3 entities documented
- [ ] Each entity has:
  - Entity name
  - Brief description
  - List of attributes (fields)
  - Data types for each attribute
  - Validation rules/constraints
- [ ] Relationships between entities mapped
  - One-to-many relationships
  - Many-to-many relationships
  - One-to-one relationships
- [ ] Primary keys identified
- [ ] Foreign keys identified

**Example Entity Documentation:**
```
Entity: User
Description: Represents a registered user of the system
Attributes:
  - id (UUID, required, unique, primary key)
  - email (string, required, unique, validated email format)
  - password_hash (string, required, bcrypt hashed)
  - first_name (string, required, max 50 chars)
  - last_name (string, required, max 50 chars)
  - created_at (timestamp, required, auto-generated)
  - last_login (timestamp, optional)

Relationships:
  - Has many: Orders (one-to-many)
  - Has one: UserProfile (one-to-one)
  - Belongs to many: Roles (many-to-many via UserRoles table)
```

### Integration Requirements

- [ ] All external integrations listed
- [ ] Each integration has:
  - Integration name/service
  - Purpose (why integration needed)
  - Protocol (REST, GraphQL, SOAP, gRPC, WebSocket)
  - Data format (JSON, XML, Protobuf)
  - Authentication method (API key, OAuth2, JWT)
  - Rate limits/quotas
  - SLA requirements
- [ ] Integration dependencies documented
- [ ] Fallback behavior defined (what happens if integration unavailable)

**Example Integration Documentation:**
```
Integration: Stripe Payment Gateway
Purpose: Process credit card payments
Protocol: REST API (HTTPS)
Data Format: JSON
Authentication: API Secret Key
Rate Limits: 100 requests/second
SLA: 99.99% uptime
Fallback: Queue payment requests for retry if Stripe unavailable
```

### User Workflows

- [ ] Key user workflows documented
- [ ] Each workflow has:
  - Workflow name
  - Actor/user role
  - Trigger/entry point
  - Step-by-step flow
  - Decision points (if/else branches)
  - Success outcome
  - Error scenarios
- [ ] Workflows visualized (flowchart or sequence diagram) - optional

### Constraints & Assumptions

- [ ] Technical constraints documented
  - Legacy system integration requirements
  - Technology limitations
  - Infrastructure constraints
- [ ] Business constraints documented
  - Budget limitations
  - Timeline constraints
  - Resource availability
- [ ] Assumptions explicitly stated
  - Each assumption has validation flag
  - Assumptions marked for confirmation during architecture phase

**Example Assumptions:**
```
ASSUMPTION: Users have modern browsers (Chrome/Firefox/Safari/Edge latest 2 versions)
VALIDATION NEEDED: Confirm with stakeholders if IE11 support required

ASSUMPTION: Average order size is <100 line items
VALIDATION NEEDED: Analyze historical order data to confirm
```

### Out-of-Scope

- [ ] Out-of-scope items clearly listed
- [ ] Rationale provided for each exclusion
- [ ] Future scope identified (deferred features)

**Purpose:** Prevent scope creep by explicitly stating what will NOT be included in current implementation.

---

## Complexity Assessment Checklist

Validate complexity assessment is accurate and complete.

### Complexity Score Calculation

- [ ] Total complexity score calculated (0-60 range)
- [ ] Score breakdown documented for all 4 dimensions:
  - Functional Complexity: 0-20 points
  - Technical Complexity: 0-20 points
  - Team/Organizational Complexity: 0-10 points
  - Non-Functional Complexity: 0-10 points
- [ ] Each dimension has sub-scores documented
- [ ] Scoring rationale provided (why each score assigned)

**Validation Logic:**
```
total_score = functional + technical + team_org + non_functional

if total_score < 0 or total_score > 60:
    ERROR: Invalid complexity score {total_score} (must be 0-60)
elif functional > 20 or technical > 20 or team_org > 10 or non_functional > 10:
    ERROR: Sub-score exceeds maximum
else:
    ✓ Complexity score valid: {total_score}/60
```

### Architecture Tier Recommendation

- [ ] Architecture tier determined from score
- [ ] Tier aligns with score ranges:
  - 0-15 points = Simple Application
  - 16-30 points = Standard Application
  - 31-45 points = Advanced Platform
  - 46-60 points = Enterprise Platform
- [ ] Tier characteristics documented
- [ ] Tier recommendation includes rationale

**Validation Logic:**
```
if score >= 0 and score <= 15:
    tier = "Simple Application"
elif score >= 16 and score <= 30:
    tier = "Standard Application"
elif score >= 31 and score <= 45:
    tier = "Advanced Platform"
elif score >= 46 and score <= 60:
    tier = "Enterprise Platform"
else:
    ERROR: Invalid score for tier determination
```

### Technology Stack Guidance

- [ ] Technology recommendations provided based on tier
- [ ] Backend recommendations listed
- [ ] Frontend recommendations listed
- [ ] Database recommendations listed
- [ ] Infrastructure recommendations listed
- [ ] Recommendations aligned with tier (not over-engineered or under-architected)

### Scalability & Performance Targets

- [ ] User base targets quantified
  - Initial: X users
  - 1 year: Y users
  - 3 years: Z users
- [ ] Transaction volume targets quantified
  - Requests per second
  - Transactions per day
- [ ] Data volume targets quantified
  - Initial database size
  - Growth rate (GB/month or TB/year)
- [ ] Performance targets specified
  - API response time (95th percentile)
  - Page load time (median)
  - Database query time (typical)

---

## Handoff Readiness Checklist

Ensure ideation phase is complete and ready for transition to architecture phase.

### Artifact Completeness

- [ ] All planned epics created and validated
- [ ] Requirements specification complete (no TBD or TODO sections)
- [ ] Complexity assessment finalized with tier and tech recommendations
- [ ] All sections of requirements spec have content (not empty placeholders)
- [ ] No placeholder text ("lorem ipsum", "example", "TBD")

### Ambiguity Resolution

- [ ] All ambiguities resolved via AskUserQuestion
- [ ] No vague requirements remain ("fast", "scalable", "user-friendly" without metrics)
- [ ] All assumptions documented with validation flags
- [ ] All open questions answered or documented for architecture phase

### Risk & Constraint Documentation

- [ ] All risks identified with likelihood and impact
- [ ] High-impact risks have mitigation strategies
- [ ] Technical constraints documented
- [ ] Business constraints documented
- [ ] Resource constraints noted

### Quality Validation

- [ ] Epic business values are quantified (not generic)
- [ ] Success metrics are measurable (not subjective)
- [ ] Acceptance criteria are testable (can verify pass/fail)
- [ ] Functional requirements follow user story format
- [ ] Non-functional requirements have specific targets
- [ ] Data models include attributes, types, constraints, and relationships
- [ ] Integration requirements specify protocols, formats, and authentication

### Next Phase Preparation

- [ ] Architecture context files checked:
  - If greenfield: Context files do not exist (will be created by architecture skill)
  - If brownfield: Context files exist (will be validated against new requirements)
- [ ] Next phase identified:
  - If no context: Transition to architecture skill (/create-context)
  - If context exists: Transition to orchestration skill (/create-sprint)
- [ ] Handoff artifacts ready:
  - Epic documents
  - Requirements specification
  - Complexity assessment
  - Technology recommendations

---

## User Communication Checklist

Validate that user has been properly informed of completion and next steps.

### Completion Summary Presented

- [ ] Summary generated using output-templates.md
- [ ] All artifact locations communicated to user
- [ ] Epic count and titles listed
- [ ] Requirements summary provided (functional count, NFR count, entities, integrations)
- [ ] Complexity score and tier communicated
- [ ] Technology recommendations presented

### Next Steps Communicated

- [ ] User informed of architecture phase purpose
- [ ] User informed of what /create-context does (6 context files)
- [ ] User informed of expected interaction (AskUserQuestion for technology choices)
- [ ] User informed of transition options:
  - Proceed to /create-context (greenfield)
  - Review requirements first (optional)
  - Skip to /create-sprint (brownfield with existing context)

### User Confirmation Requested

- [ ] AskUserQuestion used to ask user's preferred next action
- [ ] Options provided:
  - "Create context files" (invoke architecture skill)
  - "Review requirements first" (allow user to review/edit)
  - "Skip - context exists" (brownfield transition)
- [ ] User response recorded and acted upon

---

## Self-Healing Validation

If validation failures detected, attempt self-healing before reporting to user.

### Epic Generation Failures

**If epic count < planned count:**
1. Identify which epics are missing (compare Phase 4 plan with created files)
2. Generate missing epics
3. Re-validate
4. If still failing after 1 retry: Report to user

### Requirements Spec Completeness

**If requirements spec sections are empty:**
1. Identify empty sections (functional reqs, NFRs, data models, integrations)
2. Regenerate missing sections using Phase 2 discovery data
3. Re-validate
4. If still failing after 1 retry: Report to user

### Complexity Assessment Errors

**If complexity score is invalid:**
1. Recalculate using complexity-assessment-matrix.md
2. Verify all 4 dimensions scored
3. Ensure sub-scores don't exceed maximums
4. Update requirements spec with corrected assessment
5. Re-validate
6. If still failing after 1 retry: Report to user

---

## Usage Instructions

**When to Use This Reference:**
- Phase 6.4: Self-Validation (new phase added to skill)
- After all artifacts generated, before reporting completion to user
- Loaded only when validation needed

**How to Use Checklists:**
1. Load this reference: `Read(file_path=".claude/skills/devforgeai-ideation/references/validation-checklists.md")`
2. Execute each checklist section sequentially
3. Use validation commands provided to check artifacts
4. Attempt self-healing for failures
5. Report validation status to user

**Validation Flow:**
```
Phase 6.4 Start
   ↓
Artifact Creation Checklist (files exist?)
   ↓
Epic Content Quality Checklist (epics complete?)
   ↓
Requirements Spec Quality Checklist (requirements complete?)
   ↓
Complexity Assessment Checklist (scoring valid?)
   ↓
Handoff Readiness Checklist (ready for next phase?)
   ↓
User Communication Checklist (user informed?)
   ↓
Phase 6.4 Complete → Report ✅ Ideation Complete
```

**Progressive Disclosure:**
- This reference file only loaded in Phase 6.4
- Does not consume tokens during Phases 1-5
- Can be updated without modifying skill logic
- Provides comprehensive validation without bloating skill file
