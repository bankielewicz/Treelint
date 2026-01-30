---
description: Validate epic coverage and report gaps
argument-hint: "[EPIC-ID] [--interactive | --quiet | --ci] [--help]"
model: opus
allowed-tools: Read, Glob, Grep, Bash, AskUserQuestion, Skill
---

# /validate-epic-coverage - Epic Coverage Validation Command

Validate epic-to-story coverage and report gaps with actionable commands.

---

## Quick Reference

```bash
# Validate all epics
/validate-epic-coverage

# Validate single epic
/validate-epic-coverage EPIC-015

# Display help
/validate-epic-coverage --help
/validate-epic-coverage help
```

---

## Architectural Constraints

**Layer Dependencies (Enforced):**

This command implements a unidirectional dependency flow with the devforgeai-story-creation skill:

```
Flow: Command → invokes → Skill (unidirectional)
  - /validate-epic-coverage CAN invoke devforgeai-story-creation skill
  - devforgeai-story-creation skill CANNOT invoke /validate-epic-coverage command
```

**Rationale:**
- Ensures **separation of concerns**: commands orchestrate workflows, skills execute implementation
- Prevents **circular dependencies**: eliminates command→skill→command loops
- Enforces **architectural boundaries**: maintains clean three-layer architecture (Commands → Skills → Subagents)

**Violation Examples (FORBIDDEN):**
- ❌ Skills invoking /create-story or /validate-epic-coverage commands
- ❌ Skill → Command → Skill circular flow
- ❌ Circular references through intermediate components

---

## Command Workflow

### Phase 0: Argument Validation

**Parse command arguments:**
```
ARG=$1

IF ARG == "--help" OR ARG == "help":
    GOTO display_help

IF ARG is provided AND ARG does NOT match "^EPIC-[0-9]{3}$" (case-insensitive):
    Display error and valid epics
    HALT

IF ARG matches epic ID format:
    MODE = "single"
    EPIC_ID = normalize(ARG)  # Uppercase, e.g., EPIC-015
ELSE:
    MODE = "all"
```

---

### Phase 0.1: Mode Detection (AC#8)

**Parse mode flags:**
```
# Check for mode flags in arguments
INTERACTIVE_MODE = "--interactive" in ARGS OR default
QUIET_MODE = "--quiet" in ARGS OR "--ci" in ARGS
CI_ENVIRONMENT = (no TTY detected) OR (CI=true in environment)

# Determine final mode
IF QUIET_MODE OR CI_ENVIRONMENT:
    PROMPT_MODE = "quiet"      # Suppress all interactive prompts
ELSE IF INTERACTIVE_MODE:
    PROMPT_MODE = "interactive"  # Enable gap-to-story prompts (default)
ELSE:
    PROMPT_MODE = "interactive"  # Default in terminal
```

**Mode behaviors:**
- `--interactive`: Enable gap-to-story prompts (default in terminal)
- `--quiet` or `--ci`: Suppress prompts, output only (for automation)
- Auto-detect: CI environment (no TTY) automatically uses quiet mode

**Display header:**
```
Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  Epic Coverage Validation"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""
```

---

### Phase 1: Execute Validation

**Verify prerequisites:**
```
Glob(pattern="devforgeai/specs/Epics/*.epic.md")

IF no epics found:
    Display: "ℹ️ No epics found in devforgeai/specs/Epics/"
    Display: ""
    Display: "To create epics, run: /create-epic"
    Exit with success (no errors)
```

**For single-epic mode:**
```
IF MODE == "single":
    Glob(pattern="devforgeai/specs/Epics/${EPIC_ID}*.epic.md")

    IF file not found:
        Display: "❌ Epic not found: ${EPIC_ID}"
        Display: ""
        Display: "Valid epics:"
        FOR epic in Glob("devforgeai/specs/Epics/*.epic.md"):
            Extract epic_id from filename
            Display: "  • ${epic_id}"
        Display: ""
        Display: "💡 Run /validate-epic-coverage without arguments to validate all epics"
        HALT

    Run validation for single epic
```

**For all-epics mode:**
```
IF MODE == "all":
    Run validation for all epics
```

**Execute coverage report generator:**
```
# Use existing generate-report.sh for coverage analysis
Bash(command="devforgeai/epic-coverage/generate-report.sh")

# Capture and parse output
```

**Execute gap detector for actionable gaps:**
```
IF MODE == "single":
    Bash(command="devforgeai/traceability/gap-detector.sh ${EPIC_ID}")
ELSE:
    Bash(command="devforgeai/traceability/gap-detector.sh")
```

---

### Phase 2: Display Results

**Format output with visual indicators:**

**Color Legend:**
- ✅ GREEN (100% coverage): All features have stories
- ⚠️ YELLOW (50-99% coverage): Partial feature coverage
- ❌ RED (<50% coverage): Most features missing stories

**For single-epic mode, display feature-by-feature:**
```
Display: "📊 Coverage Report: ${EPIC_ID}"
Display: ""

FOR feature in epic.features:
    IF feature.has_coverage:
        Display: "✅ Feature ${N}: ${feature.name} - COVERED"
        FOR story in feature.stories:
            Display: "   └─ ${story.id} [${story.status}]"
    ELSE IF feature.has_partial:
        Display: "⚠️ Feature ${N}: ${feature.name} - PARTIAL"
    ELSE:
        Display: "❌ Feature ${N}: ${feature.name} - GAP"
        Display: "   💡 To fill gap: /create-story \"${EPIC_ID} Feature ${N}: ${feature.name}\""

Display: ""
Display: "Coverage: ${covered}/${total} features (${percentage}%)"
```

**For all-epics mode, display summary table:**
```
Display: "📊 Framework Coverage Report"
Display: ""
Display: "| Epic ID   | Title                    | Coverage |"
Display: "|-----------|--------------------------|----------|"

FOR epic in epics:
    IF coverage == 100:
        indicator = "✅"
    ELSE IF coverage >= 50:
        indicator = "⚠️"
    ELSE:
        indicator = "❌"

    Display: "| ${epic.id} | ${epic.title} | ${indicator} ${coverage}% |"

Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "Framework Coverage: ${total_coverage}% (${covered_features}/${total_features} features)"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
```

**Display actionable gaps:**
```
IF gaps_found:
    Display: ""
    Display: "🔧 Actionable Gaps (top 10):"
    Display: ""

    FOR gap in gaps (limit 10):
        # Shell-safe escape feature description
        escaped_desc = escape_shell_safe(gap.feature_description)
        Display: "  /create-story \"${gap.epic_id} Feature ${gap.feature_num}: ${escaped_desc}\""

    IF total_gaps > 10:
        Display: ""
        Display: "  ... and ${total_gaps - 10} more gaps"

    # AC#5: Batch creation hint
    Display: ""
    Display: "💡 To create all missing stories: /create-missing-stories ${EPIC_ID}"
```

---

### Phase 2.1: Interactive Gap Resolution (AC#1, AC#3)

**Skip if quiet mode:**
```
IF PROMPT_MODE == "quiet":
    # Skip interactive prompts in quiet/CI mode
    GOTO end_of_command
```

**Single gap prompt (AC#1):**
```
IF gaps.length == 1 AND PROMPT_MODE == "interactive":
    gap = gaps[0]

    Display: ""
    Display: "📝 Gap Resolution"
    Display: ""

    AskUserQuestion:
        Question: "Create story for '${gap.feature_title}'?"
        Options:
            - "Yes - Create story now"
            - "No - Skip this gap"
            - "Later - I'll create it manually"

    IF user_selection == "Yes - Create story now":
        # Generate batch context markers
        Set context:
            **Story ID:** ${next_story_id}
            **Epic ID:** ${gap.epic_id}
            **Feature Number:** ${gap.feature_number}
            **Feature Name:** ${gap.feature_title}
            **Feature Description:** ${gap.feature_title} - ${gap.feature_description}. Implements ${gap.epic_id} Feature ${gap.feature_number}.
            **Batch Mode:** false

        # Invoke story creation
        Skill(command="devforgeai-story-creation")

        Display: "✅ Story created: ${story_id}"
```

**Multiple gaps prompt (AC#3):**
```
IF gaps.length >= 2 AND PROMPT_MODE == "interactive":
    Display: ""
    Display: "📝 Gap Resolution"
    Display: ""
    Display: "Found ${gaps.length} gaps in ${EPIC_ID}."
    Display: ""

    AskUserQuestion:
        Question: "How would you like to proceed?"
        Options:
            - "Create all ${gaps.length} stories now (batch mode)"
            - "Select specific gaps to fill"
            - "Skip - I'll create stories later"

    IF user_selection == "Create all ... stories now":
        # Invoke batch creation command
        Display: "Launching batch creation..."
        # Pass control to /create-missing-stories
        Execute: /create-missing-stories ${EPIC_ID}

    ELSE IF user_selection == "Select specific gaps to fill":
        # Multi-select mode
        AskUserQuestion:
            Question: "Select gaps to create stories for:"
            MultiSelect: true
            Options: [gap.feature_title for gap in gaps]

        FOR selected_gap IN user_selections:
            # Create story for each selected gap
            Set batch context markers
            Skill(command="devforgeai-story-creation")

        Display: "✅ Created ${user_selections.length} stories"

    ELSE:
        Display: "Skipped. Run /create-missing-stories ${EPIC_ID} when ready."
```

---

## Help Text

When `--help` or `help` is provided:

```
/validate-epic-coverage - Validate epic coverage and report gaps

USAGE:
    /validate-epic-coverage [EPIC-ID] [OPTIONS]
    /validate-epic-coverage --help

ARGUMENTS:
    EPIC-ID     Optional. Validate specific epic (e.g., EPIC-015)
                If omitted, validates all epics in devforgeai/specs/Epics/

OPTIONS:
    --interactive   Enable gap-to-story prompts (default in terminal)
    --quiet         Suppress interactive prompts (for scripting)
    --ci            Same as --quiet, for CI/CD environments
    --help, help    Display this help message

EXAMPLES:
    # Validate all epics and show framework coverage
    /validate-epic-coverage

    # Validate single epic with feature-by-feature breakdown
    /validate-epic-coverage EPIC-015

    # Validate with interactive gap resolution prompts
    /validate-epic-coverage EPIC-015 --interactive

    # Validate without prompts (CI mode)
    /validate-epic-coverage EPIC-015 --quiet

OUTPUT:
    - Color-coded coverage indicators (✅ ⚠️ ❌)
    - Per-epic breakdown with feature coverage
    - Actionable /create-story commands for gaps
    - Batch creation hint: /create-missing-stories
    - Framework-wide coverage percentage
    - Interactive gap resolution prompts (unless --quiet)

RELATED COMMANDS:
    /create-story           Create story to fill coverage gap
    /create-missing-stories Batch create stories for all gaps
    /create-epic            Create new epic with features
    /audit-deferrals        Audit deferred DoD items

EXIT CODES:
    0    Success (validation completed)
    1    Error (invalid arguments or file system error)
```

---

## Error Handling

### Invalid Epic ID Format
```
❌ Invalid epic ID format: ${PROVIDED_ID}

Expected format: EPIC-NNN (e.g., EPIC-015)
The ID is case-insensitive (epic-015 works).

Run /validate-epic-coverage without arguments to see all epics.
```

### Epic Not Found
```
❌ Epic not found: ${EPIC_ID}

Valid epics:
  • EPIC-001
  • EPIC-002
  • ...

💡 Run /validate-epic-coverage without arguments to validate all epics
```

### File System Error
```
⚠️ Warning: Could not process ${FILE_PATH}
   Reason: ${ERROR_MESSAGE}

Continuing with remaining epics...
```

---

## Business Rules

1. **Epic ID Format (BR-001):** Accepts case-insensitive input. `epic-015` normalized to `EPIC-015`.

2. **Coverage Counting (BR-002):** Only stories with status >= "Dev Complete" count toward coverage. Backlog stories show as "Planned" but don't contribute.

3. **Shell-Safe Escaping (BR-003):** Feature descriptions with special characters (quotes, backticks, $) are properly escaped in /create-story suggestions.

4. **Empty Directory (BR-004):** Empty epics directory returns success with informational message.

---

## Performance Targets

- Single epic: <500ms
- All epics (20 epics, 200 stories): <3 seconds

---

## Implementation Notes

**Architecture (STORY-087):**

This command follows **lean orchestration** pattern:
- **Argument validation:** Parse epic ID or help flag
- **Service invocation:** Delegate to generate-report.sh and gap-detector.sh
- **Result display:** Format output with visual indicators

**Key services:**
- `devforgeai/epic-coverage/generate-report.sh` - Coverage statistics
- `devforgeai/traceability/gap-detector.sh` - Gap detection

**Dependencies:**
- STORY-085: Gap Detection Engine (QA Approved)
- STORY-086: Coverage Reporting System (QA Approved)
- STORY-088: /create-story Integration for Gap Resolution (In Development)

**STORY-088 Enhancements:**
- AC#1: Interactive gap-to-story prompt (Phase 2.1)
- AC#3: Batch creation prompt for multiple gaps (Phase 2.1)
- AC#5: Integration point with /create-missing-stories hint
- AC#8: Hybrid mode toggle (--interactive, --quiet, --ci)

---

**Created:** 2025-12-13 (STORY-087)
**Enhanced:** 2025-12-13 (STORY-088)
**Pattern:** Lean orchestration (orchestrate, don't implement)
