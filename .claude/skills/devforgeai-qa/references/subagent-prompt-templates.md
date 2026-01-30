# QA Subagent Prompt Templates

Standardized prompt templates for invoking QA validation subagents from devforgeai-qa skill.

**Location:** src/claude/skills/devforgeai-qa/references/subagent-prompt-templates.md
**Purpose:** Consistent invocation patterns for coverage-analyzer, anti-pattern-scanner, code-quality-auditor

---

## Template 1: coverage-analyzer

### Usage Context
**Replaces:** Phase 1 - Test Coverage Analysis Workflow (inline ~300 lines)
**When to invoke:** After story implementation, before anti-pattern scanning
**Token savings:** ~12,000 tokens (65% reduction)

### Invocation Template

```python
# Phase 1: Test Coverage Analysis
# Delegate to coverage-analyzer subagent

# Step 1: Load context files for subagent
tech_stack_content = Read(file_path="devforgeai/specs/context/tech-stack.md")
source_tree_content = Read(file_path="devforgeai/specs/context/source-tree.md")
coverage_thresholds_content = Read(file_path="src/claude/skills/devforgeai-qa/assets/config/coverage-thresholds.md")

# Step 2: Extract language for tool selection
language = extract_language_from_tech_stack(tech_stack_content)

# Step 3: Determine test command based on language
test_commands = {
    "C#": "dotnet test --collect:'XPlat Code Coverage' --results-directory:./TestResults",
    "Python": "pytest --cov=src --cov-report=json --cov-report=term",
    "Node.js": "npm test -- --coverage --coverageReporters=json-summary",
    "Go": "go test ./... -coverprofile=coverage.out",
    "Rust": "cargo tarpaulin --out Json",
    "Java": "mvn test jacoco:report"
}
test_command = test_commands.get(language, "# Unknown language")

# Step 4: Invoke coverage-analyzer subagent
coverage_result = Task(
    subagent_type="coverage-analyzer",
    description="Analyze test coverage by architectural layer",
    prompt=f"""Analyze test coverage for {story_id}.

**Context Files (READ-ONLY):**

## tech-stack.md
{tech_stack_content}

## source-tree.md
{source_tree_content}

## coverage-thresholds.md
{coverage_thresholds_content}

**Analysis Parameters:**
- Story ID: {story_id}
- Language: {language}
- Test Command: {test_command}
- Thresholds: Business Logic 95%, Application 85%, Infrastructure 80%, Overall 80%

**Instructions:**
Execute your 8-phase coverage analysis workflow:
1. Load and validate context files
2. Execute coverage command: {test_command}
3. Classify files by architectural layer (business_logic, application, infrastructure)
4. Calculate layer-specific coverage percentages
5. Validate against thresholds (95%/85%/80%)
6. Identify coverage gaps with file:line evidence
7. Generate actionable test scenarios for gaps
8. Return structured JSON with results

**Output Requirements:**
- coverage_summary: {{overall, business_logic, application, infrastructure percentages}}
- validation_result: {{per-layer pass/fail}}
- gaps: [{{file, layer, current_coverage, target_coverage, uncovered_lines, suggested_tests}}]
- blocks_qa: boolean (true if business_logic <95% OR application <85% OR overall <80%)
- violations: [severity-categorized list]
- recommendations: [actionable guidance]

**Guardrails:**
- DO NOT modify any code or tests
- DO NOT run tests (only coverage analysis)
- HALT if context files missing or contradictory
- PROVIDE file:line evidence for ALL gaps
- EXPLAIN why each threshold matters (business risk, maintenance cost)

Return JSON matching the output contract specified in your agent definition.
""",
    model="claude-model: opus-4-5-20251001"
)

# Step 5: Parse subagent response
if coverage_result["status"] != "success":
    Display: f"❌ Coverage analysis failed: {coverage_result['error']}"
    Display: f"   Remediation: {coverage_result['remediation']}"
    Return: {"status": "failure", "blocks_qa": true}
    HALT

coverage_summary = coverage_result["coverage_summary"]
coverage_blocks_qa = coverage_result["blocks_qa"]
coverage_gaps = coverage_result["gaps"]
coverage_recommendations = coverage_result["recommendations"]

# Step 6: Update QA state
blocks_qa = blocks_qa OR coverage_blocks_qa

# Step 7: Display coverage results
Display: f"\n=== Phase 1: Test Coverage Analysis ==="
Display: f"  Overall: {coverage_summary['overall_coverage']:.1f}% (threshold 80%)"
Display: f"  Business Logic: {coverage_summary['business_logic_coverage']:.1f}% (threshold 95%)"
Display: f"  Application: {coverage_summary['application_coverage']:.1f}% (threshold 85%)"
Display: f"  Infrastructure: {coverage_summary['infrastructure_coverage']:.1f}% (threshold 80%)"

IF coverage_blocks_qa:
    Display: f"\n  ⛔ BLOCKING: Coverage below thresholds"
    FOR gap in coverage_gaps[:3]:  # Show top 3
        Display: f"    • {gap['file']} ({gap['layer']}): {gap['current_coverage']:.1f}% → target {gap['target_coverage']:.1f}%"

# Step 8: Continue to Phase 2 (Anti-Pattern Detection)
# ...
```

### Response Handling

```python
# Expected response structure
{
  "status": "success",
  "story_id": "STORY-XXX",
  "coverage_summary": {
    "overall_coverage": 87.5,
    "business_logic_coverage": 96.2,
    "application_coverage": 88.1,
    "infrastructure_coverage": 79.3
  },
  "thresholds": {
    "business_logic": 95,
    "application": 85,
    "infrastructure": 80,
    "overall": 80
  },
  "validation_result": {
    "business_logic_passed": true,
    "application_passed": true,
    "infrastructure_passed": false,
    "overall_passed": true
  },
  "gaps": [...],
  "blocks_qa": false,
  "violations": [...],
  "recommendations": [...]
}

# Error response structure
{
  "status": "failure",
  "error": "Coverage command failed: ModuleNotFoundError: No module named 'pytest_cov'",
  "blocks_qa": true,
  "remediation": "Install pytest-cov: pip install pytest-cov"
}
```

### Integration Points

**Before invoking:**
- Load all 3 context files (tech-stack, source-tree, coverage-thresholds)
- Extract language from tech-stack.md
- Determine test command based on language

**After receiving response:**
- Check `status` field (success/failure)
- Update `blocks_qa` state: `blocks_qa = blocks_qa OR coverage_result["blocks_qa"]`
- Display coverage summary
- Store gaps for QA report
- Continue to Phase 2 if successful, HALT if failed

---

## Template 2: anti-pattern-scanner

### Usage Context
**Replaces:** Phase 2 - Anti-Pattern Detection Workflow (inline ~300 lines)
**When to invoke:** After coverage analysis, before spec compliance validation
**Token savings:** ~8,000 tokens (73% reduction)

### Invocation Template

```python
# Phase 2: Anti-Pattern Detection
# Delegate to anti-pattern-scanner subagent

# Step 1: Load ALL 6 context files for subagent
context_files = {}
for file in ["tech-stack.md", "source-tree.md", "dependencies.md",
             "coding-standards.md", "architecture-constraints.md", "anti-patterns.md"]:
    context_files[file] = Read(file_path=f"devforgeai/specs/context/{file}")

# Step 2: Extract language for tool selection
language = extract_language_from_tech_stack(context_files["tech-stack.md"])

# Step 3: Invoke anti-pattern-scanner subagent
anti_pattern_result = Task(
    subagent_type="anti-pattern-scanner",
    description="Scan for anti-patterns and architecture violations",
    prompt=f"""Scan codebase for anti-patterns and architecture violations.

**Context Files (ENFORCE AS LAW):**

## tech-stack.md
{context_files["tech-stack.md"]}

## source-tree.md
{context_files["source-tree.md"]}

## dependencies.md
{context_files["dependencies.md"]}

## coding-standards.md
{context_files["coding-standards.md"]}

## architecture-constraints.md
{context_files["architecture-constraints.md"]}

## anti-patterns.md
{context_files["anti-patterns.md"]}

**Scan Parameters:**
- Story ID: {story_id}
- Language: {language}
- Scan Mode: full (all 6 categories)

**Instructions:**
Execute your 9-phase anti-pattern scanning workflow:
1. Load and validate ALL 6 context files
2. Category 1: Detect library substitution (CRITICAL) - Check locked technologies
3. Category 2: Detect structure violations (HIGH) - Validate file locations
4. Category 3: Detect layer violations (HIGH) - Check cross-layer dependencies
5. Category 4: Detect code smells (MEDIUM) - God objects, long methods, etc.
6. Category 5: Detect security issues (CRITICAL) - OWASP Top 10
7. Category 6: Detect style inconsistencies (LOW) - Documentation, naming
8. Aggregate violations by severity, determine blocking status
9. Return structured JSON with categorized violations

**Detection Categories:**
1. **Library Substitution (CRITICAL):**
   - ORM swap (Dapper ↔ Entity Framework)
   - State manager swap (Zustand ↔ Redux)
   - HTTP client swap (axios ↔ fetch)
   - Validation library swap (Zod ↔ Joi)
   - Testing framework swap (Vitest ↔ Jest)

2. **Structure Violations (HIGH):**
   - Files in wrong layers (EmailService in Domain instead of Infrastructure)
   - Unexpected directories in layers
   - Infrastructure concerns in Domain layer

3. **Layer Violations (HIGH):**
   - Domain referencing Application/Infrastructure
   - Application referencing Infrastructure
   - Circular dependencies

4. **Code Smells (MEDIUM):**
   - God objects (>15 methods, >300 lines)
   - Long methods (>50 lines)
   - Magic numbers

5. **Security Issues (CRITICAL):**
   - Hard-coded secrets
   - SQL injection risk
   - XSS vulnerabilities
   - Insecure deserialization

6. **Style Inconsistencies (LOW):**
   - Missing documentation
   - Naming convention violations

**Output Requirements:**
- violations: {{critical: [], high: [], medium: [], low: []}}
- summary: {{critical_count, high_count, medium_count, low_count, total_violations}}
- blocks_qa: boolean (true if ANY critical OR high violations)
- blocking_reasons: [strings explaining why QA blocked]
- recommendations: [actionable remediation guidance]

**Guardrails:**
- DO NOT suggest fixes that violate context files
- DO NOT propose library substitutions
- HALT if context files missing or contradictory
- PROVIDE file:line evidence for ALL violations
- CLASSIFY severity correctly (CRITICAL blocks, HIGH blocks, MEDIUM warns, LOW advises)

Return JSON matching the output contract specified in your agent definition.
""",
    model="claude-model: opus-4-5-20251001"
)

# Step 4: Parse subagent response
if anti_pattern_result["status"] != "success":
    Display: f"❌ Anti-pattern scanning failed: {anti_pattern_result['error']}"
    Display: f"   Remediation: {anti_pattern_result['remediation']}"
    Return: {"status": "failure", "blocks_qa": true}
    HALT

violations = anti_pattern_result["violations"]
summary = anti_pattern_result["summary"]
anti_pattern_blocks_qa = anti_pattern_result["blocks_qa"]
blocking_reasons = anti_pattern_result["blocking_reasons"]
recommendations = anti_pattern_result["recommendations"]

# Step 5: Update QA state
blocks_qa = blocks_qa OR anti_pattern_blocks_qa

# Step 6: Display anti-pattern results
Display: f"\n=== Phase 2: Anti-Pattern Detection ==="
Display: f"  Critical: {summary['critical_count']} violations"
Display: f"  High: {summary['high_count']} violations"
Display: f"  Medium: {summary['medium_count']} violations"
Display: f"  Low: {summary['low_count']} violations"

IF anti_pattern_blocks_qa:
    Display: f"\n  ⛔ BLOCKING: {len(blocking_reasons)} reasons"
    FOR reason in blocking_reasons:
        Display: f"    • {reason}"

    # Show top 3 critical violations
    FOR violation in violations["critical"][:3]:
        Display: f"    • {violation['pattern']}: {violation['file']}:{violation['line']}"

# Step 7: Continue to Phase 3 (Spec Compliance Validation)
# ...
```

### Response Handling

```python
# Expected response structure
{
  "status": "success",
  "story_id": "STORY-XXX",
  "violations": {
    "critical": [...],
    "high": [...],
    "medium": [...],
    "low": [...]
  },
  "summary": {
    "critical_count": 1,
    "high_count": 2,
    "medium_count": 5,
    "low_count": 12,
    "total_violations": 20
  },
  "blocks_qa": true,
  "blocking_reasons": [...],
  "recommendations": [...]
}
```

---

## Template 3: code-quality-auditor

### Usage Context
**Replaces:** Phase 4 - Code Quality Metrics Workflow (inline ~250 lines)
**When to invoke:** After spec compliance validation, before QA report generation
**Token savings:** ~6,000 tokens (70% reduction)

### Invocation Template

```python
# Phase 4: Code Quality Metrics
# Delegate to code-quality-auditor subagent

# Step 1: Load context files for subagent
tech_stack_content = Read(file_path="devforgeai/specs/context/tech-stack.md")
quality_metrics_content = Read(file_path="src/claude/skills/devforgeai-qa/assets/config/quality-metrics.md")

# Step 2: Extract language for tool selection
language = extract_language_from_tech_stack(tech_stack_content)

# Step 3: Invoke code-quality-auditor subagent
quality_result = Task(
    subagent_type="code-quality-auditor",
    description="Analyze code quality metrics (complexity, duplication, maintainability)",
    prompt=f"""Analyze code quality metrics for {story_id}.

**Context Files:**

## tech-stack.md
{tech_stack_content}

## quality-metrics.md
{quality_metrics_content}

**Analysis Parameters:**
- Story ID: {story_id}
- Language: {language}
- Source Paths: ["src/", "lib/"]
- Exclude Paths: ["tests/", "migrations/", "generated/"]
- Thresholds:
  - Complexity WARNING: 15, CRITICAL: 20
  - Duplication WARNING: 20%, CRITICAL: 25%
  - Maintainability WARNING: 50, CRITICAL: 40

**Instructions:**
Execute your 8-phase code quality analysis workflow:
1. Load and validate context files
2. Execute cyclomatic complexity analysis (per function, per file)
3. Execute code duplication detection (duplicate blocks, percentage)
4. Execute maintainability index calculation (0-100 scale)
5. Generate business impact explanations (bug risk, maintenance cost, onboarding time)
6. Generate refactoring pattern recommendations (Extract Method, etc.)
7. Aggregate results and determine blocking status
8. Return structured JSON with metrics and recommendations

**Metrics to Calculate:**
1. **Cyclomatic Complexity:**
   - Average per function
   - Average per file
   - Max complexity (worst offender)
   - Functions over threshold (>20 = CRITICAL, 15-20 = WARNING)

2. **Code Duplication:**
   - Duplication percentage
   - Duplicate blocks (files, line ranges, pattern description)
   - >25% = CRITICAL, 20-25% = WARNING

3. **Maintainability Index:**
   - Average MI across all files
   - Low maintainability files (<40 = CRITICAL, 40-50 = WARNING)
   - MI formula: 171 - 5.2*ln(Volume) - 0.23*Complexity - 16.2*ln(LOC)

**Output Requirements:**
- metrics: {{complexity: {{}}, duplication: {{}}, maintainability: {{}}}}
- extreme_violations: [{{type, severity, file, line, metric, threshold, business_impact, refactoring_pattern}}]
- blocks_qa: boolean (true if ANY extreme violations)
- blocking_reasons: [strings explaining violations]
- recommendations: [actionable guidance with business context]

**Business Impact Requirements:**
For EACH extreme violation, explain:
- Bug risk: Statistical correlation with defect rates
- Testing burden: Number of test cases required
- Onboarding impact: Time to understand code
- Maintenance cost: Effort multiplier for changes

**Refactoring Pattern Requirements:**
For EACH extreme violation, provide:
- Specific pattern: Extract Method, Decompose Conditional, etc.
- Target metrics: Current → Goal
- Implementation steps: 1-5 concrete actions
- Expected outcome: Reduced complexity/duplication, improved MI

**Guardrails:**
- DO NOT modify any code
- FOCUS on EXTREME violations only (complexity >20, duplication >25%, MI <40)
- EXPLAIN business impact in quantifiable terms
- PROVIDE specific refactoring patterns, not generic advice
- HALT if analysis tools not available

Return JSON matching the output contract specified in your agent definition.
""",
    model="claude-model: opus-4-5-20251001"
)

# Step 4: Parse subagent response
if quality_result["status"] != "success":
    Display: f"❌ Code quality analysis failed: {quality_result['error']}"
    Display: f"   Remediation: {quality_result['remediation']}"
    Return: {"status": "failure", "blocks_qa": true}
    HALT

metrics = quality_result["metrics"]
extreme_violations = quality_result["extreme_violations"]
quality_blocks_qa = quality_result["blocks_qa"]
blocking_reasons = quality_result["blocking_reasons"]
recommendations = quality_result["recommendations"]

# Step 5: Update QA state
blocks_qa = blocks_qa OR quality_blocks_qa

# Step 6: Display quality metrics results
Display: f"\n=== Phase 4: Code Quality Metrics ==="
Display: f"  Complexity (avg): {metrics['complexity']['average_per_function']:.1f}"
Display: f"  Duplication: {metrics['duplication']['percentage']:.1f}%"
Display: f"  Maintainability Index: {metrics['maintainability']['average_index']:.1f}"

IF quality_blocks_qa:
    Display: f"\n  ⛔ BLOCKING: {len(extreme_violations)} extreme quality violations"
    FOR violation in extreme_violations[:3]:  # Show top 3
        Display: f"    • {violation['type'].upper()}: {violation['file']}:{violation['line']}"
        Display: f"      Metric: {violation['metric']}, Threshold: {violation['threshold']}"

# Step 7: Continue to Phase 5 (QA Report Generation)
# ...
```

### Response Parsing

**Step 1: Parse JSON Response**
```python
# Parse subagent response (JSON structure)
quality_result = json.loads(subagent_output)

# Validate status field
if quality_result["status"] != "success":
    # Handle error (see Common Error Handling Pattern)
    handle_subagent_error(quality_result, "code-quality-auditor", "Phase 4: Code Quality Metrics")
    HALT
```

**Step 2: Extract Metrics**
```python
# Extract quality metrics
complexity_metrics = quality_result["metrics"]["complexity"]
duplication_metrics = quality_result["metrics"]["duplication"]
maintainability_metrics = quality_result["metrics"]["maintainability"]

# Calculate averages for display
avg_complexity = complexity_metrics["average_per_function"]
max_complexity = complexity_metrics["max_complexity"]["score"]
duplication_pct = duplication_metrics["percentage"]
avg_mi = maintainability_metrics["average_index"]
```

**Step 3: Extract Violations**
```python
# Extract extreme violations (CRITICAL only)
extreme_violations = quality_result["extreme_violations"]

# Group by type for reporting
complexity_violations = filter(extreme_violations, type == "complexity")
duplication_violations = filter(extreme_violations, type == "duplication")
maintainability_violations = filter(extreme_violations, type == "maintainability")
```

**Step 4: Update QA Blocking State**
```python
# Update blocks_qa (OR operation to preserve previous phase blocks)
quality_blocks_qa = quality_result["blocks_qa"]
blocks_qa = blocks_qa OR quality_blocks_qa

# Collect blocking reasons
if quality_blocks_qa:
    blocking_reasons.extend(quality_result["blocking_reasons"])
```

**Step 5: Extract Recommendations**
```python
# Add quality recommendations to QA report
recommendations.extend(quality_result["recommendations"])

# Separate by type
blocking_recs = filter(quality_result["recommendations"], startswith("⛔"))
warning_recs = filter(quality_result["recommendations"], startswith("⚠️"))
positive_recs = filter(quality_result["recommendations"], startswith("✅"))
```

**Step 6: Display Results**
```python
Display: f"\n=== Phase 4: Code Quality Metrics ==="
Display: f"  Complexity (avg): {avg_complexity:.1f} (threshold: 20)"
Display: f"  Duplication: {duplication_pct:.1f}% (threshold: 25%)"
Display: f"  Maintainability Index: {avg_mi:.1f} (threshold: 40)"

IF quality_blocks_qa:
    Display: f"\n  ⛔ BLOCKING: {len(extreme_violations)} extreme quality violations"
    FOR violation in extreme_violations[:3]:  # Show top 3
        Display: f"    • {violation['type'].upper()}: {violation['file']}:{violation['line']}"
        Display: f"      {violation['metric']} (threshold: {violation['threshold']})"
        Display: f"      Impact: {violation['business_impact'][:80]}..."
ELSE:
    Display: f"  ✅ All quality metrics meet thresholds"
```

**Step 7: Store for QA Report**
```python
# Store metrics for final QA report generation (Phase 5)
qa_report_data["code_quality"] = {
    "metrics": quality_result["metrics"],
    "violations": extreme_violations,
    "recommendations": quality_result["recommendations"],
    "blocks_qa": quality_blocks_qa
}
```

### Response Handling

```python
# Expected response structure
{
  "status": "success",
  "story_id": "STORY-XXX",
  "metrics": {
    "complexity": {...},
    "duplication": {...},
    "maintainability": {...}
  },
  "extreme_violations": [...],
  "blocks_qa": false,
  "blocking_reasons": [],
  "recommendations": [...],
  "analysis_duration_ms": 3245
}
```

---

## Common Error Handling Pattern

For all three subagents, use this error handling pattern:

```python
# Generic error handler
def handle_subagent_error(result, subagent_name, phase_name):
    if result["status"] != "success":
        Display: f"\n❌ {phase_name} FAILED"
        Display: f"   Subagent: {subagent_name}"
        Display: f"   Error: {result['error']}"

        if "remediation" in result:
            Display: f"   Remediation: {result['remediation']}"

        Display: f"\n⛔ QA validation cannot proceed due to {subagent_name} failure"

        # Update QA state
        blocks_qa = true

        # Return failure to skill
        Return: {
            "status": "failure",
            "phase_failed": phase_name,
            "subagent": subagent_name,
            "error": result["error"],
            "blocks_qa": true
        }
        HALT

# Usage
handle_subagent_error(coverage_result, "coverage-analyzer", "Phase 1: Coverage Analysis")
handle_subagent_error(anti_pattern_result, "anti-pattern-scanner", "Phase 2: Anti-Pattern Detection")
handle_subagent_error(quality_result, "code-quality-auditor", "Phase 4: Code Quality Metrics")
```

---

## Subagent Invocation Checklist

Before invoking ANY subagent, verify:

- [ ] All required context files loaded and passed to subagent
- [ ] Language extracted from tech-stack.md
- [ ] Story ID available in scope
- [ ] Subagent name spelled correctly
- [ ] Model specified (claude-model: opus-4-5-20251001)
- [ ] Prompt includes complete context (all 6 context files for anti-pattern-scanner)
- [ ] Prompt specifies expected output format (JSON structure)
- [ ] Prompt includes guardrails (read-only, HALT conditions)
- [ ] Error handling implemented (check status field)
- [ ] blocks_qa state updated correctly (OR operation, not assignment)
- [ ] Display messages show subagent results
- [ ] Results stored for QA report generation

---

## Token Budget Impact

**Before subagent delegation:**
- Phase 1 (Coverage): ~12K tokens (inline)
- Phase 2 (Anti-Patterns): ~8K tokens (inline)
- Phase 4 (Quality): ~6K tokens (inline)
- **Total: ~26K tokens**

**After subagent delegation:**
- Phase 1 prompt: ~3K tokens
- Phase 2 prompt: ~4K tokens
- Phase 4 prompt: ~2K tokens
- **Total: ~9K tokens**

**Savings: ~17K tokens (65% reduction)**

---

## Testing Subagent Integration

### Integration Test Template

```python
def test_qa_skill_invokes_coverage_analyzer():
    """Test that devforgeai-qa skill successfully invokes coverage-analyzer"""
    # Given: Story with 88% coverage (below 95% business logic threshold)
    story_id = "STORY-TEST-001"

    # When: QA skill runs Phase 1
    qa_result = invoke_devforgeai_qa(story_id, mode="deep")

    # Then: coverage-analyzer was invoked
    assert "coverage_summary" in qa_result
    assert qa_result["coverage_summary"]["business_logic_coverage"] < 95

    # And: blocks_qa set correctly
    assert qa_result["blocks_qa"] == true

    # And: Gap recommendations provided
    assert len(qa_result["coverage_gaps"]) > 0


def test_qa_skill_invokes_anti_pattern_scanner():
    """Test that devforgeai-qa skill successfully invokes anti-pattern-scanner"""
    # Given: Story with library substitution (Dapper → Entity Framework)
    story_id = "STORY-TEST-002"

    # When: QA skill runs Phase 2
    qa_result = invoke_devforgeai_qa(story_id, mode="deep")

    # Then: anti-pattern-scanner was invoked
    assert "violations" in qa_result
    assert qa_result["violations"]["critical"][0]["type"] == "library_substitution"

    # And: blocks_qa set correctly
    assert qa_result["blocks_qa"] == true


def test_qa_skill_invokes_code_quality_auditor():
    """Test that devforgeai-qa skill successfully invokes code-quality-auditor"""
    # Given: Story with extreme complexity (function with complexity 28)
    story_id = "STORY-TEST-003"

    # When: QA skill runs Phase 4
    qa_result = invoke_devforgeai_qa(story_id, mode="deep")

    # Then: code-quality-auditor was invoked
    assert "metrics" in qa_result
    assert qa_result["metrics"]["complexity"]["max_complexity"]["score"] == 28

    # And: blocks_qa set correctly
    assert qa_result["blocks_qa"] == true

    # And: Business impact explanation provided
    assert "High bug risk" in qa_result["extreme_violations"][0]["business_impact"]
```

---

## Maintenance

### When to Update Templates

- **Context file format changes:** Update file loading logic
- **New language support:** Add language → tool mapping
- **Threshold changes:** Update threshold values in prompts
- **New subagent features:** Update prompt to invoke new capabilities
- **Output contract changes:** Update response parsing logic

### Template Version History

- v1.0 (2025-11-20): Initial templates for coverage-analyzer, anti-pattern-scanner, code-quality-auditor
- Updated paths to use src/ directory structure (production cutover)
