# Error Type 4: Validation Failures

Handling quality issues detected during self-validation of generated artifacts.

---

## Error Detection

**Symptom:** Generated artifacts don't meet quality standards after self-validation

**Detected during:** Phase 6.4 (Self-Validation)

**Examples:**
- Epic documents missing required YAML fields
- Requirements spec has >5 placeholder entries (TODO, TBD)
- Success metrics not measurable ("improve UX" without metric)
- Acceptance criteria not testable
- Data models missing entity relationships

**Detection logic:**

```
validation_result = run_validation_checklist()

if validation_result.has_failures:
    categorize_failures:
        critical = [f for f in failures if f.severity == "CRITICAL"]
        high = [f for f in failures if f.severity == "HIGH"]
        medium = [f for f in failures if f.severity == "MEDIUM"]

    if len(critical) > 0:
        trigger validation_failure_recovery(critical)
```

---

## Recovery Procedures

### Step 1: Identify Specific Validation Failures

```
failures = []

# Epic document validation
if not epic_documents_valid:
    for epic in epic_files:
        Read(file_path=epic, limit=20)  # Frontmatter only

        if missing_field("id"):
            failures.append({
                "type": "missing_epic_id",
                "file": epic,
                "severity": "CRITICAL",
                "fix": "Generate ID from filename"
            })

        if missing_field("business-value"):
            failures.append({
                "type": "missing_business_value",
                "file": epic,
                "severity": "CRITICAL",
                "fix": "Cannot auto-generate - require user input"
            })

        if missing_field("created"):
            failures.append({
                "type": "missing_created_date",
                "file": epic,
                "severity": "MEDIUM",
                "fix": "Auto-correct with today's date"
            })

# Requirements spec validation
if not requirements_spec_valid:
    Read(file_path=req_spec)

    placeholder_count = Grep(
        pattern="TODO|TBD|FIXME|\[.*\]",
        path=req_spec,
        output_mode="count"
    )

    if placeholder_count > 5:
        failures.append({
            "type": "too_many_placeholders",
            "file": req_spec,
            "severity": "HIGH",
            "fix": "Review requirements, complete missing sections"
        })

    vague_count = Grep(
        pattern="fast|slow|scalable|secure|reliable|good|better",
        path=req_spec,
        output_mode="count"
    )

    if vague_count > 10:
        failures.append({
            "type": "vague_requirements",
            "file": req_spec,
            "severity": "HIGH",
            "fix": "Quantify NFRs with specific metrics"
        })
```

### Step 2: Regenerate Only Failing Sections

```
for failure in failures:
    if failure.severity == "MEDIUM":
        # Auto-correct: missing_created_date → today, missing_status → "Planning"
        auto_correct_field(failure)
        auto_corrected.append(failure)

    elif failure.severity == "HIGH":
        # Attempt regeneration: vague_nfr → re-ask, missing_data_models → extract
        regenerate_section(failure.type)

    elif failure.severity == "CRITICAL":
        # Cannot auto-correct - will report to user
        pass
```

### Step 3: Re-Run Validation (Maximum 2 Attempts)

```
attempt = 0
max_attempts = 2

while attempt < max_attempts:
    # Load validation checklists
    Read(file_path=".claude/skills/devforgeai-ideation/references/validation-checklists.md")

    validation_result = run_validation_checklist()

    if validation_result.all_passed:
        ✓ Validation succeeded on attempt {attempt + 1}

        Report auto-corrections:
        "Auto-corrected {count} issues:
        {list of auto_corrected items}"

        break
    else:
        attempt += 1
        if attempt < max_attempts:
            self_heal(validation_result.failures)
        else:
            report_validation_failures_to_user(validation_result.failures)
            break
```

### Step 4: Report to User for Manual Review

```
If validation fails after 2 attempts:
    Report: """
    ⚠️ Validation Issues Detected

    Auto-corrected ({auto_count} issues):
    {list of auto-corrected issues with checkmarks}

    Remaining issues ({remaining_count}):

    **CRITICAL Issues ({critical_count}):**
    - {critical_issue_1}: {specific guidance}
      → Fix: {remediation steps}
      → File: {file_path}

    **HIGH Issues ({high_count}):**
    - {high_issue_1}: {specific guidance}

    **MEDIUM Issues ({medium_count}):**
    - {medium_issue_1}: Can defer to architecture phase

    Recommended Actions:
    1. Review file: {file_path}
    2. Address CRITICAL issues (required before proceeding)
    3. Consider fixing HIGH issues (recommended)
    4. MEDIUM issues can be deferred

    Options:
    - Ask me to fix specific issues ("Fix missing business value in EPIC-001")
    - Manually edit files
    - Proceed with warnings (CRITICAL issues must be fixed first)
    """
```

---

## Example Scenarios

### Scenario 1: Missing Epic Fields

**Error:** EPIC-001 missing `business-value` field

**Recovery:**
1. Detect as CRITICAL severity
2. Cannot auto-generate business value
3. Ask user for business value
4. Update epic frontmatter

### Scenario 2: Too Many Placeholders

**Error:** Requirements spec has 12 TODO items

**Recovery:**
1. Identify which sections have placeholders
2. Attempt to regenerate from Phase 2 data
3. Report remaining TODOs to user

### Scenario 3: Vague NFRs

**Error:** 15 instances of vague terms without quantification

**Recovery:**
1. Identify each vague term
2. Provide specific quantification options
3. Update requirements with measurable values

---

## Max Recovery Attempts

**Attempt 1:** Auto-correct MEDIUM issues, regenerate HIGH issue sections, re-validate
**Attempt 2:** Re-attempt regeneration, re-validate

**If still failing:** Report to user with specific remediation guidance, HALT on CRITICAL issues

---

## Related Patterns

- See [validation-checklists.md](validation-checklists.md) for complete validation procedures
- See [error-type-1-incomplete-answers.md](error-type-1-incomplete-answers.md) for handling vague requirements
- See [error-handling-index.md](error-handling-index.md) for error type decision tree

---

## Phase Context

This error occurs during **Phase 6.4: Self-Validation** after artifact generation, when the ideation skill validates that generated epics and requirements specifications meet quality standards.

Recovery attempts to auto-correct MEDIUM-severity issues and regenerate HIGH-severity sections before reporting remaining issues to the user.

---

**Token Budget:** ~2,000-4,000 tokens per validation cycle
