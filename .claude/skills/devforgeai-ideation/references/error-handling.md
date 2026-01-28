# Error Handling & Recovery Procedures

Comprehensive error handling for all 6 phases of the ideation workflow.

## Overview

This reference documents all error scenarios, detection methods, and recovery procedures for the ideation skill. Each error includes self-healing attempts before reporting to users.

**Error Categories:**
1. Incomplete User Answers (Phase 2)
2. Artifact Generation Failures (Phase 6.1)
3. Complexity Assessment Errors (Phase 3)
4. Validation Failures (Phase 6.4)
5. Brownfield Constraint Conflicts (Phase 5-6)
6. Directory Structure Issues (Phase 6.1)

**Recovery Strategy:** Self-heal → Retry → Report to user (max 2 attempts)

---

## Error 1: Incomplete User Answers

### Detection

**Symptom:** User responses too vague or incomplete for requirements elicitation

**Detected during:** Phase 2 (Requirements Elicitation)

**Examples:**
- User says "I don't know" to critical questions
- User provides vague terms: "fast", "scalable", "secure", "user-friendly"
- User cannot quantify NFRs
- User selects "Other" and provides unclear free-text

**Detection logic:**

```
if user_answer contains ["I don't know", "not sure", "maybe", "TBD"]:
    trigger incomplete_answer_recovery

if user_answer contains vague_terms AND no quantification:
    vague_terms = ["fast", "slow", "scalable", "secure", "reliable", "good UX"]
    trigger vague_answer_recovery
```

### Recovery Procedure

**Step 1: Ask Follow-Up Questions**

```
If user answer is vague:
    # Load domain-specific probing questions
    Read(file_path=".claude/skills/devforgeai-ideation/references/requirements-elicitation-guide.md")

    Use domain-specific probing:
    - E-commerce: "What payment methods should be supported?"
    - SaaS: "What user roles need different permissions?"
    - Fintech: "What compliance standards apply (PCI-DSS, SOC2, regulations)?"
    - Healthcare: "What patient data will be stored (PHI vs non-PHI)?"
```

**Step 2: Use AskUserQuestion with More Specific Options**

```
# Instead of: "What performance do you need?"
# Ask with quantified options:

AskUserQuestion(
    question: "What response time is acceptable?",
    header: "Performance target",
    options: [
        {
            label: "High performance",
            description: "<100ms API response, >10k concurrent users"
        },
        {
            label: "Standard performance",
            description: "<500ms API response, 1k-10k users"
        },
        {
            label: "Moderate performance",
            description: "<2s response time, <1k users"
        }
    ]
)
```

**Step 3: Provide Examples of Good Answers**

```
Explain to user:

"To help define requirements precisely, here are examples:

Instead of 'fast':
  → 'API responses under 500ms for 95th percentile'
  → 'Page loads under 2 seconds on 4G connection'

Instead of 'secure':
  → 'OAuth2 authentication with JWT tokens'
  → 'RBAC authorization with 5 roles'
  → 'AES-256 encryption for data at rest'
  → 'TLS 1.3 for data in transit'

Instead of 'scalable':
  → 'Support 10,000 concurrent users'
  → 'Horizontal scaling to 50 instances'
  → 'Database sharding for >1M records'

Would you like to provide more specific requirements?"
```

**Step 4: Document Assumptions with Validation Flags**

```
If user cannot provide specifics after follow-ups:
    # Document as assumption
    Add to requirements spec:

    **ASSUMPTION:** Average order size is <100 line items
    **VALIDATION NEEDED:** Confirm with stakeholders during architecture phase
    **RISK:** If assumption wrong, may need to redesign data model

    **ASSUMPTION:** Users have modern browsers (Chrome/Firefox/Safari/Edge latest 2 versions)
    **VALIDATION NEEDED:** Check analytics data for actual browser usage
    **RISK:** If older browsers needed, may require polyfills and testing
```

### When to Use

- User says "I don't know" or "not sure"
- User gives vague answers without quantification
- User cannot provide specifics after probing
- User selects "Other" with unclear explanation

### Max Recovery Attempts

**Attempt 1:** Follow-up questions with specific options
**Attempt 2:** Provide examples, ask again with guidance
**Attempt 3:** Document as assumption, flag for validation

**If still incomplete:** Continue with assumptions flagged, will be validated in architecture phase

---

## Error 2: Artifact Generation Failures

### Detection

**Symptom:** Epic or requirements file not created due to write errors, permissions, or path issues

**Detected during:** Phase 6.1 (Artifact Generation)

**Examples:**
- Write() tool returns permission error
- Directories don't exist (`devforgeai/specs/Epics/` not found)
- Disk full or read-only filesystem
- Invalid filename characters

**Detection logic:**

```
try:
    Write(file_path="devforgeai/specs/Epics/EPIC-001.epic.md", content=epic_content)
except FileWriteError as error:
    trigger artifact_generation_failure_recovery(error)
```

### Recovery Procedure

**Step 1: Check Directory Permissions**

```
# Ensure directories exist
required_dirs = [
    "devforgeai/specs/Epics/",
    "devforgeai/specs/requirements/",
    "devforgeai/specs/adrs/",
    "devforgeai/specs/context/"
]

for dir in required_dirs:
    # Create if missing using Write/.gitkeep pattern (Constitutional C1 compliant)
    Write(file_path=f"{dir}.gitkeep", content="")

# Retry write operation
Write(file_path="devforgeai/specs/Epics/EPIC-001.epic.md", content=epic_content)
```

**Step 2: Verify File System Paths**

```
# Check if directories were created successfully
for dir in required_dirs:
    check = Glob(pattern=f"{dir}")

    if not check:
        ERROR: Cannot create directory {dir}

        Report to user: """
        ❌ File system permissions prevent directory creation

        Manual steps required:
        1. Create directory: {dir}
        2. Grant write permissions
        3. Re-run ideation skill

        Or run:
        bash
        mkdir -p {dir}
        chmod 755 {dir}

        Then retry.
        """
```

**Step 3: Retry with Error Handling**

```
max_retries = 2

for artifact in artifacts_to_create:
    attempt = 0

    while attempt < max_retries:
        try:
            Write(file_path=artifact.path, content=artifact.content)
            ✓ Created {artifact.path}
            break
        except FileWriteError:
            attempt += 1
            if attempt >= max_retries:
                # Max retries exceeded
                Report error with manual creation instructions
```

**Step 4: Provide Manual Creation Instructions**

```
If persistent failure after retries:
    Report: """
    ❌ Could not create {artifact_name} automatically

    Manual creation steps:
    1. Create file: {file_path}
    2. Copy this content:

    ```
    {full_artifact_content}
    ```

    3. Save file
    4. Continue to next epic or run /create-context
    """
```

### When to Use

- Write() tool fails with permission error
- Directories don't exist and cannot be created
- File system issues (disk full, read-only)
- Path issues (invalid characters, too long)

### Max Recovery Attempts

**Attempt 1:** Create directories, retry write
**Attempt 2:** Verify paths, retry with error handling

**If still failing:** Provide manual creation instructions, continue with remaining artifacts

---

## Error 3: Complexity Assessment Errors

### Detection

**Symptom:** Complexity score invalid, out of range, or breakdown missing

**Detected during:** Phase 3 (Complexity Assessment) or Phase 6.4 (Validation)

**Examples:**
- Score <0 or >60
- Score breakdown missing dimensions
- Tier doesn't match score range
- Functional complexity >20, Technical >20, Team/Org >10, NFR >10

**Detection logic:**

```
if complexity_score < 0 or complexity_score > 60:
    trigger complexity_assessment_error_recovery

if functional > 20 or technical > 20 or team_org > 10 or nfr > 10:
    trigger dimension_overflow_error_recovery

if tier_mismatch(score, tier):
    trigger tier_mismatch_error_recovery
```

### Recovery Procedure

**Step 1: Recalculate Using Assessment Matrix**

```
ERROR: Invalid complexity score {complexity_score}

# Load assessment matrix
Read(file_path=".claude/skills/devforgeai-ideation/references/complexity-assessment-matrix.md")

# Recalculate all dimensions using Phase 2 data
functional_score = calculate_functional_complexity(
    user_roles_count,
    entities_count,
    integrations_count,
    workflow_complexity
)  # Result: 0-20

technical_score = calculate_technical_complexity(
    data_volume,
    concurrency_target,
    realtime_requirements
)  # Result: 0-20

team_org_score = calculate_team_complexity(
    team_size,
    distribution
)  # Result: 0-10

nfr_score = calculate_nfr_complexity(
    performance_target,
    compliance_requirements
)  # Result: 0-10

total_score = functional + technical + team_org + nfr

# Validate totals
assert 0 <= total_score <= 60
assert 0 <= functional <= 20
assert 0 <= technical <= 20
assert 0 <= team_org <= 10
assert 0 <= nfr <= 10

✓ Recalculated complexity: {total_score}/60
```

**Step 2: Verify All 4 Dimensions Scored**

```
Required dimensions:
- [ ] Functional Complexity (0-20)
- [ ] Technical Complexity (0-20)
- [ ] Team/Organizational Complexity (0-10)
- [ ] Non-Functional Complexity (0-10)

If any dimension missing:
    # Recalculate missing dimension using assessment matrix
    # Use Phase 2-5 discovery data for inputs
    missing_dimension_score = calculate_dimension(phase_data)

    # Update total score
    total_score += missing_dimension_score
```

**Step 3: Update Requirements Spec with Corrected Assessment**

```
# Read existing requirements spec
Read(file_path="devforgeai/specs/requirements/{project}-requirements.md")

# Find complexity assessment section
complexity_section = Grep(
    pattern="## Complexity Assessment",
    path="devforgeai/specs/requirements/{project}-requirements.md",
    -A=20,
    output_mode="content"
)

# Replace with corrected assessment
Edit(
    file_path="devforgeai/specs/requirements/{project}-requirements.md",
    old_string="## Complexity Assessment\n{old_content}",
    new_string="## Complexity Assessment\n\n**Total Score:** {corrected_score}/60\n**Architecture Tier:** {correct_tier}\n\n**Score Breakdown:**\n- Functional: {functional}/20\n- Technical: {technical}/20\n- Team/Org: {team_org}/10\n- NFR: {nfr}/10\n..."
)

✓ Complexity assessment corrected in requirements spec
```

**Step 4: Re-Validate Architecture Tier**

```
# Map score to tier
if score >= 0 and score <= 15:
    tier = "Tier 1: Simple Application"
elif score >= 16 and score <= 30:
    tier = "Tier 2: Moderate Application"
elif score >= 31 and score <= 45:
    tier = "Tier 3: Complex Platform"
elif score >= 46 and score <= 60:
    tier = "Tier 4: Enterprise Platform"

# Verify tier is documented in requirements spec
# Update technology recommendations for tier (from output-templates.md)

✓ Architecture tier validated: {tier}
```

### When to Use

- Complexity score is 0 or >60
- Score breakdown missing dimensions
- Dimension scores exceed max (functional >20, technical >20, etc.)
- Tier doesn't match score range
- Technology recommendations missing

### Max Recovery Attempts

**Attempt 1:** Recalculate using assessment matrix, update requirements spec

**If still failing:** HALT - Critical calculation error, require manual review

---

## Error 4: Validation Failures (Phase 6.4)

### Detection

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

### Recovery Procedure

**Step 1: Identify Specific Validation Failures**

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

**Step 2: Regenerate Only Failing Sections**

```
for failure in failures:
    if failure.severity == "MEDIUM":
        # Auto-correct medium issues
        if failure.type == "missing_created_date":
            Edit(
                file_path=failure.file,
                old_string="created:",
                new_string=f"created: {today_date}"
            )
            auto_corrected.append(failure)

        elif failure.type == "missing_status":
            Edit(
                file_path=failure.file,
                old_string="status:",
                new_string="status: Planning"
            )
            auto_corrected.append(failure)

    elif failure.severity == "HIGH":
        # Attempt regeneration for high issues
        if failure.type == "vague_nfr":
            # Re-ask NFR questions from Phase 2
            # Update requirements spec with quantified NFRs
            regenerate_nfr_section()

        elif failure.type == "missing_data_models":
            # Extract entities from functional requirements
            # Generate data model documentation
            regenerate_data_model_section()

    elif failure.severity == "CRITICAL":
        # Cannot auto-correct critical issues
        # Will report to user in Step 4
        pass
```

**Step 3: Re-Run Validation (Maximum 2 Attempts)**

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
            # Self-heal and retry
            self_heal(validation_result.failures)
        else:
            # Report to user after max attempts
            report_validation_failures_to_user(validation_result.failures)
            break
```

**Step 4: Report to User for Manual Review**

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

    - {critical_issue_2}: {specific guidance}

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

### When to Use

- Phase 6.4 validation detects quality issues
- Epic frontmatter incomplete (missing title, business-value, id)
- Requirements too vague (>10 instances of vague terms)
- Acceptance criteria not testable
- Data models missing relationships
- Success metrics not measurable

### Max Recovery Attempts

**Attempt 1:** Auto-correct MEDIUM issues, regenerate HIGH issue sections, re-validate
**Attempt 2:** Re-attempt regeneration, re-validate

**If still failing:** Report to user with specific remediation guidance, HALT on CRITICAL issues

---

## Error 5: Brownfield Constraint Conflicts

### Detection

**Symptom:** New requirements conflict with existing context files (brownfield projects)

**Detected during:** Phase 5 (Feasibility Analysis) or Phase 6.6 (Handoff)

**Examples:**
- New requirement needs React, but tech-stack.md specifies Vue
- New feature requires microservices, but architecture-constraints.md enforces monolith
- New dependency not in approved dependencies.md
- New pattern violates anti-patterns.md

**Detection logic:**

```
if existing_context_files:
    # Load existing constraints
    Read(file_path="devforgeai/specs/context/tech-stack.md")
    Read(file_path="devforgeai/specs/context/architecture-constraints.md")
    Read(file_path="devforgeai/specs/context/dependencies.md")
    Read(file_path="devforgeai/specs/context/anti-patterns.md")

    # Compare with new requirements
    conflicts = []

    if new_requirement.technology not in existing_tech_stack:
        conflicts.append({
            "type": "technology_conflict",
            "new": new_requirement.technology,
            "existing": existing_tech_stack,
            "severity": "HIGH"
        })

    if new_requirement.violates_architecture_constraints:
        conflicts.append({
            "type": "architecture_conflict",
            "constraint": constraint_violated,
            "severity": "CRITICAL"
        })

    if len(conflicts) > 0:
        trigger brownfield_conflict_recovery(conflicts)
```

### Recovery Procedure

**Step 1: Detect Conflicts**

```
# Technology conflicts
tech_conflicts = Grep(
    pattern=new_technology_name,
    path="devforgeai/specs/context/tech-stack.md",
    output_mode="count"
)

if tech_conflicts == 0:
    # Technology not in approved stack
    conflict detected: New requirement uses {new_tech}, approved stack has {existing_tech}

# Architecture pattern conflicts
constraint_violations = validate_against_constraints(new_requirement)

if len(constraint_violations) > 0:
    # Architecture constraints violated
    conflict detected: Requirement violates {constraint_name}

# Dependency conflicts
dependency_conflicts = check_dependencies(new_requirement)

if len(dependency_conflicts) > 0:
    # Unapproved dependencies needed
    conflict detected: Requirement needs {new_dependency}, not in approved list
```

**Step 2: Use AskUserQuestion to Resolve Each Conflict**

```
For each conflict:
    AskUserQuestion(
        question: "New requirement '{requirement}' conflicts with existing constraint '{constraint}'. How to resolve?",
        header: "Conflict resolution",
        options: [
            {
                label: "Update constraint",
                description: "Modify existing constraint to accommodate new requirement (creates ADR)"
            },
            {
                label: "Modify requirement",
                description: "Adjust new requirement to fit existing constraints"
            },
            {
                label: "Mark as future scope",
                description: "Defer this requirement to future release"
            }
        ],
        multiSelect: false
    )
```

**Step 3: Document Resolution in ADR Requirements**

```
If user chooses "Update constraint":
    # Document ADR requirement for architecture phase
    # Architecture skill will create ADR documenting change

    adr_requirement = {
        "number": get_next_adr_number(),
        "title": f"{constraint_type}-change-for-{requirement_name}",
        "context": f"New requirement from ideation conflicts with existing {context_file}",
        "decision": f"Update {context_file} to allow {new_value}",
        "rationale": user_provided_rationale,
        "consequences": "Impact on existing codebase and future development"
    }

    # Add to requirements spec
    Add section: "## Architecture Decision Requirements"
    Document: "ADR required for {context_file} change - will be created in architecture phase"

If user chooses "Modify requirement":
    # Update requirement to comply with constraint
    original_requirement = requirement
    modified_requirement = modify_to_comply(requirement, constraint)

    # Update in requirements spec
    Edit(
        file_path=requirements_spec,
        old_string=original_requirement,
        new_string=f"{modified_requirement}\n*(Modified to comply with {context_file})*"
    )

    ✓ Requirement modified to fit constraints

If user chooses "Mark as future scope":
    # Move to future scope section
    Move requirement from "In Scope" to "Future Scope"

    # Document reason
    Add note: "Deferred due to conflict with {context_file}"

    ✓ Requirement deferred to future release
```

**Step 4: Update Requirements Spec with Resolved Conflicts**

```
# Add conflict resolution section
Add to requirements spec:

**Constraint Conflicts Resolved:**
1. {Requirement}: {Resolution} (See ADR requirement / Modified / Deferred)
2. [... all conflicts ...]

# Ensure all conflicts documented for architecture phase
```

### When to Use

- Brownfield projects with existing context files
- New requirements need different technology than existing stack
- New architecture patterns conflict with current constraints
- Compliance requirements change
- Anti-pattern violations detected

### Max Recovery Attempts

**Not applicable** - Each conflict requires explicit user decision (not auto-correctable)

**Process:** Detect all conflicts → Present to user → Resolve each → Document resolutions

---

## Error 6: Directory Structure Issues

### Detection

**Symptom:** Expected directories don't exist or have wrong structure

**Detected during:** Phase 6.1 (Artifact Generation)

**Examples:**
- `devforgeai/specs/` directory missing
- `devforgeai/` directory missing
- Subdirectories (Epics/, specs/requirements/) don't exist
- Permission denied when creating directories

**Detection logic:**

```
required_dirs = [
    "devforgeai/specs/Epics/",
    "devforgeai/specs/requirements/",
    "devforgeai/specs/adrs/",
    "devforgeai/specs/context/"
]

for dir in required_dirs:
    exists = Glob(pattern=dir)
    if not exists:
        trigger directory_missing_error(dir)
```

### Recovery Procedure

**Step 1: Create Required Directories**

```
required_dirs = [
    "devforgeai/specs/Epics/",
    "devforgeai/specs/requirements/",
    "devforgeai/specs/adrs/",
    "devforgeai/specs/context/"
]

for dir in required_dirs:
    # Create directory using Write/.gitkeep pattern (Constitutional C1 compliant)
    Write(file_path=f"{dir}.gitkeep", content="")

    # Verify creation
    check = Glob(pattern=dir)

    if not check:
        ERROR: Could not create directory {dir}
        # Will report to user in Step 3
```

**Step 2: Check Write Permissions**

```
# Test write to each directory
for dir in required_dirs:
    test_file = f"{dir}/.test-{timestamp}"

    try:
        Write(file_path=test_file, content="test")
        # Clean up test file
        Bash(command=f"rm {test_file}")
        ✓ {dir} writable
    except:
        ERROR: No write permission to {dir}
        permission_errors.append(dir)
```

**Step 3: Report Permission Errors to User**

```
If len(permission_errors) > 0:
    Report: """
    ❌ File System Permission Errors

    Cannot write to directories:
    {list permission_errors}

    Required actions:
    1. Grant write permissions:
       bash
       chmod 755 {dir}

    2. Or create directories manually:
       bash
       mkdir -p devforgeai/specs/Epics
       mkdir -p devforgeai/specs/requirements

    3. Then re-run ideation skill

    If running in container or CI/CD, ensure volume mounts have write permissions.
    """
```

### When to Use

- First time running ideation skill in project
- File system permissions issues
- Project directory structure not initialized
- Running in restricted environments (containers, CI/CD)

### Max Recovery Attempts

**Attempt 1:** Create directories with mkdir -p
**Attempt 2:** Test write permissions, verify creation

**If still failing:** Report to user with manual creation steps

---

## General Recovery Strategy

### Recovery Hierarchy

**Level 1: Auto-Correct (No User Interaction)**
- Missing dates (use today)
- Missing status (use default "Planning")
- ID mismatches (extract from filename)
- Formatting issues (standardize)

**Level 2: Regenerate (Use Existing Data)**
- Missing frontmatter sections (regenerate from phase data)
- Incomplete sections (regenerate from discovery)
- Invalid calculations (recalculate using matrix)

**Level 3: User Resolution (AskUserQuestion)**
- Missing critical data (business-value, title)
- Conflicting requirements
- Constraint violations
- Ambiguous requirements

**Level 4: Manual Intervention (HALT)**
- File system permission errors (after retry)
- Critical validation failures (after 2 attempts)
- Irresolvable conflicts

### Max Retry Policy

**Per-error max retries:**
- Incomplete answers: 3 attempts (follow-ups → examples → assumptions)
- Artifact generation: 2 attempts (mkdir → retry write)
- Complexity errors: 1 attempt (recalculate once)
- Validation failures: 2 attempts (self-heal → regenerate)
- Constraint conflicts: No retries (require user decision)
- Directory issues: 2 attempts (create → verify permissions)

**Global policy:** Never retry infinitely, always have escape hatch (report to user)

---

## Error Reporting Format

### For Auto-Corrected Issues

```
✓ Auto-corrected {count} issues:
  - {Issue 1}: {What was fixed}
  - {Issue 2}: {What was fixed}
```

### For User-Resolvable Issues

```
⚠️ Issues requiring attention ({count}):

**CRITICAL ({count}):**
- {Issue}: {Remediation guidance}
  → Action: {specific steps}
  → File: {file_path}:{line_number}

**HIGH ({count}):**
- {Issue}: {Remediation guidance}

**MEDIUM ({count}):**
- {Issue}: Can defer to {next phase}
```

### For Blocking Errors

```
❌ HALT: {Error Type}

**Problem:** {Clear description of what went wrong}

**Detection:** {What triggered this error}

**Required Action:**
{Specific steps user must take}

**After fixing:**
{How to resume - re-run skill, continue from checkpoint, etc.}
```

---

## References Used in Error Handling

**Loaded as needed:**
- **requirements-elicitation-guide.md** - Domain-specific follow-up questions (Error 1)
- **complexity-assessment-matrix.md** - Recalculation logic (Error 3)
- **validation-checklists.md** - Validation procedures (Error 4)
- **domain-specific-patterns.md** - Pattern guidance for regeneration (Error 4)
- **output-templates.md** - Error reporting templates

---

## Error Handling Success Criteria

Error recovery successful when:
- [ ] Error detected accurately (correct categorization)
- [ ] Self-healing attempted for correctable issues
- [ ] User presented clear options for uncorrectable issues
- [ ] Max retries respected (no infinite loops)
- [ ] Work preserved (no data loss)
- [ ] Clear guidance provided (user knows what to do)
- [ ] Workflow can resume after resolution

**Token Budget:** ~2,000-8,000 tokens per error (load references, attempt recovery, report)

---

## Common Error Scenarios by Phase

**Phase 1:** Brownfield system too complex to analyze (defer detailed analysis)
**Phase 2:** Incomplete user answers (follow-ups, examples, assumptions)
**Phase 3:** Invalid complexity score (recalculate using matrix)
**Phase 4:** Too many epics (>5) or too few (<1) - scope adjustment
**Phase 5:** Constraint conflicts (AskUserQuestion for resolution)
**Phase 6:** Artifact generation failures (mkdir, retry, manual instructions)

**Most common:** Incomplete user answers (Error 1), Validation failures (Error 4)
**Most critical:** Artifact generation failures (Error 2), Constraint conflicts (Error 5)

---

**This reference provides complete error handling for robust ideation workflow execution.**
