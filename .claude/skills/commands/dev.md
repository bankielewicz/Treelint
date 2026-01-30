---
description: Implement user story using TDD workflow
argument-hint: [STORY-ID] [mode]
model: opus
allowed-tools: AskUserQuestion, Read, Skill, Bash(git:*)
execution-mode: immediate
---

# /dev - TDD Development Workflow

Execute full Test-Driven Development cycle for a user story following lean orchestration pattern.  

Do not skip any phases nor skip the devforgeai-development skill.

---

## Quick Reference

```bash
# Standard usage
/dev STORY-001

# Auto-detection: If gaps.json exists, remediation mode is enabled automatically
# The command checks devforgeai/qa/reports/STORY-XXX-gaps.json before invoking skill

# Bypass dependency checks (not recommended)
/dev STORY-001 --force

# Explicit remediation mode (overrides auto-detection)
/dev STORY-001 --fix

# Override technical debt blocking threshold (15+ items)
/dev STORY-001 --ignore-debt-threshold

# Development workflow states
Backlog | Ready for Dev | In Development → Dev Complete

# Next steps after dev complete
/qa STORY-001        # Run QA validation
/orchestrate STORY-001  # Full lifecycle
```

---

## Command Workflow

### Phase 0: Plan Mode Detection and Argument Validation

**Step 0.0: Plan Mode Auto-Exit [execution-mode: immediate]**

This command has `execution-mode: immediate` in frontmatter. If plan mode is currently active, auto-exit plan mode before proceeding:

```
IF plan mode is active:
    Display: "Note: /dev is an execution command. Exiting plan mode automatically."
    ExitPlanMode()
```

---

**Step 0.1: Parse Arguments and Flags**

```
STORY_ID = null
FORCE_FLAG = false
REMEDIATION_MODE = false
GAPS_AUTO_DETECTED = false
Mode = TDD

# Parse arguments
FOR arg in arguments:
    IF arg == "--force":
        FORCE_FLAG = true
    ELIF arg matches "STORY-[0-9]+":
        STORY_ID = arg
    ELIF arg == "--fix":
        REMEDIATION_MODE = true

IF STORY_ID empty:
    Display: "Usage: /dev STORY-NNN [--force]"
    Display: "Example: /dev STORY-001"
    Display: "         /dev STORY-001 --force  (bypass dependency checks)"
    Display: "         /dev STORY-001 --fix    (Resolve QA issues noted in STORY-[0-9]-gaps.json file)"
    HALT

IF FORCE_FLAG == true:
    Display: "⚠️  Force mode enabled - dependency checks will be bypassed"
    Display: "    This is logged for audit purposes."
    Display: ""

IF REMEDIATION_MODE == true:
    Display: "⚠️  Remediation mode enabled"
    Display: ""
    $MODE = Remediation Mode
```

**Step 0.2: Load Story File**

```
@devforgeai/specs/Stories/$1*.story.md

IF file not found:
    Display: "Story not found: $1"
    Display: "Run: Glob(pattern='devforgeai/specs/Stories/*.story.md') to list stories"
    HALT

Display: "✓ Story: $1"
```

**Step 0.3: Auto-detect gaps.json (Remediation Mode)**

```
# Construct canonical gaps path (COMP-002)
# Security: STORY_ID validated in Step 0.1 (regex: STORY-[0-9]+) - no path traversal possible
GAPS_FILE_PATH = "devforgeai/qa/reports/${STORY_ID}-gaps.json"

# Check if gaps file exists using Glob (COMP-003)
# Uses native Glob tool for gaps.json detection (per tech-stack.md)
gaps_result = Glob(pattern="${GAPS_FILE_PATH}")  # Returns match if gaps.json exists

IF gaps_result is not empty AND REMEDIATION_MODE != true:
    # Auto-detection found gaps file
    GAPS_AUTO_DETECTED = true
    REMEDIATION_MODE = true
    Display: "🔧 Auto-detected gaps.json - Remediation mode enabled"
    Display: "   Path: ${GAPS_FILE_PATH}"
    Display: ""
ELSE:  # gaps.json not found OR explicit --fix already set
    # REMEDIATION_MODE remains unchanged (false from Step 0.1 OR true from --fix)
    # No additional notification needed

# Note: Explicit --fix flag (Step 0.1) takes priority over auto-detection
```

---

### Phase 1: Invoke Skill

**Set context markers for skill:**
```
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  DevForgeAI Development Workflow"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""
Display: "**Story ID:** $1"
Display: ""
Display: "Executing TDD workflow (Red → Green → Refactor → Integration)..."
Display: ""
```

**Invoke devforgeai-development skill:**
```
**Story ID:** ${STORY_ID}
**Development Mode:** ${MODE}

Skill(command="devforgeai-development")
```

**Skill executes:**
- Phase 01: Pre-Flight Validation (git, context files, tech stack, context-preservation-validator) **READ** .claude/skills/devforgeai-development/references/preflight/_index.md
- Phase 02-05: TDD Cycle (Red → Green → Refactor → Integration) **READ** .claude/rules/workflow/tdd-workflow.md  
- Phase 06: Deferral Challenge (validate deferrals, user approval) **READ** .claude/skills/devforgeai-development/references/phase-06-deferral-challenge.md
- Phase 07: DoD Update (format validation) **READ** .claude/skills/devforgeai-development/references/dod-update-workflow.md
- Phase 08: Git Workflow (commit or file-based tracking) **READ** claude/skills/devforgeai-development/references/git-workflow-conventions.md
- Phase 09: Feedback Hook (check/invoke hooks if enabled)
- Phase 10: Result Interpretation (invoke dev-result-interpreter, return display)

**Skill returns structured result to command.**

---

### Phase 2: Display Results

**Skill has returned result object with display template:**

```
# result = {
#   status: "success|incomplete|failure",
#   display: {
#     template: "..." (formatted display),
#     next_steps: ["...", "..."]
#   },
#   story_status: "Dev Complete",
#   workflow_summary: "..."
# }

Display: result.display.template
Display: ""
Display: "Next Steps:"
FOR step in result.display.next_steps:
    Display: "  • {step}"
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
```

**No processing, parsing, or template generation in command.**
**All business logic delegated to skill, all display templates generated by dev-result-interpreter subagent.**

---

## Error Handling

**Invalid Story ID:** Usage: /dev STORY-NNN (e.g., /dev STORY-001)

**Story Not Found:** Run Glob(pattern="devforgeai/specs/Stories/*.story.md") to list stories

**Skill Failed:** Review skill output above. Common: context files missing (/create-context), git not initialized, technology conflicts (tech-stack.md)

---

## --ignore-debt-threshold Flag

Overrides the technical debt blocking threshold (default: 15 items).

**Usage:**
```bash
/dev STORY-XXX --ignore-debt-threshold
```

**When to Use:**
- Technical debt count is at or above blocking threshold (15+ open items)
- You need to proceed with development despite accumulated debt
- Emergency fixes that cannot wait for debt reduction

**Behavior:**
1. If debt count >= blocking threshold, prompts for user consent via `AskUserQuestion`:
   - Header: "Debt Override"
   - Question: "Technical debt threshold exceeded ({count} items). Override to proceed?"
   - Options: "Yes, I accept increased technical debt risk" | "No, I'll reduce debt first"
2. If user selects "Yes": Override is logged and workflow proceeds with persistent warning banner
3. If user selects "No": Workflow HALTs with remediation guidance (oldest 5 debt items shown)

**Audit Trail:**
- Override is logged to `devforgeai/workflows/{STORY-ID}-phase-state.json` with:
  - Timestamp
  - Debt count at time of override
  - User acknowledgment text

**Example Output When Blocked:**
```
❌ BLOCKED: Technical debt exceeds threshold (18 items, threshold: 15)

Oldest 5 debt items for prioritized remediation:
  1. DEBT-001: Missing unit tests for auth module (2025-11-15)
  2. DEBT-003: Hardcoded config values in payment service (2025-11-20)
  ...

To proceed anyway, use: /dev STORY-XXX --ignore-debt-threshold
```

**Reference:** STORY-289, `src/claude/memory/Constitution/tech-stack.md` (Technical Debt Threshold Configuration)

---

**Refactored 2025-11-18 (STORY-051):** 527 → 131 lines (75% reduction), 17,460 → 3,794 chars (78% reduction, 25% budget)
**Pattern:** Case Study 6 - Extracted result verification to dev-result-interpreter subagent
