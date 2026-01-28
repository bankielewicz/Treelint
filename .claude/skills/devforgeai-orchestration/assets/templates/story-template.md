---
id: STORY-XXX
title: [Story Title - What is being built]
epic: EPIC-XXX
sprint: SPRINT-XXX
status: Backlog
points: [Story points: 1, 2, 3, 5, 8, 13]
priority: [High / Medium / Low]
assigned_to: [Developer Name]
created: YYYY-MM-DD
format_version: "2.0"
---

# Story: [Title]

## Description

**As a** [user role/persona],
**I want** [capability/feature],
**so that** [business value/benefit].

**Example:**
As a returning customer, I want to use my saved payment method during checkout, so that I can complete purchases faster without re-entering card details.

## Acceptance Criteria

Define testable, specific conditions that must be met for story completion. Use Given/When/Then format for clarity.

### 1. [ ] [Criterion 1 Title]

**Given** [initial context/state],
**When** [action/event occurs],
**Then** [expected outcome].

**Example:**
- **Given** a returning user with saved payment method
- **When** the user reaches payment step in checkout
- **Then** saved payment method is displayed and selectable

---

### 2. [ ] [Criterion 2 Title]

**Given** [initial context/state],
**When** [action/event occurs],
**Then** [expected outcome].

---

### 3. [ ] [Criterion 3 Title]

**Given** [initial context/state],
**When** [action/event occurs],
**Then** [expected outcome].

---

### 4. [ ] [Criterion 4 Title]

**Given** [initial context/state],
**When** [action/event occurs],
**Then** [expected outcome].

---

*Add more criteria as needed (typically 3-7 per story)*

## Technical Specification

**Format Version:** 2.0 (Structured YAML)

Define all technical implementation details using structured YAML format for machine-readable parsing and automated validation.

```yaml
technical_specification:
  format_version: "2.0"

  components:
    # Service Component Example
    - type: "Service"
      name: "[ServiceName]"
      file_path: "src/[path]/[ServiceName].cs"
      interface: "[IServiceInterface]"
      lifecycle: "Singleton|Scoped|Transient"
      dependencies:
        - "[IDependency1]"
        - "[IDependency2]"
      requirements:
        - id: "SVC-001"
          description: "[What this service must do]"
          testable: true
          test_requirement: "Test: [Specific test for this requirement]"
          priority: "Critical|High|Medium|Low"

    # Worker Component Example
    - type: "Worker"
      name: "[WorkerName]"
      file_path: "src/[path]/[WorkerName].cs"
      interface: "BackgroundService|IHostedService"
      polling_interval_ms: 30000
      dependencies:
        - "[IDependency]"
      requirements:
        - id: "WKR-001"
          description: "[What this worker must do]"
          testable: true
          test_requirement: "Test: [Specific test for this requirement]"
          priority: "Critical|High|Medium|Low"

    # Configuration Component Example
    - type: "Configuration"
      name: "appsettings.json"
      file_path: "src/[path]/appsettings.json"
      required_keys:
        - key: "[ConfigSection.KeyName]"
          type: "string|int|bool|array|object"
          example: "[Example value]"
          required: true|false
          default: "[Default value if any]"
          validation: "[Validation rules]"
          test_requirement: "Test: [How to verify this config loads]"

    # Logging Component Example
    - type: "Logging"
      name: "Serilog|NLog|log4net"
      file_path: "src/[path]/Program.cs"
      sinks:
        - name: "File|EventLog|Database|Console"
          path: "[Log file path]"
          test_requirement: "Test: [How to verify this sink works]"

    # Repository Component Example
    - type: "Repository"
      name: "[RepositoryName]"
      file_path: "src/Infrastructure/Repositories/[RepositoryName].cs"
      interface: "[IRepositoryInterface]"
      data_access: "Dapper|EF Core|Prisma|Raw SQL"
      entity: "[EntityName]"
      table: "[dbo].[TableName]"
      requirements:
        - id: "REPO-001"
          description: "[What this repository must do]"
          testable: true
          test_requirement: "Test: [Specific test for this requirement]"
          priority: "Critical|High|Medium|Low"

    # API Endpoint Component Example
    - type: "API"
      name: "[EndpointName]"
      endpoint: "/api/[resource]/[action]"
      method: "GET|POST|PUT|PATCH|DELETE"
      authentication:
        required: true|false
        method: "Bearer Token|API Key|OAuth2"
        scopes: ["scope1", "scope2"]
      request:
        content_type: "application/json"
        schema:
          field1:
            type: "string|number|boolean|object|array"
            required: true|false
            validation: "[Validation rules]"
      response:
        success:
          status_code: 200|201|204
          schema:
            id: "UUID"
            field: "string"
        errors:
          - status_code: 400|401|403|404|422|500
            condition: "[When this error occurs]"
            schema:
              error: "string"
              message: "string"
      requirements:
        - id: "API-001"
          description: "[What this API must do]"
          testable: true
          test_requirement: "Test: [Specific test for this requirement]"
          priority: "Critical|High|Medium|Low"

    # DataModel Component Example
    - type: "DataModel"
      name: "[EntityName]"
      table: "[dbo].[TableName]"
      purpose: "[What this model represents]"
      fields:
        - name: "[FieldName]"
          type: "UUID|String|Int|DateTime|Enum|etc"
          constraints: "Primary Key|Required|Unique|etc"
          description: "[Purpose of this field]"
          test_requirement: "Test: [How to validate this field]"
      indexes:
        - name: "IX_[Table]_[Field]"
          fields: ["[Field1]", "[Field2]"]
          unique: true|false
          purpose: "[Why this index exists]"
      relationships:
        - type: "One-to-Many|Many-to-One|Many-to-Many"
          related_entity: "[RelatedEntity]"
          foreign_key: "[FK_Field]"
          cascade: "Cascade|Restrict|SetNull"
          description: "[Relationship purpose]"

  business_rules:
    - id: "BR-001"
      rule: "[Business rule description]"
      trigger: "[When this rule is evaluated]"
      validation: "[How to validate compliance]"
      error_handling: "[What happens if violated]"
      test_requirement: "Test: [How to test this rule]"
      priority: "Critical|High|Medium|Low"

  non_functional_requirements:
    - id: "NFR-001"
      category: "Performance|Security|Scalability|Reliability"
      requirement: "[NFR description]"
      metric: "[Measurable target with numbers]"
      test_requirement: "Test: [How to verify this NFR]"
      priority: "Critical|High|Medium|Low"
```

**Instructions for AI Story Creation:**
1. Use appropriate component types based on what's being built
2. Every requirement/key/sink/field should have a `test_requirement`
3. IDs should be unique and follow pattern: SVC-001, WKR-001, API-001, etc.
4. Priorities: Critical (must have), High (should have), Medium (nice to have), Low (optional)
5. All test requirements start with "Test: " prefix
6. Metrics must be measurable (include numbers, thresholds, or ranges)

**See `devforgeai/specs/STRUCTURED-FORMAT-SPECIFICATION.md` for complete schema reference and examples.**

---

## Non-Functional Requirements (NFRs)

### Performance

**Response Time:**
- **API Endpoint 1:** < [X]ms (p95), < [X]ms (p99)
- **API Endpoint 2:** < [X]ms (p95), < [X]ms (p99)

**Throughput:**
- Support [X] requests per second
- Support [X] concurrent users

**Performance Test:**
- Load test with [X] concurrent users
- Verify response time under load
- Verify no memory leaks over [X] hour run

---

### Security

**Authentication:**
- [Required / Optional / None]
- [Auth method: OAuth 2.0, JWT, API Key, etc.]

**Authorization:**
- [Role-based / Permission-based / None]
- Required roles: [Roles that can access this feature]

**Data Protection:**
- Sensitive fields: [List fields requiring encryption/masking]
- Encryption: [At rest / In transit / Both]
- PII handling: [GDPR/CCPA compliance requirements]

**Security Testing:**
- [ ] No SQL injection vulnerabilities
- [ ] No XSS vulnerabilities
- [ ] No hardcoded secrets
- [ ] Proper input validation
- [ ] Proper output encoding
- [ ] Authentication enforced
- [ ] Authorization enforced

**Rate Limiting:**
- [X] requests per [time period] per [IP / user / API key]

---

### Scalability

**Horizontal Scaling:**
- Stateless design: [Yes / No]
- Load balancing: [Required / Not Required]

**Database:**
- Expected data volume: [X] records per [time period]
- Growth rate: [X]% per [time period]
- Indexing strategy: [Described above in Data Models]

**Caching:**
- Cache strategy: [None / Redis / In-Memory]
- Cache TTL: [X] seconds/minutes
- Cache invalidation: [Strategy]

---

### Reliability

**Error Handling:**
- Follow Result Pattern (per coding-standards.md)
- Log all errors with context
- Return user-friendly error messages (no stack traces)

**Retry Logic:**
- Retry transient failures: [Yes / No]
- Max retries: [X]
- Backoff strategy: [Exponential / Linear / Fixed]

**Monitoring:**
- Metrics to track: [List key metrics]
- Alerts: [When to alert, who receives alerts]

---

### Observability

**Logging:**
- Log level: [INFO / DEBUG / WARN / ERROR]
- Log structured data (JSON format)
- Include correlation ID for request tracing
- Do NOT log sensitive data (passwords, tokens, PII)

**Metrics:**
- Request count
- Response time (p50, p95, p99)
- Error rate
- [Custom metric 1]
- [Custom metric 2]

**Tracing:**
- Distributed tracing: [Yes / No]
- Trace all external calls

---

## Dependencies

### Prerequisite Stories

Stories that must complete BEFORE this story can start:

- [ ] **STORY-XXX:** [Story title]
  - **Why:** [Explanation of dependency]
  - **Status:** [Not Started / In Progress / Complete]

- [ ] **STORY-YYY:** [Story title]
  - **Why:** [Explanation of dependency]
  - **Status:** [Not Started / In Progress / Complete]

### External Dependencies

Dependencies outside the team's control:

- [ ] **External Dependency 1:** [Description]
  - **Owner:** [Team/Vendor]
  - **ETA:** [Date]
  - **Status:** [On Track / At Risk / Blocked]
  - **Impact if delayed:** [Description]

- [ ] **External Dependency 2:** [Description]
  - **Owner:** [Team/Vendor]
  - **ETA:** [Date]
  - **Status:** [On Track / At Risk / Blocked]

### Technology Dependencies

New packages or versions required:

- [ ] **Package 1:** [Name] v[Version]
  - **Purpose:** [Why needed]
  - **Approved:** [Yes / Pending]
  - **Added to dependencies.md:** [Yes / No]

- [ ] **Package 2:** [Name] v[Version]
  - **Purpose:** [Why needed]
  - **Approved:** [Yes / Pending]

---

## Test Strategy

### Unit Tests

**Coverage Target:** 95%+ for business logic

**Test Scenarios:**
1. **Happy Path:** [Description of normal flow]
2. **Edge Cases:**
   - [Edge case 1]
   - [Edge case 2]
   - [Edge case 3]
3. **Error Cases:**
   - [Error case 1: null input]
   - [Error case 2: invalid input]
   - [Error case 3: business rule violation]

**Example Test Structure:**
```csharp
public class ServiceNameTests
{
    [Fact]
    public void MethodName_ValidInput_ReturnsExpectedResult()
    {
        // Arrange
        var sut = new ServiceName();
        var input = CreateValidInput();

        // Act
        var result = sut.MethodName(input);

        // Assert
        Assert.True(result.IsSuccess);
        Assert.Equal(expectedValue, result.Value);
    }

    [Fact]
    public void MethodName_NullInput_ReturnsError()
    {
        // Arrange
        var sut = new ServiceName();

        // Act
        var result = sut.MethodName(null);

        // Assert
        Assert.False(result.IsSuccess);
        Assert.Equal("Input cannot be null", result.Error);
    }
}
```

---

### Integration Tests

**Coverage Target:** 85%+ for application layer

**Test Scenarios:**
1. **End-to-End API Flow:** [Description]
2. **Database Integration:** [Description]
3. **External Service Integration:** [Description]

**Example Test:**
```csharp
[Fact]
public async Task PostEndpoint_ValidRequest_CreatesResource()
{
    // Arrange
    var client = _factory.CreateClient();
    var request = new RequestDTO { /* ... */ };

    // Act
    var response = await client.PostAsJsonAsync("/api/endpoint", request);

    // Assert
    response.EnsureSuccessStatusCode();
    var result = await response.Content.ReadFromJsonAsync<ResponseDTO>();
    Assert.NotNull(result.Id);
}
```

---

### E2E Tests (If Applicable)

**Coverage Target:** 10% of total tests (critical paths only)

**Test Scenarios:**
1. **Critical User Journey:** [Description]

---

## Workflow Status

- [ ] Architecture phase complete
- [ ] Development phase complete
- [ ] QA phase complete
- [ ] Released

## Notes

[Additional context, design decisions, clarifications, or open questions]

**Design Decisions:**
- [Decision 1 and rationale]
- [Decision 2 and rationale]

**Open Questions:**
- [ ] [Question 1] - **Owner:** [Name] - **Due:** [Date]
- [ ] [Question 2] - **Owner:** [Name] - **Due:** [Date]

**Related ADRs:**
- [ADR-XXX: [Title]](../ADRs/ADR-XXX.md)

**References:**
- [External documentation link]
- [Design mockup link]
- [Related ticket/issue]

---

**Story Template Version:** 1.0
**Last Updated:** 2025-10-30
