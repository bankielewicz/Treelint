---
name: devforgeai-architecture
description: Create technical specifications, ADRs, and project context documentation that prevents technical debt. Use when designing system architecture, making technology decisions, or establishing project structure. Enforces spec-driven development by creating immutable constraint files (tech-stack.md, source-tree.md, dependencies.md) that AI agents must follow.
model: claude-opus-4-5-20251101
allowed-tools:
  - Read
  - Write
  - Edit
  - Glob
  - Grep
  - AskUserQuestion
  - WebFetch
  - Bash(git:*)
---

# DevForgeAI Architecture Skill

Create immutable context files and architecture documentation that prevents technical debt through explicit constraints.

---

## ⚠️ EXECUTION MODEL: This Skill Expands Inline

**After invocation, YOU (Claude) execute these instructions phase by phase.**

**When you invoke this skill:**
1. This SKILL.md content is now in your conversation
2. You execute each phase sequentially
3. You display results as you work through phases
4. You complete with success/failure report

**Do NOT:**
- ❌ Wait passively for skill to "return results"
- ❌ Assume skill is executing elsewhere
- ❌ Stop workflow after invocation

**Proceed to "Purpose" section below and begin execution.**

---

## Purpose

This skill creates the **architectural foundation** for projects: 6 required + 1 optional context files that define boundaries AI agents must never violate.

**Generated artifacts:**
- **6 Required Context Files** (immutable constraints in `devforgeai/specs/context/`)
- **1 Optional Context File** (design-system.md for UI projects)
- **ADRs** (architecture decisions in `devforgeai/specs/adrs/`)
- **Technical Specifications** (optional, in `devforgeai/specs/`)

**Core Principle:** Prevent technical debt through explicit, enforceable constraints.

**Philosophy:**
- Locked technologies (no library substitution without ADR)
- Explicit structure (files belong in defined locations)
- Approved dependencies (no unapproved packages)
- Enforced patterns (anti-patterns forbidden)
- Documented decisions (ADRs for traceability)

---

## When to Use This Skill

**Use when:**
- Starting new projects (create initial context)
- Making technology decisions (update tech-stack + create ADR)
- Defining project structure (create/update source-tree)
- Establishing coding standards
- Context files missing (auto-invoked by development skill)
- Brownfield projects need architectural documentation

**Prerequisites:**
- None (this is typically the first skill invoked)

**Invoked by:**
- `/create-context` command
- devforgeai-ideation skill (after requirements discovery)
- devforgeai-development skill (if context files missing)

---

## Architecture Workflow (5 Phases)

**⚠️ EXECUTION STARTS HERE - You are now executing the skill's workflow.**

Each phase loads its reference file on-demand for detailed implementation.

### Phase 1: Project Context Discovery

**Purpose:** Gather project information through strategic questions

**Reference:** `context-discovery-workflow.md`

Determine project type (greenfield vs brownfield), discover existing technologies/structure if brownfield, check for existing context files, analyze gaps.

**Step 0: Conditional User Input Guidance Loading [MANDATORY]**

Detect project mode and load guidance patterns conditionally:

```python
# Detect mode via context file count
context_files = Glob(pattern="devforgeai/specs/context/*.md")
context_count = len(context_files)

if context_count == 6:
    # Brownfield mode - all context files exist
    mode = "brownfield"
    Display: "Brownfield mode detected (6 context files exist). Skipping user-input-guidance.md."
    # Skip guidance, proceed with existing constraints

elif context_count == 0:
    # Greenfield mode - no context files exist
    mode = "greenfield"
    Display: "Greenfield mode detected (no context files). Loading user-input-guidance.md..."
    guidance = Read(file_path=".claude/skills/devforgeai-architecture/references/user-input-guidance.md")
    Display: "✓ Guidance loaded - applying patterns to Phase 1 questions"

else:
    # Partial greenfield - some files exist but not all
    mode = f"partial_greenfield ({context_count}/6 files)"
    Display: f"Partial context detected ({context_count}/6 files exist). Loading user-input-guidance.md to fill gaps..."
    guidance = Read(file_path=".claude/skills/devforgeai-architecture/references/user-input-guidance.md")
```

**Pattern Application (if guidance loaded):**
- **Step 1:** Use **Open-Ended Discovery** pattern for technology inventory question
- **Step 2:** Use **Explicit Classification** pattern for architecture style (4 options: Monolithic/Microservices/Serverless/Hybrid)
- **Step 3:** Use **Bounded Choice** pattern for framework selection (filtered by language)
- **Step 4:** Use **Bounded Choice** pattern for database system selection

**See:** `references/architecture-user-input-integration.md` for complete pattern mappings and examples.

**Load detailed workflow:**
```
Read(file_path=".claude/skills/devforgeai-architecture/references/context-discovery-workflow.md")
```

---

### Phase 2: Create Immutable Context Files

**Purpose:** Generate 6 required + 1 optional context files from templates

**Reference:** `context-file-creation-workflow.md`

Load template for each file from `assets/context-templates/`, gather decisions via AskUserQuestion, customize with project-specific info, add enforcement rules (✅/❌ examples), write to `devforgeai/specs/context/`.

**Output:**
- **Required (6):** tech-stack.md, source-tree.md, dependencies.md, coding-standards.md, architecture-constraints.md, anti-patterns.md
- **Optional (1):** design-system.md (UI projects only)

**This phase was 52% of the original SKILL.md - now progressively loaded.**

**Load detailed workflow:**
```
Read(file_path=".claude/skills/devforgeai-architecture/references/context-file-creation-workflow.md")
```

---

### Phase 3: Create Architecture Decision Records

**Purpose:** Document significant technical decisions

**Reference:** `adr-creation-workflow.md` | **Policy:** `adr-policy.md` | **Template:** `adr-template.md`

Identify decisions requiring ADRs (database, ORM, framework, patterns), load template and examples from `assets/adr-examples/`, create ADR with context/decision/rationale/consequences/alternatives/enforcement sections.

**Output:** ADR files in `devforgeai/specs/adrs/`

**Load detailed workflow:**
```
Read(file_path=".claude/skills/devforgeai-architecture/references/adr-creation-workflow.md")
```

---

### Phase 4: Create Technical Specifications

**Purpose:** Generate high-level architecture documentation

**Reference:** `technical-specification-workflow.md` | **Patterns:** `system-design-patterns.md`

Create functional specs (use cases, business rules, data models), API specs (endpoints, auth), database specs (schemas, indexes), NFRs (performance, security). Use AskUserQuestion for ambiguous requirements.

**Output:** Technical spec in `devforgeai/specs/` (optional)

**Load detailed workflow:**
```
Read(file_path=".claude/skills/devforgeai-architecture/references/technical-specification-workflow.md")
```

---

### Phase 5: Validate Spec Against Context

**Purpose:** Ensure specifications respect all constraints

**Reference:** `architecture-validation.md`

Load all 6 context files, validate spec compliance (technologies, packages, structure, layer boundaries, anti-patterns). Use AskUserQuestion to resolve conflicts.

**Output:** Validated specification ready for implementation

**Load detailed workflow:**
```
Read(file_path=".claude/skills/devforgeai-architecture/references/architecture-validation.md")
```

---

## Ambiguity Detection

**CRITICAL:** Use AskUserQuestion for ANY ambiguity - technology choices unclear, multiple valid options, conflicting requirements, version/security/performance/compliance decisions.

**See `references/ambiguity-detection-guide.md` for complete scenarios.**

---

## Brownfield Projects

Existing codebases require discovery → gap analysis → migration strategy decision (gradual/full refactor/accept current) → transitional context files.

**See `references/brownfield-integration.md` for complete workflow.**

---

## Integration with Other Skills

**From:** devforgeai-ideation (requirements → architecture)
**To:** devforgeai-orchestration (story planning), devforgeai-development (implementation)
**Provides:** 6 context files (enforced by all skills), ADRs (traceability), Technical specs (guidance)

---

## Asset Templates

**Context Templates (6 files, 3,922 lines):**
- tech-stack.md, source-tree.md, dependencies.md, coding-standards.md, architecture-constraints.md, anti-patterns.md

**ADR Examples (6 files, 5,157 lines):**
- Database selection, ORM selection, State management, Clean Architecture, Deployment strategy, Scope changes

**All templates in `assets/` load on-demand.**

---

## Reference Files

**Workflow Files (6 files - Load per phase):**
- context-discovery-workflow.md, context-file-creation-workflow.md, adr-creation-workflow.md, technical-specification-workflow.md, architecture-validation.md, brownfield-integration.md

**Guide Files (4 files - Load as needed):**
- adr-policy.md, adr-template.md, ambiguity-detection-guide.md, system-design-patterns.md

---

## Scripts

- `scripts/init_context.sh` - Initialize context files for new projects
- `scripts/validate_spec.py` - Validate spec against existing context

---

## Success Criteria

Architecture phase complete when:

- [ ] All 6 required context files exist in `devforgeai/specs/context/`
- [ ] Optional design-system.md created if UI project
- [ ] Context files non-empty (no placeholders)
- [ ] At least 1 ADR created (initial architecture decision)
- [ ] All ambiguities resolved (via AskUserQuestion)
- [ ] Validation passes (Phase 5)
- [ ] Ready for story planning (next: devforgeai-orchestration)

**The goal:** Zero ambiguity = Zero technical debt from wrong assumptions.
