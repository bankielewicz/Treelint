# Changelog Update Guide

Unified guide for appending entries to the `## Change Log` section in story files.

**Status**: LOCKED
**Last Updated**: 2025-12-29
**Version**: 1.0

---

## Format Specification

### Table Format

The Change Log uses a 5-column markdown table with the following headers:

| Date | Author | Phase/Action | Change | Files Affected |
|------|--------|--------------|--------|----------------|

**Column Descriptions:**
- **Date**: Timestamp in `YYYY-MM-DD HH:MM` format (e.g., `2025-12-29 14:30`)
- **Author**: Attribution following the author pattern specification
- **Phase/Action**: Development phase or action performed (e.g., `Red (Phase 02)`, `QA Light`, `Released`)
- **Change**: Brief description of what changed (max 100 characters)
- **Files Affected**: List of files modified or `-` if none

---

## Author Pattern Specification

All entries MUST include an author matching this regex pattern:

```regex
^(claude/[a-z-]+|user/[a-zA-Z0-9_-]+|claude/opus)$
```

### Valid Author Patterns

| Pattern | Description | Example |
|---------|-------------|---------|
| `claude/{subagent}` | Subagent performing work | `claude/test-automator` |
| `user/{name}` | Human user performing work | `user/john-doe` |
| `claude/opus` | Main orchestrator (Opus) | `claude/opus` |

### Author Examples by Skill/Phase

| Skill/Phase | Recommended Author |
|-------------|-------------------|
| Story Creation | `claude/story-requirements-analyst` |
| TDD Red (Phase 02) | `claude/test-automator` |
| TDD Green (Phase 03) | `claude/backend-architect` or `claude/frontend-developer` |
| TDD Refactor (Phase 04) | `claude/refactoring-specialist` |
| Integration (Phase 05) | `claude/integration-tester` |
| DoD Update (Phase 07) | `claude/opus` |
| Git Workflow (Phase 08) | `claude/opus` |
| QA Light | `claude/qa-result-interpreter` |
| QA Deep | `claude/qa-result-interpreter` |
| Release | `claude/deployment-engineer` |

---

## Timestamp Format

**Format**: `YYYY-MM-DD HH:MM`

**Examples**:
- `2025-12-29 14:30`
- `2025-12-29 09:15`
- `2025-12-30 16:45`

**Generation**: Use current system time when appending entries.

---

## Example Changelog Entries

```markdown
## Change Log

**Current Status:** In Development

| Date | Author | Phase/Action | Change | Files Affected |
|------|--------|--------------|--------|----------------|
| 2025-12-29 14:30 | claude/story-requirements-analyst | Created | Story created | STORY-152.story.md |
| 2025-12-29 15:00 | claude/test-automator | Red (Phase 02) | Tests for AC#1-3 | tests/STORY-152/*.sh |
| 2025-12-29 15:30 | claude/backend-architect | Green (Phase 03) | Implementation | .claude/references/changelog-update-guide.md |
| 2025-12-29 16:00 | claude/refactoring-specialist | Refactor (Phase 04) | Code cleanup | - |
| 2025-12-29 16:15 | claude/qa-result-interpreter | QA Light | Passed: Coverage 98%, 0 violations | - |
| 2025-12-29 16:30 | claude/deployment-engineer | Released | Deployed to production | CHANGELOG.md |
```

---

## Edit Tool Snippets for Appending Entries

### Development Skill - Append Entry

Use the Edit tool to append new entries to the changelog table:

```python
# Step 1: Find the last entry in the changelog
# Read the story file to locate the last row

# Step 2: Append new entry using Edit
Edit(
    file_path="devforgeai/specs/Stories/STORY-XXX.story.md",
    old_string="| 2025-12-29 15:00 | claude/test-automator | Red (Phase 02) | Tests for AC#1-3 | tests/STORY-XXX/*.sh |",
    new_string="| 2025-12-29 15:00 | claude/test-automator | Red (Phase 02) | Tests for AC#1-3 | tests/STORY-XXX/*.sh |\n| 2025-12-29 15:30 | claude/backend-architect | Green (Phase 03) | Implementation complete | src/feature.ts |"
)
```

### Alternative: Append Before Next Section

```python
# If the Change Log is followed by ## Notes or similar:
Edit(
    file_path="devforgeai/specs/Stories/STORY-XXX.story.md",
    old_string="\n---\n\n## Notes",
    new_string="\n| 2025-12-29 15:30 | claude/backend-architect | Green (Phase 03) | Implementation | src/*.ts |\n\n---\n\n## Notes"
)
```

---

## Instructions for Skills and Subagents

### devforgeai-development Skill

At the end of each TDD phase, append a changelog entry:

1. **Phase 02 (Red)**: Author = `claude/test-automator`, Action = `Red (Phase 02)`
2. **Phase 03 (Green)**: Author = `claude/backend-architect` or `claude/frontend-developer`, Action = `Green (Phase 03)`
3. **Phase 04 (Refactor)**: Author = `claude/refactoring-specialist`, Action = `Refactor (Phase 04)`
4. **Phase 05 (Integration)**: Author = `claude/integration-tester`, Action = `Integration (Phase 05)`
5. **Phase 07 (DoD Update)**: Author = `claude/opus`, Action = `DoD Update (Phase 07)`
6. **Phase 08 (Git)**: Author = `claude/opus`, Action = `Git Commit (Phase 08)`

### devforgeai-qa Skill

At Phase 3.4 (Story File Update), append a changelog entry:

- **Author**: `claude/qa-result-interpreter`
- **Phase/Action**: `QA Light` or `QA Deep`
- **Change**: `{result}: Coverage {pct}%, {violations} violations`

Example:
```
| 2025-12-29 16:00 | claude/qa-result-interpreter | QA Deep | Passed: Coverage 96%, 0 violations | - |
```

### devforgeai-release Skill

At Phase 5 (Release Documentation), append a changelog entry:

- **Author**: `claude/deployment-engineer`
- **Phase/Action**: `Released`
- **Change**: Description of release (e.g., `Deployed v1.2.3 to production`)

---

## Backward Compatibility: Migration for Existing Stories

If a story file does not have a `## Change Log` section:

**Operation Order - Follow These Steps:**

To add a changelog entry, first check if the section exists, then create it if missing:

1. **First**: Detect if `## Change Log` section exists
2. **Then**: If missing, create the section after `## Definition of Done`
3. Add the `**Current Status:**` field
4. Add the table headers
5. Add an initial "Story Migrated" entry

**Content Preservation Requirement:**

When adding the Change Log section to existing stories, you MUST:
- Preserve all existing story content intact
- Keep all sections in their original order
- Only INSERT the new `## Change Log` section, do not modify other sections
- Existing content should remain unchanged after migration

```markdown
## Change Log

**Current Status:** In Development

| Date | Author | Phase/Action | Change | Files Affected |
|------|--------|--------------|--------|----------------|
| 2025-12-29 14:00 | claude/opus | Migrated | Story migrated to v2.5 format | STORY-XXX.story.md |
```

---

## Validation Rules

1. **Author pattern**: MUST match regex `^(claude/[a-z-]+|user/[a-zA-Z0-9_-]+|claude/opus)$`
2. **Date format**: MUST be `YYYY-MM-DD HH:MM`
3. **Change length**: SHOULD be <= 100 characters (truncate with ellipsis if longer)
4. **Table format**: MUST maintain 5 columns with proper separators

---

## References

- Story Template: `.claude/skills/devforgeai-story-creation/assets/templates/story-template.md`
- Development Skill: `.claude/skills/devforgeai-development/SKILL.md`
- QA Skill: `.claude/skills/devforgeai-qa/SKILL.md`
- Release Skill: `.claude/skills/devforgeai-release/SKILL.md`
