---
last_updated: [DATE]
status: ENFORCED
applies_to: [LANGUAGES - e.g., C#, TypeScript]
enforcement: CODE_REVIEW + LINTING + PRE_COMMIT_HOOK
project: [PROJECT_NAME]
---

# Coding Standards

**Project-specific code patterns and conventions.**

This file documents HOW to write code for this specific project.

---

## Purpose

While tech-stack.md defines WHAT technologies to use, this file shows HOW to use them correctly for this project.

**Key Sections:**
1. Technology-Specific Patterns (Dapper, Zustand, etc.)
2. Async/Await Standards
3. Dependency Injection Patterns
4. Validation Patterns
5. Error Handling & Logging
6. Naming Conventions
7. File Organization

---

## Technology-Specific Patterns

### [TECHNOLOGY_1] Patterns (e.g., Dapper for Data Access)

**⚠️ CRITICAL:** These patterns are project-specific and MANDATORY.

#### Pattern 1: Parameterized Queries (Security)

**✅ CORRECT:**
```csharp
public async Task<User> GetUserByEmailAsync(string email)
{
    const string sql = "SELECT * FROM Users WHERE Email = @Email";
    return await _connection.QuerySingleOrDefaultAsync<User>(
        sql,
        new { Email = email }  // ✓ Parameterized - prevents SQL injection
    );
}
```

**❌ FORBIDDEN:**
```csharp
// ❌ SQL INJECTION VULNERABILITY!!!
public async Task<User> GetUserByEmail(string email)
{
    var sql = $"SELECT * FROM Users WHERE Email = '{email}'";
    return await _connection.QuerySingleOrDefaultAsync<User>(sql);
}
```

#### Pattern 2: Multi-Mapping (Joins)

**✅ CORRECT:**
```csharp
public async Task<IEnumerable<Order>> GetOrdersWithUsersAsync()
{
    const string sql = @"
        SELECT o.*, u.*
        FROM Orders o
        INNER JOIN Users u ON o.UserId = u.Id";

    return await _connection.QueryAsync<Order, User, Order>(
        sql,
        (order, user) => {
            order.User = user;
            return order;
        },
        splitOn: "Id"  // ✓ Explicit split column
    );
}
```

#### Pattern 3: Transactions

**✅ CORRECT:**
```csharp
public async Task<int> CreateOrderWithItemsAsync(Order order, List<OrderItem> items)
{
    using var transaction = _connection.BeginTransaction();
    try
    {
        const string orderSql = @"
            INSERT INTO Orders (UserId, OrderDate, Total)
            VALUES (@UserId, @OrderDate, @Total);
            SELECT CAST(SCOPE_IDENTITY() as int)";

        var orderId = await _connection.ExecuteScalarAsync<int>(
            orderSql,
            order,
            transaction
        );

        const string itemsSql = @"
            INSERT INTO OrderItems (OrderId, ProductId, Quantity, Price)
            VALUES (@OrderId, @ProductId, @Quantity, @Price)";

        await _connection.ExecuteAsync(itemsSql, items, transaction);

        transaction.Commit();
        return orderId;
    }
    catch
    {
        transaction.Rollback();
        throw;
    }
}
```

**AI Agent Rule:**
Before writing Dapper code, read this section. Follow these exact patterns.
DO NOT introduce Entity Framework patterns (DbContext, ChangeTracker, SaveChanges).

---

### [TECHNOLOGY_2] Patterns (e.g., Zustand for State Management)

**⚠️ CRITICAL:** These patterns prevent Redux contamination.

#### Pattern 1: Basic Store

**✅ CORRECT:**
```typescript
interface UserStore {
  users: User[];
  fetchUsers: () => Promise<void>;
  addUser: (user: User) => void;
  updateUser: (id: string, updates: Partial<User>) => void;
  removeUser: (id: string) => void;
}

export const useUserStore = create<UserStore>((set, get) => ({
  users: [],

  fetchUsers: async () => {
    const users = await api.getUsers();
    set({ users });
  },

  addUser: (user) => set((state) => ({
    users: [...state.users, user]
  })),

  updateUser: (id, updates) => set((state) => ({
    users: state.users.map(u =>
      u.id === id ? { ...u, ...updates } : u
    )
  })),

  removeUser: (id) => set((state) => ({
    users: state.users.filter(u => u.id !== id)
  })),
}));
```

**❌ FORBIDDEN:**
```typescript
// ❌ NO Redux patterns in Zustand project!

// ❌ NO action types
const ADD_USER = 'users/add';

// ❌ NO action creators
export const addUser = (user) => ({ type: ADD_USER, payload: user });

// ❌ NO reducers
const userReducer = (state, action) => { /* ... */ };

// ❌ NO dispatch
store.dispatch(addUser(newUser));
```

#### Pattern 2: Derived State (Selectors)

**✅ CORRECT:**
```typescript
export const useUserStore = create<UserStore>((set, get) => ({
  users: [],

  // ✓ Zustand selector pattern
  getActiveUsers: () => get().users.filter(u => u.isActive),

  getUserById: (id: string) => get().users.find(u => u.id === id),
}));

// Usage:
const activeUsers = useUserStore(state => state.getActiveUsers());
```

#### Pattern 3: Middleware (Persistence, DevTools)

**✅ CORRECT:**
```typescript
import { create } from 'zustand';
import { persist, devtools } from 'zustand/middleware';

export const useUserStore = create<UserStore>()(
  devtools(
    persist(
      (set, get) => ({
        users: [],
        // ... store logic
      }),
      {
        name: 'user-storage',
      }
    )
  )
);
```

**AI Agent Rule:**
Use ONLY Zustand patterns. NO Redux, NO MobX, NO Recoil patterns.
If needing features not shown here, use AskUserQuestion.

---

## Async/Await Standards

### Pattern 1: Async Method Naming

**✅ CORRECT:**
```csharp
// ✓ Async methods end with "Async"
public async Task<User> GetUserAsync(int id)
{
    return await _repository.GetByIdAsync(id);
}

public async Task UpdateUserAsync(User user)
{
    await _repository.UpdateAsync(user);
}
```

**❌ FORBIDDEN:**
```csharp
// ❌ Missing "Async" suffix
public async Task<User> GetUser(int id)  // ❌ Should be GetUserAsync
{
    return await _repository.GetByIdAsync(id);
}
```

### Pattern 2: Await All the Way

**✅ CORRECT:**
```csharp
// ✓ Async all the way through the call stack
public async Task<IActionResult> GetUser(int id)
{
    var user = await _userService.GetUserAsync(id);
    return Ok(user);
}

public async Task<User> GetUserAsync(int id)
{
    return await _repository.GetByIdAsync(id);
}
```

**❌ FORBIDDEN:**
```csharp
// ❌ Blocking async calls (causes deadlocks!)
public User GetUser(int id)
{
    return _repository.GetByIdAsync(id).Result;  // ❌ .Result blocks
}

public User GetUser(int id)
{
    return _repository.GetByIdAsync(id).GetAwaiter().GetResult();  // ❌ Still blocking
}

// ❌ async void (except event handlers)
public async void ProcessOrder(Order order)  // ❌ Should return Task
{
    await _orderService.ProcessAsync(order);
}
```

### Pattern 3: ConfigureAwait

**✅ CORRECT (for library code):**
```csharp
// In library/service code: Use ConfigureAwait(false)
public async Task<User> GetUserAsync(int id)
{
    var user = await _repository.GetByIdAsync(id).ConfigureAwait(false);
    return user;
}
```

**✅ CORRECT (for ASP.NET Core):**
```csharp
// In ASP.NET Core: ConfigureAwait(false) not needed
public async Task<IActionResult> GetUser(int id)
{
    var user = await _service.GetUserAsync(id);  // ✓ No ConfigureAwait needed
    return Ok(user);
}
```

---

## Dependency Injection Patterns

### Pattern 1: Constructor Injection (MANDATORY)

**✅ CORRECT:**
```csharp
public class UserService : IUserService
{
    private readonly IUserRepository _repository;
    private readonly ILogger<UserService> _logger;
    private readonly IValidator<User> _validator;

    public UserService(
        IUserRepository repository,
        ILogger<UserService> logger,
        IValidator<User> validator)
    {
        _repository = repository ?? throw new ArgumentNullException(nameof(repository));
        _logger = logger ?? throw new ArgumentNullException(nameof(logger));
        _validator = validator ?? throw new ArgumentNullException(nameof(validator));
    }
}
```

**❌ FORBIDDEN:**
```csharp
// ❌ Service Locator (anti-pattern)
public class UserService
{
    private readonly IUserRepository _repository;

    public UserService()
    {
        _repository = ServiceLocator.Get<IUserRepository>();  // ❌ Bad!
    }
}

// ❌ Property Injection (avoid unless absolutely necessary)
public class UserService
{
    public IUserRepository Repository { get; set; }  // ❌ Prefer constructor
}

// ❌ new keyword for dependencies
public class UserService
{
    private readonly IUserRepository _repository;

    public UserService()
    {
        _repository = new UserRepository();  // ❌ Tight coupling
    }
}
```

### Pattern 2: Service Registration

**✅ CORRECT:**
```csharp
// Program.cs
builder.Services.AddScoped<IUserService, UserService>();
builder.Services.AddScoped<IUserRepository, UserRepository>();
builder.Services.AddSingleton<IEmailService, EmailService>();
builder.Services.AddTransient<IValidator<User>, UserValidator>();

// ✓ Explicit lifetimes
// Scoped: Per HTTP request (default for most services)
// Singleton: Single instance for app lifetime (e.g., configuration, caching)
// Transient: New instance every time (e.g., lightweight stateless services)
```

---

## Validation Patterns (FluentValidation)

### Pattern 1: Validator Class

**✅ CORRECT:**
```csharp
public class CreateUserValidator : AbstractValidator<CreateUserDto>
{
    public CreateUserValidator()
    {
        RuleFor(x => x.Email)
            .NotEmpty().WithMessage("Email is required")
            .EmailAddress().WithMessage("Invalid email format")
            .MaximumLength(100).WithMessage("Email too long");

        RuleFor(x => x.Password)
            .NotEmpty()
            .MinimumLength(8).WithMessage("Password must be at least 8 characters")
            .Matches(@"[A-Z]").WithMessage("Password must contain uppercase letter")
            .Matches(@"[a-z]").WithMessage("Password must contain lowercase letter")
            .Matches(@"[0-9]").WithMessage("Password must contain number");

        RuleFor(x => x.Age)
            .GreaterThanOrEqualTo(18).WithMessage("Must be 18 or older");
    }
}
```

**❌ FORBIDDEN:**
```csharp
// ❌ Data Annotations (less flexible than FluentValidation)
public class CreateUserDto
{
    [Required]
    [EmailAddress]
    public string Email { get; set; }  // ❌ We use FluentValidation
}

// ❌ Manual validation (inconsistent, hard to test)
public void CreateUser(CreateUserDto dto)
{
    if (string.IsNullOrEmpty(dto.Email))
        throw new ValidationException("Email required");

    if (!dto.Email.Contains("@"))
        throw new ValidationException("Invalid email");
}
```

---

## Error Handling & Logging

### Pattern 1: Result Pattern (Preferred)

**✅ CORRECT:**
```csharp
public async Task<Result<User>> CreateUserAsync(CreateUserDto dto)
{
    var validationResult = await _validator.ValidateAsync(dto);
    if (!validationResult.IsValid)
        return Result.Failure<User>(validationResult.Errors);

    var user = _mapper.Map<User>(dto);

    try
    {
        await _repository.AddAsync(user);
        return Result.Success(user);
    }
    catch (DuplicateEmailException ex)
    {
        _logger.LogWarning(ex, "Duplicate email: {Email}", dto.Email);
        return Result.Failure<User>("Email already exists");
    }
}
```

### Pattern 2: Structured Logging

**✅ CORRECT:**
```csharp
// ✓ Structured logging with parameters
_logger.LogInformation("Creating user with email {Email}", dto.Email);
_logger.LogWarning("User {UserId} not found", id);
_logger.LogError(ex, "Failed to create user {Email}", dto.Email);

// ✓ Log levels
// Trace: Very detailed logs (development only)
// Debug: Diagnostic information (development/troubleshooting)
// Information: Normal operations, key events
// Warning: Unexpected but handled situations
// Error: Failures requiring attention
// Critical: System-wide failures
```

**❌ FORBIDDEN:**
```csharp
// ❌ String interpolation in logs (allocates strings even if not logged)
_logger.LogInformation($"Creating user with email {dto.Email}");

// ❌ Console.WriteLine (not production-ready)
Console.WriteLine("User created");  // ❌ Use ILogger

// ❌ Logging sensitive data
_logger.LogInformation("User password: {Password}", dto.Password);  // ❌ SECURITY RISK!
_logger.LogInformation("Credit card: {Card}", dto.CreditCard);  // ❌ PCI violation!
```

---

## Naming Conventions

### C# Backend

**Classes:** PascalCase
```csharp
public class UserService { }         // ✓
public class OrderRepository { }     // ✓
public class EmailHelper { }         // ✓
```

**Interfaces:** `I` prefix + PascalCase
```csharp
public interface IUserService { }    // ✓
public interface IOrderRepository { }// ✓
```

**Private Fields:** `_camelCase` (underscore prefix)
```csharp
private readonly IUserRepository _repository;  // ✓
private readonly ILogger _logger;              // ✓
```

**Methods:** PascalCase (verb-first)
```csharp
public async Task<User> GetUserAsync(int id) { }      // ✓
public async Task CreateOrderAsync(Order order) { }   // ✓
public bool IsValidEmail(string email) { }            // ✓
```

### TypeScript/React Frontend

**Components:** PascalCase
```typescript
export const UserProfile = () => { }   // ✓
export const OrderList = () => { }     // ✓
```

**Hooks:** `use` prefix + camelCase
```typescript
export const useAuth = () => { }      // ✓
export const useUsers = () => { }     // ✓
```

**Functions/Variables:** camelCase
```typescript
const formatCurrency = (amount: number) => { }  // ✓
const isValidEmail = (email: string) => { }     // ✓
```

**Constants:** UPPER_SNAKE_CASE or camelCase
```typescript
const MAX_RETRY_COUNT = 3;           // ✓
const API_BASE_URL = 'https://...';  // ✓
```

---

## File Organization

### One Class Per File (MANDATORY)

**✅ CORRECT:**
```
UserService.cs → class UserService
IUserService.cs → interface IUserService
UserDto.cs → class UserDto
```

**❌ FORBIDDEN:**
```
UserService.cs → class UserService, interface IUserService, class UserDto  // ❌
```

### File Naming Matches Class Name

**✅ CORRECT:**
```
File: UserService.cs
Contains: public class UserService
```

**❌ FORBIDDEN:**
```
File: UserService.cs
Contains: public class UserManager  // ❌ Name mismatch
```

---

## AI Agent Integration Rules

**Before writing ANY code, AI agents must:**

1. **Read relevant sections** of this file for the technology being used
2. **Follow exact patterns** shown in examples above
3. **Never introduce alternative patterns** without AskUserQuestion
4. **If pattern not documented** → Use AskUserQuestion:

```
Question: "How should I implement [scenario]? coding-standards.md doesn't document this pattern."
Header: "Code pattern"
Options: [Suggest 2-3 approaches with examples]
Description: "Will be documented in coding-standards.md after approval"
multiSelect: false
```

### Pattern Enforcement Checklist

**For Dapper code:**
- [ ] Uses parameterized queries (security)
- [ ] Follows multi-mapping pattern (if joins)
- [ ] Uses transactions properly (if multi-table)
- [ ] NO Entity Framework patterns

**For Zustand code:**
- [ ] Uses direct actions (no Redux patterns)
- [ ] NO action types or action creators
- [ ] NO dispatch pattern
- [ ] Selectors use get() function

**For async code:**
- [ ] Method names end with "Async"
- [ ] Uses await throughout (no .Result or .Wait())
- [ ] Returns Task or Task<T>
- [ ] NO async void (except event handlers)

**For validation:**
- [ ] Uses FluentValidation (not DataAnnotations)
- [ ] Validator classes inherit AbstractValidator<T>
- [ ] NO manual validation in services

**For logging:**
- [ ] Uses structured logging (not string interpolation)
- [ ] Uses ILogger<T> (not Console.WriteLine)
- [ ] NO sensitive data logged

---

## Related Documents

- [tech-stack.md](./tech-stack.md) - Technology choices
- [dependencies.md](./dependencies.md) - Approved packages
- [anti-patterns.md](./anti-patterns.md) - What NOT to do
- [architecture-constraints.md](./architecture-constraints.md) - Layer rules

---

## Notes

[Project-specific notes, exceptions, or additional context]

**Example:**
```
Note: This project uses Dapper exclusively for data access. DO NOT introduce
Entity Framework patterns even if they seem "easier" for a specific scenario.
Consistency is more important than convenience for individual features.
```
