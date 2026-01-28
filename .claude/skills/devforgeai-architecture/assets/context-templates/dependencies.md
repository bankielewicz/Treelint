---
last_updated: [DATE]
status: LOCKED
auto_update: false
sync_with: tech-stack.md
project: [PROJECT_NAME]
---

# Approved Dependencies

**CRITICAL:** This file lists ONLY approved libraries and packages.

**AI agents MUST NOT add unlisted dependencies without explicit user approval.**

---

## Dependency Addition Protocol

**Before adding ANY NuGet package or npm package, AI agents MUST:**

1. **Check if package is listed** in this file
2. **If listed** → Use exact version specified
3. **If NOT listed** → STOP and use AskUserQuestion:

```
Question: "I need to add package [PackageName] for [functionality].
          It's not in dependencies.md. Should I add it?"
Header: "New package"
Options:
  - "Yes, add [PackageName] version [X.Y.Z]"
  - "No, use existing dependency [AlternativeName]"
  - "No, implement manually without external dependency"
Description: "This will update dependencies.md and require ADR documentation"
multiSelect: false
```

4. **After user approval:**
   - Update this file with new package
   - Add rationale for the choice
   - Create ADR documenting the decision
   - Notify team of new dependency

---

## .NET Backend Dependencies

### Core Framework

```xml
<PackageReference Include="Microsoft.AspNetCore.OpenApi" Version="[VERSION]" />
<PackageReference Include="Swashbuckle.AspNetCore" Version="[VERSION]" />
```

### Data Access (⚠️ CRITICAL - LOCKED TO PREVENT ORM SUBSTITUTION)

```xml
<!-- REQUIRED: Use [ORM_NAME] for data access -->
<PackageReference Include="[ORM_PACKAGE]" Version="[VERSION]" />
<PackageReference Include="Microsoft.Data.SqlClient" Version="[VERSION]" />

<!-- FORBIDDEN: Do NOT add these alternatives -->
<!-- <PackageReference Include="Microsoft.EntityFrameworkCore" Version="*" /> -->
<!-- <PackageReference Include="NHibernate" Version="*" /> -->
<!-- <PackageReference Include="Dapper" Version="*" /> -->

Rationale:
- See tech-stack.md for ORM decision
- See ADR-[XXX] for detailed reasoning
- Performance requirements: [SPECIFY]
- Team expertise: [SPECIFY]

Lock Reason: [WHY_THIS_CHOICE_IS_LOCKED]

⚠️ CRITICAL RULE:
If [ORM_NAME] encounters issues (connection failures, query problems):
- DO NOT suggest Entity Framework, NHibernate, or other ORMs
- Use AskUserQuestion to debug the [ORM_NAME] issue
- Maintain consistent data access patterns
- ORM substitution creates technical debt

Exception Process:
If truly reconsidering ORM choice:
1. Create ADR documenting issues
2. Get architect approval
3. Plan migration strategy
4. Update tech-stack.md
5. Update this file
```

### Migration Tool

```xml
<PackageReference Include="[MIGRATION_TOOL]" Version="[VERSION]" />

<!-- FORBIDDEN alternatives -->
<!-- <PackageReference Include="[ALTERNATIVE_TOOL]" Version="*" /> -->

Rationale: [WHY_THIS_MIGRATION_TOOL]
```

### Validation

```xml
<PackageReference Include="FluentValidation" Version="[VERSION]" />
<PackageReference Include="FluentValidation.DependencyInjectionExtensions" Version="[VERSION]" />

Rationale: Expressive syntax, testable validators, better than DataAnnotations
Alternative Considered: DataAnnotations (rejected: limited flexibility)
```

### Serialization

```xml
<!-- JSON Serialization -->
<!-- Using built-in System.Text.Json (no package needed) -->

<!-- FORBIDDEN: Do NOT add unless absolutely necessary -->
<!-- <PackageReference Include="Newtonsoft.Json" Version="*" /> -->

Rationale: System.Text.Json is built-in, faster, modern
Exception: Only add Newtonsoft.Json if required for compatibility with legacy libraries
Process: Must use AskUserQuestion before adding
```

### Testing

```xml
<!-- Unit Testing Framework -->
<PackageReference Include="xUnit" Version="[VERSION]" />
<PackageReference Include="xUnit.runner.visualstudio" Version="[VERSION]" />

<!-- FORBIDDEN: Do NOT mix test frameworks -->
<!-- <PackageReference Include="NUnit" Version="*" /> -->
<!-- <PackageReference Include="MSTest.TestFramework" Version="*" /> -->

Rationale: xUnit is modern, extensible, supports parallel execution
Lock Reason: Consistency - all tests use same framework

<!-- Mocking Library -->
<PackageReference Include="NSubstitute" Version="[VERSION]" />

<!-- FORBIDDEN alternatives -->
<!-- <PackageReference Include="Moq" Version="*" /> -->
<!-- <PackageReference Include="FakeItEasy" Version="*" /> -->

Rationale: NSubstitute has cleaner syntax than Moq for our use cases

<!-- Assertion Library -->
<PackageReference Include="FluentAssertions" Version="[VERSION]" />

Rationale: Readable assertions, better error messages than xUnit's Assert

<!-- Integration Testing -->
<PackageReference Include="Testcontainers.MsSql" Version="[VERSION]" />
<PackageReference Include="Microsoft.AspNetCore.Mvc.Testing" Version="[VERSION]" />

Rationale: Real database for accurate integration tests
```

### Logging

```xml
<PackageReference Include="Serilog" Version="[VERSION]" />
<PackageReference Include="Serilog.AspNetCore" Version="[VERSION]" />
<PackageReference Include="Serilog.Sinks.Console" Version="[VERSION]" />
<PackageReference Include="Serilog.Sinks.File" Version="[VERSION]" />

Rationale: Structured logging, multiple sinks, excellent ASP.NET Core integration
```

### Background Jobs (if applicable)

```xml
<!-- Only add if background processing is required -->
<!-- <PackageReference Include="Hangfire" Version="[VERSION]" /> -->

Rationale: [SPECIFY_IF_NEEDED]
Alternative Considered: Quartz.NET (rejected: [REASON])
```

---

## Frontend Dependencies (npm/package.json)

### Core Framework

```json
{
  "react": "^[VERSION]",
  "react-dom": "^[VERSION]",
  "typescript": "^[VERSION]"
}

// FORBIDDEN versions
// "react": "^17.x"  // Too old, missing features we need
// "react": "^19.x"  // Too new, not stable yet

Rationale: React 18 with TypeScript for type safety
```

### State Management (⚠️ CRITICAL - LOCKED TO PREVENT FRAMEWORK MIXING)

```json
{
  "zustand": "^[VERSION]"
}

// FORBIDDEN: Do NOT add these alternatives
// "redux": "*"
// "@reduxjs/toolkit": "*"
// "jotai": "*"
// "recoil": "*"
// "mobx": "*"

Rationale:
- See tech-stack.md for state management decision
- See ADR-[XXX] for detailed reasoning
- Simplicity: Minimal boilerplate vs Redux
- Performance: No Context API re-render issues
- Team preference: Developers prefer Zustand's API
- Bundle size: [SIZE] vs Redux Toolkit [SIZE]

Lock Reason: [WHY_ZUSTAND_IS_LOCKED]

⚠️ CRITICAL RULE:
Do NOT introduce Redux patterns (action types, reducers, dispatch) in this project.
All state management uses Zustand stores exclusively.

If Zustand limitations encountered:
- DO NOT add Redux or other state libraries
- Use AskUserQuestion for alternative approaches
- Consider refactoring state complexity
- Document decision in ADR if truly reconsidering

Exception Process:
If reconsidering state management library:
1. Create ADR documenting Zustand limitations
2. Get architect approval
3. Plan migration strategy (or accept coexistence temporarily)
4. Update tech-stack.md
5. Update this file
```

### Styling (⚠️ LOCKED TO PREVENT CSS-IN-JS MIXING)

```json
{
  "tailwindcss": "^[VERSION]",
  "autoprefixer": "^[VERSION]",
  "postcss": "^[VERSION]"
}

// FORBIDDEN: Do NOT add CSS-in-JS alternatives
// "styled-components": "*"
// "@emotion/react": "*"
// "@emotion/styled": "*"
// "sass": "*"
// "less": "*"

Rationale: Utility-first CSS, consistent design system, no runtime overhead
Lock Reason: [WHY_TAILWIND_IS_LOCKED]

⚠️ CRITICAL RULE:
Do NOT mix Tailwind with CSS-in-JS libraries or preprocessors.
All styling uses Tailwind utility classes.
```

### Build Tools

```json
{
  "vite": "^[VERSION]",
  "@vitejs/plugin-react": "^[VERSION]"
}

// FORBIDDEN
// "webpack": "*"
// "create-react-app": "*"  // Deprecated

Rationale: Vite is faster, modern ESM-based, better DX
```

### HTTP Client

```json
{
  "axios": "^[VERSION]"
}

// Alternative: fetch API (built-in)
// Use AskUserQuestion if preferring fetch over axios

Rationale: [SPECIFY - interceptors, better error handling, etc.]
```

### Form Handling

```json
{
  "react-hook-form": "^[VERSION]"
}

// Alternative Considered: Formik (rejected: [REASON])

Rationale: Performance, TypeScript support, minimal re-renders
```

### Testing

```json
{
  "@playwright/test": "^[VERSION]",
  "vitest": "^[VERSION]",
  "@testing-library/react": "^[VERSION]",
  "@testing-library/jest-dom": "^[VERSION]"
}

// FORBIDDEN: Do NOT mix E2E frameworks
// "cypress": "*"
// "selenium-webdriver": "*"

Rationale: Playwright is cross-browser, reliable, fast
```

---

## Common AI Mistakes to Prevent

### ❌ Mistake 1: Swapping ORM Due to "Connection Issues"

**Scenario:** Dapper query fails during development

**WRONG AI Behavior:**
```
AI: "Dapper connection is failing. Let me switch to Entity Framework Core."

Actions:
dotnet add package Microsoft.EntityFrameworkCore
dotnet add package Microsoft.EntityFrameworkCore.SqlServer

Creates DbContext, changes data access patterns
```

**Problems:**
- Violates dependencies.md and tech-stack.md
- Creates mixed ORM project (technical debt)
- Ignores locked decisions
- No ADR or approval

**CORRECT AI Behavior:**
```
AI: "Dapper connection is failing with error: [ERROR_MESSAGE]"

AI checks:
1. dependencies.md says: "Dapper 2.1.28 (LOCKED)"
2. CRITICAL RULE: "DO NOT suggest EF or other ORMs"

AI uses AskUserQuestion:
  Question: "Dapper connection is failing with error: [ERROR]. How should I proceed?"
  Header: "Data access"
  Options:
    - "Debug the Dapper connection string (maintain tech stack)"
    - "Check database availability and permissions"
    - "Try alternative Dapper connection approach"
    - "Reconsider ORM (requires ADR and architect approval)"
  multiSelect: false
```

### ❌ Mistake 2: Adding "Better" Libraries Without Permission

**Scenario:** AI finds Newtonsoft.Json has more features

**WRONG AI Behavior:**
```
AI: "Newtonsoft.Json has feature [X] that System.Text.Json lacks. Adding it."

Actions:
dotnet add package Newtonsoft.Json

Uses both serializers in same project
```

**Problems:**
- Unauthorized dependency
- Mixed serialization patterns
- No approval or documentation

**CORRECT AI Behavior:**
```
AI: "Need JSON feature [X]. System.Text.Json doesn't support this."

AI checks:
1. dependencies.md: "System.Text.Json (built-in)"
2. dependencies.md: "FORBIDDEN: Newtonsoft.Json unless absolutely necessary"

AI uses AskUserQuestion:
  Question: "Need JSON feature [X]. System.Text.Json doesn't support it. Should I:"
  Header: "Serialization"
  Options:
    - "Find workaround with System.Text.Json"
    - "Add Newtonsoft.Json (update dependencies.md + ADR)"
    - "Implement custom solution"
  multiSelect: false

If approved:
1. Update dependencies.md
2. Create ADR-[XXX]
3. Document rationale
4. Notify team
```

### ❌ Mistake 3: Mixing State Management Frameworks

**Scenario:** Feature seems easier with Redux

**WRONG AI Behavior:**
```
AI: "This complex state would be easier with Redux. Adding Redux Toolkit."

Actions:
npm install @reduxjs/toolkit react-redux

Creates Redux store alongside Zustand
```

**Problems:**
- Framework mixing (major technical debt)
- Confusing for developers
- Two state patterns in same codebase

**CORRECT AI Behavior:**
```
AI: "This feature has complex state requirements."

AI checks:
1. dependencies.md: "zustand ^4.4.7 (LOCKED)"
2. CRITICAL RULE: "Do NOT introduce Redux patterns"

AI uses AskUserQuestion:
  Question: "Feature [X] has complex state. Should I:"
  Header: "State mgmt"
  Options:
    - "Implement in Zustand (maintain tech stack)"
    - "Refactor to reduce state complexity"
    - "Split into multiple simpler stores"
    - "Reconsider state library (requires ADR)"
  multiSelect: false
```

### ❌ Mistake 4: Adding Similar Libraries

**Scenario:** Need HTTP client, project has axios

**WRONG AI Behavior:**
```
AI: "I need to make HTTP requests. Adding axios."

Actions:
npm install axios

But project already uses fetch API consistently
```

**Problems:**
- Duplicate functionality
- Inconsistent patterns
- Unnecessary dependency

**CORRECT AI Behavior:**
```
AI: "Need to make HTTP requests."

AI checks:
1. Searches existing code for HTTP patterns
2. Finds: fetch API used throughout codebase
3. dependencies.md doesn't list axios

AI uses existing pattern:
const response = await fetch('/api/users');
const data = await response.json();

// OR if axios is listed in dependencies.md:
const response = await axios.get('/api/users');
```

---

## Version Lock Policy

### Patch Updates (X.Y.Z → X.Y.Z+1)

**Allowed:** Automatically
**Reason:** Bug fixes, security patches, no breaking changes
**Process:** Update and test in development first

**Example:**
```
Current: Dapper 2.1.28
Update:  Dapper 2.1.29  ✅ Allowed
```

### Minor Updates (X.Y.Z → X.Y+1.0)

**Requires:** Team review + testing
**Reason:** New features, potential behavior changes
**Process:**
1. Test in development environment
2. Review release notes for breaking changes
3. Update staging environment
4. Verify all tests pass
5. Update production
6. Update this file

**Example:**
```
Current: React 18.2.0
Update:  React 18.3.0  ⚠️ Requires testing
```

### Major Updates (X.Y.Z → X+1.0.0)

**Requires:** ADR + architect approval
**Reason:** Breaking changes, API changes
**Process:**
1. Create ADR documenting:
   - Why upgrade is needed
   - Breaking changes analysis
   - Migration effort estimate
   - Benefits vs. risks
2. Get architect approval
3. Create migration plan
4. Test extensively in all environments
5. Update dependencies.md
6. Update tech-stack.md if patterns change
7. Update coding-standards.md with new patterns
8. Notify entire team

**Example:**
```
Current: .NET 8.0
Update:  .NET 9.0  ⚠️⚠️ Requires ADR + approval
```

---

## Security Vulnerability Response Protocol

### If Security Vulnerability Found in Approved Package

**Severity Levels:**

**Critical (CVSS 9.0-10.0):**
1. Emergency patch allowed without approval
2. Update immediately to patched version
3. Document in change log
4. Create post-incident ADR
5. Notify team immediately

**High (CVSS 7.0-8.9):**
1. Update within 24-48 hours
2. Get tech lead approval
3. Document in change log
4. Create ADR if major version jump
5. Notify team

**Medium (CVSS 4.0-6.9):**
1. Update in next sprint
2. Normal approval process
3. Document in change log

**Low (CVSS 0.1-3.9):**
1. Update during regular maintenance
2. Normal approval process

### Process

```
1. Security tool reports vulnerability (Dependabot, npm audit, etc.)
2. Assess severity using CVSS score
3. Check if patched version available
4. Follow severity-based process above
5. Update dependencies.md with new version
6. Update change log
7. If major version: Create ADR and update coding-standards.md
```

---

## Dependency Audit Log

| Date | Package | Version | Action | Approver | Reason | ADR |
|------|---------|---------|--------|----------|--------|-----|
| [DATE] | [PKG] | [VER] | Added | [NAME] | Initial setup | - |
| | | | | | | |
| | | | | | | |

**Action Types:** Added, Updated, Removed, Locked, Unlocked

---

## Related Documents

- [tech-stack.md](./tech-stack.md) - Technology decisions and rationale
- [anti-patterns.md](./anti-patterns.md) - Forbidden dependency patterns
- [coding-standards.md](./coding-standards.md) - How to use approved packages
- [docs/architecture/decisions/](../../docs/architecture/decisions/) - All ADRs

---

## Change Control

To modify this file:

1. **Identify Need:** What package needs to be added/changed and why?
2. **Use AskUserQuestion:** Get explicit user approval
3. **Create ADR:** Document the decision (for significant changes)
4. **Update This File:** Add package with rationale
5. **Update Cross-References:** Update tech-stack.md if needed
6. **Notify Team:** Announce new dependency
7. **Update CI/CD:** Ensure new package in build pipeline

---

## Notes

[Project-specific notes about dependencies, temporary exceptions, migration status, etc.]

**Example:**
```
Temporary Exception: During migration from Entity Framework to Dapper, both ORMs
will coexist for up to 3 months (until 2025-12-31). All NEW code must use Dapper.
See ADR-005 for migration plan.
```
