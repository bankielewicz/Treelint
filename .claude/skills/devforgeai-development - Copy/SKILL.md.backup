---
name: devforgeai-development
description: Implement features using Test-Driven Development (TDD) while enforcing architectural constraints from context files. Use when implementing user stories, building features, or writing code that must comply with tech-stack.md, source-tree.md, and dependencies.md. Automatically invokes devforgeai-architecture skill if context files are missing.
allowed-tools:
  - Read
  - Write
  - Edit
  - Glob
  - Grep
  - AskUserQuestion
  - Bash(git:*)
  - Bash(npm:*)
  - Bash(pytest:*)
  - Bash(dotnet:*)
  - Bash(cargo:*)
  - Bash(mvn:*)
  - Bash(gradle:*)
  - Skill
---

# DevForgeAI Development Skill

Implement features using Test-Driven Development while enforcing architectural constraints to prevent technical debt.

## Purpose

This skill guides feature implementation with:
1. **Context-driven development** - Enforces tech-stack.md, source-tree.md, dependencies.md
2. **TDD workflow** - Red → Green → Refactor cycle with spec validation
3. **Ambiguity resolution** - Uses AskUserQuestion for all unclear implementation decisions
4. **Native tool efficiency** - Uses Read/Edit/Write/Glob/Grep (40-73% token savings vs Bash)
5. **Anti-pattern prevention** - Validates against anti-patterns.md during implementation

## When to Use This Skill

Activate this skill when:
- Implementing user stories or features
- Writing new code for existing projects
- Refactoring code while maintaining specs
- Converting requirements into tested code
- Ensuring code complies with architectural decisions

## Core Principle: Enforce Context, Ask When Ambiguous

**Context files are THE LAW:**
- tech-stack.md → Technology choices (NEVER substitute libraries)
- source-tree.md → File placement rules (NEVER violate structure)
- dependencies.md → Package versions (NEVER add unapproved packages)
- coding-standards.md → Code patterns (ALWAYS follow conventions)
- architecture-constraints.md → Design rules (NEVER cross layer boundaries)
- anti-patterns.md → Forbidden patterns (ALWAYS avoid)

**When context is ambiguous → STOP and use AskUserQuestion**

---

## Development Workflow

### Phase 0: Context Validation (CRITICAL)

Before ANY code is written, validate architectural context exists.

#### 0.1 Check for Context Files

```
# Check for all 6 critical context files
Read(file_path="devforgeai/specs/context/tech-stack.md")
Read(file_path="devforgeai/specs/context/source-tree.md")
Read(file_path="devforgeai/specs/context/dependencies.md")
Read(file_path="devforgeai/specs/context/coding-standards.md")
Read(file_path="devforgeai/specs/context/architecture-constraints.md")
Read(file_path="devforgeai/specs/context/anti-patterns.md")
```

**If ANY file is missing:**

```
# Auto-invoke architecture skill
Skill(command="devforgeai-architecture")
```

**STOP development until context files exist.** This prevents technical debt from ambiguous assumptions.

#### 0.2 Load Story/Feature Specification

```
# Load the story or feature spec
Read(file_path="ai_docs/Stories/[story-id].story.md")
# OR
Read(file_path="docs/specs/[feature-name].spec.md")
```

#### 0.3 Validate Spec Against Context

**Check for conflicts:**

1. **Tech stack conflicts:**
   - Does spec require technology NOT in tech-stack.md?
   - Does spec contradict locked choices?

2. **Dependency conflicts:**
   - Does spec need packages NOT in dependencies.md?
   - Are required versions compatible?

3. **Structure conflicts:**
   - Will new files violate source-tree.md structure?
   - Are naming conventions followed?

**If conflicts detected → Use AskUserQuestion:**

```
Question: "Spec requires [X], but tech-stack.md specifies [Y]. Which is correct?"
Header: "Spec conflict"
Options:
  - "Update spec to use [Y] from tech-stack.md (maintain consistency)"
  - "Update tech-stack.md to [X] and document in ADR (architecture change)"
  - "Use both [X] and [Y] with justification (exceptional case)"
multiSelect: false
```

### Phase 1: Test-First Design (Red Phase)

Following TDD: **Write failing tests BEFORE implementation**

#### 1.1 Analyze Acceptance Criteria

From the story/spec, identify:
- **Functional requirements** (what the code must do)
- **Non-functional requirements** (performance, security, etc.)
- **Edge cases** (error conditions, validation failures)
- **Integration points** (APIs, databases, external services)

#### 1.2 Design Test Cases

For each requirement, design tests at appropriate levels:

**Unit Tests** (business logic, calculations, validation):
```
Test Level: Unit
Scope: Individual methods/functions
Framework: [From dependencies.md - xUnit/NUnit/pytest/Jest]
Example: test_calculate_discount_percentage()
```

**Integration Tests** (database, API, file I/O):
```
Test Level: Integration
Scope: Component interactions
Framework: [From dependencies.md]
Example: test_save_order_to_database()
```

**Contract Tests** (API endpoints):
```
Test Level: Contract
Scope: Request/response validation
Framework: [From dependencies.md - Pact/Spring Cloud Contract]
Example: test_create_order_endpoint_contract()
```

**E2E Tests** (full user flows):
```
Test Level: End-to-End
Scope: Complete workflows
Framework: [From dependencies.md - Playwright/Cypress/Selenium]
Example: test_complete_checkout_flow()
```

#### 1.3 Determine Test File Location

**Consult source-tree.md for test organization:**

```
Read(file_path="devforgeai/specs/context/source-tree.md")
# Find test structure section
```

**Common patterns:**
- Mirror source structure: `tests/UnitTests/Application.Tests/Services/OrderServiceTests.cs`
- Separate by type: `tests/Unit/test_order_service.py`, `tests/Integration/test_order_repository.py`
- Co-located: `src/Services/OrderService.Tests/OrderServiceTests.cs`

**If test location is ambiguous → Use AskUserQuestion:**

```
Question: "Where should tests for [ComponentName] be placed? source-tree.md doesn't specify this pattern."
Header: "Test location"
Options:
  - "tests/Unit/[ComponentName]Tests.[ext] (separate by type)"
  - "src/[Component]/[Component].Tests/[Component]Tests.[ext] (co-located)"
  - "tests/[SourcePath]/[ComponentName]Tests.[ext] (mirror source)"
multiSelect: false
```

#### 1.4 Write Failing Tests

**Use native tools (NOT Bash):**

```
# Check if test file exists
result = Glob(pattern="tests/**/*[ComponentName]Test*")

if test_file_exists:
    Read(file_path="existing_test_file")
    Edit(file_path="existing_test_file",
         old_string="[existing test class]",
         new_string="[existing test class + new test method]")
else:
    Write(file_path="new_test_file",
          content="[test class with failing test]")
```

**Follow coding-standards.md patterns:**

```
Read(file_path="devforgeai/specs/context/coding-standards.md")
# Find test naming conventions, assertion patterns, setup/teardown patterns
```

**Example test structure (C# with xUnit):**

```csharp
public class OrderServiceTests
{
    [Fact]
    public void CalculateDiscount_ValidCoupon_ReturnsDiscountedPrice()
    {
        // Arrange
        var service = new OrderService();
        var order = new Order { Total = 100m };
        var coupon = new Coupon { Code = "SAVE10", Discount = 10 };

        // Act
        var result = service.CalculateDiscount(order, coupon);

        // Assert
        Assert.Equal(90m, result.DiscountedTotal);
    }
}
```

**Run tests to verify they fail:**

```
Bash(command="[test command from dependencies.md]")
# Examples:
# - dotnet test
# - pytest
# - npm test
# - mvn test
```

**Expected: RED (test fails) ✓**

### Phase 2: Implementation (Green Phase)

Write minimal code to make tests pass while enforcing constraints.

#### 2.1 Determine Implementation File Location

**Consult source-tree.md:**

```
Read(file_path="devforgeai/specs/context/source-tree.md")
# Find placement rules for new classes/modules
```

**If location is ambiguous → Use AskUserQuestion:**

```
Question: "Where should the new [ComponentName] class be placed?"
Header: "File location"
Options:
  - "src/Application/Services/[ComponentName].cs (Application layer)"
  - "src/Domain/Services/[ComponentName].cs (Domain layer)"
  - "src/Infrastructure/Services/[ComponentName].cs (Infrastructure layer)"
  - "src/[FeatureName]/[ComponentName].cs (Vertical slice)"
multiSelect: false
```

#### 2.2 Check for Required Dependencies

**Validate against dependencies.md:**

```
Read(file_path="devforgeai/specs/context/dependencies.md")
```

**If implementation needs a package NOT in dependencies.md:**

**STOP and use AskUserQuestion:**

```
Question: "Implementation requires package '[PackageName]' for [functionality], but it's not in dependencies.md. Should I add it?"
Header: "New dependency"
Description: "This will update dependencies.md and require ADR documentation"
Options:
  - "Yes, add [PackageName] version [X.Y.Z]"
  - "No, use existing dependency [AlternativeName] from dependencies.md"
  - "No, implement manually without external dependency"
multiSelect: false
```

**After approval:**
1. Update dependencies.md
2. Create ADR documenting decision
3. Install package
4. Proceed with implementation

#### 2.3 Implement Following Coding Standards

**Load coding patterns:**

```
Read(file_path="devforgeai/specs/context/coding-standards.md")
```

**Enforce patterns during implementation:**

**Example: Async/Await (C#)**
```csharp
// ✅ CORRECT (if coding-standards.md specifies ConfigureAwait(false))
public async Task<Order> GetOrderAsync(int id)
{
    var order = await _repository.GetByIdAsync(id).ConfigureAwait(false);
    return order;
}

// ❌ FORBIDDEN (violates coding-standards.md)
public async Task<Order> GetOrderAsync(int id)
{
    var order = await _repository.GetByIdAsync(id); // Missing ConfigureAwait
    return order;
}
```

**Example: Dependency Injection (C#)**
```csharp
// ✅ CORRECT (if coding-standards.md specifies constructor injection)
public class OrderService
{
    private readonly IOrderRepository _repository;

    public OrderService(IOrderRepository repository)
    {
        _repository = repository;
    }
}

// ❌ FORBIDDEN (violates coding-standards.md - tight coupling)
public class OrderService
{
    private readonly IOrderRepository _repository = new OrderRepository();
}
```

**Example: Error Handling (C#)**
```csharp
// ✅ CORRECT (if coding-standards.md specifies Result Pattern)
public Result<Order> ProcessOrder(OrderRequest request)
{
    if (!request.IsValid())
        return Result.Failure<Order>("Invalid order request");

    var order = _mapper.Map(request);
    return Result.Success(order);
}

// ❌ FORBIDDEN (if Result Pattern is specified - uses exceptions for business logic)
public Order ProcessOrder(OrderRequest request)
{
    if (!request.IsValid())
        throw new InvalidOrderException("Invalid order request");

    return _mapper.Map(request);
}
```

#### 2.4 Validate Architecture Constraints

**Load architecture rules:**

```
Read(file_path="devforgeai/specs/context/architecture-constraints.md")
```

**Enforce layer boundaries:**

**Example: Clean Architecture**
```
Layer Dependency Matrix (from architecture-constraints.md):

| From ↓ To → | API | Application | Domain | Infrastructure |
|-------------|-----|-------------|--------|----------------|
| API         | ✓   | ✓           | ❌     | ❌             |
| Application | ❌  | ✓           | ✓      | ❌             |
| Domain      | ❌  | ❌          | ✓      | ❌             |
| Infrastructure | ❌ | ❌        | ✓      | ✓              |

RULE: Domain NEVER references Infrastructure
```

**Validation during implementation:**

```csharp
// ❌ FORBIDDEN (Domain → Infrastructure violation)
namespace MyApp.Domain.Entities
{
    using MyApp.Infrastructure.Repositories; // VIOLATION!

    public class Order
    {
        private readonly OrderRepository _repo; // NO!
    }
}

// ✅ CORRECT (Domain uses abstractions)
namespace MyApp.Domain.Entities
{
    public class Order
    {
        // Domain stays pure, no infrastructure references
    }
}

namespace MyApp.Domain.Abstractions
{
    public interface IOrderRepository // Domain defines interface
    {
        Task<Order> GetByIdAsync(int id);
    }
}

namespace MyApp.Infrastructure.Repositories
{
    using MyApp.Domain.Abstractions;

    public class OrderRepository : IOrderRepository // Infrastructure implements
    {
        // Implementation with Dapper/EF/etc.
    }
}
```

**If architecture decision is ambiguous → Use AskUserQuestion:**

```
Question: "Should [Component] use Repository Pattern or direct database access for this feature?"
Header: "Data access"
Description: "architecture-constraints.md doesn't specify pattern for this scenario"
Options:
  - "Repository Pattern (all data access through repositories)"
  - "Direct access (services can query database directly for this feature)"
  - "Hybrid (repositories for writes, direct queries for reads)"
multiSelect: false
```

#### 2.5 Use Native Tools for File Operations

**CRITICAL: Use native tools for 40-73% token savings (per your efficiency analysis)**

**Reading files:**
```
✅ CORRECT: Read(file_path="src/Services/OrderService.cs")
❌ FORBIDDEN: Bash(command="cat src/Services/OrderService.cs")
```

**Editing files:**
```
✅ CORRECT: Edit(file_path="src/Services/OrderService.cs",
                 old_string="old implementation",
                 new_string="new implementation")
❌ FORBIDDEN: Bash(command="sed -i 's/old/new/' src/Services/OrderService.cs")
```

**Finding files:**
```
✅ CORRECT: Glob(pattern="src/**/*.cs")
❌ FORBIDDEN: Bash(command="find src/ -name '*.cs'")
```

**Searching code:**
```
✅ CORRECT: Grep(pattern="class OrderService", type="cs")
❌ FORBIDDEN: Bash(command="grep -r 'class OrderService' --include='*.cs'")
```

**Creating files:**
```
✅ CORRECT: Write(file_path="src/Services/NewService.cs", content="...")
❌ FORBIDDEN: Bash(command="cat > src/Services/NewService.cs <<'EOF'...EOF")
```

**Reserve Bash ONLY for:**
- Running tests: `Bash(command="pytest")`
- Building: `Bash(command="dotnet build")`
- Git operations: `Bash(command="git status")`
- Package installation: `Bash(command="npm install")`

#### 2.6 Implement with Minimal Code

**Write only enough code to pass the test:**

```
# Use Edit tool to add implementation
Edit(file_path="src/Services/OrderService.cs",
     old_string="[location marker]",
     new_string="[minimal implementation to pass test]")
```

**Run tests:**

```
Bash(command="[test command]")
```

**Expected: GREEN (test passes) ✓**

### Phase 3: Refactor (Refactor Phase)

Improve code quality while keeping tests green.

#### 3.1 Check Anti-Patterns

**Load forbidden patterns:**

```
Read(file_path="devforgeai/specs/context/anti-patterns.md")
```

**Validate implementation doesn't violate:**

**Category 1: Library Substitution**
```
❌ FORBIDDEN: Switching from Dapper to Entity Framework
✅ DETECTION: Check tech-stack.md BEFORE suggesting alternatives
```

**Category 2: Structure Violation**
```
❌ FORBIDDEN: Creating files in wrong directories
✅ DETECTION: Validate file paths against source-tree.md
```

**Category 3: Cross-Layer Dependencies**
```
❌ FORBIDDEN: Domain referencing Infrastructure
✅ DETECTION: Check imports/using statements against architecture-constraints.md
```

**Category 4: Framework Mixing**
```
❌ FORBIDDEN: Introducing Redux patterns in Zustand project
✅ DETECTION: Check dependencies.md for locked state management
```

**Category 5: Magic Numbers/Strings**
```
❌ FORBIDDEN: Hard-coded values
✅ CORRECT: Constants or configuration

// ❌ FORBIDDEN
if (order.Total > 1000) { ... }

// ✅ CORRECT
private const decimal FREE_SHIPPING_THRESHOLD = 1000m;
if (order.Total > FREE_SHIPPING_THRESHOLD) { ... }
```

**Category 6: God Objects**
```
❌ FORBIDDEN: Classes with >500 lines or >10 responsibilities
✅ CORRECT: Single Responsibility Principle
```

**Category 7: Tight Coupling**
```
❌ FORBIDDEN: Direct instantiation of dependencies
✅ CORRECT: Constructor injection (from coding-standards.md)
```

**Category 8: Security Anti-Patterns**
```
❌ FORBIDDEN: SQL injection, XSS vulnerabilities, secrets in code
✅ CORRECT: Parameterized queries, input validation, configuration-based secrets

// ❌ FORBIDDEN (SQL injection)
var sql = $"SELECT * FROM Orders WHERE Id = {orderId}";

// ✅ CORRECT (Dapper parameterized - from coding-standards.md)
var sql = "SELECT * FROM Orders WHERE Id = @Id";
var order = await connection.QuerySingleAsync<Order>(sql, new { Id = orderId });
```

**If anti-pattern detected → Refactor immediately:**

```
Edit(file_path="[file_with_antipattern]",
     old_string="[anti-pattern code]",
     new_string="[corrected code following standards]")
```

#### 3.2 Apply SOLID Principles

**Single Responsibility:**
- Each class has one reason to change
- Extract responsibilities if class is doing too much

**Open/Closed:**
- Open for extension, closed for modification
- Use interfaces and abstractions

**Liskov Substitution:**
- Derived classes must be substitutable for base classes

**Interface Segregation:**
- Many specific interfaces > one general interface

**Dependency Inversion:**
- Depend on abstractions, not concretions
- Follow architecture-constraints.md layer rules

#### 3.3 Improve Code Quality

**While keeping tests GREEN:**

```
# Refactor implementation
Edit(file_path="src/Services/OrderService.cs",
     old_string="[working but messy code]",
     new_string="[clean, refactored code]")

# Verify tests still pass
Bash(command="[test command]")
```

**Refactoring checklist:**
- [ ] Meaningful variable/method names
- [ ] Extracted helper methods for complex logic
- [ ] Removed duplication (DRY principle)
- [ ] Added comments for non-obvious logic
- [ ] Followed naming conventions from coding-standards.md
- [ ] Proper error handling per coding-standards.md
- [ ] Logging added per coding-standards.md patterns

**Expected: Tests remain GREEN ✓**

### Phase 4: Integration & Validation

Ensure implementation integrates correctly with existing codebase.

#### 4.1 Run Full Test Suite

```
Bash(command="[full test command with coverage]")
# Examples:
# - dotnet test --collect:"XPlat Code Coverage"
# - pytest --cov=src --cov-report=term
# - npm test -- --coverage
```

**Validate:**
- [ ] All existing tests still pass (no regressions)
- [ ] New tests pass
- [ ] Code coverage meets requirements (typically 80%+)

#### 4.2 Static Analysis

**If configured in project:**

```
Bash(command="[linter command]")
# Examples:
# - dotnet format --verify-no-changes
# - pylint src/
# - npm run lint
```

**Fix any violations:**

```
Edit(file_path="[file_with_violations]",
     old_string="[violating code]",
     new_string="[compliant code]")
```

#### 4.3 Build Validation

```
Bash(command="[build command]")
# Examples:
# - dotnet build
# - npm run build
# - mvn package
```

**Expected: Build succeeds ✓**

#### 4.4 Update Documentation

**If implementation affects:**

**API contracts:**
```
Edit(file_path="docs/api/[endpoint].md",
     old_string="[old documentation]",
     new_string="[updated documentation]")
```

**Database schema:**
```
Edit(file_path="devforgeai/specs/context/source-tree.md",
     old_string="[old structure]",
     new_string="[updated structure with new tables]")
```

**New dependencies:**
```
Edit(file_path="devforgeai/specs/context/dependencies.md",
     old_string="[existing dependencies]",
     new_string="[existing + new approved dependencies]")
```

### Phase 5: Git Workflow (Integration Ready)

Prepare implementation for review and merge.

#### 5.1 Review Changes

```
Bash(command="git status")
Bash(command="git diff")
```

**Validate:**
- [ ] Only relevant files modified
- [ ] No debug code or commented-out code
- [ ] No secrets or credentials in code
- [ ] All new files in correct locations (per source-tree.md)

#### 5.2 Stage and Commit

**Following git best practices:**

```
Bash(command="git add [relevant_files]")
Bash(command="git status")  # Verify staged files
```

**Commit message format (from project conventions):**

```
Bash(command='git commit -m "$(cat <<'\''EOF'\''
[type]: [brief description]

- Implemented [feature] following TDD
- Tests: [test description]
- Compliance: tech-stack.md, coding-standards.md
- Coverage: [percentage]

Closes #[issue-number]
EOF
)"')
```

**Example commit message:**

```
feat: Implement order discount calculation

- Implemented CalculateDiscount method following TDD
- Tests: Unit tests for valid coupon, expired coupon, invalid code
- Compliance: tech-stack.md (Dapper), coding-standards.md (Result Pattern)
- Coverage: 95% for OrderService

Closes #123
```

#### 5.3 Push to Remote

```
Bash(command="git push origin [branch-name]")
```

---

## Ambiguity Resolution Protocol

**CRITICAL: Use AskUserQuestion for ALL ambiguities**

### Common Ambiguity Scenarios

#### Scenario 1: Technology Choice Ambiguous

**Trigger:** Implementation needs functionality not explicitly covered in tech-stack.md

**Example:** Spec says "implement caching" but tech-stack.md doesn't specify caching technology

**Response:**

```
Question: "Spec requires caching, but tech-stack.md doesn't specify a caching technology. Which should be used?"
Header: "Caching tech"
Description: "This will be added to tech-stack.md as a LOCKED choice"
Options:
  - "Redis (distributed cache, scalable)"
  - "Memcached (simple, fast in-memory cache)"
  - "IMemoryCache (built-in .NET, simple but not distributed)"
  - "SQL Server memory-optimized tables (database-level caching)"
multiSelect: false
```

**After answer:**
1. Update tech-stack.md
2. Create ADR documenting decision
3. Update dependencies.md with package version
4. Proceed with implementation

#### Scenario 2: Pattern Not Specified

**Trigger:** Implementation needs pattern not in coding-standards.md or architecture-constraints.md

**Example:** Async event handling pattern not documented

**Response:**

```
Question: "Implementation needs async event handling, but coding-standards.md doesn't specify the pattern. Which approach should be used?"
Header: "Event pattern"
Options:
  - "MediatR with INotification (CQRS-style events)"
  - "Custom event dispatcher with observer pattern"
  - "Built-in C# events with async handlers"
  - "Message queue (RabbitMQ/Azure Service Bus)"
multiSelect: false
```

#### Scenario 3: File Location Unclear

**Trigger:** New file type not covered in source-tree.md

**Example:** Where to place background job classes

**Response:**

```
Question: "Where should background job classes be placed? source-tree.md doesn't specify this pattern."
Header: "Jobs location"
Options:
  - "src/Application/Jobs/ (Application layer - jobs orchestrate use cases)"
  - "src/Infrastructure/Jobs/ (Infrastructure layer - jobs are infrastructure concern)"
  - "src/BackgroundJobs/ (Separate layer for job logic)"
multiSelect: false
```

#### Scenario 4: Conflicting Requirements

**Trigger:** Spec requirement conflicts with existing context files

**Example:** Spec wants MongoDB but tech-stack.md specifies SQL Server

**Response:**

```
Question: "Spec requires MongoDB for [feature], but tech-stack.md specifies SQL Server as the LOCKED database. How should this conflict be resolved?"
Header: "DB conflict"
Options:
  - "Use SQL Server (follow tech-stack.md, update spec)"
  - "Use MongoDB (update tech-stack.md, create ADR for exception)"
  - "Use both (polyglot persistence - add MongoDB to tech-stack.md)"
  - "Re-design feature to work with SQL Server"
multiSelect: false
```

#### Scenario 5: Version Ambiguity

**Trigger:** Package version not specified in dependencies.md

**Example:** Package listed but no version constraint

**Response:**

```
Question: "dependencies.md lists 'Dapper' but doesn't specify version. Which version should be used?"
Header: "Package version"
Options:
  - "Latest stable (currently 2.1.28)"
  - "Specific version 2.1.x (allow patch updates)"
  - "Specific version 2.1.28 (locked)"
multiSelect: false
```

---

## Tool Usage Protocol

**MANDATORY: Use native tools for file operations**

### File Operations (ALWAYS use native tools):
- **Reading**: Read tool, NOT `cat`, `head`, `tail`
- **Searching**: Grep tool, NOT `grep`, `rg`, `ag`
- **Finding**: Glob tool, NOT `find`, `ls -R`
- **Editing**: Edit tool, NOT `sed`, `awk`, `perl`
- **Creating**: Write tool, NOT `echo >`, `cat > <<EOF`

### Terminal Operations (Use Bash):
- **Version control**: git commands
- **Package management**: npm, pip, cargo, dotnet, maven, gradle
- **Test execution**: pytest, npm test, dotnet test, cargo test, mvn test
- **Build processes**: make, cmake, gradle, dotnet build, npm run build
- **Containers**: docker, kubectl, podman

### Communication (Use text output):
- Explain steps to user
- Provide analysis results
- Ask clarifying questions
- NOT `echo` or `printf` for communication

### Efficiency Target

**Token budget per feature implementation: <80,000 tokens**

**Breakdown:**
- Context validation: ~5,000 tokens
- Test design & writing: ~15,000 tokens
- Implementation: ~30,000 tokens
- Refactoring & validation: ~20,000 tokens
- Documentation updates: ~10,000 tokens

**Using native tools saves 40-73% vs Bash commands**

---

## Integration with Other Skills

### devforgeai-architecture

**When invoked:** Context files missing or need updates

**Interaction:**
```
# Auto-invoke if context missing
if context_files_missing:
    Skill(command="devforgeai-architecture")
    # Wait for architecture skill to complete
    # Then reload context files and continue
```

### devforgeai-qa (Future)

**When invoked:** Implementation complete, ready for QA review

**Handoff:**
- All tests passing
- Code coverage meets requirements
- No anti-patterns detected
- Build succeeds

### devforgeai-release (Future)

**When invoked:** QA approved, ready for release

**Handoff:**
- Tests passing
- Documentation updated
- Git commits created
- Ready for PR creation

---

## Success Criteria

This skill succeeds when:

- [ ] Context files validated before development starts
- [ ] All ambiguities resolved via AskUserQuestion (no assumptions)
- [ ] Tests written BEFORE implementation (TDD followed)
- [ ] Implementation follows ALL context file constraints
- [ ] No anti-patterns introduced
- [ ] All tests pass (including new and existing)
- [ ] Code coverage meets requirements
- [ ] Build succeeds
- [ ] Native tools used for file operations (achieving token efficiency)
- [ ] Documentation updated for any API/schema changes
- [ ] Git commits created with proper messages

**The goal: Zero technical debt from wrong assumptions, fully tested features that comply with architectural decisions.**
