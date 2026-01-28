---
last_updated: [DATE]
status: ENFORCED
architecture_pattern: [CLEAN_ARCHITECTURE | N_TIER | VERTICAL_SLICE | OTHER]
validation: ARCHITECTURE_TESTS + CODE_REVIEW + PRE_COMMIT_HOOK
project: [PROJECT_NAME]
---

# Architecture Constraints

**Architectural boundaries and design rules for this project.**

Architecture Pattern: [PATTERN_NAME]
Enforcement: Architecture unit tests, code review, automated validation

---

## Purpose

This file enforces:
1. Layer dependency rules (what can reference what)
2. Design pattern requirements (Repository, Service, DTO patterns)
3. Transaction boundaries
4. API design constraints

---

## Layer Dependency Rules (ENFORCED)

### Architecture Diagram

```
[CUSTOMIZE BASED ON CHOSEN PATTERN - Clean Architecture Example:]

┌─────────────┐
│     API     │  ← HTTP Controllers/Endpoints
│  (Minimal)  │
└──────┬──────┘
       ↓
┌──────────────┐
│ Application  │  ← Business Services, Use Cases
│  (Services)  │
└──────┬───────┘
       ↓
┌──────────────┐
│   Domain     │  ← Entities, Business Logic
│   (Core)     │
└──────┬───────┘
       ↑
┌──────────────┐
│Infrastructure│  ← Data Access, External Services
│ (Persistence) │
└──────────────┘

Dependencies:
API → Application → Domain
Infrastructure → Domain
Tests → Any Layer
```

### Dependency Matrix

| From ↓ To → | API | Application | Domain | Infrastructure | Tests |
|-------------|-----|-------------|--------|----------------|-------|
| **API** | ✓ | ✓ | ❌ | ❌ | ❌ |
| **Application** | ❌ | ✓ | ✓ | ❌ | ❌ |
| **Domain** | ❌ | ❌ | ✓ | ❌ | ❌ |
| **Infrastructure** | ❌ | ❌ | ✓ | ✓ | ❌ |
| **Tests** | ✓ | ✓ | ✓ | ✓ | ✓ |

✓ = Allowed
❌ = FORBIDDEN (violates architecture)

### Layer Dependency Rules (Specific)

#### Rule 1: Domain CANNOT Reference Infrastructure (CRITICAL)

**Rationale:** Domain should be pure business logic, no infrastructure dependencies.

**❌ FORBIDDEN:**
```csharp
// Domain/Entities/User.cs
using Infrastructure.Repositories;  // ❌ Domain → Infrastructure
using Microsoft.Data.SqlClient;     // ❌ Domain → Database library

public class User
{
    public int Id { get; set; }
    public string Email { get; set; }

    public void Save()
    {
        var repo = new UserRepository();  // ❌ Domain instantiating Infrastructure
        repo.SaveAsync(this).Wait();
    }
}
```

**✅ CORRECT:**
```csharp
// Domain/Entities/User.cs
// ✓ Pure domain entity, ZERO infrastructure dependencies

public class User
{
    public int Id { get; set; }
    public string Email { get; set; }

    public void UpdateEmail(string newEmail)
    {
        // ✓ Domain logic only
        if (string.IsNullOrWhiteSpace(newEmail))
            throw new DomainException("Email cannot be empty");

        if (!IsValidEmail(newEmail))
            throw new DomainException("Invalid email format");

        Email = newEmail;
    }

    private bool IsValidEmail(string email) => /* validation logic */;
}

// Domain/Interfaces/IUserRepository.cs
// ✓ Domain defines interface, Infrastructure implements

public interface IUserRepository
{
    Task<User> GetByIdAsync(int id);
    Task SaveAsync(User user);
}
```

#### Rule 2: Application CANNOT Reference API (CRITICAL)

**Rationale:** Services shouldn't know about HTTP, controllers, or API concerns.

**❌ FORBIDDEN:**
```csharp
// Application/Users/UserService.cs
using API.Controllers;           // ❌ Service → Controller
using Microsoft.AspNetCore.Mvc;  // ❌ Service knows about HTTP

public class UserService
{
    public IActionResult GetUser(int id)  // ❌ Returning HTTP result
    {
        var user = _repository.GetById(id);
        if (user == null)
            return new NotFoundResult();  // ❌ HTTP concern in service
        return new OkObjectResult(user);
    }
}
```

**✅ CORRECT:**
```csharp
// Application/Users/UserService.cs
// ✓ No HTTP dependencies

public class UserService : IUserService
{
    private readonly IUserRepository _repository;

    public async Task<Result<UserDto>> GetUserAsync(int id)
    {
        var user = await _repository.GetByIdAsync(id);
        if (user == null)
            return Result.Failure<UserDto>("User not found");

        var dto = _mapper.Map<UserDto>(user);
        return Result.Success(dto);
    }
}

// API/Endpoints/Users/GetUser.cs
// ✓ API layer handles HTTP concerns

public class GetUser : IEndpoint
{
    private readonly IUserService _userService;

    public async Task<IResult> HandleAsync(int id)
    {
        var result = await _userService.GetUserAsync(id);

        return result.IsSuccess
            ? Results.Ok(result.Value)
            : Results.NotFound(new { error = result.Error });
    }
}
```

#### Rule 3: Infrastructure Can Only Reference Domain

**Rationale:** Data access layer should only know about domain entities, not application logic.

**❌ FORBIDDEN:**
```csharp
// Infrastructure/Repositories/UserRepository.cs
using Application.Services;  // ❌ Infrastructure → Application

public class UserRepository : IUserRepository
{
    private readonly UserService _userService;  // ❌ Repository using service

    public async Task<User> GetByIdAsync(int id)
    {
        // ❌ Repository calling service
        var result = await _userService.GetUserAsync(id);
        return result.Value;
    }
}
```

**✅ CORRECT:**
```csharp
// Infrastructure/Repositories/UserRepository.cs
using Domain.Entities;   // ✓ Infrastructure can reference Domain
using Domain.Interfaces;

public class UserRepository : IUserRepository
{
    private readonly IDbConnection _connection;

    public async Task<User> GetByIdAsync(int id)
    {
        const string sql = "SELECT * FROM Users WHERE Id = @Id";
        return await _connection.QuerySingleOrDefaultAsync<User>(sql, new { Id = id });
    }

    public async Task SaveAsync(User user)
    {
        const string sql = @"
            UPDATE Users
            SET Email = @Email, Name = @Name
            WHERE Id = @Id";

        await _connection.ExecuteAsync(sql, user);
    }
}
```

### Validation (Architecture Tests)

**Automated enforcement:**

```csharp
using NetArchTest.Rules;

[Fact]
public void Domain_ShouldNot_ReferenceInfrastructure()
{
    var domainAssembly = typeof(User).Assembly;

    var result = Types.InAssembly(domainAssembly)
        .ShouldNot()
        .HaveDependencyOn("Infrastructure")
        .And()
        .ShouldNot()
        .HaveDependencyOn("Microsoft.Data.SqlClient")
        .And()
        .ShouldNot()
        .HaveDependencyOn("Dapper")
        .GetResult();

    Assert.True(result.IsSuccessful, "Domain layer has infrastructure dependencies!");
}

[Fact]
public void Application_ShouldNot_ReferenceAPI()
{
    var applicationAssembly = typeof(UserService).Assembly;

    var result = Types.InAssembly(applicationAssembly)
        .ShouldNot()
        .HaveDependencyOn("API")
        .And()
        .ShouldNot()
        .HaveDependencyOn("Microsoft.AspNetCore.Mvc")
        .GetResult();

    Assert.True(result.IsSuccessful, "Application layer references API!");
}

[Fact]
public void Infrastructure_ShouldNot_ReferenceApplication()
{
    var infrastructureAssembly = typeof(UserRepository).Assembly;

    var result = Types.InAssembly(infrastructureAssembly)
        .ShouldNot()
        .HaveDependencyOn("Application")
        .GetResult();

    Assert.True(result.IsSuccessful);
}
```

---

## Design Pattern Constraints

### Repository Pattern (MANDATORY)

**All data access MUST go through repositories.**

**✅ CORRECT:**
```csharp
// Domain/Interfaces/IUserRepository.cs
public interface IUserRepository
{
    Task<User> GetByIdAsync(int id);
    Task<IEnumerable<User>> GetAllAsync();
    Task<User> GetByEmailAsync(string email);
    Task AddAsync(User user);
    Task UpdateAsync(User user);
    Task DeleteAsync(int id);
}

// Infrastructure/Repositories/UserRepository.cs
public class UserRepository : IUserRepository
{
    private readonly IDbConnection _connection;

    public async Task<User> GetByIdAsync(int id)
    {
        const string sql = "SELECT * FROM Users WHERE Id = @Id";
        return await _connection.QuerySingleOrDefaultAsync<User>(sql, new { Id = id });
    }

    // Other methods...
}

// Application/Users/UserService.cs
public class UserService
{
    private readonly IUserRepository _repository;  // ✓ Uses interface

    public async Task<User> GetUserAsync(int id)
    {
        return await _repository.GetByIdAsync(id);  // ✓ Through repository
    }
}
```

**❌ FORBIDDEN:**
```csharp
// Application/Users/UserService.cs
public class UserService
{
    private readonly IDbConnection _connection;  // ❌ Direct DB access in service

    public async Task<User> GetUserAsync(int id)
    {
        // ❌ Service doing data access directly
        const string sql = "SELECT * FROM Users WHERE Id = @Id";
        return await _connection.QuerySingleOrDefaultAsync<User>(sql, new { Id = id });
    }
}
```

### Service Pattern (MANDATORY)

**Business logic MUST be in services, not controllers/endpoints.**

**✅ CORRECT:**
```csharp
// API/Endpoints/Users/CreateUser.cs
public class CreateUser : IEndpoint
{
    private readonly IUserService _userService;  // ✓ Delegates to service

    public async Task<IResult> HandleAsync(CreateUserDto dto)
    {
        var result = await _userService.CreateUserAsync(dto);

        return result.IsSuccess
            ? Results.Created($"/api/users/{result.Value.Id}", result.Value)
            : Results.BadRequest(new { errors = result.Errors });
    }
}

// Application/Users/UserService.cs
public class UserService : IUserService
{
    private readonly IUserRepository _repository;
    private readonly IValidator<CreateUserDto> _validator;

    public async Task<Result<UserDto>> CreateUserAsync(CreateUserDto dto)
    {
        // ✓ Validation in service
        var validationResult = await _validator.ValidateAsync(dto);
        if (!validationResult.IsValid)
            return Result.Failure<UserDto>(validationResult.Errors);

        // ✓ Business logic in service
        var existingUser = await _repository.GetByEmailAsync(dto.Email);
        if (existingUser != null)
            return Result.Failure<UserDto>("Email already exists");

        // ✓ Entity creation in service
        var user = new User
        {
            Email = dto.Email,
            Name = dto.Name,
            CreatedAt = DateTime.UtcNow
        };

        await _repository.AddAsync(user);

        return Result.Success(_mapper.Map<UserDto>(user));
    }
}
```

**❌ FORBIDDEN:**
```csharp
// API/Endpoints/Users/CreateUser.cs
public class CreateUser : IEndpoint
{
    private readonly IUserRepository _repository;  // ❌ Endpoint using repository directly

    public async Task<IResult> HandleAsync(CreateUserDto dto)
    {
        // ❌ Validation in endpoint
        if (string.IsNullOrEmpty(dto.Email))
            return Results.BadRequest("Email required");

        // ❌ Business logic in endpoint
        var existingUser = await _repository.GetByEmailAsync(dto.Email);
        if (existingUser != null)
            return Results.Conflict("Email exists");

        // ❌ No service layer
        var user = new User { Email = dto.Email };
        await _repository.AddAsync(user);

        return Results.Created($"/api/users/{user.Id}", user);
    }
}
```

### DTO Pattern (MANDATORY)

**Never expose domain entities directly to API layer.**

**✅ CORRECT:**
```csharp
// Application/Users/Dtos/UserDto.cs
public class UserDto
{
    public int Id { get; set; }
    public string Email { get; set; }
    public string Name { get; set; }
    // ✓ Only properties API consumers need
}

// Application/Users/UserService.cs
public async Task<Result<UserDto>> GetUserAsync(int id)
{
    var user = await _repository.GetByIdAsync(id);
    if (user == null)
        return Result.Failure<UserDto>("User not found");

    var dto = _mapper.Map<UserDto>(user);  // ✓ Map to DTO
    return Result.Success(dto);
}

// API/Endpoints/Users/GetUser.cs
public async Task<IResult> HandleAsync(int id)
{
    var result = await _userService.GetUserAsync(id);

    return result.IsSuccess
        ? Results.Ok(result.Value)  // ✓ Returns DTO, not entity
        : Results.NotFound();
}
```

**❌ FORBIDDEN:**
```csharp
// Application/Users/UserService.cs
public async Task<User> GetUserAsync(int id)  // ❌ Returning domain entity
{
    return await _repository.GetByIdAsync(id);
}

// API/Endpoints/Users/GetUser.cs
public async Task<IResult> HandleAsync(int id)
{
    var user = await _userService.GetUserAsync(id);
    return Results.Ok(user);  // ❌ Exposing domain entity to API
    // Risks: Internal properties exposed, lazy loading issues, etc.
}
```

---

## Transaction Management

**Transactions MUST be at service layer, not repository.**

**✅ CORRECT:**
```csharp
// Application/Orders/OrderService.cs
public class OrderService : IOrderService
{
    private readonly IDbConnection _connection;
    private readonly IOrderRepository _orderRepository;
    private readonly IOrderItemRepository _orderItemRepository;

    public async Task<Result<int>> CreateOrderWithItemsAsync(CreateOrderDto dto)
    {
        using var transaction = _connection.BeginTransaction();
        try
        {
            // ✓ Transaction managed by service (orchestrates multiple repos)
            var order = _mapper.Map<Order>(dto);
            var orderId = await _orderRepository.AddAsync(order, transaction);

            foreach (var itemDto in dto.Items)
            {
                var item = _mapper.Map<OrderItem>(itemDto);
                item.OrderId = orderId;
                await _orderItemRepository.AddAsync(item, transaction);
            }

            transaction.Commit();
            return Result.Success(orderId);
        }
        catch (Exception ex)
        {
            transaction.Rollback();
            _logger.LogError(ex, "Failed to create order with items");
            return Result.Failure<int>("Failed to create order");
        }
    }
}
```

**❌ FORBIDDEN:**
```csharp
// Infrastructure/Repositories/OrderRepository.cs
public class OrderRepository : IOrderRepository
{
    public async Task<int> AddAsync(Order order)
    {
        // ❌ Transaction in repository (can't coordinate with other repos)
        using var transaction = _connection.BeginTransaction();
        try
        {
            var orderId = await /* insert order */;

            // ❌ Repository crossing boundaries to insert items
            await /* insert items */;

            transaction.Commit();
            return orderId;
        }
        catch
        {
            transaction.Rollback();
            throw;
        }
    }
}
```

---

## API Design Constraints

### RESTful Conventions (MANDATORY)

**Follow REST principles for consistency.**

**✅ CORRECT Resource Naming:**
```
GET    /api/users              # Get all users
GET    /api/users/{id}         # Get specific user
POST   /api/users              # Create user
PUT    /api/users/{id}         # Update entire user
PATCH  /api/users/{id}         # Partial update
DELETE /api/users/{id}         # Delete user

GET    /api/users/{id}/orders  # Get user's orders
POST   /api/orders             # Create order
```

**❌ FORBIDDEN:**
```
GET    /api/getUsers           # ❌ Verb in URL
POST   /api/user/create        # ❌ Action in URL
GET    /api/users/get/{id}     # ❌ Redundant verb
DELETE /api/deleteUser?id={id} # ❌ Action + query param
POST   /api/createOrder         # ❌ Verb in URL
```

### HTTP Status Codes (MANDATORY)

**Use correct status codes:**

```csharp
// ✅ CORRECT
return Results.Ok(data);                          // 200 - Success
return Results.Created($"/api/users/{id}", user); // 201 - Created
return Results.NoContent();                       // 204 - Deleted/Updated (no content)
return Results.BadRequest(errors);                // 400 - Validation failed
return Results.Unauthorized();                    // 401 - Not authenticated
return Results.Forbid();                          // 403 - Not authorized
return Results.NotFound();                        // 404 - Resource not found
return Results.Conflict("Email exists");          // 409 - Business rule violation
return Results.StatusCode(500);                   // 500 - Server error

// ❌ FORBIDDEN
return Results.Ok("User not found");  // ❌ Should be 404, not 200
return Results.Ok(null);              // ❌ Should be 404 for missing resource
return Results.BadRequest("Server error");  // ❌ Server error should be 500
```

---

## AI Agent Enforcement Checklist

**Before writing ANY code, AI agents must verify:**

### Pre-Flight Checklist

- [ ] File being created is in correct layer (check source-tree.md)
- [ ] Dependencies only flow in allowed direction (check dependency matrix above)
- [ ] Using repository pattern for data access (not direct DB in services)
- [ ] Business logic is in service layer (not in endpoints/controllers)
- [ ] Using DTOs for API responses (not domain entities)
- [ ] Transaction management is in service layer (if multi-repo operation)
- [ ] Following REST conventions (if API endpoint)
- [ ] HTTP status codes are correct (if API endpoint)

**If ANY check fails → STOP and use AskUserQuestion**

### Cross-Layer Reference Detection

**AI must reject these using statements:**

```csharp
// In Domain layer:
using Infrastructure.Repositories;  // ❌ FORBIDDEN
using Microsoft.Data.SqlClient;     // ❌ FORBIDDEN
using Dapper;                        // ❌ FORBIDDEN

// In Application layer:
using API.Controllers;               // ❌ FORBIDDEN
using Microsoft.AspNetCore.Mvc;     // ❌ FORBIDDEN

// In Infrastructure layer:
using Application.Services;          // ❌ FORBIDDEN
```

### AskUserQuestion Triggers

**AI must use AskUserQuestion if:**

- Unclear which layer new code belongs in
- Need to reference layer that seems forbidden
- Pattern not documented in coding-standards.md
- Transaction boundary unclear (single repo or multiple?)
- API design doesn't fit REST conventions

---

## Related Documents

- [source-tree.md](./source-tree.md) - Layer directory structure
- [coding-standards.md](./coding-standards.md) - Code patterns within layers
- [anti-patterns.md](./anti-patterns.md) - Forbidden cross-layer patterns
- [tech-stack.md](./tech-stack.md) - Technology decisions

---

## Notes

[Project-specific architectural notes, exceptions, or clarifications]

**Example:**
```
Note: We chose Clean Architecture to achieve:
- Domain independence (testable without infrastructure)
- Dependency inversion (domain at center, everything depends on it)
- Flexibility (can swap infrastructure without touching domain)

Layer tests enforce these boundaries automatically.
```
