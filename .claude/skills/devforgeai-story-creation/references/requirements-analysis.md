# Phase 2: Requirements Analysis

Generate user story and acceptance criteria using requirements-analyst subagent.

## Overview

This phase transforms the feature description into structured user story format and testable acceptance criteria following DevForgeAI standards.

**RCA-007 Enhancements:**
- Step 2.0: Pre-invocation file system snapshot (NEW - Phase 2)
- Step 2.1: Enhanced prompt with 4-section template (Phase 1)
- Step 2.1.5: File creation validation checkpoint (Phase 1)
- Step 2.2: Quality validation (existing, renumbered)
- Step 2.2.5: Contract-based validation (NEW - Phase 2)
- Step 2.2.7: Post-invocation file system diff (NEW - Phase 2)
- Step 2.3: Refine if incomplete (existing, renumbered)

---

## Step 2.0: Pre-Invocation File System Snapshot (NEW - RCA-007 Phase 2)

**Objective:** Capture file system state before subagent execution to detect unauthorized file creation

**CRITICAL:** This step runs BEFORE Step 2.1 (subagent invocation) to establish baseline for diff check.

**Take snapshot:**
```python
# Capture current .story.md files
files_before_subagent = Glob(pattern="devforgeai/specs/Stories/STORY-*.story.md")
story_count_before = len(files_before_subagent)

# Capture potential supporting files for this story
supporting_file_patterns = [
    f"devforgeai/specs/Stories/{story_id}-SUMMARY.md",
    f"devforgeai/specs/Stories/{story_id}-QUICK-START.md",
    f"devforgeai/specs/Stories/{story_id}-VALIDATION-CHECKLIST.md",
    f"devforgeai/specs/Stories/{story_id}-FILE-INDEX.md",
    f"devforgeai/specs/Stories/{story_id}-DELIVERY-SUMMARY.md",
    f"devforgeai/specs/Stories/{story_id}*.md"  # Any file with story ID
]

# Check which files exist before subagent runs
import os
supporting_files_before = []
for pattern in supporting_file_patterns:
    # Expand pattern
    matching_files = Glob(pattern=pattern)
    supporting_files_before.extend(matching_files)

# Store snapshot
snapshot = {
    "timestamp": datetime.now().isoformat(),
    "story_id": story_id,
    "story_files_count": story_count_before,
    "supporting_files": supporting_files_before,
    "total_count": len(supporting_files_before)
}

Display: f"""
Step 2.0: File System Snapshot (Before Subagent)
- Story ID: {story_id}
- Existing .story.md files: {story_count_before}
- Existing files for this story: {len(supporting_files_before)}

Snapshot captured. Proceeding to Step 2.1 (subagent invocation)...
"""
```

---

## Step 2.1: Invoke Requirements Analyst Subagent (ENHANCED - RCA-007 Fix)

**Objective:** Generate user story, acceptance criteria, edge cases, and NFRs

**IMPORTANT:** This prompt has been enhanced to prevent RCA-007 violations (multi-file creation). The subagent MUST return content only (no file creation).

**Prepare detailed prompt for subagent:**

```
Task(
  subagent_type="story-requirements-analyst",  # UPDATED: Skill-specific (RCA-007 Phase 3)
  description="Generate user story content",
  prompt="""Transform feature description into structured user story for DevForgeAI framework.

**═══════════════════════════════════════════════════════════════════════════**
**PRE-FLIGHT BRIEFING:**
**═══════════════════════════════════════════════════════════════════════════**

You are being invoked by the devforgeai-story-creation skill.
This skill will assemble your output into a single .story.md file using story-template.md.

**YOUR ROLE:**
- Generate requirements content (user story, acceptance criteria, edge cases, NFRs)
- Return content as markdown sections
- Do NOT create files
- Parent skill handles file creation (Phase 5: Story File Creation)

**OUTPUT WILL BE USED IN:**
- Phase 5: Story File Creation (assembly into story-template.md)
- Your output is CONTENT for assembly, not a complete deliverable

**WORKFLOW CONTEXT:**
- Current workflow: Story creation (8-phase process)
- Current phase: Phase 2 (Requirements Analysis)
- Next phase: Phase 3 (Technical Specification)
- Final artifact: devforgeai/specs/Stories/{story_id}-{slug}.story.md

**═══════════════════════════════════════════════════════════════════════════**
**CRITICAL OUTPUT CONSTRAINTS:**
**═══════════════════════════════════════════════════════════════════════════**

1. **Format:** Return ONLY markdown text content (no file creation)
2. **Content:** Output will be inserted into story-template.md by parent skill
3. **Files:** Do NOT create separate files (SUMMARY.md, QUICK-START.md, VALIDATION-CHECKLIST.md, FILE-INDEX.md, DELIVERY-SUMMARY.md)
4. **Structure:** Output as sections: User Story, Acceptance Criteria, Edge Cases, Data Validation Rules, Non-Functional Requirements
5. **Assembly:** Parent skill (devforgeai-story-creation) will assemble all sections into single .story.md file
6. **Size:** Maximum 50,000 characters (fits in story-template.md capacity)

**Contract Reference:** .claude/skills/devforgeai-story-creation/contracts/requirements-analyst-contract.yaml

**═══════════════════════════════════════════════════════════════════════════**
**PROHIBITED ACTIONS:**
**═══════════════════════════════════════════════════════════════════════════**

You MUST NOT:
1. ❌ Create files using Write tool
2. ❌ Create files using Edit tool on non-existent files
3. ❌ Create files using Bash with output redirection (>, >>, cat <<EOF)
4. ❌ Return file paths as output (e.g., "Created: STORY-009-summary.md")
5. ❌ Return file creation statements (e.g., "File created successfully")
6. ❌ Generate multi-file deliverables (SUMMARY, QUICK-START, INDEX, etc.)
7. ❌ Write to disk in any form
8. ❌ Create comprehensive project structures (you generate CONTENT, not PROJECTS)

**Why prohibited:**
- Parent skill handles all file creation (Phase 5: Story File Creation)
- Your output is assembled with other content (tech spec from Phase 3, UI spec from Phase 4)
- Multi-file output violates DevForgeAI single-file design
- Creates framework specification violations (RCA-007)

**What to do instead:**
- ✅ Return markdown text as string
- ✅ Structure content with section headers (## User Story, ## Acceptance Criteria, etc.)
- ✅ Include all required information in text output
- ✅ Let parent skill decide file structure and naming

**═══════════════════════════════════════════════════════════════════════════**
**EXPECTED OUTPUT FORMAT:**
**═══════════════════════════════════════════════════════════════════════════**

Your output should look like this (MARKDOWN TEXT, not files):

```markdown
## User Story
**As a** [role - specific persona, not "user"],
**I want** [action - what functionality],
**so that** [benefit - business value].

## Acceptance Criteria

### AC1: [Clear, testable title]
**Given** [context - initial state]
**When** [action - what happens]
**Then** [outcome - expected result]

### AC2: [Title]
**Given** [context]
**When** [action]
**Then** [outcome]

### AC3: [Title]
**Given** [context]
**When** [action]
**Then** [outcome]

(Minimum 3 acceptance criteria)

## Edge Cases
1. **[Edge case scenario]:** [Description and expected behavior]
2. **[Edge case scenario]:** [Description]

(Minimum 2 edge cases)

## Data Validation Rules
1. **[Input parameter]:** [Validation rule]
2. **[Data format]:** [Validation rule]

## Non-Functional Requirements

### Performance
- Response time: [MEASURABLE - e.g., "< 100ms per request (p95)"]
- Throughput: [MEASURABLE - e.g., "1000 requests/second"]

### Security
- Authentication: [SPECIFIC - e.g., "JWT tokens with 15-min expiry"]
- Authorization: [SPECIFIC - e.g., "RBAC with admin/user roles"]

### Reliability
- Error handling: [SPECIFIC - e.g., "Return 400 with error details"]

### Scalability
- Concurrency: [MEASURABLE - e.g., "10,000 concurrent users"]
```

**What your output will become:**
Parent skill will insert your output into story-template.md at line 45:

```markdown
---
id: {story_id}
title: {title}
...
---

## User Story
{YOUR OUTPUT: User Story section}

## Acceptance Criteria
{YOUR OUTPUT: Acceptance Criteria section}

## Technical Specification
{Generated by Phase 3}

## Non-Functional Requirements
{YOUR OUTPUT: NFRs section}

## Edge Cases
{YOUR OUTPUT: Edge Cases section}

...
```

**Final result:** Single .story.md file at devforgeai/specs/Stories/{story_id}-{slug}.story.md

**═══════════════════════════════════════════════════════════════════════════**
**NOW PROCEED WITH REQUIREMENTS ANALYSIS:**
**═══════════════════════════════════════════════════════════════════════════**

**Feature Description:** {feature_description}

**Story Context:**
- Story ID: {story_id}
- Epic: {epic_id or 'None'}
- Priority: {priority}
- Points: {points}

**Generate the following sections as markdown text (NOT files):**

1. **User Story** (As a/I want/So that format)
   - Role: Specific user type (not generic "user")
   - Action: What the user wants to do
   - Benefit: Why this matters (business value)

2. **Acceptance Criteria** (Given/When/Then format, minimum 3)
   - Happy path scenario
   - Error/edge case scenarios
   - Data validation scenarios
   - Each criterion must be testable (can verify pass/fail)

3. **Edge Cases** (minimum 2)
   - Boundary conditions
   - Error conditions
   - Concurrent access scenarios
   - Data corruption scenarios

4. **Data Validation Rules**
   - Input constraints
   - Format requirements
   - Business rule validations

5. **Non-Functional Requirements**
   - Performance: Response time targets (e.g., <500ms, <100ms)
   - Security: Authentication, authorization, encryption needs
   - Reliability: Error handling, retry logic
   - Scalability: Concurrent user targets

**DevForgeAI Standards:**
- No vague terms ("fast", "secure", "user-friendly" without metrics)
- All NFRs must be measurable
- All acceptance criteria must be testable
- Follow spec-driven development principles

**REMINDER:** Return MARKDOWN TEXT ONLY. No file creation. No file paths in output.
"""
)
```

**Expected subagent output:**
- Markdown text with clear section headers
- User story in proper format
- 3+ acceptance criteria (Given/When/Then)
- Edge cases list (minimum 2)
- Data validation rules
- Quantified NFRs (all measurable)
- **NO file creation statements**
- **NO file paths in output**

**Subagent Migration Note (RCA-007 Phase 3):**

**Previous:** General-purpose `requirements-analyst` (`.claude/agents/requirements-analyst.md`)
- Tools: Read, Write, Edit, Grep, Glob, AskUserQuestion
- Purpose: Requirements analysis for ANY context
- Output: May create comprehensive deliverables (6 files)
- Issue: Created STORY-009-SUMMARY.md, QUICK-START.md, VALIDATION-CHECKLIST.md, FILE-INDEX.md, DELIVERY-SUMMARY.md (RCA-007 violation)

**Current:** Skill-specific `story-requirements-analyst` (`.claude/agents/story-requirements-analyst.md`)
- Tools: Read, Grep, Glob, AskUserQuestion (NO Write/Edit)
- Purpose: Requirements ONLY for devforgeai-story-creation
- Output: ONLY markdown content (file creation impossible by design)
- Fix: Cannot create files (Write/Edit tools not available)

**Migration Date:** 2025-11-06

**Fallback:** If `story-requirements-analyst` not available (not deployed yet), falls back to `requirements-analyst` with Phase 1+2 constraints (enhanced prompt + validation checkpoints).

**Benefit:** 99.9% violation prevention vs. 95-99% with general-purpose + constraints

---

## Step 2.1.5: Validate No File Creation (NEW - RCA-007 Fix)

**Objective:** Ensure subagent returned markdown content, not file artifacts

**CRITICAL:** This validation prevents RCA-007 violations (multi-file creation). Execute IMMEDIATELY after subagent returns.

**Validation procedure:**

### Check 1: File Creation Detection

**Search for file creation indicators in subagent output:**

```python
subagent_output_text = subagent_result  # The text returned by requirements-analyst

# Prohibited patterns (file creation indicators)
file_creation_patterns = [
    r"File created:",
    r"\.md created",
    r"STORY-\d+-.*\.md",
    r"Writing to file",
    r"Saved to disk",
    r"Created file:",
    r"Successfully wrote",
    r"Document generated:",
    r"SUMMARY\.md",
    r"QUICK-START\.md",
    r"VALIDATION-CHECKLIST\.md",
    r"FILE-INDEX\.md",
    r"DELIVERY-SUMMARY\.md",
    r"Write\(file_path=",
    r"Edit\(file_path=",
    r"Bash\(command=\"cat >"
]

violations_detected = []

for pattern in file_creation_patterns:
    import re
    if re.search(pattern, subagent_output_text, re.IGNORECASE):
        violations_detected.append({
            "pattern": pattern,
            "severity": "CRITICAL",
            "type": "FILE_CREATION"
        })
```

**If violations detected:**

```python
if violations_detected:
    # Log violation (RCA-007 monitoring)
    import datetime

    log_entry = f"""
[VIOLATION DETECTED]
Timestamp: {datetime.datetime.now().isoformat()}
Story ID: {story_id}
Subagent: requirements-analyst
Parent Skill: devforgeai-story-creation
Phase: Phase 2 (Requirements Analysis)
Violation Type: FILE_CREATION
Patterns Matched: {len(violations_detected)}
{chr(10).join(f"  - {v['pattern']}" for v in violations_detected)}
Output Snippet: {subagent_output_text[:300]}...
Recovery Action: re_invoke
---
"""

    # Append to violation log
    Write(
        file_path="devforgeai/logs/rca-007-violations.log",
        content=log_entry
    )

    # Display violation warning
    Display: f"""
⚠️ RCA-007 Violation Detected

The requirements-analyst subagent attempted to create files instead of returning content.
This violates the devforgeai-story-creation workflow specification.

Violations:
{chr(10).join(f"  [{v['severity']}] Pattern: {v['pattern']}" for v in violations_detected)}

Recovery: Re-invoking subagent with STRICT MODE constraints...
"""

    # Re-invoke with enhanced constraints (STRICT MODE)
    subagent_output_text = Task(
        subagent_type="requirements-analyst",
        description="Generate user story content (STRICT MODE - Retry)",
        prompt=f"""
**═══════════════════════════════════════════════════════════════════════════**
**STRICT MODE - VIOLATION RECOVERY (Retry #1)**
**═══════════════════════════════════════════════════════════════════════════**

Previous invocation violated output constraints by creating files.
This is your SECOND ATTEMPT. You MUST return content only.

VIOLATIONS DETECTED IN FIRST ATTEMPT:
{chr(10).join(f"  - {v['pattern']}" for v in violations_detected)}

**THIS INVOCATION IS STRICTLY CONTENT-ONLY:**
- NO file creation whatsoever
- NO Write tool usage
- NO Edit tool usage
- NO Bash output redirection
- Return ONLY markdown text

**═══════════════════════════════════════════════════════════════════════════**

{original_prompt_with_all_4_sections}

**═══════════════════════════════════════════════════════════════════════════**
**FINAL WARNING: NO FILE CREATION ALLOWED**
**═══════════════════════════════════════════════════════════════════════════**

Return markdown TEXT only. The parent skill will create all files.
"""
    )

    # Re-validate second attempt
    retry_violations = []
    for pattern in file_creation_patterns:
        if re.search(pattern, subagent_output_text, re.IGNORECASE):
            retry_violations.append(pattern)

    if retry_violations:
        # Second attempt also failed - HALT
        HALT: """
❌ CRITICAL: Subagent violated output constraints twice

First attempt: {len(violations_detected)} violations
Second attempt: {len(retry_violations)} violations

Manual intervention required:
1. Review subagent definition: .claude/agents/requirements-analyst.md
2. Check if subagent ignoring prompt constraints
3. Consider creating skill-specific subagent (RCA-007 Phase 3)

Exit Phase 2 - Cannot proceed with multi-file creation
"""
    else:
        # Second attempt succeeded
        Display: """
✓ Recovery Successful

Retry #1 passed validation - subagent returned content only.
No file creation detected.

Proceeding to quality validation...
"""

        # Log successful recovery
        recovery_log = f"""
[VIOLATION RECOVERED]
Timestamp: {datetime.datetime.now().isoformat()}
Story ID: {story_id}
Recovery Result: SUCCESS
Retry Attempt: 1
---
"""
        # Append recovery to log
```

**If no violations detected:**

```python
else:
    # No file creation detected - validation passed
    Display: """
✓ Step 2.1.5: File Creation Validation PASSED

No file creation indicators detected in subagent output.
Output format: Markdown content ✅
File creation: None ✅

Proceeding to quality validation (Step 2.2)...
"""
```

---

## Step 2.2: Validate Subagent Output Quality (RENUMBERED from Step 2.2)

**Objective:** Ensure requirements meet DevForgeAI quality standards

**Load acceptance criteria patterns for validation:**
```
Read(file_path=".claude/skills/devforgeai-story-creation/references/acceptance-criteria-patterns.md")
```

**Validation checks:**

```
Validate user story:
- [ ] Follows "As a [role], I want [action], so that [benefit]" format
- [ ] Role is specific (not "user", but "customer", "admin", "developer")
- [ ] Action is clear and unambiguous
- [ ] Benefit articulates business value

Validate acceptance criteria:
- [ ] Minimum 3 criteria
- [ ] Each follows Given/When/Then structure
- [ ] At least 1 happy path scenario
- [ ] At least 1 error/edge case scenario
- [ ] All criteria are testable (can write automated test)
- [ ] No ambiguous language ("should", "might", "could")

Validate NFRs:
- [ ] Performance targets quantified (e.g., "<500ms response time")
- [ ] Security requirements specific (e.g., "OAuth2 authentication, JWT tokens")
- [ ] No vague terms without metrics

If validation fails:
    # Re-invoke requirements-analyst with specific feedback
    # Or use AskUserQuestion to fill gaps
```

---

## Step 2.2.5: Contract-Based Validation (NEW - RCA-007 Phase 2)

**Objective:** Enforce subagent-skill contract specifications for formal validation

**Load contract:**
```python
contract_path = ".claude/skills/devforgeai-story-creation/contracts/requirements-analyst-contract.yaml"

# Check if contract exists
contract_exists = file_exists(contract_path)

if not contract_exists:
    Display: """
ℹ️ Contract file not found - using Phase 1 validation only

Contract: requirements-analyst-contract.yaml
Status: Not deployed (Phase 2 pending)

Using validation from Step 2.1.5 (file creation check) + Step 2.2 (quality check).
Skip contract validation.
"""
    # Skip to Step 2.3
else:
    # Load and parse contract
    import yaml

    contract_content = Read(file_path=contract_path)
    contract = yaml.safe_load(contract_content)

    # Display contract info
    Display: f"""
Validating against contract:
- Contract: {contract['skill']} <-> {contract['subagent']}
- Version: {contract['contract_version']}
- Phase: {contract['phase']}
"""
```

**Validate constraints:**
```python
violations = []

# Constraint 1: No file creation (redundant with Step 2.1.5 but comprehensive)
if contract['constraints']['no_file_creation']['enabled']:
    prohibited_patterns = contract['validation']['check_no_file_paths']['prohibited_patterns']

    for pattern in prohibited_patterns:
        import re
        if re.search(pattern, subagent_output, re.IGNORECASE):
            violations.append({
                "type": "FILE_CREATION",
                "constraint": "no_file_creation",
                "pattern": pattern,
                "severity": "CRITICAL"
            })

# Constraint 2: Required sections
if contract['validation']['check_sections_present']['enabled']:
    required_sections = contract['validation']['check_sections_present']['required_sections']

    for section in required_sections:
        if f"## {section}" not in subagent_output:
            violations.append({
                "type": "MISSING_SECTION",
                "constraint": "check_sections_present",
                "section": section,
                "severity": "HIGH"
            })

# Constraint 3: AC format
if contract['validation']['check_ac_format']['enabled']:
    min_count = contract['validation']['check_ac_format']['min_count']
    required_keywords = contract['validation']['check_ac_format']['required_keywords']

    ac_section = extract_section(subagent_output, "Acceptance Criteria")
    ac_count = ac_section.count("### AC") if ac_section else 0

    if ac_count < min_count:
        violations.append({
            "type": "INSUFFICIENT_AC",
            "constraint": "check_ac_format",
            "actual": ac_count,
            "required": min_count,
            "severity": "HIGH"
        })

    for keyword in required_keywords:
        if keyword not in ac_section:
            violations.append({
                "type": "MISSING_AC_KEYWORD",
                "constraint": "check_ac_format",
                "keyword": keyword,
                "severity": "MEDIUM"
            })

# Constraint 4: NFR measurability
if contract['validation']['check_nfr_measurability']['enabled']:
    prohibited_terms = contract['validation']['check_nfr_measurability']['prohibited_vague_terms']

    nfr_section = extract_section(subagent_output, "Non-Functional Requirements")

    for term in prohibited_terms:
        if re.search(rf'\b{term}\b', nfr_section, re.IGNORECASE):
            violations.append({
                "type": "VAGUE_NFR",
                "constraint": "check_nfr_measurability",
                "term": term,
                "severity": "MEDIUM"
            })

# Constraint 5: Output size
max_length = contract['constraints']['max_output_length']['value']
actual_length = len(subagent_output)

if actual_length > max_length:
    violations.append({
        "type": "SIZE_EXCEEDED",
        "constraint": "max_output_length",
        "actual": actual_length,
        "max": max_length,
        "severity": "MEDIUM"
    })
```

**Handle violations:**
```python
if violations:
    # Categorize by severity
    critical = [v for v in violations if v['severity'] == "CRITICAL"]
    high = [v for v in violations if v['severity'] == "HIGH"]
    medium = [v for v in violations if v['severity'] == "MEDIUM"]

    # Display violations
    Display: f"""
⚠️ Contract Validation FAILED

Contract: {contract['skill']} <-> {contract['subagent']}
Violations: {len(violations)} total

Critical: {len(critical)}
{format_violations(critical)}

High: {len(high)}
{format_violations(high)}

Medium: {len(medium)}
{format_violations(medium)}
"""

    # Apply error handling from contract
    if critical or high:
        # Get primary violation type
        primary = critical[0] if critical else high[0]
        error_type = primary['type'].lower()
        error_config = contract['error_handling'].get(f'on_{error_type}', {})

        action = error_config.get('action', 're_invoke')

        if action == 're_invoke':
            max_retries = error_config.get('max_retries', 1)

            Display: f"Recovery: Re-invoking subagent (max {max_retries} retries allowed)"

            # Note: Actual re-invocation would happen here in production
            # For now, log the violation and proceed
            # (Re-invocation logic already in Step 2.1.5 for file creation)

            # Log to violation log
            log_violation(contract, violations, "contract_validation")

    # For medium violations, log and continue
    if medium and not critical and not high:
        Display: f"""
⚠️ Medium-severity violations detected ({len(medium)})

These are warnings, not blockers. Continuing with story creation.

Violations:
{format_violations(medium)}

Recommendation: Address these in future iterations.
"""

else:
    # No violations - validation passed
    Display: f"""
✓ Contract Validation PASSED

Contract: {contract['skill']} <-> {contract['subagent']}
Version: {contract['contract_version']}
All constraints satisfied ✅

Validated:
- no_file_creation ✅
- content_only ✅
- required_sections (4/4) ✅
- ac_format (minimum 3, Given/When/Then) ✅
- nfr_measurability ✅
- size limit ({actual_length}/{max_length} chars) ✅

Proceeding to Step 2.3 (Refine if Incomplete)
"""
```

**Helper function:**
```python
def extract_section(text, section_name):
    """Extract content between section header and next header."""
    import re
    pattern = f"## {section_name}(.*?)(?=##|$)"
    match = re.search(pattern, text, re.DOTALL)
    return match.group(1).strip() if match else ""
```

---

## Step 2.2.7: Post-Invocation File System Diff (NEW - RCA-007 Phase 2)

**Objective:** Detect unauthorized files created during subagent execution

**CRITICAL:** This step runs AFTER subagent invocation to compare against snapshot from Step 2.0.

**Compare file system state:**
```python
# Capture current state (after subagent)
files_after_subagent = Glob(pattern="devforgeai/specs/Stories/STORY-*.story.md")
story_count_after = len(files_after_subagent)

# Check for new supporting files
supporting_files_after = []
for pattern in supporting_file_patterns:
    matching_files = Glob(pattern=pattern)
    supporting_files_after.extend(matching_files)

# Calculate diff
new_story_files = list(set(files_after_subagent) - set(files_before_subagent))
new_supporting_files = list(set(supporting_files_after) - set(supporting_files_before))
unauthorized_files = [f for f in new_supporting_files if not f.endswith(f"{story_id}.story.md")]

# Display diff results
Display: f"""
Step 2.2.7: File System Diff Check
- Before: {story_count_before} .story.md files
- After: {story_count_after} .story.md files
- New .story.md files: {len(new_story_files)}
- New supporting files: {len(new_supporting_files)}
- Unauthorized files: {len(unauthorized_files)}
"""
```

**Check for violations:**
```python
if len(unauthorized_files) > 0:
    # CRITICAL VIOLATION: Unauthorized files created
    Display: f"""
❌ CRITICAL: Unauthorized File Creation Detected

The requirements-analyst subagent created unauthorized files during Phase 2.
This is a CRITICAL violation of single-file design principle (RCA-007).

Unauthorized files created:
{chr(10).join(f"  - {f}" for f in unauthorized_files)}

Expected: No files created during Phase 2 (content generation only)
Actual: {len(unauthorized_files)} unauthorized files

Recovery action: Delete unauthorized files and log violation
"""

    # Delete unauthorized files (rollback)
    import os
    for file in unauthorized_files:
        try:
            os.remove(file)
            Display: f"  Deleted: {file}"
        except Exception as e:
            Display: f"  ⚠️ Failed to delete {file}: {e}"

    # Log violation to RCA-007 log
    import datetime
    log_entry = f"""
[FILE CREATION VIOLATION - File System Diff]
Timestamp: {datetime.datetime.now().isoformat()}
Story ID: {story_id}
Phase: Phase 2 (Requirements Analysis)
Subagent: requirements-analyst
Unauthorized files created: {len(unauthorized_files)}
{chr(10).join(f"  - {f}" for f in unauthorized_files)}
Action taken: Files deleted (rollback)
Recovery: Re-invoking subagent with STRICT MODE
---
"""

    # Append to log (or create if first entry beyond header)
    current_log = Read(file_path="devforgeai/logs/rca-007-violations.log")
    updated_log = current_log + log_entry

    Write(file_path="devforgeai/logs/rca-007-violations.log", content=updated_log)

    # Re-invoke subagent with stricter constraints
    Display: """
Recovery: Re-invoking requirements-analyst with STRICT MODE constraints...

This is a file system diff violation (files actually created, not just mentioned in output).
The subagent will be re-invoked with enhanced warnings.
"""

    # Note: Re-invocation would happen here
    # For now, log the violation and HALT (manual intervention needed)

    HALT: """
❌ CRITICAL: File system diff detected unauthorized file creation

Subagent created actual files during execution (not just mentioned in output).
Files have been deleted (rollback completed).

Manual intervention required:
1. Review subagent definition: .claude/agents/requirements-analyst.md
2. Check why file system safeguards were bypassed
3. Implement RCA-007 Phase 3 (skill-specific subagent) immediately

Exit Phase 2 - Cannot proceed safely
"""

elif len(new_story_files) > 1:
    # WARNING: Multiple .story.md files created (suspicious)
    Display: f"""
⚠️ WARNING: Multiple .story.md files created during Phase 2

Expected: 0 files created (Phase 2 generates content only, Phase 5 creates files)
Actual: {len(new_story_files)} .story.md files created

Files:
{chr(10).join(f"  - {f}" for f in new_story_files)}

This may indicate subagent created files prematurely.
Proceeding with caution...
"""

    # Log warning
    log_warning = f"""
[WARNING - Multiple Story Files]
Timestamp: {datetime.datetime.now().isoformat()}
Story ID: {story_id}
New files: {len(new_story_files)}
{chr(10).join(f"  - {f}" for f in new_story_files)}
Note: Phase 2 should not create files (Phase 5 handles file creation)
---
"""

    current_log = Read(file_path="devforgeai/logs/rca-007-violations.log")
    Write(file_path="devforgeai/logs/rca-007-violations.log", content=current_log + log_warning)

else:
    # No unauthorized files - file system diff passed
    Display: f"""
✓ Step 2.2.7: File System Diff PASSED

Before subagent: {story_count_before} .story.md files
After subagent: {story_count_after} .story.md files
New files: {len(new_story_files)}

Supporting files before: {len(supporting_files_before)}
Supporting files after: {len(supporting_files_after)}
Unauthorized files: 0

Phase 2 file creation compliance: ✅ PASS

Proceeding to Step 2.3 (Refine if Incomplete)...
"""
```

---

## Step 2.3: Refine if Incomplete (RENUMBERED from Step 2.3)

**Objective:** Fill gaps in subagent output via user questions

**If subagent output incomplete or vague:**

```
Use AskUserQuestion to clarify:

Example: If NFR says "fast"
Question: "What performance target is acceptable?"
Options:
  - "High performance (<100ms response, >10k concurrent users)"
  - "Standard performance (<500ms response, 1k-10k users)"
  - "Moderate performance (<2s response, <1k users)"

Example: If acceptance criteria vague
Question: "What specific behavior should be tested?"
Options:
  - Provide examples of good AC
  - Ask for edge cases
  - Request error scenarios
```

---

## Subagent Coordination

**Subagent used:** requirements-analyst

**Invoked by:** This phase (Step 2.1)

**Input provided:**
- Feature description
- Story metadata (ID, epic, priority, points)
- DevForgeAI standards

**Output expected:**
- User story
- 3+ acceptance criteria
- Edge cases
- Data validation rules
- Measurable NFRs

**Reference files used by subagent:**
- acceptance-criteria-patterns.md (1,259 lines - Given/When/Then templates)

---

## Output

**Phase 2 produces:**
- ✅ User story in proper format
- ✅ 3+ testable acceptance criteria
- ✅ Edge cases documented
- ✅ Data validation rules defined
- ✅ Measurable NFRs

---

## Error Handling

**Error 1: Subagent output incomplete**
- **Detection:** Missing user story, <3 AC, or vague NFRs
- **Recovery:** Re-invoke with specific feedback, or use AskUserQuestion to fill gaps

**Error 2: Acceptance criteria not testable**
- **Detection:** AC uses ambiguous language ("should", "might", "could")
- **Recovery:** Refine with specific assertions ("must", "will", "shall")

**Error 3: NFRs not measurable**
- **Detection:** Terms like "fast", "secure", "scalable" without metrics
- **Recovery:** Use AskUserQuestion to quantify targets

See `error-handling.md` for comprehensive error recovery procedures.

---

## Next Phase

**After Phase 2 completes →** Phase 3: Technical Specification

Load `technical-specification-creation.md` for Phase 3 workflow.
