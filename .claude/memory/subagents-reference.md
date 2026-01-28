# DevForgeAI Subagents Reference

Detailed guidance for working with the 26 specialized subagents.

---

## Overview

Subagents are specialized AI workers with domain expertise that operate in isolated contexts. They are automatically invoked by DevForgeAI skills or can be explicitly called for specific tasks. **26 subagents** are available in `.claude/agents/`.

---

## Subagent Invocation Methods

### 1. Automatic Invocation (Proactive)

Subagents are automatically invoked by DevForgeAI skills at appropriate workflow phases:

**During devforgeai-development:**
- **Phase 1 (Red)**: test-automator generates failing tests from acceptance criteria
- **Phase 2 (Green)**: backend-architect or frontend-developer implements code to pass tests
- **Phase 2 (Validation)**: context-validator checks constraint compliance
- **Phase 3 (Refactor)**: refactoring-specialist improves code quality, code-reviewer provides feedback
- **Phase 4 (Integration)**: integration-tester creates cross-component tests

**During devforgeai-qa:**
- **Phase 0 Step 2.5**: deferral-validator validates deferred DoD items (MANDATORY)
- **Light Validation**: context-validator checks constraints
- **Deep Validation**: security-auditor scans for vulnerabilities, test-automator fills coverage gaps
- **Phase 5 Step 6**: qa-result-interpreter interprets results and generates user-facing display (NEW - QA Refactoring)

**During devforgeai-ui-generator:**
- **Phase 6 Step 3.5**: ui-spec-formatter formats and validates generated UI specifications (NEW - UI Refactoring)

**During devforgeai-architecture:**
- architect-reviewer validates architecture decisions
- api-designer defines API contract standards

**During devforgeai-release:**
- deployment-engineer handles infrastructure and deployment
- security-auditor performs pre-release security scan

### 2. Explicit Invocation

Invoke subagents directly using the Task tool with `subagent_type` parameter:

```
Task(
  subagent_type="test-automator",
  description="Generate tests for calculator",
  prompt="Generate comprehensive unit tests for a calculator class with add, subtract, multiply, and divide methods. Follow TDD principles and AAA pattern."
)
```

**Examples:**

```
# Code review
Task(
  subagent_type="code-reviewer",
  description="Review authentication code",
  prompt="Review the authentication implementation in src/auth/ for security issues, code quality, and adherence to coding standards."
)

# Frontend implementation
Task(
  subagent_type="frontend-developer",
  description="Implement login component",
  prompt="Implement a login form component in React following the design system in context files. Include email/password fields, validation, and API integration."
)

# Security audit
Task(
  subagent_type="security-auditor",
  description="Audit payment processing",
  prompt="Perform comprehensive security audit of payment processing code in src/payments/ focusing on PCI compliance and OWASP Top 10."
)

# Context validation
Task(
  subagent_type="context-validator",
  description="Validate constraints",
  prompt="Check all code changes for violations of the 6 context files (tech-stack, source-tree, dependencies, coding-standards, architecture-constraints, anti-patterns)."
)
```

### 3. Parallel Execution

Multiple subagents can run simultaneously for different tasks:

```
# Send single message with multiple Task tool calls
Task(subagent_type="test-automator", description="Generate tests", prompt="...")
Task(subagent_type="documentation-writer", description="Write API docs", prompt="...")

# Both execute in parallel, return results independently
```

---

## Available Subagents

| Subagent | Purpose | Model | Token Target | When to Use |
|----------|---------|-------|--------------|-------------|
| **test-automator** | TDD test generation (unit, integration, E2E) | sonnet | <50K | Implementing features, filling coverage gaps |
| **backend-architect** | Backend implementation (clean architecture, DDD) | sonnet | <50K | Implementing backend features, APIs, services |
| **frontend-developer** | Frontend implementation (React, Vue, Angular) | sonnet | <50K | Implementing UI components, state management |
| **context-validator** | Fast constraint enforcement (6 context files) | haiku | <5K | Before commits, after implementation |
| **code-reviewer** | Code quality and security review | inherit | <30K | After implementation, during refactoring |
| **security-auditor** | OWASP Top 10, auth/authz, vulnerability scanning | sonnet | <40K | After auth code, handling sensitive data |
| **deployment-engineer** | Infrastructure, IaC, CI/CD pipelines | sonnet | <40K | Release phase, deployment configuration |
| **requirements-analyst** | User story creation, acceptance criteria | sonnet | <30K | Epic decomposition, story planning |
| **story-requirements-analyst** | Story requirements (content-only, RCA-007 fix) | sonnet | <50K | devforgeai-story-creation Phase 2 (replaces general-purpose) |
| **documentation-writer** | Technical docs, API specs, user guides | sonnet | <30K | After API implementation, when coverage <80% |
| **architect-reviewer** | Architecture validation, design patterns | sonnet | <40K | After ADRs, major architectural changes |
| **refactoring-specialist** | Safe refactoring, code smell removal | inherit | <40K | When complexity >10, code duplication >5% |
| **integration-tester** | Cross-component testing, API contracts | sonnet | <40K | After unit tests pass, API endpoints ready |
| **api-designer** | REST/GraphQL/gRPC contract design | sonnet | <30K | Creating new APIs, ensuring consistency |
| **agent-generator** | Generate framework-aware Claude Code subagents (ENHANCED v2.0) | haiku | <50K | Creating subagents for DevForgeAI, command refactoring, custom domains |
| **deferral-validator** | Deferral justification validation, circular detection | haiku | <5K | Before commits (dev), before QA approval (qa) |
| **technical-debt-analyzer** | Debt trend analysis, pattern detection, reporting | sonnet | <30K | Sprint planning, retrospectives, debt reviews |
| **tech-stack-detector** | Technology detection and tech-stack.md validation | haiku | <10K | Development workflow init, architecture validation |
| **git-validator** | Git availability check, workflow strategy, and enhanced file analysis (RCA-008 Phase 2.5) | haiku | <5K | Before development workflows, release validation (enhanced with file categorization per RCA-008) |
| **qa-result-interpreter** | QA result interpretation and display generation | haiku | <8K | After QA report generation, before user display (NEW - QA Refactoring) |
| **sprint-planner** | Sprint creation and capacity validation | sonnet | <40K | Sprint planning, story selection, capacity validation (NEW - Sprint Refactoring) |
| **ui-spec-formatter** | UI spec validation and display generation | haiku | <10K | After UI spec generation, before user display (NEW - UI Refactoring 2025-11-05) |
| **code-analyzer** | Deep codebase analysis for documentation metadata | sonnet | <50K | Brownfield documentation, architecture discovery, gap analysis (NEW - STORY-040) |
| **internet-sleuth** | Research & competitive intelligence, web research automation | haiku | <50K | Market research, technology discovery, repository archaeology (AUTO-INVOKED by ideation) |
| **pattern-compliance-auditor** | Lean orchestration pattern compliance auditing | haiku | <15K | Command refactoring analysis, budget violation detection (/audit-budget command) |
| **dev-result-interpreter** | Development workflow result interpretation and display | haiku | <8K | After /dev completes, before result display (similar to qa-result-interpreter) |

---

### agent-generator v2.0 Enhancement (2025-11-15)

The **agent-generator** subagent has been significantly enhanced to be DevForgeAI framework-aware and Claude Code best practice compliant:

**New Capabilities:**
- **Phase 0: Framework Reference Loading** - Automatically loads claude-code-terminal-expert skill, CLAUDE.md, and lean-orchestration-pattern.md
- **Enhanced System Prompt Generation** - Uses Claude Code official patterns + DevForgeAI context for comprehensive subagent creation
- **Framework Compliance Validation** - 12-point validation (6 DevForgeAI + 6 Claude Code checks) with auto-fix logic
- **Reference File Generation** - Automatically creates framework guardrail files for command-related, domain-specific, and decision-making subagents

**Key Features:**
- **Claude Code Integration:** Leverages claude-code-terminal-expert skill for official subagent patterns
- **DevForgeAI Awareness:** References context files, quality gates, workflow states
- **Lean Orchestration Compliance:** Follows protocol for command refactoring subagents
- **Auto-Fix Logic:** Suggests and applies corrections for validation failures
- **Reference File Templates:** 4 types (command-refactoring, domain-constraints, decision-guidance, custom)

**Generated Subagents Now Include:**
- Framework Integration section (context files, quality gates, skill coordination)
- Tool Usage Protocol section (native tools mandate with 40-73% savings rationale)
- Enhanced token efficiency strategies
- Structured output contracts (for result-returning subagents)
- Framework constraint awareness

**Backward Compatibility:** ✅ Phase 2 requirements workflow unchanged

**File Size:** 2,343 lines (was 855 lines - 174% growth for comprehensive framework awareness)

**See:** `devforgeai/specs/enhancements/AGENT-GENERATOR-FRAMEWORK-AWARENESS-UPDATES.md` for complete enhancement details

---

## Subagent Integration with Skills

**devforgeai-development** uses:
- **git-validator** (Phase 01 Step 1) - Enhanced with RCA-008 file categorization (story_files, code, cache) and user consent recommendations
- **tech-stack-detector** (NEW - Phase 0 Step 7)
- test-automator → backend-architect/frontend-developer → context-validator → refactoring-specialist + code-reviewer (enhanced with deferral review) → integration-tester → **deferral-validator** (Phase 5 Step 1.5)
- requirements-analyst (when creating follow-up stories for deferrals)
- architect-reviewer (when creating ADRs for scope changes)

**devforgeai-qa** uses:
- **deferral-validator** (Phase 0 Step 2.5 - validates deferred DoD items)
- context-validator → security-auditor → test-automator (coverage gaps)
- **qa-result-interpreter** (NEW - Phase 5 Step 6 - interprets results and generates display)

**devforgeai-ui-generator** uses:
- **ui-spec-formatter** (NEW - Phase 6 Step 3.5 - formats and validates UI spec results)

**devforgeai-architecture** uses:
- architect-reviewer → api-designer

**devforgeai-release** uses:
- security-auditor → deployment-engineer

**devforgeai-orchestration** uses:
- requirements-analyst (epic feature decomposition, sprint planning)
- **technical-debt-analyzer** (NEW - Phase 4.5 during sprint planning/retrospectives)
- **sprint-planner** (NEW - Phase 3 sprint planning workflow)

**devforgeai-story-creation** uses:
- **story-requirements-analyst** (NEW - RCA-007 Phase 3) - Phase 2 (Requirements Analysis)
  - Skill-specific subagent for content-only output
  - Replaces general-purpose requirements-analyst
  - Cannot create files (no Write/Edit tools)
  - Returns markdown sections for assembly into story-template.md
- api-designer (conditional - Phase 3 if API endpoints detected)

---

## Autonomous Subagent Usage

**When to autonomously invoke subagents:**

1. **Context Validation**: Always use `context-validator` before git commits or after implementation
2. **Test Generation**: Use `test-automator` when implementing features (TDD Red phase)
3. **Code Review**: Use `code-reviewer` after implementation or refactoring
4. **Security Audits**: Use `security-auditor` after auth/security code or handling sensitive data
5. **Documentation**: Use `documentation-writer` after API implementation or when coverage <80%
6. **Architecture Review**: Use `architect-reviewer` after creating ADRs or major design changes
7. **Deferral Validation** (RCA-006): Always use `deferral-validator` when stories have deferred DoD items (dev Phase 6.1.5, QA Phase 0 Step 2.5)
8. **Technical Debt Analysis** (RCA-006): Use `technical-debt-analyzer` during sprint planning or when technical-debt-register.md updates
9. **QA Result Interpretation** (NEW - QA Refactoring): Always use `qa-result-interpreter` after QA report generation to prepare user-facing display
10. **UI Spec Formatting** (NEW - UI Refactoring 2025-11-05): Always use `ui-spec-formatter` after UI spec generation to validate and format results

---

## Subagent Context Isolation

- Each subagent operates in a separate context window
- Main conversation context is preserved (token efficiency)
- Subagents return results that integrate into main workflow
- No context leakage between parallel subagents

---

## Token Efficiency with Subagents

- Subagent work happens in isolated contexts
- Main conversation only pays invocation cost (~5-10K) + summary
- Total workflow can exceed 200K tokens across subagents without affecting main context
- **Example:** Full dev cycle (test-automator 50K + backend-architect 50K + code-reviewer 30K + integration-tester 40K = 170K) appears as ~15K in main conversation

---

## Subagent Best Practices

1. **Use specific, detailed prompts**: Subagents work best with clear instructions
2. **Reference context files**: Subagents respect tech-stack, source-tree, dependencies, etc.
3. **Specify success criteria**: Define what "done" looks like in the prompt
4. **Leverage parallelism**: Run independent subagents simultaneously for speed
5. **Check validation results**: context-validator blocks on violations, fix before proceeding
6. **Trust specialized expertise**: Subagents are domain experts (security, testing, architecture)

---

## Subagent File Locations

All subagents are defined in `.claude/agents/`:
- `test-automator.md` (546 lines)
- `backend-architect.md` (728 lines)
- `frontend-developer.md` (629 lines)
- `integration-tester.md` (502 lines)
- `context-validator.md` (356 lines)
- `code-reviewer.md` (enhanced with deferral review - Section 7)
- `security-auditor.md` (550 lines)
- `refactoring-specialist.md` (471 lines)
- `requirements-analyst.md` (473 lines)
- `architect-reviewer.md` (528 lines)
- `api-designer.md` (754 lines)
- `deployment-engineer.md` (820 lines)
- `documentation-writer.md` (519 lines)
- `agent-generator.md` (2,343 lines - ENHANCED 2025-11-15: Framework-aware v2.0)
- **`deferral-validator.md`** (NEW - 181 lines - RCA-006)
- **`technical-debt-analyzer.md`** (NEW - 172 lines - RCA-006)
- **`story-requirements-analyst.md`** (NEW - ~500 lines - RCA-007 Phase 3)

- **`tech-stack-detector.md`** (NEW - ~300 lines - Command Refactoring)
- **`git-validator.md`** (NEW - ~250 lines - Command Refactoring)
- **`qa-result-interpreter.md`** (NEW - 300 lines - QA Command Refactoring 2025-11-05)
- **`ui-spec-formatter.md`** (NEW - 507 lines - UI Command Refactoring 2025-11-05)

- **`internet-sleuth.md`** (NEW - ~800 lines - Auto-invoked by ideation for research)
- **`pattern-compliance-auditor.md`** (NEW - ~400 lines - Used by /audit-budget)
- **`dev-result-interpreter.md`** (NEW - ~300 lines - Used by /dev command)

**Total:** 26 subagents (14 original + 2 from RCA-006 + 2 from /dev refactoring + 1 from /qa refactoring + 1 from /create-sprint refactoring + 1 from RCA-007 Phase 3 + 1 from STORY-040 + 3 undocumented additions + 1 from /create-ui)

Each file contains complete system prompts with tool access, model selection, and execution patterns.
