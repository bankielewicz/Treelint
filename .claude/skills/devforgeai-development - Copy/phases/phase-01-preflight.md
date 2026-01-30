# Phase 01: Pre-Flight Validation

**Entry Gate:**
```bash
devforgeai-validate phase-init ${STORY_ID} --project-root=.

Examples (--project-root applies to phase-* commands only, not check-hooks/invoke-hooks):
 - Correct: devforgeai-validate phase-init ${STORY_ID} --project-root=.
 - Incorrect: python -m devforgeai.cli.devforgeai_validate phase-init ${STORY_ID} --project-root=.
# Exit code 0: State file created, proceed
# Exit code 1: State file exists (resume scenario)
# Exit code 2: Invalid story ID - HALT
```

---

## Phase Workflow

**Purpose:** 11-step validation before TDD begins

**Required Subagents:**
- git-validator (Git availability check)
- tech-stack-detector (Technology detection)

**Steps:**

1. **Validate Git status** (git-validator subagent)
   ```
   Task(
     subagent_type="git-validator",
     description="Validate Git repository status",
     prompt="Check Git availability and repository status for workflow strategy"
   )
   ```

1.5. **User consent for git operations** (if uncommitted changes >10)
   - Use AskUserQuestion if uncommitted changes detected
   - Option: Stash, Continue, Abort

1.6. **Stash warning and confirmation** (if user chooses to stash)

1.7. **Check for existing plan file**
   ```
   Glob(".claude/plans/*.md")
   Grep(pattern="${STORY_ID}", path="{plan_file}")
   ```
   - If match found, offer to resume via AskUserQuestion

2. **Git Worktree Auto-Management** (git-worktree-manager subagent)
   ```
   Task(
     subagent_type="git-worktree-manager",
     description="Manage Git worktree for ${STORY_ID}",
     prompt="Create/manage worktree for parallel development"
   )
   ```

3. **Adapt workflow** (Git vs file-based)

4. **File-based tracking setup** (if no Git)

5. **Validate 6 context files exist**
   ```
   Read(file_path="devforgeai/specs/context/tech-stack.md")
   Read(file_path="devforgeai/specs/context/source-tree.md")
   Read(file_path="devforgeai/specs/context/dependencies.md")
   Read(file_path="devforgeai/specs/context/coding-standards.md")
   Read(file_path="devforgeai/specs/context/architecture-constraints.md")
   Read(file_path="devforgeai/specs/context/anti-patterns.md")
   ```

6. **Load story specification**
   ```
   Read(file_path="${STORY_FILE}")
   ```

7. **Validate spec vs context conflicts**

8. **Detect tech stack** (tech-stack-detector subagent)
   ```
   Task(
     subagent_type="tech-stack-detector",
     description="Detect technology stack for ${STORY_ID}",
     prompt="Auto-detect project technologies, validate against tech-stack.md"
   )
   ```

9. **Detect QA failures** (recovery mode check)

9.5. **Load structured gap data** (if gaps.json exists)
   ```
   IF Glob("tests/results/${STORY_ID}/gaps.json"):
     Read(file_path="tests/results/${STORY_ID}/gaps.json")
     SET $REMEDIATION_MODE = true
   ```

10. **Technical Debt Threshold Evaluation** (STORY-289)

    **Purpose:** Enforce tiered alerts when technical debt accumulates to warn/block new development.

    ```
    # Step 10.1: Read technical-debt-register.md for thresholds and debt count
    Read(file_path="devforgeai/technical-debt-register.md")

    # Step 10.2: Parse YAML frontmatter thresholds (configurable per-project)
    # Extract from thresholds section:
    #   warning_count (default: 5)
    #   critical_count (default: 10)
    #   blocking_count (default: 15)
    # If thresholds section missing, use defaults

    warning_count = thresholds.warning_count OR 5
    critical_count = thresholds.critical_count OR 10
    blocking_count = thresholds.blocking_count OR 15

    # Step 10.3: Extract total_open from analytics section
    total_open = analytics.total_open

    # Step 10.4: Check for --ignore-debt-threshold flag
    IGNORE_DEBT_FLAG = "--ignore-debt-threshold" in $ARGUMENTS
    ```

    **Step 10.5: Tiered Threshold Evaluation**

    ```
    IF total_open >= blocking_count:
        # BLOCKING LEVEL (15+ items by default)

        IF IGNORE_DEBT_FLAG:
            # AC#4: Override flag present - prompt for consent
            AskUserQuestion(questions=[{
                question: "Technical debt threshold exceeded ({total_open} items). Override to proceed?",
                header: "Debt Override",
                options: [
                    {label: "Yes, I accept increased technical debt risk", description: "Proceed with workflow, override logged for audit"},
                    {label: "No, I'll reduce debt first", description: "HALT workflow and show remediation guidance"}
                ],
                multiSelect: false
            }])

            IF user selects "Yes, I accept increased technical debt risk":
                # AC#5: Log override and proceed with banner
                # Update phase-state.json with override entry:
                Edit(file_path="devforgeai/workflows/${STORY_ID}-phase-state.json") to add:
                {
                    "debt_override": {
                        "timestamp": "{current_timestamp}",
                        "debt_count": {total_open},
                        "acknowledgment": "User accepted technical debt risk"
                    }
                }

                Display: "⚠️ DEBT OVERRIDE ACTIVE: Proceeding with {total_open} open debt items"
                SET $DEBT_OVERRIDE_BANNER = true
                # Workflow proceeds with persistent warning banner

            ELSE:
                # AC#6: User declined - HALT on decline and show remediation guidance
                # User must reduce debt first. Workflow does not proceed.
                # Get 5 oldest open debt items sorted by date ascending (oldest first)
                oldest_items = parse_oldest_open_items(register, count=5)

                Display:
                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
                "❌ HALT on decline - Technical Debt Remediation Required"
                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
                ""
                "Oldest 5 open debt items to prioritize (sorted ascending):"
                FOR item in oldest_items:
                    Display: "  • {item.id}: {item.description}"
                    Display: "    Estimated effort: {item.effort} points | Linked follow-up story: STORY-XXX or {item.follow_up_story OR 'None'}"
                ""
                "Suggested action: Run '/dev STORY-XXX' on existing remediation stories"
                "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

                # Workflow does not proceed - must reduce debt first
                HALT workflow
        ELSE:
            # AC#3: No override flag - HALT with blocking message
            oldest_items = parse_oldest_open_items(register, count=5)

            Display:
            "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            "❌ BLOCKED: Technical debt exceeds threshold ({total_open}/{blocking_count} items)"
            "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            ""
            "Reduce debt before starting new work."
            ""
            "Oldest 5 open debt items (DEBT-NNN IDs):"
            FOR item in oldest_items:
                Display: "  • {item.id}: {item.description}"
            ""
            "To override: /dev {STORY_ID} --ignore-debt-threshold"
            "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

            # Workflow does not proceed - HALT workflow
            HALT workflow

    ELIF total_open >= critical_count:
        # AC#2: CRITICAL LEVEL (10-14 items by default)
        # Note: Escalation from warning to critical (10 > 5 threshold escalation)
        Display:
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        "🔴 CRITICAL: Technical debt at {total_open} items (threshold: {critical_count})"
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        "Strongly recommended to reduce debt before new development."
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

        # Workflow proceeds with prominent notice

    ELIF total_open >= warning_count:
        # AC#1: WARNING LEVEL (5-9 items by default)
        Display:
        "⚠️ Technical debt warning: {total_open} open items (threshold: {warning_count})"
        "Consider addressing debt before starting new work."

        # Workflow proceeds normally

    ELSE:
        # Below all thresholds - proceed silently
        # No display needed
    ```

    **Reference:** `references/preflight/01.10-debt-threshold.md` for complete threshold evaluation workflow

**Reference:** `references/preflight/_index.md` for navigation (decomposed into 18 files)

**Load workflow references as needed:**
```
# Index with navigation links
Read(file_path=".claude/skills/devforgeai-development/references/preflight/_index.md")

# Mandatory steps (load in sequence)
Read(file_path=".claude/skills/devforgeai-development/references/preflight/01.0-project-root.md")
Read(file_path=".claude/skills/devforgeai-development/references/preflight/01.0.5-cli-check.md")
Read(file_path=".claude/skills/devforgeai-development/references/preflight/01.1-git-status.md")
Read(file_path=".claude/skills/devforgeai-development/references/preflight/01.5-context-files.md")
Read(file_path=".claude/skills/devforgeai-development/references/preflight/01.6-load-story.md")
Read(file_path=".claude/skills/devforgeai-development/references/preflight/01.7-validate-spec.md")
Read(file_path=".claude/skills/devforgeai-development/references/preflight/01.8-tech-stack.md")
Read(file_path=".claude/skills/devforgeai-development/references/preflight/01.9-qa-failures.md")

# Conditional steps (load when triggered)
# IF uncommitted_changes > 10:
Read(file_path=".claude/skills/devforgeai-development/references/preflight/01.1.5-user-consent.md")
# IF user selects stash:
Read(file_path=".claude/skills/devforgeai-development/references/preflight/01.1.6-stash-warning.md")
# IF uncommitted story files:
Read(file_path=".claude/skills/devforgeai-development/references/preflight/01.1.7-story-isolation.md")
# IF Git available + enabled:
Read(file_path=".claude/skills/devforgeai-development/references/preflight/01.2-worktree.md")
# IF story has dependencies:
Read(file_path=".claude/skills/devforgeai-development/references/preflight/01.2.5-dependency-graph.md")
# IF parallel stories:
Read(file_path=".claude/skills/devforgeai-development/references/preflight/01.2.6-file-overlap.md")
# IF Git unavailable:
Read(file_path=".claude/skills/devforgeai-development/references/preflight/01.4-file-tracking.md")

# Informational
Read(file_path=".claude/skills/devforgeai-development/references/preflight/01.3-workflow-adapt.md")
Read(file_path=".claude/skills/devforgeai-development/references/preflight/01.10-complexity.md")
Read(file_path=".claude/skills/devforgeai-development/references/preflight/completion-checkpoint.md")
```  

---

## Validation Checkpoint

**Before proceeding to Phase 02, verify:**

- [ ] git-validator subagent invoked
- [ ] Context files validated (6 files)
- [ ] Story specification loaded
- [ ] tech-stack-detector subagent invoked

**IF any checkbox UNCHECKED:** HALT workflow

---

## Observation Capture

**Before exiting this phase, reflect:**
1. Did I encounter any friction? (unclear docs, missing tools, workarounds)
2. Did anything work particularly well? (constraints that helped, patterns that fit)
3. Did I notice any repeated patterns?
4. Are there gaps in tooling/docs?
5. Did I discover any bugs?

**If YES to any:** Append to phase-state.json `observations` array:
```json
{
  "id": "obs-01-{seq}",
  "phase": "01",
  "category": "{friction|success|pattern|gap|idea|bug}",
  "note": "{1-2 sentence description}",
  "files": ["{relevant files}"],
  "severity": "{low|medium|high}"
}
```

**Reference:** `references/observation-capture.md`
    Read(file_path=".claude/skills/devforgeai-development/references/observation-capture.md")

---

**Exit Gate:**
```bash
devforgeai-validate phase-complete ${STORY_ID} --phase=01 --checkpoint-passed
# Exit code 0: Phase complete, proceed to Phase 02
# Exit code 1: Cannot complete - validation failed
```

---

**Record Subagents:**
```bash
# Record after each subagent invocation:
# (Called automatically by orchestrator)
# devforgeai-validate phase-record ${STORY_ID} --phase=01 --subagent=git-validator
# devforgeai-validate phase-record ${STORY_ID} --phase=01 --subagent=tech-stack-detector
```
