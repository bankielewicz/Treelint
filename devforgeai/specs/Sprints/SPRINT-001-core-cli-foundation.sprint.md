---
id: SPRINT-001
name: Core CLI Foundation
status: Active
epic: EPIC-001
start_date: 2026-01-27
end_date: 2026-02-03
duration_days: 7
total_points: 23
completed_points: 0
velocity: 0
created: 2026-01-27
owner: Bryan
---

# Sprint: Core CLI Foundation

## Sprint Goal

Deliver a working Treelint CLI with AST-aware symbol search capabilities, completing all features in EPIC-001 (Core CLI Foundation).

**Success Criteria:**
- `treelint search <symbol>` command works with all modifiers
- JSON/text output with TTY auto-detection
- Context modes (lines, full, signatures)
- < 50ms query latency on indexed repositories

---

## Stories

| Story | Title | Points | Priority | Status | Assigned |
|-------|-------|--------|----------|--------|----------|
| [STORY-001](../Stories/STORY-001-project-setup-cli-skeleton.story.md) | Project Setup + CLI Skeleton | 3 | Critical | Ready for Dev | - |
| [STORY-002](../Stories/STORY-002-treesitter-parser-integration.story.md) | tree-sitter Parser Integration | 5 | Critical | Ready for Dev | - |
| [STORY-003](../Stories/STORY-003-sqlite-index-storage.story.md) | SQLite Index Storage | 3 | Critical | Ready for Dev | - |
| [STORY-004](../Stories/STORY-004-search-command-logic.story.md) | Search Command Logic | 2 | Critical | Ready for Dev | - |
| [STORY-005](../Stories/STORY-005-json-text-output.story.md) | JSON/Text Output | 5 | High | Ready for Dev | - |
| [STORY-006](../Stories/STORY-006-context-modes.story.md) | Context Modes | 5 | High | Ready for Dev | - |

---

## Dependency Graph

```
STORY-001 (CLI Skeleton)
    └── STORY-002 (tree-sitter Parser)
            └── STORY-003 (SQLite Storage)
                    └── STORY-004 (Search Logic)
                            └── STORY-005 (JSON/Text Output)

STORY-002 + STORY-005
    └── STORY-006 (Context Modes)
```

**Recommended Execution Order:**
1. STORY-001 → STORY-002 → STORY-003 → STORY-004 → STORY-005 → STORY-006
2. STORY-006 can start after STORY-002 and STORY-005 complete

---

## Capacity

| Metric | Value |
|--------|-------|
| **Total Points** | 23 |
| **Duration** | 7 days |
| **Points/Day** | 3.3 |
| **Status** | Aggressive (typically 20-40 pts for 2-week sprint) |

**Risk Assessment:**
- 23 points in 7 days is aggressive but achievable
- Stories are well-defined with clear acceptance criteria
- Dependencies are sequential (reduces parallelization)
- Recommend focusing on one story at a time following TDD

---

## Progress Tracking

### Burndown

| Day | Date | Remaining | Completed | Notes |
|-----|------|-----------|-----------|-------|
| 0 | 2026-01-27 | 23 | 0 | Sprint started |
| 1 | 2026-01-28 | - | - | - |
| 2 | 2026-01-29 | - | - | - |
| 3 | 2026-01-30 | - | - | - |
| 4 | 2026-01-31 | - | - | - |
| 5 | 2026-02-01 | - | - | - |
| 6 | 2026-02-02 | - | - | - |
| 7 | 2026-02-03 | - | - | Sprint end |

### Story Status Summary

| Status | Count | Points |
|--------|-------|--------|
| Ready for Dev | 6 | 23 |
| In Development | 0 | 0 |
| Dev Complete | 0 | 0 |
| QA Approved | 0 | 0 |
| Released | 0 | 0 |

---

## Daily Standup Template

**Date:** YYYY-MM-DD

**Yesterday:**
- [ ] Completed:
- [ ] Blockers encountered:

**Today:**
- [ ] Planning to work on:
- [ ] Dependencies needed:

**Blockers:**
- [ ] None / List blockers

---

## Sprint Retrospective (End of Sprint)

*To be completed at sprint end*

### What Went Well
- TBD

### What Could Be Improved
- TBD

### Action Items
- TBD

---

## Workflow History

| Timestamp | Action | Details |
|-----------|--------|---------|
| 2026-01-27 | Created | Sprint created with 6 stories (23 pts) |
| 2026-01-27 | Stories Updated | All stories status: Backlog → Ready for Dev |

---

**Sprint Template Version:** 1.0
**Last Updated:** 2026-01-27
