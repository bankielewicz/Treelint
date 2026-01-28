# DevForgeAI Architecture Documentation

This directory contains technical architecture documentation for the DevForgeAI framework.

## Documents

| Document | Description |
|----------|-------------|
| [installer-architecture.md](./installer-architecture.md) | Comprehensive installer module architecture, including installation modes, core workflows, and security considerations |

## Architecture Overview

The DevForgeAI framework consists of several major subsystems:

```
DevForgeAI Framework
├── Installer System          # Installation, upgrade, rollback, validation
│   └── See: installer-architecture.md
├── Skills System             # Workflow automation (.claude/skills/)
├── Agents System             # Subagent delegation (.claude/agents/)
├── Commands System           # Slash commands (.claude/commands/)
├── Hooks System              # Event-driven automation
├── Feedback System           # AI analysis and retrospectives
└── Orchestration System      # Workflow coordination
```

## Quick Links

### Installer
- [Installation Modes](./installer-architecture.md#installation-modes)
- [Exit Codes](./installer-architecture.md#exit-codes)
- [Security Considerations](./installer-architecture.md#security-considerations)

### Related Documentation
- [Framework Maintainer Guide](../FRAMEWORK-MAINTAINER-GUIDE.md)
- [External Project Setup Guide](../EXTERNAL-PROJECT-SETUP-GUIDE.md)
- [Uninstall Usage Guide](../UNINSTALL-USAGE-GUIDE.md)

## Contributing

When adding architecture documentation:

1. Follow the established format (Table of Contents, diagrams, tables)
2. Include version and last-updated date
3. Cross-reference related documentation
4. Use ASCII diagrams for compatibility
