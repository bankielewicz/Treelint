# Claude Code Terminal Expert - Integration Guide

**Version:** 3.1.0
**Last Updated:** 2026-01-29

---

## Overview

This document describes how the `claude-code-terminal-expert` skill integrates with the DevForgeAI framework and other skills.

---

## Role in Framework

**Type:** Infrastructure/Knowledge Skill (not a workflow skill)

**Purpose:** Provide Claude Code Terminal expertise to support DevForgeAI users with:
- Feature discovery ("Can Claude Code...?")
- Configuration guidance
- Troubleshooting assistance
- Best practices recommendations

**Relationship:** Complements (doesn't replace) DevForgeAI workflow skills:
- devforgeai-ideation
- devforgeai-development
- devforgeai-qa
- devforgeai-release

---

## When Other Skills Should Reference This Skill

| Scenario | Trigger Pattern | Action |
|----------|-----------------|--------|
| User asks about Claude Code features | "Can Claude Code...?", "Does Claude Code have...?" | Load appropriate reference file |
| Configuration question during dev | "How do I configure...?", "What settings...?" | Load configuration-guide.md |
| Troubleshooting during workflow | "Claude Code won't...", "Error when..." | Load troubleshooting-guide.md |
| Best practices inquiry | "What's the best way to...?" | Load best-practices.md |
| Feature comparison needed | "Should I use skill or command?" | Load comparison-matrix.md |

---

## Cross-Skill References

### From devforgeai-development
When implementing features and Claude Code questions arise:
- Subagent invocation fails → `troubleshooting-guide.md` Section 3
- Permission issues occur → `configuration-guide.md` Section 4
- MCP tools unavailable → `core-features.md` Section 5
- Task management questions → `core-features.md` Section 6.1

### From devforgeai-qa
When validation tools are questioned:
- Tool documentation → `core-features.md`
- Hook behavior unclear → `integration-patterns.md` Section 3
- Background task issues → `core-features.md` Background Tasks section

### From devforgeai-architecture
When technology decisions involve Claude Code:
- CI/CD integration planned → `integration-patterns.md` Section 1-2
- Hook automation needed → `integration-patterns.md` Section 3
- Plugin architecture → `core-features.md` Section 4

---

## Token Impact

| Load Scenario | Tokens | When |
|---------------|--------|------|
| Metadata only | ~100 | Always (skill list) |
| SKILL.md triggered | ~2,000 | When skill invoked |
| Single reference | 1,200-3,500 | Per reference file |
| Typical usage | 2,100-6,000 | Discovery + 1-2 references |
| Maximum possible | ~18,000 | All files (unlikely) |

**Best Practices:**
- Load single reference file per query when possible
- Use `quick-reference.md` for simple lookups
- Load multiple references only for cross-cutting questions

---

## Maintenance Coordination

### Update Triggers
1. New Claude Code version released (check GitHub CHANGELOG.md)
2. User reports outdated information
3. Quarterly refresh cycle

### Update Command
```bash
/skill:update claude-code-terminal-expert
```

### Self-Updating Mechanism
- 29+ official documentation URLs embedded in SKILL.md
- WebFetch capability for retrieving latest docs
- Comparison & update procedure documented

### Key Documentation Sources
| Source | URL | Purpose |
|--------|-----|---------|
| Changelog | https://github.com/anthropics/claude-code/blob/main/CHANGELOG.md | Version changes |
| Docs | https://code.claude.com/docs | Official documentation |
| Best Practices | https://www.anthropic.com/engineering/claude-code-best-practices | Anthropic guidance |

---

## Version Compatibility

| Skill Version | Claude Code Version | Status |
|---------------|---------------------|--------|
| 3.1.0 | 2.1.23 | **Current** |
| 3.0.0 | 2.1.12 | Previous |
| 2.0.0 | 2.0.x | Legacy |
| 1.0.0 | 1.x | Deprecated |

---

## Reference File Index

| File | Purpose | Size | When to Load |
|------|---------|------|--------------|
| `core-features.md` | Subagents, Skills, Commands, Plugins, MCP, 2.1.x features | ~2,800 lines | Feature questions |
| `configuration-guide.md` | Settings, models, CLI, permissions, env vars | ~1,500 lines | Config questions |
| `integration-patterns.md` | CI/CD, hooks, automation, headless | ~2,800 lines | Integration questions |
| `troubleshooting-guide.md` | Installation, auth, performance, errors | ~2,100 lines | Error diagnosis |
| `advanced-features.md` | Sandboxing, networking, monitoring, security | ~3,500 lines | Advanced topics |
| `best-practices.md` | Workflows, efficiency, prompt engineering | ~1,200 lines | Best practices |
| `agent-skills-spec.md` | Agent Skills specification | ~800 lines | Skill creation |
| `quick-reference.md` | Commands, shortcuts, cheat sheet | ~700 lines | Quick lookups |
| `comparison-matrix.md` | Feature comparison, decision trees | ~600 lines | Feature selection |

---

## Integration Patterns

### Pattern 1: Reactive Loading
```
User: "Can Claude Code run background tasks?"
→ Skill triggered by keyword
→ Load core-features.md (Background Tasks section)
→ Provide answer with examples
```

### Pattern 2: Cross-Reference
```
User: "My hook isn't working"
→ Skill triggered by troubleshooting context
→ Load troubleshooting-guide.md
→ If hook-specific, also load integration-patterns.md
→ Provide diagnosis + solution
```

### Pattern 3: Progressive Disclosure
```
User: "Tell me about skills"
→ Load core-features.md Section 2 (Skills overview)
→ If follow-up: "How do I create one?"
→ Load agent-skills-spec.md for detailed specification
```

---

## Error Handling

### Skill Not Found
If claude-code-terminal-expert is not available:
1. Check `.claude/skills/` directory exists
2. Verify SKILL.md is present and valid YAML
3. Check for syntax errors in frontmatter

### Reference File Missing
If a reference file fails to load:
1. Continue with available information
2. Note the missing file in response
3. Suggest checking skill installation

### Outdated Information
If user reports outdated content:
1. Acknowledge the gap
2. Use WebFetch to retrieve latest from official docs
3. Update reference file
4. Confirm update with user

---

## Changelog

| Version | Date | Changes |
|---------|------|---------|
| 3.1.0 | 2026-01-29 | Added 2.1.23 features, task management, keybindings, this integration guide |
| 3.0.0 | 2026-01-18 | Agent Skills Spec compliance |
| 2.0.0 | 2025-12-20 | December 2025 features update |
| 1.0.0 | 2025-11-06 | Initial creation |
