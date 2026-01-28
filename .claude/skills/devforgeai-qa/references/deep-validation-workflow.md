# Deep Validation Workflow (Consolidated)

Single-load reference for deep QA validation. Contains all workflow steps for Phases 1-3.

---

## Overview

This file consolidates the following workflows for single-load efficiency:
- 1.1 Coverage Analysis (7 steps)
- 1.2 Traceability Validation (5 steps)
- 1.3 Traceability Enhancement (placeholder)
- 1.4 Runtime Smoke Test (6 steps) - Comprehensive (STORY-266, STORY-267)
- 1.5 Documentation Accuracy Validation (4 steps)
- 2.1 Anti-Pattern Detection (6 steps)
- 2.3 Spec Compliance (6 steps)
- 2.4 Code Quality Metrics (5 steps)
- 3.x Report Generation (6 steps)

**Token savings:** ~3.5K tokens (load once vs 7 separate files at ~500 each)

---

## Phase 1: Validation Workflows

### 1.1 Coverage Analysis (7 Steps)

**Step 1: Load Thresholds**
```
Thresholds (strict defaults):
- Business Logic: 95%
- Application: 85%
- Infrastructure: 80%
- Overall: 80%

Optional: Read(file_path=".claude/skills/devforgeai-qa/assets/config/coverage-thresholds.md")
```

**Step 2: Generate Coverage Reports**
```
Language-specific commands (use test_isolation_paths from Phase 0):

.NET:    dotnet test --collect:'XPlat Code Coverage' --results-directory={results_dir}
Python:  pytest --cov=src --cov-report=json:{coverage_dir}/coverage.json
Node.js: npm test -- --coverage --coverageDirectory={coverage_dir}
Go:      go test ./... -coverprofile={coverage_dir}/coverage.out
Rust:    cargo tarpaulin --out Json --output-dir {coverage_dir}
Java:    mvn test jacoco:report -Djacoco.destFile={coverage_dir}/jacoco.exec
```

**Step 3: Classify Files by Layer**
```
Read: devforgeai/specs/context/source-tree.md

Layer patterns (from source-tree):
- Business Logic: src/domain/*, src/core/*, src/services/*
- Application: src/api/*, src/controllers/*, src/handlers/*
- Infrastructure: src/data/*, src/repositories/*, src/external/*
```

**Step 4: Calculate Coverage by Layer**
```
FOR each file in coverage_report:
    layer = classify_file(file, source_tree_patterns)
    layer_coverage[layer].add(file.coverage)

Calculate: business_avg, application_avg, infrastructure_avg, overall_avg
```

**Step 5: Validate Against Thresholds**
```
IF business_coverage < 95%: CRITICAL violation
IF application_coverage < 85%: CRITICAL violation
IF infrastructure_coverage < 80%: HIGH violation
IF overall_coverage < 80%: CRITICAL violation
```

**Step 6: Identify Coverage Gaps**
```
FOR each uncovered_block:
    test_suggestion = {
        file, function, lines,
        suggested_test: generate_test_name(),
        priority: HIGH (business) | MEDIUM (app) | LOW (infra)
    }
```

**Step 7: Analyze Test Quality**
```
Checks:
- Assertion ratio (target: ≥1.5 per test)
- Over-mocking (mocks > tests * 2)
- Test pyramid (70% unit, 20% integration, 10% E2E)
```

---

### 1.2 Traceability Validation (5 Steps)

**Step 1: Extract AC Requirements**
```
ac_headers = grep "^### AC#[0-9]+" story_file
Extract: Then/And clauses, bullet requirements, metrics
Store: ac_requirements[]
```

**Step 2: Extract DoD Items**
```
dod_section = extract_between("^## Definition of Done", "^## Workflow")
Parse: checkbox lines "^- \[(x| )\] (.+)$"
Store: dod_items[] with section, status, text
```

**Step 3: Map AC → DoD**
```
FOR each ac_req:
    best_match = find_dod_match(ac_keywords, dod_items)
    IF match_score >= 0.5: mapped
    ELSE: missing_traceability
```

**Step 4: Calculate Score**
```
traceability_score = (covered / total) × 100
IF < 100: HALT workflow
```

**Step 5: Validate Deferrals**
```
IF dod_unchecked > 0:
    Check "## Approved Deferrals" section
    Match unchecked items to approved list
    IF unmatched: deferral_status = "INVALID"
```

---

### 1.3 Traceability Enhancement (Placeholder)

**Note:** This section is a placeholder to maintain sequential numbering. Traceability validation details are covered in Section 1.2.

---

### 1.4 Runtime Smoke Test (6 Steps) - STORY-266

**Purpose:** Verify that implemented deliverables actually execute at runtime. Prevents stories from being falsely marked "QA Approved" when the CLI/API fails to run.

**Framework-Agnostic:** Detects language from `tech-stack.md` and executes appropriate smoke test. Supports ALL 6 backend languages per tech-stack.md lines 127-134.

**Source:** RCA-002 REC-4 (Runtime validation gaps)

**Cross-References:**
- Implementation: STORY-266 (Language-Agnostic Runtime Smoke Test)
- Configuration: `.claude/skills/devforgeai-qa/assets/language-smoke-tests.yaml`
- Authority: `devforgeai/specs/context/tech-stack.md` (lines 7-11, 121-188)

---

#### Step 1: Detect Language from tech-stack.md

**Detection Priority** (authoritative source first):
1. **tech-stack.md** (authoritative): Read project's `devforgeai/specs/context/tech-stack.md`
2. **File system fallback**: If no tech-stack.md, detect from project files

```
Read: devforgeai/specs/context/tech-stack.md

Extract: Backend technology from "Backend Technology Options" section
Pattern: grep "C#|Python|Node.js|Java|Go|Rust"

IF NOT found in tech-stack.md:
    Fallback detection via file patterns (see Step 1b)
```

**Fallback Detection Patterns** (Step 1b):

| Language | Detection File | Secondary Pattern |
|----------|---------------|-------------------|
| Python | `pyproject.toml` OR `setup.py` | `requirements.txt` |
| Node.js | `package.json` | `node_modules/` |
| .NET | `*.csproj` OR `*.sln` | `bin/Debug/` |
| Go | `go.mod` | `main.go` |
| Java | `pom.xml` OR `build.gradle` | `src/main/java/` |
| Rust | `Cargo.toml` | `src/main.rs` |

---

#### Step 2: Determine Project Type

**Project Type Classification** (determines smoke test behavior):

CLI project type detection identifies command-line interface applications that can be executed directly. API project type detection identifies web services that require a running server. Library project type detection identifies packages that provide functionality to other code but cannot be executed standalone.

| Project Type | Detection Criteria | Smoke Test Action |
|--------------|--------------------|--------------------|
| **CLI** | Entry point script, `[scripts]` in config, `main()` function | Execute `--help` or `--version` |
| **API** | HTTP server dependency, `uvicorn`/`gunicorn`/`kestrel` | SKIP (API requires running server) |
| **Library** | No entry point, `[library]` in config, no `main()` | SKIP (libraries not directly executable) |

**Detection Decision Tree:**

```
IF pyproject.toml contains [tool.poetry.scripts]:
    TYPE = CLI
ELIF package.json contains "bin":
    TYPE = CLI
ELIF *.csproj OutputType is "Exe":
    TYPE = CLI
ELIF go.mod exists AND main.go exists:
    TYPE = CLI
ELIF pom.xml packaging is "jar" with mainClass:
    TYPE = CLI
ELIF Cargo.toml contains [[bin]]:
    TYPE = CLI
ELIF http server dependency detected:
    TYPE = API
    ACTION = SKIP with reason "API projects require running server"
ELSE:
    TYPE = Library
    ACTION = SKIP with reason "Library projects not directly executable"
```

---

#### Step 3: Load Language Configuration

**Configuration File:** `.claude/skills/devforgeai-qa/assets/language-smoke-tests.yaml`

```yaml
# Language Smoke Test Configuration
languages:
  python:
    detection_pattern: "Python 3.9+"
    entry_point_source: "pyproject.toml [tool.poetry.scripts] OR setup.py entry_points"
    smoke_test_command: "{package_name} --help"
    timeout_seconds: 10
    remediation_guidance: |
      1. Verify pyproject.toml has [tool.poetry.scripts] section
      2. Ensure package installed: pip install -e .
      3. Check entry point function exists and is importable

  nodejs:
    detection_pattern: "Node.js 18+"
    entry_point_source: "package.json 'bin' field OR 'main' field"
    smoke_test_command: "node {entry_point} --help"
    timeout_seconds: 10
    remediation_guidance: |
      1. Verify package.json has 'bin' or 'main' field
      2. Ensure dependencies installed: npm install
      3. Check entry point file exists and has shebang

  dotnet:
    detection_pattern: ".NET 6.0+"
    entry_point_source: "*.csproj AssemblyName OR project directory name"
    smoke_test_command: "dotnet run --project {project_path} -- --help"
    alt_command: "dotnet {assembly_path}.dll --help"
    timeout_seconds: 10
    remediation_guidance: |
      1. Verify *.csproj OutputType is 'Exe' (not 'Library')
      2. Build project: dotnet build
      3. Check Program.cs exists with Main() method

  go:
    detection_pattern: "Go 1.20+"
    entry_point_source: "go.mod module name + main.go"
    smoke_test_command: "go run . --help"
    alt_command: "./{binary_name} --help"
    timeout_seconds: 10
    remediation_guidance: |
      1. Verify go.mod exists in project root
      2. Ensure main.go has package main and func main()
      3. Build: go build -o {binary_name}

  java:
    detection_pattern: "Java 11+"
    entry_point_source: "pom.xml mainClass OR MANIFEST.MF Main-Class"
    smoke_test_command: "mvn exec:java -Dexec.args='--help'"
    alt_command: "java -jar {artifact_path}.jar --help"
    timeout_seconds: 15
    remediation_guidance: |
      1. Verify pom.xml has exec-maven-plugin with mainClass
      2. Build: mvn package
      3. Check main class has public static void main(String[] args)

  rust:
    detection_pattern: "Rust 1.70+"
    entry_point_source: "Cargo.toml [[bin]] name OR package name"
    smoke_test_command: "cargo run -- --help"
    alt_command: "./target/release/{binary_name} --help"
    timeout_seconds: 10
    remediation_guidance: |
      1. Verify Cargo.toml has [[bin]] section or src/main.rs exists
      2. Build: cargo build --release
      3. Check main.rs has fn main()
```

---

#### Step 4: Execute Smoke Test

**Execution Protocol:**

```
timeout = language_config.timeout_seconds  # Default: 10s

IF project_type == "CLI":
    command = substitute_placeholders(language_config.smoke_test_command)

    Bash(command="{command}", timeout={timeout})

    IF exit_code == 0:
        result = "PASSED"
    ELIF timeout_exceeded:
        result = "TIMEOUT"
        violation_type = "RUNTIME_EXECUTION_TIMEOUT"
    ELSE:
        result = "FAILED"
        violation_type = "RUNTIME_EXECUTION_FAILURE"

ELIF project_type IN ["API", "Library"]:
    result = "SKIPPED"
    skip_reason = language_config.skip_reason
```

**Language-Specific Commands Matrix:**

| Language | Primary Command | Alternative Command | Entry Point Source |
|----------|----------------|---------------------|-------------------|
| Python | `{package_name} --help` | `python -m {module_name} --help` | pyproject.toml `[tool.poetry.scripts]` |
| Node.js | `node {entry_point} --help` | `npx {package_name} --help` | package.json `bin` field |
| .NET | `dotnet run --project {path} -- --help` | `dotnet {assembly}.dll --help` | *.csproj `AssemblyName` |
| Go | `go run . --help` | `./{binary_name} --help` | go.mod `module` + main.go |
| Java | `mvn exec:java -Dexec.args='--help'` | `java -jar {artifact}.jar --help` | pom.xml `mainClass` |
| Rust | `cargo run -- --help` | `./target/release/{binary} --help` | Cargo.toml `[[bin]]` |

---

#### Step 5: Process Results and Generate Violations

**Success Output Format:**
```
Runtime smoke test PASSED: {language} CLI is executable
  Command: {executed_command}
  Exit code: 0
  Duration: {duration}ms
```

**Failure Output Format (CRITICAL Violation):**
```
Runtime smoke test FAILED: {language} CLI execution error
  Command: {executed_command}
  Exit code: {exit_code}
  Error: {stderr_output}
  Duration: {duration}ms

CRITICAL: RUNTIME_EXECUTION_FAILURE - Implementation does not execute
```

**Timeout Output Format:**
```
Runtime smoke test TIMEOUT: {language} CLI exceeded {timeout}s
  Command: {executed_command}
  Timeout: {timeout}s exceeded

CRITICAL: RUNTIME_EXECUTION_TIMEOUT - Implementation unresponsive
```

**Skip Output Format:**
```
Runtime smoke test SKIPPED: {reason}
  Project type: {project_type}
  Language: {language}

INFO: Smoke test not applicable for {project_type} projects
```

**JSON Violation Format (for gaps.json):**

```json
{
  "violations": [
    {
      "id": "RUNTIME-001",
      "severity": "CRITICAL",
      "type": "RUNTIME_EXECUTION_FAILURE",
      "category": "runtime_validation",
      "file": "{entry_point_file}",
      "line": null,
      "message": "CLI execution failed with exit code {exit_code}",
      "details": {
        "language": "{language}",
        "command": "{executed_command}",
        "exit_code": "{exit_code}",
        "stderr": "{stderr_output}",
        "duration_ms": "{duration}"
      },
      "remediation": "{language_specific_remediation}",
      "blocking": true,
      "classification": "REGRESSION"
    }
  ]
}
```

---

#### Step 6: Handle Edge Cases

**Edge Case Matrix:**

| Scenario | Behavior | Output |
|----------|----------|--------|
| No tech-stack.md | Fallback to file detection | Warning + continue |
| Multiple languages (monorepo) | Test each sequentially | Per-language status |
| Missing entry point config | Language-specific guidance | CRITICAL + remediation |
| Build not run | Attempt build first | Warning if fails |
| Unsupported language | Skip with notice | INFO: Language not in supported list |
| Permission denied | Report permissions issue | CRITICAL + chmod guidance |
| Missing dependencies | Report dependency error | CRITICAL + install guidance |

**Unsupported Language Handling:**

```
IF language NOT IN ["python", "nodejs", "dotnet", "go", "java", "rust"]:
    Display: "Runtime smoke test SKIPPED: {language} not in supported list"
    Display: "Supported languages: Python, Node.js, .NET, Go, Java, Rust"
    Display: "To add support, see Extensibility section below"
    result = "SKIPPED"
    skip_reason = "unsupported_language"
```

---

#### Extensibility Pattern (Adding New Languages)

**Configuration-Only Extension:** No code modification required. Add entry to `language-smoke-tests.yaml`.

**Required Fields per Language Entry:**

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `detection_pattern` | string | Language version pattern | "Kotlin 1.8+" |
| `entry_point_source` | string | Where to find entry point | "build.gradle.kts" |
| `smoke_test_command` | string | Command with placeholders | `kotlinc {source} -include-runtime -d {output}.jar && java -jar {output}.jar --help` |
| `timeout_seconds` | int | Execution timeout | 15 |
| `remediation_guidance` | string | Multi-line fix instructions | "1. Ensure build.gradle.kts..." |

**Example: Adding Kotlin Support:**

```yaml
# Append to .claude/skills/devforgeai-qa/assets/language-smoke-tests.yaml

  kotlin:
    detection_pattern: "Kotlin 1.8+"
    entry_point_source: "build.gradle.kts mainClass OR src/main/kotlin/Main.kt"
    smoke_test_command: "gradle run --args='--help'"
    alt_command: "java -jar build/libs/{artifact}.jar --help"
    timeout_seconds: 15
    remediation_guidance: |
      1. Verify build.gradle.kts has application plugin and mainClass
      2. Build: gradle build
      3. Check Main.kt has fun main(args: Array<String>)
```

**Verification Steps for New Language:**

1. Add configuration entry to `language-smoke-tests.yaml`
2. Create test project with entry point
3. Run `/qa {STORY-ID} --mode=deep` on project
4. Verify smoke test executes correct command
5. Verify success/failure detection works
6. Update this documentation with new language in matrices

---

### 1.5 Documentation Accuracy Validation (4 Steps)

**Purpose:** Validate that SKILL.md file claims match actual filesystem state.

**Step 1: Extract Documentation Claims**
```
FOR each SKILL.md in .claude/skills/:
    file_count_claims = grep "Total: (\d+) reference files"
    line_count_claims = grep "(\d+) lines"
    section_claims = grep "(\d+) (sections|phases)"

    Store: {skill_name, claim_type, claimed_count}
```

**Step 2: Count Actual Resources**
```
FOR each skill with claims:
    actual_file_count = Glob(pattern=".claude/skills/{skill}/references/*.md").count()
    actual_line_count = sum(Read(file).line_count for file in references/)
    actual_section_count = grep "^## " SKILL.md | count
```

**Step 3: Compare and Generate Violations**
```
FOR each claim in claims:
    IF claimed_count != actual_count:
        violations.append({
            severity: "MEDIUM",
            type: "documentation_drift",
            file: skill_path,
            message: "Claims {claimed_count} files, found {actual_count}",
            remediation: "Update SKILL.md count or add/remove files"
        })
```

**Step 4: Aggregate Results**
```
doc_accuracy_result = {
    claims_validated: total_claims,
    discrepancies: violations.length,
    details: violations
}
```

**Claim Types Validated:**

| Claim Type | Pattern | Validation Method |
|------------|---------|-------------------|
| file_count | "Total: N reference files" | Glob count |
| line_count | "N lines" | Sum of file lines |
| section_count | "N sections/phases" | Grep header count |

---

## Phase 2: Analysis Workflows

### 2.1 Anti-Pattern Detection (6 Steps)

**Step 1: Load ALL 6 Context Files**
```
Read: tech-stack.md, source-tree.md, dependencies.md,
      coding-standards.md, architecture-constraints.md, anti-patterns.md

IF ANY missing: HALT "Run /create-context"
```

**Step 2: Invoke anti-pattern-scanner Subagent**
```
Task(subagent_type="anti-pattern-scanner",
     prompt="Scan {changed_files} with 6 context files loaded")
```

**Step 3: Parse JSON Response**
```
violations_critical = result["violations"]["critical"]
violations_high = result["violations"]["high"]
violations_medium = result["violations"]["medium"]
violations_low = result["violations"]["low"]
```

**Step 4: Update blocks_qa State (OR Logic)**
```
blocks_qa = blocks_qa OR result["blocks_qa"]
```

**Step 5: Display Summary**
```
CRITICAL: blocks immediately
HIGH: blocks QA approval
MEDIUM: warning only
LOW: advisory only
```

**Step 6: Store for Report**
```
qa_report_data["anti_pattern_violations"] = {...}
```

---

### 2.1.5 Regression vs Pre-existing Classification (STORY-175)

**Purpose:** Classify violations as REGRESSION or PRE_EXISTING based on changed files.

**Step 1: Identify Changed Files**
```
changed_files = git diff --name-only HEAD~1

Edge cases:
- First commit: Use `git diff --name-only origin/main...HEAD`
- No git repo: Fallback to all REGRESSION (blocking)
```

**Step 2: Classify Each Violation**
```
FOR each violation:
    IF violation.file IN changed_files:
        classification = "REGRESSION"
    ELSE:
        classification = "PRE_EXISTING"
```

**Step 3: Set Blocking Status**
```
REGRESSION violations:
    blocking = true  (blocks QA approval)

PRE_EXISTING violations:
    blocking = false (warning only, does not block)
```

**Step 4: Display Breakdown**
```
Format: "Regressions: {count} | Pre-existing: {count}"

Example output:
Regressions: 3 | Pre-existing: 7
Blocking: 3 | Warnings: 7
```

**Implementation Module:** `devforgeai/qa/regression_classifier.py`

**Key Functions:**
- `get_changed_files()` - Execute git diff
- `classify_violations()` - Classify batch of violations
- `set_all_blocking_status()` - Set blocking for all
- `should_block_qa()` - Check if any blocking violations
- `get_breakdown()` - Format display string

**Edge Case Handling:**
| Scenario | Behavior |
|----------|----------|
| No git repository | All REGRESSION (blocking) |
| First commit | Use `origin/main...HEAD` |
| Empty changed files | All PRE_EXISTING (non-blocking) |
| Permission/Timeout errors | Return empty list safely |

---

### 2.2 Parallel Validation (3 Validators)

**Execute in SINGLE message:**
```
Task(subagent_type="test-automator", prompt="Coverage analysis...")
Task(subagent_type="code-reviewer", prompt="Code quality review...")
Task(subagent_type="security-auditor", prompt="Security scan...")
```

**Success Threshold:** 66% (2 of 3 must pass)

**Aggregate Results:**
```
success_count = sum(1 for r in results if r.passed)
IF success_count < 2: HALT
```

---

### 2.3 Spec Compliance (6 Steps)

**Step 0: Validate Story Documentation**
```
Required sections:
- Implementation Notes
- Definition of Done Status
- Test Results
- Acceptance Criteria Verification

IF missing: HALT
```

**Step 1: Load Story Specification**
```
Read: story file
Extract: AC list, API contracts, NFRs
```

**Step 2: Validate Acceptance Criteria**
```
FOR each AC:
    test_exists = Grep(pattern="test_{ac_id}")
    IF NOT test_exists: violation
    IF test.status != PASSED: violation
```

**Step 3: Validate Deferrals (MANDATORY if exist)**
```
IF any DoD item unchecked:
    Task(subagent_type="deferral-validator", ...)
    Validate: user approval, story/ADR references
```

**Step 4: Validate API Contracts**
```
FOR each endpoint in spec:
    Verify: implementation matches spec (method, path, params, response)
```

**Step 5: Validate NFRs**
```
Check: performance, security, accessibility requirements
```

**Step 6: Generate Traceability Matrix**
```
Matrix: Requirement → Test → Implementation
```

---

### 2.4 Code Quality Metrics (5 Steps)

**Step 1: Cyclomatic Complexity**
```
Tools: radon (Python), complexity-report (JS), metrics (Java)
Threshold: >10 = MEDIUM violation

Bash(command="radon cc src/ -a -nc")
```

**Step 2: Maintainability Index**
```
MI < 70: MEDIUM violation
MI < 50: HIGH violation (blocks QA)

Bash(command="radon mi src/ -s")
```

**Step 3: Code Duplication**
```
Tool: jscpd
Threshold: >5% = MEDIUM, >20% = HIGH (blocks)

Bash(command="npx jscpd --reporters consoleFull ./src")
```

**Step 4: Documentation Coverage**
```
Target: 80%
Count: documented vs undocumented public APIs
```

**Step 5: Dependency Coupling**
```
Detect: circular dependencies, high coupling (>10 deps/file)
```

---

## Phase 3: Reporting Workflows

### 3.1 Determine Overall Result

```
IF any CRITICAL OR coverage < thresholds OR parallel < 66%:
    overall_status = "FAILED"
ELIF any HIGH:
    overall_status = "PASS WITH WARNINGS"
ELSE:
    overall_status = "PASSED"
```

### 3.2 Generate QA Report (Deep Mode Only)

```
Write(file_path="devforgeai/qa/reports/{STORY-ID}-qa-report.md")

Sections:
- Executive Summary
- Coverage Analysis
- Violation Details
- Traceability Matrix
- Recommendations
```

### 3.3 Generate gaps.json (FAILED Only)

**MANDATORY if FAILED:**
```json
{
  "story_id": "STORY-XXX",
  "qa_result": "FAILED",
  "coverage_gaps": [...],
  "anti_pattern_violations": [...],
  "deferral_issues": [...],
  "remediation_sequence": [
    {"phase": "02R", "name": "Fix tests", "target_files": [...], "gap_count": X}
  ]
}
```

**Verify creation:**
```
Glob(pattern="devforgeai/qa/reports/{STORY-ID}-gaps.json")
IF NOT found: HALT
```

### 3.4 Update Story Status

```
IF PASSED/PASS WITH WARNINGS:
    Edit: "status: Dev Complete" → "status: QA Approved ✅"

IF FAILED:
    Edit: "status: Dev Complete" → "status: QA Failed ❌"

Append workflow history entry
```

### 3.5 Format Display

```
Task(subagent_type="qa-result-interpreter", prompt="Format results")
```

### 3.6 Invoke Feedback Hooks

```
Bash: devforgeai-validate check-hooks --operation=qa --status={status}
IF exit 0: devforgeai-validate invoke-hooks --operation=qa --story={STORY_ID}
```

---

## Quick Reference: Blocking Violations

| Phase | Check | Severity | Blocks QA |
|-------|-------|----------|-----------|
| 1 | Business coverage <95% | CRITICAL | Yes |
| 1 | Application coverage <85% | CRITICAL | Yes |
| 1 | Infrastructure coverage <80% | HIGH | Yes |
| 1 | Traceability <100% | CRITICAL | Yes |
| 1 | Runtime smoke test failure | CRITICAL | Yes |
| 1 | Runtime smoke test timeout | CRITICAL | Yes |
| 2 | Security vulnerabilities | CRITICAL | Yes |
| 2 | Library substitution | CRITICAL | Yes |
| 2 | Structure violations | HIGH | Yes |
| 2 | Parallel validators <66% | HIGH | Yes |
| 2 | Duplication >20% | HIGH | Yes |
| 2 | MI <50 | HIGH | Yes |
| 2 | Undocumented deferrals | HIGH | Yes |

---

## Subagent Summary

| Subagent | Phase | Invocation |
|----------|-------|------------|
| anti-pattern-scanner | 2.1 | Single Task() |
| test-automator | 2.2 | Parallel with 2 others |
| code-reviewer | 2.2 | Parallel with 2 others |
| security-auditor | 2.2 | Parallel with 2 others |
| deferral-validator | 2.3 | Conditional (if deferrals) |
| qa-result-interpreter | 3.5 | Display formatting |

---

## Phase Marker Protocol [STORY-126 Enhancement]

**Purpose:** Write marker files after each phase to enable sequential verification and resume capability.

### Marker Write Template (End of Each Phase)

```
Write(file_path="devforgeai/qa/reports/{STORY_ID}/.qa-phase-{N}.marker",
      content="phase: {N}\nstory_id: {STORY_ID}\nmode: {MODE}\ntimestamp: {TIMESTAMP}\nstatus: complete")
```

### Pre-Flight Verification Template (Start of Phases 1-4)

```
Glob(pattern="devforgeai/qa/reports/{STORY_ID}/.qa-phase-{N-1}.marker")
IF NOT found: HALT: "Phase {N-1} not completed - run in sequence"
```

### Marker Locations

| Phase | Marker File |
|-------|-------------|
| 0 | `.qa-phase-0.marker` |
| 1 | `.qa-phase-1.marker` |
| 2 | `.qa-phase-2.marker` |
| 3 | `.qa-phase-3.marker` |
| 4 | `.qa-phase-4.marker` |

**Location:** `devforgeai/qa/reports/{STORY_ID}/`

---

**Token efficiency:** ~2.5K tokens (single load) vs ~5K+ (5 separate loads)
