---
id: STORY-V11-TEST
title: Story with Vague NFR Metric
status: Backlog
format_version: "2.0"
---

# Story: Vague NFR Metric Test

## User Story

**As a** developer,
**I want** to test NFR metric validation,
**so that** vague metrics are flagged.

## Acceptance Criteria

**Given** an NFR with vague metric (no numbers or thresholds)
**When** the validator runs
**Then** it should warn about immeasurable metric

## Technical Specification

```yaml
technical_specification:
  format_version: "2.0"

  components:
    - type: "Service"
      name: "CacheService"
      file_path: "src/Services/CacheService.cs"
      interface: "ICacheService"
      requirements:
        - id: "SVC-001"
          description: "Cache frequently accessed data"
          testable: true
          test_requirement: "Test: Verify cache hit rate improves performance"
          priority: "High"

  non_functional_requirements:
    - id: "NFR-001"
      category: "Performance"
      requirement: "Cache performance should be good"
      metric: "should be fast"
      test_requirement: "Test: Measure cache performance"
      priority: "Medium"
      # Vague metric "should be fast" has no numbers - should trigger WARNING
```

## Definition of Done

- [ ] Validator warns about vague metric
