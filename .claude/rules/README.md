# DevForgeAI Rules Directory

This directory contains modular, versioned rules for the DevForgeAI framework.

## Structure

| Directory | Purpose |
|-----------|---------|
| `core/` | Critical rules that always apply |
| `workflow/` | TDD and story lifecycle rules |
| `security/` | Security and compliance rules |
| `conditional/` | Path-specific rules (activate by file type) |

## How Rules Work

1. All `.md` files in this directory are automatically loaded
2. Rules with `paths:` frontmatter only apply to matching files
3. Rules without frontmatter apply globally

## Adding New Rules

1. Create a new `.md` file in the appropriate subdirectory
2. Add YAML frontmatter if rule is conditional
3. Write clear, actionable guidance

## Version History

- v1.0 (2025-12-10): Initial creation from CLAUDE.md extraction
