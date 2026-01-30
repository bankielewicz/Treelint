---
description: Transform business idea into structured requirements
argument-hint: [business-idea-description]
model: opus
allowed-tools: Read, Write, Edit, Glob, Skill, AskUserQuestion
---

# /ideate - Transform Business Idea into Structured Requirements

**Purpose:** Entry point for DevForgeAI framework - transforms business ideas into structured epics and requirements through comprehensive analysis.

**Output:** Epic documents, requirements specification, complexity assessment

**Process:** Invokes `devforgeai-ideation` skill which executes 6-phase requirements gathering with 10-60 interactive questions.

---

## Phase 0: Brainstorm Auto-Detection

**Purpose:** Check for existing brainstorm documents and offer to use them as input.

### 0.1 Check for Existing Brainstorms

**Search for brainstorm documents:**
```
brainstorms = Glob(pattern="devforgeai/specs/brainstorms/BRAINSTORM-*.brainstorm.md")
```

**If brainstorms found:**
```
IF len(brainstorms) > 0:
  Display:
  "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Existing Brainstorm(s) Detected
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  FOR each brainstorm:
    # Read frontmatter to get title and confidence
    Read(file_path=brainstorm, limit=30)
    Display: "- {id}: {title} (Confidence: {confidence_level})"
```

**Ask user if they want to use a brainstorm:**
```
AskUserQuestion(
  questions=[{
    question: "Would you like to use an existing brainstorm as input for ideation?",
    header: "Brainstorm",
    options: [
      {
        label: "Yes - use most recent",
        description: "Pre-populate ideation with brainstorm data"
      },
      {
        label: "Yes - let me choose",
        description: "Select which brainstorm to use"
      },
      {
        label: "No - start fresh",
        description: "Begin new ideation session"
      }
    ],
    multiSelect: false
  }]
)
```

### 0.2 Load Brainstorm Context (if selected)

**If user selected a brainstorm:**
```
# Read full brainstorm document
brainstorm_content = Read(file_path=selected_brainstorm)

# Extract YAML frontmatter
frontmatter = parse_yaml_frontmatter(brainstorm_content)

# Set context markers for skill
$BRAINSTORM_CONTEXT = {
  brainstorm_id: frontmatter.id,
  problem_statement: frontmatter.problem_statement,
  target_outcome: frontmatter.target_outcome,
  user_personas: frontmatter.user_personas,
  hard_constraints: frontmatter.hard_constraints,
  must_have_capabilities: frontmatter.must_have_capabilities,
  critical_assumptions: frontmatter.critical_assumptions,
  confidence_level: frontmatter.confidence_level
}

Display:
"Pre-populated from {brainstorm_id}:
  ✓ Problem: {problem_statement}
  ✓ Users: {len(user_personas)} persona(s)
  ✓ Constraints: {len(hard_constraints)} identified
  ✓ Must-haves: {len(must_have_capabilities)} capabilities

Proceeding to ideation with brainstorm context..."
```

**If no brainstorms or user chose "start fresh":**
```
$BRAINSTORM_CONTEXT = null
# Continue to Phase 1 normally
```

### 0.3 Continue to Phase 1

Pass `$BRAINSTORM_CONTEXT` to subsequent phases. The ideation skill will use this to:
- Skip or shorten Phase 1 exploration questions (already answered in brainstorm)
- Pre-populate requirements with must-have capabilities
- Validate constraints against brainstorm findings

---

## Phase 1: Argument Validation

### 1.1 Capture Business Idea

**Business idea from user:**
```
$ARGUMENTS
```

**If no arguments provided:**

Prompt user for business idea description (free-form text input):

```
"Please describe the business idea, feature, or problem you want to explore.

Include:
- What business problem this solves
- Who the primary users/beneficiaries are
- What success looks like

(Example: 'Build a task management app for remote teams with real-time collaboration')"
```

**Note:** Detailed categorization and exploration are handled by the skill, NOT here. Command only captures the initial business idea text.

**Wait for user response before proceeding to Phase 2.**

### 1.2 Validate Description

**Minimum requirements:**
- Description has at least 10 words
- Describes a business capability or problem (not purely technical)

**If description too vague:**
```
Prompt user: "Please provide more details about:
- What business problem this solves
- Who the primary users/beneficiaries are
- What success looks like"
```

---

## Phase 2: Invoke Ideation Skill

### 2.0 Smart Project Mode Detection (STORY-134)

**Purpose:** Auto-detect project mode based on context file existence before invoking skill.

**Step 1: Count Context Files**
```
context_files = Glob(pattern="devforgeai/specs/context/*.md")
context_file_count = len(context_files)
```

**Step 2: Determine Project Mode**
```
# Business Rule:
# - context_file_count == 6 → existing project (all context files present)
# - context_file_count < 6 → new project (missing context files)
IF context_file_count == 6:
    project_mode = "existing"
ELSE:
    project_mode = "new"
```

**Step 3: Display Mode Context**
```
Display:
"━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Project Mode Detection
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

IF project_mode == "existing":
    Display: "✓ All 6 context files present - existing project detected"
    Display: "  Next steps after ideation: /orchestrate or /create-sprint"
ELSE:
    Display: "⚠ Context files incomplete ({context_file_count}/6)"
    Display: "  Next steps after ideation: /create-context to establish architecture"
```

**Step 4: Set Mode Context for Skill**

The skill's Phase 6.6 (completion-handoff.md) will read this mode marker to determine next-action recommendation:
- **New project** → recommend `/create-context [project-name]`
- **Existing project** → recommend `/orchestrate` or `/create-story`

```
$PROJECT_MODE_CONTEXT = {
    mode: project_mode,
    context_files_found: context_file_count,
    detection_method: "filesystem_glob"
}
```

### 2.1 Set Context for Skill

**Purpose:** Pass all collected context to skill via conversation markers. This prevents duplicate questions - skill reads context markers instead of re-asking.

**Prepare context markers for skill execution:**

```
Display:
"━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Context for Ideation Skill
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
```

**Required Context Markers (set before Skill invocation):**

```
**Business Idea:** $ARGUMENTS (or user-provided description from Phase 1)

**Brainstorm Context:** {brainstorm_id} (if selected from Phase 0, else "none")

**Brainstorm File:** {path to selected brainstorm file} (if selected, else "none")

**Project Mode:** {existing|new} (from Phase 2.0 detection)
```

**Display context summary:**

```
Display:
"Context passed to skill:
  • Business Idea: {first 50 chars of business idea}...
  • Brainstorm: {brainstorm_id or 'Starting fresh'}
  • Project Mode: {existing|new}

Proceeding to ideation skill..."
```

**Context Marker Protocol:** Skill Phase 1 reads these markers to skip redundant questions. When context is provided:
- Skill DOES NOT re-ask for business idea (uses **Business Idea:** marker)
- Skill DOES NOT re-ask for project classification (uses **Project Mode:** marker)
- Skill validates/confirms context instead of full exploration (when brainstorm provided)

### 2.2 Skill Invocation

**The devforgeai-ideation skill handles complete workflow:**

- **Phase 1:** Problem Exploration (5-10 questions)
- **Phase 2:** Requirements Elicitation (15-25 questions)
- **Phase 3:** Complexity Assessment (0-60 scoring)
- **Phase 4:** Epic & Feature Decomposition
- **Phase 5:** Feasibility & Constraints Analysis
- **Phase 6:** Requirements Documentation (epics, requirements spec, validation, summary, next action)

**After skill completes:**
- **Phase N:** Hook Integration - Triggers post-ideation feedback (if configured)

**Expected interaction:**
- Skill asks 10-60 questions across 6 phases
- User answers guide requirements elicitation
- Skill validates completeness internally (Phase 6.4)
- Skill generates all output artifacts
- Skill presents completion summary (Phase 6.5)
- Skill asks user for next action (Phase 6.6)

**Invoke skill:**

```
Skill(command="devforgeai-ideation")
```

**After skill invocation:**
- Skill's SKILL.md content expands inline in conversation
- **YOU execute the skill's workflow phases** (not waiting for external result)
- Follow the skill's instructions phase by phase
- Produce output as skill instructs

**The skill instructs you to:**
- Execute all 6 phases: Problem Exploration, Requirements Elicitation, Complexity Assessment, Epic Decomposition, Feasibility Analysis, Documentation
- Handle all validation, error recovery, and user interaction (AskUserQuestion flows)
- Generate all output artifacts (epics, requirements spec, complexity assessment)
- Perform self-validation in Phase 6.4

---

## Phase 3: Result Interpretation

**Purpose:** Transform skill output into user-facing summary using ideation-result-interpreter subagent.

**Invoke result interpreter:**

```
Task(
  subagent_type="ideation-result-interpreter",
  description="Format ideation results for display",
  prompt="""
Interpret ideation workflow results.

Context:
- Project mode: ${PROJECT_MODE_CONTEXT.mode}
- Context files found: ${PROJECT_MODE_CONTEXT.context_files_found}/6

Task:
1. Read generated epic files from devforgeai/specs/Epics/
2. Extract: epic count, complexity score, architecture tier
3. Determine result status (SUCCESS/WARNING/FAILURE)
4. Generate display template per ideation-result-interpreter workflow
5. Provide next step recommendations based on project mode

Return structured result with display template.
"""
)
```

**Display result:**

```
Display: result.display.template
```

**Next:** Proceed to Phase N (Hook Integration)

---

## Phase N: Hook Integration

**Invoke reusable helper function for feedback hook integration:**

```bash
# Collect ideation artifacts for context
EPIC_FILES=$(ls -1 devforgeai/specs/Epics/EPIC-*.epic.md 2>/dev/null | tr '\n' ',' | sed 's/,$//' || echo "")

# Invoke helper function (handles check-hooks + invoke-hooks)
# Helper returns: exit 0 (success or graceful skip), never fails
.claude/scripts/invoke_feedback_hooks.sh ideate completed \
  --operation-type=ideation \
  --artifacts="$EPIC_FILES" || true
```

**Helper function handles:**
- N.1: Check hook eligibility (`devforgeai-validate check-hooks --operation=ideate --status=completed`)
- N.2: Invoke hooks if eligible (`devforgeai-validate invoke-hooks --operation=ideate ...`)
- N.3: Display status ("✓ Post-ideation feedback initiated" or "⚠ skipped")
- Error handling: All failures are non-blocking, command always succeeds

**Parameters passed to hooks:**
- `--operation-type=ideation` - Identifies this as ideation operation
- `--artifacts` - Comma-separated list of created epic files

**Note:** Complexity score and questions-asked count are captured by the ideation skill internally (Phase 6 summary) and passed via environment if needed.

**Next:** Proceed to Phase completion

---

## Error Handling

### Skill Loading Failure (STORY-139)

**Purpose:** Detect and handle skill loading failures with actionable recovery instructions.

**Pre-Invocation Check (Phase 2.2):**

Before invoking the skill, perform a quick validation:

```
# Check if SKILL.md exists
skill_check = Glob(pattern=".claude/skills/devforgeai-ideation/SKILL.md")

IF skill_check is empty:
    # FILE_MISSING error
    GOTO Skill Load Error Handler with errorType="FILE_MISSING"
```

**Error Detection Logic:**

```
TRY:
    Skill(command="devforgeai-ideation")
CATCH error:
    # Categorize error type
    IF error contains "ENOENT" OR error contains "no such file":
        errorType = "FILE_MISSING"
        errorDetails = "SKILL.md not found at .claude/skills/devforgeai-ideation/"

    ELIF error contains "YAML" OR error contains "parse" OR error contains "syntax":
        errorType = "YAML_PARSE_ERROR"
        # Extract line number if available
        lineNumber = extract_line_number(error) OR "unknown"
        errorDetails = "Invalid YAML in frontmatter at line {lineNumber}"

    ELIF error contains "missing" AND (error contains "section" OR error contains "field"):
        errorType = "INVALID_STRUCTURE"
        sectionName = extract_missing_section(error) OR "unknown"
        errorDetails = "Missing required section: {sectionName}"

    ELIF error contains "EACCES" OR error contains "permission":
        errorType = "PERMISSION_DENIED"
        errorDetails = "Cannot read SKILL.md - permission denied"

    ELSE:
        errorType = "UNKNOWN"
        errorDetails = error.message

    # Preserve error context
    errorContext = {
        errorType: errorType,
        filePath: ".claude/skills/devforgeai-ideation/SKILL.md",
        expectedLocation: ".claude/skills/devforgeai-ideation/",
        details: errorDetails,
        timestamp: current_timestamp
    }

    GOTO Skill Load Error Handler
```

**Skill Load Error Handler:**

Display the following error message template:

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  ❌ Skill Loading Failure
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

The devforgeai-ideation skill failed to load.

Error Type: {errorType}
Details: {errorDetails}

Possible causes:
- SKILL.md has invalid YAML frontmatter
- SKILL.md file is missing or corrupted
- Reference files in references/ are missing

Recovery steps:
1. Check: .claude/skills/devforgeai-ideation/SKILL.md exists
2. Validate YAML frontmatter (lines 1-10)
3. Compare with GitHub version: https://github.com/anthropics/claude-code
4. Run: git checkout .claude/skills/devforgeai-ideation/

If issue persists, report at: https://github.com/anthropics/claude-code/issues
```

**Error-Specific Recovery Actions:**

| Error Type | Message | Recovery Action |
|------------|---------|-----------------|
| FILE_MISSING | "SKILL.md not found at expected location" | "Run: git checkout .claude/skills/devforgeai-ideation/" |
| YAML_PARSE_ERROR | "Invalid YAML in frontmatter at line {N}" | "Check frontmatter syntax (lines 1-10)" |
| INVALID_STRUCTURE | "Missing required section: {section_name}" | "Compare with template at https://github.com/anthropics/claude-code" |
| PERMISSION_DENIED | "Cannot read SKILL.md - permission denied" | "Check file permissions: chmod 644" |

**Session Continuity:**

After displaying the error message:
- Session remains active (no terminal crash)
- User can run other commands
- User can retry `/ideate` after repair
- No orphaned processes or corrupted state

```
# After error display, session continues
Display: "Session active. You can run other commands or retry /ideate after repair."
```

**HALT behavior:**

The error handler HALTS the /ideate command but does NOT crash the session.
The user receives actionable recovery instructions and can continue working.

---

### Skill Invocation Failed

**If skill does not execute or throws error:**

```
ERROR: devforgeai-ideation skill invocation failed

Troubleshooting steps:
1. Verify skill file exists:
   Glob(pattern=".claude/skills/devforgeai-ideation/SKILL.md")

2. Check skill is properly registered (restart Claude Code terminal if needed)

3. Verify allowed-tools permissions include Skill tool

If issue persists:
- Review skill file for syntax errors
- Check skill frontmatter is valid YAML
- Try invoking skill directly: Skill(command="devforgeai-ideation")
```

### Skill Validation Failure (Phase 6.4)

**If skill's Phase 6.4 self-validation detects critical failures:**

The skill's Phase 6.4 validates all generated artifacts and reports failures. When skill validation fails:

```
HALT: Skill validation failed

The devforgeai-ideation skill's Phase 6.4 self-validation reported critical failure(s).
Error details are displayed in the skill's validation report above.

The command does NOT attempt recovery or re-validation.
Error messages from skill Phase 6.4 are passed through verbatim.

To resolve:
1. Review the validation error message from the skill
2. Address the specific issue (e.g., missing required field, invalid YAML)
3. Re-run `/ideate [business-idea]` to retry ideation
```

**Note:** Artifact verification (YAML syntax, ID format, required fields) is delegated entirely to the skill's Phase 6.4 self-validation workflow. The command trusts skill validation results without re-verification.

### User Exits During Session

**If user cancels during skill's 10-60 question session:**

```
Ideation incomplete - user exited during requirements phase

To complete ideation:
- Re-run `/ideate [business-idea]` and answer all questions
- Or skip ideation and create requirements manually

Note: Comprehensive requirements gathering ensures zero ambiguity in specifications, preventing technical debt downstream.
```

---

## Command Complete

**This command delegates all implementation logic to the devforgeai-ideation skill.**

**Command responsibilities (lean orchestration):**
- ✅ Argument validation and capture
- ✅ Brainstorm auto-detection and loading
- ✅ Skill invocation with context markers
- ✅ Result interpretation via ideation-result-interpreter subagent (Phase 3)
- ✅ Hook eligibility checking and feedback invocation (Phase N)
- ✅ Error propagation from skill (HALT on skill validation failure)

**Skill responsibilities (all implementation):**
- ✅ Complete 6-phase requirements workflow
- ✅ User interaction (10-60 questions)
- ✅ Epic and requirements generation
- ✅ Self-validation of all artifacts (Phase 6.4)
- ✅ Error handling and recovery

**Result interpretation:** Summary presentation delegated to ideation-result-interpreter subagent (STORY-131).

**Validation delegation:** Command trusts skill's Phase 6.4 self-validation for artifact verification (YAML syntax, ID format, required fields). No duplicate validation logic in command.

**Architecture principle:** Commands orchestrate, skills implement, references provide deep knowledge through progressive disclosure.
