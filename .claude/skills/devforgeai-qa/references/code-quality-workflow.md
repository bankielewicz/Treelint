# Phase 4: Code Quality Metrics Workflow

Execute 5-step code quality analysis.

---

## Overview

Analyzes code quality through cyclomatic complexity, maintainability index, code duplication, documentation coverage, and dependency coupling.

## References Used

This workflow references:
- **quality-metrics.md** - Metric thresholds, formulas, interpretation guidelines (`.claude/skills/devforgeai-qa/assets/config/quality-metrics.md`)

---

## Step 1: Analyze Cyclomatic Complexity

**Threshold:** Complexity > 10 per method triggers MEDIUM violation

```
# Use language-specific tools
IF language == "Python":
    Bash(command="radon cc src/ -a -j > complexity.json")
    Read(file_path="complexity.json")

    # Parse results
    FOR method WITH complexity > 10:
        violations.append({
            "type": "High cyclomatic complexity",
            "severity": "MEDIUM",
            "method": method.name,
            "file": method.file,
            "complexity": method.complexity,
            "threshold": 10,
            "message": f"High complexity: {method.name} ({method.complexity})",
            "remediation": "Refactor to reduce complexity (extract methods, simplify logic)"
        })

IF language == "Node.js":
    Bash(command="npx complexity-report src/ --format json > complexity.json")
    Read(file_path="complexity.json")
    # Parse and validate

IF language == ".NET":
    # Use VS Code Metrics or similar tool
    Bash(command="dotnet tool run metrics src/ --format json > complexity.json")
    Read(file_path="complexity.json")
    # Parse and validate
```

**Tools and thresholds:** `quality-metrics.md` section "Cyclomatic Complexity"

**Complexity scale:**
- 1-5: Low (simple methods) ✅
- 6-10: Moderate (acceptable) ✅
- 11-20: High (refactor recommended) ⚠️ MEDIUM
- 21+: Very high (refactor required) ⚠️ HIGH

---

## Step 2: Calculate Maintainability Index

**Threshold:** Maintainability Index < 70 triggers MEDIUM violation

```
IF language == "Python":
    Bash(command="radon mi src/ -s -j > maintainability.json")
    Read(file_path="maintainability.json")

    # Parse results
    FOR file WITH maintainability_index < 70:
        violations.append({
            "type": "Low maintainability",
            "severity": "MEDIUM",
            "file": file.name,
            "maintainability_index": file.mi,
            "threshold": 70,
            "message": f"Low maintainability: {file.name} ({file.mi})",
            "remediation": "Improve code structure, reduce complexity, add documentation"
        })

IF language == ".NET":
    # Use Code Metrics tools
    Bash(command="dotnet tool run metrics src/ --maintainability > maintainability.txt")
    Read(file_path="maintainability.txt")
    # Parse and validate
```

**Formula and thresholds:** `quality-metrics.md` section "Maintainability Index"

**Maintainability Index scale:**
- 85-100: Highly maintainable ✅
- 70-85: Moderately maintainable ✅
- 50-70: Low maintainability ⚠️ MEDIUM
- 0-50: Very low maintainability ⚠️ HIGH

---

## Step 3: Detect Code Duplication

**Threshold:** Duplication > 5% triggers LOW violation

```
Bash(command="jscpd src/ --format json --output duplication.json")
Read(file_path="duplication.json")

duplication_data = parse_json(duplication_content)
duplication_percentage = calculate_duplication(duplication_data)

IF duplication_percentage > 5%:
    violations.append({
        "type": "Code duplication",
        "severity": "LOW",
        "percentage": duplication_percentage,
        "threshold": "5%",
        "duplicates": duplication_data.duplicates,
        "message": f"Code duplication {duplication_percentage}% (threshold 5%)",
        "remediation": "Extract duplicated code into shared functions/classes"
    })

# High duplication severity escalation
IF duplication_percentage > 10%:
    violations[-1]["severity"] = "MEDIUM"

IF duplication_percentage > 20%:
    violations[-1]["severity"] = "HIGH"
```

**Thresholds:** `quality-metrics.md` section "Code Duplication"

**Acceptable duplication:**
- <3%: Excellent ✅
- 3-5%: Good ✅
- 5-10%: Acceptable ⚠️ LOW
- 10-20%: High ⚠️ MEDIUM
- >20%: Very high ⚠️ HIGH

---

## Step 4: Measure Documentation Coverage

**Threshold:** Documentation < 80% triggers LOW violation

```
# Count documented vs undocumented public APIs
public_apis = Grep(pattern="^\\s*public", path="src/", output_mode="count")

# Count documentation comments based on language
IF language == ".NET":
    docs = Grep(pattern="^\\s*///", path="src/", output_mode="count")

IF language == "Python":
    docs = Grep(pattern="^\\s*\"\"\"", path="src/", output_mode="count")

IF language == "Node.js":
    docs = Grep(pattern="^\\s*/\\*\\*", path="src/", output_mode="count")

documentation_coverage = (docs / public_apis) * 100

IF documentation_coverage < 80%:
    violations.append({
        "type": "Low documentation coverage",
        "severity": "LOW",
        "percentage": documentation_coverage,
        "threshold": "80%",
        "documented": docs,
        "total_apis": public_apis,
        "message": f"Documentation coverage {documentation_coverage}% (target 80%)",
        "remediation": "Add XML doc comments for public APIs"
    })

# Critical documentation gap
IF documentation_coverage < 50%:
    violations[-1]["severity"] = "MEDIUM"
```

**Thresholds:** `quality-metrics.md` section "Documentation Coverage"

**Documentation standards:**
- Public APIs: REQUIRED (80%+)
- Internal APIs: RECOMMENDED (50%+)
- Private methods: OPTIONAL

---

## Step 5: Analyze Dependency Coupling

### Circular Dependencies Detection

```
# Detect circular dependencies
IF language == "Node.js":
    Bash(command="npx madge --circular src/ --json > circular.json")
    Read(file_path="circular.json")

    circular_dependencies = parse_json(circular_content)

    IF circular_dependencies.length > 0:
        violations.append({
            "type": "Circular dependencies",
            "severity": "MEDIUM",
            "count": circular_dependencies.length,
            "chains": circular_dependencies,
            "message": "Circular dependencies detected",
            "remediation": "Break circular dependencies using dependency inversion"
        })

IF language == "Python":
    # Use pydeps or similar
    Bash(command="pydeps src/ --show-cycles > cycles.txt")
    # Parse and validate
```

### High Coupling Detection

```
# Count dependencies per file
FOR each source_file in source_files:
    import_count = Grep(pattern="^import|^from.*import|^using",
                       path=source_file,
                       output_mode="count")

    IF import_count > 10:
        violations.append({
            "type": "High coupling",
            "severity": "MEDIUM",
            "file": source_file,
            "dependency_count": import_count,
            "threshold": 10,
            "message": f"High coupling: {source_file} ({import_count} dependencies)",
            "remediation": "Reduce dependencies through abstraction and dependency injection"
        })
```

**Tools and analysis:** `quality-metrics.md` section "Dependency Analysis"

**Coupling metrics:**
- 0-5 dependencies: Low coupling ✅
- 6-10 dependencies: Moderate coupling ✅
- 11-15 dependencies: High coupling ⚠️ MEDIUM
- 16+ dependencies: Very high coupling ⚠️ HIGH

---

## Phase 4 Output

**Results to carry forward:**
- Complexity analysis (methods >10 complexity)
- Maintainability index (files <70 MI)
- Duplication percentage
- Documentation coverage percentage
- Coupling analysis (circular deps, high coupling)
- Violations by severity

**Violations typically MEDIUM/LOW:**
- Code quality violations don't usually block QA
- Document in report for follow-up improvement
- HIGH violations possible for extreme cases (duplication >20%, MI <50)

**Continue to Phase 5 (Generate QA Report) after Phase 4 completes.**
