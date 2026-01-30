# ADR-EXAMPLE-002: Dapper for Data Access Layer

**Date**: 2025-10-20
**Status**: Accepted
**Deciders**: Technical Architect, Backend Lead, Database Administrator
**Project**: E-Commerce Order Management System

---

## Context

### Project Background
Building a high-performance order management system for an e-commerce platform handling:
- **Order Volume**: 10,000+ orders/day (peak: 500 orders/minute)
- **Read-Heavy Workload**: 80% reads, 20% writes
- **Response Time Requirement**: <100ms for order queries
- **Complex Queries**: Multi-table joins, aggregations, reporting
- **Database**: PostgreSQL 15.x (already selected via ADR-001)
- **Technology Stack**: .NET 8, ASP.NET Core Web API
- **Team Size**: 5 backend developers (3 senior, 2 mid-level)

### Problem Statement
We need to select an ORM (Object-Relational Mapping) tool for data access that balances:
1. **Performance**: Meet <100ms response time requirements
2. **Developer Productivity**: Reduce boilerplate code
3. **Query Control**: Support complex queries and optimizations
4. **Maintainability**: Easy to test, debug, and modify
5. **Learning Curve**: Team can adopt within 2-week sprint

### Constraints
- Must support PostgreSQL-specific features (JSON columns, full-text search)
- Must integrate with existing .NET 8 / ASP.NET Core stack
- Must support async/await patterns throughout
- Must allow query optimization for performance-critical paths
- Must support unit testing with minimal mocking complexity

### Requirements
**Functional:**
- CRUD operations for Orders, OrderItems, Customers, Products
- Complex queries with joins (Orders + OrderItems + Products + Customers)
- Bulk insert operations (order import: 1000+ records)
- Transaction support (order + payment + inventory update)
- Stored procedure support (legacy reporting procedures)

**Non-Functional:**
- Query performance: <50ms for simple queries, <100ms for complex joins
- Connection pooling and efficient resource usage
- Exception handling with detailed error messages
- Support for database migrations (via separate tool)

---

## Decision

**We will use Dapper as the primary ORM for data access.**

Specifically:
- **Dapper 2.1+** for data access operations
- **FluentMigrator** for database migrations (separate concern)
- **Repository Pattern** to encapsulate Dapper usage
- **Entity Framework Core** only for admin panel (low traffic, simpler CRUD)

---

## Rationale

### Technical Rationale

#### 1. Performance Requirements Met
Dapper is a **micro-ORM** that maps database results to objects with minimal overhead:
- **Benchmark**: 10-20ms for simple queries (vs EF Core: 30-50ms)
- **No change tracking overhead** (we control when to track changes)
- **No lazy loading surprises** (explicit query execution)
- **Direct SQL execution** allows PostgreSQL-specific optimizations

Example performance comparison (real benchmark from prototype):
```
Query: Fetch Order with 10 OrderItems + Customer + Product details

Dapper:           12ms average (5,000 iterations)
EF Core (eager):  47ms average (5,000 iterations)
EF Core (lazy):   134ms average (N+1 query problem)
NHibernate:       68ms average (5,000 iterations)

Memory allocation:
Dapper:           2.4 KB per query
EF Core:          8.7 KB per query
```

#### 2. Query Control and Flexibility
Our read-heavy workload benefits from **explicit SQL control**:
- Write optimized SQL for complex reporting queries
- Use PostgreSQL-specific features (JSON operators, CTEs, window functions)
- Fine-tune query plans with indexes and hints
- No "black box" query generation

Example of complex query we need:
```sql
-- Top 10 customers by revenue (last 30 days) with product category breakdown
WITH customer_orders AS (
  SELECT
    c.customer_id,
    c.name,
    o.order_id,
    o.order_date,
    oi.quantity,
    oi.unit_price,
    p.category_id
  FROM customers c
  JOIN orders o ON c.customer_id = o.customer_id
  JOIN order_items oi ON o.order_id = oi.order_id
  JOIN products p ON oi.product_id = p.product_id
  WHERE o.order_date >= NOW() - INTERVAL '30 days'
)
SELECT
  customer_id,
  name,
  SUM(quantity * unit_price) as total_revenue,
  jsonb_object_agg(category_id, category_revenue) as revenue_by_category
FROM (
  SELECT
    customer_id,
    name,
    category_id,
    SUM(quantity * unit_price) as category_revenue
  FROM customer_orders
  GROUP BY customer_id, name, category_id
) category_totals
GROUP BY customer_id, name
ORDER BY total_revenue DESC
LIMIT 10;
```

Dapper executes this directly. EF Core would require complex LINQ or raw SQL anyway.

#### 3. Team Experience and Learning Curve
Our team profile favors Dapper:
- **3 senior developers**: Strong SQL skills, prefer explicit control
- **2 mid-level developers**: Comfortable with SQL, can learn Dapper in <1 week
- **Existing SQL knowledge**: Team already writes SQL for reporting queries
- **Minimal abstraction**: Less "magic" = easier debugging

Learning curve assessment:
- **Dapper**: 3-5 days to proficiency (familiar SQL + simple C# mapping)
- **Entity Framework Core**: 2-3 weeks (conventions, DbContext lifecycle, tracking, migrations)
- **NHibernate**: 3-4 weeks (XML mappings, session management, HQL)

#### 4. Testing and Maintainability
Dapper simplifies unit testing:
```csharp
// Repository interface (easy to mock)
public interface IOrderRepository
{
    Task<Order> GetByIdAsync(int orderId);
    Task<IEnumerable<Order>> GetByCustomerIdAsync(int customerId);
}

// Dapper implementation (explicit SQL, no hidden behavior)
public class OrderRepository : IOrderRepository
{
    private readonly IDbConnection _db;

    public async Task<Order> GetByIdAsync(int orderId)
    {
        const string sql = @"
            SELECT o.*, oi.*, p.*
            FROM orders o
            JOIN order_items oi ON o.order_id = oi.order_id
            JOIN products p ON oi.product_id = p.product_id
            WHERE o.order_id = @OrderId";

        // Multi-mapping: explicit and predictable
        var orderDict = new Dictionary<int, Order>();

        await _db.QueryAsync<Order, OrderItem, Product, Order>(
            sql,
            (order, orderItem, product) => {
                if (!orderDict.TryGetValue(order.OrderId, out var existingOrder))
                {
                    existingOrder = order;
                    existingOrder.OrderItems = new List<OrderItem>();
                    orderDict.Add(order.OrderId, existingOrder);
                }
                orderItem.Product = product;
                existingOrder.OrderItems.Add(orderItem);
                return existingOrder;
            },
            new { OrderId = orderId },
            splitOn: "order_item_id,product_id"
        );

        return orderDict.Values.FirstOrDefault();
    }
}

// Unit test (mock IDbConnection or use in-memory SQLite)
[Fact]
public async Task GetByIdAsync_ReturnsOrderWithItems()
{
    // Arrange
    var mockDb = new Mock<IDbConnection>();
    var repo = new OrderRepository(mockDb.Object);

    // Act & Assert
    var order = await repo.GetByIdAsync(123);
    Assert.NotNull(order);
}
```

EF Core requires DbContext mocking or in-memory providers with more setup.

#### 5. Operational Considerations
- **Debugging**: SQL is visible and explicit (copy to pgAdmin for testing)
- **Performance Tuning**: Direct access to EXPLAIN ANALYZE output
- **Monitoring**: Query text is exact SQL sent to database (easier APM integration)
- **Migration Path**: Can coexist with EF Core if needed (hybrid approach)

### Architectural Fit
Aligns with Clean Architecture principles:
- **Domain Layer**: POCOs (no ORM dependencies)
- **Application Layer**: Repository interfaces (IOrderRepository)
- **Infrastructure Layer**: Dapper repositories (implementation detail)

Dapper doesn't force architectural decisions (unlike EF Core's DbContext).

---

## Consequences

### Positive Consequences

#### 1. Performance Benefits (Quantified)
- **50-70% faster query execution** compared to EF Core (benchmark validated)
- **60% lower memory allocation** per request (reduced GC pressure)
- **No N+1 query problems** (explicit queries eliminate lazy loading issues)
- **Sub-100ms response times achieved** for 95th percentile

#### 2. Productivity Gains
- **Faster debugging**: Copy SQL directly to database tool
- **Easier optimization**: Modify SQL without understanding ORM internals
- **Simpler testing**: Mock IDbConnection vs DbContext lifecycle
- **Less "magic"**: Explicit code = fewer surprises

#### 3. Cost Savings
- **Lower infrastructure costs**: 30% fewer server resources due to efficiency
- **Reduced cognitive load**: Developers understand full data access stack
- **Faster onboarding**: New developers productive in 3-5 days (vs 2-3 weeks for EF)

#### 4. Flexibility for Future Needs
- Can add EF Core selectively for specific modules (admin panel, CMS)
- Can mix raw ADO.NET for ultra-performance critical paths
- Not locked into ORM conventions or migration strategies
- PostgreSQL-specific features (JSONB, CTEs) fully accessible

### Negative Consequences

#### 1. More Boilerplate Code
**Impact**: Medium
**Mitigation**:
- Use code generators for repetitive CRUD operations
- Create base repository classes for common patterns
- Use T4 templates or Source Generators for mapping code

Example boilerplate:
```csharp
// Dapper requires explicit mapping for inserts/updates
public async Task<int> InsertAsync(Order order)
{
    const string sql = @"
        INSERT INTO orders (customer_id, order_date, status, total_amount)
        VALUES (@CustomerId, @OrderDate, @Status, @TotalAmount)
        RETURNING order_id";

    return await _db.ExecuteScalarAsync<int>(sql, order);
}
```

EF Core equivalent:
```csharp
// More concise (but less explicit)
public async Task<int> InsertAsync(Order order)
{
    _context.Orders.Add(order);
    await _context.SaveChangesAsync();
    return order.OrderId;
}
```

**Reality Check**: Boilerplate is ~30% more code, but 50% faster execution and easier to debug.

#### 2. No Automatic Change Tracking
**Impact**: Low
**Explanation**: We don't need change tracking for read-heavy workloads. For updates:
- Fetch entity → Modify properties → Explicit UPDATE statement
- This is actually **more predictable** than EF's change tracking

**Mitigation**:
- Use Unit of Work pattern if change tracking becomes critical
- Consider EF Core for specific write-heavy modules

#### 3. Manual Relationship Mapping
**Impact**: Medium
**Challenge**: Multi-mapping (Order → OrderItems → Product) requires explicit code

Dapper multi-mapping example (verbose but explicit):
```csharp
var orderDict = new Dictionary<int, Order>();

await _db.QueryAsync<Order, OrderItem, Product, Order>(
    sql,
    (order, orderItem, product) => {
        if (!orderDict.TryGetValue(order.OrderId, out var existingOrder))
        {
            existingOrder = order;
            existingOrder.OrderItems = new List<OrderItem>();
            orderDict.Add(order.OrderId, existingOrder);
        }
        orderItem.Product = product;
        existingOrder.OrderItems.Add(orderItem);
        return existingOrder;
    },
    splitOn: "order_item_id,product_id"
);
```

**Mitigation**:
- Create helper methods for common multi-mapping patterns
- Use Dapper extensions (Dapper.Contrib, DapperExtensions) for simple cases
- Document multi-mapping patterns in team wiki

#### 4. No Built-in Migration Tool
**Impact**: Low
**Resolution**: Use FluentMigrator (already planned)

Dapper focuses on querying, not schema management. We'll use:
- **FluentMigrator** for migrations (better than EF migrations for PostgreSQL)
- **DbUp** as alternative (simple, versioned SQL scripts)

Example migration:
```csharp
[Migration(20251020001)]
public class CreateOrdersTable : Migration
{
    public override void Up()
    {
        Create.Table("orders")
            .WithColumn("order_id").AsInt32().PrimaryKey().Identity()
            .WithColumn("customer_id").AsInt32().NotNullable()
            .WithColumn("order_date").AsDateTime().NotNullable()
            .WithColumn("status").AsString(50).NotNullable()
            .WithColumn("total_amount").AsDecimal(10, 2).NotNullable();
    }

    public override void Down()
    {
        Delete.Table("orders");
    }
}
```

#### 5. Less Abstraction Can Lead to SQL Duplication
**Impact**: Low
**Risk**: Similar queries duplicated across repositories

**Mitigation**:
- Create shared query builders for common patterns
- Use CTEs (Common Table Expressions) for reusable sub-queries
- Establish query conventions in coding standards

Example query builder:
```csharp
public static class OrderQueryBuilder
{
    public static string BuildOrderWithDetailsQuery(bool includeItems, bool includeCustomer)
    {
        var selectClauses = new List<string> { "o.*" };
        var joinClauses = new List<string>();

        if (includeItems)
        {
            selectClauses.Add("oi.*");
            selectClauses.Add("p.*");
            joinClauses.Add("LEFT JOIN order_items oi ON o.order_id = oi.order_id");
            joinClauses.Add("LEFT JOIN products p ON oi.product_id = p.product_id");
        }

        if (includeCustomer)
        {
            selectClauses.Add("c.*");
            joinClauses.Add("LEFT JOIN customers c ON o.customer_id = c.customer_id");
        }

        return $@"
            SELECT {string.Join(", ", selectClauses)}
            FROM orders o
            {string.Join("\n", joinClauses)}";
    }
}
```

---

## Alternatives Considered

### Alternative 1: Entity Framework Core 8

**Description**: Microsoft's full-featured ORM with conventions, migrations, and LINQ support.

**Pros**:
- **Mature ecosystem**: Large community, extensive documentation
- **LINQ queries**: Type-safe, compile-time checked queries
- **Built-in migrations**: Code-first or database-first workflows
- **Change tracking**: Automatic update detection
- **Rich querying API**: Include(), ThenInclude(), AsNoTracking()
- **Team familiarity**: Some developers have EF experience

**Cons**:
- **Performance overhead**: 2-4x slower than Dapper for complex queries (benchmarked)
- **Memory allocation**: Higher per-request allocation (change tracking)
- **Query generation complexity**: LINQ → SQL translation can be inefficient
- **N+1 query risks**: Lazy loading can cause performance issues
- **PostgreSQL limitations**: Some features require raw SQL anyway
- **Black box behavior**: Harder to debug query generation issues

**Why Rejected**:
Performance is a **non-negotiable requirement** (<100ms response time). Our benchmarks showed:
- Dapper: 12ms average for order query
- EF Core: 47ms average (4x slower)

For a read-heavy system (80% reads), this overhead compounds:
- 10,000 orders/day = 10,000 * (47ms - 12ms) = **350,000ms = 5.8 minutes of extra latency per day**
- At peak (500 orders/minute), EF could approach 100ms+ response times (unacceptable)

**Additionally**:
- Our complex reporting queries require raw SQL in EF anyway (negates LINQ benefits)
- Change tracking is unnecessary for read operations (wasted memory)
- PostgreSQL-specific features (JSONB operators, CTEs) work better with raw SQL

**Use Case for EF**: We'll use EF Core for the **admin panel** (low traffic, simpler CRUD, change tracking useful).

---

### Alternative 2: NHibernate

**Description**: Mature, full-featured ORM for .NET with HQL (Hibernate Query Language).

**Pros**:
- **Powerful mapping**: Supports complex object graphs
- **HQL**: Database-agnostic query language
- **Caching**: First-level and second-level caching
- **Mature**: 15+ years of production use
- **Lazy loading**: Automatic proxy generation

**Cons**:
- **Steep learning curve**: XML mappings or Fluent API, session management
- **Performance**: Similar overhead to EF Core (change tracking, lazy loading)
- **Configuration complexity**: Requires extensive setup
- **Declining community**: Less active than EF Core or Dapper
- **Debugging difficulty**: HQL → SQL translation issues
- **Over-engineering**: Features we don't need (criteria queries, caching layers)

**Why Rejected**:
- **Learning curve**: 3-4 weeks for team proficiency (vs 3-5 days for Dapper)
- **Performance**: No advantage over EF Core, still slower than Dapper
- **Complexity**: Adds architectural weight without benefit
- **Modern alternatives**: EF Core has caught up in features, Dapper is faster

NHibernate was popular 10 years ago but is now **legacy** in the .NET ecosystem.

---

### Alternative 3: Raw ADO.NET (SqlConnection, SqlCommand)

**Description**: Use .NET's raw database API directly without ORM.

**Pros**:
- **Maximum performance**: Zero abstraction overhead
- **Full control**: Complete control over SQL execution
- **No dependencies**: Built into .NET framework
- **Debugging simplicity**: SQL is exactly what you write

**Cons**:
- **Massive boilerplate**: Manual DataReader parsing for every query
- **Error-prone**: Easy to forget closing connections, handling nulls
- **No parameterization helpers**: Manual parameter binding
- **Testing complexity**: Hard to mock SqlConnection/SqlCommand
- **Maintenance burden**: Every query is 50+ lines of code

**Example** (raw ADO.NET for simple query):
```csharp
public async Task<Order> GetByIdAsync(int orderId)
{
    using var connection = new NpgsqlConnection(_connectionString);
    await connection.OpenAsync();

    using var command = new NpgsqlCommand(
        "SELECT order_id, customer_id, order_date, status, total_amount FROM orders WHERE order_id = @orderId",
        connection
    );
    command.Parameters.AddWithValue("@orderId", orderId);

    using var reader = await command.ExecuteReaderAsync();
    if (await reader.ReadAsync())
    {
        return new Order
        {
            OrderId = reader.GetInt32(0),
            CustomerId = reader.GetInt32(1),
            OrderDate = reader.GetDateTime(2),
            Status = reader.GetString(3),
            TotalAmount = reader.GetDecimal(4)
        };
    }

    return null;
}
```

**Dapper equivalent** (10 lines vs 28 lines):
```csharp
public async Task<Order> GetByIdAsync(int orderId)
{
    const string sql = "SELECT * FROM orders WHERE order_id = @OrderId";
    return await _db.QuerySingleOrDefaultAsync<Order>(sql, new { OrderId = orderId });
}
```

**Why Rejected**:
Dapper provides **90% of ADO.NET performance with 80% less code**. The marginal performance gain (5-10%) doesn't justify:
- 5x more code per query
- Higher bug risk (manual null handling, connection management)
- Difficulty testing (mocking SqlConnection is painful)
- Maintenance burden (code reviews for 50-line queries)

**Conclusion**: Dapper is the "sweet spot" between ADO.NET's performance and EF's productivity.

---

### Alternative 4: Hybrid Approach (Dapper + EF Core)

**Description**: Use Dapper for reads, EF Core for writes.

**Pros**:
- **Best of both worlds**: Dapper performance + EF productivity
- **Read optimization**: Fast queries where it matters (80% of traffic)
- **Write simplicity**: Change tracking for complex updates

**Cons**:
- **Complexity**: Two ORMs to maintain, learn, and debug
- **Inconsistent patterns**: Developers must know when to use which
- **Migration challenges**: EF migrations may conflict with Dapper schemas
- **Testing overhead**: Mock both IDbConnection and DbContext
- **Architectural confusion**: Which layer uses which ORM?

**Why Rejected (for primary approach)**:
While hybrid is possible, we'll **start with Dapper-first** and add EF Core only where justified:
- Primary API (order management): **Dapper only**
- Admin panel (low traffic CRUD): **EF Core** (justified: change tracking useful, performance acceptable)

This keeps the architecture simple while allowing EF where it makes sense.

**Decision**: Use hybrid **selectively** (Dapper for main API, EF for admin), not globally.

---

## Implementation Plan

### Phase 1: Infrastructure Setup (Week 1)
1. **Install NuGet Packages**:
   ```bash
   dotnet add package Dapper --version 2.1.0
   dotnet add package Npgsql --version 8.0.0
   dotnet add package FluentMigrator --version 5.0.0
   dotnet add package FluentMigrator.Runner --version 5.0.0
   ```

2. **Configure Connection Management**:
   ```csharp
   // Startup.cs / Program.cs
   builder.Services.AddScoped<IDbConnection>(sp =>
       new NpgsqlConnection(builder.Configuration.GetConnectionString("OrderDb"))
   );
   ```

3. **Create Base Repository**:
   ```csharp
   public abstract class BaseRepository
   {
       protected readonly IDbConnection _db;

       protected BaseRepository(IDbConnection db)
       {
           _db = db;
       }

       protected async Task<T> QuerySingleOrDefaultAsync<T>(string sql, object param = null)
       {
           return await _db.QuerySingleOrDefaultAsync<T>(sql, param);
       }
   }
   ```

### Phase 2: Repository Implementation (Week 2-3)
4. **Create Repositories**:
   - OrderRepository
   - CustomerRepository
   - ProductRepository
   - OrderItemRepository

5. **Implement Unit of Work** (optional, if transactions needed across repositories):
   ```csharp
   public class UnitOfWork : IUnitOfWork
   {
       private IDbTransaction _transaction;

       public async Task BeginTransactionAsync()
       {
           _transaction = _db.BeginTransaction();
       }

       public async Task CommitAsync()
       {
           _transaction.Commit();
       }

       public async Task RollbackAsync()
       {
           _transaction.Rollback();
       }
   }
   ```

### Phase 3: Migration Setup (Week 2)
6. **Configure FluentMigrator**:
   ```csharp
   services.AddFluentMigratorCore()
       .ConfigureRunner(rb => rb
           .AddPostgres()
           .WithGlobalConnectionString(connectionString)
           .ScanIn(typeof(CreateOrdersTable).Assembly).For.Migrations()
       );
   ```

7. **Create Initial Migrations**:
   - CreateOrdersTable (orders, order_items)
   - CreateCustomersTable
   - CreateProductsTable
   - CreateIndexes

### Phase 4: Testing (Week 3-4)
8. **Unit Tests** (mock IDbConnection or use SQLite in-memory):
   ```csharp
   [Fact]
   public async Task GetOrderById_ReturnsOrder()
   {
       // Arrange: Setup in-memory SQLite
       var connection = new SqliteConnection("DataSource=:memory:");
       connection.Open();

       // Create schema
       await connection.ExecuteAsync("CREATE TABLE orders (...)");
       await connection.ExecuteAsync("INSERT INTO orders VALUES (...)");

       var repo = new OrderRepository(connection);

       // Act
       var order = await repo.GetByIdAsync(1);

       // Assert
       Assert.NotNull(order);
       Assert.Equal(1, order.OrderId);
   }
   ```

9. **Integration Tests** (real PostgreSQL via Docker):
   - Test multi-mapping queries
   - Test transaction rollback
   - Test bulk insert performance

10. **Performance Benchmarks**:
    - Measure query execution times (target: <50ms simple, <100ms complex)
    - Compare Dapper vs EF Core (validate 2-4x improvement)
    - Load test: 500 concurrent requests (simulate peak traffic)

### Phase 5: Documentation (Week 4)
11. **Document Conventions**:
    - SQL naming conventions (snake_case for PostgreSQL)
    - Repository patterns (async/await, parameter naming)
    - Multi-mapping examples
    - Transaction handling

12. **Create Query Library**:
    - Common queries in `Queries/` folder
    - Reusable CTEs for reporting
    - Query builder helpers

---

## Monitoring and Metrics

### Key Performance Indicators (KPIs)

**Query Performance**:
- P50 latency: <30ms (target)
- P95 latency: <100ms (target)
- P99 latency: <200ms (acceptable)

**Resource Usage**:
- Memory allocation per request: <5KB (Dapper target: 2.4KB)
- GC pressure: <10 Gen2 collections/hour
- Connection pool utilization: <80% average

**Developer Productivity**:
- Query debugging time: <5 minutes average
- New query implementation time: <30 minutes (simple), <2 hours (complex)
- Bug fix time (data access): <1 hour average

### Monitoring Tools

**Application Performance Monitoring (APM)**:
- Use **Application Insights** or **Datadog** to track:
  - Query execution times (SQL duration)
  - Exception rates
  - Memory allocation

**Database Monitoring**:
- PostgreSQL **pg_stat_statements**: Track slow queries
- **pgAdmin** query analyzer: EXPLAIN ANALYZE for optimization

**Custom Metrics** (logged via ILogger):
```csharp
_logger.LogInformation(
    "OrderRepository.GetByIdAsync executed in {ElapsedMs}ms for OrderId {OrderId}",
    stopwatch.ElapsedMilliseconds,
    orderId
);
```

---

## Review Schedule

### 3 Months (January 2026)
**Review Criteria**:
- Are we meeting <100ms P95 latency targets?
- Is query debugging taking <5 minutes on average?
- Are developers comfortable with Dapper patterns?
- Have we encountered any Dapper limitations?

**Action Items**:
- Analyze slow query logs (optimize if needed)
- Gather developer feedback (pain points?)
- Review boilerplate code (can we reduce it?)

### 6 Months (April 2026)
**Review Criteria**:
- Has technical debt accumulated (SQL duplication)?
- Are there queries better suited for EF Core?
- Has team grown? New developers onboarded successfully?

**Action Items**:
- Refactor common query patterns into helpers
- Evaluate hybrid approach for new modules
- Update documentation based on lessons learned

### 12 Months (October 2026)
**Full ADR Review**:
- **Performance**: Still meeting targets as traffic grows?
- **Productivity**: Team still productive with Dapper?
- **Maintenance**: Code quality maintained (no spaghetti SQL)?
- **Scalability**: Dapper scales with increased complexity?

**Possible Outcomes**:
1. **Continue with Dapper**: If all targets met, decision validated
2. **Add EF Core selectively**: If some modules benefit from change tracking
3. **Re-evaluate**: If performance degrades or team struggles (unlikely)

---

## Related Documents

- **ADR-001: Database Selection** (PostgreSQL choice, impacts ORM requirements)
- **Tech Stack Documentation**: `devforgeai/specs/context/tech-stack.md` (lists Dapper as standard)
- **Dependencies Documentation**: `devforgeai/specs/context/dependencies.md` (approved Dapper + FluentMigrator packages)
- **Coding Standards**: `devforgeai/specs/context/coding-standards.md` (repository patterns, async/await)

---

## Approval and Sign-Off

**Approved By**:
- ✅ Technical Architect (performance validated via benchmarks)
- ✅ Backend Lead (team comfortable with SQL)
- ✅ Database Administrator (PostgreSQL-specific queries supported)

**Dissent**: None

**Date Approved**: 2025-10-20

---

## References

### Benchmarks
- [Dapper vs EF Core Performance Comparison](https://github.com/DapperLib/Dapper/blob/main/Readme.md#performance)
- Internal benchmark results: `docs/benchmarks/orm-comparison-2025-10.md`

### Documentation
- [Dapper Official Documentation](https://github.com/DapperLib/Dapper)
- [FluentMigrator Documentation](https://fluentmigrator.github.io/)
- [PostgreSQL with Dapper Best Practices](https://www.npgsql.org/doc/types/json.html)

### Team Resources
- Dapper tutorial: `docs/tutorials/dapper-getting-started.md`
- Multi-mapping patterns: `docs/patterns/dapper-multi-mapping.md`
- Query optimization guide: `docs/guides/postgresql-query-optimization.md`
