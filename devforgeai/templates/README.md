# DevForgeAI Feedback Templates

This directory contains feedback templates used by the DevForgeAI feedback system to structure user feedback based on operation type and context.

## Template Types

### Core Templates
- `command-template.md` - Template for slash command feedback (e.g., `/dev`, `/qa`, `/release`)
- `skill-template.md` - Template for skill feedback (e.g., `devforgeai-development`, `devforgeai-qa`)
- `subagent-template.md` - Template for subagent feedback (e.g., `backend-architect`, `test-automator`)

### Context-Aware Variations
Templates automatically adapt based on:
- **Success/Failure Status**: Different sections for passed vs failed operations
- **Operation Type**: Command-specific, skill-specific, or subagent-specific questions
- **User Context**: First-time vs repeat operations

## Template Format

All templates use YAML frontmatter + Markdown content:

```markdown
---
operation: /dev STORY-042
type: command
status: success
timestamp: 2025-11-20T10:30:00Z
story-id: STORY-042
---

# Retrospective: /dev STORY-042

## What Went Well
{User responses}

## What Went Poorly
{User responses}

## Suggestions for Improvement
{User responses}
```

## Custom Templates

Users can extend default templates by creating custom templates in `.claude/custom-templates/`. Custom templates inherit from base templates and can add additional fields without removing default fields.

See `devforgeai/config/feedback.yaml` for template configuration options.
