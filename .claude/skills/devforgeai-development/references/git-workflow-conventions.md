# Git Workflow Conventions Reference

Complete guide for version control operations during Phase 08 (Git Workflow) of the TDD development process.

---

## Phase Progress Indicator

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 08/9: Git Workflow & Commit (78% → 89% complete)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

**Display this indicator at the start of Phase 08 execution.**

---

## Overview

This reference provides conventions for:
- Branch naming strategies
- Commit message formatting (Conventional Commits)
- Commit timing in TDD workflow
- Staging strategies
- Push timing and best practices
- Git hooks integration

---

## Phase 08.0: Pre-Commit DoD Validation [MANDATORY - RCA-014 REC-3]

**Execute BEFORE any git operations.**

**Purpose:** Final safety net to prevent committing incomplete work without user approval. Defense-in-depth even if Phase 06 has bugs or is skipped.

**Rationale (RCA-014):** Phase 06 should catch all incomplete work, but this checkpoint provides redundant validation to prevent autonomous deferrals from ANY source (bugs, manual story edits, workflow skips).

### Detect Incomplete DoD Items

```
Grep(
  pattern="^- \[ \]",
  path="${STORY_FILE}",
  output_mode="content",
  -B=1
)
```

**Filter to Definition of Done section only:**

```
unchecked_dod_items = []

FOR each match:
  preceding_line = previous line (via -B=1)
  item_text = current line

  # Check if in DoD section (not AC Checklist, Implementation Notes, etc.)
  IF preceding_line contains "## Definition of Done":
    in_dod_section = true

  IF preceding_line contains "## Acceptance Criteria":
    in_dod_section = false

  IF preceding_line contains "## Workflow Status":
    in_dod_section = false

  IF preceding_line contains "## Implementation Notes":
    in_dod_section = false

  IF preceding_line contains "### AC#":
    in_dod_section = false

  IF preceding_line contains "Checklist":
    in_dod_section = false

  # Only collect items in DoD section
  IF in_dod_section AND item_text starts with "- [ ]":
    unchecked_dod_items.append(item_text)
```

### Check for User Approval

```
IF unchecked_dod_items.length > 0:
  # Check for "Approved Deferrals" section in Implementation Notes
  Grep(
    pattern="### Approved Deferrals",
    path="${STORY_FILE}",
    output_mode="files_with_matches"
  )

  IF "Approved Deferrals" section NOT found:
    # BLOCKING ERROR - No approval documented
    Display: ""
    Display: "═══════════════════════════════════════════════════════════"
    Display: "❌ PHASE 5 BLOCKED: Incomplete DoD Without Approval"
    Display: "═══════════════════════════════════════════════════════════"
    Display: ""
    Display: "Found {unchecked_dod_items.length} unchecked DoD items:"
    Display: ""
    FOR each item in unchecked_dod_items:
      Display: "  • {item}"
    Display: ""
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: "DIAGNOSIS"
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: ""
    Display: "This should have been caught in Phase 06 (Deferral Challenge)."
    Display: ""
    Display: "Possible causes:"
    Display: "  1. Phase 06 was skipped (workflow bug)"
    Display: "  2. User approved deferrals but approval not documented (bug)"
    Display: "  3. DoD was manually edited after Phase 06 (user error)"
    Display: "  4. Phase 06 detection has bug (missed these items)"
    Display: ""
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: "RESOLUTION"
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: ""
    Display: "Option 1: Investigate why Phase 06 didn't catch this"
    Display: "  → Run: /rca \"Phase 08 blocked - unchecked DoD items without approval\""
    Display: ""
    Display: "Option 2: Get user approval now (manual override)"
    Display: "  → Add '### Approved Deferrals' section to story Implementation Notes"
    Display: "  → Include: User approval timestamp, blockers, follow-up references"
    Display: "  → Re-run: /dev {STORY_ID} to retry Phase 08"
    Display: ""
    Display: "Option 3: Complete the work (recommended if time permits)"
    Display: "  → Implement missing DoD items"
    Display: "  → Mark items [x] in Definition of Done"
    Display: "  → Re-run: /dev {STORY_ID} to retry Phase 08"
    Display: ""
    Display: "═══════════════════════════════════════════════════════════"
    Display: ""

    HALT workflow
    EXIT Phase 08 with status code 1

  ELSE:
    # Approved Deferrals section exists - user approval documented
    Display: ""
    Display: "✓ Pre-Commit Validation: Unchecked DoD items found BUT user approval documented"
    Display: "  Incomplete items: {unchecked_dod_items.length}"
    Display: "  Approval: 'Approved Deferrals' section exists in Implementation Notes"
    Display: "  Status: PASS (proceeding to git commit)"
    Display: ""

    # Continue to Pre-Requisites below

ELSE:
  # All DoD items checked - perfect completion
  Display: ""
  Display: "✓ Pre-Commit Validation: All DoD items complete (100%)"
  Display: "  Status: PASS (proceeding to git commit)"
  Display: ""

  # Continue to Pre-Requisites below
```

**Success Criteria:**
- [x] Validation runs BEFORE any git operations
- [x] Detects ALL unchecked DoD items
- [x] Allows unchecked items IF "Approved Deferrals" section exists
- [x] BLOCKS commit if unchecked items WITHOUT approval
- [x] Clear error messages with 3 resolution options
- [x] Does NOT block on AC Checklist unchecked items

---

## Pre-Requisites for Phase 08 (Git Workflow)

**CRITICAL:** Before executing git commit workflow, ensure DoD format is correct.

### Phase 06-5 Bridge Workflow

**Required:** Execute DoD Update Workflow BEFORE Phase 08

**Load and execute:**
```
Read(file_path=".claude/skills/devforgeai-development/references/dod-update-workflow.md")
```

**Bridge workflow ensures:**
- [ ] All completed DoD items marked [x] in Definition of Done section
- [ ] All completed DoD items added to Implementation Notes (FLAT LIST - no ### subsections)
- [ ] devforgeai-validate validate-dod passes (exit code 0)
- [ ] Workflow Status section updated

**Why mandatory:** Git commit will FAIL if DoD format incorrect. Pre-commit hook validates that all [x] items in DoD appear in Implementation Notes.

**Common failure:** Placing DoD items under `### Definition of Done Status` subsection (validator's `extract_section()` stops at ### headers, doesn't see items)

**See:** `dod-update-workflow.md` for detailed format requirements and error fixes

---

## Phase 08.0.5: Lock Coordination for Parallel Commits [NEW - STORY-096]

**Purpose:** Serialize git commits across parallel story worktrees to prevent git index lock conflicts.

**Load lock coordination workflow:**
```
Read(file_path=".claude/skills/devforgeai-development/references/lock-file-coordination.md")
```

**Execute Steps 5.0.1 through 5.0.4 from lock-file-coordination.md:**

1. **Step 5.0.1:** Acquire `devforgeai/.locks/git-commit.lock`
2. **Step 5.0.2:** Wait with progress display if lock held (AC#2)
3. **Step 5.0.3:** Auto-remove stale locks (PID dead + age > 5 min) (AC#3)
4. **Step 5.0.4:** Prompt user if timeout exceeds 10 minutes (AC#4)

**Success Criteria:**
- [x] Lock acquired (or timeout handled with user choice)
- [x] Lock file contains PID, story_id, timestamp, hostname
- [x] Stale locks auto-removed if detected

**Failure Modes:**
- **ABORT:** User chose abort at timeout prompt → Clean exit, changes preserved
- **HALT:** Lock acquisition failed unexpectedly → Error message with recovery steps

**Proceed to git add/commit ONLY after lock acquired successfully.**

---

## AC Verification Checklist Updates (Phase 08) [NEW - RCA-011]

**Purpose:** Check off final AC items related to deployment readiness after git commit

**Execution:** After git commit succeeds, before Phase 09 (Feedback Hook)

**Load AC Checklist Update Workflow:**
```
Read(file_path=".claude/skills/devforgeai-development/references/ac-checklist-update-workflow.md")
```

**Identify Phase 08 AC Items:**
```
Grep(pattern="Phase.*: 5", path="${STORY_FILE}", output_mode="content", -B=1)
```

**Common Phase 08 items:**
- [ ] Git commit created with semantic message
- [ ] Story status updated to "Dev Complete"
- [ ] Backward compatibility verified
- [ ] Deployment readiness confirmed
- [ ] Integration notes documented

**Update Procedure:** Batch-update all Phase 08 items after commit succeeds

**Display:** "Phase 08 AC Checklist: ✓ {count} items checked | AC Progress: {X}/{Y} (100%)"

**Final Checklist Summary:**
```
Display:
"
✅ AC Verification Checklist Complete
   Total: {total} items
   Checked: {checked} ({100}%)

   Phase 02 (Test Generation): {count1} items ✓
   Phase 03 (Implementation): {count2} items ✓
   Phase 04 (Refactoring): {count3} items ✓
   Phase 05 (Integration): {count4} items ✓
   Phase 06 (Deferral): {count4.5} items ✓
   Phase 08 (Git Workflow): {count5} items ✓

   All acceptance criteria validated real-time during TDD workflow.
"
```

---

## Git Stash Safety Protocol (RCA-008)

**CRITICAL RULE:** Never stash files without user consent and clear warnings.

**Incident Reference:** RCA-008 (2025-11-13) - Autonomous git stash operation hid 21 user-created story files without consent, causing workflow disruption and user confusion.

### Prohibited Actions

❌ **NEVER do this:**
```bash
git stash push --include-untracked   # Without warning or consent!
```

**Why prohibited:**
- Hides untracked files (user-created content) from filesystem
- Files become invisible until `git stash pop` is executed
- Non-git-experts don't understand this behavior
- High risk of perceived data loss

✅ **ALWAYS do this instead:**
1. Show user what will be stashed (file list with categories)
2. Display warning about file visibility consequences
3. Get explicit confirmation via AskUserQuestion
4. Execute stash only if user confirms
5. Display recovery instructions immediately after stashing

### Stash Warning Template

**When user chooses to stash, display this warning:**

```
╔═══════════════════════════════════════════════════════════════╗
║  ⚠️  WARNING: STASHING {total_files} FILES                    ║
╠═══════════════════════════════════════════════════════════════╣
║                                                               ║
║  What 'git stash' does:                                       ║
║    • Temporarily HIDES files from your filesystem             ║
║    • Files are stored in git's stash storage                  ║
║    • They are NOT deleted (recoverable)                       ║
║    • They will NOT be visible until you restore them          ║
║                                                               ║
║  ⚠️  {untracked_count} UNTRACKED FILES WILL BE HIDDEN:        ║
║    These are NEW files you created that aren't in git yet.    ║
║    This includes {story_file_count} STORY files!              ║
║                                                               ║
║  To recover stashed files later:                              ║
║    git stash pop        # Restores and removes from stash     ║
║    git stash apply      # Restores but keeps in stash         ║
║                                                               ║
║  To preview what's stashed:                                   ║
║    git stash show stash@{0} --name-only                      ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝
```

**Then require double confirmation:**

```
AskUserQuestion(
    questions=[{
        question: "Are you SURE you want to stash {total_files} files (including {untracked_count} untracked)?",
        header: "Confirm Stash",
        options: [
            {
                label: "Yes, stash them (I understand they'll be hidden)",
                description: "Proceed with stashing. Files recoverable with 'git stash pop'."
            },
            {
                label: "No, continue without stashing instead",
                description: "Cancel stashing. Use file-based tracking. All files stay visible."
            },
            {
                label: "No, let me commit them first",
                description: "Cancel development. I'll commit files before re-running."
            }
        ]
    }]
)
```

### Recovery Instructions

**After stashing, ALWAYS display:**

```
✅ Stashed {total_files} files to stash@{0}

╔═══════════════════════════════════════════════════════════════╗
║  📝 IMPORTANT: TO RESTORE YOUR FILES                          ║
╠═══════════════════════════════════════════════════════════════╣
║                                                               ║
║  After this development session completes, run:               ║
║                                                               ║
║    git stash pop                                              ║
║                                                               ║
║  This will restore your {total_files} files.                 ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝
```

### Untracked Files Special Handling

**IF stashing untracked files (--include-untracked flag):**

1. **Count untracked files:**
   ```bash
   git status --short | grep "^??" | wc -l
   ```

2. **Show first 10 untracked files:**
   ```bash
   git status --short | grep "^??" | head -10
   ```

3. **Highlight story files if present:**
   ```bash
   git status --short | grep "^??" | grep "STORY-"
   ```

4. **Require double confirmation:**
   - First: User selects "Stash changes (advanced)" from git strategy question
   - Second: User confirms after seeing warning box and file list

5. **Provide clear recovery path:**
   - Show recovery commands BEFORE stashing
   - Show recovery reminder AFTER stashing
   - Include in post-workflow summary

**Rationale:** RCA-008 incident showed that stashing untracked files without warning caused user confusion when 21 story files disappeared unexpectedly. The `--include-untracked` flag behavior is not intuitive for non-git-experts.

### Safe Stash Commands

**Stash modified files only (DEFAULT - RECOMMENDED):**
```bash
git stash push -m "WIP: Modified files only"
# Does NOT use --include-untracked
# Modified files (M) → stashed
# Untracked files (??) → remain visible
```

**Stash everything (USE WITH CAUTION):**
```bash
git stash push -m "WIP: All files" --include-untracked
# Requires: Steps 0.1.5 and 0.1.6 user consent workflow
# Modified files (M) → stashed
# Untracked files (??) → stashed (HIDDEN from filesystem)
```

**Restore stashed files:**
```bash
git stash pop      # Restore and remove from stash
git stash apply    # Restore but keep in stash (for reuse)
git stash drop     # Delete stash permanently (use with caution)
```

### When to Use Each Stash Strategy

**Use "Stash modified only" (default) when:**
- User has untracked files (story files, new code, documentation)
- Want clean working tree for tracked files only
- Untracked files are user-created content (high value)
- DEFAULT RECOMMENDATION in Phase 01.1.5

**Use "Stash all (--include-untracked)" when:**
- User explicitly requests it after seeing warning
- All files are regenerable (cache, build artifacts, temp files)
- User has been warned about file visibility consequences
- User confirmed via Phase 01.1.6 double-confirmation flow

**Use "File-based tracking" when:**
- User wants all files visible (safest option)
- Git operations declined
- No git repository available
- User is uncertain about git stash behavior

### Stash Safety Checklist

**Before executing git stash --include-untracked:**
- [ ] User has been shown file list (via Phase 01.1.5 "Show files" option or Phase 01.1.6)
- [ ] Warning box displayed explaining visibility consequences
- [ ] Untracked file count shown (especially story files)
- [ ] Recovery commands provided BEFORE stashing
- [ ] User explicitly confirmed via AskUserQuestion
- [ ] Alternative options offered (continue without stashing, commit first)

**After executing git stash:**
- [ ] Success message displayed
- [ ] Recovery instructions shown again
- [ ] Stash reference provided (stash@{0})
- [ ] File count confirmed ({N} files stashed)

**If user cancels stashing:**
- [ ] Alternative path taken (file-based tracking or commit first)
- [ ] User informed of choice made
- [ ] Workflow continues or halts appropriately

---

## Smart Stash Strategy (RCA-008)

**Default Recommendation:** Stash modified files only, preserve untracked files.

**Rationale:** Untracked files are typically user-created content (story files, new code) with no git backup. Modified files can be recovered from git history. Separating them reduces risk of perceived data loss.

### Strategy Matrix

| File Status | Git Tracking | Stash Command | User-Created? | Recommendation | Risk Level |
|-------------|--------------|---------------|---------------|----------------|------------|
| Modified (M) | ✅ Tracked | Default `git stash` | Maybe | **Stash** | LOW (recoverable from git) |
| Untracked (??) | ❌ Not tracked | Needs `--include-untracked` | Usually | **Keep visible** | HIGH (no backup) |
| Deleted (D) | ✅ Tracked | Default `git stash` | N/A | Stash | LOW (tracked in git) |
| Added (A) | ✅ Staged | Default `git stash` | Yes | Stash or keep | MEDIUM |

### Stash Commands by Strategy

**Strategy 1: Stash Modified Only (RECOMMENDED - Default)**
```bash
git stash push -m "WIP: Modified files only"
# Does NOT use --include-untracked flag
# Result:
#   Modified files (M) → Stashed (hidden)
#   Untracked files (??) → Remain visible
#   User can continue working with untracked files visible
```

**Benefits:**
- ✅ Preserves user-created untracked files (story files, new code)
- ✅ Clean working tree for tracked files
- ✅ Low risk (untracked files have no backup, so keeping them visible is safer)
- ✅ Faster (no need to stash large untracked directories)

**Use when:**
- User has untracked files (especially story files, new code, documentation)
- Want clean working tree for tracked files only
- Untracked files are user-created content (high value)
- DEFAULT in Phase 01.1.5 (marked with ⭐ Recommended)

---

**Strategy 2: Stash Everything (USE WITH CAUTION)**
```bash
git stash push -m "WIP: All files" --include-untracked
# Requires: Steps 0.1.5 and 0.1.6 user consent workflow
# Result:
#   Modified files (M) → Stashed (hidden)
#   Untracked files (??) → Stashed (HIDDEN from filesystem)
#   Filesystem appears clean, all uncommitted work hidden
```

**Risks:**
- ⚠️ Hides user-created content with no git backup
- ⚠️ Files invisible until `git stash pop` executed
- ⚠️ User may forget to restore stash
- ⚠️ Confusing for non-git-experts

**Use when:**
- User explicitly requests it after seeing full warning (Phase 01.1.6)
- All files are regenerable (cache, build artifacts, temp files)
- User understands git stash behavior
- User confirmed via double-confirmation flow

**Required safeguards (MANDATORY):**
- Phase 01.1.5: Initial choice warning
- Phase 01.1.6: Second confirmation with file list
- Warning box explaining visibility consequences
- Recovery instructions shown before AND after stashing

---

**Strategy 3: File-Based Tracking (SAFEST)**
```bash
# No git commands executed
# All files remain visible in filesystem
# Changes tracked in devforgeai/stories/{STORY-ID}/changes/
```

**Benefits:**
- ✅ Zero risk of data loss
- ✅ All files always visible
- ✅ No git knowledge required
- ✅ Works without git repository

**Use when:**
- User wants all files visible (highest certainty)
- User declines git operations
- No git repository available
- User is uncertain about git stash behavior

**Limitations:**
- ⚠️ No version control features (branching, history, diffing)
- ⚠️ Manual file organization required
- ⚠️ Collaboration features disabled

---

### Recovery Commands Reference

**List stashes:**
```bash
git stash list
```

**Preview stash contents:**
```bash
git stash show stash@{0} --name-only           # File names only
git stash show stash@{0} --stat                # File stats
git stash show stash@{0} -p                    # Full diff
```

**Restore stashed files:**
```bash
git stash pop          # Restore and remove from stash (most common)
git stash apply        # Restore but keep in stash (for reuse)
git stash apply stash@{1}   # Restore specific stash
```

**Delete stash:**
```bash
git stash drop stash@{0}    # Delete specific stash
git stash clear             # Delete all stashes (DANGEROUS)
```

### Strategy Selection Decision Tree

```
Do you have uncommitted changes?
  ├─ NO → Proceed with clean workflow
  │
  └─ YES → Are there untracked files?
        ├─ NO (only modified files) → Stash modified files (safe)
        │
        └─ YES → What type of untracked files?
              ├─ Story files / User-created → Recommend: Keep visible OR Commit first
              ├─ Cache / Build artifacts → OK to stash all
              └─ Mixed → Recommend: Stash modified only, keep untracked visible ⭐
```

### Implementation in Phase 01.1.5

The smart stash strategy is implemented in Phase 01.1.5 of preflight-validation.md with these options (in order of safety):

1. **Continue anyway** - File-based tracking (safest, all files visible)
2. **Stash ONLY modified** - Recommended (clean tracked files, preserve untracked)
3. **Show files first** - Review before deciding
4. **Commit first** - Most organized (creates proper git history)
5. **Stash ALL** - Use with caution (requires Phase 01.1.6 warning)

**Default selection logic:**
- IF story_files > 0: Recommend option 2 (stash modified only) or option 1 (continue anyway)
- IF all files are cache: Option 5 (stash all) acceptable
- IF uncertain: Option 3 (show files first) to see what's affected

---

## Lock File Recovery

### Problem

Git fails with error: `fatal: Unable to create '.git/index.lock': File exists`

This occurs when a previous git operation was interrupted or another process is holding the lock.

### Diagnosis

```bash
# Check if lock file exists
ls -la .git/index.lock

# Check for running git processes
ps aux | grep git

# On Windows (if using PowerShell)
tasklist | findstr git
```

### Recovery

**WARNING:** Only run this if no git processes are running.

```bash
# Remove stale lock file
rm -f .git/index.lock
```

### WSL2-Specific Notes

**Common Causes:**
- VS Code with Git extension is open and polling for changes
- Cross-filesystem access between Windows (`C:\`) and WSL (`/mnt/c/`)
- Previous git command crashed without cleanup
- File system sync issues between Windows and WSL

**Prevention:**
1. Close VS Code Git panels before terminal git operations
2. Use native WSL paths (`/mnt/c/`) not Windows paths (`C:\`)
3. Avoid running git from both Windows and WSL on same repo
4. If using VS Code, disable "Git: Autofetch" setting temporarily

**Alternative Recovery (if rm fails):**
```bash
# Force remove on Windows filesystem
rm -rf .git/index.lock 2>/dev/null || cmd.exe /c "del /f /q .git\\index.lock"
```

---

## Branch Naming Conventions

### Feature Branches

**Pattern:** `feature/[STORY-ID]-[brief-description]`

**Examples:**
```bash
feature/STORY-001-user-authentication
feature/STORY-042-checkout-optimization
feature/EPIC-003-payment-integration
```

**Rules:**
- Always include story/epic ID
- Use lowercase with hyphens
- Keep description brief (2-4 words)
- Branch from main/master or development branch

### Bugfix Branches

**Pattern:** `bugfix/[STORY-ID]-[brief-description]`

**Examples:**
```bash
bugfix/STORY-015-login-redirect-issue
bugfix/BUG-028-cart-calculation-error
bugfix/critical-security-patch
```

**Rules:**
- Include bug/story ID when available
- "critical" prefix for urgent production fixes
- Branch from main/master or release branch

### Hotfix Branches

**Pattern:** `hotfix/[description]` or `hotfix/[HOTFIX-ID]-[description]`

**Examples:**
```bash
hotfix/production-crash-fix
hotfix/HOTFIX-001-payment-gateway-failure
hotfix/security-vulnerability-CVE-2024-XXXX
```

**Rules:**
- For critical production issues only
- Branch from production/main
- Merge to both production and development branches
- Deploy immediately after testing

### Release Branches

**Pattern:** `release/v[MAJOR].[MINOR].[PATCH]` or `release/[SPRINT-ID]`

**Examples:**
```bash
release/v1.2.0
release/v2.0.0-beta
release/SPRINT-003
```

**Rules:**
- Created when features complete for release
- No new features (bug fixes only)
- Merge to main and development when ready

---

## Conventional Commit Format

### Standard Structure

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Components:**
- **type**: Required - Commit category
- **scope**: Optional - Area of change (component, module, feature)
- **subject**: Required - Brief description (imperative mood, no period)
- **body**: Optional - Detailed explanation
- **footer**: Optional - References (closes #123, breaking changes)

### Commit Types

| Type | Usage | Example |
|------|-------|---------|
| **feat** | New feature or functionality | `feat: Add user authentication` |
| **fix** | Bug fix | `fix: Correct cart calculation error` |
| **refactor** | Code refactoring (no behavior change) | `refactor: Extract discount calculation method` |
| **test** | Adding or updating tests | `test: Add edge cases for order validation` |
| **docs** | Documentation changes | `docs: Update API endpoint documentation` |
| **style** | Code formatting (no logic change) | `style: Format code with Prettier` |
| **chore** | Build, dependencies, tooling | `chore: Update Dapper to 2.1.28` |
| **perf** | Performance improvements | `perf: Optimize database query for orders` |
| **ci** | CI/CD changes | `ci: Add code coverage to pipeline` |
| **build** | Build system changes | `build: Configure webpack for production` |
| **revert** | Revert previous commit | `revert: Revert "feat: Add feature X"` |

### Commit Message Examples

#### Feature Commit

```
feat(auth): Implement JWT authentication

- Added login/logout endpoints in AuthController
- Implemented token validation middleware
- Created user session management service
- Tests: 12 unit tests, 3 integration tests (95% coverage)
- Compliance: tech-stack.md (JWT), coding-standards.md (async/await)

Closes #STORY-001
```

#### Bug Fix Commit

```
fix(cart): Prevent duplicate items in shopping cart

- Added duplicate check before adding item to cart
- Updated CartService.AddItem method
- Added regression test for duplicate prevention
- Coverage: 98% for CartService

Fixes #BUG-045
```

#### Refactor Commit

```
refactor(order-service): Extract validation logic to separate class

- Created OrderValidator class
- Moved validation from OrderService to OrderValidator
- No behavior changes (all tests still passing)
- Tests: 100% pass rate maintained
- Improved testability and separation of concerns

Part of #STORY-003 refactoring phase
```

#### Test Commit

```
test(order-service): Add edge case tests for discount calculation

- Added tests for zero total
- Added tests for null coupon
- Added tests for expired coupon
- Added tests for negative discount
- Coverage increased from 87% to 95%

Part of #STORY-001 TDD cycle
```

#### Documentation Commit

```
docs(api): Update order endpoints documentation

- Added request/response examples for POST /api/orders
- Documented error codes and responses
- Updated authentication requirements
- Added rate limiting information

Related to #STORY-001
```

#### Breaking Change Commit

```
feat(api): Change order creation endpoint contract

BREAKING CHANGE: Order creation now requires customerId in request body

- Updated POST /api/orders to require customerId
- Removed customerEmail as alternative identifier
- Updated validation rules
- Migration guide added to docs/migration/v2.0.md

Closes #STORY-050
```

---

## Commit Timing in TDD Workflow

### Option 1: Single Commit Per Story (Recommended)

**When:** After Phase 08 (Integration) completes successfully

**Benefits:**
- Clean git history (one commit = one story)
- Easy to revert entire feature
- Simple workflow
- Matches story lifecycle

**Pattern:**
```bash
# After Phase 08: Integration complete
# All tests passing, QA approved, ready to commit

# Phase 08.0.5: Acquire lock (STORY-096)
python3 src/lock_file_coordinator.py acquire --story-id STORY-001 --timeout 600

# Step 5.1-5.2: Git add and commit (with lock held)
git add src/ tests/
git commit -m "$(cat <<'EOF'
feat: Implement order discount calculation

- Implemented CalculateDiscount method following TDD
- Tests: 15 unit tests, 3 integration tests
- Edge cases: null inputs, expired coupons, invalid codes
- Compliance: tech-stack.md (Dapper), coding-standards.md (Result Pattern)
- Coverage: 95% for OrderService
- QA: Light validation passed

Closes #STORY-001
EOF
)"

# Step 5.3: Release lock (STORY-096) - always release, even on failure
python3 src/lock_file_coordinator.py release --story-id STORY-001

git push origin feature/STORY-001-order-discounts
```

**Git History:**
```
feat: Implement order discount calculation (STORY-001)
feat: Add user profile page (STORY-002)
fix: Correct cart total bug (BUG-012)
```

### Option 2: Multiple Commits Per TDD Phase

**When:** For complex stories or learning/demonstration purposes

**Benefits:**
- Visible TDD progression in history
- Can revert to specific phase
- Educational (shows RED→GREEN→REFACTOR)

**Pattern:**
```bash
# After Phase 02: Tests written (RED)
git add tests/
git commit -m "test: Add failing tests for order discount calculation

- Added 15 test cases for discount scenarios
- Tests currently failing (RED phase)
- Will implement in next commit

Part of #STORY-001"

# After Phase 03: Implementation (GREEN)
git add src/
git commit -m "feat: Implement order discount calculation (Green phase)

- Implemented CalculateDiscount method
- All tests now passing
- Minimal implementation (will refactor next)

Part of #STORY-001"

# After Phase 04: Refactoring
git add src/
git commit -m "refactor: Improve discount calculation code quality

- Extracted validation logic to separate method
- Removed magic numbers
- Improved variable names
- All tests still passing

Part of #STORY-001"

# After Phase 08: Integration
git commit -m "feat: Complete order discount feature

- Integration tests passing
- QA validation passed
- Coverage: 95%
- Ready for code review

Closes #STORY-001"
```

**Git History:**
```
test: Add failing tests for order discount calculation
feat: Implement order discount calculation (Green phase)
refactor: Improve discount calculation code quality
feat: Complete order discount feature (STORY-001)
```

### Option 3: Hybrid Approach

**When:** Balance between clean history and meaningful checkpoints

**Pattern:**
```bash
# Commit after each significant milestone
# Squash before merging to main

# Checkpoint 1: Core implementation
git commit -m "feat: Implement core discount calculation logic"

# Checkpoint 2: Edge cases
git commit -m "feat: Add edge case handling for discounts"

# Checkpoint 3: Refactoring
git commit -m "refactor: Improve discount code quality"

# Before merge: Squash into single commit
git rebase -i HEAD~3
# Squash commits into one
```

### Recommended Strategy

**For most development:**
✅ **Use Option 1: Single Commit Per Story**

**Reasoning:**
- Simpler workflow
- Cleaner git history
- Easier code reviews
- Matches DevForgeAI story-driven development
- One story = one merge = one commit to main

---

## Staging Strategy

### What to Stage

**Always include:**
```bash
# Source code changes
git add src/Application/Services/AuthService.cs
git add src/Domain/Entities/User.cs

# Test files
git add tests/Application.Tests/Services/AuthServiceTests.cs
git add tests/Integration/AuthIntegrationTests.cs

# Updated documentation
git add docs/api/auth-endpoints.md

# Configuration changes (if needed for feature)
git add appsettings.json
```

**Always exclude:**
```bash
# Temporary files
git reset HEAD temp.txt
git reset HEAD *.tmp

# IDE configuration (should be in .gitignore)
.vscode/
.idea/
.vs/

# Build artifacts (should be in .gitignore)
bin/
obj/
dist/
node_modules/

# Secrets or credentials
git reset HEAD .env
git reset HEAD secrets.json
git reset HEAD credentials.config
```

### Git Add Commands

**Stage specific files:**
```bash
git add src/Services/OrderService.cs
git add tests/Services/OrderServiceTests.cs
```

**Stage by pattern:**
```bash
# All C# files in src/
git add src/**/*.cs

# All test files
git add tests/**/*Tests.cs

# All markdown files
git add docs/**/*.md
```

**Stage all changes (use carefully):**
```bash
# Stage everything (review first with git status!)
git add .
```

**Interactive staging (selective hunks):**
```bash
# Choose which changes to stage interactively
git add -p

# y = stage this hunk
# n = don't stage this hunk
# s = split into smaller hunks
# q = quit
```

### Reviewing Staged Changes

```bash
# See what's staged
git diff --staged

# See what's not staged
git diff

# See both staged and unstaged
git status
```

---

## Commit Message Template

### Bash Command Format

**Using heredoc for multi-line messages:**

```bash
git commit -m "$(cat <<'EOF'
<type>(<scope>): <subject>

- <change 1>
- <change 2>
- <change 3>
- Tests: <test description>
- Compliance: <context file references>
- Coverage: <percentage>

Closes #<story-id>
EOF
)"
```

**Important:** Use `<<'EOF'` (with quotes) to prevent bash variable expansion

### Template by Commit Type

#### feat (Feature)

```bash
git commit -m "$(cat <<'EOF'
feat(module): Brief description of feature

- Implemented <functionality>
- Added <components>
- Tests: <count> unit tests, <count> integration tests
- Compliance: tech-stack.md, coding-standards.md
- Coverage: <percentage>%

Closes #STORY-XXX
EOF
)"
```

#### fix (Bug Fix)

```bash
git commit -m "$(cat <<'EOF'
fix(module): Brief description of bug

- Fixed <issue>
- Root cause: <explanation>
- Added regression test
- Coverage: <percentage>%

Fixes #BUG-XXX
EOF
)"
```

#### refactor (Refactoring)

```bash
git commit -m "$(cat <<'EOF'
refactor(module): Brief description of refactoring

- Extracted <method/class>
- Removed duplication
- Improved <aspect>
- No behavior changes
- All tests still passing (100%)

Part of #STORY-XXX refactoring phase
EOF
)"
```

#### test (Tests Only)

```bash
git commit -m "$(cat <<'EOF'
test(module): Brief description of tests

- Added tests for <scenarios>
- Edge cases: <list>
- Coverage increased from X% to Y%

Part of #STORY-XXX TDD cycle
EOF
)"
```

---

## Push Timing and Best Practices

### When to Push

✅ **CORRECT timing:**
```bash
# After Phase 08: Integration complete
# All tests passing
# QA validation passed
# Ready for code review

git push origin feature/STORY-001-user-auth
```

✅ **CORRECT for checkpointing:**
```bash
# After significant milestone
# Tests passing
# Want to backup work

git push origin feature/STORY-001-user-auth
```

❌ **FORBIDDEN timing:**
```bash
# Don't push when tests are failing
# Don't push with commented-out code
# Don't push with secrets/credentials
# Don't push incomplete implementation (unless draft PR)
```

### Push Commands

**Standard push:**
```bash
# Push current branch to remote
git push origin feature/STORY-001-user-auth
```

**First push (set upstream):**
```bash
# Push and set upstream tracking
git push -u origin feature/STORY-001-user-auth

# Now can use simple 'git push'
git push
```

**Force push (use with caution):**
```bash
# ONLY use on personal feature branches
# NEVER use on shared branches (main, development)

git push --force-with-lease origin feature/STORY-001-user-auth
```

### Pre-Push Checklist

- [ ] All tests passing (`dotnet test` / `pytest` / `npm test`)
- [ ] Build succeeds (`dotnet build` / `npm run build`)
- [ ] No debug code or console.log statements
- [ ] No commented-out code
- [ ] No secrets or credentials in code
- [ ] Commit message follows conventions
- [ ] Changes reviewed with `git diff`

---

## Git Hooks Integration

### Pre-Commit Hook

**Purpose:** Validate changes before commit

**Checks:**
```bash
#!/bin/bash
# .git/hooks/pre-commit

# Run linter
npm run lint || exit 1

# Check for secrets
if git diff --cached | grep -i "password\|secret\|api_key"; then
    echo "❌ Secrets detected in commit"
    exit 1
fi

# Run quick tests
npm run test:quick || exit 1

echo "✅ Pre-commit checks passed"
exit 0
```

### Commit-Msg Hook

**Purpose:** Validate commit message format

**Checks:**
```bash
#!/bin/bash
# .git/hooks/commit-msg

commit_msg=$(cat "$1")

# Check conventional commit format
if ! echo "$commit_msg" | grep -E "^(feat|fix|refactor|test|docs|style|chore|perf|ci|build|revert)(\(.+\))?: .+"; then
    echo "❌ Commit message must follow conventional format"
    echo "Example: feat(auth): Add login endpoint"
    exit 1
fi

# Check message length
subject=$(echo "$commit_msg" | head -n1)
if [ ${#subject} -gt 100 ]; then
    echo "❌ Subject line must be ≤ 100 characters"
    exit 1
fi

echo "✅ Commit message validated"
exit 0
```

### Pre-Push Hook

**Purpose:** Run full validation before pushing

**Checks:**
```bash
#!/bin/bash
# .git/hooks/pre-push

# Run full test suite
echo "Running full test suite..."
npm test || exit 1

# Run code coverage
echo "Checking code coverage..."
npm run test:coverage || exit 1

# Check coverage threshold (example: 80%)
coverage=$(npm run test:coverage | grep "All files" | awk '{print $10}')
threshold=80

if [ "${coverage%.*}" -lt "$threshold" ]; then
    echo "❌ Coverage ${coverage}% below threshold ${threshold}%"
    exit 1
fi

echo "✅ Pre-push checks passed"
exit 0
```

---

## Branch Management

### Merging Strategies

#### Squash and Merge (Recommended)

**Use case:** Feature branches to main
**Benefits:** Clean history (one commit per feature)

```bash
# On GitHub/GitLab: Use "Squash and merge" button
# Combines all commits into one

# Manual squash:
git checkout main
git merge --squash feature/STORY-001-user-auth
git commit -m "feat: Implement user authentication (STORY-001)"
```

#### Merge Commit

**Use case:** Long-running branches (development → main)
**Benefits:** Preserves branch history

```bash
git checkout main
git merge feature/STORY-001-user-auth --no-ff
```

#### Rebase and Merge

**Use case:** Clean linear history
**Benefits:** No merge commits

```bash
git checkout feature/STORY-001-user-auth
git rebase main
git checkout main
git merge feature/STORY-001-user-auth --ff-only
```

### Branch Cleanup

**After merge:**
```bash
# Delete local branch
git branch -d feature/STORY-001-user-auth

# Delete remote branch
git push origin --delete feature/STORY-001-user-auth
```

---

## Multi-File Commit Organization

### Single Cohesive Commit (Recommended)

**When:** All changes are part of same story

```bash
# Stage all related files together
git add src/Services/OrderService.cs
git add src/Models/Order.cs
git add tests/Services/OrderServiceTests.cs
git add docs/api/orders.md

# Single commit for entire story
git commit -m "$(cat <<'EOF'
feat: Implement order management

- Created OrderService with CRUD operations
- Added Order model with validation
- Tests: 20 unit tests, 5 integration tests
- Documentation: Updated API docs
- Coverage: 95%

Closes #STORY-001
EOF
)"
```

### Multiple Logical Commits

**When:** Story has distinct logical phases

```bash
# Commit 1: Data model
git add src/Models/Order.cs
git commit -m "feat: Add Order data model"

# Commit 2: Service implementation
git add src/Services/OrderService.cs
git add tests/Services/OrderServiceTests.cs
git commit -m "feat: Implement OrderService with tests"

# Commit 3: API endpoints
git add src/Controllers/OrdersController.cs
git add docs/api/orders.md
git commit -m "feat: Add order API endpoints"

# Before merge: Squash into single commit
git rebase -i HEAD~3
```

---

## Git Workflow Checklist

### Pre-Commit Checklist

- [ ] Review changes: `git status`, `git diff`
- [ ] All tests passing
- [ ] Build succeeds
- [ ] No debug code
- [ ] No secrets in code
- [ ] Commit message follows format
- [ ] Only relevant files staged

### Pre-Push Checklist

- [ ] All commits follow conventions
- [ ] Full test suite passing
- [ ] Code coverage meets requirements
- [ ] No force push to shared branches
- [ ] Branch up to date with main

### Pre-Merge Checklist

- [ ] All CI checks passing
- [ ] Code review approved
- [ ] No merge conflicts
- [ ] Documentation updated
- [ ] Ready to deploy

---

## Quick Reference

### Git Commands Summary

```bash
# Status and review
git status                          # Show working tree status
git diff                            # Show unstaged changes
git diff --staged                   # Show staged changes
git log --oneline                   # Show commit history

# Staging
git add <file>                      # Stage specific file
git add .                           # Stage all changes
git add -p                          # Interactive staging

# Committing
git commit -m "message"             # Commit with inline message
git commit -m "$(cat <<'EOF'...)"   # Commit with heredoc

# Pushing
git push origin <branch>            # Push to remote
git push -u origin <branch>         # Push and set upstream
git push --force-with-lease         # Force push safely

# Branching
git checkout -b <branch>            # Create and switch to branch
git branch -d <branch>              # Delete local branch
git push origin --delete <branch>   # Delete remote branch
```

### Conventional Commit Types

| Type | Purpose |
|------|---------|
| `feat` | New feature |
| `fix` | Bug fix |
| `refactor` | Code refactoring |
| `test` | Tests only |
| `docs` | Documentation |
| `chore` | Dependencies, build |
| `perf` | Performance |
| `style` | Formatting |

### Branch Name Patterns

```
feature/STORY-XXX-description
bugfix/BUG-XXX-description
hotfix/description
release/vX.Y.Z
```

---

This reference should be used during Phase 08 (Git Workflow) to maintain clean, traceable version control history that aligns with the TDD development process and DevForgeAI story-driven workflow.