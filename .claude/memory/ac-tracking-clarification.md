# Acceptance Criteria vs. Tracking Mechanisms (RCA-012)

**Purpose:** Clarify the distinction between AC definitions and progress tracking.
**Source:** RCA-012 remediation (2025-01-21)

---

## Three Tracking Mechanisms

| Element | Purpose | Checkbox Behavior | Updated When | Source of Truth |
|---------|---------|-------------------|--------------|-----------------|
| **AC Headers** (e.g., `### AC#1: Title`) | **Define what to test** (immutable specification) | **Never marked** (no checkboxes as of v2.1) | Never (definitions are static) | Story creation |
| **AC Verification Checklist** | **Track granular progress** (real-time sub-items) | Marked `[x]` during TDD phases | End of each TDD phase (02-08) | TDD execution |
| **Definition of Done** | **Official completion record** (quality gate) | Marked `[x]` in Phase 07 | After deferrals validated, before commit | Quality gate validation |

---

## Template Evolution

- **v1.0 stories:** AC headers have `### 1. [ ]` checkbox syntax (vestigial)
- **v2.0 stories:** AC headers have `### 1. [ ]` checkbox syntax (vestigial)
- **v2.1 stories:** AC headers have `### AC#1:` format (no checkboxes) ← CURRENT

---

## How to Determine Story Completion

**Single Source of Truth: Definition of Done Section**

Decision Tree:
```
Want to know if story is complete?
  ↓
Check DoD section
  ├─ All items [x]? → Story 100% complete ✅
  └─ Some items [ ]?
      ↓
      Check for "Approved Deferrals" section
        ├─ Section exists with user approval timestamp?
        │   → Story complete with documented deferrals ✅
        └─ Section missing?
            → Story incomplete (quality gate violation) ❌
```

---

## Quality Gate Rule

QA validation enforces (Phase 0.9):

1. **100% AC-to-DoD traceability** - Every AC requirement must have corresponding DoD item
2. **Documented deferrals** - Any unchecked DoD item requires "Approved Deferrals" section with:
   - User approval timestamp (e.g., "2025-01-21 10:30 UTC")
   - Blocker justification (Dependency, Toolchain, Artifact, ADR, Low-Priority)
   - Follow-up reference (story ID or completion condition)

---

## Migration (Optional)

To update old stories (v2.0) to new format (v2.1):
```bash
bash .claude/skills/devforgeai-story-creation/scripts/migrate-ac-headers.sh <story-file>
```

---

## See Also

- `.claude/skills/devforgeai-qa/references/traceability-validation-algorithm.md`
- `devforgeai/RCA/RCA-012/MIGRATION-SCRIPT.md`
