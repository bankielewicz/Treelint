# DevForgeAI Context Files Guide

Complete guide to the 6 immutable context files that define architectural boundaries.

---

## Overview

Context files are **immutable constraints** created by the `devforgeai-architecture` skill that prevent technical debt by enforcing architectural decisions.

**Location:** `devforgeai/specs/context/`

---

## The 6 Context Files

### 1. tech-stack.md

**Purpose:** LOCKED technology choices (prevents library substitution)

**Contains:**
- Programming languages
- Frameworks and libraries
- Database systems
- Testing frameworks
- Build tools
- Deployment platforms

**Prevents:**
- Swapping approved libraries (e.g., React → Vue without approval)
- Adding unapproved frameworks
- Technology drift across the project

**Example Content:**
```markdown
## Frontend
- Framework: React 18.2+
- State Management: Zustand
- UI Components: shadcn/ui
- Styling: Tailwind CSS

## Backend
- Language: Node.js 20+
- Framework: Express.js
- ORM: Prisma
- Validation: Zod
```

---

### 2. source-tree.md

**Purpose:** Project structure rules (prevents chaos)

**Contains:**
- Directory organization
- File naming conventions
- Module boundaries
- Folder purposes

**Prevents:**
- Files in wrong locations
- Inconsistent naming patterns
- Flat directory structures
- Organizational chaos

**Example Content:**
```markdown
src/
├── domain/         # Business logic (no external dependencies)
├── application/    # Use cases and orchestration
├── infrastructure/ # External integrations (DB, APIs)
└── presentation/   # Controllers, views, UI components
```

---

### 3. dependencies.md

**Purpose:** Approved packages (prevents unapproved dependencies)

**Contains:**
- Package registry (npm, pip, nuget)
- Exact versions or version ranges
- Purpose for each dependency
- Approval status

**Prevents:**
- Unapproved package additions
- Dependency bloat
- Security vulnerabilities from unknown packages
- Version conflicts

**Example Content:**
```markdown
## Production Dependencies
- express: ^4.18.0 (Web framework)
- prisma: ^5.0.0 (Database ORM)
- zod: ^3.22.0 (Validation)

## Development Dependencies
- typescript: ^5.2.0 (Type checking)
- vitest: ^1.0.0 (Testing framework)
- prettier: ^3.0.0 (Code formatting)
```

---

### 4. coding-standards.md

**Purpose:** Code patterns (enforces consistency)

**Contains:**
- Code style conventions
- Naming patterns
- Comment standards
- File organization rules
- Error handling patterns

**Prevents:**
- Inconsistent code style
- Poor naming conventions
- Missing documentation
- Inconsistent error handling

**Example Content:**
```markdown
## Naming Conventions
- Classes: PascalCase
- Functions: camelCase
- Constants: UPPER_SNAKE_CASE
- Files: kebab-case.ts

## Code Organization
- Max 500 lines per file
- One class per file
- Group related functions
- Alphabetize imports
```

---

### 5. architecture-constraints.md

**Purpose:** Layer boundaries (prevents violations)

**Contains:**
- Dependency rules (what can import what)
- Layer isolation enforcement
- Cross-cutting concern patterns
- Integration boundaries

**Prevents:**
- Cross-layer dependencies (Domain → Infrastructure)
- Circular dependencies
- Tight coupling
- Architecture erosion

**Example Content:**
```markdown
## Layer Dependencies (Allowed)
- Presentation → Application → Domain
- Infrastructure → Domain (interfaces only)

## FORBIDDEN
- Domain → Application (violates clean architecture)
- Domain → Infrastructure (violates dependency inversion)
- Presentation → Infrastructure (bypasses application layer)
```

---

### 6. anti-patterns.md

**Purpose:** Forbidden patterns (prevents technical debt)

**Contains:**
- God Objects (classes >500 lines)
- Direct instantiation (violates DI)
- SQL concatenation (SQL injection risk)
- Hardcoded secrets
- Magic numbers
- Copy-paste code
- Other project-specific anti-patterns

**Prevents:**
- Common mistakes
- Security vulnerabilities
- Maintainability issues
- Technical debt accumulation

**Example Content:**
```markdown
## Forbidden Patterns

### 1. God Objects
❌ Classes with >500 lines
✅ Break into smaller, focused classes

### 2. Direct Instantiation
❌ new Service() in business logic
✅ Inject via constructor (dependency injection)

### 3. SQL Concatenation
❌ "SELECT * FROM users WHERE id = " + userId
✅ Use parameterized queries

### 4. Hardcoded Secrets
❌ const API_KEY = "sk-1234567890"
✅ Load from environment variables
```

---

## Context File Lifecycle

### Creation

Context files are created by `devforgeai-architecture` skill:
```
Skill(command="devforgeai-architecture")
```

Or via slash command:
```
> /create-context my-project
```

### Validation

All 6 context files must exist before development begins. Validated by:
- `devforgeai-development` skill (checks before TDD)
- `context-validator` subagent (fast validation)
- Quality gates (blocks progression if missing)

### Updates

When updating context files:
1. Use Edit tool to modify existing content
2. Document reason for change
3. Create ADR if significant architectural change
4. Update dependent files (e.g., tech-stack.md → dependencies.md)
5. Notify team if breaking change

**Important:** Context file changes require Architecture Decision Records (ADRs) for traceability.

---

## ADR Creation Process

When making significant technology decisions:

1. **Create ADR file:** `devforgeai/specs/adrs/ADR-NNN-title.md` (numbered sequentially)
2. **Document sections:**
   - Context (why this decision is needed)
   - Decision (what was chosen)
   - Rationale (why this choice)
   - Consequences (impacts and trade-offs)
   - Alternatives Considered (what was rejected and why)
3. **Update context files:** Reflect decision in tech-stack.md, dependencies.md, etc.
4. **Reference ADR:** Link in context files for traceability
5. **Maintain index:** Update README.md in `devforgeai/specs/adrs/`

**ADR Naming Convention:**
- `ADR-001-database-selection.md`
- `ADR-002-orm-selection.md`
- `ADR-003-state-management.md`

---

## Quality Gates Using Context Files

### Gate 1: Context Validation (Architecture → Ready for Dev)
- All 6 context files exist and are non-empty
- No placeholder content (TODO, TBD)

### Gate 2: Constraint Compliance (During Development)
- Light QA validates against all 6 context files
- context-validator subagent blocks on violations

### Gate 3: Deep Compliance (QA Approval)
- Deep QA validates full compliance
- Anti-patterns from anti-patterns.md detected
- Code structure matches source-tree.md
- Technologies match tech-stack.md

---

## Story Structure Requirements

Stories in `devforgeai/specs/Stories/` must include:

**YAML Frontmatter:**
```yaml
---
id: STORY-001
title: User Authentication
epic: EPIC-001
sprint: Sprint-1
status: Backlog
points: 8
priority: High
---
```

**Content Sections:**
- User story format: "As a [role], I want [feature], so that [benefit]"
- Acceptance criteria (Given/When/Then format)
- Technical specification (API contracts, data models, business rules)
- Non-functional requirements (performance, security, scalability)

---

## File Locations Quick Reference

**Context Files:**
- `devforgeai/specs/context/` (6 files, created by architecture skill)

**ADRs:**
- `devforgeai/specs/adrs/` (numbered sequentially)

**Deployment:**
- `devforgeai/deployment/` (platform-specific configs)

**QA Outputs:**
- `devforgeai/qa/coverage/`
- `devforgeai/qa/anti-patterns/`
- `devforgeai/qa/spec-compliance/`
- `devforgeai/qa/reports/`

**Stories:**
- `devforgeai/specs/Stories/` (note the dot prefix)

**Epics & Sprints:**
- `devforgeai/specs/Epics/`
- `devforgeai/specs/Sprints/`

---

## Context File Best Practices

1. **Treat as immutable** - Changes require ADRs
2. **Be specific** - "Use 2-space indentation" > "Format code properly"
3. **Include rationale** - Document WHY, not just WHAT
4. **Reference standards** - Link to official docs
5. **Keep current** - Update as project evolves
6. **Version control** - Commit with descriptive messages

---

## Remember

Context files are THE LAW for all development. Never violate tech-stack.md, source-tree.md, or dependencies.md. Use AskUserQuestion if requirements conflict with context files.
