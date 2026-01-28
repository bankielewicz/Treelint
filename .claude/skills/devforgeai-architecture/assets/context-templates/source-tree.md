---
last_updated: [DATE]
status: ENFORCED
compliance: MANDATORY
project: [PROJECT_NAME]
architecture_pattern: [CLEAN_ARCHITECTURE | N_TIER | VERTICAL_SLICE | OTHER]
---

# Source Tree Organization

**CRITICAL:** AI agents MUST adhere to this structure.

**Creating files outside this structure creates technical debt.**

---

## Project Structure

[CUSTOMIZE BASED ON CHOSEN ARCHITECTURE PATTERN - Examples below]

### Option 1: Clean Architecture (.NET Backend + React Frontend)

```
[project-root]/
├── src/
│   ├── API/                          # ASP.NET Core API
│   │   ├── Endpoints/                # Minimal API endpoints (preferred)
│   │   │   ├── Users/                # Feature: Users
│   │   │   │   ├── GetUser.cs
│   │   │   │   ├── CreateUser.cs
│   │   │   │   └── UpdateUser.cs
│   │   │   └── Orders/               # Feature: Orders
│   │   ├── Middleware/               # Custom middleware
│   │   ├── Filters/                  # Exception filters, action filters
│   │   └── Program.cs                # Application entry point
│   │
│   ├── Application/                  # Application layer (business logic)
│   │   ├── Users/                    # Feature: Users
│   │   │   ├── UserService.cs        # Business service
│   │   │   ├── UserDto.cs            # Data transfer object
│   │   │   ├── UserValidator.cs      # FluentValidation validator
│   │   │   └── IUserService.cs       # Service interface
│   │   ├── Orders/                   # Feature: Orders
│   │   └── Common/                   # Shared application logic
│   │       ├── Interfaces/
│   │       ├── Exceptions/           # Custom exceptions
│   │       └── Behaviors/            # Cross-cutting concerns
│   │
│   ├── Domain/                       # Domain layer (entities, value objects)
│   │   ├── Entities/                 # Domain entities
│   │   │   ├── User.cs
│   │   │   ├── Order.cs
│   │   │   └── Product.cs
│   │   ├── ValueObjects/             # Value objects
│   │   ├── Enums/                    # Domain enums
│   │   └── Interfaces/               # Repository interfaces
│   │       ├── IUserRepository.cs
│   │       └── IOrderRepository.cs
│   │
│   ├── Infrastructure/               # Infrastructure layer (data access)
│   │   ├── Data/                     # Database configuration
│   │   │   └── SqlConnectionFactory.cs
│   │   ├── Repositories/             # Repository implementations
│   │   │   ├── UserRepository.cs     # Dapper implementation
│   │   │   └── OrderRepository.cs
│   │   ├── Migrations/               # DbUp migration scripts
│   │   │   ├── Script0001_CreateUsersTable.sql
│   │   │   └── Script0002_CreateOrdersTable.sql
│   │   └── Configuration/            # Infrastructure configuration
│   │
│   └── WebUI/                        # React frontend
│       ├── src/
│       │   ├── components/           # React components
│       │   │   ├── common/           # Reusable components
│       │   │   │   ├── Button.tsx
│       │   │   │   └── Input.tsx
│       │   │   ├── users/            # Feature: Users components
│       │   │   │   ├── UserList.tsx
│       │   │   │   └── UserProfile.tsx
│       │   │   └── layout/           # Layout components
│       │   │       ├── Header.tsx
│       │   │       └── Footer.tsx
│       │   ├── hooks/                # Custom React hooks
│       │   │   ├── useAuth.ts
│       │   │   └── useUsers.ts
│       │   ├── stores/               # Zustand stores
│       │   │   ├── userStore.ts
│       │   │   └── authStore.ts
│       │   ├── services/             # API client services
│       │   │   ├── api.ts            # Base API client
│       │   │   ├── userService.ts
│       │   │   └── orderService.ts
│       │   ├── utils/                # Utility functions
│       │   │   ├── formatDate.ts
│       │   │   └── validators.ts
│       │   ├── types/                # TypeScript types/interfaces
│       │   │   ├── user.ts
│       │   │   └── order.ts
│       │   └── App.tsx               # Root component
│       └── public/                   # Static assets
│
├── tests/
│   ├── UnitTests/                    # Unit tests
│   │   ├── Application.UnitTests/    # Application layer tests
│   │   │   └── Users/
│   │   │       └── UserServiceTests.cs
│   │   ├── Domain.UnitTests/         # Domain layer tests
│   │   └── API.UnitTests/            # API layer tests
│   │
│   ├── IntegrationTests/             # Integration tests
│   │   ├── API.IntegrationTests/     # API integration tests
│   │   │   └── Endpoints/
│   │   │       └── Users/
│   │   │           └── GetUserTests.cs
│   │   └── Infrastructure.IntegrationTests/  # Data access tests
│   │       └── Repositories/
│   │           └── UserRepositoryTests.cs
│   │
│   └── E2ETests/                     # End-to-end tests (Playwright)
│       ├── specs/                    # Test specifications
│       │   └── users.spec.ts
│       ├── fixtures/                 # Test fixtures
│       └── pages/                    # Page object models
│           └── UserPage.ts
│
├── docs/
│   ├── architecture/                 # Architecture documentation
│   │   ├── decisions/                # ADRs
│   │   │   ├── ADR-001-database-choice.md
│   │   │   └── ADR-002-orm-selection.md
│   │   └── diagrams/                 # Architecture diagrams
│   ├── api/                          # API documentation
│   └── development/                  # Development guides
│
└── scripts/                          # Build/deployment scripts
    ├── migrations/                   # Database migration helpers
    └── testing/                      # Test automation scripts
```

---

## Naming Conventions

### Backend (C#)

**Namespaces:** `{Company}.{Project}.{Layer}.{Feature}`
- Example: `Acme.Ecommerce.Application.Users`
- Example: `Acme.Ecommerce.Infrastructure.Repositories`

**Classes:** PascalCase
- Services: `UserService`, `OrderService`, `PaymentService`
- Repositories: `UserRepository`, `OrderRepository`
- Entities: `User`, `Order`, `Product`
- DTOs: `UserDto`, `CreateUserDto`, `UpdateUserDto`

**Interfaces:** `I` prefix + PascalCase
- Example: `IUserService`, `IOrderRepository`, `IEmailService`

**Private fields:** `_camelCase` (underscore prefix)
- Example: `private readonly IUserRepository _repository;`

**Methods:** PascalCase (verb-first)
- Example: `GetUserById`, `CreateOrder`, `UpdateProduct`, `DeleteUser`

**Files:** Match class name exactly
- `UserService.cs` contains `class UserService`
- `IUserRepository.cs` contains `interface IUserRepository`

### Frontend (React/TypeScript)

**Components:** PascalCase
- Example: `UserProfile.tsx`, `OrderList.tsx`, `Button.tsx`
- File name matches component name exactly

**Hooks:** `use` prefix + camelCase
- Example: `useAuth.ts`, `useUsers.ts`, `useDebounce.ts`

**Stores:** camelCase + `Store` suffix
- Example: `userStore.ts`, `authStore.ts`, `cartStore.ts`

**Utils:** camelCase
- Example: `formatCurrency.ts`, `validateEmail.ts`, `parseDate.ts`

**Types/Interfaces:** PascalCase
- Example: `User`, `Order`, `UserProfile`
- Prefer `type` over `interface` unless extending

**Directories:** lowercase or camelCase
- Example: `components/`, `hooks/`, `services/`

---

## File Placement Rules

### Rule 1: Feature Cohesion

**Related files stay together by feature, not by file type.**

**✅ CORRECT:**
```
Application/
├── Users/
│   ├── UserService.cs
│   ├── UserDto.cs
│   ├── CreateUserDto.cs
│   ├── UserValidator.cs
│   └── IUserService.cs
└── Orders/
    ├── OrderService.cs
    ├── OrderDto.cs
    └── IOrderService.cs
```

**❌ INCORRECT:**
```
Application/
├── Services/
│   ├── UserService.cs
│   └── OrderService.cs
├── DTOs/
│   ├── UserDto.cs
│   └── OrderDto.cs
└── Validators/
    └── UserValidator.cs
```

**Exception:** Truly shared/common code goes in `Common/` directory.

### Rule 2: Layer Boundaries (ENFORCED)

**Dependency flow (allowed):**
```
API → Application → Domain ← Infrastructure
```

**ALLOWED dependencies:**
- API can reference Application
- Application can reference Domain
- Infrastructure can reference Domain
- Tests can reference any layer

**FORBIDDEN dependencies:**
- ❌ Domain CANNOT reference Infrastructure (domain is pure business logic)
- ❌ Domain CANNOT reference Application
- ❌ Application CANNOT reference API (services shouldn't know about HTTP)
- ❌ Infrastructure CANNOT reference API

**Validation:**
Before creating any file, verify layer dependency is allowed.

### Rule 3: Test Mirroring (MANDATORY)

**Test structure MUST mirror source structure exactly.**

**Source:**
```
src/Application/Users/UserService.cs
```

**Test:**
```
tests/UnitTests/Application.UnitTests/Users/UserServiceTests.cs
```

**Pattern:**
- Source: `src/{Layer}/{Feature}/{ClassName}.cs`
- Test: `tests/UnitTests/{Layer}.UnitTests/{Feature}/{ClassName}Tests.cs`

### Rule 4: One Class Per File

**Each file contains exactly ONE public class/interface.**

**✅ CORRECT:**
```
UserService.cs → class UserService
IUserService.cs → interface IUserService
UserDto.cs → class UserDto
```

**❌ INCORRECT:**
```
UserService.cs → class UserService, interface IUserService, class UserDto
```

**Exception:** Nested private classes are allowed within same file.

### Rule 5: Frontend Component Organization

**Components organized by feature and reusability.**

**✅ CORRECT:**
```
components/
├── common/              # Reusable across features
│   ├── Button.tsx
│   └── Input.tsx
├── users/               # User feature components
│   ├── UserList.tsx
│   └── UserProfile.tsx
└── orders/              # Order feature components
    └── OrderList.tsx
```

**❌ INCORRECT:**
```
components/
├── buttons/             # Don't organize by element type
│   └── UserButton.tsx
└── lists/
    └── UserList.tsx
```

---

## Ambiguity Resolution

**Before creating ANY file, AI agent must:**

1. **Check source-tree.md** - Does the file type/feature have a defined location?
2. **If location documented** → Place file in specified location
3. **If location ambiguous** → Use AskUserQuestion:

```
Question: "I need to create [FileName.cs] for [purpose].
          source-tree.md doesn't specify where this should go. Where should I place it?"
Header: "File location"
Options:
  - "src/Application/[Feature]/ (business logic)"
  - "src/Infrastructure/[Feature]/ (external concerns)"
  - "src/Domain/[Feature]/ (core domain)"
  - "Other (specify)"
Description: "Will be documented in source-tree.md for future reference"
multiSelect: false
```

4. **Update source-tree.md** after user confirms placement

---

## Enforcement Checklist

**AI agent MUST verify before creating ANY file:**

- [ ] File location matches source-tree.md structure
- [ ] Naming convention follows documented standards
- [ ] Layer boundaries respected (no forbidden dependencies)
- [ ] Feature cohesion maintained (related files together)
- [ ] Test file mirrors source file location (if creating test)
- [ ] One class per file (public classes only)

**If ANY check fails → Use AskUserQuestion for clarification**

---

## Common Structure Violations to Prevent

### ❌ Violation 1: Creating Files in Wrong Location

**Bad AI behavior:**
```
AI creates: src/Services/EmailService.cs
```

**Problem:** No `Services/` directory in structure. Should be in `Application/` or `Infrastructure/`.

**Correct AI behavior:**
```
AI checks source-tree.md:
- Email is external service → Infrastructure layer

AI uses AskUserQuestion:
  Question: "Should EmailService go in Application or Infrastructure layer?"
  Options:
    - "Application (business logic for emails)"
    - "Infrastructure (external email service integration)"

User selects: Infrastructure

AI creates: src/Infrastructure/Email/EmailService.cs
AI updates: source-tree.md to document Infrastructure/Email/ pattern
```

### ❌ Violation 2: Wrong Test Location

**Bad AI behavior:**
```
Source: src/Application/Users/UserService.cs
Test created: tests/UserServiceTests.cs
```

**Problem:** Test doesn't mirror source structure.

**Correct:**
```
Source: src/Application/Users/UserService.cs
Test: tests/UnitTests/Application.UnitTests/Users/UserServiceTests.cs
```

### ❌ Violation 3: Cross-Layer Dependency

**Bad AI behavior:**
```
// In Domain/Entities/User.cs
using Infrastructure.Repositories;  // ❌ Domain referencing Infrastructure

public class User
{
    public void Save()
    {
        var repo = new UserRepository();  // ❌ Violates layer boundary
        repo.SaveAsync(this);
    }
}
```

**Correct:**
```
// In Domain/Entities/User.cs
// Domain entities are pure POCOs - no infrastructure dependencies

public class User
{
    public int Id { get; set; }
    public string Email { get; set; }
    // Just properties, no persistence logic
}

// In Application/Users/UserService.cs
public class UserService : IUserService
{
    private readonly IUserRepository _repository;  // ✓ Service uses repository

    public async Task SaveUserAsync(User user)
    {
        await _repository.SaveAsync(user);  // ✓ Correct layer
    }
}
```

---

## Project-Specific Variations

[Document any project-specific structure variations here]

**Example:**
```
Note: This project uses feature-based organization within Application layer:
- Application/Users/ contains all user-related logic
- Application/Orders/ contains all order-related logic

This differs from traditional layer-per-type organization but improves feature cohesion.
```

---

## Related Documents

- [tech-stack.md](./../tech-stack.md) - Technology decisions
- [architecture-constraints.md](./../architecture-constraints.md) - Layer rules and patterns
- [coding-standards.md](./../coding-standards.md) - Code organization patterns

---

## Change Log

| Date | Change | Reason |
|------|--------|--------|
| [DATE] | Initial structure | Project setup |
| | | |

---

## Notes

[Additional context about structure decisions]

**Example:**
> We chose Clean Architecture over N-Tier to achieve better separation of concerns and testability. The Domain layer contains zero dependencies, making it easy to test business logic in isolation.
