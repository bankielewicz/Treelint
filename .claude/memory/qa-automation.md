# DevForgeAI QA Automation Scripts

Automated quality analysis tools for the DevForgeAI framework.

---

## Overview

The QA skill includes 6 Python scripts for automated quality analysis in `.claude/skills/devforgeai-qa/scripts/`.

---

## Installation

```bash
pip install -r .claude/skills/devforgeai-qa/scripts/requirements.txt
```

---

## Usage Examples

### 1. Generate Coverage Report

```bash
python .claude/skills/devforgeai-qa/scripts/generate_coverage_report.py \
  --project-path=. \
  --output=devforgeai/qa/coverage/coverage-report.json
```

**Purpose:** Analyze test coverage across all source files
**Output:** JSON report with line-by-line coverage data
**Thresholds:** 95% business logic, 85% application, 80% infrastructure

---

### 2. Detect Code Duplication

```bash
python .claude/skills/devforgeai-qa/scripts/detect_duplicates.py \
  --project-path=. \
  --threshold=6 \
  --output=devforgeai/qa/anti-patterns/duplicates-report.json
```

**Purpose:** Find duplicate code blocks
**Threshold:** 6+ lines of duplication triggers violation
**Output:** JSON report with duplicate locations and recommendations

---

### 3. Analyze Cyclomatic Complexity

```bash
python .claude/skills/devforgeai-qa/scripts/analyze_complexity.py \
  --project-path=. \
  --max-complexity=10 \
  --output=devforgeai/qa/anti-patterns/complexity-report.json
```

**Purpose:** Identify overly complex functions/methods
**Threshold:** Complexity > 10 triggers refactoring requirement
**Output:** JSON report with complexity scores per function

---

### 4. Security Scan

```bash
python .claude/skills/devforgeai-qa/scripts/security_scan.py \
  --project-path=. \
  --output=devforgeai/qa/anti-patterns/security-report.json
```

**Purpose:** Scan for security vulnerabilities (OWASP Top 10)
**Checks:**
- Hardcoded secrets/credentials
- SQL injection vulnerabilities
- XSS vulnerabilities
- Weak cryptography (MD5, SHA1)
- Insecure dependencies

**Output:** JSON report with vulnerability locations and severity

---

### 5. Validate Spec Compliance

```bash
python .claude/skills/devforgeai-qa/scripts/validate_spec_compliance.py \
  --story-path=devforgeai/specs/Stories/STORY-001.story.md \
  --project-path=. \
  --output=devforgeai/qa/spec-compliance/STORY-001-compliance-report.json
```

**Purpose:** Verify implementation matches acceptance criteria
**Validation:**
- All acceptance criteria have corresponding tests
- API contracts implemented as specified
- Non-functional requirements met

**Output:** JSON report with compliance status per criterion

---

### 6. Generate Test Stubs

```bash
python .claude/skills/devforgeai-qa/scripts/generate_test_stubs.py \
  --coverage-report=devforgeai/qa/coverage/coverage-report.json \
  --output-dir=tests/generated/ \
  --framework=pytest
```

**Purpose:** Auto-generate test stubs for uncovered code
**Supported Frameworks:** pytest, jest, xunit
**Output:** Test stub files in specified directory

---

## Integration with DevForgeAI Workflow

### Automatic Invocation

QA automation scripts are invoked automatically by the `devforgeai-qa` skill during:
- **Light Validation** (during development phases)
- **Deep Validation** (after story completion)

### Manual Invocation

Run scripts manually for:
- Pre-commit validation
- CI/CD pipeline integration
- Standalone quality analysis
- Coverage gap identification

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

## Quality Thresholds

### Coverage Thresholds

- **Business Logic:** 95% minimum
- **Application Layer:** 85% minimum
- **Infrastructure Layer:** 80% minimum

### Code Quality Thresholds

- **Cyclomatic Complexity:** ≤10 per function/method
- **Code Duplication:** <5% of codebase
- **Maintainability Index:** ≥70
- **Documentation Coverage:** ≥80% for public APIs

### Security Thresholds

- **Critical Vulnerabilities:** ZERO allowed
- **High Vulnerabilities:** ZERO allowed (or documented exceptions)
- **Dependency Vulnerabilities:** All must be patched
- **Secret Scanning:** 100% detection rate

---

## Script Documentation

See `.claude/skills/devforgeai-qa/scripts/README.md` for detailed documentation on:
- Script parameters and options
- Output format specifications
- Integration patterns
- Troubleshooting guide
- Framework-specific configurations

---

## Remember

The framework exists to prevent technical debt through explicit constraints and automated validation. When in doubt, ask the user—never make assumptions.
