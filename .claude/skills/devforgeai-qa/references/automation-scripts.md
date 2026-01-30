# Automation Scripts Documentation

**6 Python scripts for automated quality analysis**

---

## Overview

The QA skill includes 6 Python automation scripts to accelerate quality analysis during deep validation phases. These scripts automate repetitive analysis tasks and provide structured JSON reports.

**Scripts Location:** `.claude/skills/devforgeai-qa/scripts/`

---

## Installation

### Install Dependencies

```bash
pip install -r .claude/skills/devforgeai-qa/scripts/requirements.txt
```

### Required Dependencies

- `coverage` - Test coverage measurement
- `radon` - Cyclomatic complexity analysis
- `bandit` - Security vulnerability detection (Python)
- `pytest` - Test framework integration
- `jinja2` - Test stub template generation
- `lxml` - XML coverage report parsing
- `beautifulsoup4` - HTML report parsing

---

## Available Scripts

### 1. generate_coverage_report.py

**Phase:** Phase 1 - Test Coverage Analysis

**Purpose:** Generate comprehensive test coverage reports with line, branch, and function coverage metrics

**Usage (Story-Scoped - STORY-092):**
```bash
# Story-scoped output (recommended for concurrent QA)
python .claude/skills/devforgeai-qa/scripts/generate_coverage_report.py \
  --project-path=/path/to/project \
  --story-id=STORY-092 \
  --output=tests/coverage/{story_id}/coverage-report.json

# Legacy centralized output (deprecated)
python .claude/skills/devforgeai-qa/scripts/generate_coverage_report.py \
  --project-path=/path/to/project \
  --output=devforgeai/qa/coverage/coverage-report.json
```

**Parameters:**
- `--project-path`: Root directory of project (default: current directory)
- `--story-id`: Story identifier for story-scoped output path (e.g., STORY-092)
- `--output`: JSON output file path (default: tests/coverage/{story_id}/coverage-report.json)
- `--language`: Override language detection (python, node, dotnet, go, rust, java)

**Output Format:**
```json
{
  "timestamp": "2025-01-06T10:30:00Z",
  "overall_coverage": 87.5,
  "by_layer": {
    "business_logic": 92.3,
    "application": 85.1,
    "infrastructure": 78.9
  },
  "files": [
    {
      "file": "src/services/user_service.py",
      "layer": "application",
      "coverage": 85.1,
      "uncovered_lines": [42, 43, 87-92]
    }
  ]
}
```

**Invoked by:** Phase 1 Step 2 (Generate Coverage Reports)

---

### 2. detect_duplicates.py

**Phase:** Phase 2 - Anti-Pattern Detection (Code Duplication)

**Purpose:** Detect duplicated code blocks and calculate duplication percentage

**Usage:**
```bash
python .claude/skills/devforgeai-qa/scripts/detect_duplicates.py \
  --project-path=/path/to/project \
  --threshold=6 \
  --output=devforgeai/qa/anti-patterns/duplicates-report.json
```

**Parameters:**
- `--project-path`: Root directory of project
- `--threshold`: Minimum lines for duplication detection (default: 6)
- `--output`: JSON output file path
- `--ignore`: Patterns to ignore (e.g., tests/, migrations/)

**Output Format:**
```json
{
  "duplication_percentage": 7.2,
  "total_duplicates": 15,
  "duplicates": [
    {
      "code": "def calculate_total(items):\n    return sum(item.price for item in items)",
      "occurrences": [
        {"file": "src/services/order.py", "lines": "42-44"},
        {"file": "src/services/cart.py", "lines": "87-89"}
      ],
      "recommendation": "Extract to shared utility function"
    }
  ]
}
```

**Invoked by:** Phase 4 Step 3 (Detect Code Duplication)

---

### 3. analyze_complexity.py

**Phase:** Phase 4 - Code Quality Metrics (Cyclomatic Complexity)

**Purpose:** Calculate cyclomatic complexity and identify overly complex functions (complexity > 10)

**Usage:**
```bash
python .claude/skills/devforgeai-qa/scripts/analyze_complexity.py \
  --project-path=/path/to/project \
  --max-complexity=10 \
  --output=devforgeai/qa/anti-patterns/complexity-report.json
```

**Parameters:**
- `--project-path`: Root directory of project
- `--max-complexity`: Complexity threshold (default: 10)
- `--output`: JSON output file path

**Output Format:**
```json
{
  "average_complexity": 5.3,
  "max_complexity": 15,
  "violations": [
    {
      "function": "process_payment",
      "file": "src/services/payment.py",
      "line": 87,
      "complexity": 15,
      "threshold": 10,
      "severity": "MEDIUM",
      "recommendation": "Refactor to reduce complexity (extract helper methods)"
    }
  ]
}
```

**Invoked by:** Phase 4 Step 1 (Analyze Cyclomatic Complexity)

---

### 4. security_scan.py

**Phase:** Phase 2 - Anti-Pattern Detection (Security Vulnerabilities)

**Purpose:** Scan for common security anti-patterns (SQL injection, XSS, hardcoded secrets, etc.)

**Usage:**
```bash
python .claude/skills/devforgeai-qa/scripts/security_scan.py \
  --project-path=/path/to/project \
  --output=devforgeai/qa/anti-patterns/security-report.json
```

**Parameters:**
- `--project-path`: Root directory of project
- `--output`: JSON output file path
- `--rules`: Custom rule file (optional)

**Security Checks:**
- SQL injection (string concatenation in queries)
- XSS vulnerabilities (unescaped user input)
- Hardcoded secrets (API keys, passwords)
- Weak cryptography (MD5, SHA1)
- Insecure random (for security contexts)
- Path traversal vulnerabilities

**Output Format:**
```json
{
  "total_issues": 3,
  "critical": 1,
  "high": 1,
  "medium": 1,
  "issues": [
    {
      "type": "SQL Injection",
      "severity": "CRITICAL",
      "file": "src/repositories/user_repo.py",
      "line": 42,
      "code": "query = f\"SELECT * FROM users WHERE id = {user_id}\"",
      "recommendation": "Use parameterized queries"
    }
  ]
}
```

**Invoked by:** Phase 2 Step 3 (Run Security Scanners)

---

### 5. validate_spec_compliance.py

**Phase:** Phase 3 - Spec Compliance Validation

**Purpose:** Compare implementation against story acceptance criteria and validate API contracts

**Usage:**
```bash
python .claude/skills/devforgeai-qa/scripts/validate_spec_compliance.py \
  --story-path=devforgeai/specs/Stories/STORY-001.story.md \
  --project-path=/path/to/project \
  --output=devforgeai/qa/spec-compliance/STORY-001-compliance-report.json
```

**Parameters:**
- `--story-path`: Path to story markdown file
- `--project-path`: Root directory of project
- `--output`: JSON output file path

**Validation Checks:**
- Acceptance criteria have corresponding tests
- API endpoints from spec are implemented
- Request/response models match spec
- NFRs have validation tests

**Output Format:**
```json
{
  "overall_compliance": "PASS",
  "acceptance_criteria": {
    "total": 5,
    "passed": 5,
    "failed": 0,
    "details": [...]
  },
  "api_contracts": {
    "total": 3,
    "implemented": 3,
    "violations": []
  },
  "traceability_matrix": [...]
}
```

**Invoked by:** Phase 3 Steps 2-5 (Validate AC, API Contracts, NFRs, Traceability)

---

### 6. generate_test_stubs.py

**Phase:** Phase 5 - Test Gap Auto-Fix (optional)

**Purpose:** Generate test stub templates for untested functions/methods

**Usage:**
```bash
python .claude/skills/devforgeai-qa/scripts/generate_test_stubs.py \
  --coverage-report=devforgeai/qa/coverage/coverage-report.json \
  --output-dir=tests/generated/ \
  --framework=pytest
```

**Parameters:**
- `--coverage-report`: Coverage report JSON from generate_coverage_report.py
- `--output-dir`: Directory for generated test stubs
- `--framework`: Test framework (pytest, jest, xunit, junit)

**Output:**
- Test stub files in specified directory
- AAA pattern (Arrange, Act, Assert)
- Placeholder assertions for developer to complete

**Example Generated Stub:**
```python
# tests/generated/test_user_service.py
import pytest
from src.services.user_service import UserService

class TestUserService:
    def test_calculate_total(self):
        # Arrange
        service = UserService()
        items = []  # TODO: Add test data

        # Act
        result = service.calculate_total(items)

        # Assert
        assert result is not None  # TODO: Add specific assertion
```

**Invoked by:** Optional (when coverage gaps identified in Phase 1)

---

## Integration with Deep Validation Workflow

Scripts are automatically invoked during deep validation phases:

### Phase 1: Test Coverage Analysis (Story-Scoped - STORY-092)

```python
# Get story-scoped paths from Phase 0.5
# coverage_dir = test_isolation_paths.coverage_dir  # e.g., tests/coverage/STORY-092

# Generate coverage report (story-scoped)
Bash(command="python .claude/skills/devforgeai-qa/scripts/generate_coverage_report.py --project-path=. --story-id={story_id} --output=tests/coverage/{story_id}/coverage-report.json")

# Read and parse results
Read(file_path="tests/coverage/{story_id}/coverage-report.json")
coverage_data = parse_json(coverage_content)

# Use data in Steps 4-7
```

### Phase 2: Anti-Pattern Detection

```python
# Detect duplicates
Bash(command="python .claude/skills/devforgeai-qa/scripts/detect_duplicates.py --project-path=. --output=devforgeai/qa/anti-patterns/duplicates-report.json")

# Analyze complexity
Bash(command="python .claude/skills/devforgeai-qa/scripts/analyze_complexity.py --project-path=. --output=devforgeai/qa/anti-patterns/complexity-report.json")

# Security scan
Bash(command="python .claude/skills/devforgeai-qa/scripts/security_scan.py --project-path=. --output=devforgeai/qa/anti-patterns/security-report.json")

# Read all reports
Read duplication, complexity, security reports
Aggregate violations
```

### Phase 3: Spec Compliance

```python
# Validate spec compliance
Bash(command="python .claude/skills/devforgeai-qa/scripts/validate_spec_compliance.py --story-path=devforgeai/specs/Stories/{story_id}.story.md --output=devforgeai/qa/spec-compliance/{story_id}-compliance-report.json")

# Read and use results
Read(file_path="devforgeai/qa/spec-compliance/{story_id}-compliance-report.json")
```

---

## Manual Script Usage

Developers can run scripts manually during development:

### Quick Coverage Check

```bash
# Check coverage before committing
python .claude/skills/devforgeai-qa/scripts/generate_coverage_report.py --project-path=.
cat devforgeai/qa/coverage/coverage-report.json | jq '.overall_coverage'
```

### Security Pre-Commit Scan

```bash
# Scan for security issues before commit
python .claude/skills/devforgeai-qa/scripts/security_scan.py --project-path=.
cat devforgeai/qa/anti-patterns/security-report.json | jq '.critical'
```

### Code Quality Check

```bash
# Check complexity and duplication
python .claude/skills/devforgeai-qa/scripts/analyze_complexity.py --project-path=.
python .claude/skills/devforgeai-qa/scripts/detect_duplicates.py --project-path=.
```

---

## CI/CD Integration

Scripts can be integrated into CI/CD pipelines:

### GitHub Actions Example

```yaml
- name: Run QA Automation Scripts
  run: |
    pip install -r .claude/skills/devforgeai-qa/scripts/requirements.txt
    python .claude/skills/devforgeai-qa/scripts/generate_coverage_report.py --project-path=.
    python .claude/skills/devforgeai-qa/scripts/security_scan.py --project-path=.

- name: Check Quality Gates
  run: |
    coverage=$(jq '.overall_coverage' devforgeai/qa/coverage/coverage-report.json)
    if (( $(echo "$coverage < 80" | bc -l) )); then
      echo "Coverage below 80%: $coverage%"
      exit 1
    fi
```

---

## Script Documentation

For detailed documentation on each script:
- **Installation:** `.claude/skills/devforgeai-qa/scripts/README.md`
- **API reference:** Each script has `--help` flag
- **Examples:** `.claude/skills/devforgeai-qa/scripts/examples/`

### Get Script Help

```bash
python .claude/skills/devforgeai-qa/scripts/generate_coverage_report.py --help
python .claude/skills/devforgeai-qa/scripts/detect_duplicates.py --help
python .claude/skills/devforgeai-qa/scripts/analyze_complexity.py --help
python .claude/skills/devforgeai-qa/scripts/security_scan.py --help
python .claude/skills/devforgeai-qa/scripts/validate_spec_compliance.py --help
python .claude/skills/devforgeai-qa/scripts/generate_test_stubs.py --help
```

---

## Output Locations

**Coverage Reports:**
- `devforgeai/qa/coverage/coverage-report.json`

**Anti-Pattern Reports:**
- `devforgeai/qa/anti-patterns/duplicates-report.json`
- `devforgeai/qa/anti-patterns/complexity-report.json`
- `devforgeai/qa/anti-patterns/security-report.json`

**Spec Compliance:**
- `devforgeai/qa/spec-compliance/{STORY-ID}-compliance-report.json`

**Generated Tests:**
- `tests/generated/` (or custom output directory)

---

## Framework Integration

### Automatic Invocation

Scripts are invoked automatically by devforgeai-qa skill during deep validation:
- Phase 1 → generate_coverage_report.py
- Phase 2 → detect_duplicates.py, analyze_complexity.py, security_scan.py
- Phase 3 → validate_spec_compliance.py
- Phase 5 (optional) → generate_test_stubs.py

### Manual Invocation

Developers can run scripts independently:
- Pre-commit validation
- Local quality checks
- Development feedback loop
- Continuous improvement

---

## Quality Thresholds (Enforced by Scripts)

### Coverage Thresholds

- **Business Logic:** 95% minimum
- **Application Layer:** 85% minimum
- **Infrastructure Layer:** 80% minimum
- **Overall:** 80% minimum

### Code Quality Thresholds

- **Cyclomatic Complexity:** ≤10 per function/method
- **Code Duplication:** <5% of codebase
- **Maintainability Index:** ≥70
- **Documentation Coverage:** ≥80% for public APIs

### Security Thresholds

- **Critical Vulnerabilities:** ZERO allowed
- **High Vulnerabilities:** ZERO allowed (or documented exceptions)
- **Dependency Vulnerabilities:** All must be patched or have mitigation plan

---

## Troubleshooting

### Script Not Found

```bash
# Verify script exists
ls -la .claude/skills/devforgeai-qa/scripts/

# Verify Python installed
python3 --version

# Verify dependencies installed
pip list | grep coverage
```

### Permission Errors

```bash
# Make scripts executable
chmod +x .claude/skills/devforgeai-qa/scripts/*.py

# Or run with python explicitly
python .claude/skills/devforgeai-qa/scripts/generate_coverage_report.py
```

### Missing Dependencies

```bash
# Reinstall dependencies
pip install -r .claude/skills/devforgeai-qa/scripts/requirements.txt

# Or install individually
pip install coverage radon bandit pytest jinja2
```

### Language Not Detected

```bash
# Manually specify language
python .claude/skills/devforgeai-qa/scripts/generate_coverage_report.py \
  --project-path=. \
  --language=python
```

---

## Best Practices

### Use in Development Loop

Run scripts after each development session:
```bash
# Quick quality check
python .claude/skills/devforgeai-qa/scripts/generate_coverage_report.py --project-path=.
python .claude/skills/devforgeai-qa/scripts/security_scan.py --project-path=.
```

### Integrate with Git Hooks

Add to `.git/hooks/pre-commit`:
```bash
#!/bin/bash
python .claude/skills/devforgeai-qa/scripts/security_scan.py --project-path=.
if [ $? -ne 0 ]; then
  echo "Security scan failed - commit blocked"
  exit 1
fi
```

### Generate Test Stubs for Coverage Gaps

After coverage analysis:
```bash
# Identify gaps
python .claude/skills/devforgeai-qa/scripts/generate_coverage_report.py --project-path=.

# Generate stubs for gaps
python .claude/skills/devforgeai-qa/scripts/generate_test_stubs.py \
  --coverage-report=devforgeai/qa/coverage/coverage-report.json \
  --output-dir=tests/generated/ \
  --framework=pytest
```

---

## Script Limitations

**Scripts provide automation, not intelligence:**
- Scripts detect patterns, not intent
- Human review still required
- False positives possible
- Context-specific rules may need customization

**Scripts complement, not replace, manual review:**
- Use scripts for consistency
- Use human judgment for nuance
- Combine both for best results

---

**For complete script documentation, see:** `.claude/skills/devforgeai-qa/scripts/README.md`
