# Phase 01: Deep Validation Workflow

**Purpose:** Execute comprehensive validation checks for QA deep mode including coverage analysis, traceability, and runtime smoke testing.

---

## Overview

Phase 1 contains validation workflows that verify implementation quality before proceeding to analysis phases.

**Steps in Phase 1:**
- Step 1.1: Coverage Analysis (existing - see references/deep-validation-workflow.md)
- Step 1.2: Traceability Validation (existing - see references/deep-validation-workflow.md)
- Step 1.3: Runtime Smoke Test (NEW - language-agnostic execution verification)

---

## Step 1.3: Runtime Smoke Test

**Purpose:** Verify that implemented deliverables actually execute at runtime. Prevents stories from being falsely marked "QA Approved" when the CLI/API fails to run.

**Framework-Agnostic Design:** This step detects the project's language from `tech-stack.md` and executes the appropriate language-specific verification command.

### Step 1.3.1: Detect Project Language

```
# Read project's tech-stack.md (the authoritative source for language detection)
Read(file_path="devforgeai/specs/context/tech-stack.md")

# Extract Backend Technology section
# Look for patterns like "Python 3.9+", "Node.js 18+", ".NET 6.0+", etc.

detected_language = null

IF content contains "Python" in Backend Technology section:
    detected_language = "python"
ELIF content contains "Node.js" OR "Node" in Backend Technology section:
    detected_language = "nodejs"
ELIF content contains ".NET" OR "C#" in Backend Technology section:
    detected_language = "dotnet"
ELIF content contains "Go " OR "Golang" in Backend Technology section:
    detected_language = "go"
ELIF content contains "Java " in Backend Technology section:
    detected_language = "java"
ELIF content contains "Rust" in Backend Technology section:
    detected_language = "rust"

Display: "Detected project language: {detected_language}"
```

**Supported Languages:**
- Python (Python 3.9+)
- Node.js (Node.js 18+)
- .NET (C# .NET 6.0+)
- Go (Go 1.20+)
- Java (Java 11+)
- Rust (Rust 1.70+)

### Step 1.3.2: Load Language Configuration

```
# Load extensible language configuration
Read(file_path=".claude/skills/devforgeai-qa/assets/language-smoke-tests.yaml")

# Get language-specific configuration
language_config = config.languages[detected_language]

IF language_config is null:
    Display: "Runtime smoke test SKIPPED: Unsupported language '{detected_language}'"
    Display: "  Supported languages: Python, Node.js, .NET, Go, Java, Rust"
    SKIP to Step 1.4
```

### Step 1.3.3: Extract Entry Point

```
# Determine entry point based on language configuration
entry_point_source = language_config.entry_point_source

# Python: Read pyproject.toml or setup.py for package name
# Node.js: Read package.json for main or bin entry
# .NET: Find .csproj file path
# Go: Find main.go path
# Java: Find pom.xml or build.gradle for artifact
# Rust: Use Cargo.toml

IF entry_point not found:
    Display: "Runtime smoke test SKIPPED: Entry point not found"
    Display: "  Expected: {entry_point_source}"
    SKIP to Step 1.4
```

### Step 1.3.4: Execute Smoke Test Command

```
# Build command from configuration
smoke_command = language_config.smoke_test_command

# Execute with 10-second timeout (prevents infinite loops)
result = Bash(
    command=smoke_command,
    timeout=10000  # 10 seconds in milliseconds
)

exit_code = result.exit_code
```

**Language-Specific Commands:**

| Language | Smoke Test Command | Entry Point Source |
|----------|-------------------|-------------------|
| Python | `python -m {package_name} --help` | pyproject.toml or setup.py |
| Node.js | `node {entry_point} --help` OR `npm start -- --help` | package.json main/bin |
| .NET | `dotnet run --project {project_path} -- --help` | *.csproj file |
| Go | `go run {main_path} --help` | main.go location |
| Java | `java -jar {artifact_path} --help` | pom.xml or build.gradle |
| Rust | `cargo run -- --help` | Cargo.toml |

### Step 1.3.5: Process Results

**Exit Code Handling:**
- Exit code == 0: SUCCESS (smoke test passed)
- Exit code != 0: FAILURE (create CRITICAL violation)

```
IF exit_code == 0:
    # SUCCESS - Runtime smoke test passed
    Display: "Runtime smoke test PASSED: {detected_language} CLI is executable"

    # Add to QA observations (no violations)
    qa_observations.append({
        step: "1.3",
        name: "Runtime Smoke Test",
        status: "PASSED",
        language: detected_language,
        command: smoke_command
    })

    # Continue to next validation step
    PROCEED to Step 1.4

ELSE:
    # FAILURE - Create CRITICAL violation
    error_message = result.stderr OR result.stdout OR "Unknown error"

    violation = {
        type: "RUNTIME_EXECUTION_FAILURE",
        severity: "CRITICAL",
        message: "{detected_language} CLI cannot be executed: {error_message}",
        file: entry_point_source,
        remediation: language_config.remediation
    }

    violations.append(violation)

    # Set overall QA status to FAILED
    overall_status = "FAILED"

    Display: "Runtime smoke test FAILED: {detected_language} CLI cannot be executed"
    Display: "  Error: {error_message}"
    Display: "  Remediation: {language_config.remediation}"

    # Include in gaps.json for remediation tracking
    gaps.append({
        type: "RUNTIME_EXECUTION_FAILURE",
        severity: "CRITICAL",
        language: detected_language,
        command: smoke_command,
        error: error_message,
        remediation: language_config.remediation
    })
```

### Step 1.3.6: Handle Edge Cases

**Missing tech-stack.md:**
```
IF tech-stack.md not found:
    Display: "Runtime smoke test SKIPPED: tech-stack.md not found"
    Display: "  Run /create-context to generate context files"
    SKIP to Step 1.4 (no error, graceful degradation)
```

**Timeout Exceeded (>10 seconds):**
```
IF command times out:
    violation = {
        type: "RUNTIME_EXECUTION_FAILURE",
        severity: "CRITICAL",
        message: "Runtime smoke test TIMEOUT: exceeded 10s limit",
        remediation: "Check for infinite loops or blocking operations in startup"
    }
    overall_status = "FAILED"
```

**Library Projects (No CLI):**
```
IF project is library (no entry point):
    Display: "Runtime smoke test N/A: Project is library/API (no CLI entry point)"
    SKIP to Step 1.4 (no error)
```

**Monorepo with Multiple Languages:**
```
IF multiple languages detected:
    FOR each language in detected_languages:
        Execute smoke test for language
        Collect results separately

    Display per-language status:
    "  Python API: PASSED"
    "  Node.js Worker: FAILED"
```

---

## Extensibility

**Adding New Language Support:**

To add support for a new language (e.g., Kotlin):

1. Add entry to `.claude/skills/devforgeai-qa/assets/language-smoke-tests.yaml`:
```yaml
kotlin:
  detection_pattern: "Kotlin"
  smoke_test_command: "kotlin {main_path} --help"
  entry_point_source: "build.gradle.kts"
  remediation: "Verify Kotlin compiler installed and main function exists"
```

2. No code changes required - configuration-driven extensibility

**Configuration Location:** `.claude/skills/devforgeai-qa/assets/language-smoke-tests.yaml`

---

## Summary

| Outcome | Status | Action |
|---------|--------|--------|
| Exit code 0 | PASSED | Continue validation |
| Exit code != 0 | CRITICAL violation | Set QA status = FAILED |
| Timeout >10s | CRITICAL violation | Set QA status = FAILED |
| Missing tech-stack.md | SKIPPED | Continue (warning only) |
| Unsupported language | SKIPPED | Continue (warning only) |
| Library project | N/A | Continue (info only) |

**Violation Type:** `RUNTIME_EXECUTION_FAILURE`
**Severity:** `CRITICAL` (blocks QA approval)
**Tracking:** Included in `gaps.json` for remediation workflow

---

## References

- **Language Configuration:** `.claude/skills/devforgeai-qa/assets/language-smoke-tests.yaml`
- **tech-stack.md:** `devforgeai/specs/context/tech-stack.md` (lines 121-188 for supported languages)
- **RCA-002:** Source recommendation for runtime smoke testing
- **STORY-266:** Implementation story for language-agnostic design
