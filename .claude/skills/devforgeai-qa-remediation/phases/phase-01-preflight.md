# Phase 01: Pre-Flight Validation

**Purpose:** Validate environment, load configuration, and verify prerequisites.

---

## Flag Dependency Validation (STORY-290)

When `--add-to-debt` or `--create-stories` flags are used, validate prerequisites exist.

### Step 1.A: Validate --add-to-debt Prerequisites

```
IF $ADD_TO_DEBT == true:
    # Check STORY-287 (QA Hook Integration) completion
    # STORY-287 provides infrastructure for debt register updates

    marker_path = "src/claude/skills/devforgeai-qa-remediation/markers/STORY-287-complete.md"

    IF not file_exists(marker_path):
        Display:
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        "❌ PREREQUISITE CHECK FAILED"
        ""
        "Flag --add-to-debt requires STORY-287 (QA Hook Integration) - not yet implemented"
        ""
        "STORY-287 provides the infrastructure for:"
        "  - Post-QA technical debt detection hook"
        "  - Automated debt register updates"
        ""
        "Options:"
        "  1. Run /dev STORY-287 first"
        "  2. Remove --add-to-debt flag and use manual debt addition"
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

        HALT with exit code 1
```

### Step 1.B: Validate --create-stories Prerequisites

```
IF $CREATE_STORIES == true:
    # Check STORY-288 (Remediation Story Automation) completion
    # STORY-288 provides infrastructure for batch story creation

    marker_path = "src/claude/skills/devforgeai-qa-remediation/markers/STORY-288-complete.md"

    IF not file_exists(marker_path):
        Display:
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        "❌ PREREQUISITE CHECK FAILED"
        ""
        "Flag --create-stories requires STORY-288 (Remediation Story Automation) - not yet implemented"
        ""
        "STORY-288 provides the infrastructure for:"
        "  - Batch remediation story creation"
        "  - Automated story generation from gaps"
        ""
        "Options:"
        "  1. Run /dev STORY-288 first"
        "  2. Remove --create-stories flag and use manual story creation"
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

        HALT with exit code 1
```

### Validation Summary

| Flag | Prerequisite | Marker File |
|------|--------------|-------------|
| `--add-to-debt` | STORY-287 | `markers/STORY-287-complete.md` |
| `--create-stories` | STORY-288 | `markers/STORY-288-complete.md` |

**Error Message Pattern:**
```
"Flag {flag} requires {STORY_ID} ({description}) - not yet implemented"
```

---

## Standard Pre-Flight Steps

Continue with standard Phase 01 validation from SKILL.md:

1. Validate Project Root (Step 1.1)
2. Load Configuration (Step 1.2)
3. Parse Arguments (Step 1.3) - includes ADD_TO_DEBT and CREATE_STORIES
4. Validate Source Paths (Step 1.4)
5. Create Progress Tracker (Step 1.5)

---

## Phase 01 Output

| Variable | Value |
|----------|-------|
| `$CONFIG` | Loaded configuration object |
| `$SOURCE` | Gap file source (local/imports/all) |
| `$MIN_SEVERITY` | Severity filter threshold |
| `$EPIC_ID` | Epic to associate (or null) |
| `$DRY_RUN` | Preview mode flag |
| `$ADD_TO_DEBT` | Auto-add to debt register flag |
| `$CREATE_STORIES` | Auto-create stories flag |
| `$GAP_PATHS` | Array of glob patterns to search |
