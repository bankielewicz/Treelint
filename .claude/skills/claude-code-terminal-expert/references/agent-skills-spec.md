# Agent Skills Specification Reference

**Official Standard:** https://agentskills.io/specification
**Version:** 1.0 (December 18, 2025)
**Status:** Open Standard by Anthropic
**Adoption:** Microsoft, GitHub, Cursor, Spring AI, VS Code, DevForgeAI

---

## Overview

Agent Skills is an open specification that defines how AI agents discover, load, and use modular capabilities. It establishes a standard interface for packaging AI agent capabilities as portable, discoverable units.

### Design Principles

1. **Progressive Disclosure**: Load only what's needed, when it's needed
2. **Portability**: Skills work across compatible AI systems
3. **Discoverability**: Metadata enables automatic capability matching
4. **Composability**: Skills can be combined and chained
5. **Security**: Tool whitelisting and capability boundaries

---

## YAML Frontmatter Schema

### Complete Schema Definition

```yaml
---
# REQUIRED FIELDS (Agent Skills Spec)
name: skill-name                    # 1-64 chars, lowercase + hyphens only
description: |                      # 1-1024 chars, MUST include trigger context
  What this skill does AND when to use it.
  Use when users ask about X, need help with Y, or want to Z.

# OPTIONAL FIELDS (Agent Skills Spec)
license: MIT                        # License identifier (SPDX recommended)
compatibility: "Claude Code v2.0+"  # 1-500 chars, version requirements
metadata:                           # Arbitrary key-value map
  author: YourName
  version: "1.0.0"
  category: category-name
  last-updated: "2026-01-17"
  custom-field: any-value
allowed-tools: Read Grep Glob       # Space-delimited tool whitelist (experimental)
disable-model-invocation: false     # Prevent automatic triggering

# CLAUDE CODE EXTENSIONS (Not in Agent Skills spec, but valid)
model: claude-model: opus-4-5-20251001   # Preferred model for this skill
---
```

### Required Fields

#### `name` (Required)

**Constraints:**
- Length: 1-64 characters
- Format: Lowercase letters, numbers, and hyphens only
- No consecutive hyphens (`--`)
- Cannot start or end with a hyphen
- Must be unique within the skill discovery scope

**Valid Examples:**
```yaml
name: code-reviewer
name: test-automator
name: claude-code-terminal-expert
name: devforgeai-development
```

**Invalid Examples:**
```yaml
name: Code_Reviewer      # Uppercase and underscores
name: -code-reviewer     # Leading hyphen
name: code--reviewer     # Consecutive hyphens
name: code-reviewer-     # Trailing hyphen
name: this-name-is-way-too-long-and-exceeds-the-sixty-four-character-limit  # Too long
```

#### `description` (Required)

**Constraints:**
- Length: 1-1024 characters
- **MUST** include trigger context (when the skill should be used)
- Should describe both WHAT the skill does AND WHEN to use it

**Best Practices:**
1. Start with what the skill does
2. Include "Use when..." clause
3. List specific trigger phrases or scenarios
4. Be specific about capabilities

**Good Example:**
```yaml
description: |
  Comprehensive expert knowledge of Claude Code Terminal features, configuration,
  and capabilities. Use when users ask about Claude Code functionality ("Can Claude
  Code...?", "Does Claude Code have...?"), creating subagents/skills/commands/plugins,
  configuring settings/models/permissions, installing MCP servers, setting up
  hooks/automation, CI/CD integration, troubleshooting issues, or any Claude Code
  Terminal questions.
```

**Poor Example (Missing Trigger Context):**
```yaml
description: Helps with code review.  # Too vague, no "Use when..."
```

### Optional Fields

#### `license` (Optional)

**Purpose:** Declare the license under which the skill is distributed.

**Recommended:** Use SPDX license identifiers

```yaml
license: MIT
license: Apache-2.0
license: GPL-3.0-only
license: BSD-3-Clause
license: UNLICENSED
```

#### `compatibility` (Optional)

**Constraints:** 1-500 characters

**Purpose:** Specify version requirements or compatibility notes.

```yaml
compatibility: "Claude Code v2.0+"
compatibility: "Claude Code v2.1.0 - v2.2.x (tested on v2.1.12)"
compatibility: "Requires MCP server support"
```

#### `metadata` (Optional)

**Purpose:** Arbitrary key-value map for custom properties.

**CRITICAL:** Properties like `version`, `author`, `category` MUST be nested under `metadata`, NOT at the top level.

**Correct:**
```yaml
metadata:
  author: DevForgeAI
  version: "3.0.0"
  category: knowledge-infrastructure
  last-updated: "2026-01-17"
  custom-property: any-value
```

**WRONG (Will fail validation):**
```yaml
version: "3.0.0"      # WRONG: Must be under metadata
author: DevForgeAI    # WRONG: Must be under metadata
category: tools       # WRONG: Must be under metadata
```

**Common Metadata Properties:**

| Property | Type | Purpose |
|----------|------|---------|
| `author` | string | Skill creator/maintainer |
| `version` | string | Semver version (e.g., "3.0.0") |
| `category` | string | Classification for registry grouping |
| `last-updated` | string | ISO date of last update |
| `topics` | array | Searchable tags |
| `homepage` | string | URL to documentation |
| `repository` | string | Source code URL |

#### `allowed-tools` (Optional, Experimental)

**Purpose:** Whitelist of tools the skill can use. Security feature.

**Format:** Space-delimited list of tool names

```yaml
allowed-tools: Read Grep Glob WebFetch WebSearch
allowed-tools: Read Write Edit Bash
allowed-tools: Read Grep  # Read-only skill
```

**Available Tools:**
- `Read` - Read files
- `Write` - Create/overwrite files
- `Edit` - Modify existing files
- `Glob` - File pattern matching
- `Grep` - Content search
- `Bash` - Shell commands
- `WebFetch` - HTTP requests
- `WebSearch` - Web searches
- `Task` - Spawn subagents
- `AskUserQuestion` - User interaction
- `TodoWrite` - Task management
- `NotebookEdit` - Jupyter notebooks

**Enforcement:** Currently advisory. Future versions may enforce.

#### `disable-model-invocation` (Optional)

**Purpose:** Prevent automatic triggering; require explicit invocation only.

```yaml
disable-model-invocation: true   # Only triggered by explicit skill invocation
disable-model-invocation: false  # (default) Model can auto-invoke based on context
```

**Use Cases:**
- Dangerous operations that should require explicit user intent
- Administrative skills that shouldn't trigger accidentally
- Skills with side effects (file modifications, API calls)

---

## Directory Structure

### Standard Layout

```
skill-name/
├── SKILL.md              # Core prompt (REQUIRED, <500 lines recommended)
├── references/           # On-demand documentation (loaded when needed)
│   ├── detailed-guide.md
│   ├── api-reference.md
│   └── examples.md
├── scripts/              # Executable scripts (output only, never auto-loaded)
│   ├── helper.py
│   └── validator.sh
└── assets/               # Static assets (templates, data, never auto-loaded)
    ├── templates/
    └── data/
```

### File Purposes

| Directory/File | Auto-Loaded | Token Budget | Purpose |
|----------------|-------------|--------------|---------|
| `SKILL.md` | On activation | <5,000 tokens | Core instructions |
| `references/` | On-demand | Unlimited | Detailed documentation |
| `scripts/` | Never | N/A | Executable helpers |
| `assets/` | Never | N/A | Static resources |

### SKILL.md Requirements

1. **YAML Frontmatter**: Must be first content in file
2. **Size Limit**: Recommended <500 lines, maximum 1000 lines
3. **Progressive Disclosure**: Reference detailed docs in `references/`
4. **Execution Model**: Include clear instructions for how skill operates

**Template:**
```markdown
---
name: my-skill
description: |
  What this skill does. Use when users need X or ask about Y.
license: MIT
metadata:
  author: YourName
  version: "1.0.0"
  category: utilities
---

# Skill Title

Brief overview of the skill's purpose.

## Phase 1: Initial Step

Instructions for first phase...

## Phase 2: Next Step

Instructions for second phase...

**For detailed guidance:** → Load `references/detailed-guide.md`
```

---

## Progressive Disclosure

### Three-Tier Loading Model

Agent Skills uses progressive disclosure to minimize token consumption:

```
┌─────────────────────────────────────────────────────────────────┐
│ Level 1: DISCOVERY (~100 tokens)                                │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ name: skill-name                                            │ │
│ │ description: What it does. Use when...                      │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                           ↓                                     │
│ Level 2: ACTIVATION (<5,000 tokens)                             │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Full SKILL.md content loaded into conversation              │ │
│ │ - Frontmatter metadata                                      │ │
│ │ - Core instructions                                         │ │
│ │ - Phase definitions                                         │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                           ↓                                     │
│ Level 3: EXECUTION (Unlimited)                                  │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ On-demand loading of:                                       │ │
│ │ - references/*.md (detailed documentation)                  │ │
│ │ - scripts/* (execution helpers)                             │ │
│ │ - assets/* (templates, data)                                │ │
│ └─────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### Token Efficiency

| Loading Strategy | Tokens Used | Use Case |
|------------------|-------------|----------|
| Discovery only | ~100 | Skill listing, capability matching |
| Discovery + Activation | ~5,000 | Standard skill invocation |
| Full execution | Varies | Complex tasks needing references |

**Measured Savings:** 61% token reduction compared to monolithic skills (all content in one file).

### Implementation Pattern

**In SKILL.md:**
```markdown
## Quick Overview

Brief explanation here...

**For detailed implementation:** → Load `references/detailed-guide.md`

## Phase 1: Basic Operation

Essential instructions that fit in <5K tokens...

**For advanced scenarios:** → Load `references/advanced-scenarios.md`
```

**In references/detailed-guide.md:**
```markdown
# Detailed Implementation Guide

Comprehensive documentation that would bloat SKILL.md...
(Can be hundreds or thousands of lines)
```

---

## Validation

### Official Validator: skills-ref

**Installation:**
```bash
pip install skills-ref
```

**Basic Validation:**
```bash
skills-ref validate path/to/skill/
```

**Validation Checks:**
1. YAML frontmatter syntax
2. Required fields present (`name`, `description`)
3. Field constraints (length, format)
4. Directory structure compliance
5. File naming conventions

### Common Validation Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `Missing required field: name` | No `name` in frontmatter | Add `name: skill-name` |
| `Missing required field: description` | No `description` in frontmatter | Add description with trigger context |
| `Invalid name format` | Uppercase, underscores, or invalid chars | Use lowercase-with-hyphens |
| `Description missing trigger context` | No "Use when" clause | Add usage scenarios to description |
| `Field not allowed at top level: version` | `version` not under `metadata` | Move to `metadata.version` |
| `Name exceeds 64 characters` | Name too long | Shorten to ≤64 chars |

### Pre-Commit Validation

Add to `.claude/hooks/pre-commit`:
```bash
#!/bin/bash
for skill_dir in .claude/skills/*/; do
    if [ -f "$skill_dir/SKILL.md" ]; then
        skills-ref validate "$skill_dir" || exit 1
    fi
done
```

---

## DevForgeAI Integration

### Alignment with DevForgeAI Patterns

| Agent Skills Concept | DevForgeAI Equivalent |
|---------------------|----------------------|
| Progressive disclosure | Reference file loading |
| YAML frontmatter | Skill/Subagent metadata |
| `allowed-tools` | Tool access patterns in agents |
| `metadata.category` | Skill classification |
| Validation framework | Context file validation |

### Recommended DevForgeAI Practices

1. **Skill Metadata Consistency:**
   - All DevForgeAI skills follow Agent Skills spec
   - Use `metadata.category` for skill grouping
   - Track versions with `metadata.version`

2. **Tool Whitelisting:**
   - Document `allowed-tools` for security audits
   - Match tool access patterns in `.claude/agents/*.md`

3. **Validation Integration:**
   - Add `skills-ref validate` to `/qa` Phase 1
   - Fail QA if skills don't meet spec

4. **Context File Cross-References:**
   - `devforgeai/specs/context/tech-stack.md` - Technology constraints
   - `devforgeai/specs/context/architecture-constraints.md` - Pattern enforcement
   - `.claude/rules/` - Conditional rule loading (Claude Code native)

### Migration Guide for Existing Skills

**Step 1: Check Current Frontmatter**
```bash
head -30 .claude/skills/my-skill/SKILL.md
```

**Step 2: Add Required Fields**
```yaml
---
name: my-skill                      # Add if missing
description: |                      # Ensure includes "Use when..."
  Description here. Use when users need X.
---
```

**Step 3: Move Top-Level Fields to metadata**
```yaml
# BEFORE (non-compliant)
version: "1.0.0"
author: Me

# AFTER (compliant)
metadata:
  version: "1.0.0"
  author: Me
```

**Step 4: Validate**
```bash
skills-ref validate .claude/skills/my-skill/
```

---

## Examples

### Minimal Compliant Skill

```yaml
---
name: hello-world
description: |
  A simple greeting skill. Use when users say hello or want a greeting.
---

# Hello World Skill

When invoked, respond with a friendly greeting.
```

### Full-Featured Skill

```yaml
---
name: code-reviewer
description: |
  Expert code review assistant. Use when users ask for code review,
  want feedback on their code, need security analysis, or ask about
  best practices for code they've written.
license: MIT
compatibility: "Claude Code v2.0+ (tested on v2.1.12)"
metadata:
  author: DevForgeAI
  version: "2.0.0"
  category: development-tools
  last-updated: "2026-01-17"
  topics:
    - code-review
    - security
    - best-practices
allowed-tools: Read Grep Glob
disable-model-invocation: false
---

# Code Reviewer

Expert code review with security, performance, and maintainability analysis.

## Phase 1: Gather Context

1. Identify files to review
2. Understand the codebase structure
3. Check for existing style guides

**For detailed review criteria:** → Load `references/review-checklist.md`

## Phase 2: Perform Review

...
```

### DevForgeAI-Style Skill

```yaml
---
name: devforgeai-qa
description: |
  Quality assurance validation skill. Use when validating story implementations,
  running QA checks, verifying test coverage, or checking for anti-patterns.
  Invoked by /qa command.
license: MIT
compatibility: "Claude Code v2.0+"
metadata:
  author: DevForgeAI
  version: "4.0.0"
  category: quality-assurance
  last-updated: "2026-01-17"
  agent-skills-spec-version: "1.0"
allowed-tools: Read Grep Glob Bash Task
---

# DevForgeAI QA Skill

## ⚠️ EXECUTION MODEL

After invocation, YOU (Claude) execute these instructions phase by phase.

## Phase 0: Setup

Load validation mode and story context...

## Phase 1: Coverage Analysis

...

**For detailed coverage analysis:** → Load `references/coverage-analysis.md`
```

---

## Industry Adoption

### Adopters (as of January 2026)

| Organization | Integration | Status |
|--------------|-------------|--------|
| **Anthropic** | Claude Code, Agent SDK | Native support |
| **Microsoft** | VS Code Copilot | Full adoption |
| **GitHub** | Copilot Extensions | Native support |
| **Cursor** | Cursor IDE | Full adoption |
| **Spring AI** | Spring Framework | Plugin system |
| **JetBrains** | AI Assistant | Partial support |

### Community Resources

- **Official Spec:** https://agentskills.io/specification
- **GitHub Repo:** https://github.com/agentskills/agentskills
- **skills-ref Validator:** https://github.com/agentskills/agentskills/tree/main/skills-ref
- **Anthropic Blog:** https://www.anthropic.com/engineering/equipping-agents-for-the-real-world-with-agent-skills
- **VS Code Docs:** https://code.visualstudio.com/docs/copilot/customization/agent-skills

---

## Troubleshooting

### Skill Not Discovered

**Symptoms:** Skill doesn't appear in `/skills` list or isn't auto-invoked

**Checks:**
1. File is named `SKILL.md` (exact case)
2. Located in `.claude/skills/skill-name/SKILL.md`
3. YAML frontmatter is valid (no syntax errors)
4. `name` field is present and valid
5. `disable-model-invocation` isn't `true` (if expecting auto-invocation)

### Validation Failures

**Symptoms:** `skills-ref validate` reports errors

**Debug Steps:**
```bash
# Check YAML syntax
python -c "import yaml; yaml.safe_load(open('SKILL.md').read().split('---')[1])"

# Validate frontmatter fields
skills-ref validate . --verbose
```

### Token Overload

**Symptoms:** Skill loads slowly, context window issues

**Fixes:**
1. Move detailed content to `references/`
2. Keep SKILL.md under 500 lines
3. Use progressive disclosure pattern
4. Reference external docs instead of inlining

### Auto-Invocation Not Working

**Symptoms:** Claude doesn't automatically use skill when expected

**Checks:**
1. `description` includes clear trigger phrases ("Use when...")
2. `disable-model-invocation` is not `true`
3. User query matches description's trigger context
4. No conflicting skills with overlapping descriptions

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-12-18 | Initial release |

---

## References

- [Agent Skills Specification](https://agentskills.io/specification)
- [Agent Skills Overview](https://agentskills.io/home)
- [skills-ref Validator](https://github.com/agentskills/agentskills/tree/main/skills-ref)
- [Anthropic Engineering Blog](https://www.anthropic.com/engineering/equipping-agents-for-the-real-world-with-agent-skills)
- [Claude Code Documentation](https://code.claude.com/docs/en/skills)
- [VS Code Agent Skills](https://code.visualstudio.com/docs/copilot/customization/agent-skills)
- [Spring AI Agent Skills](https://spring.io/blog/2026/01/13/spring-ai-generic-agent-skills/)
