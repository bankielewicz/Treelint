---
name: review-qa-reports
description: Process QA gap files and create remediation stories. Scans local QA reports or imported external project reports, aggregates gaps by severity, and converts selected gaps into actionable user stories.
argument-hint: "[--source local|imports|all] [--min-severity CRITICAL|HIGH|MEDIUM|LOW] [--epic EPIC-XXX] [--dry-run] [--add-to-debt] [--create-stories]"
---

# /review-qa-reports

Process QA gap files and create remediation user stories from findings.

## Purpose

Convert QA gap findings (coverage gaps, anti-pattern violations, code quality issues, deferral problems) into actionable user stories for systematic technical debt elimination.

## Arguments

| Argument | Values | Default | Description |
|----------|--------|---------|-------------|
| `--source` | `local`, `imports`, `all` | `local` | Gap file source location |
| `--min-severity` | `CRITICAL`, `HIGH`, `MEDIUM`, `LOW` | `MEDIUM` | Filter threshold (gaps below this are deferred) |
| `--epic` | `EPIC-XXX` | null | Associate created stories with this epic |
| `--dry-run` | flag | false | Preview gaps without creating stories |
| `--add-to-debt` | boolean flag | false | Automatically add selected gaps to technical debt register without confirmation |
| `--create-stories` | boolean flag | false | Automatically create remediation stories for selected gaps without confirmation |

## Usage Examples

```bash
# Review local QA reports (default)
/review-qa-reports

# Review only imported reports from external projects
/review-qa-reports --source imports

# Review all sources (local + imports)
/review-qa-reports --source all

# Filter by severity (only HIGH and CRITICAL)
/review-qa-reports --min-severity HIGH

# Associate with an epic
/review-qa-reports --epic EPIC-025

# Preview mode (no story creation)
/review-qa-reports --dry-run

# Combined: imports only, high severity, with epic
/review-qa-reports --source imports --min-severity HIGH --epic EPIC-025

# Auto-add selected gaps to debt register (no confirmation prompt)
/review-qa-reports --add-to-debt

# Auto-create remediation stories (no confirmation prompt)
/review-qa-reports --create-stories

# Combined: create stories AND add to debt register (stories first, then debt with back-links)
/review-qa-reports --add-to-debt --create-stories
```

## Workflow

This command invokes `devforgeai-qa-remediation` skill with the following phases:

1. **Phase 01: Pre-Flight Validation** - Validate CWD, load config
2. **Phase 02: Discovery & Parsing** - Glob and parse gap files
3. **Phase 03: Aggregation & Prioritization** - Deduplicate, score, filter
4. **Phase 04: Interactive Selection** - Display summary, user selects gaps
5. **Phase 05: Batch Story Creation** - Create stories via batch mode
6. **Phase 06: Source Report Update** - Link stories to gap files
7. **Phase 07: Technical Debt Integration** - Update debt register with skipped gaps

## Execution

Parse $ARGUMENTS and invoke the skill:

```
**Arguments:** $ARGUMENTS

Skill(command="devforgeai-qa-remediation", args="$ARGUMENTS")
```

### Argument Parsing

Extract from $ARGUMENTS:

1. **--source**: Default `local` if not specified
2. **--min-severity**: Default `MEDIUM` if not specified
3. **--epic**: Optional, pass to skill if present
4. **--dry-run**: Boolean flag, default false
5. **--add-to-debt**: Boolean flag, default false. When true, skips confirmation prompt in Phase 07 and auto-adds ALL selected gaps to debt register. Sets add_to_debt=true in skill context.
6. **--create-stories**: Boolean flag, default false. When true, skips confirmation prompt in Phase 05 and auto-creates remediation stories. Sets create_stories=true in skill context.

### Source Paths

| Source | Path |
|--------|------|
| `local` | `devforgeai/qa/reports/*-gaps.json` |
| `imports` | `devforgeai/qa/imports/**/*-gaps.json` |
| `all` | Both paths combined |

## Output

### Dry-Run Mode

Displays gap summary table without creating stories:

```
┌────────────────────────────────────────────────────────────────┐
│                    QA Gap Summary (Dry Run)                    │
├──────┬──────────────┬──────────────┬──────────┬───────────────┤
│ # │ Gap Type │ File │ Severity │ Score │
├──────┼──────────────┼──────────────┼──────────┼───────────────┤
│ 1 │ coverage_gap │ indexer.py │ HIGH │ 75 │
│ 2 │ anti_pattern │ service.py │ CRITICAL │ 100 │
│ ... │ ... │ ... │ ... │ ... │
└──────┴──────────────┴──────────────┴──────────┴───────────────┘

Total Gaps Found: X
Gaps Above Threshold: Y
Stories Would Be Created: Z
```

### Full Run Mode

After story creation:

```
┌────────────────────────────────────────────────────────────────┐
│               QA Gap Remediation Complete                      │
├────────────────────────────────────────────────────────────────┤
│ Gap Files Processed: X │
│ Total Gaps Found: Y │
│ Gaps Selected: Z │
│ Stories Created: A (see devforgeai/specs/Stories/) │
│ Stories Failed: B │
│ Gaps Deferred to Debt: C │
├────────────────────────────────────────────────────────────────┤
│ Enhancement Report: devforgeai/qa/enhancement-reports/{date}.md│
└────────────────────────────────────────────────────────────────┘
```

## Related Components

| Component | Purpose |
|-----------|---------|
| `devforgeai-qa-remediation` skill | Main processing workflow |
| `devforgeai-story-creation` skill | Batch story creation |
| `technical-debt-analyzer` subagent | Optional debt analysis |
| `devforgeai/qa/reports/` | Local gap file source |
| `devforgeai/qa/imports/` | External project imports |
| `devforgeai/technical-debt-register.md` | Deferred gap tracking |

## Configuration

Settings in `devforgeai/config/qa-remediation.yaml`:
- Severity weights
- Default values
- Points estimation
- Output paths
- Technical debt integration

## See Also

- `/create-stories-from-rca` - Similar workflow for RCA recommendations
- `/qa` - QA skill that generates gap files
- `/audit-deferrals` - Audit deferred DoD items
