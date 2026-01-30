# DevForgeAI QA Automation Scripts

Python automation utilities for comprehensive code quality analysis. These scripts are referenced by the `devforgeai-qa` skill for deep validation workflows.

## Overview

Six specialized scripts for QA validation:

1. **generate_coverage_report.py** - Parse coverage data → HTML visualization
2. **analyze_complexity.py** - Calculate cyclomatic complexity
3. **detect_duplicates.py** - Find code duplication
4. **validate_spec_compliance.py** - Validate against story acceptance criteria
5. **security_scan.py** - Scan for security vulnerabilities
6. **generate_test_stubs.py** - Auto-generate test templates

## Installation

```bash
# Install dependencies
pip install -r requirements.txt

# Verify installation
python generate_coverage_report.py --help
```

## Scripts

### 1. Coverage Report Generator

Generate HTML coverage reports with layer-level breakdowns.

**Supports:** .NET (Cobertura XML), Python (pytest-cov JSON), JavaScript (Istanbul JSON)

**Usage:**

```bash
# From Python coverage
python generate_coverage_report.py coverage.json --output coverage-report.html

# From .NET coverage
python generate_coverage_report.py coverage.cobertura.xml --output dotnet-coverage.html

# With layer mapping from source-tree.md
python generate_coverage_report.py coverage.json \
  --source-tree devforgeai/specs/context/source-tree.md \
  --output coverage-report.html
```

**Output:**
- HTML report with color-coded coverage visualization
- Overall coverage percentage
- Coverage by layer (domain/application/infrastructure)
- File-by-file breakdown
- Uncovered line highlighting

### 2. Complexity Analyzer

Calculate cyclomatic complexity for codebase.

**Supports:** Python (radon), Multi-language (lizard)

**Usage:**

```bash
# Analyze with default thresholds (method: 10, class: 50)
python analyze_complexity.py src/

# Custom thresholds
python analyze_complexity.py src/ --method-threshold 15 --class-threshold 75

# Generate JSON report
python analyze_complexity.py src/ --output complexity-report.json

# Print violations only
python analyze_complexity.py src/ --violations-only
```

**Output:**
- Method complexity violations (>10)
- Class complexity violations (>50)
- Average complexity
- JSON report with detailed metrics

### 3. Duplication Detector

Find duplicate code blocks across codebase.

**Supports:** All text-based languages (language-agnostic)

**Usage:**

```bash
# Detect duplicates (minimum 6 lines)
python detect_duplicates.py src/

# Larger duplicates only
python detect_duplicates.py src/ --min-lines 10

# Generate JSON report
python detect_duplicates.py src/ --output duplication-report.json

# Specific file types
python detect_duplicates.py src/ --patterns "*.py" "*.js" "*.cs"
```

**Output:**
- Duplication percentage
- Duplicate block locations with line numbers
- Code fragment previews
- Refactoring suggestions

### 4. Spec Compliance Validator

Validate implementation against story acceptance criteria.

**Supports:** Markdown story files, pytest/xUnit/Jest tests

**Usage:**

```bash
# Validate story
python validate_spec_compliance.py ai_docs/Stories/1.1.story.md tests/

# Generate JSON report
python validate_spec_compliance.py story.md tests/ --output compliance.json
```

**How it works:**
1. Parses story markdown file
2. Extracts acceptance criteria
3. Finds tests matching each criterion (keyword-based)
4. Runs tests and validates they pass
5. Generates compliance report

**Output:**
- Compliance percentage
- Criteria passing/failing/no tests
- Test mappings for each criterion
- Failing test details

### 5. Security Scanner

Scan for common security vulnerabilities.

**Supports:** Python, C#, JavaScript/TypeScript, Java, PHP, Ruby

**Vulnerability Detection:**
- SQL Injection (CWE-89)
- Cross-Site Scripting / XSS (CWE-79)
- Hardcoded Secrets (CWE-798)
- Weak Cryptography (CWE-327)
- Path Traversal (CWE-22)
- Insecure Randomness (CWE-330)
- Command Injection (CWE-78)

**Usage:**

```bash
# Scan with console output
python security_scan.py src/

# Generate JSON report
python security_scan.py src/ --output security-report.json

# Scan specific directory
python security_scan.py src/Application/ --output app-security.json
```

**Output:**
- Violations by severity (CRITICAL/HIGH/MEDIUM/LOW)
- CWE categorization
- File and line number references
- Code snippets
- Remediation guidance

### 6. Test Stub Generator

Auto-generate test templates for untested code.

**Supports:** C# (xUnit), Python (pytest), JavaScript/TypeScript (Jest)

**Usage:**

```bash
# Auto-detect framework from file extension
python generate_test_stubs.py src/Services/OrderService.cs

# Specify framework explicitly
python generate_test_stubs.py src/order_service.py --framework pytest

# Specify output file
python generate_test_stubs.py src/user.js --output tests/user.test.js
```

**Features:**
- Parses source files to extract methods
- Generates test stubs with proper framework syntax
- Creates tests for:
  - Valid input scenarios
  - Null/invalid input handling
  - Edge cases (TODO placeholders)

**Output:**
- Complete test file with framework imports
- Test stubs for all public methods
- Arrange/Act/Assert structure
- TODO comments for completion

## Integration with devforgeai-qa Skill

These scripts are invoked by the `devforgeai-qa` skill during deep validation:

```
devforgeai-qa Deep Validation Workflow:
├── Phase 1: Coverage Analysis
│   └── generate_coverage_report.py
├── Phase 2: Anti-Pattern Detection
│   ├── analyze_complexity.py
│   ├── detect_duplicates.py
│   └── security_scan.py
├── Phase 3: Spec Compliance
│   └── validate_spec_compliance.py
└── Phase 4: Test Gap Filling
    └── generate_test_stubs.py
```

## Examples

### Complete QA Workflow

```bash
# 1. Generate coverage report
python generate_coverage_report.py coverage.json \
  --source-tree devforgeai/specs/context/source-tree.md \
  --output qa/coverage-report.html

# 2. Analyze complexity
python analyze_complexity.py src/ \
  --output qa/complexity-report.json

# 3. Detect duplicates
python detect_duplicates.py src/ \
  --output qa/duplication-report.json

# 4. Security scan
python security_scan.py src/ \
  --output qa/security-report.json

# 5. Validate spec compliance
python validate_spec_compliance.py ai_docs/Stories/1.1.story.md tests/ \
  --output qa/compliance-report.json

# 6. Generate test stubs for gaps
python generate_test_stubs.py src/Services/UncoveredService.cs \
  --output tests/Services/UncoveredServiceTests.cs
```

### CI/CD Integration

```bash
# Run all QA checks and fail on violations
set -e

echo "Running coverage analysis..."
python generate_coverage_report.py coverage.json --output qa/coverage.html

echo "Checking code complexity..."
python analyze_complexity.py src/ --violations-only

echo "Detecting duplication..."
python detect_duplicates.py src/
if [ $? -ne 0 ]; then
  echo "❌ Duplication exceeds threshold"
  exit 1
fi

echo "Security scan..."
python security_scan.py src/ --output qa/security.json
if [ $? -ne 0 ]; then
  echo "❌ Security violations found"
  exit 1
fi

echo "Validating spec compliance..."
python validate_spec_compliance.py story.md tests/
if [ $? -ne 0 ]; then
  echo "❌ Spec compliance failed"
  exit 1
fi

echo "✅ All QA checks passed!"
```

## Configuration

### Coverage Thresholds

Scripts read thresholds from `.claude/skills/devforgeai-qa/assets/config/coverage-thresholds.md`:

```markdown
# Test Coverage Thresholds (STRICT)
- Business logic: 95% minimum
- Application layer: 85% minimum
- Infrastructure: 80% minimum
- Overall project: 80% minimum
```

### Quality Metrics

Scripts read thresholds from `.claude/skills/devforgeai-qa/assets/config/quality-metrics.md`:

```markdown
# Code Quality Thresholds
- Method complexity: Maximum 10
- Class complexity: Maximum 50
- Code duplication: Maximum 5%
- Documentation coverage: 80% minimum
```

## Development

### Adding New Checks

To add a new security vulnerability pattern to `security_scan.py`:

```python
self.patterns['new_vulnerability'] = {
    'patterns': [
        r'vulnerable_pattern_regex',
    ],
    'severity': 'CRITICAL',
    'category': 'Vulnerability Type',
    'cwe': 'CWE-XXX',
    'description': 'Description of the vulnerability',
    'remediation': 'How to fix it'
}
```

### Testing Scripts

```bash
# Run script tests (if test suite exists)
pytest tests/scripts/

# Manual testing
python generate_coverage_report.py sample-data/coverage.json
python analyze_complexity.py sample-data/src/
python detect_duplicates.py sample-data/src/
```

## Troubleshooting

### "Tool not found" errors

Install missing tools:

```bash
# Radon (Python complexity)
pip install radon

# Lizard (multi-language complexity)
pip install lizard

# Coverage.py (for .coverage files)
pip install coverage
```

### "No tests found" in compliance validation

Ensure test naming matches patterns:
- Python: `test_*.py` or `*_test.py`
- C#: `*Tests.cs` with `[Fact]` or `[Test]` attributes
- JavaScript: `*.test.js` or `*.spec.js`

### Permission errors

Make scripts executable:

```bash
chmod +x *.py
```

## Token Efficiency

Scripts are designed for minimal token usage:
- Structured JSON output (not verbose text)
- Selective file reading (only relevant files)
- Efficient reporting (summaries over details)

Estimated token usage in deep validation:
- Coverage analysis: ~3,000 tokens
- Complexity analysis: ~2,000 tokens
- Duplication detection: ~2,000 tokens
- Security scan: ~3,000 tokens
- Spec compliance: ~4,000 tokens
- Test stub generation: ~1,000 tokens

**Total: ~15,000 tokens** (vs ~65,000 for full native tool analysis)

## License

Part of DevForgeAI spec-driven development framework.

## Support

For issues or enhancements:
1. Check script help: `python script.py --help`
2. Review devforgeai-qa SKILL.md for integration guidance
3. Consult devforgeai/qa/references/ for methodology details
