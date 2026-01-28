---
# Story Template Metadata
template_version: "2.7"
last_updated: "2026-01-21"
format_version: "2.7"
template_updated: 2026-01-21
---

# =============================================================================
# STORY TEMPLATE CHANGELOG
# =============================================================================
#
# Version: 2.5
# Last Updated: 2025-12-29
# Maintained by: devforgeai-story-creation skill
#
# Version History:
#
# v2.7 (2026-01-21) - Provenance XML Section (STORY-291, EPIC-049)
#   Changes:
#     - Added provenance XML section after Description for context traceability
#     - Added <origin> element with document, section attributes and quote, line_reference, quantified_impact children
#     - Added <decision> element with rationale attribute and selected, rejected, trade_off children
#     - Added <stakeholder> element with role, goal attributes and quote, source children
#     - Added <hypothesis> element with id, validation, success_criteria attributes
#     - Bumped template_version and format_version to 2.7
#   Rationale:
#     - BMAD "Artifacts Travel With Work" pattern (RESEARCH-003)
#     - Preserves brainstorm context in downstream documents
#     - Enables traceability from story to original business rationale
#   References:
#     - STORY-291: Add Provenance XML Section to Story Template v2.7
#     - EPIC-049: Context Preservation and Provenance Tracking
#     - RESEARCH-003: BMAD-METHOD
#
# v2.6 (2026-01-19) - XML Acceptance Criteria Format (STORY-280, EPIC-046)
#   Changes:
#     - Added XML acceptance criteria format with <acceptance_criteria> blocks
#     - Added complete example with all optional elements (verification, source_files)
#     - Added source_files guidance section for verification hints
#     - Added backward compatibility note: legacy markdown AC NOT supported by verification
#     - Bumped template_version and format_version to 2.6
#   Rationale:
#     - Machine-parseable AC enables automated compliance verification
#     - ac-compliance-verifier subagent requires XML format
#     - Standardized schema per coding-standards.md (lines 362-460)
#   References:
#     - STORY-280: Story Template Update for XML AC Format
#     - EPIC-046: AC Compliance Verification System
#     - coding-standards.md: XML Acceptance Criteria Schema
#
# v2.5 (2025-12-29) - Unified Change Log Section (STORY-152)
#   Changes:
#     - Replaced ## Workflow Status with unified ## Change Log section
#     - Added **Current Status:** field for tracking story state
#     - Added 5-column table: Date | Author | Phase/Action | Change | Files Affected
#     - Initial entry with author `claude/story-requirements-analyst`
#     - Shared reference guide at .claude/references/changelog-update-guide.md
#   Rationale:
#     - Unified tracking of workflow state AND file modifications
#     - Subagent attribution for audit trail
#     - Single source of truth for story history
#   References:
#     - STORY-152: Unified Story Change Log Tracking
#     - .claude/references/changelog-update-guide.md
#
# v2.4 (2025-12-23) - Story Type Classification (STORY-126)
#   Changes:
#     - Added type field to YAML frontmatter for TDD phase skipping
#     - Valid types: feature (default), documentation, bugfix, refactor
#     - Positioned after title field, before epic field
#     - Default value: feature (full TDD workflow)
#   Phase Skip Matrix:
#     - feature: No phases skipped (full TDD)
#     - documentation: Skip Phase 05 Integration (no runtime code)
#     - bugfix: Skip Phase 04 Refactor (minimal changes)
#     - refactor: Skip Phase 02 Red (tests exist)
#   Backward Compatibility:
#     - Stories without type field default to "feature"
#     - No migration required for existing stories
#   References:
#     - STORY-126: Story Type Detection & Phase Skipping
#     - coding-standards.md: Story Type Classification section
#
# v2.3 (2025-12-21) - Technical Limitations Section
#   Changes:
#     - Added Technical Limitations section after Technical Specification
#     - YAML format with id, component, limitation, decision, discovered_phase, impact
#     - Decision options: pending, defer:STORY-XXX, descope:ADR-XXX, workaround:description
#     - Discovered phase options: Architecture, Development, QA
#   Rationale:
#     - Prevents late-stage discovery of tool/technology blockers
#     - Enables explicit decision-making for capability gaps
#     - Provides audit trail for limitation handling
#   References:
#     - Architectural review of /qa STORY-118 execution
#
# v2.2 (2025-12-14) - STORY-090 Parallel Development Support
#   Changes:
#     - Added depends_on field to YAML frontmatter for dependency declaration
#     - Format: Array of STORY-NNN IDs (e.g., ["STORY-044", "STORY-045"])
#     - Positioned after points field, before status field
#     - Default value: empty array [] for stories with no dependencies
#     - Enables Feature 3 (Dependency Graph Enforcement) for parallel development workflows
#   Backward Compatibility:
#     - Compatible with v2.1 stories (depends_on field optional for existing stories)
#     - Stories without depends_on field treated as having no dependencies (empty array)
#   Rationale:
#     - Prerequisite for EPIC-010 (Parallel Story Development)
#     - Standardized array format enables automated dependency validation
#   References:
#     - STORY-090: Update Story Template to v2.2 with depends_on Field
#     - EPIC-010: Parallel Story Development with CI/CD Integration
#
# v2.1 (2025-01-21) - RCA-012 Remediation
#   Changes:
#     - Removed checkbox syntax from AC headers
#     - Format change: '### 1. [ ] Title' → '### AC#1: Title'
#     - Rationale: AC headers are definitions, not completion trackers
#     - Three-layer tracking system clarified in CLAUDE.md:
#       * TodoWrite (AI phase-level monitoring)
#       * AC Verification Checklist (granular sub-item tracking)
#       * Definition of Done (official completion record)
#   Impact:
#     - Eliminates user confusion about unchecked AC headers
#     - Clarifies AC headers are static definitions
#     - All future stories (58+) benefit from clear format
#   References:
#     - RCA-012: devforgeai/RCA/RCA-012/
#     - Root cause: Vestigial checkboxes from pre-RCA-011 design
#
# v2.0 (2025-10-30) - Structured Tech Spec (RCA-006 Phase 2)
#   Changes:
#     - Added technical_specification YAML code block
#     - Machine-readable component definitions (Service, Worker, API, etc.)
#     - Test requirements embedded in each component
#     - Improved deterministic parsing for test generation
#   Impact:
#     - Test generation accuracy improved (85% → 95%+)
#     - Validation automation enabled
#   References:
#     - devforgeai/specs/STRUCTURED-FORMAT-SPECIFICATION.md
#
# v1.0 (Initial) - Original Template
#   Features:
#     - User story format (As a... I want... So that...)
#     - AC headers with checkbox syntax (vestigial as of v2.1)
#     - Freeform technical specification
#     - Definition of Done section
#
# Migration Paths:
#   v1.0 → v2.0: Gradual (on story update)
#   v2.0 → v2.1: Optional script available
#     Location: .claude/skills/devforgeai-story-creation/scripts/migrate-ac-headers.sh
#     Documentation: devforgeai/RCA/RCA-012/MIGRATION-SCRIPT.md
#
# Backward Compatibility:
#   All versions (v1.0, v2.0, v2.1) supported by framework
#   Old stories continue to work without migration
#   Migration is optional (for visual consistency only)
#
# =============================================================================

---
id: STORY-XXX
title: [Story Title - What is being built]
type: feature
# Story type controls TDD phase skipping. Options: feature (default), documentation, bugfix, refactor
epic: EPIC-XXX
sprint: SPRINT-XXX
status: Backlog
points: [Story points: 1, 2, 3, 5, 8, 13]
depends_on: []
# Array of STORY-NNN IDs this story depends on. Examples: [], ["STORY-044"], ["STORY-044", "STORY-045"]
priority: [High / Medium / Low]
assigned_to: [Developer Name]
created: YYYY-MM-DD
format_version: "2.7"
---

# Story: [Title]

## Description

**As a** [user role/persona],
**I want** [capability/feature],
**so that** [business value/benefit].

**Example:**
As a returning customer, I want to use my saved payment method during checkout, so that I can complete purchases faster without re-entering card details.

## Provenance

> **Optional Section:** Include when story originates from a brainstorm document or has traceable business rationale. Omit for stories created without brainstorm reference.

Document the origin, decisions, stakeholders, and hypotheses that led to this story. Uses XML format for Claude's fine-tuned XML attention.

```xml
<provenance>
  <!-- Origin: Link to source brainstorm document with quoted evidence -->
  <origin document="BRAINSTORM-NNN" section="problem-statement">
    <quote>"Exact quote from brainstorm document"</quote>
    <line_reference>lines XX-YY</line_reference>
    <quantified_impact>Measurable impact statement</quantified_impact>
  </origin>

  <!-- Decision: Document the approach selected and alternatives rejected -->
  <decision rationale="selected-over-alternatives">
    <selected>Chosen approach description</selected>
    <rejected alternative="alternative-name">
      Why this alternative was not chosen
    </rejected>
    <trade_off>Trade-off accepted with this decision</trade_off>
  </decision>

  <!-- Stakeholder: Link to specific stakeholder goals from brainstorm -->
  <stakeholder role="Role Title" goal="stakeholder-goal-id">
    <quote>"Stakeholder quote from brainstorm"</quote>
    <source>BRAINSTORM-NNN, section name</source>
  </stakeholder>

  <!-- Hypothesis: Link to testable hypotheses from brainstorm -->
  <hypothesis id="H1" validation="validation-method" success_criteria="measurable criteria">
    Hypothesis statement to be validated
  </hypothesis>
</provenance>
```

### Provenance Elements Reference

| Element | Required | Attributes | Children | Purpose |
|---------|----------|------------|----------|---------|
| `<origin>` | Yes (if provenance present) | `document`, `section` | `<quote>`, `<line_reference>`, `<quantified_impact>` | Link to source document |
| `<decision>` | Optional | `rationale` | `<selected>`, `<rejected>`, `<trade_off>` | Document approach selection |
| `<stakeholder>` | Optional (repeatable) | `role`, `goal` | `<quote>`, `<source>` | Link to stakeholder goals |
| `<hypothesis>` | Optional (repeatable) | `id`, `validation`, `success_criteria` | Text content | Link to testable hypotheses |

---

## Acceptance Criteria

Define testable, specific conditions that must be met for story completion. Use XML format with `<acceptance_criteria>` blocks for machine-parseable verification.

> **IMPORTANT:** XML acceptance criteria format is REQUIRED for automated verification by the ac-compliance-verifier subagent. Legacy markdown format (Given/When/Then bullets) is NOT supported by verification tools.

### XML Acceptance Criteria Format

Use the following XML schema for each acceptance criterion:

```xml
<acceptance_criteria id="AC1" implements="COMP-XXX,COMP-YYY">
  <given>Initial context or precondition</given>
  <when>Action or event being tested</when>
  <then>Expected outcome or result</then>
  <verification>
    <source_files>
      <file hint="Main implementation">path/to/source.py</file>
      <file hint="Helper module">path/to/helper.py</file>
    </source_files>
    <test_file>path/to/test.py</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

### AC#1: [Criterion 1 Title]

```xml
<acceptance_criteria id="AC1">
  <given>[Initial context/state]</given>
  <when>[Action/event occurs]</when>
  <then>[Expected outcome]</then>
</acceptance_criteria>
```

---

### AC#2: [Criterion 2 Title]

```xml
<acceptance_criteria id="AC2" implements="COMP-001">
  <given>[Initial context/state]</given>
  <when>[Action/event occurs]</when>
  <then>[Expected outcome]</then>
  <verification>
    <source_files>
      <file>[path/to/source.ext]</file>
    </source_files>
    <test_file>[path/to/test.ext]</test_file>
  </verification>
</acceptance_criteria>
```

---

### AC#3: [Criterion 3 Title] - Complete Example

This example shows ALL optional elements for comprehensive verification:

```xml
<acceptance_criteria id="AC3" implements="COMP-002,COMP-003">
  <given>Shopping cart has items and user is authenticated</given>
  <when>User clicks checkout button</when>
  <then>Order is created with correct total and confirmation displayed</then>
  <verification>
    <source_files>
      <file hint="Checkout service">src/cart/checkout.py</file>
      <file hint="Order creation">src/orders/service.py</file>
      <file hint="Total calculation">src/cart/calculator.py</file>
    </source_files>
    <test_file>tests/STORY-XXX/test_ac3_checkout.py</test_file>
    <coverage_threshold>95</coverage_threshold>
  </verification>
</acceptance_criteria>
```

---

### AC#4: [Criterion 4 Title]

```xml
<acceptance_criteria id="AC4">
  <given>[Initial context/state]</given>
  <when>[Action/event occurs]</when>
  <then>[Expected outcome]</then>
</acceptance_criteria>
```

---

*Add more criteria as needed (typically 3-7 per story)*

---

### Source Files Guidance

The `<source_files>` element provides hints to the ac-compliance-verifier about where implementation code is located.

**When to Include Source Files:**
- For ACs that modify or create specific files
- When implementation spans multiple files
- When verification needs to locate test coverage targets

**How to Use Hints:**
- `hint` attribute describes what the file contains
- Use relative paths from project root
- List ALL files that implement the AC

**Example Hints:**
- `hint="Main service implementation"` - Primary business logic
- `hint="Repository layer"` - Data access code
- `hint="API controller"` - Endpoint handling
- `hint="Configuration"` - Settings/config changes

**How Verification Uses Source Files:**
1. ac-compliance-verifier reads `<source_files>` from AC
2. Locates corresponding test file from `<test_file>`
3. Validates tests exist for each source file
4. Checks coverage threshold is met for listed files

## Technical Specification

```yaml
technical_specification:
  format_version: "2.0"

  components:
    # Service Component Example
    - type: "Service"
      name: "[ServiceName]"
      file_path: "src/[path]/[ServiceName].cs"
      interface: "[IServiceInterface]"
      lifecycle: "Singleton|Scoped|Transient"
      dependencies:
        - "[IDependency1]"
        - "[IDependency2]"
      requirements:
        - id: "SVC-001"
          description: "[What this service must do]"
          testable: true
          test_requirement: "Test: [Specific test for this requirement]"
          priority: "Critical|High|Medium|Low"

    # Worker Component Example
    - type: "Worker"
      name: "[WorkerName]"
      file_path: "src/[path]/[WorkerName].cs"
      interface: "BackgroundService|IHostedService"
      polling_interval_ms: 30000
      dependencies:
        - "[IDependency]"
      requirements:
        - id: "WKR-001"
          description: "[What this worker must do]"
          testable: true
          test_requirement: "Test: [Specific test for this requirement]"
          priority: "Critical|High|Medium|Low"

    # Configuration Component Example
    - type: "Configuration"
      name: "appsettings.json"
      file_path: "src/[path]/appsettings.json"
      required_keys:
        - key: "[ConfigSection.KeyName]"
          type: "string|int|bool|array|object"
          example: "[Example value]"
          required: true|false
          default: "[Default value if any]"
          validation: "[Validation rules]"
          test_requirement: "Test: [How to verify this config loads]"

    # Logging Component Example
    - type: "Logging"
      name: "Serilog|NLog|log4net"
      file_path: "src/[path]/Program.cs"
      sinks:
        - name: "File|EventLog|Database|Console"
          path: "[Log file path]"
          test_requirement: "Test: [How to verify this sink works]"

    # Repository Component Example
    - type: "Repository"
      name: "[RepositoryName]"
      file_path: "src/Infrastructure/Repositories/[RepositoryName].cs"
      interface: "[IRepositoryInterface]"
      data_access: "Dapper|EF Core|Prisma|Raw SQL"
      entity: "[EntityName]"
      table: "[dbo].[TableName]"
      requirements:
        - id: "REPO-001"
          description: "[What this repository must do]"
          testable: true
          test_requirement: "Test: [Specific test for this requirement]"
          priority: "Critical|High|Medium|Low"

    # API Endpoint Component Example
    - type: "API"
      name: "[EndpointName]"
      endpoint: "/api/[resource]/[action]"
      method: "GET|POST|PUT|PATCH|DELETE"
      authentication:
        required: true|false
        method: "Bearer Token|API Key|OAuth2"
        scopes: ["scope1", "scope2"]
      request:
        content_type: "application/json"
        schema:
          field1:
            type: "string|number|boolean|object|array"
            required: true|false
            validation: "[Validation rules]"
      response:
        success:
          status_code: 200|201|204
          schema:
            id: "UUID"
            field: "string"
        errors:
          - status_code: 400|401|403|404|422|500
            condition: "[When this error occurs]"
            schema:
              error: "string"
              message: "string"
      requirements:
        - id: "API-001"
          description: "[What this API must do]"
          testable: true
          test_requirement: "Test: [Specific test for this requirement]"
          priority: "Critical|High|Medium|Low"

    # DataModel Component Example
    - type: "DataModel"
      name: "[EntityName]"
      table: "[dbo].[TableName]"
      purpose: "[What this model represents]"
      fields:
        - name: "[FieldName]"
          type: "UUID|String|Int|DateTime|Enum|etc"
          constraints: "Primary Key|Required|Unique|etc"
          description: "[Purpose of this field]"
          test_requirement: "Test: [How to validate this field]"
      indexes:
        - name: "IX_[Table]_[Field]"
          fields: ["[Field1]", "[Field2]"]
          unique: true|false
          purpose: "[Why this index exists]"
      relationships:
        - type: "One-to-Many|Many-to-One|Many-to-Many"
          related_entity: "[RelatedEntity]"
          foreign_key: "[FK_Field]"
          cascade: "Cascade|Restrict|SetNull"
          description: "[Relationship purpose]"

  business_rules:
    - id: "BR-001"
      rule: "[Business rule description]"
      trigger: "[When this rule is evaluated]"
      validation: "[How to validate compliance]"
      error_handling: "[What happens if violated]"
      test_requirement: "Test: [How to test this rule]"
      priority: "Critical|High|Medium|Low"

  non_functional_requirements:
    - id: "NFR-001"
      category: "Performance|Security|Scalability|Reliability"
      requirement: "[NFR description]"
      metric: "[Measurable target with numbers]"
      test_requirement: "Test: [How to verify this NFR]"
      priority: "Critical|High|Medium|Low"
```

**Instructions for AI Story Creation:**
1. Use appropriate component types based on what's being built
2. Every requirement/key/sink/field should have a `test_requirement`
3. IDs should be unique and follow pattern: SVC-001, WKR-001, API-001, etc.
4. Priorities: Critical (must have), High (should have), Medium (nice to have), Low (optional)
5. All test requirements start with "Test: " prefix
6. Metrics must be measurable (include numbers, thresholds, or ranges)

**See `devforgeai/specs/STRUCTURED-FORMAT-SPECIFICATION.md` for complete schema reference and examples.**

---

## Technical Limitations

Document known tool or technology limitations discovered during architecture, development, or QA phases. This prevents late-stage discovery of blockers and enables explicit decision-making.

```yaml
technical_limitations:
  # Example entries - remove or replace with actual limitations
  - id: TL-001
    component: "[Component or tool name]"
    limitation: "[Description of what cannot be done]"
    decision: "pending"  # Options: pending | defer:STORY-XXX | descope:ADR-XXX | workaround:description
    discovered_phase: "Architecture"  # Options: Architecture | Development | QA
    impact: "[How this affects the story]"
```

**Decision Options:**
- `pending` - Not yet decided, requires user input
- `defer:STORY-XXX` - Deferred to follow-up story for resolution
- `descope:ADR-XXX` - Removed from scope via Architecture Decision Record
- `workaround:description` - Alternative approach implemented

**When to Add Entries:**
- Tool cannot perform required analysis
- Framework limitation prevents full implementation
- External dependency has capability gaps
- Performance constraints prevent ideal solution

---

## Non-Functional Requirements (NFRs)

### Performance

**Response Time:**
- **API Endpoint 1:** < [X]ms (p95), < [X]ms (p99)
- **API Endpoint 2:** < [X]ms (p95), < [X]ms (p99)

**Throughput:**
- Support [X] requests per second
- Support [X] concurrent users

**Performance Test:**
- Load test with [X] concurrent users
- Verify response time under load
- Verify no memory leaks over [X] hour run

---

### Security

**Authentication:**
- [Required / Optional / None]
- [Auth method: OAuth 2.0, JWT, API Key, etc.]

**Authorization:**
- [Role-based / Permission-based / None]
- Required roles: [Roles that can access this feature]

**Data Protection:**
- Sensitive fields: [List fields requiring encryption/masking]
- Encryption: [At rest / In transit / Both]
- PII handling: [GDPR/CCPA compliance requirements]

**Security Testing:**
- [ ] No SQL injection vulnerabilities
- [ ] No XSS vulnerabilities
- [ ] No hardcoded secrets
- [ ] Proper input validation
- [ ] Proper output encoding
- [ ] Authentication enforced
- [ ] Authorization enforced

**Rate Limiting:**
- [X] requests per [time period] per [IP / user / API key]

---

### Scalability

**Horizontal Scaling:**
- Stateless design: [Yes / No]
- Load balancing: [Required / Not Required]

**Database:**
- Expected data volume: [X] records per [time period]
- Growth rate: [X]% per [time period]
- Indexing strategy: [Described above in Data Models]

**Caching:**
- Cache strategy: [None / Redis / In-Memory]
- Cache TTL: [X] seconds/minutes
- Cache invalidation: [Strategy]

---

### Reliability

**Error Handling:**
- Follow Result Pattern (per coding-standards.md)
- Log all errors with context
- Return user-friendly error messages (no stack traces)

**Retry Logic:**
- Retry transient failures: [Yes / No]
- Max retries: [X]
- Backoff strategy: [Exponential / Linear / Fixed]

**Monitoring:**
- Metrics to track: [List key metrics]
- Alerts: [When to alert, who receives alerts]

---

### Observability

**Logging:**
- Log level: [INFO / DEBUG / WARN / ERROR]
- Log structured data (JSON format)
- Include correlation ID for request tracing
- Do NOT log sensitive data (passwords, tokens, PII)

**Metrics:**
- Request count
- Response time (p50, p95, p99)
- Error rate
- [Custom metric 1]
- [Custom metric 2]

**Tracing:**
- Distributed tracing: [Yes / No]
- Trace all external calls

---

## Dependencies

### Prerequisite Stories

Stories that must complete BEFORE this story can start:

- [ ] **STORY-XXX:** [Story title]
  - **Why:** [Explanation of dependency]
  - **Status:** [Not Started / In Progress / Complete]

- [ ] **STORY-YYY:** [Story title]
  - **Why:** [Explanation of dependency]
  - **Status:** [Not Started / In Progress / Complete]

### External Dependencies

Dependencies outside the team's control:

- [ ] **External Dependency 1:** [Description]
  - **Owner:** [Team/Vendor]
  - **ETA:** [Date]
  - **Status:** [On Track / At Risk / Blocked]
  - **Impact if delayed:** [Description]

- [ ] **External Dependency 2:** [Description]
  - **Owner:** [Team/Vendor]
  - **ETA:** [Date]
  - **Status:** [On Track / At Risk / Blocked]

### Technology Dependencies

New packages or versions required:

- [ ] **Package 1:** [Name] v[Version]
  - **Purpose:** [Why needed]
  - **Approved:** [Yes / Pending]
  - **Added to dependencies.md:** [Yes / No]

- [ ] **Package 2:** [Name] v[Version]
  - **Purpose:** [Why needed]
  - **Approved:** [Yes / Pending]

---

## Test Strategy

### Unit Tests

**Coverage Target:** 95%+ for business logic

**Test Scenarios:**
1. **Happy Path:** [Description of normal flow]
2. **Edge Cases:**
   - [Edge case 1]
   - [Edge case 2]
   - [Edge case 3]
3. **Error Cases:**
   - [Error case 1: null input]
   - [Error case 2: invalid input]
   - [Error case 3: business rule violation]

*Test examples use AAA pattern (Arrange-Act-Assert). Follow language-specific conventions.*

---

### Integration Tests

**Coverage Target:** 85%+ for application layer

**Test Scenarios:**
1. **End-to-End API Flow:** [Description]
2. **Database Integration:** [Description]

---

### E2E Tests (If Applicable)

**Coverage Target:** 10% of total tests (critical paths only)

---

## Acceptance Criteria Verification Checklist

**Purpose:** Real-time progress tracking during TDD implementation. Check off items as each sub-task completes.

**Usage:** The devforgeai-development skill updates this checklist at the end of each TDD phase (Phases 1-5), providing granular visibility into AC completion progress.

**Tracking Mechanisms:**
- **TodoWrite:** Phase-level tracking (AI monitors workflow position)
- **AC Checklist:** AC sub-item tracking (user sees granular progress) ← YOU ARE HERE
- **Definition of Done:** Official completion record (quality gate validation)

### AC#1: [First Acceptance Criterion Title]

- [ ] [Sub-item 1] - **Phase:** [1-5] - **Evidence:** [test file or implementation location]
- [ ] [Sub-item 2] - **Phase:** [1-5] - **Evidence:** [test file or implementation location]
- [ ] [Sub-item 3] - **Phase:** [1-5] - **Evidence:** [test file or implementation location]

### AC#2: [Second Acceptance Criterion Title]

- [ ] [Sub-item 1] - **Phase:** [1-5] - **Evidence:** [test file or implementation location]
- [ ] [Sub-item 2] - **Phase:** [1-5] - **Evidence:** [test file or implementation location]

### AC#3: [Third Acceptance Criterion Title]

- [ ] [Sub-item 1] - **Phase:** [1-5] - **Evidence:** [test file or implementation location]
- [ ] [Sub-item 2] - **Phase:** [1-5] - **Evidence:** [test file or implementation location]

---

**Checklist Progress:** {checked}/{total} items complete ({percentage}%)

---

## Definition of Done

### Implementation
- [ ] [Implementation checklist item 1]
- [ ] [Implementation checklist item 2]
- [ ] [Implementation checklist item 3]
- [ ] [Implementation checklist item 4]

### Quality
- [ ] All [X] acceptance criteria have passing tests
- [ ] Edge cases covered ([list specific edge cases])
- [ ] Data validation enforced ([specify validation rules])
- [ ] NFRs met ([specify which NFRs])
- [ ] Code coverage >[X]% for [module/component]

### Testing
- [ ] Unit tests for [module 1]
- [ ] Unit tests for [module 2]
- [ ] Integration tests for [feature 1]
- [ ] Integration tests for [feature 2]
- [ ] E2E test: [critical user journey]

### Documentation
- [ ] [Document type 1]: [What is documented]
- [ ] [Document type 2]: [What is documented]
- [ ] [Document type 3]: [What is documented]

---

## Change Log

**Current Status:** Backlog

| Date | Author | Phase/Action | Change | Files Affected |
|------|--------|--------------|--------|----------------|
| YYYY-MM-DD HH:MM | claude/story-requirements-analyst | Created | Story created | STORY-XXX.story.md |

## Notes

[Additional context, design decisions, clarifications, or open questions]

**Backward Compatibility - Acceptance Criteria Format:**
> **Legacy markdown AC format (Given/When/Then bullets) is NOT supported by automated verification.**
> The ac-compliance-verifier subagent requires XML `<acceptance_criteria>` blocks to parse and verify ACs.
> Stories created before v2.6 may have markdown AC format - these will need manual verification or migration to XML format.

**Design Decisions:**
- [Decision 1 and rationale]
- [Decision 2 and rationale]

**Open Questions:**
- [ ] [Question 1] - **Owner:** [Name] - **Due:** [Date]
- [ ] [Question 2] - **Owner:** [Name] - **Due:** [Date]

**Related ADRs:**
- [ADR-XXX: [Title]](../ADRs/ADR-XXX.md)

**References:**
- [External documentation link]
- [Design mockup link]
- [Related ticket/issue]

---

Story Template Version: 2.7
Last Updated: 2026-01-21
