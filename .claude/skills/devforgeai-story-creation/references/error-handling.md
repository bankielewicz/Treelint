# Error Handling & Recovery Procedures

Complete error handling for story creation workflow.

## Overview

This reference documents all error scenarios across the 8-phase story creation workflow and provides specific recovery procedures.

---

## Error 1: Story ID Conflicts

**Problem:** Story file already exists with generated ID

**Detection:**
```
story_files = Glob(pattern=f"devforgeai/specs/Stories/{story_id}-*.story.md")
if story_files:
    # Conflict detected
```

**Recovery:**
```
while file_exists(f"devforgeai/specs/Stories/{story_id}-*.md"):
    # Increment story number
    story_number += 1
    story_id = f"STORY-{story_number:03d}"

# Use next available ID
# Check for gaps in sequence (e.g., STORY-001, STORY-003 missing 002)
# Fill gaps before incrementing
```

**Phase:** Phase 1 (Story Discovery)

---

## Error 2: Subagent Output Incomplete

**Problem:** requirements-analyst or api-designer returns incomplete output

**Detection:**
```
validate_subagent_output(output):
  - Check for user story presence
  - Count acceptance criteria (must be ≥3)
  - Verify Given/When/Then structure
  - Check NFRs are measurable
```

**Recovery:**
```
# Identify missing elements
missing_elements = validate_subagent_output(output)

if missing_elements:
    # Re-invoke with specific feedback
    Task(
      subagent_type="requirements-analyst",
      prompt=f"Previous output missing: {missing_elements}. Please regenerate complete output including: {specific_requirements}"
    )

    # If still incomplete after retry:
    Use AskUserQuestion to fill gaps manually
```

**Phase:** Phase 2 (Requirements Analysis) or Phase 3 (Technical Specification)

---

## Error 3: Epic/Sprint Not Found

**Problem:** User selects epic/sprint that doesn't exist

**Detection:**
```
epic_file_path = f"devforgeai/specs/Epics/{epic_id}.epic.md"
epic_exists = Glob(pattern=epic_file_path)

if not epic_exists:
    # Epic file not found
```

**Recovery:**
```
# Refresh epic/sprint lists
epic_files = Glob(pattern="devforgeai/specs/Epics/EPIC-*.epic.md")
sprint_files = Glob(pattern="devforgeai/specs/Sprints/Sprint-*.md")

# Present updated lists
AskUserQuestion with refreshed options

# Or allow user to proceed without association
```

**Phase:** Phase 1 (Story Discovery)

---

## Error 4: UI Specification Ambiguous

**Problem:** UI requirements unclear, can't generate mockup

**Detection:**
```
if requires_ui and (
    no_components_identified or
    unclear_layout or
    missing_interaction_flows
):
    # UI spec insufficient
```

**Recovery:**
```
Use AskUserQuestion for clarification:

Question: "Describe the main UI components needed"
Examples: "Login form with email/password", "Data table with search"

Question: "What user actions are required?"
Examples: "Submit form", "Click row to view details", "Filter by date"

Question: "What accessibility level is required?"
Options:
  - "WCAG AA compliance (standard)"
  - "WCAG AAA compliance (strict)"
  - "Basic accessibility (keyboard + screen reader)"

# Generate mockup from clarified requirements
```

**Phase:** Phase 4 (UI Specification)

---

## Error 5: File Write Failures

**Problem:** Cannot write story file (permissions, disk space, path issues)

**Detection:**
```
try:
    Write(file_path=story_path, content=story_content)
except WriteError:
    # Write failed
```

**Recovery:**
```
Try:
    Write(file_path=story_path, content=story_content)
Except write error:
    # Ensure directory exists
    Bash(mkdir -p devforgeai/specs/Stories/)

    # Check write permissions
    # Retry write

    # If still failing:
    Report: """
    ERROR: Cannot write story file

    Manual creation steps:
    1. Create file: {story_path}
    2. Copy this content:

    {story_content}

    3. Save file
    """
```

**Phase:** Phase 5 (Story File Creation)

---

## Error 6: Validation Failures (Phase 7)

**Problem:** Story quality validation fails

**Detection:**
```
validation_result = run_validation_checklist()

if not validation_result.passed:
    # Validation failed
    critical_failures = [f for f in validation_result.failures if f.severity == "CRITICAL"]
    high_failures = [f for f in validation_result.failures if f.severity == "HIGH"]
```

**Recovery:**
```
Max retries: 2

attempt = 0
while attempt < 2:
    validation_result = run_validation_checklist()

    if validation_result.passed:
        ✓ Validation succeeded
        break
    else:
        # Self-healing
        for failure in validation_result.failures:
            if failure.severity == "CRITICAL":
                # Regenerate failing section
                # Edit story file
            elif failure.severity == "HIGH":
                # Fix if possible, else report

        attempt += 1

if attempt >= 2:
    # Report failures to user
    Report: """
    ⚠️ Story validation issues detected

    Issues:
    {list failures}

    Recommendations:
    1. Review story file: {story_path}
    2. Fix issues manually or ask me for specific changes
    3. Validation will run again during /dev command
    """
```

**Phase:** Phase 7 (Self-Validation)

---

## Error 7: Epic/Sprint Linking Failed

**Problem:** Cannot update epic or sprint file with story reference

**Detection:**
```
# After Edit, verify story_id present
epic_content = Read(file_path=epic_file_path)
if story_id not in epic_content:
    # Linking failed
```

**Recovery:**
```
# Check if section exists
if "## Stories" not in epic_content:
    # Section missing, need to create it first
    # Then retry linking

# If section exists but Edit failed:
    # Try alternative old_string anchors
    # Or manually append to end of section
    # Verify success

# If all retries fail:
    Report: """
    ⚠️ Could not auto-link story to epic

    Manual linking steps:
    1. Open: {epic_file_path}
    2. Add to Stories section: [{story_id}] {story_title}
    """
```

**Phase:** Phase 6 (Epic/Sprint Linking)

---

## Error 8: Missing Required Context (Feature Description)

**Problem:** No feature description provided to skill

**Detection:**
```
feature_description = extract_from_conversation("Feature Description:", "Feature:")

if not feature_description or len(feature_description) < 10:
    # Description missing or too vague
```

**Recovery:**
```
AskUserQuestion(
  questions=[{
    question: "Please describe the feature you want to create a story for",
    header: "Feature description",
    options: [
      {label: "CRUD operation", description: "..."},
      {label: "Authentication/Authorization", description: "..."},
      {label: "Workflow/Process", description: "..."},
      {label: "Reporting/Analytics", description: "..."}
    ],
    multiSelect: false
  }]
)

# Then ask: "Provide detailed description of the {feature_type} feature"
# Minimum 10 words required
```

**Phase:** Phase 1 (Story Discovery)

---

## General Recovery Strategy

**For all errors:**

1. **Detect early** - Validate inputs and outputs at each phase
2. **Self-heal when possible** - Auto-correct common issues (missing dates, IDs)
3. **Ask when ambiguous** - Use AskUserQuestion for clarification
4. **Retry limited** - Max 2 attempts before reporting to user
5. **Report clearly** - Specific error messages with recovery steps
6. **Preserve state** - Don't lose work from previous phases
7. **Enable manual recovery** - Provide exact steps user can take

---

## Error Severity Levels

**CRITICAL (blocking):**
- Missing story ID
- Invalid YAML frontmatter
- File write failures
- Zero acceptance criteria
- Missing user story

**HIGH (should fix):**
- Vague NFRs (no metrics)
- Non-testable acceptance criteria
- Incomplete technical specification
- Missing epic/sprint linking (when should exist)

**MEDIUM (nice to fix):**
- Suboptimal acceptance criteria wording
- Missing optional sections
- Formatting inconsistencies

**LOW (informational):**
- Extra sections not in template
- Non-standard but valid formatting

---

## Recovery Decision Matrix

| Error Type | Severity | Auto-Heal? | Ask User? | Report & Continue? |
|------------|----------|------------|-----------|-------------------|
| Story ID conflict | CRITICAL | ✅ Yes (increment) | ❌ No | ❌ No |
| Missing AC | CRITICAL | ✅ Yes (retry subagent) | ✅ If retry fails | ❌ No |
| Vague NFR | HIGH | ❌ No | ✅ Yes (quantify) | ✅ If user declines |
| Missing UI mockup | MEDIUM | ❌ No | ✅ Yes (clarify) | ✅ If backend-only |
| File write fail | CRITICAL | ✅ Yes (retry) | ❌ No | ✅ Manual steps |
| Validation fail | HIGH | ✅ Yes (2 retries) | ✅ If retries fail | ✅ With warnings |

---

## Testing Error Recovery

**Validation tests:**
- [ ] Story ID conflict → Auto-increment works
- [ ] Subagent incomplete → Retry successful
- [ ] Epic not found → User clarification prompt
- [ ] UI ambiguous → User clarification successful
- [ ] Write fails → Manual steps provided
- [ ] Validation fails → Self-healing + report

---

## Related References

**For phase-specific errors:**
- story-discovery.md (Phase 1 errors)
- requirements-analysis.md (Phase 2 errors)
- technical-specification-creation.md (Phase 3 errors)
- ui-specification-creation.md (Phase 4 errors)
- story-file-creation.md (Phase 5 errors)
- epic-sprint-linking.md (Phase 6 errors)
- story-validation-workflow.md (Phase 7 errors)
- completion-report.md (Phase 8 errors)

**For validation procedures:**
- validation-checklists.md (1,038 lines) - Detailed validation logic
