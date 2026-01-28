# Subagent Registry Reference

Auto-generation system for maintaining CLAUDE.md subagent documentation.

## Overview

The subagent registry automatically extracts agent metadata from `.claude/agents/*.md` files and generates a consolidated registry section in CLAUDE.md. This ensures documentation never drifts from actual agent definitions.

## Frontmatter Schema

All agent files must include YAML frontmatter with these fields:

### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Agent identifier (lowercase-with-hyphens) |
| `description` | string | Purpose and when to use (max 200 chars recommended) |
| `tools` | string/array | Claude Code tools accessible to this agent |
| `model` | string | Model to use: `haiku`, `sonnet`, or `inherit` |

### Optional Fields

| Field | Type | Description |
|-------|------|-------------|
| `proactive_triggers` | array | Task patterns that should invoke this agent |
| `color` | string | Visual classification (green, cyan, etc.) |
| `permissionMode` | string | Permission handling mode |
| `skills` | string | Associated skill dependencies |

## proactive_triggers Field

Use this field to document when Opus should automatically invoke the agent.

### Format

```yaml
proactive_triggers:
  - "after code implementation"
  - "when reviewing changes"
  - "before git commit"
```

### Guidelines

1. **Be specific** - Use action-oriented phrases
2. **Avoid duplicates** - Each trigger should be unique across all agents
3. **Keep short** - Triggers should be scannable (5-10 words max)

### Examples

**Good triggers:**
- "after code implementation"
- "when coverage gaps detected"
- "during TDD Red phase"

**Avoid:**
- "when needed" (too vague)
- "use this agent for code review tasks" (too verbose)

## Registry Generation

### Usage

```bash
# Update CLAUDE.md registry
bash scripts/generate-subagent-registry.sh

# Check if registry is up-to-date (for CI/pre-commit)
bash scripts/generate-subagent-registry.sh --check

# Output to stdout only (for testing)
bash scripts/generate-subagent-registry.sh --generate-only

# Use custom agents directory
bash scripts/generate-subagent-registry.sh --agents-dir /path/to/agents
```

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success (or up-to-date in check mode) |
| 1 | Registry is stale (check mode only) |
| 2 | Error (missing markers, file errors) |

## CLAUDE.md Markers

The registry is inserted between these marker comments:

```markdown
<!-- BEGIN SUBAGENT REGISTRY -->
## Subagent Registry

*Auto-generated from .claude/agents/*.md - DO NOT EDIT MANUALLY*

| Agent | Description | Tools |
|-------|-------------|-------|
...

### Proactive Trigger Mapping

| Trigger Pattern | Recommended Agent |
|-----------------|-------------------|
...
<!-- END SUBAGENT REGISTRY -->
```

## Pre-commit Integration

The pre-commit hook automatically checks registry consistency when agent files are modified:

```bash
# In .git/hooks/pre-commit
AGENT_FILES=$(git diff --cached --name-only --diff-filter=d | grep '^\.claude/agents/.*\.md$' || true)
if [ -n "$AGENT_FILES" ]; then
    if ! bash scripts/generate-subagent-registry.sh --check; then
        echo "Registry out of date - run: bash scripts/generate-subagent-registry.sh"
        exit 1
    fi
fi
```

## Frontmatter Extraction Pattern

The script uses grep/sed/awk (no external YAML parsers) per tech-stack.md:

```bash
# Extract field from frontmatter (handles Windows line endings)
extract_field() {
    local file="$1"
    local field="$2"
    tr -d '\r' < "$file" | awk '
        /^---$/ { count++; if (count == 1) { next } if (count == 2) { exit } }
        count == 1 { print }
    ' | grep -m1 "^${field}:" | sed "s/^${field}: *//"
}
```

## Troubleshooting

### "No name field" Warning

Agent file lacks required `name:` in frontmatter. Add it:

```yaml
---
name: my-agent
description: ...
---
```

### Duplicate Trigger Warning

Two agents have the same proactive trigger. Make triggers unique or accept that first agent alphabetically wins.

### Registry Markers Not Found

CLAUDE.md is missing marker comments. Add:

```markdown
<!-- BEGIN SUBAGENT REGISTRY -->
<!-- END SUBAGENT REGISTRY -->
```

---

**Story:** STORY-109 - Subagent Registry Auto-Generation
**Created:** 2025-12-19
