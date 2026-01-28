# ADR-EXAMPLE-004: Clean Architecture for Task Management Platform

**Date**: 2025-10-28
**Status**: Accepted
**Deciders**: Technical Architect, Senior Backend Developers (2), Tech Lead
**Project**: Enterprise Task Management Platform (SaaS)

---

## Context

### Project Background
Building an enterprise task management platform for teams:
- **User Base**: 10,000+ organizations, 500K+ end users
- **Complexity**: Medium-high (projects, tasks, subtasks, dependencies, permissions, notifications, integrations)
- **Lifespan**: 5+ years (long-term maintenance)
- **Team Size**: 8 developers (4 backend, 3 frontend, 1 DevOps)
- **Technology Stack**: .NET 8, PostgreSQL, React, Docker, Kubernetes
- **Domain Logic**: Rich business rules (task dependencies, workflow automation, RBAC)

### Problem Statement
We need an architectural pattern that:
1. **Scales with complexity**: Handles growing business logic without spaghetti code
2. **Enables testing**: High test coverage (target: 90%+) for business rules
3. **Supports long-term maintenance**: Easy to understand and modify after 2-3 years
4. **Isolates dependencies**: Database, UI, external APIs can change independently
5. **Enforces boundaries**: Prevents domain logic from leaking into infrastructure

### Current Challenges
Previous projects used **N-Tier Architecture** (3-layer: UI → Business Logic → Data Access):
- ❌ **Tight coupling**: Business logic depends on Entity Framework entities (database schema)
- ❌ **Testability issues**: Testing business logic requires database mocks
- ❌ **Rigid structure**: Changing database schema breaks business logic
- ❌ **Domain pollution**: HTTP, logging, caching concerns mixed with business rules
- ❌ **Maintenance burden**: Developers fear changing code (ripple effects)

### Requirements

**Functional Requirements**:
- **Core Domain**: Tasks, Projects, Users, Teams, Permissions, Notifications
- **Business Rules**: Task dependencies (can't complete if dependencies incomplete), automated status transitions, permission checks
- **Integrations**: Slack, Microsoft Teams, Google Calendar, Jira, GitHub
- **Multi-tenancy**: Isolated data per organization

**Non-Functional Requirements**:
- **Testability**: 90%+ test coverage for business logic
- **Maintainability**: New developers productive in <2 weeks
- **Flexibility**: Swap database (PostgreSQL → MongoDB) without rewriting business logic
- **Performance**: <200ms API response time (P95)
- **Extensibility**: Add new integrations without core changes

### Constraints
- Must support .NET 8 (C# 12)
- Must integrate with existing PostgreSQL database
- Must work with existing React frontend (no major rewrites)
- Must deploy on Kubernetes (containerized)
- No breaking changes to current API contracts

---

## Decision

**We will adopt Clean Architecture as the foundational architectural pattern.**

Specifically:
- **4-Layer Structure**: Domain → Application → Infrastructure → Presentation
- **Dependency Rule**: Outer layers depend on inner layers (never reverse)
- **Domain-Driven Design (DDD)**: Entities, Value Objects, Aggregates, Domain Services
- **CQRS Lite**: Separate read/write models (not full Event Sourcing)
- **Repository Pattern**: Abstract data access behind interfaces

---

## Rationale

### Architectural Overview

Clean Architecture organizes code into **concentric layers** with strict dependency rules:

```
┌─────────────────────────────────────────────────┐
│          Presentation Layer (API)               │
│  Controllers, ViewModels, DTOs, Validators      │
│                                                 │
│  ┌───────────────────────────────────────────┐ │
│  │    Infrastructure Layer                    │ │
│  │  Repositories, EF, External APIs, Email    │ │
│  │                                            │ │
│  │  ┌─────────────────────────────────────┐  │ │
│  │  │     Application Layer                │  │ │
│  │  │  Use Cases, Commands, Queries        │  │ │
│  │  │  Interfaces, DTOs, Validators        │  │ │
│  │  │                                      │  │ │
│  │  │  ┌───────────────────────────────┐  │  │ │
│  │  │  │    Domain Layer (Core)         │  │  │ │
│  │  │  │  Entities, Value Objects       │  │  │ │
│  │  │  │  Business Rules, Domain Events │  │  │ │
│  │  │  │  NO external dependencies      │  │  │ │
│  │  │  └───────────────────────────────┘  │  │ │
│  │  └─────────────────────────────────────┘  │ │
│  └───────────────────────────────────────────┘ │
└─────────────────────────────────────────────────┘

DEPENDENCY RULE:
→ Presentation depends on Application
→ Infrastructure depends on Application
→ Application depends on Domain
→ Domain depends on NOTHING
```

### Technical Rationale

#### 1. Testability: Domain Layer is Framework-Free

**Problem with N-Tier**:
```csharp
// ❌ N-Tier: Business logic coupled to Entity Framework
public class Task
{
    public int Id { get; set; } // EF requirement
    public string Title { get; set; }
    public TaskStatus Status { get; set; }

    // Navigation properties (EF specific)
    public virtual ICollection<Task> Dependencies { get; set; }

    // Business logic mixed with data access
    public bool CanComplete()
    {
        // Requires EF to load Dependencies (N+1 query risk)
        return Dependencies.All(d => d.Status == TaskStatus.Completed);
    }
}

// Testing requires EF in-memory provider
[Fact]
public void CanComplete_ReturnsFalse_WhenDependenciesIncomplete()
{
    var options = new DbContextOptionsBuilder<AppDbContext>()
        .UseInMemoryDatabase("TestDb")
        .Options;

    using var context = new AppDbContext(options);
    var task = new Task { /* setup EF entities */ };
    context.Tasks.Add(task);
    context.SaveChanges();

    // Test requires full EF setup
    Assert.False(task.CanComplete());
}
```

**Clean Architecture Solution**:
```csharp
// ✅ Clean Architecture: Pure domain logic (no EF)
namespace TaskManagement.Domain.Entities;

public class Task : Entity<TaskId> // No EF dependencies
{
    public TaskTitle Title { get; private set; }
    public TaskStatus Status { get; private set; }
    private readonly List<Task> _dependencies = new();
    public IReadOnlyCollection<Task> Dependencies => _dependencies.AsReadOnly();

    // Factory method (domain invariants enforced)
    public static Task Create(TaskTitle title, ProjectId projectId)
    {
        var task = new Task
        {
            Id = TaskId.Create(),
            Title = title,
            Status = TaskStatus.ToDo,
            ProjectId = projectId,
            CreatedAt = DateTime.UtcNow
        };

        task.RaiseDomainEvent(new TaskCreatedEvent(task.Id, task.Title));
        return task;
    }

    // Pure business logic (no database, no EF)
    public Result MarkAsComplete()
    {
        if (!CanComplete())
            return Result.Failure("Cannot complete task with incomplete dependencies");

        Status = TaskStatus.Completed;
        RaiseDomainEvent(new TaskCompletedEvent(Id));
        return Result.Success();
    }

    public bool CanComplete() =>
        _dependencies.All(d => d.Status == TaskStatus.Completed);

    public Result AddDependency(Task dependency)
    {
        if (WouldCreateCycle(dependency))
            return Result.Failure("Adding this dependency would create a cycle");

        _dependencies.Add(dependency);
        return Result.Success();
    }

    private bool WouldCreateCycle(Task dependency)
    {
        // Pure logic (no database queries)
        var visited = new HashSet<TaskId>();
        return HasCycle(dependency, visited);
    }

    private bool HasCycle(Task task, HashSet<TaskId> visited)
    {
        if (visited.Contains(task.Id)) return true;
        visited.Add(task.Id);

        return task.Dependencies.Any(dep => HasCycle(dep, visited));
    }
}

// Testing is trivial (no EF, no mocking)
[Fact]
public void MarkAsComplete_Fails_WhenDependenciesIncomplete()
{
    // Arrange: Pure domain objects (no database)
    var task1 = Task.Create(TaskTitle.From("Task 1"), ProjectId.Create());
    var task2 = Task.Create(TaskTitle.From("Task 2"), ProjectId.Create());
    task2.AddDependency(task1);

    // Act: Pure method call
    var result = task2.MarkAsComplete();

    // Assert: No database, no mocking
    Assert.True(result.IsFailure);
    Assert.Contains("incomplete dependencies", result.Error);
}
```

**Key Benefits**:
- ✅ No EF dependencies in domain (pure C# POCOs)
- ✅ Tests run in <1ms (no database setup)
- ✅ 100% test coverage achievable (pure logic, no I/O)
- ✅ Business rules are explicit and clear

#### 2. Flexibility: Swap Infrastructure Without Domain Changes

**Scenario**: Migrate from PostgreSQL (Entity Framework) to MongoDB (MongoDB Driver).

**N-Tier Architecture** (tightly coupled):
```csharp
// ❌ Domain entities are EF entities (can't easily switch to MongoDB)
public class Task
{
    public int Id { get; set; } // PostgreSQL identity column
    public virtual ICollection<Task> Dependencies { get; set; } // EF navigation property
}

// Migrating to MongoDB requires:
// 1. Rewrite Task entity (int Id → ObjectId)
// 2. Rewrite all business logic referencing Task.Id
// 3. Rewrite navigation properties (virtual ICollection → BsonDocument?)
// 4. Rewrite all queries (LINQ to Entities → MongoDB queries)
// Estimated effort: 3-4 weeks of full rewrite
```

**Clean Architecture** (decoupled):
```csharp
// ✅ Domain: Unchanged (no EF, no MongoDB)
namespace TaskManagement.Domain.Entities;

public class Task : Entity<TaskId>
{
    public TaskId Id { get; private set; } // Custom value object (not int, not ObjectId)
    public TaskTitle Title { get; private set; }
    // No navigation properties (dependencies loaded via repository)
}

// Infrastructure: PostgreSQL Implementation
namespace TaskManagement.Infrastructure.Persistence.Postgres;

public class TaskRepository : ITaskRepository
{
    private readonly AppDbContext _context; // Entity Framework

    public async Task<Task?> GetByIdAsync(TaskId id)
    {
        var entity = await _context.Tasks
            .Include(t => t.Dependencies) // EF navigation
            .FirstOrDefaultAsync(t => t.Id == id.Value);

        return entity?.ToDomainModel(); // Map EF entity → Domain entity
    }
}

// Infrastructure: MongoDB Implementation (NEW)
namespace TaskManagement.Infrastructure.Persistence.Mongo;

public class TaskRepository : ITaskRepository
{
    private readonly IMongoCollection<TaskDocument> _collection; // MongoDB driver

    public async Task<Task?> GetByIdAsync(TaskId id)
    {
        var document = await _collection
            .Find(doc => doc.Id == id.Value.ToString())
            .FirstOrDefaultAsync();

        return document?.ToDomainModel(); // Map MongoDB document → Domain entity
    }
}

// Switching from Postgres to MongoDB:
// 1. Create TaskRepository (MongoDB implementation)
// 2. Update DI registration (one line in Startup.cs)
// 3. Domain and Application layers: ZERO CHANGES
// Estimated effort: 2-3 days (vs 3-4 weeks in N-Tier)
```

**Key Benefits**:
- ✅ Domain logic unchanged (database agnostic)
- ✅ Swap infrastructure with minimal risk
- ✅ Test domain logic without database (pure unit tests)
- ✅ Future-proof (new storage backend = new repository impl)

#### 3. Maintainability: Clear Boundaries and Separation of Concerns

**N-Tier Confusion** (where does logic belong?):
```csharp
// ❌ Business logic scattered across layers
public class TaskService // Business Logic Layer
{
    private readonly ITaskRepository _repo;
    private readonly IEmailService _email;
    private readonly ILogger _logger;

    public async Task<bool> CompleteTaskAsync(int taskId)
    {
        _logger.LogInformation("Completing task {TaskId}", taskId);

        var task = await _repo.GetByIdAsync(taskId);
        if (task == null)
        {
            _logger.LogWarning("Task not found");
            return false;
        }

        // Business rule mixed with data access
        var deps = await _repo.GetDependenciesAsync(taskId);
        if (deps.Any(d => d.Status != TaskStatus.Completed))
        {
            _logger.LogWarning("Dependencies incomplete");
            return false;
        }

        task.Status = TaskStatus.Completed; // Domain mutation
        await _repo.UpdateAsync(task); // Infrastructure call

        // Side effect (email) mixed with business logic
        await _email.SendTaskCompletedEmail(task.AssignedUserId);

        _logger.LogInformation("Task completed successfully");
        return true;
    }
}
```

**Clean Architecture Clarity**:
```csharp
// ✅ Domain: Pure business rules
namespace TaskManagement.Domain.Entities;

public class Task : Entity<TaskId>
{
    public Result MarkAsComplete()
    {
        if (!CanComplete())
            return Result.Failure("Cannot complete task with incomplete dependencies");

        Status = TaskStatus.Completed;
        RaiseDomainEvent(new TaskCompletedEvent(Id)); // Domain event (not email yet)
        return Result.Success();
    }

    private bool CanComplete() =>
        _dependencies.All(d => d.Status == TaskStatus.Completed);
}

// ✅ Application: Orchestrates use case
namespace TaskManagement.Application.Tasks.Commands;

public class CompleteTaskCommandHandler : IRequestHandler<CompleteTaskCommand, Result>
{
    private readonly ITaskRepository _repo;
    private readonly IUnitOfWork _uow;

    public async Task<Result> Handle(CompleteTaskCommand request, CancellationToken ct)
    {
        var task = await _repo.GetByIdWithDependenciesAsync(request.TaskId);
        if (task == null)
            return Result.Failure("Task not found");

        var result = task.MarkAsComplete(); // Domain method (pure logic)
        if (result.IsFailure)
            return result;

        await _repo.UpdateAsync(task);
        await _uow.CommitAsync(ct); // Persist + publish domain events

        return Result.Success();
    }
}

// ✅ Infrastructure: Side effects (email, notifications)
namespace TaskManagement.Infrastructure.EventHandlers;

public class TaskCompletedEventHandler : INotificationHandler<TaskCompletedEvent>
{
    private readonly IEmailService _email;
    private readonly ILogger<TaskCompletedEventHandler> _logger;

    public async Task Handle(TaskCompletedEvent notification, CancellationToken ct)
    {
        _logger.LogInformation("Task {TaskId} completed", notification.TaskId);

        var user = await _userRepo.GetByTaskIdAsync(notification.TaskId);
        await _email.SendTaskCompletedEmail(user.Email);
    }
}
```

**Key Benefits**:
- ✅ Domain = Pure business rules (no logging, no email, no database)
- ✅ Application = Orchestration (use case workflow)
- ✅ Infrastructure = Side effects (email, logging, database)
- ✅ New developers understand architecture in <1 week (clear layers)

#### 4. Enforces Dependency Inversion Principle (DIP)

**N-Tier Violation** (high-level depends on low-level):
```
Business Logic Layer → Data Access Layer (EF Core)
↓ depends on ↓
DbContext, IQueryable, Include(), etc.
```

**Clean Architecture Compliance** (both depend on abstractions):
```
Domain Layer ← Application Layer → Infrastructure Layer
      ↓                                    ↓
ITaskRepository (interface)     TaskRepository (implementation)
```

**Example**:
```csharp
// Domain defines contract (no implementation)
namespace TaskManagement.Domain.Repositories;

public interface ITaskRepository
{
    Task<Task?> GetByIdAsync(TaskId id);
    Task<IEnumerable<Task>> GetByProjectIdAsync(ProjectId projectId);
    Task AddAsync(Task task);
    Task UpdateAsync(Task task);
}

// Infrastructure implements contract (EF Core, Dapper, MongoDB, etc.)
namespace TaskManagement.Infrastructure.Persistence;

public class TaskRepository : ITaskRepository
{
    private readonly AppDbContext _context;

    public async Task<Task?> GetByIdAsync(TaskId id) =>
        await _context.Tasks.FindAsync(id.Value);
}

// Application uses abstraction (doesn't know about EF)
public class CompleteTaskCommandHandler
{
    private readonly ITaskRepository _repo; // Interface, not concrete class

    public CompleteTaskCommandHandler(ITaskRepository repo)
    {
        _repo = repo; // DI injects implementation
    }
}
```

**Key Benefits**:
- ✅ Domain and Application layers are testable (mock ITaskRepository)
- ✅ Infrastructure can be swapped (EF → Dapper → MongoDB)
- ✅ No tight coupling to frameworks

#### 5. Scalability: CQRS Lite for Read/Write Separation

**Challenge**: Complex read queries (reports, dashboards) don't fit into entity graphs.

**Solution**: CQRS Lite (Command Query Responsibility Segregation) without Event Sourcing.

```csharp
// Commands: Write operations (use domain entities)
public class CreateTaskCommand : IRequest<Result<TaskId>>
{
    public string Title { get; set; }
    public int ProjectId { get; set; }
}

public class CreateTaskCommandHandler : IRequestHandler<CreateTaskCommand, Result<TaskId>>
{
    private readonly ITaskRepository _repo;

    public async Task<Result<TaskId>> Handle(CreateTaskCommand request, CancellationToken ct)
    {
        var task = Task.Create(
            TaskTitle.From(request.Title),
            ProjectId.From(request.ProjectId)
        );

        await _repo.AddAsync(task);
        await _uow.CommitAsync(ct);

        return Result.Success(task.Id);
    }
}

// Queries: Read operations (bypass domain, use DTOs)
public class GetTasksByProjectQuery : IRequest<IEnumerable<TaskDto>>
{
    public int ProjectId { get; set; }
}

public class GetTasksByProjectQueryHandler : IRequestHandler<GetTasksByProjectQuery, IEnumerable<TaskDto>>
{
    private readonly IDbConnection _db; // Dapper for fast reads

    public async Task<IEnumerable<TaskDto>> Handle(GetTasksByProjectQuery request, CancellationToken ct)
    {
        const string sql = @"
            SELECT t.id, t.title, t.status, u.name AS assignee
            FROM tasks t
            LEFT JOIN users u ON t.assigned_user_id = u.id
            WHERE t.project_id = @ProjectId
            ORDER BY t.created_at DESC";

        return await _db.QueryAsync<TaskDto>(sql, new { request.ProjectId });
    }
}
```

**Key Benefits**:
- ✅ Write operations use rich domain models (business rules enforced)
- ✅ Read operations use optimized queries (DTOs, no entity overhead)
- ✅ Performance: Reads don't load unnecessary entity graphs
- ✅ Flexibility: Queries can use stored procedures, views, or raw SQL

---

## Consequences

### Positive Consequences

#### 1. High Testability (90%+ Coverage Achieved)
**Metrics** (from similar Clean Architecture projects):
- Domain Layer: 95-100% test coverage (pure logic, no mocks)
- Application Layer: 85-95% test coverage (mock repositories)
- Infrastructure Layer: 60-80% test coverage (integration tests)

**Example Test Suite** (Task entity):
```csharp
[Fact]
public void Create_CreatesTask_WithValidData()
{
    var task = Task.Create(TaskTitle.From("Test"), ProjectId.Create());
    Assert.NotNull(task);
    Assert.Equal("Test", task.Title.Value);
}

[Fact]
public void MarkAsComplete_Fails_WhenDependenciesIncomplete()
{
    var task1 = Task.Create(TaskTitle.From("Task 1"), ProjectId.Create());
    var task2 = Task.Create(TaskTitle.From("Task 2"), ProjectId.Create());
    task2.AddDependency(task1);

    var result = task2.MarkAsComplete();

    Assert.True(result.IsFailure);
}

[Fact]
public void AddDependency_PreventsCircularDependencies()
{
    var task1 = Task.Create(TaskTitle.From("Task 1"), ProjectId.Create());
    var task2 = Task.Create(TaskTitle.From("Task 2"), ProjectId.Create());
    task1.AddDependency(task2);

    var result = task2.AddDependency(task1); // Would create cycle

    Assert.True(result.IsFailure);
    Assert.Contains("cycle", result.Error);
}
```

#### 2. Long-Term Maintainability
- **Onboarding**: New developers productive in <2 weeks (clear layer structure)
- **Debugging**: Business logic isolated in domain layer (easy to find and fix)
- **Refactoring**: Changes to domain don't ripple to infrastructure (and vice versa)
- **Documentation**: Architecture is self-documenting (folder structure reflects design)

#### 3. Flexibility for Future Changes
**Scenario Examples**:
- **Database Migration**: PostgreSQL → MongoDB = 2-3 days (only infrastructure layer changes)
- **API Framework Change**: ASP.NET Core → Minimal APIs = 1-2 days (only presentation layer changes)
- **New Integration**: Add Slack notifications = 1 day (add event handler in infrastructure)

#### 4. Performance Optimization Opportunities
- **CQRS**: Optimize read queries independently (materialized views, caching)
- **Domain Events**: Async processing (send emails after transaction commits)
- **Repository Caching**: Cache frequently accessed entities (task details)

### Negative Consequences

#### 1. Increased Complexity (More Files, More Abstraction)
**Impact**: Medium

**Example**: Creating a new feature requires touching multiple layers.

**Simple CRUD Task** (N-Tier: 3 files):
```
1. TaskController.cs (API endpoint)
2. TaskService.cs (business logic)
3. Task.cs (entity + EF mapping)
```

**Clean Architecture** (6-8 files):
```
1. Task.cs (domain entity)
2. TaskId.cs (value object)
3. ITaskRepository.cs (domain interface)
4. CreateTaskCommand.cs (application command)
5. CreateTaskCommandHandler.cs (application handler)
6. TaskRepository.cs (infrastructure repository)
7. TaskController.cs (presentation API)
8. TaskDto.cs (presentation DTO)
```

**Mitigation**:
- Use **code generators** or templates (dotnet CLI templates, IDE snippets)
- Create **scaffolding scripts** (generate entity, command, handler, repository from template)
- Accept complexity as **necessary for long-term maintainability**

**Reality Check**: While more files initially, changes are **isolated** and **safer** (modify one layer without breaking others).

#### 2. Learning Curve for Team (2-3 Weeks Initial Training)
**Impact**: Medium

**Challenges**:
- Understanding layer dependencies (why can't domain depend on infrastructure?)
- Grasping DDD concepts (entities vs value objects, aggregates)
- Learning CQRS pattern (commands vs queries)

**Mitigation**:
- **Week 1**: Training workshop (Clean Architecture principles, layer walkthrough)
- **Week 2**: Pair programming (senior + junior implement first feature together)
- **Week 3**: Code reviews (senior developers review architecture compliance)
- **Documentation**: Architecture Decision Record (this ADR), coding guidelines wiki

**Reality Check**: 2-3 weeks is acceptable for a 5+ year project. Team proficiency increases productivity over time.

#### 3. Mapping Overhead (Domain ↔ Infrastructure ↔ Presentation)
**Impact**: Low

**Challenge**: Converting between domain entities, EF entities, and DTOs requires mapping code.

**Example Mapping** (Task entity):
```csharp
// Domain entity
public class Task : Entity<TaskId>
{
    public TaskId Id { get; private set; }
    public TaskTitle Title { get; private set; }
    public TaskStatus Status { get; private set; }
}

// EF entity (Infrastructure)
public class TaskEntity
{
    public Guid Id { get; set; }
    public string Title { get; set; }
    public int StatusId { get; set; }
}

// DTO (Presentation)
public class TaskDto
{
    public string Id { get; set; }
    public string Title { get; set; }
    public string Status { get; set; }
}

// Mapping logic (3 mappers needed)
public static class TaskMappers
{
    // Domain → EF Entity
    public static TaskEntity ToEntity(this Task domain) =>
        new TaskEntity
        {
            Id = domain.Id.Value,
            Title = domain.Title.Value,
            StatusId = (int)domain.Status
        };

    // EF Entity → Domain
    public static Task ToDomain(this TaskEntity entity) =>
        Task.Reconstruct(
            TaskId.From(entity.Id),
            TaskTitle.From(entity.Title),
            (TaskStatus)entity.StatusId
        );

    // Domain → DTO
    public static TaskDto ToDto(this Task domain) =>
        new TaskDto
        {
            Id = domain.Id.Value.ToString(),
            Title = domain.Title.Value,
            Status = domain.Status.ToString()
        };
}
```

**Mitigation**:
- Use **AutoMapper** for repetitive mappings (reduces boilerplate)
- Create **extension methods** for common conversions (`.ToEntity()`, `.ToDomain()`)
- **Accept mapping overhead**: Necessary for layer decoupling

**Reality Check**: Mapping is ~5-10% of codebase. The benefit (decoupling) outweighs the cost.

#### 4. Potential Over-Engineering for Simple CRUD
**Impact**: Low (for this project)

**Risk**: Simple CRUD operations (User profile, Settings) may not need full Clean Architecture.

**Example**: User profile update (2 fields: name, email)
- **N-Tier**: 30 lines (controller → service → repository)
- **Clean Architecture**: 80 lines (controller → command → handler → domain → repository)

**Mitigation**:
- **Hybrid approach**: Use simplified pattern for simple CRUD (skip domain layer)
- **CQRS**: Queries bypass domain entirely (direct DTO projections)
- **Pragmatism**: Don't apply full pattern to trivial features

**Reality Check**: Our project has **medium-high complexity** (task dependencies, workflows, permissions). Clean Architecture is justified.

---

## Alternatives Considered

### Alternative 1: N-Tier Architecture (Traditional 3-Layer)

**Description**: UI → Business Logic → Data Access (EF Core entities).

**Pros**:
- **Simple**: Easy to understand (3 layers: UI, BLL, DAL)
- **Familiar**: Most developers know this pattern
- **Less boilerplate**: Fewer files per feature (3-4 vs 6-8)

**Cons**:
- **Tight coupling**: Business logic depends on EF entities (can't swap database easily)
- **Testability issues**: Testing business logic requires mocking EF DbContext
- **Rigid**: Changing database schema breaks business logic
- **Domain pollution**: Logging, caching, HTTP concerns mixed with business rules
- **Long-term maintenance**: Code becomes spaghetti after 2-3 years

**Why Rejected**:
- **5+ year project**: Long-term maintainability is critical
- **Complex domain**: Task dependencies, workflows require isolated business logic
- **Testability**: 90%+ test coverage impossible with EF-coupled entities
- **Previous pain**: Team has experienced N-Tier maintenance nightmares

**Conclusion**: N-Tier is fine for **simple CRUD apps** (1-2 years lifespan). Our project requires **long-term architectural rigor**.

---

### Alternative 2: Vertical Slice Architecture

**Description**: Organize by feature (not layer). Each feature is self-contained.

**Structure**:
```
Features/
├── CreateTask/
│   ├── CreateTaskCommand.cs
│   ├── CreateTaskHandler.cs
│   ├── CreateTaskController.cs
│   └── CreateTaskValidator.cs
├── CompleteTask/
│   ├── CompleteTaskCommand.cs
│   ├── CompleteTaskHandler.cs
│   └── CompleteTaskController.cs
└── GetTasksByProject/
    ├── GetTasksByProjectQuery.cs
    └── GetTasksByProjectHandler.cs
```

**Pros**:
- **High cohesion**: All code for a feature in one place
- **Easy navigation**: Find all files for "CreateTask" in one folder
- **Minimal coupling**: Features are independent
- **CQRS-friendly**: Commands and queries naturally separated

**Cons**:
- **No shared domain model**: Each feature defines its own Task entity (duplication)
- **Business rule duplication**: "CanComplete" logic duplicated across features
- **No domain layer**: Where do cross-cutting domain rules live?
- **Refactoring difficulty**: Changing shared logic requires updating multiple slices

**Example Problem**:
```csharp
// Features/CompleteTask/Task.cs
public class Task
{
    public bool CanComplete() =>
        Dependencies.All(d => d.Status == TaskStatus.Completed); // Rule 1
}

// Features/ReopenTask/Task.cs
public class Task
{
    public bool CanReopen() =>
        Dependencies.All(d => d.Status == TaskStatus.Completed); // Duplicate rule?
}
```

**Why Rejected**:
- **Rich domain logic**: Our app has complex business rules (task dependencies, workflows, permissions)
- **Cross-cutting concerns**: Rules span multiple features (task dependencies affect CreateTask, CompleteTask, DeleteTask)
- **Duplication risk**: Business logic would be duplicated across slices
- **Team preference**: Team prefers layered architecture for domain-heavy apps

**Conclusion**: Vertical Slice is excellent for **simple, feature-driven apps** with minimal shared logic. Our project has **complex, interconnected domain rules** that benefit from centralized domain layer.

---

### Alternative 3: Hexagonal Architecture (Ports and Adapters)

**Description**: Similar to Clean Architecture, but emphasizes "ports" (interfaces) and "adapters" (implementations).

**Structure**:
```
Core/ (Domain + Application)
Ports/ (Interfaces)
Adapters/
├── Inbound/ (API controllers)
└── Outbound/ (Repositories, external APIs)
```

**Pros**:
- **Same benefits as Clean Architecture** (testability, flexibility, maintainability)
- **Port/Adapter naming**: Clearer intent (inbound = input, outbound = output)

**Cons**:
- **Less prescriptive**: No standard for organizing domain logic (entities? services?)
- **Terminology confusion**: "Ports" and "Adapters" are abstract (team prefers "Domain", "Application", "Infrastructure")

**Why Rejected**:
- **Semantics**: Clean Architecture terminology ("Domain", "Application", "Infrastructure") is clearer for team
- **Community**: Clean Architecture has more resources, tutorials, examples (.NET ecosystem)
- **Functionally equivalent**: Both achieve same goals (decoupling, testability)

**Conclusion**: Hexagonal and Clean Architecture are **nearly identical**. We chose Clean Architecture for **clearer terminology** and **better .NET community support**.

---

### Alternative 4: Onion Architecture

**Description**: Similar to Clean Architecture, but emphasizes "onion layers" (core = domain, outer = infrastructure).

**Key Difference from Clean Architecture**:
- **Onion**: Application layer is part of core (domain + application together)
- **Clean**: Application layer is separate (domain = pure logic, application = orchestration)

**Why Rejected**:
- **Mixing concerns**: Combining domain and application layers couples business logic with use case orchestration
- **Clean Architecture is clearer**: Separating domain and application makes responsibilities more explicit

**Conclusion**: Onion Architecture is a **precursor to Clean Architecture**. Clean Architecture is a **refinement** with clearer separation.

---

## Implementation Plan

### Phase 1: Setup and Training (Week 1-2)

**1. Project Structure Setup**:
```
TaskManagement/
├── src/
│   ├── TaskManagement.Domain/ (no external dependencies)
│   ├── TaskManagement.Application/ (depends on Domain)
│   ├── TaskManagement.Infrastructure/ (depends on Application)
│   └── TaskManagement.API/ (depends on Application)
└── tests/
    ├── TaskManagement.Domain.Tests/
    ├── TaskManagement.Application.Tests/
    └── TaskManagement.Infrastructure.IntegrationTests/
```

**2. Install NuGet Packages**:
```bash
# Domain (zero dependencies except primitives)
# (No packages needed)

# Application
dotnet add package MediatR --version 12.x
dotnet add package FluentValidation --version 11.x

# Infrastructure
dotnet add package Microsoft.EntityFrameworkCore --version 8.x
dotnet add package Npgsql.EntityFrameworkCore.PostgreSQL --version 8.x
dotnet add package Dapper --version 2.x

# API
dotnet add package Microsoft.AspNetCore.OpenApi --version 8.x
```

**3. Team Training**:
- Workshop: "Clean Architecture Principles" (2 hours)
- Hands-on: Implement "Create Project" feature together (pair programming)
- Code review: Review first Clean Architecture PR together

### Phase 2: Domain Layer Implementation (Week 3-4)

**4. Define Core Entities**:
```csharp
// Domain/Entities/Task.cs
public class Task : Entity<TaskId>
{
    public TaskTitle Title { get; private set; }
    public TaskStatus Status { get; private set; }
    // ... (implementation shown earlier)
}

// Domain/ValueObjects/TaskId.cs
public sealed class TaskId : ValueObject
{
    public Guid Value { get; }
    private TaskId(Guid value) => Value = value;
    public static TaskId Create() => new(Guid.NewGuid());
    public static TaskId From(Guid value) => new(value);
}

// Domain/ValueObjects/TaskTitle.cs
public sealed class TaskTitle : ValueObject
{
    public string Value { get; }
    private TaskTitle(string value) => Value = value;

    public static TaskTitle From(string value)
    {
        if (string.IsNullOrWhiteSpace(value))
            throw new ArgumentException("Title cannot be empty");
        if (value.Length > 200)
            throw new ArgumentException("Title too long");

        return new TaskTitle(value);
    }
}
```

**5. Define Repository Interfaces** (in Domain):
```csharp
// Domain/Repositories/ITaskRepository.cs
public interface ITaskRepository
{
    Task<Task?> GetByIdAsync(TaskId id);
    Task<IEnumerable<Task>> GetByProjectIdAsync(ProjectId projectId);
    Task AddAsync(Task task);
    Task UpdateAsync(Task task);
}
```

### Phase 3: Application Layer Implementation (Week 5-6)

**6. Implement CQRS Commands/Queries**:
```csharp
// Application/Tasks/Commands/CreateTaskCommand.cs
public class CreateTaskCommand : IRequest<Result<TaskId>>
{
    public string Title { get; set; }
    public Guid ProjectId { get; set; }
}

// Application/Tasks/Commands/CreateTaskCommandHandler.cs
public class CreateTaskCommandHandler : IRequestHandler<CreateTaskCommand, Result<TaskId>>
{
    private readonly ITaskRepository _repo;
    private readonly IUnitOfWork _uow;

    public async Task<Result<TaskId>> Handle(CreateTaskCommand request, CancellationToken ct)
    {
        var task = Task.Create(
            TaskTitle.From(request.Title),
            ProjectId.From(request.ProjectId)
        );

        await _repo.AddAsync(task);
        await _uow.CommitAsync(ct);

        return Result.Success(task.Id);
    }
}

// Application/Tasks/Queries/GetTasksByProjectQuery.cs
public class GetTasksByProjectQuery : IRequest<IEnumerable<TaskDto>>
{
    public Guid ProjectId { get; set; }
}

public class GetTasksByProjectQueryHandler : IRequestHandler<GetTasksByProjectQuery, IEnumerable<TaskDto>>
{
    private readonly IDbConnection _db; // Dapper

    public async Task<IEnumerable<TaskDto>> Handle(GetTasksByProjectQuery request, CancellationToken ct)
    {
        const string sql = "SELECT id, title, status FROM tasks WHERE project_id = @ProjectId";
        return await _db.QueryAsync<TaskDto>(sql, new { request.ProjectId });
    }
}
```

### Phase 4: Infrastructure Layer Implementation (Week 7-8)

**7. Implement EF Repository**:
```csharp
// Infrastructure/Persistence/TaskRepository.cs
public class TaskRepository : ITaskRepository
{
    private readonly AppDbContext _context;

    public async Task<Task?> GetByIdAsync(TaskId id)
    {
        var entity = await _context.Tasks
            .Include(t => t.Dependencies)
            .FirstOrDefaultAsync(t => t.Id == id.Value);

        return entity?.ToDomainModel();
    }

    public async Task AddAsync(Task task)
    {
        var entity = task.ToEntity();
        await _context.Tasks.AddAsync(entity);
    }

    public async Task UpdateAsync(Task task)
    {
        var entity = task.ToEntity();
        _context.Tasks.Update(entity);
    }
}
```

**8. Configure Dependency Injection**:
```csharp
// API/Program.cs
builder.Services.AddScoped<ITaskRepository, TaskRepository>();
builder.Services.AddScoped<IUnitOfWork, UnitOfWork>();
builder.Services.AddMediatR(cfg => cfg.RegisterServicesFromAssembly(typeof(CreateTaskCommand).Assembly));
```

### Phase 5: API Layer Implementation (Week 9)

**9. Create API Controllers**:
```csharp
// API/Controllers/TasksController.cs
[ApiController]
[Route("api/tasks")]
public class TasksController : ControllerBase
{
    private readonly IMediator _mediator;

    [HttpPost]
    public async Task<ActionResult<TaskDto>> CreateTask(CreateTaskCommand command)
    {
        var result = await _mediator.Send(command);
        return result.IsSuccess
            ? CreatedAtAction(nameof(GetTask), new { id = result.Value }, result.Value)
            : BadRequest(result.Error);
    }

    [HttpGet("{id}")]
    public async Task<ActionResult<TaskDto>> GetTask(Guid id)
    {
        var query = new GetTaskByIdQuery { Id = id };
        var task = await _mediator.Send(query);
        return task == null ? NotFound() : Ok(task);
    }
}
```

### Phase 6: Testing (Week 10-11)

**10. Write Domain Tests**:
```csharp
// Tests/Domain/TaskTests.cs
public class TaskTests
{
    [Fact]
    public void MarkAsComplete_Succeeds_WhenDependenciesComplete()
    {
        var task1 = Task.Create(TaskTitle.From("Task 1"), ProjectId.Create());
        task1.MarkAsComplete();

        var task2 = Task.Create(TaskTitle.From("Task 2"), ProjectId.Create());
        task2.AddDependency(task1);

        var result = task2.MarkAsComplete();

        Assert.True(result.IsSuccess);
        Assert.Equal(TaskStatus.Completed, task2.Status);
    }
}
```

**11. Write Application Tests**:
```csharp
// Tests/Application/CreateTaskCommandHandlerTests.cs
public class CreateTaskCommandHandlerTests
{
    [Fact]
    public async Task Handle_CreatesTask_WithValidData()
    {
        var mockRepo = new Mock<ITaskRepository>();
        var mockUow = new Mock<IUnitOfWork>();
        var handler = new CreateTaskCommandHandler(mockRepo.Object, mockUow.Object);

        var command = new CreateTaskCommand { Title = "Test Task", ProjectId = Guid.NewGuid() };
        var result = await handler.Handle(command, CancellationToken.None);

        Assert.True(result.IsSuccess);
        mockRepo.Verify(r => r.AddAsync(It.IsAny<Task>()), Times.Once);
        mockUow.Verify(u => u.CommitAsync(It.IsAny<CancellationToken>()), Times.Once);
    }
}
```

---

## Monitoring and Metrics

### Key Performance Indicators (KPIs)

**Technical Metrics**:
- **Test coverage**: 90%+ (Domain: 95%+, Application: 85%+)
- **API latency**: <200ms P95
- **Build time**: <5 minutes
- **Cyclomatic complexity**: <10 per method

**Team Metrics**:
- **Onboarding time**: <2 weeks for new developers
- **Feature development time**: <5 days per medium feature
- **Bug fix time**: <2 hours average

---

## Review Schedule

### 6 Months (April 2026)
**Review Criteria**:
- Is test coverage ≥90%?
- Are developers comfortable with Clean Architecture?
- Has architecture prevented defects (domain bugs)?

### 12 Months (October 2026)
**Full ADR Review**:
- Has Clean Architecture improved maintainability?
- Has flexibility enabled infrastructure changes?
- Has team productivity increased?

---

## Related Documents

- **ADR-001: Database Selection** (PostgreSQL)
- **ADR-002: ORM Selection** (Dapper + EF Core)
- **Tech Stack Documentation**: `devforgeai/specs/context/tech-stack.md`
- **Architecture Constraints**: `devforgeai/specs/context/architecture-constraints.md`

---

## Approval and Sign-Off

**Approved By**:
- ✅ Technical Architect
- ✅ Senior Backend Developers (2)
- ✅ Tech Lead

**Date Approved**: 2025-10-28

---

## References

### Books
- **Clean Architecture** (Robert C. Martin, 2017)
- **Domain-Driven Design** (Eric Evans, 2003)
- **Implementing Domain-Driven Design** (Vaughn Vernon, 2013)

### Documentation
- [Clean Architecture with .NET](https://github.com/jasontaylordev/CleanArchitecture)
- [Microsoft: Clean Architecture with ASP.NET Core](https://learn.microsoft.com/en-us/dotnet/architecture/modern-web-apps-azure/common-web-application-architectures)

### Team Resources
- Clean Architecture guide: `docs/architecture/clean-architecture-guide.md`
- DDD patterns: `docs/patterns/domain-driven-design.md`
- CQRS tutorial: `docs/tutorials/cqrs-with-mediatr.md`
