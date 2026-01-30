---
id: STORY-V6-TEST
title: Test Requirement with Bad Format
status: Backlog
format_version: "2.0"
---

# Story: Bad Test Format Test

## User Story

**As a** developer,
**I want** to test test_requirement format validation,
**so that** test requirements follow "Test: " prefix convention.

## Acceptance Criteria

**Given** a test_requirement without "Test: " prefix
**When** the validator runs
**Then** it should warn about format issue

## Technical Specification

```yaml
technical_specification:
  format_version: "2.0"

  components:
    - type: "Service"
      name: "InventoryService"
      file_path: "src/Services/InventoryService.cs"
      interface: "IInventoryService"
      requirements:
        - id: "SVC-001"
          description: "Update inventory counts when products sold"
          testable: true
          test_requirement: "Verify inventory updates correctly when products are sold"
          priority: "High"
          # test_requirement should start with "Test: " prefix - should trigger WARNING
```

## Definition of Done

- [ ] Validator warns about test format
