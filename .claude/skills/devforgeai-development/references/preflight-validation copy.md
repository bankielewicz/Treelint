# Phase 01: Pre-Flight Validation

**Purpose:** Comprehensive validation before TDD workflow begins. This phase ensures all prerequisites are met.

**Execution:** Before Phase 02 (Red phase) starts

**Token Cost:** ~6,000 tokens when loaded

---

## Phase Progress Indicator

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 01/10: Pre-Flight Validation (0% → 10% complete)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Display this indicator at the start of Phase 01 execution.**

---

## Overview

Phase 01 executes 10 validation steps before proceeding to TDD implementation. This prevents starting work in an invalid environment.

**Steps:**
0. **Validate Project Root (CWD)** - NEW
1. Validate Git repository status
1.7. **Story File Isolation Check** (STORY-123) - NEW
2. **Git Worktree Auto-Management** (STORY-091)
2.5. **Dependency Graph Validation** (STORY-093) - NEW
3. Adapt TDD workflow based on Git availability
4. File-based change tracking template (if no Git)
5. Validate context files exist
6. Load story specification
7. Validate spec vs context files
8. Detect and validate technology stack
9. Detect previous QA failures
9.5. Load structured gap data (if QA failed)
10. **Story Complexity Analysis** (STORY-172) - NEW

---

## Phase 01.0: Validate Project Root [MANDATORY - FIRST STEP]

**Purpose:** Ensure CWD is DevForgeAI project root before ANY file operations.

**Execute BEFORE Phase 01.1 (Git validation):**

```
# 1. Attempt to read project marker file
result = Read(file_path="CLAUDE.md")

IF result.success:
    content = result.content

    # 2. Validate it's a DevForgeAI project
    IF content_contains("DevForgeAI") OR content_contains("devforgeai"):
        CWD_VALID = true
        PROJECT_ROOT = Bash(command="pwd").output.strip()
        Display: "✓ Project root validated: {PROJECT_ROOT}"
    ELSE:
        # CLAUDE.md exists but not DevForgeAI project
        CWD_VALID = false
        Display: "⚠ CLAUDE.md found but not a DevForgeAI project"
        HALT: Use AskUserQuestion to get correct path
ELSE:
    # 3. Try secondary markers
    dir_check = Glob(pattern=".claude/skills/*.md")

    IF dir_check.has_results:
        CWD_VALID = true
        Display: "✓ Project root validated via .claude/skills/ structure"
    ELSE:
        dir_check2 = Glob(pattern="devforgeai/specs/context/*.md")

        IF dir_check2.has_results:
            CWD_VALID = true
            Display: "✓ Project root validated via devforgeai/specs/"
        ELSE:
            CWD_VALID = false
            Display: "❌ CWD Validation Failed"
            Display: "   Not in DevForgeAI project root."
            Display: "   Expected markers not found:"
            Display: "     - CLAUDE.md"
            Display: "     - .claude/skills/"
            Display: "     - devforgeai/specs/"
            HALT: Use AskUserQuestion: "Provide project root path?"
```

**On Failure:**

```
❌ CWD Validation Failed
   Current directory does not appear to be a DevForgeAI project root.
   Expected: CLAUDE.md with DevForgeAI configuration
   Found: [nothing | wrong project]

   Options:
   1. Navigate to correct project directory
   2. Run: cd /path/to/your/devforgeai/project
   3. Provide absolute path for this session
```

**CRITICAL:** Do NOT proceed to Phase 01.1 if CWD validation fails.

---

## Phase 01.0.5: CLI Availability Check [MANDATORY] (STORY-129)

**Purpose:** Verify devforgeai CLI is installed before attempting CLI-based validations. Provides graceful fallback when CLI not available.

**When to execute:** After Phase 01.0 (Project Root Validation), before Phase 01.1 (Git Validation)

**Token Cost:** ~100 tokens

**Implementation:**

```bash
# Check if devforgeai-validate CLI is available
if ! command -v devforgeai-validate &> /dev/null; then
    CLI_AVAILABLE=false
    echo "WARN: devforgeai-validate CLI not installed"
    echo "  - Hook checks will be skipped"
    echo "  - Manual validation required"
else
    CLI_AVAILABLE=true
    DEVFORGEAI_VERSION=$(devforgeai-validate --version 2>/dev/null || echo "unknown")
    echo "✓ devforgeai-validate CLI: $DEVFORGEAI_VERSION"
fi

# Store for downstream use
$CLI_AVAILABLE = CLI_AVAILABLE
```

**Downstream Impact:**

When `CLI_AVAILABLE=false`, the following steps SKIP CLI calls:

| Step | CLI Command | Skip Message |
|------|-------------|--------------|
| Phase 01.X | `devforgeai check-hooks` | "Skipping: CLI-based hook checks (CLI not available)" |
| Phase 06/07 | `devforgeai validate-dod` | "Skipping: CLI DoD validation (CLI not available)" |
| Phase 01.5 | `devforgeai validate-context` | "Skipping: CLI context validation (CLI not available)" |

**Fallback Validation When CLI Not Available:**

### Manual Validation Patterns (Claude Code Terminal Tools)

**Hook Eligibility (replaces devforgeai check-hooks):**
```
Grep(pattern="operation: dev", path="src/devforgeai/config/hooks.yaml", output_mode="count")
If count > 0: Hooks enabled for dev operation
```

**DoD Validation (replaces devforgeai validate-dod):**
```
Grep(pattern="^\\s*-\\s*\\[[ x]\\]", path="$STORY_FILE", output_mode="count")
Count represents number of DoD checkbox items
```

**Context Validation (replaces devforgeai validate-context):**
```
For each context file, verify exists:
- Read(file_path="devforgeai/specs/context/tech-stack.md")
- Read(file_path="devforgeai/specs/context/source-tree.md")
- Read(file_path="devforgeai/specs/context/dependencies.md")
- Read(file_path="devforgeai/specs/context/coding-standards.md")
- Read(file_path="devforgeai/specs/context/architecture-constraints.md")
- Read(file_path="devforgeai/specs/context/anti-patterns.md")

If ANY Read fails: Context incomplete - run /create-context
```

**Risks When CLI Validation Skipped:**

| Skipped Validation | Risk | Mitigation |
|-------------------|------|------------|
| Hook checks | Hooks may not trigger | Fallback Grep pattern detects hook config |
| DoD validation | Format errors may pass | Fallback regex validates checkboxes exist |
| Context validation | Missing files may not be detected early | Fallback Read validates 6 files exist |

**Success:** `CLI_AVAILABLE` variable set for downstream steps to check.

**Failure:** N/A - this step always succeeds (produces warning, never blocks).

---

## Phase 01.1: Validate Git Repository Status [MANDATORY]

**Invoke git-validator subagent to check Git availability:**

```
Task(
  subagent_type="git-validator",
  description="Validate Git repository status",
  prompt="Check the Git repository status for the current directory.

  Validate:
  1. Is Git installed and accessible?
  2. Is this directory a Git repository?
  3. Are there existing commits?
  4. What is the current branch?
  5. Are there uncommitted changes?

  Return JSON with Git status, assessment, and recommendations.

  CRITICAL: Always provide fallback strategy if Git unavailable - DevForgeAI must adapt gracefully."
)
```

**Parse subagent JSON response:**

```javascript
result = parse_json(subagent_output)

# Extract workflow configuration
GIT_AVAILABLE = result["git_status"]["installed"] AND result["git_status"]["repository_exists"]
WORKFLOW_MODE = result["assessment"]["workflow_mode"]  # "full", "partial", or "fallback"
CAN_COMMIT = result["assessment"]["can_commit"]
CURRENT_BRANCH = result["git_status"]["current_branch"]
UNCOMMITTED_CHANGES = result["git_status"]["uncommitted_changes"]

# Display status to user
IF WORKFLOW_MODE == "full":
    Display: "✓ Git repository validated - full workflow enabled"
    Display: "  - Repository: Initialized with {result['git_status']['commit_count']} commits"
    Display: "  - Branch: {CURRENT_BRANCH}"
    Display: "  - Uncommitted changes: {UNCOMMITTED_CHANGES}"

    IF UNCOMMITTED_CHANGES > 0:
        Display: "  ⚠️  Warning: {UNCOMMITTED_CHANGES} uncommitted changes detected"
        Display: "  Recommendation: Commit or stash before proceeding"

ELIF WORKFLOW_MODE == "partial":
    Display: "⚠ Git repository needs initial commit"
    Display: "  Repository initialized but no commits yet"
    Display: "  Recommendation:"
    FOR cmd in result["recommendations"]["commands"]:
        Display: "    {cmd}"

ELIF WORKFLOW_MODE == "fallback":
    IF result["git_status"]["installed"]:
        Display: "⚠ Git available but repository not initialized"
        Display: "  To enable full workflow:"
        FOR cmd in result["recommendations"]["commands"]:
            Display: "    {cmd}"
    ELSE:
        Display: "⚠ Git not installed - file-based workflow enabled"
        Display: "  Changes will be tracked in:"
        Display: "    devforgeai/stories/{STORY-ID}/changes/"

    Display: ""
    Display: "  Fallback mode active (limited version control features)"

# Store flags for workflow adaptation
$GIT_AVAILABLE = GIT_AVAILABLE
$WORKFLOW_MODE = WORKFLOW_MODE
$CAN_COMMIT = CAN_COMMIT
```

**Token cost:** ~500 tokens in main conversation (~3,000 in isolated subagent context)

**Benefits:**
- Context isolation (Git checks in separate context window)
- Reusable validation (other skills can use git-validator)
- Framework-aware (subagent understands fallback strategies)
- Structured output (JSON parsing vs text interpretation)

---

## Phase 01.1.5: User Consent for Git State Changes [MANDATORY IF uncommitted > 10] (RCA-008)

**CRITICAL: This step prevents autonomous file hiding (RCA-008 incident - 2025-11-13).**

**When to execute:** After git-validator returns results from Phase 01.1

**Trigger condition:**
- `uncommitted_changes > 10` OR
- `untracked_files > 0` (if git-validator provides this data)

**Purpose:** Obtain explicit user consent before any git operation that would hide/modify files, preventing the autonomous stashing incident where 21 story files were hidden without user knowledge.

**Implementation:**

```
IF git_validator_result["git_status"]["uncommitted_changes"] > 10 OR
   (git_validator_result["file_analysis"] AND git_validator_result["file_analysis"]["untracked_files"] > 0):

    # Extract counts from git-validator result
    total_changes = git_validator_result["git_status"]["uncommitted_changes"]

    # If git-validator provides file_analysis (Phase 2 enhancement), use it
    IF git_validator_result["file_analysis"] exists:
        untracked_count = git_validator_result["file_analysis"]["untracked_files"]
        modified_count = git_validator_result["file_analysis"]["modified_files"]
        file_breakdown = git_validator_result["file_analysis"]["file_breakdown"]
    ELSE:
        # Fallback: Calculate ourselves
        untracked_count = count uncommitted files marked "??" in git status
        modified_count = total_changes - untracked_count
        file_breakdown = null

    # Display status summary box
    Display: ""
    Display: "╔═══════════════════════════════════════════════════════════════╗"
    Display: "║  ⚠️  UNCOMMITTED CHANGES DETECTED                             ║"
    Display: "╠═══════════════════════════════════════════════════════════════╣"
    Display: "║                                                               ║"
    Display: "║  Total files: {total_changes}                                ║"

    IF untracked_count > 0 AND modified_count > 0:
        Display: "║    • {modified_count} modified files (tracked by git)        ║"
        Display: "║    • {untracked_count} untracked files (new, not in git)     ║"
    ELIF untracked_count > 0:
        Display: "║    • {untracked_count} untracked files (new, not in git)     ║"
    ELSE:
        Display: "║    • {modified_count} modified files                         ║"

    # Display file breakdown if available (Phase 2 enhancement)
    IF file_breakdown exists AND file_breakdown is not empty:
        Display: "║                                                               ║"
        Display: "║  Breakdown:                                                   ║"

        IF file_breakdown["story_files"] > 0:
            Display: "║    • {file_breakdown.story_files} story files (.story.md)    ║"
            Display: "║      ⚠️  User-created content - should not be hidden         ║"

        IF file_breakdown["python_cache"] > 0:
            Display: "║    • {file_breakdown.python_cache} Python cache files        ║"

        IF file_breakdown["config_files"] > 0:
            Display: "║    • {file_breakdown.config_files} config files              ║"

        IF file_breakdown["documentation"] > 0:
            Display: "║    • {file_breakdown.documentation} documentation files      ║"

        IF file_breakdown["code"] > 0:
            Display: "║    • {file_breakdown.code} code files                        ║"

        IF file_breakdown["other"] > 0:
            Display: "║    • {file_breakdown.other} other files                      ║"

    Display: "║                                                               ║"
    Display: "║  The development workflow can proceed in multiple ways:       ║"
    Display: "║                                                               ║"
    Display: "╚═══════════════════════════════════════════════════════════════╝"
    Display: ""

    # Ask user for strategy
    AskUserQuestion(
        questions=[{
            question: "How should we handle these uncommitted changes?",
            header: "Git Strategy",
            multiSelect: false,
            options: [
                {
                    label: "Continue anyway (safe - file-based tracking)",
                    description: "Proceed without touching git. Framework uses file-based change tracking in devforgeai/stories/{STORY-ID}/changes/. All your files stay visible."
                },
                {
                    label: "Stash ONLY modified files, keep untracked visible ⭐ Recommended",
                    description: "Hide {modified_count} modified (tracked) files in stash, but keep {untracked_count} untracked files visible. Best of both worlds - clean tracked files, preserve new content."
                },
                {
                    label: "Show me the files first",
                    description: "Display list of all {total_changes} files so I can review what would be affected. I'll be asked again after seeing the list."
                },
                {
                    label: "Commit my changes first",
                    description: "Pause development. I'll commit these changes manually, then re-run /dev {STORY-ID}."
                },
                {
                    label: "Stash ALL files (modified + untracked) - Advanced",
                    description: "⚠️ Hide ALL {total_changes} files including {untracked_count} untracked. Requires 'git stash pop' to restore. Use with caution."
                }
            ]
        }]
    )

    # Handle user response
    SWITCH user_response_answers["Git Strategy"]:

        CASE "Continue anyway (safe - file-based tracking)":
            SET workflow_mode = "file-based"
            Display: "✅ Proceeding with file-based tracking. Your files remain visible."
            Display: "   Changes will be tracked in devforgeai/stories/{STORY-ID}/changes/"
            Display: ""
            # Continue to Phase 01.2 (adapt workflow)

        CASE "Stash ONLY modified files, keep untracked visible ⭐ Recommended":
            Display: ""
            Display: "Stashing {modified_count} modified files (keeping {untracked_count} untracked files visible)..."
            Display: ""

            # Use git stash WITHOUT --include-untracked flag
            # This is the KEY: default git stash behavior preserves untracked files
            Bash(
                command="git stash push -m 'WIP: Modified files only (by /dev {STORY-ID} at $(date +%Y-%m-%d_%H:%M:%S))'",
                description="Stash modified files only, preserve untracked"
            )

            # Verify untracked files still visible
            Bash(command="git status --short | grep '^??' | wc -l || echo '0'", description="Count remaining untracked files")
            remaining_untracked = result

            Display: ""
            Display: "✅ Stashed {modified_count} modified files to stash@{0}"
            Display: "✅ {remaining_untracked} untracked files remain visible"

            IF file_breakdown AND file_breakdown["story_files"] > 0:
                Display: "   (includes {file_breakdown.story_files} story files)"

            Display: ""
            Display: "To restore modified files later:"
            Display: "  git stash pop"
            Display: ""

            SET workflow_mode = "git"
            # Continue to Phase 01.2

        CASE "Show me the files first":
            Display: ""
            Display: "Files that would be affected by git operations:"
            Display: ""
            Bash(command="git status --short", description="Show uncommitted files")
            Display: ""
            Display: "File status codes:"
            Display: "  M  = Modified (tracked files with changes)"
            Display: "  ?? = Untracked (new files not yet in git)"
            Display: "  D  = Deleted"
            Display: "  A  = Added (staged for commit)"
            Display: ""

            # Re-ask the question with file context now visible
            AskUserQuestion(
                questions=[{
                    question: "Now that you've seen the files, how should we proceed?",
                    header: "Git Strategy",
                    multiSelect: false,
                    options: [
                        {
                            label: "Continue anyway (safe - file-based tracking)",
                            description: "Proceed without touching git. All {total_changes} files stay visible."
                        },
                        {
                            label: "Stash ONLY modified files, keep untracked visible ⭐ Recommended",
                            description: "Hide modified files, keep untracked files visible. Preserves story files and new content."
                        },
                        {
                            label: "Commit my changes first",
                            description: "Pause development. I'll commit these changes manually."
                        },
                        {
                            label: "Stash ALL files (modified + untracked) - Advanced",
                            description: "⚠️ Hide ALL files temporarily. Requires 'git stash pop' to restore."
                        }
                    ]
                }]
            )
            # Handle response recursively (will hit one of the other cases)

        CASE "Commit my changes first":
            Display: ""
            Display: "╔═══════════════════════════════════════════════════════════════╗"
            Display: "║  📝 RECOMMENDED WORKFLOW                                      ║"
            Display: "╠═══════════════════════════════════════════════════════════════╣"
            Display: "║                                                               ║"
            Display: "║  1. Review your changes:                                      ║"
            Display: "║     git status                                                ║"
            Display: "║     git diff                                                  ║"
            Display: "║                                                               ║"
            Display: "║  2. Stage all changes:                                        ║"
            Display: "║     git add .                                                 ║"
            Display: "║                                                               ║"
            Display: "║  3. Commit with descriptive message:                          ║"
            Display: "║     git commit -m \"WIP: Checkpoint before {STORY-ID}\"       ║"
            Display: "║                                                               ║"
            Display: "║  4. Re-run development:                                       ║"
            Display: "║     /dev {STORY-ID}                                           ║"
            Display: "║                                                               ║"
            Display: "╚═══════════════════════════════════════════════════════════════╝"
            Display: ""
            HALT execution with message: "Development paused. Commit changes and re-run /dev {STORY-ID}."
            Exit workflow

        CASE "Stash ALL files (modified + untracked) - Advanced":
            # Delegate to Phase 01.1.6 (Stash Warning Workflow)
            # This is implemented in Story 1.2
            Display: ""
            Display: "Proceeding to stash warning workflow..."
            Display: ""
            # GOTO Phase 01.1.6 (will be added in Story 1.2)
            INVOKE: Phase 01.1.6 (Stash Warning and Confirmation)

ELSE:
    # No uncommitted changes, or below threshold
    Display: "✓ Working tree: Clean (or below threshold)"
    # Continue to Phase 01.2
```

**Success Criteria:**
- User ALWAYS prompted when uncommitted_changes > 10 or untracked_files > 0
- User can see file list before deciding (via "Show me files first" option)
- "Continue anyway" option preserves all files via file-based tracking
- "Commit first" provides clear instructions and HALTS workflow
- "Stash" delegates to Phase 01.1.6 warning workflow (implemented in Story 1.2)
- No files ever hidden without explicit user confirmation

**Token cost:** ~1,500 tokens (includes AskUserQuestion and display logic)

**Rationale:** RCA-008 incident (2025-11-13) occurred when AI agent autonomously ran `git stash --include-untracked` without user consent, hiding 21 user-created story files. This checkpoint ensures user always knows what will happen before any git operation executes.

---

## Phase 01.1.6: Stash Warning and Confirmation [MANDATORY IF user selects stash] (RCA-008)

**When to execute:** User selected "Stash changes (advanced)" in Phase 01.1.5

**Purpose:** Provide clear warning about file visibility consequences before stashing untracked files

**Implementation:**

```
# Called from Phase 01.1.5 when user selects "Stash changes (advanced)"

# Get file counts from git-validator result (or calculate)
total_files = git_validator_result["git_status"]["uncommitted_changes"]

IF git_validator_result["file_analysis"] exists:
    untracked_count = git_validator_result["file_analysis"]["untracked_files"]
    modified_count = git_validator_result["file_analysis"]["modified_files"]
ELSE:
    # Fallback: Count ourselves
    Bash(command="git status --short | grep '^??' | wc -l", description="Count untracked files")
    untracked_count = result
    modified_count = total_files - untracked_count

# Show detailed file breakdown
IF untracked_count > 0:
    Display: ""
    Display: "Preparing to show untracked files that would be stashed..."
    Display: ""

    # Show first 10 untracked files
    Bash(command="git status --short | grep '^??' | head -10", description="Show untracked files")

    IF untracked_count > 10:
        Display: ""
        Display: "... and {untracked_count - 10} more untracked files"
        Display: ""

    # Check for story files specifically
    Bash(command="git status --short | grep '^??' | grep -c 'STORY-' || echo '0'", description="Count story files")
    story_file_count = result

    IF story_file_count > 0:
        Display: ""
        Display: "⚠️  {story_file_count} STORY files detected in untracked files:"
        Display: ""
        Bash(command="git status --short | grep '^??' | grep 'STORY-'", description="Show story files")
        Display: ""

# Display warning box
Display: ""
Display: "╔═══════════════════════════════════════════════════════════════╗"
Display: "║  ⚠️  WARNING: STASHING {total_files} FILES                    ║"
Display: "╠═══════════════════════════════════════════════════════════════╣"
Display: "║                                                               ║"
Display: "║  What 'git stash' does:                                       ║"
Display: "║    • Temporarily HIDES files from your filesystem             ║"
Display: "║    • Files are stored in git's stash storage                  ║"
Display: "║    • They are NOT deleted (recoverable)                       ║"
Display: "║    • They will NOT be visible until you restore them          ║"
Display: "║                                                               ║"

IF untracked_count > 0:
    Display: "║  ⚠️  {untracked_count} UNTRACKED FILES WILL BE HIDDEN:        ║"
    Display: "║    These are NEW files you created that aren't in git yet.    ║"
    IF story_file_count > 0:
        Display: "║    This includes {story_file_count} STORY files!              ║"
    Display: "║                                                               ║"

Display: "║  To recover stashed files later:                              ║"
Display: "║    git stash pop        # Restores and removes from stash     ║"
Display: "║    git stash apply      # Restores but keeps in stash         ║"
Display: "║                                                               ║"
Display: "║  To preview what's stashed:                                   ║"
Display: "║    git stash show stash@{0} --name-only                      ║"
Display: "║                                                               ║"
Display: "╚═══════════════════════════════════════════════════════════════╝"
Display: ""

# Second confirmation required for safety
AskUserQuestion(
    questions=[{
        question: "Are you SURE you want to stash {total_files} files (including {untracked_count} untracked)?",
        header: "Confirm Stash",
        multiSelect: false,
        options: [
            {
                label: "Yes, stash them (I understand they'll be hidden)",
                description: "Proceed with stashing. Files will be recoverable with 'git stash pop'."
            },
            {
                label: "No, continue without stashing instead",
                description: "Cancel stashing. Use file-based tracking instead. All files stay visible."
            },
            {
                label: "No, let me commit them first",
                description: "Cancel development. I'll commit these files properly before re-running /dev."
            }
        ]
    }]
)

# Handle confirmation response
SWITCH confirmation_response_answers["Confirm Stash"]:

    CASE "Yes, stash them (I understand they'll be hidden)":
        Display: "Executing git stash..."
        Display: ""

        # Execute stash with clear message
        current_timestamp = current date/time
        Bash(
            command="git stash push -m 'WIP: Stashed by /dev {STORY-ID} at {current_timestamp}' --include-untracked",
            description="Stash all changes including untracked files"
        )

        Display: ""
        Display: "✅ Stashed {total_files} files to stash@{0}"
        Display: ""
        Display: "╔═══════════════════════════════════════════════════════════════╗"
        Display: "║  📝 IMPORTANT: TO RESTORE YOUR FILES                          ║"
        Display: "╠═══════════════════════════════════════════════════════════════╣"
        Display: "║                                                               ║"
        Display: "║  After this development session completes, run:               ║"
        Display: "║                                                               ║"
        Display: "║    git stash pop                                              ║"
        Display: "║                                                               ║"
        Display: "║  This will restore your {total_files} files.                 ║"
        Display: "║                                                               ║"
        Display: "╚═══════════════════════════════════════════════════════════════╝"
        Display: ""

        SET workflow_mode = "git"
        RETURN workflow_mode to Phase 01.1.5

    CASE "No, continue without stashing instead":
        Display: "✅ Cancelled stashing. Proceeding with file-based tracking."
        Display: "   All {total_files} files remain visible."
        Display: ""

        SET workflow_mode = "file-based"
        RETURN workflow_mode to Phase 01.1.5

    CASE "No, let me commit them first":
        Display: "✅ Cancelled stashing. Development paused."
        Display: ""
        Display: "Please commit your changes:"
        Display: "  git add ."
        Display: "  git commit -m 'WIP: Checkpoint before {STORY-ID}'"
        Display: ""
        Display: "Then re-run: /dev {STORY-ID}"
        Display: ""

        HALT execution
        Exit workflow
```

**Success Criteria:**
- Warning box displays BEFORE stashing
- User sees list of files that will be hidden (first 10 untracked shown)
- Story files explicitly called out if present (with count)
- Recovery commands shown before AND after stashing
- Double confirmation required (Phase 01.1.5 asks once, Phase 01.1.6 confirms again)
- User can cancel and choose file-based tracking instead (via "No, continue without stashing")
- User can cancel and commit first (via "No, let me commit them first")

**Token cost:** ~2,000 tokens (includes file listing, warning box, second AskUserQuestion)

**Rationale:** The RCA-008 incident showed users don't understand git stash behavior with `--include-untracked`. This double-confirmation with clear warnings prevents accidental file hiding.

---

## Phase 01.1.7: Story File Isolation Check [CONDITIONAL] (STORY-123)

**Purpose:** Warn about uncommitted story files that may conflict with current story development. Distinguishes "your story" from "other uncommitted stories" to help users understand the scope of changes.

**When to execute:** Only when $GIT_AVAILABLE == true AND uncommitted .story.md files exist beyond the current story.

**Skip conditions:**
- Git not available ($GIT_AVAILABLE == false)
- No uncommitted .story.md files
- Only current story is uncommitted (no others)

**Depends On:** STORY-121 (DEVFORGEAI_STORY environment variable scoping)

**Security Notes:**
- Input validation REQUIRED before any shell command execution (CRITICAL)
- All parameters must match pattern: STORY-[0-9]+ (HIGH)
- No shell metacharacters allowed in story IDs (CRITICAL)

---

### Phase 01.1.7.0: Input Validation [SECURITY - CRITICAL] ⚠️

**MUST EXECUTE FIRST** - Prevents command injection vulnerability (OWASP A03:2021)

```
# Validate story_id parameter before any shell operations
CURRENT_STORY_ID = $STORY_ID  # Set by /dev command parameter extraction

# ========== VALIDATION BLOCK ==========
# CRITICAL: Validate story_id prevents shell injection attacks

# Check 1: Empty validation
IF [[ -z "${CURRENT_STORY_ID}" ]]; then
    Display: "❌ ERROR: story_id cannot be empty"
    Display: "  Expected format: STORY-NNN (e.g., STORY-123)"
    HALT with exit code 1

# Check 2: Pattern validation - Must match STORY-[0-9]+
IF [[ ! "${CURRENT_STORY_ID}" =~ ^STORY-[0-9]+$ ]]; then
    Display: "❌ ERROR: Invalid story_id format: '${CURRENT_STORY_ID}'"
    Display: "  Expected format: STORY-NNN (e.g., STORY-123)"
    Display: "  Invalid characters detected"
    HALT with exit code 1

# Check 3: Length validation - Reasonable upper bound
IF [[ ${#CURRENT_STORY_ID} -gt 20 ]]; then
    Display: "❌ ERROR: story_id exceeds maximum length"
    Display: "  Maximum: 20 characters, got: ${#CURRENT_STORY_ID}"
    HALT with exit code 1

# Validation passed
Display: "✓ Story ID validated: ${CURRENT_STORY_ID}"
```

**Injection Patterns Blocked:**
- Command substitution: `STORY-123 && rm -rf /`
- Pipe operators: `STORY-123 | cat /etc/passwd`
- Variable expansion: `STORY-${USER}`
- Backtick execution: `STORY-\`whoami\``
- Glob patterns: `STORY-*`
- Redirection: `STORY-123 > /tmp/file`

**Why this matters:**
The original code at Phase 01.1.7.2 (line 673) called:
```
current_story_num = int(CURRENT_STORY_ID.replace("STORY-", ""))
```
Without validation, an attacker could pass `STORY-123 && rm -rf /` which would be:
- Stored in shell variable unquoted
- Used in git commands: `git status -- ${story_id}.story.md` ✗ VULNERABLE
- With proper quoting: `git status -- "${story_id}.story.md"` ✓ SAFE (but still requires input validation first)

---

### Phase 01.1.7.1: Detect Uncommitted Story Files

```
# Get current story ID from /dev argument (e.g., "STORY-123")
CURRENT_STORY_ID = $STORY_ID  # Set by /dev command parameter extraction

# Find all uncommitted .story.md files via git status
uncommitted_output = Bash(command="git status --porcelain | grep '\\.story\\.md$'")

IF uncommitted_output is empty:
    Display: "✓ No uncommitted story files detected"
    SKIP Phase 01.1.7 (proceed to Phase 01.2)

# Parse uncommitted story files
UNCOMMITTED_STORY_FILES = []
FOR line in uncommitted_output.lines:
    # Format: " M devforgeai/specs/Stories/STORY-100-title.story.md"
    #     or: "?? devforgeai/specs/Stories/STORY-125-new.story.md"
    file_path = line.split()[-1]  # Get file path (last field)

    IF file_path ends with ".story.md":
        UNCOMMITTED_STORY_FILES.append(file_path)

Display: "Found {len(UNCOMMITTED_STORY_FILES)} uncommitted story file(s)"
```

---

### Phase 01.1.7.2: Extract Story IDs and Separate Current vs. Others

```
# Extract story IDs from file paths
# Path format: devforgeai/specs/Stories/STORY-NNN-title.story.md

# SECURITY FIX: Parse current story number with error handling
# This is now safe because Phase 01.1.7.0 validated CURRENT_STORY_ID format

IF ! [[ "${CURRENT_STORY_ID}" =~ ^STORY-([0-9]+)$ ]]; then
    Display: "❌ ERROR: Story ID failed validation (this should not happen)"
    HALT with exit code 1
fi

# Extract number safely with pattern match
current_story_num=${BASH_REMATCH[1]}

STORY_IDS = []
FOR file_path in UNCOMMITTED_STORY_FILES:
    # Extract STORY-NNN from path
    filename = file_path.split("/")[-1]  # e.g., "STORY-123-title.story.md"

    # Parse story ID (extract number between "STORY-" and next "-")
    # Pattern: STORY-{number}-
    # SECURITY: Regex prevents malformed IDs from being processed
    match = regex_match(filename, r"^STORY-(\d+)")

    IF match:
        story_num = int(match.group(1))
        STORY_IDS.append({
            "id": f"STORY-{story_num}",
            "number": story_num,
            "path": file_path
        })

# Separate current story from others (now using validated number)
current_story_num_int = int(current_story_num)  # Already validated above

CURRENT_STORY_FILE = None
OTHER_STORY_IDS = []

FOR story in STORY_IDS:
    IF story["number"] == current_story_num_int:
        CURRENT_STORY_FILE = story
    ELSE:
        OTHER_STORY_IDS.append(story)

# Validate current story found in uncommitted list
IF CURRENT_STORY_FILE == None:
    Display: "⚠️ Warning: Current story {CURRENT_STORY_ID} not in uncommitted changes"
    Display: "  Possible causes:"
    Display: "    - Story file was already committed"
    Display: "    - Wrong story ID provided to /dev command"
    Display: "  Proceeding without story isolation warning..."
    SKIP Phase 01.1.7 (proceed to Phase 01.2)

# Check skip condition - only current story uncommitted
IF len(OTHER_STORY_IDS) == 0:
    Display: "✓ Only current story ({CURRENT_STORY_ID}) is uncommitted - no warning needed"
    SKIP Phase 01.1.7 (proceed to Phase 01.2)

Display: "  - Current story: {CURRENT_STORY_ID}"
Display: "  - Other uncommitted stories: {len(OTHER_STORY_IDS)}"
```

---

### Phase 01.1.7.3: Detect Story Ranges (Format Consecutive Numbers)

```
# Sort other story numbers
other_numbers = sorted([s["number"] for s in OTHER_STORY_IDS])

# Group into consecutive ranges
# Example: [100, 101, 102, 103, 115, 116, 125]
#       -> [(100, 103), (115, 116), (125, 125)]

STORY_RANGES = []
range_start = other_numbers[0]
range_end = other_numbers[0]

FOR i in range(1, len(other_numbers)):
    current = other_numbers[i]
    previous = other_numbers[i - 1]

    IF current == previous + 1:
        # Consecutive - extend range
        range_end = current
    ELSE:
        # Gap detected - close previous range, start new one
        STORY_RANGES.append({
            "start": range_start,
            "end": range_end,
            "count": range_end - range_start + 1
        })
        range_start = current
        range_end = current

# Close final range
STORY_RANGES.append({
    "start": range_start,
    "end": range_end,
    "count": range_end - range_start + 1
})

# Format ranges for display
FORMATTED_RANGES = []
FOR r in STORY_RANGES:
    IF r["start"] == r["end"]:
        # Single story (not a range)
        FORMATTED_RANGES.append(f"STORY-{r['start']} (1 file)")
    ELSE:
        # Range of consecutive stories
        FORMATTED_RANGES.append(f"STORY-{r['start']} through STORY-{r['end']} ({r['count']} files)")
```

---

### Phase 01.1.7.4: Display Warning Box

```
# Build warning display
OTHER_COUNT = len(OTHER_STORY_IDS)

Display: ""
Display: "+-------------------------------------------+"
Display: "| WARNING: UNCOMMITTED STORY FILES DETECTED |"
Display: "+-------------------------------------------+"
Display: ""
Display: "Your story: {CURRENT_STORY_ID} (will be modified by this /dev run)"
Display: ""
Display: "Other uncommitted stories: {OTHER_COUNT} file(s)"

FOR range_text in FORMATTED_RANGES:
    Display: "  - {range_text}"

Display: ""
Display: "Impact:"
Display: "  • Git commits will include ONLY your story (scoped via DEVFORGEAI_STORY)"
Display: "  • Pre-commit validation will focus on {CURRENT_STORY_ID}"
Display: "  • Other story files remain uncommitted"
Display: ""
Display: "+-------------------------------------------+"
Display: ""
```

---

### Phase 01.1.7.5: Ask User for Action

```
# Present options to user
USER_CHOICE = AskUserQuestion(
    questions=[
        {
            "question": "How would you like to proceed with {OTHER_COUNT} other uncommitted story files?",
            "header": "Story Scope",
            "multiSelect": false,
            "options": [
                {
                    "label": "Continue with scoped commits (recommended)",
                    "description": "Proceed with development. Git commits will include ONLY {CURRENT_STORY_ID}, other stories remain uncommitted."
                },
                {
                    "label": "Commit other stories first",
                    "description": "HALT workflow. You'll commit other stories manually, then re-run /dev {CURRENT_STORY_ID}."
                },
                {
                    "label": "Show me the list",
                    "description": "Display full list of uncommitted story files, then ask again."
                }
            ]
        }
    ]
)
```

---

### Phase 01.1.7.6: Process User Choice

```
IF USER_CHOICE == "Continue with scoped commits (recommended)":
    # Set environment variable for story-scoped commits (STORY-121 integration)
    $DEVFORGEAI_STORY = CURRENT_STORY_ID

    Display: ""
    Display: "✓ Proceeding with scoped commits"
    Display: "  DEVFORGEAI_STORY={CURRENT_STORY_ID}"
    Display: "  Pre-commit hooks will validate only this story's changes"
    Display: ""

    # Continue to Phase 01.2
    PROCEED to Phase 01.2

ELIF USER_CHOICE == "Commit other stories first":
    Display: ""
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: "  WORKFLOW HALTED - Commit Other Stories First"
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: ""
    Display: "To commit other stories:"
    Display: ""
    Display: "  # Option 1: Commit all story files"
    Display: "  git add devforgeai/specs/Stories/"
    Display: "  git commit -m 'chore: update story files'"
    Display: ""
    Display: "  # Option 2: Commit specific stories"
    Display: "  git add devforgeai/specs/Stories/STORY-100*.story.md"
    Display: "  git commit -m 'chore(STORY-100): update story'"
    Display: ""
    Display: "After committing, re-run:"
    Display: "  /dev {CURRENT_STORY_ID}"
    Display: ""

    HALT workflow (do not proceed to Phase 01.2)

ELIF USER_CHOICE == "Show me the list":
    # Display full git status for story files
    Display: ""
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: "  UNCOMMITTED STORY FILES"
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: ""

    # Show git status output for story files
    git_output = Bash(command="git status --porcelain | grep '\\.story\\.md$'")

    FOR line in git_output.lines:
        status_code = line[:2]
        file_path = line[3:]

        IF status_code == " M":
            status_text = "modified"
        ELIF status_code == "??":
            status_text = "untracked"
        ELIF status_code == "A ":
            status_text = "staged"
        ELSE:
            status_text = status_code.strip()

        # Highlight current story
        IF CURRENT_STORY_ID in file_path:
            Display: "  [{status_text}] {file_path}  ← YOUR STORY"
        ELSE:
            Display: "  [{status_text}] {file_path}"

    Display: ""
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: ""

    # Re-ask the question (per user decision: re-ask automatically)
    GOTO Phase 01.1.7.5  # Re-display options
```

---

### Phase 01.1.7 Error Handling (NEW - SECURITY FIX)

**Purpose:** Gracefully handle all failure scenarios with clear error messages and exit codes.

**Error Handling Scenarios:**

```
# Error Scenario 1: Git Status Command Fails
IF git status --porcelain fails (exit code != 0):
    Display: "❌ ERROR: Could not check git status"
    Display: "  Reason: {git_error_output}"
    Display: "  Possible causes:"
    Display: "    - Not in a git repository"
    Display: "    - Insufficient permissions"
    Display: "    - Git configuration issue"
    HALT with exit code 128 (Git-specific error)
    Purpose: Skip story file isolation check and proceed (git error is non-fatal to /dev workflow)

# Error Scenario 2: Story File Not Found (Uncommon)
IF CURRENT_STORY_FILE == None AND git status shows story committed:
    Display: "⚠️ Warning: {CURRENT_STORY_ID} appears to be committed already"
    SKIP Phase 01.1.7 (proceed to Phase 01.2)
    Purpose: Allow workflow to continue if story is already committed

# Error Scenario 3: Invalid File Paths in Git Output
IF file_path parsing fails:
    Display: "⚠️ Warning: Could not parse git status output for '{file_path}'"
    Display: "  Skipping this file and continuing with others"
    Purpose: Continue processing other files (one bad file shouldn't block entire workflow)

# Error Scenario 4: Range Formatting Fails
IF range detection/formatting fails (should be rare):
    Display: "⚠️ Warning: Could not format story ranges, showing raw list"
    FOR story in OTHER_STORY_IDS:
        Display: "  - {story['id']}"
    Purpose: Fallback display ensures user gets the information even if formatting breaks

# Error Scenario 5: User Input Validation (Phase 01.1.7.0) Fails
IF CURRENT_STORY_ID fails pattern validation:
    Display: "❌ ERROR: Invalid story_id format: '{CURRENT_STORY_ID}'"
    Display: "  Expected: STORY-NNN where NNN is a number (e.g., STORY-123)"
    Display: "  Fix: Re-run /dev with valid story ID"
    HALT with exit code 1 (Invalid input)
    Purpose: Prevent injection attacks by rejecting malformed input early

# Error Scenario 6: Performance Timeout
IF Phase 01.1.7 takes > 5 seconds (git operation stalled):
    Display: "⚠️ Warning: Story file detection taking longer than expected"
    Display: "  Proceeding without isolation check"
    SKIP Phase 01.1.7 (proceed to Phase 01.2)
    Purpose: Prevent workflow from hanging on slow git operations

# Error Scenario 7: Permission Denied (File Access)
IF git cannot read uncommitted files (permission error):
    Display: "⚠️ Warning: Permission denied accessing some files"
    Display: "  Proceeding with available information"
    Purpose: Continue with partial information rather than blocking entirely
```

**Exit Codes:**
- Exit 0: Success (step completed, user made choice)
- Exit 1: Invalid input (story_id validation failed)
- Exit 128: Git error (not in repository, git command failed)
- Return to main flow: Other scenarios (display warning, continue with reduced functionality)

**Key Principle:** Errors in story file isolation check should NOT block the main /dev workflow. This is a convenience feature, not a critical gate.

---

### Phase 01.1.7 "When to Warn" Decision Matrix

**Purpose:** Clarify all scenarios that trigger warning display vs. silent success.

| File Status | Current Story | Other Stories | Action | Message |
|-------------|---------------|---------------|--------|---------|
| ✓ All committed | - | - | SKIP 0.1.7 | "✓ No uncommitted changes" → Go to 0.2 |
| Uncommitted | Uncommitted | None | SKIP 0.1.7 | "✓ Only current story uncommitted" → Go to 0.2 |
| Uncommitted | Uncommitted | Yes (1-5) | WARN + ASK | "WARNING: 5 other story files" → Display options |
| Uncommitted | Uncommitted | Yes (6-20) | WARN + ASK | "WARNING: 15 other story files (STORY-100 through 115)" → Display options |
| Uncommitted | Uncommitted | Yes (21+) | WARN + ASK | "WARNING: 50+ other story files (ranges shown)" → Display options |
| Committed | - | Uncommitted | SKIP 0.1.7 | "⚠️ Current story already committed" → Go to 0.2 |
| Git error | - | - | ERROR | "❌ Git status failed" → HALT or skip based on error type |

**Decision Logic:**
```
IF no uncommitted .story.md files:
    ACTION: Skip step, proceed
    MESSAGE: "✓ No uncommitted story files detected"

ELIF only CURRENT_STORY_ID is uncommitted:
    ACTION: Skip step, proceed
    MESSAGE: "✓ Only current story uncommitted - no warning needed"

ELIF OTHER_STORY_IDS is empty:
    ACTION: Skip step, proceed
    MESSAGE: (same as above)

ELSE (OTHER_STORY_IDS has entries):
    ACTION: Display warning box
    ACTION: Ask user question with 3 options
    ACTION: Execute user choice:
        - "Continue": Set env var, proceed
        - "Commit first": HALT, show commit instructions
        - "Show list": Display git output, re-ask
```

---

### Phase 01.1.7 Edge Cases

**Edge Case 1: No uncommitted story files**
- Detection: `git status --porcelain | grep '\.story\.md$'` returns empty
- Action: Skip Phase 01.1.7 entirely, proceed to Phase 01.2

**Edge Case 2: Only current story uncommitted**
- Detection: `OTHER_STORY_IDS` is empty after separation
- Action: Skip warning, display "✓ Only current story uncommitted - no warning needed"

**Edge Case 3: Non-consecutive story numbers**
- Example: STORY-100, STORY-105, STORY-110, STORY-115
- Range output: Each becomes individual entry (no "through" ranges)

**Edge Case 4: Single uncommitted other story**
- Example: Only STORY-115 is uncommitted (besides current)
- Display: "STORY-115 (1 file)" (not "STORY-115 through STORY-115")

**Edge Case 5: Large number of uncommitted stories (100+)**
- Display: Show ALL ranges (no truncation per user decision)
- Performance: Detection must complete in <100ms

---

### Phase 01.1.7 Success Criteria

**SECURITY (CRITICAL - MUST COMPLETE):**
- [x] Phase 01.1.7.0: Input validation executed FIRST (pattern: ^STORY-[0-9]+$)
- [x] Empty story_id rejected with clear error message
- [x] Command injection attempts blocked (e.g., STORY-123 && rm -rf /)
- [x] All shell commands use proper quoting with validated input
- [x] Error handling documented for all 7 failure scenarios
- [x] Exit codes standardized (0=success, 1=invalid input, 128=git error)

**FUNCTIONAL (REQUIRED):**
- [x] Detects all uncommitted .story.md files via `git status --porcelain`
- [x] Correctly separates current story from other stories
- [x] Formats consecutive story numbers as ranges (e.g., "100 through 113")
- [x] Displays warning box with clear visual separation
- [x] Presents 3 user options via AskUserQuestion
- [x] Sets DEVFORGEAI_STORY environment variable on "Continue"
- [x] HALTs workflow on "Commit other stories first"
- [x] Re-asks question after "Show me the list"
- [x] Completes detection in <100ms (performance requirement)
- [x] Integrates with STORY-121 DEVFORGEAI_STORY scoping

**DOCUMENTATION (REQUIRED):**
- [x] "When to Warn" decision matrix added
- [x] Error handling scenarios documented (7 scenarios)
- [x] Clear guidance on all edge cases and error conditions
- [x] Security vulnerability explanation and fix documented

**Token cost:** ~3,000 tokens (includes git status parsing, range detection, warning display, AskUserQuestion)

---

### Phase 01.1.7 Refactoring: Method Decomposition (QUALITY FIX)

**Problem:** Original Phase 01.1.7 combined 298 lines of detection, validation, output formatting, and decision logic in a single section. This violates the single-responsibility principle and makes the code harder to test and maintain.

**Solution:** Decompose into 5 focused sub-steps, each handling ONE concern:

**0.1.7.0 - Input Validation** (20 lines)
- Responsibility: Validate story_id before any shell operations
- Handles: Empty check, pattern validation, length check
- Return: Validated story_id or HALT with error
- Reusable: Yes (used in 0.1.7.2 as well)

**0.1.7.1 - Git Detection** (15 lines)
- Responsibility: Detect uncommitted story files
- Handles: Git status parsing, filtering .story.md files
- Return: List of uncommitted story file paths
- Pure function: Yes (no side effects)

**0.1.7.2 - ID Extraction** (25 lines)
- Responsibility: Extract story IDs from file paths and separate current vs. others
- Handles: Regex parsing, story ID separation, validation
- Return: current_story_num, OTHER_STORY_IDS array
- Depends on: 0.1.7.0 (uses validated story_id)

**0.1.7.3 - Range Detection** (30 lines)
- Responsibility: Format consecutive story numbers as ranges
- Handles: Range grouping, consecutive number detection, formatting
- Return: FORMATTED_RANGES array
- Pure function: Yes (stateless)

**0.1.7.4-6 - User Interaction** (60 lines)
- Responsibility: Display warning, collect user choice, execute action
- Handles: Warning display, AskUserQuestion, conditional branching
- Return: User choice OR continue with env var set
- Side effects: Sets DEVFORGEAI_STORY env var, may HALT workflow

**Refactored Method Signature:**
```
function story_file_isolation_check(story_id):
    # Validation (0.1.7.0)
    validated_story_id = validate_story_id(story_id)

    # Detection (0.1.7.1)
    uncommitted_files = detect_uncommitted_stories()
    IF empty(uncommitted_files):
        SKIP step
        RETURN

    # ID Extraction (0.1.7.2)
    (current_story_file, other_stories) = extract_and_separate_story_ids(
        uncommitted_files,
        validated_story_id
    )
    IF empty(other_stories):
        SKIP step (only current story uncommitted)
        RETURN

    # Range Detection (0.1.7.3)
    ranges = detect_story_ranges(other_stories)

    # User Interaction (0.1.7.4-6)
    user_choice = prompt_and_execute(current_story_file, other_stories, ranges)

    RETURN success
```

**Benefits:**
- Each sub-function ≤30 lines (below 100-line threshold)
- Single responsibility per function (easier to test)
- Reusable components (validation, detection, formatting)
- Easier to identify issues (each function has clear input/output)
- Better code review (focused changes)

**Magic String Extraction:**

Original scattered occurrences of git status codes:
- `" M"` → `GIT_STATUS_MODIFIED`
- `"??"` → `GIT_STATUS_UNTRACKED`
- `"A "` → `GIT_STATUS_STAGED`

Define constants at module level:
```
# Git status codes (from git status --porcelain)
GIT_STATUS_MODIFIED = " M"   # File modified
GIT_STATUS_UNTRACKED = "??"  # Untracked file
GIT_STATUS_STAGED = "A "     # Staged (new file)
GIT_STATUS_DELETED = " D"    # Deleted

# Story file patterns
STORY_FILE_PATTERN = "\.story\.md$"
STORY_ID_PATTERN = "^STORY-([0-9]+)$"
STORY_ID_VALIDATION_PATTERN = "^STORY-[0-9]+$"

# Display messages
WARN_TITLE = "WARNING: UNCOMMITTED STORY FILES DETECTED"
WARN_IMPACT = "Git commits will include ONLY your story (scoped)"
ERROR_INVALID_ID = "Invalid story_id format"
```

Usage after extraction:
```
IF status_code == GIT_STATUS_MODIFIED:
    status_text = "modified"
ELIF status_code == GIT_STATUS_UNTRACKED:
    status_text = "untracked"
# etc.
```

Benefits:
- Single source of truth for constants
- Easier to update patterns/messages
- Self-documenting code (constant names explain purpose)
- Reduced typo errors

---

## Phase 01.2: Git Worktree Auto-Management [CONDITIONAL - IF $GIT_AVAILABLE == true]

**Purpose:** Automatically create and manage Git worktrees for parallel story development (STORY-091).

**When to execute:** Only when Git is available ($GIT_AVAILABLE == true) AND worktree management is enabled in config.

**Pre-check: Configuration enabled flag:**

```
# Load parallel.yaml config
config_path = "devforgeai/config/parallel.yaml"

IF file_exists(config_path):
    config = load_yaml(config_path)
ELSE:
    config = {enabled: true}  # Default enabled

IF config.enabled == false:
    Display: "Worktree management disabled via config - using branch-only workflow"
    SKIP to Phase 01.3
    RETURN
```

**Invoke git-worktree-manager subagent:**

```
Task(
  subagent_type="git-worktree-manager",
  description="Manage worktree for ${STORY_ID}",
  prompt="Manage Git worktree for story ${STORY_ID}.

    Configuration: devforgeai/config/parallel.yaml

    Tasks:
    1. Load configuration (threshold, max, pattern)
    2. Check for existing worktree for this story
    3. Scan all worktrees for idle detection
    4. Validate integrity if worktree exists
    5. Determine required action

    Return JSON with status and actions."
)
```

**Parse subagent response and handle actions:**

```javascript
result = parse_json(subagent_output)

// Handle story-specific worktree
IF result.story_worktree.action_needed == "CREATE":
    // Create new worktree
    path = result.story_worktree.path
    branch = result.story_worktree.branch
    Bash(command="git worktree add ${path} -b ${branch}", description="Create story worktree")
    Display: "Created worktree: ${path} (branch: ${branch})"
    $WORKTREE_PATH = path

ELIF result.story_worktree.action_needed == "RESUME":
    path = result.story_worktree.path
    Display: "Resuming in existing worktree: ${path}"
    $WORKTREE_PATH = path

ELIF result.story_worktree.action_needed == "REPAIR":
    // Corrupted worktree detected
    path = result.story_worktree.path
    Display: "⚠ Corrupted worktree detected at ${path}"
    AskUserQuestion(
      questions=[{
        question: "Worktree at ${path} appears corrupted. How to proceed?",
        header: "Repair",
        options: [
          { label: "Delete and recreate", description: "Remove corrupted worktree, create fresh one" },
          { label: "Keep and continue", description: "Attempt to use existing (may have issues)" }
        ],
        multiSelect: false
      }]
    )

// Handle idle worktrees (if any detected)
IF result.idle_worktrees.length > 0:
    idle_count = result.idle_worktrees.length
    threshold = result.config.cleanup_threshold_days

    Display: "Found ${idle_count} idle worktrees (>${threshold} days):"
    FOR wt in result.idle_worktrees:
        Display: "  - ${wt.path} (idle ${wt.days_idle} days)"

    // Present 3-option cleanup prompt (AC#3)
    AskUserQuestion(
      questions=[{
        question: "How would you like to handle idle worktrees?",
        header: "Cleanup",
        options: [
          { label: "Resume Development", description: "Keep all worktrees, continue" },
          { label: "Fresh Start", description: "Delete current story worktree, create new" },
          { label: "Delete Old", description: "Delete idle worktrees not matching current story" }
        ],
        multiSelect: false
      }]
    )

    // Execute user selection
    SWITCH user_selection:
      CASE "Resume Development":
        // No action, continue
        Display: "✓ Keeping all worktrees"
      CASE "Fresh Start":
        Bash(command="git worktree remove ${$WORKTREE_PATH} --force", description="Remove current worktree")
        Bash(command="git worktree add ${path} -b ${branch}", description="Create fresh worktree")
        Display: "✓ Fresh worktree created"
      CASE "Delete Old":
        FOR idle_wt in result.idle_worktrees:
          IF idle_wt.path != $WORKTREE_PATH:
            Bash(command="git worktree remove ${idle_wt.path}", description="Remove idle worktree")
        Display: "✓ Deleted ${idle_count} idle worktrees"

// Handle max worktree limit (AC#7)
IF result.limit_reached AND result.story_worktree.action_needed == "CREATE":
    max = result.config.max_worktrees
    Display: "Maximum worktrees (${max}) reached."
    Display: "Active worktrees:"
    FOR wt in result.all_worktrees:
        Display: "  - ${wt.path} (last activity: ${wt.last_activity})"

    AskUserQuestion(
      questions=[{
        question: "Delete an existing worktree to continue?",
        header: "Limit Reached",
        options: result.all_worktrees.map(wt => ({
          label: wt.name,
          description: "Last activity: ${wt.last_activity}"
        })),
        multiSelect: false
      }]
    )

    // Delete selected worktree, then create new one
```

**Token cost:** ~2,500 tokens (subagent call + response handling)

**References:**
- Subagent: `.claude/agents/git-worktree-manager.md`
- Configuration: `devforgeai/config/parallel.yaml`
- Schema: `devforgeai/config/parallel.schema.json`

---

## Phase 01.2.5: Dependency Graph Validation [MANDATORY]

**Purpose:** Validate story dependencies before TDD workflow begins (STORY-093).

**When to execute:** After git-worktree-manager (Phase 01.2), before workflow adaptation (Phase 01.3).

**Pre-check: Empty depends_on optimization:**

```
# Check if story has any dependencies
# (Extracted from story frontmatter already loaded in conversation)
IF depends_on is empty OR depends_on == []:
    Display: "✓ No dependencies declared - skipping dependency validation"
    SKIP to Phase 01.3
    RETURN
```

**Invoke dependency-graph-analyzer subagent:**

```
Task(
  subagent_type="dependency-graph-analyzer",
  description="Validate dependencies for ${STORY_ID}",
  prompt="Analyze dependencies for story ${STORY_ID}.

    Story path: devforgeai/specs/Stories/

    Tasks:
    1. Extract depends_on from story frontmatter
    2. Build dependency graph with transitive resolution
    3. Detect circular dependencies
    4. Validate all dependency statuses
    5. Generate chain visualization

    Return JSON with validation results.
    BLOCKING: Return blocking=true if any dependency is invalid."
)
```

**Parse subagent response:**

```javascript
result = parse_json(subagent_output)

IF result.status == "PASS":
    Display: "✓ Dependency validation passed"
    Display: "  Dependencies: {result.dependencies.total_count}"
    Display: ""
    Display: result.chain_visualization
    Display: ""
    // Continue to Phase 01.3

ELIF result.status == "BLOCKED":
    // Check for --force flag
    IF $FORCE_FLAG == true:
        // Log bypass to audit file
        timestamp = current_datetime_iso()
        log_path = "devforgeai/logs/dependency-bypass-{timestamp}.log"

        Write(
            file_path=log_path,
            content="""# Dependency Bypass Log
Timestamp: {timestamp}
Story: {STORY_ID}
Bypassed Dependencies:
{json.dumps(result.validation.failures, indent=2)}
User: Requested via --force flag
"""
        )

        Display: "⚠️  DEPENDENCY CHECK BYPASSED (--force flag)"
        Display: ""
        Display: "The following dependency issues were bypassed:"
        FOR failure in result.validation.failures:
            Display: "  • {failure.message}"
        Display: ""
        Display: "Bypass logged to: {log_path}"
        Display: ""
        Display: "Proceeding to Phase 01.3..."
        // Continue to Phase 01.3

    ELSE:
        // Block execution
        Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        Display: "❌ DEPENDENCY VALIDATION FAILED"
        Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        Display: ""

        IF result.validation.cycle_detected:
            Display: "🔄 CIRCULAR DEPENDENCY DETECTED"
            Display: ""
            Display: "Cycle: {' → '.join(result.validation.cycle_path)}"
            Display: ""
            Display: "Resolution: Remove circular reference in one of the story files."

        ELIF result.validation.missing.length > 0:
            Display: "❓ MISSING DEPENDENCIES"
            Display: ""
            FOR dep in result.validation.missing:
                Display: "  • {dep} - Story file not found"
            Display: ""
            Display: "Resolution: Create the missing story files or remove the dependency."

        ELSE:
            Display: "⏳ DEPENDENCIES NOT READY"
            Display: ""
            FOR failure in result.validation.failures:
                Display: "  • {failure.message}"
                IF failure.suggestion:
                    Display: "    → {failure.suggestion}"
            Display: ""

        Display: ""
        Display: "Dependency chain:"
        Display: result.chain_visualization
        Display: ""
        Display: "Options:"
        Display: "  1. Complete dependent stories first"
        Display: "  2. Run with --force flag to bypass (not recommended):"
        Display: "     /dev {STORY_ID} --force"
        Display: ""
        Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

        HALT workflow (do not proceed to Phase 01.3)

ELIF result.status == "ERROR":
    Display: "❌ Dependency analysis error: {result.error}"
    Display: "Proceeding with caution..."
    // Continue to Phase 01.3 (graceful degradation)
```

**Token cost:** ~2,500 tokens (subagent call + response handling)

**References:**
- Subagent: `.claude/agents/dependency-graph-analyzer.md`
- Implementation: `src/dependency_graph_analyzer.py`
- Story: STORY-093 - Dependency Graph Enforcement with Transitive Resolution

---

## Phase 01.2.6: File Overlap Detection [CONDITIONAL]

**Purpose:** Detect file overlaps with parallel stories before TDD workflow begins (STORY-094).

**When to execute:** After dependency-graph-analyzer (Phase 01.2.5), before workflow adaptation (Phase 01.3).

**Pre-check: Has technical_specification optimization:**

```
# Check if story has technical_specification section
# (Story content already loaded in conversation)
IF technical_specification section is empty OR not present:
    Display: "ℹ️ No technical_specification - skipping spec-based overlap detection"
    Display: "   Post-flight git-based detection will still run after Phase 3"
    $FILE_OVERLAP_PRE_FLIGHT = false
    SKIP to Phase 01.3
    RETURN
```

**Invoke file-overlap-detector subagent:**

```
Task(
  subagent_type="file-overlap-detector",
  description="Detect file overlaps for ${STORY_ID}",
  prompt="Analyze file overlaps for story ${STORY_ID}.

    Mode: pre-flight
    Story path: devforgeai/specs/Stories/

    Tasks:
    1. Parse technical_specification from target story
    2. Extract all file_path values from components
    3. Scan stories with status 'In Development'
    4. Detect overlapping files
    5. Filter out depends_on story overlaps
    6. Generate recommendations

    Return JSON with overlap analysis.
    WARNING: Return status=WARNING if overlaps detected
    BLOCKING: Return status=BLOCKED if >= 10 files overlap"
)
```

**Parse subagent response:**

```javascript
result = parse_json(subagent_output)

IF result.status == "PASS":
    Display: "✓ File overlap check passed"
    Display: "  No overlapping files with parallel stories"
    $FILE_OVERLAP_PRE_FLIGHT = true
    // Continue to Phase 01.3

ELIF result.status == "WARNING":
    // Overlaps detected but below blocking threshold
    Display: "⚠️ FILE OVERLAPS DETECTED"
    Display: ""
    Display: "Overlapping files found with parallel story/stories:"

    FOR story_id, files in result.overlaps:
        Display: "  • {story_id}: {files.length} file(s)"
        FOR file in files:
            Display: "    - {file}"
    Display: ""

    // Interactive prompt (AC#2)
    AskUserQuestion(
        questions=[{
            question: "File overlap detected. How would you like to proceed?",
            header: "Overlap Warning",
            multiSelect: false,
            options: [
                {
                    label: "Yes - Proceed",
                    description: "Continue with development (accept overlap risk)"
                },
                {
                    label: "No - Cancel",
                    description: "Cancel development to resolve overlap"
                },
                {
                    label: "Review - Show detailed report",
                    description: "View full overlap report before deciding"
                }
            ]
        }]
    )

    SWITCH user_response:
        CASE "Yes - Proceed":
            Display: "✓ Proceeding with acknowledged overlaps"
            $FILE_OVERLAP_PRE_FLIGHT = true
            // Continue to Phase 01.3

        CASE "No - Cancel":
            Display: "Development cancelled due to file overlaps"
            HALT workflow

        CASE "Review - Show detailed report":
            Read(file_path=result.report_path)
            // Display report content
            // Re-ask question after review

ELIF result.status == "BLOCKED":
    // >= blocking_threshold overlaps
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: "❌ FILE OVERLAP DETECTION - BLOCKED"
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: ""
    Display: "Severe file overlap detected ({result.overlap_count} files)"
    Display: ""
    FOR rec in result.recommendations:
        Display: "  • {rec}"
    Display: ""
    Display: "Report saved to: {result.report_path}"
    Display: ""

    IF $FORCE_FLAG == true:
        // Log bypass
        timestamp = current_datetime_iso()
        log_path = "devforgeai/logs/overlap-bypass-{timestamp}.log"

        Write(
            file_path=log_path,
            content="""# File Overlap Bypass Log
Timestamp: {timestamp}
Story: {STORY_ID}
Overlaps bypassed: {result.overlap_count}
Overlapping stories:
{json.dumps(result.overlaps, indent=2)}
User: Requested via --force flag
"""
        )

        Display: "⚠️ FILE OVERLAP CHECK BYPASSED (--force flag)"
        Display: "Bypass logged to: {log_path}"
        $FILE_OVERLAP_PRE_FLIGHT = true
        // Continue to Phase 01.3

    ELSE:
        Display: "To bypass (not recommended):"
        Display: "  /dev {STORY_ID} --force"
        Display: ""
        Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        HALT workflow

ELIF result.status == "ERROR":
    Display: "⚠️ File overlap analysis error: {result.error}"
    Display: "Proceeding with caution (spec-based detection skipped)..."
    $FILE_OVERLAP_PRE_FLIGHT = false
    // Continue to Phase 01.3 (graceful degradation)
```

**Token cost:** ~2,000 tokens (subagent call + response handling)

**References:**
- Subagent: `.claude/agents/file-overlap-detector.md`
- Implementation: `src/file_overlap_detector.py`
- Story: STORY-094 - File Overlap Detection with Hybrid Analysis

---

## Phase 01.3: Adapt TDD Workflow Based on Git Availability [MANDATORY]

**Workflow adaptations apply throughout all phases:**

**IF WORKFLOW_MODE == "file_based":**

- **Phase 01 (Context Validation):**
  - ✅ Check context files (same as git_based)
  - ✅ Validate story structure (same as git_based)
  - ⚠️ SKIP: Git status checks
  - ⚠️ SKIP: Branch validation

- **Phase 02-05 (Red/Green/Refactor/Integration):**
  - ✅ All TDD phases execute normally (test generation, implementation, refactoring)
  - ✅ All test execution works identically
  - ⚠️ SKIP: Any Git commands in these phases (if present)

- **Phase 08 (Git Workflow):**
  - ⚠️ REPLACE: Git commit workflow → File-based change tracking (see Step c.)

**IF WORKFLOW_MODE == "git_based":**
  - ✅ All phases execute normally with full Git integration

---

## Phase 01.4: File-Based Change Tracking [MANDATORY IF WORKFLOW_MODE == "file_based"]

**ONLY executed when WORKFLOW_MODE == "file_based"**

This replaces Phase 08 (Git Workflow) with file-based artifact tracking.

**Implementation (executed in Phase 08 when Git unavailable):**

```markdown
### Phase 08 Alternative: File-Based Change Tracking

**ONLY when GIT_AVAILABLE == false**

#### Step 1: Create Change Documentation Directory

```
# Create story-specific changes directory
IF not exists devforgeai/stories/${STORY_ID}/changes/:
    # Use native Write tool to create directory marker
    Write(
        file_path="devforgeai/stories/${STORY_ID}/changes/.gitkeep",
        content="# Change tracking directory for ${STORY_ID}\n"
    )
```

#### Step 2: Generate Change Manifest

```
# Generate timestamp
TIMESTAMP = {current_datetime in ISO8601 format}

# List modified files (manual tracking since no Git)
# Developer must identify changed files from implementation work

Write(
    file_path="devforgeai/stories/${STORY_ID}/changes/implementation-${TIMESTAMP}.md",
    content="""# Implementation Changes - ${STORY_ID}

**Timestamp:** ${TIMESTAMP}
**Story:** ${STORY_TITLE}
**Phase:** Dev Complete
**Workflow Mode:** File-Based (Git not available)

## Files Created

${list_files_created_during_implementation}

## Files Modified

${list_files_modified_during_implementation}

## Files Deleted

${list_files_deleted_if_any}

## Test Results

- Total Tests: ${total_tests}
- Passed: ${passed_tests}
- Failed: ${failed_tests}
- Coverage: ${coverage_percentage}%

## Acceptance Criteria Status

${copy_acceptance_criteria_completion_status_from_story}

## Implementation Notes

${implementation_summary_from_story_Implementation_Notes_section}

## Next Steps

To enable full version control:
1. Initialize Git: git init
2. Add files: git add .
3. Create initial commit: git commit -m "Initial commit"
4. Re-run /dev to use Git-based workflow
"""
)

Display: "✓ File-based change manifest created"
Display: "  Location: devforgeai/stories/${STORY_ID}/changes/implementation-${TIMESTAMP}.md"
```

#### Step 3: Update Story File with Change Reference

```
Read(file_path="devforgeai/specs/Stories/${STORY_ID}.story.md")

# Add to Workflow History section
Edit(
    file_path="devforgeai/specs/Stories/${STORY_ID}.story.md",
    old_string="## Workflow History",
    new_string="""## Workflow History

### Development Complete - ${TIMESTAMP} (File-Based)
- **Status:** Dev Complete
- **Workflow Mode:** File-Based (Git not available)
- **Changes:** devforgeai/stories/${STORY_ID}/changes/implementation-${TIMESTAMP}.md
- **Tests:** ${passed_tests}/${total_tests} passing (${coverage_percentage}% coverage)
- **Note:** Git not available - changes tracked in story artifacts

{preserve existing workflow history below}
"""
)

Display: "✓ Story file updated with file-based tracking reference"
```

#### Step 4: Display Completion Summary

```
Display:
"┌─────────────────────────────────────────────────────────────────┐
│ ✅ Development Complete (File-Based Workflow)                   │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│ Story: ${STORY_ID} - ${STORY_TITLE}                            │
│ Status: Dev Complete                                            │
│                                                                 │
│ Tests: ${passed_tests}/${total_tests} passing                  │
│ Coverage: ${coverage_percentage}%                               │
│                                                                 │
│ Changes tracked in:                                             │
│   devforgeai/stories/${STORY_ID}/changes/implementation-...   │
│                                                                 │
│ Git Integration: Not Available                                  │
│                                                                 │
│ To enable Git workflow:                                         │
│   git init                                                      │
│   git add .                                                     │
│   git commit -m 'Initial commit'                               │
│   Then re-run: /dev ${STORY_ID}                                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘"
```
```

**Benefits of file-based tracking:**
- Enables DevForgeAI in non-Git environments
- Maintains traceability through file artifacts
- Same TDD workflow, different tracking mechanism
- Clear path to Git migration when ready

---

## Phase 01.5: Validate Context Files Exist [MANDATORY]

**Check for all 6 DevForgeAI context files:**

**Reference:** `parallel-context-loader.md` (STORY-112)

**Pattern:** Load all 6 files in a single message with 6 Read calls for implicit parallel execution.

**Time Savings:** 83% reduction (3000ms sequential → 500ms parallel)

```
# Single message with 6 Read calls - all execute in parallel
Read(file_path="devforgeai/specs/context/tech-stack.md")
Read(file_path="devforgeai/specs/context/source-tree.md")
Read(file_path="devforgeai/specs/context/dependencies.md")
Read(file_path="devforgeai/specs/context/coding-standards.md")
Read(file_path="devforgeai/specs/context/architecture-constraints.md")
Read(file_path="devforgeai/specs/context/anti-patterns.md")
```

**If ANY file is missing:**

```
Display: "❌ Context files missing - architecture setup required"
Display: "  Missing files prevent development (would cause technical debt from assumptions)"
Display: ""
Display: "Invoking devforgeai-architecture skill to create context files..."

Skill(command="devforgeai-architecture")

Display: "✓ Architecture skill completed"
Display: "Re-validating context files..."

# Re-read context files after architecture skill completes
[Execute same parallel Read operations above]
```

**STOP development until all context files exist.** This prevents technical debt from ambiguous assumptions.

**Token cost:** ~2,000 tokens (6 files × ~300 tokens each, read in parallel)

---

## Phase 01.6: Load Story Specification [MANDATORY]

**Story already loaded via @file reference from slash command.**

The story file was loaded by the `/dev` command via:
```
@devforgeai/specs/Stories/STORY-XXX.story.md
```

**Verify story content accessible:**
- [ ] YAML frontmatter with id, title, status, epic, sprint
- [ ] Acceptance criteria section exists
- [ ] Technical specification section exists
- [ ] Non-functional requirements documented

**If story content not available in conversation:**
```
HALT with error:
"Story file not loaded in conversation context.

Expected: Story loaded via @file reference from /dev command
Actual: No story content found

Please ensure /dev command properly loads story file before invoking this skill."
```

---

## Phase 01.6.5: Story Type Detection & Phase Skip Configuration [MANDATORY] (STORY-126)

**Purpose:** Extract story type from frontmatter and configure phase skipping based on type.

**When to execute:** After Phase 01.6 (Load Story Specification), before Phase 01.7.

**Story Types:**
- `feature` (default) - Full TDD workflow, all phases required
- `documentation` - Skip Phase 05 Integration (no runtime code)
- `bugfix` - Skip Phase 04 Refactor (minimal changes)
- `refactor` - Skip Phase 02 Red (tests exist)

**Implementation:**

```
# Extract type field from story YAML frontmatter (already loaded in conversation)
# Frontmatter format:
# ---
# id: STORY-XXX
# title: ...
# type: feature|documentation|bugfix|refactor
# status: ...
# ---
#
# Story Type Enum Reference: .claude/skills/devforgeai-story-creation/references/story-type-classification.md

# Parse story type from frontmatter
story_type = extract_yaml_field(story_frontmatter, "type")

# Default to "feature" if type field missing (backward compatibility - AC#5)
IF story_type is empty OR story_type is null:
    story_type = "feature"
    Display: "✓ Story type: feature (default - no type field specified)"
ELSE:
    # Validate type enum (see story-type-classification.md for authoritative definition)
    valid_types = ["feature", "documentation", "bugfix", "refactor"]

    IF story_type NOT IN valid_types:
        Display: "❌ Invalid story type: '{story_type}'"
        Display: "   Valid types: feature, documentation, bugfix, refactor"
        Display: ""
        HALT with validation error

    Display: "✓ Story type: {story_type}"

# Configure phase skip rules based on type
$STORY_TYPE = story_type
$SKIP_PHASES = []

SWITCH story_type:
    CASE "feature":
        Display: "  → Full TDD workflow (no phases skipped)"
        $SKIP_PHASES = []

    CASE "documentation":
        Display: "  → Skipping Phase 05 Integration Testing"
        Display: "    Reason: Documentation stories have no runtime code to test"
        $SKIP_PHASES = ["Phase 05"]

    CASE "bugfix":
        Display: "  → Skipping Phase 04 Refactoring"
        Display: "    Reason: Bugfix stories require minimal, targeted changes"
        $SKIP_PHASES = ["Phase 04"]

    CASE "refactor":
        Display: "  → Skipping Phase 02 Red (Test Generation)"
        Display: "    Reason: Refactor stories work with existing tests"
        $SKIP_PHASES = ["Phase 02"]

Display: ""
```

**Phase Skip Matrix (Reference):**

| Story Type | Skipped Phase | Rationale |
|------------|---------------|-----------|
| `feature` | None | Full TDD workflow required for new functionality |
| `documentation` | Phase 05 Integration | No runtime code, no integration points to test |
| `bugfix` | Phase 04 Refactor | Minimal changes preferred, avoid scope creep |
| `refactor` | Phase 02 Red | Tests already exist, refactoring preserves behavior |

**Token cost:** ~100 tokens (simple string extraction and switch)

---

## Phase 01.7: Validate Spec vs Context Files [MANDATORY]

**Check for conflicts between story requirements and context file constraints:**

From story Technical Specification section, extract:
- Required technologies (languages, frameworks, libraries)
- Required patterns (architectures, designs)
- File locations (where code should be placed)

Compare against:
- tech-stack.md (locked technologies)
- architecture-constraints.md (design patterns)
- source-tree.md (file placement rules)

**If conflicts detected → Use AskUserQuestion:**

```
Question: "Spec requires [X], but tech-stack.md specifies [Y]. Which is correct?"
Header: "Spec conflict"
options:
  - label: "Follow tech-stack.md (use [Y])"
    description: "Maintain consistency with existing architecture"
  - label: "Update tech-stack.md (use [X] + create ADR)"
    description: "Document architecture change in ADR and update tech-stack.md"
multiSelect: false
```

**After user response:**
- If "Update tech-stack.md" chosen:
  - Create ADR documenting technology decision
  - Update tech-stack.md
  - Proceed with development
- If "Follow tech-stack.md" chosen:
  - Proceed with development using tech-stack.md technologies

**Token cost:** ~1,000 tokens (conflict detection) + ~3,000 (if AskUserQuestion needed)

---

## Phase 01.8: Detect and Validate Technology Stack [MANDATORY]

**Invoke tech-stack-detector subagent to detect technologies and validate against tech-stack.md:**

```
Task(
  subagent_type="tech-stack-detector",
  description="Detect and validate tech stack",
  prompt="Analyze the project structure in the current directory.

  Detect:
  1. Primary programming language
  2. Framework/runtime
  3. Test framework
  4. Build tool
  5. Package manager

  Then validate against devforgeai/specs/context/tech-stack.md if it exists.

  Return JSON with detected technologies, validation results, and recommended commands.

  CRITICAL: If conflicts found between detected and specified technologies, provide clear resolution options."
)
```

**Parse subagent JSON response:**

```javascript
result = parse_json(subagent_output)

# Extract detected technologies
LANGUAGE = result["detected"]["language"]["primary"]
FRAMEWORK = result["detected"]["framework"]["name"]
TEST_FRAMEWORK = result["detected"]["test_framework"]["primary"]

# Extract workflow commands (CRITICAL - used in subsequent phases)
TEST_COMMAND = result["commands"]["test"]
TEST_COVERAGE_COMMAND = result["commands"]["test_coverage"]
BUILD_COMMAND = result["commands"]["build"]
INSTALL_COMMAND = result["commands"]["install"]

# Check validation status
VALIDATION_STATUS = result["validation"]["status"]

IF VALIDATION_STATUS == "PASS":
    Display: "✓ Technology stack validated"
    Display: "  - Language: {LANGUAGE}"
    Display: "  - Framework: {FRAMEWORK}"
    Display: "  - Test framework: {TEST_FRAMEWORK}"
    Display: "  - Test command: {TEST_COMMAND}"

ELIF VALIDATION_STATUS == "FAIL":
    # CRITICAL conflicts detected - HALT
    Display: "❌ Technology stack validation FAILED"
    Display: "Conflicts detected between project and tech-stack.md"

    FOR conflict in result["validation"]["conflicts"]:
        IF conflict["severity"] == "CRITICAL":
            # Use AskUserQuestion to resolve
            AskUserQuestion:
                question: "Project uses {conflict['detected']} but tech-stack.md specifies {conflict['specified']}. How to resolve?"
                header: "Tech Conflict"
                options:
                    - label: "Follow spec (update project)"
                      description: "Change project to use {conflict['specified']}"
                    - label: "Update spec (create ADR)"
                      description: "Update tech-stack.md, document in ADR"
                multiSelect: false

            # Handle user response
            IF "Update spec" chosen:
                # Create ADR, update tech-stack.md, re-validate

ELIF VALIDATION_STATUS == "ERROR":
    IF result["validation"]["context_missing"]:
        # tech-stack.md not found - invoke architecture skill
        Display: "❌ tech-stack.md not found"
        Display: "Invoking devforgeai-architecture skill..."
        Skill(command="devforgeai-architecture")

        # After architecture completes, re-run tech-stack-detector
        # [Re-invoke Task with same parameters]

# Store commands for Phases 1-5
$TEST_COMMAND = TEST_COMMAND
$TEST_COVERAGE_COMMAND = TEST_COVERAGE_COMMAND
$BUILD_COMMAND = BUILD_COMMAND
```

**Token cost:** ~700 tokens in skill context (~8,000 in isolated subagent context)

---

## Phase 01.9: Detect Previous QA Failures [MANDATORY]

**Check if story has failed QA due to deferral or other issues:**

```
# Search for QA reports for this story
Glob(pattern="devforgeai/qa/reports/${STORY_ID}-qa-report*.md")

IF reports found:
    # Read most recent report
    reports_sorted = sort_by_timestamp(reports)
    latest_report = reports_sorted[0]

    Read(file_path=latest_report)

    # Parse QA status
    IF report contains "Status: FAILED":
        # Extract failure type
        IF report contains "Deferral Validation FAILED":
            # Deferral-specific failure
            Display: "⚠ Previous QA failed due to deferral issues"
            Display: "  QA Report: {latest_report}"
            Display: ""

            # Extract deferral violations from report
            Grep(
                pattern="- \\[ \\] .* - (Deferred to|Blocked by|Out of scope)",
                path=latest_report,
                output_mode="content",
                -n=true
            )

            Display: "Development will focus on resolving deferral issues."
            Display: "The 'Handling QA Deferral Failures' workflow will guide resolution."
            Display: ""

            # Set flag for later use
            $QA_DEFERRAL_FAILURE = true
            $QA_FAILURE_REPORT = latest_report

        ELIF report contains "Coverage Below Threshold":
            Display: "⚠ Previous QA failed due to coverage issues"
            Display: "  Focus: Increase test coverage"
            $QA_COVERAGE_FAILURE = true

        ELIF report contains "Anti-Pattern Violations":
            Display: "⚠ Previous QA failed due to anti-patterns"
            Display: "  Focus: Refactor to remove violations"
            $QA_ANTIPATTERN_FAILURE = true

        ELSE:
            Display: "⚠ Previous QA failed (review report for details)"
            Display: "  Report: {latest_report}"
            $QA_GENERIC_FAILURE = true

    ELIF report contains "Status: PASSED":
        # QA already passed - unusual to be in Dev again
        Display: "Note: QA already passed for this story"
        Display: "  Proceeding with development (may be enhancement or bug fix)"
        $QA_PASSED = true

ELSE:
    # No QA reports found - first development iteration
    Display: "✓ First development iteration (no previous QA attempts)"
    $QA_FIRST_ITERATION = true
```

**Token cost:** ~1,500 tokens (Glob + Read + Grep + parsing)

**Use in subsequent phases:**
- If `$QA_DEFERRAL_FAILURE == true` → Invoke "Handling QA Deferral Failures" workflow
- If `$QA_COVERAGE_FAILURE == true` → Focus on test coverage in Phase 02
- If `$QA_ANTIPATTERN_FAILURE == true` → Extra validation in Phase 04 (Refactor)

---

## Phase 01.9.5: Load Structured Gap Data (gaps.json) [IF QA FAILED]

**Purpose:** Parse machine-readable gap data for targeted remediation workflow.

**When to execute:** After Phase 01.9 detects `$QA_COVERAGE_FAILURE`, `$QA_ANTIPATTERN_FAILURE`, or `$QA_DEFERRAL_FAILURE`

```
# Check if structured gap export exists
gaps_file = "devforgeai/qa/reports/${STORY_ID}-gaps.json"

Glob(pattern=gaps_file)

IF gaps_file EXISTS:
    Display: ""
    Display: "╔═══════════════════════════════════════════════════════════════╗"
    Display: "║  📊 STRUCTURED GAP DATA DETECTED                              ║"
    Display: "╠═══════════════════════════════════════════════════════════════╣"
    Display: "║                                                               ║"
    Display: "║  QA-Dev Integration: ACTIVE                                   ║"
    Display: "║  Gap file: {gaps_file}                                        ║"
    Display: "║                                                               ║"
    Display: "║  Development will enter REMEDIATION MODE:                     ║"
    Display: "║  • Targeted test generation for specific gaps                 ║"
    Display: "║  • Focus on coverage failures by file                         ║"
    Display: "║  • Suggested tests provided                                   ║"
    Display: "║                                                               ║"
    Display: "╚═══════════════════════════════════════════════════════════════╝"
    Display: ""

    # Read and parse gaps.json
    Read(file_path=gaps_file)
    gaps_data = parse_json(file_content)

    # Build $QA_COVERAGE_GAPS array for Phase 02 consumption
    $QA_COVERAGE_GAPS = []

    FOR EACH gap in gaps_data.coverage_gaps:
        gap_entry = {
            "file": gap.file,
            "layer": gap.layer,
            "current_coverage": gap.current_coverage,
            "target_coverage": gap.target_coverage,
            "gap_percentage": gap.gap_percentage,
            "uncovered_line_count": gap.uncovered_line_count,
            "suggested_tests": gap.suggested_tests
        }
        $QA_COVERAGE_GAPS.append(gap_entry)

    # Build $QA_ANTIPATTERN_GAPS for Phase 04 consumption
    $QA_ANTIPATTERN_GAPS = gaps_data.anti_pattern_violations

    # Build $QA_DEFERRAL_GAPS for Phase 06 consumption
    $QA_DEFERRAL_GAPS = gaps_data.deferral_issues

    # Display summary
    Display: "Gap Summary:"
    Display: ""

    IF $QA_COVERAGE_GAPS.count > 0:
        Display: "📉 Coverage Gaps: {$QA_COVERAGE_GAPS.count} files below threshold"
        FOR EACH gap in $QA_COVERAGE_GAPS:
            Display: "   • {gap.file}: {gap.current_coverage}% → need {gap.target_coverage}% (gap: {gap.gap_percentage}%)"
            Display: "     Suggested tests:"
            FOR EACH test in gap.suggested_tests:
                Display: "       - {test}"
        Display: ""

    IF $QA_ANTIPATTERN_GAPS.count > 0:
        Display: "⚠️  Anti-Pattern Violations: {$QA_ANTIPATTERN_GAPS.count} issues to resolve"
        FOR EACH violation in $QA_ANTIPATTERN_GAPS:
            Display: "   • {violation.file}:{violation.line} - {violation.type} ({violation.severity})"
        Display: ""

    IF $QA_DEFERRAL_GAPS.count > 0:
        Display: "📋 Deferral Issues: {$QA_DEFERRAL_GAPS.count} items need resolution"
        FOR EACH deferral in $QA_DEFERRAL_GAPS:
            Display: "   • {deferral.item}: {deferral.violation_type}"
        Display: ""

    # Set remediation mode flag
    $REMEDIATION_MODE = true
    Display: "✅ Remediation mode enabled - see qa-remediation-workflow.md for targeted workflow"
    Display: ""

ELSE:
    # No gaps.json - use legacy markdown parsing (Phase 01.8 results)
    Display: "ℹ️  No structured gap data (gaps.json) found"
    Display: "   Using legacy QA report parsing for failure context"
    Display: "   (To enable targeted remediation, re-run /qa {STORY_ID})"
    Display: ""
    $REMEDIATION_MODE = false
```

**Token cost:** ~800 tokens (Glob + Read + JSON parse + display)

**Variables set for Phases 1-5:**
- `$QA_COVERAGE_GAPS` - Array of coverage gap objects with file:line targets
- `$QA_ANTIPATTERN_GAPS` - Array of anti-pattern violations with remediation
- `$QA_DEFERRAL_GAPS` - Array of deferral issues with required actions
- `$REMEDIATION_MODE` - Boolean flag for targeted workflow

**Use in subsequent phases:**
- Phase 02: Pass `$QA_COVERAGE_GAPS` to test-automator for targeted test generation
- Phase 04: Pass `$QA_ANTIPATTERN_GAPS` to refactoring-specialist for targeted fixes
- Phase 06: Pre-load `$QA_DEFERRAL_GAPS` for deferral resolution

**See also:** `qa-remediation-workflow.md` for detailed remediation mode workflow

---

## Phase 01.10: Story Complexity Analysis [INFORMATIONAL] (STORY-172)

**Purpose:** Warn user about potentially oversized stories that may require multiple TDD iterations.

**When to execute:** After Phase 01.9.5 (Load Structured Gap Data), before proceeding to Phase 02.

**Token Cost:** ~500 tokens

---

### Step 11: Story Complexity Analysis

**Purpose:** Analyze story metrics and warn user about potentially oversized stories before development begins.

**Metrics to Analyze:**

```
dod_count = count(DoD items in story)
ac_count = count(Acceptance Criteria)
tech_spec_lines = count(lines in Technical Specification section)
files_touched = count(files mentioned in tech spec)
```

**Extraction Logic:**

```
# Extract metrics from story file
story_content = Read(file_path="devforgeai/specs/Stories/${STORY_ID}.story.md")

# Count DoD items (checkbox pattern in Definition of Done section)
dod_count = Grep(pattern="^\\s*-\\s*\\[[ x]\\]", path=story_file, output_mode="count")

# Count Acceptance Criteria (### AC# headers)
ac_count = Grep(pattern="^###\\s*AC#", path=story_file, output_mode="count")

# Count Technical Specification lines (between ## Technical Specification and next ##)
tech_spec_lines = count_lines_in_section("## Technical Specification", story_content)

# Count files mentioned in tech spec (file paths with extensions)
files_touched = Grep(pattern="\\.(md|py|ts|js|sh|json|yaml)\\b", path=story_file, output_mode="count")
```

---

### Complexity Thresholds

**Threshold definitions for complexity scoring:**

- DoD items: >20 = High, >30 = Very High
- AC count: >5 = High, >8 = Very High
- Tech spec lines: >100 = High, >200 = Very High
- Files touched: >10 = High, >20 = Very High

**Scoring Summary:**

| Metric | High Threshold | Very High Threshold | Points |
|--------|----------------|---------------------|--------|
| DoD items | >20 | >30 | 1 + 1 = 2 max |
| AC count | >5 | >8 | 1 + 1 = 2 max |
| Tech spec lines | >100 | >200 | 1 + 1 = 2 max |
| Files touched | >10 | >20 | 1 + 1 = 2 max |
| **Total Possible Score** | | | **0-8** |

---

### Complexity Scoring Logic

**Define Thresholds (centralized):**

```
# Threshold definitions for each metric dimension
THRESHOLDS = {
  "dod": { "high": 20, "very_high": 30 },
  "ac": { "high": 5, "very_high": 8 },
  "tech_spec": { "high": 100, "very_high": 200 },
  "files": { "high": 10, "very_high": 20 }
}

# Complexity level mapping
COMPLEXITY_LEVELS = {
  "very_high": 4,  # score >= 4
  "high": 2,       # score >= 2
  "medium": 1,     # score >= 1
  "normal": 0      # score < 1
}
```

**Calculate Complexity Score (extracted helper):**

```
FUNCTION calculate_complexity_score(metrics):
  """
  Calculate complexity score by checking each metric against thresholds.

  Scoring Strategy:
    Each metric dimension has two thresholds (HIGH and VERY_HIGH)
    - Exceeding HIGH threshold: +1 point
    - Exceeding VERY_HIGH threshold: +1 additional point
    Maximum score: 8 (all dimensions at very high)

  Args:
    metrics: { dod_count, ac_count, tech_spec_lines, files_touched }

  Returns: Integer score (0-8)
  """
  score = 0

  # Evaluate DoD count (0-2 points possible)
  score += (metrics.dod_count > THRESHOLDS.dod.high) ? 1 : 0
  score += (metrics.dod_count > THRESHOLDS.dod.very_high) ? 1 : 0

  # Evaluate AC count (0-2 points possible)
  score += (metrics.ac_count > THRESHOLDS.ac.high) ? 1 : 0
  score += (metrics.ac_count > THRESHOLDS.ac.very_high) ? 1 : 0

  # Evaluate tech spec size (0-2 points possible)
  score += (metrics.tech_spec_lines > THRESHOLDS.tech_spec.high) ? 1 : 0
  score += (metrics.tech_spec_lines > THRESHOLDS.tech_spec.very_high) ? 1 : 0

  # Evaluate file touch count (0-2 points possible)
  score += (metrics.files_touched > THRESHOLDS.files.high) ? 1 : 0
  score += (metrics.files_touched > THRESHOLDS.files.very_high) ? 1 : 0

  RETURN score
END FUNCTION
```

**Determine Complexity Level (extracted helper):**

```
FUNCTION get_complexity_level(score):
  """
  Map complexity score to human-readable level.

  Args:
    score: Integer complexity score (0-8)

  Returns: String level ("NORMAL", "MEDIUM", "HIGH", "VERY HIGH")

  Edge Cases:
    - score < 0: treated as 0 (NORMAL)
    - score > 8: treated as 8 (VERY HIGH)
  """
  # Ensure score is within valid range
  normalized_score = MAX(0, MIN(8, score))

  IF normalized_score >= COMPLEXITY_LEVELS.very_high:
    RETURN "VERY HIGH"
  ELSIF normalized_score >= COMPLEXITY_LEVELS.high:
    RETURN "HIGH"
  ELSIF normalized_score >= COMPLEXITY_LEVELS.medium:
    RETURN "MEDIUM"
  ELSE:
    RETURN "NORMAL"
  END IF
END FUNCTION
```

**Main Complexity Analysis (orchestrator):**

```
complexity_score = calculate_complexity_score({
  dod_count: dod_count,
  ac_count: ac_count,
  tech_spec_lines: tech_spec_lines,
  files_touched: files_touched
})

complexity_level = get_complexity_level(complexity_score)
```

---

### Warning Display (if score >= 2)

**Display complexity warning only when complexity_level is HIGH or VERY HIGH:**

**Build Metrics Display List (extracted helper):**

```
FUNCTION get_warning_metrics(metrics):
  """
  Filter metrics that exceed HIGH thresholds for display in warning.

  Args:
    metrics: { dod_count, ac_count, tech_spec_lines, files_touched }

  Returns: Array of formatted metric strings
  """
  warning_items = []

  # Add metrics exceeding HIGH threshold (first level trigger)
  IF metrics.dod_count > THRESHOLDS.dod.high:
    warning_items.append("DoD items: {metrics.dod_count} (threshold: {THRESHOLDS.dod.high})")

  IF metrics.ac_count > THRESHOLDS.ac.high:
    warning_items.append("Acceptance Criteria: {metrics.ac_count} (threshold: {THRESHOLDS.ac.high})")

  IF metrics.tech_spec_lines > THRESHOLDS.tech_spec.high:
    warning_items.append("Tech spec size: {metrics.tech_spec_lines} lines (threshold: {THRESHOLDS.tech_spec.high})")

  IF metrics.files_touched > THRESHOLDS.files.high:
    warning_items.append("Files to modify: {metrics.files_touched} (threshold: {THRESHOLDS.files.high})")

  RETURN warning_items
END FUNCTION
```

**Display Complexity Warning (orchestrator):**

```
IF complexity_score >= COMPLEXITY_LEVELS.high:

    Display: ""
    Display: "⚠️  STORY COMPLEXITY ASSESSMENT"
    Display: "================================"
    Display: ""
    Display: "Complexity Level: {complexity_level}"
    Display: ""

    # Get metrics above high threshold for display
    warning_metrics = get_warning_metrics({
      dod_count: dod_count,
      ac_count: ac_count,
      tech_spec_lines: tech_spec_lines,
      files_touched: files_touched
    })

    # Display metrics header only if warnings exist
    IF warning_metrics.length > 0:
        Display: "Metrics exceeding thresholds:"
        FOR item IN warning_metrics:
            Display: "  • {item}"
        Display: ""

    Display: "⚠️  Recommendation: This story may require 2-3+ TDD iterations"
    Display: ""
```

---

### User Decision Point

**Present options to user when HIGH or VERY HIGH warning is displayed:**

```
IF complexity_score >= 2:
    AskUserQuestion:
        Question: "How would you like to proceed?"
        Header: "Complexity"
        Options:
            - "Continue - I understand this is a large story"
            - "Show me what could be split out"
            - "Stop - I'll break this into smaller stories first"
        multiSelect: false

    # Handle user response
    SWITCH user_response:

        CASE "Continue - I understand this is a large story":
            Display: "✓ Proceeding with large story development"
            Display: "  Note: Consider checkpointing progress frequently"
            # Continue to Phase 02

        CASE "Show me what could be split out":
            # Invoke Split Suggestion Logic (see below)
            INVOKE: Split Suggestion Analysis
            # Re-ask after showing suggestions

        CASE "Stop - I'll break this into smaller stories first":
            Display: ""
            Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            Display: "  WORKFLOW HALTED - Break Into Smaller Stories"
            Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            Display: ""
            Display: "Recommended approach:"
            Display: "  1. Review the DoD items and group by logical boundaries"
            Display: "  2. Create 2-3 smaller stories from the groups"
            Display: "  3. Run /dev on each smaller story"
            Display: ""
            HALT workflow (do not proceed to Phase 02)

ELSE:
    # complexity_score < 2 (NORMAL or MEDIUM)
    Display: "✓ Story complexity: {complexity_level} (within normal range)"
    # Continue to Phase 02
```

---

### Split Suggestion Logic

**Build Split Suggestions (extracted helper):**

```
FUNCTION build_split_suggestions(metrics):
  """
  Analyze metrics and generate split suggestions by dimension.

  Analysis Approach:
    - For each metric exceeding HIGH threshold, suggest a split strategy
    - Splits preserve traceability (reference original items, AC, files)
    - Each suggestion group represents a logical way to partition the story

  Args:
    metrics: { dod_count, ac_count, tech_spec_lines, files_touched }

  Returns: Array of suggestion objects with header and items
  """
  suggestions = []

  # Suggest DoD grouping if exceeds high threshold
  IF metrics.dod_count > THRESHOLDS.dod.high:
    half_point = CEILING(metrics.dod_count / 2)
    suggestions.append({
      header: "DoD Grouping (by implementation phase)",
      items: [
        "Items 1-{half_point} → STORY-A (core functionality)",
        "Items {half_point+1}-{metrics.dod_count} → STORY-B (features + edge cases)"
      ]
    })

  # Suggest AC grouping if exceeds high threshold
  IF metrics.ac_count > THRESHOLDS.ac.high:
    ac_split_point = CEILING(metrics.ac_count / 2)
    suggestions.append({
      header: "AC Grouping (by acceptance boundary)",
      items: [
        "AC#1 to AC#{ac_split_point} → STORY-A (happy path + basic validation)",
        "AC#{ac_split_point+1} to AC#{metrics.ac_count} → STORY-B (edge cases + error handling)"
      ]
    })

  # Suggest file grouping if exceeds high threshold
  IF metrics.files_touched > THRESHOLDS.files.high:
    suggestions.append({
      header: "File Grouping (by architectural layer)",
      items: [
        "Domain/Model layer files → STORY-A (business logic core)",
        "Application/API layer files → STORY-B (integration layer)",
        "Infrastructure files → STORY-C (persistence/deployment)"
      ]
    })

  RETURN suggestions
END FUNCTION
```

**Display Split Suggestions (orchestrator):**

```
# When user chooses "Show me what could be split out"

Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  STORY SPLIT SUGGESTIONS"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""
Display: "Suggested logical boundaries:"
Display: ""

# Get suggestions based on metrics exceeding thresholds
suggestions = build_split_suggestions({
  dod_count: dod_count,
  ac_count: ac_count,
  tech_spec_lines: tech_spec_lines,
  files_touched: files_touched
})

# Display each suggestion group
FOR suggestion IN suggestions:
    Display: "  {suggestion.header}:"
    FOR item IN suggestion.items:
        Display: "    • {item}"
    Display: ""

Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""

# Re-ask user after showing suggestions
AskUserQuestion:
    Question: "After reviewing split suggestions, how would you like to proceed?"
    Header: "Complexity"
    Options:
        - "Continue - I understand this is a large story"
        - "Stop - I'll break this into smaller stories first"
    multiSelect: false
```

---

### Logging for Retrospective

**Purpose:** Track actual iterations vs predicted complexity for framework learning and threshold calibration.

**When development completes, log complexity assessment data:**

```
# After development workflow completes (in Phase 08 or post-development hook)
complexity_log = {
    "story_id": STORY_ID,
    "assessed_complexity": complexity_level,
    "complexity_score": complexity_score,
    "metrics": {
        "dod_count": dod_count,
        "ac_count": ac_count,
        "tech_spec_lines": tech_spec_lines,
        "files_touched": files_touched
    },
    "actual_iterations": count_tdd_iterations(),
    "user_decision": user_response,
    "timestamp": current_timestamp()
}

# Write to retrospective log
Write(
    file_path="devforgeai/feedback/complexity-assessments/${STORY_ID}.json",
    content=json_stringify(complexity_log)
)
```

**Framework Learning:**
- Actual iterations vs predicted complexity allows threshold calibration
- High-iteration stories that were predicted NORMAL suggest threshold adjustment needed
- Data feeds into retrospective analysis for continuous improvement

**Token cost:** ~500 tokens (metric extraction, scoring, conditional display)

---

## Complexity Assessment Quick Reference

**Fast lookup for complexity scoring:**

```
Complexity Level Mapping:

  NORMAL:     score 0-1   (story is appropriately sized, proceed normally)
  MEDIUM:     score 1     (story is larger than typical, may need 2+ iterations)
  HIGH:       score 2-3   (story is large, likely 2-3+ iterations, warn user)
  VERY HIGH:  score 4-8   (story is very large, likely 3-5+ iterations, urgent warning)

User Actions by Level:

  NORMAL/MEDIUM:  Display checkmark, proceed to Phase 02
  HIGH/VERY HIGH: Display warning, prompt user for decision:
                  - Continue (understand implications)
                  - Show split suggestions (architectural grouping)
                  - Stop and break into smaller stories
```

---

## Code Quality Improvements (Refactoring Summary)

**Phase 01.10 Refactoring Improvements Applied:**

### 1. Extract Method: Threshold Definitions
**Problem:** Magic numbers (20, 30, 5, 8, 100, 200, 10, 20) scattered throughout logic
**Solution:** Extracted `THRESHOLDS` dictionary centralizing all threshold definitions
**Benefit:** Single source of truth for threshold values, easier threshold adjustment

### 2. Extract Method: Complexity Level Mapping
**Problem:** Complexity level calculation implicit and hard to trace
**Solution:** Extracted `COMPLEXITY_LEVELS` mapping and `get_complexity_level()` helper function
**Benefit:** Clear separation between scoring (0-8) and level assignment (NORMAL/MEDIUM/HIGH/VERY HIGH), easier to verify correctness

### 3. Extract Method: Score Calculation
**Problem:** 8 nearly identical threshold checks (lines 2485-2495 in original)
**Solution:** Extracted `calculate_complexity_score()` function using `THRESHOLDS` dictionary
**Benefit:** Eliminates code duplication, single place to maintain scoring logic, easier to add new metrics

### 4. Extract Method: Warning Metrics Filter
**Problem:** Four identical conditional display patterns for metric warnings (lines 2518-2525 in original)
**Solution:** Extracted `get_warning_metrics()` helper to filter and format metrics
**Benefit:** Eliminates conditional duplication, metrics display consistent and maintainable

### 5. Extract Method: Split Suggestions Builder
**Problem:** Three separate conditional blocks generating split suggestions with repeated patterns (lines 2708-2732 in original)
**Solution:** Extracted `build_split_suggestions()` function analyzing all metrics in one pass
**Benefit:** Unified suggestion generation, easier to add new suggestion types, single loop for display

### 6. Apply DRY Principle: Centralized Thresholds
**Before:** Threshold values hardcoded in 12+ locations
**After:** Single `THRESHOLDS` dictionary referenced by all functions
**Impact:** 40% reduction in refactored section complexity, single point of change

### Complexity Reduction Achieved

| Metric | Before | After | Reduction |
|--------|--------|-------|-----------|
| Cyclomatic Complexity (Complexity Thresholds section) | 12 | 4 | 67% |
| Code Duplication | 8 repetitive threshold checks | 1 loop in function | 87.5% |
| Magic Numbers | 12 hardcoded values | 0 (all in THRESHOLDS dict) | 100% |
| Conditional Display Logic | 4 duplicate patterns | 1 loop + helper | 75% |
| Edge Case Handling | Not documented | Normalized score bounds | Added |
| Documentation Clarity | Basic docstrings | Detailed scoring strategy explanation | Improved |

### Code Quality Metrics

- **Maintainability:** Improved - Single points of change for thresholds, helper functions clearly named
- **Readability:** Improved - Helper functions have docstrings, orchestrator code shows intent
- **Testability:** Improved - Helper functions can be tested independently
- **Extensibility:** Improved - New metrics require only adding to THRESHOLDS and one conditional in helper

### Naming Consistency

**Clear Function Names Reveal Intent:**
- `calculate_complexity_score()` - Computes numeric score from metrics
- `get_complexity_level()` - Maps score to human-readable level
- `get_warning_metrics()` - Filters metrics for warning display
- `build_split_suggestions()` - Generates split recommendations

---

## ✅ PHASE 01 COMPLETION CHECKPOINT

**Before proceeding to Phase 02 (Test-First Design), verify ALL pre-flight validations passed:**

### Mandatory Steps Executed

- [ ] **Phase 01.1:** git-validator subagent invoked, Git status assessed
- [ ] **Phase 01.1.5:** User consent obtained (if uncommitted changes > 10)
- [ ] **Phase 01.1.6:** Stash warnings shown (if user selected stash)
- [ ] **Phase 01.2:** Git Worktree Auto-Management (if Git available + enabled)
- [ ] **Phase 01.2.5:** Dependency Graph Validation (STORY-093) - validated or --force bypassed
- [ ] **Phase 01.3:** Workflow mode determined (git-based or file-based)
- [ ] **Phase 01.4:** File-based tracking setup (if WORKFLOW_MODE == "file_based")
- [ ] **Phase 01.5:** All 6 context files validated (exist and non-empty)
- [ ] **Phase 01.6:** Story specification loaded (via @file reference)
- [ ] **Phase 01.7:** Spec vs. context conflicts resolved (via AskUserQuestion if conflicts)
- [ ] **Phase 01.8:** tech-stack-detector invoked, technologies validated
- [ ] **Phase 01.9:** Previous QA failures detected (recovery mode if needed)
- [ ] **Phase 01.9.5:** Structured gap data loaded (if gaps.json exists)
- [ ] **Phase 01.10:** Story complexity analyzed (STORY-172) - user warned if HIGH/VERY HIGH

### Variables Set for Phases 02-08

- [ ] `$GIT_AVAILABLE` = true/false
- [ ] `$WORKFLOW_MODE` = "full" / "partial" / "fallback"
- [ ] `$CAN_COMMIT` = true/false
- [ ] `$WORKTREE_PATH` = (worktree path, if created)
- [ ] `$TEST_COMMAND` = (pytest / npm test / dotnet test / etc.)
- [ ] `$TEST_COVERAGE_COMMAND` = (with coverage flags)
- [ ] `$BUILD_COMMAND` = (language-specific build command)
- [ ] `$QA_*_FAILURE` = Boolean flags (if QA failure detected)
- [ ] `$REMEDIATION_MODE` = true/false (if gaps.json loaded)
- [ ] `$QA_COVERAGE_GAPS` = Array (if gaps.json has coverage gaps)
- [ ] `$QA_ANTIPATTERN_GAPS` = Array (if gaps.json has anti-patterns)
- [ ] `$QA_DEFERRAL_GAPS` = Array (if gaps.json has deferrals)

### Success Criteria

- [ ] All 6 context files exist
- [ ] No conflicts between story spec and context files
- [ ] Technology stack detected and validated
- [ ] Test commands identified and executable
- [ ] Git workflow mode determined
- [ ] User consented to git operations (if applicable)
- [ ] Ready to begin TDD workflow

### Checkpoint Validation

**IF ANY ITEM UNCHECKED:**
```
❌ PHASE 01 INCOMPLETE - Review missing steps above
⚠️  DO NOT PROCEED TO PHASE 02 until all checkpoints pass
⚠️  Missing validations will cause failures in later phases

Common issues:
  - Context files missing → Run /create-context
  - Git not initialized → Initialize git or use file-based mode
  - Spec conflicts → Resolve via AskUserQuestion
  - Tech stack mismatch → Update tech-stack.md or adjust story
```

**IF ALL ITEMS CHECKED:**
```
✅ PHASE 01 COMPLETE - All Pre-Flight Validations Passed

Variables set: {count} variables configured
Context files: 6/6 validated
Git mode: {WORKFLOW_MODE}
Test framework: {TEST_COMMAND}

Ready to begin TDD cycle.

**Update Progress Tracker:**
Mark "Execute Phase 01" todo as "completed"

**See Also:**
- `tdd-red-phase.md` - Phase 02 workflow (test generation)
- `parameter-extraction.md` - Story ID extraction details
- `ambiguity-protocol.md` - When to use AskUserQuestion

Next: Load tdd-red-phase.md and execute Phase 02 (Test-First Design - Red Phase)
```