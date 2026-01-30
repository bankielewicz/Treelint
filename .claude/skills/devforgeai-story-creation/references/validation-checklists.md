# Story Validation Checklists

Comprehensive validation checklists for Phase 7 (Self-Validation) to ensure story quality and completeness before finishing story creation.

## Purpose

These checklists provide step-by-step validation logic with self-healing procedures to catch issues early and ensure all stories meet DevForgeAI quality standards.

---

## YAML Frontmatter Validation

### Required Fields Checklist

```
Read story file (first 20 lines for frontmatter):

Validate presence:
- [ ] id field exists
- [ ] title field exists
- [ ] epic field exists (can be null)
- [ ] sprint field exists (can be "Backlog")
- [ ] status field exists
- [ ] priority field exists
- [ ] points field exists
- [ ] created field exists
- [ ] updated field exists
- [ ] assigned_to field exists (can be null)

If any field missing:
    CRITICAL: Incomplete frontmatter
    # Self-healing: Regenerate frontmatter with all fields
    # Edit story file
    # Retry validation
```

### Field Format Validation

```
Validate formats:

id:
- [ ] Matches pattern: STORY-\d{3}
- [ ] Matches filename prefix
- [ ] Example: STORY-042
- If invalid: Regenerate to match filename

title:
- [ ] Length: 5-80 characters
- [ ] Not empty, not placeholder ("TBD", "TODO")
- If invalid: Extract from feature description

epic:
- [ ] Format: EPIC-\d{3} or null
- [ ] If not null, epic file exists
- If epic missing: Set to null or ask user

sprint:
- [ ] Format: SPRINT-\d{1,3} or "Backlog"
- [ ] If not "Backlog", sprint file exists
- If sprint missing: Set to "Backlog"

status:
- [ ] One of: Backlog, Architecture, Ready for Dev, In Development,
              Dev Complete, QA In Progress, QA Approved, QA Failed,
              Releasing, Released, Blocked
- [ ] Initially should be "Backlog"
- If invalid: Set to "Backlog"

priority:
- [ ] One of: Critical, High, Medium, Low
- If invalid: Set to "Medium" (default)

points:
- [ ] One of: 1, 2, 3, 5, 8, 13, 21
- [ ] Is Fibonacci number
- If invalid: Ask user or default to 5

created:
- [ ] Format: YYYY-MM-DD
- [ ] Is valid date
- [ ] Not future date
- If invalid: Set to today

updated:
- [ ] Format: YYYY-MM-DD
- [ ] Is valid date
- [ ] >= created date
- If invalid: Set to today

assigned_to:
- [ ] String or null
- If present, non-empty
```

---

## User Story Format Validation

### Structure Validation

```
Read User Story section:

Validate format:
- [ ] Starts with "As a" or "As an"
- [ ] Contains "I want" or "I want to"
- [ ] Contains "So that" or "So I can"
- [ ] Complete sentence structure

If validation fails:
    HIGH: User story format incorrect

    # Self-healing: Extract role, action, benefit
    role = extract_text_between("As a", ",")
    action = extract_text_between("I want", ",")
    benefit = extract_text_between("So that", ".")

    # Regenerate:
    user_story = f"As a {role},\nI want {action},\nSo that {benefit}."

    # Edit story file
    # Retry validation
```

### Content Quality Validation

```
Validate user story content:

Role:
- [ ] Is specific (not "user", but "customer", "admin", "developer")
- [ ] Matches domain context
- If generic: Ask user for specific role

Action:
- [ ] Is clear and unambiguous
- [ ] Describes WHAT user wants (not HOW to implement)
- [ ] Verb-based (e.g., "view", "create", "update", "delete")
- If vague: Ask user for clarification

Benefit:
- [ ] Articulates business value or user value
- [ ] Explains WHY this matters
- [ ] Not obvious/redundant (e.g., "so I can use the feature")
- If weak: Ask user "What value does this provide?"
```

---

## Acceptance Criteria Validation

### Count and Structure

```
Read Acceptance Criteria section:

Validate count:
- [ ] Minimum 3 acceptance criteria
- [ ] Each has unique number (AC1, AC2, AC3, ...)
- [ ] Numbers are sequential (no gaps)

If < 3 criteria:
    HIGH: Insufficient acceptance criteria

    # Self-healing:
    # Analyze user story and generate missing criteria
    # Common patterns:
    #   - Happy path (if missing)
    #   - Validation error scenario (if missing)
    #   - Edge case scenario (if missing)

    # Edit story file to add criteria
    # Retry validation
```

### Given/When/Then Structure

```
For each acceptance criterion:

Validate structure:
- [ ] Has "Given" (precondition/context)
- [ ] Has "When" (action/trigger)
- [ ] Has "Then" (expected outcome)
- [ ] All three keywords present and in order

If structure invalid:
    MEDIUM: Acceptance criterion format incorrect

    # Self-healing: Reformat criterion
    # Extract condition, action, outcome from text
    # Restructure as:
    # **Given** {condition}
    # **When** {action}
    # **Then** {outcome}

    # Edit story file
    # Retry validation
```

### Testability Validation

```
For each acceptance criterion:

Validate testability:
- [ ] Outcome is verifiable (can write assertion)
- [ ] No subjective terms ("good", "nice", "intuitive")
- [ ] No vague terms ("fast", "responsive" without metrics)
- [ ] Specific enough to write automated test

Examples of testable vs non-testable:

✅ Testable:
"Then user is redirected to /dashboard page"
→ Can assert: expect(currentUrl).toBe('/dashboard')

❌ Not testable:
"Then user has a good experience"
→ Cannot automate "good experience" check

✅ Testable:
"Then response time is less than 500ms"
→ Can assert: expect(responseTime).toBeLessThan(500)

❌ Not testable:
"Then response is fast"
→ "fast" is subjective

If not testable:
    HIGH: Acceptance criterion not testable

    # Ask user to quantify
    # Example: "fast" → "< 500ms response time"
    # Edit story file
    # Retry validation
```

### Coverage Validation

```
Validate acceptance criteria coverage:

Required scenarios:
- [ ] At least 1 happy path (successful completion)
- [ ] At least 1 validation error scenario
- [ ] At least 1 edge case or boundary condition

Recommended scenarios:
- [ ] Concurrent access (if applicable)
- [ ] Missing/invalid data
- [ ] Permission/authorization checks
- [ ] Integration failure (if external dependencies)

If missing critical scenarios:
    MEDIUM: Incomplete acceptance criteria coverage

    # Self-healing: Generate missing scenarios
    # Edit story file to add
    # Retry validation
```

---

## Technical Specification Validation

### Format Version Validation (RCA-006 Phase 2)

**For stories created after 2025-11-07, validate v2.0 structured format:**

```python
# Check frontmatter
frontmatter = extract_yaml_frontmatter(story_content)
format_version = frontmatter.get("format_version", "1.0")

if format_version == "2.0":
    # v2.0 validation required
    validate_structured_yaml(story_content)
elif format_version == "1.0":
    # Legacy format (acceptable for backward compatibility)
    log_warning("Story uses legacy v1.0 format (consider migration)")
else:
    error("Unknown format version: {format_version}")
```

**Expected:** `format_version: "2.0"` for all new stories

### YAML Syntax Validation (v2.0 Stories)

**For v2.0 stories, validate YAML syntax:**

```python
# Extract YAML block
yaml_match = re.search(r"## Technical Specification\s+```yaml\s+(.*?)\s+```",
                        story_content, re.DOTALL)

if not yaml_match:
    error("v2.0 story missing YAML block in Technical Specification")

# Parse YAML
try:
    tech_spec = yaml.safe_load(yaml_match.group(1))
except yaml.YAMLError as e:
    error(f"Invalid YAML syntax: {e}")
```

**Expected:** Valid YAML, parseable without errors

### Structured Content Validation (v2.0 Stories)

**Validate technical_specification contains required sections:**

```python
required_sections = ["format_version", "components"]
tech_spec = tech_spec.get("technical_specification", {})

for section in required_sections:
    if section not in tech_spec:
        error(f"Missing required section: {section}")
```

**Expected:**
- `format_version: "2.0"` within YAML
- `components: [...]` array with at least 1 component

### Component Validation (v2.0 Stories)

**For each component, validate required fields:**

```python
for idx, component in enumerate(tech_spec["components"]):
    # Validate type
    if "type" not in component:
        error(f"Component {idx}: Missing 'type' field")

    comp_type = component["type"]
    if comp_type not in ["Service", "Worker", "Configuration", "Logging", "Repository", "API", "DataModel"]:
        error(f"Component {idx}: Invalid type '{comp_type}'")

    # Validate required fields by type
    required_fields = get_required_fields(comp_type)
    for field in required_fields:
        if field not in component:
            error(f"Component {idx} ({comp_type}): Missing '{field}'")
```

**Expected:** All components have type, name, file_path, and type-specific required fields

### Test Requirement Validation (v2.0 Stories)

**Validate every requirement has test_requirement:**

```python
for component in tech_spec["components"]:
    if "requirements" in component:
        for req_idx, req in enumerate(component["requirements"]):
            if "test_requirement" not in req:
                warning(f"{component['name']}: Requirement {req_idx} missing test_requirement")
            elif not req["test_requirement"].startswith("Test:"):
                warning(f"{component['name']}: test_requirement should start with 'Test:'")
```

**Expected:** All requirements have `test_requirement: "Test: [assertion]"`

### Automated Validation with validate_tech_spec.py (v2.0 Stories)

**Run validation script:**

```bash
python3 .claude/skills/devforgeai-story-creation/scripts/validate_tech_spec.py {story_file}
```

**Expected exit codes:**
- 0: PASS (v2.0 format valid)
- 1: FAIL (errors found)
- 2: Invalid arguments

**Integration:** Phase 7 self-validation should run this script automatically for v2.0 stories.

---

### API Contract Validation (If Applicable)

```
If story involves HTTP endpoints:

Validate API contracts present:
- [ ] At least 1 API endpoint documented
- [ ] HTTP method specified (GET/POST/PUT/PATCH/DELETE)
- [ ] Endpoint path follows RESTful conventions
- [ ] Request schema included (if POST/PUT/PATCH)
- [ ] Success response schema (200/201/204)
- [ ] Error response schemas (400, 401, 403, 404, 422, 500)
- [ ] Status codes documented
- [ ] Authentication requirements specified
- [ ] Validation rules defined

If API expected but not documented:
    CRITICAL: Missing API contracts

    # Re-invoke api-designer subagent
    # Generate missing contracts
    # Edit story file
    # Retry validation

Validate completeness:
For each endpoint:
    - [ ] Request schema has all required fields with types
    - [ ] Response schema shows all fields returned
    - [ ] Error responses cover all error scenarios from AC
    - [ ] Validation rules match acceptance criteria
```

### Data Model Validation

```
Validate data models:

- [ ] At least 1 entity defined (if story creates/modifies data)
- [ ] Each entity has:
  - [ ] Entity name
  - [ ] Purpose/description
  - [ ] Attributes table with columns: Field, Type, Constraints, Description
  - [ ] At least 3 attributes (id, created_at, plus domain fields)
  - [ ] Primary key identified
  - [ ] Relationships documented (if related entities exist)

If data models missing:
    HIGH: Missing data models

    # Self-healing:
    # Extract entities from user story (nouns)
    # Generate basic attribute list
    # Add to technical specification section
    # Retry validation

Validate attributes:
For each attribute:
    - [ ] Has data type (UUID, String, Integer, Boolean, DateTime, etc.)
    - [ ] Has constraints (Required/Optional, Unique, Length, Range, Format)
    - [ ] Has description

If attributes incomplete:
    MEDIUM: Incomplete attribute documentation

    # Add missing information
    # Common defaults:
    #   - id: UUID, Required, PK
    #   - created_at: DateTime, Required, Auto-generated
    #   - updated_at: DateTime, Required, Auto-updated
```

### Business Rules Validation

```
Validate business rules:

If story involves validation, calculation, or state transitions:
    - [ ] At least 1 business rule documented
    - [ ] Each rule is specific (not vague)
    - [ ] Rules explain validation logic
    - [ ] State transitions mapped (if applicable)

If missing expected business rules:
    MEDIUM: Missing business rules

    # Extract from acceptance criteria
    # Look for validation requirements, calculations, state changes
    # Document as business rules
    # Edit story file
```

### Dependencies Validation

```
Validate dependencies:

If story requires external services:
    - [ ] All dependencies listed
    - [ ] Each dependency has:
      - [ ] Service name
      - [ ] Purpose
      - [ ] Integration method
      - [ ] Authentication method
      - [ ] Fallback behavior

If dependencies mentioned in AC but not documented:
    HIGH: Missing dependency documentation

    # Extract dependencies from acceptance criteria
    # Document with integration details
    # Edit story file
```

---

## UI Specification Validation (If Applicable)

### Component Documentation Validation

```
If story has UI components:

Validate components documented:
- [ ] All components from acceptance criteria are documented
- [ ] Each component has:
  - [ ] Component name
  - [ ] Component type (Form, Table, Modal, etc.)
  - [ ] Purpose description
  - [ ] Data bindings (input/output/state)

If components missing:
    HIGH: Missing UI component documentation

    # Extract component mentions from AC
    # Generate component documentation
    # Edit story file
```

### ASCII Mockup Validation

```
Validate mockup present:
- [ ] ASCII mockup shows component layout
- [ ] Uses box-drawing characters (or ASCII-safe +-|)
- [ ] Shows interactive elements (buttons, inputs)
- [ ] Labels all major sections
- [ ] Dimensions reasonable (not too wide for markdown)

If mockup missing or incomplete:
    MEDIUM: Missing or incomplete UI mockup

    # Generate basic mockup from component list
    # Use ui-specification-guide.md templates
    # Edit story file
```

### Component Interface Validation

```
Validate component interfaces:
- [ ] Interfaces defined for main components
- [ ] Uses project's language (TypeScript, C#, Python)
- [ ] Props/properties documented
- [ ] Event handlers documented

If interfaces missing:
    LOW: Missing component interfaces

    # Generate basic interfaces
    # Use ui-specification-guide.md patterns
    # Edit story file
```

### Interaction Flow Validation

```
Validate interaction flows:
- [ ] User interactions documented step-by-step
- [ ] State changes noted for each step
- [ ] Both success and error paths covered
- [ ] Navigation flow clear

If incomplete:
    MEDIUM: Incomplete interaction flows

    # Generate flow from acceptance criteria
    # Map AC to interaction steps
    # Edit story file
```

### Accessibility Validation

```
Validate accessibility requirements:
- [ ] Keyboard navigation specified
- [ ] Screen reader labels documented (aria-label, aria-describedby)
- [ ] Focus management described
- [ ] Color contrast requirements specified

If accessibility missing:
    HIGH: Missing accessibility requirements

    # Generate standard WCAG AA requirements
    # Use ui-specification-guide.md templates
    # Edit story file

Required accessibility minimum:
- Keyboard: Tab navigation, Enter/Space actions
- Screen reader: aria-label on icon buttons, aria-describedby on form fields
- Focus: Visual indicators (3px outline)
- Contrast: 4.5:1 text, 3:1 UI components
```

---

## Non-Functional Requirements Validation

### Performance NFR Validation

```
Validate performance NFRs:
- [ ] Performance targets are quantified (not "fast")
- [ ] Response time targets specified (e.g., "<500ms")
- [ ] Throughput targets specified (e.g., "1000 req/sec")
- [ ] Page load time specified (if UI)
- [ ] Database query time specified (if applicable)

Examples of valid performance NFRs:
✅ "API response time <500ms for 95th percentile"
✅ "Page load time <2 seconds on 3G connection"
✅ "Database queries <100ms for indexed lookups"

Examples of invalid (vague):
❌ "System should be fast"
❌ "Good performance"
❌ "Responsive interface"

If vague performance NFRs:
    HIGH: Vague performance requirements

    # Use AskUserQuestion to quantify
    # Common defaults:
    #   - API response: <500ms
    #   - Page load: <2s
    #   - DB query: <100ms

    # Edit story file
    # Retry validation
```

### Security NFR Validation

```
Validate security NFRs:
- [ ] Security requirements are specific (not "secure")
- [ ] Authentication method specified (if applicable)
- [ ] Authorization model specified (if applicable)
- [ ] Encryption requirements specified (if sensitive data)
- [ ] Input validation rules documented

Examples of valid security NFRs:
✅ "Authentication: OAuth2 with JWT tokens (24h expiry)"
✅ "Authorization: RBAC with admin:write permission required"
✅ "Encryption: AES-256 for data at rest, TLS 1.3 for data in transit"
✅ "Input validation: Sanitize all user input, parameterized SQL queries"

Examples of invalid (vague):
❌ "System should be secure"
❌ "Good security practices"
❌ "Protected endpoints"

If vague security NFRs:
    HIGH: Vague security requirements

    # Ask user for specifics
    # Check if story handles sensitive data (PII, payment, health)
    # If yes: Require encryption, authentication, authorization
    # Edit story file
```

### Scalability NFR Validation

```
Validate scalability NFRs:
- [ ] Concurrent user targets specified
- [ ] Data volume targets specified
- [ ] Growth rate specified (if relevant)

Examples of valid scalability NFRs:
✅ "Support 1,000 concurrent users"
✅ "Handle 100,000 records in database"
✅ "Accommodate 20% monthly user growth"

Examples of invalid (vague):
❌ "Highly scalable"
❌ "Handles lots of users"

If vague:
    MEDIUM: Vague scalability requirements

    # Use defaults based on story complexity:
    #   - Simple (1-3 points): 100 concurrent users, 10k records
    #   - Standard (5-8 points): 1k concurrent users, 100k records
    #   - Complex (13+ points): 10k+ concurrent users, 1M+ records

    # Edit story file
```

### Usability NFR Validation

```
Validate usability NFRs:
- [ ] Usability requirements specific (not "user-friendly")
- [ ] Error messages guidance specified
- [ ] Help text requirements specified (if complex UI)

Examples of valid usability NFRs:
✅ "Error messages use plain language, no technical jargon"
✅ "All forms include help text for complex fields"
✅ "Maximum 3 clicks to complete checkout"
✅ "Confirmation required for destructive actions"

Examples of invalid (vague):
❌ "Easy to use"
❌ "Intuitive interface"

If vague:
    LOW: Vague usability requirements

    # Generate standard usability NFRs
    # Edit story file
```

---

## Completeness Validation

### All Required Sections Present

```
Validate section presence:

Required sections:
- [ ] User Story section exists
- [ ] Acceptance Criteria section exists
- [ ] Technical Specification section exists
- [ ] Non-Functional Requirements section exists
- [ ] Edge Cases & Error Handling section exists
- [ ] Definition of Done section exists
- [ ] Workflow History section exists

Optional sections (if applicable):
- [ ] UI Specification (if UI story)
- [ ] API Contracts (if API story)

If required section missing:
    CRITICAL: Missing required section

    # Self-healing: Add section with template content
    # Mark for review: "REVIEW NEEDED: Auto-generated section"
    # Edit story file
    # Retry validation
```

### No Placeholder Content

```
Scan entire story for placeholders:

Forbidden placeholders:
- "TBD" (To Be Determined)
- "TODO"
- "FIXME"
- "[placeholder]"
- "Example" or "Sample" (without actual content)
- Empty sections (## Section with no content after)

If placeholders found:
    HIGH: Placeholder content detected

    # Report specific placeholders to user
    # Cannot self-heal (need actual content)
    # User must provide missing information
```

---

## Quality Validation

### Acceptance Criteria Quality

```
For each acceptance criterion:

Validate quality:
- [ ] Describes single scenario (not multiple scenarios in one AC)
- [ ] Outcome is specific (not vague)
- [ ] Uses active voice
- [ ] Avoids implementation details (describes WHAT, not HOW)

Quality indicators:

✅ Good AC:
### AC1: User submits valid registration
**Given** user is on registration page
**When** user submits form with valid email, password (8+ chars), and name
**Then** account is created with status "unverified"
And verification email is sent to user's email address
And user is redirected to /verify-email-sent page
And success message displays "Check your email to verify your account"

❌ Bad AC (multiple scenarios):
**Given** user registers
**When** various things happen
**Then** system does stuff and maybe sends email or shows error

❌ Bad AC (implementation details):
**Given** user clicks submit
**When** form validation runs using Yup library
**Then** database INSERT query executes via Prisma ORM
→ Too much implementation detail, should focus on behavior

❌ Bad AC (vague outcome):
**Then** user is notified
→ How? Email? UI message? Push notification?

✅ Good (specific):
**Then** success message displays in green banner at top: "Account created successfully"
```

### Edge Cases Quality

```
Validate edge cases:
- [ ] Minimum 2 edge cases
- [ ] Each describes specific scenario (not generic)
- [ ] Expected behavior is defined
- [ ] Covers different categories:
  - [ ] Boundary conditions (min/max, empty/full)
  - [ ] Concurrent access (if multi-user)
  - [ ] Network/service failures (if external dependencies)
  - [ ] Data corruption/invalid state

Quality indicators:

✅ Good edge case:
1. **Case:** User closes browser during payment processing
   **Expected:** Payment completes in background (webhook confirms), user receives email with order status, can check order status on return

❌ Bad edge case (vague):
1. **Case:** Something goes wrong
   **Expected:** System handles it

If edge cases too generic:
    MEDIUM: Vague edge cases

    # Ask user for specific scenarios
    # Or generate common edge cases for story type
    # Edit story file
```

---

## Self-Healing Procedures

### Automatic Regeneration

**When to auto-regenerate:**
- Missing sections (generate from template)
- Invalid frontmatter fields (correct to valid values)
- Malformed Given/When/Then (reformat)
- Missing basic business rules (extract from AC)

**When NOT to auto-regenerate:**
- Vague acceptance criteria (need user clarification)
- Missing API contracts (need api-designer subagent)
- Placeholder content (need actual content from user)

### Retry Logic

```
Max attempts: 2

attempt = 0
while attempt < 2:
    validation_result = run_full_validation()

    if validation_result.critical_count == 0 and validation_result.high_count == 0:
        ✓ Validation passed (only medium/low issues acceptable)
        break
    else:
        attempt += 1
        if attempt < 2:
            # Self-heal critical and high issues
            for issue in validation_result.critical:
                auto_fix(issue)
            for issue in validation_result.high:
                try_auto_fix(issue)  # May fail, that's ok

            # Retry validation
        else:
            # Report to user after max attempts
            report_validation_failures(validation_result)
```

### User Reporting

```
If validation fails after self-healing attempts:

Report: """
⚠️ Story Validation Issues

The following issues need review:

CRITICAL Issues ({critical_count}):
{list critical issues with file locations}

HIGH Issues ({high_count}):
{list high issues with file locations}

MEDIUM Issues ({medium_count}):
{list medium issues - can proceed but should fix}

Recommendations:
1. Review story file: {story_path}
2. Fix critical and high issues
3. Re-run story creation or proceed to /dev (will validate again)

Or ask me to fix specific issues.
"""

HALT: Do not proceed to Phase 8 if CRITICAL or HIGH issues remain
```

---

## Cross-Reference Validation

### Epic Reference Validation

```
If epic field is not null:

Validate epic exists:
- [ ] Epic file exists at devforgeai/specs/Epics/{epic_id}.epic.md
- [ ] Epic is readable
- [ ] Epic has stories section

If epic file missing:
    CRITICAL: Referenced epic does not exist

    # Options:
    # 1. Set epic to null (unlink)
    # 2. Ask user which epic this belongs to
    # 3. Create epic placeholder (not recommended)

    # Edit story file
    # Retry validation
```

### Sprint Reference Validation

```
If sprint field is not "Backlog":

Validate sprint exists:
- [ ] Sprint file exists at devforgeai/specs/Sprints/{sprint_id}.md
- [ ] Sprint is readable
- [ ] Sprint has sprint backlog section

If sprint file missing:
    MEDIUM: Referenced sprint does not exist

    # Set sprint to "Backlog" (safe default)
    # Edit story file
    # Retry validation
```

---

## Story Size Validation

### Story Point Threshold

```
Validate story size:

If points >= 13:
    WARNING: Story might be too large

    Check acceptance criteria count:
    - If AC count > 8: Story is probably too complex
    - If AC count > 12: Story DEFINITELY too large

    Recommend splitting:
    - Suggest: "Consider splitting into 2-3 smaller stories (3-5 points each)"
    - Explain: "Large stories (13+ points) are harder to estimate, test, and review"

    Options:
    - [ ] Proceed anyway (user acknowledges risk)
    - [ ] Split story (generate split recommendations)

If user chooses to proceed:
    # Document in workflow history
    # Add note: "Large story (13 points) - consider splitting during sprint planning"

If user chooses to split:
    # Halt story creation
    # Guide user to create 2-3 smaller stories instead
```

### Acceptance Criteria Count

```
Validate AC count:

Healthy range: 3-8 acceptance criteria

If < 3:
    HIGH: Too few acceptance criteria
    # Story under-specified
    # Generate additional criteria from user story

If > 12:
    WARNING: Too many acceptance criteria
    # Story likely too large
    # Recommend splitting into multiple stories

If 8-12:
    MEDIUM: High AC count
    # Story is complex but manageable
    # Document complexity in workflow history
```

---

## Context File Compliance Validation

**Reference:** `.claude/skills/devforgeai-story-creation/references/context-validation.md`

### Greenfield Detection

```
Check for context files:

context_dir = "devforgeai/specs/context/"
context_files = Glob(pattern=f"{context_dir}*.md")

IF len(context_files) == 0:
    # Greenfield mode
    Display: "ℹ️ Greenfield mode: context compliance validation skipped"
    SKIP context validation checklists
    RETURN { greenfield: true }
```

### Technology Compliance (tech-stack.md)

```
Validate against tech-stack.md:

- [ ] All technologies in tech spec are in LOCKED list
- [ ] No PROHIBITED technologies referenced
- [ ] Framework versions match approved versions (if specified)

If violation found:
    HIGH: Unapproved technology '{technology}'

    AskUserQuestion:
      Question: "'{technology}' not in tech-stack.md. How to proceed?"
      Header: "Tech violation"
      Options:
        - "Remove from spec" - Use approved alternative
        - "Add to tech-stack.md" - Requires ADR
        - "Flag for review" - Proceed with warning
```

### File Path Compliance (source-tree.md)

```
Validate against source-tree.md:

- [ ] All file_path values in tech spec match source-tree.md structure
- [ ] Story output directory is devforgeai/specs/Stories/
- [ ] No files in FORBIDDEN directories

Critical check (BLOCKING):
- [ ] Story file written to correct directory

If violation found:
    CRITICAL (path): Invalid file path '{path}'
    HIGH (directory): Story in wrong directory

    Auto-fix: Redirect to devforgeai/specs/Stories/
```

### Dependency Compliance (dependencies.md)

```
Validate against dependencies.md:

- [ ] All packages in Dependencies section are LOCKED
- [ ] No FORBIDDEN alternatives used
- [ ] Package versions compatible (if specified)

If violation found:
    HIGH: Unapproved dependency '{package}'

    AskUserQuestion:
      Question: "'{package}' not in dependencies.md. How to proceed?"
      Header: "Dep violation"
      Options:
        - "Remove dependency" - Find alternative
        - "Add to dependencies.md" - Requires ADR
        - "Flag for review" - Proceed with warning
```

### Coverage Threshold Compliance (coding-standards.md)

```
Validate against coding-standards.md:

- [ ] Coverage thresholds in DoD match layer requirements:
  - Business logic layer: 95%
  - Application layer: 85%
  - Infrastructure layer: 80%

- [ ] Story correctly identifies its architectural layer

If violation found:
    MEDIUM: Incorrect coverage threshold

    Auto-fix: Update DoD to correct threshold for layer
```

### Architecture Compliance (architecture-constraints.md)

```
Validate against architecture-constraints.md:

- [ ] No direct controller→repository dependencies
- [ ] Layer boundaries respected in tech spec design
- [ ] No circular dependencies proposed

If violation found:
    HIGH: Layer boundary violation '{from}' → '{to}'

    AskUserQuestion:
      Question: "Design violates architecture constraints. Redesign?"
      Header: "Arch violation"
      Options:
        - "Redesign" - I'll provide compliant design
        - "Flag for review" - Proceed with warning
```

### Anti-Pattern Compliance (anti-patterns.md)

```
Validate against anti-patterns.md:

- [ ] No God Objects proposed (classes >500 lines or >20 methods)
- [ ] No SQL string concatenation in data access patterns
- [ ] No hardcoded secrets in config examples
- [ ] No Bash commands for file operations (use native tools)
- [ ] No monolithic skill designs

If violation found:
    CRITICAL: Anti-pattern detected '{pattern}'

    HALT: "Anti-pattern '{pattern}' detected. Technical spec must be revised."
    Cannot proceed until anti-pattern removed from spec
```

### Context Validation Summary

```
Generate summary after all context checks:

Context Compliance Report:
========================

Files Checked: {count}/6
- [x] tech-stack.md      (or [-] not found)
- [x] source-tree.md     (or [-] not found)
- [x] dependencies.md    (or [-] not found)
- [x] coding-standards.md (or [-] not found)
- [x] architecture-constraints.md (or [-] not found)
- [x] anti-patterns.md   (or [-] not found)

Violations:
- CRITICAL: {count}  ← BLOCKING
- HIGH: {count}      ← BLOCKING
- MEDIUM: {count}    ← Warning only
- LOW: {count}       ← Warning only

Status: {COMPLIANT | FAILED | WARNINGS}

IF CRITICAL or HIGH > 0:
    HALT: Resolve violations before completing story
ELIF MEDIUM or LOW > 0:
    WARN: Proceeding with warnings
ELSE:
    ✓ Fully compliant with all context files
```

---

## Handoff Readiness Validation

### Ready for Development

```
Validate story is ready for /dev:

Prerequisites:
- [ ] All CRITICAL and HIGH validation issues resolved
- [ ] No placeholder content (TBD, TODO)
- [ ] All acceptance criteria testable
- [ ] Technical specification complete (if technical story)
- [ ] API contracts complete (if API story)
- [ ] UI specification complete (if UI story)
- [ ] Dependencies documented (if external services)
- [ ] NFRs are measurable
- [ ] Context file compliance passed (or greenfield mode)

If prerequisites met:
    ✓ Story is ready for development (/dev command)

If prerequisites not met:
    ⚠️ Story needs review before development
    # List specific gaps
    # User must address before /dev
```

### Consistency with DevForgeAI Standards

```
Validate framework compliance:

If context files exist:
    Read: devforgeai/specs/context/tech-stack.md
    Read: devforgeai/specs/context/coding-standards.md

    Validate:
    - [ ] Technologies mentioned align with tech-stack.md
    - [ ] Code examples follow coding-standards.md
    - [ ] No forbidden anti-patterns mentioned

    If conflicts detected:
        HIGH: Story conflicts with context files

        # Use AskUserQuestion to resolve
        # Update story or context file (with ADR)

If context files don't exist:
    # Skip context validation
    # Note: Will be validated during architecture phase
```

---

## Validation Summary Template

**Use this template when reporting validation results:**

```markdown
## Validation Results

**Story:** {story_id} - {title}

### Issues by Severity

**CRITICAL ({count}):**
{list critical issues}

**HIGH ({count}):**
{list high issues}

**MEDIUM ({count}):**
{list medium issues}

**LOW ({count}):**
{list low issues}

### Self-Healing Attempts

{list auto-fixes applied}

### Status

{if all critical/high resolved:}
✅ Validation passed - Story ready for development

{if critical/high remain:}
⚠️ Validation incomplete - Review required before development

### Recommendations

{specific actions user should take}
```

---

## Progressive Disclosure

**When to load this reference:**
- Phase 7: Self-Validation (complete validation logic)

**Why progressive:**
- Not needed during Phases 1-6 (story creation workflow)
- Loaded only when validating completed story
- Saves ~400 lines from loading until needed

**Validation flow:**
```
Phase 7 Start
   ↓
Load validation-checklists.md (this file)
   ↓
Execute checklists sequentially:
   1. YAML Frontmatter Validation
   2. User Story Format Validation
   3. Acceptance Criteria Validation
   4. Technical Specification Validation
   5. UI Specification Validation (if applicable)
   6. NFR Validation
   7. Completeness Validation
   8. Cross-Reference Validation
   9. Story Size Validation
   10. Handoff Readiness Validation
   ↓
Self-healing attempts (max 2 iterations)
   ↓
Report results (critical/high/medium/low)
   ↓
If passed: Proceed to Phase 8
If failed: Report to user, halt
```

---

**Use these checklists to ensure every story meets quality standards before development begins, preventing technical debt from incomplete or ambiguous specifications.**
