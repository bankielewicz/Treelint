# Phase 2: Context File Creation Workflow

Create all 6 immutable context files that define architectural boundaries AI agents must never violate.

## Overview

This phase generates the 6 context files that serve as "the law" for the DevForgeAI framework:

1. **tech-stack.md** - Locks technology choices (prevents library substitution)
2. **source-tree.md** - Enforces project structure (prevents chaos)
3. **dependencies.md** - Locks approved packages (prevents unapproved additions)
4. **coding-standards.md** - Documents code patterns (enforces consistency)
5. **architecture-constraints.md** - Enforces layer boundaries (prevents violations)
6. **anti-patterns.md** - Explicitly forbids patterns (prevents technical debt)

**Critical Principle:** These files are IMMUTABLE constraints. Changes require ADRs and user approval.

---

## General Workflow (All 6 Files)

For each context file:

1. **Load template** from `assets/context-templates/{filename}`
2. **Gather decisions** via AskUserQuestion (technology, patterns, rules)
3. **Customize template** with project-specific information
4. **Add enforcement rules** with ✅ CORRECT vs ❌ FORBIDDEN examples
5. **Validate completeness** (no placeholders, all sections filled)
6. **Write to disk** in `devforgeai/specs/context/{filename}`

---

## File 1: tech-stack.md

**Purpose:** Lock technology choices to prevent substitution.

### Template Loading

```
Read(file_path=".claude/skills/devforgeai-architecture/assets/context-templates/tech-stack.md")
```

### Step 2.0.5: Research-Based Technology Evaluation (NEW - STORY-036)

**Purpose:** Invoke internet-sleuth agent for comparative technology analysis before presenting technology choices to user.

**When to invoke research:**
```python
# Determine if technology research needed
research_needed = (
    multiple_valid_options()  # More than one viable technology for category
    OR technology_unfamiliar()  # Team lacks experience with candidate technologies
    OR context_files_missing()  # Greenfield mode, no existing tech decisions
)

if research_needed:
    invoke_internet_sleuth_for_tech_evaluation()
else:
    skip_research()  # Use existing tech-stack.md or obvious choice
```

**Research Invocation (Before Technology Selection):**
```python
# Example: Backend technology evaluation
backend_research = Task(
  subagent_type="internet-sleuth",
  description="Backend technology evaluation",
  prompt=f"""
  Research Mode: repository-archaeology
  Research Scope: Backend framework comparison for {project_description}
  Context: Epic {epic_id} (if applicable), Workflow State: Architecture
  Required Outputs:
    - Top 3 backend frameworks with implementation patterns
    - GitHub repositories (quality score ≥7) demonstrating production usage
    - Common pitfalls + mitigation strategies
    - Performance benchmarks (if available)

  Comparison Dimensions:
    - C# .NET 8.0
    - Python FastAPI
    - Node.js Express
    - Java Spring Boot

  Constraints:
    - Respect existing tech-stack.md (if brownfield)
    - Team expertise: {team_skills}
    - Project requirements: {requirements_summary}
  """
)

# Parse research results
top_backend = backend_research.top_recommendations[0]
backend_alternatives = backend_research.top_recommendations[1:3]

# Display research summary before AskUserQuestion
display(f"""
Research Completed (Backend):
  ✓ Research ID: {backend_research.research_id}
  ✓ Top Recommendation: {top_backend.approach} (Score: {top_backend.feasibility_score}/10)
  ✓ Rationale: {top_backend.pros[0]}
  ✓ Report: {backend_research.report_path}
""")
```

**Incorporate Research into AskUserQuestion:**
```python
# Use research findings to populate option descriptions
backend_question = AskUserQuestion(
    questions=[{
        question: "Based on research, which backend technology stack should this project use?",
        header: "Backend stack",
        multiSelect: false,
        options: [
            {
                label: f"{top_backend.approach} (Recommended ⭐)",
                description: f"Score: {top_backend.feasibility_score}/10. {top_backend.pros[0]}. {top_backend.cons[0] if top_backend.cons else ''}"
            },
            {
                label: backend_alternatives[0].approach,
                description: f"Score: {backend_alternatives[0].feasibility_score}/10. {backend_alternatives[0].pros[0]}"
            },
            {
                label: backend_alternatives[1].approach,
                description: f"Score: {backend_alternatives[1].feasibility_score}/10. {backend_alternatives[1].pros[0]}"
            }
        ]
    }]
)
```

**Benefits:**
- ✅ User sees evidence-based recommendations (not arbitrary options)
- ✅ Technology scores visible (helps decision-making)
- ✅ Research report available for detailed review
- ✅ Framework compliance already validated (quality gate passed)
- ✅ ADR evidence pre-collected (research includes rationale, alternatives, consequences)

**Add Research Reference to tech-stack.md:**
```markdown
## Backend
- **Framework:** {selected_backend}
- **Rationale:** {top_backend.pros[0]}
- **Research:** [{backend_research.research_id}]({backend_research.report_path})
- **Alternatives Considered:** {backend_alternatives[0].approach}, {backend_alternatives[1].approach}
```

**Repository Pattern Integration:**
```markdown
## Implementation Patterns (from Repository Archaeology)

**Source:** {top_backend.repositories[0].name} (Quality: {top_backend.repositories[0].quality_score}/10, {top_backend.repositories[0].stars} stars)

**Recommended Pattern:** {top_backend.code_patterns[0].pattern_name}

```python
{top_backend.code_patterns[0].code_example}
```

**When to use:** {top_backend.code_patterns[0].applicability}
**Reference:** {top_backend.repositories[0].url}
```

**Fallback if research unavailable:**
```python
if research_failed or research_skipped:
    # Use standard AskUserQuestion without research context
    display("⚠️ Proceeding without research (manual technology selection)")

    backend_question = AskUserQuestion(
        questions=[{
            question: "What backend technology stack should this project use?",
            header: "Backend stack",
            options: [
                {label: "C# with .NET 8.0", description: "Enterprise-grade, statically typed"},
                {label: "Python with FastAPI", description: "Modern, async, great DX"},
                {label: "Node.js with Express", description: "JavaScript ecosystem, large community"},
                {label: "Java with Spring Boot", description: "Battle-tested, Spring ecosystem"}
            ]
        }]
    )
```

---

### Technology Decisions (via AskUserQuestion)

#### Backend Technology

```
Question: "What backend technology stack should this project use?"
Header: "Backend stack"
Options:
  - "C# with .NET 8.0"
  - "Python with FastAPI"
  - "Node.js with Express"
  - "Java with Spring Boot"
multiSelect: false
```

#### Database Technology

```
Question: "Which database should this project use?"
Header: "Database"
Options:
  - "Microsoft SQL Server"
  - "PostgreSQL"
  - "MySQL"
  - "MongoDB"
multiSelect: false
```

#### ORM/Data Access (if SQL database selected)

```
Question: "Which ORM or data access library should be used?"
Header: "ORM"
Description: "This choice will be LOCKED. AI agents cannot substitute alternatives."
Options:
  - "Dapper (micro-ORM, explicit SQL)"
  - "Entity Framework Core (full ORM)"
  - "NHibernate"
  - "ADO.NET (no ORM)"
multiSelect: false
```

#### Frontend Framework

```
Question: "Which frontend framework should this project use?"
Header: "Frontend"
Options:
  - "React with TypeScript"
  - "Vue.js with TypeScript"
  - "Angular"
  - "Svelte"
multiSelect: false
```

#### State Management (if React selected)

```
Question: "Which state management library for React?"
Header: "State mgmt"
Description: "This will prevent AI from introducing Redux/MobX alternatives."
Options:
  - "Zustand (lightweight, simple)"
  - "Redux Toolkit (full-featured)"
  - "Jotai (atomic state)"
  - "React Context API (built-in)"
multiSelect: false
```

### Customization Strategy

After gathering all decisions:

1. **Replace placeholders** in template with chosen technologies
2. **Add CRITICAL RULE sections** for each technology:
   ```markdown
   ## Backend: C# with .NET 8.0 (LOCKED)

   **CRITICAL RULE:** AI agents MUST use C# and .NET 8.0 for all backend code.

   ❌ FORBIDDEN:
   - Switching to Python/Node.js for "simpler" features
   - Using older .NET versions (6.0, 7.0)
   - Mixing languages within backend

   ✅ CORRECT:
   - All backend services use .NET 8.0
   - Use AskUserQuestion if requirements seem incompatible
   ```

3. **Document PROHIBITED alternatives** explicitly:
   ```markdown
   ## Data Access: Dapper 2.1.x (LOCKED - NOT Entity Framework)

   **PROHIBITED:**
   - Entity Framework Core (even if "easier" for relationships)
   - NHibernate
   - Any other ORM

   **Rationale:** Team preference for explicit SQL control.
   See ADR-002-orm-selection.md for full decision context.
   ```

4. **Add Ambiguity Resolution Protocol:**
   ```markdown
   ## Ambiguity Resolution Protocol

   When encountering technology decisions not documented in this file:

   1. **HALT** - Do not make assumptions
   2. **Check** dependencies.md for approved packages
   3. **Ask** via AskUserQuestion with technology options
   4. **Document** decision in this file + create ADR
   5. **Proceed** only after user approval
   ```

5. **Write to disk:**
   ```
   Write(file_path="devforgeai/specs/context/tech-stack.md", content="...")
   ```

---

## File 2: source-tree.md

**Purpose:** Define project structure to prevent chaos.

### Template Loading

```
Read(file_path=".claude/skills/devforgeai-architecture/assets/context-templates/source-tree.md")
```

### Structure Decisions (via AskUserQuestion)

#### Backend Architecture Pattern

```
Question: "Which backend architecture pattern should be used?"
Header: "Architecture"
Options:
  - "Clean Architecture (Domain/Application/Infrastructure layers)"
  - "N-Tier (Presentation/Business/Data layers)"
  - "Vertical Slice (feature-based organization)"
  - "Simple layered (minimal structure)"
multiSelect: false
```

#### Test Organization

```
Question: "How should tests be organized?"
Header: "Test structure"
Options:
  - "Mirror source structure (tests/UnitTests/Application.Tests/)"
  - "Separate by test type (tests/Unit/, tests/Integration/)"
  - "Co-located with source (src/Module/Module.Tests/)"
multiSelect: false
```

### Customization Strategy

After gathering structure decisions:

1. **Customize template** with chosen architecture pattern
2. **Document folder hierarchy:**
   ```markdown
   ## Project Structure (Clean Architecture)

   ```
   src/
   ├── Domain/              # Business logic (no infrastructure)
   │   ├── Entities/        # Domain entities
   │   ├── ValueObjects/    # Value objects
   │   └── Interfaces/      # Repository interfaces
   ├── Application/         # Use cases and orchestration
   │   ├── Services/        # Application services
   │   ├── DTOs/            # Data transfer objects
   │   └── Validators/      # FluentValidation rules
   └── Infrastructure/      # External integrations
       ├── Persistence/     # Dapper repositories
       ├── External/        # Third-party APIs
       └── Configuration/   # Startup configuration
   ```
   ```

3. **Add naming conventions:**
   ```markdown
   ## Naming Conventions

   - **Classes:** PascalCase (UserService, ProductRepository)
   - **Methods:** PascalCase (GetUserById, CreateProduct)
   - **Variables:** camelCase (userId, productName)
   - **Constants:** UPPER_SNAKE_CASE (MAX_RETRIES, API_TIMEOUT)
   - **Interfaces:** I prefix (IUserRepository, IProductService)
   - **Files:** Match class name (UserService.cs, IUserRepository.cs)
   ```

4. **Define file placement rules:**
   ```markdown
   ## File Placement Rules

   | File Type | Location | Example |
   |-----------|----------|---------|
   | Domain Entity | `src/Domain/Entities/` | `User.cs` |
   | Application Service | `src/Application/Services/` | `UserService.cs` |
   | Repository Implementation | `src/Infrastructure/Persistence/` | `UserRepository.cs` |
   | Repository Interface | `src/Domain/Interfaces/` | `IUserRepository.cs` |
   | DTO | `src/Application/DTOs/` | `UserDto.cs` |
   | Validator | `src/Application/Validators/` | `CreateUserValidator.cs` |
   | API Controller | `src/API/Controllers/` | `UsersController.cs` |
   ```

5. **Add enforcement checklist:**
   ```markdown
   ## Enforcement Checklist

   Before creating any file, verify:
   - [ ] File location matches table above
   - [ ] Naming convention followed
   - [ ] One class per file (except nested classes)
   - [ ] Namespace matches folder structure
   ```

6. **Write to disk:**
   ```
   Write(file_path="devforgeai/specs/context/source-tree.md", content="...")
   ```

---

## File 3: dependencies.md

**Purpose:** Lock approved packages to prevent substitution.

### Template Loading

```
Read(file_path=".claude/skills/devforgeai-architecture/assets/context-templates/dependencies.md")
```

### Dependency Decisions (via AskUserQuestion)

#### For Greenfield Projects

Gather initial dependencies for each technology layer:

**Migration Tool:**
```
Question: "Which migration tool should be used for database schema management?"
Header: "Migrations"
Options:
  - "FluentMigrator (code-first migrations)"
  - "DbUp (script-based migrations)"
  - "Entity Framework Migrations (if using EF Core)"
  - "Manual SQL scripts (no migration tool)"
multiSelect: false
```

**Validation Library:**
```
Question: "Which validation library should be used?"
Header: "Validation"
Options:
  - "FluentValidation (expressive, testable)"
  - "DataAnnotations (built-in, simpler)"
  - "Custom validation (no library)"
multiSelect: false
```

**Testing Framework:**
```
Question: "Which testing framework should be used?"
Header: "Test framework"
Options:
  - "xUnit (modern, extensible)"
  - "NUnit (mature, feature-rich)"
  - "MSTest (Microsoft's framework)"
multiSelect: false
```

**Mocking Library:**
```
Question: "Which mocking library for unit tests?"
Header: "Mocking"
Options:
  - "NSubstitute (clean syntax)"
  - "Moq (popular, mature)"
  - "FakeItEasy (expressive API)"
multiSelect: false
```

**E2E Testing:**
```
Question: "Which E2E testing framework for frontend?"
Header: "E2E testing"
Options:
  - "Playwright (cross-browser, modern)"
  - "Cypress (developer-friendly)"
  - "Selenium (established standard)"
multiSelect: false
```

#### For Brownfield Projects

**Discover existing dependencies:**

```
# Extract .NET dependencies
Grep(pattern="<PackageReference", glob="**/*.csproj", output_mode="content")

# Extract npm dependencies
Read(file_path="package.json")

# Check for lock files
Read(file_path="package-lock.json")  # or yarn.lock, pnpm-lock.yaml

# Extract Python dependencies
Read(file_path="requirements.txt")
Read(file_path="Pipfile")

# Extract Java dependencies
Read(file_path="pom.xml")
Read(file_path="build.gradle")
```

### Customization Strategy

#### Greenfield Approach

After gathering decisions:

1. **Populate dependencies.md** with approved packages and versions
2. **Mark LOCKED packages:**
   ```markdown
   ## Backend Core (LOCKED)

   - **Dapper** 2.1.28 (CRITICAL - Data access, NOT Entity Framework)
   - **FluentValidation** 11.8.0 (Validation library)
   - **FluentMigrator** 5.0.0 (Database migrations)
   - **Serilog** 3.1.0 (Structured logging)
   ```

3. **Add CRITICAL comments:**
   ```markdown
   **CRITICAL:** Dapper is LOCKED. AI agents must NOT substitute Entity Framework Core
   or any other ORM, even if relationships seem "easier" with EF Core.

   Rationale: Team preference for explicit SQL control and performance.
   See ADR-002-orm-selection.md for full context.
   ```

4. **Document FORBIDDEN alternatives:**
   ```markdown
   ## FORBIDDEN Packages

   - **Entity Framework Core** - Use Dapper instead (see ADR-002)
   - **Redux** - Use Zustand instead (see ADR-003)
   - **Moment.js** - Deprecated, use date-fns or Day.js
   - **Request** - Deprecated, use Axios or Fetch API
   ```

5. **Include Dependency Addition Protocol:**
   ```markdown
   ## Dependency Addition Protocol

   Before adding ANY package, AI agents MUST:

   1. **Check** if package is listed in dependencies.md
   2. **If listed** → Use exact version specified
   3. **If NOT listed** → STOP and use AskUserQuestion:
      ```
      Question: "I need to add package [Name] for [functionality].
                It's not in dependencies.md. Should I add it?"
      Header: "New package"
      Options:
        - "Yes, add [Name] version [X.Y.Z]"
        - "No, use existing dependency [Alternative]"
        - "No, implement manually without external dependency"
      Description: "This will update dependencies.md and require ADR"
      multiSelect: false
      ```
   4. **After approval** → Update dependencies.md + create ADR
   ```

#### Brownfield Approach

After discovery:

1. **Populate dependencies.md** with current packages
2. **Identify core packages** that should be LOCKED
3. **Add rationale** for each major dependency
4. **Document technical debt:**
   ```markdown
   ## Technical Debt Identified

   - **Duplicate logging** - Uses both Serilog and NLog (consolidate to Serilog)
   - **Outdated versions** - FluentValidation 9.5.0 (upgrade to 11.8.0)
   - **Unused packages** - AutoMapper referenced but not used (remove)
   ```

5. **Create migration plan** if dependencies need updating
6. **Write to disk:**
   ```
   Write(file_path="devforgeai/specs/context/dependencies.md", content="...")
   ```

---

## File 4: coding-standards.md

**Purpose:** Document HOW to write code for this specific project.

### Template Loading

```
Read(file_path=".claude/skills/devforgeai-architecture/assets/context-templates/coding-standards.md")
```

### Coding Pattern Decisions (via AskUserQuestion)

**Async/Await Standards:**
```
Question: "Should async methods use ConfigureAwait(false) in library code?"
Header: "ConfigureAwait"
Options:
  - "Yes, always use ConfigureAwait(false) in library code"
  - "No, ASP.NET Core doesn't need it"
  - "Use only in libraries, not in application code"
multiSelect: false
```

**Dependency Injection Lifetime:**
```
Question: "What is the default service lifetime for most application services?"
Header: "DI lifetime"
Options:
  - "Scoped (per HTTP request - recommended for most services)"
  - "Transient (new instance each time)"
  - "Singleton (single instance for app lifetime)"
Description: "This will be documented as the standard pattern"
multiSelect: false
```

**Error Handling Pattern:**
```
Question: "Which error handling pattern should be used?"
Header: "Error handling"
Options:
  - "Result Pattern (return Result<T> for business logic errors)"
  - "Exceptions (throw exceptions for all errors)"
  - "Hybrid (Result for business logic, exceptions for technical errors)"
multiSelect: false
```

**Logging Library:**
```
Question: "Which logging library should be used?"
Header: "Logging"
Options:
  - "Serilog (structured logging, multiple sinks)"
  - "NLog (mature, configurable)"
  - "Microsoft.Extensions.Logging (built-in)"
multiSelect: false
```

### Customization Strategy

After gathering coding pattern decisions:

1. **Customize template** with technology-specific patterns
2. **Add ✅ CORRECT vs ❌ FORBIDDEN examples:**

   **Example for Dapper:**
   ```markdown
   ## Data Access Patterns (Dapper)

   ### ✅ CORRECT: Parameterized Queries

   ```csharp
   // CORRECT - Prevents SQL injection
   var users = await connection.QueryAsync<User>(
       "SELECT * FROM Users WHERE Status = @Status",
       new { Status = status });
   ```

   ### ❌ FORBIDDEN: String Concatenation

   ```csharp
   // FORBIDDEN - SQL injection vulnerability
   var sql = $"SELECT * FROM Users WHERE Status = '{status}'";
   var users = await connection.QueryAsync<User>(sql);
   ```
   ```

   **Example for Zustand:**
   ```markdown
   ## State Management Patterns (Zustand)

   ### ✅ CORRECT: Direct Action Methods

   ```typescript
   // CORRECT - Zustand's simple approach
   const useStore = create((set) => ({
     count: 0,
     increment: () => set((state) => ({ count: state.count + 1 }))
   }));
   ```

   ### ❌ FORBIDDEN: Redux-Style Actions

   ```typescript
   // FORBIDDEN - Don't bring Redux patterns to Zustand
   const INCREMENT = 'INCREMENT';
   const increment = () => ({ type: INCREMENT });
   ```
   ```

3. **Document patterns for:**
   - Data access (ORM-specific patterns)
   - State management (Library-specific patterns)
   - Async/await conventions
   - Dependency injection
   - Validation
   - Error handling and logging
   - Naming conventions
   - File organization

4. **Add AI Agent Integration Rules:**
   ```markdown
   ## AI Agent Integration Rules

   When generating code, AI agents MUST:

   1. Follow ALL patterns in this document
   2. Use ✅ CORRECT examples as templates
   3. NEVER use ❌ FORBIDDEN patterns
   4. Check dependencies.md for approved packages
   5. Use AskUserQuestion if pattern not documented
   ```

5. **Add Pattern Enforcement Checklist:**
   ```markdown
   ## Pattern Enforcement Checklist

   Before completing any code generation:
   - [ ] All async methods follow ConfigureAwait rules
   - [ ] All DI registrations use correct lifetime
   - [ ] All error handling follows chosen pattern
   - [ ] All data access uses parameterized queries
   - [ ] All validation uses FluentValidation patterns
   - [ ] Naming conventions followed
   ```

6. **Write to disk:**
   ```
   Write(file_path="devforgeai/specs/context/coding-standards.md", content="...")
   ```

---

## File 5: architecture-constraints.md

**Purpose:** Enforce layer boundaries and design patterns.

### Template Loading

```
Read(file_path=".claude/skills/devforgeai-architecture/assets/context-templates/architecture-constraints.md")
```

### Architecture Decisions (via AskUserQuestion)

**Layer Dependency Rules (for Clean Architecture):**
```
Question: "Should the Domain layer be allowed to reference any infrastructure concerns?"
Header: "Domain purity"
Options:
  - "No, Domain must be 100% pure (no infrastructure references)"
  - "Allow minimal infrastructure (e.g., IRepository interfaces)"
Description: "This enforces Clean Architecture principles"
multiSelect: false
```

**Data Access Pattern:**
```
Question: "Should all data access go through repositories, or allow direct database access from services?"
Header: "Data access"
Options:
  - "Repository Pattern (all data access through repositories)"
  - "Direct access (services can query database directly)"
  - "Hybrid (repositories for complex queries, direct for simple)"
multiSelect: false
```

**DTO Usage:**
```
Question: "Should API controllers expose domain entities directly, or use DTOs?"
Header: "API pattern"
Options:
  - "DTO Pattern (never expose domain entities to API)"
  - "Direct entities (controllers can return domain objects)"
  - "Hybrid (DTOs for input, entities for output)"
Description: "DTOs prevent over-posting and decouple API from domain"
multiSelect: false
```

**Service Layer Pattern:**
```
Question: "Should business logic be in controllers or services?"
Header: "Business logic"
Options:
  - "Service Pattern (controllers are thin, services contain logic)"
  - "Controller-heavy (controllers can contain business logic)"
Description: "Service pattern improves testability and reusability"
multiSelect: false
```

**Transaction Management:**
```
Question: "Where should database transactions be managed?"
Header: "Transactions"
Options:
  - "Service layer (orchestrates multiple repositories)"
  - "Repository layer (each repository manages its own transactions)"
  - "Controller layer (API endpoints manage transactions)"
multiSelect: false
```

### Customization Strategy

After gathering architecture decisions:

1. **Customize template** with chosen patterns
2. **Document layer dependency matrix:**

   ```markdown
   ## Layer Dependency Matrix (Clean Architecture)

   | From ↓ To → | API | Application | Domain | Infrastructure |
   |-------------|-----|-------------|--------|----------------|
   | API         | ✓   | ✓           | ❌     | ❌             |
   | Application | ❌  | ✓           | ✓      | ❌             |
   | Domain      | ❌  | ❌          | ✓      | ❌             |
   | Infrastructure | ❌ | ❌        | ✓      | ✓              |

   **Legend:**
   - ✓ = Allowed (can reference)
   - ❌ = Forbidden (cannot reference)
   ```

3. **Add mandatory patterns:**

   **Repository Pattern (if chosen):**
   ```markdown
   ## Repository Pattern (MANDATORY)

   All data access MUST go through repositories.

   ✅ CORRECT:
   ```csharp
   public class UserService
   {
       private readonly IUserRepository _repository;

       public async Task<User> GetUserAsync(int id)
       {
           return await _repository.GetByIdAsync(id);
       }
   }
   ```

   ❌ FORBIDDEN:
   ```csharp
   public class UserService
   {
       private readonly IDbConnection _connection;

       // FORBIDDEN - Services should not access database directly
       public async Task<User> GetUserAsync(int id)
       {
           return await _connection.QuerySingleAsync<User>(...);
       }
   }
   ```
   ```

   **DTO Pattern (if chosen):**
   ```markdown
   ## DTO Pattern (MANDATORY)

   API controllers MUST use DTOs, never expose domain entities.

   ✅ CORRECT:
   ```csharp
   [HttpPost]
   public async Task<ActionResult<UserDto>> CreateUser(CreateUserDto dto)
   {
       var user = _mapper.Map<User>(dto);
       await _service.CreateAsync(user);
       return _mapper.Map<UserDto>(user);
   }
   ```

   ❌ FORBIDDEN:
   ```csharp
   [HttpPost]
   public async Task<ActionResult<User>> CreateUser(User user)
   {
       // FORBIDDEN - Exposes domain entity to API
       await _service.CreateAsync(user);
       return user;
   }
   ```
   ```

4. **Add forbidden patterns:**
   ```markdown
   ## Forbidden Patterns

   ### Cross-Layer Violations

   ❌ FORBIDDEN: Domain → Infrastructure
   ```csharp
   // FORBIDDEN in Domain/Entities/User.cs
   using Infrastructure.Persistence; // VIOLATION
   ```

   ### Business Logic in Controllers

   ❌ FORBIDDEN (if Service Pattern chosen):
   ```csharp
   [HttpPost]
   public async Task<ActionResult> CreateUser(CreateUserDto dto)
   {
       // FORBIDDEN - Business logic in controller
       if (dto.Age < 18) return BadRequest();
       if (!_repository.IsEmailUnique(dto.Email)) return Conflict();
       // ... more business logic
   }
   ```

   ✅ CORRECT:
   ```csharp
   [HttpPost]
   public async Task<ActionResult> CreateUser(CreateUserDto dto)
   {
       // CORRECT - Delegate to service
       var result = await _service.CreateUserAsync(dto);
       return result.ToActionResult();
   }
   ```
   ```

5. **Add architecture unit tests examples:**
   ```markdown
   ## Architecture Tests (NetArchTest.Rules)

   Use these tests to enforce constraints automatically:

   ```csharp
   [Fact]
   public void Domain_Should_Not_Reference_Infrastructure()
   {
       var result = Types.InAssembly(DomainAssembly)
           .Should()
           .NotHaveDependencyOn("Infrastructure")
           .GetResult();

       Assert.True(result.IsSuccessful);
   }

   [Fact]
   public void Controllers_Should_Not_Contain_Business_Logic()
   {
       var result = Types.InAssembly(ApiAssembly)
           .That().ResideInNamespace("API.Controllers")
           .Should().NotHaveDependencyOn("Domain.Entities")
           .GetResult();

       Assert.True(result.IsSuccessful);
   }
   ```
   ```

6. **Write to disk:**
   ```
   Write(file_path="devforgeai/specs/context/architecture-constraints.md", content="...")
   ```

---

## File 6: anti-patterns.md

**Purpose:** Explicitly forbid patterns that cause technical debt.

### Template Loading

```
Read(file_path=".claude/skills/devforgeai-architecture/assets/context-templates/anti-patterns.md")
```

### Customization Strategy

**Customize based on project context:**

1. **Add project-specific anti-patterns:**

   **For projects using Dapper:**
   ```markdown
   ### Category 1: Library Substitution (SEVERITY: CRITICAL)

   #### Anti-Pattern: Switching from Dapper to Entity Framework

   ❌ FORBIDDEN: AI encounters complex Dapper query → Suggests EF Core → Adds EF packages

   **Wrong Behavior:**
   ```csharp
   // AI sees complex join in Dapper
   var query = "SELECT u.*, o.* FROM Users u JOIN Orders o ON u.Id = o.UserId";

   // AI thinks: "This would be easier with EF Core relationships"
   // AI adds: Microsoft.EntityFrameworkCore package
   // AI creates: DbContext and entity configurations
   // RESULT: Mixed data access approaches = technical debt
   ```

   **Correct Behavior:**
   AI checks tech-stack.md → Sees Dapper is LOCKED → Uses AskUserQuestion:
   ```
   Question: "Complex join query needed. Dapper is locked in tech-stack.md.
             Should I use Dapper multi-mapping or request EF Core?"
   Header: "Data access"
   Options:
     - "Use Dapper multi-mapping (respects constraint)"
     - "Request EF Core exception (requires ADR + approval)"
   ```
   ```

   **For projects using Zustand:**
   ```markdown
   ### Category 4: Framework Mixing (SEVERITY: CRITICAL)

   #### Anti-Pattern: Introducing Redux patterns in Zustand project

   ❌ FORBIDDEN: AI needs complex state → Adds Redux Toolkit → Creates mixed state management

   **Wrong Behavior:**
   ```typescript
   // AI sees complex state management need
   // AI thinks: "Redux has better devtools and middleware"
   // AI adds: @reduxjs/toolkit package
   // AI creates: Redux store alongside Zustand stores
   // RESULT: Two state management libraries = confusion
   ```

   **Correct Behavior:**
   AI checks dependencies.md → Sees Zustand is LOCKED → Uses AskUserQuestion:
   ```
   Question: "Complex state management needed (async actions, middleware).
             Zustand is locked. How should I proceed?"
   Header: "State management"
   Options:
     - "Use Zustand middleware (immer, persist, devtools)"
     - "Request Redux exception (requires ADR + approval)"
   ```
   ```

   **For projects using Clean Architecture:**
   ```markdown
   ### Category 3: Cross-Layer Dependency Violations (SEVERITY: HIGH)

   #### Anti-Pattern: Domain layer referencing Infrastructure

   ❌ FORBIDDEN:
   ```csharp
   // Domain/Entities/User.cs
   using Infrastructure.Persistence; // VIOLATION

   public class User
   {
       public int Id { get; set; }
       // Domain entity should not know about persistence
       public UserRepository Repository { get; set; } // WRONG
   }
   ```

   ✅ CORRECT:
   ```csharp
   // Domain/Entities/User.cs
   // No infrastructure references - pure domain

   public class User
   {
       public int Id { get; set; }
       public string Name { get; set; }
       // Business logic only
   }

   // Domain/Interfaces/IUserRepository.cs
   public interface IUserRepository
   {
       Task<User> GetByIdAsync(int id);
   }

   // Infrastructure implements interface
   ```
   ```

2. **Add all 10 anti-pattern categories:**
   - Library Substitution
   - Structure Violation
   - Cross-Layer Dependencies
   - Framework Mixing
   - Magic Numbers/Strings
   - God Objects
   - Tight Coupling
   - Security Anti-Patterns
   - Performance Anti-Patterns
   - Test Anti-Patterns

3. **For each category, provide:**
   - ❌ FORBIDDEN example (what NOT to do)
   - ✅ CORRECT example (what to do instead)
   - AskUserQuestion pattern for when pattern is encountered
   - Severity level (CRITICAL, HIGH, MEDIUM, LOW)

4. **Add Detection and Prevention checklist:**
   ```markdown
   ## Detection and Prevention Checklist

   Before generating/modifying code:
   - [ ] Check tech-stack.md for locked technologies
   - [ ] Check dependencies.md for approved packages
   - [ ] Check source-tree.md for file placement
   - [ ] Check architecture-constraints.md for layer rules
   - [ ] Review this file for forbidden patterns
   - [ ] Use AskUserQuestion if uncertain
   ```

5. **Add AI Agent Anti-Pattern Avoidance Protocol:**
   ```markdown
   ## AI Agent Anti-Pattern Avoidance Protocol

   When generating code:

   1. **Before adding any import/using:**
      - Check if package is in dependencies.md
      - If NOT → STOP and use AskUserQuestion

   2. **Before creating any file:**
      - Check source-tree.md for correct location
      - If unclear → STOP and use AskUserQuestion

   3. **Before accessing any layer:**
      - Check architecture-constraints.md dependency matrix
      - If violation detected → STOP and use AskUserQuestion

   4. **Before suggesting technology alternative:**
      - Check tech-stack.md to see if locked
      - If locked → STOP and use AskUserQuestion

   5. **When encountering anti-pattern from this file:**
      - HALT code generation
      - Use AskUserQuestion with:
        - Current situation
        - Why pattern is problematic
        - Alternatives that respect constraints
   ```

6. **Write to disk:**
   ```
   Write(file_path="devforgeai/specs/context/anti-patterns.md", content="...")
   ```

---

## File 7: design-system.md (OPTIONAL - UI Projects Only)

**Purpose:** Define consistent visual design patterns for frontend projects.

**When to create:**
- Project includes web UI, mobile app, or desktop GUI
- Tech stack includes frontend frameworks (React, Vue, Angular, etc.)
- Team requires consistent design language across components

**When to skip:**
- Backend-only projects (APIs, microservices, CLI tools)
- User declines design system during context creation
- Project uses existing design system (document reference instead)

### Template Loading

```
Read(file_path=".claude/skills/devforgeai-architecture/assets/context-templates/design-system.md")
```

### UI Project Detection

**Check tech-stack.md for frontend technologies:**

```
Read(file_path="devforgeai/specs/context/tech-stack.md")
```

**Frontend indicators:**
- React, Vue, Angular, Svelte, Solid
- Next.js, Nuxt, SvelteKit, Remix
- React Native, Flutter, Ionic, Electron
- Tailwind CSS, styled-components, Emotion, CSS Modules

**If NO frontend technology detected:**
- Skip design-system.md creation
- Continue to next phase

### Design System Preferences (via AskUserQuestion)

**If frontend technology detected, ask:**

```
AskUserQuestion:
  Question: "This is a UI project. Would you like to generate a design system?"
  Header: "Design System"
  Options:
    - label: "Yes, custom design system"
      description: "Create custom design tokens (colors, typography, spacing, shadows)"
    - label: "Yes, use popular framework"
      description: "Document framework choice (Material UI, Chakra UI, shadcn/ui, Ant Design)"
    - label: "No, skip design system"
      description: "Project can proceed without design system constraints"
```

### Option 1: Custom Design System

**If user selects custom design system, gather preferences:**

```
AskUserQuestion:
  Question: "Select design system preferences (multiple questions)"

  Part 1 - Color Palette:
    - Primary/secondary/accent colors (brand colors)
    - Neutral scale (grays for text/backgrounds)
    - Semantic colors (success, warning, error, info)

  Part 2 - Typography:
    - Font families (heading, body, monospace)
    - Size scale (base size + ratio)
    - Line heights and letter spacing

  Part 3 - Spacing Scale:
    - Base unit (4px, 8px, or custom)
    - Scale multiplier (linear, fibonacci, or custom)

  Part 4 - Border Radius:
    - None (0px), Small (2-4px), Medium (8px), Large (16px+), Custom

  Part 5 - Shadow Style:
    - Flat (no shadows), Soft (subtle), Material (elevation-based), Custom
```

**Customize template with user preferences:**

1. **Replace color placeholders** with brand colors:
   ```yaml
   primary:
     main: "#[USER_PRIMARY_COLOR]"
     light: "#[GENERATED_LIGHT_VARIANT]"
     dark: "#[GENERATED_DARK_VARIANT]"
   ```

2. **Update typography** with chosen fonts:
   ```yaml
   font_families:
     heading: "[USER_HEADING_FONT]"
     body: "[USER_BODY_FONT]"
     monospace: "[USER_MONOSPACE_FONT]"
   ```

3. **Configure spacing scale** based on base unit:
   ```yaml
   spacing:
     xs: "[BASE_UNIT * 1]"
     sm: "[BASE_UNIT * 2]"
     md: "[BASE_UNIT * 4]"
     lg: "[BASE_UNIT * 6]"
     xl: "[BASE_UNIT * 8]"
   ```

4. **Add component patterns** relevant to tech stack:
   - React: Component composition guidelines
   - Vue: Single File Component patterns
   - Angular: Module/component structure
   - Tailwind: Utility class conventions

5. **Include accessibility standards:**
   - WCAG 2.1 Level AA minimum
   - Color contrast ratios (4.5:1 text, 3:1 UI)
   - Keyboard navigation requirements
   - Screen reader compatibility

### Option 2: Framework-Based Design

**If user selects framework-based design:**

1. **Document framework choice** in tech-stack.md:
   ```markdown
   ### UI Framework
   - Material UI 5.14.0 (LOCKED - design system provider)
   - Provides design tokens, components, theming
   ```

2. **Add framework to dependencies.md:**
   ```markdown
   ### UI/Design System
   - @mui/material 5.14.0 (LOCKED)
   - @mui/icons-material 5.14.0 (LOCKED)
   ```

3. **Create minimal design-system.md** referencing framework:
   ```markdown
   # Design System

   ## Framework-Based Design

   This project uses **Material UI 5.14.0** for design system.

   **Official Documentation:** https://mui.com/material-ui/

   ### Customization

   Custom theme configuration in `src/theme/`:
   - Primary color: [USER_PRIMARY]
   - Secondary color: [USER_SECONDARY]
   - Typography: [FONT_CHOICES]

   ### Component Standards

   All UI components MUST use Material UI components:
   - Buttons: `<Button variant="contained|outlined|text">`
   - Forms: `<TextField>`, `<Select>`, `<Checkbox>`
   - Layout: `<Container>`, `<Grid>`, `<Stack>`

   ❌ FORBIDDEN: Custom components that duplicate MUI functionality
   ```

### Option 3: Skip Design System

**If user skips design system:**

1. **Do not create design-system.md file**
2. **Add warning to tech-stack.md:**
   ```markdown
   ### UI/Design System

   **Status:** No design system defined
   **Warning:** Frontend consistency may suffer without design tokens
   **Recommendation:** Define design system before implementing UI components
   ```

3. **Continue to next phase** (design system is optional)

### Validation

**If design-system.md created, validate:**

1. **No placeholders:**
   ```
   Grep(pattern="TODO|TBD|\\[FILL IN\\]|\\[TO BE DETERMINED\\]",
        path="devforgeai/specs/context/design-system.md",
        output_mode="files_with_matches")
   ```

2. **Complete sections:**
   - Color palette defined (primary, secondary, neutral)
   - Typography specified (fonts, sizes)
   - Spacing scale documented
   - Component patterns listed
   - Accessibility requirements included

3. **Framework integration:**
   - References tech stack choice
   - Explains how design tokens integrate with framework

### Write to Disk

```
Write(file_path="devforgeai/specs/context/design-system.md", content="[customized template]")
```

---

## Ambiguity Handling During Context File Creation

When technology choices unclear during any of the 6 file workflows:

1. **HALT creation** - Do not make assumptions
2. **Use AskUserQuestion** with:
   - Clear question about the ambiguous choice
   - 2-4 concrete options
   - Description of consequences
3. **Document decision** in appropriate context file
4. **Create ADR** (in Phase 3) for the decision
5. **Proceed** only after user approval

**See `ambiguity-detection-guide.md` for complete list of ambiguity scenarios.**

---

## Output

Phase 2 produces **6 required + 1 optional context files** in `devforgeai/specs/context/`:

**Required (6 files):**
- ✅ tech-stack.md (locked technologies)
- ✅ source-tree.md (project structure)
- ✅ dependencies.md (approved packages)
- ✅ coding-standards.md (code patterns)
- ✅ architecture-constraints.md (layer boundaries)
- ✅ anti-patterns.md (forbidden patterns)

**Optional (1 file):**
- ✅ design-system.md (UI projects only - design tokens and component patterns)

These files now serve as "the law" for all AI agents during development.

**Next Phase:** Move to Phase 3 (Create Architecture Decision Records)
