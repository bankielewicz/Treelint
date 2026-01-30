---
id: STORY-V10-TEST
title: Story with Invalid YAML Syntax
status: Backlog
format_version: "2.0"
---

# Story: Invalid YAML Syntax Test

## User Story

**As a** developer,
**I want** to test YAML syntax validation,
**so that** malformed YAML is caught.

## Acceptance Criteria

**Given** a story with malformed YAML (syntax errors)
**When** the validator runs
**Then** it should fail with YAML parse error

## Technical Specification

```yaml
technical_specification:
  format_version: "2.0"

  components:
    - type: "Service
      name: BrokenService
      file_path: "src/Services/BrokenService.cs"
      requirements:
        - id SVC-001
          description: "Missing colon after id"
          test_requirement: Test: Something
            priority: "High"
```

## Definition of Done

- [ ] Validator catches YAML syntax errors
