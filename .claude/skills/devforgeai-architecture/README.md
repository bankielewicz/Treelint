# DevForgeAI Architecture Skill

**Prevents technical debt through explicit architecture documentation and AI-enforced constraints.**

## Purpose

This skill creates the **foundation** for spec-driven development by establishing immutable project constraints that AI agents must follow. It prevents common sources of technical debt:

- ❌ Library substitution (e.g., Dapper → Entity Framework)
- ❌ Structure chaos (files in wrong locations)
- ❌ Framework mixing (Redux in Zustand project)
- ❌ Assumption-driven development (guessing tech choices)

## What This Skill Creates

1. **tech-stack.md** - Locks technology choices (ORM, frameworks, libraries)
2. **source-tree.md** - Enforces project structure and file organization
3. **dependencies.md** - Approved package list with versions
4. **ADRs** - Architecture Decision Records documenting all major choices
5. **Technical specifications** - Detailed implementation guidance

## When to Use

- ✅ Starting a new project (greenfield)
- ✅ Adding features to existing projects (brownfield)
- ✅ Making technology decisions (database, framework, library choices)
- ✅ Defining project structure
- ✅ Resolving architectural ambiguities
- ✅ Creating technical specs from requirements

## Key Features

### 1. Ambiguity Detection

The skill **automatically detects** when requirements are ambiguous and uses `AskUserQuestion` to clarify:

- Technology not specified → Asks user to choose
- Multiple valid approaches → Asks for preference
- Conflicts with existing standards → Asks for resolution

See [references/ambiguity-detection-guide.md](./references/ambiguity-detection-guide.md) for 10 categories of triggers.

### 2. AskUserQuestion Integration

Every significant decision triggers an interactive question:

```
Question: "Which ORM should be used for data access?"
Options:
  - "Dapper (micro-ORM, fast, explicit SQL)"
  - "Entity Framework Core (full ORM, LINQ)"
  - "NHibernate (mature, complex)"
Description: "⚠️ This choice will be LOCKED and cannot be changed without approval"
```

### 3. Constraint Enforcement

After architecture decisions are made, they're locked in context files that **all other DevForgeAI skills enforce**:

- `devforgeai-development` checks context files before coding
- `devforgeai-qa` validates compliance during testing
- `devforgeai-release` uses architecture docs for deployment

### 4. Technical Debt Prevention

Explicit anti-pattern documentation prevents common mistakes:

- Library substitution explicitly forbidden
- Framework mixing blocked
- Structure violations detected
- Assumption-based development impossible

## Quick Start

### Option 1: Use in Claude Code

```bash
# In Claude Code terminal
"Use devforgeai-architecture skill to set up project architecture for a new C# backend with React frontend"
```

The skill will:
1. Ask questions to understand your project
2. Create context files with your choices
3. Generate ADRs for major decisions
4. Create technical specifications

### Option 2: Manual Initialization

```bash
# Run initialization script
./.claude/skills/devforgeai-architecture/scripts/init_context.sh

# Edit the generated templates
code devforgeai/specs/context/tech-stack.md
code devforgeai/specs/context/source-tree.md
```

## Skill Contents

### SKILL.md
Main skill instructions with:
- Complete architecture workflow
- Phase-by-phase guidance
- AskUserQuestion patterns
- Brownfield-specific guidance

### references/
- `adr-template.md` - Comprehensive ADR template with examples
- `ambiguity-detection-guide.md` - 10 categories of ambiguity triggers
- `tech-stack-template.md` - Technology stack documentation
- `source-tree-template.md` - Project structure documentation

### assets/context-templates/
Ready-to-use templates for all context files:
- `tech-stack.md` - Locks technology choices
- `source-tree.md` - Defines project structure
- (More templates available for dependencies, coding standards, etc.)

### scripts/
- `init_context.sh` - Initialize context files for new projects

## Integration with Other Skills

This skill creates the foundation for:

**devforgeai-development** → Enforces constraints during coding
**devforgeai-qa** → Validates implementation matches specs
**devforgeai-release** → Uses architecture docs for deployment

## Example Workflow

### Scenario: New E-Commerce Platform

```
User: "Create architecture for a new e-commerce platform with C# backend"

Skill activates and asks:

Q1: "What backend technology stack?"
→ User selects: "C# with .NET 8.0"

Q2: "Which database?"
→ User selects: "Microsoft SQL Server"

Q3: "Which ORM for data access?"
→ User selects: "Dapper (micro-ORM, fast)"

Q4: "Which frontend framework?"
→ User selects: "React with TypeScript"

Q5: "Which state management?"
→ User selects: "Zustand (lightweight)"

Skill then:
✅ Creates tech-stack.md locking all choices
✅ Creates source-tree.md with Clean Architecture structure
✅ Creates ADR-001 for database choice
✅ Creates ADR-002 for Dapper ORM selection
✅ Creates ADR-003 for React + Zustand choice
✅ Generates technical specification outline

Result: Zero-ambiguity foundation ready for development
```

## Success Metrics

The skill succeeds when:

- [ ] All technology choices documented and locked
- [ ] All ambiguities resolved (no assumptions)
- [ ] ADRs created for major decisions
- [ ] Context files ready for other skills to enforce
- [ ] Development can proceed without tech decisions

## Critical Rules

1. **NEVER assume technology choices** - Always use AskUserQuestion
2. **LOCK decisions in context files** - tech-stack.md is immutable
3. **Document rationale** - Every choice has an ADR
4. **Prevent substitution** - Explicitly forbid alternatives
5. **Enable enforcement** - Other skills will validate compliance

## Related Documentation

- [Skill Creator Guide](../skill-creator/SKILL.md)
- [DevForgeAI Documentation](../../../docs/)
- [Native Tools Efficiency Analysis](../../../devforgeai/specs/native-tools-vs-bash-efficiency-analysis.md)

## License

Part of DevForgeAI project. See project LICENSE for details.
