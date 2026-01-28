# Phase 6.4: Self-Validation Workflow

Execute comprehensive validation checks on generated artifacts with self-healing for correctable issues.

## Overview

Phase 6.4 validates all generated artifacts (epic documents, requirements spec) meet DevForgeAI quality standards before presenting to user. This phase includes automatic correction of common issues (missing dates, invalid IDs, placeholder defaults) and halts on critical failures requiring user intervention.

**Duration:** 2-5 minutes
**Output:** Validation status, auto-corrected issues list, remaining issues for user resolution

---

## Validation Strategy

**Three-tier validation approach:**

1. **Auto-correctable issues** - Fix automatically, report what was fixed
2. **User-resolvable issues** - Report to user with remediation guidance, continue
3. **Critical failures** - HALT workflow, require user intervention before proceeding

**Key principle:** Self-healing improves UX by fixing obvious issues automatically (unlike ui-generator Phase 7 which requires user resolution for all issues).

---

## Load Validation Checklists

```
Read(file_path=".claude/skills/devforgeai-ideation/references/validation-checklists.md")
```

This reference provides comprehensive validation logic for:
- Artifact creation verification
- Epic content quality standards
- Requirements specification quality
- Complexity assessment validation
- Handoff readiness criteria

**See validation-checklists.md for complete validation procedures.**

---

## Validation Sequence

### Step 1: Verify Artifact Creation

**Check epic documents exist:**

```
epic_files = Glob(pattern="devforgeai/specs/Epics/EPIC-*.epic.md")
epic_count = len(epic_files)

if epic_count == 0:
    # CRITICAL failure
    ERROR: No epic documents created

    Self-healing attempt:
    1. Review Phase 4 decomposition data
    2. Regenerate epics from decomposition
    3. Retry validation

    If still failing after 1 retry:
        HALT: Report to user - "Epic generation failed. Manual intervention required."
```

**Check requirements spec exists (if requested in Step 6.2):**

```
if requirements_spec_requested:
    req_files = Glob(pattern="devforgeai/specs/requirements/*.md")

    if len(req_files) == 0:
        # CRITICAL failure
        ERROR: No requirements specification created

        Self-healing attempt:
        1. Regenerate requirements spec from Phase 2-5 data
        2. Retry validation

        If still failing after 1 retry:
            HALT: Report to user - "Requirements spec generation failed."
```

---

### Step 2: Validate Epic Content Quality

**For each epic file:**

```
for epic_file in epic_files:
    # Read frontmatter (first 20 lines)
    Read(file_path=epic_file, limit=20)

    # Validate YAML frontmatter
    validate_frontmatter(epic_file):
        - [ ] id field matches filename (e.g., EPIC-001 in filename → id: EPIC-001)
        - [ ] title field present and non-empty
        - [ ] business-value field quantified (measurable outcome)
        - [ ] status field = "Planning" (default for new epics)
        - [ ] priority field = High|Medium|Low
        - [ ] created date in YYYY-MM-DD format
        - [ ] complexity-score present (from Phase 3)
        - [ ] architecture-tier present (Tier 1-4)

    # Auto-correct common issues
    if missing_created_date:
        # Self-heal: Add today's date
        Edit(epic_file, old_string="created:", new_string=f"created: {today_date}")
        auto_corrected.append("Added missing created date")

    if status_missing:
        # Self-heal: Set default status
        Edit(epic_file, old_string="status:", new_string="status: Planning")
        auto_corrected.append("Set default status to Planning")

    if id_mismatch:
        # Self-heal: Correct ID to match filename
        correct_id = extract_id_from_filename(epic_file)
        Edit(epic_file, old_string=f"id: {wrong_id}", new_string=f"id: {correct_id}")
        auto_corrected.append(f"Corrected ID to {correct_id}")

    # CRITICAL failures (cannot auto-correct)
    if missing_title:
        CRITICAL: Epic title missing - cannot auto-generate meaningful title
        HALT: Require user to provide epic title

    if missing_business_value:
        CRITICAL: Business value not quantified
        HALT: Require user to specify measurable business outcome
```

---

### Step 2.5: Validate Section Compliance

**Validate epic contains all constitutional sections:**

```
REQUIRED_SECTIONS = [
    "## Business Goal",
    "## Problem Statement",
    "## Vision",
    "## Scope",
    "## Success Metrics",
    "## Constraints",
    "## Assumptions",
    "## Dependencies",
    "## Technical Approach",
    "## Risks",
    "## Out of Scope"
]

for epic_file in epic_files:
    Read(file_path=epic_file)

    # Check for each required section
    missing_sections = []
    for section in REQUIRED_SECTIONS:
        if section not in epic_content:
            missing_sections.append(section)

    if len(missing_sections) > 0:
        # Report missing sections in error message
        CRITICAL: Section compliance failed - list missing sections: {missing_sections}

        # Determine remediation path based on count
        if len(missing_sections) <= 3:
            # Self-heal: Add placeholder content
            for section in missing_sections:
                add_placeholder_section(epic_file, section)
            auto_corrected.append(f"Added placeholder sections: {missing_sections}")
        else:
            # Too many missing - cannot self-heal
            HALT: Epic missing too many sections (>3)
            Recommend: Regenerate epic using /ideate command
            # User intervention required before proceeding
```

#### Schema Completeness Check

**Detect schema references in epic content:**

```
Grep(pattern="schema|interface|structure|format", path=epic_file, output_mode="content")
```

**Validate schema definitions exist:**

```
IF schema_reference found AND no code block definition:
    WARNING: Schema '{schema_name}' referenced but not defined
    Recommend: Add explicit schema definition
```

**Recommended JSON Schema format:**

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "fieldName": {"type": "string"}
  },
  "required": ["fieldName"]
}
```

**Cross-session context rule:** Another Claude session must be able to implement features without asking 'What does this schema look like?'

---

### Step 3: Validate Requirements Specification Quality

**If requirements spec created:**

```
req_file = req_files[0]
Read(file_path=req_file)

Validate structure:
- [ ] Project overview present (minimum 100 words)
- [ ] Functional requirements categorized (minimum 5 requirements)
- [ ] Non-functional requirements quantified (no vague "fast", "scalable")
- [ ] Data models documented (entities, attributes, relationships)
- [ ] Integration requirements specified (protocols, formats, authentication)
- [ ] Complexity assessment present with score 0-60
- [ ] Architecture tier recommended

# Check for vague NFRs
vague_terms = Grep(pattern="fast|slow|scalable|secure|reliable", path=req_file, output_mode="count")

if vague_terms > 0:
    # MEDIUM issue (user-resolvable, not critical)
    WARNING: Requirements spec contains {vague_terms} vague NFR terms
    Recommend: Quantify NFRs during architecture phase
    # Continue (not HALT-worthy)

# Check for placeholder content
placeholders = Grep(pattern="TODO|TBD|FIXME|\[.*\]", path=req_file, output_mode="count")

if placeholders > 5:
    # HIGH issue
    WARNING: Requirements spec has {placeholders} placeholders
    Recommend: Complete requirements before proceeding to architecture
    # Continue (will be caught in architecture phase)
```

---

### Step 4: Validate Complexity Assessment

**Extract complexity data from requirements spec:**

```
# Complexity should be in requirements spec
Read(file_path=req_file)

# Search for complexity section
complexity_section = Grep(pattern="## Complexity Assessment", path=req_file, -A=20, output_mode="content")

Validate:
- [ ] Total score in range 0-60
- [ ] Score breakdown present for all 4 dimensions:
  - Functional: 0-20
  - Technical: 0-20
  - Team/Org: 0-10
  - Non-Functional: 0-10
- [ ] Architecture tier matches score:
  - 0-15 = Tier 1 (Simple)
  - 16-30 = Tier 2 (Moderate)
  - 31-45 = Tier 3 (Complex)
  - 46-60 = Tier 4 (Enterprise)
- [ ] Technology recommendations present for tier

# Auto-correct tier mismatch
if tier_doesnt_match_score:
    correct_tier = calculate_tier_from_score(complexity_score)
    Edit(req_file, old_string=f"Tier: {wrong_tier}", new_string=f"Tier: {correct_tier}")
    auto_corrected.append(f"Corrected tier to {correct_tier} based on score {complexity_score}")

# CRITICAL failure
if complexity_score < 0 or complexity_score > 60:
    CRITICAL: Invalid complexity score {complexity_score}
    # Self-heal: Recalculate using complexity-assessment-matrix.md
    Read(file_path=".claude/skills/devforgeai-ideation/references/complexity-assessment-matrix.md")
    recalculated_score = recalculate_complexity()
    # Update requirements spec
    # Retry validation
```

---

### Step 5: Validate Handoff Readiness

**Final checklist before completion:**

```
- [ ] All planned epics created and validated
- [ ] Requirements specification complete (no TBD/TODO placeholders)
- [ ] Complexity assessment finalized (score 0-60, tier 1-4)
- [ ] No critical ambiguities remain (all resolved via AskUserQuestion)
- [ ] All assumptions documented with validation flags
- [ ] Risks identified with mitigation strategies (minimum 2 risks)
- [ ] Success metrics measurable (not vague like "better UX")
- [ ] Acceptance criteria testable (can verify pass/fail)
```

**If all validations pass:**

```
✅ Validation complete - All quality checks passed

Auto-corrected issues ({count}):
{list of auto-corrected issues}

→ Proceed to Phase 6.5 (Completion Summary)
```

**If validation fails after self-healing:**

```
⚠️ Validation issues detected

Auto-corrected ({auto_count}):
{list of auto-corrected issues}

Remaining issues ({remaining_count}):
{list with severity and remediation guidance}

CRITICAL issues: {count}
HIGH issues: {count}
MEDIUM issues: {count}

Action required:
- CRITICAL: Fix before proceeding
- HIGH: Strongly recommend fixing
- MEDIUM: Can defer to architecture phase

→ Fix issues or proceed with warnings
```

---

## Validation Success Criteria

**Zero CRITICAL failures:**
- Missing artifacts (epics, requirements spec)
- Invalid YAML structure (unparseable frontmatter)
- Missing mandatory fields (title, business-value, id)
- Complexity score out of range (not 0-60)

**Zero HIGH failures:**
- Vague requirements (>10 instances of "fast", "scalable", etc.)
- Untestable acceptance criteria
- Missing data models (no entities documented)
- Missing success metrics (no measurable outcomes)

**MEDIUM/LOW failures documented with self-healing attempts:**
- Minor placeholder content (<5 instances)
- Missing created dates (auto-corrected)
- Status field defaults (auto-corrected)
- ID mismatches (auto-corrected)

---

## Output from Step 6.4

**Validation Report (internal):**

```markdown
## Validation Results

**Status:** {PASSED|PASSED WITH WARNINGS|FAILED}

### Auto-Corrected Issues ({count})
1. {Issue 1}: {What was fixed}
2. {Issue 2}: {What was fixed}

### Remaining Issues ({count})

**CRITICAL ({count}):**
- {Issue}: {Remediation guidance}

**HIGH ({count}):**
- {Issue}: {Remediation guidance}

**MEDIUM ({count}):**
- {Issue}: {Can defer to architecture phase}

### Validation Summary
- Epics validated: {count}/{total}
- Requirements spec validated: {PASS|FAIL}
- Complexity assessment validated: {PASS|FAIL}
- Handoff readiness: {READY|NOT READY}
```

**Decision:**
- If PASSED or PASSED WITH WARNINGS → Proceed to Phase 6.5
- If FAILED with CRITICAL issues → HALT, report to user, require fixes

---

## References Used in Step 6.4

**Primary:**
- **validation-checklists.md** (569 lines) - Complete validation procedures

**On Error:**
- **error-handling.md** - Validation failure recovery procedures

---

## Success Criteria

Validation complete when:
- [ ] All epic documents validated
- [ ] Requirements spec validated (if created)
- [ ] Complexity assessment validated
- [ ] Auto-correctable issues fixed
- [ ] CRITICAL issues resolved (or halted)
- [ ] Validation report generated
- [ ] Ready to proceed to completion summary

**Token Budget:** ~2,000-4,000 tokens (load checklists, validate, auto-correct)

---

**Next Step:** Phase 6.5 (Present Completion Summary) - Load completion-handoff.md
