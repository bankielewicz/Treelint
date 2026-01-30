# Phase 5: QA Report Generation Workflow

Generate comprehensive QA report and invoke qa-result-interpreter subagent.

---

## Overview

Aggregates results from Phases 1-4, generates QA report, updates story status, tracks iteration history, and invokes qa-result-interpreter for user-facing display.

## References Used

This workflow references:
- **qa-result-formatting-guide.md** - Display templates, framework constraints, remediation guidance (used by qa-result-interpreter subagent)

---

## Step 1: Aggregate Results

Collect all validation results from previous phases.

```
qa_results = {
    "story_id": story_id,
    "validation_mode": mode,  # "deep" or "light"
    "timestamp": current_timestamp(),
    "overall_status": "PENDING",  # Determined in Step 2

    "coverage": {
        "overall": overall_coverage,
        "business_logic": business_logic_coverage,
        "application": application_coverage,
        "infrastructure": infrastructure_coverage,
        "gaps": coverage_gaps,
        "test_quality": test_quality_metrics,
        "test_pyramid": test_pyramid_analysis
    },

    "anti_patterns": {
        "critical": critical_violations,
        "high": high_violations,
        "medium": medium_violations,
        "low": low_violations,
        "total": total_violation_count,
        "security_scan_results": security_scan_results
    },

    "spec_compliance": {
        "story_documentation": story_doc_status,  # Step 0
        "acceptance_criteria": criteria_results,  # Step 2
        "deferral_validation": deferral_validation_results,  # Step 2.5 ← MANDATORY
        "api_contracts": contract_results,  # Step 3
        "nfrs": nfr_results,  # Step 4
        "traceability": traceability_matrix  # Step 5
    },

    "quality_metrics": {
        "complexity": complexity_results,
        "maintainability": maintainability_results,
        "duplication": duplication_results,
        "documentation": documentation_results,
        "coupling": coupling_results
    }
}
```

---

## Step 2: Determine Overall Status

**Decision logic:**

```
blocking_issues = []

# Coverage violations
IF critical_violations.coverage.count > 0:
    blocking_issues.append("Critical coverage violations")
    overall_status = "FAIL"

# Anti-pattern violations
IF critical_violations.anti_patterns.count > 0:
    blocking_issues.append("Critical anti-pattern violations")
    overall_status = "FAIL"

IF high_violations.anti_patterns.count > 0:
    blocking_issues.append("High severity anti-pattern violations")
    overall_status = "FAIL"

# Spec compliance violations
IF acceptance_criteria_failures > 0:
    blocking_issues.append("Acceptance criteria not met")
    overall_status = "FAIL"

# Deferral validation violations (Step 2.5)
IF deferral_validation_results.critical_violations.count > 0:
    blocking_issues.append("Critical deferral violations (circular chains)")
    overall_status = "FAIL"

IF deferral_validation_results.high_violations.count > 0:
    blocking_issues.append("High severity deferral violations (invalid/unjustified)")
    overall_status = "FAIL"

# If all critical/high checks pass
IF blocking_issues.count == 0:
    overall_status = "PASS"

    # Check for warnings (medium/low violations)
    IF medium_violations.count > 0 OR low_violations.count > 0:
        overall_status = "PASS WITH WARNINGS"
```

**Status values:**
- **PASS**: No blocking violations
- **PASS WITH WARNINGS**: No blocking violations, but medium/low issues exist
- **FAIL**: One or more blocking violations (critical/high)

---

## Step 3: Write QA Report

**Generate comprehensive markdown report:**

```
report_content = generate_qa_report(qa_results)

Write(file_path="devforgeai/qa/reports/{story_id}-qa-report.md",
      content=report_content)
```

### Report Template Structure

```markdown
# QA Report: {story_id}

**Generated:** {timestamp}
**Mode:** {deep/light}
**Status:** {PASS/FAIL}

---

## Summary

- **Overall Status:** {PASS/FAIL/PASS WITH WARNINGS}
- **Blocking Issues:** {count}
- **Total Violations:** CRITICAL: {n}, HIGH: {n}, MEDIUM: {n}, LOW: {n}
- **Test Coverage:** {overall_percentage}%
- **Quality Score:** {calculated_score}/100

### Blocking Issues

{IF blocking_issues exist}
1. {issue_1}
2. {issue_2}
...

**QA cannot be approved until these issues are resolved.**

---

## Test Coverage Analysis

### Overall Coverage: {percentage}% [{✅ PASS / ❌ FAIL}]

**By Layer:**
| Layer | Coverage | Threshold | Status |
|-------|----------|-----------|--------|
| Business Logic | {bl_percentage}% | 95% | {✅/❌} |
| Application | {app_percentage}% | 85% | {✅/❌} |
| Infrastructure | {infra_percentage}% | 80% | {✅/❌} |

### Coverage Gaps

**High Priority (Business Logic):**
1. {file}:{method} - {suggested_test}
2. ...

**Medium Priority (Application):**
1. {file}:{method} - {suggested_test}
...

### Test Quality

- Assertion ratio: {ratio} assertions/test (target: ≥1.5)
- Test pyramid: Unit {u}%, Integration {i}%, E2E {e}%
- Over-mocking: {files_with_excessive_mocking}

---

## Anti-Pattern Detection

### Critical Violations: {count}

{IF count > 0}
{FOR each critical_violation}
**{index}. {violation.type}**
- File: {violation.file}:{violation.line}
- Message: {violation.message}
- Code: `{violation.code}`
- Remediation: {violation.remediation}

{ELSE}
✅ No critical anti-pattern violations detected.

### High Violations: {count}

{list high violations similarly}

### Medium Violations: {count}

{list medium violations similarly}

### Low Violations: {count}

{list low violations similarly}

---

## Spec Compliance

### Story Documentation: [{✅ COMPLETE / ⚠️ INCOMPLETE}]

{IF story_doc_incomplete}
- Missing sections: {list missing sections}
- Impact: {impact description}

### Acceptance Criteria: {passed}/{total} [{✅ PASS / ❌ FAIL}]

{FOR each acceptance_criterion}
**AC{index}: {criterion}**
- Tests: {test_files}
- Status: {✅ PASS / ❌ FAIL / ⚠️ NO TESTS}
- Coverage: {percentage}%

### Deferral Validation: [{✅ PASS / ❌ FAIL}]

**CRITICAL SECTION (Step 2.5):**

{IF deferrals exist}
**deferral-validator subagent:** ✅ Invoked (protocol followed)
**Total deferred items:** {count}
**Validation status:** {PASSED/FAILED}

**Validation Results:**
- Multi-level chains detected: {count} ← RCA-007
- Circular chains detected: {count}
- Invalid story references: {count}
- Missing ADRs: {count}
- Unjustified deferrals: {count}
- Unnecessary deferrals: {count}

{IF violations exist}
**Deferral Violations:**
{FOR each deferral_violation}
{index}. {item}: {violation_type} ({severity})
   - Reason: "{current_reason}"
   - Issue: {violation_message}
   - Required: {remediation}

**QA Status:** FAILED (deferral violations must be resolved)

{ELSE}
**Deferrals Validated:**
- All {count} deferrals have valid technical justification
- {story_split_count} follow-up stories created/referenced
- {adr_count} ADRs documented
- {blocker_count} external blockers tracked (with ETAs)

{ELSE}
**No deferred items found** - All Definition of Done items complete ✅

### API Contracts: [{✅ PASS / ❌ FAIL}]

{FOR each api_endpoint}
**{method} {path}**
- Implemented: {✅/❌}
- Request model: {✅ MATCH / ❌ MISMATCH}
- Response model: {✅ MATCH / ❌ MISMATCH}
- Status codes: {status}

### Non-Functional Requirements: [{✅ PASS / ⚠️ PARTIAL / ❌ FAIL}]

{FOR each nfr}
**{nfr.type}: {nfr.description}**
- Tests: {✅ EXISTS / ❌ MISSING}
- Target: {nfr.target}
- Actual: {measured_value}
- Status: {✅/⚠️/❌}

### Traceability Matrix

| Requirement | Tests | Implementation | Status | Coverage |
|-------------|-------|----------------|--------|----------|
{FOR each requirement in traceability_matrix}
| {req.description} | {req.tests} | {req.implementation} | {req.status} | {req.coverage}% |

---

## Code Quality Metrics

### Cyclomatic Complexity

- Methods >10: {count}
- Highest complexity: {max_complexity} ({method_name})
- Average complexity: {avg_complexity}

**Methods requiring refactoring:**
{list methods with complexity >10}

### Maintainability Index

- Files <70 MI: {count}
- Lowest MI: {min_mi} ({file_name})
- Average MI: {avg_mi}

**Files requiring improvement:**
{list files with MI <70}

### Code Duplication

- Duplication: {percentage}% (threshold: 5%)
- Duplicate blocks: {count}
- Status: [{✅ PASS / ⚠️ WARN / ❌ FAIL}]

### Documentation Coverage

- Coverage: {percentage}% (target: 80%)
- Documented APIs: {documented_count}/{total_apis}
- Status: [{✅ PASS / ⚠️ LOW}]

### Dependency Coupling

- Circular dependencies: {count}
- High coupling files: {count}

---

## Recommendations

{based on violations and warnings, generate actionable recommendations}

---

## Next Steps

{IF overall_status == "PASS"}
✅ **QA Approved** - Story ready for release
- Run `/release {story_id}` to deploy to staging
- Or continue with `/orchestrate {story_id}` if in automated workflow

{IF overall_status == "FAIL"}
❌ **QA Failed** - Resolve blocking issues before approval
- Review violations above
- Run `/dev {story_id}` to fix issues
- Re-run `/qa {story_id}` after fixes applied

{IF overall_status == "PASS WITH WARNINGS"}
⚠️ **QA Approved with Warnings** - Story can proceed but has technical debt
- Consider addressing warnings in follow-up story
- Document technical debt in devforgeai/technical-debt-register.md
```

**Report file location:** `devforgeai/qa/reports/{story_id}-qa-report.md`

**Story-Scoped Test Output Locations (STORY-092):**
- **Coverage Data:** `tests/coverage/{story_id}/` (e.g., coverage.json, coverage.xml)
- **Test Results:** `tests/results/{story_id}/` (e.g., test-results.xml, test-output.log)
- **Test Logs:** `tests/logs/{story_id}/` (console output capture)

---

## Step 3.5: Generate Structured Gap Export (gaps.json)

**Purpose:** Create machine-readable JSON export for `/dev` remediation workflow.

**When to generate:** ONLY when `overall_status == "FAIL"` (QA failures need targeted remediation)

### Generate gaps.json

```
IF overall_status == "FAIL":
    
    gaps_data = {
        "story_id": story_id,
        "qa_result": "FAILED",
        "generated_at": timestamp,
        "qa_report_file": "devforgeai/qa/reports/{story_id}-qa-report.md",

        # Story-scoped test output paths (STORY-092)
        "coverage_data_path": "tests/coverage/{story_id}/",
        "test_results_path": "tests/results/{story_id}/",
        "test_logs_path": "tests/logs/{story_id}/",

        "coverage_gaps": [],
        "anti_pattern_violations": [],
        "deferral_issues": []
    }

    # Extract coverage gaps with actionable details
    FOR EACH file in coverage_gaps:
        gap_entry = {
            "file": file.path,
            "layer": file.layer,  # "business_logic", "application", "infrastructure"
            "current_coverage": file.coverage_percentage,
            "target_coverage": layer_threshold(file.layer),  # 95/85/80
            "gap_percentage": target_coverage - current_coverage,
            "uncovered_line_count": file.uncovered_lines.count,
            "suggested_tests": generate_test_suggestions(file)
        }
        gaps_data.coverage_gaps.append(gap_entry)

    # Extract anti-pattern violations (CRITICAL and HIGH only - blocking)
    FOR EACH violation in (critical_violations + high_violations):
        violation_entry = {
            "file": violation.file,
            "line": violation.line,
            "type": violation.type,
            "severity": violation.severity,
            "message": violation.message,
            "remediation": violation.remediation
        }
        gaps_data.anti_pattern_violations.append(violation_entry)

    # Extract deferral issues
    FOR EACH deferral in deferral_validation_results.violations:
        deferral_entry = {
            "item": deferral.item,
            "violation_type": deferral.type,
            "severity": deferral.severity,
            "current_reason": deferral.reason,
            "issue": deferral.message,
            "remediation": deferral.required_action
        }
        gaps_data.deferral_issues.append(deferral_entry)

    # Write structured gap export
    Write(file_path="devforgeai/qa/reports/{story_id}-gaps.json",
          content=json.dumps(gaps_data, indent=2))
```

### gaps.json Schema

```json
{
  "story_id": "STORY-078",
  "qa_result": "FAILED",
  "generated_at": "2025-12-06T08:30:00Z",
  "qa_report_file": "devforgeai/qa/reports/STORY-078-qa-report.md",

  "coverage_gaps": [
    {
      "file": "installer/rollback.py",
      "layer": "business_logic",
      "current_coverage": 63.6,
      "target_coverage": 95.0,
      "gap_percentage": 31.4,
      "uncovered_line_count": 56,
      "suggested_tests": [
        "Test rollback on corrupted backup file",
        "Test rollback when target directory is read-only",
        "Test partial rollback recovery after interruption",
        "Test rollback error handling for missing backup"
      ]
    }
  ],

  "anti_pattern_violations": [
    {
      "file": "src/service.py",
      "line": 45,
      "type": "God Object",
      "severity": "HIGH",
      "message": "Class exceeds 500 lines with 15 responsibilities",
      "remediation": "Extract responsibilities into separate classes"
    }
  ],

  "deferral_issues": [
    {
      "item": "Integration tests for external API",
      "violation_type": "MISSING_ADR",
      "severity": "HIGH",
      "current_reason": "External API not available",
      "issue": "No ADR documenting this decision",
      "remediation": "Create ADR-XXX documenting external API blocker"
    }
  ]
}
```

### Test Suggestion Generation

```
FUNCTION generate_test_suggestions(file):
    suggestions = []

    # Analyze uncovered code patterns
    FOR EACH uncovered_block in file.uncovered_blocks:
        IF uncovered_block.type == "error_handler":
            suggestions.append(f"Test error handling in {uncovered_block.function}")
        IF uncovered_block.type == "conditional_branch":
            suggestions.append(f"Test {uncovered_block.condition} branch in {uncovered_block.function}")
        IF uncovered_block.type == "edge_case":
            suggestions.append(f"Test edge case: {uncovered_block.description}")

    # Add layer-specific suggestions
    IF file.layer == "business_logic":
        suggestions.append("Add unit tests for business rule validation")
    IF file.layer == "application":
        suggestions.append("Add integration tests for service orchestration")
    IF file.layer == "infrastructure":
        suggestions.append("Add tests for external system interaction")

    RETURN suggestions[:4]  # Return top 4 suggestions (natural language)
```

**Rationale:** Natural language test descriptions are optimal for Claude - they give scenario intent without forcing naming conventions.

### gaps.json Lifecycle

```
QA FAILED:
  → Create: devforgeai/qa/reports/{story_id}-gaps.json

/dev REMEDIATION:
  → Read: devforgeai/qa/reports/{story_id}-gaps.json
  → Enter remediation mode, target specific gaps
  → Fix issues with targeted test generation

QA RE-RUN:
  IF overall_status == "PASS":
    → Move gaps.json to resolved archive
    → Update story Implementation Notes with resolution reference
  ELSE:
    → Overwrite gaps.json with new gap data
```

---

## Step 3.6: Archive Resolved Gaps (QA PASS)

**Purpose:** Move gaps.json to resolved archive when QA passes after previous failure.

```
IF overall_status == "PASS" OR overall_status == "PASS WITH WARNINGS":

    # Check if gaps.json exists from previous failed QA
    gaps_file = "devforgeai/qa/reports/{story_id}-gaps.json"

    Glob(pattern=gaps_file)

    IF gaps_file EXISTS:
        # Move to resolved archive
        Read(file_path=gaps_file)
        gaps_content = file_content

        Write(file_path="devforgeai/qa/resolved/{story_id}-gaps.json",
              content=gaps_content)

        # Delete original gaps file
        Bash(command="rm {gaps_file}")

        # Update story Implementation Notes
        implementation_notes_update = """
### Coverage Gap Resolution

**Date:** {timestamp}
**Resolution file:** `devforgeai/qa/resolved/{story_id}-gaps.json`

Coverage gaps from previous QA failure have been resolved via QA-Dev integration workflow.
"""

        # Append to story Implementation Notes section
        Edit(story file to add implementation_notes_update to Implementation Notes)
```

**Result:** Clean state after QA passes, audit trail preserved in resolved/ folder.

---

## Step 4: Update Story Status

**Edit story YAML frontmatter:**

```
Read(file_path="devforgeai/specs/Stories/{story_id}.story.md")

IF overall_status == "PASS" OR overall_status == "PASS WITH WARNINGS":
    Edit(file_path="devforgeai/specs/Stories/{story_id}.story.md",
         old_string="status: Dev Complete",
         new_string="status: QA Approved ✅")

    # Add QA completion to workflow history
    workflow_history_entry = f"""
- **{timestamp}**: QA validation PASSED ({mode} mode)
  - Coverage: {overall_coverage}%
  - Violations: {violation_summary}
  - Report: `devforgeai/qa/reports/{story_id}-qa-report.md`
"""
    Append workflow_history_entry to story Workflow History section

IF overall_status == "FAIL":
    Edit(file_path="devforgeai/specs/Stories/{story_id}.story.md",
         old_string="status: Dev Complete",
         new_string="status: QA Failed ❌")

    # Add failure details to workflow history
    workflow_history_entry = f"""
- **{timestamp}**: QA validation FAILED ({mode} mode)
  - Blocking issues: {blocking_issues}
  - Report: `devforgeai/qa/reports/{story_id}-qa-report.md`
  - Action required: Fix issues and re-run `/qa {story_id}`
"""
    Append workflow_history_entry to story Workflow History section
```

**Status transitions:**
- "Dev Complete" → "QA Approved" (if PASS)
- "Dev Complete" → "QA Failed" (if FAIL)
- "QA Failed" → "QA Approved" (if re-validation PASS)

---

## Step 5: Track QA Iteration History (RCA-007 Enhancement)

**Purpose:** Maintain audit trail of QA attempts and deferral resolutions

### Check if This is a Re-Validation

```
Grep(pattern="## QA Validation History", path="devforgeai/specs/Stories/{story_id}.story.md")

IF found:
    # This is a re-validation
    Read QA Validation History section
    Count previous QA attempts (search for "### QA Attempt" entries)
    attempt_number = previous_attempts + 1
ELSE:
    # First QA validation
    attempt_number = 1
```

### Append QA Iteration Entry to Story File

**Create or update QA Validation History section:**

```
Edit story file to add QA history section (if not exists):

## QA Validation History

### QA Attempt {attempt_number} - {timestamp} - {PASSED/FAILED}

**Mode:** {deep/light}
**Duration:** {duration} minutes
**QA Report:** `devforgeai/qa/reports/{story_id}-qa-report{-attempt-N if N>1}.md`

**Results:**
- **Test Coverage:** {overall_coverage}% ({✅ PASS / ❌ FAIL})
  - Business Logic: {bl_coverage}% (threshold: 95%)
  - Application: {app_coverage}% (threshold: 85%)
  - Infrastructure: {infra_coverage}% (threshold: 80%)

- **Violations:** CRITICAL: {n}, HIGH: {n}, MEDIUM: {n}, LOW: {n}

- **Deferral Validation:** {✅ INVOKED / ❌ SKIPPED} ⭐ RCA-007

{IF Deferral Validation INVOKED}
**Deferral Validation Results:**
- **deferral-validator subagent:** ✅ Invoked (protocol followed)
- Total deferred items: {count}
- Validation status: {PASSED/FAILED}
- Multi-level chains detected: {count} ← RCA-007 KEY METRIC
- Circular chains detected: {count}
- Invalid story references: {count}
- Missing ADRs: {count}
- Unnecessary deferrals: {count}

{IF deferral validation FAILED}
**Deferral Issues:**
{FOR each deferral_violation}
{index}. {item}: {violation_type} - {severity}
   - Reason: "{current_reason}"
   - Issue: {violation_message}
   - Required: {remediation}

**Resolution Required:**
- {specific_action_steps from deferral-validator}
- Run `/dev {story_id}` to fix deferrals, then re-run `/qa`

{ELSE IF deferral validation PASSED}
**Deferrals Validated:**
- All {count} deferrals have valid technical justification
- {follow_up_story_count} follow-up stories created/referenced (exist and include work)
- {adr_count} ADRs documented (exist and reference deferrals)
- {blocker_count} external blockers tracked (with ETAs)
- {story_split_count} story splits (single-hop, no chains)

**Validation Evidence:**
- deferral-validator subagent invoked: ✅ (protocol followed)
- Multi-level chain detection: {result} ← RCA-007 KEY VALIDATION
- Circular chain detection: {result}
- Story reference validation: {result}
- ADR requirement check: {result}
- Technical blocker verification: {result}

{ELSE IF Deferral Validation SKIPPED}
⚠️ **PROTOCOL VIOLATION (RCA-007):**
- **deferral-validator subagent:** ❌ NOT invoked (manual validation only)
- **Issue:** Mandatory protocol not followed
- **Risk:** Multi-level chains, missing ADRs, invalid references not detected
- **Evidence:** RCA-007 shows manual validation misses critical issues
- **Required:** Re-validation with proper deferral-validator invocation

**Action Required:**
This QA validation is INVALID. Must re-run with proper protocol:
1. Re-run `/qa {story_id}` (will invoke deferral-validator)
2. Review deferral validation results
3. Fix any violations detected

{IF overall_status == "PASS"}
**✅ QA PASSED**
- All quality gates passed
- Story approved for release

{IF overall_status == "FAIL"}
**❌ QA FAILED**
- Blocking issues must be resolved
- See violations sections above
- Run `/dev {story_id}` to fix, then re-run `/qa`
```

### Track Cumulative Metrics

```
Calculate and log:
- Total QA attempts for this story: {attempt_number}
- Deferral resolution time: {first_attempt_date} → {final_pass_date} (if applicable)
- Number of deferrals resolved across attempts: {count}
- Number of deferrals justified: {count}
```

### Warn on Excessive Attempts

```
IF attempt_number > 3:
    WARN: "Story has failed QA {attempt_number} times. Consider:
           - Story scope too large (split into smaller stories)
           - DoD items not properly estimated
           - Systemic issues with story planning

           Recommend reviewing with Product Owner and Tech Lead."
```

**Purpose of QA Validation History:**
- Tracks all QA attempts for audit trail
- Records deferral validation compliance (RCA-007)
- Detects stories with excessive failures
- Preserves evidence of protocol adherence
- Enables retrospective analysis

---

## Step 6: Invoke qa-result-interpreter Subagent

**Purpose:** Parse QA report and generate user-facing display template

### Subagent Invocation

```
Task(
    subagent_type="qa-result-interpreter",
    description="Interpret QA results for display",
    prompt="Parse QA validation results and generate user-facing display template.

            QA report file: devforgeai/qa/reports/{story_id}-qa-report.md

            Analysis required:
            1. Extract overall status (PASS/FAIL/PASS WITH WARNINGS)
            2. Categorize violations by severity
            3. Determine display mode (light/deep)
            4. Generate appropriate template (PASSED/FAILED/PARTIAL)
            5. Provide remediation guidance
            6. Recommend next steps

            Follow framework constraints in qa-result-formatting-guide.md.

            Return structured JSON:
            {
              \"display\": {
                \"template\": \"[complete formatted display]\",
                \"mode\": \"deep\",
                \"status\": \"PASS\"
              },
              \"violations\": {
                \"critical\": [...],
                \"high\": [...],
                \"medium\": [...],
                \"low\": [...]
              },
              \"recommendations\": {
                \"immediate\": [...],
                \"follow_up\": [...]
              },
              \"next_steps\": \"[guidance for user]\"
            }"
)
```

### Subagent Output Processing

```
interpreter_result = parse_subagent_response()

# Store formatted display
formatted_display = interpreter_result.display.template

# Store recommendations
immediate_actions = interpreter_result.recommendations.immediate
follow_up_actions = interpreter_result.recommendations.follow_up

# Store next steps
next_steps = interpreter_result.next_steps
```

### Subagent References

**qa-result-interpreter subagent** consults:
- `qa-result-formatting-guide.md` - Display guidelines, framework constraints, tone, emoji usage

**Subagent ensures:**
- Framework-aware display (respects DevForgeAI conventions)
- Appropriate tone (PASS: encouraging, FAIL: constructive)
- Actionable guidance (specific remediation steps)
- Context-appropriate templates (light/deep, pass/fail)

---

## Phase 5 Output

**Results to return to caller:**

```
{
    "status": overall_status,  # "PASS", "FAIL", "PASS WITH WARNINGS"
    "story_status": updated_story_status,  # "QA Approved", "QA Failed"
    "report_file": "devforgeai/qa/reports/{story_id}-qa-report.md",
    "display": formatted_display,  # From qa-result-interpreter
    "violations": {
        "critical": critical_count,
        "high": high_count,
        "medium": medium_count,
        "low": low_count
    },
    "coverage": {
        "overall": overall_percentage,
        "by_layer": layer_coverage_summary
    },
    "deferral_validation": {
        "invoked": True/False,  # RCA-007 compliance tracking
        "status": "PASSED"/"FAILED"/"N/A",
        "violations": deferral_violation_count
    },
    "next_steps": next_steps,  # From qa-result-interpreter
    "qa_attempt": attempt_number
}
```

**Workflow completes:** QA skill returns this result to invoking command/skill

---

## Integration with Commands

### /qa Command Integration

```
# Command invokes skill
Skill(command="devforgeai-qa")

# Skill executes Phases 1-5
# Skill returns result (above structure)

# Command displays result.display (formatted template)
# Command shows result.next_steps
```

**Token efficiency:**
- Command: ~2K tokens (lean orchestration)
- Skill: ~65K tokens (isolated context, deep mode)
- Subagent: ~8K tokens (qa-result-interpreter, isolated context)
- **Total main conversation:** ~2K tokens (98% efficiency)

### devforgeai-development Skill Integration

```
# Development skill auto-invokes QA (light mode) after TDD phases
Skill(command="devforgeai-qa")

# If light QA fails:
#   Development continues with fixes
#   Re-runs light QA

# After dev complete:
#   Deep QA invoked (manual or via orchestration)
```

---

**Phase 5 generates complete QA assessment with formatted display for user presentation.**
