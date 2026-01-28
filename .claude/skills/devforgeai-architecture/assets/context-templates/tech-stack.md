---
last_updated: [DATE]
status: LOCKED
requires_approval: [ROLE - e.g., Tech Lead, Architect]
project: [PROJECT_NAME]
---

# Technology Stack

**CRITICAL:** This document defines the ONLY approved technologies for this project.

**AI agents MUST NOT introduce alternatives without explicit user approval via AskUserQuestion.**

---

## Language & Runtime

**Primary Language:** [LANGUAGE] [VERSION]
- **Version:** [SPECIFIC_VERSION]
- **Rationale:** [WHY_THIS_LANGUAGE]
- **Prohibited:** [OTHER_LANGUAGES - unless explicitly requested]

**Example:**
```
Primary Language: C# 12.0
- Version: .NET 8.0 LTS
- Rationale: Team expertise (5+ years), enterprise support, performance requirements
- Prohibited: F#, VB.NET (unless explicitly requested for specific use case)
```

---

## Database

**RDBMS:** [DATABASE_NAME] [VERSION]
- **Why:** [RATIONALE]
- **Connection Library:** [LIBRARY] [VERSION]
- **Prohibited:** [ALTERNATIVE_DATABASES]

**Example:**
```
RDBMS: Microsoft SQL Server 2022
- Why: Existing enterprise license, DBA team expertise, integration with existing systems
- Connection Library: Microsoft.Data.SqlClient 5.1.2
- Prohibited: PostgreSQL, MySQL, SQLite (unless explicitly approved for specific use case)
```

### Data Access Layer (⚠️ CRITICAL - LOCKS ORM CHOICE)

**ORM/Data Access:** [ORM_NAME] [VERSION]
- **Why:** [DETAILED_RATIONALE]
- **Prohibited:** [ALTERNATIVE_ORMS_WITH_REASONS]
- **⚠️ CRITICAL RULE:** If [CHOSEN_ORM] encounters issues, DO NOT suggest alternatives. Use AskUserQuestion to resolve.

**Example:**
```
ORM/Data Access: Dapper 2.1.28
- Why:
  - Performance: 10x faster than EF Core for read-heavy workload
  - Control: Explicit SQL for complex query optimization
  - Team expertise: 5+ years Dapper experience
  - Simplicity: Minimal abstraction, easier debugging
- Prohibited:
  - ❌ Entity Framework Core (performance overhead)
  - ❌ NHibernate (complexity, learning curve)
  - ❌ Raw ADO.NET (excessive boilerplate)

⚠️ CRITICAL RULE:
If Dapper connection fails or query issues arise, DO NOT suggest Entity Framework or other ORMs.
Use AskUserQuestion to debug the Dapper issue, never substitute the ORM.

Rationale: Maintaining consistent data access patterns is more important than working around
temporary difficulties. ORM substitution creates technical debt.
```

### Migration Tool

**Database Migrations:** [MIGRATION_TOOL] [VERSION]
- **Why:** [RATIONALE]
- **Prohibited:** [ALTERNATIVES]

**Example:**
```
Database Migrations: DbUp 5.0.37
- Why: SQL-file based migrations, version control friendly, works well with Dapper
- Prohibited: Entity Framework Migrations (we're not using EF), Fluent Migrator
```

---

## Frontend

**Framework:** [FRAMEWORK] [VERSION] with [LANGUAGE] [VERSION]
- **Why:** [RATIONALE]
- **Build Tool:** [BUILD_TOOL] [VERSION]
- **Prohibited:** [ALTERNATIVE_FRAMEWORKS]

**Example:**
```
Framework: React 18.x with TypeScript 5.x
- Why: Team expertise, ecosystem maturity, component reusability, strong typing
- Build Tool: Vite 5.x (fast builds, modern ESM)
- Prohibited: Vue.js, Angular, Svelte (unless explicitly approved)
```

### State Management (⚠️ CRITICAL - PREVENTS FRAMEWORK MIXING)

**State Management:** [LIBRARY] [VERSION]
- **Why:** [DETAILED_RATIONALE]
- **Prohibited:** [ALTERNATIVES_WITH_REASONS]
- **⚠️ CRITICAL RULE:** Do NOT mix state management patterns. All state uses [CHOSEN_LIBRARY].

**Example:**
```
State Management: Zustand 4.4.x
- Why:
  - Simplicity: Minimal boilerplate vs Redux
  - Performance: No Context API re-render issues
  - Team preference: Developers prefer its API
  - Bundle size: 2KB vs 12KB (Redux Toolkit)
- Prohibited:
  - ❌ Redux / Redux Toolkit (too much boilerplate for our needs)
  - ❌ React Context API (performance issues at scale)
  - ❌ Jotai, Recoil, MobX (team unfamiliar, adds learning curve)

⚠️ CRITICAL RULE:
Do NOT introduce Redux patterns, action types, reducers, etc. in this project.
All state management uses Zustand stores exclusively.

If Zustand limitations encountered, use AskUserQuestion before introducing alternatives.
```

### Styling (⚠️ CRITICAL - PREVENTS CSS-IN-JS MIXING)

**Styling:** [STYLING_APPROACH] [VERSION]
- **Why:** [RATIONALE]
- **Prohibited:** [ALTERNATIVES]

**Example:**
```
Styling: Tailwind CSS 3.4.x
- Why: Utility-first, consistent design system, rapid development, small bundle
- Prohibited:
  - ❌ styled-components (runtime overhead)
  - ❌ Emotion (runtime overhead)
  - ❌ CSS Modules (prefer utility-first approach)
  - ❌ Sass/Less (Tailwind provides all needed features)

⚠️ CRITICAL RULE:
Do NOT mix Tailwind with CSS-in-JS libraries. All styling uses Tailwind utility classes.
```

---

## API Layer

**API Framework:** [FRAMEWORK] [VERSION]
- **Why:** [RATIONALE]
- **Prohibited:** [ALTERNATIVES]

**Example:**
```
API Framework: ASP.NET Core 8.0 Minimal APIs
- Why: Performance, simplicity, modern C# features, less boilerplate than controllers
- Prohibited:
  - ❌ Controller-based APIs (prefer Minimal APIs for this project)
  - ❌ GraphQL (not needed for our use cases)
  - ❌ gRPC (REST is sufficient)
  - ❌ SOAP (legacy technology)
```

### Serialization

**JSON Serialization:** [LIBRARY] [VERSION]
- **Why:** [RATIONALE]
- **Prohibited:** [ALTERNATIVES]

**Example:**
```
JSON Serialization: System.Text.Json (built-in)
- Why: Performance, built-in to .NET, modern JSON features, source generators
- Prohibited:
  - ❌ Newtonsoft.Json (slower, legacy, adds dependency)

Note: Only add Newtonsoft.Json if absolutely required for compatibility with external libraries.
Must use AskUserQuestion before adding.
```

### Validation

**Input Validation:** [LIBRARY] [VERSION]
- **Why:** [RATIONALE]
- **Prohibited:** [ALTERNATIVES]

**Example:**
```
Input Validation: FluentValidation 11.8.x
- Why: Expressive syntax, testable validators, separation of concerns
- Prohibited:
  - ❌ Data Annotations (limited flexibility, harder to test)
  - ❌ Manual validation (error-prone, inconsistent)
```

### API Documentation

**API Documentation:** [TOOL] [VERSION]
- **Why:** [RATIONALE]

**Example:**
```
API Documentation: Swashbuckle 6.5.x (OpenAPI 3.0)
- Why: Standard OpenAPI spec, interactive Swagger UI, code generation support
```

---

## Testing

### Unit Testing (⚠️ CRITICAL - PREVENTS FRAMEWORK MIXING)

**Unit Test Framework:** [FRAMEWORK] [VERSION]
- **Why:** [RATIONALE]
- **Mocking Library:** [LIBRARY] [VERSION]
- **Assertion Library:** [LIBRARY] [VERSION]
- **Prohibited:** [ALTERNATIVES]

**Example:**
```
Unit Test Framework: xUnit 2.6.x
- Why: Modern, extensible, parallel test execution, preferred by team
- Mocking Library: NSubstitute 5.1.x
  - Why: Clean syntax, easier than Moq for our use cases
- Assertion Library: FluentAssertions 6.12.x
  - Why: Readable assertions, better error messages
- Prohibited:
  - ❌ NUnit, MSTest (prefer xUnit for consistency)
  - ❌ Moq (prefer NSubstitute's syntax)

⚠️ CRITICAL RULE:
Do NOT mix test frameworks. All tests use xUnit + NSubstitute + FluentAssertions.
```

### Integration Testing

**Integration Test Framework:** [FRAMEWORK] [VERSION]
- **Test Infrastructure:** [TOOLS]

**Example:**
```
Integration Test Framework: xUnit 2.6.x (same as unit tests)
- Test Infrastructure:
  - Testcontainers.MsSql 3.6.0 (real SQL Server in Docker)
  - Microsoft.AspNetCore.Mvc.Testing 8.0.0 (in-memory API testing)
- Why: Real database for accurate integration tests, easy CI/CD integration
```

### End-to-End Testing

**E2E Test Framework:** [FRAMEWORK] [VERSION]
- **Why:** [RATIONALE]
- **Prohibited:** [ALTERNATIVES]

**Example:**
```
E2E Test Framework: Playwright 1.40.x with TypeScript
- Why: Cross-browser, reliable, fast, built-in video recording, better than Selenium
- Prohibited:
  - ❌ Selenium (slower, less reliable)
  - ❌ Cypress (browser limitations)
  - ❌ Puppeteer (Chromium-only)
```

### Test Coverage

**Coverage Target:** [PERCENTAGE]%
- **Tool:** [COVERAGE_TOOL]
- **Minimum Required:** [MINIMUM]%

**Example:**
```
Coverage Target: 90%
- Tool: coverlet (built-in .NET coverage)
- Minimum Required: 80% (CI/CD fails below this)
```

---

## Additional Technologies

### Logging

**Logging Framework:** [FRAMEWORK] [VERSION]
- **Why:** [RATIONALE]

**Example:**
```
Logging Framework: Serilog 3.x
- Why: Structured logging, multiple sinks, excellent ASP.NET Core integration
- Sinks: Console, File, Application Insights
```

### Dependency Injection

**DI Container:** [CONTAINER] [VERSION]
- **Why:** [RATIONALE]

**Example:**
```
DI Container: Microsoft.Extensions.DependencyInjection (built-in)
- Why: Built-in, sufficient for our needs, no additional dependencies
- Prohibited: Autofac, Ninject (built-in DI is adequate)
```

### Background Jobs

**Background Processing:** [LIBRARY] [VERSION]
- **Why:** [RATIONALE]

**Example:**
```
Background Processing: Hangfire 1.8.x
- Why: Reliable, persistent jobs, dashboard, SQL Server storage
- Prohibited: Quartz.NET (more complex than needed)
```

---

## Ambiguity Resolution Protocol

**CRITICAL:** When encountering ANY ambiguity, AI agents MUST follow this protocol:

### Step 1: Check This File

Before adding any technology, library, or making any architectural decision:
1. Search this file for relevant technology category
2. If documented → Use ONLY the specified technology
3. If not documented → Proceed to Step 2

### Step 2: Check Existing Project

For brownfield projects:
1. Search codebase for similar functionality
2. If similar technology exists → Use that for consistency
3. If no similar technology → Proceed to Step 3

### Step 3: Use AskUserQuestion

**Never assume.** Always ask:

```
Question: "This project requires [functionality]. The tech-stack.md doesn't
          specify a technology choice. Which should I use?"
Header: "[Category]"
Options: [2-4 relevant technology options]
Description: "This decision will be locked in tech-stack.md after selection."
multiSelect: false
```

### Step 4: Update This File

After user confirms choice:
1. Update this file with the new technology
2. Add rationale for the choice
3. Document prohibited alternatives
4. Create ADR documenting the decision

---

## Change Control

This file is LOCKED. Changes require explicit approval.

### To Modify This Tech Stack

1. **Create ADR** documenting:
   - Why change is needed
   - What will be changed
   - Impact analysis (effort, risk, benefits)
   - Migration strategy (for brownfield)

2. **Get Approval** from:
   - [APPROVAL_ROLE_1] (e.g., Tech Lead)
   - [APPROVAL_ROLE_2] (e.g., Architect)

3. **Update This File** with:
   - New technology specification
   - Updated rationale
   - Change log entry below

4. **Notify Team:**
   - Update developers
   - Update AI agent configurations
   - Update CI/CD pipelines

### Version Policy

**Patch updates (X.Y.Z → X.Y.Z+1):** Allowed automatically
**Minor updates (X.Y → X.Y+1):** Requires team review
**Major updates (X → X+1):** Requires ADR + approval

---

## Common AI Mistakes to Prevent

### ❌ Mistake 1: Swapping ORM Due to "Connection Issues"

**Scenario:** Dapper query fails during development

**WRONG:**
```
AI: "Dapper is having issues. Let me switch to Entity Framework Core."
# Proceeds to add EF Core packages
# Creates DbContext
```

**CORRECT:**
```
AI: "Dapper query failed. tech-stack.md locks data access to Dapper.
     Let me use AskUserQuestion to resolve this issue."

AskUserQuestion:
  Question: "Dapper query is failing with error: [X]. How should I proceed?"
  Options:
    - "Debug the Dapper query (maintain tech stack)"
    - "Try alternative Dapper approach"
    - "Use stored procedure for this query"
    - "Reconsider ORM (requires ADR and approval)"
```

### ❌ Mistake 2: Adding "Better" Libraries Without Permission

**Scenario:** AI finds a library it perceives as "better"

**WRONG:**
```
AI: "Newtonsoft.Json has more features than System.Text.Json.
     Let me add it."
# Adds Newtonsoft.Json
```

**CORRECT:**
```
AI: "System.Text.Json is approved. It lacks feature [X] that I need.
     Should I use a workaround or request adding Newtonsoft.Json?"

AskUserQuestion:
  Question: "Need JSON feature [X]. System.Text.Json doesn't support it. Should I:"
  Options:
    - "Find workaround with System.Text.Json"
    - "Add Newtonsoft.Json (requires updating tech-stack.md)"
    - "Implement custom solution"
```

### ❌ Mistake 3: Mixing State Management Frameworks

**Scenario:** Feature seems easier with Redux

**WRONG:**
```
AI: "This feature would be easier with Redux.
     Let me add Redux Toolkit."
# Mixes Zustand and Redux in same project
```

**CORRECT:**
```
AI: "tech-stack.md locks state management to Zustand.
     This feature is complex. Should I:"

AskUserQuestion:
  Question: "Feature [X] is complex in Zustand. Should I:"
  Options:
    - "Implement in Zustand (maintain tech stack)"
    - "Refactor approach to work better with Zustand"
    - "Reconsider state management (requires ADR)"
```

---

## Related Documents

- [dependencies.md](./../dependencies.md) - Approved package list with versions
- [architecture-constraints.md](./../architecture-constraints.md) - Layer boundaries and patterns
- [anti-patterns.md](./../anti-patterns.md) - Forbidden approaches
- [docs/architecture/decisions/](./../../docs/architecture/decisions/) - All ADRs

---

## Change Log

| Date | Change | Approver | Reason | ADR |
|------|--------|----------|---------|-----|
| [DATE] | Initial version | [NAME] | Project setup | - |
| | | | | |

---

## Notes

[Any additional context, temporary exceptions, or important considerations that don't fit above sections]

**Example:**
> Temporary Exception: During migration from EF Core to Dapper, both ORMs may coexist for up to 3 months. All NEW code must use Dapper. See ADR-005 for migration plan.
