# Anti-Pattern Detection Reference (Consolidated)

**Consolidated:** anti-pattern-detection.md + anti-pattern-detection-workflow.md
**Token savings:** ~1.2K tokens (single load vs 2 separate loads)
**Version:** 2.0 (STORY-265)

---

## Table of Contents

- [Overview](#overview)
- [Phase 2 Workflow](#phase-2-workflow)
  - [Step 1: Load Context Files](#step-1-load-all-6-context-files)
  - [Step 2: Invoke Subagent](#step-2-invoke-anti-pattern-scanner-subagent)
  - [Step 3: Parse Response](#step-3-parse-json-response)
  - [Step 4: Update State](#step-4-update-blocks_qa-state-or-logic)
  - [Step 5: Display Summary](#step-5-display-violations-summary)
  - [Step 6: Store Results](#step-6-store-violations-for-qa-report)
- [Detection Algorithms](#detection-algorithms)
- [Anti-Pattern Categories](#anti-pattern-categories)
- [Severity Assessment](#severity-assessment)
- [False Positive Handling](#false-positive-handling)
- [Quick Reference](#quick-reference)

---

## Overview

This guide provides detection algorithms and workflow for identifying anti-patterns that lead to technical debt. Used in QA Phase 2 for comprehensive architecture violation detection.

**Subagent:** anti-pattern-scanner
**Token Efficiency:** ~3K tokens (vs ~8K tokens inline) = 73% reduction

---

## Phase 2 Workflow

### Step 1: Load ALL 6 Context Files

**CRITICAL:** anti-pattern-scanner requires ALL 6 context files. Load them into conversation context:

```
context_files = {}

FOR each file in ["tech-stack", "source-tree", "dependencies", "coding-standards", "architecture-constraints", "anti-patterns"]:
  file_path = f"devforgeai/specs/context/{file}.md"

  content = Read(file_path=file_path)

  context_files[file] = {
    "path": file_path,
    "content": content
  }
```

**If ANY file missing:**
```
Display: "Context file missing: {file_path}"
Display: "Run /create-context to generate context files"
HALT workflow
```

---

### Step 2: Invoke anti-pattern-scanner Subagent

**Subagent invocation pattern:**

```
anti_pattern_result = Task(
  subagent_type="anti-pattern-scanner",
  description="Scan codebase for architecture violations",
  model="claude-model: opus-4-5-20251001",
  prompt=f"""
Scan codebase for architecture violations and security issues.

**Story ID:** {story_id}
**Scan Mode:** full
**Language:** {detected_language}

**Context Files Loaded (ALL 6 REQUIRED):**

1. **tech-stack.md:**
```
{context_files["tech-stack"]["content"]}
```

2. **source-tree.md:**
```
{context_files["source-tree"]["content"]}
```

3. **dependencies.md:**
```
{context_files["dependencies"]["content"]}
```

4. **coding-standards.md:**
```
{context_files["coding-standards"]["content"]}
```

5. **architecture-constraints.md:**
```
{context_files["architecture-constraints"]["content"]}
```

6. **anti-patterns.md:**
```
{context_files["anti-patterns"]["content"]}
```

**Detection Categories (check all 6):**
- Category 1: Library Substitution (CRITICAL) - 5 technology types
- Category 2: Structure Violations (HIGH) - 3 validation checks
- Category 3: Layer Violations (HIGH) - 2 dependency checks
- Category 4: Code Smells (MEDIUM) - 3 metric checks
- Category 5: Security Vulnerabilities (CRITICAL) - 4 OWASP checks
- Category 6: Style Inconsistencies (LOW) - 2 linting checks

**Expected Output:** JSON with violations grouped by severity, blocking status, and remediation guidance.

Execute anti-pattern-scanner specification: Load context -> Scan 6 categories -> Return structured violations.
"""
)
```

---

### Step 3: Parse JSON Response

**Validate response structure:**

```
IF anti_pattern_result["status"] == "failure":
  Display: f"Anti-pattern scanner failed: {anti_pattern_result['error']}"
  Display: f"Remediation: {anti_pattern_result['remediation']}"
  blocks_qa = true
  HALT

# Extract violations by severity
violations_critical = anti_pattern_result["violations"]["critical"]
violations_high = anti_pattern_result["violations"]["high"]
violations_medium = anti_pattern_result["violations"]["medium"]
violations_low = anti_pattern_result["violations"]["low"]

# Extract summary
total_violations = anti_pattern_result["summary"]["total_violations"]
blocks_qa_anti_pattern = anti_pattern_result["blocks_qa"]
```

---

### Step 4: Update blocks_qa State (OR Logic)

**CRITICAL:** Use OR operation to preserve existing blocks from Phase 1:

```
# Preserve previous blocking state from Phase 1 (coverage)
blocks_qa = blocks_qa OR blocks_qa_anti_pattern

IF blocks_qa_anti_pattern:
  blocking_reasons.extend(anti_pattern_result["blocking_reasons"])
```

**Example:**
- Phase 1 coverage blocks (coverage <95%) -> blocks_qa = true
- Phase 2 anti-patterns clean -> blocks_qa_anti_pattern = false
- Result: blocks_qa remains true (OR operation preserves Phase 1 block)

---

### Step 5: Display Violations Summary

**Display format:**

```
Display: ""
Display: "Phase 2: Anti-Pattern Detection"
Display: ""

IF total_violations == 0:
  Display: "No violations detected - Code complies with all architectural constraints"
ELSE:
  Display: f"Total Violations: {total_violations}"
  Display: ""

  IF len(violations_critical) > 0:
    Display: f"CRITICAL ({len(violations_critical)}):"
    FOR v in violations_critical:
      Display: f"  - {v['pattern']}: {v['file']}:{v['line']}"
      Display: f"    {v['remediation']}"

  IF len(violations_high) > 0:
    Display: f"HIGH ({len(violations_high)}):"
    FOR v in violations_high:
      Display: f"  - {v['pattern']}: {v['file']}:{v['line']}"

  IF len(violations_medium) > 0:
    Display: f"MEDIUM ({len(violations_medium)}) - Warnings only"

  IF len(violations_low) > 0:
    Display: f"LOW ({len(violations_low)}) - Advisory only"

Display: ""

IF blocks_qa_anti_pattern:
  Display: "QA BLOCKED by anti-pattern violations"
  Display: "Fix CRITICAL and/or HIGH violations before proceeding"
ELSE:
  Display: "Phase 2 Complete - No blocking violations"

Display: ""
```

---

### Step 6: Store Violations for QA Report

**Add violations to QA report data:**

```
qa_report_data["anti_pattern_violations"] = {
  "total": total_violations,
  "by_severity": {
    "critical": violations_critical,
    "high": violations_high,
    "medium": violations_medium,
    "low": violations_low
  },
  "blocks_qa": blocks_qa_anti_pattern,
  "blocking_reasons": anti_pattern_result["blocking_reasons"] if blocks_qa_anti_pattern else []
}
```

---

## Detection Algorithms

### Static Code Analysis

**Grep-based pattern matching:**
```python
def detect_sql_injection(source_path):
    # Pattern: String concatenation in SQL queries
    patterns = [
        r'ExecuteRawSql\(.*\+',
        r'string\.Format.*SELECT',
        r'f"SELECT.*\{',
        r'`SELECT.*\$\{'
    ]

    violations = []
    for pattern in patterns:
        matches = grep(pattern, source_path)
        for match in matches:
            violations.append({
                "severity": "CRITICAL",
                "file": match.file,
                "line": match.line,
                "pattern": pattern,
                "fix": "Use parameterized queries"
            })

    return violations
```

### Complexity Analysis

**Calculate cyclomatic complexity:**
```python
def calculate_complexity(method_ast):
    """
    Complexity = 1 + number of decision points
    Decision points: if, while, for, case, catch, &&, ||, ?:
    """
    complexity = 1

    for node in ast.walk(method_ast):
        if isinstance(node, (ast.If, ast.While, ast.For, ast.ExceptHandler)):
            complexity += 1
        elif isinstance(node, ast.BoolOp):  # and, or
            complexity += len(node.values) - 1

    return complexity
```

### Dependency Graph Analysis

**Detect circular dependencies:**
```python
def detect_circular_dependencies(project_path):
    # Build dependency graph
    graph = {}
    for file in find_source_files(project_path):
        imports = extract_imports(file)
        graph[file] = imports

    # Find cycles using DFS
    visited = set()
    stack = set()
    cycles = []

    def dfs(node, path):
        if node in stack:
            # Cycle detected
            cycle_start = path.index(node)
            cycles.append(path[cycle_start:])
            return

        if node in visited:
            return

        visited.add(node)
        stack.add(node)

        for dependency in graph.get(node, []):
            dfs(dependency, path + [node])

        stack.remove(node)

    for file in graph:
        dfs(file, [])

    return cycles
```

---

## Anti-Pattern Categories

### Category 1: Library Substitution (CRITICAL)

**Detection:**
```python
def detect_library_substitution(tech_stack_md, source_path):
    # Load locked technologies
    locked_tech = parse_tech_stack(tech_stack_md)

    violations = []

    # Check ORM substitution
    if locked_tech["ORM"] == "Dapper":
        ef_usage = grep("using EntityFramework|using Microsoft.EntityFrameworkCore", source_path)
        if ef_usage:
            violations.append({
                "severity": "CRITICAL",
                "category": "Library Substitution",
                "locked_tech": "Dapper",
                "actual_tech": "Entity Framework",
                "files": ef_usage.files
            })

    # Check state management (React)
    if locked_tech.get("StateManagement") == "Zustand":
        redux_usage = grep("from '@reduxjs/toolkit'|import.*redux", source_path)
        if redux_usage:
            violations.append({
                "severity": "CRITICAL",
                "category": "Library Substitution",
                "locked_tech": "Zustand",
                "actual_tech": "Redux"
            })

    return violations
```

### Category 2: Structure Violations (HIGH)

**Detection:**
```python
def detect_structure_violations(source_tree_md, files):
    rules = parse_source_tree_rules(source_tree_md)
    violations = []

    for file in files:
        # Determine expected location based on file type
        expected_location = get_expected_location(file, rules)

        if file.path != expected_location:
            violations.append({
                "severity": "HIGH",
                "category": "Structure Violation",
                "file": file.path,
                "expected": expected_location,
                "rule": rules.get_rule_for_file(file)
            })

    return violations
```

### Category 3: Cross-Layer Dependencies (CRITICAL)

**Detection:**
```python
def detect_layer_violations(architecture_constraints_md, source_path):
    # Load layer dependency matrix
    matrix = parse_layer_matrix(architecture_constraints_md)
    # Example: Domain can only reference Domain

    violations = []

    # Check Domain layer purity
    domain_files = glob("src/Domain/**/*.cs")
    for file in domain_files:
        imports = extract_imports(file)

        for imp in imports:
            if "Infrastructure" in imp or "Application" in imp:
                violations.append({
                    "severity": "CRITICAL",
                    "category": "Cross-Layer Dependency",
                    "file": file,
                    "violation": imp,
                    "rule": "Domain must not reference Application/Infrastructure"
                })

    return violations
```

### Category 4: Security Anti-Patterns (CRITICAL)

**Pattern detection:**
```python
patterns = {
    "sql_injection": [
        r'ExecuteRawSql\(.*\+',
        r'string\.Format.*SELECT',
        r'f"SELECT.*\{',
        r'\$"SELECT.*\{',
        r'`SELECT.*\$\{'
    ],
    "xss": [
        r'innerHTML\s*=',
        r'dangerouslySetInnerHTML',
        r'document\.write\('
    ],
    "hardcoded_secrets": [
        r'password\s*=\s*["\'][^"\']{8,}["\']',
        r'api_?key\s*=\s*["\'][^"\']+["\']',
        r'connectionstring\s*=\s*["\'].*password='
    ]
}
```

### Category 5: Code Smells (MEDIUM/LOW)

**God Object Detection:**
```python
def detect_god_objects(source_path):
    violations = []

    for file in find_source_files(source_path):
        lines = count_lines(file)
        methods = count_methods(file)
        responsibilities = estimate_responsibilities(file)

        if lines > 500 or methods > 20 or responsibilities > 5:
            violations.append({
                "severity": "MEDIUM",
                "category": "God Object",
                "file": file,
                "lines": lines,
                "methods": methods,
                "responsibilities": responsibilities,
                "fix": "Split into smaller, focused classes"
            })

    return violations
```

**Magic Number Detection:**
```python
def detect_magic_numbers(source_path):
    # Pattern: Numeric literals (not in const/enum declarations)
    pattern = r'(?<!const\s)(?<!enum\s)\b\d{3,}\b'

    violations = []
    matches = grep(pattern, source_path)

    for match in matches:
        # Filter out acceptable cases
        if is_in_const_declaration(match) or is_in_test(match.file):
            continue

        violations.append({
            "severity": "LOW",
            "category": "Magic Number",
            "file": match.file,
            "line": match.line,
            "number": match.text,
            "fix": "Extract to named constant"
        })

    return violations
```

---

## Severity Assessment

### Severity Levels

| Level | Description | Examples | Action |
|-------|-------------|----------|--------|
| CRITICAL | Security/Architecture violations | SQL injection, layer violations | Block immediately |
| HIGH | Spec/Design violations | API mismatch, structure violations | Block release |
| MEDIUM | Maintainability issues | High complexity, code smells | Address soon |
| LOW | Code quality | Documentation, formatting | Technical debt |

### Severity Decision Tree

```python
def assess_severity(violation):
    # Security issues are always CRITICAL
    if violation.category in ["SQL Injection", "XSS", "Hardcoded Secret"]:
        return "CRITICAL"

    # Architecture violations are CRITICAL
    if violation.category == "Cross-Layer Dependency":
        return "CRITICAL"

    # Library substitution is CRITICAL
    if violation.category == "Library Substitution":
        return "CRITICAL"

    # Structure violations are HIGH
    if violation.category == "Structure Violation":
        return "HIGH"

    # Complexity/maintainability is MEDIUM
    if violation.category in ["High Complexity", "God Object", "Long Method"]:
        return "MEDIUM"

    # Code quality is LOW
    return "LOW"
```

---

## False Positive Handling

### Context-Aware Assessment

```python
def reduce_false_positives(violations, context):
    filtered = []

    for violation in violations:
        # Check if violation is in test code
        if is_test_file(violation.file):
            # Lower severity for test code
            if violation.severity == "MEDIUM":
                violation.severity = "LOW"

        # Check if pattern is in legacy code
        if is_legacy_code(violation.file, context):
            # Document as accepted technical debt
            violation.note = "Legacy code - accepted technical debt"

        # Check if pattern has documented exception
        if has_documented_exception(violation, context):
            continue  # Skip this violation

        filtered.append(violation)

    return filtered
```

### When to Use AskUserQuestion

```python
def handle_ambiguous_violation(violation):
    # Pattern: Method complexity slightly over threshold
    if violation.category == "High Complexity" and violation.complexity <= 12:
        # Ask user if this is acceptable
        return ask_user_question(
            "Method has complexity 12 (threshold 10) but is well-tested. Accept?",
            options=["Yes, accept", "No, refactor required"]
        )

    # Pattern: Large method with good reason
    if violation.category == "Long Method" and violation.is_cohesive:
        return ask_user_question(
            "Method is 150 lines but cohesive (single responsibility). Accept?",
            options=["Yes, acceptable", "No, extract methods"]
        )
```

---

## Error Handling

**Scenario 1: Context file missing**
```
{
  "status": "failure",
  "error": "Required context file not found: devforgeai/specs/context/tech-stack.md",
  "blocks_qa": true,
  "remediation": "Run /create-context to generate context files"
}

-> Display error, set blocks_qa=true, HALT
```

**Scenario 2: Contradictory rules**
```
{
  "status": "failure",
  "error": "Context files contradictory: tech-stack.md locks Dapper, dependencies.md lists EF",
  "blocks_qa": true,
  "remediation": "Resolve contradiction - update tech-stack.md or dependencies.md"
}

-> Display error, set blocks_qa=true, HALT
```

---

## Quick Reference

### Detection Commands

```bash
# SQL Injection
grep -r "ExecuteRawSql(.*+" --include="*.cs"

# Hardcoded Secrets
grep -ri "password\s*=\s*[\"']" src/

# Cross-layer violations
grep -r "using.*Infrastructure" src/Domain/

# Magic numbers
grep -rE "\s\d{3,}\s" src/ --exclude-dir=tests

# God objects
find src/ -name "*.cs" -exec wc -l {} \; | awk '$1 > 500'
```

### Common Patterns

| Anti-Pattern | Grep Pattern | Severity |
|--------------|--------------|----------|
| SQL Injection | `string.Format.*SELECT` | CRITICAL |
| Hardcoded Secret | `password\s*=\s*"[^"]+"` | CRITICAL |
| Layer Violation | `using.*Infrastructure` in Domain | CRITICAL |
| Magic Number | `\b\d{3,}\b` (not in const) | LOW |
| Long Method | Methods > 100 lines | MEDIUM |
| High Coupling | > 10 dependencies | MEDIUM |

---

## Token Efficiency

**Before (inline):** ~8K tokens
- Manual context file loading: 1K
- Inline pattern matching (6 categories): 5K
- Violation formatting: 2K

**After (subagent):** ~3K tokens
- Context file loading: 1K
- Subagent invocation prompt: 1.5K
- Response parsing: 0.5K

**Savings:** 5K tokens (73% reduction) per QA validation
**Per 100 stories:** 500K tokens saved

---

This reference should be loaded when performing comprehensive anti-pattern detection during deep QA validation.
