# Phase Marker Operations

**Status:** Reference Document
**Last Updated:** 2026-01-08
**Story:** STORY-193

## Overview

Phase markers are YAML files that track QA workflow progress. They enable:
- Sequential phase verification (pre-flight checks)
- Resume capability for interrupted sessions
- Audit trail for completed phases

**Location Pattern:** `devforgeai/qa/reports/{STORY_ID}/.qa-phase-{N}.marker`

---

## Write New Marker

Use the native **Write** tool for creating new marker files.

**Pattern:**
```
Write(
    file_path="devforgeai/qa/reports/{STORY_ID}/.qa-phase-{N}.marker",
    content="phase: {N}\nstory_id: {STORY_ID}\nmode: {MODE}\ntimestamp: {ISO_TIMESTAMP}\nstatus: complete"
)
```

**Example:**
```
Write(
    file_path="devforgeai/qa/reports/STORY-193/.qa-phase-0.marker",
    content="phase: 0\nstory_id: STORY-193\nmode: light\ntimestamp: 2026-01-08T12:00:00Z\nstatus: complete"
)
```

**Rationale:** Native tools are required per tech-stack.md (lines 206-207):
> ❌ `Bash(command="echo 'content' > file.txt")` - Use Write() instead

Native tools provide 40-73% token efficiency over Bash equivalents.

---

## Write Tool Workaround

**Issue:** The Write tool typically requires a prior Read of the target file. For NEW files (like markers), this creates a challenge since the file doesn't exist yet.

**Solution:** The Write tool CAN create new files without prior Read when:
1. The file path does not exist
2. The parent directory exists (create with `mkdir -p` first if needed)

**Workaround Pattern:**
```
# 1. Ensure directory exists (only once per story)
Bash(command="mkdir -p devforgeai/qa/reports/{STORY_ID}")

# 2. Write marker directly (no prior Read needed for new files)
Write(
    file_path="devforgeai/qa/reports/{STORY_ID}/.qa-phase-{N}.marker",
    content="phase: {N}\nstory_id: {STORY_ID}\n..."
)
```

**Note:** This workaround is specific to NEW file creation. Modifying existing files still requires Read-before-Edit.

---

## Verify Marker Exists

Use **Glob** to check if a marker file exists before proceeding to the next phase.

**Pattern:**
```
Glob(pattern="devforgeai/qa/reports/{STORY_ID}/.qa-phase-{N}.marker")
```

**Example - Pre-flight for Phase 1:**
```
# Verify Phase 0 completed before starting Phase 1
Glob(pattern="devforgeai/qa/reports/STORY-193/.qa-phase-0.marker")

IF NOT found:
    HALT: "Phase 0 not completed - run setup first"
ELSE:
    Display: "✓ Phase 0 verified complete"
```

**All Phase Pre-flight Checks:**
| Starting Phase | Required Marker |
|----------------|-----------------|
| Phase 1 | `.qa-phase-0.marker` |
| Phase 2 | `.qa-phase-1.marker` |
| Phase 3 | `.qa-phase-2.marker` |
| Phase 4 | `.qa-phase-3.marker` |

---

## Cleanup Markers (QA PASSED only)

Use **Bash rm** to remove marker files after successful QA completion.

**Pattern:**
```
Bash(command="rm devforgeai/qa/reports/{STORY_ID}/.qa-phase-*.marker")
```

**Example:**
```
IF overall_status == "PASSED":
    # Clean up all phase markers for this story
    Bash(command="rm devforgeai/qa/reports/STORY-193/.qa-phase-*.marker")
    Display: "✓ Phase markers cleaned up for STORY-193"
ELSE:
    # Retain markers for debugging and resume
    Display: "⚠️ QA FAILED - Markers retained for debugging"
```

**Rationale:** The `rm` command is an **exception** to the native tools rule because:
1. It's a cleanup/deletion operation, not file content modification
2. No native "Delete" tool exists in Claude Code Terminal
3. Bulk wildcard deletion (`*.marker`) is more efficient via Bash

**When to Clean:**
- ✅ QA PASSED - markers no longer needed
- ❌ QA FAILED - retain for debugging and resume capability

---

## Complete Marker Lifecycle

```
Phase 0 Start
    ↓
mkdir -p {report_dir}              # Create directory if needed
    ↓
Write(.qa-phase-0.marker)          # Write Phase 0 marker
    ↓
Phase 1 Pre-flight
    ↓
Glob(.qa-phase-0.marker)           # Verify Phase 0 complete
    ↓
[Phase 1 execution]
    ↓
Write(.qa-phase-1.marker)          # Write Phase 1 marker
    ↓
... [Phases 2-3 follow same pattern]
    ↓
Phase 4 Complete
    ↓
IF PASSED: rm .qa-phase-*.marker   # Cleanup on success
IF FAILED: retain markers           # Keep for debugging
```

---

## Marker File Format

```yaml
# .qa-phase-{N}.marker
phase: {N}                    # Phase number (0-4)
story_id: {STORY_ID}          # Story identifier
mode: {MODE}                  # light or deep
timestamp: {ISO_8601}         # When phase completed
status: complete              # Always "complete" when written
```

**Example:**
```yaml
phase: 2
story_id: STORY-193
mode: deep
timestamp: 2026-01-08T14:30:00Z
status: complete
```

---

## Error Handling

**Directory Missing:**
```
IF Write fails with "directory not found":
    Bash(command="mkdir -p devforgeai/qa/reports/{STORY_ID}")
    Retry Write operation
```

**Permission Denied:**
```
IF Write fails with "permission denied":
    Display: "❌ Cannot write marker - check directory permissions"
    HALT: "Fix permissions on devforgeai/qa/reports/"
```

**Stale Marker Detected:**
```
IF marker timestamp > 1 hour old:
    Display: "⚠️ Stale marker detected - previous run may have failed"
    AskUserQuestion: "Remove stale marker and restart?"
```

---

## Cross-References

- **Phase Marker Protocol:** `.claude/skills/devforgeai-qa/SKILL.md` (lines 302-340)
- **Test Isolation Service:** `references/test-isolation-service.md`
- **Tech Stack (Native Tools):** `devforgeai/specs/context/tech-stack.md` (lines 198-211)
- **Session Checkpoint:** `references/deep-validation-workflow.md`

---

## Change Log

| Date | Author | Change |
|------|--------|--------|
| 2026-01-08 | claude/backend-architect | Initial creation (STORY-193) |
