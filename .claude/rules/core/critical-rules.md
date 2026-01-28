---
description: Critical DevForgeAI rules that always apply
version: "1.0"
created: 2025-12-10
source: Extracted from CLAUDE.md
---

# Critical Rules - ALWAYS Follow

## 1. Technology Decisions
ALWAYS check tech-stack.md before suggesting technologies. If spec requires tech not in tech-stack.md → HALT and use AskUserQuestion.

## 2. File Operations (CRITICAL for Token Efficiency)
Use native tools (40-73% token savings): `Read`, `Edit`, `Write`, `Glob`, `Grep`. NEVER use Bash for file operations.

## 3. Ambiguity Resolution
Use AskUserQuestion for ALL ambiguities: technology not specified, multiple valid approaches, conflicting requirements, security-sensitive decisions.

## 4. Context Files Are Immutable
Never violate: tech-stack.md, source-tree.md, dependencies.md, coding-standards.md, architecture-constraints.md, anti-patterns.md. Changes require Architecture Decision Records (ADRs).

## 5. TDD Is Mandatory
Tests before implementation: Red → Green → Refactor.

## 6. Quality Gates Are Strict
Critical/High violations block progression. Coverage thresholds: 95%/85%/80%.

## 7. No Library Substitution
Technologies in tech-stack.md are locked. Swap requires: user approval + ADR + tech-stack update.

## 8. Anti-Patterns Are Forbidden
Check anti-patterns.md before suggesting: God Objects (>500 lines), direct instantiation (use DI), SQL concatenation, hardcoded secrets.

## 9. Document All Decisions
Architecture decisions require ADRs in `devforgeai/specs/adrs/`.

## 10. Ask, Don't Assume
When in doubt → HALT and use AskUserQuestion.

## 11. Git Operations Require User Approval
NEVER stash, reset --hard, force push, delete branches, or amend commits without user approval.

## 12. Citation Requirements for Recommendations
Technology/architecture recommendations MUST cite authoritative context files with line-specific references. See `.claude/rules/core/citation-requirements.md` for formats and categories.
