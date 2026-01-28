# Preflight Validation Reference Index

**Phase 01 Token Budget:** Load only what's needed per step
**Total Original:** 3,020 lines → Now decomposed into 18 files

---

## Overview

Phase 01 validates environment, git state, and story specifications before TDD execution.

**Steps (in order):**
0. Validate Project Root (CWD)
1. Validate Git repository status
1.5. User consent for Git changes
1.6. Stash warning workflow
1.7. Story File Isolation Check
2. Git Worktree Auto-Management
2.5. Dependency Graph Validation
2.6. File Overlap Detection
3. Adapt TDD workflow based on Git availability
4. File-based change tracking (if no Git)
5. Validate context files exist
6. Load story specification
6.5. Story Type Detection
7. Validate spec vs context files
8. Detect and validate technology stack
9. Detect previous QA failures
9.5. Load structured gap data
10. Story Complexity Analysis

---

## File Navigation

### Mandatory Steps (Execute in Order)

| Step | File | Lines | Description |
|------|------|-------|-------------|
| 01.0 | [01.0-project-root.md](01.0-project-root.md) | ~63 | Validate project root |
| 01.0.5 | [01.0.5-cli-check.md](01.0.5-cli-check.md) | ~78 | CLI availability check |
| 01.1 | [01.1-git-status.md](01.1-git-status.md) | ~83 | Git repository validation |
| 01.5 | [01.5-context-files.md](01.5-context-files.md) | ~43 | Context file validation |
| 01.6 | [01.6-load-story.md](01.6-load-story.md) | ~113 | Story loading + type detection |
| 01.7 | [01.7-validate-spec.md](01.7-validate-spec.md) | ~39 | Spec vs context validation |
| 01.8 | [01.8-tech-stack.md](01.8-tech-stack.md) | ~93 | Tech stack detection |
| 01.9 | [01.9-qa-failures.md](01.9-qa-failures.md) | ~186 | QA failure + gaps.json |

### Conditional Steps (Execute When Triggered)

| Step | File | Trigger | Lines |
|------|------|---------|-------|
| 01.1.5 | [01.1.5-user-consent.md](01.1.5-user-consent.md) | uncommitted > 10 | ~244 |
| 01.1.6 | [01.1.6-stash-warning.md](01.1.6-stash-warning.md) | user selects stash | ~171 |
| 01.1.7 | [01.1.7-story-isolation.md](01.1.7-story-isolation.md) | uncommitted story files | ~639 |
| 01.2 | [01.2-worktree.md](01.2-worktree.md) | Git available + enabled | ~149 |
| 01.2.5 | [01.2.5-dependency-graph.md](01.2.5-dependency-graph.md) | story has dependencies | ~140 |
| 01.2.6 | [01.2.6-file-overlap.md](01.2.6-file-overlap.md) | parallel stories | ~164 |
| 01.4 | [01.4-file-tracking.md](01.4-file-tracking.md) | Git unavailable | ~146 |

### Informational Steps

| Step | File | Lines | Description |
|------|------|-------|-------------|
| 01.3 | [01.3-workflow-adapt.md](01.3-workflow-adapt.md) | ~25 | Workflow mode selection |
| 01.10 | [01.10-complexity.md](01.10-complexity.md) | ~430 | Story complexity scoring |
| Final | [completion-checkpoint.md](completion-checkpoint.md) | ~81 | Final validation checklist |

---

## Loading Pattern

```
# Load mandatory files in sequence
Read("references/preflight/01.0-project-root.md")
Read("references/preflight/01.0.5-cli-check.md")
Read("references/preflight/01.1-git-status.md")

# Load conditional files based on triggers
IF uncommitted_changes > 10:
    Read("references/preflight/01.1.5-user-consent.md")

IF uncommitted_story_files:
    Read("references/preflight/01.1.7-story-isolation.md")

# Continue with remaining mandatory files...
```

---

## Variables Set by Phase 01

After completion, these variables are available for Phases 02-08:

- `$GIT_AVAILABLE` - Boolean
- `$WORKFLOW_MODE` - "full" | "partial" | "fallback"
- `$CAN_COMMIT` - Boolean
- `$WORKTREE_PATH` - String (if created)
- `$TEST_COMMAND` - String
- `$TEST_COVERAGE_COMMAND` - String
- `$BUILD_COMMAND` - String
- `$QA_*_FAILURE` - Boolean flags
- `$REMEDIATION_MODE` - Boolean
- `$QA_COVERAGE_GAPS` - Array
- `$STORY_TYPE` - String
- `$SKIP_PHASES` - Array
