# Refactoring Patterns Reference

Complete catalog of refactoring techniques for improving code quality during Phase 05 (Refactor) of the TDD workflow.

## When to Refactor

**Golden Rule: Refactor ONLY when tests are GREEN**

✅ **CORRECT Timing:**
- After tests pass (Phase 05: Refactor)
- When code smells are detected
- When complexity exceeds thresholds
- During dedicated refactoring phase of TDD

❌ **FORBIDDEN Timing:**
- During RED phase (tests failing)
- While writing implementation (GREEN phase incomplete)
- When tests are broken
- Under time pressure without tests

**Refactor Frequency:**
- After every RED → GREEN cycle
- Continuously (small refactorings)
- When adding new functionality (prep work)
- During code reviews

---

## Common Refactoring Techniques

### Extract Method

**Problem:** Long method with multiple responsibilities or complex logic

**When to use:**
- Methods > 50 lines
- Multiple levels of abstraction in one method
- Repeated code blocks
- Complex conditional logic
- Commented sections (comments often mark extractable methods)

**Pattern:**
```csharp
// ❌ BEFORE: Long method doing too much
public void ProcessOrder(Order order)
{
    // Validate order
    if (order.Total < 0) throw new InvalidOperationException("Negative total");
    if (order.Items.Count == 0) throw new InvalidOperationException("Empty order");
    if (order.CustomerId == 0) throw new InvalidOperationException("No customer");

    // Calculate discount
    decimal discount = 0m;
    if (order.CouponCode != null)
    {
        var coupon = _couponRepository.GetByCode(order.CouponCode);
        if (coupon != null && coupon.ExpiryDate > DateTime.Now)
        {
            discount = order.Total * (coupon.Discount / 100m);
        }
    }

    // Apply discount
    order.DiscountAmount = discount;
    order.FinalTotal = order.Total - discount;

    // Save order
    _orderRepository.Save(order);

    // Send confirmation
    var emailBody = $"Order #{order.Id} confirmed. Total: ${order.FinalTotal}";
    _emailService.Send(order.CustomerEmail, "Order Confirmation", emailBody);
}

// ✅ AFTER: Extracted into focused methods
public void ProcessOrder(Order order)
{
    ValidateOrder(order);
    ApplyDiscounts(order);
    SaveOrder(order);
    SendConfirmationEmail(order);
}

private void ValidateOrder(Order order)
{
    if (order.Total < 0) throw new InvalidOperationException("Negative total");
    if (order.Items.Count == 0) throw new InvalidOperationException("Empty order");
    if (order.CustomerId == 0) throw new InvalidOperationException("No customer");
}

private void ApplyDiscounts(Order order)
{
    var discount = CalculateDiscount(order);
    order.DiscountAmount = discount;
    order.FinalTotal = order.Total - discount;
}

private decimal CalculateDiscount(Order order)
{
    if (order.CouponCode == null) return 0m;

    var coupon = _couponRepository.GetByCode(order.CouponCode);
    if (coupon == null || coupon.ExpiryDate <= DateTime.Now) return 0m;

    return order.Total * (coupon.Discount / 100m);
}

private void SaveOrder(Order order)
{
    _orderRepository.Save(order);
}

private void SendConfirmationEmail(Order order)
{
    var emailBody = $"Order #{order.Id} confirmed. Total: ${order.FinalTotal}";
    _emailService.Send(order.CustomerEmail, "Order Confirmation", emailBody);
}
```

**Benefits:**
- Single Responsibility Principle
- Easier to test (can test each method independently)
- Easier to understand (method names document intent)
- Reusable extracted methods

### Extract Class

**Problem:** Class with too many responsibilities (God Object)

**When to use:**
- Classes > 500 lines
- Classes with >10 public methods
- Classes handling multiple domains
- High coupling between subset of methods

**Pattern:**
```csharp
// ❌ BEFORE: God Object doing everything
public class OrderService
{
    // Order processing
    public void CreateOrder(Order order) { ... }
    public void UpdateOrder(Order order) { ...}
    public void DeleteOrder(int id) { ... }

    // Discount calculation
    public decimal CalculateDiscount(Order order, Coupon coupon) { ... }
    public bool ValidateCoupon(Coupon coupon) { ... }
    public Coupon GetBestCoupon(Order order) { ... }

    // Email notifications
    public void SendOrderConfirmation(Order order) { ... }
    public void SendShippingNotification(Order order) { ... }
    public void SendCancellationEmail(Order order) { ... }

    // Inventory management
    public void ReserveInventory(Order order) { ... }
    public void ReleaseInventory(Order order) { ... }
    public bool CheckInventoryAvailability(Order order) { ... }
}

// ✅ AFTER: Focused classes with single responsibilities
public class OrderService
{
    private readonly DiscountCalculator _discountCalculator;
    private readonly OrderNotifier _orderNotifier;
    private readonly InventoryManager _inventoryManager;

    public OrderService(
        DiscountCalculator discountCalculator,
        OrderNotifier orderNotifier,
        InventoryManager inventoryManager)
    {
        _discountCalculator = discountCalculator;
        _orderNotifier = orderNotifier;
        _inventoryManager = inventoryManager;
    }

    public void CreateOrder(Order order)
    {
        _inventoryManager.ReserveInventory(order);
        var discount = _discountCalculator.CalculateBestDiscount(order);
        order.ApplyDiscount(discount);
        SaveOrder(order);
        _orderNotifier.SendOrderConfirmation(order);
    }

    // Other order CRUD operations
}

public class DiscountCalculator
{
    public decimal CalculateDiscount(Order order, Coupon coupon) { ... }
    public bool ValidateCoupon(Coupon coupon) { ... }
    public Coupon GetBestCoupon(Order order) { ... }
    public decimal CalculateBestDiscount(Order order) { ... }
}

public class OrderNotifier
{
    public void SendOrderConfirmation(Order order) { ... }
    public void SendShippingNotification(Order order) { ... }
    public void SendCancellationEmail(Order order) { ... }
}

public class InventoryManager
{
    public void ReserveInventory(Order order) { ... }
    public void ReleaseInventory(Order order) { ... }
    public bool CheckInventoryAvailability(Order order) { ... }
}
```

### Introduce Parameter Object

**Problem:** Methods with long parameter lists (>4 parameters)

**When to use:**
- Parameter lists > 4 arguments
- Parameters that naturally group together
- Same parameters repeated across methods

**Pattern:**
```csharp
// ❌ BEFORE: Long parameter list
public Order CreateOrder(
    int customerId,
    string customerName,
    string customerEmail,
    string shippingAddress,
    string billingAddress,
    string paymentMethod,
    string couponCode)
{
    // ...
}

// ✅ AFTER: Parameter object
public class CreateOrderRequest
{
    public int CustomerId { get; set; }
    public string CustomerName { get; set; }
    public string CustomerEmail { get; set; }
    public string ShippingAddress { get; set; }
    public string BillingAddress { get; set; }
    public string PaymentMethod { get; set; }
    public string CouponCode { get; set; }
}

public Order CreateOrder(CreateOrderRequest request)
{
    ValidateRequest(request);
    // ...
}
```

### Replace Conditional with Polymorphism

**Problem:** Complex if/switch statements based on type

**When to use:**
- Type-checking with if/switch
- Repeated conditionals on same type check
- Adding new types requires modifying conditionals

**Pattern:**
```csharp
// ❌ BEFORE: Type checking with conditionals
public decimal CalculateShipping(Order order, string shipMethod)
{
    if (shipMethod == "Standard")
    {
        return order.Weight * 0.5m;
    }
    else if (shipMethod == "Express")
    {
        return order.Weight * 1.5m + 10m;
    }
    else if (shipMethod == "Overnight")
    {
        return order.Weight * 3.0m + 25m;
    }
    else
    {
        throw new InvalidOperationException("Unknown shipping method");
    }
}

// ✅ AFTER: Polymorphism with strategy pattern
public interface IShippingStrategy
{
    decimal CalculateShipping(Order order);
}

public class StandardShipping : IShippingStrategy
{
    public decimal CalculateShipping(Order order)
    {
        return order.Weight * 0.5m;
    }
}

public class ExpressShipping : IShippingStrategy
{
    public decimal CalculateShipping(Order order)
    {
        return order.Weight * 1.5m + 10m;
    }
}

public class OvernightShipping : IShippingStrategy
{
    public decimal CalculateShipping(Order order)
    {
        return order.Weight * 3.0m + 25m;
    }
}

public class ShippingCalculator
{
    private readonly Dictionary<string, IShippingStrategy> _strategies;

    public ShippingCalculator()
    {
        _strategies = new Dictionary<string, IShippingStrategy>
        {
            ["Standard"] = new StandardShipping(),
            ["Express"] = new ExpressShipping(),
            ["Overnight"] = new OvernightShipping()
        };
    }

    public decimal CalculateShipping(Order order, string method)
    {
        if (!_strategies.TryGetValue(method, out var strategy))
            throw new InvalidOperationException($"Unknown shipping method: {method}");

        return strategy.CalculateShipping(order);
    }
}
```

### Replace Magic Numbers with Named Constants

**Problem:** Hardcoded values without explanation

**Pattern:**
```csharp
// ❌ BEFORE: Magic numbers
public bool IsEligibleForFreeShipping(Order order)
{
    return order.Total >= 50m;
}

public decimal CalculateLoyaltyPoints(Order order)
{
    return order.Total * 0.05m;
}

// ✅ AFTER: Named constants
public class OrderService
{
    private const decimal FREE_SHIPPING_THRESHOLD = 50m;
    private const decimal LOYALTY_POINTS_RATE = 0.05m;

    public bool IsEligibleForFreeShipping(Order order)
    {
        return order.Total >= FREE_SHIPPING_THRESHOLD;
    }

    public decimal CalculateLoyaltyPoints(Order order)
    {
        return order.Total * LOYALTY_POINTS_RATE;
    }
}
```

### Remove Duplication (DRY Principle)

**Problem:** Same code repeated in multiple places

**Pattern:**
```csharp
// ❌ BEFORE: Duplication
public void ProcessOnlineOrder(Order order)
{
    ValidateOrder(order);
    _orderRepository.Save(order);
    _inventoryService.Reserve(order);
    _emailService.SendConfirmation(order.CustomerEmail, order);
}

public void ProcessPhoneOrder(Order order)
{
    ValidateOrder(order);
    _orderRepository.Save(order);
    _inventoryService.Reserve(order);
    _emailService.SendConfirmation(order.CustomerEmail, order);
    _callCenter.LogPhoneOrder(order);
}

// ✅ AFTER: Extracted common logic
public void ProcessOnlineOrder(Order order)
{
    ProcessOrderCore(order);
}

public void ProcessPhoneOrder(Order order)
{
    ProcessOrderCore(order);
    _callCenter.LogPhoneOrder(order);
}

private void ProcessOrderCore(Order order)
{
    ValidateOrder(order);
    _orderRepository.Save(order);
    _inventoryService.Reserve(order);
    _emailService.SendConfirmation(order.CustomerEmail, order);
}
```

---

## Code Smell Identification

### Method-Level Smells

#### Long Method (>50 lines)
**Detection:** Method exceeds 50 lines
**Solution:** Extract Method refactoring
**Priority:** High

#### Long Parameter List (>4 parameters)
**Detection:** Method has >4 parameters
**Solution:** Introduce Parameter Object
**Priority:** Medium

#### Complex Conditionals
**Detection:** Nested if/else >3 levels deep
**Solution:** Extract Method, Replace Conditional with Polymorphism
**Priority:** High

#### Dead Code
**Detection:** Unused methods, unreachable code
**Solution:** Delete unused code
**Priority:** Low (but do it!)

### Class-Level Smells

#### God Object (>500 lines)
**Detection:** Class exceeds 500 lines or has >10 public methods
**Solution:** Extract Class
**Priority:** High

#### Data Class
**Detection:** Class with only properties, no behavior
**Solution:** Move behavior to where data lives
**Priority:** Medium

#### Feature Envy
**Detection:** Method uses more methods/properties from another class than its own
**Solution:** Move Method to the envied class
**Priority:** Medium

#### Primitive Obsession
**Detection:** Using primitives instead of small objects (e.g., string for email addresses)
**Solution:** Introduce Value Objects
**Priority:** Low

### Code Structure Smells

#### Shotgun Surgery
**Detection:** Single change requires modifications in many classes
**Solution:** Move Method, Extract Class to consolidate related behavior
**Priority:** High

#### Divergent Change
**Detection:** Class changes for multiple unrelated reasons
**Solution:** Extract Class to separate concerns
**Priority:** High

---

## Refactoring Safety Protocol

### Always Keep Tests Green

**Critical Rule: Run tests after EVERY refactoring**

```
1. Run tests BEFORE refactoring → ✅ GREEN
2. Make ONE small refactoring change
3. Run tests IMMEDIATELY → Must stay ✅ GREEN
4. If tests fail: Revert and try different approach
5. If tests pass: Commit and continue
```

**Example Workflow:**
```bash
# 1. Verify tests pass before refactoring
dotnet test
# ✅ All tests pass

# 2. Make ONE refactoring (e.g., Extract Method)
# Use Edit tool to refactor code

# 3. Run tests immediately
dotnet test
# ✅ All tests still pass

# 4. Commit refactoring
git add src/Services/OrderService.cs
git commit -m "refactor: Extract CalculateDiscount method"

# 5. Repeat for next refactoring
```

### Invoke Light QA After Refactoring

**After completing refactorings in Phase 05:**

```
Skill(command="devforgeai-qa --mode=light --story={story_id}")
```

**Light QA validates:**
- Build succeeds
- All tests pass
- No anti-patterns introduced
- Code quality metrics acceptable

**HALT if violations detected**

---

## Refactoring Decision Trees

### When to Refactor vs When to Rewrite

**Change Impact Assessment:**

**< 30% of code needs change:**
- ✅ **Refactor** (incremental improvement)
- Keep existing structure
- Tests guide refactoring
- Lower risk

**30-70% of code needs change:**
- ⚠️ **Evaluate both approaches**
- Consider technical debt level
- Consider time constraints
- Consider team familiarity

**> 70% of code needs change:**
- ✅ **Rewrite** (from scratch with TDD)
- Existing code too constraining
- Tests define new implementation
- Clean slate opportunity

### When to Extract Method

**Decision Tree:**
```
Is method > 50 lines?
├─ YES → Extract Method
└─ NO → Continue...
    Are there commented sections?
    ├─ YES → Extract Method (comments indicate separate concerns)
    └─ NO → Continue...
        Is logic repeated elsewhere?
        ├─ YES → Extract Method
        └─ NO → Continue...
            Is conditional logic complex (>3 levels)?
            ├─ YES → Extract Method
            └─ NO → Keep as-is
```

### When to Extract Class

**Decision Tree:**
```
Is class > 500 lines?
├─ YES → Extract Class
└─ NO → Continue...
    Does class have >10 public methods?
    ├─ YES → Extract Class
    └─ NO → Continue...
        Are there natural groupings of methods?
        ├─ YES → Extract Class
        └─ NO → Continue...
            Do subsets of methods use different fields?
            ├─ YES → Extract Class
            └─ NO → Keep as-is
```

---

## Language-Specific Refactoring Patterns

### C# / .NET Refactoring

**IDE Tools:**
- Visual Studio: Ctrl+R, Ctrl+M (Extract Method)
- ReSharper: Alt+Enter (Quick fixes)
- Rider: Alt+Enter

**Common C# Refactorings:**
```csharp
// Extract Method
// Extract Interface
// Introduce Variable
// Inline Variable
// Change Signature
// Move Type to Matching File
```

**C#-Specific Patterns:**
- Use expression-bodied members for simple methods
- Use pattern matching instead of type checking
- Use null-coalescing operators (??, ??=)
- Use LINQ for collection operations

### Python Refactoring

**IDE Tools:**
- PyCharm: Ctrl+Alt+Shift+T (Refactor menu)
- VS Code: F2 (Rename), Ctrl+Shift+R (Extract Method)

**Common Python Refactorings:**
```python
# Extract method
# Extract variable
# Inline variable
# Rename symbol
# Move class/function
```

**Python-Specific Patterns:**
- Use list/dict comprehensions
- Use context managers (with statement)
- Use decorators for cross-cutting concerns
- Use type hints for better IDE support

### JavaScript/TypeScript Refactoring

**IDE Tools:**
- VS Code: F2 (Rename), Ctrl+Shift+R (Refactor menu)
- WebStorm: Ctrl+Alt+Shift+T (Refactor menu)

**Common JS/TS Refactorings:**
```typescript
// Extract function
// Extract constant
// Convert to arrow function
// Convert to async/await
// Destructure parameters
```

**TypeScript-Specific Patterns:**
- Add type annotations incrementally
- Use interfaces for contracts
- Use enums instead of magic strings
- Use optional chaining (?.)

---

## Refactoring Anti-Patterns

### Over-Engineering

**Problem:** Adding abstraction prematurely

**Example:**
```csharp
// ❌ FORBIDDEN: Premature abstraction
public interface IOrderValidator { }
public interface IOrderValidatorFactory { }
public interface IOrderValidatorProvider { }
public class CompositeOrderValidator : IOrderValidator { }
// ...for validating one simple field

// ✅ CORRECT: Simple validation for simple needs
public void ValidateOrder(Order order)
{
    if (order.Total < 0)
        throw new InvalidOperationException("Total cannot be negative");
}
```

**Rule:** Don't add abstraction until you have 3+ concrete cases (Rule of Three)

### Premature Optimization

**Problem:** Optimizing before measuring

**Example:**
```csharp
// ❌ FORBIDDEN: Complex optimization without profiling
public decimal CalculateTotal(List<OrderItem> items)
{
    // Parallel processing for 3 items? Really?
    return items.AsParallel().Sum(item => item.Price * item.Quantity);
}

// ✅ CORRECT: Simple, clear code (optimize if proven slow)
public decimal CalculateTotal(List<OrderItem> items)
{
    return items.Sum(item => item.Price * item.Quantity);
}
```

**Rule:** Measure first, optimize second (Donald Knuth: "Premature optimization is the root of all evil")

### Breaking Tests During Refactoring

**Problem:** Changing behavior, not just structure

**Example:**
```csharp
// ❌ FORBIDDEN: Changed behavior during refactoring
// Original: Throws exception
public void ValidateOrder(Order order)
{
    if (order.Total < 0)
        throw new InvalidOperationException("Negative total");
}

// Refactored (WRONG): Returns bool instead
public bool ValidateOrder(Order order)
{
    return order.Total >= 0; // TESTS NOW FAIL!
}

// ✅ CORRECT: Preserve behavior
public void ValidateOrder(Order order)
{
    if (order.Total < 0)
        throw new InvalidOperationException("Negative total");
    // Same behavior, maybe improved error message
}
```

---

## Refactoring Checklist

Before committing refactored code:

- [ ] All tests still pass (✅ GREEN)
- [ ] No behavior changes (same input → same output)
- [ ] Code is more readable than before
- [ ] Duplication removed (DRY principle followed)
- [ ] Methods have clear, single responsibilities
- [ ] Classes under 500 lines
- [ ] Methods under 50 lines
- [ ] No magic numbers (constants used)
- [ ] No code smells introduced
- [ ] Light QA validation passed
- [ ] Git commit with "refactor:" prefix created

---

## Quick Reference

### Refactoring Triggers

| Smell | Threshold | Refactoring Technique |
|-------|-----------|----------------------|
| Long Method | >50 lines | Extract Method |
| Long Parameter List | >4 params | Introduce Parameter Object |
| God Object | >500 lines or >10 methods | Extract Class |
| Duplication | Same code in 2+ places | Extract Method/Class |
| Complex Conditionals | >3 nesting levels | Extract Method, Polymorphism |
| Magic Numbers | Hardcoded values | Replace with Named Constants |
| Feature Envy | Uses other class more | Move Method |
| Data Class | Only properties, no behavior | Move behavior to class |

### Safety Protocol

1. ✅ Tests GREEN before starting
2. 🔨 Make ONE small refactoring
3. ▶️ Run tests immediately
4. ✅ Tests still GREEN? Commit
5. ❌ Tests RED? Revert, try different approach
6. 🔁 Repeat for next refactoring

### Tools by Language

**C# / .NET:** Visual Studio, ReSharper, Rider
**Python:** PyCharm, VS Code with Python extension
**JavaScript/TypeScript:** VS Code, WebStorm
**Java:** IntelliJ IDEA, Eclipse
**Go:** GoLand, VS Code with Go extension

---

This reference should be used during Phase 05 (Refactor) of the TDD workflow to systematically improve code quality while maintaining test integrity.