# Coverage Quality Gate - STORY-089 AC#2

## Purpose

Coverage validation quality gate for `/orchestrate` workflow, enforced during sprint planning phase. Ensures epic-to-story traceability before stories enter sprints.

## Integration Point

**Sprint Planning Phase (Phase 3)** in devforgeai-orchestration skill.

Runs after story validation, before sprint assignment.

## Threshold Logic

| Coverage | Status | Exit Code | Action |
|----------|--------|-----------|--------|
| >= 80% | PASS (green) | 0 | Continue workflow |
| 70-80% | WARN (yellow) | 1 | Display warning, prompt to continue |
| < 70% | BLOCK (red) | 2 | Halt workflow, require remediation |

## Invocation

```bash
devforgeai/traceability/coverage-gate.sh [OPTIONS]

# Test with specific coverage
devforgeai/traceability/coverage-gate.sh --coverage 75

# Validate real files
devforgeai/traceability/coverage-gate.sh --epic-dir devforgeai/specs/Epics --story-dir devforgeai/specs/Stories

# Use custom thresholds
devforgeai/traceability/coverage-gate.sh --config thresholds.json
```

## Output Format

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Coverage Quality Gate - Sprint Planning Phase
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Epic Coverage Breakdown:
========================

  Epic ID         Title                                    Features Coverage
  -------         -----                                    -------- --------
  ✓ EPIC-015      Epic Coverage Validation & Requir...         6      83%
  ⚠ EPIC-016      Another Epic                                 4      75%
  ✗ EPIC-017      Needs Work                                   5      40%

Gate Result:
============

⚠ WARNING: Coverage 72% is below optimal (70-80%)

Workflow may proceed with warnings.
Consider addressing coverage gaps before next sprint.
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## Remediation Steps (Shown on BLOCK)

When coverage < 70%:

```
Remediation Steps:
==================

  Current coverage: 65%
  Required for pass: 80%

  Suggested actions:
    1. Run /validate-epic-coverage to identify gaps
    2. Run /create-missing-stories EPIC-ID to create stories for gaps
    3. Review orphaned stories and reassign to correct epics
    4. Consider splitting large epics into smaller ones
```

## Configuration

Thresholds loaded from: `devforgeai/traceability/thresholds.json`

```json
{
  "coverage_gates": {
    "pass": 80,
    "warn": 70,
    "block": 70
  }
}
```

## Performance Targets

- Gate check overhead: <500ms
- Full coverage scan (20 epics + 100 stories): <2 seconds

## Dependencies

- `devforgeai/traceability/gap-detector.sh` - Gap detection engine
- `devforgeai/epic-coverage/generate-report.sh` - Report generation

## Workflow Integration

```
Sprint Planning Phase:
  1. Validate story format
  2. Check dependencies
  3. ★ Run coverage quality gate ← THIS GATE
  4. Assign stories to sprint
  5. Update sprint capacity
```

## Error Handling

- Missing epic directory: Warning, use empty coverage
- Missing story directory: Warning, use 0% coverage
- Script execution error: Exit code 3, workflow halts
