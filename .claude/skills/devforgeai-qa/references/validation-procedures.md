# Validation Procedures

Detailed procedures for light and deep validation modes in the DevForgeAI QA workflow.

---

## Light Validation Procedures

**Purpose**: Fast feedback loop during development phases

**Token Budget**: ~10,000 tokens per validation

**Use Cases**:
- After Phase 2 (Implementation - Green)
- After Phase 3 (Refactor)
- After Phase 4 (Integration)

---

### Step 1: Syntax & Build Validation

**Check compilation and syntax:**

```
# Determine build command from tech-stack.md
Read(file_path="devforgeai/specs/context/tech-stack.md")

# Execute build based on detected language
IF language == ".NET":
    Bash(command="dotnet build")
ELSE IF language == "Python":
    Bash(command="python -m py_compile src/**/*.py")
ELSE IF language == "Node.js":
    Bash(command="npm run build")
ELSE IF language == "Go":
    Bash(command="go build ./...")
ELSE IF language == "Rust":
    Bash(command="cargo build")
ELSE IF language == "Java":
    Bash(command="mvn compile")

IF build fails:
    BLOCK immediately
    Report: Compilation errors detected
    Detail: [Parse error messages from build output]
    Action: Fix syntax/build errors before continuing
    Return: BLOCKED
```

**Check dependency resolution:**

```
# Check dependencies are installable/resolvable
IF language == ".NET":
    Bash(command="dotnet restore")
ELSE IF language == "Python":
    Bash(command="pip install -r requirements.txt --dry-run")
ELSE IF language == "Node.js":
    Bash(command="npm install --dry-run")
ELSE IF language == "Go":
    Bash(command="go mod download")
ELSE IF language == "Rust":
    Bash(command="cargo fetch")
ELSE IF language == "Java":
    Bash(command="mvn dependency:resolve")

IF dependencies fail:
    BLOCK immediately
    Report: Dependency resolution failed
    Detail: [Parse dependency error messages]
    Action: Fix missing/conflicting packages
    Return: BLOCKED
```

---

### Step 2: Basic Linting

**Run language-specific linter:**

```
# Determine linter from tech-stack.md
Read(file_path="devforgeai/specs/context/tech-stack.md")

# Run linter
IF language == ".NET":
    Bash(command="dotnet format --verify-no-changes")
ELSE IF language == "Python":
    Bash(command="pylint src/")
ELSE IF language == "Node.js":
    Bash(command="npm run lint")
ELSE IF language == "Go":
    Bash(command="golint ./...")
ELSE IF language == "Rust":
    Bash(command="cargo clippy")
ELSE IF language == "Java":
    Bash(command="mvn checkstyle:check")

IF linting violations found:
    # Attempt auto-fix for safe violations
    IF language == ".NET":
        Bash(command="dotnet format")
    ELSE IF language == "Python":
        Bash(command="black src/")
        Bash(command="isort src/")
    ELSE IF language == "Node.js":
        Bash(command="npm run lint -- --fix")
    ELSE IF language == "Rust":
        Bash(command="cargo fmt")

    # Re-run linter to check if fixed
    [Re-execute linter command]

    IF still failing:
        BLOCK immediately
        Report: Linting violations remain after auto-fix
        Detail: [Violations that couldn't be auto-fixed]
        Action: Fix manually (naming conventions, structure issues)
        Return: BLOCKED
```

---

### Step 3: Test Execution

**Determine which tests to run based on phase:**

```
IF phase == "implementation":
    # Run only new tests (related to current story)
    # Filter by test file name or test method name

    # Example for .NET:
    Bash(command="dotnet test --filter 'FullyQualifiedName~NewFeature'")

    # Example for Python:
    Bash(command="pytest tests/test_new_feature.py")

    # Example for Node.js:
    Bash(command="npm test -- --testNamePattern='NewFeature'")

IF phase == "refactor":
    # Run all tests affected by refactoring
    # Use git diff to identify changed files, run related tests

    Bash(command="git diff --name-only HEAD")
    # Parse changed files
    # Identify test files that cover changed code

    # Run affected tests
    Bash(command="[test_command] --filter [affected_tests]")

IF phase == "integration":
    # Run FULL test suite
    Bash(command="[test_command]")
```

**Parse test results:**

```
# Read test output
# Parse for failures

IF any test fails:
    BLOCK immediately
    Report: Test failures detected
    Detail:
        - Failed test name
        - Failure message
        - Stack trace (if available)
        - Expected vs actual values
    Action: Fix failing tests before continuing
    Return: BLOCKED
```

**Quick coverage check (integration phase only):**

```
IF phase == "integration":
    # Run tests with coverage
    IF language == ".NET":
        Bash(command="dotnet test --collect:'XPlat Code Coverage'")
        Read(file_path="TestResults/*/coverage.cobertura.xml")
    ELSE IF language == "Python":
        Bash(command="pytest --cov=src --cov-report=json")
        Read(file_path="coverage.json")
    ELSE IF language == "Node.js":
        Bash(command="npm test -- --coverage --coverageReporters=json")
        Read(file_path="coverage/coverage-summary.json")

    # Parse overall coverage percentage
    overall_coverage = extract_coverage_percentage(coverage_data)

    IF overall_coverage < 80%:
        BLOCK immediately
        Report: Coverage below minimum threshold
        Detail: Overall coverage {overall_coverage}%, minimum 80%
        Action: Add tests to meet minimum threshold
        Return: BLOCKED
```

---

### Step 4: Quick Anti-Pattern Scan

**CRITICAL violations only (light mode):**

#### 1. SQL Injection Patterns

```
# Search for string concatenation in SQL queries
Grep(pattern="ExecuteRawSql\\(.*\\+|string\\.Format.*SELECT|f\"SELECT.*\\{|`SELECT.*\\$\\{",
     path="src/",
     output_mode="content",
     -n=true)

IF found:
    BLOCK immediately
    Report: CRITICAL - SQL Injection vulnerability
    Detail: [File and line numbers where found]
    Fix: Use parameterized queries
    Example from coding-standards.md:
        // ❌ VULNERABLE
        var sql = $"SELECT * FROM Users WHERE Id = {userId}";

        // ✅ CORRECT
        var sql = "SELECT * FROM Users WHERE Id = @UserId";
        connection.Query<User>(sql, new { UserId = userId });
    Return: BLOCKED
```

#### 2. Hardcoded Secrets

```
# Search for hardcoded passwords, API keys, tokens
Grep(pattern="password|apikey|api_key|secret|token|connectionstring\\s*=\\s*[\"']",
     path="src/",
     -i=true,
     output_mode="content",
     -n=true)

IF found:
    # Filter false positives (variable names vs values)
    FOR each match:
        IF looks_like_hardcoded_value(match):
            BLOCK immediately
            Report: CRITICAL - Hardcoded secret detected
            Detail: {file}:{line}
            Fix: Move to environment variables or configuration
            Example:
                // ❌ WRONG
                var apiKey = "sk_live_abc123def456";

                // ✅ CORRECT
                var apiKey = configuration["ApiKey"];
            Return: BLOCKED
```

#### 3. Cross-Layer Violations

```
# Check Domain layer purity
Read(file_path="devforgeai/specs/context/architecture-constraints.md")
# Extract layer dependency rules

# Domain → Infrastructure violation
Grep(pattern="using.*Infrastructure|import.*infrastructure",
     path="src/Domain/",
     output_mode="content",
     -n=true)

IF found:
    BLOCK immediately
    Report: CRITICAL - Layer boundary violation
    Detail: Domain layer references Infrastructure
    Fix: Use abstractions/interfaces in Domain, implementations in Infrastructure
    Reference: architecture-constraints.md layer matrix
    Return: BLOCKED

# Application → Concrete Infrastructure violation
Grep(pattern="new.*Repository|new.*DbContext|new.*HttpClient",
     path="src/Application/",
     output_mode="content",
     -n=true)

IF found:
    BLOCK immediately
    Report: CRITICAL - Direct instantiation of infrastructure
    Detail: Application layer directly creates infrastructure objects
    Fix: Use dependency injection instead of 'new'
    Return: BLOCKED
```

#### 4. Library Substitution

```
Read(file_path="devforgeai/specs/context/tech-stack.md")
# Extract LOCKED technologies

# Example: If Dapper is locked, check for EF usage
IF tech_stack.ORM == "Dapper":
    Grep(pattern="using EntityFramework|using Microsoft\\.EntityFrameworkCore|from sqlalchemy",
         path="src/",
         output_mode="content",
         -n=true)

    IF found:
        BLOCK immediately
        Report: CRITICAL - Library substitution detected
        Detail: Code uses EntityFramework but tech-stack.md specifies Dapper
        Fix: Use Dapper as specified in tech-stack.md (LOCKED)
        Return: BLOCKED

# Example: If Zustand is locked, check for Redux usage
IF tech_stack.StateManagement == "Zustand":
    Grep(pattern="from '@reduxjs/toolkit'|import.*redux",
         path="src/",
         output_mode="content",
         -n=true)

    IF found:
        BLOCK immediately
        Report: CRITICAL - Library substitution detected
        Detail: Code uses Redux but tech-stack.md specifies Zustand
        Fix: Use Zustand as specified
        Return: BLOCKED
```

#### 5. File Location Violations

```
Read(file_path="devforgeai/specs/context/source-tree.md")
# Extract structure rules

# Get modified files
Bash(command="git diff --name-only HEAD")

FOR each modified_file:
    # Check file location against source-tree.md rules

    # Example: Services should be in src/Application/Services/
    IF file contains "Service" class AND NOT file.startswith("src/Application/Services/"):
        BLOCK immediately
        Report: CRITICAL - File in wrong location
        Detail: {file} should be in src/Application/Services/
        Fix: Move file to correct location per source-tree.md
        Return: BLOCKED

    # Example: Domain entities should be in src/Domain/Entities/
    IF file contains "Entity" class AND NOT file.startswith("src/Domain/Entities/"):
        BLOCK immediately
        Report: CRITICAL - File in wrong location
        Detail: {file} should be in src/Domain/Entities/
        Fix: Move file per source-tree.md
        Return: BLOCKED
```

---

### Light Validation Decision

```
# Aggregate all results
total_violations = count_violations(all_checks)

IF total_violations == 0:
    Report: ✅ Light validation PASSED
    Detail:
        - Build: SUCCESS
        - Linting: PASSED
        - Tests: {passed_count} passed
        - Coverage: {coverage}% (integration phase only)
        - Anti-patterns: None detected
    Allow: Continue development to next phase
    Return: SUCCESS

IF total_violations > 0:
    Report: ❌ Light validation FAILED
    Detail:
        - Total violations: {total_violations}
        - CRITICAL: {critical_count}
        - Build errors: {build_error_count}
        - Test failures: {test_failure_count}
        - Anti-patterns: {anti_pattern_count}
    Block: Stop development immediately
    Action: Fix all violations before continuing
    Return: BLOCKED
```

---

## Deep Validation Procedures

**Purpose**: Comprehensive quality validation after story completion

**Token Budget**: ~65,000 tokens per validation

**Use Case**: After story is complete, before release

---

### Phase 1: Test Coverage Analysis

**See:** `./coverage-analysis-guide.md` for detailed procedures

**Summary**:
1. Run full test suite with coverage
2. Parse coverage data by layer
3. Calculate percentages by layer
4. Validate against strict thresholds (95%/85%/80%)
5. Identify coverage gaps
6. Analyze test quality (assertions, mocking)
7. Validate test pyramid distribution

---

### Phase 2: Anti-Pattern Detection

**See:** `./anti-patterns-catalog.md` for complete catalog

**Summary**:
1. Load anti-patterns.md
2. Scan for 10+ categories:
   - Library substitution (CRITICAL)
   - Structure violations (HIGH)
   - Cross-layer dependencies (CRITICAL)
   - Code smells (MEDIUM/LOW)
   - Security anti-patterns (CRITICAL)
   - Performance anti-patterns (MEDIUM)
   - Maintainability issues (MEDIUM)
3. Run security scanners (Bandit, npm audit, etc.)
4. Check for dependency vulnerabilities
5. Categorize violations by severity

---

### Phase 3: Spec Compliance Validation

**See:** `./spec-compliance-validation.md` for detailed procedures

**Summary**:
1. Load story specification
2. Validate each acceptance criterion has tests
3. Verify API contracts match spec
4. Validate error handling compliance
5. Validate non-functional requirements
6. Generate traceability matrix

---

### Phase 4: Code Quality Metrics

**See:** `./quality-metrics-guide.md` for metric formulas

**Summary**:
1. Analyze cyclomatic complexity
2. Calculate maintainability index
3. Detect code duplication
4. Measure documentation coverage
5. Analyze dependency coupling
6. Generate quality score

---

### Phase 5: Generate QA Report

**Aggregate all results:**

```
qa_results = {
    "story_id": story_id,
    "timestamp": current_timestamp(),
    "overall_status": determine_status(),
    "coverage": {
        "overall": overall_coverage,
        "business_logic": business_logic_coverage,
        "application": application_coverage,
        "infrastructure": infrastructure_coverage,
        "gaps": coverage_gaps
    },
    "anti_patterns": {
        "critical": critical_violations,
        "high": high_violations,
        "medium": medium_violations,
        "low": low_violations
    },
    "spec_compliance": {
        "acceptance_criteria": criteria_results,
        "api_contracts": contract_results,
        "nfrs": nfr_results
    },
    "quality_metrics": {
        "complexity": complexity_results,
        "maintainability": maintainability_results,
        "duplication": duplication_results,
        "documentation": documentation_results
    }
}
```

**Determine overall status:**

```
IF critical_violations.count > 0:
    overall_status = "FAIL"
    blocking_reason = "Critical violations must be resolved"
ELSE IF high_violations.count > 0:
    overall_status = "FAIL"
    blocking_reason = "High severity violations must be resolved"
ELSE IF coverage_results.any_below_threshold:
    overall_status = "FAIL"
    blocking_reason = "Coverage below thresholds"
ELSE IF acceptance_criteria_failures.count > 0:
    overall_status = "FAIL"
    blocking_reason = "Acceptance criteria not met"
ELSE:
    overall_status = "PASS"
    blocking_reason = null
```

**Write QA report:**

```
# Generate comprehensive report
report = generate_qa_report(qa_results)

Write(file_path="devforgeai/qa/reports/{story_id}-qa-report.md",
      content=report)
```

**Update story status:**

```
Read(file_path="ai_docs/Stories/{story_id}.story.md")

IF overall_status == "PASS":
    Edit(file_path="ai_docs/Stories/{story_id}.story.md",
         old_string="status: In Development",
         new_string="status: QA Approved ✅")
ELSE:
    Edit(file_path="ai_docs/Stories/{story_id}.story.md",
         old_string="status: In Development",
         new_string="status: QA Failed ❌")
```

---

## Validation Mode Selection

**Decision logic:**

```
IF invoked_by == "devforgeai-development":
    mode = "light"
    phase = determine_phase_from_context()

IF invoked_manually:
    IF story_status == "Dev Complete":
        mode = "deep"
    ELSE:
        # Ask user
        AskUserQuestion:
        Question: "Which validation mode?"
        Header: "QA Mode"
        Options:
          - "Light - Fast checks for development"
          - "Deep - Comprehensive analysis for release"
        multiSelect: false

IF mode == "light":
    run_light_validation()
ELSE:
    run_deep_validation()
```

---

## Success Criteria

### Light Validation Success
- Build succeeds
- Linting passes
- Tests pass
- Coverage ≥ 80% (integration phase)
- Zero CRITICAL anti-patterns
- Token usage < 10,000

### Deep Validation Success
- Coverage meets strict thresholds
- Zero CRITICAL violations
- Zero HIGH violations (or approved)
- All acceptance criteria validated
- Quality metrics within thresholds
- Comprehensive report generated
- Story status updated
- Token usage < 65,000

---

**Reference**: Load this file when executing validation workflows in devforgeai-qa skill.
