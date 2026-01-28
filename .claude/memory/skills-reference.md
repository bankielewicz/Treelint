# DevForgeAI Skills Reference

Detailed guidance for working with 16 functional skills (14 devforgeai-* + 2 Claude Code infrastructure skills), plus 1 incomplete skill (internet-sleuth-integration).

**Skills breakdown:**
- 14 DevForgeAI workflow skills (devforgeai-*)
- 2 Claude Code infrastructure skills (claude-code-terminal-expert, skill-creator)
- 1 incomplete (internet-sleuth-integration - use internet-sleuth subagent instead)

---

## CRITICAL: Skill Execution Model

**Skills expand inline. YOU execute the instructions.**

### After Invoking a Skill

When you use `Skill(command="devforgeai-[name]")`:

**Step 1: Skill Content Expands**
- The skill's SKILL.md file is injected into conversation
- You now have access to skill's workflow instructions

**Step 2: Execute Skill Workflow**
- Read the skill's Phase 01 instructions
- Execute Phase 01 (validation, setup)
- Display Phase 01 results
- Continue to Phase 02
- Execute each phase sequentially
- Display results as you go

**Step 3: Complete Workflow**
- Execute all phases through completion
- Display final success/failure report
- Update files as skill instructs (story status, etc.)

### Common Mistake: Passive Waiting

**❌ WRONG:**
```
Skill(command="devforgeai-development")
"The skill is running, I'll wait for results"

[Stops and waits] ← THIS IS WRONG
```

**✅ CORRECT:**
```
Skill(command="devforgeai-development")
"Skill expanded, now executing its instructions"

[Reads Phase 01 from expanded skill content]

[Executes Phase 01: Git validation]

[Displays Phase 01 results]

[Continues to Phase 02...]

[Completes all phases]

[Displays final report]
```

### Skills vs Subagents

| Aspect | Skills | Subagents |
|--------|--------|--------------|
| **Tool** | Skill tool | Task tool |
| **Execution** | You execute inline | Agent executes isolated |
| **Output** | You produce by following instructions | Agent returns structured result |
| **Waiting** | Never wait (you're doing the work) | Wait for agent result |

**See also:**
- `CLAUDE.md` - Section "CRITICAL: How Skills Work" (complete explanation)
- `.claude/memory/skill-execution-troubleshooting.md` (emergency recovery procedures)

---

## CWD Validation Best Practices

Skills use **relative paths** (e.g., `devforgeai/specs/Stories/`). These paths assume CWD is project root.

### Before File Operations

1. **Verify CWD is project root:**
   ```
   Read(file_path="CLAUDE.md")  # Must succeed
   # Content must contain "DevForgeAI"
   ```

2. **If CLAUDE.md missing, check secondary markers:**
   ```
   Glob(pattern=".claude/skills/*.md")      # Or
   Glob(pattern="devforgeai/specs/context/*.md")
   ```

3. **If validation fails → HALT:**
   - Do NOT proceed with file operations
   - Use `AskUserQuestion` to get correct path

### Glob Tool Behavior

- **Always recursive** - no way to limit depth
- **Use `path` param** with absolute paths for reliability:
  ```
  Glob(pattern="*.md", path="/absolute/path/.claude/rules/core")
  ```
- **For single files** - use `Read()` instead (no recursion)

### Pattern Syntax

| Syntax | Works | Example |
|--------|-------|---------|
| `*` wildcard | ✅ | `*.md` |
| `**` recursive | ✅ Always ON | Already implicit |
| `[...]` char class | ✅ | `STORY-10[0-5]*.md` |
| `{...}` brace expansion | ❌ | Use char class instead |

---

## When to Invoke Skills

### devforgeai-brainstorming

**Use when:**
- User has vague business problem without clear requirements
- Pre-ideation discovery needed (stakeholders, constraints, hypotheses)
- Starting from scratch without defined scope
- User says "I'm not sure what I need" or "help me explore this"
- **This is the EARLIEST entry point - use BEFORE ideation skill**

**Invocation:**
```
Skill(command="devforgeai-brainstorming")
```

**What it does:**
- 7-phase Business Analyst discovery workflow
- Stakeholder identification and goal mapping
- 5 Whys root cause analysis
- Opportunity and constraint discovery
- Hypothesis formation with validation criteria
- MoSCoW prioritization and Impact-Effort matrix
- Generates AI-consumable brainstorm document

**Output:**
- `devforgeai/specs/brainstorms/BRAINSTORM-NNN-title.brainstorm.md`
- Checkpoints for session continuity
- Structured input for `/ideate` command

**Session Continuity:**
- Context window monitoring at 70% threshold
- Automatic checkpoint generation
- Resume with `/brainstorm --resume BRAINSTORM-ID`

**Next step:** `/ideate` (auto-detects brainstorm documents)

---

### devforgeai-ideation

**Use when:**
- User has business idea without technical specs
- Starting greenfield projects ("I want to build...")
- Adding major features to existing systems
- Exploring solution spaces and feasibility
- User requests requirements discovery or epic creation
- **This is the entry point - use BEFORE architecture skill**

**Invocation:**
```
Skill(command="devforgeai-ideation")
```

### User Input Guidance

**For effective ideation:** Business ideas should clearly describe the problem, target market, and expected outcomes. Provide specific context rather than vague statements. The skill will guide you through 6 phases of discovery with 10-60 interactive questions.

**File:** `src/claude/skills/devforgeai-ideation/references/user-input-guidance.md`

**Load command:**
```
Read(file_path="src/claude/skills/devforgeai-ideation/references/user-input-guidance.md")
```

**Example:** "Build an AI-powered task prioritization system for software teams that reduces sprint planning time by 40% and improves velocity tracking by 25%"

---

### devforgeai-architecture

**Use when:**
- Context files missing or need updates
- Making technology decisions
- Defining project structure
- Documenting architectural decisions (ADRs)
- Brownfield projects need architectural documentation
- **Use after ideation, before orchestration/development**

**Invocation:**
```
Skill(command="devforgeai-architecture")
```

### User Input Guidance

**For architecture setup:** Provide project context (greenfield/brownfield status), team size, deployment targets, and technology preferences. The skill will ask detailed questions about architecture decisions, constraints, and patterns. Be ready to approve or discuss technology selections.

**File:** `src/claude/skills/devforgeai-architecture/references/user-input-guidance.md`

**Load command:**
```
Read(file_path="src/claude/skills/devforgeai-architecture/references/user-input-guidance.md")
```

**Example:** "Greenfield SaaS platform for 50-person startup, needs Django backend with PostgreSQL, React frontend, deploy to AWS, must support 10K concurrent users"

**Workflow (5 Phases - Progressive Disclosure):**
1. Project Context Discovery (greenfield vs brownfield, technology inventory)
2. Create Immutable Context Files (6 files: tech-stack, source-tree, dependencies, coding-standards, architecture-constraints, anti-patterns)
3. Create Architecture Decision Records (document all major decisions with ADRs)
4. Create Technical Specifications (optional: use cases, API specs, database schemas, NFRs)
5. Validate Spec Against Context (ensure no conflicts)

**Reference Files (REFACTORED 2025-01-06 - Progressive Loading):**

**Workflow Files (6 files, 2,964 lines):**
- context-discovery-workflow.md (169 lines) - Phase 1
- context-file-creation-workflow.md (1,050 lines) - Phase 2 (MASSIVE consolidation of all 6 context file workflows)
- adr-creation-workflow.md (386 lines) - Phase 3
- technical-specification-workflow.md (392 lines) - Phase 4
- architecture-validation.md (200 lines) - Phase 5
- brownfield-integration.md (767 lines) - Existing project adoption

**Guide Files (4 files, 2,153 lines):**
- adr-policy.md (324 lines) - When to create ADRs
- adr-template.md (217 lines) - ADR structure
- ambiguity-detection-guide.md (540 lines) - When to use AskUserQuestion
- system-design-patterns.md (1,072 lines) - Architecture patterns

**Asset Files (12 files, 9,079 lines):**
- 6 context templates (3,922 lines)
- 6 ADR examples (5,157 lines)

**Entry Point:** SKILL.md - 212 lines (78% reduction from 978)
**Token Efficiency:** 4.6x improvement (7,824→1,696 tokens on activation)

**Key Features:**
- **Progressive disclosure** - Each phase loads its workflow file on-demand
- **Template-based** - Uses comprehensive asset templates (no inline workflows)
- **Ambiguity resolution** - MUST use AskUserQuestion for all technology/pattern decisions
- **Brownfield support** - Discovery → gap analysis → migration strategy
- **Immutable constraints** - 6 context files serve as "the law" for all AI agents

---

### devforgeai-orchestration

**Use when:**
- Starting new epics or sprints
- Creating stories from requirements
- Managing story workflow progression
- Enforcing quality gates
- Tracking deferred work (NEW - RCA-006)
- Analyzing technical debt (NEW - RCA-006)
- **Creating epics with feature decomposition** (NEW - 2025-11-06)

**Invocation (Story Management Mode):**
```
# Load story first
@devforgeai/specs/Stories/STORY-001.story.md

Skill(command="devforgeai-orchestration")
```

**Invocation (Epic Creation Mode):**
```
# Set context markers
**Epic name:** User Authentication System
**Command:** create-epic

Skill(command="devforgeai-orchestration")
```

**Invocation (Sprint Planning Mode):**
```
# Set context markers
**Sprint Name:** Sprint-1
**Command:** create-sprint
**Selected Stories:** STORY-001, STORY-002, STORY-003

Skill(command="devforgeai-orchestration")
```

### User Input Guidance

**For story/epic/sprint management:** Provide story IDs, epic names, or sprint metadata as context markers. The skill orchestrates complex workflows including epic decomposition, sprint capacity validation, and technical debt analysis. Answer all interactive questions to ensure proper story linking and dependency management.

**File:** `src/claude/skills/devforgeai-orchestration/references/user-input-guidance.md`

**Load command:**
```
Read(file_path="src/claude/skills/devforgeai-orchestration/references/user-input-guidance.md")
```

**Example (Epic):** "Epic name: Real-Time Notifications (business goal: reduce response time to <100ms, timeline: Q2 2025, stakeholders: platform team + mobile team)"

**Example (Sprint):** "Sprint Name: Performance Optimization Sprint-5 (20-40 point capacity, start: Jan 27, end: Feb 9)"

**Key Features:**
- Tracks **deferred work** (Phase 06) - ensures follow-up stories/ADRs exist
- Invokes **technical-debt-analyzer** during sprint planning (identifies stale debt, circular deferrals)
- Validates **deferral tracking** (story references, ADR references)
- Creates **follow-up stories** for missing references
- **Epic creation workflow** (Phase 4A - NEW 2025-11-06) - 8-phase comprehensive epic creation
- **Sprint planning workflow** (Phase 3 - 2025-11-05) - Sprint creation with capacity validation
- **Sprint retrospective** (Phase 7 - RCA-006 Phase 2 NEW 2025-11-06) - Auto-audit technical debt, create debt reduction sprints

**Epic Creation Workflow (8 Phases - NEW):**
1. Epic Discovery - Generate EPIC-ID, check duplicates
2. Context Gathering - Goal, timeline, priority, stakeholders, success criteria (4 AskUserQuestion flows)
3. Feature Decomposition - requirements-analyst subagent, 3-8 features, user review loop
4. Technical Assessment - architect-reviewer subagent, complexity 0-10, risk identification, context file validation
5. Epic File Creation - Populate epic-template.md, write to devforgeai/specs/Epics/{EPIC-ID}.epic.md
6. Requirements Spec - Optional requirements-analyst subagent, write to devforgeai/specs/requirements/
7. Validation & Self-Healing - 9 validation checks, auto-correct correctable issues, HALT on critical failures
8. Completion Summary - Return structured JSON to command

**Reference Files (Progressive Loading - REFACTORED 2025-01-06):**

**Core Workflow (9 files):**
- mode-detection.md (329 lines) - 4 modes detection logic
- checkpoint-detection.md (474 lines) - Resume functionality
- story-validation.md (345 lines) - Pre-execution validation
- skill-invocation.md (509 lines) - 4 skill coordination
- story-status-update.md (278 lines) - Status transitions
- qa-retry-workflow.md (919 lines) - Max 3 attempts with recovery
- deferred-tracking.md (714 lines) - Technical debt tracking
- next-action-determination.md (287 lines) - Workflow guidance
- orchestration-finalization.md (513 lines) - Completion summary

**Epic Management (4 files):**
- epic-management.md (496 lines - Phases 1-2)
- feature-decomposition-patterns.md (903 lines - Phase 3)
- technical-assessment-guide.md (914 lines - Phase 4)
- epic-validation-checklist.md (760 lines - Phase 7)

**Sprint Management (1 file):**
- sprint-planning-guide.md (631 lines)

**State Management (2 files):**
- workflow-states.md (585 lines)
- state-transitions.md (1,105 lines)

**Supporting (4 files):**
- quality-gates.md (1,017 lines)
- story-management.md (633 lines)
- user-interaction-patterns.md (513 lines) - 12 AskUserQuestion patterns
- troubleshooting.md (935 lines) - 13 common issues + solutions

**Total Reference Content:** 20 files, 12,860 lines (loaded progressively on-demand)

**Entry Point:** SKILL.md - 230 lines (93% reduction from 3,249)
**Token Efficiency:** 14.1x improvement (25,992→1,840 tokens on activation)

**Subagents Used:**
- requirements-analyst (Epic Phase 3 feature decomposition, Phase 6 optional requirements spec)
- architect-reviewer (Epic Phase 4 technical assessment)
- sprint-planner (Sprint Phase 3 complete workflow)
- technical-debt-analyzer (Phase 06 debt trend analysis)

---

### devforgeai-story-creation

**Use when:**
- User runs `/create-story [feature-description]` command
- Orchestration skill creates stories from epic features
- Development skill creates tracking stories for deferred DoD items
- Sprint planning requires story generation
- Transforming feature descriptions into structured stories
- **Use after ideation/architecture, before development**

**Invocation:**
```
# Feature description in conversation context
**Feature Description:** {description}

Skill(command="devforgeai-story-creation")
```

### User Input Guidance

**For story generation:** Provide specific feature descriptions including user workflows, business value, acceptance criteria ideas, and any technical constraints. The skill will generate complete story documents with AC, tech specs, UI specs (if applicable), and DoD. Be precise about requirements to avoid rework.

**File:** Effective Prompting Guide (`src/claude/memory/effective-prompting-guide.md`) + story examples (in skill references)

**Load command:**
```
Read(file_path="src/claude/memory/effective-prompting-guide.md")
```

**Example:** "User login with email/password, session timeout after 30 minutes, account lockout after 5 failed attempts, TOTP MFA support, password reset via email with 24-hour token expiration"

**Workflow (8 Phases):**
1. **Story Discovery** - Generate story ID, discover epic/sprint context, collect metadata
2. **Requirements Analysis** - Invoke requirements-analyst subagent, generate user story + AC
3. **Technical Specification** - Invoke api-designer subagent (if API), define data models, business rules
4. **UI Specification** - Document components, mockups, interfaces, accessibility (if UI detected)
5. **Story File Creation** - Build YAML frontmatter + all markdown sections, write to disk
6. **Epic/Sprint Linking** - Update parent documents with story references
7. **Self-Validation** - Validate quality, self-heal issues, ensure completeness
8. **Completion Report** - Present summary, guide user to next actions

**Output:**
- Complete story document in `devforgeai/specs/Stories/{STORY-ID}-{slug}.story.md`
- All sections: User story, AC (3+ Given/When/Then), tech spec, UI spec (if applicable), NFRs, edge cases, DoD
- Epic/sprint files updated (if associated)
- Self-validated for quality

**Subagents Used:**
- **story-requirements-analyst** (Phase 2) - User story and acceptance criteria (RCA-007 Phase 3 - skill-specific, content-only)
  - Replaces general-purpose requirements-analyst
  - Cannot create files (no Write/Edit tools by design)
  - Returns markdown content for assembly into story-template.md
- api-designer (Phase 3, conditional) - API contracts if endpoints detected

**Reference Files (6 files, 7,477 lines):**
- story-structure-guide.md - YAML frontmatter, sections, formatting
- acceptance-criteria-patterns.md - Given/When/Then patterns by story type
- technical-specification-guide.md - API contracts, data models, business rules
- ui-specification-guide.md - ASCII mockups, components, accessibility (WCAG AA)
- validation-checklists.md - Quality validation, self-healing
- story-examples.md - 4 complete examples (CRUD, auth, workflow, reporting)

**Key Features:**
- **Self-validation** (Phase 7) - Ensures quality before completion
- **Progressive disclosure** - 6 reference files loaded only when needed
- **Framework-aware** - Respects context files, integrates with other skills
- **Reusable** - Invoked by command, orchestration, development, sprint planning
- **Complete specifications** - API contracts, data models, UI mockups, accessibility

---

### devforgeai-ui-generator

**Use when:**
- Story requires UI components (forms, dashboards, dialogs)
- Generating visual specifications from requirements
- Creating mockups-as-code for web, desktop, or terminal interfaces
- Need to translate acceptance criteria into UI components
- **Invoked after architecture (requires context files), before or during development**

**Invocation:**
```
# Story mode - load story first
@devforgeai/specs/Stories/STORY-001.story.md
Skill(command="devforgeai-ui-generator")

# Standalone mode - provide description
**Component description:** Login form with validation
Skill(command="devforgeai-ui-generator")
```

### User Input Guidance

**For UI specification:** Describe components with specific requirements: platform (web/desktop/terminal), interactive elements, validation rules, accessibility level (WCAG AA/AAA), responsive behavior, and design constraints. The skill will ask about design patterns, framework preferences, and styling conventions aligned with your project.

**File:** `src/claude/skills/devforgeai-ui-generator/references/user-input-guidance.md`

**Load command:**
```
Read(file_path="src/claude/skills/devforgeai-ui-generator/references/user-input-guidance.md")
```

**Example:** "Multi-step form wizard with email/password validation, password strength meter, WCAG AA accessibility, responsive grid for mobile/tablet/desktop, client-side validation with error messages below fields"

---

### devforgeai-development

**Use when:**
- Implementing user stories or features
- Writing new code with TDD
- Refactoring while maintaining specs
- Resolving QA deferral failures (NEW - RCA-006)

**Prerequisites:**
- ✅ Git repository initialized (recommended, not required)
- ✅ Context files exist (devforgeai/specs/context/*.md)
- ✅ Story file exists (devforgeai/specs/Stories/{STORY-ID}.story.md)

**Git Availability:**
- **With Git:** Full workflow (branch management, commits, version control)
- **Without Git:** File-based tracking (changes documented in story artifacts)
- **Auto-detects:** Skill automatically checks Git availability and adapts workflow

**Invocation:**
```
# Load story first
@devforgeai/specs/Stories/STORY-001.story.md

Skill(command="devforgeai-development")
```

### User Input Guidance

**For TDD implementation:** Load story file with clear acceptance criteria and technical specifications. No feature description needed - AC from the story file drives TDD workflow. Ensure story is in "Ready for Dev" status with complete context files. The skill will handle Red→Green→Refactor cycle automatically.

**File:** Story file (acceptance criteria are the test specifications)

**Load command:**
```
Read(file_path="devforgeai/specs/Stories/STORY-001.story.md")
```

**Example:** Story file contains 5+ Given/When/Then AC and detailed tech spec with API contracts, data models, and business rules - skill uses these directly as TDD requirements.

**Key Features (Enhanced 2025-11-06 - RCA-006, 2025-11-13 - RCA-008):**
- **Lean command architecture:** /dev command delegates to skill (513 lines, down from 860)
- **Subagent-powered validation:**
  - **git-validator** subagent (Phase 01 Step a.) - Git status and workflow strategy (enhanced with file analysis per RCA-008)
  - **tech-stack-detector** subagent (Phase 01 Step g.) - Technology detection and validation
- **Git-aware workflow:** Automatically detects Git and uses file-based fallback if unavailable
- **RCA-008 Git Safety Enhancements (2025-11-13):**
  - **User consent checkpoint** (Step 0.1.5): Requires explicit approval for git operations affecting >10 files
  - **Stash warning workflow** (Step 0.1.6): Double confirmation with file visibility warnings for stash operations
  - **Smart stash strategy**: Recommends stashing modified-only files, preserving untracked user content
  - **git-validator file analysis**: Categorizes files (story_files, code, cache) for informed consent
- **Phase 06: Deferral Challenge Checkpoint** (RCA-006):
  - Challenges ALL deferrals (pre-existing from template + new from TDD)
  - Invokes deferral-validator subagent for blocker validation
  - Requires user approval for EVERY deferred item (zero autonomous deferrals)
  - Timestamps all approvals for audit trail
  - Enforces "Attempt First, Defer Only If Blocked" pattern
- **Phase 08: New Incomplete Items:**
  - Layer 1: Python format validator (~200 tokens, <100ms)
  - Layer 2: Interactive checkpoint (AskUserQuestion for new items only)
  - Layer 3: Git commit or file-based tracking
- **QA failure recovery:** Detects previous QA failures, guides resolution workflow (Phase 01 Step h.)
- **Framework-aware subagents:** All subagents understand DevForgeAI constraints (prevent silos)
- **Zero autonomous deferrals:** User approval mandatory for all deferred/incomplete DoD items

---

### devforgeai-qa

**Auto-invoked during development, or use manually for:**
- Deep validation after story completion
- Pre-release quality gates
- Technical debt assessment
- Deferral validation (NEW - RCA-006)

**Invocation:**
```
# Load story first
@devforgeai/specs/Stories/STORY-001.story.md

# Deep validation
**Validation mode:** deep
Skill(command="devforgeai-qa")

# Light validation
**Validation mode:** light
Skill(command="devforgeai-qa")
```

### User Input Guidance

**For QA validation:** Load story file and specify validation mode (light/deep). No feature description needed - validation rules are determined by story AC and project context files. Story should be in "Dev Complete" status with all AC implemented. The skill will validate against tech-stack, coding-standards, and architecture-constraints.

**File:** Story file (AC specifications) and project context files

**Load command:**
```
Read(file_path="devforgeai/specs/Stories/STORY-001.story.md")
```

**Example:** Story in "Dev Complete" status with implementation → skill validates AC pass/fail, coverage metrics, code quality, deferral justifications, spec compliance.

**Key Features (RCA-006 Enhanced):**
- Validates **deferred DoD items** via deferral-validator subagent (Phase 01 Step b.5)
- **FAILS QA** on CRITICAL (circular deferrals) or HIGH (unjustified deferrals) violations
- Tracks **QA iteration history** (attempts, violations, resolutions) in story file
- Enables **feedback loop**: QA FAIL → Dev fix → QA retry

---

### devforgeai-qa-remediation

**Use when:**
- QA gap files need to be converted to stories
- Processing imported QA reports from external projects
- Systematically addressing accumulated technical debt
- After QA validation produces gap files (`*-gaps.json`)

**Invoked by:** `/review-qa-reports` command

**Invocation:**
```
# Via command (recommended)
/review-qa-reports --source local --min-severity HIGH

# Direct skill invocation
Skill(command="devforgeai-qa-remediation")
```

### User Input Guidance

**For gap remediation:** Specify source (`local`, `imports`, or `all`) and minimum severity threshold. The skill discovers gap files, aggregates and scores gaps, presents interactive selection, then creates stories for selected gaps.

**Gap Sources:**
- Local: `devforgeai/qa/reports/*-gaps.json`
- Imports: `devforgeai/qa/imports/**/*-gaps.json`

**7-Phase Workflow:**
1. **Phase 01:** Pre-flight validation, load config
2. **Phase 02:** Discovery & parsing (glob gap files, parse JSON)
3. **Phase 03:** Aggregation & prioritization (dedupe, score, filter)
4. **Phase 04:** Interactive selection (display summary, user selects)
5. **Phase 05:** Batch story creation (invoke devforgeai-story-creation)
6. **Phase 06:** Source report update (add `implemented_in` to gap JSON)
7. **Phase 07:** Technical debt integration (add skipped gaps to register)

**Key Features:**
- Processes 4 gap types: coverage, anti-pattern, code quality, deferral
- Severity scoring: CRITICAL=100, HIGH=75, MEDIUM=50, LOW=25
- Skipped gaps automatically added to `devforgeai/technical-debt-register.md`
- Enhancement reports generated in `devforgeai/qa/enhancement-reports/`

**Reference Files:**
- `references/gap-discovery-workflow.md` - Phase 02 parsing
- `references/gap-aggregation-algorithm.md` - Phase 03 scoring
- `references/gap-to-story-mapping.md` - Phase 05 context markers
- `references/report-update-protocol.md` - Phase 06 updates
- `references/technical-debt-update.md` - Phase 07 debt register

---

### devforgeai-release

**Use when:**
- Story status = "QA Approved" (ready for production)
- Coordinated sprint releases (multiple stories together)
- Hotfix deployments (critical bug fix, still requires QA)
- Rollback operations (production issue detected)
- **This is the final stage - use AFTER QA approval**

**Invocation:**
```
# Load story first
@devforgeai/specs/Stories/STORY-001.story.md

# Default (staging)
Skill(command="devforgeai-release")

# Explicit staging
**Environment:** staging
Skill(command="devforgeai-release")

# Production deployment
**Environment:** production
Skill(command="devforgeai-release")
```

### User Input Guidance

**For deployment operations:** Load story file and specify environment (staging/production). Story must be "QA Approved" status. No feature description needed - deployment specs come from story and project deployment configuration. The skill will validate pre-conditions, execute deployment, run smoke tests, and handle rollback if needed.

**File:** Story file (deployment specs) and deployment configuration (`devforgeai/deployment/`)

**Load command:**
```
Read(file_path="devforgeai/specs/Stories/STORY-001.story.md")
```

**Example:** Story with "QA Approved" status → skill deploys to staging, runs smoke tests, then (if approved) deploys to production with automated rollback capability.

---

## Workflow Sequences

### For New Projects or Major Features

```
1. devforgeai-ideation
   ↓ (discover requirements, create epics)

2. devforgeai-architecture
   ↓ (create context files, make tech decisions)

3. devforgeai-orchestration
   ↓ (create sprints, generate stories)

4. devforgeai-ui-generator [OPTIONAL]
   ↓ (generate UI specs if story has UI components)

5. devforgeai-development
   ↓ (implement stories with TDD)

6. devforgeai-qa
   ↓ (validate quality, coverage, compliance)

7. devforgeai-release
   (deploy to production)
```

### For Existing Projects with Defined Context

```
1. devforgeai-orchestration OR devforgeai-story-creation
   ↓ (orchestration: create stories from epics)
   ↓ (story-creation: create individual story from feature description)

2. devforgeai-ui-generator [OPTIONAL]
   ↓ (generate UI specs if needed)

3. devforgeai-development
   ↓ (implement with TDD)

4. devforgeai-qa
   ↓ (validate)

5. devforgeai-release
   (deploy)
```

### For Individual Story Creation

```
1. devforgeai-story-creation
   ↓ (transform feature description → complete story)

2. devforgeai-ui-generator [OPTIONAL]
   ↓ (add UI specifications if needed)

3. devforgeai-development
   ↓ (implement story with TDD)

4. devforgeai-qa
   ↓ (validate implementation)

5. devforgeai-release
   (deploy to production)
```

### For UI-Focused Stories

```
1. devforgeai-architecture
   ↓ (ensure context files exist)

2. devforgeai-ui-generator
   ↓ (interactive UI spec generation)

3. devforgeai-development
   ↓ (implement UI with tests)

4. devforgeai-qa
   (validate UI implementation)
```

---

---

## CRITICAL: Skills Cannot Accept Parameters

**From official Claude documentation:**
> "Skills CANNOT accept command-line style parameters. All parameters are conveyed through natural language in the conversation."

### How to Pass "Parameters" to Skills

**❌ WRONG:**
```
Skill(command="devforgeai-qa --mode=deep --story=STORY-001")
Skill(command="devforgeai-development --story=STORY-001")
Skill(command="devforgeai-release --env=production")
```

**✅ CORRECT:**
```
# Step 1: Load story content into conversation
@devforgeai/specs/Stories/STORY-001.story.md

# Step 2: Set context with explicit statements
**Story ID:** STORY-001
**Validation Mode:** deep
**Environment:** staging

# Step 3: Invoke skill WITHOUT arguments
Skill(command="devforgeai-qa")

# Skill will extract story ID from YAML frontmatter in loaded story file
# Skill will extract mode from "Validation Mode: deep" statement
# Skill will extract environment from "Environment: staging" statement
```

### Why This Works

1. **@file loads content** - Story YAML frontmatter becomes part of conversation
2. **Explicit statements provide context** - Skills search conversation for patterns like "Mode: deep"
3. **Skills read conversation** - Extract needed information using pattern matching
4. **No parameter passing** - Skills operate on available conversation context only

---

## Skill Integration

Skills automatically invoke each other when needed:
- **devforgeai-development** auto-invokes **devforgeai-qa** (light mode) after each TDD phase
- **devforgeai-ideation** auto-transitions to **devforgeai-architecture**
- **devforgeai-orchestration** invokes other skills based on workflow state

---

## Skill-Specific Documentation

For detailed skill documentation, see:
- `.claude/skills/devforgeai-ideation/SKILL.md`
- `.claude/skills/devforgeai-architecture/SKILL.md`
- `.claude/skills/devforgeai-orchestration/SKILL.md`
- `.claude/skills/devforgeai-ui-generator/SKILL.md`
- `.claude/skills/devforgeai-development/SKILL.md`
- `.claude/skills/devforgeai-qa/SKILL.md`
- `.claude/skills/devforgeai-release/SKILL.md`

**Reference Files:** Each skill has a `references/` directory with detailed guides loaded progressively as needed.

---

### devforgeai-rca

**Use when:**
- User reports framework breakdown or process failure
- Workflow didn't follow intended process
- Skill/command violated lean orchestration pattern
- Quality gate was bypassed unexpectedly
- Context file constraints were ignored
- Workflow state transition was invalid
- User says "Perform RCA: [issue description]"

**Invocation:**
```
# User reports issue
**Issue Description:** {description}
**Severity:** {CRITICAL/HIGH/MEDIUM/LOW}

Skill(command="devforgeai-rca")
```

### User Input Guidance

**For root cause analysis:** Provide clear description of the framework breakdown or process failure with affected components (skill/command/subagent). Include severity level (CRITICAL/HIGH/MEDIUM/LOW). The skill will read relevant files, perform 5 Whys analysis, and generate actionable recommendations with exact implementation details.

**File:** Context files (tech-stack, coding-standards, architecture-constraints) + affected component files

**Load command:**
```
Read(file_path="devforgeai/RCA/")
```

**Example:** "devforgeai-development didn't validate context files before TDD cycle, allowing use of unapproved technologies (CRITICAL)"

**Workflow (8 Phases):**
1. **Phase 0:** Issue Clarification - Extract details or use AskUserQuestion, generate RCA number/title
2. **Phase 1:** Auto-Read Files - Read skills, commands, subagents, context files based on component type
3. **Phase 2:** 5 Whys Analysis - Progressive questioning with evidence backing
4. **Phase 3:** Evidence Collection - Organize excerpts, validate context files, analyze workflow state
5. **Phase 4:** Recommendation Generation - Prioritized fixes with exact implementation
6. **Phase 5:** RCA Document Creation - Write to devforgeai/RCA/RCA-XXX-slug.md
7. **Phase 6:** Validation & Self-Check - Verify completeness, self-heal issues
8. **Phase 7:** Completion Report - Return summary to command

**Key Features:**
- **5 Whys methodology** - Systematic root cause identification
- **Auto-reads relevant files** - Based on affected component (skill/command/subagent)
- **Evidence collection** - File excerpts with line numbers and significance
- **Exact implementation** - Copy-paste ready code/text in recommendations
- **Framework-aware** - Understands context files, quality gates, workflow states
- **Progressive disclosure** - 5 reference files (~4,000 lines) loaded as needed
- **Evidence-based only** - No aspirational recommendations
- **Auto-generates RCA document** - Complete formatted document

**Output:**
- `devforgeai/RCA/RCA-XXX-{slug}.md` (complete RCA document)
- Completion report (RCA number, root cause, recommendation counts, next steps)

**Reference Files (Progressive Loading):**
- 5-whys-methodology.md (800 lines) - 5 Whys technique and patterns
- evidence-collection-guide.md (700 lines) - What to examine and how
- recommendation-framework.md (900 lines) - Priority criteria, implementation details
- rca-writing-guide.md (600 lines) - Document structure and formatting
- framework-integration-points.md (1,000 lines) - DevForgeAI component relationships

**Asset Templates:**
- rca-document-template.md - Complete RCA structure
- 5-whys-template.md - 5 Whys formatting
- evidence-section-template.md - Evidence organization
- recommendation-template.md - Recommendation structure

**Integration:**
- Invoked by: `/rca` command
- Analyzes: All devforgeai-* skills, all commands, all subagents
- References: Context files, quality gates, workflow states, lean orchestration pattern
- Outputs to: `devforgeai/RCA/` directory

**Character Budget:** Command 9,500 chars (63%), Skill 1,326 lines

---

### claude-code-terminal-expert

**Use when:**
- User asks about Claude Code Terminal features ("Can Claude Code...?" / "Does Claude Code have...?")
- Creating subagents, skills, slash commands, plugins, or hooks
- Configuring settings, models, permissions, or MCP servers
- Setting up CI/CD integration (GitHub Actions, GitLab)
- Troubleshooting Claude Code issues (installation, auth, performance)
- Any questions about Claude Code Terminal capabilities
- **This is infrastructure support - provides terminal expertise**

**Invocation:**
```
# Skill automatically triggers on Claude Code Terminal questions
# No special invocation needed - model-invoked based on question context
```

### User Input Guidance

**For Claude Code questions:** Ask questions about Claude Code Terminal features, capabilities, or troubleshooting. The skill will automatically load official documentation and answer with authoritative, up-to-date information. No special preparation needed - ask naturally about subagents, skills, commands, plugins, CI/CD integration, or configuration.

**File:** Official Claude Code documentation (code.claude.com)

**Load command:**
```
Read(file_path="src/claude/memory/skills-reference.md")
```

**Example:** "How do I create a custom subagent with file access?" or "Does Claude Code Terminal support GitHub Actions integration?"

**Key Features:**
- **Comprehensive knowledge:** 28 topics covering all Claude Code Terminal features
- **Self-updating:** Can fetch latest docs from code.claude.com when needed
- **Progressive disclosure:** 6 reference files + 2 assets loaded as needed
- **100% official:** All content from official Anthropic documentation
- **Token efficient:** 95% savings vs loading all docs (2,100-6,000 tokens typical)

**Reference Files (Progressive Loading):**
- core-features.md (2,428 lines - Subagents, Skills, Commands, Plugins, MCP)
- configuration-guide.md (1,513 lines - Settings, Models, CLI, Permissions)
- integration-patterns.md (2,790 lines - CI/CD, Hooks, Headless, Containers)
- troubleshooting-guide.md (2,128 lines - Installation, Auth, Performance, Errors)
- advanced-features.md (3,553 lines - Sandboxing, Network, Monitoring, Security)
- best-practices.md (1,230 lines - Workflows, Efficiency, Prompts, Token Optimization)

**Asset Files:**
- quick-reference.md (726 lines - Command cheat sheet, keyboard shortcuts)
- comparison-matrix.md (600 lines - Feature comparison, decision matrices)

**Documentation URLs (Self-Updating):**
All 29 official code.claude.com URLs embedded for auto-updates

**Skill Type:** Knowledge/expertise skill (not workflow execution)

**Integration:** Complements DevForgeAI by providing Claude Code Terminal knowledge, reducing "Claude doesn't know this feature" friction

---

### devforgeai-documentation

**Use when:**
- Generating project documentation (README, guides, API docs)
- Updating documentation after story completion
- Analyzing documentation coverage gaps
- Creating architecture diagrams from codebase
- Brownfield documentation (analyze existing code)

**Invocation:**
```
# Story-based documentation
@devforgeai/specs/Stories/STORY-040.story.md
Skill(command="devforgeai-documentation")

# Codebase analysis mode
**Mode:** brownfield-analysis
Skill(command="devforgeai-documentation")
```

### User Input Guidance

**For documentation generation:** Load story file for story-based docs, or specify codebase analysis mode for brownfield documentation. The skill will generate complete documentation sets including README, guides, API docs, and architecture diagrams. No feature description needed - documentation is derived from story specifications or codebase analysis.

**File:** Story file (devforgeai/specs/Stories/) or source code directory (src/, lib/, etc.)

**Load command:**
```
Read(file_path="devforgeai/specs/Stories/STORY-040.story.md")
```

**Example:** Story with complete AC and tech spec → skill generates README, API documentation, developer guide, and Mermaid architecture diagrams.

**Key Features:**
- **Dual mode:** Greenfield (story-based docs) + Brownfield (codebase analysis)
- **Auto-invoked:** After story completion (if documentation hook enabled)
- **Uses code-analyzer subagent:** Deep codebase analysis for metadata extraction
- **Generates:** README, developer guides, API docs, architecture diagrams, roadmaps

---

### devforgeai-feedback

**Use when:**
- Capturing retrospective feedback after dev/qa/release operations
- Analyzing process improvement opportunities
- Building organizational knowledge from past work
- Hook-driven feedback collection (event-driven)

**Invocation:**
```
# Manual feedback
**Operation:** dev
**Story ID:** STORY-037
Skill(command="devforgeai-feedback")

# Auto-invoked via hooks (after /dev, /qa, /release)
```

### User Input Guidance

**For feedback collection:** Provide operation type (dev/qa/release) and story ID for manual feedback. The skill will ask context-aware retrospective questions about process improvements, learnings, and pain points. No feature description needed - feedback is based on the operation and story context.

**File:** Story file and operation context

**Load command:**
```
Read(file_path="devforgeai/specs/Stories/STORY-037.story.md")
```

**Example:** Operation: dev, Story: STORY-037 → skill asks about implementation challenges, design decisions, testing approach, and suggestions for future similar stories.

**Key Features:**
- **Event-driven:** Auto-invoked via hooks after operations complete
- **Adaptive questioning:** Context-aware retrospective questions
- **Persistence:** Stores feedback sessions in devforgeai/feedback/
- **Indexing:** Searchable feedback history
- **Integration:** Works with devforgeai CLI (check-hooks, invoke-hooks)

---

### devforgeai-mcp-cli-converter

**Use when:**
- Converting MCP servers to CLI utilities
- Creating Claude Code-compatible wrappers for async MCP tools
- Rapid prototyping of tool integrations
- Bridging MCP async patterns to sync CLI patterns

**Invocation:**
```
**MCP Server:** weather-mcp
**Pattern:** api-wrapper
Skill(command="devforgeai-mcp-cli-converter")
```

### User Input Guidance

**For MCP-to-CLI conversion:** Specify MCP server name and conversion pattern (api-wrapper, file-system, browser-automation). The skill will generate standalone CLI utilities that Claude Code Terminal can invoke directly. Provide MCP server capabilities and desired CLI behavior specifications.

**File:** MCP server specification and conversion pattern reference

**Load command:**
```
Read(file_path="src/claude/memory/skills-reference.md")
```

**Example:** "MCP Server: weather-mcp, Pattern: api-wrapper → generates weather-cli that wraps MCP server methods as shell commands"

**Key Features:**
- **Converts MCP → CLI:** Standalone executables Claude Code can invoke
- **Auto-generates skill:** Creates complementary skill for CLI usage
- **Pattern-based:** API wrapper, file system, browser automation patterns
- **No MCP overhead:** CLI runs without MCP server infrastructure

---

### devforgeai-subagent-creation

**Use when:**
- User runs /create-agent command
- Need custom DevForgeAI-aware subagent
- Creating domain-specific subagents with framework integration

**Invocation:**
```
**Subagent Name:** my-custom-validator
**Mode:** guided
Skill(command="devforgeai-subagent-creation")
```

### User Input Guidance

**For custom subagent creation:** Specify subagent name (lowercase-with-hyphens format) and mode (guided/template/domain/custom). The skill will guide you through subagent design ensuring framework compliance and Claude Code Terminal integration. Provide clear purpose, capabilities, and tool requirements.

**File:** Claude Code patterns and DevForgeAI framework constraints

**Load command:**
```
Read(file_path="src/claude/memory/skills-reference.md")
```

**Example:** "Name: code-metrics-analyzer, Mode: domain (architecture), Purpose: analyze code complexity and generate quality metrics"

**Key Features:**
- **Orchestrates agent-generator v2.0:** Delegates to agent-generator subagent
- **Framework-aware:** Generated subagents reference context files, quality gates
- **12-point validation:** Ensures Claude Code + DevForgeAI compliance
- **Reference files:** Auto-generates framework guardrails when needed
- **Invoked by:** /create-agent command

---

### internet-sleuth-integration

**Status:** ⚠️ INCOMPLETE - Skill directory exists but no SKILL.md file

**Current state:**
- Has `assets/` directory with research-report-template.md
- Has `references/` directory with 4 files:
  - competitive-analysis-patterns.md
  - discovery-mode-methodology.md
  - repository-archaeology-guide.md
  - research-principles.md
- Missing: SKILL.md entry point

**Functionality:**
- Research capabilities provided by **internet-sleuth subagent** (in .claude/agents/)
- Skill integration pending/incomplete
- References available for future skill completion

**Note:** For research functionality, use the internet-sleuth subagent directly via Task tool, not this incomplete skill.

---

### devforgeai-research

**Use when:**
- Conducting competitive analysis (AWS Kiro, Cursor, etc.)
- Technology evaluation (new libraries, frameworks, tools)
- Market research (developer trends, pain points, statistics)
- Integration planning (external APIs, services)
- Architecture research (design patterns, best practices)
- Preserving research findings across sessions

**Invocation:**
```
# Start new research
/research "AWS Kiro Competitive Analysis"

# Resume existing research
/research --resume RESEARCH-001

# Search existing research
/research --search "authentication"

# List all research
/research --list

# Filter by category
/research --list --category competitive
```

### User Input Guidance

**For research sessions:** Provide clear research topic and answer interactive questions about category (competitive/technology/market/integration/architecture) and research questions. The skill will conduct web searches, synthesize findings, generate recommendations, and persist results in structured documents.

**File:** Research documents in `devforgeai/specs/research/`

**Load command:**
```
Read(file_path="devforgeai/specs/research/research-index.md")
```

**Example effective input:**
```
/research "Competitive analysis of AWS Kiro vs DevForgeAI spec-driven development approach"
```

**Workflow (6 Phases):**
1. **Phase 0:** Initialization (mode detection, ID generation, load index)
2. **Phase 1:** Topic Definition (topic, category, research questions, duplicate check)
3. **Phase 2:** Research Execution (category-specific web searches and analysis)
4. **Phase 3:** Findings Synthesis (theme extraction, insight generation, recommendations)
5. **Phase 4:** Documentation (write research doc, update index, create assets folder)
6. **Phase 5:** Cross-Reference (link to epics, stories, ADRs)

**Output:**
- Research document: `devforgeai/specs/research/RESEARCH-NNN-{slug}.research.md`
- Research index updated: `devforgeai/specs/research/research-index.md`
- Assets folder (optional): `devforgeai/specs/research/RESEARCH-NNN/`

**Key Features:**
- **Persistent knowledge:** Research survives session restarts
- **Queryable:** Search and list existing research
- **Cross-referenced:** Links to epics, stories, ADRs
- **Staleness tracking:** 6-month review reminders
- **Evidence-based:** All findings cite sources with confidence levels

**Reference Files (Progressive Loading):**
- research-workflow.md (~760 lines) - Detailed phase execution
- citation-standards.md (~160 lines) - Source formatting
- search-strategies.md (~310 lines) - Search tips by category
- assets/templates/research-template.md (~63 lines) - Document template

**Created:** 2026-01-18

---

## Skill Count Summary

**Functional Skills: 15**
- **devforgeai-* (14):** ideation, architecture, orchestration, story-creation, ui-generator, development, qa, release, rca, documentation, feedback, mcp-cli-converter, subagent-creation, research
- **Infrastructure (1):** claude-code-terminal-expert

**Incomplete Skills: 1**
- internet-sleuth-integration (missing SKILL.md)

**EPIC-010 Added: 1**
- devforgeai-github-actions (GitHub Actions CI/CD orchestration)

**Total After EPIC-010: 15 functional skills** (14 current + 1 new)

---
