# Phase 7: Style Inconsistencies Detection

**Purpose:** Detect documentation and naming convention violations
**Severity:** LOW (advisory only, does NOT block QA)
**Duration:** <2 seconds

---

## Check 1: Missing Documentation

**Detection:**
```
FOR each public class, method, property:
  IF not has_documentation_comment(item):
    violations.append({
      "pattern": "Missing documentation",
      "evidence": f"Public {item.type} {item.name} lacks documentation",
      "remediation": "Add XML doc comments (C#), JSDoc (JS/TS), docstrings (Python)",
      "severity": "LOW"
    })
```

---

## Check 2: Naming Conventions

**By language:**

**C#:**
- Classes: PascalCase
- Methods: PascalCase
- Fields: _camelCase (private), PascalCase (public)

**JavaScript/TypeScript:**
- Classes: PascalCase
- Functions: camelCase
- Constants: UPPER_CASE

**Python:**
- Classes: PascalCase
- Functions: snake_case
- Constants: UPPER_CASE

**Detection:**
```
naming_violations = run_language_linter(language)

IF language == "csharp":
  Bash(command="dotnet format --verify-no-changes")
ELIF language == "javascript":
  Bash(command="eslint src/")
ELIF language == "python":
  Bash(command="pylint src/")
```

---

## Important: NEVER Block QA

```
blocks_qa = False  # Style checks are advisory only
```
