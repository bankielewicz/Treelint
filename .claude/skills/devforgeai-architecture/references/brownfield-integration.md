# Brownfield Integration Guide

Working with existing projects that need DevForgeAI adoption.

## Overview

Brownfield projects require discovering existing architecture before creating context files. Unlike greenfield (new) projects where you define architecture from scratch, brownfield projects must:

1. **Discover** current state (technologies, structure, patterns)
2. **Document** existing architecture as baseline
3. **Decide** migration strategy (gradual vs full refactor vs accept current state)
4. **Create** context files reflecting reality (not ideals)

**Critical Principle:** Context files must match reality. Document what IS, not what you wish it was.

---

## Discovery Phase

### Step 1: Check for Existing Context Files

```
Glob(pattern="devforgeai/specs/context/*.md")
```

**If files exist:**
- Load them as constraints
- Validate they match actual codebase
- If mismatches found → Update context files to reflect reality
- Proceed to validation phase (specs must respect existing context)

**If files DON'T exist:**
- This is technical debt (project has no documented architecture)
- Discovery required (analyze project to create baseline)
- Proceed with brownfield discovery workflow

---

### Step 2: Discover Project Technologies

**Find project type:**
```
# .NET projects
Glob(pattern="**/*.sln")
Glob(pattern="**/*.csproj")

# Node.js projects
Glob(pattern="**/package.json")

# Python projects
Glob(pattern="**/requirements.txt")
Glob(pattern="**/Pipfile")
Glob(pattern="**/pyproject.toml")

# Java projects
Glob(pattern="**/pom.xml")
Glob(pattern="**/build.gradle")

# Go projects
Glob(pattern="**/go.mod")

# Ruby projects
Glob(pattern="**/Gemfile")
```

**Analyze dependencies:**

**For .NET:**
```
# Extract NuGet packages
Grep(pattern="<PackageReference", glob="**/*.csproj", output_mode="content")

# Look for Entity Framework
Grep(pattern="Microsoft.EntityFrameworkCore", glob="**/*.csproj")

# Look for Dapper
Grep(pattern="Dapper", glob="**/*.csproj")

# Check target framework
Grep(pattern="<TargetFramework>", glob="**/*.csproj", output_mode="content")
```

**For Node.js:**
```
# Extract npm packages
Read(file_path="package.json")

# Check lock file for exact versions
Read(file_path="package-lock.json")

# Look for React
Grep(pattern="\"react\":", file_path="package.json")

# Look for state management
Grep(pattern="redux|zustand|jotai|mobx", file_path="package.json")
```

**For Python:**
```
# Extract pip packages
Read(file_path="requirements.txt")

# Or pipenv
Read(file_path="Pipfile")

# Look for FastAPI vs Flask vs Django
Grep(pattern="fastapi|flask|django", file_path="requirements.txt")
```

---

### Step 3: Discover Project Structure

**Analyze folder organization:**
```
# Find main source directory
Glob(pattern="src/**/*")
Glob(pattern="lib/**/*")
Glob(pattern="app/**/*")

# Find test directory
Glob(pattern="tests/**/*")
Glob(pattern="test/**/*")
Glob(pattern="**/*.test.*")

# Understand layers (Clean Architecture?)
Glob(pattern="**/Domain/**/*")
Glob(pattern="**/Application/**/*")
Glob(pattern="**/Infrastructure/**/*")

# Or N-Tier?
Glob(pattern="**/Business/**/*")
Glob(pattern="**/Data/**/*")
Glob(pattern="**/Presentation/**/*")
```

**Identify patterns:**
```
# Look for repositories
Grep(pattern="class.*Repository", output_mode="files_with_matches")
Grep(pattern="interface.*IRepository", output_mode="files_with_matches")

# Look for services
Grep(pattern="class.*Service", output_mode="files_with_matches")

# Look for DTOs
Grep(pattern="class.*Dto", output_mode="files_with_matches")

# Check for DI usage
Grep(pattern="services.AddScoped|services.AddTransient|services.AddSingleton")
```

---

### Step 4: Gap Analysis

**Compare discovered state vs desired state:**

```markdown
## Gap Analysis

### Current State (Discovered)

**Technologies:**
- Backend: C# with .NET 6.0
- Database: SQL Server 2019
- ORM: Entity Framework Core 6.0
- Frontend: React 17 with JavaScript (not TypeScript)
- State: Redux Toolkit
- Testing: NUnit + Moq

**Structure:**
- Mixed architecture (some Clean, some N-Tier)
- Tests co-located with source
- No consistent folder naming

**Patterns:**
- Mix of repository pattern and direct DbContext access
- Some DTOs, some direct entities in API
- Inconsistent DI lifetimes (mix of Scoped, Transient, Singleton)

### Desired State (From Spec/Preferences)

**Technologies:**
- Backend: C# with .NET 8.0 (upgrade)
- Database: SQL Server (same ✓)
- ORM: Dapper (change from EF Core)
- Frontend: React 18 with TypeScript (upgrade + add TypeScript)
- State: Zustand (change from Redux)
- Testing: xUnit + NSubstitute (change frameworks)

**Structure:**
- Clean Architecture (full consistency)
- Tests mirror source structure
- Consistent naming conventions

**Patterns:**
- 100% repository pattern
- 100% DTO pattern
- Consistent Scoped DI lifetimes

### Gap Summary

**Technology changes required:**
1. Upgrade .NET 6 → .NET 8 (minor, low risk)
2. Migrate EF Core → Dapper (major, high risk)
3. Add TypeScript to React (moderate risk)
4. Replace Redux with Zustand (major, moderate risk)
5. Change test frameworks (moderate, low risk)

**Structure changes required:**
1. Reorganize to full Clean Architecture
2. Move tests to mirror structure
3. Standardize naming

**Pattern changes required:**
1. Remove direct DbContext usage
2. Add DTOs where missing
3. Standardize DI lifetimes

**Estimated effort:** 6-8 weeks
```

---

### Step 5: Migration Strategy Decision

Use AskUserQuestion to determine approach:

```
Question: "Project currently uses [CURRENT], but [DESIRED] is preferred. How should we proceed?"
Header: "Migration"
Options:
  - "Gradual migration (new code uses [DESIRED], legacy stays [CURRENT])"
  - "Full refactor (convert all to [DESIRED])"
  - "Accept current state (continue with [CURRENT], update context to match)"
  - "Reassess preference (maybe [CURRENT] is better for this project)"
multiSelect: false
```

**For each gap, decide migration approach:**

**Example: EF Core → Dapper Migration**
```
Question: "Project uses EF Core, but Dapper is preferred for performance. Migration strategy?"
Header: "ORM migration"
Options:
  - "Gradual: New repositories use Dapper, existing stay EF Core (6-8 weeks)"
  - "Full refactor: Convert all repositories to Dapper (12-16 weeks)"
  - "Accept EF Core: Update tech-stack.md to lock EF Core (no migration)"
  - "Hybrid: Dapper for read-heavy, EF Core for write-heavy (complexity)"
multiSelect: false
```

---

### Step 6: Create Context Files Reflecting Decision

**If "Accept current state" chosen:**

Create context files documenting CURRENT state, not desired state:

```markdown
## tech-stack.md (reflects reality)

### Backend: C# with .NET 6.0 (CURRENT - upgrade to 8.0 planned)

**Status:** Transitional
**Current:** .NET 6.0
**Target:** .NET 8.0 (migrate in Q2 2025)

### Data Access: Entity Framework Core 6.0 (CURRENT - NOT migrating to Dapper)

**Decision:** After cost/benefit analysis, EF Core is appropriate for this project.
**Rationale:** See ADR-007-retain-ef-core.md

❌ PREVIOUSLY CONSIDERED: Dapper migration
✅ CURRENT DECISION: Retain EF Core

AI agents MUST use Entity Framework Core, NOT Dapper.
```

**If "Gradual migration" chosen:**

Document transitional state:

```markdown
## tech-stack.md (transitional state)

### Data Access: Migrating from EF Core to Dapper

**Status:** TRANSITIONAL (In Progress)

**For NEW code:**
- ✅ Use Dapper 2.1.x (all new repositories)

**For EXISTING code:**
- ⚠️ Entity Framework Core 6.0 remains (legacy repositories)
- Do NOT convert unless explicitly tasked

**Migration Plan:**
- Phase 1 (Q1 2025): All new repositories use Dapper
- Phase 2 (Q2 2025): Convert read-heavy repositories to Dapper
- Phase 3 (Q3 2025): Convert remaining repositories
- Phase 4 (Q4 2025): Remove EF Core dependency

**AI Agent Rules:**
1. NEW repositories → MUST use Dapper
2. EXISTING repositories → Leave as EF Core (do not convert)
3. Modifications to existing → Keep EF Core (don't mix)

See ADR-008-gradual-dapper-migration.md for full plan.
```

**If "Full refactor" chosen:**

Document refactoring plan, but CURRENT context reflects old state:

```markdown
## tech-stack.md (pre-refactor)

### Data Access: Entity Framework Core 6.0 (TEMPORARY - refactoring to Dapper)

**Status:** Pre-refactor
**Current:** Entity Framework Core 6.0 (all repositories)
**Target:** Dapper 2.1.x (full migration)

**Refactor Timeline:** 12-16 weeks (see ADR-009-full-dapper-refactor.md)

**AI Agent Rules During Refactor:**
1. READ ONLY mode during refactor (no new features)
2. Follow refactor tasks sequentially
3. Test each repository conversion before proceeding
4. Maintain 100% test coverage throughout

**After refactor completes:** Update this file to lock Dapper.
```

---

## Brownfield-Specific Patterns

### Pattern 1: Document Then Decide

```
1. Discover current state (technologies, structure, patterns)
2. Document current state in context files AS-IS
3. Analyze gaps between current and desired
4. Use AskUserQuestion for migration strategy
5. Update context files to reflect decision
```

**Never:** Create idealized context files that don't match reality.

### Pattern 2: Transitional States

For gradual migrations, context files document BOTH old and new:

```markdown
## State Management: Transitional (Redux → Zustand)

**For NEW components:**
✅ Use Zustand (all new state)

**For EXISTING components:**
⚠️ Redux remains (do not convert unless tasked)

**AI Agent Rules:**
- NEW components → Zustand hooks
- EXISTING components → Keep Redux
- No mixing within single component
```

### Pattern 3: Accept Reality

Sometimes current state is better than "ideal":

```markdown
## ORM: Entity Framework Core (Accepted)

**Original preference:** Dapper for performance
**Decision:** Retain EF Core after cost/benefit analysis

**Rationale:**
- Migration cost: 12 weeks development
- Performance gap: Not critical for our workload (< 500 users)
- Team expertise: Strong EF Core skills
- Complexity: EF relationships simplify our domain model

**Conclusion:** EF Core is appropriate for this project.
See ADR-010-retain-ef-core.md for full analysis.
```

---

## Validation After Discovery

After creating context files from brownfield discovery:

1. **Validate context matches reality:**
   ```
   # Check tech-stack.md lists actual dependencies
   Grep(pattern="Entity Framework", file_path="devforgeai/specs/context/tech-stack.md")
   Grep(pattern="Entity Framework", glob="**/*.csproj")
   # Both should match
   ```

2. **Check for technical debt:**
   - Duplicate packages (both Serilog and NLog)
   - Outdated versions (React 16 when 18 is available)
   - Unused dependencies (AutoMapper referenced but not used)
   - Inconsistent patterns (some repos, some direct access)

3. **Document debt in context files:**
   ```markdown
   ## Technical Debt Identified

   - **Duplicate logging:** Uses both Serilog AND NLog (consolidate to Serilog)
   - **Outdated React:** Version 16 (upgrade to 18 recommended)
   - **Unused packages:** AutoMapper referenced but not used (remove)
   - **Inconsistent patterns:** 70% use repositories, 30% direct DbContext
   ```

4. **Create debt reduction plan** (optional ADR)

---

## Common Brownfield Scenarios

### Scenario 1: No Existing Context Files (Most Common)

**Action:**
- Analyze project to discover current state
- Create baseline context files documenting AS-IS
- Identify technical debt
- Use AskUserQuestion for migration decisions
- Update context files to reflect chosen strategy

### Scenario 2: Partial Context Files Exist

**Action:**
- Load existing context files
- Validate they match reality (they may be outdated)
- Create missing context files
- Reconcile conflicts between files
- Ensure consistency across all 6 files

### Scenario 3: Context Files Exist But Violate

**Action:**
- Load existing context files
- Validate spec against them
- If spec violates constraints → Use AskUserQuestion:
  ```
  Question: "Spec requires [X], but tech-stack.md specifies [Y]. Which is correct?"
  Options:
    - "Update spec to use [Y] (respect existing constraint)"
    - "Update tech-stack.md to allow [X] (create ADR for change)"
  ```

---

## Gap Analysis Template

Use this template to document gaps:

```markdown
## Gap Analysis: [Project Name]

### Current State

**Technologies:**
- Language: [discovered]
- Framework: [discovered]
- Database: [discovered]
- ORM: [discovered]
- Frontend: [discovered]
- State: [discovered]

**Structure:**
- Architecture style: [discovered]
- Layer organization: [discovered]
- Test organization: [discovered]

**Patterns:**
- Repository pattern: [%]
- Service pattern: [%]
- DTO pattern: [%]
- DI usage: [%]

**Technical Debt:**
- [Issue 1]
- [Issue 2]
- [Issue 3]

### Desired State

**Technologies:**
- [Differences from current]

**Structure:**
- [Differences from current]

**Patterns:**
- [Differences from current]

### Gap

**Technology changes:**
1. [Change 1] - Effort: [estimate], Risk: [low/med/high]
2. [Change 2] - Effort: [estimate], Risk: [low/med/high]

**Structure changes:**
1. [Change 1] - Effort: [estimate], Risk: [low/med/high]

**Pattern changes:**
1. [Change 1] - Effort: [estimate], Risk: [low/med/high]

**Total effort:** [estimate]
**Total risk:** [assessment]

### Migration Strategy

[Document chosen strategy from AskUserQuestion]

**Rationale:**
[Why this strategy was selected]

**Timeline:**
[When changes will happen]

**Phases:**
1. [Phase 1 description]
2. [Phase 2 description]
```

---

## Migration Strategy Patterns

### Strategy 1: Gradual Migration (Recommended)

**Approach:**
- NEW code uses desired technologies/patterns
- EXISTING code remains unchanged (stable)
- Conversion happens incrementally as features are modified

**Context file documentation:**
```markdown
## [Technology]: TRANSITIONAL

**For NEW code:** Use [new tech]
**For EXISTING code:** Keep [old tech]

**Rules:**
- No mixing within single file/component
- Test both tech stacks separately
- Plan conversion milestones

**Timeline:**
- Phase 1: All new code uses [new tech]
- Phase 2: Convert high-priority modules
- Phase 3: Convert remaining modules
- Phase 4: Remove [old tech]
```

**Pros:**
- Lower risk (incremental changes)
- Continuous delivery (no big-bang release)
- Learn as you go
- Easier to rollback

**Cons:**
- Two tech stacks to maintain temporarily
- Longer migration timeline
- Some complexity managing both

### Strategy 2: Full Refactor

**Approach:**
- Stop new feature development
- Convert entire codebase in dedicated refactor sprint(s)
- Big-bang release after conversion complete

**Context file documentation:**
```markdown
## [Technology]: PRE-REFACTOR

**Current:** [old tech] (all code)
**Target:** [new tech] (after refactor)

**Refactor Status:** In Progress (Week X of Y)

**AI Agent Rules:**
- READ ONLY mode during refactor
- Follow refactor tasks sequentially
- No new features until refactor complete
```

**Pros:**
- Clean cutover (no dual maintenance)
- Shorter calendar time (if dedicated)
- Consistent tech stack immediately

**Cons:**
- Higher risk (big changes at once)
- Feature development paused
- Harder to rollback
- Requires extensive testing

### Strategy 3: Accept Current State

**Approach:**
- Current technology is actually fine
- Cost of migration outweighs benefits
- Document current state as standard

**Context file documentation:**
```markdown
## [Technology]: [Current Tech] (ACCEPTED)

**Original preference:** [alternative tech]
**Decision:** Retain [current tech] after analysis

**Rationale:**
- Migration cost: [estimate]
- Benefit: [assessment]
- Conclusion: [current tech] is appropriate

See ADR-NNN-retain-[current-tech].md for full analysis.
```

**Pros:**
- Zero migration cost
- No disruption to development
- Team expertise retained

**Cons:**
- May not get "ideal" architecture
- Technical debt may persist
- Performance/maintainability trade-offs

---

## Brownfield Context File Creation

After migration strategy decided:

### For "Accept Current State"

Create context files documenting CURRENT technologies:

```
tech-stack.md → Lists actual dependencies (EF Core, Redux, etc.)
source-tree.md → Documents actual structure (as-is)
dependencies.md → Lists current packages with versions
coding-standards.md → Documents current patterns (discovered)
architecture-constraints.md → Documents current layer rules (discovered)
anti-patterns.md → Customized for current tech stack
```

**Important:** Files reflect reality, not ideals.

### For "Gradual Migration"

Create context files documenting BOTH states:

```
tech-stack.md → Lists BOTH old (existing) and new (for new code)
  - Sections for "New Code" and "Existing Code"
  - Clear rules for which to use when

source-tree.md → Documents target structure
  - New code follows new structure
  - Existing code grandfathered in old structure

dependencies.md → Lists BOTH old and new packages
  - Mark as "Legacy" or "Standard"

coding-standards.md → Documents target patterns
  - Examples for both old and new approaches
  - Conversion guidelines

architecture-constraints.md → Documents target architecture
  - Allow transitional states
  - Phase-out plan for old patterns

anti-patterns.md → Forbids mixing old and new
  - "Don't use Redux in Zustand components"
  - "Don't mix EF Core and Dapper in same repository"
```

### For "Full Refactor"

Create context files documenting TARGET state:

```
tech-stack.md → Lists target technologies
  - Note: "Refactoring from [old] in progress"

source-tree.md → Documents target structure

dependencies.md → Lists target packages

All files note: "Refactor in progress, current code uses [old tech]"
```

**During refactor:** Update as sections complete.

---

## Common Brownfield Pitfalls

### Pitfall 1: Idealized Context Files

**Problem:**
```
tech-stack.md says: "Use Dapper"
Actual codebase: 100% Entity Framework Core
Result: Development skill halts (violation detected)
```

**Solution:**
Document reality in context files, THEN decide migration.

### Pitfall 2: Assuming Migration is Free

**Problem:**
```
See EF Core in codebase
Think: "Dapper is better"
Create ADR for Dapper
Update tech-stack.md to Dapper
Result: 12-week refactor required, not budgeted
```

**Solution:**
Analyze migration cost BEFORE updating context files. Use AskUserQuestion for migration decisions.

### Pitfall 3: No Technical Debt Documentation

**Problem:**
```
Discover: Mix of patterns, duplicate dependencies, outdated versions
Context files: Don't mention technical debt
Result: Debt hidden, never addressed
```

**Solution:**
Always document technical debt in context files with reduction plan.

---

## Success Criteria

Brownfield integration succeeds when:

- [ ] All 6 context files created
- [ ] Context files match actual codebase state
- [ ] Gap analysis documented
- [ ] Migration strategy decided (via AskUserQuestion)
- [ ] Technical debt identified and documented
- [ ] ADRs created for any architecture changes
- [ ] Context files include transitional states (if gradual migration)
- [ ] Validation passes (spec respects context constraints)

**Output:** Context files reflecting brownfield reality, ready for development.
