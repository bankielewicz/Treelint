# Test Coverage Analysis Reference (Consolidated)

**Consolidated:** coverage-analysis.md + coverage-analysis-workflow.md
**Token savings:** ~1.2K tokens (single load vs 2 separate loads)
**Version:** 2.0 (STORY-265)

---

## Overview

Test coverage measures how much of your source code is executed during testing. This guide provides comprehensive techniques for analyzing, improving, and maintaining test coverage.

---

## Phase 1: Coverage Analysis Workflow (7 Steps)

### Step 1: Load Coverage Thresholds

```
Read(file_path=".claude/skills/devforgeai-qa/assets/config/coverage-thresholds.md")

IF file not found:
    Use default strict thresholds:
    - Business Logic: 95%
    - Application: 85%
    - Infrastructure: 80%
    - Overall: 80%
```

### Step 2: Generate Coverage Reports (Story-Scoped)

```
# Get story-scoped paths from Phase 0.5
results_dir = test_isolation_paths.results_dir
coverage_dir = test_isolation_paths.coverage_dir

IF language == ".NET":
    Bash(command="dotnet test --collect:'XPlat Code Coverage' --results-directory={results_dir}")
    coverage_file = "{results_dir}/*/coverage.cobertura.xml"

IF language == "Python":
    Bash(command="pytest --cov=src --cov-report=json:{coverage_dir}/coverage.json")
    coverage_file = "{coverage_dir}/coverage.json"

IF language == "Node.js":
    Bash(command="npm test -- --coverage --coverageDirectory={coverage_dir}")
    coverage_file = "{coverage_dir}/coverage-summary.json"

IF language == "Go":
    Bash(command="go test ./... -coverprofile={coverage_dir}/coverage.out")
    coverage_file = "{coverage_dir}/coverage.out"

IF language == "Rust":
    Bash(command="cargo tarpaulin --out Json --output-dir {coverage_dir}")
    coverage_file = "{coverage_dir}/tarpaulin-report.json"

IF language == "Java":
    Bash(command="mvn test jacoco:report -Djacoco.destFile={coverage_dir}/jacoco.exec")
    coverage_file = "{coverage_dir}/jacoco.xml"
```

### Step 3: Classify Files by Layer

```
Read(file_path="devforgeai/specs/context/source-tree.md")

Layer patterns (from source-tree):
- Business Logic: src/domain/*, src/core/*, src/services/*
- Application: src/api/*, src/controllers/*, src/handlers/*
- Infrastructure: src/data/*, src/repositories/*, src/external/*
```

### Step 4: Calculate Coverage by Layer

```
FOR each file in coverage_report:
    layer = classify_file(file, source_tree_patterns)
    layer_coverage[layer].add(file.coverage)

Calculate: business_avg, application_avg, infrastructure_avg, overall_avg
```

### Step 5: Validate Against Thresholds

```
IF business_logic_coverage < 95%: CRITICAL violation
IF application_coverage < 85%: CRITICAL violation
IF infrastructure_coverage < 80%: HIGH violation
IF overall_coverage < 80%: CRITICAL violation
```

### Step 6: Identify Coverage Gaps

```
FOR each uncovered_block:
    test_suggestion = {
        file, function, lines,
        suggested_test: generate_test_name(),
        priority: HIGH (business) | MEDIUM (app) | LOW (infra)
    }
```

### Step 7: Analyze Test Quality

- Assertion ratio (target: ≥1.5 per test)
- Over-mocking detection (mocks > tests * 2)
- Test pyramid validation (70% unit, 20% integration, 10% E2E)

---

## Coverage Metrics Explained

### Line Coverage (Statement Coverage)

**Definition:** Percentage of code lines executed during tests

**Example:**
```csharp
public decimal CalculateDiscount(Order order, Coupon coupon)
{
    if (coupon.IsExpired())          // Line 1 - Executed?
    {
        return order.Total;          // Line 2 - Executed?
    }

    var discount = order.Total * coupon.Percentage;  // Line 3 - Executed?
    return order.Total - discount;   // Line 4 - Executed?
}
```

**Coverage Calculation:**
- Test executes lines 1, 3, 4 → 75% line coverage (3/4 lines)
- Missing: Line 2 (expired coupon path)

**Limitation:** Can achieve 100% line coverage without testing all scenarios

### Branch Coverage (Decision Coverage)

**Definition:** Percentage of decision branches (if/else, switch) executed

**Example:**
```csharp
if (coupon.IsExpired())     // Branch point
{
    return order.Total;     // Branch A
}
else
{
    return ApplyDiscount(); // Branch B
}
```

**Coverage Calculation:**
- Test A: coupon expired → Covers Branch A only → 50% branch coverage
- Test B: coupon valid → Covers Branch B only → 50% branch coverage
- Both tests: 100% branch coverage

**Better than line coverage:** Ensures both paths tested

### Path Coverage

**Definition:** Percentage of all possible execution paths tested

**Example:**
```csharp
public Result ValidateOrder(Order order)
{
    if (order == null) return Error("Null order");        // Decision 1
    if (order.Total <= 0) return Error("Invalid total");  // Decision 2
    if (order.Items.Empty) return Error("No items");      // Decision 3
    return Success();
}
```

**Possible Paths:** 2³ = 8 paths
1. null → Error
2. not null, total ≤ 0 → Error
3. not null, total > 0, no items → Error
4. not null, total > 0, has items → Success
5-8. Various combinations

**Path coverage:** 4/8 = 50% (if only testing the 4 above)

**Challenge:** Path count grows exponentially (loops, nested conditions)

### Condition Coverage (Multiple Condition Coverage)

**Definition:** Tests all combinations of boolean sub-expressions

**Example:**
```csharp
if (user.IsActive && user.HasPermission("admin"))
{
    GrantAccess();
}
```

**Condition Coverage Tests:**
1. IsActive = true, HasPermission = true → Condition 1 ✓
2. IsActive = true, HasPermission = false → Condition 2 ✓
3. IsActive = false, HasPermission = true → Condition 3 ✓
4. IsActive = false, HasPermission = false → Condition 4 ✓

**100% condition coverage:** All 4 combinations tested

## Coverage Tools by Language

### .NET / C#

**Primary Tools:**
1. **coverlet** (Open source, cross-platform)
   ```bash
   dotnet test --collect:"XPlat Code Coverage"
   ```

2. **dotnet-coverage** (Microsoft official)
   ```bash
   dotnet-coverage collect "dotnet test" -f xml -o coverage.xml
   ```

3. **Fine Code Coverage** (Visual Studio extension)
   - Real-time coverage highlighting
   - Line-by-line coverage visualization

**Coverage Report Formats:**
- Cobertura XML (standard)
- OpenCover XML
- JSON
- HTML (via ReportGenerator)

**Generate HTML Report:**
```bash
dotnet test --collect:"XPlat Code Coverage"
reportgenerator -reports:coverage.cobertura.xml -targetdir:coverage-report -reporttypes:Html
```

### Python

**Primary Tool:** coverage.py (pytest-cov wrapper)

```bash
# Run tests with coverage
pytest --cov=src --cov-report=html --cov-report=term --cov-report=json

# Coverage configuration (.coveragerc)
[run]
source = src
omit = */tests/*, */migrations/*

[report]
exclude_lines =
    pragma: no cover
    def __repr__
    raise AssertionError
    raise NotImplementedError
```

**Coverage Formats:**
- Terminal report (quick overview)
- HTML report (detailed, interactive)
- JSON (machine-readable)
- XML (Cobertura for CI/CD)

### JavaScript / TypeScript

**Primary Tools:**
1. **Istanbul (nyc)** - Most popular
   ```bash
   npm test -- --coverage
   # OR
   nyc --reporter=html --reporter=text mocha tests/
   ```

2. **c8** - Modern V8 coverage
   ```bash
   c8 npm test
   ```

**Jest Configuration:**
```json
{
  "jest": {
    "collectCoverage": true,
    "coverageDirectory": "coverage",
    "coverageReporters": ["json", "html", "text"],
    "coverageThreshold": {
      "global": {
        "branches": 80,
        "functions": 80,
        "lines": 80,
        "statements": 80
      }
    }
  }
}
```

### Java

**Primary Tool:** JaCoCo (Java Code Coverage)

```xml
<!-- Maven pom.xml -->
<plugin>
    <groupId>org.jacoco</groupId>
    <artifactId>jacoco-maven-plugin</artifactId>
    <version>0.8.10</version>
    <executions>
        <execution>
            <goals>
                <goal>prepare-agent</goal>
            </goals>
        </execution>
        <execution>
            <id>report</id>
            <phase>test</phase>
            <goals>
                <goal>report</goal>
            </goals>
        </execution>
    </executions>
</plugin>
```

## Analyzing Coverage Reports

### Parsing coverage.json

**Structure (Cobertura format):**
```json
{
  "line-rate": 0.85,
  "branch-rate": 0.78,
  "lines-covered": 850,
  "lines-valid": 1000,
  "branches-covered": 156,
  "branches-valid": 200,
  "packages": [
    {
      "name": "src.Services",
      "line-rate": 0.82,
      "classes": [
        {
          "name": "OrderService",
          "filename": "src/Services/OrderService.cs",
          "line-rate": 0.75,
          "lines": [
            {"number": 42, "hits": 10},
            {"number": 43, "hits": 0},
            {"number": 44, "hits": 8}
          ]
        }
      ]
    }
  ]
}
```

**Extracting Coverage by Layer:**
```python
import json

def analyze_coverage_by_layer(coverage_file):
    with open(coverage_file) as f:
        data = json.load(f)

    layers = {
        "Domain": {"covered": 0, "total": 0},
        "Application": {"covered": 0, "total": 0},
        "Infrastructure": {"covered": 0, "total": 0}
    }

    for package in data["packages"]:
        # Determine layer from package name
        if "Domain" in package["name"]:
            layer = "Domain"
        elif "Application" in package["name"]:
            layer = "Application"
        elif "Infrastructure" in package["name"]:
            layer = "Infrastructure"
        else:
            continue

        # Aggregate coverage
        for cls in package["classes"]:
            total_lines = len(cls["lines"])
            covered_lines = sum(1 for line in cls["lines"] if line["hits"] > 0)

            layers[layer]["total"] += total_lines
            layers[layer]["covered"] += covered_lines

    # Calculate percentages
    for layer in layers:
        if layers[layer]["total"] > 0:
            coverage = (layers[layer]["covered"] / layers[layer]["total"]) * 100
            print(f"{layer}: {coverage:.2f}%")
```

### Identifying Untested Code Paths

**Algorithm:**
1. Parse coverage data for files with < threshold coverage
2. For each file, extract uncovered line ranges
3. Read source file
4. Map line ranges to methods/functions
5. Generate test suggestions

**Implementation:**
```python
def identify_untested_code(coverage_data, threshold=80):
    gaps = []

    for file_data in coverage_data["files"]:
        coverage_pct = file_data["line_coverage"]

        if coverage_pct < threshold:
            # Extract uncovered line ranges
            uncovered_ranges = []
            start = None

            for line in file_data["lines"]:
                if line["hits"] == 0:
                    if start is None:
                        start = line["number"]
                    end = line["number"]
                else:
                    if start is not None:
                        uncovered_ranges.append((start, end))
                        start = None

            # Map to methods
            methods = extract_methods_from_lines(file_data["filename"], uncovered_ranges)

            gaps.append({
                "file": file_data["filename"],
                "coverage": coverage_pct,
                "uncovered_lines": uncovered_ranges,
                "uncovered_methods": methods,
                "suggested_tests": generate_test_names(methods)
            })

    return gaps
```

### Prioritizing Coverage Gaps

**Priority Matrix:**

| Coverage | Layer | Priority | Action |
|----------|-------|----------|--------|
| < 95% | Business Logic | CRITICAL | Immediate tests required |
| < 85% | Application | HIGH | Add tests in current sprint |
| < 80% | Infrastructure | MEDIUM | Add tests when touching code |
| < 60% | Any | CRITICAL | Major gap, comprehensive testing needed |

**Prioritization Algorithm:**
```python
def prioritize_gaps(gaps, source_tree_md):
    # Load layer definitions
    layers = parse_source_tree(source_tree_md)

    for gap in gaps:
        # Determine layer
        layer = get_layer(gap["file"], layers)

        # Assign priority
        if layer == "Domain" or "Service" in gap["file"]:
            if gap["coverage"] < 95:
                gap["priority"] = "CRITICAL"
        elif layer == "Application":
            if gap["coverage"] < 85:
                gap["priority"] = "HIGH"
        elif layer == "Infrastructure":
            if gap["coverage"] < 80:
                gap["priority"] = "MEDIUM"

        # Check for security-critical code
        if any(keyword in gap["file"].lower() for keyword in ["auth", "security", "crypto"]):
            gap["priority"] = "CRITICAL"

    return sorted(gaps, key=lambda x: priority_order[x["priority"]])
```

## Coverage Improvement Strategies

### Strategy 1: Add Missing Unit Tests

**Identify testable units:**
```python
def find_untested_methods(file_path, coverage_data):
    # Parse source file for methods
    methods = extract_methods(file_path)

    # Check coverage for each method
    untested = []
    for method in methods:
        lines = range(method.start_line, method.end_line)
        covered_lines = [l for l in lines if coverage_data.is_covered(l)]

        coverage_pct = len(covered_lines) / len(lines) * 100

        if coverage_pct < 80:
            untested.append({
                "method": method.name,
                "coverage": coverage_pct,
                "complexity": calculate_complexity(method),
                "test_suggestion": generate_test_name(method)
            })

    return untested
```

**Generate test suggestions:**
```csharp
// Untested method
public decimal CalculateShipping(Order order, Address address)
{
    if (order.Total > 100) return 0m;  // Free shipping
    if (address.IsInternational) return 25m;
    return 10m;
}

// Suggested tests:
// 1. CalculateShipping_OrderOver100_ReturnsFreeShipping
// 2. CalculateShipping_InternationalAddress_Returns25
// 3. CalculateShipping_DomesticUnder100_Returns10
```

### Strategy 2: Add Edge Case Tests

**Common edge cases:**
- Null inputs
- Empty collections
- Boundary values (0, -1, MAX_VALUE)
- Concurrent access
- Expired/invalid data

**Example:**
```csharp
// Original test (happy path only)
[Fact]
public void CalculateDiscount_ValidCoupon_ReturnsDiscount()
{
    var result = service.CalculateDiscount(order, coupon);
    Assert.Equal(90m, result);
}

// Add edge case tests
[Fact]
public void CalculateDiscount_NullOrder_ThrowsException()
{
    Assert.Throws<ArgumentNullException>(() =>
        service.CalculateDiscount(null, coupon));
}

[Fact]
public void CalculateDiscount_ZeroTotal_ReturnsZero()
{
    order.Total = 0m;
    var result = service.CalculateDiscount(order, coupon);
    Assert.Equal(0m, result);
}

[Fact]
public void CalculateDiscount_ExpiredCoupon_ReturnsOriginalPrice()
{
    coupon.ExpiryDate = DateTime.Now.AddDays(-1);
    var result = service.CalculateDiscount(order, coupon);
    Assert.Equal(order.Total, result);
}
```

### Strategy 3: Add Integration Tests

**When unit tests aren't enough:**
- Database interactions
- External API calls
- File I/O operations
- Multi-component workflows

**Example (Dapper repository):**
```csharp
public class OrderRepositoryIntegrationTests : IDisposable
{
    private readonly SqlConnection _connection;

    public OrderRepositoryIntegrationTests()
    {
        _connection = new SqlConnection(TestConfig.ConnectionString);
        _connection.Open();
        SeedTestData();
    }

    [Fact]
    public async Task GetByIdAsync_ExistingOrder_ReturnsOrder()
    {
        // This tests actual database interaction
        var repository = new OrderRepository(_connection);
        var orderId = 1;

        var order = await repository.GetByIdAsync(orderId);

        Assert.NotNull(order);
        Assert.Equal(orderId, order.Id);
        // Coverage now includes Dapper query execution
    }

    public void Dispose()
    {
        CleanupTestData();
        _connection.Dispose();
    }
}
```

### Strategy 4: Balance Test Pyramid

**Target Distribution:**
- Unit tests: 70%
- Integration tests: 20%
- E2E tests: 10%

**Rebalancing Example:**

Before (inverted pyramid):
- Unit: 30% (slow feedback)
- Integration: 20%
- E2E: 50% (brittle, slow)

After rebalancing:
1. Convert E2E tests to unit tests where possible
   - E2E: "User can add to cart" → Unit: "AddToCart method works"
   - E2E: "User can checkout" → Unit: "ProcessCheckout handles payment"

2. Keep E2E for critical user journeys only
   - Complete purchase flow
   - Authentication flow
   - Critical business workflows

Result:
- Unit: 70% (fast, reliable)
- Integration: 20% (validates integrations)
- E2E: 10% (validates critical paths)

## Test Quality Assessment

### Assertion Count Analysis

**Goal:** Ensure tests validate behavior (not just execute code)

```python
def analyze_test_quality(test_file):
    test_count = count_test_methods(test_file)
    assertion_count = count_assertions(test_file)

    assertions_per_test = assertion_count / test_count

    if assertions_per_test < 1.5:
        return {
            "quality": "LOW",
            "issue": "Low assertion count",
            "avg_assertions": assertions_per_test,
            "recommendation": "Tests may not be validating enough"
        }
    elif assertions_per_test > 5:
        return {
            "quality": "MEDIUM",
            "issue": "High assertion count",
            "avg_assertions": assertions_per_test,
            "recommendation": "Tests may be testing too much (split into smaller tests)"
        }
    else:
        return {
            "quality": "GOOD",
            "avg_assertions": assertions_per_test
        }
```

### Over-Mocking Detection

**Problem:** Tests with excessive mocks may test mocks instead of behavior

```csharp
// ❌ Over-mocked test (testing mocks, not real behavior)
[Fact]
public void ProcessOrder_Always_CallsDependencies()
{
    var mockRepo = new Mock<IOrderRepository>();
    var mockEmail = new Mock<IEmailService>();
    var mockPayment = new Mock<IPaymentService>();
    var mockInventory = new Mock<IInventoryService>();
    var mockShipping = new Mock<IShippingService>();
    var mockLogging = new Mock<ILogger>();

    var service = new OrderService(
        mockRepo.Object,
        mockEmail.Object,
        mockPayment.Object,
        mockInventory.Object,
        mockShipping.Object,
        mockLogging.Object
    );

    service.ProcessOrder(order);

    // Just verifying mocks were called, not actual business logic
    mockRepo.Verify(x => x.Save(It.IsAny<Order>()), Times.Once);
    mockEmail.Verify(x => x.Send(It.IsAny<Email>()), Times.Once);
    // ... more mock verifications
}

// ✅ Better: Test behavior with minimal mocking
[Fact]
public void ProcessOrder_ValidOrder_SavesAndReturnsResult()
{
    // Only mock external dependencies
    var mockPayment = new Mock<IPaymentService>();
    mockPayment.Setup(x => x.Process(It.IsAny<Payment>()))
              .Returns(new PaymentResult { Success = true });

    // Use real objects for internal logic
    var repository = new InMemoryOrderRepository();
    var service = new OrderService(repository, mockPayment.Object);

    var result = service.ProcessOrder(order);

    // Verify actual behavior
    Assert.True(result.IsSuccess);
    Assert.NotNull(repository.GetById(result.OrderId));
}
```

**Detection algorithm:**
```python
def detect_over_mocking(test_file):
    test_methods = extract_test_methods(test_file)

    for test in test_methods:
        mock_count = count_mocks(test)
        assertion_count = count_assertions(test)

        if mock_count > 5:
            warnings.append({
                "test": test.name,
                "issue": "Excessive mocking",
                "mocks": mock_count,
                "recommendation": "Consider testing real behavior instead of mocking everything"
            })

        if mock_count > assertion_count * 2:
            warnings.append({
                "test": test.name,
                "issue": "More mocks than assertions",
                "recommendation": "Test may be verifying mocks instead of behavior"
            })
```

## Coverage Anti-Patterns

### Anti-Pattern 1: Testing for Metrics (Not Behavior)

**Problem:** Writing trivial tests just to increase coverage percentage

```csharp
// ❌ BAD: Trivial test that adds no value
[Fact]
public void Order_SetTotal_SetsTotal()
{
    var order = new Order();
    order.Total = 100m;
    Assert.Equal(100m, order.Total);  // Testing a property getter/setter
}

// ✅ GOOD: Test actual behavior
[Fact]
public void Order_AddItem_UpdatesTotalCorrectly()
{
    var order = new Order();
    order.AddItem(new OrderItem { Price = 50m, Quantity = 2 });
    Assert.Equal(100m, order.Total);  // Testing business logic
}
```

### Anti-Pattern 2: Ignoring Complex Branches

**Problem:** Achieving high line coverage but missing complex paths

```csharp
public Result ValidatePayment(Payment payment)
{
    if (payment == null) return Error("Null payment");
    if (payment.Amount <= 0) return Error("Invalid amount");

    if (payment.Method == PaymentMethod.CreditCard)
    {
        if (!ValidateCreditCard(payment.CardNumber))
            return Error("Invalid card");

        if (payment.Amount > 10000 && !payment.IsVerified)
            return Error("Large payment requires verification");
    }

    return Success();
}

// ❌ BAD: Only test happy path (50% branch coverage)
[Fact]
public void ValidatePayment_ValidCard_ReturnsSuccess()
{
    var payment = new Payment
    {
        Method = PaymentMethod.CreditCard,
        Amount = 100,
        CardNumber = "4111111111111111"
    };

    var result = ValidatePayment(payment);

    Assert.True(result.IsSuccess);
}

// ✅ GOOD: Test all branches
[Theory]
[InlineData(null, "Null payment")]
[InlineData(-1, "Invalid amount")]
[InlineData(0, "Invalid amount")]
public void ValidatePayment_InvalidInput_ReturnsError(decimal? amount, string expectedError)
{
    var payment = amount.HasValue ? new Payment { Amount = amount.Value } : null;
    var result = ValidatePayment(payment);
    Assert.Equal(expectedError, result.Error);
}

[Fact]
public void ValidatePayment_InvalidCard_ReturnsError()
{
    var payment = new Payment { Method = PaymentMethod.CreditCard, CardNumber = "1234" };
    var result = ValidatePayment(payment);
    Assert.Contains("Invalid card", result.Error);
}

[Fact]
public void ValidatePayment_LargeUnverifiedPayment_ReturnsError()
{
    var payment = new Payment
    {
        Method = PaymentMethod.CreditCard,
        Amount = 15000,
        IsVerified = false,
        CardNumber = "4111111111111111"
    };

    var result = ValidatePayment(payment);
    Assert.Contains("verification", result.Error);
}
```

### Anti-Pattern 3: Testing Implementation Details

**Problem:** Tests break when refactoring (brittle tests)

```csharp
// ❌ BAD: Testing private implementation
[Fact]
public void ProcessOrder_Always_CallsInternalValidation()
{
    var service = new OrderService();
    var order = new Order();

    // Using reflection to test private method
    var method = typeof(OrderService).GetMethod("ValidateOrder",
        BindingFlags.NonPublic | BindingFlags.Instance);
    var result = method.Invoke(service, new[] { order });

    Assert.NotNull(result);  // Breaks when refactoring
}

// ✅ GOOD: Test public behavior
[Fact]
public void ProcessOrder_InvalidOrder_ReturnsError()
{
    var service = new OrderService();
    var order = new Order { Total = -10m };  // Invalid

    var result = service.ProcessOrder(order);

    Assert.False(result.IsSuccess);
    Assert.Contains("Invalid", result.Error);
}
```

## Best Practices

### 1. Set Appropriate Thresholds

```yaml
coverage_thresholds:
  business_logic: 95%    # Critical code, high threshold
  application: 85%       # Application logic
  infrastructure: 80%    # Infrastructure code
  ui_components: 70%     # UI (harder to test)
  legacy_code: 60%       # Incremental improvement
```

### 2. Focus on Critical Paths

Prioritize coverage for:
- Business logic (domain services)
- Financial calculations
- Security/authentication
- Data validation
- Error handling

### 3. Use Coverage as a Guide, Not a Goal

- 100% coverage doesn't mean bug-free
- Focus on meaningful tests, not coverage percentage
- Some code may not need tests (simple getters/setters, DTOs)

### 4. Integrate into CI/CD

```yaml
# .github/workflows/test.yml
- name: Run tests with coverage
  run: dotnet test --collect:"XPlat Code Coverage"

- name: Check coverage threshold
  run: |
    coverage=$(jq '.summary.lineCoverage' coverage.json)
    if (( $(echo "$coverage < 80" | bc -l) )); then
      echo "Coverage $coverage% is below 80% threshold"
      exit 1
    fi
```

### 5. Review Coverage in Code Reviews

- Require new code to have 80%+ coverage
- Flag decreases in overall coverage
- Discuss uncovered critical paths

## Quick Reference

### Common Coverage Commands

```bash
# .NET
dotnet test --collect:"XPlat Code Coverage"
reportgenerator -reports:coverage.cobertura.xml -targetdir:report

# Python
pytest --cov=src --cov-report=html

# JavaScript
npm test -- --coverage

# Java
mvn test jacoco:report
```

### Coverage File Locations

```
.NET: TestResults/*/coverage.cobertura.xml
Python: .coverage, htmlcov/
JavaScript: coverage/
Java: target/site/jacoco/
```

### Interpreting Coverage Percentages

| Coverage | Interpretation | Action |
|----------|----------------|--------|
| 95%+ | Excellent | Maintain |
| 85-95% | Very Good | Minor improvements |
| 80-85% | Good | Target met |
| 70-80% | Fair | Add tests for critical paths |
| < 70% | Poor | Significant testing needed |

This reference should be loaded when performing comprehensive coverage analysis during deep QA validation.
