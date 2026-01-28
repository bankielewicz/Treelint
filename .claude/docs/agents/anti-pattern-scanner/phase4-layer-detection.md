# Phase 4: Layer Boundary Violations Detection

**Purpose:** Detect cross-layer dependencies violating architecture-constraints.md
**Severity:** HIGH (blocks QA)
**Duration:** <3 seconds

---

## Dependency Rules (Clean Architecture)

```
Domain → No dependencies (pure domain logic)
Application → Domain only
Infrastructure → Application + Domain
Presentation → Application (NOT Infrastructure directly)
```

---

## Check 1: Domain Referencing Application/Infrastructure

**Violation:** Domain imports from Application

**Detection:**
```
FOR each file in Domain/:
  imports = extract_imports(file)

  IF any import from "Application/" OR "Infrastructure/":
    violations.append({
      "file": file.path,
      "line": import_line,
      "pattern": "Domain layer cannot reference Application or Infrastructure",
      "evidence": import_statement,
      "remediation": "Use dependency inversion - define interface in Domain, implement in Infrastructure",
      "severity": "HIGH"
    })
```

---

## Check 2: Circular Dependencies

**Violation:** Application → Infrastructure → Application

**Detection:**
```
dependency_graph = build_dependency_graph()

circles = detect_cycles(dependency_graph)

FOR each circle in circles:
  violations.append({
    "pattern": "Circular dependency detected",
    "evidence": " → ".join(circle),
    "remediation": "Break cycle using dependency inversion or refactor into single layer"
  })
```

---

## Output

```json
{
  "violations": [
    {
      "file": "src/Domain/Entities/Order.cs",
      "line": 3,
      "pattern": "Domain layer cannot reference Application",
      "evidence": "using Application.Services;",
      "remediation": "Define IOrderService interface in Domain, implement in Application",
      "severity": "HIGH"
    }
  ]
}
```
