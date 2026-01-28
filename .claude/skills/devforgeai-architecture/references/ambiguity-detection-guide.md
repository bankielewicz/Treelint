# Ambiguity Detection Guide for Architecture Phase

**Purpose:** Identify when to use AskUserQuestion to prevent technical debt from assumptions.

## Core Principle

**NEVER make assumptions about technology choices, architecture patterns, or implementation approaches.**

When in doubt → Ask. When clear → Document.

---

## Ambiguity Categories and Triggers

### Category 1: Technology Selection Ambiguity

**Trigger:** Requirement mentions capability without specifying technology.

**Examples:**

| Requirement | Ambiguity | Must Ask |
|-------------|-----------|----------|
| "Add authentication" | OAuth? JWT? Session? Passkeys? | ✅ |
| "Implement caching" | Memory? Redis? CDN? SQL? | ✅ |
| "Add logging" | Serilog? NLog? Built-in? AppInsights? | ✅ |
| "Store files" | Blob? S3? Filesystem? Database? | ✅ |
| "Send emails" | SMTP? SendGrid? AWS SES? | ✅ |
| "Real-time updates" | WebSockets? SignalR? Server-Sent Events? Polling? | ✅ |

**AskUserQuestion Pattern:**
```
Question: "The requirement specifies [feature] but not the technology. Which should be used?"
Header: "Tech choice"
Options: [2-4 relevant technology options]
Description: "This will be locked in tech-stack.md"
multiSelect: false
```

---

### Category 2: Framework/Library Version Ambiguity

**Trigger:** Technology specified but version unclear or multiple versions possible.

**Examples:**

| Specified | Ambiguity | Must Ask |
|-----------|-----------|----------|
| "Use React" | React 17 or 18? Major differences in behavior | ✅ |
| "Use .NET" | .NET 6 LTS or 8 LTS? Different support timelines | ✅ |
| ".NET Framework" | 4.8 (legacy) or .NET (modern)? Critical distinction | ✅ |
| "Use Python" | Python 3.10, 3.11, or 3.12? Syntax differences | ✅ |

**AskUserQuestion Pattern:**
```
Question: "[Technology] version not specified. Which version should be used?"
Header: "Version"
Options: [2-3 supported versions with descriptions]
Description: "Affects support timeline and available features"
multiSelect: false
```

---

### Category 3: Architecture Pattern Ambiguity

**Trigger:** Multiple valid patterns exist for implementing a feature.

**Examples:**

| Requirement | Ambiguity | Must Ask |
|-------------|-----------|----------|
| "Create REST API" | Minimal APIs vs Controller-based? | ✅ |
| "Organize backend code" | Clean Architecture? N-Tier? Vertical Slice? | ✅ |
| "Handle errors" | Exceptions? Result pattern? Error codes? | ✅ |
| "Manage state" | Global? Context? Redux? Zustand? | ✅ |
| "Validate input" | Client-side? Server-side? Both? Which library? | ✅ |

**AskUserQuestion Pattern:**
```
Question: "Which architecture pattern should be used for [feature]?"
Header: "Pattern"
Options: [2-4 pattern choices with trade-offs]
Description: "Affects codebase organization and maintainability"
multiSelect: false
```

---

### Category 4: Data Access Ambiguity

**Trigger:** Database operations needed but approach not specified.

**Examples:**

| Requirement | Ambiguity | Must Ask |
|-------------|-----------|----------|
| "Access database" | ORM? Micro-ORM? Raw SQL? | ✅ |
| "Query data" | LINQ? SQL? Stored procedures? | ✅ |
| "Handle transactions" | Manual? Automatic? Distributed? | ✅ |
| "Migrate schema" | Code-first? Database-first? Manual migrations? | ✅ |

**AskUserQuestion Pattern:**
```
Question: "What data access approach should be used?"
Header: "Data access"
Options:
  - "Dapper (micro-ORM, explicit SQL)"
  - "Entity Framework Core (full ORM, LINQ)"
  - "ADO.NET (direct database access)"
Description: "Critical decision affecting performance and maintainability"
multiSelect: false
```

**Special Case - ORM Selection:**

This is CRITICAL because LLMs often substitute ORMs when encountering issues.

```
Question: "Which ORM should be used for data access?"
Header: "ORM choice"
Options:
  - "Dapper 2.x (micro-ORM, fast, explicit SQL)"
  - "Entity Framework Core 8.x (full ORM, migrations, LINQ)"
  - "NHibernate (mature ORM, complex configuration)"
  - "No ORM (raw ADO.NET)"
Description: "⚠️ CRITICAL: This choice will be LOCKED in tech-stack.md. AI agents cannot substitute alternatives without explicit approval."
multiSelect: false
```

---

### Category 5: Testing Strategy Ambiguity

**Trigger:** Testing requirements unclear or not specified.

**Examples:**

| Requirement | Ambiguity | Must Ask |
|-------------|-----------|----------|
| "Add tests" | Unit? Integration? E2E? All? | ✅ |
| "Test coverage" | 80%? 90%? 100%? Which metrics? | ✅ |
| "Testing framework" | xUnit? NUnit? MSTest? (for .NET) | ✅ |
| "Mock dependencies" | Which mocking library? | ✅ |
| "E2E testing" | Playwright? Cypress? Selenium? | ✅ |

**AskUserQuestion Pattern:**
```
Question: "What testing strategy should be implemented?"
Header: "Testing"
Options:
  - "Unit tests only (90% coverage)"
  - "Unit + integration tests"
  - "Unit + integration + E2E"
  - "Match existing project patterns"
Description: "Affects test infrastructure and CI/CD setup"
multiSelect: true  # Can select multiple test types
```

---

### Category 6: Security & Compliance Ambiguity

**Trigger:** Security-sensitive operations without clear security requirements.

**Examples:**

| Requirement | Ambiguity | Must Ask |
|-------------|-----------|----------|
| "Store passwords" | Hashing algorithm? Salt? Pepper? | ✅ |
| "Authenticate API" | JWT? OAuth2? API keys? | ✅ |
| "Encrypt data" | At rest? In transit? Both? Algorithm? | ✅ |
| "Handle PII" | GDPR compliance? Data anonymization? | ✅ |
| "Authorize access" | RBAC? ABAC? Claims-based? | ✅ |

**AskUserQuestion Pattern:**
```
Question: "What security measures should be applied for [sensitive operation]?"
Header: "Security"
Options: [Security approaches with compliance implications]
Description: "⚠️ Security decision with compliance implications"
multiSelect: true  # May need multiple security measures
```

---

### Category 7: Performance Requirements Ambiguity

**Trigger:** Performance mentioned without specific targets.

**Examples:**

| Requirement | Ambiguity | Must Ask |
|-------------|-----------|----------|
| "Make it fast" | < 100ms? < 500ms? < 2s? | ✅ |
| "Handle scale" | 100 users? 10,000 users? 1M users? | ✅ |
| "Optimize database" | Target query time? Acceptable load time? | ✅ |
| "Improve throughput" | Requests/sec target? Concurrent connections? | ✅ |

**AskUserQuestion Pattern:**
```
Question: "What are the performance targets for [feature]?"
Header: "Performance"
Options:
  - "High performance (< 100ms, 10K+ concurrent users)"
  - "Standard (< 500ms, 1K concurrent users)"
  - "Acceptable (< 2s, 100 concurrent users)"
Description: "Affects architecture decisions and optimization strategy"
multiSelect: false
```

---

### Category 8: Project Structure Ambiguity

**Trigger:** New type of code/file not covered by existing source-tree.md.

**Examples:**

| Need to Create | Ambiguity | Must Ask |
|----------------|-----------|----------|
| Background job | Where do background jobs go? | ✅ |
| Utility helper | Shared utilities location? | ✅ |
| Configuration | App settings, constants, or both? | ✅ |
| External integration | Infrastructure or Application layer? | ✅ |

**AskUserQuestion Pattern:**
```
Question: "Where should [new file type] be placed in the project structure?"
Header: "File location"
Options:
  - "src/Application/[Feature]/"
  - "src/Infrastructure/[Feature]/"
  - "src/Domain/[Feature]/"
  - "Other (specify)"
Description: "Will be documented in source-tree.md"
multiSelect: false
```

---

### Category 9: Brownfield-Specific Ambiguity

**Trigger:** Existing code conflicts with new requirements or best practices.

**Examples:**

| Situation | Ambiguity | Must Ask |
|-----------|-----------|----------|
| Legacy uses EF, new spec implies Dapper | Migration strategy? Continue with EF? | ✅ |
| Old code has no tests | Add tests now? Match old patterns? | ✅ |
| Existing structure different from preferred | Adapt or refactor? | ✅ |
| Multiple coding styles present | Which to follow? Standardize? | ✅ |

**AskUserQuestion Pattern:**
```
Question: "Existing code uses [OldApproach], but [NewApproach] is preferred. How should we proceed?"
Header: "Migration"
Options:
  - "Gradual migration (new code uses new approach)"
  - "Full refactor (convert all to new approach)"
  - "Accept technical debt (continue with old approach)"
  - "Reassess preference (old approach may be better here)"
Description: "Affects project consistency and migration effort"
multiSelect: false
```

---

### Category 10: Deployment & Infrastructure Ambiguity

**Trigger:** Deployment target or infrastructure not specified.

**Examples:**

| Requirement | Ambiguity | Must Ask |
|-------------|-----------|----------|
| "Deploy to cloud" | AWS? Azure? GCP? | ✅ |
| "Use containers" | Docker? Kubernetes? ECS? | ✅ |
| "CI/CD pipeline" | GitHub Actions? Azure DevOps? Jenkins? | ✅ |
| "Monitor application" | Application Insights? Datadog? Custom? | ✅ |

**AskUserQuestion Pattern:**
```
Question: "What is the target deployment infrastructure?"
Header: "Infrastructure"
Options: [Infrastructure options]
Description: "Affects architecture, costs, and operational complexity"
multiSelect: false
```

---

## Detection Algorithm

Use this decision tree to determine if AskUserQuestion is needed:

```
FOR EACH requirement in spec:

  1. Is technology specified explicitly?
     NO → AskUserQuestion (Category 1: Technology Selection)
     YES → Continue to 2

  2. Is version/variant specified?
     NO → AskUserQuestion (Category 2: Version Ambiguity)
     YES → Continue to 3

  3. Check tech-stack.md: Is this technology documented?
     NO → AskUserQuestion (Category 1: Technology Selection)
     YES → Continue to 4

  4. Does requirement conflict with tech-stack.md?
     YES → AskUserQuestion (Category 9: Conflict Resolution)
     NO → Continue to 5

  5. Is implementation pattern specified?
     NO → AskUserQuestion (Category 3: Architecture Pattern)
     YES → Continue to 6

  6. Check source-tree.md: Is file placement clear?
     NO → AskUserQuestion (Category 8: Project Structure)
     YES → Continue to 7

  7. Is testing strategy specified?
     NO → AskUserQuestion (Category 5: Testing Strategy)
     YES → Continue to 8

  8. Are performance targets specified?
     NO → AskUserQuestion (Category 7: Performance Requirements)
     YES → Continue to 9

  9. Are security requirements specified?
     NO → AskUserQuestion (Category 6: Security & Compliance)
     YES → Proceed with implementation (no ambiguity)
```

---

## High-Risk Ambiguities (Always Ask)

These ambiguities have highest technical debt risk:

### 1. ORM/Data Access Technology ⚠️ CRITICAL

**Why Critical:** LLMs frequently substitute ORMs when encountering difficulties.

**Always ask explicitly:**
```
Question: "Which ORM should be used? (This will be LOCKED and cannot be changed without approval)"
Options: [Specific ORM choices]
```

**Then lock in tech-stack.md:**
```markdown
## Data Access (LOCKED TO DAPPER)
- Dapper 2.1.28
- PROHIBITED: Entity Framework Core, NHibernate
- If Dapper issues arise, DO NOT substitute. Use AskUserQuestion to debug.
```

### 2. State Management Library ⚠️ CRITICAL

**Why Critical:** LLMs often mix frameworks (e.g., introduce Redux in Zustand project).

**Always ask explicitly:**
```
Question: "Which state management approach? (This will prevent mixing frameworks)"
Options: [Specific library choices]
```

**Then lock in tech-stack.md:**
```markdown
## State Management (LOCKED TO ZUSTAND)
- Zustand 4.x
- PROHIBITED: Redux, MobX, Jotai
- Do not mix patterns. All state management uses Zustand.
```

### 3. Project Structure ⚠️ HIGH RISK

**Why Critical:** Inconsistent structure leads to maintenance nightmares.

**Always establish before any coding:**
```
Question: "Which architecture pattern should organize the codebase?"
Options: [Clean Architecture, N-Tier, Vertical Slice, etc.]
```

**Then document in source-tree.md with explicit rules.**

### 4. API Style ⚠️ HIGH RISK

**Why Critical:** Mixing REST/GraphQL/gRPC creates inconsistent APIs.

**Always ask explicitly:**
```
Question: "Which API style should be used?"
Options: [REST, GraphQL, gRPC, etc.]
```

**Then lock in tech-stack.md and prohibit alternatives.**

---

## Examples of Good vs Bad Ambiguity Handling

### ❌ BAD: Making Assumptions

**Scenario:** Spec says "add caching"

**Bad AI behavior:**
```
AI: "I'll add Redis for caching"
# Proceeds to add StackExchange.Redis package
# Updates code to use Redis
# NO QUESTION ASKED
```

**Problems:**
- Assumed Redis (could be in-memory, SQL Server, etc.)
- Added unapproved dependency
- Created potential infrastructure dependency
- No ADR documenting decision

---

### ✅ GOOD: Asking First

**Scenario:** Spec says "add caching"

**Good AI behavior:**
```
AI: "The spec requires caching but doesn't specify the technology.
     Let me ask for clarification."

AskUserQuestion:
  Question: "Which caching approach should be used?"
  Header: "Caching"
  Options:
    - "In-memory caching (IMemoryCache built-in)"
    - "Distributed caching with Redis"
    - "SQL Server caching (database-backed)"
    - "CDN caching (for static assets)"
  Description: "Affects infrastructure requirements and scaling"
  multiSelect: false

User selects: "In-memory caching"

AI: "Creating ADR for caching decision...
     Updating tech-stack.md with approved caching technology...
     No new infrastructure dependencies required."
```

**Benefits:**
- No assumptions made
- User's choice documented
- tech-stack.md updated
- ADR created with rationale
- No unauthorized dependencies

---

### ❌ BAD: Conflicting with Existing Standards

**Scenario:** Brownfield project uses Dapper, AI encounters query complexity

**Bad AI behavior:**
```
AI: "This Dapper query is complex. Entity Framework Core would make this easier."
# Adds Microsoft.EntityFrameworkCore package
# Creates DbContext
# Implements feature with EF Core
```

**Problems:**
- Violated tech-stack.md
- Mixed ORMs in same project
- Created technical debt
- Ignored existing patterns

---

### ✅ GOOD: Respecting Constraints

**Scenario:** Brownfield project uses Dapper, AI encounters query complexity

**Good AI behavior:**
```
AI: "This query is complex. Let me check tech-stack.md..."

Read(file_path="devforgeai/specs/context/tech-stack.md")
# Sees: "Data Access (LOCKED TO DAPPER)"
# Sees: "PROHIBITED: Entity Framework Core"

AI: "Tech-stack.md locks data access to Dapper.
     I should not substitute ORMs. Let me ask for help."

AskUserQuestion:
  Question: "This complex query is difficult in Dapper. Should I:"
  Header: "Data access"
  Options:
    - "Optimize Dapper query (maintain tech stack)"
    - "Use stored procedure for this query"
    - "Break query into simpler parts"
    - "Reconsider ORM choice (requires ADR)"
  multiSelect: false
```

**Benefits:**
- Respected tech-stack.md constraints
- Asked before violating standards
- Offered solutions within constraints
- Made ORM change explicit decision

---

## Summary Checklist

Before finalizing ANY architecture decision, verify:

- [ ] All technology choices explicitly confirmed via AskUserQuestion
- [ ] All versions specified (no "latest" or ambiguous versions)
- [ ] All decisions documented in appropriate context files
- [ ] ADRs created for significant decisions
- [ ] tech-stack.md updated with LOCKED choices
- [ ] source-tree.md updated with new patterns
- [ ] dependencies.md updated with approved packages
- [ ] anti-patterns.md updated with forbidden alternatives
- [ ] No assumptions made about unclear requirements

**If ANY checkbox is unchecked → Use AskUserQuestion to resolve**

---

## Key Principle

**"When in doubt, ask. When clear, lock it in."**

The goal is ZERO ambiguity entering the development phase, preventing technical debt from wrong assumptions.
