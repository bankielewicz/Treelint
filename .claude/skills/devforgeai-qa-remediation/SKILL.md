---
name: devforgeai-qa-remediation
description: Process QA gap files and create remediation user stories. Use when converting QA findings into actionable development work. Invoked by /review-qa-reports command.
tools: Read, Write, Edit, Glob, Grep, AskUserQuestion
model: inherit
---

# devforgeai-qa-remediation

Process QA gap files and create remediation user stories from findings.

## Purpose

Convert QA gap findings (coverage gaps, anti-pattern violations, code quality issues, deferral problems) into actionable user stories, eliminating technical debt through structured remediation.

## When to Use

- After QA validation produces gap files (`*-gaps.json`)
- When importing QA reports from external projects
- When systematically addressing accumulated technical debt
- When gaps need to be converted to trackable stories

## Entry Point

This skill is invoked by `/review-qa-reports` command with arguments:

```
$ARGUMENTS:
  --source local|imports|all      # Gap file source
  --min-severity CRITICAL|HIGH|MEDIUM|LOW  # Filter threshold
  --epic EPIC-XXX                 # Optional epic association
  --dry-run                       # Preview mode
  --add-to-debt                   # Auto-add gaps to debt register (no confirmation)
  --create-stories                # Auto-create remediation stories (no confirmation)
```

**Flag Routing Context:**
When flags are parsed, they set context variables:
- `add_to_debt=true` → Phase 07 skips confirmation prompt
- `create_stories=true` → Phase 05 uses batch mode without prompts

**Combined Flag Operation (--add-to-debt --create-stories):**
When BOTH flags are present, stories are created first, then debt entries are added:
1. Phase 05 executes before Phase 07 (stories created first)
2. Phase 07 executes SECOND (gaps added to debt register)
3. Debt entries have Follow-up field pre-populated with created STORY-XXX IDs
4. Summary displays: "Created N remediation stories AND added N gaps to debt register with back-links"

---

## Phase 01: Pre-Flight Validation

**Purpose:** Validate environment and load configuration.

### Step 1.1: Validate Project Root

```
Read(file_path="CLAUDE.md")
```

**HALT if:** File not found or doesn't contain "DevForgeAI"
**Action:** "Navigate to project root and retry"

### Step 1.2: Load Configuration

```
Read(file_path="devforgeai/config/qa-remediation.yaml")
```

**HALT if:** Config file missing
**Action:** "Run CP-1 from plan to create config"

Extract configuration values:
- `sources.local` → local gap path
- `sources.imports` → imports gap path
- `defaults.min_severity` → default severity threshold
- `defaults.sprint` → default sprint for stories
- `technical_debt.register_path` → debt register location

### Step 1.3: Parse Arguments

From $ARGUMENTS, extract:

| Argument | Variable | Default |
|----------|----------|---------|
| `--source` | `$SOURCE` | `local` |
| `--min-severity` | `$MIN_SEVERITY` | From config |
| `--epic` | `$EPIC_ID` | null |
| `--dry-run` | `$DRY_RUN` | false |
| `--add-to-debt` | `$ADD_TO_DEBT` | false |
| `--create-stories` | `$CREATE_STORIES` | false |

### Step 1.4: Validate Source Paths

Based on `$SOURCE`:

| Source Value | Path to Check |
|--------------|---------------|
| `local` | `devforgeai/qa/reports/` |
| `imports` | `devforgeai/qa/imports/` |
| `all` | Both paths |

```
Glob(pattern="{source_path}/*-gaps.json")
```

**HALT if:** No gap files found in specified source
**Action:** "No gap files found. Generate with /qa or import files."

### Step 1.5: Create Progress Tracker

```
TodoWrite with phases:
1. Phase 02: Discovery & Parsing
2. Phase 03: Aggregation & Prioritization
3. Phase 04: Interactive Selection
4. Phase 05: Batch Story Creation
5. Phase 06: Source Report Update
6. Phase 07: Technical Debt Integration
```

### Phase 01 Output

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

---

## Phase 02: Discovery & Parsing

**Purpose:** Find and parse all gap files from specified sources.

**Reference:** For detailed parsing algorithm, see `references/gap-discovery-workflow.md`

### Step 2.1: Glob Gap Files

Based on `$SOURCE`:

```
# For local
Glob(pattern="devforgeai/qa/reports/*-gaps.json")

# For imports (recursive)
Glob(pattern="devforgeai/qa/imports/**/*-gaps.json")
```

Store results in `$GAP_FILES[]` array.

### Step 2.2: Parse Each Gap File

For each file in `$GAP_FILES`:

```
Read(file_path="{gap_file}")
```

Parse JSON and extract:
- `story_id` → Source story identifier
- `qa_result` → PASSED or FAILED
- `qa_date` → When QA ran
- `coverage_gaps[]` → Coverage gap entries
- `anti_pattern_violations[]` → Anti-pattern entries
- `code_quality_violations[]` → Quality metric entries
- `deferral_issues[]` → Deferral problem entries

### Step 2.3: Validate Gap Structure

For each gap entry, verify required fields:

**coverage_gaps:**
- `file` (string) - Required
- `layer` (string) - Required
- `current_coverage` (number) - Required
- `target_coverage` (number) - Required

**anti_pattern_violations:**
- `file` (string) - Required
- `type` (string) - Required
- `severity` (string) - Required

**code_quality_violations:**
- `file` (string) - Required
- `metric` (string) - Required
- `severity` (string) - Required

**deferral_issues:**
- `item` (string) - Required
- `severity` (string) - Required

### Step 2.4: Build Unified Gap List

Create `$ALL_GAPS[]` array with normalized entries:

```json
{
  "id": "GAP-{sequence}",
  "source_file": "{gap_file_path}",
  "source_story": "{story_id}",
  "type": "coverage_gap|anti_pattern|code_quality|deferral",
  "file": "{affected_file}",
  "severity": "CRITICAL|HIGH|MEDIUM|LOW",
  "description": "{gap_description}",
  "details": { /* original gap entry */ }
}
```

### Phase 02 Output

| Variable | Value |
|----------|-------|
| `$GAP_FILES` | Array of gap file paths processed |
| `$ALL_GAPS` | Unified array of normalized gap entries |
| `$FILES_PROCESSED` | Count of gap files read |
| `$TOTAL_GAPS` | Count of all gaps found |

---

## Phase 03: Aggregation & Prioritization

**Purpose:** Deduplicate, score, and filter gaps for processing.

**Reference:** For detailed algorithm, see `references/gap-aggregation-algorithm.md`

### Step 3.1: Deduplicate Gaps

Identify duplicates by matching:
- Same `file` + Same `type` + Similar `description`

For duplicates:
- Keep highest severity entry
- Merge source references
- Increment occurrence count

Store in `$UNIQUE_GAPS[]`.

### Step 3.2: Calculate Priority Scores

For each gap in `$UNIQUE_GAPS`, calculate score:

```
Severity weights (from config):
  CRITICAL: 100
  HIGH: 75
  MEDIUM: 50
  LOW: 25

Gap type modifiers:
  deferral_issue: +25 (RCA compliance priority)
  coverage_gap (Business Logic): +10
  anti_pattern (security-related): +15
```

Formula: `score = severity_weight + type_modifier`

### Step 3.3: Filter by Severity Threshold

Based on `$MIN_SEVERITY`, filter gaps:

| $MIN_SEVERITY | Keep Severities |
|---------------|-----------------|
| `CRITICAL` | CRITICAL only |
| `HIGH` | CRITICAL, HIGH |
| `MEDIUM` | CRITICAL, HIGH, MEDIUM |
| `LOW` | All severities |

Store passing gaps in `$FILTERED_GAPS[]`.
Store filtered-out gaps in `$DEFERRED_GAPS[]` (for Phase 07).

### Step 3.4: Sort by Score

Sort `$FILTERED_GAPS` descending by score.

### Phase 03 Output

| Variable | Value |
|----------|-------|
| `$UNIQUE_GAPS` | Deduplicated gap array |
| `$FILTERED_GAPS` | Gaps above severity threshold (sorted) |
| `$DEFERRED_GAPS` | Gaps below threshold (for debt register) |
| `$GAPS_DEDUPLICATED` | Count of duplicates removed |
| `$GAPS_ABOVE_THRESHOLD` | Count passing filter |

---

## Phase 04: Interactive Selection

**Purpose:** Display gap summary and let user select which gaps to convert to stories.

### Step 4.1: Display Gap Summary Table

Format and display:

```
┌────────────────────────────────────────────────────────────────────────────┐
│                         QA Gap Summary                                      │
├────┬────────────────┬────────────────────────────┬──────────┬──────┬───────┤
│ #  │ Type           │ File                       │ Severity │ Score│ Source│
├────┼────────────────┼────────────────────────────┼──────────┼──────┼───────┤
│ 1  │ anti_pattern   │ src/service.py             │ CRITICAL │ 100  │ S-035 │
│ 2  │ coverage_gap   │ src/indexer.py             │ HIGH     │ 85   │ S-078 │
│ 3  │ code_quality   │ src/processor.py           │ HIGH     │ 75   │ S-082 │
│ ...│ ...            │ ...                        │ ...      │ ...  │ ...   │
└────┴────────────────┴────────────────────────────┴──────────┴──────┴───────┘

Summary:
  Gap Files Processed: X
  Total Gaps Found: Y
  After Deduplication: Z
  Above Severity Threshold: A
  Below Threshold (Deferred): B
```

### Step 4.2: Handle Dry-Run Mode

**If `$DRY_RUN` is true:**

Display summary only, then return:

```
Dry-Run Complete:
  Would create X stories
  Would defer Y gaps to technical debt register

To proceed with story creation, run without --dry-run
```

**EXIT skill - no further phases executed in dry-run mode.**

### Step 4.3: Interactive Selection

**If `$DRY_RUN` is false:**

Use AskUserQuestion for multi-select:

```
AskUserQuestion:
  question: "Select gaps to convert to stories:"
  header: "Gap Selection"
  options:
    - label: "All gaps above threshold (A items)"
      description: "Create stories for all filtered gaps"
    - label: "CRITICAL only (X items)"
      description: "Only CRITICAL severity gaps"
    - label: "CRITICAL + HIGH only (Y items)"
      description: "CRITICAL and HIGH severity gaps"
    - label: "None - cancel"
      description: "Exit without creating stories"
  multiSelect: false
```

### Step 4.4: Process Selection

Based on user selection:

| Selection | Action |
|-----------|--------|
| "All gaps above threshold" | `$SELECTED_GAPS = $FILTERED_GAPS` |
| "CRITICAL only" | Filter to CRITICAL |
| "CRITICAL + HIGH only" | Filter to CRITICAL, HIGH |
| "None - cancel" | EXIT skill with message |

### Phase 04 Output

| Variable | Value |
|----------|-------|
| `$SELECTED_GAPS` | Gaps chosen for story creation |
| `$SELECTION_COUNT` | Count of selected gaps |

---

## Phase 05: Batch Story Creation

**Purpose:** Convert selected gaps to user stories via batch mode.

**Reference:** For mapping rules, see `references/gap-to-story-mapping.md`

### Step 5.1: Initialize Tracking

```
$CREATED_STORIES = []
$FAILED_STORIES = []
$BATCH_INDEX = 0
```

### Step 5.2: Get Next Story ID

Query existing stories to find next available ID:

```
Glob(pattern="devforgeai/specs/Stories/STORY-*.story.md")
```

Extract IDs, find highest, increment. Account for gaps in sequence.

### Step 5.3: For Each Selected Gap

Loop through `$SELECTED_GAPS`:

#### 5.3.1: Generate Story Context Markers

Map gap to story markers:

```markdown
**Story ID:** STORY-{next_id}
**Epic ID:** {$EPIC_ID or null}
**Feature Name:** {generated_from_gap}
**Feature Description:** {detailed_description}
**Priority:** {mapped_from_severity}
**Points:** {estimated_from_gap_size}
**Type:** {refactor or bugfix}
**Sprint:** Backlog
**Batch Mode:** true
**Source Report:** {gap_source_file}
**Source Gap Type:** {gap_type}
**Source Gap Index:** {index_in_array}
```

**Feature Name Generation:**

| Gap Type | Template |
|----------|----------|
| coverage_gap | "Improve test coverage for {layer} layer in {file}" |
| anti_pattern | "Fix {violation_type} violation in {file}" |
| code_quality | "Reduce {metric} in {file}" |
| deferral | "Resolve deferred DoD item: {item_description}" |

**Priority Mapping:**

| Gap Severity/Layer | Story Priority |
|--------------------|----------------|
| CRITICAL | High |
| HIGH | High |
| Business Logic coverage | High |
| MEDIUM | Medium |
| Application coverage | Medium |
| LOW | Low |
| Infrastructure coverage | Low |

**Points Estimation:**

| Gap Type | Condition | Points |
|----------|-----------|--------|
| coverage_gap | gap < 5% | 2 |
| coverage_gap | gap 5-15% | 3 |
| coverage_gap | gap > 15% | 5 |
| anti_pattern | CRITICAL | 5 |
| anti_pattern | HIGH | 3 |
| anti_pattern | MEDIUM | 2 |
| anti_pattern | LOW | 1 |
| code_quality | HIGH | 3 |
| code_quality | MEDIUM | 2 |
| deferral | any | 3 |

#### 5.3.2: Invoke Story Creation

```
Skill(command="devforgeai-story-creation")
```

With context markers in conversation.

**On Success:**
- Add to `$CREATED_STORIES[]` with story_id and gap_id
- Increment `$BATCH_INDEX`

**On Failure:**
- Add to `$FAILED_STORIES[]` with gap_id and error
- Log error
- Continue to next gap (failure isolation)

### Step 5.4: Batch Completion Summary

After all gaps processed:

```
Batch Story Creation Complete:
  Stories Created: {count of $CREATED_STORIES}
  Stories Failed: {count of $FAILED_STORIES}

Created Stories:
  - STORY-XXX: {gap description}
  - STORY-YYY: {gap description}
  ...

Failed:
  - GAP-ZZZ: {error message}
```

### Phase 05 Output

| Variable | Value |
|----------|-------|
| `$CREATED_STORIES` | Array of {story_id, gap_id, gap_type} |
| `$FAILED_STORIES` | Array of {gap_id, error} |
| `$STORIES_CREATED_COUNT` | Count of successful creations |

---

## Phase 06: Source Report Update

**Purpose:** Update gap files with story references and generate enhancement report.

**Reference:** For update protocol, see `references/report-update-protocol.md`

### Step 6.1: Update Gap Files

For each entry in `$CREATED_STORIES`:

1. Read the source gap file
2. Find the gap entry by type and index
3. Add `"implemented_in": "STORY-XXX"` field
4. Write updated JSON

```
Read(file_path="{gap_source_file}")
// Parse JSON
// Add implemented_in field to matching gap entry
Edit(file_path="{gap_source_file}", old_string=..., new_string=...)
```

**Note:** Only update LOCAL gap files. Imported files are read-only.

### Step 6.2: Generate Enhancement Report

Create report at:
```
devforgeai/qa/enhancement-reports/{YYYY-MM-DD}-enhancement-report.md
```

**Report Template:**

```markdown
# QA Enhancement Report - {YYYY-MM-DD}

## Summary

| Metric | Value |
|--------|-------|
| Gap Files Processed | {$FILES_PROCESSED} |
| Total Gaps Found | {$TOTAL_GAPS} |
| Gaps Selected | {$SELECTION_COUNT} |
| Stories Created | {$STORIES_CREATED_COUNT} |
| Stories Failed | {count of $FAILED_STORIES} |
| Gaps Deferred (Added to Debt) | {count of $DEFERRED_GAPS} |

## Created Stories

| Story ID | Gap Type | Source File | Priority | Points |
|----------|----------|-------------|----------|--------|
{for each $CREATED_STORIES}
| {story_id} | {gap_type} | {file} | {priority} | {points} |
{end for}

## Failed Story Creations

{if $FAILED_STORIES not empty}
| Gap | Error | Remediation |
|-----|-------|-------------|
{for each $FAILED_STORIES}
| {gap_id} | {error} | Retry or manual creation |
{end for}
{else}
No failures.
{end if}

## Deferred to Technical Debt

| Gap ID | Severity | Reason |
|--------|----------|--------|
{for each $DEFERRED_GAPS}
| {id} | {severity} | Below --min-severity threshold |
{end for}

## Source Reports Updated

| Report | Gaps Linked | Status |
|--------|-------------|--------|
{for each unique source file}
| {filename} | {count} | Updated |
{end for}

## Next Steps

- [ ] Review created stories in `devforgeai/specs/Stories/`
- [ ] Assign stories to sprint if appropriate
- [ ] Review deferred gaps in `devforgeai/technical-debt-register.md`
```

Write report:
```
Write(file_path="devforgeai/qa/enhancement-reports/{date}-enhancement-report.md", content=...)
```

### Phase 06 Output

| Variable | Value |
|----------|-------|
| `$REPORTS_UPDATED` | Count of gap files updated |
| `$ENHANCEMENT_REPORT_PATH` | Path to generated report |

---

## Phase 07: Technical Debt Integration

**Purpose:** Add skipped/deferred gaps to technical debt register.

**Reference:** For register format, see `references/technical-debt-update.md`

### Step 7.1: Check Config

Read `technical_debt.auto_add_skipped` from config.

**If false:** Skip this phase, return summary.

### Step 7.2: Read Current Register

```
Read(file_path="devforgeai/technical-debt-register.md")
```

Parse current structure to find "## Open Debt Items" section.

### Step 7.3: Generate Debt Entries

For each gap in `$DEFERRED_GAPS`:

```markdown
---
#### [GAP-{id}]: {description}

**Date Added:** {YYYY-MM-DD}
**Type:** QA Gap ({gap_type})
**Source Report:** {source_file}
**Original Severity:** {severity}
**Skipped Reason:** Below min-severity threshold (--min-severity {$MIN_SEVERITY})
**Follow-up:** Run `/review-qa-reports --min-severity LOW` to include
**Priority:** {mapped_priority}
**Status:** Open
**Resolution Target:** Next debt reduction sprint
**Estimated Effort:** {points} points

**Gap Details:**
- File: {file_path}
- Layer: {layer or N/A}
- Current: {current_value}
- Target: {target_value}

---
```

### Step 7.4: Append to Register

```
Edit(file_path="devforgeai/technical-debt-register.md",
     old_string="## Open Debt Items\n\n",
     new_string="## Open Debt Items\n\n{new_entries}\n\n")
```

### Step 7.5: Update Analysis Section

Update statistics in register:

```
Edit(file_path="devforgeai/technical-debt-register.md",
     old_string="**Total Open Items:** {old_count}",
     new_string="**Total Open Items:** {new_count}")
```

### Step 7.6: Optional Analyzer Invocation

If `technical_debt.invoke_analyzer` is true:

```
Task(subagent_type="technical-debt-analyzer",
     prompt="Analyze technical debt register and generate trend report")
```

### Phase 07 Output

| Variable | Value |
|----------|-------|
| `$DEBT_ENTRIES_ADDED` | Count of entries added to register |
| `$REGISTER_UPDATED` | Boolean success flag |

---

## Final Summary

After all phases complete, display:

```
┌────────────────────────────────────────────────────────────────────────────┐
│                    QA Gap Remediation Complete                              │
├────────────────────────────────────────────────────────────────────────────┤
│ Gap Files Processed:     {$FILES_PROCESSED}                                 │
│ Total Gaps Found:        {$TOTAL_GAPS}                                      │
│ Gaps Selected:           {$SELECTION_COUNT}                                 │
│ Stories Created:         {$STORIES_CREATED_COUNT}                           │
│ Stories Failed:          {count of $FAILED_STORIES}                         │
│ Gaps Deferred to Debt:   {$DEBT_ENTRIES_ADDED}                              │
├────────────────────────────────────────────────────────────────────────────┤
│ Enhancement Report: {$ENHANCEMENT_REPORT_PATH}                              │
│ Technical Debt Register: devforgeai/technical-debt-register.md              │
└────────────────────────────────────────────────────────────────────────────┘

Next Steps:
1. Review created stories in devforgeai/specs/Stories/
2. Assign stories to sprint via /create-sprint
3. Review deferred gaps in technical-debt-register.md
4. Re-run with --min-severity LOW to address deferred gaps
```

---

## Success Criteria

| Criterion | Validation |
|-----------|------------|
| Gap files discovered | `$FILES_PROCESSED > 0` |
| Gaps parsed | `$TOTAL_GAPS > 0` |
| Selection completed | User made selection or dry-run completed |
| Stories created | `$STORIES_CREATED_COUNT >= 0` |
| Source reports updated | Local files have `implemented_in` |
| Debt register updated | Deferred gaps added (if any) |
| Enhancement report generated | File exists at expected path |

---

## Error Handling

### HALT Conditions

| Condition | Action |
|-----------|--------|
| Project root invalid | HALT with navigation instructions |
| Config file missing | HALT with creation instructions |
| No gap files found | HALT with generation/import guidance |
| User cancels selection | EXIT gracefully with message |

### Continue Conditions

| Condition | Action |
|-----------|--------|
| Individual story creation fails | Log error, continue to next gap |
| Gap file update fails (imports) | Skip update (imports are read-only) |
| Debt register update fails | Log warning, continue with summary |

---

## Reference Files

| Reference | Purpose |
|-----------|---------|
| `references/gap-discovery-workflow.md` | Phase 02 detailed parsing |
| `references/gap-aggregation-algorithm.md` | Phase 03 scoring and filtering |
| `references/gap-to-story-mapping.md` | Phase 05 context marker generation |
| `references/report-update-protocol.md` | Phase 06 update procedures |
| `references/technical-debt-update.md` | Phase 07 register format |

---

## Related Components

| Component | Relationship |
|-----------|--------------|
| `/review-qa-reports` command | Invokes this skill |
| `devforgeai-story-creation` skill | Batch mode story creation |
| `devforgeai-qa` skill | Produces gap files |
| `technical-debt-analyzer` subagent | Optional debt analysis |
| `devforgeai/technical-debt-register.md` | Deferred gap tracking |
