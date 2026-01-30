# Phase 1: Test Coverage Analysis Workflow

Execute 7-step coverage analysis and validation against strict thresholds.

---

## Overview

Analyzes test coverage by layer (business logic, application, infrastructure) and validates against DevForgeAI strict thresholds: 95%/85%/80%.

## References Used

This workflow references:
- **coverage-analysis.md** - Thresholds, tools, layer analysis procedures, gap identification, test quality assessment

---

## Step 1: Load Coverage Thresholds

```
Read(file_path=".claude/skills/devforgeai-qa/assets/config/coverage-thresholds.md")

IF file not found:
    Use default strict thresholds:
    - Business Logic: 95%
    - Application: 85%
    - Infrastructure: 80%
    - Overall: 80%
```

**Reference:** `coverage-analysis.md` for threshold rationale and definitions

---

## Step 2: Generate Coverage Reports (Story-Scoped - STORY-092)

**Prerequisite:** `test_isolation_paths` variable from Phase 0.5

```
Read(file_path="devforgeai/specs/context/tech-stack.md")
# Extract language

# Get story-scoped paths from Phase 0.5
results_dir = test_isolation_paths.results_dir   # e.g., tests/results/STORY-092
coverage_dir = test_isolation_paths.coverage_dir # e.g., tests/coverage/STORY-092

# Execute language-specific coverage command with story-scoped output paths
# See: ./language-specific-tooling.md

IF language == ".NET":
    Bash(command="dotnet test --collect:'XPlat Code Coverage' --results-directory={results_dir}")
    coverage_file = "{results_dir}/*/coverage.cobertura.xml"

IF language == "Python":
    Bash(command="pytest --cov=src --cov-report=json:{coverage_dir}/coverage.json --junitxml={results_dir}/test-results.xml")
    coverage_file = "{coverage_dir}/coverage.json"

IF language == "Node.js":
    Bash(command="npm test -- --coverage --coverageDirectory={coverage_dir}")
    coverage_file = "{coverage_dir}/coverage-summary.json"

IF language == "Go":
    Bash(command="go test ./... -coverprofile={coverage_dir}/coverage.out -json > {results_dir}/test-results.json")
    coverage_file = "{coverage_dir}/coverage.out"

IF language == "Rust":
    Bash(command="cargo tarpaulin --out Json --output-dir {coverage_dir}")
    coverage_file = "{coverage_dir}/tarpaulin-report.json"

IF language == "Java":
    Bash(command="mvn test jacoco:report -Djacoco.destFile={coverage_dir}/jacoco.exec")
    coverage_file = "{coverage_dir}/jacoco.xml"

Read(file_path=coverage_file)
```

**Reference:** `language-specific-tooling.md` for complete tool commands by language
**Reference:** `test-isolation-service.md` for path resolution and directory management

---

## Step 3: Classify Files by Layer

```
Read(file_path="devforgeai/specs/context/source-tree.md")
# Extract layer definitions

# Classify files into Business Logic, Application, Infrastructure
# Based on source-tree patterns

business_logic_files = Glob matching business logic patterns
application_files = Glob matching application patterns
infrastructure_files = Glob matching infrastructure patterns
```

**Layer classification algorithm:** See `coverage-analysis.md` section "Classifying Files by Architectural Layer"

---

## Step 4: Calculate Coverage by Layer

Parse coverage data and aggregate by layer.

**Algorithm:**
```
FOR each file in coverage_report:
    layer = classify_file(file, source_tree_patterns)

    IF layer == "business_logic":
        business_logic_coverage.add(file.coverage)
    ELIF layer == "application":
        application_coverage.add(file.coverage)
    ELIF layer == "infrastructure":
        infrastructure_coverage.add(file.coverage)
    ELSE:
        unclassified.add(file)

# Calculate averages
business_logic_avg = average(business_logic_coverage)
application_avg = average(application_coverage)
infrastructure_avg = average(infrastructure_coverage)
overall_avg = average(all_coverage)
```

**Detailed procedures:** `coverage-analysis.md` section "Calculating Coverage by Layer"

---

## Step 5: Validate Against Thresholds

```
violations = []

IF business_logic_coverage < 95%:
    violations.append({
        "type": "Coverage below threshold",
        "severity": "CRITICAL",
        "layer": "Business Logic",
        "actual": business_logic_coverage,
        "threshold": "95%",
        "message": "Business logic coverage below 95%"
    })

IF application_coverage < 85%:
    violations.append({
        "type": "Coverage below threshold",
        "severity": "CRITICAL",
        "layer": "Application",
        "actual": application_coverage,
        "threshold": "85%",
        "message": "Application coverage below 85%"
    })

IF infrastructure_coverage < 80%:
    violations.append({
        "type": "Coverage below threshold",
        "severity": "HIGH",
        "layer": "Infrastructure",
        "actual": infrastructure_coverage,
        "threshold": "80%",
        "message": "Infrastructure coverage below 80%"
    })

IF overall_coverage < 80%:
    violations.append({
        "type": "Coverage below threshold",
        "severity": "CRITICAL",
        "layer": "Overall",
        "actual": overall_coverage,
        "threshold": "80%",
        "message": "Overall coverage below 80%"
    })
```

**Threshold enforcement:** CRITICAL violations block QA approval immediately

---

## Step 6: Identify Coverage Gaps

Find untested methods and generate test suggestions.

**Algorithm:**
```
# Find uncovered lines
uncovered_lines = extract_uncovered_from_report(coverage_file)

# Identify untested functions/methods
FOR each uncovered_block in uncovered_lines:
    # Analyze code to determine what's untested
    code_block = Read(file=uncovered_block.file,
                     offset=uncovered_block.start_line,
                     limit=uncovered_block.end_line - uncovered_block.start_line)

    # Extract function/method name
    function_name = extract_function_name(code_block)

    # Suggest test
    test_suggestion = {
        "file": uncovered_block.file,
        "function": function_name,
        "lines": f"{uncovered_block.start_line}-{uncovered_block.end_line}",
        "suggested_test": generate_test_name(function_name),
        "priority": calculate_priority(uncovered_block.layer)
    }

    coverage_gaps.append(test_suggestion)
```

**Detailed procedures:** `coverage-analysis.md` section "Identifying Untested Code Paths"

**Gap prioritization:**
- Business logic gaps: HIGH priority
- Application gaps: MEDIUM priority
- Infrastructure gaps: LOW priority

---

## Step 7: Analyze Test Quality

### Assertion Count

```
# Count assertions per test (target: ≥1.5)
FOR each test_file in test_files:
    assertion_count = Grep(pattern="assert|expect|should",
                          path=test_file,
                          output_mode="count")
    test_count = Grep(pattern="def test_|it\\(|test\\(",
                     path=test_file,
                     output_mode="count")

    assertion_ratio = assertion_count / test_count

    IF assertion_ratio < 1.5:
        warnings.append(MEDIUM: f"{test_file} has weak tests (avg {assertion_ratio} assertions/test)")
```

### Over-Mocking Detection

```
# Detect over-mocking (mocks > tests * 2)
FOR each test_file:
    mock_count = Grep(pattern="mock|Mock|stub|Stub|spy|Spy",
                     path=test_file,
                     output_mode="count")
    test_count = count_tests(test_file)

    IF mock_count > test_count * 2:
        warnings.append(MEDIUM: f"{test_file} has excessive mocking ({mock_count} mocks for {test_count} tests)")
```

### Test Pyramid Validation

```
# Validate test pyramid (70% unit, 20% integration, 10% E2E)
unit_tests = Glob(pattern="tests/unit/**/*").count
integration_tests = Glob(pattern="tests/integration/**/*").count
e2e_tests = Glob(pattern="tests/e2e/**/*").count

total_tests = unit_tests + integration_tests + e2e_tests

unit_percentage = (unit_tests / total_tests) * 100
integration_percentage = (integration_tests / total_tests) * 100
e2e_percentage = (e2e_tests / total_tests) * 100

IF unit_percentage < 60% OR unit_percentage > 80%:
    warnings.append(LOW: f"Test pyramid imbalanced: {unit_percentage}% unit (target 70%)")

IF integration_percentage < 10% OR integration_percentage > 30%:
    warnings.append(LOW: f"Test pyramid imbalanced: {integration_percentage}% integration (target 20%)")

IF e2e_percentage > 15%:
    warnings.append(LOW: f"Too many E2E tests: {e2e_percentage}% (target <10%)")
```

**Detailed procedures:** `coverage-analysis.md` sections:
- "Test Quality Assessment"
- "Test Pyramid Validation"
- "Assertion Adequacy Analysis"

---

## Phase 1 Output

**Results to carry forward:**
- Coverage metrics by layer
- Coverage violations (CRITICAL/HIGH)
- Coverage gaps with test suggestions
- Test quality warnings
- Test pyramid analysis

**Violations block QA approval if:**
- Any CRITICAL violation (business <95%, application <85%, overall <80%)
- HIGH violations (infrastructure <80%)

**Continue to Phase 2 (Anti-Pattern Detection) after Phase 1 completes.**
