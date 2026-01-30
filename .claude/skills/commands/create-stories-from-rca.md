---
name: create-stories-from-rca
description: Create user stories from RCA recommendations
argument-hint: "RCA-NNN [--help]"
model: opus
allowed-tools: Read, Write, Edit, Glob, Grep, AskUserQuestion, Skill, TodoWrite
---

# /create-stories-from-rca - Create Stories from RCA Recommendations

Parse RCA documents, select recommendations interactively, and create stories in batch mode.

**Component Orchestration:** Parse (STORY-155) → Select (STORY-156) → Create (STORY-157) → Link (STORY-158)

---

## Usage

```bash
/create-stories-from-rca RCA-NNN [--threshold HOURS]
/create-stories-from-rca --help
/create-stories-from-rca help

# Examples:
/create-stories-from-rca RCA-022
/create-stories-from-rca RCA-022 --threshold 2
```

---

## Help Text

When `--help` or `help` argument is provided:

```
/create-stories-from-rca - Create user stories from RCA recommendations

USAGE:
    /create-stories-from-rca RCA-NNN [--threshold HOURS]
    /create-stories-from-rca --help | help

ARGUMENTS:
    RCA-NNN         Required. RCA document ID (e.g., RCA-022). Case-insensitive.

OPTIONS:
    --threshold N   Filter recommendations with effort >= N hours
    --help, help    Display this help message

PROCESS:
    1. Parse RCA document and extract recommendations
    2. Filter by effort threshold and sort by priority
    3. Display summary table for interactive selection
    4. Create stories for selected recommendations
    5. Update RCA document with story links

RELATED COMMANDS:
    /rca            Create new RCA document
    /create-story   Create individual story
    /brainstorm     Transform business ideas
    /dev            Start story implementation
```

---

## Error Message Templates

```
ERROR_MISSING_RCA_ID:
    "❌ RCA ID required"
    "Usage: /create-stories-from-rca RCA-NNN"
    "Available RCAs:"

ERROR_RCA_NOT_FOUND:
    "❌ RCA not found: ${RCA_ID}"
    "Available RCAs:"

ERROR_INVALID_FORMAT:
    "❌ Invalid RCA format"
    "Expected: RCA-NNN (where NNN are 3 digits)"
```

---

## Argument Parsing

```
ARG = first argument from $ARGUMENTS

IF ARG == "--help" OR ARG == "help":
    Display: [Help Text above]
    HALT

RCA_ID = extract from arguments matching "RCA-[0-9]+" (case-insensitive)

IF RCA_ID empty:
    Display: [ERROR_MISSING_RCA_ID]
    FOR rca in Glob("devforgeai/RCA/*.md"):
        Display: "  • ${rca_id}"
    HALT

RCA_ID = uppercase(RCA_ID)  # rca-022 → RCA-022

IF Glob("devforgeai/RCA/${RCA_ID}*.md") not found:
    Display: [ERROR_RCA_NOT_FOUND]
    FOR rca in Glob("devforgeai/RCA/*.md"):
        Display: "  • ${rca_id}"
    HALT
```

---

## Phase Orchestration Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                  /create-stories-from-rca Workflow               │
│                    (4 Main Orchestration Phases)                 │
└─────────────────────────────────────────────────────────────────┘

Phase 1-5: RCA PARSING (STORY-155)
  ├─ Input: RCA ID (e.g., RCA-022)
  ├─ Process: Parse document, extract recommendations
  ├─ Output: Structured recommendation list
  └─ Reference: parsing-workflow.md

       ↓

Phase 6-9: INTERACTIVE SELECTION (STORY-156)
  ├─ Input: Parsed recommendations
  ├─ Process: Present to user, get selection
  ├─ Output: Selected recommendations
  └─ Reference: selection-workflow.md

       ↓

Phase 10: BATCH STORY CREATION (STORY-157)
  ├─ Input: Selected recommendations
  ├─ Process: Create stories with context
  ├─ Output: Array of created story IDs
  └─ Reference: batch-creation-workflow.md

       ↓

Phase 11: RCA-STORY LINKING (STORY-158)
  ├─ Input: Created story IDs
  ├─ Process: Update RCA document with links
  ├─ Output: Updated RCA with traceability
  └─ Reference: linking-workflow.md
```

| Phase | Component | Story | Role |
|-------|-----------|-------|------|
| 1-5 | RCA Parser | STORY-155 | Parse RCA and extract recommendations |
| 6-9 | Selection | STORY-156 | Interactive recommendation selection |
| 10 | Batch Creator | STORY-157 | Create stories in batch mode |
| 11 | Linker | STORY-158 | Link stories back to RCA document |

---

## Phase 1-5: RCA Parsing

**See:** `references/create-stories-from-rca/parsing-workflow.md`

1. **Locate RCA File**: `Glob(pattern="devforgeai/RCA/${RCA_ID}*.md")`
2. **Parse Frontmatter**: Extract id, title, severity, status
3. **Extract Recommendations**: Parse `### REC-N:` sections
4. **Filter/Sort**: Apply effort threshold and priority sorting
5. **Display Results**: Show recommendations with effort estimates

---

## Phase 6-9: Interactive Selection

**See:** `references/create-stories-from-rca/selection-workflow.md`

```
AskUserQuestion(
    question: "Which recommendations to convert?",
    multiSelect: true,
    options: ["All recommendations", "REC-1: Title", "None - cancel"]
)
```

---

## Phase 10: Batch Story Creation

**See:** `references/create-stories-from-rca/batch-creation-workflow.md`

```
FOR recommendation in selected:
    batch_context = {
        story_id: get_next_story_id(),
        feature_name: recommendation.title,
        priority: map_priority(recommendation.priority),
        batch_mode: true,
        source_rca: RCA_ID,
        source_recommendation: recommendation.id
    }
    Skill(command="devforgeai-story-creation", args="--batch")
```

---

## Phase 11: RCA-Story Linking

**See:** `references/create-stories-from-rca/linking-workflow.md`

1. Update implementation checklist: `- [ ] REC-1` → `- [ ] REC-1: See STORY-NNN`
2. Add inline references: `**Implemented in:** STORY-NNN`
3. Update RCA status if all recommendations linked

---

## Business Rules & Constraints

| Rule | Constraint | Implementation | Phase |
|------|-----------|-----------------|-------|
| BR-001: Effort Threshold | Filter recommendations with effort >= threshold hours | Applied in Phase 1-5 parsing | Parsing |
| BR-002: Priority Sorting | Sort recommendations by priority: CRITICAL > HIGH > MEDIUM > LOW | Applied in Phase 1-5 parsing | Parsing |
| BR-003: Story Points Mapping | 1 story point = 4 hours of effort | Used in batch creation | Phase 10 |
| BR-004: Failure Isolation | Continue processing remaining items on individual failures | Applied throughout workflow | All Phases |
| BR-005: Size Limit | Command file < 15,000 characters (lean orchestration) | Design constraint | File |
| BR-006: Case Normalization | Accept case-insensitive RCA IDs (rca-022 → RCA-022) | Applied in argument parsing | Parsing |
| BR-007: File Existence | Verify RCA file exists before processing | Check in argument parsing | Parsing |

---

## Edge Cases

| Edge Case | Behavior |
|-----------|----------|
| Missing frontmatter | Extract ID from filename |
| No recommendations | Display message, exit |
| All filtered | "No recommendations meet threshold" |
| Invalid REC ID | Log warning, ignore |

---

## Error Handling

| Error Type | Handling |
|------------|----------|
| Validation Error | Log, continue to next |
| Skill Error | Log, continue to next |
| ID Conflict | Increment, retry once |

---

## Implementation Reference Files

All detailed phase workflows are documented in dedicated reference files for maintainability and modularity.

### Phase Reference Files

| Phase | Component | File | Purpose |
|-------|-----------|------|---------|
| 1-5 | RCA Parser | `references/create-stories-from-rca/parsing-workflow.md` | RCA parsing, extraction, filtering algorithm |
| 6-9 | Selection | `references/create-stories-from-rca/selection-workflow.md` | Interactive user selection process |
| 10 | Batch Creator | `references/create-stories-from-rca/batch-creation-workflow.md` | Story batch creation and context mapping |
| 11 | Linker | `references/create-stories-from-rca/linking-workflow.md` | RCA document update and story linking |

**Note:** All reference files are located at: `.claude/commands/references/create-stories-from-rca/`
