# Phase 08: Git Workflow & Commit

**Entry Gate:**
```bash
devforgeai-validate phase-check ${STORY_ID} --from=07 --to=08

Examples (--project-root applies to phase-* commands only, not check-hooks/invoke-hooks):
 - Correct: devforgeai-validate phase-init ${STORY_ID} --project-root=.
 - Incorrect: python -m devforgeai.cli.devforgeai_validate phase-init ${STORY_ID} --project-root=.
# Exit code 0: Transition allowed
# Exit code 1: Phase 07 not complete - HALT (DoD not validated)
# Exit code 2: DoD validation failed - HALT
```

---

## Phase Workflow

**Purpose:** Budget enforcement, handle incomplete items, git commit

**Required Subagents:** None (git operations)

**Pre-Requisite:** DoD format validated in Phase 07

**Steps:**

1.6. **Budget enforcement**
   - Check deferral budget per story
   - HALT if budget exceeded

1.7. **Handle new incomplete items**
   - Any items discovered during Phase 08 need deferral challenge
   - Return to Phase 06 if new deferrals detected

1.8. **Lock acquisition** (parallel story support)
   ```
   # Acquire story lock for commit
   # See references/lock-file-coordination.md
       Read(file_path=".claude/skills/devforgeai-development/references/lock-file-coordination.md")

   ```

2.0. **Stage files for commit**
   ```bash
   git add ${STORY_FILE}
   git add ${IMPL_FILES}
   git add ${TEST_FILES}
   ```

2.1. **Create git commit**
   ```bash
   git commit -m "$(cat <<'EOF'
   feat(${STORY_ID}): ${STORY_TITLE}

   - Implemented ${FEATURE_DESCRIPTION}
   - Tests passing (X/Y)
   - Coverage: ${COVERAGE}%

   Refs: ${STORY_ID}
   EOF
   )"
   ```

2.2. **Verify commit success**
   ```bash
   git log -1 --format="%H %s"
   # Should show new commit
   ```

2.3. **Release lock**
   ```
   # Release story lock after commit
   ```

2.4. **Update story status**
   - Story file status should already be "Dev Complete" from Phase 07

2.5. **Update AC Checklist (deployment items)**
   ```
   Edit(
     file_path="${STORY_FILE}",
     old_string="- [ ] Git commit",
     new_string="- [x] Git commit"
   )
   ```

**Reference:** `references/git-workflow-conventions.md` for complete workflow
    Read(file_path=".claude/skills/devforgeai-development/references/git-workflow-conventions.md")

---

## Git Commit Message Format

```
feat(STORY-XXX): Brief description

- Key implementation detail 1
- Key implementation detail 2
- Test results summary

Refs: STORY-XXX
```

**Commit Types:**
- `feat` - New feature
- `fix` - Bug fix
- `refactor` - Code refactoring
- `test` - Test changes
- `docs` - Documentation

---

## Validation Checkpoint

**Before proceeding to Phase 09, verify:**

- [ ] Git commit succeeded (commit hash exists)
- [ ] Story file included in commit
- [ ] AC Checklist (deployment items) updated ([ ] → [x])

**IF any checkbox UNCHECKED:** HALT workflow

### AC Checklist Update Verification (RCA-003)

After Step 2.5 completes, verify AC Checklist was actually updated:
```
Grep(pattern="- \\[x\\].*[Gg]it", path="${STORY_FILE}")
# Should find checked git/deployment-related items
# If no matches found: AC Checklist update was skipped - HALT
```

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
  "id": "obs-08-{seq}",
  "phase": "08",
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
devforgeai-validate phase-complete ${STORY_ID} --phase=08 --checkpoint-passed
# Exit code 0: Phase complete, proceed to Phase 09
# Exit code 1: Cannot complete - commit failed
```
