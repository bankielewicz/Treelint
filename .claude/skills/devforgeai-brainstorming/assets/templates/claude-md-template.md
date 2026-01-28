---
id: claude-md-template
title: CLAUDE.md Template for Brainstorm-Generated Projects
version: "1.0"
created: 2025-12-23
status: Published
---

# CLAUDE.md Brainstorm Template

Template for generating initial CLAUDE.md from brainstorming session data.

## Usage

Load this template during Phase 7 Step 7.7 and populate with session variables.

---

## Template Content

```markdown
# {{project_name}} - AI Assistant Context

## What You're Working On

**{{project_name}}** is the software you are building. {{project_description}}

### Core Concept

{{core_concept}}

### You Are Building This Tool

{{project_name}} is the **project** - the software being developed. When you make changes, write code, or implement features, you are building {{project_name}} itself.

---

## Development Framework

This project uses **DevForgeAI Spec-Driven Development Framework** as its guardrails.

### Framework vs Project

| Aspect | DevForgeAI (Framework) | {{project_name}} (Project) |
|--------|------------------------|---------------------------|
| **Role** | Development methodology | Product being built |
| **Contains** | Skills, commands, workflows | Source code, features |
| **Location** | `devforgeai/`, `.claude/` | `src/` |
| **Purpose** | How to develop | What to develop |

### DevForgeAI Commands Available

| Command | Purpose |
|---------|---------|
| `/ideate` | Transform brainstorm into requirements |
| `/create-context` | Generate architecture constraint files |
| `/create-epic` | Create epic with feature breakdown |
| `/create-story` | Create user story with acceptance criteria |
| `/dev` | Implement story using TDD workflow |
| `/qa` | Run quality validation |
| `/release` | Deploy to target environment |

### Context Files (Constraints)

When these files exist, they constrain your implementation decisions:

| File | Purpose |
|------|---------|
| `devforgeai/specs/context/tech-stack.md` | Allowed technologies |
| `devforgeai/specs/context/source-tree.md` | Required file structure |
| `devforgeai/specs/context/dependencies.md` | Approved dependencies |
| `devforgeai/specs/context/coding-standards.md` | Code style rules |
| `devforgeai/specs/context/architecture-constraints.md` | Architecture patterns |
| `devforgeai/specs/context/anti-patterns.md` | Forbidden patterns |

**Important:** If context files don't exist yet, run `/create-context` before implementation.

---

## {{project_name}} MVP Scope

### Must-Have Features (v1)

{{#each mvp_features}}
{{@index}}. **{{this.name}}**
   - {{this.description}}
{{/each}}

### Technology Stack (MVP)

| Component | Technology |
|-----------|------------|
{{#each tech_stack}}
| {{this.component}} | {{this.technology}} |
{{/each}}

*Note: Tech stack will be finalized during /create-context phase.*

---

## Project Structure

```
{{project_slug}}/
├── src/
│   └── {{project_slug}}/
│       └── ...
├── tests/
├── devforgeai/
│   └── specs/
│       ├── brainstorms/
│       │   └── {{brainstorm_file}}
│       ├── context/
│       └── stories/
├── CLAUDE.md
└── README.md
```

---

## Key Design Decisions

{{#each design_decisions}}
### {{this.decision}}

**Rationale:** {{this.rationale}}
{{/each}}

---

## Workflow Checklist

When starting work on {{project_name}}:

1. **Check for stories:** `ls devforgeai/specs/stories/`
2. **If no stories:** Run `/ideate` then `/create-epic` then `/create-story`
3. **If no context files:** Run `/create-context` first
4. **Implement with TDD:** Use `/dev STORY-XXX`
5. **Validate quality:** Use `/qa STORY-XXX`

---

## Brainstorm Reference

The initial brainstorming session is documented at:
`devforgeai/specs/brainstorms/{{brainstorm_file}}`

Key insights from brainstorming:
{{#each key_insights}}
- {{this}}
{{/each}}

---

{{#if research_artifacts}}
## Research Artifacts

{{#each research_artifacts}}
- `{{this.file}}` - {{this.description}}
{{/each}}
{{/if}}

---

## Important Reminders

1. **{{project_name}} = Project** - You are building this tool
2. **DevForgeAI = Framework** - Guides how you build it
3. **Context files are immutable** - Don't violate them
4. **TDD is mandatory** - Tests before implementation
{{#each custom_reminders}}
5. **{{this}}**
{{/each}}

---

## Getting Help

- **DevForgeAI commands:** Run `/help` or check `.claude/commands/`
- **{{project_name}} requirements:** See brainstorm document
- **Architecture decisions:** Check `devforgeai/specs/adrs/` (when created)
```

---

## Variable Reference

| Variable | Source | Description |
|----------|--------|-------------|
| `{{project_name}}` | topic | Human-readable name |
| `{{project_slug}}` | topic | URL-safe slug |
| `{{project_description}}` | problem_statement + ideal_state | Combined description |
| `{{core_concept}}` | topic + ideal_state | Core concept summary |
| `{{mvp_features}}` | Phase 6 | must_have_capabilities |
| `{{tech_stack}}` | Phase 4 | technology_ideas array |
| `{{design_decisions}}` | Phase 5 | hypotheses as decisions |
| `{{brainstorm_file}}` | Session | Brainstorm filename |
| `{{key_insights}}` | All phases | Key takeaways array |
| `{{research_artifacts}}` | Phase 3 | If research conducted |
| `{{custom_reminders}}` | Phase 4/5 | Constraint-derived reminders |

---

**Version:** 1.0 | **Status:** Published | **Created:** 2025-12-23
