---
description: Git operations policy for DevForgeAI
version: "1.0"
created: 2025-12-10
source: .claude/memory/git-operations-policy.md
---

# Git Operations Policy

## Safe Operations (Auto-Approved)
- `git status`
- `git log`
- `git diff`
- `git branch` (list only)
- `git add`
- `git commit` (with message)
- `git push` (to current branch)

## Requires User Approval
- `git stash`
- `git reset --hard`
- `git push --force`
- `git branch -d` (delete)
- `git checkout -b` (new branch)
- `git merge`
- `git rebase`
- `git commit --amend`

## Forbidden Without Explicit Request
- `git push --force` to main/master
- `git reset --hard HEAD~N` (N > 1)
- Modifying `.git/hooks/`
- `git config --global`
