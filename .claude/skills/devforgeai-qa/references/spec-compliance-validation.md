---
name: spec-compliance-validation
description: Consolidated spec validation and workflow for QA Phase 2.3
version: "2.0"
consolidated_from:
  - spec-validation.md
  - spec-compliance-workflow.md
story: STORY-265
token_savings: "~1.5K tokens"
tags:
  - qa
  - validation
  - compliance
  - deferral
---

# Spec Compliance Validation Reference (Consolidated)

**Consolidated:** spec-validation.md + spec-compliance-workflow.md
**Token savings:** ~1.5K tokens (single load vs 2 separate loads)
**Version:** 2.0 (STORY-265)

---

## Table of Contents

- [Overview](#overview)
- [Phase 3 Workflow](#phase-3-workflow)
  - [Step 0: Validate Story Documentation](#step-0-validate-story-documentation-critical)
  - [Step 1: Load Story Specification](#step-1-load-story-specification)
  - [Step 2: Validate Acceptance Criteria](#step-2-validate-acceptance-criteria)
  - [Step 2.4: Pre-Flight Deferral Detection](#step-24-pre-flight-deferral-detection-mandatory)
  - [Step 2.5: Validate Deferred DoD Items](#step-25-validate-deferred-definition-of-done-items-mandatory---cannot-skip)
  - [Step 3: Validate API Contracts](#step-3-validate-api-contracts)
  - [Step 4: Validate Non-Functional Requirements](#step-4-validate-non-functional-requirements)
  - [Step 5: Generate Traceability Matrix](#step-5-generate-traceability-matrix)
- [Validation Algorithms](#validation-algorithms)
- [Deferral Validation Categories](#deferral-validation-categories)
- [Quick Reference](#quick-reference)

---

## Overview

Phase 3 ensures implementation matches story specification and validates all deferred Definition of Done items.

**CRITICAL:** Step 2.5 (deferral validation) CANNOT be skipped. See `dod-protocol.md` for rationale.

**Purpose:**
- Validates implementation against story specifications and acceptance criteria
- Enforces deferral validation protocol (RCA-007)
- Generates traceability matrix (requirement → test → code)

---

## Phase 3 Workflow

### Step 0: Validate Story Documentation (CRITICAL)

**Before validating code, verify story file is properly documented**

```
Read(file_path="devforgeai/specs/Stories/{story_id}.story.md")
```

#### Check for Implementation Notes Section

```
IF "## Implementation Notes" NOT found in story:
    VIOLATION:
      Type: "Story documentation missing"
      Severity: HIGH
      Message: "Story file lacks Implementation Notes section"
      Impact: "Cannot validate spec compliance without documented implementation"
      Remediation: "Developer must update story file with Implementation Notes before QA approval"

    Record violation in QA report

    IF deep mode:
        FAIL QA - Story documentation is mandatory for deep validation
    ELSE IF light mode:
        WARN - Story documentation missing (not blocking for light QA)
```

#### If Implementation Notes Exist, Validate Completeness

```
Extract Implementation Notes section

Check required subsections:
1. Definition of Done Status
   - Verify: Each DoD item from story has status ([x] or [ ])
   - Verify: Incomplete items ([]) have reason (deferred/blocked/out of scope)

2. Test Results
   - Verify: Test counts present (unit/integration/e2e)
   - Verify: Coverage percentage documented
   - Verify: All tests passing status documented

3. Acceptance Criteria Verification
   - Verify: Each acceptance criterion has verification entry
   - Verify: Verification method documented (test name, manual check, etc.)

4. Files Created/Modified
   - Verify: At least one file listed
   - Verify: Files organized by layer (if applicable)

IF any required subsection missing:
    VIOLATION:
      Type: "Story documentation incomplete"
      Severity: MEDIUM
      Message: "Implementation Notes missing required section: {section_name}"
      Remediation: "Add {section_name} to Implementation Notes"

    Record violation

    IF deep mode:
        WARN - Documentation incomplete (may impact compliance validation)
```

#### Validation Success

```
✓ Story documentation complete
✓ Definition of Done documented
✓ Implementation decisions preserved
✓ Test results recorded
✓ Acceptance criteria verification present

Continue to Step 1 (Load Story Specification)
```

---

### Step 1: Load Story Specification

```
Read(file_path="devforgeai/specs/Stories/{story_id}.story.md")
# Extract:
# - Acceptance criteria
# - API contracts
# - Non-functional requirements
# - Business rules
# - Implementation Notes (now validated in Step 0)
```

**Parse story sections:**
- Acceptance Criteria → List of Given/When/Then scenarios
- Technical Specification → API endpoints, data models, business rules
- Non-Functional Requirements → Performance, security, scalability targets
- Implementation Notes → DoD status, test results, AC verification

---

### Step 2: Validate Acceptance Criteria

**Using Implementation Notes for cross-reference**

```
FOR each acceptance_criterion:
    # Extract keywords
    keywords = extract_keywords(criterion)

    # Find tests matching criterion
    test_pattern = create_pattern(keywords)
    tests = Grep(pattern=test_pattern, path="tests/", output_mode="files_with_matches")

    IF no tests found:
        violations.append({
            "type": "Missing test for acceptance criterion",
            "severity": "HIGH",
            "criterion": criterion,
            "message": f"No tests for acceptance criterion: {criterion}",
            "suggested_test": generate_test_name(keywords),
            "remediation": "Create test covering this acceptance criterion"
        })

    IF tests found:
        # Verify tests pass
        Bash(command="[run specific tests]")

        IF tests fail:
            violations.append({
                "type": "Failing test for acceptance criterion",
                "severity": "CRITICAL",
                "criterion": criterion,
                "test_file": tests[0],
                "message": f"Tests fail for criterion: {criterion}",
                "remediation": "Fix failing tests before QA approval"
            })
            BLOCK: QA approval CANNOT proceed
```

---

### Step 2.4: Pre-Flight Deferral Detection [MANDATORY]

**Purpose:** Technically enforce deferral-validator invocation when deferrals exist.

**Background:** RCA-007 documented manual validation missing multi-level chains. This pre-flight check ensures deferral-validator subagent is ALWAYS invoked when DoD items are incomplete.

**Constitution Alignment:** HALT immediately on violations (anti-patterns.md line 218)

#### Detection Logic

```
# Step 1: Count unchecked DoD items
Grep(pattern="- \\[ \\]", file_path="devforgeai/specs/Stories/{STORY-ID}.story.md")

unchecked_count = grep_result.count

# Step 2: Set deferral flag
IF unchecked_count > 0:
    $DEFERRALS_EXIST = true
    Display: "⚠️ Found {unchecked_count} unchecked DoD items"
    Display: "   deferral-validator subagent REQUIRED per RCA-007"
ELSE:
    $DEFERRALS_EXIST = false
    Display: "✓ All DoD items complete - no deferrals"
```

#### Enforcement at Step 2.5 End

```
IF $DEFERRALS_EXIST == true:
    # Verify deferral-validator was invoked
    Check conversation for: Task(subagent_type="deferral-validator"...)

    IF NOT found:
        Display: "❌ VIOLATION: Deferral validation CANNOT be skipped (RCA-007)"
        Display: "Found {unchecked_count} unchecked DoD items but deferral-validator was not invoked"
        Display: "Resolution: Invoke deferral-validator subagent now"
        HALT: "Deferral validation is MANDATORY when deferrals exist"

        # Force invocation
        Task(subagent_type="deferral-validator",
             prompt="Validate deferred DoD items for {STORY_ID}. Check for:
                     - User approval for each deferral
                     - Story/ADR references exist
                     - No circular chains (A→B→A)
                     - No multi-level chains (A→B→C)")
    ELSE:
        Display: "✓ deferral-validator invoked - protocol followed"
```

**Reference:** See RCA-007 in `devforgeai/RCA/` for incident details.

---

### Step 2.5: Validate Deferred Definition of Done Items (MANDATORY - CANNOT SKIP)

⚠️ **CRITICAL ENFORCEMENT RULE (RCA-007):**

This step is MANDATORY and CANNOT be skipped under any circumstances.

#### Protocol Adherence Check

**If you are tempted to skip this step:**
1. **STOP immediately** - Do not proceed with manual validation
2. **Use AskUserQuestion** to request permission from user with clear justification
3. **Document deviation** in QA report (why protocol was not followed)
4. **Mark QA status** as "PASSED WITH EXCEPTIONS" (not standard PASSED)

**PROHIBITED Shortcuts (from RCA-007):**
- ❌ "Manual validation is equivalent" - It misses multi-level chains, ADR checks
- ❌ "Story already approved" - Re-validation must be thorough
- ❌ "Token optimization" - Cannot bypass quality for efficiency
- ❌ "Reason exists = valid" - Reason must be VALIDATED

**See `dod-protocol.md` for complete protocol requirements.**

#### Extract Incomplete DoD Items

```
Read Implementation Notes > Definition of Done Status section
Parse all items marked [ ] (incomplete)

incomplete_dod_items = []

FOR each dod_item in story.definition_of_done:
    IF dod_item.status == "[ ]":  # Incomplete
        incomplete_dod_items.append({
            "item": dod_item.description,
            "reason": extract_deferral_reason(dod_item)
        })
```

#### If No Incomplete Items

```
IF incomplete_dod_items.count == 0:
    Display: "✓ All Definition of Done items complete - no deferral validation needed"
    Continue to Step 3 (Validate API Contracts)
```

#### If Incomplete Items Found - MANDATORY Validation

```
HALT QA validation

Display to user:
"❌ STORY-{story_id} has {count} deferred items - deferral validation REQUIRED

Deferred items found:
{list each item with its deferral reason}

Invoking deferral-validator subagent (MANDATORY)...
This automated check validates:
- Referenced stories exist and include deferred work
- No multi-level deferral chains (A→B→C) ← RCA-007
- No circular deferral chains (A→B→A)
- ADR approval for scope changes
- Technical justification for external blockers
- Story reference accuracy

Proceeding with automated validation..."
```

#### Invoke deferral-validator Subagent

```
Task(
    subagent_type="deferral-validator",
    description="Validate deferral justifications for QA",
    prompt="Validate all deferred DoD items for QA approval.

            Story loaded in conversation.

            Perform comprehensive validation:
            - Technical blocker verification (external dependencies)
            - Implementation feasibility check (code patterns in spec)
            - ADR requirement for scope changes (DoD item in scope but deferred)
            - Circular deferral detection (STORY-A → STORY-B → STORY-A)
            - Multi-level chain detection (STORY-A → STORY-B → STORY-C) ← RCA-007
            - Referenced story validation (exists and includes work)

            Return JSON validation report with violations by severity."
)
```

#### Handle Validation Results

**CRITICAL Violations (Circular Deferrals):**
```
IF validation_results.critical_violations.count > 0:
    QA Status: FAILED
    HALT QA approval

    Display:
    "❌ QA FAILED - Circular deferral chains detected

    Circular chains prevent work completion. Developer must:
    {remediation from subagent}

    Run `/dev {story_id}` to fix deferrals, then re-run `/qa {story_id}`"
```

**HIGH Violations:**
- Unjustified deferrals (no reason or vague reason)
- Invalid story references (STORY-XXX doesn't exist)
- Unnecessary deferrals (implementation feasible now)
- Multi-level chains (STORY-A → STORY-B → STORY-C) ← RCA-007

**MEDIUM Violations (non-blocking):**
- Scope change without ADR (should document decision)
- External blocker missing ETA (should track timeline)

---

### Step 3: Validate API Contracts

**Verify all API endpoints from spec are implemented correctly**

```
FOR each api_endpoint in spec.technical_specification.api_contracts:
    # Check endpoint exists
    endpoint_pattern = create_route_pattern(api_endpoint)
    found = Grep(pattern=endpoint_pattern, path="src/")

    IF not found:
        violations.append({
            "type": "Missing API endpoint",
            "severity": "CRITICAL",
            "endpoint": api_endpoint.path,
            "method": api_endpoint.method,
            "message": f"Endpoint not implemented: {api_endpoint.method} {api_endpoint.path}",
            "remediation": "Implement missing endpoint per spec"
        })
        BLOCK: QA approval CANNOT proceed

    IF found:
        # Validate request/response models
        actual_request = extract_request_model(found.file)
        actual_response = extract_response_model(found.file)

        IF actual_request != api_endpoint.request_model:
            violations.append({
                "type": "API contract violation - request model",
                "severity": "HIGH",
                ...
            })

        IF actual_response != api_endpoint.response_model:
            violations.append({
                "type": "API contract violation - response model",
                "severity": "HIGH",
                ...
            })
```

**Contract elements validated:**
- HTTP method (GET/POST/PUT/DELETE)
- Route path and parameters
- Request body schema
- Response body schema
- Status codes
- Authentication requirements

---

### Step 4: Validate Non-Functional Requirements

**Verify NFRs from spec are addressed**

```
FOR each nfr in spec.non_functional_requirements:
    IF nfr.type == "performance":
        # Check for performance tests
        perf_tests = Glob(pattern="tests/Performance/**/*")

        IF no perf_tests:
            violations.append({
                "type": "Missing performance tests",
                "severity": "HIGH",
                "nfr": nfr.description,
                "message": f"No performance tests for NFR: {nfr.description}",
                "remediation": "Create performance tests to validate requirement"
            })

    IF nfr.type == "security":
        # Check authentication/authorization
        endpoints = find_api_endpoints()

        FOR endpoint in endpoints:
            IF not has_auth_attribute(endpoint):
                violations.append({
                    "type": "Missing authentication",
                    "severity": "CRITICAL",
                    ...
                })
                BLOCK: QA approval CANNOT proceed

    IF nfr.type == "scalability":
        Check for caching, pagination, async processing
        Document findings

    IF nfr.type == "availability":
        health_check = Grep(pattern="/health|/ping", path="src/")
        IF not found:
            violations.append(MEDIUM: "No health check endpoint for availability NFR")
```

**NFR types validated:**
- Performance (response time, throughput, resource usage)
- Security (authentication, authorization, encryption)
- Scalability (caching, load balancing, pagination)
- Availability (health checks, monitoring, SLA)
- Reliability (error handling, retries, timeouts)
- Maintainability (code quality, documentation)

---

### Step 5: Generate Traceability Matrix

**Create mapping: Requirement → Tests → Implementation**

```
traceability_matrix = []

FOR criterion in acceptance_criteria:
    tests = find_tests_for_criterion(criterion)
    impl = find_implementation(criterion)

    matrix_entry = {
        "requirement": criterion,
        "tests": [test.name for test in tests],
        "implementation": [file for file in impl],
        "status": "COMPLETE" if tests and impl else "INCOMPLETE",
        "coverage": calculate_coverage(tests, impl)
    }

    traceability_matrix.append(matrix_entry)

    IF matrix_entry.status == "INCOMPLETE":
        violations.append({
            "type": "Incomplete traceability",
            "severity": "HIGH",
            "criterion": criterion,
            "missing": "tests" if not tests else "implementation",
            "message": f"Requirement not fully traced: {criterion}",
            "remediation": f"Add missing {'tests' if not tests else 'implementation'}"
        })
```

**Matrix format:**

| Requirement | Tests | Implementation | Status | Coverage |
|-------------|-------|----------------|--------|----------|
| User can login with email | test_login_with_email.py | AuthController.Login() | ✅ COMPLETE | 100% |
| Invalid email rejected | test_login_validation.py | EmailValidator.Validate() | ✅ COMPLETE | 100% |
| Password reset flow | (none found) | PasswordResetService | ❌ INCOMPLETE | 0% |

---

## Validation Algorithms

### Acceptance Criteria Mapping

```python
def validate_acceptance_criteria(story_file, test_path):
    story = parse_story(story_file)
    criteria = story["acceptance_criteria"]

    validation_report = []

    for criterion in criteria:
        # Extract keywords
        keywords = extract_keywords(criterion)
        # Example: "User can add items to cart"
        # Keywords: ["add", "items", "cart"]

        # Find tests
        test_pattern = f"({'|'.join(keywords)})"
        matching_tests = grep(test_pattern, test_path, ignore_case=True)

        if not matching_tests:
            validation_report.append({
                "criterion": criterion,
                "status": "FAIL",
                "issue": "No tests found",
                "action": f"Add test: Test_Can{keywords[0].title()}_{keywords[-1].title()}"
            })
        else:
            results = run_tests(matching_tests)
            validation_report.append({
                "criterion": criterion,
                "status": "PASS" if results.all_passed else "FAIL",
                "tests": matching_tests,
                "results": results
            })

    return validation_report
```

### API Endpoint Validation

```python
def validate_api_endpoint(spec_endpoint, source_path):
    violations = []

    # Check endpoint exists
    endpoint_pattern = f"(Route|@route|app\\.{spec_endpoint.method.lower()}).*{spec_endpoint.path}"
    found = grep(endpoint_pattern, source_path)

    if not found:
        violations.append({
            "severity": "CRITICAL",
            "issue": f"Endpoint not found: {spec_endpoint.method} {spec_endpoint.path}",
            "action": "Implement missing endpoint"
        })
        return violations

    # Validate request model
    controller_file = found[0].file
    actual_request = extract_request_model(controller_file, spec_endpoint.path)
    expected_request = spec_endpoint.request_model

    if not models_match(actual_request, expected_request):
        violations.append({
            "severity": "HIGH",
            "issue": "Request model mismatch",
            "expected": expected_request,
            "actual": actual_request,
            "endpoint": spec_endpoint.path
        })

    # Validate response model
    actual_response = extract_response_model(controller_file, spec_endpoint.path)
    expected_response = spec_endpoint.response_model

    if not models_match(actual_response, expected_response):
        violations.append({
            "severity": "HIGH",
            "issue": "Response model mismatch",
            "expected": expected_response,
            "actual": actual_response
        })

    return violations
```

### Database Schema Validation

```python
def validate_database_schema(spec, migration_path):
    violations = []

    for table in spec.tables:
        migration_pattern = f"*{table.name}*.cs"
        migrations = glob(f"{migration_path}/{migration_pattern}")

        if not migrations:
            violations.append({
                "table": table.name,
                "issue": "Migration not found",
                "action": f"Create migration for {table.name} table"
            })
            continue

        # Validate schema matches spec
        migration = read_migration(migrations[0])
        actual_schema = parse_schema(migration)

        if not schema_matches(actual_schema, table.schema):
            violations.append({
                "table": table.name,
                "issue": "Schema mismatch",
                "expected": table.schema,
                "actual": actual_schema
            })

    return violations
```

### Error Handling Validation

```python
def validate_error_handling(coding_standards, source_path):
    pattern = coding_standards["error_handling"]["pattern"]
    violations = []

    if pattern == "Result Pattern":
        business_logic = glob("src/{Services,Application}/**/*.cs")

        for file in business_logic:
            public_methods = grep("public.*\\(", file)

            for method in public_methods:
                if not returns_result_type(method):
                    exceptions = grep("throw new.*Exception", file, context=method)

                    if exceptions:
                        violations.append({
                            "file": file,
                            "method": method.name,
                            "issue": "Uses exceptions instead of Result Pattern",
                            "expected": "Return Result<T> for business logic errors"
                        })

    return violations
```

---

## Deferral Validation Categories

### Category 1: Valid Deferrals (Pass Validation)

**1. External Blocker**
- Pattern: "Blocked by {external_system}: {specific_reason}"
- Example: "Blocked by Payment API v2 (available 2025-12-01)"
- Validation: Check blocker is external (not internal code)

**2. Scope Change with ADR**
- Pattern: "Out of scope: ADR-XXX"
- Example: "Out of scope: ADR-042 descoped performance benchmarks"
- Validation: Verify ADR-XXX exists, created recently, documents this change

**3. Story Split**
- Pattern: "Deferred to STORY-XXX: {justification}"
- Example: "Deferred to STORY-125 (performance optimization epic)"
- Validation: Verify STORY-XXX exists, includes deferred work, no circular chain, no multi-level chain (RCA-007)

### Category 2: Invalid Deferrals (FAIL QA)

| Pattern | Violation | Severity |
|---------|-----------|----------|
| No justification | Missing deferral justification | HIGH |
| Vague reason ("Will add later") | Invalid deferral reason | HIGH |
| Story A → Story B → Story A | Circular deferral detected | CRITICAL |
| Story A → Story B → Story C | Multi-level chain (RCA-007) | HIGH |
| STORY-XXX doesn't exist | Referenced story not found | HIGH |
| "Out of scope" without ADR | Scope change requires ADR | MEDIUM |
| Feasible now | Unnecessary deferral | HIGH |

**Full decision tree:** `deferral-decision-tree.md`

---

## Quick Reference

### Validation Checklist

- [ ] All acceptance criteria have tests
- [ ] All tests pass
- [ ] API endpoints match spec
- [ ] Request/response models match spec
- [ ] Database schema matches spec
- [ ] Error handling follows standards
- [ ] NFRs validated (performance, security)
- [ ] Traceability complete (requirement → test → code)
- [ ] Deferral validation invoked (if deferrals exist)

### Phase 3 Output

**Results to carry forward:**
- Story documentation validation (Step 0)
- Acceptance criteria validation (Step 2)
- **Deferral validation results (Step 2.5)** ← MANDATORY
- API contract validation (Step 3)
- NFR validation (Step 4)
- Traceability matrix (Step 5)
- Violations by severity

### Step 2.5 Cannot Be Skipped

**Reinforcement:**
- Light mode: Step 2.5 executes (if deferrals exist)
- Deep mode: Step 2.5 executes (if deferrals exist)
- Re-validation: Step 2.5 executes (always check)
- No deferrals: Step 2.5 skips validation (but still checks for deferrals)

**Protocol compliance tracked in QA Validation History:**
```
### QA Attempt {N}
...
**Deferral Validation:** ✅ INVOKED (protocol followed)
  OR
**Deferral Validation:** ❌ SKIPPED (protocol violation - RCA-007)
```

---

**Continue to Phase 4 (Code Quality Metrics) after Phase 3 completes.**

---

This reference should be loaded when performing spec compliance validation during QA.
