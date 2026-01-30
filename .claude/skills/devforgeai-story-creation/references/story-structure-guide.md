# Story Structure Guide

Complete reference for story document structure, YAML frontmatter fields, required sections, and formatting standards.

## Purpose

This guide defines the canonical structure for DevForgeAI story documents, ensuring consistency across all stories and enabling automated validation.

---

## YAML Frontmatter Reference

### Required Fields

**`id`** (String)
- Format: `STORY-XXX` where XXX is zero-padded 3-digit number
- Examples: `STORY-001`, `STORY-042`, `STORY-156`
- Must match filename prefix
- Sequential numbering (no gaps unless intentional)

**`title`** (String)
- Length: 5-80 characters
- Format: Sentence case, descriptive
- Examples: "User registration with email verification", "Shopping cart checkout"
- Avoid: Technical jargon, vague terms like "Implement feature"

**`epic`** (String or null)
- Format: `EPIC-XXX` or `null`
- Must reference existing epic document
- Examples: `EPIC-001`, `EPIC-003`, `null`

**`sprint`** (String)
- Format: `SPRINT-XXX` or `"Backlog"`
- Examples: `SPRINT-001`, `SPRINT-005`, `Backlog`
- "Backlog" means not assigned to any sprint

**`status`** (String)
- Initial value: Always `"Backlog"`
- Valid values: See Workflow States section below
- Changed by workflow commands (/dev, /qa, /release)

**`priority`** (String)
- Valid values: `Critical | High | Medium | Low`
- Critical: Blocking other work, urgent
- High: Important for release
- Medium: Should do soon
- Low: Nice to have

**`points`** (Integer)
- Valid values: `1 | 2 | 3 | 5 | 8 | 13 | 21`
- Fibonacci scale for estimation
- Guidelines:
  - 1-2: Trivial, few hours
  - 3-5: Standard, 1-3 days
  - 8: Complex, 3-5 days
  - 13+: Very complex, consider splitting

**`created`** (Date)
- Format: `YYYY-MM-DD`
- Value: Date story was created
- Example: `2025-11-05`

**`updated`** (Date)
- Format: `YYYY-MM-DD`
- Value: Date story was last modified
- Initially same as `created`
- Updated when story content changes

**`assigned_to`** (String or null)
- Format: Developer name or `null`
- Examples: `"John Doe"`, `null`
- Initially `null` (unassigned)
- Set during sprint planning or development

### Optional Fields

**`tags`** (Array of strings)
- Examples: `[feature, security]`, `[bug, critical]`, `[ui, frontend]`
- Used for categorization and filtering
- Initially empty: `[]`

**`blocked_by`** (String or null)
- Format: `STORY-XXX` or `null`
- References story that must be completed first
- Example: `STORY-042` (this story blocked until STORY-042 is done)

**`estimated_hours`** (Integer, optional)
- Hour estimate (alternative to story points)
- Example: `16` (2 days × 8 hours)

**`actual_hours`** (Integer, optional)
- Actual time spent (filled after completion)
- Example: `18` (slightly over estimate)

**`completed`** (Date, optional)
- Format: `YYYY-MM-DD`
- Date story was released to production
- Added when status changes to "Released"

**`qa_approved_date`** (Date, optional)
- Date QA approval granted
- Added when status changes to "QA Approved"

---

## Workflow States

Stories progress through 11 sequential states:

| Status | Description | Next States |
|--------|-------------|-------------|
| **Backlog** | Initial state, not assigned | Architecture |
| **Architecture** | Architecture decisions made | Ready for Dev |
| **Ready for Dev** | Prerequisites met, can start | In Development |
| **In Development** | TDD cycle in progress | Dev Complete |
| **Dev Complete** | Code implemented, tests passing | QA In Progress |
| **QA In Progress** | Quality validation running | QA Approved, QA Failed |
| **QA Approved** | Quality gates passed | Releasing |
| **QA Failed** | Quality gates failed | In Development (fix and retry) |
| **Releasing** | Deployment in progress | Released |
| **Released** | Deployed to production | (terminal state) |
| **Blocked** | Cannot proceed due to dependency | (any state when unblocked) |

---

## Required Sections

Every story MUST include these markdown sections (order matters):

### 1. User Story
```markdown
## User Story

As a [specific role],
I want [specific action/capability],
So that [specific benefit/business value].
```

**Guidelines:**
- Role must be specific: "customer", "admin", "developer" (not generic "user")
- Action must be clear: "register with email", not "sign up"
- Benefit must articulate business value

### 2. Acceptance Criteria
```markdown
## Acceptance Criteria

### AC1: [Criterion title]
**Given** [context/precondition]
**When** [action/trigger]
**Then** [expected outcome]

### AC2: [Criterion title]
**Given** [context/precondition]
**When** [action/trigger]
**Then** [expected outcome]

[... minimum 3 criteria ...]
```

**Guidelines:**
- Minimum 3 acceptance criteria
- Number sequentially (AC1, AC2, AC3, ...)
- Include happy path + error scenarios
- Each must be testable (automated test can verify)
- Use Given/When/Then structure strictly

### 3. Technical Specification
```markdown
## Technical Specification

### API Contracts (if applicable)

#### Endpoint: [METHOD] /api/path

**Request:**
```json
{schema}
```

**Response (2XX):**
```json
{schema}
```

**Response (4XX/5XX):**
```json
{error schema}
```

### Data Models

#### Entity: EntityName
| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| field | Type | Required, Unique | Description |

### Business Rules

1. Rule description
2. Validation logic
3. Calculation formulas

### Dependencies

- External service: Purpose and integration method
- Database: Requirements
```

**Guidelines:**
- API contracts required if story involves HTTP endpoints
- Data models required if story creates/modifies data
- Business rules required if story has validation/calculation logic
- Dependencies list all external services, libraries, infrastructure

---

## Technical Specification Section (v2.0 Structured YAML)

**Format Version:** 2.0 (Current) - Structured YAML format

**Since:** 2025-11-07 (RCA-006 Phase 2)

**Purpose:** Machine-readable technical specifications enabling automated validation and test generation

**Structure:**

````markdown
## Technical Specification

```yaml
technical_specification:
  format_version: "2.0"

  components:
    - type: "Service|Worker|Configuration|Logging|Repository|API|DataModel"
      name: "[ComponentName]"
      file_path: "src/[path]/[file]"
      requirements:
        - id: "[COMP-001]"
          description: "[What must be implemented]"
          testable: true
          test_requirement: "Test: [Specific test assertion]"
          priority: "Critical|High|Medium|Low"

  business_rules:
    - id: "BR-001"
      rule: "[Business rule description]"
      test_requirement: "Test: [Validation test]"

  non_functional_requirements:
    - id: "NFR-001"
      category: "Performance|Security|Scalability|Reliability"
      requirement: "[NFR description]"
      metric: "[Measurable target with numbers]"
      test_requirement: "Test: [How to verify]"
```
````

**Complete schema:** `devforgeai/specs/STRUCTURED-FORMAT-SPECIFICATION.md`

**Legacy v1.0 Format (Deprecated):**

Stories created before 2025-11-07 may use freeform text format. These are still supported (backward compatibility) but should be migrated to v2.0 for Phase 3 automated validation.

**Frontmatter Requirement:**

All v2.0 stories MUST have `format_version: "2.0"` in YAML frontmatter:

```yaml
---
id: STORY-XXX
format_version: "2.0"  # <-- REQUIRED for v2.0
---
```

**Validation:**

Run `validate_tech_spec.py` to verify v2.0 format:
```bash
python3 .claude/skills/devforgeai-story-creation/scripts/validate_tech_spec.py STORY-XXX.story.md
```

---

### 4. UI Specification (if applicable)
```markdown
## UI Specification

### Components

#### Component: ComponentName
**Type:** [Form|Table|Modal|Card|Chart]
**Purpose:** Description
**Data Bindings:** Input/output/state

### Layout Mockup

```
[ASCII mockup using box-drawing characters]
```

### Component Interfaces

```typescript
interface ComponentProps {
  // props
}
```

### User Interactions

1. Step-by-step flow
2. State changes
3. Navigation

### Accessibility

- Keyboard: Tab navigation, Enter/Space actions
- Screen reader: aria-label, role attributes
- Focus: Visual indicators
- Contrast: WCAG AA compliance
```

**Guidelines:**
- Only include if story has user interface components
- ASCII mockup shows visual layout
- Component interfaces use project's language (TypeScript, C#, etc.)
- Accessibility requirements must be specific (not generic)

### 5. Non-Functional Requirements
```markdown
## Non-Functional Requirements

### Performance
- Response time < [X]ms
- Throughput: [X] requests/second
- Page load time < [X]s

### Security
- Authentication: [method]
- Authorization: [RBAC/ABAC]
- Encryption: [at rest/in transit]
- Input validation: [sanitization rules]

### Usability
- [Specific usability requirements]
- Error messages: [guidelines]
- Help text: [requirements]

### Scalability
- Concurrent users: [X]
- Data volume: [X] records
- Growth rate: [X]% per month
```

**Guidelines:**
- All NFRs must be measurable (no vague terms)
- Performance: Specific response time targets
- Security: Specific methods and standards
- Scalability: Quantified user/data targets

### 6. Edge Cases & Error Handling
```markdown
## Edge Cases & Error Handling

1. **Case:** Description of edge case
   **Expected:** Expected behavior

2. **Case:** Description of error condition
   **Expected:** Error handling and recovery
```

**Guidelines:**
- Minimum 2 edge cases
- Cover boundary conditions (min/max values, empty data)
- Cover error conditions (network failures, invalid input)
- Specify expected behavior (not just "show error")

### 7. Definition of Done
```markdown
## Definition of Done

### Implementation
- [ ] All acceptance criteria implemented
- [ ] Unit tests written and passing
- [ ] Integration tests written and passing
- [ ] Code follows coding-standards.md
- [ ] No violations of architecture-constraints.md

### Testing
- [ ] All acceptance criteria have automated tests
- [ ] Test coverage meets thresholds (95%/85%/80%)
- [ ] All tests passing (100% pass rate)

### Documentation
- [ ] Code comments for complex logic
- [ ] API documentation generated

### Security
- [ ] Input validation implemented
- [ ] No hardcoded secrets
- [ ] Security scan passed
```

**Guidelines:**
- Standard checklist for all stories
- Items checked during /dev workflow
- Modified for story-specific requirements
- Validated during /qa workflow

---

## ⚠️ CRITICAL: Deferral Anti-Pattern (RCA-006)

### ❌ ANTI-PATTERN: Pre-Deferring DoD Items

**DO NOT do this during story creation:**

```markdown
## Definition of Done

### Implementation
- [ ] All acceptance criteria implemented
- [ ] Unit tests written and passing
- [ ] Performance benchmarks (P95 < 50ms)
  - **Deferred to STORY-009:** Requires benchmarking infrastructure
  - **Justification:** No infrastructure exists yet
```

**Why this is WRONG:**
1. **Deferrals made BEFORE attempting implementation** - Violates "attempt first" principle
2. **Assumes blockers exist without verifying** - May be based on incorrect assumptions
3. **Bypasses Phase 4.5 Deferral Challenge Checkpoint** - Pre-justified deferrals appear valid
4. **Accumulates premature technical debt** - Creates debt that may not be necessary

---

### ✅ CORRECT PATTERN: Attempt First, Defer Only If Blocked

**Do this during story creation:**

```markdown
## Definition of Done

### Implementation
- [ ] All acceptance criteria implemented
- [ ] Unit tests written and passing
- [ ] Performance benchmarks (P95 < 50ms, P50 < 20ms)
  - **Benchmark file:** benches/api_performance.rs
  - **Success criteria:** Criterion report shows targets met
  - **Note:** If blocked during implementation, document blocker via /dev Phase 4.5
```

**During /dev workflow (Phase 4.5 - Deferral Challenge Checkpoint):**

If developer discovers an ACTUAL blocker during implementation:

1. Developer attempts to implement performance benchmark
2. Discovers blocker: "Cache directory empty, need artifacts from STORY-008"
3. Phase 4.5 prompts user: "How should we handle this incomplete item?"
4. User selects: "Defer to story"
5. User provides: Target story (STORY-009) and reason
6. System updates story file:
   ```markdown
   - [ ] Performance benchmarks (P95 < 50ms, P50 < 20ms)
     - **Deferred to STORY-009:** Requires cached grammar artifacts from STORY-008
     - **Blocker verified:** 2024-11-06 15:30:00 UTC
     - **User approved:** 2024-11-06 15:30:00 UTC
   ```

**Benefits of this approach:**
- ✅ Deferrals based on ACTUAL blockers encountered during implementation
- ✅ "Attempt first" principle enforced automatically
- ✅ User approval required for ALL deferrals (zero autonomous deferrals)
- ✅ Accurate technical debt tracking (only real blockers)
- ✅ Prevents assumptions from becoming permanent debt

---

### Deferral Budget Limits

**Per-Story Constraints:**
- **Maximum 3 deferred items** per story
- **Maximum 20% of total DoD items** can be deferred
- **No circular deferrals** (STORY-A → STORY-B → STORY-A)

**Example:**

```
Total DoD items: 14
Deferred items: 3
Deferral percentage: (3 / 14) * 100 = 21.4%

Status: ❌ OVER BUDGET (exceeds 20% limit)

Recommendation:
- Complete more items to reduce deferrals below 20%
- OR split story into multiple smaller stories
```

**Enforcement:**
- **During /create-story:** Warning if template has pre-deferrals
- **During /dev Phase 4.5:** User challenged on ALL deferrals
- **During /dev Phase 5:** Budget check blocks "Dev Complete" if over limit
- **During /qa:** Stories with excessive deferrals flagged

---

### When Deferrals Are Appropriate

**Valid reasons to defer DoD items:**

1. **External dependency blocking work:**
   - Third-party API not available until specific date
   - Vendor library upgrade required (outside team control)
   - Production environment not ready

2. **Prerequisite story not complete:**
   - Current story requires artifacts from STORY-XXX
   - Cannot test without STORY-XXX deployed
   - Dependency story in "In Development" status

3. **Toolchain/infrastructure unavailable:**
   - Nightly toolchain required but not installed
   - Performance testing infrastructure not set up
   - Security scanning tools not configured

4. **Scope change documented in ADR:**
   - Requirement removed from scope via ADR-XXX
   - Feature descoped due to timeline constraints
   - Business decision to defer feature

**Invalid reasons to defer:**

1. ❌ "Looks hard, will do later"
2. ❌ "Don't know how to implement this"
3. ❌ "Not enough time in sprint"
4. ❌ "Will add tests after release"
5. ❌ "Documentation can wait"

**If reason is invalid:** Complete the work now or split the story.

---

### Template Best Practices

**When creating story templates:**

1. **Start with complete DoD checklist** - All items marked `[ ]` incomplete
2. **Do NOT pre-justify deferrals** - Leave justifications blank
3. **Add notes for guidance** - E.g., "Note: If blocked, document via /dev Phase 4.5"
4. **Focus on success criteria** - Define what "done" looks like
5. **Trust the process** - Phase 4.5 will handle blockers discovered during implementation

**Example of well-structured DoD item:**

```markdown
- [ ] Performance benchmarks meet SLA
  - **Target:** P95 < 50ms, P50 < 20ms
  - **Test file:** benches/api_performance.rs
  - **Success criteria:** Criterion report shows all targets met
  - **Note:** If infrastructure unavailable, Phase 4.5 will prompt for deferral
```

**NOT:**

```markdown
- [ ] Performance benchmarks
  - Deferred to STORY-009: No infrastructure  ← PRE-JUSTIFIED (WRONG)
```

---

### 8. Workflow History
```markdown
## Workflow History

- **{YYYY-MM-DD HH:MM:SS}** - Story created, status: Backlog
- **{YYYY-MM-DD HH:MM:SS}** - {status_change_description}
```

**Guidelines:**
- Initialized with creation timestamp
- Appended by workflow commands (/dev, /qa, /release)
- Format: `- **{timestamp}** - {description}`
- Chronological order (newest at bottom)

---

## Section Ordering

Sections must appear in this order:

1. YAML Frontmatter (delimited by `---`)
2. User Story
3. Acceptance Criteria
4. Technical Specification
5. UI Specification (if applicable)
6. Non-Functional Requirements
7. Edge Cases & Error Handling
8. Definition of Done
9. Workflow History

**Optional sections can be omitted:**
- UI Specification (if no UI components)
- API Contracts (if no HTTP endpoints)

**Never omit:**
- User Story
- Acceptance Criteria
- Non-Functional Requirements
- Definition of Done
- Workflow History

---

## Markdown Formatting Standards

### Headers

- Use `##` for main sections (User Story, Acceptance Criteria, etc.)
- Use `###` for subsections (AC1, AC2, API endpoints, etc.)
- Use `####` for sub-subsections (Entity names, Component names, etc.)

### Lists

- Use `-` for unordered lists
- Use `1.`, `2.`, `3.` for ordered lists
- Use `- [ ]` for checkboxes (Definition of Done)

### Code Blocks

- Use triple backticks with language: ```json, ```typescript, ```python
- Always specify language for syntax highlighting
- Use `bash` for command-line examples

### Tables

- Use GitHub-flavored markdown tables
- Include header row with `|---|---|` separator
- Align columns for readability (though not required)

### Emphasis

- Use **bold** for Given/When/Then keywords
- Use `code` for field names, variables, filenames
- Use *italic* sparingly (prefer bold for emphasis)

---

## File Naming Convention

**Format:** `{STORY-ID}-{slug}.story.md`

**Slug generation:**
- Convert title to lowercase
- Replace spaces with hyphens
- Remove special characters (except hyphens)
- Limit to 50 characters

**Examples:**
- Title: "User Registration with Email Verification"
- Filename: `STORY-042-user-registration-with-email-verification.story.md`

- Title: "Shopping Cart Checkout & Payment"
- Filename: `STORY-018-shopping-cart-checkout-and-payment.story.md`

---

## Validation Rules

### Frontmatter Validation

```
Required field checks:
- id matches filename prefix
- title is 5-80 characters
- epic is EPIC-XXX format or null
- sprint is SPRINT-XXX or "Backlog"
- status is valid workflow state
- priority is Critical|High|Medium|Low
- points is Fibonacci number (1,2,3,5,8,13,21)
- created is YYYY-MM-DD format
- updated is YYYY-MM-DD format
- assigned_to is string or null
```

### Content Validation

```
Section presence:
- [ ] User Story section exists
- [ ] Acceptance Criteria section exists (min 3 criteria)
- [ ] Technical Specification section exists
- [ ] Non-Functional Requirements section exists
- [ ] Definition of Done section exists
- [ ] Workflow History section exists

Section ordering:
- [ ] Sections appear in correct order (see Section Ordering above)

Format validation:
- [ ] User story follows As a/I want/So that format
- [ ] All AC follow Given/When/Then format
- [ ] All code blocks have language specified
- [ ] All tables have header separators
```

---

## Common Mistakes to Avoid

### ❌ Vague User Story
```
Bad: "As a user, I want to login, so that I can access the app"
```
**Problems:**
- Generic role ("user" - be specific)
- Obvious benefit (doesn't explain WHY login matters)

**Good:**
```
As a registered customer,
I want to login with my email and password,
So that I can access my order history and track shipments securely.
```

### ❌ Non-Testable Acceptance Criteria
```
Bad: "System should be fast and responsive"
```
**Problems:**
- "should" is ambiguous (not definitive)
- "fast" is not measurable
- Cannot write automated test

**Good:**
```
### AC5: Performance meets targets
**Given** user initiates login request
**When** credentials are validated
**Then** response returns within 500ms (95th percentile)
```

### ❌ Missing Given/When/Then Structure
```
Bad: "User can reset their password"
```
**Problems:**
- No context (when does this happen?)
- No trigger (how does user initiate?)
- No outcome (what happens after?)

**Good:**
```
### AC6: Password reset flow
**Given** user is on login page and has forgotten password
**When** user clicks "Forgot Password" and enters email
**Then** password reset email is sent within 30 seconds
```

### ❌ Incomplete Technical Specification
```
Bad:
### API Contracts
POST /api/register
```
**Problems:**
- No request schema
- No response schema
- No error responses
- No validation rules

**Good:**
```
### API Contracts

#### Endpoint: POST /api/auth/register

**Request:**
```json
{
  "email": "user@example.com",
  "password": "SecurePass123!",
  "name": "John Doe"
}
```

**Response (201 Created):**
```json
{
  "user_id": "uuid-v4",
  "email": "user@example.com",
  "verification_sent": true
}
```

**Response (400 Bad Request):**
```json
{
  "error": "Validation failed",
  "details": [
    "Password must be at least 8 characters",
    "Email format invalid"
  ]
}
```

**Validation Rules:**
- email: Required, email format, unique
- password: Required, min 8 chars, must include uppercase, lowercase, number, special char
- name: Required, 2-100 chars
```

### ❌ Vague Non-Functional Requirements
```
Bad:
### Performance
- System should be fast
- Good user experience
```
**Problems:**
- "fast" not measurable
- "good UX" subjective

**Good:**
```
### Performance
- API response time <500ms for 95th percentile
- Page load time <2 seconds on 3G connection
- Support 1,000 concurrent users
- Database queries <100ms
```

---

## Template Usage

**Base template location:**
`.claude/skills/devforgeai-story-creation/assets/templates/story-template.md`

**Loading template:**
```
template = Read(file_path=".claude/skills/devforgeai-story-creation/assets/templates/story-template.md")
```

**Replacing placeholders:**
```
story_content = template
  .replace("{STORY-ID}", story_id)
  .replace("{TITLE}", title)
  .replace("{EPIC}", epic_id or "null")
  .replace("{USER_STORY}", user_story_text)
  .replace("{ACCEPTANCE_CRITERIA}", ac_markdown)
  .replace("{TECHNICAL_SPEC}", tech_spec_markdown)
  .replace("{UI_SPEC}", ui_spec_markdown or "")
  .replace("{NFRS}", nfr_markdown)
  .replace("{EDGE_CASES}", edge_cases_markdown)
  .replace("{TIMESTAMP}", current_timestamp)
```

---

## Progressive Disclosure

**When to load this reference:**
- Phase 5: Story File Creation (structure and formatting guidance)
- Phase 7: Self-Validation (validation rules for frontmatter and sections)

**Why progressive:**
- Not needed during Phases 1-4 (discovery, requirements, specifications)
- Loaded only when constructing story file
- Saves tokens during early phases (~400 lines not loaded until needed)

---

## Integration with DevForgeAI

**Context files respected:**
- **tech-stack.md:** Technology choices inform technical specification
- **source-tree.md:** File location follows project structure
- **coding-standards.md:** Code examples follow standards
- **architecture-constraints.md:** Technical spec respects layer boundaries
- **anti-patterns.md:** Story avoids forbidden patterns

**Used by:**
- devforgeai-story-creation skill (this skill)
- devforgeai-orchestration skill (story lifecycle management)
- devforgeai-development skill (TDD implementation)
- devforgeai-qa skill (validation against acceptance criteria)

---

**This guide ensures all stories follow consistent structure, enabling automated processing and zero ambiguity during implementation.**
