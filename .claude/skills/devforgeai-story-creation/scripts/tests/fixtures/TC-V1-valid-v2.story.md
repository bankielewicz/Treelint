---
id: STORY-V1-TEST
title: Valid v2.0 Story with Structured Tech Spec
status: Backlog
format_version: "2.0"
---

# User Story

As a developer, I want a valid v2.0 story structure, so that the validator passes.

## Acceptance Criteria

**Given** a properly structured v2.0 story
**When** the validator runs
**Then** validation should pass with no errors or warnings

## Technical Specification

```yaml
technical_specification:
  format_version: "2.0"

  components:
    - type: "Service"
      name: "UserService"
      file_path: "src/Services/UserService.cs"
      interface: "IUserService"
      lifecycle: "Scoped"
      requirements:
        - id: "SVC-001"
          description: "Service must authenticate users via JWT tokens"
          testable: true
          test_requirement: "Test: Verify JWT token validation accepts valid tokens and rejects expired tokens"
          priority: "High"

  business_rules:
    - id: "BR-001"
      rule: "JWT tokens expire after 24 hours"
      validation: "Check token expiration claim"
      test_requirement: "Test: Token older than 24 hours rejected with 401 Unauthorized"
      priority: "High"

  non_functional_requirements:
    - id: "NFR-001"
      category: "Performance"
      requirement: "Authentication response time must be under 200ms"
      metric: "p95 latency < 200ms"
      test_requirement: "Test: Load test with 1000 users, measure p95 latency < 200ms"
      priority: "High"
```

## Definition of Done

- [x] Code implemented
- [x] Tests passing
- [x] Documentation complete
