---
id: readme-brainstorm-template
title: README.md Template for Brainstorm-Generated Projects
version: "1.0"
created: 2025-12-23
status: Published
---

# README Brainstorm Template

Template for generating initial README.md from brainstorming session data.

## Usage

Load this template during Phase 7 Step 7.7 and populate with session variables.

---

## Template Content

```markdown
# {{project_name}}

{{tagline}}

## Problem

{{problem_statement}}

### Pain Points

{{#each pain_points}}
- **{{this.description}}** ({{this.impact}})
{{/each}}

### Current vs {{project_name}}

| Current Approach | {{project_name}} Approach |
|------------------|---------------------------|
{{#each comparisons}}
| {{this.current}} | {{this.proposed}} |
{{/each}}

---

## Architecture

```
{{architecture_placeholder}}
```

*Architecture will be defined during /create-context phase.*

---

## MVP Features

{{#each mvp_features}}
### {{this.name}}
{{this.description}}

```bash
# Example usage
{{this.example}}
```
{{/each}}

---

## Installation

```bash
# TODO: Add installation instructions after /create-context
pip install {{project_slug}}  # or npm install, cargo install, etc.
```

---

## Usage

```bash
# TODO: Add usage examples after implementation
{{project_slug}} --help
```

---

## Configuration

*Configuration options will be documented during implementation.*

---

## Post-MVP Roadmap

Features planned for future versions:

{{#each post_mvp}}
### {{this.name}} ({{this.priority}})
{{this.description}}
{{/each}}

---

## Development

This project uses the **DevForgeAI Spec-Driven Development Framework**.

### Quick Start

1. **Review brainstorm:** `devforgeai/specs/brainstorms/{{brainstorm_file}}`
2. **Create requirements:** Run `/ideate`
3. **Generate architecture:** Run `/create-context`
4. **Plan work:** Run `/create-epic` then `/create-story`
5. **Implement with TDD:** Run `/dev STORY-XXX`

See [CLAUDE.md](CLAUDE.md) for AI assistant context and workflow details.

---

## Contributing

*Contribution guidelines will be added after initial release.*

---

## License

{{license_placeholder}}
```

---

## Variable Reference

| Variable | Source | Description |
|----------|--------|-------------|
| `{{project_name}}` | topic (sanitized) | Human-readable project name |
| `{{project_slug}}` | topic (slugified) | URL/command-safe name |
| `{{tagline}}` | topic | Full topic text as tagline |
| `{{problem_statement}}` | Phase 2 | Synthesized problem statement |
| `{{pain_points}}` | Phase 2 | Array of {description, impact} |
| `{{comparisons}}` | Phase 2/3 | Array of {current, proposed} |
| `{{architecture_placeholder}}` | Constant | Placeholder ASCII art |
| `{{mvp_features}}` | Phase 6 | must_have_capabilities array |
| `{{post_mvp}}` | Phase 6 | should_have + could_have array |
| `{{brainstorm_file}}` | Session | Brainstorm filename |
| `{{license_placeholder}}` | Constant | "MIT License" or TBD |

---

## Default Values

| Variable | Default |
|----------|---------|
| `{{architecture_placeholder}}` | `[Component Diagram - Define during /create-context]` |
| `{{license_placeholder}}` | `MIT License - See LICENSE file` |

---

**Version:** 1.0 | **Status:** Published | **Created:** 2025-12-23
