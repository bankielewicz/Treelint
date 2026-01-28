# Phase 3: Technical Specification Creation

Generate technical specifications using **structured YAML format (v2.0)** for machine-readable parsing and automated validation.

## Overview

This phase creates the technical foundation for implementation using structured YAML that enables:
- **Deterministic parsing:** 95%+ accuracy (vs 85% with freeform text)
- **Automated validation:** Component coverage validation in Phase 3
- **Comprehensive test generation:** Every component has explicit test requirements
- **Zero ambiguity:** Machine-readable schema eliminates interpretation errors

**Format Version:** 2.0 (Structured YAML) - **Default for all new stories**

**See:** `devforgeai/specs/STRUCTURED-FORMAT-SPECIFICATION.md` for complete schema reference

**RCA-006 Phase 2: Structured Technical Specifications**

Starting with v2.0, technical specifications use structured YAML format instead of freeform text. This enables:
- **Phase 1 Step 4:** 95%+ accuracy in coverage gap detection (vs 85% with freeform)
- **Phase 3 (Future):** Automated implementation validation
- **Test Generation:** Direct mapping from component requirements to tests

**v2.0 Format Overview:**
```yaml
technical_specification:
  format_version: "2.0"

  components:  # 7 component types: Service, Worker, Configuration, Logging, Repository, API, DataModel
    - type: "[ComponentType]"
      name: "[ComponentName]"
      file_path: "src/[path]/[file]"
      requirements:
        - id: "[COMP-001]"
          description: "[What must be implemented]"
          testable: true
          test_requirement: "Test: [Specific test assertion]"
          priority: "Critical|High|Medium|Low"

  business_rules:  # Domain logic and constraints
    - id: "BR-001"
      rule: "[Business rule]"
      test_requirement: "Test: [How to validate]"

  non_functional_requirements:  # Performance, security, scalability
    - id: "NFR-001"
      category: "Performance|Security|Scalability|Reliability"
      requirement: "[NFR description]"
      metric: "[Measurable target with numbers]"
      test_requirement: "Test: [How to verify]"
```

**Component Type Selection:**
- **Service:** Hosted services, application services with lifecycle
- **Worker:** Background tasks, polling loops, scheduled jobs
- **Configuration:** appsettings.json, environment variables, config files
- **Logging:** Log sinks, logging configuration
- **Repository:** Data access layer, database interactions
- **API:** HTTP endpoints (REST, GraphQL, gRPC)
- **DataModel:** Database entities, DTOs, domain objects

**RCA-007 Enhancements (Phase 2):**
- Step 3.0: Pre-invocation file system snapshot (NEW - for api-designer)
- Step 3.2: Enhanced api-designer prompt with 4-section template (Phase 1)
- Step 3.2.5: Contract-based validation (NEW - Phase 2)
- Step 3.2.7: Post-invocation file system diff (NEW - Phase 2)

---

## Evidence-Verification Pre-Flight (NEW - RCA-020)

**Purpose:** Enforce the Read-Quote-Cite-Verify protocol before generating technical specifications.

This section ensures all technical specification claims are verified against actual target files BEFORE story creation. This prevents stories with unverified claims (e.g., "Remove Bash mkdir commands" when no such commands exist in target files).

**Source:** RCA-020 (Story Creation Missing Evidence-Based Verification)
**Protocol Reference:** `.claude/rules/core/citation-requirements.md` (Read-Quote-Cite-Verify protocol)

**When to Execute:** Before Step 3.0 (Pre-Invocation File System Snapshot), immediately after feature description is provided.

---

### Step EV-1: Target File Identification

**Objective:** Identify all target files mentioned in the feature description for verification.

**Extract target_files from feature description and technical scope:**

```python
# Extraction logic
target_files = []

# Parse feature description for file references
# Look for:
# - Files claimed to have violations (e.g., "SKILL.md has Bash cat commands")
# - Files needing modifications (e.g., "Update technical-specification-creation.md")
# - Configuration files requiring updates (e.g., "Modify devforgeai/config/settings.yaml")

patterns_to_extract = [
    r'[\w\-/]+\.md',           # Markdown files
    r'[\w\-/]+\.yaml',         # YAML files
    r'[\w\-/]+\.py',           # Python files
    r'[\w\-/]+\.ts',           # TypeScript files
    r'\.claude/[\w\-/]+',      # .claude directory paths
    r'devforgeai/[\w\-/]+',    # devforgeai directory paths
]

for pattern in patterns_to_extract:
    matches = Grep(pattern=pattern, path=feature_description_text)
    target_files.extend(matches)

# Deduplicate
target_files = list(set(target_files))

Display: f"""
Step EV-1: Target File Identification Complete

Extracted {len(target_files)} target files from feature description:
{chr(10).join(f"  - {f}" for f in target_files)}

Proceeding to Step EV-2 (Read and Verify)...
"""
```

---

### Step EV-2: Read and Verify Each Target File

**Objective:** Verify all claims about target files using native Read() and Grep tools.

**Verification workflow:**

```python
verification_results = []

FOR each file in target_files:

    # Step 2a: Check file existence
    IF file does NOT exist:
        # HALT if file not found - cannot verify claims
        HALT: f"""❌ CRITICAL: Target file not found: {file}

Cannot verify claims about this file.
Aborting story creation.

Recovery options:
1. Check file path is correct
2. Remove unverifiable claims from feature description
3. Create the file first if it should exist
"""

    ELSE:
        # Step 2b: Read file content
        file_content = Read(file_path=file)

        # Step 2c: Verify each claim about this file
        FOR each claim about this file:

            # Use Grep to search for claimed content
            search_result = Grep(
                pattern=claim.pattern,
                path=file,
                output_mode="content"
            )

            # Record verification result
            verification_results.append({
                "file": file,
                "claim": claim.description,
                "verified": len(search_result.matches) > 0,
                "lines": [match.line_number for match in search_result.matches],
                "count": len(search_result.matches)
            })

            Display: f"""
Verifying claim: "{claim.description}"
  File: {file}
  Pattern: {claim.pattern}
  Found: {len(search_result.matches)} matches
  Lines: {[match.line_number for match in search_result.matches]}
  Status: {"✓ VERIFIED" if len(search_result.matches) > 0 else "✗ NOT FOUND"}
"""
```

---

### Step EV-3: Evidence Sufficiency Validation

**Objective:** Validate that EVERY claim has supporting evidence before proceeding.

**Validation logic:**

```python
# Check: For EVERY claim, is there supporting evidence?
unverified_claims = [r for r in verification_results if not r["verified"]]

IF len(unverified_claims) > 0:
    HALT: f"""
❌ CRITICAL: Cannot verify claim(s)

The following claims could not be verified against target files:

{chr(10).join(f'''
Claim: "{c['claim']}"
  File checked: {c['file']}
  Evidence found: None
''' for c in unverified_claims)}

Recovery options:
1. If claim is speculative → Remove from story
2. If claim is valid → Check target file path is correct
3. If pattern is wrong → Adjust search pattern

Story creation cannot proceed with unverified claims.
All technical specifications must have evidence.
"""

ELSE:
    # All claims verified
    Display: f"""
✓ Step EV-3: Evidence Sufficiency Validation PASSED

All {len(verification_results)} claims verified:
{chr(10).join(f"  ✓ {r['claim']} ({r['count']} matches in {r['file']})" for r in verification_results)}

Proceeding to Step EV-4 (Generate verified_violations YAML)...
"""
```

---

### Step EV-4: Generate verified_violations YAML Section

**Objective:** Generate a structured YAML section documenting all verified violations with specific file paths, line numbers, and counts.

**YAML Template:**

```yaml
verified_violations:
  description: "Claims verified during story creation ({YYYY-MM-DD})"
  protocol: "Read-Quote-Cite-Verify"
  source_rca: "RCA-020"
  locations:
    - file: "{target_file_1}"
      lines: [12, 45, 89]
      count: 3
      claim: "Bash mkdir commands found"
    - file: "{target_file_2}"
      lines: []
      count: 0
      note: "No violations found - file compliant"
    - file: "{target_file_3}"
      lines: [156, 203]
      count: 2
      claim: "Hardcoded paths detected"
```

**Generation logic:**

```python
from datetime import date

verified_violations_yaml = {
    "verified_violations": {
        "description": f"Claims verified during story creation ({date.today().isoformat()})",
        "protocol": "Read-Quote-Cite-Verify",
        "source_rca": "RCA-020",
        "locations": []
    }
}

FOR result in verification_results:
    location_entry = {
        "file": result["file"],
        "lines": result["lines"],  # Specific line numbers, e.g., [12, 45, 89]
        "count": result["count"]
    }

    IF result["count"] == 0:
        location_entry["note"] = "No violations found - file compliant"
    ELSE:
        location_entry["claim"] = result["claim"]

    verified_violations_yaml["verified_violations"]["locations"].append(location_entry)

Display: f"""
Step EV-4: verified_violations YAML Generated

```yaml
{yaml.dump(verified_violations_yaml, default_flow_style=False)}
```

This section will be embedded in the story's Technical Specification.
Evidence verification complete. Proceeding to Step 3.0...
"""
```

---

### Evidence-Verification Pre-Flight Summary

| Step | Action | Output |
|------|--------|--------|
| EV-1 | Target File Identification | `target_files` array |
| EV-2 | Read and Verify Each File | `verification_results` with lines/counts |
| EV-3 | Evidence Sufficiency Check | HALT if unverified, CONTINUE if all verified |
| EV-4 | Generate verified_violations YAML | Structured YAML for story embedding |

**On Completion:** Proceed to Step 3.0 (Pre-Invocation File System Snapshot) with verified claims.

---

## Step 3.0: Pre-Invocation File System Snapshot (NEW - RCA-007 Phase 2)

**Objective:** Capture file system state before api-designer execution

**NOTE:** This step only runs if API requirements detected (Step 3.1) and api-designer will be invoked (Step 3.2).

**Take snapshot (same as Step 2.0):**
```python
# Capture current .story.md and API spec files
files_before_api_designer = Glob(pattern="devforgeai/specs/Stories/STORY-*.story.md")

# Capture potential API spec files
api_spec_patterns = [
    f"devforgeai/specs/api/{story_id}-api-spec.yaml",
    f"devforgeai/specs/Stories/{story_id}-api-spec.yaml",
    f"devforgeai/specs/Stories/*-api-spec.yaml",
    f"devforgeai/specs/api/*.yaml"
]

api_files_before = []
for pattern in api_spec_patterns:
    matching_files = Glob(pattern=pattern)
    api_files_before.extend(matching_files)

# Store snapshot
api_snapshot = {
    "timestamp": datetime.now().isoformat(),
    "story_id": story_id,
    "api_files_count": len(api_files_before),
    "api_files": api_files_before
}

Display: f"""
Step 3.0: File System Snapshot (Before api-designer)
- Existing API spec files: {len(api_files_before)}

Snapshot captured. Proceeding to Step 3.2 (api-designer invocation)...
"""
```

---

## Step 3.1: Detect API Requirements

**Objective:** Determine if story requires API endpoints

**Keyword analysis on feature description and acceptance criteria:**

```
api_keywords = ["API", "endpoint", "REST", "GraphQL", "request", "response",
                "POST", "GET", "PUT", "DELETE", "fetch", "call"]

requires_api = any(keyword in feature_description.upper() or
                   keyword in ac_text.upper()
                   for keyword in api_keywords)
```

**If API detected or uncertain:**
```
AskUserQuestion(
  questions=[{
    question: "Does this story require API endpoints?",
    header: "API needed",
    options: [
      {
        label: "Yes - backend API",
        description: "HTTP endpoints for data operations"
      },
      {
        label: "No - frontend only",
        description: "No backend API needed"
      },
      {
        label: "No - backend logic without HTTP API",
        description: "Business logic but no HTTP endpoints"
      }
    ],
    multiSelect: false
  }]
)
```

---

## Step 3.2: Generate API Contracts (If Needed) (ENHANCED - RCA-007 Fix)

**Objective:** Design API contracts using api-designer subagent

**IMPORTANT:** This prompt has been enhanced to prevent RCA-007 violations. The api-designer MUST return YAML/markdown content only (no separate api-spec.yaml file).

**Invoke API designer subagent:**

```
if requires_api:
    Task(
      subagent_type="api-designer",
      description="Design API contracts (content only)",
      prompt="""Design API contracts for user story following DevForgeAI standards.

**═══════════════════════════════════════════════════════════════════════════**
**PRE-FLIGHT BRIEFING:**
**═══════════════════════════════════════════════════════════════════════════**

You are being invoked by the devforgeai-story-creation skill.
This skill will embed your API specification into the Technical Specification section of .story.md.

**YOUR ROLE:**
- Generate OpenAPI 3.0 specification
- Return specification as YAML text
- Do NOT create files
- Parent skill embeds this in story document (Phase 5: Story File Creation)

**OUTPUT WILL BE USED IN:**
- Phase 5: Story File Creation (embedded in Technical Specification section)
- Your output is CONTENT for embedding, not a standalone API spec file

**WORKFLOW CONTEXT:**
- Current workflow: Story creation (8-phase process)
- Current phase: Phase 3 (Technical Specification)
- Next phase: Phase 4 (UI Specification)
- Final artifact: devforgeai/specs/Stories/{story_id}-{slug}.story.md

**═══════════════════════════════════════════════════════════════════════════**
**CRITICAL OUTPUT CONSTRAINTS:**
**═══════════════════════════════════════════════════════════════════════════**

1. **Format:** Return ONLY OpenAPI 3.0 YAML text (no file creation)
2. **Content:** Output will be embedded in Technical Specification section of .story.md
3. **Files:** Do NOT create separate files (api-spec.yaml, endpoints.md, schemas.md, models.md)
4. **Structure:** Single OpenAPI YAML document with paths, components, security, info
5. **Assembly:** Parent skill will wrap this in ```yaml code fence and insert into story
6. **Size:** Maximum 30,000 characters (fits in story technical spec section)

**Contract Reference:** .claude/skills/devforgeai-story-creation/contracts/api-designer-contract.yaml

**═══════════════════════════════════════════════════════════════════════════**
**PROHIBITED ACTIONS:**
**═══════════════════════════════════════════════════════════════════════════**

You MUST NOT:
1. ❌ Create api-spec.yaml file
2. ❌ Create separate schema files (user-schema.yaml, request-schemas.yaml)
3. ❌ Create endpoint documentation files (endpoints.md, api-docs.md)
4. ❌ Return file paths (e.g., "Created: api-spec.yaml")
5. ❌ Use Write or Edit tools
6. ❌ Use Bash for file creation
7. ❌ Generate multi-file API documentation
8. ❌ Create Postman collections or other artifacts

**Why prohibited:**
- Parent skill handles all file creation (Phase 5)
- Your output is embedded in story document (not standalone file)
- Multi-file output violates DevForgeAI single-file design
- Creates framework specification violations (RCA-007)

**What to do instead:**
- ✅ Return OpenAPI YAML as text string
- ✅ Include all schemas inline (components.schemas section)
- ✅ Document all endpoints in single YAML
- ✅ Let parent skill embed this in story document

**═══════════════════════════════════════════════════════════════════════════**
**EXPECTED OUTPUT FORMAT:**
**═══════════════════════════════════════════════════════════════════════════**

Your output should look like this (YAML TEXT, not files):

```yaml
openapi: 3.0.0
info:
  title: {Feature Name} API
  version: 1.0.0

paths:
  /api/endpoint:
    post:
      summary: {Description}
      requestBody:
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/RequestModel'
      responses:
        200:
          description: Success
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ResponseModel'

components:
  schemas:
    RequestModel:
      type: object
      properties:
        field1: {type: string}
        field2: {type: integer}

    ResponseModel:
      type: object
      properties:
        result: {type: string}
```

**What your output will become:**
Parent skill will embed your YAML in Technical Specification section:

```markdown
## Technical Specification

### API Contract

```yaml
{YOUR OUTPUT: OpenAPI YAML specification}
```

### Data Models
(Extracted from your components.schemas)
```

**Final result:** Single .story.md file with embedded API spec (not separate api-spec.yaml file)

**═══════════════════════════════════════════════════════════════════════════**
**NOW PROCEED WITH API DESIGN:**
**═══════════════════════════════════════════════════════════════════════════**

**User Story:** {user_story}

**Acceptance Criteria:**
{acceptance_criteria_list}

**Generate API contracts in OpenAPI 3.0 YAML format (as TEXT, not file):**

1. **HTTP Method and Endpoint Path**
   - RESTful conventions (nouns for resources, verbs for actions)
   - Example: POST /api/users, GET /api/users/{id}

2. **Request Schema**
   - Headers (Content-Type, Authorization)
   - Query parameters (with types and constraints)
   - Request body (JSON schema with validation rules)

3. **Response Schemas**
   - Success responses (200, 201, 204)
   - Error responses (400, 401, 403, 404, 422, 500)
   - Include all fields with types

4. **Authentication Requirements**
   - Auth method (Bearer token, API key, OAuth2)
   - Required scopes/permissions

5. **Validation Rules**
   - Input validation (required fields, formats, ranges)
   - Business rule validation

**Context:** DevForgeAI framework, tech-stack.md compliance required

**REMINDER:** Return OpenAPI YAML TEXT only. No file creation (no api-spec.yaml file).
"""
    )
```

**Validate API designer output:**
```
API contract must include:
- [ ] HTTP method (GET, POST, PUT, DELETE, PATCH)
- [ ] Endpoint path (RESTful naming)
- [ ] Request schema (if POST/PUT/PATCH)
- [ ] Success response schema (200, 201, etc.)
- [ ] Error response schemas (400, 401, 403, 404, 500)
- [ ] Status codes documented
- [ ] Authentication requirements specified
- [ ] Validation rules defined
```

**Load technical specification guide for templates:**
```
Read(file_path=".claude/skills/devforgeai-story-creation/references/technical-specification-guide.md")
```

---

## Step 3.2.5: Contract-Based Validation for API Designer (NEW - RCA-007 Phase 2)

**Objective:** Enforce api-designer contract specifications

**NOTE:** This step only runs if api-designer was invoked (API requirements detected).

**Load contract:**
```python
contract_path = ".claude/skills/devforgeai-story-creation/contracts/api-designer-contract.yaml"

if file_exists(contract_path):
    import yaml
    contract_content = Read(file_path=contract_path)
    contract = yaml.safe_load(contract_content)

    Display: f"""
Validating API designer output against contract:
- Contract: {contract['skill']} <-> {contract['subagent']}
- Version: {contract['contract_version']}
- Phase: {contract['phase']}
"""

    # Validate API designer output
    violations = []

    # Check 1: No file creation (api-spec.yaml, etc.)
    prohibited_patterns = contract['validation']['check_no_file_references']['prohibited_patterns']

    for pattern in prohibited_patterns:
        import re
        if re.search(pattern, api_designer_output, re.IGNORECASE):
            violations.append({
                "type": "FILE_CREATION",
                "pattern": pattern,
                "severity": "CRITICAL"
            })

    # Check 2: Valid OpenAPI YAML
    if contract['validation']['check_yaml_validity']['enabled']:
        try:
            # Attempt to parse YAML
            yaml.safe_load(api_designer_output)
        except yaml.YAMLError as e:
            violations.append({
                "type": "INVALID_YAML",
                "error": str(e),
                "severity": "CRITICAL"
            })

    # Check 3: OpenAPI version
    if contract['validation']['check_openapi_version']['enabled']:
        if "openapi: 3.0" not in api_designer_output:
            violations.append({
                "type": "INVALID_OPENAPI_VERSION",
                "severity": "HIGH"
            })

    # Display results
    if violations:
        Display: f"""
⚠️ API Designer Contract Validation FAILED

Violations: {len(violations)}
{format_violations(violations)}

Recovery: Apply error handling from contract
"""
    else:
        Display: f"""
✓ API Designer Contract Validation PASSED

OpenAPI YAML valid ✅
No file references ✅
Version 3.0.0 ✅

Proceeding to Step 3.2.7 (file system diff)...
"""
```

---

## Step 3.2.7: Post-Invocation File System Diff for API Designer (NEW - RCA-007 Phase 2)

**Objective:** Detect unauthorized API spec files created during api-designer execution

**NOTE:** This step only runs if api-designer was invoked.

**Compare file system:**
```python
# Capture current state (after api-designer)
api_files_after = []
for pattern in api_spec_patterns:
    matching_files = Glob(pattern=pattern)
    api_files_after.extend(matching_files)

# Calculate diff
new_api_files = list(set(api_files_after) - set(api_files_before))

# Check for unauthorized files
if len(new_api_files) > 0:
    # CRITICAL: API spec files created
    Display: f"""
❌ CRITICAL: Unauthorized API Spec Files Created

The api-designer subagent created separate API spec files.
This violates single-file design (API should be embedded in .story.md).

Unauthorized files:
{chr(10).join(f"  - {f}" for f in new_api_files)}

Recovery: Delete files and log violation
"""

    # Delete unauthorized files
    for file in new_api_files:
        Bash(command=f"rm '{file}'")
        Display: f"  Deleted: {file}"

    # Log violation
    import datetime
    log_entry = f"""
[FILE CREATION VIOLATION - API Designer]
Timestamp: {datetime.datetime.now().isoformat()}
Story ID: {story_id}
Phase: Phase 3 (Technical Specification)
Subagent: api-designer
Unauthorized files: {len(new_api_files)}
{chr(10).join(f"  - {f}" for f in new_api_files)}
Action: Files deleted
---
"""

    current_log = Read(file_path="devforgeai/logs/rca-007-violations.log")
    Write(file_path="devforgeai/logs/rca-007-violations.log", content=current_log + log_entry)

    # HALT (critical violation)
    HALT: """
❌ CRITICAL: api-designer created separate API spec files

Files have been deleted (rollback).
API specification should be embedded in .story.md (not separate file).

Manual intervention required.
"""

else:
    # No unauthorized files
    Display: f"""
✓ Step 3.2.7: File System Diff PASSED (api-designer)

API files before: {len(api_files_before)}
API files after: {len(api_files_after)}
New files: 0

api-designer returned YAML text (no file creation) ✅

Proceeding to Step 3.3 (Define Data Models)...
"""
```

---

## Step 3.3: Define Data Models

**Objective:** Extract entities and define data structures

**Extract entities from user story and acceptance criteria:**

```
# Look for nouns in user story
# Example: "As a customer, I want to view my orders"
# Entities: Customer, Order

entities = extract_entities(user_story, acceptance_criteria)

For each entity:
    Define:
    - Entity name
    - Purpose (brief description)
    - Attributes (fields):
      - Field name
      - Data type (String, Integer, UUID, DateTime, Boolean, etc.)
      - Constraints (Required, Unique, Min/Max length, Format)
      - Default value (if applicable)
    - Relationships:
      - One-to-many (e.g., User has many Orders)
      - Many-to-many (e.g., User belongs to many Roles)
      - One-to-one (e.g., User has one Profile)
    - Primary key
    - Foreign keys
    - Indexes (for performance)
```

**Example data model:**
```
Entity: User
Purpose: Represents a registered user of the system

Attributes:
  - id: UUID, Required, Primary Key, Auto-generated
  - email: String(255), Required, Unique, Email format validation
  - password_hash: String(255), Required, Bcrypt hashed
  - name: String(100), Required
  - created_at: DateTime, Required, Auto-generated
  - updated_at: DateTime, Required, Auto-updated

Relationships:
  - Has many: Orders (one-to-many)
  - Belongs to many: Roles (many-to-many via UserRoles table)

Indexes:
  - email (unique index for fast lookup)
```

---

## Step 3.4: Document Business Rules

**Objective:** Extract and document business logic rules

**Extract rules from acceptance criteria:**

```
Business rules define:
- Validation logic (e.g., "Password must be 8+ chars with uppercase, lowercase, number, special char")
- Calculation formulas (e.g., "Total price = sum(item.price * item.quantity) + shipping_cost")
- State transition rules (e.g., "Order can only be cancelled if status is 'Pending' or 'Processing'")
- Constraints (e.g., "Maximum 10 items per order")
- Defaults (e.g., "New user role defaults to 'Customer'")
```

**Example:**
```
Business Rules:
1. Password must be at least 8 characters with uppercase, lowercase, number, and special character
2. Email verification token expires after 24 hours
3. Verification email sent asynchronously (background job queue)
4. Unverified users cannot log in (authentication check blocks unverified accounts)
5. Email addresses are case-insensitive (stored as lowercase)
```

---

## Step 3.5: Identify Dependencies

**Objective:** Document external services and infrastructure needs

**Extract from acceptance criteria and technical requirements:**

```
Dependencies include:
- External services (payment gateway, email service, SMS provider)
- Third-party APIs (maps, weather, social media)
- Database requirements (PostgreSQL, Redis, MongoDB)
- Infrastructure (S3 for file storage, CDN for assets)
- Background job queue (Celery, Bull, Hangfire)
```

**For each dependency:**
```
Document:
- Service name
- Purpose (why needed)
- Integration method (REST API, SDK, library)
- Authentication (API key, OAuth)
- SLA requirements (uptime, response time)
- Fallback behavior (what if service unavailable)
```

**Example:**
```
Dependencies:
1. Email Service: SendGrid or SMTP
   - Purpose: Send verification emails
   - Integration: SendGrid SDK or nodemailer
   - Authentication: API Key
   - SLA: 99.9% uptime, <30 second delivery
   - Fallback: Queue for retry (max 3 attempts)

2. Background Job Queue: Redis + Bull (Node.js) or Celery (Python)
   - Purpose: Async email sending
   - Integration: In-process queue
   - SLA: Process within 60 seconds
   - Fallback: Dead letter queue for failed jobs
```

---

## Step 3.6: Context File Validation (Conditional)

**Objective:** Validate technical specification against constitutional context files

**Reference:** `.claude/skills/devforgeai-story-creation/references/context-validation.md`

**Trigger:** Only if `devforgeai/specs/context/` directory exists with context files

**Workflow:**

```
1. Check for context files:
   context_files = Glob(pattern="devforgeai/specs/context/*.md")

   IF len(context_files) == 0:
     Display: "ℹ️ Greenfield mode: context validation skipped"
     SKIP to Phase 4
```

```
2. If tech-stack.md exists:
   Read tech-stack.md
   Extract all technology names from generated technical spec
   Validate each technology appears in approved list

   IF violation found:
     AskUserQuestion:
       Question: "Technical spec references '{technology}' which is not in tech-stack.md. How to proceed?"
       Header: "Tech violation"
       Options:
         - "Remove from spec"
           Description: "Use approved alternative from tech-stack.md"
         - "Add to tech-stack.md"
           Description: "Requires ADR - update approved technologies"
         - "Flag for review"
           Description: "Proceed with warning, review later"
```

```
3. If dependencies.md exists:
   Read dependencies.md
   Extract all package names from Dependencies section (Step 3.5)
   Validate each package appears in approved list

   IF violation found:
     AskUserQuestion:
       Question: "Dependency '{package}' is not in dependencies.md. How to proceed?"
       Header: "Dep violation"
       Options:
         - "Remove dependency"
           Description: "Find alternative or remove requirement"
         - "Add to dependencies.md"
           Description: "Requires ADR - add approved package"
         - "Flag for review"
           Description: "Proceed with warning, review later"
```

```
4. If architecture-constraints.md exists:
   Read architecture-constraints.md
   Check layer dependency matrix against proposed design
   Validate no forbidden cross-layer imports

   IF violation found:
     AskUserQuestion:
       Question: "Design violates layer boundary: {from_layer} → {to_layer}. How to proceed?"
       Header: "Arch violation"
       Options:
         - "Redesign"
           Description: "I'll provide compliant design"
         - "Flag for review"
           Description: "Proceed with warning, review later"
```

```
5. If anti-patterns.md exists:
   Read anti-patterns.md
   Scan technical spec for forbidden patterns
   Alert if SQL concatenation, God class, etc. detected

   IF violation found (CRITICAL severity):
     HALT: "Anti-pattern detected: {pattern}. Technical spec must be revised."
     AskUserQuestion for redesign guidance
```

**Output:** Validation report embedded in workflow state

```yaml
context_validation:
  phase: "3.6"
  files_checked: ["tech-stack.md", "dependencies.md", "architecture-constraints.md", "anti-patterns.md"]
  violations_found: 0
  status: "PASSED"
```

**On CRITICAL/HIGH Violations:** HALT and use AskUserQuestion
**On MEDIUM/LOW Violations:** Warn and continue with note

---

## Subagent Coordination

**Subagent used:** api-designer (conditional)

**Invoked if:** API endpoints detected in feature description or acceptance criteria

**Input provided:**
- User story
- Acceptance criteria
- Context files (tech-stack.md for technology constraints)

**Output expected:**
- OpenAPI 3.0 style contract
- HTTP method, endpoint path
- Request/response schemas
- Authentication requirements
- Validation rules

**Reference files used by subagent:**
- technical-specification-guide.md (API contract templates, patterns)

---

## Output

**Phase 3 produces:**
- ✅ API contracts (if applicable)
- ✅ Data models with attributes, constraints, relationships
- ✅ Business rules documented
- ✅ Dependencies identified with integration details

---

## Error Handling

**Error 1: API contracts incomplete**
- **Detection:** Missing request schema, response schemas, or status codes
- **Recovery:** Re-invoke api-designer with specific feedback

**Error 2: Data models vague**
- **Detection:** Missing data types, constraints, or relationships
- **Recovery:** Use AskUserQuestion to clarify entity structure

**Error 3: Business rules ambiguous**
- **Detection:** Rules contain "should", "might", "could" (not definitive)
- **Recovery:** Refine to use "must", "will", "shall"

See `error-handling.md` for comprehensive error recovery procedures.

---

## Next Phase

**After Phase 3 completes →** Phase 4: UI Specification

Load `ui-specification-creation.md` for Phase 4 workflow.