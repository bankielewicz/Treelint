# Phase 3: ADR Creation Workflow

Generate Architecture Decision Records for significant technical decisions.

## Overview

ADRs document technical decisions with context, rationale, and consequences. They provide traceability for why specific technologies and patterns were chosen, enabling future teams to understand the reasoning behind architectural choices.

**Purpose:** Document ALL significant technical decisions made during context file creation.

---

## References Used

This workflow references:
- **adr-policy.md** - When to create ADRs, what qualifies as "significant"
- **adr-template.md** - ADR structure and required sections

## ADR Examples Available

`assets/adr-examples/` contains 6 complete examples demonstrating proper ADR format:

- **ADR-EXAMPLE-001-database-selection.md** (563 lines) - PostgreSQL vs MySQL vs MongoDB for e-commerce
- **ADR-EXAMPLE-002-orm-selection.md** (816 lines) - Dapper vs Entity Framework Core for data access
- **ADR-EXAMPLE-003-state-management.md** (1,193 lines) - Zustand vs Redux for React state
- **ADR-EXAMPLE-004-clean-architecture.md** (1,216 lines) - Clean Architecture vs N-Tier vs Vertical Slice
- **ADR-EXAMPLE-005-deployment-strategy.md** (1,101 lines) - Kubernetes vs Azure App Service vs AWS ECS
- **ADR-EXAMPLE-006-scope-descope.md** (268 lines) - Handling scope changes and deferrals

---

## Workflow

### Step 1: Load ADR Resources

**Load policy:**
```
Read(file_path=".claude/skills/devforgeai-architecture/references/adr-policy.md")
```

**Load template:**
```
Read(file_path=".claude/skills/devforgeai-architecture/references/adr-template.md")
```

**Load relevant example:**

For database decisions:
```
Read(file_path=".claude/skills/devforgeai-architecture/assets/adr-examples/ADR-EXAMPLE-001-database-selection.md")
```

For ORM/data access decisions:
```
Read(file_path=".claude/skills/devforgeai-architecture/assets/adr-examples/ADR-EXAMPLE-002-orm-selection.md")
```

For state management decisions:
```
Read(file_path=".claude/skills/devforgeai-architecture/assets/adr-examples/ADR-EXAMPLE-003-state-management.md")
```

For architecture pattern decisions:
```
Read(file_path=".claude/skills/devforgeai-architecture/assets/adr-examples/ADR-EXAMPLE-004-clean-architecture.md")
```

For deployment strategy decisions:
```
Read(file_path=".claude/skills/devforgeai-architecture/assets/adr-examples/ADR-EXAMPLE-005-deployment-strategy.md")
```

For scope change decisions:
```
Read(file_path=".claude/skills/devforgeai-architecture/assets/adr-examples/ADR-EXAMPLE-006-scope-descope.md")
```

---

### Step 2: Identify Decisions Requiring ADRs

From Phase 2 (context file creation), identify all significant decisions:

**Typical ADR candidates:**
- Database choice (SQL Server vs PostgreSQL vs MongoDB)
- ORM selection (Dapper vs Entity Framework vs NHibernate)
- State management library (Zustand vs Redux vs Jotai)
- Architecture pattern (Clean Architecture vs N-Tier vs Vertical Slice)
- Testing strategy (xUnit vs NUnit, Playwright vs Cypress)
- Deployment platform (Kubernetes vs App Service vs ECS)
- Frontend framework (React vs Vue vs Angular)
- Validation library (FluentValidation vs DataAnnotations)

**Use adr-policy.md to determine** if a decision warrants an ADR.

---

### Step 3: Create ADRs for Each Major Decision

**For each significant decision:**

#### 3.1 Generate ADR Number

```
# Find existing ADRs
Glob(pattern="devforgeai/specs/adrs/ADR-*.md")

# Count existing + 1 = next number
# Example: 3 existing → next is ADR-004
```

#### 3.2 Create ADR Using Template

**Load template as base:**
```
Read(file_path=".claude/skills/devforgeai-architecture/references/adr-template.md")
```

**Populate sections:**

**1. Title and Metadata:**
```markdown
# ADR-001: Use Dapper for Data Access

Date: 2025-10-29
Status: Accepted
Deciders: [Team Lead, Architect]
```

**2. Context Section:**
```markdown
## Context

We need a data access strategy for the new e-commerce platform. The system will be read-heavy with complex queries for product catalogs, inventory, and reporting. Performance is critical for customer experience.

**Requirements:**
- Handle 10,000+ products with multiple variants
- Support complex filtering (price range, categories, attributes)
- Real-time inventory updates
- Analytics dashboards with complex aggregations
- Must support SQL Server (client requirement)
```

**3. Decision Section:**
```markdown
## Decision

We will use Dapper 2.1.x as our ORM for data access.

**Scope:**
- All repository implementations will use Dapper
- Direct SQL queries for complex reports
- FluentMigrator for schema migrations
- No Entity Framework Core usage
```

**4. Rationale Section:**
```markdown
## Rationale

**Performance:**
- Dapper benchmarks show 10x faster execution vs EF Core for our query patterns
- Zero overhead for complex joins (direct SQL mapping)
- Critical for real-time inventory and search performance

**Control:**
- Team prefers explicit SQL for complex queries
- Full optimization control over query execution plans
- Easier to debug query performance issues

**Experience:**
- Team has 5 years Dapper experience
- Minimal learning curve
- Existing code samples and patterns available

**Simplicity:**
- Micro-ORM approach reduces abstraction layers
- Direct mapping between SQL and objects
- Less "magic" than full ORMs
```

**5. Consequences Section:**
```markdown
## Consequences

### Positive

- **Superior performance** - 10x faster for read-heavy workload
- **Full SQL control** - Optimize complex queries easily
- **Minimal learning curve** - Team already expert with Dapper
- **Simplified debugging** - SQL is explicit, not generated
- **Better testability** - Mock IDbConnection easily

### Negative

- **No change tracking** - Must manually track entity state
- **More mapping code** - Explicit mapping vs auto-mapping
- **No automatic migrations** - Must write migration scripts manually (using FluentMigrator)
- **Relationship management** - Manual joins vs navigation properties
- **Less abstraction** - More SQL knowledge required

### Trade-offs Accepted

- Manual mapping code (50-100 lines per entity) vs performance
- Explicit SQL (higher code volume) vs query control
- Learning investment for new team members vs current team efficiency
```

**6. Alternatives Considered Section (ENHANCED with Research Integration - STORY-036):**

**If internet-sleuth research available:**
```python
# Check if research was conducted during Phase 2 Step 2.0.5
if research_result exists:
    # Use research findings for alternatives section
    alternatives = research_result.top_recommendations[1:]  # Exclude top (that's the Decision)

    for alt in alternatives:
        add_alternative_section(
            name=alt.approach,
            pros=alt.pros,
            cons=alt.cons,
            decision_rationale=alt.rejection_reason,
            research_reference=research_result.research_id
        )

    # Add research attribution
    adr_content += f"""
**Research Evidence:** [{research_result.research_id}]({research_result.report_path})
**Repository Examples:** {format_repos(research_result.repositories)}
"""
```

**Example with research:**
```markdown
## Alternatives Considered

*Evidence from repository archaeology research: [RESEARCH-002](devforgeai/specs/research/shared/RESEARCH-002-backend-framework-evaluation.md)*

### Entity Framework Core

**Pros:**
- Automatic change tracking (reduces boilerplate code)
- LINQ queries (more C#-like, type-safe)
- Code-first migrations (schema versioning built-in)
- Navigation properties for relationships (easier joins)

**Cons:**
- 10x slower for read-heavy workloads (benchmarked in production repos)
- Generated SQL often suboptimal for complex queries
- Abstraction hides query execution details (debugging harder)
- Harder to optimize performance (requires deep EF knowledge)

**Decision:** Rejected due to performance overhead (10x slower) documented in 3 high-quality repositories (avg quality: 8.5/10). Our read-heavy workload (95% reads) prioritizes query performance over development speed.

**Evidence:**
- Repository: [aspnetcore-realworld](https://github.com/gothinkster/aspnetcore-realworld-example-app) (Quality: 9/10, 1.8K stars)
- Benchmark: SELECT with 5 JOINs: Dapper 15ms, EF Core 147ms (9.8x slower)
- Source: RESEARCH-002 Section 4.2.3 (Performance Analysis)
```

**Example without research (traditional):**
```markdown
## Alternatives Considered

### Entity Framework Core

**Pros:**
- Automatic change tracking
- LINQ queries (more C#-like)
- Code-first migrations
- Navigation properties for relationships

**Cons:**
- 10x slower for our read-heavy patterns
- Generated SQL often suboptimal for complex queries
- Abstraction hides query execution details
- Harder to optimize performance

**Decision:** Rejected due to performance overhead and team preference for SQL control.

### NHibernate

**Pros:**
- Mature, feature-rich ORM
- Powerful querying with HQL
- Excellent caching support

**Cons:**
- Complex configuration (XML or fluent)
- Steeper learning curve than Dapper or EF
- Overkill for our needs
- Team has no experience with it

**Decision:** Rejected due to complexity and learning curve.

### Raw ADO.NET

**Pros:**
- Maximum performance (no ORM overhead)
- Full control over everything
- No external dependencies

**Cons:**
- Excessive boilerplate code
- Manual connection management
- No query composition
- Harder to test

**Decision:** Rejected due to excessive boilerplate. Dapper provides similar performance with better DX.
```

**7. Enforcement Section:**
```markdown
## Enforcement

This decision is enforced through DevForgeAI context files:

1. **tech-stack.md:**
   ```markdown
   ## Data Access: Dapper 2.1.x (LOCKED - NOT Entity Framework)
   ```

2. **dependencies.md:**
   ```markdown
   ## Data Access (LOCKED)
   - Dapper 2.1.28 (CRITICAL - NOT Entity Framework Core)
   - FluentMigrator 5.0.0 (Schema migrations)
   ```

3. **anti-patterns.md:**
   ```markdown
   ### Anti-Pattern: Switching from Dapper to Entity Framework
   ❌ FORBIDDEN: AI suggests EF Core for "easier" relationships
   ✅ CORRECT: Use Dapper multi-mapping, respect locked choice
   ```

**AI agents MUST:**
- Use Dapper for all data access
- NOT add Entity Framework packages
- Use AskUserQuestion if Dapper seems insufficient
```

#### 3.3 Write ADR to Disk

```
Write(file_path="devforgeai/specs/adrs/ADR-001-database-selection.md", content="...")
```

---

### Step 4: Create ADRs for All Major Decisions

**Minimum ADRs typically needed:**

1. **ADR-001: Database Selection** - Which database and why
2. **ADR-002: ORM Selection** - Which data access approach and why
3. **ADR-003: State Management** - Which state library and why (if applicable)
4. **ADR-004: Architecture Pattern** - Which architecture style and why
5. **ADR-005: Testing Strategy** - Testing frameworks and approach
6. **ADR-006: Deployment Strategy** - Deployment platform and approach (if known)

**Optional ADRs (create as needed):**
- Frontend framework choice
- Build tool selection
- CI/CD platform
- Monitoring and logging approach
- Caching strategy
- API design approach (REST vs GraphQL vs gRPC)

---

### Step 5: Create ADR Index

**Create README.md in devforgeai/specs/adrs/:**

```markdown
# Architecture Decision Records

This directory contains all architecture decisions for this project.

## Active ADRs

- [ADR-001: Database Selection](.//ADR-001-database-selection.md) - PostgreSQL chosen for JSONB support
- [ADR-002: ORM Selection](./ADR-002-orm-selection.md) - Dapper chosen for performance
- [ADR-003: State Management](./ADR-003-state-management.md) - Zustand chosen for simplicity
- [ADR-004: Architecture Pattern](./ADR-004-clean-architecture.md) - Clean Architecture for maintainability
- [ADR-005: Testing Strategy](./ADR-005-testing-strategy.md) - xUnit + Playwright approach

## ADR Naming Convention

ADR-{NNN}-{title-slug}.md

Example: ADR-001-database-selection.md

## When to Create ADRs

See [adr-policy.md](../references/adr-policy.md) for guidance on when ADRs are required.

## ADR Template

See [adr-template.md](../references/adr-template.md) for the standard ADR structure.
```

```
Write(file_path="devforgeai/specs/adrs/README.md", content="...")
```

---

## ADR Directory Structure

After Phase 3 completion:

```
devforgeai/specs/adrs/
├── README.md (index of all ADRs)
├── ADR-001-database-selection.md
├── ADR-002-orm-selection.md
├── ADR-003-state-management.md
├── ADR-004-architecture-pattern.md
└── ADR-005-testing-strategy.md
```

---

## Success Criteria

Phase 3 succeeds when:

- [ ] At least 1 ADR created (initial architecture decision)
- [ ] All technology decisions from Phase 2 documented
- [ ] Each ADR follows template structure
- [ ] ADR index (README.md) created
- [ ] All ADRs reference context files (enforcement section)
- [ ] Alternatives considered for each decision
- [ ] Rationale clearly documented

**Next Phase:** Move to Phase 4 (Create Technical Specifications)
