---
description: List and manage Git worktrees for parallel development
argument-hint: "[--help]"
model: opus
allowed-tools: Read, Glob, Bash(git:*), Bash(du:*), AskUserQuestion, Task
---

# /worktrees - Git Worktree Management

View and manage Git worktrees created by /dev for parallel story development.

---

## Quick Reference

```bash
# List all worktrees
/worktrees

# Show help
/worktrees --help
```

---

## Command Workflow

### Phase 01: Argument Validation

**Parse arguments:**
```
SHOW_HELP = false

FOR arg in arguments:
    IF arg == "--help" OR arg == "-h":
        SHOW_HELP = true

IF SHOW_HELP:
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: "  /worktrees - Git Worktree Management"
    Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    Display: ""
    Display: "Usage: /worktrees [--help]"
    Display: ""
    Display: "Displays all Git worktrees with:"
    Display: "  - Story ID, Path, Age, Size, Status, Last Activity"
    Display: "  - Cleanup candidates (idle >7 days)"
    Display: "  - Interactive menu for actions"
    Display: ""
    Display: "Actions:"
    Display: "  1. Cleanup all candidates - Remove idle worktrees"
    Display: "  2. Cleanup selected - Choose specific worktrees"
    Display: "  3. Inspect worktree - View details"
    Display: "  4. Resume development - Get path to continue work"
    Display: "  5. Cancel - Exit without changes"
    Display: ""
    HALT
```

---

### Phase 02: Git Availability Check

**Verify Git is available:**
```bash
git --version >/dev/null 2>&1
```

**On failure:**
```
Display: "❌ Git not available"
Display: "   Install Git: https://git-scm.com/downloads"
HALT
```

---

### Phase 03: Discover Worktrees

**Get worktree list:**
```bash
git worktree list --porcelain
```

**Parse output into structured data:**
```
worktrees = []
FOR each worktree entry:
    Extract: path, branch, commit

    # Skip main worktree
    IF branch == "main" OR branch == "master":
        CONTINUE

    # Extract story ID from branch name (e.g., "story-091" -> "STORY-091")
    story_id = extract_story_id(branch)

    # Get last activity (last commit date in worktree)
    last_activity = git log -1 --format="%ci" in worktree

    # Calculate age in days
    days_idle = (now - last_activity).days

    # Get size
    size = du -sh worktree_path

    worktrees.append({
        story_id, path, branch, last_activity, days_idle, size
    })
```

**On no worktrees found:**
```
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  Git Worktrees"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""
Display: "No active worktrees found."
Display: ""
Display: "Create worktrees by running /dev STORY-NNN"
HALT
```

---

### Phase 04: Get Story Statuses

**For each worktree, lookup story status:**
```
FOR worktree in worktrees:
    story_file = Glob("devforgeai/specs/Stories/*{story_id}*.story.md")

    IF story_file exists:
        status = Read story_file frontmatter "status" field
    ELSE:
        status = "Unknown"

    worktree.status = status
```

---

### Phase 05: Display Worktree Table

**Format and display table:**
```
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  Git Worktrees"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""

# Table header
Display: "Story ID     Path                          Age        Size    Status          Last Activity"
Display: "───────────────────────────────────────────────────────────────────────────────────────────"

# Table rows
threshold = 7  # days
FOR worktree in worktrees:
    idle_marker = "⚠️" IF worktree.days_idle > threshold ELSE "  "
    age = format_age(worktree.days_idle)  # "2 days", "< 1 day", etc.

    Display: "{story_id}   {path}   {age}   {size}   {status}{idle_marker}   {last_activity}"
```

---

### Phase 06: Identify Cleanup Candidates

**Find idle worktrees:**
```
threshold = 7  # days
cleanup_candidates = [wt for wt in worktrees IF wt.days_idle > threshold]

IF cleanup_candidates:
    IF len(cleanup_candidates) == 1:
        Display: ""
        Display: "⚠️  1 worktree idle >7 days (cleanup candidate)"
    ELSE:
        Display: ""
        Display: "⚠️  {len} worktrees idle >7 days (cleanup candidates)"
ELSE:
    Display: ""
    Display: "✓ All worktrees have recent activity"
```

---

### Phase 07: Present Action Menu

**Display interactive menu:**
```
AskUserQuestion(
    questions=[{
        "question": "What would you like to do with the worktrees?",
        "header": "Action",
        "multiSelect": false,
        "options": [
            {
                "label": "Cleanup all candidates",
                "description": "Remove all worktrees idle >7 days with Released/QA Approved status"
            },
            {
                "label": "Cleanup selected",
                "description": "Choose specific worktrees to remove"
            },
            {
                "label": "Inspect worktree",
                "description": "View detailed info about a worktree"
            },
            {
                "label": "Resume development",
                "description": "Get path to resume work in a worktree"
            },
            {
                "label": "Cancel",
                "description": "Exit without changes"
            }
        ]
    }]
)
```

---

### Phase 08: Execute Selected Action

**Handle user selection:**

#### Option 1: Cleanup all candidates
```
safe_statuses = ["Released", "QA Approved", "Backlog"]
unsafe_statuses = ["In Development", "Dev Complete", "QA In Progress"]

FOR candidate in cleanup_candidates:
    IF candidate.status in safe_statuses:
        # Safe to delete
        Bash: git worktree remove --force {candidate.path}
        Display: "✓ Deleted {story_id} (status: {status})"

    ELIF candidate.status in unsafe_statuses:
        # Require confirmation
        confirm = AskUserQuestion("Delete {story_id}? Status is {status} - work may be lost")

        IF confirm:
            Bash: git worktree remove --force {candidate.path}
            Display: "✓ Deleted {story_id}"
        ELSE:
            Display: "⏭ Skipped {story_id}"

    ELSE:
        # Unknown status - require confirmation
        Display: "⚠️ {story_id} has unknown status, skipping"

Display: ""
Display: "Cleanup complete"
```

#### Option 2: Cleanup selected
```
# Present multi-select of worktrees
selection = AskUserQuestion(
    question="Select worktrees to cleanup:",
    header="Select",
    multiSelect=true,
    options=[{label: wt.story_id, description: f"{wt.path} ({wt.status})"} for wt in worktrees]
)

FOR selected in selection:
    # Apply same safe cleanup logic as Option 1
    ...
```

#### Option 3: Inspect worktree
```
# Present worktree selection
selection = AskUserQuestion(
    question="Select worktree to inspect:",
    header="Inspect",
    multiSelect=false,
    options=[{label: wt.story_id, description: wt.path} for wt in worktrees]
)

worktree = find_worktree(selection)

Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  Worktree Details: {worktree.story_id}"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""
Display: "Story ID:      {worktree.story_id}"
Display: "Path:          {worktree.path}"
Display: "Branch:        {worktree.branch}"
Display: "Age:           {worktree.days_idle} days"
Display: "Size:          {worktree.size}"
Display: "Status:        {worktree.status}"
Display: "Last Activity: {worktree.last_activity}"
Display: ""

# Show recent commits
Display: "Recent Commits:"
Bash: git -C {worktree.path} log --oneline -5
```

#### Option 4: Resume development
```
# Present worktree selection
selection = AskUserQuestion(
    question="Select worktree to resume:",
    header="Resume",
    multiSelect=false,
    options=[{label: wt.story_id, description: f"{wt.path} ({wt.status})"} for wt in worktrees]
)

worktree = find_worktree(selection)

Display: ""
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: "  Resume Development: {worktree.story_id}"
Display: "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
Display: ""
Display: "To resume development, run:"
Display: ""
Display: "  cd {worktree.path}"
Display: "  /dev {worktree.story_id}"
Display: ""
```

#### Option 5: Cancel
```
Display: ""
Display: "No changes made."
```

---

## Edge Case Handling

### Corrupted Worktree
```
IF worktree directory missing but git knows about it:
    Display: "⚠️ Orphaned worktree detected: {path}"
    Display: "   Run: git worktree prune"
```

### Story File Not Found
```
IF story file missing:
    worktree.status = "Unknown"
    # Continue with display
```

### Git Command Failures
```
ON git command failure:
    Display: "❌ Failed: {command}"
    Display: "   Error: {error_message}"
    # Continue with remaining worktrees
```

---

## Performance Notes

- Worktree listing: `git worktree list` is fast (~100ms)
- Status lookup: File reads cached (~50ms per file)
- Size calculation: `du -sh` can be slow for large worktrees
- Target: < 5 seconds total for up to 20 worktrees

---

**Story:** STORY-095 - /worktrees Management Command
**Created:** 2025-12-17
**Lines:** ~200
