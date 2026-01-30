---
id: STORY-V9-TEST
title: Story with Empty Components Array
status: Backlog
format_version: "2.0"
---

# Story: Empty Components Test

## User Story

**As a** developer,
**I want** to test empty components validation,
**so that** stories must have at least one component.

## Acceptance Criteria

**Given** a story with components: []
**When** the validator runs
**Then** it should fail with "No components defined" error

## Technical Specification

```yaml
technical_specification:
  format_version: "2.0"

  components: []
```

## Definition of Done

- [ ] Validator catches empty components array
