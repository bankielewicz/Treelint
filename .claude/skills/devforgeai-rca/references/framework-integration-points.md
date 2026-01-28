# Framework Integration Points for DevForgeAI RCA

**Purpose:** Guide for understanding DevForgeAI framework structure to identify breakdown root causes

**Target Audience:** devforgeai-rca skill (Claude executing RCA Phases 1-4)

---

## Table of Contents

1. [DevForgeAI Context Files](#devforgeai-context-files)
2. [Quality Gates](#quality-gates)
3. [Workflow States](#workflow-states)
4. [Lean Orchestration Pattern](#lean-orchestration-pattern)
5. [Common Breakdown Categories](#common-breakdown-categories)
6. [Evidence Location by Breakdown Type](#evidence-location-by-breakdown-type)

---

## DevForgeAI Context Files

### The 6 Immutable Constraint Files

**Location:** `devforgeai/specs/context/`

**Files:**
1. **tech-stack.md** - LOCKED technology choices
2. **source-tree.md** - Project structure rules
3. **dependencies.md** - Approved packages
4. **coding-standards.md** - Code patterns
5. **architecture-constraints.md** - Layer boundaries
6. **anti-patterns.md** - Forbidden patterns

### Context File Breakdown Patterns

**Pattern 1: Technology Substitution**
```
Issue: Code uses unapproved technology
Root Cause: tech-stack.md constraint not enforced
Evidence Location: tech-stack.md + implementation code + context-validator
```

**Pattern 2: Structure Violation**
```
Issue: Files in wrong directories
Root Cause: source-tree.md rules not validated
Evidence Location: source-tree.md + file system + validation code
```

**Pattern 3: Dependency Violation**
```
Issue: Unapproved package added
Root Cause: dependencies.md check missing
Evidence Location: dependencies.md + package.json/requirements.txt + validator
```

**Pattern 4: Anti-Pattern Introduction**
```
Issue: God Object (>500 lines) created
Root Cause: anti-patterns.md not enforced
Evidence Location: anti-patterns.md + violating file + QA validation
```

### What to Check

**For context file issues:**
- [ ] Does context file exist?
- [ ] Does constraint exist in file?
- [ ] Does implementation violate constraint?
- [ ] Is validator supposed to catch this?
- [ ] Was validator invoked?
- [ ] If invoked, why didn't it catch?

**Evidence to collect:**
1. Context file with constraint definition
2. Implementation showing violation
3. Validator code (should catch but didn't)
4. Workflow that should invoke validator
5. Why validator wasn't invoked

---

## Quality Gates

### The 4 Quality Gates

**Gate 1: Context Validation** (Architecture → Ready for Dev)
- Criteria: All 6 context files exist and non-empty
- Validated by: devforgeai-architecture skill
- Blocks: Development if files missing

**Gate 2: Test Passing** (Dev Complete → QA In Progress)
- Criteria: Build succeeds, all tests pass (100% pass rate)
- Validated by: devforgeai-development skill (light QA)
- Blocks: QA if tests failing

**Gate 3: QA Approval** (QA Approved → Releasing)
- Criteria: Coverage meets thresholds, zero CRITICAL/HIGH violations
- Validated by: devforgeai-qa skill (deep validation)
- Blocks: Release if quality insufficient

**Gate 4: Release Readiness** (Releasing → Released)
- Criteria: QA approved, all checkboxes complete, no blocking dependencies
- Validated by: devforgeai-release skill
- Blocks: Production deployment if not ready

### Quality Gate Breakdown Patterns

**Pattern 1: Gate Bypass**
```
Issue: Story progressed past gate without meeting criteria
Root Cause: Gate validation returned PASS incorrectly
Evidence Location: Gate definition + validation code + story state
```

**Pattern 2: Gate Criteria Wrong**
```
Issue: Gate validated wrong metric
Root Cause: Criteria definition incorrect or outdated
Evidence Location: Gate documentation + validation logic + metric definition
```

**Pattern 3: Gate Not Enforced**
```
Issue: Gate validation skipped
Root Cause: Workflow doesn't include gate check
Evidence Location: Workflow state transitions + orchestration skill
```

### What to Check

**For quality gate issues:**
- [ ] Which gate was involved? (1, 2, 3, or 4)
- [ ] What are the gate criteria? (from CLAUDE.md or protocols)
- [ ] Was gate validation executed?
- [ ] If executed, what was the result?
- [ ] If PASS, why incorrect?
- [ ] If not executed, why skipped?

**Evidence to collect:**
1. Gate definition (CLAUDE.md or quality-gates.md)
2. Story file showing state before/after gate
3. Validation code for gate
4. Workflow history (was gate transition recorded?)

---

## Workflow States

### The 11 Workflow States

```
1. Backlog
2. Architecture
3. Ready for Dev
4. In Development
5. Dev Complete
6. QA In Progress
7. QA Failed (terminal)
8. QA Approved
9. Releasing
10. Released
11. Cancelled (terminal)
```

### Valid State Transitions

```
Backlog → Architecture → Ready for Dev → In Development → Dev Complete →
QA In Progress → [QA Approved | QA Failed] → Releasing → Released
```

**Terminal states:** QA Failed, Cancelled (no forward transitions)

### Workflow State Breakdown Patterns

**Pattern 1: Invalid Transition**
```
Issue: Story jumped states (Backlog → In Development, skipping intermediate)
Root Cause: State transition validation missing
Evidence Location: Story file + orchestration skill + state-transitions.md
```

**Pattern 2: Stuck State**
```
Issue: Story stuck in state (never progresses)
Root Cause: Criteria to exit state never met
Evidence Location: Story file + workflow state definition + completion criteria
```

**Pattern 3: Rollback Without Terminal**
```
Issue: Story regressed (QA Approved → In Development) without going through QA Failed
Root Cause: State transition allows invalid rollback
Evidence Location: Story workflow history + state-transitions.md
```

### What to Check

**For workflow state issues:**
- [ ] What was the current state?
- [ ] What was the previous state?
- [ ] Is transition valid? (check state-transitions.md)
- [ ] Was transition properly recorded? (workflow history section)
- [ ] What criteria were checked?
- [ ] Who/what updated the state? (skill invocation)

**Evidence to collect:**
1. Story file YAML frontmatter (current state)
2. Story Workflow History section (transitions)
3. State transition documentation
4. Skill that manages state (orchestration)
5. Quality gate definitions (if gate-related transition)

---

## Lean Orchestration Pattern

### Pattern Definition

**Commands orchestrate. Skills validate. Subagents specialize.**

**Character budget:** Commands must be <15K chars

**Responsibilities:**
- **Commands:** Argument parsing, context loading, skill invocation, result display
- **Skills:** Business logic, workflow execution, subagent coordination, validation
- **Subagents:** Specialized tasks in isolated contexts

### Lean Orchestration Violations

**Violation Type 1: Command with Business Logic**
```
Issue: Command contains validation algorithms, report parsing, decision logic
Root Cause: Logic not extracted to skill during implementation/refactoring
Evidence Location: Command .md file + lean-orchestration-pattern.md
```

**Violation Type 2: Command Over Budget**
```
Issue: Command >15K characters
Root Cause: Too much documentation, business logic, or templates in command
Evidence Location: Command .md file + wc -c output
```

**Violation Type 3: Skills-First Violation**
```
Issue: Command invokes subagent directly (Command → Subagent, bypassing Skill)
Root Cause: No skill layer for coordination
Evidence Location: Command .md (subagent invocation) + missing skill integration
```

**Violation Type 4: Skill with User Interaction**
```
Issue: Skill uses AskUserQuestion for UX decisions
Root Cause: User interaction belongs in command, not skill
Evidence Location: Skill SKILL.md (AskUserQuestion calls) + pattern documentation
```

### What to Check

**For lean orchestration issues:**
- [ ] Is command <15K chars? (`wc -c < command.md`)
- [ ] Does command have business logic? (validation, parsing, decisions)
- [ ] Does command invoke skill? (Skill(command="..."))
- [ ] Does command display pre-generated results? (no parsing)
- [ ] Does skill contain workflow logic?
- [ ] Does skill invoke subagents for specialized tasks?

**Evidence to collect:**
1. Command .md file (check phases, character count)
2. Skill SKILL.md (check if logic should be here)
3. lean-orchestration-pattern.md (pattern definition)
4. Similar refactored commands (reference implementations)

---

## Common Breakdown Categories

### Category 1: Validation Missing

**Symptoms:**
- Workflow proceeds without checking preconditions
- Constraints violated without detection
- Invalid state transitions allowed

**Root Causes:**
- Validation step not in workflow
- Validator exists but not invoked
- Validator broken or incomplete

**Evidence to collect:**
- Workflow file (skill/command)
- Validator code (subagent or skill phase)
- Expected validation (from protocol/documentation)

**Example RCAs:**
- RCA-010 (example): Context file validation missing

### Category 2: Autonomous Operations

**Symptoms:**
- Framework makes decisions without user input
- User approval required but not obtained
- Autonomous deferrals, commits, stashes

**Root Causes:**
- AskUserQuestion not invoked
- Pre-existing approvals accepted without challenge
- Approval checkpoint missing from workflow

**Evidence to collect:**
- Workflow file (where approval should be)
- User approval pattern (from CLAUDE.md Critical Rule #11)
- Story file or git history (autonomous operation evidence)

**Example RCAs:**
- RCA-006: Autonomous deferrals
- RCA-008: Autonomous git stashing

### Category 3: Progressive Disclosure Violation

**Symptoms:**
- All reference files loaded at once
- Token usage exceeds targets
- Skill loads unnecessary context

**Root Causes:**
- Instructions say "Read all references"
- No conditional loading logic
- Skill not refactored for progressive disclosure

**Evidence to collect:**
- Skill SKILL.md (Read instructions)
- Reference files (which ones loaded)
- Token usage (from conversation history)
- Progressive disclosure pattern (from refactored skills)

**Example RCAs:**
- Skills created before progressive disclosure pattern

### Category 4: Subagent Framework Silo

**Symptoms:**
- Subagent violates DevForgeAI constraints
- Subagent unaware of context files
- Subagent doesn't follow quality gates

**Root Causes:**
- Subagent system prompt lacks framework awareness
- No reference file for framework constraints
- Subagent created before framework-aware pattern

**Evidence to collect:**
- Subagent .md file (system prompt)
- Framework-aware pattern (from agent-generator v2.0)
- When subagent was created (git log)
- Reference file (if exists)

**Example RCAs:**
- RCA-007: Multi-file story creation (subagent not constrained)

### Category 5: Documentation Gap

**Symptoms:**
- Users confused about framework behavior
- Expected behavior not documented
- Instructions unclear or missing

**Root Causes:**
- Documentation never created
- Documentation outdated
- Documentation in wrong location

**Evidence to collect:**
- Relevant documentation file (or absence)
- User confusion point (conversation history)
- Where documentation should be (CLAUDE.md, memory files, protocols)

### Category 6: Template/Pattern Violation

**Symptoms:**
- Component doesn't follow established pattern
- Inconsistent architecture across components
- Pattern applied incorrectly

**Root Causes:**
- Pattern not known when component created
- Pattern documentation unclear
- Pattern not enforced

**Evidence to collect:**
- Component showing violation
- Pattern documentation (lean-orchestration-pattern.md, etc.)
- Reference implementation (commands/skills that follow pattern)

**Example RCAs:**
- RCA-009: Skill execution incomplete workflow (pattern not followed)

---

## Evidence Location by Breakdown Type

### Skill Breakdown → Read These Files

**Primary:**
1. `.claude/skills/{skill}/SKILL.md` - Main skill workflow
2. `.claude/skills/{skill}/references/*.md` - Reference files loaded by skill

**Secondary:**
3. `.claude/agents/{subagent}.md` - Subagents invoked by skill
4. Story files (if skill operates on stories)
5. Context files (if skill should validate constraints)

**Tertiary:**
6. Related skills (if skill invokes others)
7. Commands that invoke this skill
8. Existing RCAs for this skill

**Example: devforgeai-development breakdown**
```
Read:
1. .claude/skills/devforgeai-development/SKILL.md (primary)
2. .claude/skills/devforgeai-development/references/*.md (loaded refs)
3. .claude/agents/test-automator.md (invoked in Phase 1)
4. .claude/agents/backend-architect.md (invoked in Phase 2)
5. .claude/commands/dev.md (command that invokes skill)
6. devforgeai/RCA/RCA-009-skill-execution-incomplete-workflow.md (related)
```

### Command Breakdown → Read These Files

**Primary:**
1. `.claude/commands/{command}.md` - Command file

**Secondary:**
2. Skill invoked by command (`.claude/skills/{skill}/SKILL.md`)
3. `devforgeai/protocols/lean-orchestration-pattern.md` - Pattern definition

**Tertiary:**
4. Reference commands (refactored examples)
5. `.claude/memory/commands-reference.md` - Command documentation
6. Existing RCAs for this command

**Example: /dev breakdown**
```
Read:
1. .claude/commands/dev.md (primary)
2. .claude/skills/devforgeai-development/SKILL.md (invoked skill)
3. devforgeai/protocols/lean-orchestration-pattern.md (pattern)
4. .claude/commands/qa.md (reference implementation)
5. .claude/memory/commands-reference.md (command docs)
```

### Subagent Breakdown → Read These Files

**Primary:**
1. `.claude/agents/{subagent}.md` - Subagent system prompt

**Secondary:**
2. Reference file for subagent (if exists in skill/references/)
3. Skills that invoke this subagent

**Tertiary:**
4. `.claude/agents/agent-generator.md` - Subagent pattern definition
5. `.claude/memory/subagents-reference.md` - Subagent documentation
6. Similar subagents (reference implementations)

**Example: deferral-validator breakdown**
```
Read:
1. .claude/agents/deferral-validator.md (primary)
2. .claude/skills/devforgeai-development/SKILL.md (invokes in Phase 4.5)
3. .claude/skills/devforgeai-qa/SKILL.md (invokes in Phase 0)
4. devforgeai/RCA/RCA-006-autonomous-deferrals.md (why created)
```

### Context File Violation → Read These Files

**Primary:**
1. Violated context file (`devforgeai/specs/context/{file}.md`)
2. Implementation showing violation

**Secondary:**
3. `.claude/agents/context-validator.md` - Should have caught
4. Skill that should have invoked validator

**Tertiary:**
5. Related ADRs (if constraint has ADR)
6. `CLAUDE.md` - Context file descriptions

**Example: tech-stack.md violation**
```
Read:
1. devforgeai/specs/context/tech-stack.md (constraint)
2. src/components/App.vue (violation - uses Vue, not React)
3. .claude/agents/context-validator.md (validator)
4. .claude/skills/devforgeai-development/SKILL.md (should invoke validator)
```

### Quality Gate Bypass → Read These Files

**Primary:**
1. Gate definition (`CLAUDE.md` or `devforgeai/protocols/quality-gates.md`)
2. Story file showing state before/after gate

**Secondary:**
3. Skill that validates gate (devforgeai-qa, devforgeai-development, etc.)
4. Validation code in skill

**Tertiary:**
5. `devforgeai/protocols/workflow-states.md` - State transition rules
6. Related RCAs (other gate bypasses)

### Workflow State Error → Read These Files

**Primary:**
1. Story file (`devforgeai/specs/Stories/{STORY-ID}.story.md`) - YAML + Workflow History
2. `devforgeai/protocols/workflow-states.md` - State definitions

**Secondary:**
3. `devforgeai/protocols/state-transitions.md` - Valid transitions
4. `.claude/skills/devforgeai-orchestration/SKILL.md` - State management

**Tertiary:**
5. Skills that update state (development, qa, release)
6. Commands that trigger state changes

---

## Breakdown Investigation Cheat Sheet

**Use this quick reference during Phase 1 (Auto-Read Files):**

### Quick Diagnosis

**Issue type: "Command doesn't work"**
```
Read:
1. .claude/commands/{command}.md
2. Skill invoked by command
3. lean-orchestration-pattern.md
```

**Issue type: "Skill doesn't validate X"**
```
Read:
1. .claude/skills/{skill}/SKILL.md
2. Validator subagent (if should exist)
3. Context file with constraint X
```

**Issue type: "Story in wrong state"**
```
Read:
1. devforgeai/specs/Stories/{STORY-ID}.story.md
2. workflow-states.md
3. state-transitions.md
4. devforgeai-orchestration SKILL.md
```

**Issue type: "Test coverage too low"**
```
Read:
1. Story Definition of Done
2. devforgeai-development SKILL.md (TDD phases)
3. test-automator subagent
4. coverage thresholds (from CLAUDE.md)
```

**Issue type: "Autonomous operation"**
```
Read:
1. Component that operated autonomously
2. CLAUDE.md Critical Rules (especially #11 for git)
3. User approval pattern (RCA-006, RCA-008)
4. AskUserQuestion usage in workflow
```

**Issue type: "Character budget exceeded"**
```
Read:
1. Command .md file (wc -c)
2. lean-orchestration-pattern.md (budget limits)
3. Refactored command example (qa.md, create-sprint.md)
4. command-budget-reference.md
```

**Issue type: "Performance degraded"**
```
Read:
1. Component with performance issue
2. Token efficiency guidelines (token-efficiency.md)
3. Native tools vs Bash (native-tools-vs-bash-efficiency-analysis.md)
4. Progressive disclosure pattern
```

---

## Framework Component Relationships

### Skill → Subagent Relationships

**Invocation patterns:**
```
devforgeai-development:
  Phase 0: git-validator, tech-stack-detector
  Phase 1: test-automator
  Phase 2: backend-architect OR frontend-developer
  Phase 3: context-validator, refactoring-specialist, code-reviewer
  Phase 4: integration-tester
  Phase 4.5: deferral-validator

devforgeai-qa:
  Phase 0: deferral-validator
  Phase 2: context-validator
  Phase 3: security-auditor
  Phase 4: test-automator (coverage gaps)
  Phase 5: qa-result-interpreter

devforgeai-orchestration:
  Epic creation: requirements-analyst, architect-reviewer
  Sprint planning: sprint-planner
  Deferral tracking: technical-debt-analyzer
```

**When to check:** If subagent should have been invoked but wasn't

### Command → Skill Relationships

```
/dev → devforgeai-development
/qa → devforgeai-qa
/release → devforgeai-release
/orchestrate → devforgeai-orchestration
/create-story → devforgeai-story-creation
/create-ui → devforgeai-ui-generator
/create-context → devforgeai-architecture
/ideate → devforgeai-ideation
/create-epic → devforgeai-orchestration (epic mode)
/create-sprint → devforgeai-orchestration (sprint mode)
```

**When to check:** If command didn't invoke expected skill

### Skill → Skill Relationships

```
devforgeai-development:
  Auto-invokes: devforgeai-qa (light mode) after each TDD phase

devforgeai-ideation:
  Auto-transitions: devforgeai-architecture (if context files missing)

devforgeai-orchestration:
  Coordinates: All other skills based on workflow phase
```

**When to check:** If skill coordination failed

---

## RCA Pattern Library

### Patterns from Existing RCAs

**RCA-006 Pattern: Pre-Existing Approvals Bypass**
```
Problem: Pre-existing approvals accepted without re-validation
Root Cause: Workflow assumes pre-existing = user-approved
Solution: Challenge ALL items, regardless of pre-existing state
Implementation: Add challenge checkpoint (Phase 4.5)
```

**RCA-007 Pattern: Subagent File Creation**
```
Problem: Subagent creates files (should only return content)
Root Cause: Subagent has Write tool access
Solution: Create skill-specific subagent without Write/Edit
Implementation: New subagent with Read/Grep/Glob only
```

**RCA-008 Pattern: Autonomous Git Operations**
```
Problem: Git operations executed without user approval
Root Cause: No approval checkpoint before destructive git commands
Solution: AskUserQuestion before stash, reset, force push
Implementation: Add git approval protocol (CLAUDE.md Critical Rule #11)
```

**RCA-009 Pattern: Skill Execution Incomplete**
```
Problem: Skill invoked but workflow stopped prematurely
Root Cause: Misunderstanding of skill execution model (inline vs async)
Solution: Clarify execution model in documentation
Implementation: Add "Execution Model" section to all skills
```

### Pattern Application

**When similar issue occurs:**
1. Search existing RCAs for similar pattern
2. Check if same root cause type
3. Apply proven solution pattern
4. Reference original RCA in "Related RCAs"

---

## Cross-Reference Tables

### Breakdown Type → Files to Examine

| Breakdown Type | Primary Files | Secondary Files | Pattern RCA |
|----------------|---------------|-----------------|-------------|
| Validation Missing | Skill SKILL.md | Validator subagent, Context file | RCA-010 (example) |
| Autonomous Operation | Skill/Command, CLAUDE.md | User approval pattern | RCA-006, RCA-008 |
| Quality Gate Bypass | Gate definition, Story file | Validation code | N/A |
| Workflow State Error | Story file, state-transitions.md | Orchestration skill | N/A |
| Context Violation | Context file, Implementation | context-validator | N/A |
| Lean Violation | Command .md, lean-orchestration-pattern.md | Refactored commands | RCA-009 |
| Subagent Silo | Subagent .md, agent-generator.md | Framework-aware pattern | RCA-007 |

### Component Type → Evidence Location

| Component | Primary Evidence | Secondary Evidence | Framework Context |
|-----------|------------------|---------------------|-------------------|
| Skill | SKILL.md | references/*.md, invoked subagents | skills-reference.md |
| Command | command.md | Invoked skill, pattern doc | commands-reference.md |
| Subagent | subagent.md | Reference file, invoking skills | subagents-reference.md |
| Context File | context file | Validator, ADRs | context-files-guide.md |
| Quality Gate | CLAUDE.md | Validation code, story file | qa-automation.md |

---

## Reference

**Core Framework Documentation:**
- `CLAUDE.md` - Framework overview, critical rules, quality gates
- `.claude/memory/skills-reference.md` - All skills
- `.claude/memory/subagents-reference.md` - All subagents
- `.claude/memory/commands-reference.md` - All commands
- `.claude/memory/context-files-guide.md` - Context file details

**Protocols:**
- `devforgeai/protocols/lean-orchestration-pattern.md` - Command/skill architecture
- `devforgeai/protocols/workflow-states.md` - State definitions
- `devforgeai/protocols/state-transitions.md` - Valid transitions
- `devforgeai/protocols/quality-gates.md` - Gate criteria

**RCA Examples:**
- `devforgeai/RCA/RCA-006-autonomous-deferrals.md`
- `devforgeai/RCA/RCA-007-multi-file-story-creation.md`
- `devforgeai/RCA/RCA-008-autonomous-git-stashing.md`
- `devforgeai/RCA/RCA-009-skill-execution-incomplete-workflow.md`

---

**End of Framework Integration Points**

**Total: ~1,000 lines**
