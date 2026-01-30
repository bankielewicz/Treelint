# Phase 03: Report Generation

**Purpose:** Generate QA validation report and invoke post-QA hooks

**Execution Order:** After Phase 02 (AC Verification) completes

**Expected Outcome:** QA report generated, hooks invoked (if enabled)

---

## Phase Progress Indicator

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Phase 03/05: Report Generation (60% → 80% complete)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

---

## Step 1: Compile Verification Results

Aggregate results from Phase 02 AC Verification:

```
verification_results = {
    "story_id": "${STORY_ID}",
    "total_acs": count,
    "passed": [],      # AC items with status PASS
    "partial": [],     # AC items with status PARTIAL
    "not_implemented": [],  # AC items with status NOT_IMPLEMENTED
    "failed": []       # AC items with status FAIL
}
```

---

## Step 2: Post-QA Debt Detection Hook (STORY-287)

**COMP-001: IF EXISTS Pattern for Hook Invocation**

Check if post-qa-debt-detection hook should be invoked:

```python
# Step 2.1: Check hook file exists (COMP-001-REQ-001)
hook_path = "src/claude/hooks/post-qa-debt-detection.sh"

IF file_exists(hook_path):
    # Step 2.2: Check hooks.yaml configuration (COMP-001-REQ-002)
    hooks_config = Read(file_path="devforgeai/config/hooks.yaml")

    hook_entry = find_hook_by_id(hooks_config, "post-qa-debt-detection")

    IF hook_entry is None OR hook_entry.enabled == false:
        Display: "ℹ️  post-qa-debt-detection hook disabled in config"
        # Skip hook invocation, proceed to report generation
        GOTO Step 3

    # Step 2.3: Check if gaps exist (optimization)
    gaps = verification_results["partial"] + verification_results["not_implemented"]

    IF len(gaps) == 0:
        Display: "✓ No AC verification gaps - skipping debt detection hook"
        GOTO Step 3

    # Step 2.4: Invoke hook with verification context (COMP-001-REQ-003)
    Display: "🔍 Invoking post-qa-debt-detection hook..."

    hook_result = Bash(
        command=f"bash {hook_path} --story {STORY_ID}",
        input=json.dumps(verification_results),
        timeout=30000
    )

    # Step 2.5: Process hook exit code (BR-003)
    IF hook_result.exit_code == 0:
        # Proceed - no gaps or user declined
        Display: "✓ Hook completed - proceeding with report"

    ELIF hook_result.exit_code == 1:
        # Warn - gaps added to register
        Display: "⚠️ Technical debt entries added - check register"

    ELIF hook_result.exit_code == 2:
        # Halt - error occurred
        Display: "❌ Hook error - halting QA workflow"
        HALT Phase 03

ELSE:
    # Hook file does not exist - skip gracefully (Edge Case 4)
    Display: "ℹ️  post-qa-debt-detection hook not found - skipping"
    # Continue to Step 3
```

**Hook Invocation Summary:**

| Condition | Action |
|-----------|--------|
| Hook file missing | Skip, continue to report |
| Hook disabled in config | Skip, continue to report |
| No gaps detected | Skip, continue to report |
| Gaps exist, hook enabled | Invoke hook |
| Exit code 0 | Proceed with report |
| Exit code 1 | Warn, proceed with report |
| Exit code 2 | HALT workflow |

---

## Step 3: Generate QA Report

Create QA validation report with all findings:

```markdown
# QA Validation Report - ${STORY_ID}

**Date:** ${current_date}
**Status:** ${overall_status}

## AC Verification Summary

| Status | Count | Percentage |
|--------|-------|------------|
| PASS | ${pass_count} | ${pass_pct}% |
| PARTIAL | ${partial_count} | ${partial_pct}% |
| NOT_IMPLEMENTED | ${not_impl_count} | ${not_impl_pct}% |
| FAIL | ${fail_count} | ${fail_pct}% |

## Detailed Results

[Per-AC breakdown...]

## Technical Debt Tracking

${debt_detection_summary}

## Recommendations

[Based on findings...]
```

---

## Step 4: Write Report to File

```
report_path = f"devforgeai/qa/reports/{STORY_ID}-qa-report.md"
Write(file_path=report_path, content=report_content)

Display: "✓ QA report generated: {report_path}"
```

---

## Step 5: Update Story Status

If all ACs pass:
```
Edit(
    file_path=story_file,
    old_string="status: QA In Progress",
    new_string="status: QA Approved"
)
```

If any ACs fail or are incomplete:
```
# Generate gaps.json for remediation
gaps_path = f"devforgeai/qa/reports/{STORY_ID}-gaps.json"
Write(file_path=gaps_path, content=gaps_json)

Display: "⚠️ QA gaps recorded - run /dev {STORY_ID} --fix to remediate"
```

---

## Phase 03 Completion Checkpoint

- [ ] Verification results compiled
- [ ] Post-qa-debt-detection hook checked (IF EXISTS)
- [ ] Hook invoked if enabled and gaps exist
- [ ] QA report generated
- [ ] Story status updated
- [ ] Gaps file created (if applicable)

---

**References:**
- STORY-287: QA Hook Integration
- EPIC-048: Technical Debt Register Automation
- devforgeai/config/hooks.yaml: Hook configuration
- src/claude/hooks/post-qa-debt-detection.sh: Hook implementation
