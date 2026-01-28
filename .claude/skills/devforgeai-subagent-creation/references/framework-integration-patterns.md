# DevForgeAI Framework Integration Patterns

**Purpose:** Framework context for devforgeai-subagent-creation skill to ensure generated subagents are DevForgeAI-aware

---

## DevForge AI Context Files (6 Immutable Constraints)

Generated subagents must reference appropriate context files based on their domain:

### 1. tech-stack.md
**Purpose:** Locked technology choices
**When subagents reference:**
- Backend subagents: Validate technology suggestions match approved stack
- Frontend subagents: Check framework/library compliance
- All subagents: Before suggesting any technology or library

### 2. source-tree.md
**Purpose:** Project structure rules
**When subagents reference:**
- Backend/Frontend subagents: Before creating files or suggesting locations
- All subagents: When organizing generated code

### 3. dependencies.md
**Purpose:** Approved packages only
**When subagents reference:**
- Backend/Frontend subagents: Before adding any package dependency
- All subagents: When suggesting libraries

### 4. coding-standards.md
**Purpose:** Code style and patterns
**When subagents reference:**
- Backend/Frontend subagents: When generating or reviewing code
- QA subagents: When validating code quality
- Test subagents: When organizing test files

### 5. architecture-constraints.md
**Purpose:** Layer boundaries
**When subagents reference:**
- Backend/Architecture subagents: When validating layer dependencies
- All implementation subagents: Before cross-layer calls

### 6. anti-patterns.md
**Purpose:** Forbidden patterns
**When subagents reference:**
- QA subagents: Primary detection list
- Code review subagents: What to flag
- All subagents: What NOT to generate

---

## Context File Integration by Domain

### Backend Subagents
**Must reference:** All 6 context files
- tech-stack.md, source-tree.md, dependencies.md, coding-standards.md, architecture-constraints.md, anti-patterns.md

**Reasoning:** Backend code touches all layers, must respect all constraints

### Frontend Subagents
**Must reference:** 3 context files
- tech-stack.md (framework, state management)
- source-tree.md (component locations)
- coding-standards.md (component patterns)

### QA Subagents
**Must reference:** 2 context files
- anti-patterns.md (detection list)
- coding-standards.md (validation rules)

### Architecture Subagents
**Must reference:** All 6 context files
- Comprehensive awareness required for system design

### Security Subagents
**Must reference:** 3 context files
- anti-patterns.md (security violations)
- coding-standards.md (security patterns)
- architecture-constraints.md (layer isolation for security)

### Deployment Subagents
**Must reference:** 3 context files
- tech-stack.md (deployment platform)
- dependencies.md (deployment tools)
- source-tree.md (deployment artifact locations)

---

## Quality Gates (4 Gates)

Generated subagents participate in quality gates:

### Gate 1: Context Validation (Architecture → Ready for Dev)
- **Participants:** architecture subagents, context-validator
- **Check:** All 6 context files exist and validated
- **Action:** Block if context missing

### Gate 2: Test Passing (Dev Complete → QA In Progress)
- **Participants:** test-automator, backend-architect, frontend-developer
- **Check:** Build succeeds, all tests pass (100%)
- **Action:** Block if tests fail

### Gate 3: QA Approval (QA Approved → Releasing)
- **Participants:** security-auditor, code-reviewer, context-validator, test-automator
- **Check:** Coverage thresholds (95%/85%/80%), zero CRITICAL/HIGH violations
- **Action:** Block if violations or coverage below threshold

### Gate 4: Release Readiness (Releasing → Released)
- **Participants:** deployment-engineer, security-auditor
- **Check:** QA approved, all checkboxes complete, no blockers
- **Action:** Block if not ready

---

## Workflow States (11-State Progression)

```
Backlog → Architecture → Ready for Dev → In Development → Dev Complete →
QA In Progress → [QA Approved | QA Failed] → Releasing → Released
```

**Subagent roles by state:**

- **Backlog → Architecture:** requirements-analyst, architect-reviewer, api-designer
- **Ready for Dev → In Development:** test-automator (generates tests first)
- **In Development:** backend-architect, frontend-developer, documentation-writer
- **In Development → Dev Complete:** code-reviewer, refactoring-specialist, integration-tester
- **Dev Complete → QA In Progress:** context-validator (light validation)
- **QA In Progress:** security-auditor, test-automator (coverage gaps), context-validator (deep)
- **QA Approved → Releasing:** deployment-engineer
- **Releasing:** deployment-engineer, security-auditor (final scan)

---

## Skill Integration Patterns

### devforgeai-development Skill

**Subagents invoked:**
- **Phase 1 (Red):** test-automator
- **Phase 2 (Green):** backend-architect OR frontend-developer
- **Phase 2 (Validation):** context-validator
- **Phase 3 (Refactor):** refactoring-specialist, code-reviewer
- **Phase 4 (Integration):** integration-tester

**Sequential execution:**
test-automator → backend-architect → context-validator → refactoring-specialist + code-reviewer → integration-tester

### devforgeai-qa Skill

**Subagents invoked:**
- **Phase 0 Step 2.5:** deferral-validator (MANDATORY)
- **Light Validation:** context-validator
- **Deep Validation:** security-auditor, test-automator (coverage gaps)
- **Phase 5 Step 6:** qa-result-interpreter (result formatting)

**Parallel execution possible:**
- security-auditor + test-automator (independent analyses)

### devforgeai-architecture Skill

**Subagents invoked:**
- architect-reviewer (validates decisions)
- api-designer (defines contracts)

### devforgeai-release Skill

**Subagents invoked:**
- security-auditor (pre-release scan)
- deployment-engineer (infrastructure + deployment)

### devforgeai-orchestration Skill

**Subagents invoked:**
- requirements-analyst (epic decomposition, sprint planning)
- technical-debt-analyzer (sprint retrospectives)
- sprint-planner (sprint creation)

### devforgeai-story-creation Skill

**Subagents invoked:**
- story-requirements-analyst (Phase 2 - user story + AC)
- api-designer (Phase 3 - if API detected)

### devforgeai-ui-generator Skill

**Subagents invoked:**
- ui-spec-formatter (Phase 6 Step 3.5 - format results)

---

## Token Efficiency Mandate

**All generated subagents MUST use native tools for file operations:**

**40-73% Token Savings (Evidence-Based):**
- Read vs cat: 40% savings
- Grep vs grep command: 60% savings
- Glob vs find: 73% savings

**Real session impact:**
- 274K tokens (Bash) → 108K tokens (Native) = 61% savings

**Template {tools} field suggestions by domain:**
- **Backend:** Read, Write, Edit, Grep, Glob, Bash(git:*|npm:*|pip:*|dotnet:*)
- **Frontend:** Read, Write, Edit, Grep, Glob, Bash(npm:*)
- **QA:** Read, Grep, Glob, Bash(pytest:*|npm:test|dotnet:test)
- **Security:** Read, Grep, Glob, Bash(npm:audit|pip:check)
- **Deployment:** Read, Write, Bash(docker:*|kubectl:*|terraform:*)
- **Architecture:** Read, Write, Edit, Grep, Glob
- **Documentation:** Read, Write, Edit, Grep, Glob

---

## Model Selection Guidelines

**From Claude Code official patterns:**

- **haiku:** Simple, deterministic tasks (<10K tokens)
  - Examples: Validation, formatting, simple analysis, context checking
  - Use for: context-validator, deferral-validator, git-validator

- **sonnet:** Complex reasoning, code generation (10-50K tokens)
  - Examples: Implementation, design, security audits, test generation
  - Use for: backend-architect, frontend-developer, test-automator, security-auditor

- **opus:** Maximum capability (>50K tokens)
  - Rarely needed, extremely complex tasks only

- **inherit:** Match main conversation model
  - Use for: Adaptive behavior subagents (code-reviewer, refactoring-specialist)

**Template {model} field suggestions:**
- Backend/Frontend implementation: sonnet
- QA/Validation: haiku
- Code review/Refactoring: inherit
- Architecture/Design: sonnet
- Documentation: sonnet
- Deployment: sonnet

---

## Subagent Naming Conventions

**Pattern:** `[domain]-[function]` or `[function]-[specialization]`

**Examples:**
- backend-architect (domain-function)
- test-automator (function-specialization)
- api-designer (function-specialization)
- security-auditor (function-specialization)
- deployment-engineer (function-specialization)

**Rules:**
- Lowercase only
- Hyphens for word separation
- No underscores, spaces, or special characters
- Descriptive of subagent's role

---

**This reference provides framework context for generating DevForgeAI-aware subagents. All generated subagents must integrate with the patterns documented here.**
