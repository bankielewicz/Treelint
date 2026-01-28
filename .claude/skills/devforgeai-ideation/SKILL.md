---
name: devforgeai-ideation
description: Transform business ideas and problems into structured requirements through guided discovery, requirements elicitation, and feasibility analysis. Use when starting new projects (greenfield), planning features for existing systems (brownfield), or exploring solution spaces before architecture and development. Supports simple apps through multi-tier platforms via progressive complexity assessment.
allowed-tools:
  - Read
  - Write
  - Edit
  - Glob
  - Grep
  - AskUserQuestion
  - WebFetch
  - Bash(git:*)
  - Skill
  - TodoWrite
model: claude-opus-4-5-20251101
---

# DevForgeAI Ideation Skill

Transform raw business ideas, problems, and opportunities into structured, actionable requirements that drive spec-driven development with zero technical debt.

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

This skill serves as the **entry point** for the entire DevForgeAI framework. It transforms vague business ideas into concrete, implementable requirements through systematic discovery, requirements elicitation, complexity assessment, and feasibility analysis.

**Use BEFORE architecture and development skills.**

### Core Philosophy

**"Start with Why, Then What, Then How"**
- **Why:** Business value, user needs, success metrics
- **What:** Functional/non-functional requirements, constraints
- **How:** Technical approach (delegated to architecture skill)

**"Ask, Don't Assume"**
- Use AskUserQuestion for ALL ambiguities
- Never infer requirements from incomplete information
- Validate assumptions explicitly

**"Right-size the Solution"**
- Progressive complexity assessment (simple → enterprise)
- Don't over-engineer simple problems
- Don't under-architect complex platforms

---

## When to Use This Skill

### ✅ Trigger Scenarios

- User has business idea without technical specs
- Starting greenfield projects ("I want to build...")
- Adding major features to existing systems
- Exploring solution spaces and feasibility
- User requests requirements discovery or epic creation

### ❌ When NOT to Use

- Context files already exist (use devforgeai-architecture to update)
- Story-level work (use devforgeai-story-creation)
- Technical implementation (use devforgeai-development)

---

## Ideation Workflow (6 Phases)

**⚠️ EXECUTION STARTS HERE - You are now executing the skill's workflow.**

Each phase loads its reference file on-demand for detailed implementation.

### Phase 1: Discovery & Problem Understanding
**Reference:** `discovery-workflow.md` | **Questions:** 5-10 | **Output:** Problem statement, user personas, scope boundaries

**Step 0 - Context Marker Detection (from /ideate command):**

Before proceeding with discovery, check if context markers are available from command:

```
# Detect context markers passed from /ideate command Phase 2
IF context contains "**Business Idea:**":
  # Extract business idea from conversation context
  session.business_idea = extract_from_context("**Business Idea:**")
  session.context_provided = true

  Display:
  "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Context Received from Command
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✓ Business Idea: {session.business_idea}
  ✓ Project Mode: {extract_from_context('**Project Mode:**') or 'to be determined'}
  ✓ Brainstorm: {extract_from_context('**Brainstorm Context:**') or 'none'}

  Skipping redundant questions - context already provided."

ELSE:
  session.context_provided = false
  # Proceed with full discovery
```

**Context-Aware Discovery:** When context is provided:
- DO NOT ask for business idea (already provided in **Business Idea:** marker)
- DO NOT ask for project type if **Project Mode:** marker is present
- Validate/confirm context instead of re-asking

**Step 0.1 - Brainstorm Schema Validation (STORY-301):**

When brainstorm input is provided, validate against schema before processing:

```
IF $BRAINSTORM_CONTEXT is provided:
  # Schema validation for brainstorm document
  Read(file_path="src/claude/skills/devforgeai-orchestration/references/skill-output-schemas.yaml")

  # Validate brainstorm input against schema
  validation_result = validate_brainstorm_schema($BRAINSTORM_CONTEXT)

  IF validation_result.status == "FAILED":
    HALT workflow
    Display: "❌ Schema validation failed for brainstorm document"
    Display: validation_result.errors
    RETURN

  IF validation_result.status == "WARN":
    Display: "⚠️ Schema validation passed with warnings (legacy document)"
    # Proceed with degraded context preservation

  IF validation_result.status == "PASSED":
    Display: "✓ Schema validation passed for brainstorm"
```

**Step 0.2 - Brainstorm Handoff Detection:**

After schema validation, check if brainstorm context is available:

```
IF $BRAINSTORM_CONTEXT is provided (from /ideate command Phase 0):
  # Load brainstorm handoff reference
  Read(file_path=".claude/skills/devforgeai-ideation/references/brainstorm-handoff-workflow.md")

  # Pre-populate session from brainstorm
  session.problem_statement = $BRAINSTORM_CONTEXT.problem_statement
  session.user_personas = $BRAINSTORM_CONTEXT.user_personas
  session.constraints = $BRAINSTORM_CONTEXT.hard_constraints
  session.must_have_requirements = $BRAINSTORM_CONTEXT.must_have_capabilities

  # Display pre-population summary
  Display:
  "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Continuing from Brainstorm: {$BRAINSTORM_CONTEXT.brainstorm_id}
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Pre-populated:
    ✓ Problem: {problem_statement}
    ✓ Users: {len(user_personas)} persona(s)
    ✓ Constraints: {len(constraints)} identified
    ✓ Must-haves: {len(must_have_requirements)} capabilities

  Confidence: {$BRAINSTORM_CONTEXT.confidence_level}"

  IF $BRAINSTORM_CONTEXT.confidence_level == "HIGH":
    # Skip Phase 1 discovery, proceed to Requirements Elicitation
    Display: "→ Skipping discovery (HIGH confidence from brainstorm)"
    GOTO Requirements Elicitation Phase
  ELSE:
    # Shortened Phase 1 - validate only
    session.skip_discovery = true
    # Continue to Step 0.5 with validation-only questions

ELSE:
  # No brainstorm context - full discovery
  session.skip_discovery = false
```

**Step 0.5 - Load User Input Patterns (Error-Tolerant):**

Before proceeding with discovery questions, attempt to load guidance patterns:

`Read(file_path=".claude/skills/devforgeai-ideation/references/user-input-guidance.md")`

If load fails: Continue with standard discovery questions (no halt)

**Patterns Available (When Loaded):**
- Open-Ended Discovery ("Tell me about..."), Scope Verification
- Closed Confirmation (yes/no validation)
- Bounded Choice (predefined options for timelines, budgets)
- Explicit Classification (persona types, user roles)
- Comparative Ranking (feature priorities, 1-5 scale)

**Selective Loading Strategy:** Full file loads in Step 0.5 (~40% used in Phase 1, remainder for Phases 2-6). Reduces Phase 1 token overhead to acceptable levels.

**Step 1 - Discovery Execution:**

IF session.skip_discovery (from brainstorm):
  # Validate only - ask 1-3 confirmation questions
  AskUserQuestion: "Is the problem statement still accurate?"
  AskUserQuestion: "Any personas to add?"
ELSE:
  # Full discovery
  Determine project type (greenfield/brownfield), analyze existing system, explore problem space, define scope.

**Load:** `Read(file_path=".claude/skills/devforgeai-ideation/references/discovery-workflow.md")`

### Phase 2: Requirements Elicitation
**Reference:** `requirements-elicitation-workflow.md` + `requirements-elicitation-guide.md` | **Questions:** 10-60 | **Output:** Functional/NFR requirements, data models, integrations

Systematic questioning to extract user stories, data entities, external integrations, and quantified NFRs.

**Load:** `Read(file_path=".claude/skills/devforgeai-ideation/references/requirements-elicitation-workflow.md")`

### Phase 3: Complexity Assessment & Architecture Planning
**Reference:** `complexity-assessment-workflow.md` + `complexity-assessment-matrix.md` | **Output:** Complexity score (0-60), tier (1-4), tech recommendations

Score across 4 dimensions: Functional (0-20), Technical (0-20), Team/Org (0-10), NFR (0-10). Maps to architecture tier.

**Load:** `Read(file_path=".claude/skills/devforgeai-ideation/references/complexity-assessment-workflow.md")`

### Phase 4: Epic & Feature Decomposition
**Reference:** `epic-decomposition-workflow.md` + `domain-specific-patterns.md` | **Output:** 1-3 epics, 3-8 features/epic, roadmap

Break initiative into epics (4-8 week efforts), features (1-2 sprints), and high-level stories (1-5 days).

**Load:** `Read(file_path=".claude/skills/devforgeai-ideation/references/epic-decomposition-workflow.md")`

### Phase 5: Feasibility & Constraints Analysis
**Reference:** `feasibility-analysis-workflow.md` + `feasibility-analysis-framework.md` | **Output:** Feasibility assessment, risk register

Evaluate technical/business/resource feasibility, identify risks with mitigations, validate brownfield constraints.

**Load:** `Read(file_path=".claude/skills/devforgeai-ideation/references/feasibility-analysis-workflow.md")`

---

### Phase 6: Requirements Documentation & Handoff
**Workflow:** 3 sub-phases | **Output:** Epic documents, requirements spec (optional), completion summary

**6.1-6.3 Artifact Generation:** Generate epics, optional requirements spec, verify creation, transition to architecture
**Load:** `Read(file_path=".claude/skills/devforgeai-ideation/references/artifact-generation.md")`

**6.4 Self-Validation:** Validate artifacts, auto-correct issues, HALT on critical failures
**Load:** `Read(file_path=".claude/skills/devforgeai-ideation/references/self-validation-workflow.md")`

**6.5-6.6 Completion & Handoff:** Present summary, determine next action based on project mode
**Load:** `Read(file_path=".claude/skills/devforgeai-ideation/references/completion-handoff.md")`

**Phase 6.6 Mode-Based Next Actions (STORY-134):**
- **greenfield** mode → recommend `/create-context` to establish architecture constraints
- **brownfield** mode → recommend `/orchestrate` or `/create-sprint` for sprint planning

The command's **Mode:** marker is read in Phase 6.6 to determine appropriate next steps.

---

## AskUserQuestion Usage

**10-60 strategic questions** across 6 phases (Phase 1: 5-10, Phase 2: 10-60, Phases 3-6: 1-8 each). All question patterns, templates, and best practices in `user-interaction-patterns.md`.

**Load:** `Read(file_path=".claude/skills/devforgeai-ideation/references/user-interaction-patterns.md")`

---

## Error Handling

**6 error types** with detection logic and recovery procedures (self-heal → retry → report).

**Index:** `Read(file_path=".claude/skills/devforgeai-ideation/references/error-handling-index.md")`

**Error Type Files (load on-demand):**
1. **error-type-1-incomplete-answers.md** - Vague/incomplete user responses (Phase 2)
2. **error-type-2-artifact-failures.md** - File write/permission errors (Phase 6.1)
3. **error-type-3-complexity-errors.md** - Invalid scores, tier mismatch (Phase 3)
4. **error-type-4-validation-failures.md** - Quality issues, missing fields (Phase 6.4)
5. **error-type-5-constraint-conflicts.md** - Brownfield context conflicts (Phase 5)
6. **error-type-6-directory-issues.md** - Missing directories, permissions (Phase 6.1)

---

## Integration

**→ devforgeai-architecture** (greenfield: create context files) | **→ devforgeai-orchestration** (brownfield: sprint planning)

**Outputs:** Epic documents, requirements specs, complexity tier, technology recommendations

---

## Success Criteria

- [ ] Business problem defined (measurable)
- [ ] 1-3 epics with 3-8 features each
- [ ] Complexity scored (0-60, tier 1-4)
- [ ] Feasibility confirmed, risks mitigated
- [ ] Epic documents generated, validated
- [ ] Next action determined
- [ ] No critical ambiguities

**Token Budget:** ~35K-100K (isolated context)

---

## Reference Files

Load these on-demand during workflow execution:

### Phase Workflows (10 files - NEW)
- **discovery-workflow.md** - Phase 1: Problem understanding (274 lines)
- **requirements-elicitation-workflow.md** - Phase 2: Question flow (368 lines)
- **complexity-assessment-workflow.md** - Phase 3: Scoring algorithm (308 lines)
- **epic-decomposition-workflow.md** - Phase 4: Feature breakdown (309 lines)
- **feasibility-analysis-workflow.md** - Phase 5: Constraints check (378 lines)
- **artifact-generation.md** - Phase 6.1-6.3: Document generation (689 lines)
- **self-validation-workflow.md** - Phase 6.4: Quality checks (351 lines)
- **completion-handoff.md** - Phase 6.5-6.6: Summary and next action (721 lines)
- **user-interaction-patterns.md** - AskUserQuestion templates (411 lines)
- **error-handling-index.md** - Decision tree for error type selection (~100 lines)
- **error-type-1-incomplete-answers.md** - Vague user responses (~165 lines)
- **error-type-2-artifact-failures.md** - File write errors (~135 lines)
- **error-type-3-complexity-errors.md** - Complexity score issues (~155 lines)
- **error-type-4-validation-failures.md** - Quality validation issues (~210 lines)
- **error-type-5-constraint-conflicts.md** - Brownfield conflicts (~175 lines)
- **error-type-6-directory-issues.md** - Directory structure issues (~130 lines)

### Supporting Guides (8 files - existing)
- **requirements-elicitation-guide.md** - Domain-specific question patterns (659 lines)
- **complexity-assessment-matrix.md** - Complete 0-60 scoring rubric (617 lines)
- **domain-specific-patterns.md** - Decomposition patterns by domain (744 lines)
- **feasibility-analysis-framework.md** - Feasibility checklists (587 lines)
- **validation-checklists.md** - Quality validation procedures (604 lines)
- **output-templates.md** - Summary templates, tech recommendations (780 lines)
- **user-input-guidance.md** - Framework-internal guidance for eliciting complete requirements (897 lines)
  - Contains: 15 elicitation patterns, 28 AskUserQuestion templates, NFR quantification table
  - Section 5: Skill Integration Guide (devforgeai-ideation and devforgeai-story-creation patterns)
- **brainstorm-data-mapping.md** - Field mapping between brainstorm output and ideation input (419 lines)
  - Contains: 6 field mapping tables, transformation rules, phase behavior changes
  - Related: brainstorm-handoff-workflow.md (detection/selection) uses these mappings

**Total:** 24 reference files, ~10,500 lines (loaded progressively, not upfront)
**Error handling:** 7 files replace single 1,062-line file (token efficiency via selective loading)

---

## Best Practices

1. **Ask strategic questions** - User-guided discovery
2. **Progressive questioning** - Broad→specific (5→60 questions)
3. **Validate assumptions** - Confirm before documenting
4. **Document early risks** - Phase 5 feasibility analysis
5. **Clear handoff** - Next action: architecture or orchestration

---

**The goal:** Transform business ideas into structured, actionable requirements with zero ambiguity, enabling downstream skills to implement with zero technical debt.
