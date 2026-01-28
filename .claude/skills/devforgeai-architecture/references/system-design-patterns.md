# System Design Patterns Reference

Common architecture patterns for DevForgeAI projects. Use this guide when making architectural decisions during context file creation.

---

## Layered Architecture Patterns

### Clean Architecture (Onion Architecture)

**Description:**
A layered architecture pattern that enforces dependency rules: outer layers can depend on inner layers, but never the reverse. Also known as Hexagonal Architecture or Ports and Adapters.

**Layers (inside-out):**
1. **Domain** (Core) - Business entities, value objects, domain services, domain events
2. **Application** - Use cases, application services, interfaces (ports), DTOs
3. **Infrastructure** - Data access, external services, frameworks, adapters
4. **Presentation** (API/UI) - Controllers, views, request/response models

**Dependency Rules:**
```
Domain → (Nothing)                      # Pure business logic
Application → Domain only               # Uses domain, defines interfaces
Infrastructure → Domain + Application   # Implements interfaces defined by Application
Presentation → Application only         # Never directly references Domain or Infrastructure
```

**When to Use:**
- Medium to complex applications (10+ business entities)
- Long-term maintainability is priority
- High testability requirements
- Team experienced with layered architecture
- Domain-Driven Design (DDD) approach
- Complex business rules that change frequently

**When NOT to Use:**
- Simple CRUD applications (over-engineering)
- Rapid prototyping with changing requirements
- Team unfamiliar with pattern (steep learning curve)
- Small projects (<6 months development)

**Example Project Structure:**
```
src/
├── Domain/
│   ├── Entities/           # Business entities (Order, Product, User)
│   ├── ValueObjects/       # Immutable values (Money, Address, Email)
│   ├── Interfaces/         # Domain contracts (IRepository, IDomainService)
│   ├── Services/           # Domain services (business logic)
│   ├── Events/             # Domain events (OrderCreated, PaymentProcessed)
│   └── Exceptions/         # Domain-specific exceptions
├── Application/
│   ├── UseCases/           # Application use cases (CreateOrder, ProcessPayment)
│   ├── Services/           # Application services (orchestration)
│   ├── Interfaces/         # Application contracts (IEmailService, IPaymentGateway)
│   ├── DTOs/               # Data transfer objects
│   └── Validators/         # Input validation (FluentValidation)
├── Infrastructure/
│   ├── Persistence/        # Database context, migrations
│   ├── Repositories/       # Repository implementations
│   ├── Services/           # External service integrations
│   ├── Messaging/          # Message queues, event bus
│   └── Configuration/      # DI registration, configuration
└── API/ (or Web/)
    ├── Controllers/        # API endpoints
    ├── Middleware/         # HTTP pipeline components
    ├── Filters/            # Action filters, exception filters
    └── Models/             # Request/response view models
```

**Context File Impact:**
- **tech-stack.md**: Should specify "Clean Architecture" as locked pattern
- **source-tree.md**: Should enforce 4-layer directory structure
- **architecture-constraints.md**: Should define dependency matrix (Domain → Nothing, etc.)
- **anti-patterns.md**: Should forbid layer violations (Domain → Infrastructure)

**Benefits:**
- Testability: Domain and Application layers highly testable (no infrastructure dependencies)
- Flexibility: Swap infrastructure without affecting business logic
- Maintainability: Clear separation of concerns
- Independence: Framework, database, UI can be replaced

**Trade-offs:**
- Complexity: More files, interfaces, and abstractions
- Learning Curve: Developers need to understand dependency rules
- Boilerplate: More code for simple operations
- Initial Development: Slower start compared to simple layered

---

### N-Tier Architecture (3-Tier)

**Description:**
Traditional layered architecture with presentation, business logic, and data access tiers. Each tier can run on separate physical machines.

**Tiers:**
1. **Presentation Tier** - UI, API controllers, views
2. **Business Logic Tier** - Services, business rules, validation
3. **Data Access Tier** - Repositories, database operations, ORM

**Dependency Flow:**
```
Presentation → Business Logic → Data Access → Database
```

**When to Use:**
- Standard enterprise applications
- Team familiar with traditional layering
- Clear separation of concerns needed
- Moderate complexity (not simple CRUD, not microservices)
- Physical separation of tiers desired (scaling, security zones)

**When NOT to Use:**
- Simple applications (over-engineering)
- Microservices architecture (use different pattern)
- Highly complex domain logic (use Clean Architecture instead)

**Example Project Structure:**
```
src/
├── Presentation/
│   ├── Web/                # MVC views, Razor pages
│   ├── API/                # REST API controllers
│   └── ViewModels/         # Presentation models
├── Business/
│   ├── Services/           # Business logic
│   ├── Models/             # Business entities
│   ├── Validators/         # Business rules
│   └── Interfaces/         # Service contracts
└── DataAccess/
    ├── Repositories/       # Data access layer
    ├── Context/            # ORM context (EF Core, NHibernate)
    ├── Entities/           # Database models
    └── Migrations/         # Schema migrations
```

**Benefits:**
- Familiar pattern: Most developers understand tiered architecture
- Clear separation: Each tier has distinct responsibility
- Physical separation: Tiers can be deployed separately
- Technology flexibility: Each tier can use different tech stack

**Trade-offs:**
- Tight coupling: Changes in one tier often require changes in others
- Testing challenges: Business logic may depend on data access
- Performance: Network calls between physical tiers add latency

---

### Vertical Slice Architecture

**Description:**
Organize code by feature/use case rather than technical layers. Each feature is a vertical slice through all layers, promoting high cohesion within features and low coupling between features.

**Structure:**
```
src/Features/
├── Users/
│   ├── Register/
│   │   ├── RegisterCommand.cs      # Input model
│   │   ├── RegisterHandler.cs      # Business logic
│   │   ├── RegisterValidator.cs    # Validation
│   │   ├── RegisterEndpoint.cs     # API endpoint
│   │   └── RegisterTests.cs        # Tests
│   ├── Login/
│   │   ├── LoginCommand.cs
│   │   ├── LoginHandler.cs
│   │   └── LoginEndpoint.cs
│   └── UpdateProfile/
│       ├── UpdateProfileCommand.cs
│       ├── UpdateProfileHandler.cs
│       └── UpdateProfileEndpoint.cs
├── Products/
│   ├── Create/
│   ├── Update/
│   ├── Delete/
│   └── Search/
└── Orders/
    ├── CreateOrder/
    ├── CancelOrder/
    └── GetOrderHistory/
```

**When to Use:**
- Feature-focused development
- Frequent feature additions (minimize cross-cutting changes)
- Small to medium applications
- Rapid iteration and deployment
- Microservices-style organization within a monolith
- CQRS pattern implementation

**When NOT to Use:**
- Complex shared business logic across features
- Strict layering requirements
- Team prefers traditional layered architecture

**Benefits:**
- High cohesion: Everything for a feature in one place
- Low coupling: Features are isolated, easy to add/remove
- Easy to understand: Find all code for a feature quickly
- Parallel development: Teams can work on different features independently
- Microservices ready: Easy to extract features into separate services

**Trade-offs:**
- Code duplication: Some logic may be duplicated across features
- Shared concerns: Cross-cutting features (logging, validation) need careful design
- Database schema: Shared tables may create coupling

**MediatR Pattern (Common with Vertical Slice):**
```csharp
// Command
public record RegisterCommand(string Email, string Password) : IRequest<Result<User>>;

// Handler (contains all logic for this feature)
public class RegisterHandler : IRequestHandler<RegisterCommand, Result<User>>
{
    public async Task<Result<User>> Handle(RegisterCommand request, CancellationToken ct)
    {
        // Validation, business logic, data access all in one handler
    }
}

// Endpoint
[HttpPost("/api/users/register")]
public async Task<IActionResult> Register(RegisterCommand command)
{
    var result = await _mediator.Send(command);
    return result.IsSuccess ? Ok(result.Value) : BadRequest(result.Error);
}
```

---

## Microservices Patterns

### Service Decomposition Strategies

**By Business Capability:**
Organize services around business functions, not technical concerns.

```
User Service:
- User registration
- Profile management
- Authentication
- Authorization

Order Service:
- Create orders
- Order fulfillment
- Order history
- Inventory reservation

Payment Service:
- Process payments
- Refunds
- Payment history
- PCI compliance

Notification Service:
- Send emails
- Send SMS
- Push notifications
- Template management
```

**By Subdomain (Domain-Driven Design):**
Use bounded contexts to define service boundaries. Each bounded context becomes a microservice.

```
E-Commerce Bounded Contexts:
├── Identity Context        → Identity Service
├── Catalog Context         → Catalog Service
├── Shopping Context        → Shopping Cart Service
├── Ordering Context        → Order Service
├── Payment Context         → Payment Service
├── Shipping Context        → Shipping Service
└── Notification Context    → Notification Service
```

**Decomposition Criteria:**
- **Single Responsibility**: Each service does one thing well
- **Independent Deployability**: Services can be deployed without affecting others
- **Team Autonomy**: One team owns one service
- **Data Ownership**: Each service owns its data (no shared databases)
- **Bounded Context**: Clear domain boundaries

**When to Use Microservices:**
- Large, complex applications
- Multiple teams (>15 developers)
- Different scaling requirements per service
- Polyglot technology needs (different services, different tech stacks)
- Continuous deployment culture
- Cloud-native architecture

**When NOT to Use:**
- Small applications or teams (<10 developers)
- Unclear domain boundaries
- Network latency critical (local calls are faster)
- Team lacks DevOps maturity
- Operational complexity not acceptable

---

### API Gateway Pattern

**Purpose:** Single entry point for clients, routes requests to appropriate microservices

**Responsibilities:**
- **Request Routing**: Route /api/users/* to User Service, /api/orders/* to Order Service
- **Authentication/Authorization**: Validate JWT tokens, enforce access control
- **Rate Limiting**: Prevent abuse, implement quotas
- **Request Aggregation**: Combine multiple backend calls into single response
- **Protocol Translation**: HTTP → gRPC, REST → GraphQL
- **Caching**: Cache responses for performance
- **Load Balancing**: Distribute requests across service instances

**Technologies:**
- **Kong**: Open-source, plugin-based, high performance
- **Nginx**: Lightweight, fast, custom configurations
- **Ocelot**: .NET-specific, easy integration with ASP.NET Core
- **AWS API Gateway**: Managed service, serverless integration
- **Azure API Management**: Enterprise features, policy-based
- **Traefik**: Cloud-native, dynamic configuration

**Example Architecture:**
```
Client (Web/Mobile)
    ↓
API Gateway (Kong/Ocelot)
    ↓
┌─────────┬─────────┬─────────┬─────────┐
│  User   │ Product │  Order  │ Payment │
│ Service │ Service │ Service │ Service │
└─────────┴─────────┴─────────┴─────────┘
```

**Benefits:**
- Single endpoint for clients (simplified client code)
- Cross-cutting concerns centralized (auth, rate limiting)
- Backend flexibility (change services without changing client)
- Reduced chattiness (aggregate multiple calls)

**Trade-offs:**
- Single point of failure (mitigate with redundancy)
- Potential bottleneck (mitigate with caching, load balancing)
- Added complexity (another component to manage)

---

### Event-Driven Architecture

**Pattern:** Services communicate via events (asynchronous, loosely coupled)

**Components:**
- **Event Producers**: Services that publish events (Order Service publishes OrderCreated)
- **Event Bus/Broker**: Message broker (RabbitMQ, Kafka, Azure Service Bus, AWS SNS/SQS)
- **Event Consumers**: Services that subscribe to events (Notification Service subscribes to OrderCreated)

**Example Flow:**
```
1. Order Service creates order
   ↓
2. Order Service publishes OrderCreated event to message broker
   ↓
3. Multiple consumers react:
   - Inventory Service reserves stock
   - Notification Service sends confirmation email
   - Analytics Service records order for reporting
   - Shipping Service prepares shipment
```

**Event Types:**
- **Domain Events**: Something happened in the domain (OrderCreated, PaymentProcessed)
- **Integration Events**: Events published to other services/systems
- **Notification Events**: Inform other parts of the system

**When to Use:**
- Loose coupling required (services don't need to know about each other)
- Asynchronous processing acceptable (email, analytics can be eventual)
- High scalability needed (consumers can be scaled independently)
- Complex workflows across services (saga pattern)
- Audit trails needed (event sourcing)

**Technologies:**
- **RabbitMQ**: Flexible routing, supports multiple protocols (AMQP, MQTT, STOMP)
- **Apache Kafka**: High throughput, distributed log, event streaming
- **Azure Service Bus**: Managed service, .NET integration, topics and queues
- **AWS SNS/SQS**: Simple, serverless-friendly, pay-per-use
- **Redis Streams**: Lightweight, fast, built into Redis

**Benefits:**
- Decoupling: Producers don't know about consumers
- Scalability: Consumers scale independently
- Resilience: If consumer down, events queue for later processing
- Flexibility: Add new consumers without changing producers

**Trade-offs:**
- Eventual consistency: Changes not immediately visible across services
- Complexity: Distributed systems are harder to debug
- Ordering: Events may arrive out of order (need idempotency)
- Monitoring: Harder to trace requests across async boundaries

---

## Data Access Patterns

### Repository Pattern

**Purpose:** Abstraction layer between domain and data access, provides collection-like interface for domain entities

**Structure:**
```csharp
// Domain layer defines interface
public interface IUserRepository
{
    Task<User> GetByIdAsync(int id);
    Task<User> GetByEmailAsync(string email);
    Task<IEnumerable<User>> GetAllAsync();
    Task AddAsync(User user);
    Task UpdateAsync(User user);
    Task DeleteAsync(int id);
}

// Infrastructure layer implements interface
public class UserRepository : IUserRepository
{
    private readonly IDbConnection _connection; // Dapper
    // OR
    private readonly AppDbContext _context; // Entity Framework Core

    public async Task<User> GetByIdAsync(int id)
    {
        // Implementation using Dapper, EF Core, or any data access technology
        return await _connection.QuerySingleOrDefaultAsync<User>(
            "SELECT * FROM Users WHERE Id = @Id", new { Id = id });
    }

    // Other methods...
}
```

**Benefits:**
- **Testability**: Mock repositories in unit tests (no database needed)
- **Abstraction**: Domain doesn't know about data access technology
- **Centralized Logic**: Common queries in one place
- **Flexibility**: Swap data access technology without changing domain

**When to Use:**
- Domain-Driven Design
- Multiple data sources (SQL, NoSQL, APIs)
- Complex queries need abstraction
- High testability requirements

**When NOT to Use:**
- Simple CRUD applications (over-engineering)
- Direct database access acceptable
- Performance critical (repository adds abstraction overhead)

**Trade-offs:**
- **Abstraction**: Extra layer adds complexity
- **Generic Repositories**: Tempting but often becomes code smell (leaky abstraction)
- **N+1 Queries**: Need careful design to avoid performance issues

---

### Unit of Work Pattern

**Purpose:** Coordinate multiple repository operations in a single transaction

**Pattern:**
```csharp
public interface IUnitOfWork : IDisposable
{
    IUserRepository Users { get; }
    IOrderRepository Orders { get; }
    IProductRepository Products { get; }

    Task<int> CommitAsync(); // Save all changes in single transaction
    void Rollback();
}

// Usage
public class OrderService
{
    private readonly IUnitOfWork _unitOfWork;

    public async Task ProcessOrderAsync(CreateOrderCommand command)
    {
        var user = await _unitOfWork.Users.GetByIdAsync(command.UserId);
        var product = await _unitOfWork.Products.GetByIdAsync(command.ProductId);

        var order = new Order(user.Id, product.Id, command.Quantity);
        await _unitOfWork.Orders.AddAsync(order);

        product.ReduceStock(command.Quantity);
        await _unitOfWork.Products.UpdateAsync(product);

        await _unitOfWork.CommitAsync(); // Both order and product updated in single transaction
    }
}
```

**When to Use:**
- Multiple entities changed in one operation
- Transaction coordination needed (all-or-nothing)
- Working with Entity Framework Core or NHibernate (built-in Unit of Work via DbContext)

**When NOT to Use:**
- Single entity operations (repository is sufficient)
- Using Dapper (doesn't have change tracking - manual transaction handling better)
- Microservices (distributed transactions are complex - use saga pattern instead)

---

### Active Record vs Data Mapper

**Active Record:**
Domain objects have data access methods (tight coupling).

```csharp
public class User
{
    public int Id { get; set; }
    public string Name { get; set; }

    public void Save()
    {
        // SQL to save user
    }

    public static User FindById(int id)
    {
        // SQL to load user
    }
}

// Usage
var user = new User { Name = "John" };
user.Save(); // Domain object knows how to persist itself
```

**Pros:** Simple, less code
**Cons:** Tight coupling, hard to test, violates Single Responsibility

**Used by:** Ruby on Rails, Laravel (Active Record pattern)

**Data Mapper:**
Separate domain objects from data access (loose coupling).

```csharp
// Domain object (pure business logic)
public class User
{
    public int Id { get; set; }
    public string Name { get; set; }

    // No data access methods
}

// Repository handles persistence
public class UserRepository
{
    public void Save(User user)
    {
        // SQL to save user
    }

    public User FindById(int id)
    {
        // SQL to load user
    }
}

// Usage
var user = new User { Name = "John" };
_userRepository.Save(user); // Repository handles persistence
```

**Pros:** Loose coupling, testable, clean domain
**Cons:** More code, more abstraction

**Used by:** Clean Architecture, DDD, most modern .NET applications

---

## API Design Patterns

### RESTful API Design

**Principles:**
- **Resources as Nouns**: `/users`, `/products`, `/orders` (not verbs like `/getUsers`)
- **HTTP Methods**: GET (read), POST (create), PUT (update), DELETE (delete), PATCH (partial update)
- **Status Codes**: 200 OK, 201 Created, 204 No Content, 400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found, 500 Internal Server Error
- **Stateless**: Each request contains all information needed (no server session)
- **HATEOAS**: Hypermedia as the Engine of Application State (links in responses)

**Example:**
```
GET    /api/users              # List all users (200 OK)
GET    /api/users/{id}         # Get single user (200 OK or 404 Not Found)
POST   /api/users              # Create user (201 Created, Location header)
PUT    /api/users/{id}         # Full update (200 OK or 204 No Content)
PATCH  /api/users/{id}         # Partial update (200 OK)
DELETE /api/users/{id}         # Delete user (204 No Content)

GET    /api/users/{id}/orders  # Get user's orders (nested resource)
POST   /api/users/{id}/orders  # Create order for user
```

**Best Practices:**
- Use plural nouns for collections (`/users` not `/user`)
- Use query parameters for filtering (`/api/users?status=active&role=admin`)
- Use pagination for large collections (`/api/users?page=1&pageSize=20`)
- Version your API (`/api/v1/users` or `Accept: application/vnd.api.v1+json`)
- Use DTOs to prevent over-posting (don't expose domain entities directly)

---

### CQRS (Command Query Responsibility Segregation)

**Pattern:** Separate read and write operations into different models

**Structure:**
```csharp
// Commands (Write Model) - Modify state
public class CreateUserCommand : IRequest<int>
{
    public string Email { get; set; }
    public string Password { get; set; }
}

public class CreateUserHandler : IRequestHandler<CreateUserCommand, int>
{
    public async Task<int> Handle(CreateUserCommand command, CancellationToken ct)
    {
        // Validate, create user, save to write database
        // Optimized for writes (normalized schema)
    }
}

// Queries (Read Model) - Read state
public class GetUserQuery : IRequest<UserDto>
{
    public int UserId { get; set; }
}

public class GetUserHandler : IRequestHandler<GetUserQuery, UserDto>
{
    public async Task<UserDto> Handle(GetUserQuery query, CancellationToken ct)
    {
        // Read from read database (could be denormalized for performance)
    }
}
```

**Benefits:**
- **Optimized Models**: Write model normalized, read model denormalized for performance
- **Scalability**: Scale reads and writes independently
- **Complexity Separation**: Complex business logic on writes, simple queries on reads
- **Multiple Read Models**: Different read models for different use cases (reporting, search, API)

**When to Use:**
- Different read and write requirements (complex writes, simple reads)
- High read/write ratio imbalance (95% reads, 5% writes)
- Complex business logic on writes (domain events, validation, workflows)
- Multiple clients with different data needs

**Trade-offs:**
- **Complexity**: More code, more models
- **Eventual Consistency**: Read model updated asynchronously (may lag behind write model)
- **Duplication**: Some logic may be duplicated across command and query handlers

**Technologies:**
- **MediatR**: Implements mediator pattern for CQRS in .NET
- **Event Sourcing**: Often combined with CQRS (write model stores events, read model built from events)

---

## Caching Strategies

### Cache-Aside (Lazy Loading)

**Pattern:**
1. Check cache for data
2. If **cache hit**: Return cached value
3. If **cache miss**: Load from database, populate cache, return value

```csharp
public async Task<User> GetUserAsync(int userId)
{
    var cacheKey = $"user:{userId}";

    // Check cache
    var cachedUser = await _cache.GetAsync<User>(cacheKey);
    if (cachedUser != null)
        return cachedUser; // Cache hit

    // Cache miss - load from database
    var user = await _userRepository.GetByIdAsync(userId);

    // Populate cache
    await _cache.SetAsync(cacheKey, user, TimeSpan.FromMinutes(10));

    return user;
}
```

**When to Use:** Read-heavy workloads, data doesn't change often
**Technologies:** Redis, Memcached, IMemoryCache (.NET)

---

### Write-Through

**Pattern:**
1. Write to cache **and** database simultaneously
2. Cache always in sync with database

```csharp
public async Task UpdateUserAsync(User user)
{
    var cacheKey = $"user:{user.Id}";

    // Update database
    await _userRepository.UpdateAsync(user);

    // Update cache (write-through)
    await _cache.SetAsync(cacheKey, user, TimeSpan.FromMinutes(10));
}
```

**When to Use:** Consistency critical, cache must always be fresh

---

### Write-Behind (Write-Back)

**Pattern:**
1. Write to cache immediately
2. Asynchronously write to database later (batched)

**When to Use:** Write performance critical, eventual consistency acceptable

**Trade-offs:** Risk of data loss if cache crashes before database write

---

## Authentication Patterns

### JWT (JSON Web Tokens)

**Pattern:** Stateless authentication with signed tokens

**Flow:**
1. User logs in with credentials (POST /api/auth/login)
2. Server validates credentials, generates JWT containing user claims
3. Client stores JWT (localStorage, sessionStorage, cookie)
4. Client includes JWT in subsequent requests (Authorization: Bearer {token})
5. Server validates JWT signature and expiration
6. Server extracts user claims from JWT (no database lookup needed)

**JWT Structure:**
```
Header.Payload.Signature

Header:   { "alg": "HS256", "typ": "JWT" }
Payload:  { "userId": 123, "email": "user@example.com", "roles": ["admin"], "exp": 1699999999 }
Signature: HMAC-SHA256(base64(header) + "." + base64(payload), secret)
```

**When to Use:**
- Stateless API (no server sessions, horizontal scaling)
- Microservices (token valid across services)
- Mobile/SPA clients (token stored client-side)
- Third-party API access (OAuth 2.0 uses JWT)

**Benefits:**
- Stateless: No server-side session storage
- Scalable: No session database needed
- Cross-domain: Works across different domains

**Trade-offs:**
- Revocation: Cannot invalidate token before expiration (use short expiration + refresh tokens)
- Size: Larger than session ID (included in every request)
- Security: Must use HTTPS, protect secret key

---

### Session-Based Authentication

**Pattern:** Server maintains session state

**Flow:**
1. User logs in with credentials
2. Server creates session, stores session ID in cookie (Set-Cookie: sessionId=abc123)
3. Client automatically sends session cookie with requests
4. Server validates session ID, retrieves user from session store

**When to Use:**
- Traditional web applications (server-side rendering)
- Simple authentication needs
- Session storage available (Redis, database)

**Benefits:**
- Simple implementation
- Easy revocation (delete session)
- Server controls session lifetime

**Trade-offs:**
- Stateful: Session storage required (doesn't scale horizontally without shared session store)
- Same-domain: Cookies don't work across domains easily

---

### OAuth 2.0 / OpenID Connect

**Pattern:** Delegated authentication (third-party identity providers)

**When to Use:**
- "Login with Google/Facebook/GitHub"
- API authorization scopes (read:user, write:repo)
- Enterprise SSO (Single Sign-On)
- Avoid password management (delegate to identity provider)

**Flows:**
- **Authorization Code Flow**: Web applications (most secure)
- **Client Credentials Flow**: Service-to-service authentication
- **Implicit Flow**: SPAs (deprecated, use Authorization Code + PKCE)

---

## Database Patterns

### Database per Service (Microservices)

**Pattern:** Each microservice has its own dedicated database

**Architecture:**
```
User Service     → User Database (PostgreSQL)
Order Service    → Order Database (PostgreSQL)
Product Service  → Product Database (MongoDB)
Analytics Service → Analytics Database (ClickHouse)
```

**Benefits:**
- **Service Independence**: Services deploy and scale independently
- **Technology Flexibility**: Polyglot persistence (each service chooses optimal database)
- **Isolated Failures**: Database failure affects only one service
- **Clear Ownership**: One team owns one service and its data

**Challenges:**
- **Distributed Transactions**: No ACID across services (use saga pattern)
- **Data Consistency**: Eventual consistency instead of immediate
- **Joins Across Services**: Must query multiple services and join in application code
- **Data Duplication**: Some data may be replicated across services

**Patterns to Address Challenges:**
- **Saga Pattern**: Coordinate distributed transactions with compensating actions
- **Event Sourcing**: Publish domain events, other services build read models
- **API Composition**: Query multiple services and combine results
- **CQRS**: Separate write model (normalized) from read model (denormalized with duplicated data)

---

### Shared Database (Monolith/N-Tier)

**Pattern:** All services/modules share a single database

**Architecture:**
```
User Module    ↘
Order Module   → Single Shared Database (PostgreSQL)
Product Module ↗
```

**Benefits:**
- **ACID Transactions**: Full transaction support across tables
- **Simple Joins**: Query data across tables easily
- **Consistency**: Immediate consistency (no eventual consistency issues)
- **Simpler Operations**: One database to manage

**Challenges:**
- **Coupling**: Schema changes affect all modules
- **Scalability Bottleneck**: Database becomes single point of contention
- **Deployment Coupling**: Schema migrations require coordination
- **Team Conflicts**: Multiple teams changing same database

**When to Use:**
- Monolithic applications
- N-tier architecture
- Strong consistency requirements
- Single team or small team

---

## Scalability Patterns

### Horizontal Scaling (Scale Out)

**Pattern:** Add more instances/servers to distribute load

**Example:**
```
Load Balancer
    ↓
┌────────────┬────────────┬────────────┐
│ Instance 1 │ Instance 2 │ Instance 3 │
└────────────┴────────────┴────────────┘
```

**When to Use:**
- Stateless applications (no server session state)
- Cloud-native applications (easy to add instances)
- Unpredictable traffic patterns (auto-scaling)

**Technologies:**
- **Kubernetes**: Container orchestration, auto-scaling
- **AWS Auto Scaling**: EC2, ECS, Lambda
- **Azure App Service**: Built-in auto-scaling

**Benefits:**
- **No Limits**: Can scale infinitely by adding instances
- **High Availability**: Multiple instances provide redundancy
- **Cost-Effective**: Add small instances as needed

**Requirements:**
- **Stateless Design**: No in-memory sessions (use distributed cache)
- **Shared Data Store**: Database, cache must be external
- **Load Balancer**: Distribute traffic across instances

---

### Vertical Scaling (Scale Up)

**Pattern:** Increase resources of existing server (CPU, RAM, disk)

**When to Use:**
- Database servers (vertical scaling often effective)
- Legacy applications (can't easily scale horizontally)
- Short-term capacity increase
- Stateful applications (difficult to distribute)

**Limits:**
- Hardware limits (can't scale infinitely)
- Downtime required (must restart to add resources)
- Expensive (larger instances cost more)

---

### Load Balancing Strategies

**Round Robin:**
- Distribute requests sequentially (Instance 1 → Instance 2 → Instance 3 → Instance 1...)
- Simple, fair distribution
- Doesn't account for instance load

**Least Connections:**
- Route to instance with fewest active connections
- Better for long-lived connections
- Accounts for varying request complexity

**IP Hash:**
- Route based on client IP hash
- Same client always goes to same instance
- Useful for sticky sessions (but prefer stateless)

**Weighted Round Robin:**
- Instances have weights based on capacity
- Powerful instance gets more traffic
- Useful for heterogeneous instances

**Technologies:**
- Nginx, HAProxy, AWS ALB, Azure Load Balancer, Google Cloud Load Balancer

---

## Reference for Pattern Selection

When making architecture decisions during context file creation:

### Step 1: Read Existing Context (Brownfield)

```
Read(file_path="devforgeai/specs/context/architecture-constraints.md")
```

Check if architecture pattern already defined.

### Step 2: Use AskUserQuestion for Pattern Selection

```
Question: "Which architecture pattern should be used for this project?"
Header: "Architecture"
Description: "This will be documented in architecture-constraints.md as the LOCKED pattern"
Options:
  - "Clean Architecture (Domain-centric, highly testable, complex)"
  - "N-Tier (Traditional layering, moderate complexity)"
  - "Vertical Slice (Feature-focused, high cohesion)"
  - "Microservices (Distributed, highly scalable, very complex)"
multiSelect: false
```

### Step 3: Document Decision in ADR

**Load ADR template for structure:**
```
Read(file_path=".claude/skills/devforgeai-architecture/references/adr-template.md")
```

**Review relevant ADR examples for guidance:**

For architecture pattern decisions:
```
Read(file_path=".claude/skills/devforgeai-architecture/assets/adr-examples/ADR-EXAMPLE-004-clean-architecture.md")
```

For database decisions:
```
Read(file_path=".claude/skills/devforgeai-architecture/assets/adr-examples/ADR-EXAMPLE-001-database-selection.md")
```

**Document your decision:**
- Pattern choice and rationale (why this pattern for this project)
- Trade-offs (benefits and drawbacks)
- Alternatives considered (and why rejected)
- Implementation guidance
- Review schedule

### Step 4: Update Context Files

**architecture-constraints.md:**
- Define layer dependency rules for chosen pattern
- Document mandatory patterns (Repository, Service, DTO)
- Document forbidden patterns (layer violations)

**source-tree.md:**
- Define directory structure for pattern
- Document file placement rules
- Provide examples

**Example for Clean Architecture:**
```markdown
# architecture-constraints.md

## Chosen Pattern: Clean Architecture

### Layer Dependency Matrix

| From ↓ To → | API | Application | Domain | Infrastructure |
|-------------|-----|-------------|--------|----------------|
| API         | ✓   | ✓           | ❌     | ❌             |
| Application | ❌  | ✓           | ✓      | ❌             |
| Domain      | ❌  | ❌          | ✓      | ❌             |
| Infrastructure | ❌ | ❌        | ✓      | ✓              |

### Mandatory Patterns
- Repository Pattern (all data access through repositories)
- Service Pattern (business logic in Application services)
- DTO Pattern (API never exposes domain entities)
- Dependency Inversion (Application defines interfaces, Infrastructure implements)

### Forbidden Patterns
- Domain → Infrastructure dependency (Domain must be pure)
- API → Domain dependency (API uses Application layer only)
- Direct Entity Exposure (API controllers must use DTOs)
```

---

**This reference should be consulted during Phase 2 (Create Immutable Context Files) when defining architecture-constraints.md for projects.**
