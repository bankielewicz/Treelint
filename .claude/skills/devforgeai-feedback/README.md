# DevForgeAI Feedback Skill

Automated feedback collection system with event-driven hooks for DevForgeAI operations.

## Quick Start

Enable automatic feedback in 3 steps:

### Step 1: Configure Trigger Mode

Edit `devforgeai/config/feedback.yaml`:

```yaml
enabled: true
trigger_mode: failures-only  # Options: always, failures-only, specific-operations, never
```

### Step 2: Enable Hook(s)

Edit `devforgeai/config/hooks.yaml` and set `enabled: true` for desired hooks:

```yaml
hooks:
  - id: post-dev-feedback
    enabled: true  # Enable this hook
```

### Step 3: Run an Operation

```bash
/dev STORY-001  # Feedback automatically triggers on completion
```

## Feature Overview

### Event-Driven Hooks
- **Automatic triggering** on operation completion (dev, qa, release, orchestrate)
- **Pattern matching** for flexible operation filtering (exact, glob, regex)
- **Condition-based** triggers (duration, status, token usage)
- **Timeout protection** prevents hanging operations

### Context Extraction
- **TodoWrite analysis** extracts operation context automatically
- **Phase detection** identifies completed workflow phases
- **Error capture** extracts failure details for targeted questions
- **Secret sanitization** removes sensitive data before feedback

### Adaptive Questioning
- **Context-aware** questions based on operation type and outcome
- **Duration-sensitive** different questions for long vs short operations
- **Failure-focused** targeted questions when operations fail
- **Fatigue prevention** skip tracking and question limits

## Configuration Quick Reference

### feedback.yaml

| Setting | Options | Default | Description |
|---------|---------|---------|-------------|
| `enabled` | true/false | true | Master enable/disable |
| `trigger_mode` | always, failures-only, specific-operations, never | failures-only | When to collect feedback |
| `max_questions` | 0-20 | 5 | Questions per session (0=unlimited) |
| `allow_skip` | true/false | true | Allow skipping questions |
| `max_consecutive_skips` | 1-10 | 3 | Skips before pausing |

### hooks.yaml

| Field | Required | Description |
|-------|----------|-------------|
| `id` | Yes | Unique identifier (lowercase, hyphens) |
| `operation_type` | Yes | command, skill, or subagent |
| `operation_pattern` | Yes | Exact name, glob (*), or regex (^...$) |
| `trigger_status` | Yes | [success], [failure], or [success, failure] |
| `enabled` | No | Default: true |

## Documentation

| Document | Description |
|----------|-------------|
| [User Guide](../../../docs/guides/feedback-system-user-guide.md) | Complete configuration guide |
| [Architecture](../../../docs/architecture/hook-system-design.md) | System design with diagrams |
| [Troubleshooting](../../../docs/guides/feedback-troubleshooting.md) | Common issues and solutions |
| [Migration Guide](../../../docs/guides/feedback-migration-guide.md) | Enable on existing projects |
| [HOOK-SYSTEM.md](./HOOK-SYSTEM.md) | Complete technical reference |
| [SKILL.md](./SKILL.md) | Skill execution workflow |

## Directory Structure

```
.claude/skills/devforgeai-feedback/
├── README.md                 # This file
├── SKILL.md                  # Skill execution workflow
├── HOOK-SYSTEM.md            # Complete hook system reference (27KB)
├── references/
│   ├── context-extraction.md      # Context extraction patterns
│   ├── context-sanitization.md    # Secret/PII removal
│   ├── adaptive-questioning.md    # Question selection logic
│   └── feedback-question-templates.md  # Question templates
└── templates/
    └── message-templates.md       # Feedback message formats
```

## Related Commands

| Command | Description |
|---------|-------------|
| `/feedback` | Manual feedback collection |
| `/feedback-config` | View/edit configuration |
| `/feedback-search` | Search feedback history |
| `/audit-hooks` | Audit hook registry |

## Support

- **Issues:** Check [Troubleshooting Guide](../../../docs/guides/feedback-troubleshooting.md)
- **Configuration:** See [User Guide](../../../docs/guides/feedback-system-user-guide.md)
- **Architecture:** See [Hook System Design](../../../docs/architecture/hook-system-design.md)
