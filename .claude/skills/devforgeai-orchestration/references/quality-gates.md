# Quality Gates Reference

Comprehensive reference for all quality gates in the DevForgeAI orchestration workflow.

## Overview

Quality gates are **validation checkpoints** that stories must pass before progressing to the next workflow stage. They enforce quality standards and prevent defects from reaching production.

### Gate Philosophy

- **Shift-left quality:** Catch issues early in development
- **Automated enforcement:** No manual approval needed for objective criteria
- **Fail-fast:** Block immediately when standards not met
- **Clear feedback:** Provide actionable guidance when gate fails

### Gate Hierarchy

```
Gate 1: Context Validation (before development)
  ↓
Gate 2: Test Passing (before QA)
  ↓
Gate 3: QA Approval (before release)
  ↓
Gate 4: Release Readiness (before production)
```

---

## Gate 1: Context Validation Gate

### Purpose

Ensure architectural foundation exists and is valid before development begins.

### Location in Workflow

**Checkpoint:** Architecture → Ready for Dev transition

### When Evaluated

- After `devforgeai-architecture` skill completes
- Before developer can start TDD workflow
- Automatically during orchestration

### Pass Criteria

All 6 checks must pass:

#### Check 1: All Context Files Exist

```
Required files:
✓ devforgeai/specs/context/tech-stack.md
✓ devforgeai/specs/context/source-tree.md
✓ devforgeai/specs/context/dependencies.md
✓ devforgeai/specs/context/coding-standards.md
✓ devforgeai/specs/context/architecture-constraints.md
✓ devforgeai/specs/context/anti-patterns.md

Validation:
FOR each file:
    IF NOT file_exists(file):
        FAIL: "Missing context file: {file}"
```

#### Check 2: Files Not Empty

```
Minimum size: 100 characters per file

Validation:
FOR each file:
    file_size = get_file_size(file)
    IF file_size < 100:
        FAIL: "{file} appears to be empty or minimal content"
```

#### Check 3: No Placeholder Content

```
Forbidden content:
- "TODO"
- "TBD"
- "To be determined"
- "[Insert X here]"
- Template boilerplate not replaced

Validation:
FOR each file:
    Read(file_path=file)
    IF "TODO" in content OR "TBD" in content:
        WARN: "{file} contains placeholder content"
        AskUserQuestion: "Proceed anyway or complete placeholders?"
```

#### Check 4: Architecture Constraints Defined

```
Required in architecture-constraints.md:
✓ Layer boundaries documented
✓ Dependency rules specified (layer dependency matrix)
✓ Design patterns identified
✓ No circular dependencies allowed

Validation:
Read(file_path="devforgeai/specs/context/architecture-constraints.md")

IF "layer" not in content.lower():
    FAIL: "No layer boundaries defined"

IF "dependencies" not in content.lower():
    FAIL: "No dependency rules specified"
```

#### Check 5: Tech Stack Locked

```
Required in tech-stack.md:
✓ All technology choices documented
✓ Versions specified (not "latest")
✓ Rationale provided (or ADR reference)
✓ At least one "LOCKED" designation

Validation:
Read(file_path="devforgeai/specs/context/tech-stack.md")

IF "LOCKED" not in content:
    WARN: "No technologies marked as LOCKED"
    AskUserQuestion: "Proceed without locked tech stack?"

# Check for version specifications
technologies = extract_technologies(content)
FOR tech in technologies:
    IF tech.version == "latest" OR tech.version.empty:
        WARN: "{tech.name} has no specific version"
```

#### Check 6: Source Tree Documented

```
Required in source-tree.md:
✓ Folder structure defined
✓ File organization rules specified
✓ Naming conventions documented
✓ Example paths provided

Validation:
Read(file_path="devforgeai/specs/context/source-tree.md")

IF content_length < 300:
    FAIL: "Source tree documentation too minimal"

IF "src/" not in content AND "source/" not in content:
    WARN: "No source directory structure defined"
```

### Failure Actions

**When Gate Fails:**

1. **BLOCK** transition to Ready for Dev
2. Report which checks failed
3. Provide fix guidance
4. Re-invoke `devforgeai-architecture` if needed
5. Retry validation after fixes

**Example Failure Report:**
```
❌ Context Validation Gate FAILED

Failed Checks:
1. ✗ tech-stack.md contains placeholder "TODO: Add ORM choice"
2. ✗ architecture-constraints.md missing layer dependency matrix

Action Required:
1. Complete tech-stack.md (choose ORM, lock version)
2. Add layer dependency rules to architecture-constraints.md

Re-running architecture skill to complete context files...
```

### Bypass Mechanism

**Exceptions Allowed:** Yes, with documentation

**Bypass Process:**
```
IF context_validation_fails:
    AskUserQuestion:
    Question: "Context validation failed. Proceed anyway?"
    Header: "Context validation"
    Options:
      - "No, fix issues first (recommended)"
      - "Yes, proceed with incomplete context (document reason)"
      - "Skip specific check only (which?)"

    IF "Yes, proceed":
        Document exception in story:
            """
            ## Context Validation Exception
            - Date: {timestamp}
            - Reason: {user_provided_reason}
            - Failed Checks: {failed_checks}
            - Risk: Development without complete architecture context
            - Approved by: {user}
            """
        Allow transition
        Add to technical debt backlog
```

---

## Gate 2: Test Passing Gate

### Purpose

Ensure basic code quality and functionality before comprehensive QA validation.

### Location in Workflow

**Checkpoint:** Dev Complete → QA In Progress transition

### When Evaluated

- After TDD workflow Phase 6 (Git) completes
- Before deep QA validation starts
- Automatically during orchestration

### Pass Criteria

All 4 checks must pass:

#### Check 1: Build Succeeds

```
Build validation:
✓ No compilation errors
✓ No missing dependencies
✓ All projects build successfully
✓ Assets compile correctly

Validation:
# Run build command (language-specific)
build_result = Bash(command="[build_command]")
# Examples:
# - dotnet build
# - npm run build
# - python -m py_compile src/**/*.py
# - mvn compile

IF build_result.exit_code != 0:
    FAIL: "Build failed"
    Detail: {build_errors}
    Action: Fix compilation errors, return to In Development

# Check for warnings that should be errors
IF build_result.warnings > threshold:
    WARN: "{warning_count} build warnings detected"
```

#### Check 2: All Tests Pass

```
Test validation:
✓ Unit tests: 100% pass rate
✓ Integration tests: 100% pass rate
✓ E2E tests: 100% pass rate (if applicable)
✓ No flaky tests (3 consecutive runs pass)

Validation:
# Run test command (language-specific)
test_result = Bash(command="[test_command]")
# Examples:
# - dotnet test
# - pytest
# - npm test
# - mvn test

IF test_result.exit_code != 0:
    FAIL: "Tests failing"
    Detail:
        Total: {total_tests}
        Passed: {passed_tests}
        Failed: {failed_tests}
        Failures:
            {list_of_failed_tests_with_error_messages}
    Action: Fix failing tests, return to In Development

# Check for skipped tests
IF test_result.skipped > 0:
    WARN: "{skipped_count} tests skipped"
    AskUserQuestion: "Proceed with skipped tests?"
```

#### Check 3: No Syntax/Linting Errors

```
Linting validation:
✓ Code formatting correct (or auto-fixed)
✓ No critical linting violations
✓ Naming conventions followed
✓ No syntax errors

Validation:
# Run linter (language-specific)
lint_result = Bash(command="[linter_command]")
# Examples:
# - dotnet format --verify-no-changes
# - pylint src/
# - npm run lint

IF lint_result.has_errors:
    # Try auto-fix
    Bash(command="[auto_fix_command]")
    # Examples:
    # - dotnet format
    # - black src/
    # - npm run lint:fix

    # Re-run linter
    lint_result = Bash(command="[linter_command]")

    IF lint_result.still_has_errors:
        FAIL: "Linting violations remain after auto-fix"
        Detail: {unfixable_violations}
        Action: Fix manually, return to In Development
```

#### Check 4: Light Validation Passed

```
Light validation:
✓ Quick anti-pattern scan clean
✓ No critical security issues
✓ No obvious architecture violations
✓ File locations valid

Validation:
# Light validation runs during development (phases 3, 4, 5)
# Check for any blocking issues from light validation

IF light_validation_blocked:
    FAIL: "Light validation found blocking issues"
    Detail: {light_validation_issues}
    Action: Review and fix issues, return to In Development

# Common light validation checks:
# - SQL injection patterns
# - Hardcoded secrets
# - Cross-layer violations
# - Library substitution
```

### Failure Actions

**When Gate Fails:**

1. **BLOCK** transition to QA In Progress
2. Report specific failures
3. Return to In Development status
4. Require fixes before retrying
5. Re-run validation automatically after fixes

**Example Failure Report:**
```
❌ Test Passing Gate FAILED

Failed Checks:
1. ✗ Tests failing: 3 out of 47 tests
   - OrderServiceTests.CalculateDiscount_ExpiredCoupon_ReturnsOriginalPrice
   - UserServiceTests.Register_DuplicateEmail_ThrowsException
   - PaymentServiceTests.ProcessPayment_InsufficientFunds_ReturnsError

2. ✓ Build succeeds
3. ✓ Linting passed
4. ✓ Light validation clean

Action Required:
1. Fix 3 failing tests
2. Re-run development workflow
3. Gate will automatically re-validate

Returning to In Development status...
```

### Bypass Mechanism

**Exceptions Allowed:** NO

**Strict Enforcement:**
- All tests MUST pass (no exceptions)
- Build MUST succeed (no exceptions)
- No bypass allowed for Test Passing Gate
- This is a minimum quality bar

---

## Gate 3: QA Approval Gate

### Purpose

Ensure production readiness through comprehensive quality validation.

### Location in Workflow

**Checkpoint:** QA Approved → Releasing transition

### When Evaluated

- After deep QA validation completes with PASS status
- Before release can be initiated
- Automatically during orchestration

### Pass Criteria

All 9 checks must pass:

#### Check 1: Deep Validation PASSED

```
QA Status:
✓ Overall status = PASS
✓ QA report generated
✓ Report exists at devforgeai/qa/reports/{story-id}-qa-report.md

Validation:
Read(file_path="devforgeai/qa/reports/{story_id}-qa-report.md")

qa_status = parse_qa_status(report)
IF qa_status != "PASS":
    FAIL: "QA validation did not pass (status: {qa_status})"
```

#### Check 2: Coverage Meets Strict Thresholds

```
Coverage thresholds (STRICT):
✓ Business logic: ≥ 95%
✓ Application layer: ≥ 85%
✓ Infrastructure: ≥ 80%
✓ Overall project: ≥ 80%

Validation:
coverage = parse_coverage(qa_report)

IF coverage.business_logic < 95:
    FAIL: "Business logic coverage {coverage.business_logic}% < 95% threshold"

IF coverage.application < 85:
    FAIL: "Application coverage {coverage.application}% < 85% threshold"

IF coverage.infrastructure < 80:
    # May allow exception with approval
    IF NOT has_coverage_exception(qa_report):
        FAIL: "Infrastructure coverage {coverage.infrastructure}% < 80% threshold"

IF coverage.overall < 80:
    FAIL: "Overall coverage {coverage.overall}% < 80% threshold"
```

#### Check 3: Zero CRITICAL Violations

```
Critical violations (FORBIDDEN):
✗ SQL injection
✗ XSS vulnerabilities
✗ Hardcoded secrets
✗ Command injection
✗ Path traversal
✗ Insecure deserialization
✗ Circular deferrals (NEW - RCA-006)
✗ Library substitution
✗ Layer violations

Validation:
violations = parse_violations(qa_report)
critical_count = count_by_severity(violations, "CRITICAL")

IF critical_count > 0:
    FAIL: "{critical_count} CRITICAL violations detected"
    Detail:
        {list_critical_violations_with_files_and_lines}
    Action: Fix all critical security/architecture/deferral issues

Deferral-specific CRITICAL violations:
- Circular deferral chains (STORY-A → STORY-B → STORY-A)
```

#### Check 4: Zero HIGH Violations (or Approved Exceptions)

```
High violations (BLOCKED unless approved):
- Architecture layer violations
- Missing authorization checks
- Weak cryptography
- Missing input validation
- High complexity (>15)
- Unjustified deferrals (NEW - RCA-006)
- Invalid story references in deferrals (NEW - RCA-006)
- Unnecessary deferrals - implementation feasible (NEW - RCA-006)
- Missing acceptance criteria tests

Validation:
high_count = count_by_severity(violations, "HIGH")

IF high_count > 0:
    # Check for approved exceptions

Deferral-specific HIGH violations:
- DoD item deferred without valid technical justification
- Referenced follow-up story doesn't exist
- Deferred work not in referenced story's scope
- Implementation feasible now (code pattern in spec, <50 lines, no blockers)
    exceptions = parse_approved_exceptions(qa_report)

    unapproved_high = high_count - exceptions.high_count

    IF unapproved_high > 0:
        FAIL: "{unapproved_high} unapproved HIGH violations"
        Detail: {list_unapproved_violations}
        Action: Fix violations OR request exception approval
```

#### Check 5: All Acceptance Criteria Validated

```
Acceptance criteria validation:
✓ Tests exist for each criterion
✓ Tests pass
✓ Coverage adequate for criterion

Validation:
criteria_validation = parse_criteria_validation(qa_report)

FOR each criterion in story.acceptance_criteria:
    IF criterion NOT in criteria_validation:
        FAIL: "No tests found for criterion: {criterion}"

    IF criteria_validation[criterion].status != "PASS":
        FAIL: "Tests failing for criterion: {criterion}"

    IF criteria_validation[criterion].coverage < 80%:
        WARN: "Low coverage for criterion: {criterion} ({coverage}%)"
```

#### Check 6: API Contracts Match Spec

```
API contract validation (if applicable):
✓ Endpoints exist as specified
✓ Request models match spec
✓ Response models match spec
✓ Error handling compliant

Validation:
IF story.has_api_endpoints:
    api_validation = parse_api_validation(qa_report)

    FOR each endpoint in story.api_endpoints:
        IF endpoint NOT in api_validation:
            FAIL: "API endpoint not implemented: {endpoint.method} {endpoint.path}"

        IF api_validation[endpoint].request_mismatch:
            FAIL: "Request model mismatch for {endpoint.path}"
            Detail: {request_diff}

        IF api_validation[endpoint].response_mismatch:
            FAIL: "Response model mismatch for {endpoint.path}"
            Detail: {response_diff}
```

#### Check 7: NFRs Validated

```
Non-functional requirements validation:
✓ Performance tests pass (if applicable)
✓ Security scan clean
✓ Scalability requirements met

Validation:
IF story.has_nfrs:
    nfr_validation = parse_nfr_validation(qa_report)

    IF story.nfrs.performance:
        IF NOT nfr_validation.performance.passed:
            FAIL: "Performance NFR not met"
            Detail:
                Expected: {nfr.performance.target}
                Actual: {nfr_validation.performance.actual}

    IF story.nfrs.security:
        IF nfr_validation.security.vulnerabilities > 0:
            FAIL: "Security NFR not met - vulnerabilities detected"

    IF story.nfrs.scalability:
        IF NOT nfr_validation.scalability.passed:
            WARN: "Scalability NFR validation incomplete"
```

#### Check 8: Code Quality Metrics Within Thresholds

```
Quality metrics thresholds:
✓ Cyclomatic complexity: Methods ≤10, Classes ≤50
✓ Maintainability index: ≥70
✓ Code duplication: ≤5%
✓ Documentation coverage: ≥80% (public APIs)

Validation:
quality_metrics = parse_quality_metrics(qa_report)

# Complexity
IF quality_metrics.max_method_complexity > 10:
    high_complexity_methods = filter_high_complexity(quality_metrics.methods)
    IF high_complexity_methods.count > 3:
        FAIL: "{count} methods exceed complexity threshold"
    ELSE:
        WARN: "Some methods have high complexity"

# Maintainability
avg_maintainability = quality_metrics.average_maintainability_index
IF avg_maintainability < 70:
    FAIL: "Maintainability index {avg_maintainability} < 70"

# Duplication
IF quality_metrics.duplication_percentage > 5:
    WARN: "Code duplication {duplication}% > 5% threshold"

# Documentation
IF quality_metrics.documentation_coverage < 80:
    WARN: "Documentation coverage {coverage}% < 80%"

# Deferrals (NEW - RCA-006)
deferral_violations_medium = count_by_type_and_severity(violations, "deferral", "MEDIUM")
IF deferral_violations_medium > 0:
    WARN: "{count} MEDIUM deferral issues detected"
    Detail:
        - Scope changes without ADR documentation
        - External blockers missing ETA
    Action: Document exceptions in QA report OR create ADRs
```

**Deferral-Specific MEDIUM Violations:**
- Scope change without ADR (DoD item removed from scope but no ADR-XXX documentation)
- External blocker missing resolution condition (no ETA or "when available" statement)
- Invalid deferral reason format (doesn't match required patterns)

#### Check 9: QA Report Generated

```
Report validation:
✓ Report exists
✓ Contains all 5 validation phases
✓ Action items documented (if any)
✓ Recommendations provided

Validation:
Read(file_path=f"devforgeai/qa/reports/{story_id}-qa-report.md")

required_sections = [
    "Test Coverage Analysis",
    "Anti-Pattern Detection",
    "Spec Compliance",
    "Code Quality Metrics",
    "Recommendations"
]

FOR section in required_sections:
    IF section NOT in report_content:
        FAIL: "QA report missing section: {section}"

IF report_length < 1000:  # Arbitrary minimum
    WARN: "QA report seems incomplete (too short)"
```

### Failure Actions

**When Gate Fails:**

1. **BLOCK** transition to Releasing
2. Transition to QA Failed status
3. Create detailed action items
4. Return to In Development
5. Require fixes and re-validation

**Example Failure Report:**
```
❌ QA Approval Gate FAILED

Failed Checks:
1. ✗ Coverage below threshold
   - Business Logic: 92% (required: 95%)
   - Infrastructure: 75% (required: 80%)

2. ✗ 2 HIGH violations detected
   - Weak password hashing (MD5 used instead of bcrypt)
   - Missing authorization check on DELETE /api/users/{id}

3. ✓ Zero CRITICAL violations
4. ✓ All acceptance criteria validated
5. ✓ API contracts match spec
6. ✓ NFRs validated
7. ✓ Code quality metrics within thresholds
8. ✓ QA report generated

Overall: FAIL (2 of 9 checks failed)

Action Items:
1. [P0] Replace MD5 password hashing with bcrypt
2. [P0] Add authorization check to DELETE endpoint
3. [P1] Add tests to increase business logic coverage to 95%
4. [P1] Add tests for infrastructure utilities

After fixes:
- Re-run development workflow
- QA will automatically re-validate
- If PASS, story will move to QA Approved
```

### Bypass Mechanism

**Exceptions Allowed:** Limited, with strong justification

**Exception Categories:**

1. **Coverage Exceptions:**
   - Non-critical code (logging, diagnostics)
   - Requires `AskUserQuestion` approval
   - Documented in QA report

2. **Quality Metric Exceptions:**
   - Inherent business complexity
   - Requires justification
   - Documented as accepted technical debt

3. **NO Exceptions For:**
   - CRITICAL violations (never allowed)
   - HIGH violations (must be approved explicitly)
   - Test failures (must all pass)

---

## Gate 4: Release Readiness Gate

### Purpose

Final safety check before production deployment.

### Location in Workflow

**Checkpoint:** Releasing → Released transition

### When Evaluated

- After deployment completes
- Before marking story as Released
- Automatically by release skill

### Pass Criteria

All 6 checks must pass:

#### Check 1: QA Approved

```
QA validation:
✓ Story status = QA Approved (or Releasing)
✓ QA report status = PASS
✓ QA report recent (< 7 days old)

Validation:
story_status = get_story_status(story_id)

IF story_status NOT in ["QA Approved", "Releasing"]:
    FAIL: "Story not QA approved (status: {story_status})"

Read(file_path=f"devforgeai/qa/reports/{story_id}-qa-report.md")
qa_status = parse_qa_status(report)

IF qa_status != "PASS":
    FAIL: "QA report shows FAIL status"

report_age = now - report_timestamp
IF report_age.days > 7:
    WARN: "QA report is {report_age.days} days old"
    AskUserQuestion: "Re-run QA validation before release?"
```

#### Check 2: All Workflow Checkboxes Complete

```
Workflow validation:
✓ Architecture phase complete
✓ Development phase complete
✓ QA phase complete

Validation:
Read(file_path="ai_docs/Stories/{story_id}.story.md")
workflow_status = extract_section("Workflow Status")

IF NOT "- [x] Architecture phase complete" in workflow_status:
    FAIL: "Architecture phase not marked complete"

IF NOT "- [x] Development phase complete" in workflow_status:
    FAIL: "Development phase not marked complete"

IF NOT "- [x] QA phase complete" in workflow_status:
    FAIL: "QA phase not marked complete"
```

#### Check 3: No Blocking Dependencies

```
Dependency validation:
✓ All prerequisite stories released
✓ External dependencies available
✓ No unresolved blockers

Validation:
dependencies = extract_section("Dependencies")

# Check prerequisite stories
FOR story_id in dependencies.prerequisite_stories:
    prerequisite_status = get_story_status(story_id)
    IF prerequisite_status != "Released":
        FAIL: "Prerequisite story not released: {story_id} (status: {prerequisite_status})"

# Check external dependencies
FOR dependency in dependencies.external:
    dependency_available = check_dependency_availability(dependency)
    IF NOT dependency_available:
        FAIL: "External dependency unavailable: {dependency}"

# Check for active blockers
IF story_has_active_blockers(story_id):
    FAIL: "Story has active blockers"
```

#### Check 4: Deployment Successful

```
Deployment validation:
✓ Deployment completed without errors
✓ All services started successfully
✓ Database migrations applied (if needed)
✓ Configuration deployed correctly

Validation:
deployment_status = check_deployment_status()

IF deployment_status.status != "success":
    FAIL: "Deployment failed"
    Detail: {deployment_status.errors}
    Action: Rollback deployment

IF deployment_status.has_warnings:
    WARN: "Deployment completed with warnings"
    Detail: {deployment_status.warnings}

# Verify services started
FOR service in deployed_services:
    IF NOT service.is_running:
        FAIL: "Service failed to start: {service.name}"
        Action: Rollback deployment
```

#### Check 5: Health Checks Pass

```
Health validation:
✓ Application health endpoint responds
✓ Database connectivity verified
✓ External API dependencies accessible
✓ No critical errors in logs (last 5 minutes)

Validation:
# Application health
health_response = http_get("https://api.production.com/health")
IF health_response.status_code != 200:
    FAIL: "Health check failed (HTTP {health_response.status_code})"

health_data = health_response.json()
IF health_data.status != "healthy":
    FAIL: "Application reports unhealthy status"
    Detail: {health_data.details}

# Database connectivity
db_health = check_database_connectivity()
IF NOT db_health.connected:
    FAIL: "Database not accessible"

# External dependencies
FOR dependency in external_apis:
    dep_health = check_dependency_health(dependency)
    IF NOT dep_health.accessible:
        FAIL: "External dependency not accessible: {dependency}"

# Log errors
recent_errors = check_logs_for_errors(last_minutes=5)
IF recent_errors.critical_count > 0:
    FAIL: "{recent_errors.critical_count} critical errors in logs"
    Detail: {recent_errors.sample_errors}
```

#### Check 6: Rollback Plan Ready

```
Rollback validation:
✓ Rollback procedure documented
✓ Previous version identified
✓ Rollback tested (in staging)
✓ Rollback triggers defined

Validation:
# Check for rollback documentation
IF NOT deployment_has_rollback_plan():
    WARN: "No documented rollback plan"
    AskUserQuestion: "Proceed without rollback plan?"

# Verify rollback capability
rollback_ready = verify_rollback_capability()
IF NOT rollback_ready.can_rollback:
    FAIL: "Rollback not possible"
    Detail: {rollback_ready.blockers}
```

### Failure Actions

**When Gate Fails:**

1. **BLOCK** transition to Released
2. Execute rollback plan (if deployment issue)
3. Investigate failure
4. Fix issues
5. Retry release

**Example Failure Report:**
```
❌ Release Readiness Gate FAILED

Failed Checks:
1. ✓ QA Approved
2. ✓ All workflow checkboxes complete
3. ✓ No blocking dependencies
4. ✗ Deployment partially failed
   - Service: order-service (failed to start)
   - Error: Port 8080 already in use
5. ✗ Health checks failing
   - Health endpoint: HTTP 503
   - Database: Connected
   - External APIs: Accessible
   - Log errors: 3 critical errors detected
6. ✓ Rollback plan ready

Action: Executing rollback to previous version...

Rollback Steps:
1. Stop failed deployment
2. Restore previous version
3. Verify health checks
4. Investigate deployment failure

After fixing:
- Resolve port conflict
- Re-run deployment
- Gate will re-validate
```

### Bypass Mechanism

**Exceptions Allowed:** NO (for production)

**Strict Enforcement:**
- All health checks MUST pass
- Deployment MUST succeed
- No bypass allowed for production releases
- For non-production environments, may allow exceptions with documentation

---

## Gate Summary Matrix

| Gate | When | Can Fail? | Can Bypass? | Auto-Retry? |
|------|------|-----------|-------------|-------------|
| Context Validation | Architecture → Ready for Dev | Yes | Yes, with docs | No |
| Test Passing | Dev Complete → QA In Progress | Yes | No | After fixes |
| QA Approval | QA Approved → Releasing | Yes | Limited | After fixes |
| Release Readiness | Releasing → Released | Yes | No (prod) | After fixes |

## Gate Enforcement Levels

### CRITICAL (Block Immediately, No Bypass)
- Test Passing Gate: Tests must pass
- QA Approval Gate: Zero CRITICAL violations
- Release Readiness Gate: Health checks must pass

### HIGH (Block, Allow Exceptions with Approval)
- Context Validation Gate: Context files exist
- QA Approval Gate: Coverage thresholds, HIGH violations

### MEDIUM (Warning, Can Proceed with Documentation)
- Context Validation Gate: Placeholder content
- QA Approval Gate: Quality metrics slightly below threshold

### LOW (Advisory Only)
- Stale context files
- Documentation coverage below ideal

---

**Use this reference when:**
- Implementing quality gate logic
- Understanding gate requirements
- Debugging gate failures
- Requesting gate exceptions
- Training on quality standards
