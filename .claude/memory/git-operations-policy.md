# Git Operations Policy (RCA-008)

**Purpose:** Prevent autonomous git operations that hide or destroy user files.
**Source:** RCA-008 incident (2025-11-13) - autonomous git stash hid 21 user-created story files.

---

## NEVER Execute Autonomously

The following git commands require **user approval via AskUserQuestion**:

- `git stash` (especially with `--include-untracked`)
- `git reset --hard`
- `git branch -D` (delete branches)
- `git push --force`
- `git commit --amend` (for commits not created in current session)
- Any operation affecting >10 files

---

## Required Approval Pattern

```
AskUserQuestion(
    questions=[{
        question: "Git operation will affect {N} files. How should we proceed?",
        header: "Git Action",
        multiSelect: false,
        options: [
            {
                label: "Show me the files first",
                description: "Display file list before deciding."
            },
            {
                label: "Proceed (I understand the consequences)",
                description: "[Clear explanation of what will happen]"
            },
            {
                label: "Cancel (use alternative approach)",
                description: "[What alternative will be used instead]"
            }
        ]
    }]
)
```

---

## Exceptions (No Approval Needed)

- `git status` (read-only)
- `git diff` (read-only)
- `git log` (read-only)
- `git add` for current story files (≤5 files created in this session)
- `git commit` for current story implementation (TDD workflow)

---

## File-Based Fallback

When git operations are declined or unavailable:
- Use `devforgeai/stories/{STORY-ID}/changes/` directory
- Document changes in `changes-manifest.md`
- Preserve all user files (nothing hidden)

---

## See Also

- `devforgeai/RCA/RCA-008-autonomous-git-stashing.md` (full incident analysis)
- `.claude/skills/devforgeai-development/references/git-workflow-conventions.md`
- `.claude/skills/devforgeai-development/references/preflight/_index.md` (Steps 0.1.5, 0.1.6)
