# Test Coverage Thresholds

## Minimum Coverage Requirements (STRICT)

### By Layer
- **Critical Business Logic (Domain/Services):** 95% minimum
- **Application Layer:** 85% minimum
- **Infrastructure Layer:** 80% minimum
- **Overall Project:** 80% minimum

### By Component Type
- **API Controllers:** 85%
- **Business Services:** 95%
- **Repositories:** 80%
- **Utilities:** 75%
- **DTOs/Models:** 60% (properties auto-covered)

## Test Pyramid Distribution

- **Unit Tests:** 70% of total coverage
- **Integration Tests:** 20% of total coverage
- **E2E Tests:** 10% of total coverage

**Rationale:**
- Unit tests are fast, reliable, easy to maintain
- Integration tests validate component interactions
- E2E tests validate critical user journeys

## Per-File Coverage

- **New Files:** 90% minimum
- **Modified Files:** Must not decrease overall coverage
- **Legacy Files:** 60% minimum (incremental improvement)

## Coverage Exceptions

### When to Request Exception

Use AskUserQuestion when coverage falls below threshold for valid reasons:

1. **Non-critical infrastructure code**
   - Logging utilities
   - Diagnostic helpers
   - Development tools

2. **External dependencies**
   - Third-party library wrappers
   - Generated code

3. **UI/Presentation layer**
   - Simple view components
   - Styling code

### Exception Approval Process

1. QA skill detects coverage below threshold
2. Use AskUserQuestion with context
3. Document reason in QA report
4. Create action item for future improvement
5. Add to accepted technical debt

## Coverage Quality Requirements

### Minimum Assertions

- **Average assertions per test:** 1.5-5
- **Tests without assertions:** 0 (no empty tests)

### Mocking Limits

- **Maximum mocks per test:** 5
- **Mocking ratio:** < 2 mocks per test (avoid over-mocking)

### Branch Coverage

- **Critical paths:** 100% branch coverage
- **Business logic:** 90% branch coverage
- **Overall:** 80% branch coverage

## Enforcement

### During Development (Light Validation)

- Check overall coverage > 80% (integration phase only)
- Block if below minimum

### After Story Completion (Deep Validation)

- Validate all layer-specific thresholds
- Check test pyramid distribution
- Verify test quality metrics
- Generate detailed gap report

## Tools Configuration

### .NET (coverlet)
```xml
<PropertyGroup>
    <CollectCoverage>true</CollectCoverage>
    <CoverletOutputFormat>cobertura</CoverletOutputFormat>
    <Threshold>80</Threshold>
    <ThresholdType>line,branch</ThresholdType>
    <ThresholdStat>total</ThresholdStat>
</PropertyGroup>
```

### Python (pytest-cov)
```ini
[tool:pytest]
addopts =
    --cov=src
    --cov-report=html
    --cov-report=term
    --cov-fail-under=80
```

### JavaScript (Jest)
```json
{
  "coverageThreshold": {
    "global": {
      "branches": 80,
      "functions": 80,
      "lines": 80,
      "statements": 80
    }
  }
}
```

## References

- [Coverage Analysis Reference](../../references/coverage-analysis.md)
- [Test Pyramid Best Practices](https://martinfowler.com/articles/practical-test-pyramid.html)
- [Coverage Metrics Guide](https://en.wikipedia.org/wiki/Code_coverage)
