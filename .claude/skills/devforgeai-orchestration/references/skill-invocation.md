# Phase 2: Skill Invocation Coordination

This phase coordinates automatic invocation of other DevForgeAI skills based on story status and workflow stage.

## Purpose

**Automate skill execution** throughout the story lifecycle:
- Invoke devforgeai-architecture when context files needed
- Invoke devforgeai-development for implementation
- Invoke devforgeai-qa for validation
- Invoke devforgeai-release for deployment

**Key principle:** Orchestration skill acts as workflow coordinator, delegating specialized work to domain-specific skills.

---

## Skills Coordinated

### 1. devforgeai-architecture

**When invoked:** Story status = "Backlog" AND context files missing

**Purpose:** Generate 6 architectural context files

**Transition:** Architecture → Ready for Dev (after context files created)

---

### 2. devforgeai-development

**When invoked:** Story status = "Ready for Dev" OR checkpoint resumes from Phase 2

**Purpose:** Execute TDD implementation cycle

**Transition:** In Development → Dev Complete (after TDD complete)

---

### 3. devforgeai-qa

**When invoked:** Story status = "Dev Complete" OR checkpoint resumes from Phase 3

**Purpose:** Deep quality validation

**Transition:** QA In Progress → [QA Approved | QA Failed]

---

### 4. devforgeai-release

**When invoked:** Story status = "QA Approved" OR checkpoint resumes from Phase 4/5

**Purpose:** Deploy to staging and production

**Transition:** Releasing → Released (after production deployment)

---

## Invocation Patterns

### Architecture Phase (Backlog → Ready for Dev)

#### Context Check

```
Glob(pattern="devforgeai/specs/context/*.md")

Expected: 6 files
- tech-stack.md
- source-tree.md
- dependencies.md
- coding-standards.md
- architecture-constraints.md
- anti-patterns.md

IF count == 6:
  context_files_exist = true
  Skip architecture invocation
  Proceed to development

IF count < 6:
  context_files_exist = false
  Invoke architecture skill
```

#### Invocation

```
IF context_files_exist == false:
  Display: "Context files missing. Invoking architecture skill..."

  Skill(command="devforgeai-architecture")

  Wait for completion

  Validate context files created:
    Glob(pattern="devforgeai/specs/context/*.md")
    IF count == 6:
      Success: "✓ Context files created"
    ELSE:
      HALT: "Architecture skill completed but context files still missing"

  Update story status:
    Edit(
      file_path="devforgeai/specs/Stories/{story_id}.story.md",
      old_string="status: Architecture",
      new_string="status: Ready for Dev"
    )

  Append workflow history:
    "### {timestamp} - Architecture Phase Complete
     Context files created by devforgeai-architecture skill
     Status: Architecture → Ready for Dev"
```

**Gate enforced:** Gate 1 - Context Validation

**See:** `quality-gates.md` for complete requirements

---

### Development Phase (Ready for Dev → Dev Complete)

#### Invocation

```
Display: "🔨 Starting Development Phase..."
Display: "Invoking devforgeai-development skill for TDD workflow"

Skill(command="devforgeai-development")

# NOTE: Cannot pass story ID as parameter to skill
# Story ID must be in conversation context (already loaded in Phase 0/1)
```

#### Development Skill Workflow

The development skill executes **6 phases** in isolated context:
1. **Phase 0:** Pre-flight validation (Git, tech stack, context files)
2. **Phase 1:** Test-First (Red) - Generate failing tests from acceptance criteria
3. **Phase 2:** Implementation (Green) - Write code to pass tests + light QA
4. **Phase 3:** Refactor - Improve code quality + light QA
5. **Phase 4:** Integration Testing - Cross-component tests + light QA
6. **Phase 5:** Git Workflow - Three-layer DoD validation + commit

**Estimated tokens:** ~85,000 tokens (in isolated context, not main conversation)

#### Post-Development Validation

```
Wait for development skill completion

Verify story status updated:
  Read(file_path="devforgeai/specs/Stories/{story_id}.story.md")
  Extract: status

  IF status == "Dev Complete":
    Success: "✓ Development phase complete"
    Append workflow history:
      "### {timestamp} - Development Phase Complete
       Implementation finished by devforgeai-development skill
       Status: In Development → Dev Complete
       Checkpoint: DEV_COMPLETE"
  ELSE:
    HALT: "Development skill completed but status not updated.
           Expected: Dev Complete
           Actual: ${actual_status}"
```

**Gate enforced:** Gate 2 - Test Passing (checked by development skill)

**See:** `quality-gates.md` for test passing requirements

---

### QA Phase (Dev Complete → QA Approved/Failed)

#### Invocation

```
Display: "🔍 Starting QA Validation Phase..."
Display: "Invoking devforgeai-qa skill for deep validation"

# Set validation mode context
**Validation Mode:** deep

Skill(command="devforgeai-qa")

# Story ID already in context from Phase 0/1
# Mode extracted from context marker above
```

#### QA Skill Workflow

The QA skill executes **5 phases** in isolated context:
1. **Phase 1:** Test Coverage Analysis (95%/85%/80% thresholds)
2. **Phase 2:** Anti-Pattern Detection (God Objects, SQL injection, etc.)
3. **Phase 3:** Spec Compliance Validation (acceptance criteria met)
4. **Phase 4:** Code Quality Metrics (complexity, maintainability)
5. **Phase 5:** Generate QA Report + invoke qa-result-interpreter subagent

**Estimated tokens:** ~65,000 tokens (in isolated context)

#### Post-QA Validation

```
Wait for QA skill completion

Read QA report:
  Read(file_path="devforgeai/qa/reports/{story_id}-qa-report.md")

Parse result:
  Grep(pattern="Overall Result: (PASS|FAIL)")
  Extract: qa_result

IF qa_result == "PASS":
  Update status: QA In Progress → QA Approved

  Append workflow history:
    "### {timestamp} - QA Phase Complete
     Deep validation PASSED
     Status: QA In Progress → QA Approved
     Checkpoint: QA_APPROVED

     Coverage: {coverage_percentage}%
     Violations: 0 CRITICAL, 0 HIGH
     Report: devforgeai/qa/reports/{story_id}-qa-report.md"

  Proceed to Phase 4 (Release)

ELSE IF qa_result == "FAIL":
  Update status: QA In Progress → QA Failed

  # QA retry loop handled in Phase 3.5
  # See qa-retry-workflow.md

  Extract violations:
    Read violations section from report
    Count CRITICAL, HIGH, MEDIUM, LOW

  Append workflow history:
    "### {timestamp} - QA Phase Failed (Attempt 1)
     Deep validation FAILED
     Status: QA In Progress → QA Failed

     Violations:
     - CRITICAL: {critical_count}
     - HIGH: {high_count}
     - MEDIUM: {medium_count}
     - LOW: {low_count}

     Report: devforgeai/qa/reports/{story_id}-qa-report.md"

  Return to Phase 2 (Development) OR enter retry loop (Phase 3.5)
```

**Gate enforced:** Gate 3 - QA Approval

**See:** `quality-gates.md` for QA approval criteria

**See:** `qa-retry-workflow.md` for retry loop logic

---

### Release Phase (QA Approved → Released)

#### Staging Deployment

```
Display: "🚀 Starting Staging Deployment..."

# Set environment context
**Environment:** staging

Skill(command="devforgeai-release")

# Story ID already in context
# Environment extracted from context marker
```

**Release skill workflow (staging):**
1. Pre-release validation (QA approved check)
2. Deploy to staging environment
3. Execute smoke tests
4. Validate deployment success
5. Create STAGING_COMPLETE checkpoint

**Estimated tokens:** ~40,000 tokens (in isolated context)

#### Post-Staging Validation

```
Wait for staging deployment completion

Verify checkpoint created:
  Read(file_path="devforgeai/specs/Stories/{story_id}.story.md")
  Grep(pattern="Checkpoint: STAGING_COMPLETE", file=workflow_history)

  IF found:
    Success: "✓ Staging deployment complete"
    Proceed to production deployment
  ELSE:
    HALT: "Staging deployment failed. Check deployment logs."
```

---

#### Production Deployment

```
Display: "🚀 Starting Production Deployment..."

# Set environment context
**Environment:** production

Skill(command="devforgeai-release")

# Story ID already in context
# Environment extracted from context marker
```

**Release skill workflow (production):**
1. Pre-release validation (staging complete check)
2. Deploy to production environment
3. Execute smoke tests
4. Validate deployment success
5. Create PRODUCTION_COMPLETE checkpoint
6. Update story status: Releasing → Released

**Estimated tokens:** ~40,000 tokens (in isolated context)

#### Post-Production Validation

```
Wait for production deployment completion

Verify story status:
  Read(file_path="devforgeai/specs/Stories/{story_id}.story.md")
  Extract: status

  IF status == "Released":
    Success: "✓ Production deployment complete"

    Append workflow history:
      "### {timestamp} - Release Phase Complete
       Production deployment successful
       Status: Releasing → Released
       Checkpoint: PRODUCTION_COMPLETE

       Environments:
       - Staging: Deployed {staging_timestamp}
       - Production: Deployed {production_timestamp}"

  ELSE:
    HALT: "Production deployment failed. Status not updated to Released."
```

**Gate enforced:** Gate 4 - Release Readiness

**See:** `quality-gates.md` for release readiness criteria

---

## Skill Invocation Protocol

### Context-Based Parameter Passing

**Skills cannot accept command-line parameters.** Parameters are conveyed through conversation context.

#### Correct Pattern

```
# Step 1: Load story (provides story ID in context)
Read(file_path="devforgeai/specs/Stories/STORY-048-production-cutover-documentation.story.md")

# Step 2: Set explicit context markers
**Story ID:** STORY-048
**Validation Mode:** deep
**Environment:** staging

# Step 3: Invoke skill WITHOUT parameters
Skill(command="devforgeai-qa")

# Skill extracts parameters from conversation context
```

#### Incorrect Pattern (Will Not Work)

```
# ❌ WRONG - Skills don't accept parameters
Skill(command="devforgeai-qa --mode=deep --story=STORY-048")

# ❌ WRONG - Parameters ignored
Skill(command="devforgeai-development --story=STORY-048")
```

---

### Waiting for Skill Completion

**Skills execute in isolated contexts.** The orchestration skill waits for skill completion signals:

```
Invoke skill:
  Skill(command="devforgeai-{skillname}")

Wait for completion:
  # Skill returns when workflow complete
  # No explicit wait mechanism needed (synchronous execution)

Validate completion:
  # Check expected side effects (files created, status updated)
  # Verify checkpoints or markers in story file
```

**Key insight:** Skills execute synchronously. Control returns to orchestration skill after skill completes.

---

## Skill Integration Map

**Complete skill integration coverage:**

1. **devforgeai-ideation** - Requirements discovery → Architecture transition
2. **devforgeai-architecture** - Context file generation (invoked in Architecture phase)
3. **devforgeai-orchestration** - Workflow coordination (this skill)
4. **devforgeai-story-creation** - Story generation from features
5. **devforgeai-ui-generator** - UI specification generation
6. **devforgeai-development** - TDD implementation (invoked in Development phase)
7. **devforgeai-qa** - Quality validation (invoked in QA phase)
8. **devforgeai-release** - Deployment (invoked in Release phase)

**Orchestration skill invokes:** architecture, development, qa, release (4 of 8 skills)

**Other skills:** ideation (entry point), story-creation (reusable), ui-generator (optional), orchestration (coordinator)

---

## Error Handling

### Skill Invocation Fails

```
Error: Skill execution fails or returns error

Action:
  Display skill error message
  HALT orchestration
  Preserve current checkpoint (don't create new one)
  Log failure in workflow history:
    "### {timestamp} - {Skill Name} Failed
     Error: {error_message}
     Phase: {current_phase}
     Orchestration halted. Resolve issue and retry."

  Return: "SKILL_INVOCATION_FAILED"
```

---

### Skill Completes but Status Not Updated

```
Error: Skill completes but expected status change didn't occur

Action:
  HALT: "Skill ${skill_name} completed but story status not updated.
         Expected: ${expected_status}
         Actual: ${actual_status}

         Possible causes:
         - Skill encountered non-blocking issues
         - Status update logic failed
         - Story file write permission issue

         Review skill output for errors."

  Return: "STATUS_UPDATE_FAILED"
```

---

### Quality Gate Blocks Skill Invocation

```
Error: Attempting to invoke skill but quality gate blocks

Action:
  HALT: "Quality Gate ${gate_number} blocks progression.
         Cannot invoke ${skill_name} until requirements met.

         Requirements:
         ${gate_requirements}

         See quality-gates.md for details."

  Return: "QUALITY_GATE_BLOCKED"
```

---

## Related Files

- **checkpoint-detection.md** - Phase 0 (determines starting phase before skills invoked)
- **story-validation.md** - Phase 1 (validates story before skill invocation)
- **qa-retry-workflow.md** - Phase 3.5 (handles QA failures)
- **orchestration-finalization.md** - Phase 6 (after all skills complete)
- **quality-gates.md** - Gate requirements that block skill invocation
- **workflow-states.md** - Story states that trigger different skill invocations
