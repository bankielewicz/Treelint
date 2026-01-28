# ADR-XXX: [Decision Title]

**Date:** YYYY-MM-DD
**Status:** [Proposed | Accepted | Deprecated | Superseded by ADR-XXX]
**Deciders:** [List of people involved in decision]
**Tags:** [architecture, database, frontend, security, etc.]

## Context

Describe the context and problem statement that necessitates this decision.

- What is the architectural challenge or question?
- What forces or constraints are at play?
- Why does this decision need to be made now?
- What are the business or technical drivers?

**Example:**
> We need to choose a data access strategy for the new customer management system. The system will handle 10,000+ concurrent users with read-heavy workload (90% reads, 10% writes). The database contains complex relationships and requires optimized query performance.

## Decision

State the architecture decision made. Use present tense: "We will..."

**Example:**
> We will use Dapper 2.1.x as our data access micro-ORM for all database operations.

## Rationale

Explain why this decision was made. Include:
- Key factors that influenced the decision
- Technical considerations
- Team considerations (expertise, learning curve)
- Performance implications
- Cost implications
- Timeline implications

**Example:**
> **Performance:** Dapper benchmarks show 10x faster execution than Entity Framework Core for our read-heavy queries. This is critical for meeting our < 100ms response time requirement.
>
> **Control:** Dapper provides explicit control over SQL queries, allowing our experienced database team to optimize complex joins and queries that would be difficult to express efficiently in LINQ.
>
> **Team Expertise:** Our development team has 5+ years of Dapper experience from previous projects, resulting in zero learning curve and immediate productivity.
>
> **Simplicity:** Dapper's micro-ORM approach avoids the complexity of full-featured ORMs like EF Core, reducing cognitive overhead and making debugging easier.

## Consequences

### Positive

List the positive outcomes and benefits of this decision.

**Example:**
- **Performance:** Query execution 10x faster, meeting sub-100ms requirement
- **Maintainability:** Explicit SQL is easier to debug and optimize
- **Team velocity:** Zero learning curve, immediate productivity
- **Flexibility:** Full control over query optimization and execution plans
- **Simplicity:** Minimal abstraction layer, less "magic" to understand

### Negative

List the negative outcomes, trade-offs, and drawbacks.

**Example:**
- **Manual mapping:** No automatic change tracking, requires manual mapping code
- **Boilerplate:** More code than EF Core for simple CRUD operations
- **SQL expertise required:** Team must maintain SQL skills, can't rely on LINQ
- **No migration tools:** Migrations handled manually (DbUp) instead of automatic
- **Limited abstraction:** Less database-agnostic than higher-level ORMs

### Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| SQL injection vulnerabilities | Medium | High | Enforce parameterized queries, code review checklist |
| Performance regression if team writes inefficient SQL | Low | Medium | SQL review by DBA, query performance monitoring |
| Difficulty onboarding new team members without Dapper experience | Medium | Low | Create internal Dapper patterns guide, pair programming |

## Alternatives Considered

List alternative options that were evaluated and why they were not chosen.

### Alternative 1: Entity Framework Core

**Pros:**
- Automatic change tracking
- LINQ queries (more C#-like)
- Automatic migrations
- Larger community and ecosystem

**Cons:**
- 10x slower for our read-heavy workload
- Less control over query optimization
- More complex abstraction layer
- Potential N+1 query problems

**Reason for rejection:** Performance overhead unacceptable for our requirements.

### Alternative 2: NHibernate

**Pros:**
- Mature, battle-tested ORM
- Powerful caching features
- Extensive mapping capabilities

**Cons:**
- Steep learning curve
- Complex configuration
- Team has no experience with it
- Overkill for our needs

**Reason for rejection:** Learning curve and complexity not justified by our requirements.

### Alternative 3: Raw ADO.NET

**Pros:**
- Maximum performance and control
- No third-party dependencies
- Zero abstraction overhead

**Cons:**
- Excessive boilerplate code
- Manual connection and transaction management
- More error-prone
- Lower productivity

**Reason for rejection:** Dapper provides 95% of the performance with 50% of the boilerplate.

## Implementation

Describe how this decision will be implemented and enforced.

**Example:**
1. **Add to tech-stack.md:**
   ```markdown
   ## Data Access (LOCKED TO DAPPER)
   - Dapper 2.1.28
   - Microsoft.Data.SqlClient 5.1.2
   - PROHIBITED: Entity Framework Core, NHibernate
   ```

2. **Add to dependencies.md:**
   ```xml
   <!-- REQUIRED: Use Dapper for data access -->
   <PackageReference Include="Dapper" Version="2.1.28" />

   <!-- FORBIDDEN: Do NOT add these -->
   <!-- <PackageReference Include="Microsoft.EntityFrameworkCore" Version="*" /> -->
   ```

3. **Add to anti-patterns.md:**
   ```markdown
   ## ❌ FORBIDDEN: ORM Substitution
   AI agents must NOT substitute Dapper with Entity Framework or other ORMs,
   even if encountering difficulties. Use AskUserQuestion instead.
   ```

4. **Create coding standards:**
   - Document Dapper query patterns
   - Establish repository interface patterns
   - Define transaction handling approach
   - Create SQL injection prevention checklist

5. **Training:**
   - Create internal Dapper best practices guide
   - Code review checklist includes Dapper patterns
   - Onboarding includes Dapper workshop

## Validation and Review

### Review Criteria
- [ ] Performance benchmarks validate 10x improvement claim
- [ ] Team members trained on Dapper patterns
- [ ] Repository pattern examples documented
- [ ] Anti-pattern prevention in place

### Review Schedule
- **Initial review:** 3 months after implementation
- **Full review:** 12 months after implementation
- **Trigger review if:** Performance degrades, team composition changes, or technology landscape shifts significantly

### Success Metrics
- API response times consistently < 100ms (p95)
- Zero ORM-related production incidents in first 6 months
- Developer velocity maintained or improved (measured by feature delivery rate)
- Code review feedback shows consistent Dapper pattern usage

## Related Decisions

Link to related ADRs:
- ADR-002: Database Schema Design
- ADR-003: Repository Pattern Implementation
- ADR-005: Transaction Management Strategy

## References

- [Dapper Official Documentation](https://github.com/DapperLib/Dapper)
- [Performance Benchmarks: Dapper vs EF Core](https://example.com/benchmarks)
- [Internal: Team Dapper Experience Survey](./team-surveys/dapper-expertise.md)
- [Performance Requirements Document](../requirements/performance.md)

---

## Change Log

| Date | Change | Author | Reason |
|------|--------|--------|--------|
| 2025-10-29 | Initial decision | Tech Lead | Project setup |
| | | | |

---

## Notes

Additional context, discussions, or decisions that don't fit above sections but are important for future reference.

**Example:**
> During decision process, we also considered using stored procedures exclusively for data access. This was rejected because it would separate business logic between C# and SQL, making the codebase harder to maintain and test. However, we may use stored procedures for specific complex analytical queries where SQL optimization is critical.
