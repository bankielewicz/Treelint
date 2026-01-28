# Phase 5: Code Smells Detection

**Purpose:** Detect technical debt indicators (god objects, long methods, magic numbers)
**Severity:** MEDIUM (warning only, does NOT block QA)
**Duration:** <5 seconds

---

## Check 1: God Objects (>15 methods or >300 lines)

**Detection:**
```
FOR each class in codebase:
  method_count = count_methods(class)
  line_count = count_lines(class)

  IF method_count > 15 OR line_count > 300:
    violations.append({
      "file": class.file,
      "line": class.start_line,
      "pattern": "God object",
      "evidence": f"{class.name} has {method_count} methods, {line_count} lines",
      "remediation": "Refactor into smaller classes following Single Responsibility Principle",
      "severity": "MEDIUM"
    })
```

---

## Check 2: Long Methods (>50 lines)

**Detection:**
```
FOR each method in codebase:
  line_count = count_lines_in_method(method)

  IF line_count > 50:
    violations.append({
      "pattern": "Long method",
      "evidence": f"{method.name} is {line_count} lines",
      "remediation": "Extract helper methods to reduce complexity",
      "severity": "MEDIUM"
    })
```

---

## Check 3: Magic Numbers

**Detection:**
```
FOR each file in codebase:
  numbers = Grep(pattern="\\b[2-9]\\d+\\b", file=file)  # Numbers >= 20

  FOR each match in numbers:
    IF NOT is_configuration(match) AND NOT is_test_data(match):
      violations.append({
        "pattern": "Magic number",
        "evidence": match.line,
        "remediation": "Extract to named constant",
        "severity": "MEDIUM"
      })
```

**Exceptions:** 0, 1, -1, 10, 100, 1000 (common values)

---

## Important: NEVER Block QA

```
blocks_qa = False  # Code smells are warnings only
```
