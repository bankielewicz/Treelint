---
last_updated: [DATE]
status: ENFORCED
violation_severity: CRITICAL
enforcement: PRE_COMMIT_HOOK + CODE_REVIEW + ARCHITECTURE_TESTS
project: [PROJECT_NAME]
---

# Anti-Patterns (FORBIDDEN)

**These patterns are EXPLICITLY FORBIDDEN in this project.**

AI agents must NEVER implement these patterns. Violations create technical debt.

---

## Philosophy

**"Explicitly forbid what causes technical debt."**

Instead of hoping AI agents avoid mistakes, we document exactly what NOT to do with specific code examples showing wrong vs. right approaches.

### Why This File Exists

LLMs are powerful but can:
- Substitute libraries when encountering difficulties
- Mix frameworks (Redux in Zustand projects)
- Create files in random locations
- Violate layer boundaries
- Make assumptions about unclear requirements

This file PREVENTS these mistakes by showing explicit examples of forbidden patterns.

---

## Anti-Pattern Categories

### Category 1: Library Substitution
### Category 2: Structure Violation
### Category 3: Cross-Layer Dependency Violation
### Category 4: Framework Mixing
### Category 5: Magic Numbers/Strings
### Category 6: Improper Error Handling
### Category 7: Test Structure Violations
### Category 8: Direct Database Access in Services
### Category 9: Assumption-Based Development
### Category 10: Configuration Hardcoding

---

## ❌ Anti-Pattern 1: Library Substitution

**SEVERITY: CRITICAL**

### Description

Swapping approved libraries when encountering difficulties instead of debugging the approved library.

### Why It's Forbidden

- Violates tech-stack.md and dependencies.md decisions
- Creates dependency sprawl and version conflicts
- Fragments team knowledge across multiple tools
- Introduces technical debt that compounds over time
- Makes onboarding harder (which library do we use?)

### Specific Examples

#### Example 1.1: ORM Substitution (Dapper → Entity Framework)

**❌ FORBIDDEN:**

```csharp
// Scenario: Dapper query is complex
// BAD AI behavior:

// AI encounters this Dapper query:
public async Task<IEnumerable<OrderWithDetails>> GetOrdersWithDetails()
{
    const string sql = @"
        SELECT o.*, u.*, oi.*, p.*
        FROM Orders o
        INNER JOIN Users u ON o.UserId = u.Id
        INNER JOIN OrderItems oi ON o.Id = oi.OrderId
        INNER JOIN Products p ON oi.ProductId = p.Id";

    // AI thinks: "This multi-mapping is complex, EF Core would be easier"
    // WRONG: Proceeds to add Entity Framework
}

// AI executes:
dotnet add package Microsoft.EntityFrameworkCore
dotnet add package Microsoft.EntityFrameworkCore.SqlServer

// AI creates:
public class AppDbContext : DbContext
{
    public DbSet<Order> Orders { get; set; }
    public DbSet<User> Users { get; set; }
    public DbSet<OrderItem> OrderItems { get; set; }
    public DbSet<Product> Products { get; set; }
}

// NOW PROJECT HAS BOTH DAPPER AND EF CORE! (Major technical debt)
// Some queries use Dapper, some use EF - total inconsistency
```

**✅ CORRECT:**

```csharp
// Scenario: Dapper query is complex
// CORRECT AI behavior:

// AI encounters complex query and thinks: "This is complex. Let me check tech-stack.md"

// AI reads tech-stack.md:
// "Data Access: Dapper (LOCKED)"
// "PROHIBITED: Entity Framework Core"
// "If issues arise: Use AskUserQuestion to debug, DO NOT substitute"

// AI uses AskUserQuestion:
Question: "This Dapper query requires complex multi-mapping. Should I:"
Header: "Data access"
Options:
  - "Optimize the Dapper multi-mapping query (maintain tech stack)"
  - "Break into simpler separate queries"
  - "Create a stored procedure for complex query"
  - "Reconsider ORM choice (requires ADR and approval)"
Description: "tech-stack.md locks data access to Dapper"
multiSelect: false

// User selects: "Optimize the Dapper multi-mapping query"

// AI implements proper Dapper solution:
public async Task<IEnumerable<OrderWithDetails>> GetOrdersWithDetails()
{
    const string sql = @"
        SELECT o.*, u.*, oi.*, p.*
        FROM Orders o
        INNER JOIN Users u ON o.UserId = u.Id
        INNER JOIN OrderItems oi ON o.Id = oi.OrderId
        INNER JOIN Products p ON oi.ProductId = p.Id";

    var orderDict = new Dictionary<int, OrderWithDetails>();

    await _connection.QueryAsync<Order, User, OrderItem, Product, OrderWithDetails>(
        sql,
        (order, user, orderItem, product) =>
        {
            if (!orderDict.TryGetValue(order.Id, out var orderEntry))
            {
                orderEntry = new OrderWithDetails { /* map order and user */ };
                orderDict.Add(order.Id, orderEntry);
            }

            orderItem.Product = product;
            orderEntry.Items.Add(orderItem);
            return orderEntry;
        },
        splitOn: "Id,Id,Id"
    );

    return orderDict.Values;
}

// ✓ Maintains Dapper consistency
// ✓ Respects tech-stack.md
// ✓ No technical debt
```

#### Example 1.2: State Management Substitution (Zustand → Redux)

**❌ FORBIDDEN:**

```typescript
// Scenario: Complex state management needed
// BAD AI behavior:

// AI encounters complex state requirements:
// - Multiple async actions
// - Derived state
// - State persistence

// AI thinks: "Redux Toolkit would handle this better"
// WRONG: Proceeds to add Redux

// AI executes:
npm install @reduxjs/toolkit react-redux

// AI creates Redux setup:
import { createSlice } from '@reduxjs/toolkit';

const userSlice = createSlice({
  name: 'users',
  initialState: { users: [], loading: false },
  reducers: {
    setUsers: (state, action) => { /* ... */ },
    addUser: (state, action) => { /* ... */ },
  },
});

// NOW PROJECT HAS BOTH ZUSTAND AND REDUX! (Framework mixing)
// Some components use Zustand, some use Redux - complete chaos
```

**✅ CORRECT:**

```typescript
// Scenario: Complex state management needed
// CORRECT AI behavior:

// AI encounters complex requirements and checks tech-stack.md:
// "State Management: Zustand (LOCKED)"
// "PROHIBITED: Redux, Redux Toolkit"
// "If limitations: Use AskUserQuestion before adding alternatives"

// AI uses AskUserQuestion:
Question: "Need complex state management with async actions and persistence. Should I:"
Header: "State mgmt"
Options:
  - "Implement in Zustand with middleware (maintain tech stack)"
  - "Split into multiple simpler stores"
  - "Use Zustand with zustand/middleware for persistence"
  - "Reconsider state library (requires ADR)"
Description: "tech-stack.md locks state management to Zustand"
multiSelect: false

// User selects: "Implement in Zustand with middleware"

// AI implements Zustand solution:
import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface UserStore {
  users: User[];
  loading: boolean;
  fetchUsers: () => Promise<void>;
  addUser: (user: User) => Promise<void>;
  // Derived state
  activeUsers: () => User[];
}

export const useUserStore = create<UserStore>()(
  persist(
    (set, get) => ({
      users: [],
      loading: false,

      fetchUsers: async () => {
        set({ loading: true });
        try {
          const users = await api.getUsers();
          set({ users, loading: false });
        } catch (error) {
          set({ loading: false });
        }
      },

      addUser: async (user) => {
        const newUser = await api.createUser(user);
        set((state) => ({ users: [...state.users, newUser] }));
      },

      // Derived state selector
      activeUsers: () => get().users.filter(u => u.isActive),
    }),
    {
      name: 'user-storage',
    }
  )
);

// ✓ Pure Zustand patterns
// ✓ No Redux contamination
// ✓ Maintains tech stack consistency
```

### Detection Mechanisms

**Pre-commit hook:**
```bash
# Check for unauthorized ORM packages
if grep -q "Microsoft.EntityFrameworkCore" **/*.csproj; then
    echo "❌ Entity Framework detected but Dapper is locked"
    exit 1
fi

# Check for Redux in Zustand project
if grep -q "@reduxjs/toolkit" package.json && grep -q "zustand" package.json; then
    echo "❌ Redux and Zustand both present - framework mixing forbidden"
    exit 1
fi
```

**Architecture tests:**
```csharp
[Fact]
public void Project_ShouldNot_ReferenceForbiddenORMs()
{
    var assemblies = GetProjectAssemblies();
    var forbiddenOrms = new[] {
        "Microsoft.EntityFrameworkCore",
        "NHibernate"
    };

    foreach (var assembly in assemblies)
    {
        var references = assembly.GetReferencedAssemblies();
        var forbidden = references.Where(r => forbiddenOrms.Contains(r.Name));
        Assert.Empty(forbidden);
    }
}
```

### Enforcement

**AI agents must:**
1. Check tech-stack.md before suggesting any library
2. Check dependencies.md for approved packages list
3. If encountering difficulty with approved library → Use AskUserQuestion
4. NEVER silently substitute approved libraries
5. Document any approved substitutions in ADR

---

## ❌ Anti-Pattern 2: Structure Violation

**SEVERITY: HIGH**

### Description

Creating files outside documented source-tree.md structure or inventing new directory structures.

### Why It's Forbidden

- Makes codebase navigation confusing
- Breaks IDE/tooling expectations
- Harder for new developers to onboard
- Violates team conventions
- Creates inconsistent organization

### Specific Examples

#### Example 2.1: Inventing New Directories

**❌ FORBIDDEN:**

```
// BAD AI behavior: Creates directories not in source-tree.md

src/
├── Core/                    // ❌ Not documented in source-tree.md
│   ├── Services/            // ❌ Invented structure
│   │   └── EmailService.cs
│   └── Utilities/           // ❌ Not documented
│       └── StringHelper.cs
├── Helpers/                 // ❌ Not in source-tree.md
│   └── DateHelper.cs
└── Common/                  // ❌ Ambiguous location
    └── ValidationHelper.cs
```

**✅ CORRECT:**

```
// CORRECT AI behavior: Follows source-tree.md

// AI checks source-tree.md before creating EmailService:
// "Services belong in: src/Application/[Feature]/"
// "Truly shared utilities: src/Application/Common/"

src/
└── Application/
    ├── Email/                      // ✓ Feature-based organization
    │   └── EmailService.cs         // ✓ Follows source-tree.md
    ├── Common/                     // ✓ Documented location for shared code
    │   ├── Extensions/
    │   │   ├── StringExtensions.cs  // ✓ Static extension methods
    │   │   └── DateExtensions.cs
    │   └── Validators/
    │       └── ValidationHelpers.cs
```

**If location ambiguous, AI uses AskUserQuestion:**

```
Question: "I need to create EmailService for sending emails. Where should it go?"
Header: "File location"
Options:
  - "src/Application/Email/EmailService.cs (business logic)"
  - "src/Infrastructure/Email/EmailService.cs (external service)"
Description: "Will be documented in source-tree.md for future reference"
multiSelect: false
```

#### Example 2.2: Test Structure Not Mirroring Source

**❌ FORBIDDEN:**

```
Source: src/Application/Users/UserService.cs

// BAD test locations:
tests/UserTests.cs                        // ❌ Doesn't mirror source
tests/Services/UserServiceTests.cs        // ❌ Organized by type, not feature
tests/UnitTests/UserServiceTests.cs       // ❌ Missing layer information
```

**✅ CORRECT:**

```
Source: src/Application/Users/UserService.cs
Test:   tests/UnitTests/Application.UnitTests/Users/UserServiceTests.cs
        ✓ Mirrors source structure exactly

Source: src/Infrastructure/Repositories/UserRepository.cs
Test:   tests/IntegrationTests/Infrastructure.IntegrationTests/Repositories/UserRepositoryTests.cs
        ✓ Mirrors source structure
```

### Detection

```bash
# Pre-commit hook checks for files outside source-tree.md structure
python devforgeai/scripts/validate_structure.py

# Fails if:
# - Files in undocumented directories
# - Test structure doesn't mirror source
# - Naming conventions violated
```

---

## ❌ Anti-Pattern 3: Cross-Layer Dependency Violation

**SEVERITY: CRITICAL**

### Description

Domain layer referencing Infrastructure, or other forbidden layer dependencies per architecture-constraints.md.

### Why It's Forbidden

- Violates Clean Architecture principles
- Makes testing harder (domain should be pure)
- Creates circular dependencies
- Couples business logic to infrastructure details
- Breaks Dependency Inversion Principle

### Specific Examples

#### Example 3.1: Domain Calling Infrastructure

**❌ FORBIDDEN:**

```csharp
// Domain/Entities/User.cs
using Infrastructure.Repositories;  // ❌ Domain → Infrastructure
using Microsoft.Data.SqlClient;     // ❌ Domain → Data access library

public class User
{
    public int Id { get; set; }
    public string Email { get; set; }

    public void Save()
    {
        // ❌ Domain entity directly using repository
        var repo = new UserRepository();
        repo.SaveAsync(this).Wait();
    }

    public static User LoadFromDatabase(int id)
    {
        // ❌ Domain entity directly accessing database
        using var connection = new SqlConnection("connection string");
        const string sql = "SELECT * FROM Users WHERE Id = @Id";
        return connection.QuerySingle<User>(sql, new { Id = id });
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

    // ✓ Domain logic only
    public void UpdateEmail(string newEmail)
    {
        if (string.IsNullOrWhiteSpace(newEmail))
            throw new DomainException("Email cannot be empty");

        if (!EmailValidator.IsValid(newEmail))
            throw new DomainException("Invalid email format");

        Email = newEmail;
    }

    public bool IsActive => /* business rule */;
}

// Domain/Interfaces/IUserRepository.cs
// ✓ Domain defines interface, Infrastructure implements

public interface IUserRepository
{
    Task<User> GetByIdAsync(int id);
    Task SaveAsync(User user);
}

// Application/Users/UserService.cs
// ✓ Application layer orchestrates persistence

public class UserService : IUserService
{
    private readonly IUserRepository _repository;

    public UserService(IUserRepository repository)
    {
        _repository = repository;
    }

    public async Task UpdateUserEmailAsync(int userId, string newEmail)
    {
        var user = await _repository.GetByIdAsync(userId);
        user.UpdateEmail(newEmail);  // ✓ Domain logic
        await _repository.SaveAsync(user);  // ✓ Infrastructure abstraction
    }
}
```

#### Example 3.2: Application Referencing API Layer

**❌ FORBIDDEN:**

```csharp
// Application/Users/UserService.cs
using API.Controllers;           // ❌ Service → Controller
using Microsoft.AspNetCore.Mvc;  // ❌ Service knows about HTTP

public class UserService
{
    // ❌ Service method returning HTTP result
    public IActionResult GetUser(int id)
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

### Detection

```csharp
// Architecture unit tests
[Fact]
public void Domain_ShouldNot_ReferenceInfrastructure()
{
    var domainAssembly = typeof(User).Assembly;
    var infrastructureAssembly = typeof(UserRepository).Assembly;

    var result = Types.InAssembly(domainAssembly)
        .ShouldNot()
        .HaveDependencyOn(infrastructureAssembly.GetName().Name)
        .GetResult();

    Assert.True(result.IsSuccessful, "Domain references Infrastructure!");
}

[Fact]
public void Application_ShouldNot_ReferenceAPI()
{
    var applicationAssembly = typeof(UserService).Assembly;
    var apiAssembly = typeof(Program).Assembly;

    var result = Types.InAssembly(applicationAssembly)
        .ShouldNot()
        .HaveDependencyOn(apiAssembly.GetName().Name)
        .GetResult();

    Assert.True(result.IsSuccessful);
}
```

---

## ❌ Anti-Pattern 4: Framework Mixing

**SEVERITY: HIGH**

### Description

Introducing patterns from different frameworks in same codebase (e.g., Redux patterns in Zustand project).

### Why It's Forbidden

- Cognitive overhead for developers (which pattern to follow?)
- Maintenance nightmare
- Onboarding confusion
- Inconsistent codebase
- Violates principle of least surprise

### Specific Examples

#### Example 4.1: Redux Patterns in Zustand Project

**❌ FORBIDDEN:**

```typescript
// BAD: Mixing Redux and Zustand patterns

// File: stores/userStore.ts
import { create } from 'zustand';

// ❌ Redux-style action types in Zustand project
const ADD_USER = 'users/add';
const REMOVE_USER = 'users/remove';
const UPDATE_USER = 'users/update';

// ❌ Redux-style action creators
export const addUser = (user: User) => ({
  type: ADD_USER,
  payload: user
});

// ❌ Using Zustand but with Redux patterns
export const useUserStore = create((set) => ({
  users: [],

  // ❌ Dispatch pattern in Zustand!
  dispatch: (action: any) => {
    switch (action.type) {
      case ADD_USER:
        set((state) => ({ users: [...state.users, action.payload] }));
        break;
      case REMOVE_USER:
        set((state) => ({ users: state.users.filter(u => u.id !== action.payload) }));
        break;
    }
  }
}));

// Usage in components:
const store = useUserStore();
store.dispatch(addUser(newUser));  // ❌ Redux pattern in Zustand project
```

**✅ CORRECT:**

```typescript
// CORRECT: Pure Zustand patterns

import { create } from 'zustand';

interface UserStore {
  users: User[];
  addUser: (user: User) => void;
  removeUser: (id: string) => void;
  updateUser: (id: string, updates: Partial<User>) => void;
}

export const useUserStore = create<UserStore>((set) => ({
  users: [],

  // ✓ Direct Zustand actions, no Redux patterns
  addUser: (user) => set((state) => ({
    users: [...state.users, user]
  })),

  removeUser: (id) => set((state) => ({
    users: state.users.filter(u => u.id !== id)
  })),

  updateUser: (id, updates) => set((state) => ({
    users: state.users.map(u =>
      u.id === id ? { ...u, ...updates } : u
    )
  })),
}));

// Usage in components:
const { addUser } = useUserStore();
addUser(newUser);  // ✓ Direct Zustand pattern
```

### Detection

```bash
# Linting rule detects Redux keywords in Zustand project
# .eslintrc.js
rules: {
  'no-restricted-syntax': [
    'error',
    {
      selector: 'VariableDeclarator[id.name=/ACTION_TYPE/]',
      message: 'Redux action types forbidden - use Zustand patterns'
    },
    {
      selector: 'CallExpression[callee.name="dispatch"]',
      message: 'Redux dispatch pattern forbidden - use direct Zustand actions'
    }
  ]
}
```

---

## ❌ Anti-Pattern 5: Magic Numbers/Strings

**SEVERITY: MEDIUM**

### Description

Hard-coded configuration values, magic numbers, or sensitive data in source code.

### Why It's Forbidden

- Security risk (credentials in source)
- Environment-specific values hard-coded
- Difficult to change across codebase
- Testing difficulties
- Violates DRY principle

### Examples

**❌ FORBIDDEN:**

```csharp
public class EmailService
{
    public async Task SendAsync(string to, string subject, string body)
    {
        // ❌ Hard-coded SMTP configuration
        var client = new SmtpClient("smtp.gmail.com", 587);

        // ❌ CREDENTIALS IN SOURCE CODE!!!
        client.Credentials = new NetworkCredential(
            "noreply@company.com",
            "SuperSecret123!"
        );

        // ❌ Hard-coded sender
        var message = new MailMessage("noreply@company.com", to, subject, body);

        await client.SendAsync(message);
    }
}

public class PaymentService
{
    public async Task ProcessPayment(decimal amount)
    {
        // ❌ Hard-coded API key
        var client = new HttpClient();
        client.DefaultRequestHeaders.Add("X-API-Key", "sk_live_abc123xyz");

        // ❌ Hard-coded URL
        var response = await client.PostAsync(
            "https://api.stripe.com/v1/charges",
            /* ... */
        );
    }
}
```

**✅ CORRECT:**

```csharp
public class EmailService
{
    private readonly EmailSettings _settings;

    public EmailService(IOptions<EmailSettings> settings)
    {
        _settings = settings.Value;
    }

    public async Task SendAsync(string to, string subject, string body)
    {
        // ✓ Configuration from appsettings.json
        var client = new SmtpClient(_settings.SmtpHost, _settings.SmtpPort);

        // ✓ Credentials from configuration or Azure Key Vault
        client.Credentials = new NetworkCredential(
            _settings.FromAddress,
            _settings.Password  // From secure configuration
        );

        // ✓ Sender from configuration
        var message = new MailMessage(_settings.FromAddress, to, subject, body);

        await client.SendAsync(message);
    }
}

// appsettings.json
{
  "EmailSettings": {
    "SmtpHost": "smtp.gmail.com",
    "SmtpPort": 587,
    "FromAddress": "noreply@company.com",
    "Password": ""  // Set via environment variable or Azure Key Vault
  }
}

public class PaymentService
{
    private readonly PaymentSettings _settings;

    public PaymentService(IOptions<PaymentSettings> settings)
    {
        _settings = settings.Value;
    }

    public async Task ProcessPayment(decimal amount)
    {
        var client = new HttpClient();

        // ✓ API key from secure configuration
        client.DefaultRequestHeaders.Add("X-API-Key", _settings.ApiKey);

        // ✓ URL from configuration
        var response = await client.PostAsync(
            _settings.ApiBaseUrl + "/charges",
            /* ... */
        );
    }
}
```

---

## AI Agent Pre-Implementation Checklist

**Before writing ANY code, AI agents must verify:**

- [ ] Libraries used are listed in dependencies.md
- [ ] File location matches source-tree.md
- [ ] Layer boundaries respected (check architecture-constraints.md)
- [ ] Code patterns match coding-standards.md
- [ ] No anti-patterns from this file being introduced
- [ ] No hard-coded configuration values
- [ ] No framework mixing
- [ ] Test structure mirrors source (if creating tests)
- [ ] No magic numbers or strings
- [ ] No cross-layer violations

**If ANY check fails → STOP and use AskUserQuestion for clarification**

---

## Enforcement Mechanisms

### 1. Pre-Commit Hooks

```bash
#!/bin/bash
# .git/hooks/pre-commit

# Run all validation scripts
python devforgeai/scripts/detect_anti_patterns.py || exit 1
python devforgeai/scripts/validate_dependencies.py || exit 1
python devforgeai/scripts/validate_architecture.py || exit 1
```

### 2. Architecture Tests

```csharp
// Run as part of unit test suite
[Fact]
public void AllTests_ShouldNot_ViolateAntiPatterns()
{
    ValidateNoLibrarySubstitution();
    ValidateNoFrameworkMixing();
    ValidateLayerBoundaries();
    ValidateTestStructure();
}
```

### 3. Code Review Checklist

- [ ] No unapproved dependencies added
- [ ] Files in correct locations per source-tree.md
- [ ] No layer boundary violations
- [ ] No framework mixing
- [ ] Configuration properly externalized
- [ ] Tests mirror source structure

---

## Related Documents

- [tech-stack.md](./tech-stack.md) - Approved technologies
- [dependencies.md](./dependencies.md) - Approved packages
- [source-tree.md](./source-tree.md) - Project structure
- [coding-standards.md](./coding-standards.md) - Correct patterns
- [architecture-constraints.md](./architecture-constraints.md) - Layer rules

---

## Notes

[Project-specific notes about anti-patterns, discovered violations, lessons learned]

**Example:**
```
2025-10-29: Discovered developer mixed Dapper and EF Core in UserRepository.
Created this anti-pattern documentation to prevent recurrence.
See incident report: docs/incidents/2025-10-29-orm-mixing.md
```
