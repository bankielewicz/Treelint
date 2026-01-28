# Phase 3: Structure Violations Detection

**Purpose:** Detect files in wrong layers or unexpected directories
**Severity:** HIGH (blocks QA)
**Duration:** <3 seconds

---

## Check 1: Files in Wrong Layer

**Given source-tree.md defines:**
```
Domain/: Entities, Value Objects, Domain Services (no infrastructure)
Application/: Use Cases, DTOs, Application Services
Infrastructure/: Repositories, External Services, Data Access
Presentation/: Controllers, Views, UI Components
```

**Violation:** EmailService in Domain instead of Infrastructure

**Detection:**
```
layer_definitions = parse_source_tree_md()

FOR each file in codebase:
  file_layer = determine_layer_from_path(file)
  file_type = analyze_imports_and_dependencies(file)

  IF file_layer == "Domain" AND file_type contains infrastructure_concerns:
    violations.append({
      "file": file.path,
      "line": evidence_line,
      "pattern": "Infrastructure concern in Domain layer",
      "evidence": "using System.Data.SqlClient;",
      "remediation": f"Move {file.name} to Infrastructure/Services/"
    })
```

---

## Check 2: Unexpected Directories

**Violation:** src/Domain/Utilities/ when source-tree.md doesn't allow Utilities in Domain

**Detection:**
```
allowed_subdirs = get_allowed_subdirs_for_layer("Domain")  # [Entities, ValueObjects, Services]

actual_subdirs = get_directories_in_layer("src/Domain/")

FOR subdir in actual_subdirs:
  IF subdir NOT IN allowed_subdirs:
    violations.append({
      "file": f"src/Domain/{subdir}/",
      "pattern": "Unexpected directory in layer",
      "remediation": f"Remove {subdir} or update source-tree.md to allow it in Domain layer"
    })
```

---

## Check 3: Infrastructure Concerns in Domain

**Infrastructure indicators:**
- Database: `DbContext`, `SqlConnection`, `SELECT`, `INSERT`
- HTTP: `HttpClient`, `RestClient`, `axios`
- File I/O: `File.Read`, `fs.readFile`, `open()`
- External APIs: Third-party SDK imports

**Detection:**
```
FOR each file in Domain/:
  content = Read(file)

  IF content matches infrastructure_patterns:
    violations.append({
      "severity": "HIGH",
      "pattern": "Infrastructure concern in Domain layer",
      "evidence": matched_line
    })
```

---

## Output

```json
{
  "violations_found": 2,
  "violations": [
    {
      "file": "src/Domain/Services/EmailService.cs",
      "line": 12,
      "pattern": "Infrastructure concern in Domain layer",
      "evidence": "private readonly IEmailClient _emailClient;",
      "remediation": "Move EmailService to Infrastructure/Services/. Domain should only define IEmailService interface.",
      "severity": "HIGH"
    }
  ]
}
```
