# Story Type Classification

**Status**: LOCKED
**Last Updated**: 2025-12-23
**Version**: 1.0 (Moved from coding-standards.md per STORY-126 QA remediation)

## Overview

Stories can specify a `type` field in YAML frontmatter to optimize the TDD workflow. This enables phase skipping based on story characteristics.

---

## Story Type Enum (Authoritative Definition)

```yaml
# Valid story types in frontmatter
type: feature        # Default - all phases required
type: documentation  # Skip Phase 05 Integration
type: bugfix         # Skip Phase 04 Refactor
type: refactor       # Skip Phase 02 Red
```

**Validation**: The `/dev` command validates story type in Phase 01.6 (Pre-Flight Validation). Invalid types cause a HALT.

---

## Phase Skip Matrix

| Story Type | Skipped Phase | When to Use | Rationale |
|------------|---------------|-------------|-----------|
| `feature` | None | New functionality, feature enhancements | Full TDD workflow required |
| `documentation` | Phase 05 Integration | Documentation-only, no runtime code | No integration points to test |
| `bugfix` | Phase 04 Refactor | Bug fixes, quick patches | Minimal changes, avoid scope creep |
| `refactor` | Phase 02 Red | Code refactoring, quality improvements | Tests already exist |

---

## Workflow Diagrams by Type

### Feature (Default)
```
Phase 01 → Phase 02 → Phase 03 → Phase 04 → Phase 05 → Phase 06 → ...
(Pre-Flight) (Red)    (Green)   (Refactor) (Integrate) (Deferral)
```
All phases execute. Use for new features or significant enhancements.

### Documentation
```
Phase 01 → Phase 02 → Phase 03 → Phase 04 → [SKIP] → Phase 06 → ...
(Pre-Flight) (Red)    (Green)   (Refactor)          (Deferral)
```
Skips integration testing. Use for docs-only changes with no runtime code.

### Bugfix
```
Phase 01 → Phase 02 → Phase 03 → [SKIP] → Phase 05 → Phase 06 → ...
(Pre-Flight) (Red)    (Green)          (Integrate) (Deferral)
```
Skips refactoring. Use for targeted bug fixes where minimal changes preferred.

### Refactor
```
Phase 01 → [SKIP] → Phase 03 → Phase 04 → Phase 05 → Phase 06 → ...
(Pre-Flight)        (Green)   (Refactor) (Integrate) (Deferral)
```
Skips writing new tests (Red phase). Use when refactoring code with existing test coverage.

---

## Backward Compatibility

Stories without the `type` field default to `feature` type (full TDD workflow). No migration required for existing stories. No warnings are displayed for missing type field.

---

## Usage Example

```yaml
---
id: STORY-126
title: Story Type Detection & Phase Skipping
type: feature    # Explicitly set story type
status: Backlog
priority: MEDIUM
story-points: 5
epic: EPIC-025
---
```

---

## Cross-References

- **Phase 01.6 Type Detection**: `.claude/skills/devforgeai-development/references/preflight/_index.md`
- **Phase Skip Logic**: `.claude/skills/devforgeai-development/references/integration-testing.md`
- **Story Template**: `.claude/skills/devforgeai-story-creation/assets/templates/story-template.md`

---

## Changelog

| Version | Date | Change |
|---------|------|--------|
| 1.0 | 2025-12-23 | Moved from `devforgeai/specs/context/coding-standards.md` to skill reference (HIGH-001 remediation) |
