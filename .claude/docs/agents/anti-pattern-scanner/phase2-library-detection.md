# Phase 2: Library Substitution Detection - Detailed Workflow

**Purpose:** Detect library substitutions that violate tech-stack.md locked technologies

**Severity:** CRITICAL (always blocks QA)
**Duration:** <5 seconds

---

## Technology Types to Check

### 1. ORM (Object-Relational Mapping)

**Common ORMs by language:**
- **C#/.NET:** Entity Framework, Dapper, NHibernate
- **Python:** SQLAlchemy, Django ORM, Peewee
- **JavaScript/TypeScript:** Prisma, TypeORM, Sequelize
- **Java:** Hibernate, MyBatis, JPA
- **Ruby:** ActiveRecord, Sequel
- **Go:** GORM, sqlx

**Detection pattern:**
```
locked_orm = extract_from_tech_stack("ORM")  # e.g., "Dapper"

alternative_orms = ["Entity Framework", "NHibernate"] if locked_orm == "Dapper"

FOR each alternative in alternative_orms:
  search_results = Grep(pattern=alternative, glob="**/*.cs")

  IF search_results found:
    violations.append({
      "file": result.file,
      "line": result.line,
      "pattern": "ORM substitution",
      "evidence": result.match_line,
      "remediation": f"Replace {alternative} with {locked_orm}. See tech-stack.md line X",
      "severity": "CRITICAL",
      "category": "library_substitution",
      "locked_technology": locked_orm,
      "detected_technology": alternative
    })
```

---

### 2. State Manager

**Common state managers:**
- **React:** Zustand, Redux, MobX, Recoil
- **Vue:** Pinia, Vuex
- **Angular:** NgRx, Akita

**Detection pattern:**
```
locked_state = extract_from_tech_stack("State Management")

IF locked_state == "Zustand":
  alternatives = ["Redux", "MobX", "Recoil"]

FOR each alternative:
  Grep(pattern=f"import .* from '{alternative.lower()}'", glob="**/*.{ts,tsx,js,jsx}")
```

---

### 3. HTTP Client

**Common HTTP clients:**
- **JavaScript/TypeScript:** axios, fetch, superagent
- **C#/.NET:** HttpClient, RestSharp, Flurl
- **Python:** requests, httpx, aiohttp
- **Java:** Apache HttpClient, OkHttp

**Detection pattern:**
```
locked_http = extract_from_tech_stack("HTTP Client")

IF locked_http == "axios":
  # Check for fetch() calls (browser native)
  Grep(pattern="fetch\\(", glob="**/*.{ts,tsx,js,jsx}")

  # Check for superagent imports
  Grep(pattern="import .* from 'superagent'", glob="**/*.{ts,tsx,js,jsx}")
```

---

### 4. Validation Library

**Common validation libraries:**
- **JavaScript/TypeScript:** Zod, Joi, Yup, class-validator
- **C#/.NET:** FluentValidation, DataAnnotations
- **Python:** Pydantic, marshmallow, cerberus

**Detection pattern:**
```
locked_validation = extract_from_tech_stack("Validation")

IF locked_validation == "Zod":
  alternatives = ["Joi", "Yup", "class-validator"]

  FOR each alternative:
    Grep(pattern=f"import .* from '{alternative.lower()}'")
```

---

### 5. Testing Framework

**Common testing frameworks:**
- **JavaScript/TypeScript:** Vitest, Jest, Mocha, Jasmine
- **C#/.NET:** xUnit, NUnit, MSTest
- **Python:** pytest, unittest, nose
- **Java:** JUnit, TestNG
- **Go:** testing (built-in), Ginkgo
- **Ruby:** RSpec, Minitest

**Detection pattern:**
```
locked_testing = extract_from_tech_stack("Testing Framework")

IF locked_testing == "Vitest":
  alternatives = ["Jest", "Mocha", "Jasmine"]

  FOR each alternative:
    Grep(pattern=f"from '{alternative.lower()}'", glob="**/*.test.{ts,tsx,js,jsx}")
```

---

## Output Structure

**Violation object for library substitution:**

```json
{
  "file": "/absolute/path/to/UserRepository.cs",
  "line": 1,
  "pattern": "ORM substitution",
  "evidence": "using Microsoft.EntityFrameworkCore;",
  "remediation": "Replace Entity Framework Core with Dapper. See tech-stack.md: 'ORM: Dapper (locked)'",
  "severity": "CRITICAL",
  "category": "library_substitution",
  "locked_technology": "Dapper",
  "detected_technology": "Entity Framework Core",
  "tech_stack_reference": "tech-stack.md line 45"
}
```

---

## Special Cases

**Case 1: Multiple ORMs (migration scenario)**
- If both locked ORM AND alternative found → Check dependencies.md
- If alternative in dependencies.md → Migration in progress (warning, not error)
- If alternative NOT in dependencies → Violation (CRITICAL)

**Case 2: Native vs Library (HTTP client)**
- `fetch()` is native JavaScript, not a library substitution
- Only flag if tech-stack.md explicitly locks a library and native is prohibited

**Case 3: Test utilities vs frameworks**
- Testing utilities (e.g., testing-library) are NOT frameworks
- Only flag framework substitutions (Jest → Vitest, etc.)

---

## Return Format

```json
{
  "violations_found": 3,
  "violations": [
    {...},  // ORM substitution
    {...},  // State manager substitution
    {...}   // HTTP client substitution
  ],
  "categories_checked": ["ORM", "State Management", "HTTP Client", "Validation", "Testing"],
  "execution_time_ms": 2500
}
```
