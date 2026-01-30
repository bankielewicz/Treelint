# Test-Driven Development Patterns

## TDD Cycle: Red → Green → Refactor

### Red Phase: Write Failing Test

**Principle:** Write the test for behavior that doesn't exist yet

**Pattern:**
1. Identify ONE specific behavior to test
2. Write test that exercises that behavior
3. Run test → MUST fail (if passes, test is wrong!)
4. Verify failure is for the RIGHT reason (not syntax error)

**Example (C# with xUnit):**

```csharp
[Fact]
public void CalculateDiscount_ValidCoupon_ReturnsDiscountedPrice()
{
    // Arrange
    var service = new OrderService(); // Doesn't exist yet → compilation error
    var order = new Order { Total = 100m };
    var coupon = new Coupon { Code = "SAVE10", Discount = 10 };

    // Act
    var result = service.CalculateDiscount(order, coupon); // Method doesn't exist

    // Assert
    Assert.Equal(90m, result.DiscountedTotal);
}
```

**Run:** `dotnet test` → ❌ RED (compilation error or method missing)

### Green Phase: Make Test Pass

**Principle:** Write MINIMAL code to pass the test (resist the urge to add "extra" features)

**Pattern:**
1. Create just enough code to compile
2. Implement simplest solution that passes test
3. Run test → MUST pass
4. Don't add features not covered by tests

**Example:**

```csharp
public class OrderService
{
    public DiscountResult CalculateDiscount(Order order, Coupon coupon)
    {
        // Simplest implementation
        var discountAmount = order.Total * (coupon.Discount / 100m);
        var discountedTotal = order.Total - discountAmount;

        return new DiscountResult
        {
            OriginalTotal = order.Total,
            DiscountAmount = discountAmount,
            DiscountedTotal = discountedTotal
        };
    }
}
```

**Run:** `dotnet test` → ✅ GREEN (test passes)

### Refactor Phase: Improve Code

**Principle:** Improve code quality WITHOUT changing behavior (tests stay green)

**Pattern:**
1. Extract magic numbers to constants
2. Extract complex logic to helper methods
3. Rename variables for clarity
4. Remove duplication
5. Run tests after EVERY change → MUST stay green

**Example:**

```csharp
public class OrderService
{
    private const int PERCENTAGE_DIVISOR = 100;

    public DiscountResult CalculateDiscount(Order order, Coupon coupon)
    {
        var discountAmount = CalculateDiscountAmount(order.Total, coupon.Discount);
        var discountedTotal = ApplyDiscount(order.Total, discountAmount);

        return new DiscountResult
        {
            OriginalTotal = order.Total,
            DiscountAmount = discountAmount,
            DiscountedTotal = discountedTotal
        };
    }

    private decimal CalculateDiscountAmount(decimal total, int discountPercentage)
    {
        return total * (discountPercentage / (decimal)PERCENTAGE_DIVISOR);
    }

    private decimal ApplyDiscount(decimal total, decimal discountAmount)
    {
        return total - discountAmount;
    }
}
```

**Run:** `dotnet test` → ✅ GREEN (tests still pass)

---

## Test Structure Patterns

### AAA Pattern (Arrange-Act-Assert)

**Most common pattern for unit tests**

```csharp
[Fact]
public void MethodName_Scenario_ExpectedBehavior()
{
    // Arrange: Set up test data and dependencies
    var dependency = new Mock<IDependency>();
    var sut = new SystemUnderTest(dependency.Object);
    var input = new TestInput { Value = 42 };

    // Act: Execute the method being tested
    var result = sut.MethodName(input);

    // Assert: Verify the outcome
    Assert.Equal(expectedValue, result);
}
```

### Test Naming Convention

**Pattern:** `MethodName_Scenario_ExpectedBehavior`

**Examples:**
- `CalculateDiscount_ValidCoupon_ReturnsDiscountedPrice`
- `CalculateDiscount_ExpiredCoupon_ReturnsOriginalPrice`
- `CalculateDiscount_NullCoupon_ThrowsArgumentNullException`
- `CalculateDiscount_InvalidCouponCode_ReturnsError`

**Why this matters:**
- Clear test intent
- Easy to identify which scenario failed
- Documents expected behavior

### Given-When-Then (BDD Style)

**Alternative to AAA, more readable for complex scenarios**

```csharp
[Fact]
public void Should_ReturnDiscountedPrice_When_CouponIsValid()
{
    // Given a valid order and coupon
    var order = new Order { Total = 100m };
    var coupon = new Coupon { Code = "SAVE10", Discount = 10, ExpiryDate = DateTime.Now.AddDays(1) };
    var service = new OrderService();

    // When calculating the discount
    var result = service.CalculateDiscount(order, coupon);

    // Then the discounted price should be returned
    Assert.Equal(90m, result.DiscountedTotal);
}
```

---

## Test Types & When to Use

### Unit Tests

**Scope:** Single method/function in isolation

**When to use:**
- Testing business logic
- Testing calculations
- Testing validation rules
- Testing data transformations

**Pattern:**
```csharp
public class OrderServiceTests
{
    [Fact]
    public void CalculateTotal_MultipleItems_ReturnsSumOfPrices()
    {
        // Arrange
        var service = new OrderService();
        var items = new List<OrderItem>
        {
            new OrderItem { Price = 10m, Quantity = 2 },
            new OrderItem { Price = 15m, Quantity = 1 }
        };

        // Act
        var total = service.CalculateTotal(items);

        // Assert
        Assert.Equal(35m, total);
    }
}
```

**Characteristics:**
- Fast (milliseconds)
- No external dependencies (use mocks/fakes)
- Deterministic (same input → same output)
- Test pyramid base (most tests should be unit tests)

### Integration Tests

**Scope:** Multiple components working together

**When to use:**
- Testing database access (repositories)
- Testing API endpoints
- Testing file I/O
- Testing external service integration

**Pattern (Database Integration with Dapper):**
```csharp
public class OrderRepositoryTests : IDisposable
{
    private readonly SqlConnection _connection;

    public OrderRepositoryTests()
    {
        _connection = new SqlConnection("test_connection_string");
        _connection.Open();
        SetupTestDatabase();
    }

    [Fact]
    public async Task GetByIdAsync_ExistingOrder_ReturnsOrder()
    {
        // Arrange
        var repository = new OrderRepository(_connection);
        var orderId = await InsertTestOrder(); // Test data setup

        // Act
        var order = await repository.GetByIdAsync(orderId);

        // Assert
        Assert.NotNull(order);
        Assert.Equal(orderId, order.Id);
    }

    public void Dispose()
    {
        CleanupTestDatabase();
        _connection.Dispose();
    }
}
```

**Characteristics:**
- Slower (seconds)
- Uses real dependencies (database, filesystem, etc.)
- May require setup/teardown
- Test pyramid middle layer

### Contract Tests

**Scope:** API request/response contracts

**When to use:**
- Testing API endpoints
- Verifying request/response formats
- Ensuring backward compatibility

**Pattern (ASP.NET Core):**
```csharp
public class OrdersControllerTests : IClassFixture<WebApplicationFactory<Program>>
{
    private readonly HttpClient _client;

    public OrdersControllerTests(WebApplicationFactory<Program> factory)
    {
        _client = factory.CreateClient();
    }

    [Fact]
    public async Task CreateOrder_ValidRequest_ReturnsCreatedOrder()
    {
        // Arrange
        var request = new CreateOrderRequest
        {
            CustomerId = 123,
            Items = new[] { new OrderItemRequest { ProductId = 1, Quantity = 2 } }
        };

        // Act
        var response = await _client.PostAsJsonAsync("/api/orders", request);

        // Assert
        response.EnsureSuccessStatusCode();
        Assert.Equal(HttpStatusCode.Created, response.StatusCode);

        var order = await response.Content.ReadFromJsonAsync<Order>();
        Assert.NotNull(order);
        Assert.Equal(123, order.CustomerId);
    }
}
```

### End-to-End Tests

**Scope:** Complete user workflows

**When to use:**
- Testing critical user journeys
- Testing UI interactions
- Testing multi-step processes

**Pattern (Playwright):**
```csharp
[Test]
public async Task CompleteCheckoutFlow()
{
    // Navigate to product page
    await page.GotoAsync("https://localhost:5001/products/1");

    // Add to cart
    await page.ClickAsync("#add-to-cart");

    // Go to cart
    await page.ClickAsync("#view-cart");

    // Verify item in cart
    await Expect(page.Locator(".cart-item")).ToHaveCountAsync(1);

    // Proceed to checkout
    await page.ClickAsync("#checkout");

    // Fill checkout form
    await page.FillAsync("#email", "test@example.com");
    await page.FillAsync("#card-number", "4242424242424242");

    // Submit order
    await page.ClickAsync("#submit-order");

    // Verify success
    await Expect(page.Locator(".order-confirmation")).ToBeVisibleAsync();
}
```

**Characteristics:**
- Slowest (seconds to minutes)
- Tests entire system
- Brittle (UI changes break tests)
- Test pyramid top (fewest tests)

---

## Mocking Patterns

### When to Mock

**Mock external dependencies:**
- Databases
- External APIs
- File system
- Time (DateTime.Now)
- Random number generators
- Email services
- Payment gateways

**Don't mock:**
- Simple DTOs/POCOs
- Value objects
- Internal domain logic
- Code you're testing

### Mock Setup (NSubstitute)

**Pattern:**
```csharp
public class OrderServiceTests
{
    [Fact]
    public async Task ProcessOrder_ValidOrder_SavesOrder()
    {
        // Arrange: Create mock repository
        var mockRepository = Substitute.For<IOrderRepository>();
        mockRepository
            .SaveAsync(Arg.Any<Order>())
            .Returns(Task.FromResult(new Order { Id = 123 }));

        var service = new OrderService(mockRepository);
        var order = new Order { CustomerId = 1, Total = 100m };

        // Act
        var result = await service.ProcessOrderAsync(order);

        // Assert: Verify repository was called
        await mockRepository.Received(1).SaveAsync(Arg.Is<Order>(o => o.CustomerId == 1));
        Assert.Equal(123, result.Id);
    }
}
```

### Verify Behavior vs State

**State verification (prefer when possible):**
```csharp
// Verify the RESULT of the operation
var result = service.CalculateDiscount(order, coupon);
Assert.Equal(90m, result.DiscountedTotal);
```

**Behavior verification (when side effects matter):**
```csharp
// Verify the METHOD WAS CALLED
await service.SendOrderConfirmationEmail(order);
await emailService.Received(1).SendEmailAsync(
    Arg.Is<string>(email => email == "customer@example.com"),
    Arg.Any<string>(),
    Arg.Any<string>()
);
```

---

## Test Data Builders

**Problem:** Complex object setup clutters tests

**Solution:** Builder pattern for test data

**Pattern:**
```csharp
public class OrderBuilder
{
    private int _customerId = 1;
    private decimal _total = 100m;
    private List<OrderItem> _items = new();
    private OrderStatus _status = OrderStatus.Pending;

    public OrderBuilder WithCustomerId(int customerId)
    {
        _customerId = customerId;
        return this;
    }

    public OrderBuilder WithTotal(decimal total)
    {
        _total = total;
        return this;
    }

    public OrderBuilder WithStatus(OrderStatus status)
    {
        _status = status;
        return this;
    }

    public OrderBuilder WithItem(string productName, decimal price, int quantity)
    {
        _items.Add(new OrderItem
        {
            ProductName = productName,
            Price = price,
            Quantity = quantity
        });
        return this;
    }

    public Order Build()
    {
        return new Order
        {
            CustomerId = _customerId,
            Total = _total,
            Items = _items,
            Status = _status
        };
    }
}

// Usage in tests
[Fact]
public void ProcessOrder_PendingOrder_UpdatesStatus()
{
    // Arrange
    var order = new OrderBuilder()
        .WithCustomerId(123)
        .WithTotal(150m)
        .WithItem("Product A", 50m, 2)
        .WithItem("Product B", 50m, 1)
        .WithStatus(OrderStatus.Pending)
        .Build();

    var service = new OrderService();

    // Act
    service.ProcessOrder(order);

    // Assert
    Assert.Equal(OrderStatus.Processing, order.Status);
}
```

---

## Edge Cases & Error Testing

### Test Edge Cases

**Common edge cases:**
- Null inputs
- Empty collections
- Boundary values (0, -1, MAX_VALUE)
- Invalid formats
- Expired data
- Concurrent access

**Pattern:**
```csharp
public class OrderServiceEdgeCaseTests
{
    [Fact]
    public void CalculateDiscount_NullOrder_ThrowsArgumentNullException()
    {
        var service = new OrderService();
        Assert.Throws<ArgumentNullException>(() => service.CalculateDiscount(null, new Coupon()));
    }

    [Fact]
    public void CalculateDiscount_NullCoupon_ThrowsArgumentNullException()
    {
        var service = new OrderService();
        Assert.Throws<ArgumentNullException>(() => service.CalculateDiscount(new Order(), null));
    }

    [Fact]
    public void CalculateDiscount_ZeroTotal_ReturnsZeroDiscount()
    {
        var service = new OrderService();
        var order = new Order { Total = 0m };
        var coupon = new Coupon { Discount = 10 };

        var result = service.CalculateDiscount(order, coupon);

        Assert.Equal(0m, result.DiscountAmount);
    }

    [Fact]
    public void CalculateDiscount_ExpiredCoupon_ReturnsOriginalPrice()
    {
        var service = new OrderService();
        var order = new Order { Total = 100m };
        var coupon = new Coupon { Discount = 10, ExpiryDate = DateTime.Now.AddDays(-1) };

        var result = service.CalculateDiscount(order, coupon);

        Assert.Equal(100m, result.DiscountedTotal);
    }
}
```

### Test Error Handling (Result Pattern)

**If coding-standards.md specifies Result Pattern:**

```csharp
[Fact]
public void ValidateOrder_InvalidOrder_ReturnsFailureResult()
{
    var service = new OrderService();
    var invalidOrder = new Order { Total = -10m }; // Invalid: negative total

    var result = service.ValidateOrder(invalidOrder);

    Assert.False(result.IsSuccess);
    Assert.Equal("Order total cannot be negative", result.Error);
}

[Fact]
public void ValidateOrder_ValidOrder_ReturnsSuccessResult()
{
    var service = new OrderService();
    var validOrder = new Order { Total = 100m, CustomerId = 1 };

    var result = service.ValidateOrder(validOrder);

    Assert.True(result.IsSuccess);
    Assert.Null(result.Error);
}
```

---

## Code Coverage Guidance

### Coverage Targets

**Target levels:**
- **Critical business logic:** 95%+ coverage
- **Most application code:** 80%+ coverage
- **Simple DTOs/POCOs:** 60%+ coverage (properties auto-tested)
- **Infrastructure/UI:** 70%+ coverage

### What to Cover

**High priority:**
- Business logic calculations
- Validation rules
- State transitions
- Error handling paths
- Edge cases

**Lower priority:**
- Property getters/setters (unless logic)
- Constructors (unless complex)
- Simple mapping code
- Auto-generated code

### Coverage Reports

**Generate coverage:**
```bash
# .NET
dotnet test --collect:"XPlat Code Coverage"

# Python
pytest --cov=src --cov-report=html --cov-report=term

# JavaScript
npm test -- --coverage
```

**Analyze coverage:**
- Identify untested code paths
- Prioritize testing critical paths
- Don't chase 100% (diminishing returns)
- Focus on meaningful coverage, not metrics

---

## Test Maintenance

### Keep Tests Fast

**Slow tests = tests don't get run**

**Strategies:**
- Use mocks for external dependencies
- Minimize database operations
- Parallelize test execution
- Avoid Thread.Sleep (use async properly)

### Keep Tests Independent

**Each test should:**
- Set up its own data
- Not depend on other tests
- Clean up after itself
- Be runnable in any order

**Anti-pattern:**
```csharp
// ❌ FORBIDDEN: Tests depend on execution order
public class BadTests
{
    private static Order _sharedOrder;

    [Fact]
    public void Test1_CreateOrder() // Must run first!
    {
        _sharedOrder = new Order { Id = 1 };
    }

    [Fact]
    public void Test2_UpdateOrder() // Depends on Test1
    {
        _sharedOrder.Status = OrderStatus.Completed; // Fails if Test1 didn't run
    }
}
```

**Correct pattern:**
```csharp
// ✅ CORRECT: Each test is independent
public class GoodTests
{
    [Fact]
    public void UpdateOrder_PendingOrder_UpdatesStatus()
    {
        // Arrange: Each test creates its own data
        var order = new Order { Id = 1, Status = OrderStatus.Pending };
        var service = new OrderService();

        // Act
        service.UpdateOrderStatus(order, OrderStatus.Completed);

        // Assert
        Assert.Equal(OrderStatus.Completed, order.Status);
    }
}
```

### Keep Tests Readable

**Tests are documentation**

**Guidelines:**
- Clear test names
- Use AAA or Given-When-Then
- Extract complex setup to helper methods
- Use test builders for complex objects
- Avoid logic in tests (no if/loops)
- One assertion per test (when possible)

---

## TDD Anti-Patterns to Avoid

### The Liar

**Problem:** Test passes when it should fail, or claims to test something it doesn't

```csharp
// ❌ FORBIDDEN: Test doesn't actually test anything
[Fact]
public void CalculateTotal_Always_ReturnsValue()
{
    var service = new OrderService();
    var result = service.CalculateTotal(new List<OrderItem>());
    Assert.NotNull(result); // Useless assertion
}
```

### The Slow Poke

**Problem:** Tests take too long to run

```csharp
// ❌ FORBIDDEN: Unnecessary delays
[Fact]
public async Task ProcessOrder_Always_Succeeds()
{
    await Task.Delay(5000); // Why?!
    var service = new OrderService();
    var result = service.ProcessOrder(new Order());
    Assert.True(result.IsSuccess);
}
```

### The Giant

**Problem:** Test does too much, tests multiple scenarios

```csharp
// ❌ FORBIDDEN: Testing too many things at once
[Fact]
public void OrderService_AllScenarios_Work()
{
    var service = new OrderService();

    // Test 1: Create order
    var order1 = service.CreateOrder(new CreateOrderRequest());
    Assert.NotNull(order1);

    // Test 2: Update order
    order1.Status = OrderStatus.Completed;
    service.UpdateOrder(order1);
    Assert.Equal(OrderStatus.Completed, order1.Status);

    // Test 3: Delete order
    service.DeleteOrder(order1.Id);
    Assert.Null(service.GetOrder(order1.Id));
}
```

**Solution:** Split into separate focused tests

### The Mockery

**Problem:** Over-mocking, mocking things that shouldn't be mocked

```csharp
// ❌ FORBIDDEN: Mocking simple value objects
var mockOrder = Substitute.For<IOrder>(); // Order should be a simple POCO!
mockOrder.Total.Returns(100m);
```

### The Inspector

**Problem:** Testing private implementation details instead of public behavior

```csharp
// ❌ FORBIDDEN: Testing internal state instead of behavior
[Fact]
public void ProcessOrder_Always_SetsInternalFlag()
{
    var service = new OrderService();
    service.ProcessOrder(new Order());

    // Using reflection to check private field
    var field = typeof(OrderService).GetField("_isProcessed", BindingFlags.NonPublic);
    Assert.True((bool)field.GetValue(service)); // BAD!
}
```

**Solution:** Test public behavior only

---

## TDD in Practice: Complete Example

### Requirement

"Implement order discount calculation. Apply coupon discount percentage to order total. Expired coupons should not apply. Invalid coupon codes should return an error."

### Step 1: Write First Test (Red)

```csharp
public class OrderServiceTests
{
    [Fact]
    public void CalculateDiscount_ValidCoupon_ReturnsDiscountedPrice()
    {
        // Arrange
        var service = new OrderService(); // Doesn't exist yet
        var order = new Order { Total = 100m };
        var coupon = new Coupon { Code = "SAVE10", Discount = 10, ExpiryDate = DateTime.Now.AddDays(1) };

        // Act
        var result = service.CalculateDiscount(order, coupon);

        // Assert
        Assert.Equal(90m, result.DiscountedTotal);
    }
}
```

**Run:** ❌ RED (OrderService doesn't exist)

### Step 2: Minimal Implementation (Green)

```csharp
public class OrderService
{
    public DiscountResult CalculateDiscount(Order order, Coupon coupon)
    {
        var discountAmount = order.Total * (coupon.Discount / 100m);
        return new DiscountResult
        {
            DiscountedTotal = order.Total - discountAmount
        };
    }
}

public class DiscountResult
{
    public decimal DiscountedTotal { get; set; }
}
```

**Run:** ✅ GREEN

### Step 3: Add Next Test (Red)

```csharp
[Fact]
public void CalculateDiscount_ExpiredCoupon_ReturnsOriginalPrice()
{
    var service = new OrderService();
    var order = new Order { Total = 100m };
    var coupon = new Coupon { Code = "SAVE10", Discount = 10, ExpiryDate = DateTime.Now.AddDays(-1) };

    var result = service.CalculateDiscount(order, coupon);

    Assert.Equal(100m, result.DiscountedTotal);
}
```

**Run:** ❌ RED (expired coupon still applies discount)

### Step 4: Implement Expiry Check (Green)

```csharp
public DiscountResult CalculateDiscount(Order order, Coupon coupon)
{
    if (coupon.ExpiryDate < DateTime.Now)
    {
        return new DiscountResult { DiscountedTotal = order.Total };
    }

    var discountAmount = order.Total * (coupon.Discount / 100m);
    return new DiscountResult
    {
        DiscountedTotal = order.Total - discountAmount
    };
}
```

**Run:** ✅ GREEN

### Step 5: Refactor

```csharp
public class OrderService
{
    public DiscountResult CalculateDiscount(Order order, Coupon coupon)
    {
        if (IsCouponExpired(coupon))
        {
            return CreateNonDiscountedResult(order);
        }

        return CreateDiscountedResult(order, coupon);
    }

    private bool IsCouponExpired(Coupon coupon)
    {
        return coupon.ExpiryDate < DateTime.Now;
    }

    private DiscountResult CreateNonDiscountedResult(Order order)
    {
        return new DiscountResult
        {
            OriginalTotal = order.Total,
            DiscountAmount = 0m,
            DiscountedTotal = order.Total
        };
    }

    private DiscountResult CreateDiscountedResult(Order order, Coupon coupon)
    {
        var discountAmount = CalculateDiscountAmount(order.Total, coupon.Discount);
        return new DiscountResult
        {
            OriginalTotal = order.Total,
            DiscountAmount = discountAmount,
            DiscountedTotal = order.Total - discountAmount
        };
    }

    private decimal CalculateDiscountAmount(decimal total, int discountPercentage)
    {
        return total * (discountPercentage / 100m);
    }
}
```

**Run:** ✅ GREEN (all tests still pass after refactoring)

### Step 6: Continue with Remaining Requirements

```csharp
// Test for invalid coupon code
[Fact]
public void CalculateDiscount_InvalidCouponCode_ReturnsError()
{
    var service = new OrderService();
    var order = new Order { Total = 100m };
    var coupon = new Coupon { Code = "", Discount = 10, ExpiryDate = DateTime.Now.AddDays(1) };

    var result = service.CalculateDiscount(order, coupon);

    Assert.False(result.IsSuccess);
    Assert.Equal("Invalid coupon code", result.Error);
}
```

**Continue cycle: Red → Green → Refactor**

---

## Quick Reference

### TDD Workflow Checklist

- [ ] **Red:** Write failing test for ONE behavior
- [ ] **Verify:** Test fails for RIGHT reason
- [ ] **Green:** Write MINIMAL code to pass
- [ ] **Verify:** Test passes
- [ ] **Refactor:** Improve code quality
- [ ] **Verify:** Tests still pass
- [ ] **Repeat:** Next behavior

### Test Quality Checklist

- [ ] Clear test names (MethodName_Scenario_ExpectedBehavior)
- [ ] AAA or Given-When-Then structure
- [ ] Each test is independent
- [ ] Tests run fast (<100ms for unit tests)
- [ ] No logic in tests (no if/loops)
- [ ] Edge cases covered
- [ ] Error paths tested
- [ ] Mocks used appropriately (external dependencies only)
- [ ] Tests document expected behavior

### Code Coverage Checklist

- [ ] Critical business logic: 95%+ coverage
- [ ] Application code: 80%+ coverage
- [ ] All error paths tested
- [ ] Edge cases covered
- [ ] Integration points tested

This reference guide should be used throughout the TDD development process to ensure high-quality, maintainable tests that drive implementation while preventing technical debt.
