---
id: STORY-V3-TEST
title: Story with Invalid Component Type
status: Backlog
format_version: "2.0"
---

# Story: Invalid Component Type Test

## User Story

**As a** developer,
**I want** to test component type validation,
**so that** invalid types are caught by the validator.

## Acceptance Criteria

**Given** a component with type "BackgroundTask" (not one of 7 valid types)
**When** the validator runs
**Then** it should fail with error about unknown type

## Technical Specification

```yaml
technical_specification:
  format_version: "2.0"

  components:
    - type: "BackgroundTask"
      name: "EmailHandler"
      file_path: "src/Tasks/EmailHandler.cs"
      requirements:
        - id: "TASK-001"
          description: "Send email notifications"
          testable: true
          test_requirement: "Test: Verify email sending functionality"
          priority: "Medium"
```

## Definition of Done

- [ ] Validator catches invalid type
