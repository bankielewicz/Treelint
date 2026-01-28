# Technical Debt Type Derivation Workflow

**Skill Reference:** devforgeai-development
**Parent Workflow:** technical-debt-register-workflow.md
**Section:** 6.6.4 (Derive Entry Fields)

---

## Purpose

Derive technical debt entry type from justification text using pattern matching. Classifies deferrals into one of 4 categories: Story Split, Scope Change, External Blocker, or default.

**Token Cost:** ~400-600 tokens (extracted from main workflow for progressive disclosure)

---

## Debt Type Categories

| Type | Pattern | Follow-up Format |
|------|---------|------------------|
| Story Split | "Deferred to STORY-" | STORY-XXX |
| Scope Change | "Out of scope: ADR-" | ADR-XXX |
| External Blocker | "Blocked by:" | TBD |
| Default | (no pattern match) | TBD |

---

## Type Derivation Algorithm (BR-004)

```
# Input: deferral.justification (user-provided text)
justification = deferral.justification

# Pattern matching for debt type classification
IF "Deferred to STORY-" in justification:
    type = "Story Split"
    follow_up = extract_pattern("STORY-[0-9]+", justification)

ELIF "Out of scope: ADR-" in justification:
    type = "Scope Change"
    follow_up = extract_pattern("ADR-[0-9]+", justification)

ELIF "Blocked by:" in justification:
    type = "External Blocker"
    follow_up = "TBD"  # May need follow-up story created

ELSE:
    type = "External Blocker"  # Default (BR-004)
    follow_up = "TBD"

Display: "  Type: {type}"
Display: "  Follow-up: {follow_up}"
```

---

## Priority and Effort Derivation

```
# Priority: Inherit from story frontmatter or default to Medium
story_priority = Grep(pattern="^priority:", path="${STORY_FILE}", output_mode="content")
priority = story_priority.split(":")[1].strip() if story_priority else "Medium"

# Effort: From story points or TBD
story_points = Grep(pattern="^points:", path="${STORY_FILE}", output_mode="content")
effort = story_points.split(":")[1].strip() + " points" if story_points else "TBD"

Display: "  Priority: {priority}"
```

---

## Field Summary

| Field | Source | Default |
|-------|--------|---------|
| Type | Pattern matching on justification | External Blocker |
| Priority | Story frontmatter `priority:` | Medium |
| Status | Constant | Open |
| Effort | Story frontmatter `points:` | TBD |
| Follow-up | Extracted from justification | TBD |
| Source | Constant (BR-002) | dev_phase_06 |
| Date | Current date | ISO format |

---

## Integration

**Called by:** technical-debt-register-workflow.md (Step 6.6.4)
**Returns:** type, priority, effort, follow_up, source, status, current_date

---

## Change Log

| Date | Change | Reference |
|------|--------|-----------|
| 2026-01-24 | Extracted from technical-debt-register-workflow.md | STORY-305 |
